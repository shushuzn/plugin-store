
# polymarket-plugin -- Skill Summary

## Overview
This plugin enables AI agents to trade prediction markets on Polymarket, a decentralized platform on Polygon where users can buy and sell outcome tokens for real-world events. The plugin supports both traditional prediction markets (elections, sports, crypto price targets) and short-term 5-minute crypto Up/Down markets. It provides two trading modes: EOA mode (direct wallet trading requiring gas for approvals) and POLY_PROXY mode (gasless trading via a deployed proxy contract).

## Usage
First verify region access with `polymarket-plugin check-access`, then connect your onchainos wallet and choose between EOA or POLY_PROXY trading mode. Fund your wallet with USDC.e on Polygon and start trading with `buy` and `sell` commands.

## Commands
| Command | Description |
|---------|-------------|
| `check-access` | Verify region is not restricted |
| `list-markets` | Browse active prediction markets with filtering options |
| `list-5m` | List 5-minute crypto Up/Down markets |
| `get-market` | Get detailed market info and order book |
| `get-positions` | View current open positions |
| `balance` | Show POL and USDC.e balances |
| `buy` | Purchase YES/NO outcome shares |
| `sell` | Sell outcome shares |
| `cancel` | Cancel open orders |
| `redeem` | Redeem winning tokens after market resolves |
| `setup-proxy` | Deploy proxy wallet for gasless trading |
| `deposit` | Transfer USDC.e to proxy wallet |
| `switch-mode` | Switch between EOA and proxy trading modes |

## Triggers
Activate when users want to trade prediction markets, check market prices, place bets on events, or use phrases like "buy polymarket shares," "prediction market trade," "bet on," or mention 5-minute crypto markets. Also triggers on onboarding phrases like "new to polymarket" or "polymarket setup."
