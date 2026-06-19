---
globs: ["**/*.rs", "**/*.ts", "**/*.tsx"]
exclude: ["**/target/**", "**/node_modules/**", "**/dist/**"]
---

# Token-2022 coding rules

Auto-loaded on Rust/TS files. These prevent the most common Token-2022 bugs. They are constraints,
not suggestions — when generating Token-2022 code, follow them. Full rationale lives in the skill's
`reference/` files.

## ATA derivation — always pass the Token-2022 program id

```ts
// BAD — derives the WRONG ATA for a Token-2022 mint
const ata = getAssociatedTokenAddressSync(mint, owner);

// GOOD
const ata = getAssociatedTokenAddressSync(mint, owner, false, TOKEN_2022_PROGRAM_ID);
```
```rust
// GOOD (Rust)
let ata = get_associated_token_address_with_program_id(&owner, &mint, &spl_token_2022::id());
```

## Mint creation — size for extensions, init extensions before the mint

```ts
// BAD — fixed/guessed size underfunds rent for an extended mint
createAccount({ space: 82, ... });

// GOOD — size for the chosen extensions, then init extensions, then the mint
const space = getMintLen([ExtensionType.TransferHook /*, ... */]);
// createAccount(space) -> initialize each extension -> initializeMint -> (metadata fields)
```

## Fee mints — never use a plain transfer; never assume gross == net

```ts
// BAD — fails on a transfer-fee mint, and over-credits if it didn't
createTransferCheckedInstruction(src, mint, dst, owner, amount, decimals);

// GOOD — fee-aware; recipient receives amount - fee
createTransferCheckedWithFeeInstruction(src, mint, dst, owner, amount, decimals, fee);
// integrators: credit (amount - fee) or read the destination balance delta
```

## Transfer-hook mints — resolve extra accounts; don't hand-build transfers

```ts
// BAD — omits the hook's extra accounts; the transfer reverts
createTransferCheckedInstruction(...);

// GOOD — resolves ExtraAccountMetaList + hook program automatically
await createTransferCheckedWithTransferHookInstruction(
  connection, src, mint, dst, owner, amount, decimals, [], commitment, TOKEN_2022_PROGRAM_ID,
);
```

## Transfer-hook programs — the mandatory checks

```rust
// In Execute, ALWAYS:
check_token_account_is_transferring(source)?;        // the `transferring` flag must be set
check_token_account_is_transferring(destination)?;
// validate the ExtraAccountMetaList PDA matches, and that any account you read is program-owned
if *aux_account.owner != *program_id { return Err(ProgramError::IllegalOwner); }
// only act for mints you support
```
Never give a hook program abusable signing authority, and never rely on a hook for a security
guarantee during open DEX trading (hooks are commonly bypassed).

## Reading a mint — inspect, don't assume

```ts
// GOOD — know which extensions a mint actually has before integrating
const mint = await getMint(conn, mintPk, "confirmed", TOKEN_2022_PROGRAM_ID);
const exts = getExtensionTypes(mint.tlvData);
```

## Versions & facts — don't hardcode stale claims

- Pin crate/npm versions against docs.rs / npm at build time; don't trust a year-old number.
- Confidential transfers depend on the ZK ElGamal Proof Program — verify mainnet availability
  before relying on it; do not assume it is enabled.
- Prefer `@solana/kit` + `@solana-program/token-2022` for new JS; `@solana/spl-token` only for
  existing web3.js v1 code.
