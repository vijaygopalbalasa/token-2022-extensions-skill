# Resources — curated, current sources

Official and high-signal sources for Token-2022. Prefer these over blog posts; verify versions
against the registries.

## Official docs
- Token Extensions overview (the canonical extension list): https://solana.com/docs/tokens/extensions
- Transfer fees: https://solana.com/docs/tokens/extensions/transfer-fees
- Scaled UI amount: https://solana.com/docs/tokens/extensions/scaled-ui-amount
- Pausable: https://solana.com/docs/tokens/extensions/pausable
- Confidential transfer (check current availability): https://solana.com/docs/tokens/extensions/confidential-transfer
- Transfer hook guide: https://solana.com/developers/guides/token-extensions/transfer-hook
- Transfer Hook Interface examples: https://www.solana-program.com/docs/transfer-hook-interface/examples

## Crates (Rust) — verify latest on docs.rs
- spl-token-2022: https://docs.rs/crate/spl-token-2022/latest
- spl-transfer-hook-interface: https://docs.rs/crate/spl-transfer-hook-interface/latest
- spl-tlv-account-resolution: https://docs.rs/crate/spl-tlv-account-resolution/latest
- spl-token-metadata-interface: https://docs.rs/crate/spl-token-metadata-interface/latest
- spl-token-group-interface: https://docs.rs/crate/spl-token-group-interface/latest
- spl-associated-token-account: https://docs.rs/crate/spl-associated-token-account/latest

## Packages (JS/TS) — verify latest on npm
- @solana/kit: https://www.npmjs.com/package/@solana/kit
- @solana-program/token-2022: https://www.npmjs.com/package/@solana-program/token-2022
- @solana/spl-token (legacy v1): https://www.npmjs.com/package/@solana/spl-token
- litesvm: https://www.npmjs.com/package/litesvm

## Security
- Neodyme — "A Pwn Story" on Token-2022 footguns (hooks, fees, delegate): https://neodyme.io/en/blog/token-2022/
- Confidential transfer incident post-mortem (June 2025): https://solana.com/news/post-mortem-june-25-2025
- Token-2022 tracking issues: https://github.com/solana-program/token-2022/issues

## Testing
- LiteSVM: https://github.com/LiteSVM/litesvm
- Mollusk: https://github.com/anza-xyz/mollusk
- Surfpool: https://github.com/txtx/surfpool

> All version-specific claims in this skill were verified on **2026-06-19**. When in doubt, the
> registry (docs.rs / npm) and the official docs above are authoritative — re-check rather than
> trust a pinned number that may have moved.
