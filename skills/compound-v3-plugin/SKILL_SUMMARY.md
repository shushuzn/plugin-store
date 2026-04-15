
# compound-v3-plugin -- Skill Summary

## Overview
This plugin provides comprehensive access to Compound V3 (Comet) lending protocol functionality, enabling users to manage lending positions across Ethereum, Base, Arbitrum, and Polygon networks. It supports the complete lending lifecycle including supplying collateral, borrowing base assets (primarily USDC), repaying debt, withdrawing funds, and claiming COMP rewards. All write operations include safety features like confirmation gates, preview modes, and pre-transaction validation to ensure secure interactions with the protocol.

## Usage
First ensure your wallet is connected via `onchainos wallet login`, then use commands like `compound-v3 get-markets` to view market data or `compound-v3 supply --asset 0x... --amount 1.0` to supply collateral. All write operations require the `--confirm` flag to execute on-chain after showing a preview.

## Commands
| Command | Description |
|---------|-------------|
| `get-markets` | View market statistics including APRs, utilization, and totals |
| `get-position` | Check account position, balances, and collateralization status |
| `supply` | Supply collateral or base assets (auto-repays debt if applicable) |
| `borrow` | Borrow base assets against supplied collateral |
| `repay` | Repay borrowed base assets (partial or full repayment) |
| `withdraw` | Withdraw supplied collateral (requires zero outstanding debt) |
| `claim-rewards` | Claim accrued COMP rewards from lending activities |

## Triggers
Activate this skill when users mention Compound V3, Comet lending operations, or use trigger phrases like "compound supply", "compound borrow", "compound repay", "compound withdraw", "compound rewards", "compound position", or "compound market". Also trigger for lending-related queries on supported chains (Ethereum, Base, Arbitrum, Polygon).
