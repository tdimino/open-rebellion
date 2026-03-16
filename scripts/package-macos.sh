#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION="${1:-dev}"
echo "Packaging Open Rebellion macOS build (v${VERSION})..."

# Build release binary
PATH="/usr/bin:$PATH" cargo build --manifest-path "$ROOT/Cargo.toml" -p rebellion-app --release

# Create distribution directory
DIST="dist/open-rebellion-macos-${VERSION}"
rm -rf "${DIST}"
mkdir -p "${DIST}"

# Copy binary
cp target/release/open-rebellion "${DIST}/" 2>/dev/null || cp target/release/rebellion-app "${DIST}/open-rebellion"

# Create README for distribution
cat > "${DIST}/README.txt" << 'INNER_EOF'
Open Rebellion — Star Wars Rebellion Reimplementation

SETUP:
1. Purchase Star Wars Rebellion from GOG.com or Steam
2. Copy the GData/ directory from your installation to data/base/
3. Run: ./open-rebellion data/base

CONTROLS:
- Scroll to zoom, right-drag to pan, left-click to select
- Space: pause/unpause, 1/2/3: speed, R: reset view, Esc: quit
- O/F/M/N: panels, E: encyclopedia, Tab: mod manager

More info: https://github.com/tdimino/open-rebellion
INNER_EOF

# Create archive
cd dist
tar -czf "open-rebellion-macos-${VERSION}.tar.gz" "open-rebellion-macos-${VERSION}/"
cd ..

echo "Created: dist/open-rebellion-macos-${VERSION}.tar.gz"
ls -lh "dist/open-rebellion-macos-${VERSION}.tar.gz"
