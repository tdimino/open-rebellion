---
title: "Roadmap"
description: "Development milestones from Galaxy Viewer through Release packaging"
category: "agent-docs"
created: 2026-03-11
updated: 2026-03-16
---

# Roadmap

## Galaxy Viewer -- COMPLETE
*Interactive galaxy data viewer -- native + WASM*

Commits: `4e258bb` (scaffold) > `5905497` (dat-dumper + types) > `fc968ec` (renderer + WASM) > `22b61a6` (review fixes)

Delivered:
- Cargo workspace with 5 crates
- dat-dumper: 22/22 documented DAT files parsed, 3 structural patterns, round-trip byte validated
- rebellion-core: DatId newtype, 11 slotmap keys, dat/ enums, world/ structs, GameWorld root
- rebellion-data: DAT to GameWorld loader (sectors > systems > ships > fighters > characters)
- rebellion-render: macroquad galaxy map with pan/zoom/click-to-select, egui info panel + status bar
- WASM build: 2.9MB browser artifact via scripts/build-wasm.sh

## Living Galaxy -- COMPLETE
*Automated galaxy simulation — shipped 2026-03-13 by Knesset Athirat*

All 11 deliverables shipped. ~8.7K LOC, 105+ unit tests. See `docs/reports/2026-03-13-knesset-athirat-swarm-report.md`.

Delivered:
- TEXTSTRA.DLL string extraction via pelite (real entity names)
- GameClock with pause/1x/2x/4x speed controls
- 9 seed table loaders + fleet instantiation
- All 51/51 DAT parsers with round-trip validation (29 new)
- Manufacturing system with production queues
- Mission system (diplomacy, recruitment) -- ported from rebellion2's Mission.cs
- Event system (conditional triggers, chaining, deterministic rng)
- AI manager (officer assignment, production priority, fleet deployment)
- Message log UI (egui bottom panel, 6 color-coded categories)
- Mod loader: TOML manifests, RFC 7396 merge patch, semver, hot reload
- Main loop integration (all systems wired)

## War Room -- COMPLETE
*Full strategy game without combat*

Committed: `6d47a10` (integration wiring — fog/fleet/panels/encyclopedia/audio into main loop)

Delivered:
- Player faction selection + 5 UI panels (officers, fleets, manufacturing, missions)
- Fleet movement on galaxy map (diamond icons, dashed routes, ETA labels)
- Fog of war with monotonic reveal + advance intel at 50% transit
- Encyclopedia viewer with 4 tabs + BMP texture cache from EData/
- Audio system via quad-snd (CoreAudio/ALSA/WebAudio)
- Main loop integration: all draw calls + event hooks wired

## War Machine -- COMPLETE
*Complete strategy game*

1. Space combat: auto-resolve first, then tactical 2D view
2. Ground combat: regiment engagement, orbital bombardment
3. Blockade mechanics, Death Star construction/destruction
4. Victory conditions (capture enemy HQ, destroy Death Star / find Rebel base)
5. 3D tactical models: nano-banana-pro concept art → Hunyuan3D Pro/Meshy → Blender sprite sheets. Pipeline proven in World War Watcher (14 models). See `@agent_docs/assets.md`.

Ghidra RE of REBEXE.EXE complete: 5,127 functions decompiled, combat call chain fully traced, bombardment formula decoded, 111 GNPRTB parameters mapped, C++ class hierarchy reconstructed. See `ghidra/notes/` for 7 scholar docs (4,179 lines) and 5,127 decompiled C files.

## Full Parity -- COMPLETE
*Feature-complete — shipped 2026-03-16 by Knesset Shapash*

Delivered (v0.5.0):
- 4 scripted story chains (Luke Dagobah, Final Battle, Bounty Hunters, Jabba's Palace)
- Han Solo speed bonus, betrayal mechanics, decoy system, escape system
- 10 new Character fields promoted from DAT, mission state flags
- 6 new EventConditions + 6 new EventActions, 15 RE event ID constants
- All 9 mission types with world-mutating effects (sabotage, assassination, rescue, abduction, incite uprising)

## Mod Workshop -- COMPLETE
*Release candidate — Knesset Elat (v0.6.0)*

Delivered:
- Sensor-radius fog (detection field on CapitalShipClass)
- Captivity state tracking (is_captive, captured_by, capture_tick)
- Research pure advance() contract (caller applies level-ups)
- Save format v4 with mod metadata + FNV-1a hash, migration framework
- ModRuntime: discovery, enable/disable, structured errors, config persistence
- Mod Manager egui panel (discover, toggle, reload)
- ESCAPETB per-tick escape check wired into main loop

## Release -- READY
*Release packaging — Knesset Hokhmah (v0.7.0)*

Delivered:
- 8 new story event chains (0x1e1, 0x1e5, 0x390-0x39A)
- 4 new EventCondition variants + 3 new EventAction variants
- Release packaging scripts (macOS + web)
- Example mod (star-destroyer-rebalance)
- GitHub Actions CI (test + WASM build)
- Vendored web/gl.js
- Release profile (LTO, strip, codegen-units=1)

Remaining:
- HD asset pack bulk execution
- Video playback (Smacker → WebM)
- Distribution: itch.io (web), Homebrew (macOS), GitHub Releases

## AI Overhaul -- COMPLETE
*v0.13.0 — AI balance, observability, autoresearch*

Delivered:
- Distance-based fleet transit (Euclidean, DISTANCE_SCALE=2, MIN_TRANSIT_TICKS=10)
- Starting force distribution (Empire 10 systems, Alliance 3)
- Garrison strength scoring (ships + troops + facilities)
- Galaxy-wide strategic bucketing (7 categories)
- Per-fleet attack targeting with deconfliction + battle repeat penalty
- Two-pass deployment (original parity: primary assign + secondary redistribute)
- Faction-asymmetric doctrine (Empire→HQ strike, Alliance→guerrilla)
- Role-based character AI (Jedi reserved, diplomats prioritized)
- Production doctrine (capital ships > fighters > yards)
- Build completion wiring (manufactured items added to GameWorld)
- HQ garrison defense (first fleet defends HQ)
- ControlKind state machine (Uncontrolled, Controlled, Contested, Uprising)
- Human-readable JSONL event payloads (system/character names)
- Enhanced --summary (galaxy control, transit state, combat diagnostics)
- --jsonl streaming flag for stdout
- REPL commands: systems, transit, events N
- eval_game_quality.py (8 sub-metrics, degenerate detection)
- autoresearch_loop.py (Karpathy self-improvement loop)
- AI parity tracker (maps 6 original functions to our implementation)
- Death Star fleet exemption (always targets enemy HQ)

Campaign results: VICTORY at tick 1188, 211 battles, eval score 0.59

## AI Parity Status (as of 2026-03-21)

Based on 3-agent review against Ghidra RE of REBEXE.EXE. See `agent_docs/systems/ai-parity-tracker.md` for full matrix.

### Core Pipeline (6 Functions)

| # | Original | Status | Gap |
|---|----------|--------|-----|
| 1 | FUN_00519d00 Galaxy evaluation | DONE | 7 buckets + control_ratio + aggression scaling |
| 2 | FUN_00537180 Primary deployment | DONE | Per-fleet targeting with scoring function |
| 3 | FUN_005385f0 Secondary deployment | DONE | Aggression-scaled: offensive piles onto attacks, defensive distributes across undefended systems |
| 4 | FUN_00502020 Garrison strength | DONE | Ships + troops + facilities |
| 5 | FUN_00508250 Dispatch validation | MISSING | Original has 18 boolean AND checks; we have no pre-dispatch validation |
| 6 | FUN_00520580 Movement orders | DONE | With already_moving dedup |

### Confirmed Gaps (P0)

1. **AI research dispatch** — AI never advances tech tree. No `evaluate_research()`. Characters with ship_design/troop_training skills sit idle. ~80 LOC to implement.
2. **Ratio-based galaxy evaluation** — Original scales aggression proportionally to galaxy control %. A faction with 3/200 systems should be defensive; 100/200 should be aggressive. We treat all the same.
3. **Proportional redistribution (Pass 2)** — Original divides remaining units across ALL friendly systems by count + remainder. We pick the first undefended system.

### Confirmed Gaps (P1)

4. **Troop deployment AI** — No `AIAction::MoveTroops`. AI doesn't build or deploy ground forces. Stranded troops (troops without ships) not detected.
5. **Defense facility construction** — `evaluate_production` only builds capital ships, fighters, and yards. No defense facilities at key systems.
6. **Dispatch validation cascade** — Original has 18 pre-checks (alive, not captured, faction match, capacity, etc.). We dispatch without validation.
7. **Faction-specific Pass 2 evaluator** — Original uses `FUN_00506ea0` for different Alliance vs Empire redistribution. We use the same logic for both.

### Possible Gaps (Need More RE)

8. Death Star AI beyond "go to HQ" (escort, retreat, target selection between HQ and populated systems)
9. `FUN_0052e970` scoring function not fully decoded — may have more parameters than our 4-factor model
10. `FUN_005202d0` system pre-validation — may check "not being bombarded" or "not in uprising"
11. Early-game vs late-game scaling — original may have phase-dependent thresholds

### Code Hygiene (from simplicity reviewer)

- 27 LOC dead code: unused `combat_score`, `sys_json`, `hull * 1`, unused imports, unused `GalaxyState` fields
- 5 stale doc comments still referencing `controlling_faction` (now `ControlKind`)
- 80 LOC test boilerplate reducible via `impl Default for Character`

---

## Roadmap: AI Parity Completion (v0.14.0)

### Phase A: Config Externalization — COMPLETE
*Unblocks autoresearch. ~160 LOC.*

- [x] Created `crates/rebellion-core/src/tuning.rs` — `GameConfig` with 4 sub-configs (`AiConfig`, `MovementConfig`, `ProductionConfig`, `ScoringConfig`), 16 tunable parameters
- [x] Externalized all AI/movement/production constants — threaded `&GameConfig` through `AISystem::advance()`, `score_attack_target()`, `fleet_transit_ticks_with_config()`
- [x] Added `--config <path>` to playtest binary (JSON with `#[serde(default)]`)
- [x] Default config at `configs/autoresearch/default.json`
- [x] Parity/augmentation split documented per-field in tuning.rs
- [x] Updated `autoresearch_loop.py` to write and pass config files

### Phase B: Research Dispatch (P0 Gap #1) — COMPLETE
*AI tech tree progression. ~60 LOC.*

- [x] Added `evaluate_research()` to AISystem — assigns idle characters to Ship/Troop/Facility trees
- [x] Characters matched by primary skill (ship_design ≥ 30, troop_training ≥ 30, facility_design ≥ 30)
- [x] `AIAction::DispatchResearch` variant + telemetry event
- [x] Wired into simulation tick (both factions) and interactive app

### Phase C: Ratio-Based Galaxy Evaluation (P0 Gap #2) — COMPLETE
*Aggression scaling. ~30 LOC.*

- [x] Ported FUN_0053e190: `control_ratio` and `aggression` fields on GalaxyState
- [x] Galaxy control % → aggression: 10% → 0.18, 50% → 0.5, 90% → 0.82
- [x] `max_fronts` scaled by aggression (weak faction opens fewer attack fronts)
- [x] Pass 2 behavior: high aggression → pile onto attacks; low → reinforce

### Phase D: Proportional Redistribution (P0 Gap #3) — COMPLETE
*Replace Pass 2. ~25 LOC.*

- [x] Defensive reinforcement distributes across ALL undefended systems (not just first)
- [x] Round-robin by fewest incoming reinforcements (even distribution)
- [x] Falls back to all controlled systems when no undefended remain

### Phase E: Combat Spread
*Territory expansion via diplomacy. ~40 LOC.*

- [ ] Successful diplomacy missions flip neutral systems to faction control
- [ ] AI dispatches diplomacy to border-adjacent neutral systems
- [ ] More controlled systems → more attack targets → diverse battle locations

### Phase F: Autoresearch Execution
*Run the loop, find optimal config.*

- [ ] Build release binary
- [ ] Run 20 iterations × 3 seeds
- [ ] Analyze: best config, score trajectory, parameter sensitivity
- [ ] Document findings in `autoresearch/results/`

---

## Roadmap: Addons (v0.15.0+)

### Addon 1: Create Your Own Character (Demiurgos)
*Plan at `docs/plans/2026-03-21-addon-create-your-own-character.md`*

**Phase 1: Homeworld System** (universal — enriches base game)
- [ ] Add `homeworld: Option<SystemKey>` to Character
- [ ] Create `data/homeworlds.json` with canon homeworld assignments (Luke→Tatooine, etc.)
- [ ] HomeworldModifier: +10% at homeworld, -5% when enemy controls, -15 loyalty on fall
- [ ] Wire into missions.rs probability + betrayal.rs loyalty
- [ ] Display in Officers panel and Encyclopedia

**Phase 2: Character Creation Core**
- [ ] Add `CharacterOrigin`, `AcademyArchetype`, `ArchetypeTemplate` to rebellion-core
- [ ] 6 archetypes: Diplomat, Operative, FleetOfficer, GroundCommander, Engineer, Fringer
- [ ] 7-step creation UI as egui modal (faction → archetype → skills → homeworld → Force roll → name → review)
- [ ] Point-buy within archetype floors/ceilings (5-point increments)
- [ ] Max 3 custom characters per game

**Phase 3: Force Sensitivity**
- [ ] 8-16% base chance depending on species + archetype
- [ ] Success: jedi_probability 15-30 (Luke = 100 — custom characters strictly inferior)
- [ ] Wire to existing JediSystem progression

**Phase 4: Portraits & Flavor**
- [ ] 128 portrait BMPs (8 per species × 2 factions × 8 species)
- [ ] Backstory templates: "Former Separatist Holdout", "Outer Rim Survivor", "Imperial Defector"
- [ ] Encyclopedia integration

**Phase 5: Mod Integration**
- [ ] Extend mod loader for `"action": "add"` entity creation
- [ ] DatId allocation from reserved custom range (0xFF000000+)
- [ ] Example mod: "Kira Noss" character pack

### Addon 2: Expanded Galaxy (Future)
- Additional star systems from EU (200 → 400)
- New sectors (Unknown Regions, Corporate Sector)
- Hyperspace lane network (optional overlay on Euclidean model)

### Addon 3: Tactical Combat View (Future)
- 2D tactical view for space combat (currently auto-resolve only)
- Ship placement, formation selection, manual targeting
- 3D model sprite sheets from Hunyuan3D Pro pipeline

---

## Known Technical Debt

- dat-dumper in `tools/` is also a library dep of rebellion-data -- works but unconventional
- WASM cfg guards added but browser data loading returns error stub (no fetch API yet)
- Save v3 files rejected (bincode layout incompatible) -- no migration possible without SaveStateV3
- `enabled_sorted()` silently returns empty on dependency resolution errors
- `ModConfig::load()` silently drops corrupted config.toml
- No additive entity creation in mods -- patches only modify existing entities (until Addon 1 Phase 5)
- 27 LOC dead code identified by simplicity reviewer (2026-03-21)
- 5 stale doc comments referencing old `controlling_faction` field
