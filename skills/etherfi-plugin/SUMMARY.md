# etherfi-plugin
A liquid restaking protocol for Ethereum that enables depositing ETH to receive eETH, wrapping/unwrapping eETH/weETH tokens, unstaking back to ETH, and monitoring positions with APY tracking.

## Highlights
- Deposit ETH to receive eETH liquid staking tokens through ether.fi protocol
- Wrap eETH into weETH (ERC-4626) to earn auto-compounding staking + EigenLayer rewards
- Unwrap weETH back to eETH to realize accumulated yield
- Two-step unstaking process: request withdrawal and claim ETH after finalization
- Real-time position tracking with USD valuations and current APY rates
- Secure transaction preview system requiring explicit confirmation before broadcast
- Integration with onchainos CLI for TEE-sandboxed transaction signing
- Support for checking withdrawal request finalization status via NFT tokens

