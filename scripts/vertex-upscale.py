#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["google-cloud-aiplatform", "pillow"]
# ///
"""
Vertex AI Imagen 4.0 Upscale — non-generative super-resolution.

Unlike Gemini edit (which reimagines the image), Imagen upscale cleanly
enlarges without hallucinating new content. Best for portraits, small
sprites, and anything where fidelity matters more than detail invention.

Prerequisites:
    gcloud auth login
    gcloud config set project YOUR_PROJECT_ID
    gcloud services enable aiplatform.googleapis.com

Usage:
    uv run scripts/vertex-upscale.py --input image.png
    uv run scripts/vertex-upscale.py --input image.png --factor 4
    uv run scripts/vertex-upscale.py --input-dir scripts/upscale-test/originals/ --output scripts/upscale-test/vertex/
"""

import argparse
import os
import sys
from pathlib import Path


def upscale_image(input_path: Path, output_path: Path, factor: int = 4) -> Path | None:
    """Upscale a single image via Vertex AI Imagen 4.0."""
    try:
        from google.cloud import aiplatform
        from vertexai.preview.vision_models import Image, ImageGenerationModel
    except ImportError:
        print("ERROR: google-cloud-aiplatform not installed.", file=sys.stderr)
        print("  Run: uv pip install google-cloud-aiplatform", file=sys.stderr)
        sys.exit(1)

    # Initialize Vertex AI
    project = os.environ.get("GOOGLE_CLOUD_PROJECT", os.environ.get("GCLOUD_PROJECT"))
    if not project:
        import subprocess
        result = subprocess.run(
            ["gcloud", "config", "get-value", "project"],
            capture_output=True, text=True,
        )
        project = result.stdout.strip()

    if not project:
        print("ERROR: No GCP project set. Run: gcloud config set project YOUR_PROJECT", file=sys.stderr)
        sys.exit(1)

    location = os.environ.get("GOOGLE_CLOUD_LOCATION", "us-central1")
    aiplatform.init(project=project, location=location)

    print(f"  Loading model imagen-4.0-upscale-preview...")
    model = ImageGenerationModel.from_pretrained("imagen-4.0-upscale-preview")

    print(f"  Upscaling {input_path.name} ({factor}x)...")
    source_image = Image.load_from_file(str(input_path))

    response = model.upscale_image(
        image=source_image,
        upscale_factor=factor,
    )

    if response.images:
        output_path.parent.mkdir(parents=True, exist_ok=True)
        response.images[0].save(str(output_path))
        from PIL import Image as PILImage
        img = PILImage.open(output_path)
        print(f"  Saved: {output_path.name} ({img.size[0]}x{img.size[1]})")
        return output_path
    else:
        print(f"  ERROR: No image in response", file=sys.stderr)
        return None


def main() -> None:
    parser = argparse.ArgumentParser(description="Vertex AI Imagen 4.0 non-generative upscale")
    parser.add_argument("--input", type=Path, help="Single image to upscale")
    parser.add_argument("--input-dir", type=Path, help="Directory of images to batch upscale")
    parser.add_argument("--output", type=Path, help="Output file or directory")
    parser.add_argument("--factor", type=int, default=4, choices=[2, 4], help="Upscale factor (default: 4)")
    args = parser.parse_args()

    if args.input:
        out = args.output or args.input.parent / f"{args.input.stem}-vertex-{args.factor}x{args.input.suffix}"
        upscale_image(args.input, out, args.factor)
    elif args.input_dir:
        out_dir = args.output or args.input_dir.parent / "vertex"
        out_dir.mkdir(parents=True, exist_ok=True)
        images = sorted(args.input_dir.glob("*.png")) + sorted(args.input_dir.glob("*.jpg"))
        print(f"Batch upscaling {len(images)} images ({args.factor}x) via Vertex AI Imagen 4.0")
        for img in images:
            out = out_dir / f"{img.stem}-vertex-{args.factor}x.png"
            if out.exists():
                print(f"  SKIP: {out.name} exists")
                continue
            upscale_image(img, out, args.factor)
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()
