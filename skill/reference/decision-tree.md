# Decision Tree — which extensions for which token

> Start here. Most extensions are set **at mint creation and cannot be added later**, so the
> wrong choice is a re-launch. Decide deliberately, then confirm against
> [compatibility-matrix.md](compatibility-matrix.md) before writing code.

## Step 0 — do you even need Token-2022?

Use **base SPL Token** (program `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`) when you only need a
plain fungible token with no special transfer behaviour. Base Token has the widest
wallet/DEX/CEX support and zero surprises. Reach for **Token-2022**
(`TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`) only when you need one of the behaviours below.
Token-2022 is not "the new default you should always pick" — it is a toolbox you opt into per need.

## Step 1 — pick by goal

| Your goal | Extensions to consider | Watch out for |
|---|---|---|
| **On-mint metadata** (name/symbol/uri without Metaplex) | `MetadataPointer` + `TokenMetadata` | pointer must point at the mint itself; account grows when you add fields → reallocate ([metadata-and-groups.md](metadata-and-groups.md)) |
| **Protocol fee on every transfer** | `TransferFeeConfig` (+ `TransferFeeAmount` on accounts) | recipient gets `amount − fee`; must use `transfer_checked_with_fee`; breaks naive integrators ([transfer-fees.md](transfer-fees.md)) |
| **Allowlist / KYC-gated / geo-gated transfers** | `TransferHook` (+ your hook program) | many DEXs **bypass hooks**; needs an `ExtraAccountMetaList`; security-sensitive ([transfer-hooks.md](transfer-hooks.md)) |
| **Soulbound / non-transferable credential, badge, ticket** | `NonTransferable` | can still be burned/closed; pairs with auto `ImmutableOwner` |
| **Recoverable / clawback-able token** (gov, compliance) | `PermanentDelegate` | delegate can drain **any** holder — a rug vector; disclose it ([security.md](security.md)) |
| **New accounts start frozen until approved** | `DefaultAccountState = Frozen` + a freeze authority flow | every new ATA is born Frozen — vaults/escrow break unless they thaw |
| **Stablecoin / regulated token that may need to halt** | `Pausable` (+ usually `PermanentDelegate`, `DefaultAccountState`) | pausing halts ALL movement; integrators must handle paused state |
| **Display-only yield (UI shows accruing balance)** | `InterestBearingConfig` | interest is **display only**; on-chain `amount` does NOT change |
| **Rebasing / stock-split display (multiplier on UI)** | `ScaledUiAmount` | raw `amount` unchanged; integrators reading raw amounts get wrong numbers |
| **Force inbound transfers to carry a memo** (exchanges, accounting) | `MemoTransfer` (account-level) | senders must add a Memo instruction or the transfer fails |
| **Block privileged token actions via CPI** (wallet safety) | `CpiGuard` (account-level) | programs can't transfer via direct CPI — must use the delegate flow |
| **Collection / grouping of tokens** | `GroupPointer` + `TokenGroup`, `GroupMemberPointer` + `TokenGroupMember` | uses the token-group interface |
| **Private balances / amounts** | `ConfidentialTransfer*` | ⚠️ depends on the ZK ElGamal Proof Program — **check mainnet availability first** ([confidential-transfers.md](confidential-transfers.md)) |
| **Let the mint be closed to reclaim rent** | `MintCloseAuthority` | mint can only close when supply is 0 |

(Extension behaviours and gotchas: https://solana.com/docs/tokens/extensions)

## Step 2 — worked profiles (copy-paste starting points)

These are opinionated defaults. Always re-check [compatibility-matrix.md](compatibility-matrix.md).

### Loyalty / points token (off-chain redeemable, not traded)
- `MetadataPointer` + `TokenMetadata` (branding), optionally `NonTransferable` (if points must not be sold).
- Skip transfer fees/hooks — keep it simple; it never needs to trade on a DEX.

### Memecoin meant to trade on DEXs
- `MetadataPointer` + `TokenMetadata` only. **Avoid** transfer fees and transfer hooks unless you
  have confirmed every target DEX honors them — otherwise pools will misprice or refuse the token.
- If you want a "tax", confirm the specific AMM supports `TransferFeeConfig` in pools first.

### Compliant / regulated token (RWA, tokenized equity, regulated stablecoin)
- `TransferHook` (allowlist / sanctions screening) + `PermanentDelegate` (clawback) +
  `DefaultAccountState = Frozen` (approve before use) + `Pausable` (emergency halt) +
  `MetadataPointer`+`TokenMetadata`.
- This is the heaviest profile and the one most likely to hit venue-compatibility limits — it is
  usually meant for a controlled/permissioned distribution, not open DEX trading.

### Soulbound credential / proof-of-attendance / badge
- `NonTransferable` + `MetadataPointer` + `TokenMetadata`. Optionally `MintCloseAuthority`.

### Yield-display or rebasing stablecoin
- `InterestBearingConfig` (display-only accrual) **or** `ScaledUiAmount` (multiplier) +
  metadata. Document loudly that raw `amount` ≠ displayed amount so integrators don't misaccount.

## Step 3 — combination rules & conflicts

- **Compatibility shrinks as you add extensions.** Each non-trivial extension (hook, fee,
  confidential, non-transferable) narrows where the token works. Add the *fewest* extensions that
  meet the goal.
- **`NonTransferable` + `TransferFeeConfig`/`TransferHook` is contradictory** — non-transferable
  tokens don't transfer, so transfer-time extensions are moot.
- **`TransferHook` + `TransferFeeConfig` together** is allowed but doubles your integration burden
  (callers must add hook extra-accounts *and* use the fee-aware transfer instruction).
- **`PermanentDelegate`** + open DEX trading = users' tokens can be clawed back; this is fine for
  compliance tokens, hostile for "community" tokens. Choose intentionally.
- **`ConfidentialTransfer*`** changes the whole UX and currently depends on a network feature that
  may be disabled — never design a launch around it without verifying availability today.

## Step 4 — before you commit

1. List the venues the token must live on (which wallets, DEXs, CEXs).
2. Open [compatibility-matrix.md](compatibility-matrix.md) and confirm each chosen extension is
   supported there. Drop any extension that isn't.
3. Read the per-extension reference for each survivor.
4. Generate code from [clients.md](clients.md); run [security.md](security.md)'s checklist.
