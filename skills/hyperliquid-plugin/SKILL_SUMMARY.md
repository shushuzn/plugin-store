
# hyperliquid-plugin -- Skill Summary

## Overview
This plugin enables AI agents to trade perpetual futures on Hyperliquid, a high-performance on-chain DEX built on its own L1 blockchain. It provides comprehensive trading functionality including order placement with optional TP/SL brackets, position monitoring, fund management between Arbitrum and Hyperliquid, and real-time market data access. All trades are settled in USDC with leverage up to 100x in either cross or isolated margin modes.

## Usage
Run `hyperliquid register` once to set up your signing address, then use `hyperliquid quickstart` to check your asset status and get guided next steps. All trading commands require `--confirm` to execute after previewing.

## Commands
| Command | Description |
|---------|-------------|
| `quickstart` | Check assets across Arbitrum/Hyperliquid and get guided next step |
| `positions [--show-orders]` | View open perpetual positions and PnL |
| `prices [--coin SYMBOL]` | Get current market prices |
| `order --coin --side --size [--type] [--price] [--leverage] [--sl-px] [--tp-px] [--confirm]` | Place perpetual order with optional TP/SL bracket |
| `close --coin [--size] [--confirm]` | Market-close an open position |
| `tpsl --coin [--sl-px] [--tp-px] [--size] [--confirm]` | Set stop-loss/take-profit on existing position |
| `cancel --coin [--confirm]` | Cancel open orders |
| `deposit --amount [--confirm]` | Deposit USDC from Arbitrum |
| `withdraw --amount [--confirm]` | Withdraw USDC to Arbitrum |
| `transfer --amount --from --to [--confirm]` | Transfer between perp/spot accounts |

## Triggers
Activate this skill when users mention trading on Hyperliquid, placing perpetual orders, checking HL positions, setting stop-loss/take-profit levels, or managing funds between Arbitrum and Hyperliquid. Also trigger on phrases like "HL long/short", "Hyperliquid perps", or "trade derivatives".
