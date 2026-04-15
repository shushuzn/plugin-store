
# meteora-plugin -- Skill Summary

## Overview
The meteora-plugin provides comprehensive access to Meteora's Dynamic Liquidity Market Maker (DLMM) protocol on Solana, enabling users to search pools, execute swaps, manage liquidity positions, and perform DeFi operations. It combines read operations via REST API calls with on-chain transaction execution through onchainos integration, supporting both dry-run previews and live transactions with automatic risk warnings.

## Usage
Install via the auto-injected setup commands, then use `meteora <command>` to interact with pools and positions. Always run commands with `--dry-run` first to preview transactions before execution.

## Commands
| Command | Description |
|---------|-------------|
| `meteora get-pools` | Search and list DLMM pools with filtering and sorting |
| `meteora get-pool-detail --address <pool>` | Get detailed pool information |
| `meteora get-swap-quote --from-token <mint> --to-token <mint> --amount <amt>` | Get swap quote |
| `meteora swap --from-token <mint> --to-token <mint> --amount <amt>` | Execute token swap |
| `meteora get-user-positions` | View LP positions for wallet |
| `meteora add-liquidity --pool <addr> --amount-x <amt> --amount-y <amt>` | Add liquidity to pool |
| `meteora remove-liquidity --pool <addr> --position <addr>` | Remove liquidity from position |
| `meteora quickstart --pool <addr>` | Get wallet balance check and deposit recommendation |

## Triggers
Activate this skill when users want to interact with Meteora DLMM pools on Solana, including swapping tokens, providing/removing liquidity, checking pool performance, or managing LP positions. Use for DeFi yield farming and liquidity provision strategies.
