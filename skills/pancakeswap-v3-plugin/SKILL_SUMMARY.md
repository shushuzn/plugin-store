
# pancakeswap-v3-plugin -- Skill Summary

## Overview
The pancakeswap-v3-plugin enables AI agents to interact with PancakeSwap V3, the leading concentrated liquidity DEX, across multiple chains including BNB Chain, Base, Arbitrum, Ethereum, and Linea. It provides comprehensive functionality for token swaps via SmartRouter, liquidity position management through the NonfungiblePositionManager, and portfolio assessment tools, with built-in safety features like balance verification, slippage protection, and transaction confirmation requirements.

## Usage
Install the plugin binary and connect your wallet via `onchainos wallet login`. Use commands with `--dry-run` to preview transactions, then add `--confirm` to execute on-chain operations.

## Commands
| Command | Description |
|---------|-------------|
| `quote` | Get swap quote without executing transaction |
| `swap` | Execute token swap via SmartRouter (requires --confirm) |
| `pools` | List all pools for a token pair across fee tiers |
| `positions` | View LP positions for a wallet address |
| `add-liquidity` | Mint new concentrated liquidity position (requires --confirm) |
| `remove-liquidity` | Remove liquidity and collect tokens (requires --confirm) |
| `quickstart` | Check wallet status and get onboarding guidance |

## Triggers
Activate this skill when users mention "pancakeswap", "swap on pancake", "PCS", "add liquidity pancakeswap", "pancakeswap pool", or "PancakeSwap V3". Also trigger for concentrated liquidity operations and multi-chain DEX activities on supported networks.
