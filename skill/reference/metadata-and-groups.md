# Metadata & Groups

Token-2022 can store metadata **directly on the mint** (no separate Metaplex account) and can
express **collections** via groups/members. Source: https://solana.com/docs/tokens/extensions

## On-mint metadata: `MetadataPointer` + `TokenMetadata`

Two extensions work together:

- **`MetadataPointer`** — tells wallets/indexers *where* this token's metadata lives. When you
  embed metadata in the mint itself, the pointer must point **at the mint's own address**.
- **`TokenMetadata`** — the embedded metadata: `name`, `symbol`, `uri`, an `update_authority`, the
  `mint`, and arbitrary `additional_metadata` key/value fields. Uses the
  `spl-token-metadata-interface` (2026 line: `1.x`). (https://docs.rs/crate/spl-token-metadata-interface/latest)

### Gotchas

1. **Pointer must match.** A `MetadataPointer` pointing somewhere other than the mint (when you
   intended on-mint metadata) lets a third party host spoofable metadata. Point it at the mint.
2. **Variable length → reallocate.** `TokenMetadata` (and especially `additional_metadata`) grows
   the mint account. You must size/realloc the account and fund rent before writing fields. Adding
   a custom field later requires another realloc + rent top-up.
3. **Ordering.** Initialize `MetadataPointer` (and reserve space) as part of mint setup, before
   writing the metadata fields. See account-sizing notes in [clients.md](clients.md).
4. **Metaplex vs Token-2022 metadata.** On-mint metadata avoids a separate Metaplex Token Metadata
   account, but some tooling still expects Metaplex metadata — confirm your target wallets/markets
   read Token-2022 on-mint metadata (most modern wallets do). See
   [compatibility-matrix.md](compatibility-matrix.md).

### Typical setup order

```text
1. allocate mint with space for: MetadataPointer + (reserved) TokenMetadata
2. initialize MetadataPointer(authority, metadata_address = mint)
3. initialize mint (decimals, mint authority, ...)
4. initialize TokenMetadata(name, symbol, uri, update_authority, mint)
5. (optional) update_field for each additional_metadata entry (realloc + rent as needed)
```

## Groups & members (collections)

For "this token belongs to collection X" semantics:

- **`GroupPointer` + `TokenGroup`** on the **group/parent** mint — declares a group and tracks
  `max_size` / current size.
- **`GroupMemberPointer` + `TokenGroupMember`** on each **member** mint — links it to the group.
- Uses `spl-token-group-interface` (2026 line: `0.7.x`). (https://docs.rs/crate/spl-token-group-interface/latest)

### Gotchas

- The group's member count is enforced against `max_size`; design it up front.
- Pointers (like metadata) should reference the correct config account; mismatches break
  indexers' ability to resolve the collection.
- Groups are newer and less universally indexed than Metaplex collections — verify your target
  marketplaces/indexers understand Token-2022 groups before relying on them for discovery.

## When to use what

| Need | Use |
|---|---|
| Name/symbol/image on a fungible or simple token, no Metaplex dep | `MetadataPointer` + `TokenMetadata` |
| Custom on-chain attributes (e.g. tier, region) | `TokenMetadata.additional_metadata` |
| "Collection of tokens" relationship | `GroupPointer`/`TokenGroup` + `GroupMemberPointer`/`TokenGroupMember` |
| Rich NFT tooling / established marketplace support | Consider Metaplex Core/Token Metadata instead — delegate to an NFT skill |

Exact builders and versions: [clients.md](clients.md).
