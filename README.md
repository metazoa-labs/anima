## Anima Wallet

A crypto wallet for the Aptos ecosystem.

<img src="./assets/marquee.svg" alt="drawing" width="100%"/>

Note: This is a desktop app. For the Anima browser plugin, work is in progress for reusing these components for a Metamask Snap integration here: https://github.com/metazoa-labs/metamask-snap-move.
## Based on Tauri-Wallet
A framework for crypto wallets using Rust and Typescript, thanks to Tauri and Svelte.

This is a skeleton for building wallets it comes with certain features out of the box.

- [x] Builds Windows, MacOs, Linux apps for distribution.
- [ ] Mobile iOS and Android, coming soon thanks to Tauri.
- [x] Auto-updating app over the air.
- [x] Uses the OS keyring to store private keys.
- [x] Scaffold for signing transactions in memory-safe Rust environment.
- [x] Account profile creation an saving to disk.
- [x] Multi-account by default.
- [x] Internationalized starting with `en` and `zh_cn`.
- [x] Set "playlists" of fullnode peers to connect to. 
- [x] Developer debugging view.
## Development

### Quick Start:
Asssuming you have Rust, Node, and Yarn installed, do this:
```
make dev
```
Otherwise follow the link for [development environment setup](docs/devs/get-started.md).

## DONE:
- [x] account creation, using BIP39 mnemonic phrases.
- [x] save account to OS keyring.
- [x] user interfaces for wallet selector, settings, debug mode, transactions.
- [x] switch between different aptos network chains.
- [x] publish initial set of seed peers for fullnodes.
- [x] fetch LedgerInfo on each network change.
- [x] query on-chain account resources, parse into Rust types.
- [x] signing of transactions in rust.
- [x] UX for debug mode to query the root 0x0 state.

## TODO:

- [ ] Query Balances (Blocking on Devnet faucet and coins);
- [ ] Submit transaction and error handling (Blocking on Devnet capabilities)
