#!/usr/bin/env python3
"""Make an edited copy of one of our original-game saves.

    save_edit.py hyperdrive SOURCE.sav OUTPUT.sav
    save_edit.py deathstar  SOURCE.sav OUTPUT.sav

Both recipes are pinned to `saves/empire-day2-start.sav` and edit Fleet 1's
Imperial Star Destroyer. They change bytes in place, refuse any other source,
never overwrite, and prove that only the intended bytes changed. The layout
and the acceptance gates are in `agent_docs/original-save-editing.md`; an
edit is a hypothesis until the original loads, re-saves and reloads it.
"""

import hashlib
import struct
import sys
from pathlib import Path

SOURCE_SHA256 = "d9da6a3bff71a70130ccd7aecb9ab4a98becf7bfef5e3fc7918c0970ea7cbc79"
SOURCE_SIZE = 410_763
ISD_NAME = b"Imperial Star Destroyer 1"
ISD_ID = 0x77F
# The two copies of the ISD's record: the full graph and the shorter one.
ISD_FULL, ISD_SHORT = 0x1778E, 0x48EE5


def u32(data, offset):
    return struct.unpack_from("<I", data, offset)[0]


def check_record(data, start, graph):
    """Assert the named record at `start` is the ISD in the given graph."""
    family, ident, key, cls, named = struct.unpack_from("<5I", data, start + 4)
    length = struct.unpack_from("<H", data, start + 0x18)[0]
    name = data[start + 0x1A : start + 0x1A + length]
    assert (family, ident, key, cls, named, name) == (
        0x14, ISD_ID, graph, 0x85, 1, ISD_NAME,
    ), (hex(start), family, ident, hex(key), cls, named, name)
    return start + 0x1A + length  # payload


def hyperdrive(data):
    """One primary and one backup hyperdrive hit (`+0x64` bits 16-23,
    FUN_00501780 / FUN_00501820) in both copies of the ISD's record. The
    capital ship serializer (FUN_00501f00) streams `+0x60` then `+0x64` as
    the last two payload words: words 19-20 of the full copy, 9-10 of the
    short one (ghidra/notes/capital-ship-save-layout.md)."""
    changed = set()
    for start, graph, word in ((ISD_FULL, 0x80000081, 20), (ISD_SHORT, 0x800000A1, 10)):
        payload = check_record(data, start, graph)
        assert u32(data, payload) == 0x00030047  # +0x50 flags, the same in both
        damage = payload + word * 4
        assert u32(data, damage - 4) == 0  # +0x60 hull damage
        assert u32(data, damage) == 0
        struct.pack_into("<I", data, damage, 0x0011_0000)
        changed.add(damage + 2)
    return changed


def deathstar(data):
    """Family 0x14 -> 0x18 and class 0x85 -> 0x88 (DatId 0x18000088) in
    both copies of the ISD's record."""
    changed = set()
    for start, graph in ((ISD_FULL, 0x80000081), (ISD_SHORT, 0x800000A1)):
        check_record(data, start, graph)
        struct.pack_into("<I", data, start + 0x04, 0x18)
        struct.pack_into("<I", data, start + 0x10, 0x88)
        changed |= {start + 0x04, start + 0x10}
    return changed


RECIPES = {"hyperdrive": hyperdrive, "deathstar": deathstar}


def main():
    if len(sys.argv) != 4 or sys.argv[1] not in RECIPES:
        sys.exit(__doc__)
    recipe, source, output = RECIPES[sys.argv[1]], Path(sys.argv[2]), Path(sys.argv[3])
    if output.exists() or output.resolve() == source.resolve():
        sys.exit(f"refusing to write {output}: it exists or is the source")
    original = source.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    if digest != SOURCE_SHA256 or len(original) != SOURCE_SIZE:
        sys.exit(f"{source} is not empire-day2-start.sav ({digest})")
    edited = bytearray(original)
    expected = recipe(edited)
    actual = {i for i, (a, b) in enumerate(zip(original, edited)) if a != b}
    assert len(edited) == len(original) and actual == expected, (actual, expected)
    with output.open("xb") as handle:
        handle.write(edited)
    assert output.read_bytes() == edited
    print(f"{sys.argv[1]}: changed {', '.join(hex(i) for i in sorted(actual))}")
    print(f"output sha256 {hashlib.sha256(edited).hexdigest()}")


if __name__ == "__main__":
    main()
