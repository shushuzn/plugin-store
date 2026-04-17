
# hyperliquid-plugin -- Skill Summary

## Overview
This skill enables AI agents to trade perpetual futures and spot assets on Hyperliquid, a high-performance on-chain DEX built on its own L1 blockchain. It provides comprehensive trading functionality including order placement with bracket orders (stop-loss/take-profit), position monitoring, fund management across Arbitrum and Hyperliquid chains, and automated wallet setup. All operations use proper EIP-712 signing for security and support both preview and execution modes.

## Usage
First run `hyperliquid register` to set up your signing address, then use `hyperliquid quickstart` to check your balance status and get guided next steps. All trading commands require `--confirm` flag to execute after previewing.

## Commands
| Command | Description |
|---------|-------------|
| `hyperliquid quickstart` | Check wallet status across chains and get recommended next action |
| `hyperliquid register` | Set up signing address for trading operations |
| `hyperliquid positions [--address] [--show-orders]` | View open perpetual positions and PnL |
| `hyperliquid prices [--coin]` | Get current market prices for all or specific coins |
| `hyperliquid order --coin --side --size [--type] [--price] [--leverage] [--sl-px] [--tp-px] [--confirm]` | Place perpetual orders with optional bracket orders |
| `hyperliquid close --coin [--size] [--confirm]` | Market close existing positions |
| `hyperliquid tpsl --coin [--sl-px] [--tp-px] [--size] [--confirm]` | Set stop-loss/take-profit on existing positions |

## Triggers
An AI agent should activate this skill when users mention trading perpetuals, checking positions, or managing funds on Hyperliquid, including phrases like "trade on Hyperliquid", "HL long/short", "check my HL positions", "Hyperliquid prices", or "withdraw from Hyperliquid". Also triggers for setup-related requests like "register Hyperliquid" or general trading intent with Hyperliquid-specific terminology.
