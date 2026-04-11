
# lido -- Skill Summary

## Overview
This skill enables interaction with the Lido liquid staking protocol on Ethereum mainnet, allowing users to stake ETH for stETH (liquid staking tokens), manage withdrawal requests through Lido's queue system, and track staking rewards. All operations are secured through onchainos with mandatory user confirmation for write transactions.

## Usage
Install the plugin via `npx skills add okx/plugin-store --skill lido`, then use commands like `lido stake --amount-eth 1.0` to begin staking. For write operations, review the transaction preview first, then add `--confirm` to execute.

## Commands
| Command | Description |
|---|---|
| `lido stake` | Stake ETH to receive stETH with optional referral |
| `lido get-apy` | Get current 7-day average stETH staking APR |
| `lido balance` | Check stETH balance and shares for an address |
| `lido request-withdrawal` | Request withdrawal of stETH for ETH (2-step process) |
| `lido get-withdrawals` | List pending and past withdrawal requests with status |
| `lido claim-withdrawal` | Claim finalized withdrawal(s) and receive ETH |

## Triggers
Activate this skill when users want to stake ETH for liquid rewards, need to manage Lido staking positions, or want to exit staking positions through the withdrawal queue. Use for Lido-specific operations on Ethereum mainnet only.
