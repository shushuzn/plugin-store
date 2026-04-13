
# polymarket-plugin

Trade prediction markets on Polymarket — buy and sell YES/NO outcome tokens on real-world events using USDC.e on Polygon.

## Highlights
- Buy and sell outcome tokens on Polymarket prediction markets
- Support for both binary (YES/NO) and categorical markets
- Two trading modes: direct EOA trading or gasless proxy wallet trading
- Check positions and PnL across all markets
- Cancel open orders and redeem winning tokens
- Built-in region restriction checking for compliance
- Automatic API credential derivation from wallet signatures
- On-chain settlement with USDC.e on Polygon mainnet

---SEPARATOR---

# polymarket-plugin -- Skill Summary

## Overview

This skill enables AI agents to interact with Polymarket prediction markets on Polygon. It provides comprehensive trading functionality including buying and selling outcome tokens, managing positions, placing limit orders, and redeeming winnings. The plugin supports both direct EOA trading (requiring gas for approvals) and gasless proxy wallet trading. All operations use USDC.e as the base currency and integrate with the onchainos wallet system for secure transaction signing.

## Usage

Install the plugin, connect an onchainos wallet with `onchainos wallet login`, verify region access with `polymarket check-access`, fund your wallet with USDC.e on Polygon, then start trading with commands like `polymarket buy --market-id <slug> --outcome yes --amount 50`.

## Commands

| Command | Description |
|---------|-------------|
| `check-access` | Verify region is not restricted |
| `list-markets` | Browse active prediction markets |
| `get-market` | Get market details and order book |
| `get-positions` | View open positions and PnL |
| `balance` | Show POL and USDC.e balances |
| `buy` | Buy outcome shares with USDC.e |
| `sell` | Sell outcome shares for USDC.e |
| `cancel` | Cancel open orders |
| `redeem` | Redeem winning tokens after resolution |
| `setup-proxy` | Deploy gasless trading proxy wallet |
| `deposit` | Transfer USDC.e to proxy wallet |
| `switch-mode` | Change trading mode (EOA/proxy) |

## Triggers

Activate this skill when users want to trade prediction markets, buy YES/NO tokens, check Polymarket positions, place bets on real-world events, or manage orders on Polymarket. Also trigger for setup questions like "how do I use Polymarket" or "I just installed polymarket".
