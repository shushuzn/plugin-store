
# compound-v3 -- Skill Summary

## Overview
This plugin provides comprehensive access to Compound V3 (Comet) lending protocol functionality across Ethereum, Base, Arbitrum, and Polygon networks. It enables users to supply collateral, borrow base assets, repay debt, withdraw collateral, and claim COMP rewards with built-in safety checks and transaction previewing capabilities.

## Usage
Install the plugin via OKX plugin store and ensure your wallet is connected with `onchainos wallet login`. Use dry-run mode to preview transactions before execution, and the plugin will guide you through confirmation steps for all write operations.

## Commands
| Command | Description |
|---------|-------------|
| `get-markets` | View market statistics (utilization, APRs, total supply/borrow) |
| `get-position` | View account position (balances, collateralization status) |
| `supply` | Supply collateral or base asset (auto-repays debt if applicable) |
| `borrow` | Borrow base asset against collateral |
| `repay` | Repay borrowed base asset (partial or full) |
| `withdraw` | Withdraw supplied collateral (requires zero debt) |
| `claim-rewards` | Claim COMP token rewards |

## Triggers
Activate this skill when users want to interact with Compound V3 protocol using trigger phrases like "compound supply", "compound borrow", "compound repay", "compound withdraw", "compound rewards", "compound position", or "compound market". The skill handles lending, borrowing, and reward claiming operations on supported networks.
