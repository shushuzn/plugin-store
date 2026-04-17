
# morpho-plugin -- Skill Summary

## Overview
This skill enables interaction with Morpho, a permissionless lending protocol with over $5B TVL. It provides access to both Morpho Blue isolated lending markets and MetaMorpho curated vaults across Ethereum and Base networks. Users can supply assets to earn yield, borrow against collateral, manage positions with health factor monitoring, and claim rewards. The skill prioritizes safety with mandatory transaction previews and user confirmation before any funds movement.

## Usage
Run pre-flight dependency checks once per session, then use commands like `morpho positions` to view your portfolio, `morpho markets --asset USDC` to browse lending opportunities, or `morpho supply --vault <address> --asset USDC --amount 1000` to deposit (always preview first without --confirm, then re-run with --confirm after user approval).

## Commands
- `morpho positions` - View all Morpho Blue and MetaMorpho positions with health factors
- `morpho markets [--asset SYMBOL]` - List lending markets with APYs and utilization rates
- `morpho vaults [--asset SYMBOL]` - Browse MetaMorpho vaults with curators and yields
- `morpho supply --vault <addr> --asset <symbol> --amount <n> [--confirm]` - Deposit to MetaMorpho vault
- `morpho withdraw --vault <addr> --asset <symbol> --amount <n>|--all [--confirm]` - Withdraw from vault
- `morpho borrow --market-id <hex> --amount <n> [--confirm]` - Borrow from Morpho Blue market
- `morpho repay --market-id <hex> --amount <n>|--all [--confirm]` - Repay debt (dust-free with --all)
- `morpho supply-collateral --market-id <hex> --amount <n> [--confirm]` - Add collateral
- `morpho withdraw-collateral --market-id <hex> --amount <n>|--all [--confirm]` - Remove collateral
- `morpho claim-rewards [--confirm]` - Claim Merkl incentive rewards
- Global flags: `--chain <1|8453>`, `--from <addr>`, `--dry-run`

## Triggers
Activate this skill when users mention Morpho lending/borrowing operations, earning yield on Morpho vaults, checking Morpho positions or health factors, viewing lending market rates, or claiming Morpho rewards. Also trigger for phrases like "supply to metamorpho", "borrow from morpho blue", or "morpho interest rates".
