# transfer-hook-allowlist — reference program

A minimal, production-shaped **Token-2022 transfer hook** that gates every transfer on a
destination-owner **allowlist**. It is the worked example behind
[`skill/reference/transfer-hooks.md`](../skill/reference/transfer-hooks.md): real interface
plumbing, the required security checks, and real tests — no pseudocode.

- **Native Rust** (no Anchor) against the `spl-transfer-hook-interface` 2.x crates, so it tracks
  the installed Solana 3.x `cargo build-sbf` toolchain with no framework version friction.
- Implements the SPL transfer-hook interface (`Execute`, `InitializeExtraAccountMetaList`,
  `UpdateExtraAccountMetaList` is intentionally rejected) plus two admin instructions
  (`InitializeWhiteList`, `AddToWhiteList`) and a client-friendly custom
  `InitializeExtraAccountMetaList` (tag 2).
- Security checks mirror the canonical guidance: the hook verifies the `transferring` flag on the
  token accounts, validates the `ExtraAccountMetaList` PDA, and enforces program ownership of the
  allowlist account before reading it.

> This is a teaching/reference program. Before any mainnet use, audit it and read
> [`skill/reference/security.md`](../skill/reference/security.md). It is MIT-licensed, has no
> hidden CPI and no opaque behaviour.

## Layout

```
program/
├── Cargo.toml
├── src/
│   ├── lib.rs          # modules
│   ├── entrypoint.rs   # SBF entrypoint
│   ├── instruction.rs  # custom admin instructions (tags 0/1/2)
│   ├── processor.rs    # interface handlers + allowlist enforcement
│   └── state.rs        # fixed-capacity allowlist (no borsh; arrayref offsets)
├── tests/
│   └── logic.rs        # host unit tests (cargo test)
└── e2e/                # end-to-end test in LiteSVM (real Token-2022 transfer)
    ├── transfer-hook.e2e.ts
    ├── fetch-fixtures.sh
    └── package.json
```

## Build

```bash
cargo build-sbf            # produces target/deploy/transfer_hook_allowlist.so
```

## Tests

### 1. Unit tests (host, no network) — allowlist logic + instruction parsing

```bash
cargo test --test logic
```

These cover allowlist membership, capacity, dedup, authority storage, account-size validation,
and instruction pack/unpack round-trips.

### 2. End-to-end (LiteSVM, real Token-2022 program)

Loads the **actual** Token-2022 program (dumped from mainnet) and the built hook into LiteSVM and
drives a real `transferChecked` through the hook:

```bash
cargo build-sbf                       # build the hook .so first
cd e2e
npm install
npm run fixtures                      # dumps Token-2022 from mainnet (needs network + Solana CLI)
npm test
```

The e2e asserts:
1. transfer to an **allowlisted** destination **succeeds** (and balances move: dest +100, src −100);
2. transfer to a **non-allowlisted** destination **fails** (the hook rejects it);
3. transfer that **omits** the hook's extra accounts **fails**.

## Verified results (last run 2026-06-19, on this machine)

Toolchain: `rustc 1.94.1`, `solana-cli 3.1.12` (`cargo build-sbf` platform-tools v1.52),
Node `v20.19.4`, `litesvm@0.8.0`, `@solana/spl-token@0.4.14`.

- `cargo build-sbf` → `target/deploy/transfer_hook_allowlist.so` (~99 KB), exit 0.
- `cargo test --test logic` → **10 passed; 0 failed**.
- `npm test` (e2e) → **10 checks passed** (the three behaviours above + setup assertions).

Program id of the local build keypair:
`GcFyb4FFissxhZyPx7CSTX62nC4U7UepG6Ed2x7c83mR` (regenerated per machine on first build).

> Everything stated here is reproducible with the commands above. The e2e is a genuine transfer
> through the mainnet Token-2022 program — nothing about the token program is mocked.
