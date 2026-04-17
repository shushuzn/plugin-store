
# hyperliquid-plugin

Trade perpetuals on Hyperliquid DEX with full position management, spot trading, and cross-chain USDC deposits.

## Highlights
- Place market/limit perpetual orders with optional stop-loss and take-profit brackets
- Check open positions, unrealized PnL, and margin usage across perp and spot accounts
- One-command position closing and TP/SL management on existing positions
- Real-time price feeds for all Hyperliquid perpetual and spot markets
- Cross-chain USDC deposits from Arbitrum with automatic bridge integration
- Transfer funds between perpetual and spot accounts on Hyperliquid L1
- Withdraw USDC back to Arbitrum or other supported chains
- Automatic leverage adjustment and margin mode selection (cross/isolated)

---SEPARATOR---

# hyperliquid-plugin -- Skill Summary

## Overview

This plugin provides comprehensive trading capabilities for Hyperliquid, a high-performance on-chain perpetuals exchange built on its own L1 blockchain. It enables users to trade perpetual contracts with CEX-like speed while maintaining full on-chain settlement, manage positions with advanced order types including stop-loss and take-profit brackets, transfer funds between perpetual and spot accounts, and deposit/withdraw USDC cross-chain from Arbitrum. All operations support both read-only queries and write transactions with EIP-712 signing.

## Usage

Run the one-time setup with `hyperliquid register` to configure your signing address, then use `hyperliquid quickstart` to check your account status and get guided next steps. All write operations require the `--confirm` flag for execution.

## Commands

| Command | Description |
|---------|-------------|
| `hyperliquid quickstart` | Check wallet assets across Arbitrum/Hyperliquid and get guided next steps |
| `hyperliquid register` | One-time setup to register your signing address for trading |
| `hyperliquid positions [--show-orders]` | View open perpetual positions and account summary |
| `hyperliquid prices [--coin SYMBOL]` | Get current market prices for all or specific coins |
| `hyperliquid order --coin SYMBOL --side buy/sell --size AMOUNT [--type limit] [--price PRICE] [--sl-px PRICE] [--tp-px PRICE] [--leverage N] [--isolated] [--confirm]` | Place perpetual orders with optional TP/SL brackets |
| `hyperliquid close --coin SYMBOL [--size AMOUNT] [--confirm]` | Market-close open positions |
| `hyperliquid tpsl --coin SYMBOL [--sl-px PRICE] [--tp-px PRICE] [--size AMOUNT] [--confirm]` | Set stop-loss/take-profit on existing positions |
| `hyperliquid orders [--coin SYMBOL]` | List open orders |
| `hyperliquid cancel --coin SYMBOL --oid ORDER_ID [--confirm]` | Cancel specific orders |
| `hyperliquid deposit --amount USDC [--confirm]` | Deposit USDC from Arbitrum to Hyperliquid |
| `hyperliquid withdraw --amount USDC [--confirm]` | Withdraw USDC from Hyperliquid to Arbitrum |
| `hyperliquid transfer --amount USDC --from perp/spot --to spot/perp [--confirm]` | Transfer between perp and spot accounts |
| `hyperliquid spot-balances` | View spot account balances |
| `hyperliquid spot-prices [--coin SYMBOL]` | Get spot market prices |
| `hyperliquid spot-order --coin SYMBOL --side buy/sell --size AMOUNT [--type limit] [--price PRICE] [--confirm]` | Place spot orders |
| `hyperliquid address` | Show your wallet address for deposits |

## Triggers

Activate this skill when users mention trading perpetuals, checking positions, placing orders, or managing funds on Hyperliquid, including phrases like "trade on Hyperliquid", "HL long/short", "check my Hyperliquid positions", or "Hyperliquid prices".
