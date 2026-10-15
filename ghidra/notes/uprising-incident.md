---
title: "Uprising Incident and Probability Tables"
description: "How table ids map to DAT files, and how the uprising incident turns UPRIS1TB/UPRIS2TB outcome codes into losses"
category: "ghidra"
created: 2026-09-26
updated: 2026-09-26
tags: [uprising, upris1tb, upris2tb, decoy, mission-tables, table-ids]
---

# Uprising Incident and Probability Tables

Recovered 2026-09-26 with Ghidra 12.1.3 headless (read-only project).

## Table ids

`FUN_0058b420` registers every IntTable, SeedTable, and parameter table with
`FUN_0058b810(registry, 0x642, resource_id, table_id)`. `FUN_005952a0` builds
the path from two `RT_RCDATA` strings loaded by `FUN_005f49e0`: resource
`0x642` is `GDATA\` and `resource_id` is the file name. The ids are fixed:

| Id | File | Id | File |
|----|------|----|------|
| 1 | SYFCCRTB | 0x19 | ABDCMSTB |
| 2 | SYFCRMTB | 0x1a | INCTMSTB |
| 3 | CMUNALTB | 0x1b | DSSBMSTB |
| 4 | CMUNEMTB | 0x1c | SUBDMSTB |
| 5 | CSCRHTTB | 0x1d | ASSNMSTB |
| 10 | TDECOYTB | 0x28 | UPRIS1TB |
| 11 | FDECOYTB | 0x29 | UPRIS2TB |
| 12 | FOILTB | 0x2a | INFORMTB |
| 13 | RLEVADTB | 0x2b | RESRCTB |
| 0x14 | DIPLMSTB | 0x2c | ESCAPETB |
| 0x15 | RESCMSTB | 0x3c | GNPRTB |
| 0x16 | SBTGMSTB | 0x3d | SDPRTB |
| 0x17 | ESPIMSTB | | |
| 0x18 | RCRTMSTB | | |

`FUN_0053e240(id, x, &out)` returns the entry value for `x` (`+0x24` of the
matching entry via `FUN_0055bed0` / `FUN_0058b7e0`). `FUN_0053e340` looks the
value up and rolls it with `FUN_0053e2f0` (draw `0..99 < value`).
`FUN_0053e310` wraps `FUN_0053e340`.

Direct consumers:

| Function | Table | Argument |
|----------|-------|----------|
| `FUN_0055c680` | DIPLMSTB | `(p3 - p2) + p1` |
| `FUN_0055c7e0` | RESCMSTB | `p1` |
| `FUN_0055c890` | SBTGMSTB | `(p1 + p2) / 2` |
| `FUN_0055c6c0` | ESPIMSTB | `p1` |
| `FUN_0055c700` | RCRTMSTB | `p1 - p2` |
| `FUN_0055c810` | ABDCMSTB | `p1 - p2` |
| `FUN_0055c740` | INCTMSTB | `(p1 - p2) - p3` |
| `FUN_0055c8d0` | DSSBMSTB | `(p1 + p2) / 2` |
| `FUN_0055c780` | SUBDMSTB | `(p3 - p2) + p1` |
| `FUN_0055c850` | ASSNMSTB | `p1 - p2` |
| `FUN_0055e410` | TDECOYTB / FDECOYTB | `(p2 - p4) - FUN_0053e190(p3, DAT_006bb710)`, table `(p5 != 0) + 10`; called by `FUN_00588b90` |
| `FUN_0055e470` | FOILTB | `(p1 - p4 - p5) - FUN_0053e190(p2, DAT_006bb70c) - DAT_006bb714`; called by `FUN_00588a90` |
| `FUN_0055bfa0` | RLEVADTB | `p2 - p3` |
| `FUN_0055e7e0` | ESCAPETB | `(p2 + p3) - p4 - p5` |
| `FUN_00559ce0` | UPRIS1TB, UPRIS2TB | see below |
| `FUN_00559db0` | INFORMTB | random draw plus a GNPRTB offset |
| `FUN_00559ee0` | RESRCTB | random draw |

## System vtable and incident bits

The system vtable base is `0x0065e640` (constructor `FUN_00507130`;
`0x0065e63c` is the second base at `+0x30`).

| `+0x88` bit | Incident | Setter | Master slot |
|-------------|----------|--------|-------------|
| 2 (mask 4) | uprising in progress | `FUN_0050a4a0` | `+0x214` `FUN_00510f20` (timers) |
| 9 | uprising visible | `FUN_0050bb00` | |
| 11 | Death Star in the sector | `FUN_0050a890` | from `FUN_005178b0` (`support-shifts.md`) |
| 16 | uprising incident | `FUN_0050aa50` | `+0x248` `FUN_00511840` -> `FUN_0050d030` |
| 17 | informant incident | `FUN_0050aac0` | `+0x24c` `FUN_00511860` -> `FUN_0050d510` |
| 18 | disaster incident | `FUN_0050ab30` | `+0x250` `FUN_00511930` |
| 19 | resource incident | | |
| 20 | blockade and battle pending | | |

## Uprising lifecycle (bit 2)

- Start and keep, `FUN_0050b800` (per-system update `FUN_00508250`): on when
  the system is populated (`+0x88` bit 0), held by side 1 or 2, and either
  already in revolt or short of troops (surplus `+0x7c` < 0, `FUN_0050b500` =
  regiments - requirement `+0x80`). It clears when the system is no longer
  held or populated. Control never changes: `FUN_0050a130` only writes the
  uprising side into `+0x78` bits 4-5.
- The stored requirement (`FUN_0050b5a0`) is `FUN_00559fe0(side, support,
  death_star, in_uprising, 1)`: halved by GNPRTB 7680 only when bit 11 (the
  Death Star in the sector) is set and the side is the Empire,
  doubled by GNPRTB 7682 only in an uprising, 0 unless side 1 or 2.
  `FUN_0050bb00` sets bit 9 when the system is held, populated, in revolt and
  the surplus is >= 0.
- End, `FUN_0050c910`, runs only from a Subdue Uprising success
  (`FUN_00569c20`): it ends the revolt when every regiment at the system
  (`FUN_00504c40`, no side filter) covers the requirement computed without the
  uprising doubling (`FUN_00559fb0`).
- `FUN_0050a4a0` enables two timers through slot `+0x214`, `FUN_00510f20`:
  event 900 and event 0x38d, the uprising incident. A timer delay
  (`FUN_00586130`) is `min + rand[0..=spread]` and reschedules after each fire
  (`FUN_005862a0`); the incident uses GNPRTB 7701 (30) and 7702 (70), so it
  fires every 30 to 100 ticks. Event 0x38d (`FUN_005660c0` -> `FUN_0050cb80`)
  pulses bit 16 only while bit 2 is on.

## Uprising incident (`FUN_0050d030`)

The incident acts for the holder (`+0x24 >> 6 & 3`, 1 Alliance or 2 Empire;
anything else returns). It first pulses bit 17 on and off (`FUN_0050aac0`,
the informant incident's setter) and then calls `FUN_00559ce0`:

```
d      = rand[0..=G7708] + G7707                 ; 9 and 1, drawn twice
score  = d1 + d2
       + ceil((G7761 - support) / -G7762)       ; 60 and -10, only when support < 60 (FUN_0055a050)
       - k * regiments(side)                    ; k = G7680 (2) if bit 11 && side == 2, else 1
       + p6 + p7 - p5
```

- `support` is `FUN_00507270`: `+0x58` for side 1, `100 - +0x58` for side 2.
- `p5` (`FUN_005091f0`) counts Empire regiments of class `0x10000006`
  (Stormtroopers, TEXTSTRA 9344).
- `p6` and `p7` live at `+0x54 -> +0x74` and `+0x78`, written by
  `FUN_005484d0` (from the mission state change `FUN_00546ea0`): the sum over
  active Incite Uprising missions (family 0x56, MISSNSD 64) of the average
  agent leadership (slot `+0x1f4`, the short at `+0x88`) divided by GNPRTB
  6144 (10), and minus the same sum over Subdue Uprising missions (0x57,
  MISSNSD 128).

Both codes come from step lookups: `FUN_00595090` returns the value of the
largest threshold `<= x`, clamped to the first row. Shipped tables: UPRIS1TB
`(1→0, 6→1, 10→2)`, UPRIS2TB `(1→0, 9→3, 11→4, 12→5)`. `FUN_0050d150`
applies each code to the holder at the system:

| Code | Effect |
|------|--------|
| 0 | none |
| 1 | a random facility of the holder (families `0x20..0x2f`) is destroyed, reason 8 |
| 2 | a random regiment of the holder is destroyed, reason 8 |
| 3 | a random free character (`+0xac` bit 0 clear) is injured: slot `+0x2e4` `FUN_004ef5f0`, chance `max(G2565, 100 - combat)`, injury `rand(chance) + rand(G2567) + G2566` (1, 29, 1) through `FUN_0053e990` into `+0x94`, `CharacterInjuryNotif` |
| 4 | one prisoner the holder keeps (`+0xac` bit 0 set) is freed: slot `+0x214` `FUN_004ef570` -> `FUN_004ee3e0(1)` sets `+0x98` to 1; `FUN_004f18e0` clears the captor, sets autorouting and raises event 0x30a |
| 5 | every such prisoner is freed |

Codes 3 to 5 pick characters through `FUN_004f2640(system, 1, side)`: the
system's direct children of types `0x30..0x3c` that are usable (`+0x50`
bit 0). The walk skips characters in a fleet or on a mission, which are
children of that fleet or mission, and a destroyed or en-route character is
not usable (`object-state-flags.md`).

Finally `FUN_0050c9f0` changes support by `+0x54 -> +0x7c`: GNPRTB 6145
(-2) while an Incite Uprising mission is active, else 0. `FUN_00559be0`
divides the change by GNPRTB 7681 (2) when bit 11 is set (the Death Star is
in the sector) and the change favours side 1 or hurts side 2, and `FUN_0053e0d0` clamps the result to
0..100.

## Subdue Uprising success (`FUN_00569c20`)

Support rises by `FUN_0055cb10(mission side, system side)`: on its own side's
system `G6187 + rand[0..=G6188]` (1..20); at a neutral system (side 3, "Neutral" in `FUN_004f8c60`)
`G6189 + rand[0..=G6190]` (1..10); otherwise 0. Then `FUN_0050c910` runs the
end check. The success check itself (`FUN_00569b90`) is SUBDMSTB through
`FUN_0055c780(leadership, support, p5)`.

## Disaster incident (bit 18)

A galaxy timer (`FUN_00556fa0`, event 0x38f) fires every `G7717 +
rand[0..=G7718]` (1..400) ticks. `FUN_00566760` -> `FUN_00556b50` picks a
random system with `+0x50` bit 6 (GameObjExisting), and `FUN_0050cdc0`
pulses bit 18 only when its energy (`+0x5c`) or raw materials (`+0x64`) is
non-zero. `FUN_00511930` then:

1. Erodes resources with `FUN_00559e10(&raw, &energy)`: for each unit, lose it
   with chance `(remaining total) * G7715` (5) percent; if nothing was lost,
   lose one raw material (else one energy); finally `raw = min(raw, energy)`.
2. Destroys each facility not en route (`+0x50` bit 4 clear), of either side,
   with chance G7716 (10) percent, reason 0xb: manufacturing and production
   (`0x28..0x2f`) first, then defense (`0x22..0x27`).

## Other incidents

- Resource (bit 19): event 0x390 every `G7719 + rand[0..=G7720]` (1..500)
  ticks, `FUN_00556be0` -> `FUN_0050cc70` (RESRCTB, `FUN_00559ee0`).
- Informant (bit 17): event 0x38e on a per-system timer (`+0x40`, GNPRTB
  7703/7704), `FUN_0050cbe0` rolls support, then `FUN_00511860` ->
  `FUN_0050d510` (INFORMTB).

## Still open

- Slot `+0x1bc` / `+0x1c4` of the object the decoy roll checks
  (see `decoy-roll.md`).
- The character injury at `+0x94` has no port field; the port reports it
  but does not store it.
