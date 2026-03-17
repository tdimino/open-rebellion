---
title: "External Sources"
description: "All external references used for Open Rebellion mechanics documentation"
category: "reference"
created: 2026-03-16
updated: 2026-03-16
---

# External Sources

All external references used for Open Rebellion game mechanics documentation, organized by authority tier.

## Tier 1: Official Sources

| Source | Type | Notes |
|--------|------|-------|
| Star Wars Rebellion Game Manual | Print/PDF | Shipped with 1998 retail release. Rules, unit stats, mission descriptions. |
| Prima's Official Strategy Guide | Print | Detailed mechanics tables, character stats, mission probability breakdowns. |
| GOG.com Store Page | Digital | DRM-free distribution; includes manual PDF. |
| Steam Store Page | Digital | Current digital distribution channel. |

## Tier 2: Community Reverse Engineering

| Source | Type | Notes |
|--------|------|-------|
| Mechanics Inside Rebellion | Web | Deep community analysis of hidden mechanics, probability tables, combat formulas. |
| Metasharp Star Wars Rebellion Editor | Tool (C#) | DAT file editor with C# parsers. Source at `~/Desktop/Programming/StarWarsRebellionEditor.NET/`. |
| rebellion2 | Code (C#) | C# game model reference with Mission.cs, combat logic. Source at `~/Desktop/Programming/rebellion2/`. |
| GameFAQs Guides | Web | Multiple community-written walkthroughs and mechanics FAQs. |

## Tier 3: Reference

| Source | Type | Notes |
|--------|------|-------|
| Wookieepedia | Wiki | Star Wars canon reference for ship classes, character backgrounds, unit lore. |
| Wikipedia: Star Wars Rebellion | Wiki | General game overview, reception, platform history. |
| StrategyWiki: Star Wars Rebellion | Wiki | Community strategy guides, building orders, faction analysis. |

## Tier 4: In-Repo (Primary Authority)

| Source | Path | Notes |
|--------|------|-------|
| Ghidra RE: Entity System | `ghidra/notes/entity-system.md` | Character fields, Force/Jedi, fleet notifications, object hierarchy. 669 lines. |
| Ghidra RE: Economy Systems | `ghidra/notes/economy-systems.md` | Resources, system control, uprising thresholds, blockade. |
| Ghidra RE: Combat Formulas | `ghidra/notes/combat-formulas.md` | 7-phase space combat, weapon arcs, shield absorption, hull damage. |
| Ghidra RE: Annotated Functions | `ghidra/notes/annotated-functions.md` | 5,127 decompiled functions with signatures and purposes. |
| Ghidra RE: C++ Class Hierarchy | `ghidra/notes/cpp-class-hierarchy.md` | CRebObject → CNotifyObject inheritance, vtable layout. |
| Ghidra RE: Modder Taxonomy | `ghidra/notes/modders-taxonomy.md` | DAT file field purposes for mod developers. |
| Ghidra RE: Rust Implementation Guide | `ghidra/notes/rust-implementation-guide.md` | Mapping C++ patterns to Rust idioms, field offsets, system design. |
| Rust Source | `crates/rebellion-core/src/` | Canonical implementation: 15 simulation systems, 105+ tests. |
| DAT Binary Files | `data/base/` | Original game data files, parsed by dat-dumper (51/51 with round-trip validation). |
