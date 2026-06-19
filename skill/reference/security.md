# Security — extension footguns & pre-launch checklist

Token-2022 extensions add power and, with it, ways to lose funds or trust. This is the security
pass to run before mainnet. Primary source for the program-level footguns:
https://neodyme.io/en/blog/token-2022/

## The high-severity footguns

### `PermanentDelegate` = unlimited clawback over every holder
The permanent delegate can transfer or burn tokens from **any** account of the mint, without the
owner's consent. Legitimate for compliance/recovery tokens; a rug vector for "community" tokens.
- If you set it: **disclose it prominently** (README, token page) and ideally hold the authority in
  a multisig/governance.
- If you integrate someone else's token: treat a permanent delegate as "these tokens can vanish."
  A vault/protocol holding such tokens can be drained by the delegate.

### `DefaultAccountState = Frozen` breaks naive flows
New token accounts (including ATAs) are created **Frozen**. Airdrops, escrows, vaults, and
"create-ATA-then-transfer" flows fail unless a freeze authority thaws the account first. Design the
thaw/approval step explicitly.

### Transfer hooks (see [transfer-hooks.md](transfer-hooks.md))
A hook program must: verify the mint, check the `transferring` flag, validate token accounts belong
to the mint, and never hold abusable CPI authority. A hook that trusts its inputs is the classic
Token-2022 vulnerability. Also remember hooks are **bypassable on many DEXs** — don't rely on a
hook for a security guarantee during open trading.

### Transfer fees → gross/net accounting (see [transfer-fees.md](transfer-fees.md))
Any program that assumes "amount sent == amount received" mis-accounts on a fee mint. Use balance
deltas or compute the fee from config. Don't forget: accounts with withheld fees can't be closed.

### `Pausable`
A pause authority can halt all movement. Integrators must handle the paused state (don't assume
transfers always succeed). Hold the pause authority securely.

## The quieter footguns

- **`MetadataPointer` mismatch** — pointer not pointing at the mint (for on-mint metadata) lets a
  third party serve spoofable metadata. Verify the pointer.
- **`ScaledUiAmount` / `InterestBearingConfig`** — the displayed amount ≠ the raw `amount`. A
  protocol that reads raw `amount` and treats it as the "real" balance will be wrong. Decide which
  amount your logic uses and document it.
- **`ConfidentialTransfer*`** — depends on a network feature that may be disabled; pending-balance
  spam; auditor-key configuration. See [confidential-transfers.md](confidential-transfers.md).
- **`MintCloseAuthority`** — the mint can be closed (supply must be 0) and the address reused;
  indexers/integrators caching mint state should handle closure.
- **CPI Guard / Memo-transfer** — integrators must add the memo / use the delegate flow or
  transfers fail; not a vuln, but a frequent "why does my transfer revert" support load.

## Authority hygiene (applies to all extensions)

- Prefer **multisig / governance** for any standing authority: mint, freeze, permanent delegate,
  pause, transfer-fee config, withdraw-withheld, metadata update.
- Set authorities you don't need to **`None`** at creation (e.g. revoke mint authority for a fixed
  supply; revoke freeze authority if you never freeze).
- Document every live authority and who holds it — undisclosed authorities are a trust red flag.

## Pre-launch checklist

- [ ] Every extension is one you actually need (smallest set that meets the goal).
- [ ] Compatibility verified on **devnet** for each target wallet / DEX / CEX ([compatibility-matrix.md](compatibility-matrix.md)).
- [ ] Mint sizing uses `getMintSize`/`try_calculate_account_len`; extension init runs before `initializeMint`.
- [ ] ATAs derived with the **Token-2022 program ID**.
- [ ] Fee mints: integrators use `transfer_checked_with_fee` and net-amount accounting; harvest-before-close handled.
- [ ] Hook program: mint check + `transferring` flag + account-ownership checks + minimal authority; e2e tests for allow/deny/missing-accounts.
- [ ] `PermanentDelegate` / `Pausable` / freeze authorities held in multisig and **disclosed**.
- [ ] Unneeded authorities revoked (`None`).
- [ ] `DefaultAccountState=Frozen` flows have an explicit thaw/approval path.
- [ ] Confidential transfers: confirmed live on mainnet **today** before relying on them.
- [ ] Tests run and pass ([testing.md](testing.md)).
