
# polymarket -- Skill Summary

## Overview
This skill enables AI agents to interact with Polymarket prediction markets on Polygon, allowing users to trade YES/NO outcome tokens on real-world events. It provides comprehensive market discovery, position management, and trading capabilities with built-in safety features including liquidity checks, slippage protection, and automatic credential management through onchainos wallet integration.

## Usage
Install the polymarket binary and ensure onchainos CLI is available for trading operations. Read-only commands (browsing markets, checking positions) work immediately, while trading requires an active onchainos wallet connected to Polygon with USDC.e balance.

## Commands
- `polymarket list-markets [--limit N] [--keyword text]` - Browse active prediction markets
- `polymarket get-market --market-id <id>` - Get market details and order book
- `polymarket get-positions [--address addr]` - View open positions and PnL
- `polymarket buy --market-id <id> --outcome <yes/no> --amount <usdc> [--price <0-1>]` - Buy outcome shares
- `polymarket sell --market-id <id> --outcome <yes/no> --shares <amount> [--price <0-1>]` - Sell outcome shares
- `polymarket cancel --order-id <id> | --market <id> | --all` - Cancel orders

## Triggers
Activate when users want to trade prediction markets, check Polymarket positions, browse prediction markets, or ask about buying/selling YES/NO tokens on real-world events. Also trigger for phrases like "polymarket price," "prediction market trade," or "buy polymarket shares."
