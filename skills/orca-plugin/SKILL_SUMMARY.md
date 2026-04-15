
# orca-plugin -- Skill Summary

## Overview
This plugin provides access to Orca's concentrated liquidity AMM on Solana, enabling token swaps and pool queries through the Whirlpools CLMM program. It offers both read operations (pool queries, swap quotes) via direct API calls and write operations (token swaps) with comprehensive safety checks including security scanning, price impact analysis, and user confirmation requirements.

## Usage
Install the plugin and use commands like `orca get-pools` to query liquidity pools, `orca get-quote` to get swap estimates, and `orca swap` to execute token swaps. Always run swaps with `--dry-run` first and confirm with the user before executing real transactions.

## Commands
| Command | Description |
|---------|-------------|
| `orca get-pools --token-a <MINT_A> --token-b <MINT_B>` | List Whirlpool pools for a token pair |
| `orca get-quote --from-token <MINT> --to-token <MINT> --amount <AMOUNT>` | Get swap quote estimate |
| `orca swap --from-token <MINT> --to-token <MINT> --amount <AMOUNT>` | Execute token swap with safety checks |

## Triggers
Activate this skill when users want to swap tokens on Solana, query Orca liquidity pools, get swap quotes, or need access to the Orca/Whirlpools DEX. Also responds to phrases like "orca swap", "swap tokens on solana", "orca pools", and "get swap quote".
