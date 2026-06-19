# Confidential Transfers

Confidential transfers encrypt token **amounts** on-chain (balances/transfer amounts are hidden;
addresses are still public) using ElGamal encryption and zero-knowledge proofs verified by the
**ZK ElGamal Proof Program**.

## ⚠️ Availability first — do not design a launch around this without checking

As of this skill's last review (**2026-06**), confidential transfers were **not usable on Solana
mainnet**: the ZK ElGamal Proof Program was **disabled in June 2025** after a soundness
vulnerability that could let an attacker forge proofs (risking unauthorized mint/balance
manipulation). Re-enablement was pending audits.

- Incident post-mortem: https://solana.com/news/post-mortem-june-25-2025
- Tracking: https://github.com/solana-program/token-2022/issues/657
- **Verify the current status today** before building on it:
  https://solana.com/docs/tokens/extensions/confidential-transfer

The official Solana dev guidance during this window was that confidential transfers work only on a
dedicated ZK test surface (a "ZK edge" surfnet), not mainnet. **If your product depends on
confidential transfers, confirm the proof program is live on mainnet right now — treat any "it's
back" claim as something to verify against the docs/issue above, not assume.**

This is exactly the kind of fact that rots: this file states the situation as of 2026-06 and tells
you where to check, rather than hardcoding "it works" or "it's broken" forever.

## What it is (when available)

Extensions involved:
- `ConfidentialTransferMint` (mint) — enables CT and stores auditor/encryption config.
- `ConfidentialTransferAccount` (account) — added via `reallocate`; holds encrypted **pending** and
  **available** balances.
- `ConfidentialTransferFeeConfig` / `ConfidentialTransferFeeAmount` — CT combined with transfer fees.
- `ConfidentialMintBurn` — confidential mint/burn.

Flow (conceptually): deposit public tokens → encrypted **pending** balance → `applyPendingBalance`
to move into **available** → confidential transfer to another CT account → withdraw back to public.
Each step submits ZK proofs (range proofs, equality proofs) verified by the proof program.

### Gotchas

1. **Network dependency** (above) dominates everything else.
2. **Pending-balance spam.** An attacker can credit many tiny pending amounts; accounts have a
   `max_pending_balance_credit_counter`, and users must `applyPendingBalance` to consolidate.
3. **Not DEX/wallet-supported.** Even when the proof program is live, mainstream wallets and DEXs
   generally do not support CT UX — see [compatibility-matrix.md](compatibility-matrix.md).
4. **Heavier client stack.** Requires the ZK SDK (`solana-zk-sdk`) and proof generation; key
   management (ElGamal + AES keys derived from the owner) is extra surface area.
5. **Auditor key.** CT mints can be configured with an auditor ElGamal key so a designated party
   can decrypt amounts — decide your compliance posture up front.

## Practical recommendation

For almost all 2026 builds: **do not** rely on confidential transfers unless (a) you've confirmed
the proof program is enabled on mainnet today, and (b) your wallets/venues support the CT flow.
If you only need "private-ish" amounts for a controlled set of parties, consider whether a
permissioned design (allowlist hook + off-chain confidentiality) meets the requirement with far
less risk. Exact crate/SDK versions, when you do build CT, belong in [clients.md](clients.md) and
must be re-verified against the proof program's current status.
