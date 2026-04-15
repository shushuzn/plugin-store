
# meteora-plugin -- Skill Summary

## Overview
The meteora-plugin provides comprehensive access to Meteora's Dynamic Liquidity Market Maker (DLMM) protocol on Solana. It enables users to search liquidity pools, get swap quotes, manage LP positions, execute swaps, and add/remove liquidity. The plugin combines direct API calls for read operations with on-chain transaction execution via onchainos integration, supporting both preview modes and confirmed transactions with proper user consent flows.

## Usage
Install the plugin and run `meteora <command>` to interact with Meteora DLMM pools. Use `--dry-run` flags to preview operations before execution, and the plugin will prompt for user confirmation on all write operations.

## Commands
| Command | Description |
|---------|-------------|
| `meteora get-pools` | Search and list DLMM pools with filtering options |
| `meteora get-pool-detail --address <addr>` | Get detailed information for a specific pool |
| `meteora get-swap-quote --from-token <mint> --to-token <mint> --amount <amt>` | Get swap price quotes |
| `meteora get-user-positions` | View user's LP positions with token balances |
| `meteora swap --from-token <mint> --to-token <mint> --amount <amt>` | Execute token swaps |
| `meteora add-liquidity --pool <addr> --amount-x <amt> --amount-y <amt>` | Add liquidity to pools |
| `meteora remove-liquidity --pool <addr> --position <addr>` | Remove liquidity from positions |
| `meteora quickstart --pool <addr>` | Get wallet-based liquidity recommendations |

## Triggers
An AI agent should activate this skill when users want to interact with Meteora DLMM on Solana, including searching for liquidity pools, checking LP positions, getting swap quotes, or performing DeFi operations like swapping tokens or managing liquidity positions.
