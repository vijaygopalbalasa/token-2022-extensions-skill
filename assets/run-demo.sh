#!/usr/bin/env bash
# Fallback demo driver (no VHS needed): run this while screen-recording your terminal,
# or pipe into asciinema. Renders the same three proofs the GIF shows.
#
#   asciinema:  asciinema rec demo.cast -c "bash assets/run-demo.sh" && agg demo.cast assets/demo.gif
#   manual:     start a screen recorder, then: bash assets/run-demo.sh
#
# Prereqs (so it's fast): build the program and install deps first —
#   (cd program && cargo build-sbf)
#   (cd program/e2e && npm install && npm run fixtures)
#   (cd skill/scripts && npm install)
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
say() { printf "\n\033[1;36m%s\033[0m\n" "$1"; sleep 1; }

say "# 1) Unit tests for the allowlist + instruction logic"
( cd "$ROOT/program" && cargo test --test logic 2>&1 | tail -4 ); sleep 2

say "# 2) End-to-end: a REAL Token-2022 transfer through the hook (LiteSVM)"
( cd "$ROOT/program/e2e" && npm test 2>&1 | tail -13 ); sleep 2

say "# 3) Inspect a real mainnet Token-2022 mint (PYUSD)"
( cd "$ROOT/skill/scripts" && npx tsx check-extensions.ts 2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo 2>&1 | head -14 ); sleep 2

say "# Decide -> wire -> ship Token-2022 safely. SKILL.md routes the rest."
