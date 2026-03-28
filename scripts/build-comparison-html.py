#!/usr/bin/env python3
"""Build side-by-side comparison HTML for HD upscale dry-run.

Shows original BMP alongside HD upscale for visual QA.

Usage:
    uv run --with pillow scripts/build-comparison-html.py
    open data/hd/comparison.html
"""

import base64
import io
import json
from pathlib import Path

from PIL import Image

PROJECT = Path(__file__).parent.parent
HD_BASE = PROJECT / "data" / "hd"
OUTPUT = HD_BASE / "comparison.html"

DLL_DIRS = {
    "gokres": PROJECT / "data" / "base" / "ui" / "gokres-dll" / "BMP",
    "strategy": PROJECT / "data" / "base" / "ui" / "strategy-dll" / "BMP",
    "common": PROJECT / "data" / "base" / "ui" / "common-dll" / "BMP",
    "tactical": PROJECT / "data" / "base" / "ui" / "tactical-dll" / "BMP",
}

DLL_OUTPUT_DIRS = {
    "gokres": "gokres-dll",
    "strategy": "strategy-dll",
    "common": "common-dll",
    "tactical": "tactical-dll",
}


def img_to_data_uri(path: Path, max_dim: int = 0) -> str:
    img = Image.open(path)
    if img.mode in ("P", "RGBA", "LA"):
        img = img.convert("RGB")
    if max_dim and max(img.size) > max_dim:
        scale = max_dim / max(img.size)
        img = img.resize((int(img.size[0] * scale), int(img.size[1] * scale)), Image.LANCZOS)
    buf = io.BytesIO()
    fmt = "PNG" if path.suffix.lower() == ".png" else "JPEG"
    img.save(buf, format=fmt, quality=90)
    mime = "image/png" if fmt == "PNG" else "image/jpeg"
    return f"data:{mime};base64,{base64.b64encode(buf.getvalue()).decode()}"


def main():
    sels_path = PROJECT / "data" / "reference-selections.json"
    if not sels_path.exists():
        print("No reference-selections.json found")
        return

    sels = json.loads(sels_path.read_text())

    # Group by pack
    packs: dict[str, list] = {}
    for key, sel in sels.items():
        dll = sel["dll"]
        rid = sel["resource_id"]
        pack = sel["pack"]
        hd_path = HD_BASE / DLL_OUTPUT_DIRS[dll] / f"{rid}.png"
        orig_path = DLL_DIRS[dll] / f"{rid}.bmp"

        if not hd_path.exists():
            continue

        if pack not in packs:
            packs[pack] = []
        packs[pack].append({
            "dll": dll,
            "rid": rid,
            "entity": sel.get("entity", ""),
            "refs": sel.get("references", []),
            "orig": orig_path,
            "hd": hd_path,
        })

    total = sum(len(v) for v in packs.values())
    print(f"Building comparison for {total} images across {len(packs)} packs")

    html = ["""<!DOCTYPE html>
<html><head><meta charset="utf-8">
<title>Open Rebellion HD Upscale Dry-Run</title>
<style>
body { background: #1a1a2e; color: #e0e0e0; font-family: 'SF Mono', monospace; margin: 20px; }
h1 { color: #ffd700; border-bottom: 2px solid #ffd700; padding-bottom: 8px; }
h2 { color: #c0a030; margin-top: 40px; }
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(500px, 1fr)); gap: 20px; }
.card { background: #16213e; border: 1px solid #333; border-radius: 8px; padding: 16px; }
.card-title { color: #ffd700; font-size: 14px; margin-bottom: 8px; }
.card-meta { color: #888; font-size: 11px; margin-bottom: 12px; }
.compare { display: flex; gap: 16px; align-items: flex-start; }
.compare img { border: 1px solid #444; border-radius: 4px; }
.orig { image-rendering: pixelated; }
.refs { font-size: 10px; color: #666; margin-top: 8px; word-break: break-all; }
.stats { color: #aaa; font-size: 12px; margin: 20px 0; }
.blocked { color: #ff6b6b; font-style: italic; }
</style></head><body>
<h1>Open Rebellion HD Upscale Dry-Run</h1>
<p class="stats">""" + f"{total} images across {len(packs)} packs" + """</p>
"""]

    for pack_name in sorted(packs.keys()):
        items = packs[pack_name]
        html.append(f'<h2>{pack_name} ({len(items)} images)</h2>')
        html.append('<div class="grid">')

        for item in items:
            orig_uri = img_to_data_uri(item["orig"])
            hd_uri = img_to_data_uri(item["hd"], max_dim=400)

            orig_img = Image.open(item["orig"])
            hd_img = Image.open(item["hd"])
            scale = f"{hd_img.size[0]/orig_img.size[0]:.0f}x"

            label = item["entity"] or f"{item['dll']}/{item['rid']}"
            refs_str = "<br>".join(Path(r).name for r in item["refs"][:3])

            html.append(f'''<div class="card">
<div class="card-title">{label}</div>
<div class="card-meta">{item["dll"]}/{item["rid"]} — {orig_img.size[0]}x{orig_img.size[1]} → {hd_img.size[0]}x{hd_img.size[1]} ({scale})</div>
<div class="compare">
  <div><img class="orig" src="{orig_uri}" width="{min(orig_img.size[0]*3, 150)}" title="Original"><br><small>Original</small></div>
  <div><img src="{hd_uri}" width="350" title="HD Upscale"><br><small>HD ({scale})</small></div>
</div>
<div class="refs">Refs: {refs_str}</div>
</div>''')

        html.append('</div>')

    html.append('</body></html>')

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text("\n".join(html))
    print(f"Written to {OUTPUT}")


if __name__ == "__main__":
    main()
