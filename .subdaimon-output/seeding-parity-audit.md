# Game Seeding Parity Audit

**Source**: [TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals/initial_game_seeding_logic](https://github.com/TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals/tree/main/initial_game_seeding_logic)
**Our Repo**: `/Users/tomdimino/Desktop/Programming/open-rebellion/`
**Date**: 2026-03-24

---

## Executive Summary

Our implementation covers **fixed asset seeding** (fleets, garrisons, facilities at named systems) from the 9 DAT seed tables. However, the original game's seeding is a **parameter-driven procedural generator** that dynamically assigns system control, support, energy, raw materials, mines, facilities, garrison troops, common units, and character stats using ~50 GNPRTB parameters and weighted random tables. We implement almost none of this procedural layer. Our seeding is static table-driven; the original is randomized and difficulty-aware.

**Overall parity: ~30%** — Fixed asset placement works. Procedural galaxy generation is missing.

---

## Detailed Parity Matrix

### 1. Core System Buckets (Political Control Distribution)

| Aspect | Original (TheArchitect2018) | Our Implementation | Status | Gap |
|--------|---------------------------|-------------------|--------|-----|
| Count core systems by type (0x90) | `system_count_by_type(session, 144)` — counts all core systems | No equivalent — we don't classify core vs rim at seed time | **MISSING** | We load `family_id` as `ExplorationStatus` (Explored/Unexplored) but don't use it to distinguish core vs rim for seeding |
| Compute bucket counts from GNPRTB params 7680/7681 | `get_core_system_support_params()` reads params 7680 (strong) / 7681 (weak) per side + difficulty | Not implemented | **MISSING** | Params 7680/7681 are side-dependent (Alliance/Empire) and difficulty-dependent. We load GNPRTB.DAT but don't use it for seeding. |
| `calculate_value(core_count, percent)` = `floor(n * m / 100)` | Computes alliance_strong, alliance_weak, empire_strong, empire_weak, neutral remainder | Not implemented | **MISSING** | No bucket computation at all |
| Subtract 1 from empire_strong for Coruscant | `_empire_strong -= 1` hard-coded correction | Not implemented | **MISSING** | — |
| Neutral = remainder after all buckets | `_neutral = core_count - all buckets` | Not implemented | **MISSING** | — |

### 2. System Iteration Order

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| Shuffle all systems before seeding | `shuffle_array(session.sectors)` — Fisher-Yates shuffle | Not implemented | **MISSING** | We iterate systems in DAT file order |
| Random system assignment from shuffled order | Political control assigned in shuffled order by draining bucket counts | Not implemented | **MISSING** | — |

### 3. Special Systems (Coruscant, Yavin, Alliance HQ)

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| Coruscant: always populated, charted, Empire-controlled, support=100 | `PROP_SYSTEM_UNCHARTED false`, `PROP_SYSTEM_CONTROL EMPIRE`, `PROP_SYSTEM_SUPPORT 100` | `seeds.rs:237-243` — Coruscant garrison + fleet + facilities placed. Control derived post-hoc from asset factions in `lib.rs:244-296`. | **PARTIAL** | Support value not set (no support field in world model). Charted/populated not explicitly set. Control is asset-derived rather than explicit. |
| Coruscant gets: Empire HQ facilities (faclcr), Coruscant units (cmuncr), Empire main fleet (cmunef), Emperor Palpatine | Scripted placement functions | `seeds.rs:237-243,264-271` — CMUNCRTB + CMUNEFTB + FACLCRTB applied. Palpatine placement: characters are loaded but NOT placed at specific systems during seeding. | **PARTIAL** | Characters are loaded into arena but `current_system = None` at load time. Only fleet-assigned characters get location via `lib.rs:300-311`. Named character placement at specific systems is **MISSING**. |
| Yavin: always populated, charted, Alliance-controlled, support=100 | Same as Coruscant but Alliance side | `seeds.rs:255-262` — CMUNYVTB garrison applied. Control derived from assets. | **PARTIAL** | Same gaps: no support value, no explicit control set, no charted/populated flag. |
| Yavin personnel: Leia, Luke, Han, Wedge, Chewbacca, Jan Dodonna placed at Yavin | `seed_yavin_personnel()` places 6 named characters | **MISSING** | Characters loaded but NOT assigned to Yavin. No character-to-system placement at seed time. |
| Alliance HQ: first eligible rim system (not Yavin), populated, charted, Alliance-controlled, support=100 | Dynamic selection from shuffled rim systems | **MISSING** | No dynamic Alliance HQ selection. FACLHQTB placed at Yavin (YAVIN_SEQ_ID) rather than a separate HQ system. |
| Alliance HQ gets: HQ facilities (faclhq), HQ units (cmunhq), Alliance main fleet (cmunaf), Mon Mothma | Scripted functions | `seeds.rs:246-253,273-280` — CMUNHQTB + FACLHQTB applied to Yavin. But original puts these at a DIFFERENT system from Yavin. | **PARTIAL** | Both Yavin garrison and Alliance HQ are placed at Yavin in our code. Original puts HQ at a separate random rim system. Mon Mothma not placed at HQ. |
| Alliance main fleet 50/50 Yavin vs HQ | `random_outcome(param_5129)` — param 5129 = 50 | **MISSING** | Fleet placed statically, no random location roll |

### 4. Populated vs Unpopulated Systems

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| Core systems: `random_outcome(param_7730)` — 100% in stock data | Core systems always populated | Not implemented — all systems loaded as-is from DAT | **MISSING** | No populated/unpopulated distinction |
| Rim systems: `random_outcome(param_7731)` — 31% in stock data | Only ~31% of rim systems start populated | Not implemented | **MISSING** | All systems treated equally |
| Unpopulated systems: no facilities, no garrison, support=50 | Default state for non-populated | Not implemented | **MISSING** | No population-conditional logic |

### 5. Support Values

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| `popularity_alliance` / `popularity_empire` field | Original uses integer `support` 0-100, single value per system | We have `popularity_alliance: f32` + `popularity_empire: f32` (dual 0.0-1.0) | **STRUCTURAL MISMATCH** | Original has ONE support value (0-100) per system. We have TWO popularity floats. Initial values are `0.0` for both at load time. |
| Core owned-with-support: base=60, random extra=30 (params 7682/7683) | `get_core_system_support_value()` with difficulty/side params | Not implemented | **MISSING** | — |
| Core owned-without-support: base=20, random extra=30 (params 7684/7685) | Same function, different params | Not implemented | **MISSING** | — |
| Core neutral: `random(0..18) + (50 - 18/2)` → 41-59 range (param 7764) | Centered on 50 with param-driven spread | Not implemented | **MISSING** | — |
| Rim populated support: `random(0..0) + 50` = exactly 50 (param 7765=0) | Simple formula | Not implemented | **MISSING** | Our popularity starts at 0.0 |
| Rim unpopulated support: always 50 | Hard-coded | Not implemented | **MISSING** | — |

### 6. Energy and Raw Materials

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| System has `total_energy` and `raw_materials` fields | Per-system economic capacity | **MISSING** | No energy or raw materials fields on `System` struct |
| Core energy: `clamp(10 + random(0..4), 0, 15)` (params 7721/7722/7713/7714) | Parameter-driven | **MISSING** | — |
| Core raw: `clamp(5 + random(0..9), 0, 15)`, then `min(raw, energy)` (params 7723/7724) | Raw capped by energy | **MISSING** | — |
| Rim energy: `clamp(1 + random(0..4) + random(0..9), 0, 15)` (params 7725-7727) | Two random terms | **MISSING** | — |
| Rim raw: `clamp(1 + random(0..14), 0, 15)`, then `min(raw, energy)` | Single random term | **MISSING** | — |
| Post-adjustment: ensure energy >= used_energy, raw >= mine_count | Facility seeding may overshoot | **MISSING** | — |

### 7. Facility Seeding (Procedural)

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| Fixed HQ facilities from FACLCRTB / FACLHQTB | Table-driven | `seeds.rs:264-280` — FACLCRTB at Coruscant, FACLHQTB at Yavin | **MATCH** | Correctly loads and places |
| Procedural facility loop: once per energy slot | `for count in 0..total_energy`, mine or weighted facility per slot | **MISSING** | No procedural facility generation |
| Mine chance: `(total_raw - mine_count) * mine_multiplier` (core=4, rim=2 via params 7766/7767) | Higher raw → more mines | **MISSING** | — |
| Non-mine facility: weighted random from syfccr (core) or syfcrm (rim) tables | Refinery 36%, Shipyard 43%, Training 3%, etc. | **MISSING** | We don't load SYFCCRTB or SYFCRMTB tables |
| Core systems more mine-biased than rim (multiplier 4 vs 2) | Intentional balance difference | **MISSING** | — |

### 8. Garrison Troops (Uprising Prevention)

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| Base garrison threshold: param 7761 = 60 | If support < 60, troops required | **MISSING** | No garrison-for-support logic at seed time |
| Troop formula: `floor((60 - support + 9) / 10)` (param 7762 = -10) | Ceil division of support gap | **MISSING** | — |
| Alliance → ALLIANCE_ARMY_REGIMENT, Empire → IMPERIAL_ARMY_REGIMENT | Faction-appropriate troops | **MISSING** | — |
| Neutral systems exempt | No garrison on neutral systems | **MISSING** | — |

### 9. Starting Fleets and Fixed Units

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| Yavin units from cmunyv | Table CMUNYVTB | `seeds.rs:255-262` | **MATCH** | Loaded and applied |
| Alliance HQ units from cmunhq | Table CMUNHQTB | `seeds.rs:246-253` (at Yavin, not separate HQ) | **PARTIAL** | Placed at wrong system (Yavin instead of random HQ) |
| Coruscant units from cmuncr | Table CMUNCRTB | `seeds.rs:237-243` | **MATCH** | Loaded and applied |
| Alliance main fleet from cmunaf | Table CMUNAFTB (2 groups) | `seeds.rs:206-216` | **PARTIAL** | Groups distributed across 3 proximity-based Alliance systems instead of Yavin + HQ with 50/50 roll |
| Empire main fleet from cmunef | Table CMUNEFTB (1 group) | `seeds.rs:196-204` | **PARTIAL** | Distributed across 10 proximity-based Empire systems instead of all at Coruscant |

### 10. Common Unit Seeding (Maintenance-Budget-Based)

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| Compute available maintenance: `total_maintenance - total_current_cost` | Per-side budget | **MISSING** | No maintenance budget tracking at seed time |
| Deploy budget: `floor(available * side_percent / 100)` from params 5168-5170 | Size-dependent (standard/large/huge), side-dependent, difficulty-dependent | **MISSING** | — |
| Pick random eligible system | `PROP_SYSTEM_SEED_GET_BY_COUNT` — random among owned systems | **MISSING** | — |
| Roll weighted unit bundle from cmunal (Alliance) or cmunem (Empire) | E.g., ISD + TIE + Stormtroopers, or Bulk Cruiser + X-wings | **MISSING** | We apply these as static garrison tables, not as budget-driven random deployment |
| Ship bundles create/join fleets; non-ship bundles go to system inventory | Object type routing | **MISSING** | — |
| Loop until budget exhausted | Spend down maintenance allocation | **MISSING** | — |

### 11. Character Stat Generation

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| `setup_character()` generates stats from base + random(0..variance) | Every character gets randomized stats at game start | **MISSING** | Characters loaded with raw DAT base/variance values. No stat roll at seed time. `Character` stores `SkillPair { base, variance }` but never rolls them into actual values. |
| Jedi: `jedi = datasheet.jedi \|\| random_outcome(jedi_probability)` | Some characters randomly become Jedi | **MISSING** | `jedi_probability` loaded but never rolled |
| Force level: `jedi_level_base + random(0..jedi_level_variance)` | Random force level | **MISSING** | — |
| Diplomacy/espionage/combat with Jedi force boost: `base + floor((base + random) * force / 100)` | Force-sensitives get stat boosts | **MISSING** | — |
| Empire Seat of Power bonus on leadership: `leadership + floor((leadership + random) * param_3076 / 100)` | Empire characters get leadership bonus | **MISSING** | — |
| All characters start with `rank = 0` | Fresh ranks | Not explicitly set (no rank field) | **MISSING** | No rank field on Character |

### 12. Fixed Personnel Placement

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| Yavin: Leia, Luke, Han, Wedge, Chewbacca, Jan Dodonna | 6 named characters at Yavin | **MISSING** | Characters loaded but `current_system = None`. Only fleet-carried characters get location tracking. |
| Alliance HQ: Mon Mothma | At dynamically-selected HQ | **MISSING** | Mon Mothma loaded but not placed |
| Coruscant: Emperor Palpatine | At Coruscant | **MISSING** | Palpatine loaded but not placed |

### 13. Random Personnel Distribution

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| Empire main list always seeded: Vader, Jerjerrod, Ozzel, Piett, Veers, Needa | `seed_personnel_randomly()` — random system or fleet ship | **MISSING** | Characters loaded but not distributed |
| Extra personnel count from params 5120-5122 (galaxy size dependent) | E.g., 4 extra per side in huge galaxy | **MISSING** | — |
| `get_major_character()`: random unplaced character from mnchar | Filters by side, excludes already-seeded | **MISSING** | — |
| Placement: random index across (system_count + fleet_count), either planet or fleet ship | Even distribution across territory | **MISSING** | — |

### 14. Difficulty Modifiers on Initial State

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| Params 7680/7681 vary by difficulty (a= alliance side, e= empire side) | Strong/weak support percentages change with difficulty and which side you play | **MISSING** | Difficulty selected in GameSetupState but not used during seeding |
| Maintenance budgets (params 5168-5170) vary by size and difficulty | More units on harder difficulties | **MISSING** | — |
| Character stat rolls unaffected by difficulty | Same stats regardless | N/A | — |
| Support values (params 7682-7685) have side-specific columns | Different base support for Alliance vs Empire at each difficulty | **MISSING** | — |

### 15. Random Seed Handling

| Aspect | Original | Our Implementation | Status | Gap |
|--------|---------|-------------------|--------|-----|
| System shuffle uses `Math.random()` (no seeded RNG in the JS reference) | Effectively non-deterministic | No shuffle at all | **MISSING** | We have `rand_xoshiro` in playtest but don't use seeded RNG for galaxy generation |
| Each random roll uses `get_random_number(max)` / `get_random_outcome(percent)` | Utility functions for bounded random | No random rolls during seeding | **MISSING** | — |

---

## Structural Gaps Not Tracked in Existing Docs

These gaps are NOT mentioned in `agent_docs/roadmap.md` or `agent_docs/systems/ai-parity-tracker.md`:

1. **No procedural galaxy generation** — The entire seeding pipeline (sections 1-2, 4-8, 10, 13) is missing. Our seeding is a static table loader, not the randomized parameter-driven generator the original uses.

2. **No support/popularity initial values** — `popularity_alliance` and `popularity_empire` start at `0.0` for all systems. The original sets support values from 20-100 for controlled systems and ~41-59 for neutrals.

3. **No energy/raw materials system** — The `System` struct has no `total_energy` or `raw_materials` fields. These are fundamental to the original's economy and facility placement.

4. **No character stat rolling** — Characters retain their raw DAT `base`/`variance` values. The original rolls these into concrete stats at game start, with Jedi force boosts and Empire Seat of Power bonuses.

5. **No character system placement** — Named characters (Leia at Yavin, Palpatine at Coruscant, etc.) are loaded into the arena but have `current_system = None` unless they happen to be in a seeded fleet.

6. **Alliance HQ placed at Yavin** — The original selects a random rim system (not Yavin) for the Alliance HQ. Our code places both Yavin garrison AND Alliance HQ facilities at Yavin, collapsing two distinct locations into one.

7. **Fleet distribution uses proximity model** — Our `select_starting_systems()` distributes fleets across Empire's 10 nearest / Alliance's 3 nearest systems. The original places fleets ONLY at the 3 special systems (Coruscant, Yavin, Alliance HQ) and uses the maintenance-budget common unit seeding for additional fleet distribution.

8. **No maintenance-budget unit seeding** — The largest source of starting forces (section 10) is entirely absent. The original computes available maintenance per side, then repeatedly rolls random unit bundles and places them at random owned systems until the budget is exhausted.

9. **No garrison-for-support troops** — The original adds garrison troops to prevent uprisings on low-support systems at seed time. We have uprising mechanics during gameplay but no preventive garrison at start.

10. **Difficulty/galaxy size don't affect seeding** — `GameSetupState` captures these choices but they're not passed to `load_game_data()` or `apply_seeds()`. The original uses ~20 difficulty-keyed GNPRTB parameters to vary initial state.

---

## What We DO Implement Correctly

| Feature | Implementation | Notes |
|---------|---------------|-------|
| 9 seed table loading (DAT parsing) | `seeds.rs` — all 9 tables parsed and applied | Round-trip validated |
| Coruscant fleet + garrison | CMUNEFTB + CMUNCRTB at Coruscant | Items correctly dispatched by family byte |
| Yavin garrison | CMUNYVTB at Yavin | Ground units placed correctly |
| Facility placement at HQs | FACLCRTB (Coruscant) + FACLHQTB (at Yavin) | Facility types correctly routed |
| Army tables | CMUNEMTB + CMUNALTB distributed across starting systems | Items dispatched correctly |
| Family byte item dispatch | `dispatch_fleet_item()`, `dispatch_ground_item()`, `dispatch_facility_item()` | Correct routing by 0x10/0x14/0x1C/0x20/0x28-0x2D/0x3C |
| Control derived from assets | `lib.rs:244-296` — post-seed faction detection from asset ownership | Reasonable approximation |
| GNPRTB loading | `lib.rs:313-329` — all entries loaded into `GnprtbParams` | Available but not used for seeding |

---

## Parity Summary Table

| Category | Items Checked | MATCH | PARTIAL | MISSING | Parity % |
|----------|:---:|:---:|:---:|:---:|:---:|
| 1. Core System Buckets | 5 | 0 | 0 | 5 | 0% |
| 2. System Iteration Order | 2 | 0 | 0 | 2 | 0% |
| 3. Special Systems | 7 | 0 | 5 | 2 | 36% |
| 4. Populated vs Unpopulated | 3 | 0 | 0 | 3 | 0% |
| 5. Support Values | 6 | 0 | 0 | 6 | 0% |
| 6. Energy & Raw Materials | 6 | 0 | 0 | 6 | 0% |
| 7. Facility Seeding | 5 | 1 | 0 | 4 | 20% |
| 8. Garrison Troops | 4 | 0 | 0 | 4 | 0% |
| 9. Starting Fleets/Units | 5 | 2 | 3 | 0 | 70% |
| 10. Common Unit Seeding | 6 | 0 | 0 | 6 | 0% |
| 11. Character Stats | 6 | 0 | 0 | 6 | 0% |
| 12. Fixed Personnel | 3 | 0 | 0 | 3 | 0% |
| 13. Random Personnel | 4 | 0 | 0 | 4 | 0% |
| 14. Difficulty Modifiers | 4 | 1 | 0 | 3 | 25% |
| 15. Random Seed Handling | 2 | 0 | 0 | 2 | 0% |
| **TOTAL** | **68** | **4** | **8** | **56** | **~12%** |

*(PARTIAL counted as 0.5 for parity %)*

---

## Recommended Priority Order for Implementation

1. **Character stat rolling + placement** (sections 11-12) — Needed for any gameplay involving characters. Characters with `SkillPair { base, variance }` but no concrete stats are non-functional.

2. **System support/popularity initialization** (section 5) — Without initial support values, diplomacy/uprising mechanics have nothing to work with.

3. **Core/rim system classification + political control buckets** (sections 1-2, 4) — Prerequisite for all procedural seeding.

4. **Alliance HQ as separate system** (section 3) — Critical placement fix.

5. **Energy/raw materials + procedural facilities** (sections 6-7) — Economic layer.

6. **Maintenance-budget common unit seeding** (section 10) — The biggest source of starting forces.

7. **Garrison troops for uprising prevention** (section 8) — Balance feature.

8. **Random personnel distribution** (section 13) — Non-fixed characters need homes.

9. **Difficulty modifier integration** (section 14) — Connect `GameSetupState` to seeding params.

10. **Seeded RNG** (section 15) — Determinism for testing and reproducibility.
