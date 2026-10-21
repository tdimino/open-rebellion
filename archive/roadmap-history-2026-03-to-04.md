# Roadmap History, March and April 2026

Delivery notes moved verbatim from `agent_docs/roadmap.md` on 2026-10-19; they record what was built then, establish no current parity or acceptance, and their parity figures are retired.

## Galaxy Viewer: historical implementation tranche delivered
*Interactive galaxy data viewer -- native + WASM*

Commits: `4e258bb` (scaffold) > `5905497` (dat-dumper + types) > `fc968ec` (renderer + WASM) > `22b61a6` (review fixes)

Delivered:
- Cargo workspace with 5 crates
- dat-dumper: 22/22 documented DAT files parsed, 3 structural patterns, round-trip byte validated
- rebellion-core: DatId newtype, 11 slotmap keys, dat/ enums, world/ structs, GameWorld root
- rebellion-data: DAT to GameWorld loader (sectors > systems > ships > fighters > characters)
- rebellion-render: macroquad galaxy map with pan/zoom/click-to-select, egui info panel + status bar
- WASM build: 2.9MB browser artifact via scripts/build-wasm.sh

## Living Galaxy: historical implementation tranche delivered
*Automated galaxy simulation—shipped 2026-03-13 by Knesset Athirat*

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

## War Room: historical implementation tranche delivered
*Full strategy game without combat*

Committed: `6d47a10` (integration wiring—fog/fleet/panels/encyclopedia/audio into main loop)

Delivered:
- Player faction selection + 5 UI panels (officers, fleets, manufacturing, missions)
- Fleet movement on galaxy map (diamond icons, dashed routes, ETA labels)
- Fog of war with monotonic reveal + advance intel at 50% transit
- Encyclopedia viewer with 4 tabs + BMP texture cache from EData/
- Audio system via quad-snd (CoreAudio/ALSA/WebAudio)
- Main loop integration: all draw calls + event hooks wired

## War Machine: historical implementation tranche delivered
*Major strategy systems implemented*

1. Space combat: auto-resolve first, then tactical 2D view
2. Ground combat: regiment engagement, orbital bombardment
3. Blockade mechanics, Death Star construction/destruction
4. Asymmetric victory conditions (control Coruscant; destroy and occupy the mobile Alliance HQ; preserve Standard leader requirements)
5. 3D tactical models: nano-banana-pro concept art → Hunyuan3D Pro/Meshy → Blender sprite sheets. Pipeline proven in World War Watcher (14 models). See `agent_docs/assets.md`.

Ghidra RE of REBEXE.EXE has extensive simulation coverage: the combat call
chain, bombardment formula, 111 GNPRTB parameters, and the C++ class hierarchy
are mapped. Interface RE remains active because 2,790 of 4,934 canonical text
exports are empty. Use the saved project and the
[interface RE ledger](../docs/qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.md)
for UI work.

## Full Parity: historical implementation tranche delivered
*Named delivery tranche shipped 2026-03-16 by Knesset Shapash; final parity remains open*

Delivered (v0.5.0):
- 4 scripted story chains (Luke Dagobah, Final Battle, Bounty Hunters, Jabba's Palace)
- Han Solo speed bonus, betrayal mechanics, decoy system, escape system
- 10 new Character fields promoted from DAT, mission state flags
- 6 new EventConditions + 6 new EventActions, 15 RE event ID constants
- All 9 mission types with world-mutating effects (sabotage, assassination, rescue, abduction, incite uprising)

## Mod Workshop: historical implementation tranche delivered
*Release candidate—Knesset Elat (v0.6.0)*

Delivered:
- Sensor-radius fog (detection field on CapitalShipClass)
- Captivity state tracking (is_captive, captured_by, capture_tick)
- Research pure advance() contract (caller applies level-ups)
- Save format v4 with mod metadata + FNV-1a hash, migration framework
- ModRuntime: discovery, enable/disable, structured errors, config persistence
- Mod Manager egui panel (discover, toggle, reload)
- ESCAPETB per-tick escape check wired into main loop

## Release: historical packaging tranche delivered
*Release packaging work by Knesset Hokhmah (v0.7.0); release acceptance remains open*

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

## AI Overhaul: historical implementation tranche delivered
*v0.13.0—AI balance, observability, autoresearch*

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

## UI Rebuild—IN PROGRESS
*v0.16.0—Player-facing UI rebuild following `docs/plans/archive/2026-03-22-001-feat-complete-game-ui-rebuild-plan.md`*

### Completed (2026-03-23/24)
- Phase 1.1/1.2 prototype: custom text main menu and separate setup screen. These are now superseded by the authentic cockpit gate below.
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

### Remaining UI—HD Visual Polish (IN PROGRESS)

- [x] Reconstruct the browser's original 640x480 shuttle-cockpit composition, exact pointer regions, visible settings, direct faction start, and single-loop menu music. Browser review R3 passed 11/11 after finding two R2 defects ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-cockpit.md)).
- [x] Complete the cockpit endpoints and lifecycle: Credits, explicit M4 multiplayer status, gain/mute, return-to-menu, clean second-campaign reset, exact `MDATA.300` menu cue, and original `COMMON.DLL` button sounds. All 500 tests pass; completion evidence is recorded in [F-016C](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-completion.md). Native interactive visual acceptance and semantic canvas accessibility remain release-hardening work.
- [x] Expose all 14 authentic controls through clipped browser semantics while preserving the bitmap-only presentation, exact Rust action/SFX paths, and four-request startup. All 502 tests pass; independent review verified 84/84 focus transitions and all browser gates in [F-016D](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-semantics.md). The then-open native visual gate closes in F-016E below.
- [x] Complete F-016E and P03: add a clearly documented, non-original 30×22 music-only cockpit control using Fable-reviewed Jiff Gorda/SWG Project Thorn cues; preserve SFX while muted; complete native visual acceptance; and pass 10/10 browser checks across four viewports, exact hit geometry, semantics, audio, navigation, and error gates. All 504 tests pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-music-toggle.md)).

The April 2026 model shootout remains comparison evidence, but its universal
UltraSharp conclusion is superseded by the
[faithful-HD plan](../docs/plans/2026-09-10-faithful-hd-pipeline/). Original
pixels are now the default parity profile. Optional HD uses reviewed manifests
and family-specific routing.

```bash
uv run scripts/faithful_hd_pipeline.py generate # Deterministic candidate generation
uv run scripts/faithful_hd_pipeline.py verify   # Hash and reconstruction gates
python3 scripts/model-shootout.py --html-only    # Regenerate comparison at data/hd/shootout/comparison.html
```

| Item | Status |
|------|--------|
| Historical model shootout (8 models × 20 BMPs) | COMPLETE; preference evidence only |
| Explicit profiles and manifest foundation | COMPLETE under P45A; [evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-faithful-hd-foundation.md) |
| Complete-family learned-model comparison | PENDING under P45B |
| EData encyclopedia images (187 owned originals) | TRANSPORT AND EXACT DISPLAY COMPLETE; AUTHENTIC WINDOWS OPEN |

### Knesset Kothar wa Khasis. Historical implementation (2026-04-07)
*U2 plus the first C1 attempt from Knesset Resheph. P44 supersedes the advisor result.*

Delivered:
- **U2—Native video playback for decoded WebM cutscenes**: new `crates/rebellion-render/src/video_player.rs` with `VideoPlayer::open/advance/current_frame/is_finished/stop`. Runtime path is PNG frame sequences + WAV sidecars (no ffmpeg/libvpx/gstreamer runtime deps). `scripts/decode-cutscenes.sh` is a one-time local ffmpeg decode that produces `assets/references/cutscene-frames/<name>/frame-*.png` + `metadata.json` + `<name>.wav` (ignored by git). `GameMode::Cutscene` added to `rebellion-app/src/main.rs`, plays `000.webm` on startup before MainMenu and `201.webm`/`202.webm` on victory/defeat. ESC/SPACE skip handling. Graceful `VideoError::NotDecoded` path logs a skip message and proceeds when decoded assets are missing. WASM build unaffected via a cfg-gated no-op stub. New dep: `quad-snd` on rebellion-render (player owns its own audio context).
- **C1 first attempt**: a cascading BIN parser and sorted-frame fallback were added, but later interface review showed that this did not prove original motion or even transport the custom type-302 pixels. P44 replaces the visible idle path with the exact indexed anchors and cumulative type-302 deltas for both factions. `RE-ADV-02` still owns authored action, cadence, preemption, and voice mapping.

Known gaps left for future work:
- Only ~24% of advisor BINs (183 of ~752 per faction) match the simple `u16 count + u16 ids` shape. The other ~76% declare inconsistent lengths and are silently skipped—the game almost certainly uses one or more additional header variants (timing bytes, loop counts, control flags) that have not been decoded. The 24% that do parse now drive authored sequence length/variation; the rest fall through to the legacy path.
- Victory/defeat cutscenes return directly to `MainMenu` rather than chaining through the existing egui victory modal.
- Cutscene audio plays at fixed volume through the player's own quad-snd context rather than through `AudioVolumeState`.
- Task #132 report in `docs/plans/archive/2026-04-03-001-feat-knesset-resheph-final-sprint-plan.md` tracks these.

### Game Seeding Parity—CRITICAL GAP (~12% parity)

*Discovered 2026-03-24 via audit against TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals/initial_game_seeding_logic*

ExecPlan: `docs/plans/completed/2026-03-24-003-game-seeding-parity-execplan.md`

**What works**: 9 seed table DAT parsing + dispatch, Coruscant fleet/garrison/facilities, Yavin garrison, army table distribution.

**Fixed (M1-M4, 2026-03-24):**
1. ~~Character stat rolling + placement~~—DONE. SkillPair rolled to concrete stats, named characters placed (Luke/Leia/Han at Yavin, Vader/Palpatine at Coruscant, Mon Mothma at random HQ). 8 tests.
2. ~~Alliance HQ randomization~~—DONE. Random rim system selected as Rebel HQ, deterministic with RNG seed.
3. ~~Fleet distribution model~~—DONE. 3-system model (Coruscant/Yavin/Rebel HQ) replaces proximity spread.
4. ~~Difficulty/galaxy size affecting seeds~~—DONE. SeedOptions threaded through to apply_seeds.
5. ~~Energy/raw materials fields~~—DONE. System struct extended with is_populated, total_energy, raw_materials.

**Fixed (M5-M8, 2026-03-25):**
6. ~~Support/popularity initialization~~—DONE. Control buckets (core strong/weak/neutral), support 20-100 for controlled, ~41-59 for neutral.
7. ~~Procedural galaxy generation~~—DONE. Core/rim bucket percentages from SDPRTB 7680/7681. Energy/raw materials from GNPRTB 7721-7727. Facility generation from SYFCCRTB/SYFCRMTB.
8. ~~Maintenance-budget common unit seeding~~—DONE. Budget from SDPRTB 5168/5169/5170, bundle rolling from CMUNALTB/CMUNEMTB, low-support garrison pass.

**Seeding parity: COMPLETE.** Verified against TheArchitect2018 seed.js—rim exclusion from maintenance budget is parity-correct.

### Combat Review Fixes—DONE (2026-03-25)

All 10 review findings from Knesset Ma'at resolved:
- Shield absorption double-rounding → single i64 integer division
- `recall_fighters` grounded squad loss → `originally_launched` parameter
- Hardcoded `difficulty: 2` → `GameWorld.difficulty_index` threaded everywhere (space, ground, bombardment)
- TroopClassDef fallback warning → `eprintln!` on missing class
- Tactical fighter shield bypass → shield absorption added
- Tactical weapon type from hull_max → actual `CapitalShipClass` weapon stats
- Dead code (`ButtonSprite::disabled`, `atk_class_defense`) → removed
- Deprecated API (`Frame::none()`) → `Frame::NONE`

This tranche corrected the listed combat defects. Formula, tactical/automatic,
and full combat acceptance remain open under P23–P26.

## Knesset Resheph: historical implementation tranche delivered
*Final parity sprint—10 delivered tasks across combat, AI, WASM, UI, and eval*

Plan: `docs/plans/archive/2026-04-03-001-feat-knesset-resheph-final-sprint-plan.md`

Delivered:
- Combat: per-weapon-type damage in `phase_weapon_fire()` now multiplies turbolaser, ion cannon, and laser cannon arcs by their class-specific `*_attack_strength` scalars using i64 math throughout. Zero attack strength falls back to raw arc count. 3 tests added.
- Death Star docs: replaced "placeholder" / "not yet decompiled" wording with "best available approximation" after exhaustive GNPRTB search confirmed the current values are not parameterized.
- AI parity: `FUN_00506ea0` reads per-faction deployment budgets at `+0xc4` (Alliance) and `+0xc8` (Empire); their values are not recovered, and `AiConfig` carries no budget.
- AI parity: dispatch validation advanced from 4/18 to 10/18 decoded gates via new `can_dispatch_to_system()` and `can_dispatch_fleet()` helpers, including strength comparison, loyalty gating, faction checks, and empty-fleet rejection.
- AI behavior: `evaluate_uprising_prevention()` now sends diplomats to low-support controlled systems before they flip.
- AI behavior: `evaluate_ds_escort()` now routes the nearest available fleet to the Death Star as an escort.
- WASM save/load: `rebellion-data` replaced browser stubs with a real `localStorage` backend using manual base64 encode/decode, plus save/load/list/delete support and separate metadata keys for fast `list_saves()`.
- WASM UI assets: `BmpCache` gained cfg-gated path rebasing for browser texture loads and a named DLL resource ID catalog grouped by source (`common`, `strategy`, `tactical`, `gokres`).
- WASM audio: `audio_base_path()` and `AUDIO_PREFIX` now normalize browser audio asset paths.
- Eval parity: `scripts/eval_parity.py` was rewritten into a golden-value oracle backed by `scripts/golden_values.json`, covering all mapped 111 GNPRTB bindings plus combat, economy, research, AI, movement, and victory constants with pass/fail reporting.

## Knesset Shamash-Bet Dabora 3: historical implementation tranche delivered
*Story events + betrayal telemetry sprint—8 R-tasks delivered*

Plan: `docs/plans/archive/2026-04-08-001-feat-knesset-shamash-bet-story-events-cutscene-plan.md`

Delivered:
- R1: EVT_HAN_RESCUE (0x200)—telemetry twin of 0x383, fires same-tick via EventFired chaining. Silent (no actions). +19 lines.
- R2: EVT_JABBA_PRISONERS (0x231)—consolidator for palace captures. Three OR-branch variants (one per capture ID: 0x385 Leia, 0x387 Chewie, 0x399 Luke). Self-guard via EventNotFired ensures exactly one fires per game. +36 lines.
- R3: EVT_HAN_PERMANENT_FREEZE (0x39B)—5-stage carbonite escape countdown (FAIL_1 through FAIL_5, ticks 145→205) using existing primitives only (EventFired + TickAtLeast + EventNotFired + CharacterIsCaptive). Terminal state when countdown completes without rescue or self-escape. 7 new constants in events.rs. +81 lines.
- R4: Final Battle heritage gate—single 0x220 EVT_FINAL_BATTLE (no event split). 0x396 action flips `heritage_known` via SetHeritageKnown. Nothing reads the flag; the story chains' port is `or-port-story-research-c80`.
- R5: Bounty Hunters real spawn—EVT_BOUNTY_ATTACK (0x212) now includes SpawnSpecialForce { at_character: han } + CharacterAssignedToFleet precondition (SF-#7). Test helper assign_han_to_fleet() added; 10 existing tests updated with fleet setup.
- R9: EVT_TRAITOR_REVEALED (0x361)—emitted BEFORE faction flip in integrator with original_faction payload.
- R10: EVT_SIDE_CHANGE (0x386)—emitted AFTER faction flip with DI-H2-compliant payload (char_name, not CharacterKey).
- R13: Stale "notifications" removed from section comment.
- 446 tests (347 core + 50 data + 46 render + 3 doc), zero warnings.

## Knesset Tammuz: historical implementation tranche delivered
*Full parity sprint—7 phases across combat, telemetry, cutscenes, AI, and advisor*

Plan: `docs/plans/completed/2026-04-12-001-feat-knesset-tammuz-full-parity-sprint-plan.md`

Delivered:
- Phase 1: SpecialForceSpawned arena wiring (A2—creates SpecialForceUnit in world). A1/A3-A6 already shipped in prior sprints.
- Phase 2: Mission telemetry emissions—EVT_INFORMANT_INTEL (R6), EVT_SABOTEUR_DETECTED (R7), EVT_CHARACTER_HEALTH (R8), EVT_CHARACTER_KILLED from assassination (R11). Death Star kill path was already done.
- Phase 3: Emperor Palpatine 1.5× combat modifier (R12)—scales weapon fire damage when Emperor is co-located.
- Phase 4: Cutscene state machine—`GameMode::Cutscene { kind: CutsceneKind }` struct variant with Intro/Victory/Defeat/Story(u32) kinds. Post-cutscene transitions (Story→Galaxy, others→MainMenu). `GameMode::VictoryModal` with egui overlay. Save lock-out during cutscenes/victory. 8 story cutscene triggers (101–108) mapped to event IDs (0x221, 0x210, 0x212, 0x383, 0x220, 0x393, 0x396, 0x397).
- Phase 5: Advisor BIN v2 cascading decoder—parse rate 12%→99%. Four binary formats discovered (v1 explicit, v2 sequential range, v3 BMP-mapped range, v4 BMP single). ~42% of sequences now use direct BMP resource ID lookup instead of modulo. 11 new tests.
- Phase 6: AI parity closeout—dispatch validators 10/18→15/18, troop deployment with frontline awareness, Death Star multi-target selection + retreat logic, reconnaissance mission dispatch. 7 new tests.
- Phase 7: Code hygiene—5 stale `controlling_faction` doc comments fixed, unused imports removed.
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
| 5 | FUN_00508250 Dispatch validation | DONE | **All 18 checks resolved.** 15 directly ported, #2/#3/#4 CLOSED—per-cycle caps replace C++ allocation budget tracking. |
| 6 | FUN_00520580 Movement orders | DONE | **Decoded: 2-field struct setter** (not transit calc). 9 lines. |

### Remaining Gaps (P2)—ALL CLOSED (2026-04-14)

5. **Defense facility construction**—CLOSED. FUN_00508660 is an entity-type dispatcher, not a priority function. Current order is FAITHFUL.
7. **AI resource rebalancing**—CLOSED. FUN_00559ee0 looks up RESRCTB (table 0x2b) with a random draw; the resource perturbation is minor. Negligible gameplay impact.

---

## Roadmap: AI Parity Completion (v0.14.0)

### Phase A: Config Externalization—COMPLETE
*Unblocks autoresearch. ~160 LOC.*

- [x] Created `crates/rebellion-core/src/tuning.rs`—`GameConfig` with 4 sub-configs (`AiConfig`, `MovementConfig`, `ProductionConfig`, `ScoringConfig`), 16 tunable parameters
- [x] Externalized all AI/movement/production constants—threaded `&GameConfig` through `AISystem::advance()`, `score_attack_target()`, `fleet_transit_ticks_with_config()`
- [x] Added `--config <path>` to playtest binary (JSON with `#[serde(default)]`)
- [x] Default config at `configs/autoresearch/default.json`
- [x] Parity/augmentation split documented per-field in tuning.rs
- [x] Updated `autoresearch_loop.py` to write and pass config files

### Phase B: Research Dispatch (P0 Gap #1)—COMPLETE
*AI tech tree progression. ~60 LOC.*

- [x] Added `evaluate_research()` to AISystem—assigns idle characters to Ship/Troop/Facility trees
- [x] Characters matched by primary skill (ship_design ≥ 30, troop_training ≥ 30, facility_design ≥ 30)
- [x] `AIAction::DispatchResearch` variant + telemetry event
- [x] Wired into simulation tick (both factions) and interactive app

### Phase C: Ratio-Based Galaxy Evaluation (P0 Gap #2)—COMPLETE
*Aggression scaling. ~30 LOC.*

- [x] Ported FUN_0053e190: `control_ratio` and `aggression` fields on GalaxyState
- [x] Galaxy control % → aggression: 10% → 0.18, 50% → 0.5, 90% → 0.82
- [x] `max_fronts` scaled by aggression (weak faction opens fewer attack fronts)
- [x] Pass 2 behavior: high aggression → pile onto attacks; low → reinforce

### Phase D: Proportional Redistribution (P0 Gap #3)—COMPLETE
*Replace Pass 2. ~25 LOC.*

- [x] Defensive reinforcement distributes across ALL undefended systems (not just first)
- [x] Round-robin by fewest incoming reinforcements (even distribution)
- [x] Falls back to all controlled systems when no undefended remain

### Phase E: Combat Spread—COMPLETE
*Territory expansion via diplomacy. ~20 LOC.*

- [x] Diplomacy-to-control transition: popularity ≥ 0.6 with ≥ 0.1 lead flips ControlKind
- [x] Existing diplomacy targeting already covers neutral/enemy systems
- [x] More controlled territory → more attack targets → diverse battle locations

### Phase F: Autoresearch Execution—COMPLETE
*Run 001: 20 iterations × 3 seeds.*

- [x] Release binary built (1.5MB, LTO)
- [x] Run 20 iterations × 3 seeds × 5000 ticks
- [x] Baseline: 0.5449 → Final: 0.6483 (+19% improvement)
- [x] 2 accepted mutations: distance_scale 2→3, mfg_target 50→55
- [x] Results at `autoresearch/run-001/`, best config at `configs/autoresearch/best_game_config.json`
- [x] Finding: parameter space is narrow—most mutations cause degenerate games
