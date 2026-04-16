
# etherfi-plugin -- Skill Summary

## Overview
The etherfi-plugin enables liquid restaking on Ethereum through the ether.fi protocol. Users can deposit ETH to receive eETH tokens, wrap them into yield-bearing weETH (ERC-4626) tokens that auto-compound staking and EigenLayer restaking rewards, manage unwrapping and unstaking operations, and monitor their positions with real-time APY data. All write operations use a secure two-step confirmation process with transaction previews before broadcasting.

## Usage
Install the plugin through the auto-injected setup commands, ensure onchainos CLI is configured with your wallet, then use commands like `etherfi stake --amount 0.1 --confirm` to deposit ETH or `etherfi positions` to check balances. All transaction commands require the `--confirm` flag after reviewing the preview.

## Commands
- `etherfi positions [--owner ADDRESS]` - View eETH/weETH balances, exchange rates, and APY
- `etherfi stake --amount ETH [--confirm]` - Deposit ETH to receive eETH tokens  
- `etherfi wrap --amount EETH [--confirm]` - Wrap eETH into yield-bearing weETH
- `etherfi unwrap --amount WEETH [--confirm]` - Unwrap weETH back to eETH
- `etherfi unstake --amount EETH [--confirm]` - Request eETH withdrawal (step 1)
- `etherfi unstake --claim --token-id ID [--confirm]` - Claim ETH after finalization (step 2)

## Triggers
Activate this skill when users want to stake ETH on ether.fi, manage eETH/weETH positions, earn liquid staking rewards with EigenLayer restaking, or check their ether.fi protocol balances and APY. Also triggered by phrases like "wrap eETH", "unstake from ether.fi", or "liquid restaking rewards".
