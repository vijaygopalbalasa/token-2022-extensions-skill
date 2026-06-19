# Testing Token-2022 & transfer-hook programs

Tests run locally and in-process — no validator round-trips. The goal for a hook/extension is to
assert behaviour: allowed transfer succeeds, disallowed transfer **fails**, fee math is exact,
frozen accounts can't move, etc.

## Tools (2026)

| Tool | Lang | Use it for |
|---|---|---|
| **LiteSVM** (`litesvm` crate; npm `litesvm`) | Rust + TS | Fast in-process SVM. Default for client + program tests. Load your built `.so` and the Token-2022 program. npm `1.x` is Kit-native (`@solana/kit`); the last `@solana/web3.js` v1 release is `0.8.0` (what the reference `program/e2e/` pins, since it uses spl-token v1). Pick the line that matches your client. |
| **Mollusk** (`mollusk-svm`, `mollusk-svm-programs-token`) | Rust | Instruction-level unit tests + **CU benchmarking**. Best for tight hook-logic tests. |
| **Surfpool / Surfnet** (`cargo install surfpool`) | any | Integration tests against realistic mainnet state (clone the Token-2022 program + real accounts, time-travel). Best for the full `transfer_checked` → hook CPI path. Default runner under `anchor test` on Anchor 1.x. |
| **Bankrun** (`solana-bankrun`) | TS | Legacy — being replaced by LiteSVM. Prefer LiteSVM for new tests. |

Rust LiteSVM crate version must match your resolved `solana-*` line (e.g. `litesvm 0.9.x` for
solana `~3.1`). Pin against docs.rs.

## Token-2022 program ID to load

```
TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
```

LiteSVM/Mollusk need the Token-2022 program present. Either use the harness's built-in SPL programs
(if the version includes Token-2022) or add it from a local `.so` / cloned account. The reference
[`program/`](../../program) shows the exact wiring it used and the source of the Token-2022 `.so`.

## What to assert for a transfer hook (the reference program's tests)

1. **Setup:** create a Token-2022 mint with the `TransferHook` extension pointing at the built hook
   program; initialize the `ExtraAccountMetaList` PDA; create source + destination token accounts.
2. **Allowed path:** add an entry to the allowlist; `transfer_checked` (with the resolved extra
   accounts) **succeeds**; balances move.
3. **Denied path:** transfer to/from a non-allowlisted party **fails** (the hook rejects it).
4. **Missing-accounts path:** a `transfer_checked` that omits the extra accounts **fails** (proves
   the hook is actually in the path).
5. (If applicable) **`transferring` flag:** calling `Execute` out-of-band (flag unset) is rejected.

These are concrete pass/fail assertions, not "it compiled." See [`program/`](../../program).

## What to assert for transfer fees

- After `transfer_checked_with_fee(amount, fee)`, **recipient balance delta == `amount − fee`**.
- A plain `transfer_checked` on the fee mint **fails**.
- `harvest_withheld_tokens_to_mint` then `withdraw_withheld_tokens_from_mint` moves the expected
  total into the fee vault.

## What to assert for other extensions

- `NonTransferable`: any transfer fails; burn/close still works.
- `DefaultAccountState=Frozen`: a freshly created account is Frozen; transfer fails until thawed.
- `MetadataPointer`+`TokenMetadata`: `fetchMint` returns the expected name/symbol/uri and custom fields.

## Running the reference program's tests

See [`program/README.md`](../../program). The repo's top-level test command builds the hook with
`cargo build-sbf` and runs the LiteSVM suite — and the README states exactly which commands were
run and their result, so nothing here is a claim you can't reproduce.

> Each test should assert a concrete behaviour (a specific pass/fail), not just that code runs.
