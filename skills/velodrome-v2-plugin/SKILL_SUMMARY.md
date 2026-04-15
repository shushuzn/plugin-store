
# velodrome-v2-plugin -- Skill Summary

## Overview
This plugin enables AI agents to interact with Velodrome V2, the largest decentralized exchange on Optimism, supporting token swaps and liquidity provision in classic AMM pools. It handles both volatile pools (using constant product formula for assets like WETH/USDC) and stable pools (using low-slippage curves for correlated assets like USDC/DAI), while providing comprehensive liquidity management capabilities including adding/removing positions and claiming VELO token rewards from gauges.

## Usage
Install the plugin and run commands like `velodrome-v2 quote --token-in WETH --token-out USDC --amount-in 0.1` for price quotes or `velodrome-v2 swap --token-in WETH --token-out USDC --amount-in 0.1 --confirm` for executing swaps. All write operations require the `--confirm` flag after reviewing transaction details.

## Commands
| Command | Purpose |
|---------|---------|
| `quote` | Get swap quotes between tokens without executing |
| `swap` | Execute token swaps with slippage protection |
| `pools` | Query pool information, reserves, and addresses |
| `positions` | View LP token balances and underlying assets |
| `add-liquidity` | Add liquidity to earn trading fees |
| `remove-liquidity` | Remove liquidity and withdraw tokens |
| `claim-rewards` | Claim VELO emissions from gauge rewards |

## Triggers
An AI agent should activate this skill when users want to trade tokens on Optimism, provide liquidity to earn yield, or manage existing positions on Velodrome V2. It's particularly useful for accessing the deepest liquidity pools on Optimism and maximizing trading efficiency.
