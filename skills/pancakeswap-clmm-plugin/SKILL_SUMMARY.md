
# pancakeswap-clmm-plugin -- Skill Summary

## Overview
This plugin provides comprehensive farming capabilities for PancakeSwap V3 concentrated liquidity positions. It enables users to stake their V3 LP NFTs into MasterChefV3 contracts to earn CAKE rewards, harvest accumulated rewards, collect trading fees, and monitor farming opportunities across multiple chains. The plugin works seamlessly with the pancakeswap-v3 plugin to provide a complete V3 liquidity management and farming workflow.

## Usage
First create a V3 LP position using pancakeswap-v3, then use this plugin to stake the NFT with `pancakeswap-clmm farm --token-id <ID>` to start earning CAKE rewards. All write operations require the `--confirm` flag to execute after showing a preview.

## Commands
| Command | Description |
|---------|-------------|
| `farm --token-id <ID>` | Stake LP NFT into MasterChefV3 to earn CAKE |
| `unfarm --token-id <ID>` | Withdraw staked NFT and harvest pending CAKE |
| `harvest --token-id <ID>` | Claim CAKE rewards without withdrawing NFT |
| `collect-fees --token-id <ID>` | Collect swap fees from unstaked positions |
| `pending-rewards --token-id <ID>` | View pending CAKE rewards (read-only) |
| `farm-pools` | List active farming pools with CAKE incentives |
| `positions` | View LP positions with optional staked position lookup |

## Triggers
An AI agent should activate this skill when users want to farm CAKE rewards on PancakeSwap V3 positions, harvest rewards, collect fees, or view farming opportunities. Use trigger phrases like "stake LP NFT", "farm CAKE", "harvest rewards", "collect fees", or "PancakeSwap farming".
