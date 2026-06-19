---
name: token-2022-extensions
description: Token-2022 (SPL Token Extensions) expert for Solana — choose, combine, wire, and ship token extensions safely. Use when the user mentions Token-2022, token extensions, transfer hook, transfer fee, confidential transfer, permanent delegate, non-transferable or soulbound tokens, interest-bearing tokens, default account state, mint close authority, metadata pointer / token metadata, token groups, scaled UI amount, or pausable tokens — or asks which extensions to use, how to create a mint with extensions, why a token breaks on a DEX/wallet/CEX, fee accounting (gross vs net), or how to test an extension or transfer-hook program.
user-invocable: true
license: MIT
compatibility: [claude-code, codex]
metadata:
  version: "2026-06"
  program: Token-2022 (TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb)
  last-stack-review: "2026-06-19"
---

# Token-2022 / SPL Token Extensions Skill

> Extends core Solana development. For base program/Anchor/Pinocchio/frontend patterns,
> delegate to [solana-dev-skill](https://github.com/solana-foundation/solana-dev-skill).
> This skill owns one thing well: **the Token Extensions discipline** — deciding *which*
> extensions to use, *wiring* them correctly (especially transfer hooks and fees), and
> *not shipping a token that breaks the moment it touches a DEX, wallet, or exchange.*

## What this skill is for

Use this skill when the task involves SPL **Token-2022** (a.k.a. Token Extensions):

- **Designing a token** — picking the right extension set for a use case (loyalty points,
  stablecoin, RWA/equity, memecoin, soulbound credential, gated/compliant token).
- **Creating mints & accounts** with extensions (correct ordering, sizing, ATA derivation).
- **Transfer hooks** — writing the hook program, the `ExtraAccountMetaList`, resolving extra
  accounts client-side, and the security checks that stop hooks from being a footgun.
- **Transfer fees** — fee config, the withheld-fee harvest/withdraw flow, and the
  gross-vs-net accounting that breaks naive integrators.
- **Compatibility triage** — answering "will this token work on Jupiter / Phantom / Raydium /
  a CEX?" before launch, not after.
- **Security review** of an extension config (permanent delegate, default-frozen, pausable…).
- **Testing** extension and hook programs (LiteSVM / Mollusk / Surfpool).

If the user only needs a **plain** SPL token with no special behaviour, say so and point them
at base SPL Token — Token-2022 is not free complexity.

## First rule: extensions are immutable at the mint level

Most extensions must be enabled **at mint creation** and **cannot be added later**. The wrong
extension set is a re-launch, not a patch. So the decision step is the most important step —
start at the decision tree, not at the code.

## How to use this skill (operating procedure)

1. **Classify the goal.** What is the token *for*? → [reference/decision-tree.md](reference/decision-tree.md)
2. **Confirm compatibility** with the venues the token must live on (DEX/wallet/CEX) *before*
   committing to an extension. → [reference/compatibility-matrix.md](reference/compatibility-matrix.md)
3. **Read the focused reference** for each chosen extension (hooks, fees, etc.) below.
4. **Generate code** using the current 2026 stack in [reference/clients.md](reference/clients.md)
   and obey the repo's `rules/token-2022.md` coding rules (style + safety).
5. **Inspect & test.** Run [scripts/check-extensions.ts](scripts/check-extensions.ts) against the
   resulting mint; write tests per [reference/testing.md](reference/testing.md).
6. **Security pass** before mainnet → [reference/security.md](reference/security.md).

## Default stack (2026, verify before pinning)

- **JS/TS (new builds):** `@solana/kit` + `@solana-program/token-2022` (Codama-generated, tree-shakable).
- **JS/TS (legacy v1 codebases only):** `@solana/spl-token`.
- **Rust:** `spl-token-2022` + `spl-associated-token-account`; hooks via `spl-transfer-hook-interface`
  + `spl-tlv-account-resolution`. Anchor **1.x** (`anchor-spl`), TS package `@anchor-lang/core`.
- **Program ID:** `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`.
- Exact versions and the "why" live in [reference/clients.md](reference/clients.md). Pin against
  docs.rs / npm at build time — this file states intent, not frozen version numbers.

## Progressive disclosure (read only what the task needs)

> Do **not** pre-load these. Open the one file the current question maps to.

### Decide
- [reference/decision-tree.md](reference/decision-tree.md) — use-case → recommended extension set (start here)
- [reference/extensions-catalog.md](reference/extensions-catalog.md) — every extension, mint vs account, one-line purpose + gotcha
- [reference/compatibility-matrix.md](reference/compatibility-matrix.md) — DEX / wallet / CEX support; "will it work?"

### Wire (per extension)
- [reference/transfer-hooks.md](reference/transfer-hooks.md) — hook interface, `ExtraAccountMetaList`, account resolution, hook security
- [reference/transfer-fees.md](reference/transfer-fees.md) — fee config, withheld harvest/withdraw, `transfer_checked_with_fee`, gross/net
- [reference/metadata-and-groups.md](reference/metadata-and-groups.md) — metadata pointer + on-mint metadata, token groups/members
- [reference/confidential-transfers.md](reference/confidential-transfers.md) — confidential transfers + **current mainnet availability status**

### Build & ship
- [reference/clients.md](reference/clients.md) — create mints/accounts with extensions (Kit + Rust + legacy), versions, ATA + sizing rules
- [reference/security.md](reference/security.md) — extension footguns & a pre-launch checklist
- [reference/testing.md](reference/testing.md) — LiteSVM / Mollusk / Surfpool for extensions & hooks
- [reference/resources.md](reference/resources.md) — curated official sources

## Task routing

| User asks about… | Read |
|---|---|
| "which extensions should I use / token design" | reference/decision-tree.md |
| "what extensions exist / what does X do" | reference/extensions-catalog.md |
| "will it work on Jupiter/Phantom/a CEX" | reference/compatibility-matrix.md |
| transfer hook, allowlist/KYC-gated transfer, ExtraAccountMetaList | reference/transfer-hooks.md |
| transfer fee, withheld fees, fee on transfer, gross vs net | reference/transfer-fees.md |
| metadata, name/symbol/uri on mint, collections/groups | reference/metadata-and-groups.md |
| confidential / private balances, ZK proof program | reference/confidential-transfers.md |
| create a mint with extensions, code in JS/Rust, ATA, sizing | reference/clients.md |
| permanent delegate, default frozen, pausable, "is this safe" | reference/security.md |
| how to test a hook / extension program | reference/testing.md |
| docs / official links | reference/resources.md |

## Bundled (optional) — see repo root

- **agents/** — `token-extensions-engineer` (build), `token-extensions-reviewer` (security pass)
- **commands/** — `/design-token`, `/add-transfer-hook`, `/audit-token-2022`
- **rules/** — `token-2022.md` auto-loads coding rules on Rust/TS token files
- **program/** — a working, tested **allowlist transfer-hook** reference program

## Operating principles

- **Decision before code.** Extensions are mostly immutable; choosing wrong is expensive.
- **Compatibility is a feature, not an afterthought.** A token nobody can trade is a dead token.
- **Never assume gross == net.** Fee mints deliver `amount − fee`; account for it.
- **Hooks are not honored everywhere.** Many DEXs bypass transfer hooks — design accordingly.
- **State current facts, not frozen numbers.** Versions and network availability (e.g.
  confidential transfers) change; cite and verify, don't hardcode stale claims.
