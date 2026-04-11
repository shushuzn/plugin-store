
# lido -- Skill Summary

## Overview
The Lido plugin enables interaction with the Lido liquid staking protocol on Ethereum mainnet, allowing users to stake ETH to receive stETH (a rebasing liquid staking token), request withdrawals back to ETH, monitor withdrawal status, and claim finalized withdrawals. All write operations require explicit user confirmation and route through the onchainos CLI for secure transaction handling.

## Usage
Install the plugin and ensure onchainos CLI is available with a logged-in wallet for write operations. Use commands like `lido stake --amount-eth 1.0` to stake ETH or `lido get-apy` to check current staking rewards.

## Commands
| Command | Description |
|---|---|
| `lido stake --amount-eth <amount>` | Stake ETH to receive stETH |
| `lido get-apy` | Get current stETH staking APR |
| `lido balance [--address <addr>]` | Check stETH balance |
| `lido request-withdrawal --amount-eth <amount>` | Request withdrawal of stETH for ETH |
| `lido get-withdrawals [--address <addr>]` | List pending and past withdrawal requests |
| `lido claim-withdrawal --ids <id1,id2>` | Claim finalized withdrawal(s) |

## Triggers
Activate this skill when users want to stake ETH for liquid staking rewards, manage Lido staking positions, or need to withdraw staked ETH back to regular ETH. Also useful for checking staking APY rates and monitoring withdrawal request status.
