#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SITE="$ROOT/.artifacts/interface-parity/site"
TARGET="$ROOT/target/wasm32-unknown-unknown/release/open-rebellion.wasm"

"$ROOT/scripts/build-wasm.sh"

echo "Building isolated interface-test WASM fixture artifact…"
cargo build --manifest-path "$ROOT/Cargo.toml" \
    --target wasm32-unknown-unknown \
    -p rebellion-app \
    --release \
    --features interface-test-fixtures

mkdir -p "$SITE"
cp "$TARGET" "$SITE/open-rebellion-test.wasm"
if command -v wasm-opt >/dev/null 2>&1; then
    if wasm-opt -O3 --strip-debug \
        --enable-nontrapping-float-to-int \
        --enable-bulk-memory \
        --enable-bulk-memory-opt \
        --enable-mutable-globals \
        --enable-sign-ext \
        --enable-reference-types \
        --enable-multivalue \
        -o "$SITE/open-rebellion-test.opt.wasm" \
        "$SITE/open-rebellion-test.wasm"; then
        mv "$SITE/open-rebellion-test.opt.wasm" "$SITE/open-rebellion-test.wasm"
    else
        rm -f "$SITE/open-rebellion-test.opt.wasm"
        echo "WARNING: test wasm-opt failed; retaining the unoptimized fixture artifact."
    fi
fi

node "$ROOT/tools/interface-parity/prepare-site.mjs"
node "$ROOT/tools/interface-parity/validate-catalog.mjs"
node "$ROOT/tools/interface-parity/verify-production-exclusion.mjs"
echo "Prepared isolated fixture site at $SITE"
