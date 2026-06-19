# Demo assets

A short terminal demo proving the skill's reference program builds and its tests pass, plus the
live mint inspector. Attach the rendered GIF to the bounty submission.

## Render the GIF

**Option A — VHS (recommended):**
```bash
# one-time so the GIF stays short:
(cd ../program && cargo build-sbf)
(cd ../program/e2e && npm install && npm run fixtures)
(cd ../skill/scripts && npm install)
# render (from repo root):
vhs assets/demo.tape        # -> assets/demo.gif
```
Install VHS: https://github.com/charmbracelet/vhs

**Option B — asciinema + agg (no VHS):**
```bash
asciinema rec demo.cast -c "bash assets/run-demo.sh"
agg demo.cast assets/demo.gif
```

**Option C — any screen recorder:** start recording, then `bash assets/run-demo.sh`.

## What the demo shows
1. `cargo test --test logic` → 10 unit tests pass (allowlist + instruction logic).
2. `npm test` (e2e) → 10 checks pass: a real Token-2022 transfer through the hook —
   allowlisted succeeds, non-allowlisted is rejected, missing-accounts fails.
3. `check-extensions.ts` against mainnet PYUSD → real extensions + risk flags.
