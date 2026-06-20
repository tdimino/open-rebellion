#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC_DIR="/Users/seikenberg/Games/Windows/REBELLION/REBELLION/MDATA"
OUT_DIR="$ROOT/assets/references/cutscene-frames"

mkdir -p "$OUT_DIR"

shopt -s nullglob
sources=("$SRC_DIR"/MDATA.*)
shopt -u nullglob

for src in "${sources[@]}"; do
    ext="${src##*.}"
    name="$ext" # "000", "201", etc.
    
    frame_dir="$OUT_DIR/$name"
    audio_path="$OUT_DIR/$name.wav"
    tmp_frame_dir="$OUT_DIR/.$name.tmp"
    tmp_audio_path="$OUT_DIR/.$name.wav.tmp"

    echo "Decoding MDATA.$name"
    rm -rf "$tmp_frame_dir"
    mkdir -p "$tmp_frame_dir"
    rm -f "$tmp_audio_path"
    trap 'rm -rf "$tmp_frame_dir" "$tmp_audio_path"' EXIT

    if ! ffprobe -v error -show_streams "$src" | grep -q codec_type=video; then
        echo "No video stream in $name, skipping"
        continue
    fi

    ffmpeg -y -loglevel error -i "$src" "$tmp_frame_dir/frame-%05d.png"
    
    # Not all videos have audio. If this fails, we just don't create audio.
    if ffprobe -v error -show_streams "$src" | grep -q codec_type=audio; then
        ffmpeg -y -loglevel error -i "$src" -vn -f wav -c:a pcm_s16le "$tmp_audio_path"
    fi

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
    if [ -f "$tmp_audio_path" ]; then
        mv "$tmp_audio_path" "$audio_path"
    fi
    trap - EXIT
done

echo "Decoded MDATA cutscenes written to $OUT_DIR"
