
# curve -- Skill Summary

## Overview
The Curve plugin provides comprehensive access to Curve Finance DEX operations across Ethereum, Arbitrum, Base, Polygon, and BSC. It enables stablecoin swaps, liquidity provision, pool queries, and balance management through direct integration with Curve's smart contracts and API. All write operations require user confirmation and support dry-run previews for safety.

## Usage
Install the plugin and use natural language like "swap 1000 USDC for DAI on Curve" or "show Curve pools on Ethereum". Write operations will prompt for confirmation after showing a dry-run preview.

## Commands
| Command | Description |
|---------|-------------|
| `get-pools` | List available Curve pools with TVL and APY data |
| `get-pool-info` | Get detailed information about a specific pool |
| `get-balances` | Show LP token balances for a wallet |
| `quote` | Get swap quote between two tokens |
| `swap` | Execute token swap on Curve |
| `add-liquidity` | Add liquidity to a Curve pool |
| `remove-liquidity` | Remove liquidity from a Curve pool |

## Triggers
Activate when users want to swap stablecoins, manage Curve liquidity positions, or query Curve pool data. Trigger phrases include "Curve swap", "add liquidity Curve", "Curve pool APY", and "remove Curve LP".
