#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["google-cloud-aiplatform", "pillow"]
# ///
"""
Batch upscale DLL BMPs via Vertex AI Imagen 4.0 (non-generative super-resolution).

Pure faithful enlargement — no hallucination, no references needed.
Respects the 5 RPM quota with 13s delay between requests.

Output: data/hd/{dll-dir}/{resource_id}.png

Usage:
    uv run scripts/vertex-upscale-batch.py                     # all BMPs
    uv run scripts/vertex-upscale-batch.py --dry-run            # preview
    uv run scripts/vertex-upscale-batch.py --pack ship-status   # single pack
    uv run scripts/vertex-upscale-batch.py --dll gokres --range 1000-1999
    uv run scripts/vertex-upscale-batch.py --limit 10 --force

Prerequisites:
    gcloud auth application-default login
    GCP project with Vertex AI API enabled (default: dream-daimon)
"""

import argparse
import json
import os
import sys
import time
from pathlib import Path

from PIL import Image

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
OUTPUT_BASE = PROJECT_DIR / "data" / "hd"

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


def load_packs() -> dict:
    return json.loads((SCRIPT_DIR / "reference-packs.json").read_text())


def route_bmp(dll: str, rid: int, routing: list[dict]) -> str | None:
    for rule in routing:
        if rule["dll"] == dll and rule["range"][0] <= rid <= rule["range"][1]:
            return rule["pack"]
    return None


_model = None


def upscale_one(bmp_path: Path, output_path: Path, factor: int = 4) -> Path | None:
    global _model
    from google.cloud import aiplatform
    from vertexai.preview.vision_models import Image as VImage, ImageGenerationModel

    if _model is None:
        project = os.environ.get("GOOGLE_CLOUD_PROJECT", "dream-daimon")
        location = os.environ.get("GOOGLE_CLOUD_LOCATION", "us-central1")
        aiplatform.init(project=project, location=location)
        _model = ImageGenerationModel.from_pretrained("imagen-4.0-upscale-preview")

    # Convert BMP to PNG for Vertex
    img = Image.open(bmp_path)
    if img.mode in ("P", "RGBA", "LA"):
        img = img.convert("RGB")
    tmp_png = output_path.parent / f"_tmp_{output_path.stem}.png"
    tmp_png.parent.mkdir(parents=True, exist_ok=True)
    img.save(tmp_png, "PNG")

    try:
        source = VImage.load_from_file(str(tmp_png))
        response = _model.upscale_image(image=source, upscale_factor=f"x{factor}")

        if hasattr(response, "images") and response.images:
            response.images[0].save(str(output_path))
        elif hasattr(response, "save"):
            response.save(str(output_path))
        elif hasattr(response, "_image_bytes"):
            output_path.write_bytes(response._image_bytes)
        else:
            return None

        out_img = Image.open(output_path)
        print(f"  → {output_path.name} ({out_img.size[0]}x{out_img.size[1]})")
        return output_path
    except Exception as e:
        print(f"  ERROR: {type(e).__name__}: {e}", file=sys.stderr)
        return None
    finally:
        tmp_png.unlink(missing_ok=True)


def main() -> None:
    parser = argparse.ArgumentParser(description="Batch Vertex AI Imagen 4.0 upscale for DLL BMPs")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--pack", help="Only process this pack")
    parser.add_argument("--dll", help="Only process this DLL")
    parser.add_argument("--range", help="Resource ID range, e.g. 1000-1999")
    parser.add_argument("--limit", type=int, default=0)
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--factor", type=int, default=4, choices=[2, 4])
    parser.add_argument("--delay", type=float, default=13.0, help="Seconds between requests (5 RPM quota)")
    args = parser.parse_args()

    packs_config = load_packs()
    packs = packs_config["packs"]
    routing = packs_config["routing"]

    range_min, range_max = 0, 999999
    if args.range:
        parts = args.range.split("-")
        range_min = int(parts[0])
        range_max = int(parts[1]) if len(parts) > 1 else range_min

    # Build work list
    work: list[dict] = []
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

            out_path = OUTPUT_BASE / DLL_OUTPUT_DIRS[dll_name] / f"{rid}.png"
            if out_path.exists() and not args.force:
                continue

            work.append({"bmp": bmp, "dll": dll_name, "rid": rid,
                         "pack": pack_name, "out": out_path})

    if args.limit:
        work = work[:args.limit]

    print(f"BMPs to upscale: {len(work)} (Vertex AI Imagen 4.0, {args.factor}x)")

    if args.dry_run:
        dist: dict[str, int] = {}
        for w in work:
            dist[w["pack"]] = dist.get(w["pack"], 0) + 1
        for pn, cnt in sorted(dist.items(), key=lambda x: -x[1]):
            print(f"  {pn}: {cnt}")
        cost = len(work) * 0.005
        eta_min = len(work) * args.delay / 60
        print(f"\nEstimated: ${cost:.2f}, ~{eta_min:.0f} min at {args.delay}s/req")
        return

    processed = 0
    errors = 0
    t0 = time.time()

    for i, w in enumerate(work, 1):
        print(f"  [{i}/{len(work)}] {w['dll']}/{w['rid']} ({w['pack']})...", end=" ", flush=True)

        result = upscale_one(w["bmp"], w["out"], args.factor)
        if result:
            processed += 1
        else:
            errors += 1

        if i < len(work):
            time.sleep(args.delay)

        if i % 25 == 0:
            elapsed = time.time() - t0
            rate = i / elapsed * 60
            remaining = (len(work) - i) / (rate / 60) if rate > 0 else 0
            print(f"  --- {processed}/{i} done, {errors} errors, ~{remaining/60:.0f}m remaining ---")

    elapsed = time.time() - t0
    print(f"\nDone. {processed} upscaled, {errors} errors in {elapsed:.0f}s")
    print(f"Output: {OUTPUT_BASE}")


if __name__ == "__main__":
    main()
