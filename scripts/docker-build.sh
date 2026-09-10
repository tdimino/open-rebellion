#!/usr/bin/env bash
# Orchestrates the full browser-build pipeline inside the Docker "builder"
# service: stage original game data, convert+decode cutscenes, build WASM.
#
# Expects the original, legally-owned Star Wars Rebellion installation
# mounted read-only at $ORIGINAL_GAME_DIR (see docker-compose.yml /
# .env.example — STAR_WARS_REBELLION_DIR on the host).
#
# Set FORCE_REBUILD=1 to redo cutscene conversion/decoding even if output
# already exists. Everything else is idempotent and safe to rerun.
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

echo "=== [1/6] Staging DAT/DLL files into data/base/ ==="
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

echo "=== [2/6] Staging UI bitmaps (tools/stage-ui-assets) ==="
if go run ./tools/stage-ui-assets --verify --output data/base/ui >/dev/null 2>&1; then
    echo "data/base/ui already verified — skipping extraction."
else
    go run ./tools/stage-ui-assets --source data/base --output data/base/ui
fi

echo "=== [3/6] Converting Smacker cutscenes to WebM ==="
MDATA_SRC_DIR="$ORIGINAL_GAME_DIR/MDATA"
if [ ! -d "$MDATA_SRC_DIR" ]; then
    found_marker="$(find "$ORIGINAL_GAME_DIR" -iname 'MDATA.101' -print -quit)"
    MDATA_SRC_DIR=""
    [ -n "$found_marker" ] && MDATA_SRC_DIR="$(dirname "$found_marker")"
fi

REF_VIDEOS="assets/references/ref-videos"
mkdir -p "$REF_VIDEOS"
CUTSCENE_NUMBERS=(000 001 003 004 005 101 102 103 104 105 106 107 108 201 202)
if [ -n "$MDATA_SRC_DIR" ] && [ -d "$MDATA_SRC_DIR" ]; then
    for n in "${CUTSCENE_NUMBERS[@]}"; do
        src="$MDATA_SRC_DIR/MDATA.$n"
        dst="$REF_VIDEOS/$n.webm"
        if [ -f "$src" ] && { [ ! -f "$dst" ] || [ "$FORCE_REBUILD" = "1" ]; }; then
            echo "Converting MDATA.$n -> $n.webm"
            ffmpeg -y -loglevel error -i "$src" -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus "$dst"
        fi
    done
else
    echo "WARNING: no MDATA directory found under $ORIGINAL_GAME_DIR; cutscenes will stay disabled."
fi

echo "=== [4/6] Decoding cutscenes to frame sequences ==="
NEED_DECODE="$FORCE_REBUILD"
for n in "${CUTSCENE_NUMBERS[@]}"; do
    if [ -f "$REF_VIDEOS/$n.webm" ] && [ ! -d "assets/references/cutscene-frames/$n" ]; then
        NEED_DECODE=1
    fi
done
if [ "$NEED_DECODE" = "1" ]; then
    ./scripts/decode-cutscenes.sh
else
    echo "Cutscene frames already decoded — skipping."
fi

echo "=== [5/6] Building WASM + browser runtime pack ==="
export REBELLION_MDATA_DIR="${MDATA_SRC_DIR:-$ORIGINAL_GAME_DIR/MDATA}"
export REBELLION_GAME_DIR="$ROOT/data/base"
./scripts/build-wasm.sh

echo "=== [6/6] Preparing modding reference data ==="
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
