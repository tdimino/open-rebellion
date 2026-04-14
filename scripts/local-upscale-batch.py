#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow", "spandrel", "torch", "torchvision", "numpy"]
# ///
"""
Batch HD upscale all DLL BMPs using UltraSharp V2 via Spandrel + MPS.

Processes all BMPs in data/base/ui/{dll}/BMP/ and outputs 4x PNG to
data/hd/{dll}/{resource_id}.png. Skips already-processed files.

The BmpCache in bmp_cache.rs picks up HD overrides automatically:
  data/hd/{dll-name}/{resource_id}.png

Usage:
    python3 scripts/local-upscale-batch.py                  # All DLLs
    python3 scripts/local-upscale-batch.py --dll gokres-dll # One DLL
    python3 scripts/local-upscale-batch.py --dry-run        # Count only
    python3 scripts/local-upscale-batch.py                   # Resumes by default (skips existing)
"""

import argparse
import gc
import os
import sys
import time
from pathlib import Path

import numpy as np
import torch
from PIL import Image
from spandrel import ImageModelDescriptor, ModelLoader

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

PROJECT_ROOT = Path(__file__).resolve().parent.parent
BMP_BASE = PROJECT_ROOT / "data" / "base" / "ui"
HD_BASE = PROJECT_ROOT / "data" / "hd"
MODEL_PATH = Path.home() / "tools" / "upscale-models" / "4x-UltraSharpV2.pth"

DLL_DIRS = ["common-dll", "gokres-dll", "strategy-dll", "tactical-dll"]

# MPS cache flush interval — prevents OOM on long runs
FLUSH_INTERVAL = 50


# ---------------------------------------------------------------------------
# Tensor I/O
# ---------------------------------------------------------------------------

def pil_to_tensor(img: Image.Image, device: torch.device) -> torch.Tensor:
    arr = np.array(img.convert("RGB"), dtype=np.float32) / 255.0
    return torch.from_numpy(arr).permute(2, 0, 1).unsqueeze(0).to(device)


def tensor_to_pil(tensor: torch.Tensor) -> Image.Image:
    arr = tensor.squeeze(0).clamp(0, 1).permute(1, 2, 0).cpu().float().numpy()
    return Image.fromarray((arr * 255.0).round().astype(np.uint8), mode="RGB")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="Batch HD upscale with UltraSharp V2")
    parser.add_argument("--dll", type=str, help="Process only this DLL (e.g., gokres-dll)")
    parser.add_argument("--dry-run", action="store_true", help="Count files only, don't process")
    parser.add_argument("--force", action="store_true", help="Re-process existing HD PNGs")
    args = parser.parse_args()

    dlls = [args.dll] if args.dll else DLL_DIRS

    # Collect work list
    work = []
    skipped = 0
    for dll in dlls:
        bmp_dir = BMP_BASE / dll / "BMP"
        hd_dir = HD_BASE / dll
        if not bmp_dir.exists():
            print(f"SKIP: {bmp_dir} not found")
            continue
        for f in sorted(bmp_dir.iterdir()):
            if not f.suffix.lower() == ".bmp":
                continue
            rid = f.stem
            out_path = hd_dir / f"{rid}.png"
            if out_path.exists() and not args.force:
                skipped += 1
                continue
            work.append((f, out_path, dll, rid))

    total = len(work) + skipped
    print(f"Found {total} BMPs across {len(dlls)} DLLs")
    print(f"  Already done: {skipped}")
    print(f"  To process:   {len(work)}")

    if args.dry_run or not work:
        return

    # Create output dirs
    for dll in dlls:
        (HD_BASE / dll).mkdir(parents=True, exist_ok=True)

    # Load model
    print(f"\nLoading {MODEL_PATH.name}...")
    model = ModelLoader().load_from_file(str(MODEL_PATH))
    assert isinstance(model, ImageModelDescriptor)

    if torch.backends.mps.is_available():
        device = torch.device("mps")
        print("Using MPS (Apple Silicon GPU)")
    else:
        device = torch.device("cpu")
        print("Using CPU")

    model.to(device).eval()

    # Process
    t0 = time.time()
    success = 0
    errors = 0

    for i, (bmp_path, out_path, dll, rid) in enumerate(work):
        pct = (i + 1) / len(work) * 100
        elapsed = time.time() - t0
        rate = (i + 1) / elapsed if elapsed > 0 else 0
        eta = (len(work) - i - 1) / rate if rate > 0 else 0

        print(f"  [{i+1:4d}/{len(work)}] {pct:5.1f}% {dll}/{rid}", end="", flush=True)

        try:
            img = Image.open(bmp_path).convert("RGB")
            tensor = pil_to_tensor(img, device)

            with torch.no_grad():
                output = model(tensor)

            result = tensor_to_pil(output)
            result.save(out_path)

            w, h = result.size
            print(f"  -> {w}x{h}  ({rate:.1f} img/s, ETA {eta:.0f}s)")

            del tensor, output, result
            success += 1

        except Exception as e:
            print(f"  ERROR: {e}")
            errors += 1

        # MPS memory management
        if (i + 1) % FLUSH_INTERVAL == 0:
            gc.collect()
            if torch.backends.mps.is_available():
                torch.mps.empty_cache()

    elapsed = time.time() - t0
    print(f"\nDone in {elapsed:.1f}s ({elapsed/60:.1f} min)")
    print(f"  Success: {success}")
    print(f"  Errors:  {errors}")
    print(f"  Skipped: {skipped}")
    print(f"  Total:   {success + skipped}/{total}")


if __name__ == "__main__":
    main()
