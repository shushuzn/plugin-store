
# morpho-plugin -- Skill Summary

## Overview
The morpho-plugin enables interaction with Morpho, a permissionless lending protocol with over $5B TVL operating on two layers: Morpho Blue isolated lending markets and MetaMorpho ERC-4626 vaults curated by risk managers like Gauntlet and Steakhouse. Users can supply assets to earn yield, borrow against collateral, manage positions, and claim rewards across Ethereum Mainnet and Base networks.

## Usage
Install with `npx skills add okx/plugin-store-community --skill morpho`, then connect your wallet using `onchainos wallet login`. All write operations follow a safe preview-then-confirm flow where commands first show transaction details without `--confirm`, then execute after user approval.

## Commands
| Command | Description |
|---------|-------------|
| `morpho positions` | View all Morpho Blue and MetaMorpho positions with health factors |
| `morpho markets [--asset SYMBOL]` | List Morpho Blue markets with APYs and utilization rates |
| `morpho vaults [--asset SYMBOL]` | Browse MetaMorpho vaults with APYs and curators |
| `morpho supply --vault ADDR --asset SYMBOL --amount N [--confirm]` | Supply assets to MetaMorpho vault |
| `morpho withdraw --vault ADDR --asset SYMBOL --amount N\|--all [--confirm]` | Withdraw from MetaMorpho vault |
| `morpho borrow --market-id HEX --amount N [--confirm]` | Borrow from Morpho Blue market |
| `morpho repay --market-id HEX --amount N\|--all [--confirm]` | Repay Morpho Blue debt |
| `morpho supply-collateral --market-id HEX --amount N [--confirm]` | Supply collateral to Morpho Blue market |
| `morpho withdraw-collateral --market-id HEX --amount N\|--all [--confirm]` | Withdraw collateral from Morpho Blue market |
| `morpho claim-rewards [--confirm]` | Claim Merkl rewards |

## Triggers
Activate this skill when users mention "supply to morpho", "deposit to morpho vault", "borrow from morpho", "repay morpho loan", "morpho health factor", "my morpho positions", "morpho interest rates", "claim morpho rewards", "morpho markets", or "metamorpho vaults". Also trigger for DeFi lending/borrowing needs on Ethereum or Base networks.
