#!/usr/bin/env python3
"""Batch rename reference images using vision LLM descriptions.

Uses OpenRouter → Gemini Flash Lite to describe each image in 5-8 words,
then renames the file to a slugified version of the description.

Usage:
    uv run --with requests,pillow scripts/batch-rename-references.py --dry-run
    uv run --with requests,pillow scripts/batch-rename-references.py
    uv run --with requests,pillow scripts/batch-rename-references.py --directory characters-pages
"""

import argparse
import base64
import io
import json
import os
import re
import sys
import time
from pathlib import Path

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

OPENROUTER_URL = "https://openrouter.ai/api/v1/chat/completions"
MODEL = "google/gemini-2.0-flash-lite-001"

PROMPT = (
    "Describe this Star Wars image in 5-8 words for use as a filename. "
    "Include character names, ship types, or specific content. Be specific. "
    "Examples: 'luke-skywalker-portrait-painted', 'star-destroyer-cutaway-engines', "
    "'rebel-base-hoth-illustration', 'blaster-rifle-technical-diagram', "
    "'mon-mothma-seated-briefing-room'. "
    "Return ONLY the description, nothing else."
)

IMAGE_EXTENSIONS = {".png", ".jpg", ".jpeg", ".gif", ".webp"}

# Directories to process (relative to assets/references/)
TARGET_DIRS = [
    "ref-illustrated-books/characters-pages",
    "ref-illustrated-books/planets-pages",
    "ref-illustrated-books/weapons-images",
    "ref-illustrated-books/starfighters-images",
    "ref-illustrated-books/rotj-pages",
    "ref-cross-sections",
]

# Directories already named — skip
SKIP_DIRS = [
    "ref-wookieepedia-portraits",
    "ref-ccg-card-art",
    "ref-gb-concept-art",
    "ref-galactic-battlegrounds",
]

# ---------------------------------------------------------------------------
# Image preprocessing
# ---------------------------------------------------------------------------

def preprocess_image(path: Path, max_dim: int = 512) -> bytes:
    """Downscale and JPEG-compress an image for cheap API calls."""
    from PIL import Image

    img = Image.open(path)
    # Convert palette/RGBA to RGB
    if img.mode in ("P", "RGBA", "LA"):
        img = img.convert("RGB")

    # Downscale to max_dim on longest side
    w, h = img.size
    if max(w, h) > max_dim:
        scale = max_dim / max(w, h)
        img = img.resize((int(w * scale), int(h * scale)), Image.LANCZOS)

    buf = io.BytesIO()
    img.save(buf, format="JPEG", quality=75)
    return buf.getvalue()


# ---------------------------------------------------------------------------
# Vision LLM call
# ---------------------------------------------------------------------------

def describe_image(image_bytes: bytes, api_key: str) -> str:
    """Send image to Gemini Flash Lite via OpenRouter, get description."""
    import requests

    b64 = base64.b64encode(image_bytes).decode()
    resp = requests.post(
        OPENROUTER_URL,
        headers={
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json",
            "HTTP-Referer": "https://github.com/tdimino/open-rebellion",
            "X-Title": "Open Rebellion Batch Rename",
        },
        json={
            "model": MODEL,
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": f"data:image/jpeg;base64,{b64}",
                                "detail": "low",
                            },
                        },
                        {"type": "text", "text": PROMPT},
                    ],
                }
            ],
            "max_tokens": 30,
            "temperature": 0.1,
        },
        timeout=30,
    )
    resp.raise_for_status()
    data = resp.json()
    return data["choices"][0]["message"]["content"].strip()


# ---------------------------------------------------------------------------
# Slugify
# ---------------------------------------------------------------------------

def slugify(text: str) -> str:
    """Convert description to a clean filename slug."""
    # Remove quotes, asterisks, markdown
    text = text.strip("'\"*` ")
    # Lowercase
    text = text.lower()
    # Replace non-alphanumeric with hyphens
    text = re.sub(r"[^a-z0-9]+", "-", text)
    # Collapse multiple hyphens
    text = re.sub(r"-+", "-", text)
    # Strip leading/trailing hyphens
    text = text.strip("-")
    # Truncate to 60 chars
    return text[:60]


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def process_directory(
    dir_path: Path,
    api_key: str,
    dry_run: bool = False,
    limit: int = 0,
    manifest_file=None,
):
    """Process all images in a directory."""
    images = sorted(
        p
        for p in dir_path.iterdir()
        if p.is_file() and p.suffix.lower() in IMAGE_EXTENSIONS
    )

    if limit > 0:
        images = images[:limit]

    total = len(images)
    renamed = 0
    errors = 0
    used_names = set()

    # Collect existing names to avoid collisions
    for p in dir_path.iterdir():
        if p.is_file():
            used_names.add(p.name)

    for i, img_path in enumerate(images, 1):
        try:
            # Preprocess
            img_bytes = preprocess_image(img_path)

            # Describe
            description = describe_image(img_bytes, api_key)
            slug = slugify(description)

            if not slug or len(slug) < 3:
                slug = f"unnamed-{img_path.stem}"

            # Build new filename
            new_name = f"{slug}{img_path.suffix.lower()}"

            # Handle collisions
            counter = 2
            base = slug
            while new_name in used_names and new_name != img_path.name:
                new_name = f"{base}-{counter}{img_path.suffix.lower()}"
                counter += 1

            new_path = img_path.parent / new_name

            # Skip if already correctly named
            if new_path == img_path:
                print(f"  [{i}/{total}] SKIP (already named): {img_path.name}")
                continue

            prefix = "DRY" if dry_run else " OK"
            print(f"  [{i}/{total}] {prefix}  {img_path.name} → {new_name}")

            if not dry_run:
                # Remove old name from used set, add new
                used_names.discard(img_path.name)
                img_path.rename(new_path)
                used_names.add(new_name)
                renamed += 1

                if manifest_file:
                    manifest_file.write(
                        json.dumps(
                            {
                                "dir": str(dir_path),
                                "old": img_path.name,
                                "new": new_name,
                                "description": description,
                            }
                        )
                        + "\n"
                    )

            # Rate limit: ~60 req/min
            time.sleep(0.5)

        except Exception as e:
            print(f"  [{i}/{total}] ERROR {img_path.name}: {e}", file=sys.stderr)
            errors += 1
            time.sleep(1)  # Back off on error

    return total, renamed, errors


def main():
    parser = argparse.ArgumentParser(description="Batch rename reference images via vision LLM")
    parser.add_argument("--dry-run", action="store_true", help="Preview renames without executing")
    parser.add_argument("--directory", type=str, help="Process only this subdirectory (e.g. 'characters-pages')")
    parser.add_argument("--limit", type=int, default=0, help="Process only first N images per directory")
    parser.add_argument("--base", type=str, default="assets/references", help="Base references directory")
    args = parser.parse_args()

    # Get API key
    api_key = os.environ.get("OPENROUTER_API_KEY")
    if not api_key:
        print("ERROR: OPENROUTER_API_KEY not set. Source ~/.config/env/secrets.env first.", file=sys.stderr)
        sys.exit(1)

    base = Path(args.base)
    if not base.exists():
        print(f"ERROR: Base directory not found: {base}", file=sys.stderr)
        sys.exit(1)

    # Determine which directories to process
    if args.directory:
        # Find the directory in any of the target dirs
        found = None
        for td in TARGET_DIRS:
            candidate = base / td
            if candidate.name == args.directory or str(td) == args.directory:
                found = candidate
                break
        if not found:
            found = base / args.directory
        if not found.exists():
            print(f"ERROR: Directory not found: {found}", file=sys.stderr)
            sys.exit(1)
        dirs_to_process = [found]
    else:
        dirs_to_process = [base / td for td in TARGET_DIRS if (base / td).exists()]

    # Open manifest
    manifest_path = base / "renames.jsonl"
    mode = "a" if manifest_path.exists() else "w"

    print(f"{'DRY RUN — ' if args.dry_run else ''}Processing {len(dirs_to_process)} directories")
    print(f"Provider: OpenRouter → {MODEL}")
    print(f"Manifest: {manifest_path}")
    if args.limit:
        print(f"Limit: {args.limit} images per directory")
    print()

    total_all = 0
    renamed_all = 0
    errors_all = 0

    with open(manifest_path, mode) as mf:
        for dir_path in dirs_to_process:
            count = sum(1 for p in dir_path.iterdir() if p.suffix.lower() in IMAGE_EXTENSIONS)
            print(f"=== {dir_path.name} ({count} images) ===")
            total, renamed, errors = process_directory(
                dir_path, api_key, dry_run=args.dry_run, limit=args.limit, manifest_file=mf
            )
            total_all += total
            renamed_all += renamed
            errors_all += errors
            print()

    print(f"Done. {total_all} processed, {renamed_all} renamed, {errors_all} errors.")
    if args.dry_run:
        print("(Dry run — no files were actually renamed)")


if __name__ == "__main__":
    main()
