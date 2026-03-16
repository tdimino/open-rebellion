#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION="${1:-dev}"
echo "Packaging Open Rebellion web build (v${VERSION})..."

# Build WASM
bash "$ROOT/scripts/build-wasm.sh"

# Create distribution directory
DIST="dist/open-rebellion-web-${VERSION}"
rm -rf "${DIST}"
mkdir -p "${DIST}"

# Copy web assets
cp web/index.html "${DIST}/"
cp web/gl.js "${DIST}/"
cp web/open-rebellion.wasm "${DIST}/"

# Create zip
cd dist
zip -r "open-rebellion-web-${VERSION}.zip" "open-rebellion-web-${VERSION}/"
cd ..

echo "Created: dist/open-rebellion-web-${VERSION}.zip"
ls -lh "dist/open-rebellion-web-${VERSION}.zip"
