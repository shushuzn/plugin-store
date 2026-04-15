
# pancakeswap-v3-plugin -- Skill Summary

## Overview
This skill provides comprehensive PancakeSwap V3 functionality for token swapping and concentrated liquidity management across five major blockchain networks. It enables users to execute optimal token swaps through the SmartRouter, manage concentrated liquidity positions with customizable price ranges, view pool information and LP positions, and perform all operations with built-in slippage protection and transaction safety checks.

## Usage
Install the plugin using the auto-injected setup commands, ensure your wallet is connected via `onchainos wallet login`, then use commands like `pancakeswap-v3 swap`, `pancakeswap-v3 add-liquidity`, or `pancakeswap-v3 quote`. All write operations require explicit `--confirm` flag for execution.

## Commands
| Command | Description |
|---------|-------------|
| `quote` | Get swap quotes without executing transactions |
| `swap` | Execute token swaps via SmartRouter |
| `pools` | List all pools for a token pair across fee tiers |
| `positions` | View LP positions for a wallet address |
| `add-liquidity` | Mint new concentrated liquidity positions |
| `remove-liquidity` | Remove liquidity and collect tokens from positions |

## Triggers
Activate this skill when users mention PancakeSwap operations like token swapping, liquidity management, or pool queries, especially when they reference "pancakeswap", "swap on pancake", "add liquidity", or "PancakeSwap V3" on supported networks.
