/// quickstart: Check wallet state and emit guided onboarding steps for new users.
///
/// Flow:
///   1. Resolve Solana wallet address (sync, via onchainos)
///   2. Fetch SOL balance via Solana RPC getBalance
///   3. Emit JSON with status + next steps
use anyhow::Result;

const SOLANA_RPC: &str = "https://api.mainnet-beta.solana.com";
const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;
const MIN_SOL_READY: f64 = 0.01;

/// Fetch SOL balance in lamports via Solana JSON-RPC getBalance.
async fn sol_balance_lamports(wallet: &str) -> u64 {
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": [wallet, {"commitment": "confirmed"}]
    });
    match client
        .post(SOLANA_RPC)
        .json(&body)
        .send()
        .await
    {
        Ok(resp) => {
            match resp.json::<serde_json::Value>().await {
                Ok(json) => json["result"]["value"].as_u64().unwrap_or(0),
                Err(_) => 0,
            }
        }
        Err(_) => 0,
    }
}

pub async fn run() -> Result<()> {
    // Resolve wallet (sync)
    let wallet = crate::onchainos::resolve_wallet_solana()?;

    // Progress to stderr
    let short = &wallet[..wallet.len().min(8)];
    eprintln!("Checking assets for {}... on Solana...", short);

    // Fetch SOL balance
    let lamports = sol_balance_lamports(&wallet).await;
    let sol = lamports as f64 / LAMPORTS_PER_SOL;
    let sol_str = format!("{:.6}", sol);

    let (status, suggestion, next_command, onboarding_steps) = if sol >= MIN_SOL_READY {
        let steps = serde_json::json!([
            {
                "step": 1,
                "description": "Get a swap quote (no gas):",
                "command": format!(
                    "raydium-plugin get-swap-quote --input-mint {} --output-mint {} --amount 0.1",
                    SOL_MINT, USDC_MINT
                )
            },
            {
                "step": 2,
                "description": "Execute swap:",
                "command": format!(
                    "raydium-plugin swap --input-mint {} --output-mint {} --amount 0.1 --confirm",
                    SOL_MINT, USDC_MINT
                )
            },
            {
                "step": 3,
                "description": "Get token price:",
                "command": format!("raydium-plugin get-token-price --mints {}", SOL_MINT)
            }
        ]);
        (
            "ready",
            "Your wallet has SOL. Get a quote or swap tokens on Raydium.",
            format!(
                "raydium-plugin get-swap-quote --input-mint {} --output-mint {} --amount 0.1",
                SOL_MINT, USDC_MINT
            ),
            steps,
        )
    } else {
        let steps = serde_json::json!([
            {
                "step": 1,
                "description": "Send SOL to your wallet on Solana mainnet:",
                "wallet": wallet,
                "note": "Minimum recommended: 0.1 SOL (covers fees + swap amount)"
            },
            {
                "step": 2,
                "description": "Run quickstart again:",
                "command": "raydium-plugin quickstart"
            }
        ]);
        (
            "no_funds",
            "Send SOL to your wallet before swapping.",
            "raydium-plugin quickstart".to_string(),
            steps,
        )
    };

    let output = serde_json::json!({
        "ok": true,
        "about": "Raydium is Solana's leading AMM — swap tokens at competitive rates with deep liquidity across hundreds of pairs.",
        "wallet": wallet,
        "chain": "Solana",
        "assets": {
            "sol_balance": sol_str
        },
        "status": status,
        "suggestion": suggestion,
        "next_command": next_command,
        "onboarding_steps": onboarding_steps
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
