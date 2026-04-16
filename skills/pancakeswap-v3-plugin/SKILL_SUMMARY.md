
# pancakeswap-v3-plugin -- Skill Summary

## Overview
This skill enables token swapping and concentrated liquidity management on PancakeSwap V3, the leading DEX protocol. It provides comprehensive functionality for trading tokens, providing liquidity in specific price ranges, and managing LP positions across multiple chains including BNB Chain, Base, Ethereum, Arbitrum, and Linea. All operations include proper slippage protection, balance validation, and multi-step transaction handling.

## Usage
Install the plugin using the auto-injected setup commands, then use commands like `pancakeswap-v3 swap` for token swaps or `pancakeswap-v3 add-liquidity` for providing liquidity. Ensure your wallet is connected via `onchainos wallet login` before executing write operations.

## Commands
- `quote` - Get swap quotes without executing transactions
- `swap` - Swap tokens via SmartRouter (requires --confirm)
- `pools` - List all pools for a token pair across fee tiers
- `positions` - View LP positions for a wallet address
- `add-liquidity` - Mint new concentrated liquidity positions (requires --confirm)
- `remove-liquidity` - Remove liquidity and collect tokens (requires --confirm)

## Triggers
Activate when users mention "pancakeswap", "swap on pancake", "PCS swap", "add liquidity pancakeswap", "remove liquidity", or "PancakeSwap V3". Also trigger for concentrated liquidity management tasks or multi-chain DEX operations on supported networks.
