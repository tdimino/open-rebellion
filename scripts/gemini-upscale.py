#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["google-genai", "pillow"]
# ///
"""
HD upscale DLL BMPs via Gemini Pro with vision-LLM-selected reference images.

Reads reference-selections.json (produced by select-references.py) to get the
best 2-5 reference images for each BMP. Sends source + references + pack-
specific prompt to Gemini 3 Pro Image for generative upscaling.

Output: data/hd/{dll-dir}/{resource_id}.png

Usage:
    # Full run (all BMPs with selections)
    uv run scripts/gemini-upscale.py

    # Dry-run: show what would be processed
    uv run scripts/gemini-upscale.py --dry-run

    # Single pack
    uv run scripts/gemini-upscale.py --pack portrait-major

    # Single DLL + range
    uv run scripts/gemini-upscale.py --dll gokres --range 2112-2120

    # Use Flash model (cheaper, lower quality)
    uv run scripts/gemini-upscale.py --fast

    # Limit concurrent requests
    uv run scripts/gemini-upscale.py --limit 10

    # Force re-upscale existing outputs
    uv run scripts/gemini-upscale.py --force

Prerequisites:
    GEMINI_API_KEY in environment
    data/reference-selections.json (run select-references.py first)
"""

import argparse
import json
import os
import sys
import time
from pathlib import Path

from google import genai
from google.genai import types
from PIL import Image

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
OUTPUT_BASE = PROJECT_DIR / "data" / "hd"
SELECTIONS_PATH = PROJECT_DIR / "data" / "reference-selections.json"

PRO_MODEL = "gemini-3-pro-image-preview"
FLASH_MODEL = "gemini-3.1-flash-image-preview"

DLL_DIRS = {
    "gokres": PROJECT_DIR / "data" / "base" / "ui" / "gokres-dll" / "BMP",
    "strategy": PROJECT_DIR / "data" / "base" / "ui" / "strategy-dll" / "BMP",
    "common": PROJECT_DIR / "data" / "base" / "ui" / "common-dll" / "BMP",
    "tactical": PROJECT_DIR / "data" / "base" / "ui" / "tactical-dll" / "BMP",
}

DLL_OUTPUT_DIRS = {
    "gokres": "gokres-dll",
    "strategy": "strategy-dll",
    "common": "common-dll",
    "tactical": "tactical-dll",
}

# Rate limit: Gemini Pro ~10 RPM for image generation
REQUEST_DELAY = 7.0  # seconds between requests


# ---------------------------------------------------------------------------
# Load config
# ---------------------------------------------------------------------------

def load_packs() -> dict:
    packs_path = SCRIPT_DIR / "reference-packs.json"
    return json.loads(packs_path.read_text())


def load_selections() -> dict:
    if not SELECTIONS_PATH.exists():
        print(f"ERROR: {SELECTIONS_PATH} not found", file=sys.stderr)
        print("Run select-references.py first.", file=sys.stderr)
        sys.exit(1)
    return json.loads(SELECTIONS_PATH.read_text())


# ---------------------------------------------------------------------------
# Upscale a single BMP
# ---------------------------------------------------------------------------

def upscale_bmp(
    client: genai.Client,
    model: str,
    bmp_path: Path,
    ref_paths: list[Path],
    prompt: str,
    output_path: Path,
) -> Path | None:
    """Upscale a BMP with Gemini using reference images."""
    # Build multi-image content: prompt + source + references
    source_img = Image.open(bmp_path)
    if source_img.mode in ("P", "RGBA", "LA"):
        source_img = source_img.convert("RGB")
    src_w, src_h = source_img.size

    # The source BMP is the sole authority on composition, subject, and color.
    # References only inform texture quality and painted brush-stroke style.
    # Never mention character/ship names — that causes Gemini to hallucinate
    # its own idea of the subject instead of faithfully enhancing the source.
    full_prompt = (
        f"IMAGE 1 is the source — a {src_w}x{src_h} pixel sprite from a 1998 strategy game. "
        f"IMAGES 2+ are style/texture references only.\n\n"
        f"{prompt}\n\n"
        f"CRITICAL: The output must be a faithful higher-resolution version of IMAGE 1. "
        f"Do NOT change the face, pose, silhouette, composition, or color palette. "
        f"Do NOT reimagine or reinterpret the subject. "
        f"Use the reference images ONLY to inform texture detail and painted style. "
        f"The source image is the ground truth — every pixel decision defaults to it."
    )

    contents: list = [full_prompt, source_img]

    # Add reference images
    for ref_path in ref_paths:
        if ref_path.exists():
            ref_img = Image.open(ref_path)
            if ref_img.mode in ("P", "RGBA", "LA"):
                ref_img = ref_img.convert("RGB")
            contents.append(ref_img)

    try:
        response = client.models.generate_content(
            model=model,
            contents=contents,
            config=types.GenerateContentConfig(
                response_modalities=["IMAGE", "TEXT"],
                temperature=0.4,
            ),
        )
    except Exception as e:
        print(f"API ERROR: {type(e).__name__}: {e}", file=sys.stderr)
        return None

    # Extract image from response
    if not response.candidates:
        print("ERROR: No candidates in response", file=sys.stderr)
        return None

    candidate = response.candidates[0]
    if not candidate.content or not candidate.content.parts:
        finish = getattr(candidate, "finish_reason", "unknown")
        safety = getattr(candidate, "safety_ratings", None)
        print(f"ERROR: No content (finish_reason={finish})", file=sys.stderr)
        if safety:
            for rating in safety:
                print(f"  Safety: {rating.category} = {rating.probability}", file=sys.stderr)
        return None

    for part in candidate.content.parts:
        if part.inline_data and part.inline_data.mime_type.startswith("image/"):
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_bytes(part.inline_data.data)
            out_img = Image.open(output_path)
            out_w, out_h = out_img.size
            size_kb = len(part.inline_data.data) / 1024
            print(f"  → {output_path.name} ({out_w}x{out_h}, {size_kb:.0f}KB)")
            return output_path

    # No image — show text response
    for part in candidate.content.parts:
        if part.text:
            print(f"  Model said: {part.text[:200]}", file=sys.stderr)
    return None


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(description="HD upscale DLL BMPs via Gemini with reference images")
    parser.add_argument("--dry-run", action="store_true", help="Show what would be processed")
    parser.add_argument("--pack", help="Only process BMPs for this pack")
    parser.add_argument("--dll", help="Only process BMPs from this DLL")
    parser.add_argument("--range", help="Resource ID range, e.g. 2112-2120")
    parser.add_argument("--limit", type=int, default=0, help="Max BMPs to process")
    parser.add_argument("--fast", action="store_true", help="Use Flash model (cheaper)")
    parser.add_argument("--force", action="store_true", help="Re-upscale existing outputs")
    parser.add_argument("--delay", type=float, default=REQUEST_DELAY, help="Seconds between API calls")
    args = parser.parse_args()

    api_key = os.environ.get("GEMINI_API_KEY", "")
    if not api_key and not args.dry_run:
        print("ERROR: GEMINI_API_KEY not set", file=sys.stderr)
        sys.exit(1)

    packs_config = load_packs()
    packs = packs_config["packs"]
    routing = packs_config["routing"]
    selections = load_selections()

    model = FLASH_MODEL if args.fast else PRO_MODEL

    # Parse range filter
    range_min, range_max = 0, 999999
    if args.range:
        parts = args.range.split("-")
        range_min = int(parts[0])
        range_max = int(parts[1]) if len(parts) > 1 else range_min

    # Build work list from selections
    work: list[tuple[str, dict]] = []  # (key, selection)
    for key, sel in selections.items():
        dll = sel["dll"]
        rid = sel["resource_id"]
        pack_name = sel["pack"]

        if args.dll and dll != args.dll:
            continue
        if args.pack and pack_name != args.pack:
            continue
        if not (range_min <= rid <= range_max):
            continue
        if not sel.get("references"):
            continue

        # Check if output already exists
        out_dir = OUTPUT_BASE / DLL_OUTPUT_DIRS[dll]
        out_path = out_dir / f"{rid}.png"
        if out_path.exists() and not args.force:
            continue

        work.append((key, sel))

    if args.limit:
        work = work[:args.limit]

    print(f"Model: {model}")
    print(f"BMPs to upscale: {len(work)}")

    if args.dry_run:
        dist: dict[str, int] = {}
        for _, sel in work:
            dist[sel["pack"]] = dist.get(sel["pack"], 0) + 1
        for pn, cnt in sorted(dist.items(), key=lambda x: -x[1]):
            print(f"  {pn}: {cnt}")
        # Cost estimate
        cost_per = 0.134 if not args.fast else 0.067
        total_cost = len(work) * cost_per
        print(f"\nEstimated cost: ${total_cost:.2f} ({len(work)} × ${cost_per})")
        return

    client = genai.Client(api_key=api_key)

    processed = 0
    errors = 0
    t0 = time.time()

    for i, (key, sel) in enumerate(work, 1):
        dll = sel["dll"]
        rid = sel["resource_id"]
        pack_name = sel["pack"]
        pack = packs[pack_name]
        bmp_path = DLL_DIRS[dll] / f"{rid}.bmp"
        if not bmp_path.exists():
            print(f"  [{i}/{len(work)}] SKIP: {bmp_path} not found")
            errors += 1
            continue

        out_dir = OUTPUT_BASE / DLL_OUTPUT_DIRS[dll]
        out_path = out_dir / f"{rid}.png"

        # Resolve reference paths
        ref_paths = [PROJECT_DIR / r for r in sel["references"]]
        ref_count = sum(1 for r in ref_paths if r.exists())

        label = sel.get("entity") or f"{dll}/{rid}"
        print(f"  [{i}/{len(work)}] {label} ({pack_name}, {ref_count} refs)...", end=" ", flush=True)

        result = upscale_bmp(
            client=client,
            model=model,
            bmp_path=bmp_path,
            ref_paths=ref_paths,
            prompt=pack["prompt"],
            output_path=out_path,
        )

        if result:
            processed += 1
        else:
            errors += 1

        # Rate limit
        if i < len(work):
            time.sleep(args.delay)

        # Progress checkpoint every 25 BMPs
        if i % 25 == 0:
            elapsed = time.time() - t0
            rate = i / elapsed * 60  # per minute
            remaining = (len(work) - i) / (rate / 60) if rate > 0 else 0
            print(f"  --- {processed} done, {errors} errors, "
                  f"{rate:.1f}/min, ~{remaining/60:.0f}m remaining ---")

    elapsed = time.time() - t0
    cost_per = 0.134 if not args.fast else 0.067
    total_cost = processed * cost_per

    print(f"\nDone. {processed} upscaled, {errors} errors in {elapsed:.0f}s")
    print(f"Estimated cost: ${total_cost:.2f}")
    print(f"Output: {OUTPUT_BASE}")


if __name__ == "__main__":
    main()
