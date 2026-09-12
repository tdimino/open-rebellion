#!/usr/bin/env bash
# Orchestrates the full browser-build pipeline inside the Docker "builder"
# service: stage original game data, convert+decode cutscenes, build WASM.
#
# Expects the original, legally-owned Star Wars Rebellion installation
# mounted read-only at $ORIGINAL_GAME_DIR (see docker-compose.yml /
# .env.example — STAR_WARS_REBELLION_DIR on the host).
#
# Set FORCE_REBUILD=1 to permit replacing changed or incomplete asset outputs.
# Verified unchanged cutscenes are reused. Extraction is safe to rerun.
#
# Set PREPARE_MODDING=1 to also dump every original .DAT table (and
# TEXTSTRA.DLL's name strings) to data/base/json/ — the reference modders
# need to find dat_id values and field names before writing overlay patches.
# See README_MOD.md.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

ORIGINAL_GAME_DIR="${ORIGINAL_GAME_DIR:-/original-game}"
FORCE_REBUILD="${FORCE_REBUILD:-0}"
PREPARE_MODDING="${PREPARE_MODDING:-0}"

if [ ! -d "$ORIGINAL_GAME_DIR" ] || [ -z "$(ls -A "$ORIGINAL_GAME_DIR" 2>/dev/null)" ]; then
    echo "ERROR: $ORIGINAL_GAME_DIR is missing or empty." >&2
    echo "Set STAR_WARS_REBELLION_DIR in .env to your original game install." >&2
    exit 1
fi

echo "=== [1/4] Staging DAT/DLL files into data/base/ ==="
mkdir -p data/base
find "$ORIGINAL_GAME_DIR" -iname '*.DAT' -exec cp -n {} data/base/ \;
find "$ORIGINAL_GAME_DIR" -iname '*.DLL' -exec cp -n {} data/base/ \;
# stage-ui-assets requires exact uppercase filenames.
for f in data/base/*.[Dd][Aa][Tt] data/base/*.[Dd][Ll][Ll]; do
    [ -e "$f" ] || continue
    upper="$(dirname "$f")/$(basename "$f" | tr '[:lower:]' '[:upper:]')"
    [ "$f" = "$upper" ] || mv -n "$f" "$upper"
done
DAT_COUNT=$(find data/base -maxdepth 1 -iname '*.DAT' | wc -l | tr -d ' ')
echo "Staged $DAT_COUNT .DAT files."

MDATA_SRC_DIR="$ORIGINAL_GAME_DIR/MDATA"
if [ ! -d "$MDATA_SRC_DIR" ]; then
    found_marker="$(find "$ORIGINAL_GAME_DIR" -iname 'MDATA.101' -print -quit)"
    MDATA_SRC_DIR=""
    [ -n "$found_marker" ] && MDATA_SRC_DIR="$(dirname "$found_marker")"
fi

echo "=== [2/4] Extracting and verifying all supported assets ==="
asset_args=(--source data/base --output data/base/ui --mdata "$MDATA_SRC_DIR")
if [ "$FORCE_REBUILD" = "1" ]; then
    asset_args+=(--force)
fi
go run ./tools/stage-ui-assets "${asset_args[@]}"

echo "=== [3/4] Building WASM + browser runtime pack ==="
export REBELLION_MDATA_DIR="${MDATA_SRC_DIR:-$ORIGINAL_GAME_DIR/MDATA}"
export REBELLION_GAME_DIR="$ROOT/data/base"
./scripts/build-wasm.sh

echo "=== [4/4] Preparing modding reference data ==="
if [ "$PREPARE_MODDING" = "1" ]; then
    DAT_DUMPER="$ROOT/target/release/dat-dumper"
    if [ ! -x "$DAT_DUMPER" ]; then
        echo "Building dat-dumper..."
        cargo build --manifest-path "$ROOT/Cargo.toml" -p dat-dumper --release
    fi
    mkdir -p data/base/json
    echo "Dumping all .DAT tables to data/base/json/ ..."
    "$DAT_DUMPER" --gdata data/base --output data/base/json
    if [ -f data/base/TEXTSTRA.DLL ]; then
        echo "Extracting TEXTSTRA.DLL name strings to data/base/json/textstra.json ..."
        "$DAT_DUMPER" --gdata data/base --extract-strings --output data/base/json
    else
        echo "WARNING: TEXTSTRA.DLL not found in data/base; name-string lookup will be unavailable."
    fi
    echo "Mod reference data ready in data/base/json/. See README_MOD.md."
else
    echo "PREPARE_MODDING is not set to 1 — skipping. See README_MOD.md to enable it."
fi

echo
echo "Build complete. Serve web/ with the 'web' compose service, or manually:"
echo "  python3 -m http.server 8080 -d web/"
