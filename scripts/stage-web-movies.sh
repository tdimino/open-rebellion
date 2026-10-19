#!/usr/bin/env bash
# Stage the original game's movies for the browser build in web/data/movies/.
#
#   scripts/stage-web-movies.sh
#   scripts/stage-web-movies.sh --verify DIR
#
# The fifteen MDATA movies are Smacker 2 files, which no browser plays. Each
# one is converted with the locally installed ffmpeg to WebM (VP9 + Opus) as
# web/data/movies/<number>.webm, the name the shell's movie overlay asks for
# (web/index.html). A movie already converted from an unchanged original
# with the same ffmpeg settings is kept: each conversion leaves a sidecar,
# .<number>.settings, holding the hash of those settings. Neither the
# originals nor the conversions are ever committed: web/data/ is ignored.
#
# Exits 1 when it staged fewer than all of them (no ffmpeg, a missing
# MDATA file); a development caller that accepts a partial set says so
# itself. --verify DIR stages nothing and exits 1 unless DIR holds every
# <number>.webm, for a release package's check.
#
# REBELLION_MDATA_DIR  the original installation's MDATA directory
#                      (default ../star-wars-rebellion/MDATA)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MDATA_DIR="${REBELLION_MDATA_DIR:-$ROOT/../star-wars-rebellion/MDATA}"
OUT_DIR="$ROOT/web/data/movies"

# The files FUN_00421ba0 names for movie ids 0x65-0x74; MDATA.002 has a
# name but no file (ghidra/notes/movies.md).
MOVIES=(000 001 003 004 005 101 102 103 104 105 106 107 108 201 202)

if [ "${1:-}" = "--verify" ]; then
    [ $# -eq 2 ] || { echo "usage: $0 --verify DIR" >&2; exit 2; }
    missing=0
    for number in "${MOVIES[@]}"; do
        if [ ! -s "$2/$number.webm" ]; then
            echo "ERROR: $2/$number.webm is missing." >&2
            missing=$((missing + 1))
        fi
    done
    [ "$missing" -eq 0 ] || exit 1
    echo "All ${#MOVIES[@]} movies present in $2."
    exit 0
fi

if ! command -v ffmpeg >/dev/null 2>&1; then
    echo "WARNING: ffmpeg not found; the browser build will skip every movie." >&2
    exit 1
fi

# The encoder settings; a change to them re-encodes every movie.
FFMPEG_ARGS=(-c:v libvpx-vp9 -crf 30 -b:v 0 -row-mt 1 -c:a libopus -f webm)
SETTINGS="$(printf '%s\n' "${FFMPEG_ARGS[@]}" | shasum -a 256 | cut -d' ' -f1)"

mkdir -p "$OUT_DIR"
# A failed conversion leaves no half-written file behind.
tmp=""
trap '[ -z "$tmp" ] || rm -f "$tmp"' EXIT
staged=0
for number in "${MOVIES[@]}"; do
    src="$MDATA_DIR/MDATA.$number"
    out="$OUT_DIR/$number.webm"
    sidecar="$OUT_DIR/.$number.settings"
    if [ ! -f "$src" ]; then
        rm -f "$out" "$sidecar"
        echo "WARNING: $src not found; the browser build will skip movie $number." >&2
        continue
    fi
    if [ "$out" -nt "$src" ] && [ "$(cat "$sidecar" 2>/dev/null)" = "$SETTINGS" ]; then
        staged=$((staged + 1))
        continue
    fi
    echo "Converting MDATA.$number to $number.webm…"
    tmp="$OUT_DIR/.$number.webm"
    ffmpeg -y -loglevel error -i "$src" "${FFMPEG_ARGS[@]}" "$tmp"
    mv "$tmp" "$out"
    tmp=""
    printf '%s\n' "$SETTINGS" > "$sidecar"
    staged=$((staged + 1))
done
echo "Staged $staged of ${#MOVIES[@]} movies in web/data/movies/."
[ "$staged" -eq "${#MOVIES[@]}" ]
