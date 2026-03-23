#!/usr/bin/env bash
# Build Open Rebellion for wasm32-unknown-unknown and stage artifacts in web/.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TARGET_DIR="$ROOT/target/wasm32-unknown-unknown/release"
GDATA="$ROOT/data/base"
WEB_DATA="$ROOT/web/data/base"

echo "Building rebellion-app for wasm32…"
PATH="/usr/bin:$PATH" cargo build --manifest-path "$ROOT/Cargo.toml" \
    --target wasm32-unknown-unknown \
    -p rebellion-app \
    --release

# The binary name may be rebellion-app or open-rebellion depending on the build
WASM_SRC="$TARGET_DIR/open-rebellion.wasm"
if [ ! -f "$WASM_SRC" ]; then
    WASM_SRC="$TARGET_DIR/rebellion-app.wasm"
fi
cp "$WASM_SRC" "$ROOT/web/open-rebellion.wasm"
echo "Copied open-rebellion.wasm → web/"

# gl.js comes from macroquad/miniquad. Must be vendored in repo.
if [ ! -f "$ROOT/web/gl.js" ]; then
    echo "ERROR: web/gl.js not found. It should be committed in the repo."
    exit 1
fi
echo "gl.js present (vendored)."

# ── Copy DAT files for WASM HTTP fetch ──────────────────────────────────
echo "Copying DAT files to web/data/base/…"
mkdir -p "$WEB_DATA"
cp "$GDATA"/*.DAT "$WEB_DATA/" 2>/dev/null || true
# Copy DLL too (for string lookup if native-style loading is ever ported)
cp "$GDATA"/*.DLL "$WEB_DATA/" 2>/dev/null || true

# Extract TEXTSTRA strings to JSON for WASM (pelite can't target WASM)
echo "Extracting TEXTSTRA.DLL strings to textstra.json…"
DAT_DUMPER="${ROOT}/target/release/dat-dumper"
if [ ! -f "$DAT_DUMPER" ]; then
    echo "Building dat-dumper for string extraction…"
    PATH="/usr/bin:$PATH" cargo build --manifest-path "$ROOT/Cargo.toml" -p dat-dumper --release
fi
if [ -f "$GDATA/TEXTSTRA.DLL" ]; then
    "$DAT_DUMPER" --gdata "$GDATA" --extract-strings --output "$WEB_DATA"
else
    echo "{}" > "$WEB_DATA/textstra.json"
    echo "WARNING: TEXTSTRA.DLL not found. Entity names will use fallback format."
fi

DAT_COUNT=$(ls -1 "$WEB_DATA"/*.DAT 2>/dev/null | wc -l | tr -d ' ')
echo "Staged $DAT_COUNT DAT files + textstra.json in web/data/base/"

WASM_SIZE=$(du -h "$ROOT/web/open-rebellion.wasm" | cut -f1)
echo "Done. WASM size: $WASM_SIZE"
echo "Serve web/ with any HTTP server, e.g.: python3 -m http.server 8080 -d web/"
