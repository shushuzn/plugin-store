
# aave-v3-plugin -- Skill Summary

## Overview
This plugin provides complete access to Aave V3 lending and borrowing functionality across four major chains (Ethereum, Polygon, Arbitrum, Base). Users can supply assets to earn yield, borrow against collateral, monitor health factors to avoid liquidation, manage collateral settings, and claim rewards. The plugin integrates with the onchainos CLI for secure transaction execution and uses runtime address resolution for maximum security.

## Usage
Connect your wallet with `onchainos wallet login`, then use natural language commands like "supply 1000 USDC to aave" or "check my aave health factor". All write operations require user confirmation and support dry-run simulation for safety.

## Commands
| Command | Purpose |
|---------|---------|
| `aave-v3-plugin supply` | Deposit assets to earn interest |
| `aave-v3-plugin withdraw` | Redeem supplied assets |
| `aave-v3-plugin borrow` | Borrow against collateral |
| `aave-v3-plugin repay` | Repay outstanding debt |
| `aave-v3-plugin health-factor` | Check liquidation risk |
| `aave-v3-plugin positions` | View current portfolio |
| `aave-v3-plugin reserves` | List market rates and APYs |
| `aave-v3-plugin set-collateral` | Enable/disable asset as collateral |
| `aave-v3-plugin set-emode` | Configure efficiency mode |
| `aave-v3-plugin claim-rewards` | Collect accrued incentives |

## Triggers
Activate this skill when users mention Aave-related actions like "supply to aave", "borrow from aave", "aave health factor", "my aave positions", "aave interest rates", or want to manage DeFi lending/borrowing positions. Also triggered by liquidation risk concerns or yield optimization queries.
