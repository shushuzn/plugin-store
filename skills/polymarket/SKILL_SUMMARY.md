
# polymarket -- Skill Summary

## Overview
This skill enables trading on Polymarket prediction markets on Polygon, where users buy and sell outcome tokens (YES/NO or categorical) that resolve to $1.00 for winners and $0.00 for losers. Prices represent implied probabilities of real-world events. The plugin handles order signing via EIP-712, automatic USDC.e approvals, and credential derivation from the onchainos wallet, with gasless settlement when orders match.

## Usage
Install the polymarket plugin and ensure onchainos CLI is available for trading operations. Read-only commands (browsing markets, checking positions) work immediately, while buy/sell/cancel commands require a connected Polygon wallet with USDC.e balance.

## Commands
| Command | Description |
|---------|-------------|
| `polymarket list-markets` | Browse active prediction markets with optional keyword filtering |
| `polymarket get-market --market-id <id>` | Get detailed market info and order book data |
| `polymarket get-positions` | View open positions with PnL tracking |
| `polymarket buy --market-id <id> --outcome <outcome> --amount <usdc>` | Buy outcome shares with USDC.e |
| `polymarket sell --market-id <id> --outcome <outcome> --shares <amount>` | Sell outcome shares |
| `polymarket cancel --order-id <id>` | Cancel specific order, market orders, or all orders |

## Triggers
Activate this skill when users want to trade prediction markets, check Polymarket positions, browse betting odds on real-world events, or manage existing Polymarket orders and positions.
