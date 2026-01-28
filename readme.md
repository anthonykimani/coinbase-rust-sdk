# Coinbase Rust SDK

Learning log and implementation notes for porting the TypeScript SDK to Rust.

## Progress so far

### Config module (`src/config.rs`)
- Added a default base path constant: `https://api.cdp.coinbase.com/platform`.
- `Config::new` now sets the default base path automatically.
- Added `with_base_path(self, ...) -> Self` for builder-style overrides.
- Added `validate(&self)` to check API key id/secret are non-empty.
- Added `build_url(&self, path)` that joins base path + path safely.
- Added unit tests for `build_url` slash handling.

### Coinbase client (`src/coinbase.rs`)
- Added `Coinbase::new(config)` to create a client with `reqwest::Client`.
- Added `Coinbase::build_url(&self, path)` delegating to `Config::build_url`.
- Added a unit test for `Coinbase::build_url`.
- `generate_jwt` now calls `config.validate()` before proceeding.
- URI binding string now uses the TS format: `"METHOD host/path"`.

### Claims type (`src/types.rs`)
- Added `Claims` struct constructor: `Claims::new(...)`.
- Enabled `Serialize` on `Claims` for future JWT signing.

## JWT flow status

Implemented:
- URI binding string formatting.
- Claims constructor in `types.rs`.

Pending:
- Build `Claims` inside `generate_jwt`.
- Add signing (EC + Ed25519) using Rust crypto crates.
- Add nonce, iat/nbf/exp handling aligned with TS.

## Notes

This repo is being ported from:
`@coinbase-sdk-nodejs` (TypeScript)
