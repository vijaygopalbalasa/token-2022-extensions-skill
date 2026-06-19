---
description: Audit a Token-2022 mint (and its transfer hook, if any) for security and compatibility risks before integrating or launching.
---

# /audit-token-2022

Given a mint address (or your own pre-launch config), produce a severity-ordered findings report.
Backed by `skill/reference/security.md` and `compatibility-matrix.md`.

## Steps

1. **Enumerate extensions.** Run `skill/scripts/check-extensions.ts <MINT> [--url <RPC>]` (or
   `getMint` + `getExtensionTypes`). Never assume which extensions are present.

2. **Authority audit.** For every authority (mint, freeze, permanent delegate, pause, transfer-fee
   config, withdraw-withheld, metadata update): identify the holder, whether it's a multisig,
   whether it's disclosed, and whether unneeded authorities are revoked.

3. **Flag high-severity behaviours:**
   - PermanentDelegate → holders can be drained/clawed back (intended? disclosed?)
   - DefaultAccountState=Frozen → do downstream flows thaw accounts?
   - TransferFeeConfig → net accounting + `transfer_checked_with_fee` used?
   - Pausable → pause authority secured; integrators handle paused state?
   - TransferHook → review the hook program (transferring-flag check, PDA validation, ownership
     checks, minimal authority; remember DEX bypass).

4. **Quieter flags:** metadata-pointer mismatch, scaled-ui/interest-bearing raw-vs-display amount,
   confidential-transfer network availability, mint-close-authority reuse.

5. **Compatibility verdict.** Will this token work on the user's target wallets/DEXs/CEXs? Mark
   anything that needs venue verification.

6. **Report:** findings ordered Critical→Info, each with concrete impact + fix, then a go/no-go and
   the `security.md` pre-launch checklist marked pass/fail/n-a.

Only assert what the on-chain data supports. No theoretical findings.
