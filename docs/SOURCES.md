---
title: "External Sources"
description: "All external references used for Open Rebellion mechanics documentation"
category: "reference"
created: 2026-03-16
updated: 2026-03-16
tags: [sources, bibliography, references]
---

# External Sources

All external references used for Open Rebellion game mechanics documentation, organized by authority tier.

## Tier 1: Official Sources

| Source | URL | Notes |
|--------|-----|-------|
| Star Wars Rebellion Game Manual | [archive.org](https://archive.org/details/star-wars-rebellion-manual) / [swrebellion.net PDF](https://files.swrebellion.net/Star_Wars_Rebellion-Manual.pdf) | Shipped with 1998 retail release. Rules, unit stats, mission descriptions. |
| Prima's Official Strategy Guide | [archive.org](https://archive.org/details/star-wars-rebellion-guide) / [swrebellion.net PDF](https://files.swrebellion.net/Star_Wars_Rebellion_Guide_PRIMA.pdf) | 276 pages. Detailed mechanics tables, character stats, mission probability breakdowns. |
| GOG.com Store Page | [gog.com](https://www.gog.com/en/game/star_wars_rebellion) | DRM-free distribution; includes manual PDF. |
| Steam Store Page | [store.steampowered.com](https://store.steampowered.com/app/441550/) | Current digital distribution channel. |

## Tier 2: Community Reverse Engineering

| Source | URL | Notes |
|--------|-----|-------|
| Mechanics Inside Rebellion (Part I) | [swrebellion.net](https://swrebellion.net/forums/topic/282-mechanics-inside-rebellion/) | Deep community analysis of hidden mechanics, probability tables, combat formulas. |
| Mechanics Inside Rebellion (Part II) | [swrebellion.net](https://swrebellion.net/forums/topic/9639-mechanics-inside-rebellion-part-ii/) | Continuation thread with extended mechanics discussion. |
| Metasharp Star Wars Rebellion Editor | [github.com](https://github.com/MetasharpNet/StarWarsRebellionEditor.NET) | 686-commit .NET DAT file editor. Local: `~/Desktop/Programming/StarWarsRebellionEditor.NET/`. |
| rebellion2 | Local only | C# game model reference with Mission.cs, combat logic. `~/Desktop/Programming/rebellion2/`. |
| GameFAQs Strategy Guide (Rogue) | [gamefaqs.gamespot.com](https://gamefaqs.gamespot.com/pc/198776-star-wars-rebellion/faqs/19754) | Comprehensive walkthrough and strategy. |
| GameFAQs Ship Guides (Sashanan) | [Rebel Ships](https://gamefaqs.gamespot.com/pc/198776-star-wars-rebellion/faqs/11040) / [Imperial Ships](https://gamefaqs.gamespot.com/pc/198776-star-wars-rebellion/faqs/10833) | Detailed per-faction ship analysis. |
| GOG Reference Spreadsheet | [gog.com](https://www.gog.com/forum/star_wars_rebellion/rebellion_reference_spreadsheet) | Community data tables (unit stats, costs). |

## Tier 3: Reference

| Source | URL | Notes |
|--------|-----|-------|
| Wookieepedia | [starwars.fandom.com](https://starwars.fandom.com/wiki/Star_Wars:_Rebellion_(video_game)) | Star Wars canon reference for ship classes, character backgrounds, unit lore. |
| Wikipedia | [en.wikipedia.org](https://en.wikipedia.org/wiki/Star_Wars:_Rebellion) | General game overview, reception, platform history. |
| StrategyWiki | [strategywiki.org](https://strategywiki.org/wiki/Star_Wars:_Rebellion) | Community strategy guides, building orders, faction analysis. |

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
