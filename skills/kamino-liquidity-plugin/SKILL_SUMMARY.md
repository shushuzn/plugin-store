
# kamino-liquidity-plugin -- Skill Summary

## Overview
This skill provides access to Kamino Liquidity KVault earn vaults on Solana, allowing users to deposit tokens into auto-compounding vaults that generate yield through automated liquidity allocation strategies. Users can view available vaults, track their positions, deposit tokens to earn yield, and withdraw their shares back to underlying tokens.

## Usage
Install the plugin and use commands like `kamino-liquidity vaults` to browse vaults, `kamino-liquidity deposit --vault <address> --amount <amount>` to invest, and `kamino-liquidity withdraw --vault <address> --amount <shares>` to exit positions. All write operations require user confirmation.

## Commands
| Command | Description |
|---------|-------------|
| `vaults` | List all available KVault earn vaults with optional filtering |
| `positions` | View your share balances across all vaults |
| `deposit` | Deposit tokens into a vault to earn yield |
| `withdraw` | Redeem shares for underlying tokens |

## Triggers
Activate when users mention Kamino vaults, Kamino liquidity, depositing to Kamino, Kamino earn, KVault, or Kamino yield vault operations. Also responds to Chinese phrases like Kamino流动性, Kamino保险库, 存入Kamino, or Kamino赚取收益.
