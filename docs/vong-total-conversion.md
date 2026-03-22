---
title: "Yuuzhan Vong Total Conversion: Visual Reference Scope"
description: "Asset replacement inventory and visual reference catalog for the NJO total conversion mod"
category: "reference"
created: 2026-03-15
updated: 2026-03-16
tags: [total-conversion, yuuzhan-vong, modding]
---

# Yuuzhan Vong Total Conversion: Visual Reference Scope

## Context

A New Jedi Order total conversion mod for Open Rebellion requires replacing every faction-specific visual asset in the game with Yuuzhan Vong equivalents (one faction) and updated New Republic equivalents (other faction). This plan scopes the exact asset replacement inventory and catalogs every available visual reference source.

## Game Asset Replacement Inventory

The original game has **681 faction-specific visual assets** across 8 categories:

### Per-Faction Entity Counts (must replace for Vong side)

| Category | Count | Original (Empire) | Vong Equivalent |
|----------|-------|-------------------|-----------------|
| Capital ships | 15 | ISD, SSD, Victory SD, etc. | Miid ro'ik, Kor Chokk, Matalok, etc. |
| Fighters | 4 | TIE Fighter/Interceptor/Bomber/Defender | Yorik-et variants (coralskippers) |
| Troops | 5 | Stormtrooper, Fleet, Army, War Droid, Dark Trooper | Warriors, Chazrach, Rakamat, Voxyn, Slayers |
| Special forces | 5 | Commandos, Noghri, Bounty Hunters + 2 | Infiltrators (Nom Anor), Shapers, Hunters + 2 |
| Characters (major) | 3 | Palpatine, Vader + 1 | Shimrra, Tsavong Lah, Nom Anor |
| Characters (minor) | ~27 | Imperial officers | Domain commanders, intendants, shapers |
| Facilities | 10 (shared) | Mine, Refinery, Shipyard, Shield, etc. | Yorik coral pits, Damutek, Yammosk pool, etc. |
| Mission illustrations | ~10 | Espionage, sabotage, etc. | Vong-themed equivalents |

### Visual Asset Types Per Entity

Each entity needs multiple asset types:

| Asset Type | Source | Resolution | Count Per Entity |
|------------|--------|-----------|-----------------|
| Encyclopedia image | EDATA/*.bmp | 400x200 | 1 |
| Status panel sprite | GOKRES.DLL | 64-256px | 2-4 |
| Damage diagram | GOKRES.DLL | 256x256 | 2 (status + detail) |
| Tactical sprite | TACTICAL.DLL | 64x64 | 10 (5 colors × close/far) |
| Map icon | STRATEGY.DLL | 32x32 | 1-3 |

### Total Assets for Vong Faction

| Category | Entities | Assets/Entity | Total |
|----------|----------|--------------|-------|
| Capital ships | 15 | ~8 (ency + status + damage + map) | ~120 |
| Fighters | 4 | ~12 (ency + 10 tactical sprites) | ~48 |
| Troops | 5 | ~4 (ency + status + tactical) | ~20 |
| Special forces | 5 | ~3 (ency + status) | ~15 |
| Characters | 30 | ~3 (ency + portrait + map) | ~90 |
| Facilities | 10 | ~4 (ency + status + map) | ~40 |
| Missions | 10 | 1 (ency illustration) | ~10 |
| **Total** | **79** | | **~343** |

Plus New Republic side updates: ~343 more. **Entity subtotal: ~686 visual assets.**

### UI Chrome & HUD Replacement (beyond entity assets)

| UI Category | Files | TC Action | Priority |
|-------------|-------|-----------|----------|
| Main menu | 6 | Replace entirely with NJO-era art | High |
| Tactical HUD panels | 29 | Reskin faction colors (Empire → Vong organic green/brown) | High |
| Faction buttons | 33 | Reskin labels + colors | Medium |
| Death Star controls | 8 | Replace with Worldship controls | High |
| Event screens | 83 | Replace faction-specific event art | Medium |
| Droid advisor | 501 animated frames | Replace with Vong advisor (villip? yammosk?) | Medium |
| Briefing screens | ~40-100 (ALBRIEF/EMBRIEF.DLL) | Replace cockpit with organic bridge | High |
| Facility indicators | 16 | Replace industrial → organic Vong | Medium |
| Galaxy chrome | 5 | Keep as-is (neutral) | None |
| Weapon gauges | 5 | Keep as-is (universal) | None |
| **UI subtotal** | **~395 BMP + ~1,900 BIN animation data** | | |

**True TC grand total: ~1,081 visual BMPs** (686 entity + ~395 UI BMP). Plus ~1,900 BIN animation sequence files (droid advisors + briefings) that may need regeneration.

See `assets/references/ref-ui/INDEX.md` for full categorized inventory with per-category TC notes.

## Yuuzhan Vong Entity Mapping

### Capital Ships (15 slots)

| Slot | Replaces | Vong Ship | Class | Role |
|------|----------|-----------|-------|------|
| 1 | ISD | **Miid ro'ik** | Warship | Main battle cruiser |
| 2 | SSD | **Kor Chokk grand cruiser** | Grand cruiser | Super capital ship |
| 3 | Victory SD | **Matalok** | Cruiser | Medium warship |
| 4 | Victory II | **Yorik-vec** | Assault cruiser | Heavy assault |
| 5 | Strike Cruiser | **Ro'ik chuun M'arh** | Frigate | Patrol/escort |
| 6 | Interdictor | **Dovin basal ship** | Interdiction | Gravity well (organic) |
| 7 | Carrack | **I'Friil Ma-Nat** | Corvette | Light escort |
| 8 | Lancer | **A-vek Iin** | Light pursuit | Anti-fighter |
| 9 | Dreadnaught | **Suuv Ban D'Krid** | Carrier | Fighter carrier |
| 10 | Escort Carrier | **A-Vek Liluunu** | Transport carrier | Carrier variant |
| 11 | Galleon | **Slaveship** | Transport | Troop transport |
| 12 | Star Galleon | **Yorik-trema** | Landing craft | Assault lander |
| 13 | Assault Transport | **Sh'rip Sh'pa** | Spawn ship | Boarding craft |
| 14 | Death Star | **Worldship (Koros-Strohna)** | Mobile base | Faction HQ |
| 15 | Imperial II | **Yammka warship** | Heavy warship | Elite variant |

### Fighters (4 slots)

| Slot | Replaces | Vong Equivalent |
|------|----------|----------------|
| 1 | TIE Fighter | **Yorik-et** (standard coralskipper) |
| 2 | TIE Interceptor | **Yorik-et pursuit variant** |
| 3 | TIE Bomber | **Yorik-et bomber variant** (magma spitter) |
| 4 | TIE Defender | **Tsik vai** (elite coralskipper) |

### Troops (5 slots)

| Slot | Replaces | Vong Equivalent |
|------|----------|----------------|
| 1 | Stormtrooper | **Warrior caste** (amphistaff + vonduun armor) |
| 2 | Fleet Regiment | **Chazrach** (slave soldiers) |
| 3 | Army Regiment | **Domain warriors** (regular forces) |
| 4 | War Droid | **Rakamat** (siege beast) |
| 5 | Dark Trooper | **Voxyn** (Jedi-hunting bioweapon) |

### Special Forces (5 slots)

| Slot | Replaces | Vong Equivalent |
|------|----------|----------------|
| 1 | Commandos | **Slayers** (elite warriors) |
| 2 | Noghri | **Hunters** (tracking specialists) |
| 3 | Bounty Hunters | **Intendant operatives** (Nom Anor-type infiltrators) |
| 4 | — | **Shapers** (tech specialists) |
| 5 | — | **Shamed One agents** (saboteurs) |

### Facilities (10 slots)

| Slot | Replaces | Vong Equivalent |
|------|----------|----------------|
| 1 | Mine | **Maw luur** (resource extractor) |
| 2 | Refinery | **Biotic processing vat** |
| 3 | Orbital Shipyard | **Yorik coral growing pit** |
| 4 | Advanced Shipyard | **Grand coral pit** |
| 5 | KDY-150 | **Shaper damutek** (research lab) |
| 6 | LNR Series 1 | **Plasma cannon emplacement** |
| 7 | GenCore Level 1 | **Dovin basal defense** (planetary shield) |
| 8 | LNR Series 2 | **Yaret-Kor battery** (volcano cannon) |
| 9 | GenCore Level 2 | **Advanced dovin basal array** |
| 10 | Death Star Shield | **Yammosk pool** (war coordinator) |

### Key Characters (Vong side, 30 slots)

**Major (3):** Shimrra Jamaane, Tsavong Lah, Nom Anor
**Minor (27):** Nas Choka, Shedao Shai, Khalee Lah, Malik Carr, Qurang Lah, Nen Yim, Onimi, Czulkang Lah, Harrar, Vergere, Drathul, Viqi Shesh (collaborator), + 15 domain commanders

## Visual Reference Sources (Ranked by Quality)

### Tier 1: Must-Scrape (official, high-res)

| Source | Content | Access |
|--------|---------|--------|
| **Wookieepedia Category:Images_of_Yuuzhan_Vong** | 157 official images | Jina scrape |
| **New Essential Guide to Vehicles & Vessels** | Ship anatomy diagrams | [Archive.org](https://archive.org/details/starwarsnewessen0000blac) |
| **Tsuyoshi Nagano NJO covers (21)** | Characters, battles, ships | Already have 10; need 11 more |
| **Dark Horse Invasion comics** | Warriors, coralskippers | Panel-by-panel extraction |
| **SWTCG cards** | All entity types | [swtcg.com](https://swtcg.com/Cards?subtype=Yuuzhan+Vong) |

### Tier 2: High-Quality Fan Art

| Source | Content | Access |
|--------|---------|--------|
| **dolynick (DeviantArt)** | 4K PBR ship renders (Miid ro'ik, Worldship, A-Vek Liluunu) | Direct download |
| **Yuuzhan Vong at War mod (ModDB)** | All ship classes in strategy game context | [Gallery](https://www.moddb.com/mods/yuuzhan-vong-at-war/images) |
| **DeviantArt tag:yuuzhanvong** | Warriors, ships, environments | Browse + download |
| **Pinterest collections** | 120+ curated images | Browse |

### Tier 3: Supplementary

| Source | Content | Notes |
|--------|---------|-------|
| **WotC RPG Yuuzhan Vong Sourcebook** | Character + vehicle illustrations | [Scribd](https://www.scribd.com/document/166180758/Yuuzhan-Vong-Sourcebook) |
| **Star Wars Miniatures** | 3D sculpt photos (armor proportions) | RebelScum, Noble Knight |
| **SW Galaxies archives** | In-game Vong content (fragmentary) | SWGEmu |
| **Nagano's portfolio** | [artas1.com](https://artas1.com/tsuyoshi_nagano/) | May have unpublished work |

### Confirmed Gaps (no official art exists)

- Worldship interiors
- Embrace of Pain (standalone)
- Cognition hood (standalone)
- Most biological tech items (dovin basal, villip as standalone)
- Facility interiors (damutek, yammosk pool)

These gaps must be filled via **nano-banana-pro generation** using the Nagano/Essential Guide art as style references.

## Deliverables

| # | Deliverable | Description |
|---|-------------|-------------|
| 1 | `assets/references/ref-vong-ships/` | Ship reference collection (dolynick renders + Essential Guide + Wookieepedia) |
| 2 | `assets/references/ref-vong-warriors/` | Ground force references (TCG cards + fan art + comic panels) |
| 3 | `assets/references/ref-vong-characters/` | Named character references (covers + Wookieepedia) |
| 4 | `assets/references/ref-vong-biotech/` | Biological technology references |
| 5 | `assets/references/ref-vong-facilities/` | Facility/structure references (sparse — generation needed) |
| 6 | `assets/references/ref-njo-japanese/` | Already have 10 Nagano covers; need remaining 11 |
| 7 | `data/manifests/vong-tc.json` | Complete entity mapping manifest (79 entities × asset types) |
| 8 | `docs/vong-total-conversion.md` | Design document with all mappings and generation pipeline |

## Next Steps

1. **Complete Nagano collection** — scrape remaining 11 of 21 NJO covers from Reddit gallery
2. **Scrape dolynick's 4K renders** — Miid ro'ik, Worldship, A-Vek Liluunu, Slayer Ship
3. **Download SWTCG card images** — all Yuuzhan Vong cards from swtcg.com
4. **Scrape Wookieepedia** — all 157 images from Category:Images_of_Yuuzhan_Vong
5. **Create generation manifest** — map every entity slot to reference images + prompt template
6. **Generate gap-fill art** — nano-banana-pro for facilities, biotech, worldship interiors
