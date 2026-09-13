---
title: "Roadmap"
description: "Development milestones from Galaxy Viewer through Release packaging"
category: "agent-docs"
created: 2026-03-11
updated: 2026-09-12
tags: [roadmap, planning, milestones, parity]
---

# Roadmap

<!-- interface-parity-status:start -->
Required interface families: 43. Complete: 0. Partial: 9. Failing: 34. Blocked: 0. Strictly accepted cells: 0/564.
The current CMD-02 GID catalog has 38 scenarios (29 baseline cells and nine additional native filter variants) and 152 faction/viewport executions. Its strict original-evidence and cross-browser gate remains open.
<!-- interface-parity-status:end -->

The delivery notes before “Audit-Driven Parity” are historical implementation
records. They do not establish current parity or release acceptance; the active
audit-driven milestones are the source of truth.

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

## War Room: historical implementation tranche delivered
*Full strategy game without combat*

Committed: `6d47a10` (integration wiring — fog/fleet/panels/encyclopedia/audio into main loop)

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
*Release candidate — Knesset Elat (v0.6.0)*

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

### Remaining UI — HD Visual Polish (IN PROGRESS)

- [x] Reconstruct the browser's original 640x480 shuttle-cockpit composition, exact pointer regions, visible settings, direct faction start, and single-loop menu music. Astra-medium R3 passed 11/11 after finding two R2 defects ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-cockpit.md)).
- [x] Complete the cockpit endpoints and lifecycle: Credits, explicit M4 multiplayer status, gain/mute, return-to-menu, clean second-campaign reset, exact `MDATA.300` menu cue, and original `COMMON.DLL` button sounds. All 500 tests pass; Astra-medium completion evidence is recorded in [F-016C](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-completion.md). Native interactive visual acceptance and semantic canvas accessibility remain release-hardening work.
- [x] Expose all 14 authentic controls through clipped browser semantics while preserving the bitmap-only presentation, exact Rust action/SFX paths, and four-request startup. All 502 tests pass; Astra-medium verified 84/84 focus transitions and all browser gates in [F-016D](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-semantics.md). The then-open native visual gate closes in F-016E below.
- [x] Complete F-016E and P03: add a clearly documented, non-original 30×22 music-only cockpit control using Fable-reviewed Jiff Gorda/SWG Project Thorn cues; preserve SFX while muted; complete native visual acceptance; and pass Astra-medium 10/10 across four viewports, exact hit geometry, semantics, audio, navigation, and error gates. All 504 tests pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-music-toggle.md)).

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
| EData encyclopedia images (~330) | NOT STARTED |

### Knesset Kothar wa Khasis. Historical implementation (2026-04-07)
*U2 plus the first C1 attempt from Knesset Resheph. P44 supersedes the advisor result.*

Delivered:
- **U2 — Native video playback for decoded WebM cutscenes**: new `crates/rebellion-render/src/video_player.rs` with `VideoPlayer::open/advance/current_frame/is_finished/stop`. Runtime path is PNG frame sequences + WAV sidecars (no ffmpeg/libvpx/gstreamer runtime deps). `scripts/decode-cutscenes.sh` is a one-time local ffmpeg decode that produces `assets/references/cutscene-frames/<name>/frame-*.png` + `metadata.json` + `<name>.wav` (ignored by git). `GameMode::Cutscene` added to `rebellion-app/src/main.rs`, plays `000.webm` on startup before MainMenu and `201.webm`/`202.webm` on victory/defeat. ESC/SPACE skip handling. Graceful `VideoError::NotDecoded` path logs a skip message and proceeds when decoded assets are missing. WASM build unaffected via a cfg-gated no-op stub. New dep: `quad-snd` on rebellion-render (player owns its own audio context).
- **C1 first attempt**: a cascading BIN parser and sorted-frame fallback were added, but later interface review showed that this did not prove original motion or even transport the custom type-302 pixels. P44 replaces the visible idle path with the exact indexed anchors and cumulative type-302 deltas for both factions. `RE-ADV-02` still owns authored action, cadence, preemption, and voice mapping.

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

This tranche corrected the listed combat defects. Formula, tactical/automatic,
and full combat acceptance remain open under P23–P26.

## Knesset Resheph: historical implementation tranche delivered
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

## Knesset Shamash-Bet Dabora 3: historical implementation tranche delivered
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

## Knesset Tammuz: historical implementation tranche delivered
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
| 5 | FUN_00508250 Dispatch validation | DONE | **All 18 checks resolved.** 15 directly ported, #2/#3/#4 CLOSED — per-cycle caps replace C++ allocation budget tracking. |
| 6 | FUN_00520580 Movement orders | DONE | **Decoded: 2-field struct setter** (not transit calc). 9 lines. |

### Remaining Gaps (P2) — ALL CLOSED (2026-04-14)

5. **Defense facility construction** — CLOSED. FUN_00508660 is an entity-type dispatcher, not a priority function. Current order is FAITHFUL.
7. **AI resource rebalancing** — CLOSED. FUN_00558660 has no decompiled source; cross-ref describes minor random resource perturbation via RESRCTB (4 entries). Negligible gameplay impact.

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

## Audit-Driven Parity, Browser, and Multiplayer Roadmap

The completed milestones above describe implementation delivery, not final
release acceptance. The September 2026 [full-functionality audit](../docs/qa/2026-09-08-full-functionality-audit/)
owns behavior acceptance. Its sister [original-interface audit](../docs/qa/2026-09-10-interface-parity-audit/)
owns visible surface, bitmap, geometry, hotspot, navigation, animation, and audio
acceptance. Their JSON ledgers provide stable finding and feature IDs.

### M0: Truth and Critical Integration — 1 week

- [x] Wire Save, Load, Delete, and the main-menu save picker; Astra-medium r2 passed the 33/33 browser deletion/reload gate after exposing and fixing empty-slot Load activation ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-save-delete.md)).
- [ ] Surface browser corruption/quota errors and move production saves to asynchronous IndexedDB.
- [x] Correct the native HD root, prefetch `TROOPSD.DAT`, and package all 52 game-data entries plus 2,231 BMPs in deterministic `runtime.orpk` (`F-014A`; [Astra evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-runtime-pack.md)).
- [x] Map all 38 fighter/capital-ship records to authentic GOKRES fleet miniatures and pass Astra-medium bitmap/interaction proof (`4589d2e`; F-010B).
- [x] Add the F-011A versioned canonical save-state fingerprint, v9 integrity verification, v8 compatibility, and lossless browser metadata; Astra-medium r3 passed all 34 save/reload/load and bitmap assertions ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-state-fingerprints.md)).
- [x] Add the F-011B1 v10 continuation envelope for simulation RNG, second AI, repair, combat cooldowns, and active configuration; preserve real v9 artifacts and pass the Astra-medium 40/40 browser continuation gate ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-state-continuation.md)).
- [x] Resolve the original cockpit command-to-animation table from Ghidra and screenshot evidence, implement it without inferred sequential mappings, and pass the Astra-medium browser bitmap/hotspot/audio tranche ([reference](main-menu-parity.md); [evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-cockpit.md)).
- [x] Persist the original cockpit's faction, difficulty, galaxy size, and game type through save v11. All 495 workspace tests passed and Astra-medium R2 passed 18/18 browser state, bitmap, storage, network, and error gates. The selection path is closed; the source review later reopened victory resolution semantics ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-game-setup-propagation.md)).
- [x] Close P04 and the remaining P03 functional gates: Credits and Multiplayer destinations, gain/mute, return-to-menu audio, original menu SFX, and clean second-campaign reset. Native build/startup passed; F-016D/E later closed the then-open semantic and native visual gates ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-completion.md)).
- [x] Close F-016D browser semantics: one navigation landmark exposes all 14 original hotspots with selection state, cyclic keyboard traversal, bitmap focus, shared action/SFX routing, and no visible replacement controls. Astra-medium passed all nine acceptance gates ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-semantics.md)).
- [x] Close P03 final presentation: native visual acceptance and Astra-medium 10/10 browser acceptance pass for the documented music-only extension without changing the original 14-control contract ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-music-toggle.md)).
- [ ] Make a clean packaged browser artifact boot in CI.
- [x] Link README parity claims to current acceptance evidence.
- [x] Add F-011B2 replay format v1: strict JSON, typed `{tick, sequence, actor}` commands, checkpoint positions, configuration identity, and canonical per-file/aggregate hashes for all 51 simulation DATs ([contract](deterministic-replay.md); [evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-contract.md)).
- [x] Add the F-011B3 recorder/executor with fail-fast checkpoints, save-v11 continuation, stable state-affecting iteration, and a five-process native original-data golden ([contract](deterministic-replay.md); [evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-execution.md)).
- [x] Execute one exact F-011B4 artifact in native and WASM. All nine checkpoints, initial/final fingerprints, and artifact text match; Astra medium passed success, fail-closed, and normal four-request startup gates ([contract](deterministic-replay.md); [evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-wasm-equivalence.md)).

### M1: Simulation Correctness and Determinism — 2–3 weeks

- [x] Prevent in-transit fleet redispatch and travel-progress resets. Active
  orders are immutable until arrival or explicit cancellation, AI candidates
  exclude transit and same-pass reservations, and seed 42 reduced attack
  orders from 306,012 to 78,946 ([F-007A evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-fleet-redispatch.md)).
- [x] Model fleet position explicitly, merge compatible arrivals, and prevent
  production from attaching to in-transit fleets ([F-007B evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-fleet-position-consolidation.md)).
- [x] Wire ordinary player fleet dispatch through the validated authoritative
  departure path and pass Astra-medium bitmap acceptance for both factions
  ([F-007C evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-player-fleet-dispatch.md)).
- [x] Resolve every hostile task force at a system as one bounded engagement,
  persist fighter attrition, correct fighter launch and shield handling, and
  suppress unchanged five-tick stalemates. Seed 42 now reports two decisive
  engagements instead of 603 repeated combat events
  ([F-007D evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-system-combat-resolution.md)).
- [ ] Extend stable ordering and versioned random streams through five 5,000-tick campaign seeds.
- [x] Close the F-007E logistics checkpoint: require control for AI production,
  eliminate friendly cycling and production-fed transit fan-in, retain an HQ
  defender, preserve blockaded surface troops, load real `TROOPSD.DAT` class
  stats, and persist repair episodes in save v12. Five seeds finish with 0%
  transit, a 1.0 move/arrival ratio, at most 1.2× initial fleets, and only 0–5
  repair starts ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-ai-campaign-logistics.md)).
- [x] Close the verified portion of the F-007E conquest checkpoint: transport regiments within living
  ship capacity, preserve cargo through transit and consolidation, land after
  orbital control, continue unresolved surface battles, persist tactical
  casualties, and occupy systems. Deterministic character capture and
  occupation-based Imperial HQ victory are current behaviors, not accepted
  parity, and have been reopened
  ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-troop-transport-occupation.md)).
- [x] Add player troop selection to the bitmap fleet workflow with live
  capacity, authoritative embarkation, transit, and automatic landing
  ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-player-troop-dispatch.md)).
- [x] Establish a cited historical campaign baseline from the original manual,
  contemporary guides, and player testimony. It separates binding mechanics,
  observed original-AI behavior, reverse-engineered behavior, desired
  enhancements, and provisional engineering guards
  ([reference](../docs/reference/campaign-history/)).
- [x] Correct victory semantics: distinguish Coruscant capture from destruction
  and occupation of the mobile Alliance HQ; preserve Standard leader
  conjunctions after Death Star fire; make Death Star loss nonterminal; remove
  the uncited 200-tick grace period; and route contested, unopposed, manual, and
  tactical bombardment through the persisted HQ-survival flag
  ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-victory-contract.md)).
- [ ] Replace deterministic mass capture with validated capture/evasion
  behavior.
- [ ] Establish faction liveness and the full cited campaign loop, including
  production, diplomacy, recruitment, intelligence, research, blockade
  follow-up, uprisings, bombardment, HQ relocation, and principal missions.
  Do not require symmetric attack counts from an asymmetric game.
- [ ] Enable AI Death Star construction/fire/cleanup and repair parity oracles.
- [ ] Pass five reproducibly configured 5,000-tick seeds: transit ≤10%, orders
  ≤1.5× arrivals, and fleet arena ≤3× initial. Record 50–400 engagements, eight
  systems, busiest-system ≤40%, and 10% minority-faction attack share as
  diagnostics until original telemetry or calibrated playtests justify hard
  lower bounds.

Parameter autoresearch remains paused until this milestone closes; otherwise it
would tune around known simulation feedback defects.

### M2: One Authoritative Game Engine — 3 weeks

- [ ] Route the app and playtest through one simulation tick API and event sink.
- [ ] Make automatic, tactical, and ground combat share resumable core state and calculations. Tactical ground damage and occupation capture now persist.
- [ ] Construct the victory modal and unify win/loss transitions.
- [ ] Remove `AdvanceTicks` or make it execute the real simulation.
- [ ] Prove identical fingerprints for the same seed and command stream across app/playtest, native/WASM, and auto/tactical paths.

### M3: Exceptional Browser Runtime — 3 weeks

- [x] Replace thousands of serial startup requests with deterministic `runtime.orpk`; both factions now boot in four requests with lazy bitmap decode and zero loose asset requests (`F-014A`).
- [ ] Complete `UIP-T01` through `UIP-T05`: replace the synthetic strategy,
  object-window, report, and tactical surfaces with the original bitmap-driven
  compositions for both factions. No invented visible control may satisfy a
  parity cell.
- [x] Complete P44 / `RE-ADV-01` for the advisor idle-frame tranche: recover
  type-302 decoding, stage all 3,988 ALSPRITE/EMSPRITE frames, render both
  faction pairs in native/WASM, and fail closed with browser-visible diagnostics.
- [x] Complete P46A / the first `RE-STR-01` checkpoint: render both faction
  shells on one centered 640x480 canvas, crop the extra source row, apply the
  recovered galaxy apertures, and share their transform across map layers,
  hit tests, blockades, and advisors. The reference rail and the rest of
  `UIP-T01` remain open.
- [x] Complete P46B / the primary `RE-STR-02` checkpoint: remove the replacement
  text strip; render both factions' six exact normal and captured-press bitmap
  pairs; reproduce native capture, release, palette-key, and strict-edge input;
  prevent them from opening replacement destinations; and withhold the
  replacement message and status bars that covered them. A0 captures, the disabled path,
  original destination windows, speed and GID controls, and the rest of
  `UIP-T01` remain open.
- [x] Complete the first UIP-B01 strategic-navigation checkpoint: replace the
  invented sidebar with recovered sector and detailed-system shells, original
  planet and tab resources, double-click routing, modeless focus and close,
  plus both factions' 12-slot minimize/restore rail geometry. Nested tab
  compositions, commands, exact rail thumbnails, uncommon states, and the
  complete `CMD-03` and `CMD-04` acceptance matrices remain open.
- [x] Complete P46D / the detailed-system item checkpoint: populate all six
  core tabs with source-mapped GOKRES miniatures and displayed entity labels;
  restore selection, foreground focus, and the three-column scrolled viewport
  with STRATEGY `10365` through `10369`; preserve tab, item, and scroll state
  through rail restoration; and prevent stale hostile views from exposing
  opposing objects without current system intelligence. Nested compositions,
  drag and command paths, exact original intelligence semantics, uncommon
  states, exact rail thumbnails, and A0 acceptance remain open.
- [x] Complete P47A / the first `UIP-B02` checkpoint: draw the original bright
  STRATEGY 902 galaxy at canvas origin beneath the strategic layers, preserve
  source-aligned faction crops, use the same original bytes in native and
  packaged WASM, and pass muted Astra browser acceptance with no findings.
  P47B later proves this is the Display Off resource.
- [x] Complete P47B / the default Popular Support checkpoint: select STRATEGY
  903 for active GID, restore the exact faction GID control, compact legend,
  native marker families and support thresholds, remove synthetic overlays
  from the default parity view, preserve system activation, and pass four
  muted Astra faction/viewport campaigns with no product findings. The
  code-built menu, expanded legend, remaining modes and overlays, sector art,
  exact map input, and complete `CMD-02` matrix remain open.
- [x] Adopt the interface-parity acceleration sidecar: bundle the complete GID
  family, build one permanent fresh-process muted browser harness, use
  deterministic test-only fixtures, generate repeated status prose from
  canonical JSON, and reserve full workspace, WASM, and Astra verification for
  the family gate. The first test-only 38-scenario harness and code-built GID
  command tree are implemented. All 152 muted browser probes pass, but no
  original-executable captures are accepted. The original floating legend,
  non-support filter predicates, special markers, native comparison, and
  complete `CMD-02` acceptance remain open.
- [x] Restore the P49 Message Index rail resting artwork: map nine controls
  per faction from `FUN_00427270` and verify all eighteen STRATEGY BMPs
  pixel-for-pixel in the muted 640x480 browser harness. `CMD-08` remains open
  for illuminated/unread states, pointer routing, the original index window,
  categories, navigation, and its full acceptance matrix.
- [x] Restore the P50 GID root and submenu frame from STRATEGY 10100 through
  10107. Paint repeated edges in four batched meshes, remove the invented menu
  fade, and check native-size root pixels against the source BMPs in both
  factions. Menu interior, typography, exact geometry, remaining GID rules,
  original-executable captures, and `CMD-02` acceptance remain open.
- [x] Remove the P51 unproven GID row-hover wash, retain submenu routing, and
  verify every visible source-frame pixel under the detailed system window.
  Original hover/focus art and full window-stacking acceptance remain open.
- [x] Complete the P52 first tactical-shell checkpoint: render TACTICAL 1000
  on the 640×480 canvas, replace active-combat replacement panels, wire bitmap
  pause, red Alliance and green Imperial highlight/dim states, and zoom, and
  verify both factions and viewports
  in fresh muted browser sessions. Procedural battle contents and all 106
  `TAC-01` through `TAC-07` cells remain open.
- [x] Complete the P53 tactical input-state checkpoint: apply the original
  palette-key hit mask and outer-edge exclusion to each working tactical
  control, render zoom pressed resources 1045/1047 while held, and verify
  inert matte pixels plus exact zoom round-trip restoration in both factions
  and viewports. Drag/repeat semantics and all 106 tactical cells remain open.
- [x] Complete P54 reproducible tactical 3D staging: preserve all 87 type-301
  DirectX meshes and 397 type-303 texture or palette resources from an owned
  `TACTICAL.DLL` in a validated content-addressed store. Keep the proprietary
  output ignored. Binary X and type-303 decoding begin in P55.
- [ ] Continue UIP-B06 with source-mapped selected-unit, task-force, fighter,
  navigation, camera, maneuver, tactic, mission, result, Death Star, and audio
  states. Stage and map original type-301/type-303 battle graphics before
  replacing procedural ships, fighters, planets, and effects.
- [ ] Execute `RE-ADV-02` for authored SPT/BIN/FDT action, cadence, preemption,
  and voice mappings; extend the decoder/pack to briefings and every other
  required resource kind; then consume the proven shell/control/window mappings.
- [ ] Add Brotli compression plus bounded raw-byte and decoded-texture caches to the verified runtime-pack foundation.
- [ ] Complete P45 faithful-HD modernization. P45A is complete: original parity
  is separate from opt-in HD, manifests are source-bound, and deterministic
  indexed scaling plus fail-closed fallback are verified. P45B-P45E own complete-family model evaluation, protected masks,
  optional browser packs, and cross-browser acceptance.
- [ ] Enable high DPI; use one egui pass; cache sector geometry.
- [ ] Move saves from synchronous base64 `localStorage` to compressed asynchronous IndexedDB.
- [ ] Add authored advisor voice and preemption after user gesture; advisor idle
  assets are now staged from a contributor-owned installation.
- [ ] Exercise real mouse/keyboard input and bitmap screenshots in Chrome, Firefox, and Safari.
- [ ] Meet budgets: ≤3 s cold start at 50 Mbps/30 ms, ≤4 pre-menu requests, ≤8 ms frame, ≤12 ms WASM tick at 1,000 fleets, ≤5 MB optimized WASM, and ≤256 MB combined memory after 10 minutes.

### M4: Authoritative Multiplayer — 6–8 weeks after M1–M3

- [ ] Add validated `{player, tick, sequence}` commands and route single-player through the same boundary.
- [ ] Add `rebellion-net` and `rebellion-server` responsibilities for authoritative simulation and fog-filtered faction deltas/snapshots.
- [ ] Start with in-process test transport and secure WebSockets; never expose the hidden full `GameWorld` to clients.
- [ ] Add prediction/reconciliation, reconnect snapshots plus command log, persistence, authentication, rate limits, TLS, and observability.
- [ ] Pass two clients for 5,000 ticks with hashes matching every 250 ticks; no 200 ms RTT input stalls; ≤1 reconcile per 100 commands; reconnect ≤60 s; reject 100% of illegal commands.
- [ ] Consider trusted lockstep/WebRTC only after deterministic replay is proven.

### M5: Continuous Release Proof — parallel from M0

- [ ] Restore a reviewed CI provider; GitHub Actions workflow definitions are intentionally local/untracked as of 2026-09-08, so current gates are manual.
- [ ] Enforce format, strict clippy, native, packaged-WASM, and clean-package boot gates.
  Local `make all` now covers tests, formatting, strict all-target Clippy, and
  native build; packaged browser and CI enforcement remain open.
- [ ] Add at least 20 app/browser integration cases and promote long campaigns when stable.
- [ ] Generate the complete bitmap resource ledger from the entity map; fail on unexplained misses. Fleet-list miniatures are verified in F-010B.
- [x] Establish 564 stable required interface baseline cells, link all 43
  required surface families to 27 bounded retrieval or removal packages, and
  explicitly cover 106 space-battle cells across `TAC-01` through `TAC-07`.
- [ ] Close every required surface-state cell in the original-interface audit
  with authoritative captures, exact-resource provenance, screenshot and hotspot
  comparison, complete navigation edges, and zero unknown or invented pixels.
- [ ] Retain screenshot, console, network, performance, data-hash, and save-schema evidence. F-010B fleet and F-014A runtime-pack evidence are retained; the full ledger remains open.
- [x] Revalidate current project documentation from `main` through GitHub Pages.
  Run `34505121163` built and deployed commit `bcc224f`; the landing page,
  documentation and campaign indexes, contract, source ledger, audit, and
  preserved manual all return HTTP 200.
- [ ] Close every supported P00–P40 functionality pass and every required
  interface surface-state cell from release artifacts before claiming 100%
  functionality.

### v1.0: Password-Protected Cloudflare Release

- [ ] Deploy the self-contained browser artifact to Cloudflare Pages with Pages Functions middleware.
- [ ] Keep `SITE_PASSWORD` and `SESSION_SECRET` in encrypted Wrangler/Pages secrets; commit no credentials or reusable auth token.
- [ ] Sign an expiring `HttpOnly; Secure; SameSite=Lax` session cookie and provide logout; reject missing, expired, or tampered sessions.
- [ ] Gate HTML, WASM, DAT, bitmap/HD packs, save endpoints, WebSockets, and multiplayer APIs—not only the landing page.
- [ ] Add CSP and security headers plus immutable caching for hashed public assets; never publicly cache authenticated HTML or auth responses.
- [ ] Maintain preview and production environments, commit-linked deploys, smoke checks, retained artifacts, and documented rollback.
- [ ] Have `/codex-orchestrator` run Astra at medium effort against the deployed URL for authentication, single-player, multiplayer, input, performance, and bitmap screenshot acceptance in Chrome, Firefox, and Safari.

v1.0 exits only when the protected Cloudflare deployment passes P39, the
GitHub Pages documentation passes P40, and every supported earlier pass. This follows the local Pages `functions/_middleware`
pattern while replacing hardcoded passwords/cookie values with encrypted
secrets and signed sessions.

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

### Addon 3: Tactical Combat Convergence (Future)
- Bring the existing 2D tactical view onto the authoritative combat path.
- Complete ship placement, formation selection, manual targeting, and parity
  with automatic result application.
- 3D model sprite sheets from Hunyuan3D Pro pipeline

---

## Known Technical Debt

- dat-dumper in `tools/` is also a library dep of rebellion-data -- works but unconventional
- Browser data now loads from deterministic `runtime.orpk`; compression, HD
  entries, and bounded caches remain open.
- Save v3 files rejected (bincode layout incompatible) -- no migration possible without SaveStateV3
- `enabled_sorted()` silently returns empty on dependency resolution errors
- `ModConfig::load()` silently drops corrupted config.toml
- No additive entity creation in mods -- patches only modify existing entities (until Addon 1 Phase 5)
- 27 LOC dead code identified by simplicity reviewer (2026-03-21)
- 5 stale doc comments referencing old `controlling_faction` field
