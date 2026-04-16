# etherfi-plugin
A liquid restaking plugin for Ethereum that enables ETH deposits to receive eETH, wrap/unwrap eETH/weETH tokens, unstake back to ETH, and view positions with APY data.

## Highlights
- Deposit ETH into ether.fi protocol to receive liquid staking token eETH
- Wrap eETH into weETH (ERC-4626) to earn auto-compounding staking + EigenLayer rewards
- Unwrap weETH back to eETH to realize accumulated yields
- Two-step ETH withdrawal process with request and claim functionality
- Real-time position tracking with USD valuation and current APY rates
- Secure transaction preview before confirmation with --confirm flag
- Integration with onchainos wallet for safe transaction signing
- Support for both connected wallet and specific address queries

