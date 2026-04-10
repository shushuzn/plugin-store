
# etherfi -- Skill Summary

## Overview
This plugin enables liquid restaking on Ethereum through ether.fi protocol. Users can deposit ETH to receive eETH liquid staking tokens, wrap eETH into yield-bearing weETH (ERC-4626) to earn combined Ethereum staking and EigenLayer restaking rewards, manage token conversions between eETH/weETH, and execute unstaking back to ETH through a two-step withdrawal process. All operations include balance validation, allowance management, and real-time APY tracking.

## Usage
Run commands without `--confirm` to preview transactions, then add `--confirm` to broadcast. Use `etherfi positions` to check balances and current APY before executing any operations.

## Commands
| Command | Description |
|---------|-------------|
| `etherfi positions [--owner ADDRESS]` | View eETH/weETH balances and protocol APY |
| `etherfi stake --amount ETH [--confirm]` | Deposit ETH to receive eETH |
| `etherfi wrap --amount EETH [--confirm]` | Wrap eETH into weETH (ERC-4626) |
| `etherfi unwrap --amount WEETH [--confirm]` | Unwrap weETH back to eETH |
| `etherfi unstake --amount EETH [--confirm]` | Request ETH withdrawal (step 1) |
| `etherfi unstake --claim --token-id ID [--confirm]` | Claim ETH after finalization (step 2) |

## Triggers
Activate when users want to stake ETH on ether.fi, wrap/unwrap between eETH/weETH tokens, unstake back to ETH, check liquid restaking positions, or view current staking APY. Also trigger for EigenLayer restaking rewards and yield-bearing token management.
