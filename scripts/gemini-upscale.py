#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["google-genai", "google-cloud-aiplatform", "pillow"]
# ///
"""
Hybrid HD upscaler — Gemini Pro for portraits, Vertex AI Imagen for everything else.

Routes each BMP to the best upscaler based on reference-packs.json `upscale_method`:
  - "gemini": Gemini Pro with vision-LLM-selected reference images (portraits only)
  - "vertex": Vertex AI Imagen 4.0 non-generative super-resolution (everything else)

Output: data/hd/{dll-dir}/{resource_id}.png

Usage:
    uv run scripts/gemini-upscale.py                          # all BMPs
    uv run scripts/gemini-upscale.py --dry-run                 # preview
    uv run scripts/gemini-upscale.py --pack portrait-major     # single pack
    uv run scripts/gemini-upscale.py --method vertex           # force vertex for all
    uv run scripts/gemini-upscale.py --method gemini           # force gemini for all
    uv run scripts/gemini-upscale.py --dll gokres --range 2112-2120
    uv run scripts/gemini-upscale.py --force --limit 10

Prerequisites:
    GEMINI_API_KEY for Gemini packs
    gcloud auth application-default login for Vertex packs
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


def load_packs() -> dict:
    return json.loads((SCRIPT_DIR / "reference-packs.json").read_text())


def load_selections() -> dict:
    if SELECTIONS_PATH.exists():
        return json.loads(SELECTIONS_PATH.read_text())
    return {}


def route_bmp(dll: str, rid: int, routing: list[dict]) -> str | None:
    for rule in routing:
        if rule["dll"] == dll and rule["range"][0] <= rid <= rule["range"][1]:
            return rule["pack"]
    return None


# ---------------------------------------------------------------------------
# Vertex AI Imagen upscale (non-generative)
# ---------------------------------------------------------------------------

_vertex_model = None

def upscale_vertex(bmp_path: Path, output_path: Path, factor: int = 4) -> Path | None:
    """Upscale via Vertex AI Imagen 4.0 — pure super-resolution, no hallucination."""
    global _vertex_model
    try:
        from google.cloud import aiplatform
        from vertexai.preview.vision_models import Image as VImage, ImageGenerationModel
    except ImportError:
        print("ERROR: google-cloud-aiplatform not installed", file=sys.stderr)
        return None

    if _vertex_model is None:
        project = os.environ.get("GOOGLE_CLOUD_PROJECT", os.environ.get("GCLOUD_PROJECT"))
        if not project:
            import subprocess
            result = subprocess.run(["gcloud", "config", "get-value", "project"],
                                    capture_output=True, text=True)
            project = result.stdout.strip()
        if not project:
            print("ERROR: No GCP project set", file=sys.stderr)
            return None
        location = os.environ.get("GOOGLE_CLOUD_LOCATION", "us-central1")
        aiplatform.init(project=project, location=location)
        _vertex_model = ImageGenerationModel.from_pretrained("imagen-4.0-upscale-preview")

    # Convert BMP to PNG for Vertex
    img = Image.open(bmp_path)
    if img.mode in ("P", "RGBA", "LA"):
        img = img.convert("RGB")
    tmp_png = output_path.parent / f"_tmp_{output_path.stem}.png"
    tmp_png.parent.mkdir(parents=True, exist_ok=True)
    img.save(tmp_png, "PNG")

    try:
        source = VImage.load_from_file(str(tmp_png))
        response = _vertex_model.upscale_image(image=source, upscale_factor=f"x{factor}")

        if hasattr(response, 'images') and response.images:
            response.images[0].save(str(output_path))
        elif hasattr(response, 'save'):
            response.save(str(output_path))
        elif hasattr(response, '_image_bytes'):
            output_path.write_bytes(response._image_bytes)
        else:
            print(f"ERROR: Unknown response type: {type(response)}", file=sys.stderr)
            return None

        out_img = Image.open(output_path)
        print(f"  → {output_path.name} ({out_img.size[0]}x{out_img.size[1]}) [vertex]")
        return output_path
    except Exception as e:
        print(f"VERTEX ERROR: {type(e).__name__}: {e}", file=sys.stderr)
        return None
    finally:
        tmp_png.unlink(missing_ok=True)


# ---------------------------------------------------------------------------
# Gemini Pro upscale (with references)
# ---------------------------------------------------------------------------

def upscale_gemini(
    client, model: str, bmp_path: Path, ref_paths: list[Path],
    prompt: str, output_path: Path,
) -> Path | None:
    """Upscale via Gemini Pro with reference images — for portraits only."""
    from google.genai import types

    source_img = Image.open(bmp_path)
    if source_img.mode in ("P", "RGBA", "LA"):
        source_img = source_img.convert("RGB")
    src_w, src_h = source_img.size

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
    for ref_path in ref_paths:
        if ref_path.exists():
            try:
                ref_img = Image.open(ref_path)
                if ref_img.mode in ("P", "RGBA", "LA"):
                    ref_img = ref_img.convert("RGB")
                contents.append(ref_img)
            except Exception:
                pass

    try:
        response = client.models.generate_content(
            model=model, contents=contents,
            config=types.GenerateContentConfig(
                response_modalities=["IMAGE", "TEXT"], temperature=0.4,
            ),
        )
    except Exception as e:
        print(f"GEMINI ERROR: {type(e).__name__}: {e}", file=sys.stderr)
        return None

    if not response.candidates:
        return None
    candidate = response.candidates[0]
    if not candidate.content or not candidate.content.parts:
        finish = getattr(candidate, "finish_reason", "unknown")
        print(f"ERROR: No content (finish_reason={finish})", file=sys.stderr)
        return None

    for part in candidate.content.parts:
        if part.inline_data and part.inline_data.mime_type.startswith("image/"):
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_bytes(part.inline_data.data)
            out_img = Image.open(output_path)
            size_kb = len(part.inline_data.data) / 1024
            print(f"  → {output_path.name} ({out_img.size[0]}x{out_img.size[1]}, {size_kb:.0f}KB) [gemini]")
            return output_path

    for part in candidate.content.parts:
        if part.text:
            print(f"  Model said: {part.text[:200]}", file=sys.stderr)
    return None


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(description="Hybrid HD upscaler — Gemini for portraits, Vertex for the rest")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--pack", help="Only process this pack")
    parser.add_argument("--dll", help="Only process this DLL")
    parser.add_argument("--range", help="Resource ID range, e.g. 2112-2120")
    parser.add_argument("--limit", type=int, default=0)
    parser.add_argument("--method", choices=["gemini", "vertex"], help="Force method for all packs")
    parser.add_argument("--fast", action="store_true", help="Use Gemini Flash instead of Pro")
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--delay", type=float, default=7.0)
    parser.add_argument("--vertex-factor", type=int, default=4, choices=[2, 4])
    args = parser.parse_args()

    packs_config = load_packs()
    packs = packs_config["packs"]
    routing = packs_config["routing"]
    selections = load_selections()

    range_min, range_max = 0, 999999
    if args.range:
        parts = args.range.split("-")
        range_min = int(parts[0])
        range_max = int(parts[1]) if len(parts) > 1 else range_min

    # Build work list from ALL BMPs via routing (not just selections)
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

            pack = packs[pack_name]
            method = args.method or pack.get("upscale_method", "vertex")

            out_path = OUTPUT_BASE / DLL_OUTPUT_DIRS[dll_name] / f"{rid}.png"
            if out_path.exists() and not args.force:
                continue

            # For gemini method, get reference selections
            sel_key = f"{dll_name}:{rid}"
            sel = selections.get(sel_key, {})
            refs = sel.get("references", [])

            work.append({
                "bmp": bmp, "dll": dll_name, "rid": rid,
                "pack_name": pack_name, "method": method,
                "out_path": out_path, "refs": refs,
                "entity": sel.get("entity"),
            })

    if args.limit:
        work = work[:args.limit]

    # Count by method
    gemini_count = sum(1 for w in work if w["method"] == "gemini")
    vertex_count = sum(1 for w in work if w["method"] == "vertex")
    print(f"BMPs to upscale: {len(work)} (gemini: {gemini_count}, vertex: {vertex_count})")

    if args.dry_run:
        dist: dict[str, dict] = {}
        for w in work:
            pn = w["pack_name"]
            if pn not in dist:
                dist[pn] = {"count": 0, "method": w["method"]}
            dist[pn]["count"] += 1
        for pn, d in sorted(dist.items(), key=lambda x: -x[1]["count"]):
            print(f"  {pn}: {d['count']} [{d['method']}]")
        gemini_cost = gemini_count * 0.134
        vertex_cost = vertex_count * 0.005  # Vertex is much cheaper
        print(f"\nEstimated cost: ${gemini_cost + vertex_cost:.2f} "
              f"(gemini: {gemini_count}×$0.134=${gemini_cost:.2f}, "
              f"vertex: {vertex_count}×$0.005=${vertex_cost:.2f})")
        return

    # Init clients as needed
    gemini_client = None
    if gemini_count > 0:
        from google import genai
        api_key = os.environ.get("GEMINI_API_KEY", "")
        if not api_key:
            print("ERROR: GEMINI_API_KEY not set (needed for portrait packs)", file=sys.stderr)
            sys.exit(1)
        gemini_client = genai.Client(api_key=api_key)

    gemini_model = FLASH_MODEL if args.fast else PRO_MODEL
    processed = 0
    errors = 0
    t0 = time.time()

    for i, w in enumerate(work, 1):
        label = w["entity"] or f"{w['dll']}/{w['rid']}"
        ref_count = len([r for r in w["refs"] if (PROJECT_DIR / r).exists()]) if w["refs"] else 0
        method_tag = w["method"]
        print(f"  [{i}/{len(work)}] {label} ({w['pack_name']}, {method_tag}"
              + (f", {ref_count} refs" if ref_count else "") + ")...", end=" ", flush=True)

        if w["method"] == "vertex":
            result = upscale_vertex(w["bmp"], w["out_path"], args.vertex_factor)
        else:
            pack = packs[w["pack_name"]]
            ref_paths = [PROJECT_DIR / r for r in w["refs"]]
            result = upscale_gemini(
                gemini_client, gemini_model, w["bmp"], ref_paths,
                pack["prompt"], w["out_path"],
            )

        if result:
            processed += 1
        else:
            errors += 1

        if i < len(work):
            delay = args.delay if w["method"] == "gemini" else 13.0  # Vertex: 5 RPM quota
            time.sleep(delay)

        if i % 25 == 0:
            elapsed = time.time() - t0
            rate = i / elapsed * 60
            remaining = (len(work) - i) / (rate / 60) if rate > 0 else 0
            print(f"  --- {processed} done, {errors} errors, "
                  f"{rate:.1f}/min, ~{remaining/60:.0f}m remaining ---")

    elapsed = time.time() - t0
    print(f"\nDone. {processed} upscaled, {errors} errors in {elapsed:.0f}s")
    print(f"Output: {OUTPUT_BASE}")


if __name__ == "__main__":
    main()
