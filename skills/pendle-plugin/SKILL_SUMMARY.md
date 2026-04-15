
# pendle-plugin -- Skill Summary

## Overview
The pendle-plugin enables AI agents to interact with Pendle Finance's yield tokenization protocol, allowing users to trade fixed-yield Principal Tokens (PT), speculate on Yield Tokens (YT), provide liquidity to AMM pools, and mint/redeem PT+YT pairs from underlying assets. The plugin supports operations across Ethereum, Arbitrum, BSC, and Base networks with comprehensive market data access and position management.

## Usage
Install the plugin through the auto-injected dependencies, then use commands like `pendle list-markets` to browse available pools or `pendle buy-pt` to purchase fixed-yield tokens. All write operations require a dry-run preview and user confirmation before execution.

## Commands
| Command | Purpose |
|---------|---------|
| `list-markets` | Browse available Pendle markets and pools |
| `get-market` | Get detailed market information and APY history |
| `get-positions` | View current Pendle token holdings and positions |
| `get-asset-price` | Check prices for PT, YT, LP, or SY tokens |
| `buy-pt` | Purchase Principal Tokens for fixed yield exposure |
| `sell-pt` | Sell Principal Tokens back to underlying assets |
| `buy-yt` | Buy Yield Tokens for floating yield speculation |
| `sell-yt` | Sell Yield Tokens back to underlying assets |
| `add-liquidity` | Provide single-token liquidity to Pendle AMM |
| `remove-liquidity` | Withdraw liquidity from Pendle AMM pools |
| `mint-py` | Mint PT+YT pairs from underlying tokens |
| `redeem-py` | Redeem PT+YT pairs back to underlying assets |

## Triggers
Activate this skill when users mention Pendle-related operations like "buy PT", "sell YT", "Pendle fixed yield", "add Pendle liquidity", "mint PT YT", "Pendle positions", or "Pendle markets". The skill should also trigger for yield trading, fixed yield strategies, and liquidity provision discussions related to Pendle Finance.
