# etherfi
A liquid restaking protocol on Ethereum that allows users to deposit ETH to receive eETH, wrap/unwrap between eETH/weETH for yield-bearing rewards, and unstake back to ETH.

## Highlights
- Liquid restaking with eETH tokens that maintain liquidity while earning staking rewards
- ERC-4626 weETH wrapper that auto-compounds staking + EigenLayer restaking rewards
- Direct ETH deposits into ether.fi LiquidityPool with 1:1 eETH minting
- Two-step withdrawal process with WithdrawRequestNFT for ETH redemption
- Real-time position tracking with current APY and TVL data
- Preview-first transaction flow with explicit confirmation requirements
- Automatic allowance management for token wrapping operations
- Integration with onchainos wallet for secure transaction signing

