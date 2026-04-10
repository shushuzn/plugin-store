
# pancakeswap-v2 -- Skill Summary

## Overview
This skill enables interaction with PancakeSwap V2's constant-product automated market maker (AMM) across three major chains: BSC, Base, and Arbitrum One. It provides comprehensive functionality for token swapping, liquidity provision, and pool management using the traditional xyk formula with 0.25% swap fees. The skill handles both read operations (quotes, reserves, balances) and write operations (swaps, liquidity management) with built-in safety measures and transaction confirmation workflows.

## Usage
Use this skill for PancakeSwap V2 operations by specifying the desired action, tokens, and amounts. All write operations require user confirmation and support dry-run previews before execution.

## Commands
| Command | Description |
|---------|-------------|
| `quote` | Get expected swap output amounts |
| `swap` | Execute token swaps with slippage protection |
| `add-liquidity` | Provide liquidity to earn LP tokens |
| `remove-liquidity` | Withdraw liquidity and burn LP tokens |
| `get-pair` | Look up pair contract addresses |
| `get-reserves` | Check current pool reserves |
| `lp-balance` | View LP token balances |

## Triggers
Activate this skill when users want to swap tokens on PancakeSwap V2, manage liquidity positions, check prices or pool data, or interact with xyk AMM pools on BSC, Base, or Arbitrum networks. Use for V2 operations only, not V3 or concentrated liquidity.
