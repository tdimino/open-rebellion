#!/usr/bin/env bash
# Build the rebellion-web WASM module and the webapp frontend bundle.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WEB_CRATE="$ROOT/crates/rebellion-web"
APP_DIR="$ROOT/webapp"
PKG_OUT="$APP_DIR/src/wasm/pkg"

echo "==> Building rebellion-web WASM bindings"
cd "$WEB_CRATE"
PATH="/usr/bin:$PATH" wasm-pack build \
    --target web \
    --out-dir "$PKG_OUT" \
    --out-name rebellion_web \
    --release

echo "==> Installing webapp dependencies"
cd "$APP_DIR"
npm install --silent

echo "==> Building webapp bundle"
npm run build

echo ""
echo "Done!"
echo "  WASM:      $PKG_OUT/"
echo "  webapp/dist/ — production bundle"
echo ""
echo "Serve locally with:"
echo "  cd $APP_DIR && npm run preview"
