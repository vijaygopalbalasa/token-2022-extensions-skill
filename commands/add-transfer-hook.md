---
description: Add a Token-2022 transfer hook (e.g. allowlist/KYC gate) to a mint, with the required security checks and client wiring.
---

# /add-transfer-hook

Stand up a transfer hook correctly — program, `ExtraAccountMetaList`, client resolution, and tests.
Read `skill/reference/transfer-hooks.md` first; the `program/` directory is a built, tested example.

## Steps

1. **Confirm it's the right tool.** Hooks are bypassed by many DEXs — they're for permissioned/
   controlled transfers, not enforcing rules during open swaps. Verify with the compatibility
   matrix. If the token must trade freely on DEXs, reconsider.

2. **Define the rule.** What must the hook enforce? (allowlist, denylist, KYC gate, per-transfer
   accounting, fee-to-treasury). Identify the extra accounts the rule needs.

3. **Write/adapt the hook program** (native Rust against `spl-transfer-hook-interface`, or Anchor
   with `#[interface(spl_transfer_hook_interface::execute)]`). Mandatory checks:
   - `transferring` flag on source & destination token accounts,
   - validate the `ExtraAccountMetaList` PDA,
   - verify program-ownership of any account you read,
   - only act for supported mints; no abusable signing authority.
   Start from `program/src/` and change the rule in `process_execute`.

4. **Declare extra accounts** in `InitializeExtraAccountMetaList` (seeds/literals via
   `spl-tlv-account-resolution`). Fund any PDA you create (0-lamport accounts are reaped).

5. **Create the mint** with the `TransferHook` extension pointing at the hook program (sizing +
   init order per `clients.md`), then initialize the meta-list.

6. **Client wiring:** use `createTransferCheckedWithTransferHookInstruction` (v1) or the Kit
   resolver — never hand-build a hooked transfer.

7. **Test for real** (LiteSVM/Surfpool): allowed transfer succeeds, disallowed fails, and a
   transfer missing the extra accounts fails. Mirror `program/e2e/transfer-hook.e2e.ts`.

8. **Review** with `token-extensions-reviewer` before mainnet.
