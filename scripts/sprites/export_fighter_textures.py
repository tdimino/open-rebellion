#!/usr/bin/env python3
"""Export the native type-303 fighter textures (TACTICAL.DLL ids 4000-4134) as palette PNGs (Track C).

These are the original fighter sprites the retained-mode renderer draws for squadrons: 77
indexed textures (41 at 32x32, 36 at 16x16), all `battle_active` (rule 1), so the colour comes
from the battle palette selected per system (5531-5557). The export keeps the palette indices
(mode P, palette block = the chosen battle palette, no transparency) so it is a fidelity
reference, unlike assets/references/ref-squadron-sprites/, whose 14 files are MetasharpNet
`Names303` 128x128 upscales with the palette discarded.

  uv run scripts/sprites/export_fighter_textures.py                 # palette 5531 -> ref-fighter-textures/
  uv run scripts/sprites/export_fighter_textures.py --palette 5540 --out /tmp/refs

Writes {id}-{size}-{family}.png, palette.json (palette id, palette sha256, per-file digests; the RGB
table itself is not written, it is original game data)
and INDEX.md. Family names come from docs/reference/asset-library/tactical-lookup.json `fighters`
(id ranges base..base+9); ids outside every range are `unknown` (4044, 4049).
PNGs under assets/references/ are gitignored (extracted from game data); INDEX.md and palette.json
are committed.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from collections import Counter
from pathlib import Path

SPRITES_DIR = Path(__file__).resolve().parent
PROJECT_ROOT = SPRITES_DIR.parents[1]
sys.path.insert(0, str(SPRITES_DIR))
import tactical3d  # noqa: E402

LOOKUP = PROJECT_ROOT / "docs" / "reference" / "asset-library" / "tactical-lookup.json"
DEFAULT_OUT = PROJECT_ROOT / "assets" / "references" / "ref-fighter-textures"
ID_RANGE = (4000, 4134)
EXPECTED_COUNT = 77
EXPECTED_SIZES = {(32, 32): 41, (16, 16): 36}


def family_table() -> list[tuple[int, int, str]]:
    """(first_id, last_id, family slug) from tactical-lookup.json fighters rows [ordinal, base, close, ?, name]."""
    rows = json.loads(LOOKUP.read_text())["fighters"]
    table = []
    for row in rows:
        base, name = int(row[1]), str(row[-1])
        slug = name.lower().replace(" ", "-")
        table.append((base, base + 9, slug))
    return table


def family_for(resource_id: int, table) -> str:
    for lo, hi, slug in table:
        if lo <= resource_id <= hi:
            return slug
    return "unknown"


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--palette", type=int, default=tactical3d.DEFAULT_BATTLE_PALETTE, help="battle palette id (5531-5557)")
    p.add_argument("--out", type=Path, default=DEFAULT_OUT)
    p.add_argument("--runtime-root", type=Path, default=tactical3d.RUNTIME_ROOT)
    a = p.parse_args()

    manifest = tactical3d.Manifest.load(a.runtime_root)
    palette = manifest.palette(a.palette)
    palette_bytes = bytes(c for rgb in palette for c in rgb)
    table = family_table()
    ids = sorted(i for i, rec in manifest.textures_by_id.items()
                 if ID_RANGE[0] <= i <= ID_RANGE[1] and rec.get("kind") == "indexed_rle")
    a.out.mkdir(parents=True, exist_ok=True)

    files, sizes, rules = [], Counter(), Counter()
    for rid in ids:
        tex = manifest.texture_by_id(rid)
        rules[tex.rule_name] += 1
        if tex.palette_rule != 1:
            sys.exit(f"{rid}: palette rule {tex.rule_name}; fighters are expected to be battle_active")
        sizes[(tex.width, tex.height)] += 1
        fam = family_for(rid, table)
        name = f"{rid}-{tex.width}x{tex.height}-{fam}.png"
        img = tactical3d.texture_to_image(tex, palette)
        img.save(a.out / name, optimize=True)
        rec = manifest.textures_by_id[rid]
        files.append({"id": rid, "file": name, "size": [tex.width, tex.height], "family": fam,
                      "palette_rule": tex.rule_name, "trailing_bytes": len(tex.trailing),
                      "object_sha256": rec.get("object_sha256"), "source_sha256": rec.get("source_sha256"),
                      "indices_sha256": hashlib.sha256(tex.indices).hexdigest(), "png_sha256": sha256_file(a.out / name)})

    problems = []
    if len(files) != EXPECTED_COUNT:
        problems.append(f"expected {EXPECTED_COUNT} textures, found {len(files)}")
    if dict(sizes) != EXPECTED_SIZES:
        problems.append(f"expected sizes {EXPECTED_SIZES}, found {dict(sizes)}")

    sidecar = {"schema": "open-rebellion/fighter-texture-refs-v1", "palette_id": a.palette,
               "palette_sha256": hashlib.sha256(palette_bytes).hexdigest(),   # the RGB table itself stays in game data
               "id_range": list(ID_RANGE), "count": len(files), "sizes": {f"{w}x{h}": n for (w, h), n in sorted(sizes.items())},
               "families": sorted({f["family"] for f in files}), "files": files}
    (a.out / "palette.json").write_text(json.dumps(sidecar, indent=2))

    by_family = Counter(f["family"] for f in files)
    lines = [
        "# Native fighter textures (type-303, TACTICAL.DLL 4000-4134)",
        "",
        "Palette-preserving exports of the original squadron sprites: mode-P PNGs whose indices are the",
        f"staged `ORTINDEX` bytes and whose palette block is battle palette **{a.palette}** (`battle_active`",
        "rule; every system selects its own palette in 5531-5557, re-export with `--palette`).",
        "These are fidelity references. `ref-squadron-sprites/` holds 128x128 MetasharpNet `Names303`",
        "upscales with the palette discarded and must not be used to judge colour or pixel structure.",
        "",
        f"Generated by `scripts/sprites/export_fighter_textures.py` from `{a.runtime_root.relative_to(PROJECT_ROOT) if a.runtime_root.is_relative_to(PROJECT_ROOT) else a.runtime_root}`.",
        "PNGs are gitignored (extracted game data); `palette.json` carries per-file digests.",
        "",
        f"| Count | 32x32 | 16x16 | Palette sha256 |", "|---|---|---|---|",
        f"| {len(files)} | {sizes.get((32, 32), 0)} | {sizes.get((16, 16), 0)} | `{sidecar['palette_sha256'][:16]}...` |",
        "",
        "| Family | Ids | Files |", "|---|---|---|",
    ]
    for lo, hi, slug in table + [(0, 0, "unknown")]:
        members = [f for f in files if f["family"] == slug]
        if not members:
            continue
        id_span = f"{members[0]['id']}-{members[-1]['id']}" if len(members) > 1 else str(members[0]["id"])
        lines.append(f"| {slug} | {id_span} | {by_family[slug]} |")
    lines += ["", "| Id | Size | Family | File |", "|---|---|---|---|"]
    lines += [f"| {f['id']} | {f['size'][0]}x{f['size'][1]} | {f['family']} | `{f['file']}` |" for f in files]
    (a.out / "INDEX.md").write_text("\n".join(lines) + "\n")

    print(f"{len(files)} textures -> {a.out} (palette {a.palette}); sizes {dict(sizes)}; families {dict(by_family)}")
    for pr in problems:
        print(f"FAIL {pr}", file=sys.stderr)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
