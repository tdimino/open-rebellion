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

## Uprising incident

System `+0x88` bit 18 (`0x40000`) is the uprising incident. `FUN_0050ab30`
sets it, notifies both side views (`FUN_00510620` → view slot `+0x2dc`,
`FUN_00512580`, `SystemUprisingIncidentNotif`), and calls the master's slot
`+0x250`, `FUN_00511840`, which runs `FUN_0050d030` when the bit turns on.
System vtable `0x0065e638`: `+0x24c` no-op, `+0x250` `FUN_00511840`,
`+0x254` `FUN_00511860`, `+0x258` `FUN_00511930` (disaster), `+0x2d8`
`FUN_00512540`, `+0x2dc` `FUN_00512580`, `+0x2e0` `FUN_005125d0`
(informant), `+0x2e4` `FUN_00512620` (disaster).

Trigger: `FUN_00556b50` (called by `FUN_00566760`) picks one system at
random among those with `+0x50` bit 6, and `FUN_0050cdc0` pulses bit 18 on
it when its `+0x5c` or `+0x64` is non-zero.

`FUN_0050d030` resolves the incident for the controlling side
(`+0x24 >> 6 & 3`, 1 Alliance or 2 Empire; neutral returns). It pulses
`+0x88` bit 17 (`FUN_0050aac0`), then calls `FUN_00559ce0` with:

| Argument | Source |
|----------|--------|
| side | `+0x24 >> 6 & 3` |
| support | `FUN_00507270`: `+0x58` for side 1, `100 - +0x58` for side 2 |
| strong flag | `+0x88 >> 11 & 1` |
| troops | `FUN_00509020(system, side, 1)`: the side's regiments |
| p5 | `FUN_005091f0`: Empire regiments of class `0x10000006` |
| p6, p7 | `+0x54 → +0x74`, `+0x54 → +0x78` |

`FUN_00559ce0` computes

```
score = 2 * GNPRTB[7707] + rand(GNPRTB[7708]) + rand(GNPRTB[7708])
      + FUN_0055a050(support)            ; uprising threshold
      - k * troops                       ; k = GNPRTB[7680] if strong && side == 2, else 1
      + p6 + p7 - p5
```

and looks `score` up in UPRIS1TB and UPRIS2TB, yielding two outcome codes.
Shipped tables: UPRIS1TB `(1→0, 6→1, 10→2)`, UPRIS2TB `(1→0, 9→3, 11→4,
12→5)`. `FUN_0050d150` applies each code to the controlling side at the
system:

| Code | Effect |
|------|--------|
| 0 | none |
| 1 | a random facility (families `0x20..0x2f`) is destroyed, reason 8 |
| 2 | a random regiment is destroyed, reason 8 |
| 3 | a random character (`0x30..0x3b`) with `+0xac` bit 0 clear gets slot `+0x2e4` |
| 4 | a random character with `+0xac` bit 0 set gets slot `+0x214` |
| 5 | every character with `+0xac` bit 0 set gets slot `+0x214` |

Finally `FUN_0050c9f0` adds `FUN_00559be0(+0x54 → +0x7c, side, strong)` to
the side's support (divided by GNPRTB 7681, `DAT_006bb400`, when the
system's strong flag is set and the change favours side 1 or hurts side 2) and
stores it
through `FUN_00509c30`.

## Still open

- The meaning of the character slots `+0x2e4` and `+0x214` and of
  `+0xac` bit 0 on characters: the eleven character-like vtables that hold
  the role notifiers disagree on these slots.
- The object at system `+0x54` (fields `+0x74`, `+0x78`, `+0x7c`) and the
  system fields `+0x5c` / `+0x64` tested by the trigger.
- The cadence of `FUN_00566760`.
