
# hyperliquid -- Skill Summary

## Overview
This skill provides comprehensive trading functionality for the Hyperliquid perpetual derivatives exchange built on Hyperliquid L1. It enables users to manage positions, place orders with advanced bracket strategies, monitor real-time prices, and seamlessly bridge USDC from Arbitrum. The plugin handles both read-only operations (positions, prices) via REST API and write operations (trading, deposits) through EIP-712 signing with the onchainos wallet system.

## Usage
Run `hyperliquid register` once to set up your signing address, then use commands like `hyperliquid order --coin BTC --side buy --size 0.01 --confirm` to trade. All write operations require `--confirm` flag to execute after previewing.

## Commands
| Command | Description |
|---------|-------------|
| `positions` | Check open perpetual positions and account summary |
| `prices` | Get current market mid prices for all or specific coins |
| `order` | Place market/limit orders with optional TP/SL brackets |
| `close` | Market-close an open position |
| `tpsl` | Set stop-loss and/or take-profit on existing positions |
| `cancel` | Cancel open orders by order ID |
| `deposit` | Deposit USDC from Arbitrum to Hyperliquid |
| `register` | One-time setup to detect signing address |

## Triggers
Activate when users mention trading perpetuals, managing positions on Hyperliquid, placing orders with stop-loss/take-profit, or bridging USDC from Arbitrum. Also triggered by phrases like "HL order", "Hyperliquid perps", or specific trading actions like "long BTC on Hyperliquid".
