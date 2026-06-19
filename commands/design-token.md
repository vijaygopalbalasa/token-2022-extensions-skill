---
description: Choose the right Token-2022 extension set for a token, with compatibility and security baked in.
---

# /design-token

Turn a token goal into a concrete, safe extension set + a build plan. Use before writing any mint code.

## Steps

1. **Clarify the goal** (ask only what's missing):
   - What is the token for? (loyalty/points, memecoin, stablecoin, RWA/equity, soulbound, utility)
   - Must it trade on DEXs / be deposited to CEXs / live in specific wallets?
   - Any control needs? (clawback, freeze/approve, pause, fees, on-chain metadata)

2. **Map goal → extensions** using `skill/reference/decision-tree.md`. Pick the **smallest** set
   that meets the goal. Note that most mint extensions are immutable after creation.

3. **Compatibility gate** with `skill/reference/compatibility-matrix.md`. Drop any extension a
   required venue doesn't support (especially transfer hooks/fees on DEX-traded tokens). State the
   tradeoffs explicitly.

4. **Security pass** with `skill/reference/security.md` for the chosen set (permanent delegate
   disclosure, default-frozen flows, authority custody).

5. **Output a token spec:**
   - Final extension list (mint-level vs account-level) + one-line reason each
   - Authorities and who should hold them (prefer multisig; revoke unneeded ones)
   - Compatibility note (safe venues / verify-first venues)
   - Build plan: mint sizing + init order, ATA notes, and which `reference/*.md` to follow
   - A devnet compatibility test checklist before mainnet

Hand the spec to `token-extensions-engineer` to implement, then `token-extensions-reviewer` before launch.
