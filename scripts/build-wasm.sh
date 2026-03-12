#!/usr/bin/env bash
# Build Open Rebellion for wasm32-unknown-unknown and stage artifacts in web/.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TARGET_DIR="$ROOT/target/wasm32-unknown-unknown/release"

echo "Building rebellion-app for wasm32…"
cargo build --manifest-path "$ROOT/Cargo.toml" \
    --target wasm32-unknown-unknown \
    -p rebellion-app \
    --release

cp "$TARGET_DIR/rebellion-app.wasm" "$ROOT/web/open-rebellion.wasm"
echo "Copied open-rebellion.wasm → web/"

# gl.js comes from macroquad/miniquad. If it isn't already present, fetch it.
if [ ! -f "$ROOT/web/gl.js" ]; then
    echo "Fetching gl.js from macroquad…"
    curl -sSL "https://not-fl3.github.io/miniquad-samples/gl.js" -o "$ROOT/web/gl.js"
    echo "Downloaded gl.js → web/"
else
    echo "gl.js already present."
fi

WASM_SIZE=$(du -h "$ROOT/web/open-rebellion.wasm" | cut -f1)
echo "Done. WASM size: $WASM_SIZE"
echo "Serve web/ with any HTTP server, e.g.: python3 -m http.server 8080 -d web/"
