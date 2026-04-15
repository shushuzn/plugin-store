
# meteora-plugin -- Skill Summary

## Overview
The meteora-plugin provides comprehensive access to Meteora's Dynamic Liquidity Market Maker (DLMM) protocol on Solana. It enables users to search liquidity pools, obtain swap quotes, manage LP positions, execute token swaps, and add/remove liquidity with advanced features like customizable bin ranges and automatic position management. The plugin integrates with onchainos for secure transaction execution and provides detailed on-chain position analysis.

## Usage
Install the plugin using the auto-injected dependency commands, then use `meteora <command>` to interact with Meteora pools. Most operations support `--dry-run` for previewing before execution, and write operations require user confirmation before proceeding.

## Commands
| Command | Description |
|---------|-------------|
| `meteora get-pools` | Search and list DLMM pools with filtering and sorting options |
| `meteora get-pool-detail --address <pool>` | Get detailed information about a specific pool |
| `meteora get-swap-quote --from-token <mint> --to-token <mint> --amount <amount>` | Get swap quote between tokens |
| `meteora get-user-positions [--wallet <addr>] [--pool <pool>]` | View LP positions with computed token amounts |
| `meteora swap --from-token <mint> --to-token <mint> --amount <amount> [--slippage <pct>] [--dry-run]` | Execute token swaps with confirmation |
| `meteora add-liquidity --pool <pool> [--amount-x <float>] [--amount-y <float>] [--bin-range <n>] [--dry-run]` | Add liquidity to DLMM pools |
| `meteora remove-liquidity --pool <pool> --position <position> [--pct <1-100>] [--close] [--dry-run]` | Remove liquidity from positions |
| `meteora quickstart --pool <pool>` | Check wallet balances and get recommended deposit commands |

## Triggers
An AI agent should activate this skill when users want to interact with Meteora DLMM pools on Solana, including swapping tokens, providing liquidity, checking positions, or analyzing pool performance. Use this skill for any DeFi operations specifically related to Meteora's liquidity protocol.
