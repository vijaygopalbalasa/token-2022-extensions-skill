# Transfer Hooks

A transfer hook makes Token-2022 CPI into **your** program on every transfer of the mint, so you
can enforce custom logic (allowlist, KYC gate, royalty, per-transfer accounting). It is the most
powerful and most footgun-prone extension.

> Before building a hook, read [compatibility-matrix.md](compatibility-matrix.md): **many DEXs
> bypass hooks.** A hook is reliable for permissioned/controlled transfers, not for enforcing
> rules during open DEX swaps.

## How it works (the moving parts)

1. The **mint** has the `TransferHook` extension, storing your hook **program ID**.
2. Your **hook program** implements the `spl-transfer-hook-interface`.
3. For each mint, your program owns an **`ExtraAccountMetaList`** account (a PDA) that declares any
   extra accounts your hook needs. Token-2022 reads it to know what to pass in.
4. On `transfer_checked`, the **caller** must include those extra accounts. Token-2022 then CPIs
   your program's `Execute` with `(source, mint, destination, owner, extra_account_meta_list, …extras)`.
5. During this CPI, Token-2022 sets a `transferring` flag on the `TransferHookAccount` of the
   involved token accounts. Your hook **must** verify it.

## The interface facts (pin to the crate)

- Crate: **`spl-transfer-hook-interface`** (2026 line: `2.x`), with `spl-tlv-account-resolution`
  (`0.11.x`) for the `ExtraAccountMetaList`. (https://docs.rs/crate/spl-transfer-hook-interface/latest)
- The `Execute` instruction discriminator is the SPL-discriminator hash of the string
  **`spl-transfer-hook-interface:execute`** (hyphens — it matches the crate name; verified against
  the interface crate's `#[discriminator_hash_input(...)]`) — **not** Anchor's `global:<name>`
  namespace. If you
  use Anchor, you must annotate the handler with `#[interface(spl_transfer_hook_interface::execute)]`
  (Anchor 0.30+) or the instruction is rejected as invalid.
- `ExtraAccountMetaList` PDA seeds: **`[b"extra-account-metas", mint_pubkey]`**, derived under your
  **hook program's** ID.
- Instruction set on the interface: `Execute { amount }`, `InitializeExtraAccountMetaList`,
  `UpdateExtraAccountMetaList`.

> ⚠️ **Always copy the exact `ExecuteInstruction` field layout and the Execute account ordering
> from the `spl-transfer-hook-interface` crate version you build against.** Do not hand-transcribe
> byte offsets from any guide (including this one) — the crate is the source of truth.

## Security model — the checks a hook MUST do

From the Token-2022 security write-up (https://neodyme.io/en/blog/token-2022/):

1. **Verify the mint.** Only run logic for mints your hook supports. Otherwise an attacker can
   point any mint's hook at your program and touch your PDAs.
2. **Check the `transferring` flag** on the `TransferHookAccount`s — your hook should only act
   during a real transfer, never when called out-of-band.
3. **Validate token accounts belong to the expected mint.**
4. **Do not grant the hook abusable authority.** A hook must not be able to perform arbitrary
   signed CPI or re-enter the token program. Keep its powers minimal.
5. (Tradeoff) You *can* require the caller to be Token-2022, but that **blocks legitimate
   program-to-program CPI transfers** — decide consciously.

A hook that skips checks 1–3 is the classic Token-2022 vulnerability.

## Native Rust skeleton (interface-level)

The working, compiled, and tested reference implementation lives in
[`program/`](../../program) (an allowlist hook). Skeleton shape:

```rust
use spl_transfer_hook_interface::instruction::TransferHookInstruction;

pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    match TransferHookInstruction::unpack(data)? {
        TransferHookInstruction::Execute { amount } => {
            // accounts (fixed order): source, mint, destination, owner/authority,
            //                         extra_account_meta_list PDA, ...resolved extras
            // 1) check the source/destination TransferHookAccount `transferring` flag
            // 2) verify the mint is supported
            // 3) run your rule (e.g. allowlist lookup on a resolved extra account)
            process_execute(program_id, accounts, amount)
        }
        TransferHookInstruction::InitializeExtraAccountMetaList { .. } => {
            // write the ExtraAccountMetaList PDA describing your extra accounts
        }
        TransferHookInstruction::UpdateExtraAccountMetaList { .. } => { /* ... */ }
    }
}
```

See [`program/src/lib.rs`](../../program) for the full, building version with the allowlist logic,
the `ExtraAccountMetaList` initialization via `spl-tlv-account-resolution`, and the security checks.

## Client side — invoking a transfer through a hook

The caller must resolve and attach the extra accounts. Don't build the `transfer_checked` accounts
by hand:

- **JS (Kit):** use the resolver helpers from `@solana-program/token-2022` to add the extra
  account metas for the hook before sending `transferChecked`.
- **JS (legacy v1):** `createTransferCheckedWithTransferHookInstruction(...)` from
  `@solana/spl-token` resolves the `ExtraAccountMetaList` and appends the accounts.
- **Rust (offchain):** resolve via `spl-transfer-hook-interface`'s offchain helpers
  (`resolve_extra_account_metas`) before submitting.

If you forget the extra accounts, the transfer fails — that is the hook doing its job, not a bug.

## Testing a hook (real tests, see [testing.md](testing.md))

- Unit-test the hook program's `Execute` logic in isolation with **LiteSVM** or **Mollusk**
  (load both your built `.so` and the Token-2022 program).
- End-to-end (`transfer_checked` → Token-2022 → hook CPI with extra accounts) with **Surfpool**
  or LiteSVM, asserting: an allowed transfer succeeds, a disallowed transfer **fails**, and a
  transfer missing the extra accounts fails.
- The reference [`program/`](../../program) ships exactly these tests.
