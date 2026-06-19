# Evaluations

Concrete scenarios to verify the skill actually changes behaviour for the better. Each has a
prompt, what a correct response must do, and a fail signal (what a no-skill / "AI slop" answer
typically gets wrong). Run by giving an agent the prompt with the skill installed.

> These are behavioural evals (judged by the criteria), plus two that are mechanically checkable
> via the repo's own tests/scripts. The skill was written by first noting the no-skill failure
> modes below, then writing the minimal guidance to fix them.

## E1 — Extension choice (decision quality)
**Prompt:** "I want to launch a community memecoin on Solana that trades on Jupiter and Raydium.
Should I use Token-2022 with a 5% transfer fee and a transfer hook?"
**Correct must:** warn that transfer hooks are commonly **bypassed by DEXs** and fee tokens have
uneven DEX/CEX support; recommend the **smallest** set (metadata only) for a freely-traded token;
point to `compatibility-matrix.md`; tell the user to verify on devnet first.
**Fail signal:** enthusiastically adds hook + fee with no compatibility warning.

## E2 — ATA derivation correctness (mechanical)
**Prompt:** "Get the associated token account address for this Token-2022 mint and owner in TS."
**Correct must:** pass `TOKEN_2022_PROGRAM_ID` as the token-program argument to
`getAssociatedTokenAddressSync`.
**Fail signal:** omits the program id (derives the wrong ATA). This is encoded in `rules/token-2022.md`.

## E3 — Fee accounting (avoids a real money bug)
**Prompt:** "My vault transfers `amount` of a transfer-fee token in and credits the user `amount`.
Is that right?"
**Correct must:** explain the recipient receives `amount − fee`, so the vault over-credits; fix via
net accounting or balance delta; mention `transfer_checked_with_fee` is required. See
`transfer-fees.md`.
**Fail signal:** says it's fine / doesn't mention the fee deduction.

## E4 — Transfer hook security (program correctness)
**Prompt:** "Write a Token-2022 transfer hook that enforces an allowlist."
**Correct must:** check the `transferring` flag on the token accounts, validate the
`ExtraAccountMetaList` PDA, verify ownership of the allowlist account, and resolve extra accounts
client-side. The repo's `program/` is the reference; its tests assert allow/deny/missing-accounts.
**Fail signal:** a hook that trusts inputs / skips the transferring check; a client that hand-builds
the transfer without extra accounts.
**Mechanical check:** `cd program && cargo build-sbf && cargo test --test logic` and
`cd program/e2e && npm i && npm run fixtures && npm test` → all green (10 + 10).

## E5 — Currency / no stale facts (anti-hallucination)
**Prompt:** "Show me how to add confidential transfers to my mint for mainnet today."
**Correct must:** flag that confidential transfers depend on the ZK ElGamal Proof Program, which
was **disabled on mainnet** (June 2025 incident) and must be verified as live **today** before
relying on it; not present it as a working mainnet feature. See `confidential-transfers.md`.
**Fail signal:** confidently provides mainnet confidential-transfer code with no availability caveat.

## E6 — Inspect a real mint (mechanical)
**Prompt:** "What extensions does PYUSD (2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo) have and
what should I watch out for?"
**Correct must:** use `skill/scripts/check-extensions.ts` (or `getMint`+`getExtensionTypes`) rather
than guess; surface PermanentDelegate / TransferFeeConfig / TransferHook risks.
**Mechanical check:** `cd skill/scripts && npm i && npx tsx check-extensions.ts 2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo`
prints 8 extensions and ≥5 risk flags (verified 2026-06-19).
