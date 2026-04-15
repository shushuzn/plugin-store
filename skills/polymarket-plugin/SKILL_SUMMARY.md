
# polymarket-plugin

Trade prediction markets on Polymarket - buy and sell YES/NO outcome tokens on real-world events using Polygon-based trading infrastructure.

## Highlights
- Trade binary YES/NO and categorical prediction markets
- Support for 5-minute crypto price direction markets (BTC, ETH, SOL, etc.)
- Two trading modes: direct EOA trading or gasless proxy wallet trading
- Real-time market browsing with keyword filtering and category search
- Position tracking and order management capabilities
- Automatic winning token redemption after market resolution
- Region-based access verification for compliance
- Seamless integration with onchainos wallet infrastructure

---

# polymarket-plugin -- Skill Summary

## Overview

This plugin enables AI agents to interact with Polymarket, a prediction market platform on Polygon where users trade outcome tokens representing real-world event probabilities. It supports both binary (YES/NO) and categorical markets across topics like politics, sports, crypto prices, and breaking news. The plugin provides two trading modes: direct EOA trading (requiring POL gas per transaction) and proxy wallet trading (gasless after one-time setup), with automatic credential derivation and EIP-712 order signing via the onchainos wallet infrastructure.

## Usage

First verify region access with `polymarket-plugin check-access`, then connect an onchainos wallet on Polygon and fund it with USDC.e. Browse markets using `list-markets` or `list-5m` for crypto price direction bets, then execute trades with `buy` and `sell` commands.

## Commands

| Command | Description |
|---------|-------------|
| `check-access` | Verify region is not restricted |
| `list-markets` | Browse prediction markets with filtering options |
| `list-5m` | List 5-minute crypto up/down markets |
| `get-market` | Get detailed market info and order book |
| `get-positions` | View open positions and P&L |
| `balance` | Show POL and USDC.e balances |
| `buy` | Purchase outcome shares |
| `sell` | Sell outcome shares |
| `cancel` | Cancel open orders |
| `redeem` | Redeem winning tokens |
| `setup-proxy` | Deploy gasless trading proxy wallet |
| `deposit` | Transfer funds to proxy wallet |
| `switch-mode` | Change trading mode (EOA/proxy) |

## Triggers

Activate when users want to trade prediction markets, bet on real-world events, explore crypto price direction markets, or check positions on Polymarket. Common trigger phrases include "bet on," "prediction market," "polymarket," "5-minute market," or requests about political elections, sports outcomes, and crypto price targets.
