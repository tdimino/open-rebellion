#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["google-genai", "pillow"]
# ///
"""
Generate Star Wars Rebellion visual assets using nano-banana-pro reference collections.

Each category has 6-14 curated reference images (originals + upscaled). The script
loads all references for the chosen category and passes them to Gemini 3 Pro Image
(or Flash via --fast) alongside a prompt to generate new assets in the same style.

Usage:
    # Single generation with category references
    uv run scripts/generate-rebellion-assets.py \
      --category capital-ships \
      --prompt "Interdictor Cruiser with four gravity well domes, 3/4 profile"

    # Use prompt template (fills {description})
    uv run scripts/generate-rebellion-assets.py \
      --category planets \
      --description "Volcanic planet with lava rivers and dark ash clouds"

    # Compare Pro vs Flash output
    uv run scripts/generate-rebellion-assets.py \
      --category fighters \
      --description "TIE Defender, three-wing configuration" \
      --compare

    # Batch from manifest
    uv run scripts/generate-rebellion-assets.py \
      --category planets \
      --manifest data/manifests/planets.json

    # Use originals only (not upscaled) as references
    uv run scripts/generate-rebellion-assets.py \
      --category characters \
      --description "Admiral Piett, Imperial officer" \
      --originals-only

    # List categories
    uv run scripts/generate-rebellion-assets.py --list

Prerequisites:
    GEMINI_API_KEY in environment
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

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
REFS_DIR = PROJECT_DIR / "assets" / "references"
OUTPUT_DIR = PROJECT_DIR / "data" / "generated"

PRO_MODEL = "gemini-3-pro-image-preview"
FLASH_MODEL = "gemini-3.1-flash-image-preview"


def get_api_key() -> str:
    key = os.environ.get("GEMINI_API_KEY", "")
    if not key:
        print("ERROR: GEMINI_API_KEY not set", file=sys.stderr)
        sys.exit(1)
    return key


def list_categories() -> dict[str, dict]:
    """Discover reference collections and their contents."""
    cats = {}
    for d in sorted(REFS_DIR.iterdir()):
        if not d.is_dir() or not d.name.startswith("ref-"):
            continue
        short_name = d.name.removeprefix("ref-")
        upscaled = sorted(f for f in d.glob("*.png") if "-orig" not in f.name and f.name != "prompt-template.txt")
        originals = sorted(d.glob("*-orig.png"))
        template = d / "prompt-template.txt"
        cats[short_name] = {
            "dir": d,
            "upscaled": upscaled,
            "originals": originals,
            "template": template.read_text().strip() if template.exists() else None,
            "count": len(upscaled),
        }
    return cats


def load_reference_images(category: dict, originals_only: bool = False) -> list[Path]:
    """Load reference image paths for a category."""
    if originals_only:
        return category["originals"]
    # Use upscaled versions (better detail for Gemini) — cap at 14 (API max)
    return category["upscaled"][:14]


def build_prompt(category: dict, prompt: str | None, description: str | None) -> str:
    """Build the generation prompt from template or explicit text."""
    if prompt:
        return prompt
    if description and category["template"]:
        return category["template"].replace("{description}", description)
    if description:
        return description
    print("ERROR: Provide --prompt or --description", file=sys.stderr)
    sys.exit(1)


def generate_image(
    client: genai.Client,
    model: str,
    prompt: str,
    ref_paths: list[Path],
    output_path: Path,
    quiet: bool = False,
) -> Path | None:
    """Generate an image with reference images via Gemini. Returns output path or None."""
    # Build contents: prompt text + reference images
    contents: list = [prompt + "\n\nMatch the artistic style of these reference images:"]

    for ref in ref_paths:
        img = Image.open(ref)
        contents.append(img)

    if not quiet:
        print(f"  Generating with {len(ref_paths)} references via {model}...")

    try:
        response = client.models.generate_content(
            model=model,
            contents=contents,
            config=types.GenerateContentConfig(
                response_modalities=["IMAGE", "TEXT"],
                temperature=0.6,
            ),
        )
    except Exception as e:
        print(f"  ERROR: {type(e).__name__}: {e}", file=sys.stderr)
        return None

    # Extract generated image from response
    if not response.candidates:
        print("  ERROR: No candidates in response", file=sys.stderr)
        return None

    candidate = response.candidates[0]
    if not candidate.content or not candidate.content.parts:
        finish = getattr(candidate, "finish_reason", "unknown")
        print(f"  ERROR: No content parts (finish_reason={finish})", file=sys.stderr)
        # Check for safety ratings
        safety = getattr(candidate, "safety_ratings", None)
        if safety:
            for rating in safety:
                print(f"  Safety: {rating.category} = {rating.probability}", file=sys.stderr)
        return None

    for part in candidate.content.parts:
        if part.inline_data and part.inline_data.mime_type.startswith("image/"):
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_bytes(part.inline_data.data)
            if not quiet:
                size_kb = len(part.inline_data.data) / 1024
                img = Image.open(output_path)
                print(f"  Saved: {output_path.name} ({img.size[0]}x{img.size[1]}, {size_kb:.0f} KB)")
            return output_path

    print("  ERROR: No image in response", file=sys.stderr)
    if not quiet:
        # Show text response for debugging
        for part in response.candidates[0].content.parts:
            if part.text:
                print(f"  Model said: {part.text[:200]}")
    return None


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate SW Rebellion assets with nano-banana-pro references")
    parser.add_argument("--category", help="Reference collection (e.g., capital-ships, planets)")
    parser.add_argument("--prompt", help="Full generation prompt")
    parser.add_argument("--description", help="Short description — inserted into category prompt template")
    parser.add_argument("--output", type=Path, help="Output file path")
    parser.add_argument("--filename", help="Output filename (without extension)")
    parser.add_argument("--manifest", type=Path, help="JSON manifest for batch generation")
    parser.add_argument("--compare", action="store_true", help="Generate with both Pro and Flash for comparison")
    parser.add_argument("--fast", action="store_true", help="Use Flash model (faster, cheaper)")
    parser.add_argument("--originals-only", action="store_true", help="Use original-resolution refs (not upscaled)")
    parser.add_argument("--list", action="store_true", help="List available categories")
    parser.add_argument("--quiet", action="store_true")
    args = parser.parse_args()

    categories = list_categories()

    if args.list:
        print(f"{'Category':<25s} {'Refs':<6s} {'Template':<8s}")
        print("-" * 45)
        for name, cat in categories.items():
            has_tmpl = "yes" if cat["template"] else "no"
            print(f"{name:<25s} {cat['count']:>3d}    {has_tmpl}")
        print(f"\nTotal: {sum(c['count'] for c in categories.values())} reference images in {len(categories)} categories")
        return

    if not args.category:
        print("ERROR: --category required (or use --list)", file=sys.stderr)
        sys.exit(1)

    if args.category not in categories:
        print(f"Unknown category: {args.category}", file=sys.stderr)
        print(f"Available: {', '.join(categories.keys())}", file=sys.stderr)
        sys.exit(1)

    cat = categories[args.category]
    key = get_api_key()
    client = genai.Client(api_key=key)

    # Batch mode
    if args.manifest:
        manifest = json.loads(args.manifest.read_text())
        items = manifest if isinstance(manifest, list) else manifest.get("items", [])
        ref_paths = load_reference_images(cat, args.originals_only)
        out_dir = args.output or OUTPUT_DIR / args.category

        print(f"Batch: {len(items)} items from {args.manifest.name}")
        print(f"Category: {args.category} ({len(ref_paths)} refs)")
        print()

        model = FLASH_MODEL if args.fast else PRO_MODEL
        success = 0
        for i, item in enumerate(items, 1):
            desc = item.get("description", item.get("prompt", ""))
            fname = item.get("filename", f"{args.category}_{i:03d}")
            prompt = build_prompt(cat, item.get("prompt"), desc)
            out_path = out_dir / f"{fname}.png"

            print(f"[{i}/{len(items)}] {fname}")
            if generate_image(client, model, prompt, ref_paths, out_path, args.quiet):
                success += 1
            time.sleep(2)  # Rate limiting

        print(f"\nBatch complete: {success}/{len(items)} generated in {out_dir}")
        return

    # Single generation
    prompt = build_prompt(cat, args.prompt, args.description)
    ref_paths = load_reference_images(cat, args.originals_only)

    print(f"Category: {args.category} ({len(ref_paths)} references)")
    print(f"Prompt: {prompt[:100]}{'...' if len(prompt) > 100 else ''}")
    print()

    if args.compare:
        # Generate with both models
        fname = args.filename or f"{args.category}_compare"
        out_dir = args.output.parent if args.output else OUTPUT_DIR / args.category

        pro_path = out_dir / f"{fname}_pro.png"
        flash_path = out_dir / f"{fname}_flash.png"

        print("=== Pro Model ===")
        generate_image(client, PRO_MODEL, prompt, ref_paths, pro_path, args.quiet)
        time.sleep(2)
        print("\n=== Flash Model ===")
        generate_image(client, FLASH_MODEL, prompt, ref_paths, flash_path, args.quiet)

        print(f"\nComparison saved to {out_dir}/")
    else:
        model = FLASH_MODEL if args.fast else PRO_MODEL
        fname = args.filename or f"{args.category}_gen"
        out_path = args.output or OUTPUT_DIR / args.category / f"{fname}.png"

        generate_image(client, model, prompt, ref_paths, out_path, args.quiet)


if __name__ == "__main__":
    main()
