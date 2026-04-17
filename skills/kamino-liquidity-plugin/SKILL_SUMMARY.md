
# kamino-liquidity-plugin -- Skill Summary

## Overview

This plugin enables interaction with Kamino Liquidity KVault earn vaults on Solana, allowing users to deposit single tokens into auto-compounding yield optimization vaults. The plugin handles vault discovery, position tracking, deposits, withdrawals, and provides a streamlined onboarding experience with wallet status checking and transaction previewing capabilities.

## Usage

Install the plugin and run `kamino-liquidity quickstart` to check wallet status and get personalized recommendations. Use `kamino-liquidity vaults` to browse available vaults, then deposit tokens with the `deposit` command after previewing with `--dry-run`.

## Commands

| Command | Description |
|---------|-------------|
| `quickstart` | Show wallet status and suggest next actions |
| `vaults` | List all available KVault earn vaults |
| `positions` | View current share balances across vaults |
| `deposit` | Deposit tokens into a vault (requires `--confirm`) |
| `withdraw` | Redeem shares for underlying tokens (requires `--confirm`) |

## Triggers

Activate this skill when users mention Kamino vaults, Kamino liquidity, depositing to Kamino, Kamino earn, KVault, or Kamino yield vault operations. Also triggered by Chinese terms like Kamino流动性, Kamino保险库, 存入Kamino, or Kamino赚取收益.
