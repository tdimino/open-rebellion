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

# ── wasm-opt: shrink and optimize final artifact ──────────────────────────
# Recovers the size overhead from web-sys/wasm-bindgen etc.
# Install via `brew install binaryen` or the binaryen release binaries.
if command -v wasm-opt >/dev/null 2>&1; then
    BYTES_BEFORE=$(wc -c < "$ROOT/web/open-rebellion.wasm" | tr -d ' ')
    # Feature flags for modern Rust-generated WASM. Rust's LLVM backend
    # emits these by default since ~1.60.
    wasm-opt -O3 --strip-debug \
        --enable-nontrapping-float-to-int \
        --enable-bulk-memory \
        --enable-bulk-memory-opt \
        --enable-mutable-globals \
        --enable-sign-ext \
        --enable-reference-types \
        --enable-multivalue \
        -o "$ROOT/web/open-rebellion.wasm" \
        "$ROOT/web/open-rebellion.wasm"
    BYTES_AFTER=$(wc -c < "$ROOT/web/open-rebellion.wasm" | tr -d ' ')
    SAVED=$((BYTES_BEFORE - BYTES_AFTER))
    PCT=$(( (SAVED * 100) / BYTES_BEFORE ))
    echo "wasm-opt -O3: ${BYTES_BEFORE} → ${BYTES_AFTER} bytes (saved ${SAVED}, ${PCT}%)"
else
    echo "WARNING: wasm-opt not found. Install binaryen for a smaller release build."
fi

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

# ── Stage UI BMPs into web/data/ui/ ─────────────────────────────────────────
UI_SRC="$ROOT/data/base/ui"
WEB_UI="$ROOT/web/data/ui"
if [ -d "$UI_SRC" ]; then
    echo "Copying staged UI BMPs to web/data/ui/…"
    mkdir -p "$WEB_UI"
    cp -r "$UI_SRC"/. "$WEB_UI/"
    UI_COUNT=$(find "$WEB_UI" -name "*.bmp" 2>/dev/null | wc -l | tr -d ' ')
    echo "Staged $UI_COUNT UI BMPs in web/data/ui/"

    # Generate BMP manifest for WASM pre-fetch (HTTP can't enumerate dirs)
    echo "Generating BMP manifest for WASM…"
    MANIFEST="$WEB_UI/bmp-manifest.json"
    python3 -c "
import json, pathlib, sys
root = pathlib.Path('$WEB_UI')
entries, skipped = [], 0
for dll_dir in sorted(root.iterdir()):
    if not dll_dir.is_dir():
        continue
    bmp_dir = dll_dir / 'BMP'
    if not bmp_dir.is_dir():
        continue
    dll_name = dll_dir.name
    for f in sorted(bmp_dir.glob('*.bmp')):
        try:
            entries.append({'dll': dll_name, 'id': int(f.stem)})
        except ValueError:
            skipped += 1
            print(f'  WARNING: skipping non-numeric BMP: {f}', file=sys.stderr)
if skipped:
    print(f'  Skipped {skipped} non-numeric BMP files', file=sys.stderr)
with open('$MANIFEST', 'w') as fh:
    json.dump(entries, fh, separators=(',', ':'))
print(f'  {len(entries)} entries in bmp-manifest.json')
if not entries:
    print('  WARNING: manifest is empty — no BMPs found', file=sys.stderr)
"
else
    echo "WARNING: data/base/ui/ not found — run scripts/stage-ui-assets.py first."
fi

WASM_SIZE=$(du -h "$ROOT/web/open-rebellion.wasm" | cut -f1)
echo "Done. WASM size: $WASM_SIZE"
echo "Serve web/ with any HTTP server, e.g.: python3 -m http.server 8080 -d web/"
