
# aave-v3-plugin -- Skill Summary

## Overview
This plugin provides comprehensive access to Aave V3, the leading decentralized lending protocol with over $43B TVL. It enables users to supply assets to earn yield, borrow against collateral, monitor health factors, and manage positions across Ethereum, Polygon, Arbitrum, and Base networks. The plugin handles complex operations like automatic WETH wrapping, smart contract approvals, and real-time health factor calculations to ensure safe borrowing practices.

## Usage
Install the plugin and ensure your wallet is connected via `onchainos wallet login`. Use trigger phrases like "supply to aave", "borrow from aave", or "check my aave positions" to activate lending, borrowing, and monitoring functions.

## Commands
| Command | Description |
|---------|-------------|
| `aave-v3-plugin supply` | Deposit assets to earn interest |
| `aave-v3-plugin withdraw` | Redeem aTokens from positions |
| `aave-v3-plugin borrow` | Borrow assets against collateral |
| `aave-v3-plugin repay` | Repay outstanding debt |
| `aave-v3-plugin health-factor` | Check liquidation risk status |
| `aave-v3-plugin positions` | View current supply/borrow positions |
| `aave-v3-plugin reserves` | List market rates and APYs |
| `aave-v3-plugin set-collateral` | Enable/disable asset as collateral |
| `aave-v3-plugin set-emode` | Set efficiency mode for correlated assets |
| `aave-v3-plugin claim-rewards` | Collect protocol rewards |

## Triggers
Activate this skill when users mention Aave-related activities like lending, borrowing, checking positions, or monitoring health factors. The plugin responds to phrases like "supply to aave", "borrow from aave", "aave health factor", and "my aave positions".
