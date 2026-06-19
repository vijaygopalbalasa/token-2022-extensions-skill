# Adding this skill to the Solana AI Kit

Two clean ways to slot `token-2022-extensions` into [`solanabr/solana-ai-kit`](https://github.com/solanabr/solana-ai-kit),
matching the kit's existing patterns. **Replace `<your-username>` with the repo you submit.**

This skill also directly answers the kit's open issue **#12 "Add full Token Extension skill suite."**

---

## Option A — Registry entry (lowest friction, mirrors merged PRs #13/#18)

Add one object to `.claude/skills/skill-registry.json`. Format matches the existing entries:

```json
{
  "id": "token-2022-extensions",
  "name": "Token-2022 Extensions",
  "type": "skill",
  "domain": "solana-defi",
  "description": "Decision-first, tested successor to the kit's flat token-2022.md: choose the right extension set, wire transfer hooks/fees/metadata/groups correctly, and ship tokens that don't break on DEXs/wallets/CEXs. Includes a built+tested allowlist transfer-hook program and a mint inspector. Closes issue #12.",
  "source": "https://github.com/<your-username>/token-2022-extensions-skill",
  "install": {
    "method": "submodule",
    "command": "git submodule add https://github.com/<your-username>/token-2022-extensions-skill .claude/skills/ext/token-2022",
    "env": []
  },
  "license": "MIT",
  "maintainer": "<your-username>",
  "signal": {
    "stars": 0,
    "last_commit": "2026-06-19",
    "reputability": "emerging — verify before install"
  },
  "default_installed": false,
  "safety": "clean — native-Rust reference program (no hidden CPI), MIT, no telemetry; e2e tests load the mainnet Token-2022 program only for local testing",
  "tags": [
    "token-2022",
    "token-extensions",
    "transfer-hook",
    "transfer-fee",
    "metadata",
    "security"
  ]
}
```

---

## Option B — Submodule + hub routing (full integration)

1. **Add the submodule** (never `git add` the directory itself — it must register as a submodule):
   ```bash
   git submodule add https://github.com/<your-username>/token-2022-extensions-skill .claude/skills/ext/token-2022
   git add .gitmodules .claude/skills/ext/token-2022
   ```
   `.gitmodules` entry:
   ```
   [submodule ".claude/skills/ext/token-2022"]
       path = .claude/skills/ext/token-2022
       url = https://github.com/<your-username>/token-2022-extensions-skill
   ```

2. **Add a routing row** to the Task Routing table in `.claude/skills/SKILL.md` (same shape as the
   Jupiter/Metaplex rows):
   ```markdown
   | Token-2022 / token extensions (transfer hook, fee, metadata, permanent delegate, non-transferable, confidential) | [token-2022/](ext/token-2022/skill/SKILL.md) | Choose + wire + audit SPL Token Extensions; allowlist transfer-hook reference program |
   ```

3. **Satisfy the kit's Ripple Map** (or CI's `test_cross_references.sh` will fail):
   - add a row to the README submodules table + the repo tree,
   - add it to the `QUICK-START.md` tree.

4. **Validate locally before opening the PR:**
   ```bash
   bash validate.sh && bash tests/run_all.sh
   ```

---

## Suggested PR title & opening

Following the kit's convention and its taste for explicit non-duplication:

> **feat: token-2022-extensions — decision-first, tested successor to token-2022.md (closes #12)**
>
> Issue #12 asks to expand the kit's existing flat `token-2022.md` into the full extensions
> discipline. This does that: a thin progressive-disclosure `SKILL.md` → 11 focused topic files,
> adding what the current reference lacks — a use-case **decision tree**, a **DEX/wallet/CEX
> compatibility matrix**, the current 2026 stack (`@solana/kit` + `@solana-program/token-2022`),
> and an honest confidential-transfers-disabled-on-mainnet stance.
>
> Ships a built, tested native-Rust allowlist transfer-hook program (`cargo build-sbf`; 10 unit +
> 10 LiteSVM e2e checks against the real Token-2022 program — the deny path proven load-bearing) and
> a mint inspector verified on mainnet (PYUSD). MIT. Suggest replacing or cross-linking the existing
> `token-2022.md`; this is orthogonal to the security skills (audit) and launch-token (creation).

Attach: the `evals/evals.md` results and the `program/README.md` verified-results section as proof
of the quality bar.
