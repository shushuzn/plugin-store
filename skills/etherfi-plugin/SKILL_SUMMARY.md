
# etherfi-plugin -- Skill Summary

## Overview
The ether.fi plugin provides comprehensive liquid restaking functionality on Ethereum, allowing users to deposit ETH to receive eETH tokens, wrap them into yield-bearing weETH (ERC-4626), and manage their liquid staking positions. The plugin handles the complete lifecycle from initial ETH deposits through to final withdrawals, while providing real-time APY data and position tracking with USD valuations.

## Usage
Install the plugin and ensure onchainos CLI is configured with your wallet. Run commands without `--confirm` first to preview transactions, then add `--confirm` to broadcast to the blockchain.

## Commands
| Command | Description |
|---------|-------------|
| `etherfi positions [--owner ADDRESS]` | View eETH/weETH balances, total value, and current APY |
| `etherfi stake --amount ETH [--confirm]` | Deposit ETH to receive eETH tokens |
| `etherfi wrap --amount EETH [--confirm]` | Convert eETH to yield-bearing weETH |
| `etherfi unwrap --amount WEETH [--confirm]` | Convert weETH back to eETH |
| `etherfi unstake --amount EETH [--confirm]` | Request ETH withdrawal (step 1) |
| `etherfi unstake --claim --token-id ID [--confirm]` | Claim ETH after finalization (step 2) |

## Triggers
Activate this skill when users want to stake ETH on ether.fi, manage eETH/weETH positions, check liquid staking yields, or perform ETH withdrawals from the ether.fi protocol. Also trigger for queries about ether.fi APY, liquid restaking rewards, or EigenLayer restaking integration.
