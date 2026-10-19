#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION="${1:-dev}"
if [[ ! "${VERSION}" =~ ^[A-Za-z0-9._-]+$ ]]; then
    echo "ERROR: version may contain only letters, numbers, dots, underscores, and hyphens."
    exit 1
fi
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

# Runtime data is required by the WASM loading screen. Ship the deterministic
# pack rather than thousands of loose files; the original game data remains
# local, generated, and intentionally untracked.
if [ ! -f web/data/runtime.orpk ]; then
    echo "ERROR: scripts/build-wasm.sh did not create web/data/runtime.orpk."
    exit 1
fi
mkdir -p "${DIST}/data"
cp web/data/runtime.orpk "${DIST}/data/"

# Refuse to create an artifact that can compile but cannot boot.
if [ ! -s "${DIST}/data/runtime.orpk" ]; then
    echo "ERROR: packaged browser runtime pack is missing or empty."
    exit 1
fi

# Movies play from <video> elements, outside the runtime pack: each is
# fetched only when it plays, and a missing one is skipped in the browser.
# A dev package may lack some; any other version ships all of them.
mkdir -p "${DIST}/data/movies"
if [ "${VERSION}" = "dev" ]; then
    bash "$ROOT/scripts/stage-web-movies.sh" \
        || echo "WARNING: dev package without every movie; the browser skips the missing ones."
    shopt -s nullglob
    movies=(web/data/movies/*.webm)
    shopt -u nullglob
    if [ ${#movies[@]} -gt 0 ]; then
        cp "${movies[@]}" "${DIST}/data/movies/"
    fi
else
    bash "$ROOT/scripts/stage-web-movies.sh"
    cp web/data/movies/*.webm "${DIST}/data/movies/"
    bash "$ROOT/scripts/stage-web-movies.sh" --verify "${DIST}/data/movies"
fi

# Record hashes for every shipped runtime file so release and deployment
# verification can prove which data and code the browser loaded.
(
    cd "${DIST}"
    find . -type f ! -name SHA256SUMS -print \
        | LC_ALL=C sort \
        | while IFS= read -r file; do shasum -a 256 "${file}"; done \
        > SHA256SUMS
)

# Create zip
ZIP="${ROOT}/dist/open-rebellion-web-${VERSION}.zip"
rm -f "${ZIP}"
cd dist
if command -v zip >/dev/null 2>&1; then
    zip -rq "open-rebellion-web-${VERSION}.zip" "open-rebellion-web-${VERSION}/"
else
    python3 -m zipfile -c \
        "open-rebellion-web-${VERSION}.zip" \
        "open-rebellion-web-${VERSION}/"
fi
cd ..

echo "Created: ${ZIP}"
ls -lh "${ZIP}"
