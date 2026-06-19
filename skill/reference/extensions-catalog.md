# Extensions Catalog

Canonical list of Token-2022 extensions, split by where they live (mint vs token account), with
purpose and the one gotcha that bites people. Source for the full set:
https://solana.com/docs/tokens/extensions

> **Immutability:** mint-level extensions are generally fixed at mint creation and cannot be added
> later. Account-level extensions are added when the token account is created/reallocated.

## Mint-level extensions

| Extension | Purpose | Gotcha |
|---|---|---|
| `TransferFeeConfig` | Fee rate (bps) + max fee + authorities to set/withdraw fees | The **recipient receives `amount − fee`**; the fee is withheld on the destination account (the sender is still debited the full `amount`). Must transfer with `transfer_checked_with_fee`. See [transfer-fees.md](transfer-fees.md). |
| `MintCloseAuthority` | Authority allowed to close the mint and reclaim rent | Mint can only be closed when supply is 0. |
| `TransferHook` | Requires a CPI to your transfer-hook-interface program on every transfer | DEXs/AMMs frequently **bypass** hooks; the Execute discriminator hashes `spl-transfer-hook-interface:execute` (hyphens), not Anchor's namespace. See [transfer-hooks.md](transfer-hooks.md). |
| `DefaultAccountState` | Default `state` for newly created token accounts (e.g. `Frozen`) | New ATAs can be born **Frozen**; escrow/vault/airdrop logic breaks unless it thaws or handles frozen accounts. |
| `NonTransferable` | "Soulbound" — tokens cannot be transferred | Can still be burned/closed; forces `ImmutableOwner` on accounts. |
| `InterestBearingConfig` | Token accrues interest over time | Interest is **display-only** (UI amount); the on-chain `amount` does not change. |
| `PermanentDelegate` | A permanent delegate with unlimited transfer/burn authority over **all** accounts of this mint | Can drain or burn any holder's balance — a clawback feature and a rug vector. Disclose it. |
| `MetadataPointer` | Points to the account holding this token's metadata | Should point at the mint itself when using embedded `TokenMetadata`; a mismatched pointer is spoofable. |
| `TokenMetadata` | Embeds metadata (name/symbol/uri + custom fields) directly in the mint | Variable length — reallocate the mint before writing fields; pointer must match. Uses the token-metadata interface. |
| `GroupPointer` | Points to the account holding token-group config | Pair with `TokenGroup`. |
| `TokenGroup` | Marks the mint as a group (collection parent) | Uses `spl-token-group-interface`. |
| `GroupMemberPointer` | Points to the account holding group-member config | Pair with `TokenGroupMember`. |
| `TokenGroupMember` | Marks the mint as a member of a group | Member count is tracked on the group. |
| `ScaledUiAmount` | Applies an updatable multiplier to the **displayed** UI amount (rebasing/stock-split display) | Stores `multiplier`, `new_multiplier`, `new_multiplier_effective_timestamp`; raw `amount` is unchanged, so integrators reading raw amounts get wrong values. (https://solana.com/docs/tokens/extensions/scaled-ui-amount) |
| `Pausable` | A pause authority can halt mint/burn/transfer | Pausing halts **all** movement; integrators must handle the paused state gracefully. (https://solana.com/docs/tokens/extensions/pausable) |
| `ConfidentialTransferMint` | Enables confidential (encrypted-amount) transfers + auditor config | Depends on the ZK ElGamal Proof Program — **check mainnet availability**. See [confidential-transfers.md](confidential-transfers.md). |
| `ConfidentialTransferFeeConfig` | Confidential transfers + encrypted withheld fees | Same ZK dependency; pairs CT with transfer fees. |
| `ConfidentialMintBurn` | Mint/burn of confidential tokens | Newer; same ZK dependency. |

## Token-account-level extensions

| Extension | Purpose | Gotcha |
|---|---|---|
| `TransferFeeAmount` | Holds withheld transfer fees on the account | Account **cannot be closed** until withheld fees are harvested to the mint. |
| `ImmutableOwner` | Account owner authority can never change | **Auto-set on every ATA** by the ATA program; only matters for manually created accounts. |
| `MemoTransfer` | Requires inbound transfers to include a memo | Senders must prepend a Memo-program instruction or the transfer fails. |
| `CpiGuard` | Blocks privileged token instructions invoked via CPI | When on, programs cannot transfer via direct CPI — must use a delegate flow. |
| `NonTransferableAccount` | Marks an account of a non-transferable mint | Set automatically. |
| `TransferHookAccount` | Marks an account of a transfer-hook mint; carries the `transferring` flag during a transfer | Hook programs **must** check this flag. See [transfer-hooks.md](transfer-hooks.md). |
| `PausableAccount` | Marks an account of a pausable mint | Set automatically. |
| `ConfidentialTransferAccount` | Encrypted pending/available balances for confidential transfers | Added via `reallocate`; can be spammed up to `max_pending_balance_credit_counter`. |
| `ConfidentialTransferFeeAmount` | Encrypted withheld transfer fees | — |

## Reading extensions off a live mint

Don't guess which extensions a mint has — inspect it. Use
[../scripts/check-extensions.ts](../scripts/check-extensions.ts), or in code:

- **JS (Kit):** `fetchMint(...)` from `@solana-program/token-2022` and read `data.extensions`.
- **JS (legacy v1):** `getMint(connection, mint, commitment, TOKEN_2022_PROGRAM_ID)` then
  `getExtensionData(...)` / `getTransferFeeConfig(mint)` etc. from `@solana/spl-token`.
- **Rust:** `StateWithExtensions::<Mint>::unpack(&account.data)` from `spl_token_2022`, then
  `get_extension::<T>()`.

(See [clients.md](clients.md) for exact packages/versions.)
