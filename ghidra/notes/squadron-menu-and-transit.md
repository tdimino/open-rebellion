# Squadron Object Menu and In-Transit Behavior

Two questions: what does a planet-held fighter squadron's right-click menu
offer, and where does a squadron or regiment live while travelling between
systems?

---

## 1  Squadron Object Menu (`FUN_00503930`)

`FUN_00503930` is the squadron class's `GetOrderList` (vtable `+0x3c`).
It is a guarded singleton: global `DAT_006b2b38` prevents re-entry.

### Call chain

```
FUN_00503930          ; squadron GetOrderList
  → FUN_00557d80(0, &DAT_006b2b40)   ; build into output list
      → FUN_00557ce0                  ; build base combat-unit list
          → FUN_00558380              ; empty base
              → FUN_00558350
                  → FUN_004f2a10(0, &DAT_006bb368)  ; init empty list
          → add 0x201                 ; Move        (TEXTSTRA 12312)
          → add 0x202                 ; Confirmed Move (TEXTSTRA 12311)
          → add 0x204                 ; (no STRATEGY record → invisible)
          → add 0x200                 ; Scrap       (TEXTSTRA 12295)
      → FUN_00520600                  ; copy base to output
```

`FUN_0051d990` always appends **Encyclopedia** (`0x100`, TEXTSTRA 12292) and
**Status** (`0x103`, TEXTSTRA 12293) to whatever the class returns.

### Final menu

| Order | Kind   | TEXTSTRA | Visible | Enable condition |
|-------|--------|----------|---------|------------------|
| Move  | 0x201  | 12312    | yes     | `FUN_004f9860`: same side, not en route |
| Confirmed Move | 0x202 | 12311 | yes | same check as Move |
| (unnamed) | 0x204 | — | **no**: no STRATEGY.DLL `RT_RCDATA` record | — |
| Scrap | 0x200  | 12295    | yes     | same side, not en route |
| Encyclopedia | 0x100 | 12292 | yes | always enabled for an object |
| Status | 0x103 | 12293 | yes | enabled when selection is an object (not a system) |

### Comparison with other class menus

The base list (`FUN_00557ce0`) is shared by **squadrons**, **regiments**
(`FUN_00504b30`) and **capital ships** (`FUN_00502bd0`). All three get
Move, Confirmed Move, (invisible 0x204) and Scrap. Capital ships add
Rename (`0x203`) and Create Fleet (`0x270`) on top.

Fleet-carried fighters have no separate class; they are entries in the
fleet's fighter array. A fleet's menu (`FUN_004ff8e0`) covers the whole
fleet: Move, Confirmed Move, Bombardment sub-menu, Assault, Rename, Scrap.
A carried squadron has its own right-click in the Fleet window (observed,
below).

### Port difference

`object_menu.rs` returned an empty order list for `MenuObject::Fighter` and
`MenuObject::SystemFighter`; it now returns `&[MOVE, CONFIRMED_MOVE, SCRAP]`,
the regiment's list. Correction (2026-10-10): a defense facility does not
share this list. Every facility class uses `FUN_005399e0` ->
`FUN_00539c20`, Scrap alone (`facility-order-lists.md`).

Observed in the original (2026-10-10, capture VM, save
`empire-day2-start`): Coruscant, Fleet 1's window, Fighters tab, a right-click
on one TIE Fighter squadron opens Move, Confirmed Move, Encyclopedia, Status,
Scrap, in that order, all drawn alike. Choosing Confirmed Move turns the
pointer into the targeting crosshair (`25-confirmed-move.png`), so it is
enabled for a squadron; the port keeps it disabled. An earlier draft of this note said the
Fleet window has no per-squadron right-click; that rested on the port's model
(squadrons as fleet fighter entries) and is wrong. The original's carried
squadrons are `0x1c` objects in a ship's container (`FUN_00504c40`,
`FUN_00556390`), so they take `FUN_00503930`'s list like a planet's.
Evidence: `.artifacts/native-checks/2026-10-10-original-fleet-squadron-menu/`
(`23-fighters.png`, `24-rclick-squadron.png`).

---

## 2  In-Transit Object Location

### Original (`REBEXE.EXE`)

When a squadron or regiment moves between systems, `FUN_00556390` (execute)
does the following at once:

1. **Reparents** the object to its destination container (the target
   system's list or the target fleet's cargo).
2. **Sets the en-route bit** (`+0x50` bit 4).

The object lives in the destination container for the duration of travel.
It is not removed from any global walk. On arrival (`FUN_004fc080` or
the tick expiry path), the en-route bit is cleared.

### Iterator modes and derived bits (`object-state-flags.md`)

| Mode | Bit   | Name     | Meaning |
|------|-------|----------|---------|
| 1    | +0x50 bit 0 | usable | existing AND completed AND **NOT en route** |
| 2    | +0x50 bit 4 | en route | travelling between systems |
| 3    | +0x50 bit 6 | existing | created AND NOT destroyed |
| 4    | —     | always   | every slot |

### (a) Maintenance during transit

**Original**: `FUN_0052fff0` (side maintenance recompute) walks every
object of families `0x10..0x3f` via `FUN_004f6010` that **exists** (`+0x50`
bit 6). An en-route object's bit 6 stays set—bit 4 (en route) does not
clear bit 6 (existing). **En-route objects count toward maintenance.**
(`top-bar-resource-counters.md`, lines 101–156.)

**Port**: `maintenance_used()` in `resources.rs` walks `world.fleets`,
`world.systems`, `world.troops`, `world.special_forces`,
`world.defense_facilities`, `world.manufacturing_facilities` and
`world.production_facilities`. When a squadron or regiment departs
(`troop_transport.rs`, `move_squadron` / `move_regiment`), the object is
removed from its world container and stored in `TroopTransportState`'s
`squadron_transit` / `transit` `Vec`. Because `maintenance_used` does not
walk those transit vectors, **en-route objects cost zero maintenance**.

**Bug**: the user observed Alliance maintenance 288 → 284 → 288 across a
trip (a 4-point drop while one unit was in transit). The original would
hold at 288 throughout.

### (b) Presence during transit (`FUN_00509710`)

**Original**: `FUN_00509710` (side-present test) uses mode 1 iterators
(`FUN_004ffef0` for fleets, `FUN_00503a50` for squadrons). Mode 1 =
**usable** = existing AND completed AND NOT en route. An en-route object
is reparented to the destination but is not usable, so it counts toward
presence at **neither origin nor destination**.
(`system-squadrons-combat.md`.)

**Port**: `side_present()` in `combat.rs` checks
`system.squadron_count(is_alliance)` and the system's fleet list. A
transiting unit is in neither container. **Result matches the original:
no presence at either end.**

### Summary table

| Property | Original | Port | Match? |
|----------|----------|------|--------|
| Maintenance | charged (exists) | not charged (out of world) | **NO** |
| Presence at origin | none (not usable) | none (removed) | yes |
| Presence at destination | none (not usable) | none (not yet placed) | yes |
| Object location | destination container, en-route bit set | `TroopTransportState` vec | differs in mechanism |

---

## Confidence

| Claim | Confidence | Source |
|-------|------------|--------|
| Squadron menu = Move + CM + (invisible 0x204) + Scrap | **high** | disassembly of FUN_00503930, FUN_00557d80, FUN_00557ce0, FUN_00558380, FUN_00558350 |
| Shared base list with regiments and capital ships | **high** | FUN_00504b30.c, FUN_00502bd0.c both call FUN_00557ce0/FUN_00558380 |
| 0x204 invisible (no STRATEGY record) | **high** | object-popup-menu.md, confirmed by the sorted-insert skip |
| En-route object reparented to destination | **high** | regiment-unload.md FUN_00556390 |
| Maintenance uses "existing" filter (bit 6) | **high** | top-bar-resource-counters.md FUN_0052fff0, FUN_004f6010 |
| En-route bit does not clear existing bit | **high** | object-state-flags.md derived-bit table |
| Port drops maintenance during transit | **high** | resources.rs `maintenance_used` + troop_transport.rs `move_squadron`/`move_regiment` |
| Presence excludes en-route (mode 1) | **high** | system-squadrons-combat.md FUN_00509710 |

---

## Source files

- `FUN_00503930`: disassembly at 0x503930 in REBEXE.EXE (36 bytes)
- `FUN_00557d80`: disassembly at 0x557d80 (26 bytes)
- `FUN_00557ce0.c`: ghidra/notes/
- `FUN_00558380.c`: ghidra/notes/
- `FUN_00504b30.c`: ghidra/notes/ (regiment, same base list)
- `FUN_00502bd0.c`: ghidra/notes/ (capital ship, base + Rename + Create Fleet)
- `FUN_0052fff0`: top-bar-resource-counters.md lines 101–156
- `FUN_00509710`: system-squadrons-combat.md
- `object-state-flags.md`: iterator modes and bit definitions
- `regiment-unload.md`: reparent + en-route bit on cross-system move
- `crates/rebellion-render/src/object_menu.rs`: port's menu lists (line 406)
- `crates/rebellion-core/src/resources.rs`: port's `maintenance_used` (line 43)
- `crates/rebellion-core/src/combat.rs`: port's `side_present` (line 89)
- `crates/rebellion-core/src/troop_transport.rs`: port's transit storage
