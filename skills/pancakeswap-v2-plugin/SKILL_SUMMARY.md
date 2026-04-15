
# pancakeswap-v2-plugin -- Skill Summary

## Overview
This plugin enables interaction with PancakeSwap V2's constant-product (xyk) automated market maker across three major chains. It provides comprehensive DeFi functionality including token swaps, liquidity provision/removal, price quotes, and pool analytics. All write operations use exact-amount ERC-20 approvals and support dry-run simulation for safe transaction preview.

## Usage
Install via the auto-injected setup commands, then use commands like `pancakeswap-v2 --chain 56 swap --token-in USDT --token-out CAKE --amount-in 100` for trading. Always run with `--dry-run` flag first to preview transactions before execution.

## Commands
| Command | Purpose |
|---------|---------|
| `quote` | Get expected swap output amounts |
| `swap` | Execute token swaps |
| `add-liquidity` | Provide liquidity to pools |
| `remove-liquidity` | Withdraw liquidity from pools |
| `get-pair` | Find pair contract address |
| `get-reserves` | Check pool reserves and pricing |
| `lp-balance` | View LP token balances |

## Triggers
Activate when users mention PancakeSwap V2 operations like "swap on pancakeswap v2", "add liquidity pancake", "pcs v2 quote", or "check pancake pair". Do not use for PancakeSwap V3 or concentrated liquidity operations.
