
# morpho -- Skill Summary

## Overview
The morpho skill enables interaction with Morpho, a permissionless lending protocol with over $5B TVL. It supports both Morpho Blue isolated lending markets and MetaMorpho ERC-4626 vaults curated by risk managers like Gauntlet and Steakhouse. Users can supply assets to earn yield, borrow against collateral, manage positions with health factor monitoring, and claim rewards across Ethereum Mainnet and Base networks.

## Usage
Install with `npx skills add okx/plugin-store-community --skill morpho`. Always run write operations with `--dry-run` first, then confirm before executing on-chain transactions. Ensure your wallet is connected via `onchainos wallet login` before use.

## Commands
| Command | Description |
|---------|-------------|
| `morpho positions` | View your positions and health factors |
| `morpho markets [--asset SYMBOL]` | List Morpho Blue markets with APYs |
| `morpho vaults [--asset SYMBOL]` | List MetaMorpho vaults |
| `morpho supply --vault ADDR --asset SYMBOL --amount N` | Supply to MetaMorpho vault |
| `morpho withdraw --vault ADDR --asset SYMBOL --amount N` | Withdraw from vault |
| `morpho borrow --market-id HEX --amount N` | Borrow from Morpho Blue |
| `morpho repay --market-id HEX --amount N` | Repay debt |
| `morpho supply-collateral --market-id HEX --amount N` | Add collateral |
| `morpho claim-rewards` | Claim Merkl rewards |

## Triggers
Activate when users mention supplying/depositing to Morpho vaults, borrowing from Morpho Blue, checking Morpho positions or health factors, viewing Morpho interest rates, repaying Morpho loans, or claiming Morpho rewards. Also trigger for Chinese equivalents like "Morpho存款", "从Morpho借款", or "健康因子".
