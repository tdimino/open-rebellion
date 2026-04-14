#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow", "spandrel", "torch", "torchvision", "numpy", "replicate"]
# ///
"""
HD Upscaling Model Shootout — compare 8 upscale configurations on 20 representative BMPs.

Runs each model on 20 test BMPs (4 per category: portrait, ship, sprite, UI, event scene)
and generates a comparison HTML with lazy-loaded file-relative image paths.

Models:
  1. Real-ESRGAN x4plus (ncnn-vulkan, local)
  2. PBRify_UpscalerV4 (DAT2 via Spandrel+MPS, local)
  3. UltraSharpV2 (DAT2 via Spandrel+MPS, local)
  4. GTAV_rgt-s_dither (RGT via Spandrel+MPS, local)
  5. FSDedither Riven (ESRGAN via Spandrel+MPS, local)
  6. Vertex AI Imagen 4.0 (API, $0.005/img)
  7. Topaz Gigapixel CGI (Replicate API, $0.05/img)
  8. Palette reconstruction + PBRify (classical dedither + DAT2)

Usage:
    python3 scripts/model-shootout.py                    # Run all models
    python3 scripts/model-shootout.py --local-only       # Skip API models (free)
    python3 scripts/model-shootout.py --models 1,2,5     # Run specific models
    python3 scripts/model-shootout.py --html-only        # Regenerate comparison HTML
"""

import argparse
import gc
import json
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

import numpy as np
import torch
from PIL import Image

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------

PROJECT_ROOT = Path(__file__).resolve().parent.parent
SAMPLES_JSON = PROJECT_ROOT / "scripts" / "shootout-samples.json"
OUTPUT_DIR = PROJECT_ROOT / "data" / "hd" / "shootout"
MODELS_DIR = Path.home() / "tools" / "upscale-models"
REALESRGAN_BIN = Path.home() / "tools" / "realesrgan" / "realesrgan-ncnn-vulkan"

MODEL_FILES = {
    "pbrify": MODELS_DIR / "4x-PBRify_UpscalerV4.pth",
    "ultrasharp": MODELS_DIR / "4x-UltraSharpV2.pth",
    "gtav_dither": MODELS_DIR / "4xTextures_GTAV_rgt-s_dither.pth",
    "fsdedither_riven": MODELS_DIR / "4x-FSDedither-Riven.pth",
}

MODEL_CONFIGS = [
    {"id": 1, "name": "Real-ESRGAN x4plus", "slug": "realesrgan", "method": "ncnn"},
    {"id": 2, "name": "PBRify_UpscalerV4", "slug": "pbrify", "method": "spandrel"},
    {"id": 3, "name": "UltraSharpV2", "slug": "ultrasharp", "method": "spandrel"},
    {"id": 4, "name": "GTAV_rgt-s_dither", "slug": "gtav_dither", "method": "spandrel"},
    {"id": 5, "name": "FSDedither Riven", "slug": "fsdedither_riven", "method": "spandrel"},
    {"id": 6, "name": "Vertex AI Imagen 4.0", "slug": "vertex", "method": "vertex"},
    {"id": 7, "name": "Topaz Gigapixel CGI", "slug": "topaz", "method": "topaz"},
    {"id": 8, "name": "Palette Recon + PBRify", "slug": "palette_pbrify", "method": "palette_spandrel"},
]

# ---------------------------------------------------------------------------
# Tensor conversion
# ---------------------------------------------------------------------------

def pil_to_tensor(img: Image.Image, device: torch.device) -> torch.Tensor:
    """PIL Image -> BCHW float32 [0,1] tensor."""
    arr = np.array(img.convert("RGB"), dtype=np.float32) / 255.0
    return torch.from_numpy(arr).permute(2, 0, 1).unsqueeze(0).to(device)


def tensor_to_pil(tensor: torch.Tensor) -> Image.Image:
    """BCHW float32 tensor -> PIL Image."""
    arr = tensor.squeeze(0).clamp(0, 1).permute(1, 2, 0).cpu().float().numpy()
    return Image.fromarray((arr * 255.0).round().astype(np.uint8), mode="RGB")


# ---------------------------------------------------------------------------
# Palette reconstruction (classical de-dithering)
# ---------------------------------------------------------------------------

def palette_reconstruct(img: Image.Image) -> Image.Image:
    """Classical de-dithering for indexed-palette BMPs.

    For each 2x2 block, if pixels alternate between two palette colors,
    replace the block with their average — reconstructing the original
    gradient the dithering tried to approximate.
    """
    rgb = img.convert("RGB")

    arr = np.array(rgb, dtype=np.float32)
    h, w, _ = arr.shape
    out = arr.copy()

    # Process 2x2 blocks — detect dithering pattern
    for y in range(0, h - 1, 2):
        for x in range(0, w - 1, 2):
            block = arr[y:y+2, x:x+2]  # (2,2,3)
            # Check if block has exactly 2 unique colors in a checkerboard
            flat = block.reshape(4, 3)
            unique = np.unique(flat, axis=0)
            if len(unique) == 2:
                # Check for checkerboard pattern (diagonal pairs match)
                tl, tr, bl, br = flat[0], flat[1], flat[2], flat[3]
                is_checker = (np.array_equal(tl, br) and np.array_equal(tr, bl)
                              and not np.array_equal(tl, tr))
                if is_checker:
                    avg = unique.mean(axis=0)
                    out[y:y+2, x:x+2] = avg

    return Image.fromarray(out.astype(np.uint8), mode="RGB")


# ---------------------------------------------------------------------------
# Spandrel model cache
# ---------------------------------------------------------------------------

_spandrel_cache: dict[str, object] = {}
_device: torch.device | None = None


def get_device() -> torch.device:
    global _device
    if _device is None:
        if torch.backends.mps.is_available():
            _device = torch.device("mps")
            print(f"  Using MPS (Apple Silicon GPU)")
        else:
            _device = torch.device("cpu")
            print(f"  Using CPU")
    return _device


def load_spandrel_model(slug: str):
    """Load a Spandrel model, caching it for reuse."""
    if slug in _spandrel_cache:
        return _spandrel_cache[slug]

    from spandrel import ImageModelDescriptor, ModelLoader

    path = MODEL_FILES[slug]
    print(f"  Loading {path.name}...")
    model = ModelLoader().load_from_file(str(path))
    assert isinstance(model, ImageModelDescriptor)
    device = get_device()
    model.to(device).eval()
    _spandrel_cache[slug] = model
    return model


def unload_all_spandrel():
    """Free all cached models and GPU memory."""
    global _spandrel_cache
    _spandrel_cache.clear()
    gc.collect()
    if torch.backends.mps.is_available():
        torch.mps.empty_cache()


# ---------------------------------------------------------------------------
# Upscale methods
# ---------------------------------------------------------------------------

def upscale_ncnn(bmp_path: Path, out_path: Path) -> bool:
    """Real-ESRGAN via ncnn-vulkan CLI."""
    if not REALESRGAN_BIN.exists():
        print(f"  SKIP: realesrgan-ncnn-vulkan not found at {REALESRGAN_BIN}")
        return False

    # Convert BMP to temp PNG first (ncnn expects PNG/JPG)
    tmp_png = out_path.with_suffix(".tmp.png")
    img = Image.open(bmp_path).convert("RGB")
    img.save(tmp_png)

    try:
        result = subprocess.run(
            [str(REALESRGAN_BIN), "-i", str(tmp_png), "-o", str(out_path),
             "-n", "realesrgan-x4plus", "-s", "4", "-f", "png"],
            capture_output=True, text=True, timeout=60,
            cwd=str(REALESRGAN_BIN.parent),
        )
        return out_path.exists()
    except subprocess.TimeoutExpired:
        print(f"  TIMEOUT: Real-ESRGAN on {bmp_path.name}")
        return False
    finally:
        tmp_png.unlink(missing_ok=True)


def upscale_spandrel(bmp_path: Path, out_path: Path, model_slug: str) -> bool:
    """Upscale via Spandrel + MPS/CPU."""
    model = load_spandrel_model(model_slug)
    device = get_device()

    img = Image.open(bmp_path).convert("RGB")
    tensor = pil_to_tensor(img, device)

    try:
        with torch.no_grad():
            output = model(tensor)
        result = tensor_to_pil(output)
        result.save(out_path)
        del tensor, output
        return True
    except RuntimeError as e:
        if "mps" in str(e).lower():
            print(f"  MPS error, falling back to CPU: {e}")
            cpu_tensor = pil_to_tensor(img, torch.device("cpu"))
            cpu_model = load_spandrel_model(model_slug)
            # Temporarily move to CPU
            cpu_model_copy = cpu_model.model.cpu()
            with torch.no_grad():
                output = cpu_model_copy(cpu_tensor)
            result = tensor_to_pil(output)
            result.save(out_path)
            cpu_model.model.to(device)  # Move back
            del cpu_tensor, output
            return True
        raise


def upscale_palette_spandrel(bmp_path: Path, out_path: Path) -> bool:
    """Classical palette reconstruction + PBRify upscale."""
    img = Image.open(bmp_path)
    dedithered = palette_reconstruct(img)
    model = load_spandrel_model("pbrify")
    device = get_device()
    tensor = pil_to_tensor(dedithered, device)

    try:
        with torch.no_grad():
            output = model(tensor)
        result = tensor_to_pil(output)
        result.save(out_path)
        del tensor, output
        return True
    except RuntimeError as e:
        if "mps" in str(e).lower():
            print(f"  MPS error, falling back to CPU: {e}")
            cpu_tensor = pil_to_tensor(dedithered, torch.device("cpu"))
            cpu_model = model.model.cpu()
            with torch.no_grad():
                output = cpu_model(cpu_tensor)
            result = tensor_to_pil(output)
            result.save(out_path)
            model.model.to(device)
            del cpu_tensor, output
            return True
        raise


def upscale_vertex(bmp_path: Path, out_path: Path) -> bool:
    """Vertex AI Imagen 4.0 upscale."""
    try:
        from google.cloud import aiplatform
        from vertexai.preview.vision_models import Image as VImage, ImageGenerationModel
    except ImportError:
        print("  SKIP: google-cloud-aiplatform not installed")
        return False

    # Convert BMP to temp PNG
    tmp_png = out_path.with_suffix(".tmp.png")
    img = Image.open(bmp_path).convert("RGB")
    img.save(tmp_png)

    try:
        import vertexai
        project = os.environ.get("GOOGLE_CLOUD_PROJECT", "dream-daimon")
        vertexai.init(project=project, location="us-central1")

        model = ImageGenerationModel.from_pretrained("imagen-4.0-upscale-preview")
        vimg = VImage.load_from_file(str(tmp_png))
        response = model.upscale_image(image=vimg, upscale_factor="x4")

        if response and response.images:
            response.images[0].save(str(out_path))
            return True
        return False
    except Exception as e:
        print(f"  Vertex error: {e}")
        return False
    finally:
        tmp_png.unlink(missing_ok=True)


def upscale_topaz(bmp_path: Path, out_path: Path) -> bool:
    """Topaz Gigapixel CGI via Replicate API."""
    try:
        import replicate
    except ImportError:
        print("  SKIP: replicate not installed")
        return False

    if not os.environ.get("REPLICATE_API_TOKEN"):
        print("  SKIP: REPLICATE_API_TOKEN not set")
        return False

    # Convert BMP to PNG for upload
    tmp_png = out_path.with_suffix(".tmp.png")
    img = Image.open(bmp_path).convert("RGB")
    img.save(tmp_png)

    try:
        with open(tmp_png, "rb") as f:
            output = replicate.run(
                "topazlabs/image-upscale",
                input={
                    "image": f,
                    "enhance_model": "CGI",
                    "upscale_factor": "4x",
                    "output_format": "png",
                },
            )
        # output is a FileOutput object — read bytes directly
        if output:
            data = output.read()
            with open(out_path, "wb") as f2:
                f2.write(data)
            return True
        return False
    except Exception as e:
        print(f"  Topaz error: {e}")
        return False
    finally:
        tmp_png.unlink(missing_ok=True)


# ---------------------------------------------------------------------------
# HTML generation
# ---------------------------------------------------------------------------

def generate_comparison_html(samples: list[dict], models_run: list[dict]):
    """Generate comparison HTML with file-relative paths and lazy loading."""
    html_path = OUTPUT_DIR / "comparison.html"

    categories = {}
    for s in samples:
        cat = s["category"]
        if cat not in categories:
            categories[cat] = []
        categories[cat].append(s)

    lines = [
        "<!DOCTYPE html>",
        "<html><head>",
        "<meta charset='utf-8'>",
        "<title>Open Rebellion HD Upscaling — Model Shootout</title>",
        "<style>",
        "  body { background: #111; color: #eee; font-family: system-ui; margin: 20px; }",
        "  h1 { color: #ffd700; }",
        "  h2 { color: #e0b030; border-bottom: 1px solid #333; padding-bottom: 8px; }",
        "  h3 { color: #aaa; }",
        "  .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 16px; margin: 16px 0; }",
        "  .card { background: #1a1a1a; border: 1px solid #333; border-radius: 8px; padding: 12px; }",
        "  .card img { width: 100%; image-rendering: pixelated; border-radius: 4px; }",
        "  .card .label { font-size: 13px; color: #888; margin-top: 6px; }",
        "  .card .model-name { font-weight: bold; color: #ddd; }",
        "  .original { border-color: #ffd700; }",
        "  .original .label { color: #ffd700; }",
        "  .meta { font-size: 12px; color: #666; }",
        "  .nav { position: sticky; top: 0; background: #111; padding: 10px 0; z-index: 10; }",
        "  .nav a { color: #ffd700; margin-right: 16px; text-decoration: none; }",
        "  .nav a:hover { text-decoration: underline; }",
        "</style>",
        "</head><body>",
        "<h1>Open Rebellion HD Upscaling — Model Shootout</h1>",
        f"<p class='meta'>Generated {time.strftime('%Y-%m-%d %H:%M')} — "
        f"{len(samples)} samples x {len(models_run)} models = {len(samples) * len(models_run)} outputs</p>",
        "<div class='nav'>",
    ]

    for cat in categories:
        lines.append(f"  <a href='#{cat}'>{cat.title()}</a>")
    lines.append("</div>")

    for cat, cat_samples in categories.items():
        lines.append(f"<h2 id='{cat}'>{cat.title()}</h2>")

        for sample in cat_samples:
            dll = sample["dll"]
            rid = sample["resource_id"]
            lines.append(f"<h3>{dll} / {rid} ({sample['width']}x{sample['height']})</h3>")
            lines.append("<div class='grid'>")

            # Original (convert to PNG for display)
            orig_png = OUTPUT_DIR / "originals" / f"{dll}_{rid}.png"
            if orig_png.exists():
                rel = orig_png.relative_to(OUTPUT_DIR)
                lines.append(f"  <div class='card original'>")
                lines.append(f"    <img src='{rel}' loading='lazy'>")
                lines.append(f"    <div class='label'>ORIGINAL ({sample['width']}x{sample['height']})</div>")
                lines.append(f"  </div>")

            # Each model output
            for mc in models_run:
                slug = mc["slug"]
                out_png = OUTPUT_DIR / slug / f"{dll}_{rid}.png"
                if out_png.exists():
                    rel = out_png.relative_to(OUTPUT_DIR)
                    w, h = Image.open(out_png).size
                    lines.append(f"  <div class='card'>")
                    lines.append(f"    <img src='{rel}' loading='lazy'>")
                    lines.append(f"    <div class='label'><span class='model-name'>{mc['name']}</span> ({w}x{h})</div>")
                    lines.append(f"  </div>")

            lines.append("</div>")

    lines.extend(["</body></html>"])
    html_path.write_text("\n".join(lines))
    print(f"\nComparison HTML: {html_path}")
    return html_path


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="HD Upscaling Model Shootout")
    parser.add_argument("--local-only", action="store_true", help="Skip API models (Vertex, Topaz)")
    parser.add_argument("--models", type=str, help="Comma-separated model IDs to run (e.g., 1,2,5)")
    parser.add_argument("--html-only", action="store_true", help="Only regenerate comparison HTML")
    args = parser.parse_args()

    # Load samples
    with open(SAMPLES_JSON) as f:
        samples = json.load(f)
    print(f"Loaded {len(samples)} test samples from {SAMPLES_JSON.name}")

    # Filter models
    models_to_run = MODEL_CONFIGS[:]
    if args.local_only:
        models_to_run = [m for m in models_to_run if m["method"] not in ("vertex", "topaz")]
    if args.models:
        ids = set(int(x) for x in args.models.split(","))
        models_to_run = [m for m in models_to_run if m["id"] in ids]

    if args.html_only:
        generate_comparison_html(samples, models_to_run)
        return

    print(f"Running {len(models_to_run)} models on {len(samples)} samples")
    print(f"Models: {', '.join(m['name'] for m in models_to_run)}")

    # Create output dirs
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    (OUTPUT_DIR / "originals").mkdir(exist_ok=True)
    for mc in models_to_run:
        (OUTPUT_DIR / mc["slug"]).mkdir(exist_ok=True)

    # Save originals as RGB PNGs
    print("\nConverting originals to RGB PNG...")
    for sample in samples:
        src = PROJECT_ROOT / sample["path"]
        dst = OUTPUT_DIR / "originals" / f"{sample['dll']}_{sample['resource_id']}.png"
        if not dst.exists():
            img = Image.open(src).convert("RGB")
            img.save(dst)

    # Run each model
    for mc in models_to_run:
        slug = mc["slug"]
        method = mc["method"]
        print(f"\n{'='*60}")
        print(f"Model {mc['id']}: {mc['name']} ({method})")
        print(f"{'='*60}")

        t0 = time.time()
        success = 0
        skip = 0

        for i, sample in enumerate(samples):
            dll = sample["dll"]
            rid = sample["resource_id"]
            bmp_path = PROJECT_ROOT / sample["path"]
            out_path = OUTPUT_DIR / slug / f"{dll}_{rid}.png"

            if out_path.exists():
                print(f"  [{i+1}/{len(samples)}] {dll}/{rid} — cached")
                success += 1
                continue

            print(f"  [{i+1}/{len(samples)}] {dll}/{rid} ({sample['width']}x{sample['height']})...", end=" ", flush=True)

            try:
                ok = False
                if method == "ncnn":
                    ok = upscale_ncnn(bmp_path, out_path)
                elif method == "spandrel":
                    ok = upscale_spandrel(bmp_path, out_path, slug)
                elif method == "palette_spandrel":
                    ok = upscale_palette_spandrel(bmp_path, out_path)
                elif method == "vertex":
                    ok = upscale_vertex(bmp_path, out_path)
                    if ok:
                        time.sleep(13)  # 5 RPM rate limit
                elif method == "topaz":
                    ok = upscale_topaz(bmp_path, out_path)

                if ok:
                    w, h = Image.open(out_path).size
                    print(f"OK ({w}x{h})")
                    success += 1
                else:
                    print("FAILED")
                    skip += 1
            except Exception as e:
                print(f"ERROR: {e}")
                skip += 1

            # MPS memory management
            if method in ("spandrel", "palette_spandrel") and i % 10 == 9:
                gc.collect()
                if torch.backends.mps.is_available():
                    torch.mps.empty_cache()

        elapsed = time.time() - t0
        print(f"  Done: {success}/{len(samples)} succeeded, {skip} failed ({elapsed:.1f}s)")

        # Free model between different Spandrel models to save memory
        if method in ("spandrel", "palette_spandrel"):
            if slug in _spandrel_cache:
                del _spandrel_cache[slug]
                gc.collect()
                if torch.backends.mps.is_available():
                    torch.mps.empty_cache()

    # Generate comparison HTML
    unload_all_spandrel()
    generate_comparison_html(samples, models_to_run)
    print(f"\nShootout complete! Serve with:")
    print(f"  cd {OUTPUT_DIR} && python3 -m http.server 8080")


if __name__ == "__main__":
    main()
