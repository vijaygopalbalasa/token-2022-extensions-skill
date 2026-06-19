# Clients — creating mints/accounts with extensions (2026 stack)

How to actually create Token-2022 mints and accounts in code, which packages to use, and the two
rules that cause silent failures (ATA derivation + account sizing).

> **Version policy:** the numbers below are the latest **verified** stable as of **2026-06-19**
> (crates.io / docs.rs / npm). Pin against the registry at build time — this file states intent
> and known-good ranges, and flags anything uncertain. Don't trust a version number that's a year
> old without re-checking.

## Which client?

| You are… | Use | Token-2022 package |
|---|---|---|
| Writing new JS/TS | `@solana/kit` | `@solana-program/token-2022` (Codama-generated, Kit-native) |
| Maintaining a web3.js **v1** codebase | `@solana/web3.js` v1 | `@solana/spl-token` (legacy; still works with Token-2022) |
| Writing a Rust client/program | — | `spl-token-2022` (+ `spl-associated-token-account`) |

Recommended for new builds: **`@solana/kit` + `@solana-program/token-2022`**. The npm docs for
`@solana/spl-token` themselves say to use `@solana-program/token` / `@solana-program/token-2022`
with web3.js v2 / Kit.

## Verified versions (2026-06-19)

### JS / TS (npm)
| Package | Version | Notes |
|---|---|---|
| `@solana/kit` | `6.10.0` (2026-06-16) | modern tree-shakable API (web3.js v2 successor) |
| `@solana-program/token-2022` | `0.12.0` (2026-06-17) | exports `TOKEN_2022_PROGRAM_ADDRESS`, `getMintSize`, `findAssociatedTokenPda`, `fetchMint`, … |
| `@solana-program/token` | `0.14.0` (2026-06-16) | base Token program client |
| `@solana/spl-token` | `0.4.14` (2025-09-02) | **legacy / web3.js v1**; maintained, not preferred for new code |
| `litesvm` (npm) | `1.1.0` (Kit-native); `0.8.0` (last web3.js v1) | in-process SVM for tests; pick the line matching your client ([testing.md](testing.md)) |
| `gill` | `0.14.0` | optional third-party Kit wrapper |

### Rust (crates.io / docs.rs)
| Crate | Version | Notes |
|---|---|---|
| `spl-token-2022` | `11.0.0` (2026-05-08) | Token Extensions program crate |
| `spl-token` | `9.0.0` | base Token program |
| `spl-associated-token-account` | `8.0.0` | ATA derivation/creation |
| `spl-transfer-hook-interface` | `2.1.0` (2025-11-05) | transfer-hook programs ([transfer-hooks.md](transfer-hooks.md)) |
| `spl-tlv-account-resolution` | `0.11.1` | `ExtraAccountMetaList` |
| `spl-token-metadata-interface` | `1.0.0` | on-mint metadata |
| `spl-token-group-interface` | `0.7.2` | groups/members |
| `spl-pod` | `0.7.3` | ⚠️ `0.8.0` was **yanked** — pin `0.7.3` |
| `spl-token-client` | `0.19.1` | verify latest on docs.rs before pinning |

**Solana / Anchor:** Solana CLI **3.x** (granular `solana-*` crates `^3`; `solana-program` removed
as a project dep — use `solana-signer` etc.). Anchor: **1.x** is current (`anchor-spl` 1.x, TS pkg
**`@anchor-lang/core`**); **0.31/0.32** are still widely used (TS pkg `@coral-xyz/anchor`). Match
your program's Anchor version to its TS client import path.

> The reference [`program/`](../../program) in this repo is **native Rust** (no Anchor) precisely
> to avoid Anchor↔Solana version friction; it pins the spl crates above and builds with the
> installed `cargo build-sbf`.

## Rule 1 — ATA derivation MUST pass the Token-2022 program ID

The associated-token-account PDA is derived from `[owner, token_program, mint]`. For a Token-2022
mint you must pass **`TOKEN_2022_PROGRAM_ADDRESS`** as the token-program seed. Using the base Token
ID derives a **different, wrong** ATA (a very common bug).

```ts
// Kit
import { findAssociatedTokenPda, TOKEN_2022_PROGRAM_ADDRESS } from '@solana-program/token-2022';
const [ata] = await findAssociatedTokenPda({ owner, mint, tokenProgram: TOKEN_2022_PROGRAM_ADDRESS });
```
```rust
// Rust
use spl_associated_token_account::get_associated_token_address_with_program_id;
let ata = get_associated_token_address_with_program_id(&owner, &mint, &spl_token_2022::id());
```

Program IDs:
- Token-2022: `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`
- base Token: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`

## Rule 2 — size the mint for its extensions, in the right order

Base Token mints are fixed-size (82 bytes). **Token-2022 mint size depends on the extensions**, so
you must compute it before allocating, or `createAccount` underfunds rent and fails. Also,
**extension-init instructions must run before `initializeMint`** (and metadata fields are written
after, with realloc).

```ts
// Kit: compute size for the chosen extensions before creating the account
import { getMintSize } from '@solana-program/token-2022';
const space = getMintSize([/* ExtensionType.MetadataPointer, ... */]);
// createAccount(space, lamports = rentExempt(space), owner = TOKEN_2022_PROGRAM_ADDRESS)
// then: initialize each extension  ->  initializeMint  ->  (metadata: initialize + update_field)
```
```rust
// Rust: ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::MetadataPointer, ...])
```

Typical transaction order for a mint with extensions:
1. `createAccount` (size = `getMintSize(extensions)`, owner = Token-2022)
2. initialize each **mint extension** (e.g. `initializeMetadataPointer`, `initializeTransferFeeConfig`,
   `initializeTransferHook`, `initializeNonTransferableMint`, …)
3. `initializeMint` (decimals, authorities)
4. for metadata: `initializeTokenMetadata`, then `updateField` per `additional_metadata` (realloc + rent)

## Reading a mint's extensions

```ts
// Kit
import { fetchMint } from '@solana-program/token-2022';
const mint = await fetchMint(rpc, mintAddress);
// mint.data.extensions -> inspect which are present
```
```rust
use spl_token_2022::extension::StateWithExtensions;
use spl_token_2022::state::Mint;
let state = StateWithExtensions::<Mint>::unpack(&account.data)?;
let fee = state.get_extension::<spl_token_2022::extension::transfer_fee::TransferFeeConfig>()?;
```

Or just run [../scripts/check-extensions.ts](../scripts/check-extensions.ts) against any mint.

## Sources
- `@solana/spl-token` (npm) — recommends `@solana-program/token(-2022)` for v2/Kit: https://www.npmjs.com/package/@solana/spl-token
- Token extensions overview: https://solana.com/docs/tokens/extensions
- `spl-token-2022` crate: https://docs.rs/crate/spl-token-2022/latest
