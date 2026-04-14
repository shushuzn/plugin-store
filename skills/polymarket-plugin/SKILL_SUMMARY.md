
# polymarket-plugin -- Skill Summary

## Overview
This skill enables AI agents to trade prediction markets on Polymarket, a decentralized platform on Polygon where users can buy and sell outcome tokens representing real-world events. The plugin supports browsing markets, placing buy/sell orders, managing positions, and redeeming winning tokens. It offers two trading modes: direct EOA trading (requires POL gas for approvals) or proxy wallet trading (gasless after one-time setup). Markets include binary YES/NO events and categorical outcomes, with prices representing implied probabilities.

## Usage
Connect an onchainos wallet with Polygon support, verify region access with `polymarket-plugin check-access`, fund with USDC.e, then browse markets and trade using simple commands like `buy --market-id <slug> --outcome yes --amount 10`.

## Commands
| Command | Description |
|---------|-------------|
| `check-access` | Verify region is not restricted |
| `list-markets` | Browse active prediction markets with optional filters |
| `list-5m` | List upcoming 5-minute crypto up/down markets |
| `get-market` | Get detailed market information and order book |
| `get-positions` | View current trading positions |
| `balance` | Show POL and USDC.e balances for EOA and proxy wallets |
| `buy` | Purchase YES/NO outcome shares |
| `sell` | Sell outcome shares from positions |
| `cancel` | Cancel open orders |
| `redeem` | Redeem winning tokens after market resolution |
| `setup-proxy` | Deploy proxy wallet for gasless trading |
| `deposit` | Transfer USDC.e from EOA to proxy wallet |
| `switch-mode` | Switch between EOA and proxy trading modes |

## Triggers
Activate when users want to trade prediction markets, bet on outcomes, check Polymarket positions, or ask about specific events like elections, sports, or crypto price targets. Also trigger for onboarding phrases like "new to polymarket" or "how do I use polymarket" to provide guided setup.
