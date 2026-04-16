# etherfi-plugin
A liquid restaking protocol plugin for Ethereum that enables ETH deposits to receive eETH, wrap/unwrap eETH/weETH tokens, and manage positions with APY tracking.

## Highlights
- Deposit ETH to receive eETH liquid staking tokens through ether.fi protocol
- Wrap eETH into weETH (ERC-4626) to earn auto-compounding staking + EigenLayer restaking rewards
- Unwrap weETH back to eETH to realize accumulated gains
- Two-step withdrawal process: request unstaking and claim ETH after finalization
- Real-time position tracking with balances, exchange rates, and current APY
- Secure transaction preview system requiring explicit confirmation before broadcasting
- Direct on-chain integration with automatic allowance management for token operations
- Support for both connected wallet and specific address balance queries

