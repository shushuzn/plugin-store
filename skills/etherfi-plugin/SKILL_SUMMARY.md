
# etherfi-plugin -- Skill Summary

## Overview
The etherfi-plugin enables liquid restaking on Ethereum through the ether.fi protocol. Users can deposit ETH to receive eETH liquid staking tokens, wrap them into weETH for auto-compounding yields from Ethereum staking and EigenLayer restaking rewards, and manage the complete lifecycle from staking to withdrawal. The plugin provides secure transaction handling with preview-first confirmation and real-time position monitoring with APY tracking.

## Usage
Run `etherfi positions` to view current balances and yields. Use `etherfi stake --amount <ETH> --confirm` to deposit ETH, `etherfi wrap --amount <eETH> --confirm` to convert to yield-bearing weETH, and `etherfi unstake --amount <eETH> --confirm` followed by `etherfi unstake --claim --token-id <id> --confirm` for withdrawals.

## Commands
| Command | Description |
|---------|-------------|
| `etherfi positions [--owner ADDRESS]` | View eETH/weETH balances, USD values, and protocol APY |
| `etherfi stake --amount ETH [--confirm]` | Deposit ETH to receive eETH tokens |
| `etherfi wrap --amount eETH [--confirm]` | Wrap eETH into yield-bearing weETH |
| `etherfi unwrap --amount weETH [--confirm]` | Unwrap weETH back to eETH |
| `etherfi unstake --amount eETH [--confirm]` | Request eETH withdrawal (step 1) |
| `etherfi unstake --claim --token-id ID [--confirm]` | Claim ETH after finalization (step 2) |

## Triggers
Activate this skill when users want to participate in Ethereum liquid restaking, earn staking and EigenLayer rewards, or manage their ether.fi positions. Use for any mentions of eETH, weETH, ether.fi staking, liquid restaking, or EigenLayer yield opportunities.
