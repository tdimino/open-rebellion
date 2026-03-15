#!/usr/bin/env bash
# Optimize GLBs from models-staging/ for Open Rebellion tactical combat
#
# Pipeline: strip textures → weld → simplify → prune → DRACO compress
# Adapted from World War Watcher's prepare-models.sh.
#
# Unlike WWW (which renders 3D in Three.js), Open Rebellion pre-renders
# models to sprite sheets via Blender (macroquad has no glTF loader).
# The optimized GLBs here are intermediate — fed to render-sprite-sheets.py.
#
# Usage:
#   bash scripts/prepare-rebellion-models.sh                    # all staging GLBs
#   bash scripts/prepare-rebellion-models.sh isd-hunyuan.glb    # specific model
#   bash scripts/prepare-rebellion-models.sh --preview          # also write pre-DRACO for QA
#
# Prerequisites:
#   npx gltf-transform --version  (@gltf-transform/cli)
#   node (for strip-textures.mjs if present)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
STAGING_DIR="$SCRIPT_DIR/models-staging"
OUTPUT_DIR="$SCRIPT_DIR/../data/models/optimized"
PREVIEW_DIR="$SCRIPT_DIR/models-optimized"

# Less aggressive simplification than WWW (5%) — tactical sprites need
# recognizable silhouettes at larger screen sizes than globe icons.
SIMPLIFY_RATIO=0.10
SIMPLIFY_ERROR=1.0

# DRACO quantization — 14-bit position minimum (11-bit destroys silhouettes)
QUANTIZE_POSITION=14
QUANTIZE_NORMAL=8
QUANTIZE_TEXCOORD=10

WRITE_PREVIEW=false

# Parse flags
POSITIONAL=()
for arg in "$@"; do
  case "$arg" in
    --preview) WRITE_PREVIEW=true ;;
    *) POSITIONAL+=("$arg") ;;
  esac
done
set -- "${POSITIONAL[@]+"${POSITIONAL[@]}"}"

# Ensure gltf-transform is available
if ! npx gltf-transform --version >/dev/null 2>&1; then
  echo "ERROR: gltf-transform not found. Run: npm install -g @gltf-transform/cli"
  exit 1
fi

if [ ! -d "$STAGING_DIR" ]; then
  echo "ERROR: Staging directory not found: $STAGING_DIR"
  echo "Run generate-rebellion-models.py first."
  exit 1
fi

mkdir -p "$OUTPUT_DIR"
[ "$WRITE_PREVIEW" = true ] && mkdir -p "$PREVIEW_DIR"

# Determine which files to process
if [ $# -gt 0 ]; then
  FILES=("$@")
else
  FILES=()
  for f in "$STAGING_DIR"/*.glb; do
    [ -f "$f" ] || continue
    FILES+=("$(basename "$f")")
  done
fi

if [ ${#FILES[@]} -eq 0 ]; then
  echo "No GLB files found in $STAGING_DIR"
  exit 0
fi

echo "Processing ${#FILES[@]} GLB(s)..."
echo ""

TOTAL_IN=0
TOTAL_OUT=0
PROCESSED=0

for filename in "${FILES[@]}"; do
  src="$STAGING_DIR/$filename"
  if [ ! -f "$src" ]; then
    echo "SKIP: $filename (not found in staging)"
    continue
  fi

  basename="${filename%.glb}"
  out="$OUTPUT_DIR/$filename"
  tmp_dir=$(mktemp -d)

  in_size=$(stat -f%z "$src" 2>/dev/null || stat -c%s "$src" 2>/dev/null)
  in_kb=$((in_size / 1024))
  echo "[$basename] ${in_kb} KB input"

  # Step 1: Copy to temp (strip-textures.mjs is optional for this pipeline
  # since we're rendering sprites, but stripping reduces optimize time)
  cp "$src" "$tmp_dir/input.glb"
  current="$tmp_dir/input.glb"

  # Step 2a: Simplify geometry (separate step — optimize doesn't expose ratio/error flags)
  echo "  simplify: ratio=${SIMPLIFY_RATIO}, error=${SIMPLIFY_ERROR}..."
  npx gltf-transform simplify \
    "$current" "$tmp_dir/simplified.glb" \
    --ratio "$SIMPLIFY_RATIO" \
    --error "$SIMPLIFY_ERROR" \
    2>/dev/null || {
      echo "  WARNING: simplify failed, using input as-is"
      cp "$current" "$tmp_dir/simplified.glb"
    }
  current="$tmp_dir/simplified.glb"

  # Step 2b: Optimize (weld + prune)
  echo "  optimize: weld, prune..."
  npx gltf-transform optimize \
    "$current" "$tmp_dir/optimized.glb" \
    2>/dev/null || {
      echo "  WARNING: optimize failed, using simplified version"
      cp "$current" "$tmp_dir/optimized.glb"
    }
  current="$tmp_dir/optimized.glb"

  # Step 2b: Generate normals if missing
  npx gltf-transform normals "$current" "$tmp_dir/normals.glb" 2>/dev/null || {
    cp "$current" "$tmp_dir/normals.glb"
  }
  current="$tmp_dir/normals.glb"

  # Write pre-DRACO preview if requested
  if [ "$WRITE_PREVIEW" = true ]; then
    cp "$current" "$PREVIEW_DIR/$filename"
  fi

  # Step 3: DRACO compress
  echo "  draco: pos=${QUANTIZE_POSITION}bit, normal=${QUANTIZE_NORMAL}bit..."
  # Note: edgebreaker is the internal default for mesh connectivity — not a CLI flag
  npx gltf-transform draco \
    "$current" "$out" \
    --quantize-position "$QUANTIZE_POSITION" \
    --quantize-normal "$QUANTIZE_NORMAL" \
    --quantize-texcoord "$QUANTIZE_TEXCOORD" \
    2>/dev/null || {
      echo "  WARNING: DRACO failed, copying optimized version"
      cp "$current" "$out"
    }

  out_size=$(stat -f%z "$out" 2>/dev/null || stat -c%s "$out" 2>/dev/null)
  out_kb=$((out_size / 1024))
  ratio=$((out_size * 100 / in_size))

  echo "  output: ${out_kb} KB (${ratio}% of original)"

  TOTAL_IN=$((TOTAL_IN + in_size))
  TOTAL_OUT=$((TOTAL_OUT + out_size))
  PROCESSED=$((PROCESSED + 1))

  # QA flags
  if [ "$out_size" -gt 512000 ]; then
    echo "  WARNING: output > 500 KB"
  fi

  rm -rf "$tmp_dir"
  echo ""
done

# Final audit
echo "════════════════════════════════════════════════"
echo "Processed: $PROCESSED model(s)"
echo "Total input:  $((TOTAL_IN / 1024)) KB"
echo "Total output: $((TOTAL_OUT / 1024)) KB"
if [ "$TOTAL_IN" -gt 0 ]; then
  echo "Compression:  $((TOTAL_OUT * 100 / TOTAL_IN))%"
fi
echo ""
echo "Output: $OUTPUT_DIR/"
echo ""

# Per-model audit
echo "Model Audit:"
echo "────────────────────────────────────────────────"
printf "%-35s %8s\n" "Model" "Size"
echo "────────────────────────────────────────────────"
for f in "$OUTPUT_DIR"/*.glb; do
  [ -f "$f" ] || continue
  name=$(basename "$f")
  size=$(stat -f%z "$f" 2>/dev/null || stat -c%s "$f" 2>/dev/null)
  kb=$((size / 1024))
  flag=""
  [ "$size" -gt 512000 ] && flag=" WARNING"
  [ "$size" -lt 100 ] && flag=" LOW-SIZE"
  printf "%-35s %6d KB%s\n" "$name" "$kb" "$flag"
done
echo "────────────────────────────────────────────────"
echo ""
echo "Next: blender --background --python scripts/render-sprite-sheets.py"
