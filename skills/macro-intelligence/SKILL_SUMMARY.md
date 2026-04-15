
# macro-intelligence -- Skill Summary

## Overview
This skill creates a unified macro intelligence feed by monitoring 7+ news sources (NewsNow, Polymarket, Telegram, 6551.io OpenNews, Finnhub, FRED, Fear & Greed Index), classifying macro events using AI, scoring sentiment, and exposing clean trading signals via HTTP API. It provides real-time market context data including price tickers, macro indicators, and sentiment analysis without executing any trades itself.

## Usage
Run `python3 macro_news.py` to start all collectors and HTTP server on port 3252. Open `http://localhost:3252` to view the monitoring dashboard with live signals and market data.

## Commands
This is a reference skill with no CLI commands.

## Triggers
Activate this skill when you need macro market intelligence, sentiment analysis, or real-time monitoring of Fed policy, CPI data, whale movements, geopolitical events, or RWA developments. Use it as a data feed for downstream trading strategies that need classified macro signals.
