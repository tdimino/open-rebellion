---
title: "Roadmap"
description: "Development milestones from Galaxy Viewer through Release packaging"
category: "agent-docs"
created: 2026-03-11
updated: 2026-04-06
tags: [roadmap, planning, milestones, parity]
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

## UI Rebuild — IN PROGRESS
*v0.16.0 — Player-facing UI rebuild following `docs/plans/2026-03-22-001-feat-complete-game-ui-rebuild-plan.md`*

### Completed (2026-03-23/24)
- Phase 1.1: Main Menu Screen (title, New Game / Load Game / Quit)
- Phase 1.2: Game Setup (galaxy size, difficulty, faction selection)
- Phase 1.4: TEXTSTRA.DLL string extraction (1,347 entity names)
- Phase 2.4: Star Wars egui theme (dark space + gold/amber accents, Liberation Sans)
- Phase 3.3: System Context Menu (right-click: control, popularity, garrison, actions)
- Phase 3.4: Fleet Context Menu (right-click: composition, commander, transit, actions)
- Phase 4.1: Research Panel (3 tech tree tabs, progress bars, character assignment)
- Phase 4.2: Jedi Training Panel (Force roster, XP bars, training controls)
- Phase 4.3: Fleet Composition Editor (assign/remove officers, merge fleets)
- Phase 4.4: Bombardment Targeting Panel (fleet selection, damage forecast, fire button)
- Phase 4.5: Death Star Control Panel (construction progress, superlaser targeting, movement)
- Phase 4.6: Loyalty & Uprising Dashboard (per-system danger, betrayal risk)
- Phase 4.7: Expanded System Detail Panel (fleets, troops, facilities, popularity bars)
- Phase 4.8: Character Detail View (skills, Force progression, location, assignment)

### Completed (Knesset Demiurge, 2026-03-24)
- Phase 2.1-2.2: Cockpit Frame + Control Buttons (BmpCache, faction chrome, 9 buttons)
- Phase 3.1: Facility Status Icons (colored squares by type, zoom-scaled)
- Phase 3.2: Sector Boundaries (Graham scan convex hull, 3 SectorGroup colors)
- Phase 3.5: Blockade Visual Indicators (red ring + fill on blockaded systems)
- Phase 5: Tactical Combat View (2D arena, phased combat, targeting, retreat, ground)
- Phase 6: Audio Integration (quad-snd soundtrack, 285 voice lines, SFX, context music)
- Phase 7: Entity Portraits (GOKRES.DLL 61 portraits + 57 ship status views)

### Remaining UI
- Phase 7: HD Visual Polish (upscaled BMPs beyond original art)

### Knesset Kothar wa Khasis — COMPLETE (2026-04-07)
*U2 + C1 — the two final deferred tasks from Knesset Resheph*

Delivered:
- **U2 — Native video playback for decoded WebM cutscenes**: new `crates/rebellion-render/src/video_player.rs` with `VideoPlayer::open/advance/current_frame/is_finished/stop`. Runtime path is PNG frame sequences + WAV sidecars (no ffmpeg/libvpx/gstreamer runtime deps). `scripts/decode-cutscenes.sh` is a one-time local ffmpeg decode that produces `assets/references/cutscene-frames/<name>/frame-*.png` + `metadata.json` + `<name>.wav` (ignored by git). `GameMode::Cutscene` added to `rebellion-app/src/main.rs`, plays `000.webm` on startup before MainMenu and `201.webm`/`202.webm` on victory/defeat. ESC/SPACE skip handling. Graceful `VideoError::NotDecoded` path logs a skip message and proceeds when decoded assets are missing. WASM build unaffected via a cfg-gated no-op stub. New dep: `quad-snd` on rebellion-render (player owns its own audio context).
- **C1 — BIN-driven advisor animation**: `parse_advisor_bin()` in `rebellion-render/src/advisor.rs` parses the `u16 frame_count + u16 frame_id[]` format with `BinError::{TruncatedHeader, TruncatedFrames, LengthMismatch}` real-error rejection. `AdvisorState` now carries `bin_sequences`, `current_sequence`, and `frame_cursor`; the rewritten `update(dt)` walks authored sequences on their `default_interval` and picks the next sequence from idle/normal/critical bands (contiguous thirds of the sorted valid BIN list) based on current `AdvisorPriority`. BIN frame IDs map into the sorted BMP pool via modulo — commented as honest best-effort because the real DLL resource-index table is still undocumented. Falls back to the legacy sorted-frame cycling if zero BINs parse successfully, so the droid never freezes. 3 new parser/state tests added.

Known gaps left for future work:
- Only ~24% of advisor BINs (183 of ~752 per faction) match the simple `u16 count + u16 ids` shape. The other ~76% declare inconsistent lengths and are silently skipped — the game almost certainly uses one or more additional header variants (timing bytes, loop counts, control flags) that have not been decoded. The 24% that do parse now drive authored sequence length/variation; the rest fall through to the legacy path.
- Victory/defeat cutscenes return directly to `MainMenu` rather than chaining through the existing egui victory modal.
- Cutscene audio plays at fixed volume through the player's own quad-snd context rather than through `AudioVolumeState`.
- Task #132 report in `docs/plans/2026-04-03-001-feat-knesset-resheph-final-sprint-plan.md` tracks these.

### Game Seeding Parity — CRITICAL GAP (~12% parity)

*Discovered 2026-03-24 via audit against TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals/initial_game_seeding_logic*

Full report: `.subdaimon-output/seeding-parity-audit.md`

**What works**: 9 seed table DAT parsing + dispatch, Coruscant fleet/garrison/facilities, Yavin garrison, army table distribution.

**Fixed (M1-M4, 2026-03-24):**
1. ~~Character stat rolling + placement~~ — DONE. SkillPair rolled to concrete stats, named characters placed (Luke/Leia/Han at Yavin, Vader/Palpatine at Coruscant, Mon Mothma at random HQ). 8 tests.
2. ~~Alliance HQ randomization~~ — DONE. Random rim system selected as Rebel HQ, deterministic with RNG seed.
3. ~~Fleet distribution model~~ — DONE. 3-system model (Coruscant/Yavin/Rebel HQ) replaces proximity spread.
4. ~~Difficulty/galaxy size affecting seeds~~ — DONE. SeedOptions threaded through to apply_seeds.
5. ~~Energy/raw materials fields~~ — DONE. System struct extended with is_populated, total_energy, raw_materials.

**Fixed (M5-M8, 2026-03-25):**
6. ~~Support/popularity initialization~~ — DONE. Control buckets (core strong/weak/neutral), support 20-100 for controlled, ~41-59 for neutral.
7. ~~Procedural galaxy generation~~ — DONE. Core/rim bucket percentages from SDPRTB 7680/7681. Energy/raw materials from GNPRTB 7721-7727. Facility generation from SYFCCRTB/SYFCRMTB.
8. ~~Maintenance-budget common unit seeding~~ — DONE. Budget from SDPRTB 5168/5169/5170, bundle rolling from CMUNALTB/CMUNEMTB, low-support garrison pass.

**Seeding parity: COMPLETE.** Verified against TheArchitect2018 seed.js — rim exclusion from maintenance budget is parity-correct.

### Combat Review Fixes — DONE (2026-03-25)

All 10 review findings from Knesset Ma'at resolved:
- Shield absorption double-rounding → single i64 integer division
- `recall_fighters` grounded squad loss → `originally_launched` parameter
- Hardcoded `difficulty: 2` → `GameWorld.difficulty_index` threaded everywhere (space, ground, bombardment)
- TroopClassDef fallback warning → `eprintln!` on missing class
- Tactical fighter shield bypass → shield absorption added
- Tactical weapon type from hull_max → actual `CapitalShipClass` weapon stats
- Dead code (`ButtonSprite::disabled`, `atk_class_defense`) → removed
- Deprecated API (`Frame::none()`) → `Frame::NONE`

**Combat parity: 99%.** Only 74 informational GNPRTB parameters deferred.

## Knesset Resheph -- COMPLETE
*Final parity sprint — 10 delivered tasks across combat, AI, WASM, UI, and eval*

Plan: `docs/plans/2026-04-03-001-feat-knesset-resheph-final-sprint-plan.md`

Delivered:
- Combat: per-weapon-type damage in `phase_weapon_fire()` now multiplies turbolaser, ion cannon, and laser cannon arcs by their class-specific `*_attack_strength` scalars using i64 math throughout. Zero attack strength falls back to raw arc count. 3 tests added.
- Death Star docs: replaced "placeholder" / "not yet decompiled" wording with "best available approximation" after exhaustive GNPRTB search confirmed the current values are not parameterized.
- AI parity: `AiConfig` now includes faction-specific deployment budgets (`alliance_deploy_budget = 0.6`, `empire_deploy_budget = 0.8`) and fleet deployment scales `max_fronts` per faction for `FUN_00506ea0` parity.
- AI parity: dispatch validation advanced from 4/18 to 10/18 decoded gates via new `can_dispatch_to_system()` and `can_dispatch_fleet()` helpers, including strength comparison, loyalty gating, faction checks, and empty-fleet rejection.
- AI behavior: `evaluate_uprising_prevention()` now sends diplomats to low-support controlled systems before they flip.
- AI behavior: `evaluate_ds_escort()` now routes the nearest available fleet to the Death Star as an escort.
- WASM save/load: `rebellion-data` replaced browser stubs with a real `localStorage` backend using manual base64 encode/decode, plus save/load/list/delete support and separate metadata keys for fast `list_saves()`.
- WASM UI assets: `BmpCache` gained cfg-gated path rebasing for browser texture loads and a named DLL resource ID catalog grouped by source (`common`, `strategy`, `tactical`, `gokres`).
- WASM audio: `audio_base_path()` and `AUDIO_PREFIX` now normalize browser audio asset paths.
- Eval parity: `scripts/eval_parity.py` was rewritten into a golden-value oracle backed by `scripts/golden_values.json`, covering all mapped 111 GNPRTB bindings plus combat, economy, research, AI, movement, and victory constants with pass/fail reporting.

## Knesset Shamash-Bet Dabora 3 -- COMPLETE
*Story events + betrayal telemetry sprint — 8 R-tasks delivered*

Plan: `docs/plans/2026-04-08-001-feat-knesset-shamash-bet-story-events-cutscene-plan.md`

Delivered:
- R1: EVT_HAN_RESCUE (0x200) — telemetry twin of 0x383, fires same-tick via EventFired chaining. Silent (no actions). +19 lines.
- R2: EVT_JABBA_PRISONERS (0x231) — consolidator for palace captures. Three OR-branch variants (one per capture ID: 0x385 Leia, 0x387 Chewie, 0x399 Luke). Self-guard via EventNotFired ensures exactly one fires per game. +36 lines.
- R3: EVT_HAN_PERMANENT_FREEZE (0x39B) — 5-stage carbonite escape countdown (FAIL_1 through FAIL_5, ticks 145→205) using existing primitives only (EventFired + TickAtLeast + EventNotFired + CharacterIsCaptive). Terminal state when countdown completes without rescue or self-escape. 7 new constants in events.rs. +81 lines.
- R4: Final Battle heritage gate — single 0x220 EVT_FINAL_BATTLE (no event split). 0x396 action flips `heritage_known` via SetHeritageKnown. Render layer branches BMP in `event_screen.rs`: offset 24 (Vader vs Student Luke) when unknown, offset 32 (Emperor & Vader vs Knight Luke) when known. +heritage_known parameter on event_id_to_resource + show_event_screen.
- R5: Bounty Hunters real spawn — EVT_BOUNTY_ATTACK (0x212) now includes SpawnSpecialForce { at_character: han } + CharacterAssignedToFleet precondition (SF-#7). Test helper assign_han_to_fleet() added; 10 existing tests updated with fleet setup.
- R9: EVT_TRAITOR_REVEALED (0x361) — emitted BEFORE faction flip in integrator with original_faction payload.
- R10: EVT_SIDE_CHANGE (0x386) — emitted AFTER faction flip with DI-H2-compliant payload (char_name, not CharacterKey).
- R13: Stale "notifications" removed from section comment.
- 446 tests (347 core + 50 data + 46 render + 3 doc), zero warnings.

## Knesset Tammuz -- COMPLETE
*Full parity sprint — 7 phases across combat, telemetry, cutscenes, AI, and advisor*

Plan: `docs/plans/2026-04-12-001-feat-knesset-tammuz-full-parity-sprint-plan.md`

Delivered:
- Phase 1: SpecialForceSpawned arena wiring (A2 — creates SpecialForceUnit in world). A1/A3-A6 already shipped in prior sprints.
- Phase 2: Mission telemetry emissions — EVT_INFORMANT_INTEL (R6), EVT_SABOTEUR_DETECTED (R7), EVT_CHARACTER_HEALTH (R8), EVT_CHARACTER_KILLED from assassination (R11). Death Star kill path was already done.
- Phase 3: Emperor Palpatine 1.5× combat modifier (R12) — scales weapon fire damage when Emperor is co-located.
- Phase 4: Cutscene state machine — `GameMode::Cutscene { kind: CutsceneKind }` struct variant with Intro/Victory/Defeat/Story(u32) kinds. Post-cutscene transitions (Story→Galaxy, others→MainMenu). `GameMode::VictoryModal` with egui overlay. Save lock-out during cutscenes/victory. 8 story cutscene triggers (101–108) mapped to event IDs (0x221, 0x210, 0x212, 0x383, 0x220, 0x393, 0x396, 0x397).
- Phase 5: Advisor BIN v2 cascading decoder — parse rate 12%→99%. Four binary formats discovered (v1 explicit, v2 sequential range, v3 BMP-mapped range, v4 BMP single). ~42% of sequences now use direct BMP resource ID lookup instead of modulo. 11 new tests.
- Phase 6: AI parity closeout — dispatch validators 10/18→15/18, troop deployment with frontline awareness, Death Star multi-target selection + retreat logic, reconnaissance mission dispatch. 7 new tests.
- Phase 7: Code hygiene — 5 stale `controlling_faction` doc comments fixed, unused imports removed.
- 465 tests (355 core + 50 data + 57 render + 3 doc), zero failures.

## AI Parity Status (as of 2026-04-12)

Based on 3-agent review + 23-function GhidraMCP session (2026-03-23) + TheArchitect2018 wiki cross-reference + Knesset Tammuz D1-D5. See `agent_docs/systems/ai-parity-tracker.md` for full matrix.

### Core Pipeline (6 Functions)

| # | Original | Status | Gap |
|---|----------|--------|-----|
| 1 | FUN_00519d00 Galaxy evaluation | DONE | 7 buckets + control_ratio + aggression scaling |
| 2 | FUN_00537180 Primary deployment | AUGMENTED | Per-fleet scoring (4-factor). Original was per-system with capacity check only. **Our model is superior.** |
| 3 | FUN_005385f0 Secondary deployment | AUGMENTED | Original uses FUN_0052e970 (capacity check) + FUN_00506ea0 (faction evaluator). Our aggression model is more nuanced. |
| 4 | FUN_00502020 Garrison strength | DONE | Ships + troops + facilities |
| 5 | FUN_00508250 Dispatch validation | DONE | **15 of 18 sub-functions ported.** Remaining 3 (#2/#3/#4) are allocation budget tracking — our per-cycle caps serve the same anti-over-dispatch purpose (FAITHFUL). |
| 6 | FUN_00520580 Movement orders | DONE | **Decoded: 2-field struct setter** (not transit calc). 9 lines. |

### Remaining Gaps (P2)

5. **Defense facility construction** — `evaluate_production` builds capships, fighters, yards, troops, defenses but original priority order not confirmed.
7. **AI resource rebalancing** — FUN_00558660 not yet examined.

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

### Phase E: Combat Spread — COMPLETE
*Territory expansion via diplomacy. ~20 LOC.*

- [x] Diplomacy-to-control transition: popularity ≥ 0.6 with ≥ 0.1 lead flips ControlKind
- [x] Existing diplomacy targeting already covers neutral/enemy systems
- [x] More controlled territory → more attack targets → diverse battle locations

### Phase F: Autoresearch Execution — COMPLETE
*Run 001: 20 iterations × 3 seeds.*

- [x] Release binary built (1.5MB, LTO)
- [x] Run 20 iterations × 3 seeds × 5000 ticks
- [x] Baseline: 0.5449 → Final: 0.6483 (+19% improvement)
- [x] 2 accepted mutations: distance_scale 2→3, mfg_target 50→55
- [x] Results at `autoresearch/run-001/`, best config at `configs/autoresearch/best_game_config.json`
- [x] Finding: parameter space is narrow — most mutations cause degenerate games

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
