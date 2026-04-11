
# curve -- Skill Summary

## Overview
The Curve plugin provides comprehensive access to Curve Finance, the leading stablecoin DEX. It enables users to swap stablecoins with minimal slippage, add/remove liquidity from pools, query APY rates, and manage LP positions across Ethereum, Arbitrum, Base, Polygon, and BSC. The plugin handles token approvals automatically and supports both read-only queries and on-chain transactions with user confirmation.

## Usage
Install the plugin and use natural language commands like "swap 1000 USDC for DAI on Curve" or "show Curve pools on Ethereum". All write operations require user confirmation after a dry-run preview.

## Commands
| Command | Purpose |
|---------|---------|
| `get-pools` | List Curve pools with TVL and APY data |
| `get-pool-info` | Get detailed information for a specific pool |
| `get-balances` | Check LP token balances across all positions |
| `quote` | Get swap quotes with price impact |
| `swap` | Execute stablecoin swaps |
| `add-liquidity` | Deposit tokens to earn LP tokens |
| `remove-liquidity` | Withdraw from pools (proportional or single-coin) |

## Triggers
Activate this skill when users mention Curve-specific operations like "swap on Curve", "Curve pool APY", "add liquidity to Curve", or when they want to perform stablecoin swaps with low slippage. Use for Curve Finance operations only, not other DEXes.
