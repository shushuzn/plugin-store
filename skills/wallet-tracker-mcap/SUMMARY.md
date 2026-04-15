# wallet-tracker-mcap
A Solana wallet copy-trading bot that monitors target wallets and auto-mirrors meme token trades with market cap gating, tiered take-profit, and comprehensive risk controls.

## Highlights
- **Two Follow Modes**: MC_TARGET (wait for market cap proof) or INSTANT (immediate follow)
- **5 Exit Triggers**: Mirror sell, stop loss, tiered take-profit, trailing stop, and time stop
- **4-Tier Risk Grading**: Honeypot, rug history, wash trading, and liquidity drain detection
- **Safety Gates**: Liquidity, holders, top10, dev hold, and bundle checks before every trade
- **TEE Signing**: onchainos Agentic Wallet with private keys never leaving secure enclave
- **Paper Mode**: Safe testing with MODE="paper" and PAUSED=True by default
- **Web Dashboard**: Live positions, watch list, trades, and feed at http://localhost:3248
- **Zero Dependencies**: Python 3.8+ stdlib only plus onchainos CLI

