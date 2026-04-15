
# wallet-tracker-mcap -- Skill Summary

## Overview
This skill implements a Solana wallet copy-trading bot that monitors target wallets for meme token trades and automatically mirrors their buy/sell decisions with comprehensive safety checks. It offers two follow modes (MC_TARGET for safer market cap-gated entries or INSTANT for immediate following), multiple exit strategies including tiered take-profit and trailing stops, and 4-tier risk assessment to protect against honeypots and rug pulls. All trades are executed through onchainos Agentic Wallet with TEE signing for maximum security.

## Usage
Install with `npx skills add okx/plugin-store --skill wallet-tracker-mcap`, configure target wallets and risk parameters in config.py, then run `python3 wallet_tracker.py`. Start in paper mode for safe testing before switching to live trading.

## Commands
This is a reference skill with no CLI commands.

## Triggers
Activate this skill when users want to copy-trade specific Solana wallets, follow "smart money" or whale movements, or implement automated meme token trading strategies with risk controls. Best suited for users who have identified successful traders they want to mirror while maintaining their own safety parameters.
