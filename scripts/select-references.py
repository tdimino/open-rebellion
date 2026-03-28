#!/usr/bin/env python3
"""Select best reference images for each DLL BMP using vision LLM.

For each source BMP, builds a contact sheet of candidate references from the
matching pack (via reference-packs.json routing), sends both to Gemini Flash
Lite via OpenRouter, and asks the LLM to pick the best 3-5 references.

Output: data/reference-selections.json — consumed by gemini-upscale.py.

Usage:
    uv run --with requests,pillow scripts/select-references.py --dry-run
    uv run --with requests,pillow scripts/select-references.py
    uv run --with requests,pillow scripts/select-references.py --pack portrait-major --limit 5
    uv run --with requests,pillow scripts/select-references.py --dll gokres --range 2000-2100
"""

import argparse
import base64
import io
import json
import math
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

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
REFS_DIR = PROJECT_DIR / "assets" / "references"
OUTPUT_PATH = PROJECT_DIR / "data" / "reference-selections.json"

DLL_DIRS = {
    "gokres": PROJECT_DIR / "data" / "base" / "ui" / "gokres-dll" / "BMP",
    "strategy": PROJECT_DIR / "data" / "base" / "ui" / "strategy-dll" / "BMP",
    "common": PROJECT_DIR / "data" / "base" / "ui" / "common-dll" / "BMP",
    "tactical": PROJECT_DIR / "data" / "base" / "ui" / "tactical-dll" / "BMP",
}

IMAGE_EXTENSIONS = {".png", ".jpg", ".jpeg", ".gif", ".webp", ".bmp"}

# Max references on a contact sheet before we pre-filter
CONTACT_SHEET_MAX = 36
CONTACT_SHEET_THUMB = 128  # thumbnail size in pixels


# ---------------------------------------------------------------------------
# Load config files
# ---------------------------------------------------------------------------

def load_packs() -> dict:
    packs_path = SCRIPT_DIR / "reference-packs.json"
    return json.loads(packs_path.read_text())


def load_entity_map() -> dict:
    map_path = PROJECT_DIR / "data" / "resource-entity-map.json"
    if map_path.exists():
        return json.loads(map_path.read_text())
    return {}


def load_existing_selections() -> dict:
    if OUTPUT_PATH.exists():
        return json.loads(OUTPUT_PATH.read_text())
    return {}


# ---------------------------------------------------------------------------
# Route BMP to pack
# ---------------------------------------------------------------------------

def route_bmp(dll: str, rid: int, routing: list[dict]) -> str | None:
    """Find which pack a BMP belongs to via routing rules."""
    for rule in routing:
        if rule["dll"] == dll and rule["range"][0] <= rid <= rule["range"][1]:
            return rule["pack"]
    return None


# ---------------------------------------------------------------------------
# Gather candidate references for a pack
# ---------------------------------------------------------------------------

def gather_candidates(pack: dict) -> list[Path]:
    """Collect all reference image paths from a pack's reference_dirs."""
    candidates = []
    for ref_dir_str in pack["reference_dirs"]:
        ref_dir = PROJECT_DIR / ref_dir_str
        if not ref_dir.exists():
            continue
        for f in sorted(ref_dir.rglob("*")):
            if f.is_file() and f.suffix.lower() in IMAGE_EXTENSIONS:
                if f.name in ("prompt-template.txt", ".DS_Store"):
                    continue
                candidates.append(f)
    return candidates


def pre_filter_by_entity(
    candidates: list[Path], entity_slug: str, faction: str | None
) -> list[Path]:
    """Pre-filter candidates by entity name slug match in filename."""
    # Split slug into parts for flexible matching
    slug_parts = entity_slug.split("-")

    scored: list[tuple[float, Path]] = []
    for c in candidates:
        fname = c.stem.lower()
        # Count how many slug parts appear in filename
        matches = sum(1 for part in slug_parts if part in fname)
        if matches == 0:
            continue
        score = matches / len(slug_parts)
        # Bonus for faction match
        if faction and faction in fname:
            score += 0.2
        scored.append((score, c))

    scored.sort(key=lambda x: -x[0])
    return [c for _, c in scored]


# ---------------------------------------------------------------------------
# Contact sheet builder
# ---------------------------------------------------------------------------

def build_contact_sheet(
    images: list[Path], thumb_size: int = CONTACT_SHEET_THUMB, max_images: int = CONTACT_SHEET_MAX
) -> tuple[bytes, list[str]]:
    """Build a labeled contact sheet grid. Returns (JPEG bytes, list of labels).

    Each cell has a number label (1, 2, 3...) overlaid on the thumbnail.
    """
    from PIL import Image, ImageDraw, ImageFont

    images = images[:max_images]
    n = len(images)
    if n == 0:
        return b"", []

    cols = min(6, n)
    rows = math.ceil(n / cols)

    # Label height below each thumbnail
    label_h = 16
    cell_w = thumb_size
    cell_h = thumb_size + label_h

    sheet_w = cols * cell_w
    sheet_h = rows * cell_h
    sheet = Image.new("RGB", (sheet_w, sheet_h), (32, 32, 32))
    draw = ImageDraw.Draw(sheet)

    labels = []
    for i, img_path in enumerate(images):
        col = i % cols
        row = i // cols
        x = col * cell_w
        y = row * cell_h

        try:
            img = Image.open(img_path)
            if img.mode in ("P", "RGBA", "LA"):
                img = img.convert("RGB")
            img.thumbnail((thumb_size, thumb_size), Image.LANCZOS)
            # Center in cell
            paste_x = x + (thumb_size - img.width) // 2
            paste_y = y + (thumb_size - img.height) // 2
            sheet.paste(img, (paste_x, paste_y))
        except Exception:
            pass

        # Draw number label
        label = str(i + 1)
        draw.text((x + 4, y + thumb_size + 1), label, fill=(255, 255, 100))
        # Truncated filename as secondary label
        fname = img_path.stem[:20]
        draw.text((x + 22, y + thumb_size + 1), fname, fill=(180, 180, 180))

        labels.append(str(img_path.relative_to(PROJECT_DIR)))

    buf = io.BytesIO()
    sheet.save(buf, format="JPEG", quality=80)
    return buf.getvalue(), labels


# ---------------------------------------------------------------------------
# Image preprocessing
# ---------------------------------------------------------------------------

def preprocess_bmp(path: Path, max_dim: int = 256) -> bytes:
    """Load and compress a source BMP for the API call."""
    from PIL import Image

    img = Image.open(path)
    if img.mode in ("P", "RGBA", "LA"):
        img = img.convert("RGB")
    w, h = img.size
    if max(w, h) > max_dim:
        scale = max_dim / max(w, h)
        img = img.resize((int(w * scale), int(h * scale)), Image.LANCZOS)
    buf = io.BytesIO()
    img.save(buf, format="JPEG", quality=80)
    return buf.getvalue()


# ---------------------------------------------------------------------------
# Vision LLM call
# ---------------------------------------------------------------------------

def select_references_via_llm(
    source_bytes: bytes,
    contact_bytes: bytes,
    n_candidates: int,
    pack_prompt: str,
    entity_name: str | None,
    max_refs: int,
    api_key: str,
) -> list[int]:
    """Ask vision LLM to pick the best reference indices from the contact sheet."""
    import requests

    source_b64 = base64.b64encode(source_bytes).decode()
    contact_b64 = base64.b64encode(contact_bytes).decode()

    prompt = (
        f"I'm upscaling this small game sprite from a 1998 strategy game based on the "
        f"original Star Wars trilogy (1977-1983). The upscale prompt is:\n"
        f'"{pack_prompt}"\n\n'
        f"The contact sheet shows {n_candidates} numbered candidate reference images.\n"
        f"Pick the {max_refs} best references that would most help guide the upscale.\n\n"
        f"RULES:\n"
        f"- Match by visual similarity to the source: subject, pose, color palette, angle\n"
        f"- ONLY select painted art, illustrations, or original trilogy film stills\n"
        f"- REJECT images that are mostly text, book pages, or tables of contents\n"
        f"- REJECT photos from post-2005 Star Wars (prequels, sequels, TV shows like "
        f"Andor, Mandalorian, Rogue One) — different actors, wrong era\n"
        f"- PREFER painted/illustrated art over photographs\n\n"
        f"Return ONLY the numbers separated by commas, e.g.: 3,7,12\n"
        f"Best {max_refs} references:"
    )

    resp = requests.post(
        OPENROUTER_URL,
        headers={
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json",
            "HTTP-Referer": "https://github.com/tdimino/open-rebellion",
            "X-Title": "Open Rebellion Reference Selector",
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
                                "url": f"data:image/jpeg;base64,{source_b64}",
                                "detail": "low",
                            },
                        },
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": f"data:image/jpeg;base64,{contact_b64}",
                                "detail": "high",
                            },
                        },
                        {"type": "text", "text": prompt},
                    ],
                }
            ],
            "max_tokens": 40,
            "temperature": 0.1,
        },
        timeout=60,
    )
    resp.raise_for_status()
    data = resp.json()
    text = data["choices"][0]["message"]["content"].strip()

    # Parse comma-separated numbers
    numbers = re.findall(r"\d+", text)
    indices = []
    for n in numbers:
        idx = int(n) - 1  # 1-indexed to 0-indexed
        if 0 <= idx < n_candidates:
            indices.append(idx)
    return indices[:max_refs]


# ---------------------------------------------------------------------------
# Main processing
# ---------------------------------------------------------------------------

def process_bmp(
    bmp_path: Path,
    dll: str,
    rid: int,
    pack_name: str,
    pack: dict,
    candidates: list[Path],
    entity_map: dict,
    api_key: str,
    dry_run: bool = False,
) -> dict | None:
    """Process a single BMP: build contact sheet, call LLM, return selection."""
    entity_name = None
    entity_slug = None
    faction = None

    # Check entity map for this resource ID
    entity = entity_map.get(str(rid))
    if entity:
        entity_name = entity.get("entity")
        entity_slug = entity.get("slug")
        faction = entity.get("faction")

    # Pre-filter candidates for large pools
    filtered = candidates
    if entity_slug and len(candidates) > CONTACT_SHEET_MAX:
        pre_filtered = pre_filter_by_entity(candidates, entity_slug, faction)
        if len(pre_filtered) >= pack["max_refs"]:
            filtered = pre_filtered[:CONTACT_SHEET_MAX]
        # If pre-filter returned too few, pad with random candidates
        if len(filtered) < CONTACT_SHEET_MAX:
            remaining = [c for c in candidates if c not in filtered]
            filtered = filtered + remaining[: CONTACT_SHEET_MAX - len(filtered)]
    elif len(candidates) > CONTACT_SHEET_MAX:
        # No entity — take evenly spaced samples
        step = len(candidates) / CONTACT_SHEET_MAX
        filtered = [candidates[int(i * step)] for i in range(CONTACT_SHEET_MAX)]

    if dry_run:
        return {
            "dll": dll,
            "resource_id": rid,
            "pack": pack_name,
            "entity": entity_name,
            "n_candidates": len(filtered),
            "references": [],  # dry-run
        }

    # Build contact sheet
    contact_bytes, labels = build_contact_sheet(filtered)
    if not contact_bytes:
        return None

    # Preprocess source BMP
    source_bytes = preprocess_bmp(bmp_path)

    # Call LLM
    indices = select_references_via_llm(
        source_bytes=source_bytes,
        contact_bytes=contact_bytes,
        n_candidates=len(filtered),
        pack_prompt=pack["prompt"],
        entity_name=entity_name,
        max_refs=pack["max_refs"],
        api_key=api_key,
    )

    selected_paths = [labels[i] for i in indices if i < len(labels)]

    return {
        "dll": dll,
        "resource_id": rid,
        "pack": pack_name,
        "entity": entity_name,
        "n_candidates": len(filtered),
        "references": selected_paths,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Select best references for DLL BMP upscaling")
    parser.add_argument("--dry-run", action="store_true", help="Preview without API calls")
    parser.add_argument("--pack", help="Only process BMPs for this pack")
    parser.add_argument("--dll", help="Only process BMPs from this DLL (gokres, strategy, common, tactical)")
    parser.add_argument("--range", help="Resource ID range, e.g. 2000-2100")
    parser.add_argument("--limit", type=int, default=0, help="Max BMPs to process (0 = all)")
    parser.add_argument("--force", action="store_true", help="Re-select even if already in selections.json")
    parser.add_argument("--output", type=Path, default=OUTPUT_PATH, help="Output JSON path")
    args = parser.parse_args()

    api_key = os.environ.get("OPENROUTER_API_KEY", "")
    if not api_key and not args.dry_run:
        print("ERROR: OPENROUTER_API_KEY not set", file=sys.stderr)
        sys.exit(1)

    packs_config = load_packs()
    packs = packs_config["packs"]
    routing = packs_config["routing"]
    entity_map = load_entity_map()
    existing = load_existing_selections()

    # Parse range filter
    range_min, range_max = 0, 999999
    if args.range:
        parts = args.range.split("-")
        range_min = int(parts[0])
        range_max = int(parts[1]) if len(parts) > 1 else range_min

    # Pre-build candidate lists per pack
    pack_candidates: dict[str, list[Path]] = {}
    for pack_name, pack in packs.items():
        pack_candidates[pack_name] = gather_candidates(pack)

    # Collect all BMPs to process
    work: list[tuple[Path, str, int, str]] = []  # (path, dll, rid, pack_name)
    for dll_name, bmp_dir in DLL_DIRS.items():
        if args.dll and dll_name != args.dll:
            continue
        if not bmp_dir.exists():
            continue
        for bmp in sorted(bmp_dir.glob("*.bmp")):
            rid = int(bmp.stem)
            if not (range_min <= rid <= range_max):
                continue
            pack_name = route_bmp(dll_name, rid, routing)
            if not pack_name:
                continue
            if args.pack and pack_name != args.pack:
                continue

            # Skip already-selected (keyed by "dll:rid")
            key = f"{dll_name}:{rid}"
            if key in existing and not args.force:
                continue

            work.append((bmp, dll_name, rid, pack_name))

    if args.limit:
        work = work[:args.limit]

    print(f"BMPs to process: {len(work)}")
    if args.dry_run:
        # Show pack distribution
        dist: dict[str, int] = {}
        for _, _, _, pn in work:
            dist[pn] = dist.get(pn, 0) + 1
        for pn, cnt in sorted(dist.items(), key=lambda x: -x[1]):
            n_cands = len(pack_candidates.get(pn, []))
            print(f"  {pn}: {cnt} BMPs, {n_cands} candidate refs")
        return

    # Process
    selections = dict(existing)  # start from existing
    processed = 0
    errors = 0
    t0 = time.time()

    for i, (bmp, dll, rid, pack_name) in enumerate(work, 1):
        key = f"{dll}:{rid}"
        pack = packs[pack_name]
        candidates = pack_candidates[pack_name]
        entity = entity_map.get(str(rid))
        entity_label = entity["entity"] if entity else ""

        print(f"  [{i}/{len(work)}] {dll}/{rid} ({pack_name}) {entity_label}...", end=" ", flush=True)

        try:
            result = process_bmp(
                bmp_path=bmp,
                dll=dll,
                rid=rid,
                pack_name=pack_name,
                pack=pack,
                candidates=candidates,
                entity_map=entity_map,
                api_key=api_key,
            )
            if result and result["references"]:
                selections[key] = result
                refs_str = ", ".join(Path(r).name for r in result["references"])
                print(f"→ {len(result['references'])} refs: {refs_str[:80]}")
                processed += 1
            else:
                print("→ no refs selected")
                errors += 1
        except Exception as e:
            print(f"→ ERROR: {e}")
            errors += 1

        # Rate limit: ~10 req/sec for Flash Lite
        time.sleep(0.15)

        # Checkpoint every 50 BMPs
        if i % 50 == 0:
            args.output.parent.mkdir(parents=True, exist_ok=True)
            args.output.write_text(json.dumps(selections, indent=2, ensure_ascii=False))
            elapsed = time.time() - t0
            rate = i / elapsed
            remaining = (len(work) - i) / rate if rate > 0 else 0
            print(f"  --- checkpoint: {processed} selected, {errors} errors, "
                  f"{rate:.1f} BMP/s, ~{remaining/60:.0f}m remaining ---")

    # Final save
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(selections, indent=2, ensure_ascii=False))

    elapsed = time.time() - t0
    print(f"\nDone. {processed} selections, {errors} errors in {elapsed:.0f}s")
    print(f"Output: {args.output}")


if __name__ == "__main__":
    main()
