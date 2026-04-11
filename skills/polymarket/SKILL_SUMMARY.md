
# polymarket -- Skill Summary

## Overview
This skill enables trading on Polymarket prediction markets where users buy and sell outcome tokens representing real-world event probabilities. Markets can be binary (YES/NO) or categorical (multiple outcomes like election candidates), with each outcome token resolving to $1.00 (winner) or $0.00 (loser) based on event results. The plugin integrates with onchainos wallet for Polygon-based trading, automatically handling API credential derivation, EIP-712 order signing, and USDC.e/CTF token approvals.

## Usage
Install the polymarket binary, connect your onchainos wallet to Polygon (chain 137), and ensure sufficient USDC.e balance for buying. Use `polymarket list-markets` to browse available prediction markets, then execute trades with `polymarket buy` or `polymarket sell` commands.

## Commands
| Command | Description |
|---------|-------------|
| `list-markets` | Browse active prediction markets with optional keyword filtering |
| `get-market` | Get detailed market info and order book by condition ID or slug |
| `get-positions` | View open positions with P&L for wallet address |
| `buy` | Buy outcome shares with USDC.e (limit or market orders) |
| `sell` | Sell outcome shares for USDC.e (limit or market orders) |
| `cancel` | Cancel specific orders, all orders for a market, or all open orders |

## Triggers
Activate this skill when users want to trade prediction markets, check Polymarket positions, browse event markets, or place/cancel orders on outcome tokens. Also trigger for phrases like "buy polymarket shares," "sell prediction market position," or "check my polymarket positions."
