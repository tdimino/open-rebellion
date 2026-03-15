#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
"""
Curate reference image collections for nano-banana-pro asset generation.

Extracts representative BMPs from the MetasharpNet editor, converts to RGB PNG,
and optionally upscales via waifu2x for better Gemini reference quality.

Usage:
    uv run scripts/curate-references.py                    # all categories
    uv run scripts/curate-references.py --category capital-ships  # one category
    uv run scripts/curate-references.py --upscale          # upscale via waifu2x (4x)
    uv run scripts/curate-references.py --list             # show categories and counts
"""

import argparse
import shutil
import subprocess
import sys
from pathlib import Path

from PIL import Image

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
REFS_DIR = PROJECT_DIR / "assets" / "references"

# MetasharpNet editor location
EDITOR_DIR = Path.home() / "Desktop" / "Programming" / "StarWarsRebellionEditor.NET" / "SwRebellionEditor"

# ── Reference collection definitions ──────────────────────────────────────────
# Each collection picks representative exemplars from the editor BMPs.
# "source_dir" is relative to EDITOR_DIR, "picks" are glob patterns or specific files.

COLLECTIONS: dict[str, dict] = {
    "ref-capital-ships": {
        "source_dir": "capital ships/EDATA",
        "picks": [
            "042-edata*mon-calamari*",     # Mon Cal Cruiser — iconic Rebel
            "044-edata*assault-frigate*",   # Assault Frigate
            "045-edata*nebulon-b*",         # Nebulon-B — iconic silhouette
            "047-edata*corellian-corvette*", # Tantive IV
            "057-edata*imperial-star*",      # ISD — iconic Imperial
            "063-edata*super-star*",         # SSD
        ],
        "prompt_template": (
            "Star Wars capital ship encyclopedia illustration, {description}, "
            "3/4 profile view on starfield background, detailed hull plating, "
            "engine glow, 1998 pre-rendered CGI aesthetic, "
            "same artistic style as reference images"
        ),
    },
    "ref-fighters": {
        "source_dir": "fighters/EDATA",
        "picks": [
            "036-edata*x-wing*",
            "037-edata*y-wing*",
            "034-edata*a-wing*",
            "035-edata*b-wing*",
        ],
        "prompt_template": (
            "Star Wars fighter encyclopedia illustration, {description}, "
            "side-profile view showing wing configuration, engine thrust glow, "
            "starfield background, 1998 pre-rendered CGI aesthetic, "
            "same artistic style as reference images"
        ),
    },
    "ref-characters": {
        "source_dir": "characters/EDATA",
        "picks": [
            "*luke-skywalker*",
            "*darth-vader*",
            "*ackbar*",
            "*mon-mothma*",
            "*thrawn*",
            "*han-solo*",
        ],
        "prompt_template": (
            "Star Wars character portrait, {description}, "
            "painted portrait with dark background, dramatic lighting from the left, "
            "1998 pre-rendered CGI aesthetic, "
            "same artistic style as reference images"
        ),
    },
    "ref-planets": {
        "source_dir": "systems/EDATA",
        "picks": [
            "*coruscant*",
            "*hoth*",
            "*tatooine*",
            "*endor*",
            "*yavin*",
            "*dagobah*",
            "*bespin*",
            "*kashyyyk*",
        ],
        "prompt_template": (
            "Star Wars planet encyclopedia illustration, {description}, "
            "painted background with atmospheric glow, orbital view, "
            "1998 pre-rendered CGI aesthetic, "
            "same artistic style as reference images"
        ),
    },
    "ref-troops": {
        "source_dir": "troops/EDATA",
        "picks": ["*"],  # All 10 troop types
        "max_picks": 8,
        "prompt_template": (
            "Star Wars ground military unit illustration, {description}, "
            "formation view with terrain background, "
            "1998 pre-rendered CGI aesthetic, "
            "same artistic style as reference images"
        ),
    },
    "ref-special-forces": {
        "source_dir": "special forces/EDATA",
        "picks": ["*"],  # All 9 types
        "max_picks": 6,
        "prompt_template": (
            "Star Wars special forces operative illustration, {description}, "
            "dark atmospheric background, covert mission aesthetic, "
            "1998 pre-rendered CGI aesthetic, "
            "same artistic style as reference images"
        ),
    },
    "ref-facilities": {
        "source_dir": "buildings/EDATA",
        "picks": ["*"],  # All facility types
        "max_picks": 9,
        "prompt_template": (
            "Star Wars military facility illustration, {description}, "
            "industrial/military architecture on planet surface, "
            "1998 pre-rendered CGI aesthetic, "
            "same artistic style as reference images"
        ),
    },
    "ref-missions": {
        "source_dir": "missions+events/EDATA",
        "picks": ["*"],
        "max_picks": 8,
        "prompt_template": (
            "Star Wars mission scene illustration, {description}, "
            "dramatic narrative composition, cinematic lighting, "
            "1998 pre-rendered CGI aesthetic, "
            "same artistic style as reference images"
        ),
    },
    "ref-damage-diagrams": {
        "source_dir": "capital ships/GOKRES.DLL/Bitmap",
        "picks": [
            "*damage*imperial-star*",
            "*damage*mon-calamari*",
            "*damage*nebulon*",
            "*damage*super-star*",
            "*damage*corvette*",
            "*damage*assault*",
        ],
        "prompt_template": (
            "Star Wars ship technical schematic, {description}, "
            "wire-frame damage diagram on dark background, blueprint aesthetic, "
            "same artistic style as reference images"
        ),
    },
    "ref-squadron-sprites": {
        "source_dir": "fighters/TACTICAL.DLL/303",
        "picks": [
            "*x-wing*close*",
            "*y-wing*close*",
            "*a-wing*close*",
            "*b-wing*close*",
            "*tie-fighter*close*",
            "*tie-interceptor*close*",
            "*tie-bomber*close*",
            "*tie-defender*close*",
        ],
        "prompt_template": (
            "Star Wars fighter sprite, {description}, "
            "top-down orthographic view, transparent background, "
            "pixel art game asset, same artistic style as reference images"
        ),
    },
    "ref-battle-backgrounds": {
        "source_dir": "systems/TACTICAL.DLL/303",
        "picks": [
            "*coruscant*",
            "*hoth*",
            "*endor*",
            "*tatooine*",
            "*yavin*",
            "*bespin*",
        ],
        "prompt_template": (
            "Star Wars space battle background, {description}, "
            "planet horizon with starfield, tactical combat view, "
            "1998 pre-rendered CGI aesthetic, "
            "same artistic style as reference images"
        ),
    },
}


def find_matching_files(source_dir: Path, patterns: list[str], max_picks: int = 14) -> list[Path]:
    """Find files matching glob patterns in source directory."""
    matches = []
    for pattern in patterns:
        found = sorted(source_dir.glob(f"{pattern}.bmp")) + sorted(source_dir.glob(pattern))
        for f in found:
            if f.is_file() and f not in matches:
                matches.append(f)
                if len(matches) >= max_picks:
                    return matches
    return matches


def convert_to_png(src: Path, dst: Path) -> None:
    """Convert BMP (possibly indexed palette) to RGB PNG."""
    img = Image.open(src).convert("RGB")
    dst.parent.mkdir(parents=True, exist_ok=True)
    img.save(dst, "PNG")


def upscale_file(src_png: Path, dst_png: Path, scale: int = 4) -> bool:
    """Upscale a single PNG via waifu2x wrapper. Returns True on success."""
    waifu2x = shutil.which("waifu2x") or shutil.which("waifu2x-ncnn-vulkan")
    if not waifu2x:
        return False
    result = subprocess.run(
        [waifu2x, "-i", str(src_png), "-o", str(dst_png), "-n", "1", "-s", str(scale), "-f", "png"],
        capture_output=True, text=True,
    )
    return result.returncode == 0 and dst_png.exists()


def curate_collection(name: str, config: dict, do_upscale: bool, quiet: bool) -> int:
    """Extract and convert reference images for one collection. Returns count."""
    source_dir = EDITOR_DIR / config["source_dir"]
    out_dir = REFS_DIR / name

    if not source_dir.exists():
        if not quiet:
            print(f"  [{name}] Source not found: {source_dir}")
        return 0

    max_picks = config.get("max_picks", 14)
    matches = find_matching_files(source_dir, config["picks"], max_picks)

    if not matches:
        if not quiet:
            print(f"  [{name}] No matching files in {source_dir}")
        return 0

    count = 0
    for f in matches:
        # Clean filename for output
        stem = f.stem.lower().replace(" ", "-")
        out_png = out_dir / f"{stem}.png"

        if out_png.exists():
            count += 1
            continue

        # Always save the original-resolution PNG
        orig_png = out_dir / f"{stem}-orig.png"
        if not orig_png.exists():
            convert_to_png(f, orig_png)

        if do_upscale:
            if not out_png.exists():
                if upscale_file(orig_png, out_png):
                    if not quiet:
                        print(f"  [{name}] {f.name} → {out_png.name} (4x) + {orig_png.name}")
                else:
                    # Fallback: copy original as the main ref
                    shutil.copy2(orig_png, out_png)
                    if not quiet:
                        print(f"  [{name}] {f.name} → {out_png.name} (no upscale)")
        else:
            if not out_png.exists():
                convert_to_png(f, out_png)
            if not quiet:
                print(f"  [{name}] {f.name} → {out_png.name} + {orig_png.name}")

        count += 1

    # Write prompt template
    template_path = out_dir / "prompt-template.txt"
    if not template_path.exists():
        template_path.write_text(config["prompt_template"] + "\n")

    return count


def main() -> None:
    parser = argparse.ArgumentParser(description="Curate reference image collections")
    parser.add_argument("--category", help="Single category to curate (e.g., capital-ships)")
    parser.add_argument("--upscale", action="store_true", help="Upscale via waifu2x (4x)")
    parser.add_argument("--list", action="store_true", help="List categories and source counts")
    parser.add_argument("--quiet", action="store_true")
    args = parser.parse_args()

    if not EDITOR_DIR.exists():
        print(f"MetasharpNet editor not found at {EDITOR_DIR}", file=sys.stderr)
        sys.exit(1)

    if args.list:
        print(f"{'Collection':<25s} {'Source Dir':<40s} {'Picks':<6s}")
        print("-" * 75)
        for name, config in COLLECTIONS.items():
            source_dir = EDITOR_DIR / config["source_dir"]
            count = len(list(source_dir.glob("*.bmp"))) if source_dir.exists() else 0
            max_p = config.get("max_picks", len(config["picks"]))
            print(f"{name:<25s} {config['source_dir']:<40s} {min(count, max_p):>3d}/{count}")
        return

    # Determine which collections to process
    if args.category:
        # Allow shorthand: "capital-ships" matches "ref-capital-ships"
        key = args.category if args.category.startswith("ref-") else f"ref-{args.category}"
        if key not in COLLECTIONS:
            print(f"Unknown category: {args.category}", file=sys.stderr)
            print(f"Available: {', '.join(c.removeprefix('ref-') for c in COLLECTIONS)}", file=sys.stderr)
            sys.exit(1)
        targets = {key: COLLECTIONS[key]}
    else:
        targets = COLLECTIONS

    print(f"Curating {len(targets)} reference collection(s)...")
    if args.upscale:
        print("Upscaling enabled (4x via waifu2x)")
    print()

    total = 0
    for name, config in targets.items():
        count = curate_collection(name, config, args.upscale, args.quiet)
        if not args.quiet:
            print(f"  [{name}] {count} reference image(s)")
        total += count

    print(f"\nTotal: {total} reference image(s) in {REFS_DIR}")


if __name__ == "__main__":
    main()
