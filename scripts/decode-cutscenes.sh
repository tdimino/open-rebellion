#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC_DIR="$ROOT/assets/references/ref-videos"
OUT_DIR="$ROOT/assets/references/cutscene-frames"

if ! command -v ffmpeg >/dev/null 2>&1; then
    echo "ffmpeg is required. Install it, then rerun this script." >&2
    exit 1
fi

if ! command -v ffprobe >/dev/null 2>&1; then
    echo "ffprobe is required. Install it, then rerun this script." >&2
    exit 1
fi

mkdir -p "$OUT_DIR"

shopt -s nullglob
sources=("$SRC_DIR"/*.webm)
shopt -u nullglob

if [ "${#sources[@]}" -eq 0 ]; then
    echo "No WebM cutscenes found in $SRC_DIR" >&2
    exit 0
fi

for src in "${sources[@]}"; do
    name="$(basename "$src" .webm)"
    frame_dir="$OUT_DIR/$name"
    audio_path="$OUT_DIR/$name.wav"
    tmp_frame_dir="$OUT_DIR/.$name.tmp"
    tmp_audio_path="$OUT_DIR/.$name.wav.tmp"

    echo "Decoding $name.webm"
    # Decode into a temp dir first, then atomically rename into place on success.
    # If ffmpeg or ffprobe fails mid-run, the existing decoded cutscene is preserved.
    rm -rf "$tmp_frame_dir"
    mkdir -p "$tmp_frame_dir"
    rm -f "$tmp_audio_path"
    trap 'rm -rf "$tmp_frame_dir" "$tmp_audio_path"' EXIT

    ffmpeg -y -loglevel error -i "$src" "$tmp_frame_dir/frame-%05d.png"
    ffmpeg -y -loglevel error -i "$src" -vn -c:a pcm_s16le "$tmp_audio_path"

    width="$(ffprobe -v error -select_streams v:0 -show_entries stream=width -of default=nw=1:nk=1 "$src")"
    height="$(ffprobe -v error -select_streams v:0 -show_entries stream=height -of default=nw=1:nk=1 "$src")"
    fps_raw="$(ffprobe -v error -select_streams v:0 -show_entries stream=avg_frame_rate -of default=nw=1:nk=1 "$src")"
    fps="$(awk -F/ '{ if (NF == 2 && $2 != 0) printf "%.6f", $1 / $2; else printf "%.6f", $1; }' <<<"$fps_raw")"
    frame_count="$(find "$tmp_frame_dir" -maxdepth 1 -name 'frame-*.png' | wc -l | tr -d ' ')"

    cat >"$tmp_frame_dir/metadata.json" <<EOF
{
  "fps": $fps,
  "width": $width,
  "height": $height,
  "frame_count": $frame_count
}
EOF

    # Commit: replace the previous decoded copy atomically.
    rm -rf "$frame_dir"
    mv "$tmp_frame_dir" "$frame_dir"
    mv "$tmp_audio_path" "$audio_path"
    trap - EXIT
done

echo "Decoded cutscenes written to $OUT_DIR"
echo "These assets are ignored by git. Run this once per machine before using native cutscenes."
