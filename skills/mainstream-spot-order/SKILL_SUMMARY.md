
# mainstream-spot-order -- Skill Summary

## Overview
This is a sophisticated spot trading system that waits for consensus across 6 technical indicators before executing trades on major cryptocurrency pairs (SOL, ETH, BTC, BNB, AVAX, DOGE). The system features AI-powered auto-research that continuously tests and optimizes the trading strategy, comprehensive backtesting capabilities, and both paper and live trading modes with real-time data collection and analysis.

## Usage
Start by running data collection and backtesting to validate the strategy, then proceed to paper trading for live validation before considering real trades. The system provides a web dashboard for monitoring and uses the onchainos CLI for data collection and trade execution.

## Commands
- `python3 collect.py --backfill` - Collect historical price data
- `python3 collect.py --daemon` - Start continuous data collection with dashboard
- `python3 backtest.py --pair SOL` - Run strategy backtest on historical data
- `python3 live.py --pair SOL` - Start paper/live trading engine
- `onchainos wallet login` - Login to OKX Agentic Wallet for live trading
- Dashboard access at `http://localhost:3250` for real-time monitoring

## Triggers
An AI agent should activate this skill when users mention mainstream spot trading, multi-chain DEX trading, or want to trade major cryptocurrency pairs like SOL, ETH, BTC with a systematic approach. It's particularly suitable when users seek a research-driven, consensus-based trading strategy rather than high-frequency or speculative trading.
