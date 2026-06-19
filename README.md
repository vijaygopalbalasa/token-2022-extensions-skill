# Token-2022 Extensions Skill for Claude Code / Codex

> Pick · wire · ship SPL Token Extensions safely — a decision-first, **tested** Token-2022 skill
> for the Solana AI Kit. Closes the kit's open [issue #12](https://github.com/solanabr/solana-ai-kit/issues/12).
>
> Maintained for [Superteam Brazil](https://superteam.com.br) · MIT licensed.

## The problem this solves

Token-2022 (SPL Token Extensions) is the 2026 toolbox for new Solana tokens — transfer fees,
transfer hooks, confidential transfers, permanent delegate, non-transferable, metadata-on-mint, and
more. But the knowledge is **scattered**, most mint extensions are **immutable after creation**, and
the failure modes are expensive:

- a token that **breaks on every DEX** because it has a transfer hook nobody honors,
- a vault that **over-credits** because it ignored a transfer fee (`amount − fee`),
- the **wrong ATA** because the Token-2022 program id wasn't passed,
- a hook program that's a **security hole** because it trusts its inputs,
- "add confidential transfers" code for a feature that's **disabled on mainnet**.

This skill exists to **decide the right extension set, wire it correctly, and not ship a token that
breaks the moment it touches a DEX, wallet, or exchange.**

### How this differs from the kit's existing `token-2022.md`

The Solana AI Kit already ships a `token-2022.md` — a single ~700-line **flat reference** of
per-extension code snippets (legacy `@solana/spl-token`, confidential transfers shown as usable, no
decision or compatibility guidance, illustrative-only test snippets). Issue #12 asks to expand that
into "the full extensions (one skill for each/each niche)." This skill is the **decision-first,
tested, progressively-loaded successor** that answers it:

| Existing `token-2022.md` | This skill |
|---|---|
| One 700-line flat file | Thin `SKILL.md` router → 11 focused topic files (progressive disclosure) |
| Reference snippets only | A **built + tested** allowlist transfer-hook program (`cargo build-sbf`; 10 unit + 10 LiteSVM e2e against the real Token-2022 program) |
| No decision guidance | `decision-tree.md` (use-case → extension set) + `compatibility-matrix.md` (will it work on Jupiter/Phantom/CEX?) |
| Legacy `@solana/spl-token` throughout | Current 2026 stack (`@solana/kit` + `@solana-program/token-2022`), with legacy noted |
| Confidential transfers shown as usable | Honest: flags they're **disabled on mainnet** (June 2025 ZK incident) — verify before use |
| — | A runnable mint inspector (`check-extensions.ts`), a security-review agent, and `/audit-token-2022` |

## What's inside

```
token-2022-extensions-skill/
├── skill/
│   ├── SKILL.md                 # thin router (progressive disclosure)
│   ├── reference/               # 11 focused topic files, loaded on demand
│   │   ├── decision-tree.md            extensions-catalog.md     compatibility-matrix.md
│   │   ├── transfer-hooks.md           transfer-fees.md          metadata-and-groups.md
│   │   ├── confidential-transfers.md   clients.md                security.md
│   │   ├── testing.md                  resources.md
│   └── scripts/
│       └── check-extensions.ts  # inspect any mint's extensions + risks (runs vs mainnet)
├── agents/      token-extensions-engineer (build) · token-extensions-reviewer (security)
├── commands/    /design-token · /add-transfer-hook · /audit-token-2022
├── rules/       token-2022.md  (auto-loads coding rules on Rust/TS files)
├── program/     allowlist transfer-hook reference program (with unit + e2e tests)
└── evals/       behavioural + mechanical evaluation scenarios
```

## Tests & verification

- `program/` is a native-Rust transfer hook that compiles with `cargo build-sbf`, with **10 host
  unit tests** and a **LiteSVM end-to-end test** that drives a real Token-2022 transfer (loaded
  from the mainnet program) and asserts the allowlist allows/denies correctly. See
  `program/README.md` for the exact commands and results.
- The inspector script runs against real mainnet mints (verified on PYUSD).
- Versions are pinned against the 2026 stack (verified 2026-06-19); facts that change over time
  (e.g. confidential transfers being disabled on mainnet) are flagged with a "verify" note.
- `SKILL.md` is a thin router; topic files load only when needed (progressive disclosure).

## Install

```bash
# personal install (all projects)
./install.sh

# or choose personal vs project-local
./install-custom.sh
```

This copies `skill/` to `~/.claude/skills/token-2022-extensions`. To also use the bundled
sub-agents and slash commands, copy `agents/`, `commands/`, and `rules/` into your matching
`.claude/` folders (the custom installer prints a reminder).

## Use it

Ask Claude things like:
- "Which Token-2022 extensions should I use for a loyalty-point token?"
- "Add a transfer hook that enforces an allowlist to my mint."
- "Will my transfer-fee token work on Jupiter and Phantom?"
- "Audit this mint: `<address>`."

Or drive it explicitly: `/design-token`, `/add-transfer-hook`, `/audit-token-2022`.

## Run the reference program's tests

```bash
cd program && cargo build-sbf && cargo test --test logic        # build + 10 unit tests
cd e2e && npm install && npm run fixtures && npm test            # 10 e2e checks (real Token-2022)
cd skill/scripts && npm install && npx tsx check-extensions.ts <MINT>   # inspect any mint
```

## Adding to the Solana AI Kit

See [`KIT-INTEGRATION.md`](KIT-INTEGRATION.md) for the `skill-registry.json` entry and the
submodule/PR snippet that slots this into `solanabr/solana-ai-kit`.

## License

MIT © 2026 Superteam Brazil. No hidden executables, no telemetry, no opaque behaviour.
