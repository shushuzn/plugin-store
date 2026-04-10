
# curve -- Skill Summary

## Overview
The Curve plugin enables interaction with Curve Finance DEX for stablecoin swaps and liquidity management. It provides read-only operations for querying pools, APYs, and balances, plus write operations for swapping tokens and managing liquidity positions. All operations support multiple chains and include safety features like dry-run previews and automatic allowance management.

## Usage
Install the curve binary automatically on first use, then use commands like `curve get-pools` to browse available pools, `curve quote` for swap estimates, and `curve swap` to execute trades. All write operations require user confirmation after a dry-run preview.

## Commands
| Command | Description |
|---------|-------------|
| `get-pools` | List Curve pools with TVL and APY data |
| `get-pool-info` | Get detailed information for a specific pool |
| `get-balances` | Check LP token balances for a wallet |
| `quote` | Get swap quote with slippage calculations |
| `swap` | Execute token swap on Curve |
| `add-liquidity` | Add liquidity to a Curve pool |
| `remove-liquidity` | Remove liquidity from a Curve pool |

## Triggers
Activate this skill when users mention Curve-specific operations like "swap on Curve", "Curve pool APY", "add liquidity Curve", or need stablecoin swaps with low slippage. Use for Curve Finance interactions only, not other DEXs.
