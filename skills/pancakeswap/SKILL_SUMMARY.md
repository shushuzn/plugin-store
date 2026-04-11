
# pancakeswap -- Skill Summary

## Overview
This skill enables token swapping and concentrated liquidity management on PancakeSwap V3, the leading decentralized exchange on BNB Chain, Base, and Arbitrum. It provides comprehensive functionality for trading tokens, providing liquidity, and managing LP positions with built-in safety checks, slippage protection, and multi-chain support across three major networks.

## Usage
Install the pancakeswap binary via the plugin system, ensure your wallet is connected with `onchainos wallet login`, then use commands like `pancakeswap quote`, `pancakeswap swap`, or `pancakeswap add-liquidity` to interact with PancakeSwap V3 pools.

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
Activate this skill when users mention "pancakeswap", "swap on pancake", "PCS swap", liquidity management on PancakeSwap, or trading on BNB Chain, Base, or Arbitrum DEXes. Use for PancakeSwap V3 concentrated liquidity operations only.
