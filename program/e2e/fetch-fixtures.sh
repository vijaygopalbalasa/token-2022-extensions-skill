#!/bin/bash
# Fetch the real Token-2022 program binary from mainnet so the e2e test can load
# it into LiteSVM. Requires the Solana CLI and network access. Run once before
# `npm test`. The .so is git-ignored (it's ~1.4 MB and reproducible from chain).
set -e
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/fixtures"
mkdir -p "$DIR"
OUT="$DIR/spl_token_2022.so"
if [ -f "$OUT" ]; then
  echo "✓ already present: $OUT"
  exit 0
fi
echo "→ dumping Token-2022 program from mainnet..."
solana program dump TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb "$OUT" \
  --url https://api.mainnet-beta.solana.com
echo "✓ wrote $OUT"
