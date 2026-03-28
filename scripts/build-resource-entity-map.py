#!/usr/bin/env python3
"""Build resource-entity-map.json mapping DLL resource IDs to entity names.

Maps GOKRES portrait/mini-icon resource IDs to character names, and ship
resource IDs to ship class names, using DAT JSON + TEXTSTRA string table.

Usage:
    uv run scripts/build-resource-entity-map.py
"""

import json
import re
from pathlib import Path

BASE = Path(".")
STRINGS = json.loads((BASE / "web/data/base/textstra.json").read_text())
CHARS_MJ = json.loads((BASE / "data/base/json/MJCHARSD.json").read_text())
CHARS_MN = json.loads((BASE / "data/base/json/MNCHARSD.json").read_text())
SHIPS = json.loads((BASE / "data/base/json/CAPSHPSD.json").read_text())


def slugify(name: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")


def build_character_map() -> dict:
    """Map character DatIds to names and factions."""
    chars = {}
    for src in [CHARS_MJ, CHARS_MN]:
        for c in src["characters"]:
            name = STRINGS.get(str(c["text_stra_dll_id"]), f"Unknown-{c['id']}")
            faction = "alliance" if c.get("is_alliance") else "empire"
            chars[c["id"]] = {"name": name, "faction": faction, "slug": slugify(name)}
    return chars


def build_ship_map() -> dict:
    """Map ship class DatIds to names and factions."""
    ships = {}
    for s in SHIPS["ships"]:
        name = STRINGS.get(str(s["text_stra_dll_id"]), f"Unknown-{s['id']}")
        faction = "alliance" if s.get("is_alliance") else "empire"
        ships[s["id"]] = {"name": name, "faction": faction, "slug": slugify(name)}
    return ships


def map_portraits(chars: dict) -> dict:
    """Map GOKRES portrait resource IDs (2000-2999) to character names.

    Formula: portrait_id - 0x600 for major chars, portrait_id - 0x700 for minor chars.
    """
    portrait_dir = BASE / "data/base/ui/gokres-dll/BMP"
    mapping = {}

    for bmp in sorted(portrait_dir.glob("*.bmp")):
        rid = int(bmp.stem)
        if not (2000 <= rid <= 2999):
            continue

        # Try both offsets
        for offset in [0x600, 0x700]:
            char_id = rid - offset
            if char_id in chars:
                c = chars[char_id]
                mapping[str(rid)] = {
                    "type": "portrait",
                    "entity": c["name"],
                    "faction": c["faction"],
                    "slug": c["slug"],
                    "char_id": char_id,
                }
                break
        else:
            # Special case: 2128 is Luke Skywalker Jedi Knight variant
            if rid == 2128:
                mapping[str(rid)] = {
                    "type": "portrait",
                    "entity": "Luke Skywalker (Jedi)",
                    "faction": "alliance",
                    "slug": "luke-skywalker-jedi",
                    "char_id": 578,
                }

    return mapping


def map_mini_icons(chars: dict) -> dict:
    """Map GOKRES mini-icon resource IDs (19000-19999) to character names.

    Same offset pattern as portraits.
    """
    icon_dir = BASE / "data/base/ui/gokres-dll/BMP"
    mapping = {}

    for bmp in sorted(icon_dir.glob("*.bmp")):
        rid = int(bmp.stem)
        if not (19000 <= rid <= 19999):
            continue

        # Mini-icons use different offsets: 19000 range
        # Pattern: mini_icon_id = char_id + some_offset
        # Let's try: 19008 maps to char 832 (Ackbar) → offset = 19008 - 832 = 18176 = 0x4700
        for offset in [0x4700, 0x4600, 0x4500]:
            char_id = rid - offset
            if char_id in chars:
                c = chars[char_id]
                mapping[str(rid)] = {
                    "type": "mini-icon",
                    "entity": c["name"],
                    "faction": c["faction"],
                    "slug": c["slug"],
                    "char_id": char_id,
                }
                break

    return mapping


def map_ship_views(ships: dict) -> dict:
    """Map GOKRES ship status view resource IDs (1000-1999) to ship names."""
    ship_dir = BASE / "data/base/ui/gokres-dll/BMP"
    mapping = {}

    for bmp in sorted(ship_dir.glob("*.bmp")):
        rid = int(bmp.stem)
        if not (1000 <= rid <= 1999):
            continue

        # Ship views use offset from ship class ID
        for offset in [0x700, 0x600, 0x500]:
            ship_id = rid - offset
            if ship_id in ships:
                s = ships[ship_id]
                mapping[str(rid)] = {
                    "type": "ship-status",
                    "entity": s["name"],
                    "faction": s["faction"],
                    "slug": s["slug"],
                }
                break

    return mapping


def main():
    chars = build_character_map()
    ships = build_ship_map()

    # Build the full map
    resource_map = {}

    # Portraits (2000-2999)
    portraits = map_portraits(chars)
    resource_map.update(portraits)
    print(f"Portraits: {len(portraits)}/61 mapped")

    # Mini-icons (19000-19999)
    mini_icons = map_mini_icons(chars)
    resource_map.update(mini_icons)
    print(f"Mini-icons: {len(mini_icons)}/76 mapped")

    # Ship views (1000-1999)
    ship_views = map_ship_views(ships)
    resource_map.update(ship_views)
    print(f"Ship views: {len(ship_views)}/57 mapped")

    # Summary
    print(f"\nTotal mapped: {len(resource_map)} resource IDs")
    print(f"Unmapped portraits: {61 - len(portraits)}")
    print(f"Unmapped mini-icons: {76 - len(mini_icons)}")
    print(f"Unmapped ship views: {57 - len(ship_views)}")

    # Write output
    out = BASE / "data/resource-entity-map.json"
    out.write_text(json.dumps(resource_map, indent=2, ensure_ascii=False))
    print(f"\nWritten to {out}")


if __name__ == "__main__":
    main()
