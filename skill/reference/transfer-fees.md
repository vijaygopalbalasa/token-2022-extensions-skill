# Transfer Fees

`TransferFeeConfig` makes every transfer of a mint withhold a fee. It's how you build a protocol
"tax". The trap is accounting: **the recipient receives `amount − fee`**, and naive integrators
that assume gross == net will mis-account. Source: https://solana.com/docs/tokens/extensions/transfer-fees

## Config fields

`TransferFeeConfig` stores:
- `transfer_fee_basis_points` — fee as basis points of the transfer amount (1 bp = 0.01%).
- `maximum_fee` — absolute cap in base units (so large transfers don't pay an unbounded fee).
- An **epoch-aware** pair (`older` + `newer` fee, each with the epoch it takes effect) — fee
  changes are scheduled for a future epoch, not applied instantly.
- Authorities: a **transfer-fee config authority** (can change the fee) and a **withdraw-withheld
  authority** (can collect fees).

## You must use the fee-aware transfer instruction

A plain `transfer` / `transfer_checked` on a fee mint **fails**. Use **`transfer_checked_with_fee`**,
which takes the explicit `fee` and asserts it matches the on-chain config — preventing silent
under/over-payment. Compute the expected fee from the config (`amount × bps`, capped at
`maximum_fee`) before building the instruction.

## Where fees go — the withheld-fee flow

Fees are **not** sent to a treasury on each transfer. They are **withheld on the recipient's token
account** (the `TransferFeeAmount` account extension), then collected in two steps:

1. **`HarvestWithheldTokensToMint`** — permissionless; anyone can sweep withheld fees from holder
   accounts into the mint's withheld accumulator. (Often run as a cron by the protocol.)
2. **`WithdrawWithheldTokensFromMint`** — the withdraw-withheld authority moves the accumulated
   fees from the mint into a fee-vault token account.
   - Variant: **`WithdrawWithheldTokensFromAccounts`** — pull directly from a list of accounts,
     skipping the harvest step.

Consequence: **a holder's token account cannot be closed while it still holds withheld fees** —
harvest first, then close.

## Integrator pitfalls (the part that loses money)

1. **Gross vs net.** If you transfer `amount` into a vault/escrow and then credit the user
   `amount`, you over-credit by `fee`. Fix by either:
   - computing `fee` from the config and crediting `amount − fee`, **or**
   - reading the destination balance **delta** (`balance_after − balance_before`) and crediting that.
2. **Don't hardcode the fee.** It's epoch-scheduled and can change — read it from the mint.
3. **Closing accounts.** Harvest withheld fees before attempting to close a token account.
4. **DEX/CEX support is uneven.** Some pools and many CEX deposit flows don't handle fee tokens
   correctly. See [compatibility-matrix.md](compatibility-matrix.md).
5. **Max-fee cap.** Large transfers pay `maximum_fee`, not `bps × amount` — your fee revenue model
   must account for the cap.

## Minimal flow (pseudocode)

```text
create mint with TransferFeeConfig(bps, max_fee, config_auth, withdraw_auth)
...
fee = min(amount * bps / 10_000, max_fee)
transfer_checked_with_fee(source, mint, dest, owner, amount, decimals, fee)   // recipient gets amount - fee
...
# later, collect:
harvest_withheld_tokens_to_mint([holder_accounts...])     # permissionless
withdraw_withheld_tokens_from_mint(mint -> fee_vault, withdraw_auth)
```

Exact instruction builders are in [clients.md](clients.md) (`@solana-program/token-2022` for Kit,
`@solana/spl-token` for legacy v1, `spl-token-2022` for Rust). Write tests that assert the
**recipient delta equals `amount − fee`** — see [testing.md](testing.md).
