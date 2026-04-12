
# pancakeswap-clmm -- Skill Summary

## Overview
This skill provides comprehensive PancakeSwap V3 CLMM (Concentrated Liquidity Market Maker) farming capabilities, allowing users to stake their V3 LP NFTs into MasterChefV3 contracts to earn CAKE rewards. It handles the complete farming lifecycle including staking, harvesting rewards, collecting swap fees, and managing positions across multiple chains including BSC, Ethereum, Base, and Arbitrum.

## Usage
Install the plugin and use commands like `pancakeswap-clmm farm --token-id 12345` to stake LP NFTs or `pancakeswap-clmm harvest --token-id 12345 --confirm` to claim CAKE rewards. All write operations require `--confirm` flag for execution, otherwise they show previews.

## Commands
| Command | Description |
|---------|-------------|
| `farm --token-id <ID>` | Stake LP NFT into MasterChefV3 to earn CAKE |
| `unfarm --token-id <ID>` | Withdraw LP NFT and harvest pending rewards |
| `harvest --token-id <ID>` | Claim CAKE rewards without unstaking |
| `collect-fees --token-id <ID>` | Collect swap fees from unstaked positions |
| `pending-rewards --token-id <ID>` | View pending CAKE rewards |
| `farm-pools` | List active farming pools with CAKE incentives |
| `positions` | View LP positions with optional staked inclusion |

## Triggers
Activate this skill when users want to farm CAKE rewards with their PancakeSwap V3 LP positions, harvest accumulated rewards, or collect swap fees. Use trigger phrases like "stake LP NFT", "farm CAKE", "harvest rewards", or "PancakeSwap farming".
