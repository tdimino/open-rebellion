#!/usr/bin/env python3
"""Make an edited copy of one of our original-game saves.

    save_edit.py hyperdrive SOURCE.sav OUTPUT.sav
    save_edit.py deathstar  SOURCE.sav OUTPUT.sav
    save_edit.py damage     SOURCE.sav OUTPUT.sav
    save_edit.py subsystems SOURCE.sav OUTPUT.sav
    save_edit.py engines    SOURCE.sav OUTPUT.sav
    save_edit.py fragile    SOURCE.sav OUTPUT.sav
    save_edit.py stranded   SOURCE.sav OUTPUT.sav
    save_edit.py doomed     SOURCE.sav OUTPUT.sav
    save_edit.py squadron   SOURCE.sav OUTPUT.sav
    save_edit.py trench     SOURCE.sav OUTPUT.sav

hyperdrive and deathstar are pinned to `saves/empire-day2-start.sav`; damage,
subsystems, engines, fragile, stranded and doomed to
`saves/empire-day30-yaga-alert-control.sav`, where they edit the ships at the
battle; squadron to `saves/alliance-deathstar-at-coruscant.sav`; trench to
`saves/empire-day660-deathstar-pre-alert.sav`. They change bytes in place, refuse any other source, never overwrite, and prove that only the intended bytes changed. The layout
and the acceptance gates are in `agent_docs/original-save-editing.md`; an
edit is a hypothesis until the original loads, re-saves and reloads it.
"""

import hashlib
import struct
import sys
from pathlib import Path

DAY2 = ("d9da6a3bff71a70130ccd7aecb9ab4a98becf7bfef5e3fc7918c0970ea7cbc79", 410_763)
DAY30 = ("8a7a992327c8af5aa6f2580a0220084741da1e37664d15e218c476c36a975636", 421_096)
CORUSCANT = ("5825d980881950daa22fb91e9e0ade5cc76f9a81f7daa41419d508f0585ea4c5", 1_387_851)
ISD_NAME = b"Imperial Star Destroyer 1"
ISD_ID = 0x77F
# The two copies of the ISD's record: the full graph and the shorter one.
ISD_FULL, ISD_SHORT = 0x1778E, 0x48EE5


def require(condition, detail="layout check failed"):
    """An assert that `python3 -O` cannot remove: every check here guards
    a write into an original save."""
    if not condition:
        raise SystemExit(f"refusing: {detail}")


def u32(data, offset):
    return struct.unpack_from("<I", data, offset)[0]


def check_record(data, start, graph, ident=ISD_ID, cls=0x85, name=ISD_NAME):
    """Assert the named record at `start` is the ship in the given graph."""
    found = struct.unpack_from("<5I", data, start + 4)
    length = struct.unpack_from("<H", data, start + 0x18)[0]
    found += (data[start + 0x1A : start + 0x1A + length],)
    require(found == (0x14, ident, graph, cls, 1, name), (hex(start), found))
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
        require(u32(data, payload) == 0x00030047)  # +0x50 flags, the same in both
        damage = payload + word * 4
        require(u32(data, damage - 4) == 0)  # +0x60 hull damage
        require(u32(data, damage) == 0)
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


# In the day-30 save each ship has a third graph copy (0x80000091), short
# like the 0xa1 one. (record start, graph, payload word of +0x60).
DAY30_SHIPS = (
    (0x77F, 0x85, ISD_NAME, ((0x1AE28, 0x80000081, 19), (0x32C96, 0x80000091, 9),
                             (0x4BBED, 0x800000A1, 9))),
    (0x7AA, 0x83, b"Carrack Light Cruiser 2", ((0x1B29F, 0x80000081, 19),
        (0x32F41, 0x80000091, 9), (0x4BF24, 0x800000A1, 9))),
)


# The Alliance's one ship at Yaga Minor: Bulk Cruiser (CAPSHPSD 0x41, hull
# 1200), in the Alliance graphs 0x80000041 (full) and 0x80000051 (short).
BULK_CRUISER = (0x796, 0x41, b"Bulk Cruiser 3", ((0x1B385, 0x80000041, 19),
                                                  (0x32FEB, 0x80000051, 9)))
FLAGS = {0x77F: 0x000301C7, 0x7AA: 0x000301C7, 0x796: 0x00030056}  # +0x50


def stage(data, staged, ships=DAY30_SHIPS):
    """Write (hull damage, packed counters) to every copy of each ship."""
    changed = set()
    for (ident, cls, name, copies), (hull, packed) in zip(ships, staged, strict=True):
        for start, graph, word in copies:
            payload = check_record(data, start, graph, ident, cls, name)
            require(u32(data, payload) == FLAGS[ident])
            at = payload + word * 4
            require(u32(data, at) == 0 and u32(data, at + 4) == 0)
            struct.pack_into("<2I", data, at, hull, packed)
            changed |= {i for i in range(at, at + 8) if data[i]}
    return changed


def damage(data):
    """Stage a critical and a damaged ship for the tactical status panel.
    The ISD (hull 2750, CAPSHPSD 0x85) takes 2200 hull damage, leaving 20%,
    and one shield, two weapons, one tractor and one engine hit: `+0x64`
    nibbles 0-3, 4-7, 8-11 and 12-15 (FUN_00501510, FUN_005015a0,
    FUN_00501640, FUN_005016e0, called in that order by FUN_0040cfc0). The
    Carrack (hull 1000) takes 400, leaving 60%."""
    return stage(data, ((2200, 0x0000_1121), (400, 0)))


def subsystems(data):
    """Two hits on each of the ISD's shield generator, weapons, tractor and
    engines at full hull, so each icon shows its own damage rather than the
    hull scaling (FUN_005b1770) that dims the first three in `damage`."""
    return stage(data, ((0, 0x0000_2222), (0, 0)))


def engines(data):
    """Four engine hits on the ISD (the limit, FUN_005b1970) and nothing
    else: two hits left the engine icon unchanged in `subsystems`."""
    return stage(data, ((0, 0x0000_4000), (0, 0)))


def fragile(data):
    """Leave the Alliance's Bulk Cruiser 10 of 1200 hull, so the first hits
    destroy it: a target lost, a destruction and a Destroyed result. Both
    of its hyperdrives (CAPSHPSD 80 and 120) are out too: with them, the
    first such copy withdrew instead of dying (2026-10-12)."""
    return stage(data, ((1190, 0x0011_0000),), (BULK_CRUISER,))


def stranded(data):
    """Both of the Bulk Cruiser's hyperdrives out at full hull: it cannot
    withdraw, so the battle runs long enough to capture it running."""
    return stage(data, ((0, 0x0011_0000),), (BULK_CRUISER,))


def doomed(data):
    """Leave the ISD 10 of 2750 hull, so the first Alliance hits destroy the
    selected ship while the Carrack keeps the battle going. The Bulk
    Cruiser's hyperdrives are out as in `stranded`: with them, the first
    such copy's Alliance withdrew before a hit landed (2026-10-12)."""
    return stage(data, ((2740, 0), (0, 0), (0, 0x0011_0000)),
                 DAY30_SHIPS + (BULK_CRUISER,))


# Fleet 2's first X-wing squadron (Mon Calamari Cruiser 2) in the Coruscant
# save: its full record (Alliance graph 0x41, 17 payload words) and the short
# one (0x51, 10 words). The third copy (0x62) is the Empire's last sighting
# and already differs from these on some squadrons, so it is left alone.
XWING_ID, XWING_CLASS, XWING_SHIP = 0x17B8, 3, 0x14001396
XWING_COPIES = ((0x3A0D0, 0x80000041, 16), (0x9E1F3, 0x80000051, 9))


def squadron(data):
    """Lose 7 of the squadron's 12 craft: Status should read Squadron Size
    5:12. Like a ship's hull, `+0x60` holds damage, not the size left
    (FUN_005032c0; 0 in every full squadron), and it is the last payload
    word of both copies."""
    changed = set()
    for start, graph, word in XWING_COPIES:
        found = struct.unpack_from("<6I", data, start)
        require(found[1:] == (0x1C, XWING_ID, graph, XWING_CLASS, 0), (hex(start), found))
        payload = start + 0x18
        require(u32(data, payload) == 0x000300C7)  # +0x50 flags, the same in both
        if word == 16:
            require(u32(data, payload + 9 * 4) == XWING_SHIP)  # attached to the cruiser
        damage = payload + word * 4
        require(u32(data, damage) == 0)
        struct.pack_into("<I", data, damage, 7)
        changed.add(damage)
    return changed


# Fleet 6's six TIE Fighter squadrons (class 5), carried by Imperial Star
# Destroyer 1 (0x140009f1) beside Death Star 1 on its way to the day-676
# battle at Coruscant: full records (Empire graph 0x81, 17 payload words)
# and short ones (0xa1, 10 words). The third copy (0x92) is the Alliance's
# last sighting and is left alone, as in `squadron`.
DAY660 = ("5973cf91e8ee5cd3684c1dc83966c2d049d456081b7623a09f64d7f43b778de1", 1_040_089)
TIE_CLASS, TIE_SHIP = 5, 0x140009F1
TIE_COPIES = tuple(
    (ident, full, short)
    for ident, full, short in zip(range(0x9F2, 0x9F8),
                                  (0x4C4A0, 0x4C4FC, 0x4C558, 0x4C5B4, 0x4C610, 0x4C66C),
                                  (0xBB01E, 0xBB05E, 0xBB09E, 0xBB0DE, 0xBB11E, 0xBB15E))
)


def trench(data):
    """Empty the six Imperial squadrons coming to Coruscant (damage 12 of
    12, FUN_005032c0), so the Alliance's squadrons outnumber the Empire's
    in the battle. The Alliance AI orders the trench run (order 6,
    FUN_005c8780) only while FUN_005ad930 holds: the station attackable
    (FUN_005ba450: completed, hull left, no family 0x25 shield at the
    system), no trench run yet, and strictly more Alliance squadrons than
    Imperial ones. An emptied squadron as destroyed is untested."""
    changed = set()
    for ident, *copies in TIE_COPIES:
        for start, graph, word in zip(copies, (0x80000081, 0x800000A1), (16, 9)):
            found = struct.unpack_from("<6I", data, start)
            require(found[1:] == (0x1C, ident, graph, TIE_CLASS, 0), (hex(start), found))
            payload = start + 0x18
            require(u32(data, payload) == 0x00030056)  # +0x50 flags, the same in both
            if word == 16:
                require(u32(data, payload + 9 * 4) == TIE_SHIP)  # aboard the ISD
            damage = payload + word * 4
            require(u32(data, damage) == 0)
            struct.pack_into("<I", data, damage, 12)
            changed.add(damage)
    return changed


RECIPES = {"hyperdrive": (hyperdrive, DAY2), "deathstar": (deathstar, DAY2),
           "damage": (damage, DAY30), "subsystems": (subsystems, DAY30),
           "engines": (engines, DAY30), "fragile": (fragile, DAY30),
           "stranded": (stranded, DAY30), "doomed": (doomed, DAY30),
           "squadron": (squadron, CORUSCANT), "trench": (trench, DAY660)}


def main():
    if len(sys.argv) != 4 or sys.argv[1] not in RECIPES:
        sys.exit(__doc__)
    (recipe, (sha256, size)), source, output = RECIPES[sys.argv[1]], Path(sys.argv[2]), Path(sys.argv[3])
    if output.exists() or output.resolve() == source.resolve():
        sys.exit(f"refusing to write {output}: it exists or is the source")
    original = source.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    if digest != sha256 or len(original) != size:
        sys.exit(f"{source} is not the save {sys.argv[1]} is pinned to ({digest})")
    edited = bytearray(original)
    expected = recipe(edited)
    actual = {i for i, (a, b) in enumerate(zip(original, edited)) if a != b}
    require(len(edited) == len(original) and actual == expected, (actual, expected))
    with output.open("xb") as handle:
        handle.write(edited)
    require(output.read_bytes() == edited)
    print(f"{sys.argv[1]}: changed {', '.join(hex(i) for i in sorted(actual))}")
    print(f"output sha256 {hashlib.sha256(edited).hexdigest()}")


if __name__ == "__main__":
    main()
