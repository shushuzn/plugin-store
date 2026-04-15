
# lido-plugin -- Skill Summary

## Overview
This plugin enables interaction with the Lido liquid staking protocol on Ethereum mainnet, allowing users to stake ETH for stETH tokens, manage withdrawal requests, and track staking performance. All write operations are secured through the onchainos CLI with mandatory user confirmation before transaction submission.

## Usage
Install via the plugin store and ensure onchainos CLI is available. Use commands like `lido stake --amount-eth 1.0` to stake ETH or `lido get-apy` to check current returns.

## Commands
| Command | Description |
|---|---|
| `lido stake` | Stake ETH to receive stETH |
| `lido get-apy` | Get current stETH staking APR |
| `lido balance` | Check stETH balance |
| `lido request-withdrawal` | Request withdrawal of stETH for ETH |
| `lido get-withdrawals` | List pending and past withdrawal requests |
| `lido claim-withdrawal` | Claim finalized withdrawal(s) |
| `lido wrap` | Convert stETH to wstETH |
| `lido unwrap` | Convert wstETH back to stETH |

## Triggers
Activate when users want to stake ETH for liquid staking rewards, manage Lido staking positions, or need to withdraw staked ETH back to regular ETH on Ethereum mainnet.
