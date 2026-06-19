# Demo recording for the token-2022-extensions skill.
# Renders a GIF with VHS (https://github.com/charmbracelet/vhs):  vhs assets/demo.tape
# Prereqs (so the GIF is short — do these BEFORE rendering):
#   cd program && cargo build-sbf
#   cd program/e2e && npm install && npm run fixtures
#   cd skill/scripts && npm install
# Run vhs from the repo root.

Output assets/demo.gif

Set Shell "bash"
Set FontSize 18
Set Width 1280
Set Height 760
Set Padding 16
Set Theme "Catppuccin Mocha"

Type "# Token-2022 Extensions skill — it actually works, with real tests"
Enter
Sleep 1500ms

Type "# 1) Unit tests for the allowlist + instruction logic"
Enter
Type "cd program && cargo test --test logic 2>&1 | tail -4"
Enter
Sleep 6s

Type "# 2) End-to-end: a REAL Token-2022 transfer through the hook (LiteSVM)"
Enter
Type "cd e2e && npm test 2>&1 | tail -13"
Enter
Sleep 8s

Type "# allowlisted -> succeeds, non-allowlisted -> rejected by the hook, missing accounts -> fails"
Enter
Sleep 2s

Type "# 3) Inspect a real mainnet Token-2022 mint (PYUSD)"
Enter
Type "cd ../../skill/scripts && npx tsx check-extensions.ts 2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo 2>&1 | head -14"
Enter
Sleep 7s

Type "# Decide -> wire -> ship Token-2022 safely. SKILL.md routes the rest."
Enter
Sleep 3s
