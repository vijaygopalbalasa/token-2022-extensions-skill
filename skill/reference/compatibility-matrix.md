# Compatibility Matrix — will my token actually work?

The most common Token-2022 failure is shipping a token that a DEX won't pool, a wallet won't
display, or a CEX won't credit. Check this **before** you launch, because mint extensions are
immutable.

> ⚠️ **Read this first.** Per-venue support changes constantly. The directional state below is
> accurate as of 2026-06 but the exact, current behaviour of any specific DEX/wallet/CEX is
> **the thing you must verify yourself before mainnet** — by testing the actual flow, not by
> trusting a table. Treat anything not marked "confirmed" as "verify".

## The two extensions that cause almost all problems

1. **`TransferHook`** — requires the caller to add extra accounts and CPI into your program on
   every transfer. Many AMMs/DEXs do **not** do this and instead **bypass or refuse** hooked
   tokens. A hook that enforces an allowlist is meaningless if the DEX swaps around it. Assume
   hooks are **not** honored on open DEX trading unless a specific venue documents support.
2. **`TransferFeeConfig`** — recipients receive `amount − fee`. Venues that assume gross == net
   (naive AMMs, some CEX deposit crediting) will misprice or miscredit. Fee support in AMM pools
   exists on some venues but is not universal.

## Directional support (2026-06 — verify per venue)

| Surface | Metadata | Transfer Fee | Transfer Hook | Confidential | Non-Transferable |
|---|---|---|---|---|---|
| **Wallets** (Phantom, Solflare, Backpack) | Displayed | Generally handled | Partial | Network-disabled (see below) | Shown; transfer blocked |
| **DEXs/AMMs** (Jupiter, Raydium, Orca, Meteora) | OK | Some pools support; not universal | **Commonly bypassed / unsupported** | Unsupported | Can't trade (by design) |
| **CEX deposits** | OK | **Risky** (gross/net mismatch) | **Risky** | Unsupported | Rejected |

Notes & sources:
- Major wallets render Token-2022 metadata and handle fee tokens; per-extension UI nuances
  (scaled-UI-amount display, pausable status banners) vary by wallet and are **not uniformly
  confirmed** — verify in the actual wallet. (Phantom Token-2022 support: https://help.phantom.com/hc/en-us/articles/44063915243283)
- Transfer hooks being bypassed/limited at the point of trade is a known ecosystem gap; confirm
  for each specific DEX before relying on hook enforcement during swaps. (Directional; verify per DEX.)
- **Confidential transfers are effectively unavailable on mainnet** as of this review — the ZK
  ElGamal Proof Program was disabled in June 2025 after a soundness bug, and re-enablement is
  pending. So no wallet/DEX can support confidential transfers on mainnet right now regardless of
  their own code. See [confidential-transfers.md](confidential-transfers.md) and verify current
  status: https://solana.com/docs/tokens/extensions/confidential-transfer

## Decision rule

- **Token must trade freely on DEXs / be deposited to CEXs** → use the **smallest** extension set:
  `MetadataPointer` + `TokenMetadata` only. Avoid hooks and fees unless every target venue
  confirms support.
- **Token is permissioned / controlled distribution** (RWA, compliance, internal) → heavier
  extensions (hook, permanent delegate, default-frozen, pausable) are fine, because you control
  where it moves and you are not relying on open DEX liquidity.
- **Anything in between** → list venues, test the actual deposit/swap path on devnet (or with
  Surfpool against cloned mainnet accounts, see [testing.md](testing.md)) before committing.

## A pre-launch compatibility check (do this, don't assume)

1. Create the mint with your chosen extensions on **devnet**.
2. Try the real flows: add liquidity / swap on each target DEX; deposit to a test wallet on each
   target wallet app; if a CEX is in scope, confirm their Token-2022 + extension policy in writing.
3. For fee tokens: verify the venue accounts for net-received amount, not gross.
4. For hook tokens: verify the venue actually adds your `ExtraAccountMetaList` accounts to the
   transfer — if it doesn't, your hook isn't running.
5. Only after the real flows pass, replicate the mint config on mainnet.
