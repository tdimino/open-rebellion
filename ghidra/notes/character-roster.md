---
title: "Character Roster: Recruitment, Major and Minor Classes, Officer Rank"
description: "How the recruit pick filters the one character container (minors 0x38..0x3b only, by side, unrecruited), who may recruit (majors only), the major and minor class kinds and the mission-lead tier, officer rank +0x96 (CommandKind) and its DAT eligibility flags, and what a roster mod must change"
category: "ghidra"
created: 2026-10-14
updated: 2026-10-14
tags: [characters, recruitment, officer-rank, major-minor, mission-lead, mod-surface]
---

# Character roster

Answers task-graph node `or-q-mod-roster-5laz` (epic `or-ep-mod-surface-vdch`), from the Feature Wiki page `executable/mod-roster`; the recruit iterator `FUN_0056f450`, the pick `FUN_0055fc80`, the recruiter check `FUN_0056b550`, the rank check `FUN_004ed750` with its class getters, and the mission-lead pick `FUN_00521300` were reread here. Sides are the game's: 1 Alliance, 2 Empire. `hyp:` marks anything not traced through code. Literal character ids at game start are in `scenario-literals.md` and `maintenance-and-vader-start.md`.

## Summary

1. **Recruit pick** (`FUN_0055fc80`): walk the one character container with a family range `0x38..0x3b` (minors only), state mode 4 (accepts every state), the recruiting side's bits `+0x24 >> 6 & 3`, then `+0x50` bit 1 clear; draw one at random and place it with `FUN_0055fe70`. Nothing else filters the pick.
2. **Major and minor share one container and one DAT parser**; they differ in factory (six major families `0x30`..`0x35`, one minor family `0x38`), in three vtable slots, and in one mission rule: only majors may be sent on Recruitment.
3. **Officer rank `+0x96` is the CommandKind**: 0 none, 1 Commander, 2 Admiral, 3 General. Only the command orders `0x260`..`0x263` write it; moves and lost commands clear it; the DAT has no rank field.
4. **Rank eligibility is DAT**: class `+0xac` `can_be_commander`, `+0xa8` `can_be_admiral`, `+0xb0` `can_be_general`, tested by `FUN_004ed750`.
5. The recruit pick and the rank logic name no character id.

## The recruit pick

Both sides run the same code; the side is the argument. `FUN_0055fc80(out, side, outId, ctx)` is called by the Recruitment result (`0x0056b9a0`, `mission-lifecycle.md`) and by new-game stage `0x15` through `FUN_00562610`.

| # | Test | Function |
|---|---|---|
| 1 | The container exists (game root `DAT_006b2bb0 + 0xb0`) | `FUN_00506e20` |
| 2 | Family byte in `[0x38, 0x3c)` | `FUN_0056f450` (range), `FUN_005131d0` |
| 3 | State mode 4, which accepts every state (`object-state-flags.md`) | `FUN_005131d0` |
| 4 | Side bits `+0x24 >> 6 & 3` equal the side | `FUN_005131d0` (iterator flag `+0x14` = 1, `FUN_00513090`) |
| 5 | `+0x50` bit 1 clear (not yet recruited) | `FUN_0055ef30` (count), `FUN_0055fc80` (pick), `FUN_0055fe70` (guard) |
| 6 | Index `FUN_0053e290(count - 1)` in container order | `FUN_0055fc80` |
| 7 | The id resolves to a character (family `0x30..0x3b`) | `FUN_0055fe70` → `FUN_00505190` |
| 8 | Slot `+0xa8(system, ctx)` places it: the system exists, is not the galaxy, shares the character's `+0x24 & 0x30` bits, and the container move succeeds | `FUN_004f8630` → `FUN_004f6fd0`, `FUN_004fc600` |
| 9 | `+0x50` bits 1 and 2 are set | `FUN_004f7480(1)`, `FUN_004f74f0(1)` |

When the count was 1, `FUN_0055fc80` sets the side's `+0xb8` (`FUN_0052f590(side, 1)`; side objects at root `+0xc4` and `+0xc8`). A side with `+0xb8` set cannot create a Recruitment mission: `FUN_0056b370` refuses with status `(0x40, 0x35)`.

The pick does not test the target system, death (`+0x50` bit 3), capture (`+0xac` bit 0) or a mission. hyp: a dead or captured minor whose bit 1 is still clear can be drawn; settle by reading the kill path `0x004ef690` and the release `FUN_004ef510` for bits 1 and 2.

**Who may recruit.** `FUN_0056b370` (creation check of the Recruitment class, factory `FUN_0056b300`) runs `FUN_0056b550` or `FUN_0056b680` over three member lists (`+0x18`, `+0x1c`, `+0x20` or `+0x24`, `+0x28`, `+0x2c`, by `+8`); every member must have family `0x30..0x37`, otherwise status `(0x40, 6)` and outcome -1. Only a major can be sent on Recruitment.

## Major and minor classes

**One container.** `FUN_00506e20` returns the single container at the game root `+0xb0`; recruit pick, count and mission iterators filter it by family range.

**Factories.** `FUN_0053f100` creates an object from a DatId through the family registry (`FUN_0051ebf0`: 58 `FUN_00540540(family, factory)` calls, capacity 60; lookup `FUN_00540500(id >> 24)`); the save loader uses the same registry (`FUN_0053a1e0`).

| Family | Factory | Constructor chain | Vtable |
|---|---|---|---|
| `0x30` | `FUN_0056eee0` | `FUN_0056ece0` → `FUN_00594520` → `FUN_004eceb0` | `0x006657f0` |
| `0x31` | `FUN_0056ffe0` | per-character subclass over `FUN_00594520` | not read |
| `0x32` (Luke) | `FUN_0056f8e0` | `FUN_0056f4e0` → `FUN_00594520` | `0x00665fd8` |
| `0x33`, `0x34`, `0x35` | `FUN_00572810`, `FUN_00573ba0`, `FUN_005755c0` | hyp: `FUN_00572530`, `FUN_005739c0`, `FUN_005753e0` → `FUN_00594520` | not read |
| `0x38` (every minor) | `FUN_0056f230` | `FUN_0056f020` → `FUN_004eceb0` | `0x00665be0` |
| `0x36`, `0x37`, `0x39`..`0x3b` | none | | |

`FUN_00594520` (vtable `0x0066b608`) is the major base; each major family has its own subclass, so there is no generic major. MJCHARSD records carry `family_id` `0x30`..`0x35` (one each), MNCHARSD records all `0x38` (54). The class-record loaders (`FUN_005915f0` major, `FUN_005912b0` minor, ranges registered by `FUN_00569300`) parse identically through `FUN_00595980`; class `+0x98`..`+0xb8` hold, in DAT order, `jedi_probability`, `is_known_jedi`, `jedi_level_base`, `jedi_level_variance`, `can_be_admiral`, `can_be_commander`, `can_be_general`, `is_unable_to_betray`, `is_jedi_trainer`.

**Slots that differ** (215 slots; base `0x0065ca70`, major base `0x0066b608`, minor `0x00665be0`):

| Slot | Base | Major | Minor | Use |
|---|---|---|---|---|
| `+0x1fc` | no-op | returns 3 (`0x00526f00`) | returns 2 (`0x0043c1f0`) | mission-lead tier |
| `+0x1c0` | `FUN_004ed460` | `FUN_00594660`: status `(0x30, 1)`, returns 1 | `FUN_004ed460`: status 1 with a rank, 2 when a prisoner | an order check; which order asks it is not traced |
| `+0x1d0` | returns 1 | returns 0 | returns 1 | not traced |
| `+0x2e0` | 0 | 0 | `0x0056f160` | not traced |

Capture `+0x20c` (`FUN_004ef480`), release `+0x210` (`FUN_004ef510`), kill `+0x2ec` (`0x004ef690`), evade `+0x208` (`FUN_004ef450`), order check `+0x2d0` (`FUN_004ed750`) and `+0x2f4` / `+0x2f8` are the same functions for both kinds. Capture, death and escape therefore differ only where code names a literal id (the six majors `FUN_004025f0`, the four leaders `FUN_004024d0`, Luke and Vader `FUN_00402cf0`) or reads `is_unable_to_betray` (all six majors, 3 of 54 minors).

**Mission lead.** `FUN_00521300` (from `FUN_00522870`, `FUN_00593870`) walks a mission's members (`0x30..0x3f`) and keeps, among those not captured, not flagged by `+0x78 & 0xc` and with slot `+0x1d4` = 0, the one with the greater `+0x1fc` tier, then the greater leadership `+0x1f4`. A major leads whenever one is on the team.

## Officer rank (`+0x96`, CommandKind)

- **Setter** `FUN_004ee350` (string `Invalid CommandKind value`, notification `CharacterCommandKindNotif`); its range check `FUN_0053fc90(value, 0, value)` accepts any short, so the gate is `FUN_004ed750`.
- **Names.** The change notifier `FUN_004f17f0` (slot `+0x330`) adds `0x6800`, `0x6c00` or `0x7000` to the character's name string id for ranks 1, 2, 3 (the Commander, Admiral and General name blocks, `decoy-roll.md`) and sets the commanded key `+0xa8` (`FUN_004ee740`).
- **Orders.** `0x260` None, `0x261` Commander, `0x262` Admiral, `0x263` General (`ai-order-emission.md`) reach `FUN_00561bb0` (vtable `0x00663290`): a character target (`0x30..0x3b`), the kind from the order, the commanded ship, fleet or system (`FUN_0055ed30`), then slot `+0x2f4` (`FUN_004ef340`: checks `+0x2d4` and `+0x2d0`) and slot `+0x2f8` (`FUN_004ef400`): the current kind toggles the rank off, any other kind sets it (`FUN_004ee350`) and the commanded key. Players use the popup Command submenu; the AI uses module 12.
- **Eligibility** `FUN_004ed750` (slot `+0x2d0`): kind 0 needs a current rank; 1 `can_be_commander` (`FUN_004ed1e0`, class `+0xac`); 2 `can_be_admiral` (`FUN_004ed1c0`, `+0xa8`); 3 `can_be_general` (`FUN_004ed200`, `+0xb0`); other kinds refuse with status `(0x30, 4)`. A character on a mission, injured (`+0x94`) or a prisoner (`+0xac` bit 0) is refused first. In the base data Luke, Han and Vader have all three flags, Mon Mothma, Leia and the Emperor none.
- **Clears.** `FUN_004eeed0` (callers `FUN_004f1720`, `FUN_004f1a80`) clears the rank when the character is injured, a prisoner or gone, or `+0x68 >> 24` is outside `0x50..0x7f`; `FUN_00561bb0` when the commanded unit differs from the target; `FUN_00560d50` when the character or its ship moves.
- **Readers.** `FUN_00509330` and `FUN_004fd790` → `FUN_005006f0` find the character holding a rank at a system or in a fleet (the detecting officer of `decoy-roll.md`); `FUN_00402230` turns ranks 1, 2, 3 into the AI record bits `0x4000000`, `0x1000000`, `0x2000000`.
- **Start value** 0 (`FUN_004eceb0`); the save codec `FUN_004ef7e0` / `FUN_004ef940` carries `+0x96`.

## Limits

| Limit | Kind | Function |
|---|---|---|
| Who may be recruited: families `0x38..0x3b`, the side, not yet recruited | literal range, runtime flag, DAT side flags | `FUN_0056f450`, `FUN_005131d0`, `FUN_0055ef30` |
| Who may recruit: families `0x30..0x37` | literal | `FUN_0056b550`, `FUN_0056b680` |
| Pool-empty flag, sides 1 and 2 | runtime `+0xb8` | `FUN_0055fc80`, `FUN_0052f590`, `FUN_0056b370` |
| Character families that can exist: `0x30`..`0x35`, `0x38` | table (code), 60-family registry, 58 used | `FUN_0051ebf0`, `FUN_00540540` |
| Rank eligibility | DAT `can_be_*` | `FUN_004ed750` |
| Ranks 1..3 and their name offsets | literal | `FUN_004ed750`, `FUN_004f17f0` |
| Mission-lead tier (major 3, minor 2) | literal slot returns | `0x00526f00`, `0x0043c1f0`, `FUN_00521300` |
| Random recruits placed at a new game | DAT SDPRTB `0x1400`..`0x1402` per side (1, 2, 4); hyp: indexed by galaxy size | `FUN_0055d7d0`, `FUN_0055f7b0` |
| Recruitment success and leadership gain | DAT RCRTMSTB, GNPRTB 6159 | `mission-lifecycle.md` |

## Mod surface

1. **More minors:** none, as family-`0x38` MNCHARSD records; families `0x39`..`0x3b` need a factory registration (two free slots) and a minor class.
2. **More majors:** each of the six families has its own class (Luke's carries story notifiers); a record in `0x36` / `0x37` needs a new factory and class.
3. **Minors as recruiters:** the `0x30..0x37` pair in `FUN_0056b550` and `FUN_0056b680`.
4. **Majors as recruits:** the `0x38..0x3c` pair in `FUN_0056f450` (shared by `FUN_0055ef30`).
5. **Rank:** who may hold a rank is DAT; a fourth rank needs `FUN_004ed750`, `FUN_004f17f0`, new order ids beside `0x260`..`0x263`, `FUN_00402230` and the detector tiers.

## Functions

| Function | Role |
|---|---|
| [FUN_0055fc80](FUN_0055fc80.c), [FUN_0055ef30](FUN_0055ef30.c), [FUN_0055fe70](FUN_0055fe70.c) | Recruit pick, pool count, placement |
| [FUN_0056f450](FUN_0056f450.c), [FUN_005131d0](FUN_005131d0.c), [FUN_00506e20](FUN_00506e20.c) | Minor iterator and filter; the character container |
| [FUN_0056b370](FUN_0056b370.c), [FUN_0056b550](FUN_0056b550.c), [FUN_0056b680](FUN_0056b680.c) | Recruitment creation check and its majors-only member test |
| [FUN_0052f590](FUN_0052f590.c) | Side pool-empty flag |
| [FUN_0051ebf0](FUN_0051ebf0.c), [FUN_00540540](FUN_00540540.c), [FUN_00540500](FUN_00540500.c), [FUN_0053f100](FUN_0053f100.c) | Family factory registry |
| [FUN_0056ece0](FUN_0056ece0.c), [FUN_0056f4e0](FUN_0056f4e0.c), [FUN_00594520](FUN_00594520.c), [FUN_0056f020](FUN_0056f020.c), [FUN_004eceb0](FUN_004eceb0.c) | Major and minor constructors and the character base |
| [FUN_00521300](FUN_00521300.c) | Mission lead by tier, then leadership |
| [FUN_004ee350](FUN_004ee350.c), [FUN_004f17f0](FUN_004f17f0.c), [FUN_004ee740](FUN_004ee740.c) | Rank setter, change notifier, commanded key |
| [FUN_00561bb0](FUN_00561bb0.c), [FUN_004ef340](FUN_004ef340.c), [FUN_004ef400](FUN_004ef400.c) | Command orders `0x260`..`0x263` |
| [FUN_004ed750](FUN_004ed750.c), [FUN_004ed1c0](FUN_004ed1c0.c), [FUN_004ed1e0](FUN_004ed1e0.c), [FUN_004ed200](FUN_004ed200.c) | Rank eligibility from the class flags |
| [FUN_004eeed0](FUN_004eeed0.c) | Rank clear |
| [FUN_00509330](FUN_00509330.c), [FUN_005006f0](FUN_005006f0.c), [FUN_00402230](FUN_00402230.c) | Rank readers |

## Open questions

- Which order asks character slot `+0x1c0` (callers of `FUN_00540da0`, `FUN_0052ed50`, `FUN_0054e170`).
- What the three Recruitment member lists are (`FUN_0056b300`'s constructor, `FUN_005868c0`).
- What a new game does with a record in a family without a factory, or a missing named character (`FUN_00562520`, `FUN_005136d0`).
- Whether any reader indexes characters by the low 24 bits of the DatId, which would bound an era's id space.
- The special-force tier in `+0x1fc` (class vtable `0x0065e160`).
- Whether a dead or captured minor can be re-drawn by the recruit pick.
