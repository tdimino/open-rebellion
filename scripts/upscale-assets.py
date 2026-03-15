#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
"""
HD Asset Upscaler — batch upscale original Rebellion BMPs to PNG via waifu2x

Converts 256-color indexed palette BMPs to RGB, then upscales via
waifu2x-ncnn-vulkan (Homebrew, native Apple Silicon). Falls back to
PIL-only nearest-neighbor if waifu2x is not installed.

Usage:
    uv run scripts/upscale-assets.py                          # all EData BMPs
    uv run scripts/upscale-assets.py --input EDATA.042        # single file
    uv run scripts/upscale-assets.py --scale 8                # 8x for hero assets
    uv run scripts/upscale-assets.py --denoise 0              # no denoising
    uv run scripts/upscale-assets.py --dry-run                # preview without processing
    uv run scripts/upscale-assets.py --method pil             # PIL only (no waifu2x)

Prerequisites:
    brew install waifu2x-ncnn-vulkan   # recommended
"""

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from PIL import Image

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
BASE_DIR = PROJECT_DIR / "data" / "base" / "EData"
HD_DIR = PROJECT_DIR / "data" / "hd" / "EData"
MANIFEST_PATH = PROJECT_DIR / "data" / "hd" / "manifest.json"


def find_waifu2x() -> str | None:
    """Find waifu2x-ncnn-vulkan binary."""
    return shutil.which("waifu2x-ncnn-vulkan")


def file_hash(path: Path) -> str:
    """SHA-256 hash of file contents."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def convert_bmp_to_rgb_png(src: Path, dst: Path) -> None:
    """Convert indexed-palette BMP to RGB PNG for upscaler input."""
    img = Image.open(src).convert("RGB")
    dst.parent.mkdir(parents=True, exist_ok=True)
    img.save(dst, "PNG")


def upscale_waifu2x(input_dir: Path, output_dir: Path, scale: int, denoise: int) -> bool:
    """Batch upscale via waifu2x-ncnn-vulkan. Returns True on success."""
    binary = find_waifu2x()
    if not binary:
        return False

    output_dir.mkdir(parents=True, exist_ok=True)
    cmd = [
        binary,
        "-i", str(input_dir),
        "-o", str(output_dir),
        "-n", str(denoise),
        "-s", str(scale),
        "-f", "png",
    ]

    print(f"Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, capture_output=True, text=True)

    if result.returncode != 0:
        print(f"waifu2x error: {result.stderr}", file=sys.stderr)
        return False

    return True


def upscale_pil(src: Path, dst: Path, scale: int) -> None:
    """Fallback: PIL nearest-neighbor upscale (no AI, preserves pixels)."""
    img = Image.open(src).convert("RGB")
    new_size = (img.width * scale, img.height * scale)
    upscaled = img.resize(new_size, Image.NEAREST)
    dst.parent.mkdir(parents=True, exist_ok=True)
    upscaled.save(dst, "PNG")


def load_manifest() -> dict:
    """Load existing manifest or return empty dict."""
    if MANIFEST_PATH.exists():
        return json.loads(MANIFEST_PATH.read_text())
    return {}


def save_manifest(manifest: dict) -> None:
    """Save manifest to disk."""
    MANIFEST_PATH.parent.mkdir(parents=True, exist_ok=True)
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")


def list_edata_files(base_dir: Path, single: str | None = None) -> list[Path]:
    """List EDATA files to process."""
    if single:
        path = base_dir / single
        if path.exists():
            return [path]
        # Try with EDATA. prefix
        path = base_dir / f"EDATA.{single}"
        if path.exists():
            return [path]
        print(f"File not found: {single} in {base_dir}", file=sys.stderr)
        return []

    files = sorted(base_dir.glob("EDATA.*"))
    if not files:
        # Also try extensionless numbered files
        files = sorted(f for f in base_dir.iterdir() if f.is_file() and f.name.startswith("EDATA"))
    return files


def main() -> None:
    parser = argparse.ArgumentParser(description="Upscale Rebellion BMPs to HD PNGs")
    parser.add_argument("--input", help="Single file to process (e.g., EDATA.042)")
    parser.add_argument("--scale", type=int, default=4, help="Upscale factor (default: 4)")
    parser.add_argument("--denoise", type=int, default=1, help="waifu2x denoise level -1..3 (default: 1)")
    parser.add_argument("--method", choices=["waifu2x", "pil", "auto"], default="auto",
                        help="Upscaling method (default: auto = waifu2x if available)")
    parser.add_argument("--base-dir", type=Path, default=BASE_DIR, help="Source BMP directory")
    parser.add_argument("--output-dir", type=Path, default=HD_DIR, help="Output PNG directory")
    parser.add_argument("--dry-run", action="store_true", help="List files without processing")
    parser.add_argument("--force", action="store_true", help="Re-process even if manifest hash matches")
    args = parser.parse_args()

    # Find source files
    files = list_edata_files(args.base_dir, args.input)
    if not files:
        print(f"No EDATA files found in {args.base_dir}")
        print("Extract EData/ from a legal copy of Star Wars Rebellion first.")
        sys.exit(1)

    print(f"Found {len(files)} EDATA file(s) in {args.base_dir}")

    if args.dry_run:
        for f in files:
            try:
                img = Image.open(f)
                w, h = img.size
                mode = img.mode
                out_w, out_h = w * args.scale, h * args.scale
                print(f"  {f.name}: {w}x{h} ({mode}) → {out_w}x{out_h} PNG")
            except Exception as e:
                print(f"  {f.name}: ERROR reading — {e}")
        return

    # Load manifest for cache invalidation
    manifest = load_manifest()

    # Determine method
    if args.method == "waifu2x":
        if not find_waifu2x():
            print("waifu2x-ncnn-vulkan not found. Install or add to PATH.", file=sys.stderr)
            print("  Binary: ~/tools/waifu2x/waifu2x-ncnn-vulkan-20250915-macos/", file=sys.stderr)
            print("  Wrapper: ~/.local/bin/waifu2x", file=sys.stderr)
            sys.exit(1)
        use_waifu2x = True
    elif args.method == "auto":
        use_waifu2x = bool(find_waifu2x())
    else:
        use_waifu2x = False

    if use_waifu2x:
        print(f"Method: waifu2x-ncnn-vulkan (scale={args.scale}x, denoise={args.denoise})")
    else:
        print(f"Method: PIL nearest-neighbor (scale={args.scale}x)")

    if use_waifu2x:
        # waifu2x processes directories — convert BMPs to RGB PNGs in a temp dir first
        with tempfile.TemporaryDirectory(prefix="rebellion-upscale-") as tmpdir:
            tmp_input = Path(tmpdir) / "input"
            tmp_output = Path(tmpdir) / "output"
            tmp_input.mkdir()

            skipped = 0
            converted_files: list[tuple[Path, str, str]] = []  # (src_path, out_name, hash)
            for f in files:
                src_hash = file_hash(f)
                out_name = f"EDATA_{f.name.split('.')[-1]}.png" if "." in f.name else f"{f.name}.png"

                # Check manifest for unchanged files
                if not args.force and manifest.get(f.name, {}).get("hash") == src_hash:
                    skipped += 1
                    continue

                # Convert palette BMP → RGB PNG for waifu2x input
                convert_bmp_to_rgb_png(f, tmp_input / out_name)
                converted_files.append((f, out_name, src_hash))

            if skipped:
                print(f"Skipped {skipped} unchanged file(s)")

            if not converted_files:
                print("All files up to date.")
                save_manifest(manifest)
                return

            print(f"Converting {len(converted_files)} file(s)...")

            # Run waifu2x on the temp directory
            if not upscale_waifu2x(tmp_input, tmp_output, args.scale, args.denoise):
                print("waifu2x failed, falling back to PIL", file=sys.stderr)
                for f, out_name, src_hash in converted_files:
                    upscale_pil(f, args.output_dir / out_name, args.scale)
                    manifest[f.name] = {"hash": src_hash, "output": out_name, "scale": args.scale}
            else:
                # Copy results to final output, update manifest only for files that exist
                args.output_dir.mkdir(parents=True, exist_ok=True)
                for f, out_name, src_hash in converted_files:
                    src_png = tmp_output / out_name
                    if src_png.exists():
                        shutil.copy2(src_png, args.output_dir / out_name)
                        manifest[f.name] = {"hash": src_hash, "output": out_name, "scale": args.scale}

    else:
        # PIL mode: process one at a time
        for i, f in enumerate(files, 1):
            src_hash = file_hash(f)
            out_name = f"EDATA_{f.name.split('.')[-1]}.png" if "." in f.name else f"{f.name}.png"

            if not args.force and manifest.get(f.name, {}).get("hash") == src_hash:
                continue

            print(f"  [{i}/{len(files)}] {f.name} → {out_name}")
            upscale_pil(f, args.output_dir / out_name, args.scale)
            manifest[f.name] = {"hash": src_hash, "output": out_name, "scale": args.scale}

    save_manifest(manifest)
    count = len(list(args.output_dir.glob("*.png")))
    print(f"\nDone. {count} HD PNG(s) in {args.output_dir}")


if __name__ == "__main__":
    main()
