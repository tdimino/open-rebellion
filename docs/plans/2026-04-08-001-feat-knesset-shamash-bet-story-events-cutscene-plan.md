---
title: "Knesset Shamash-Bet — Story Events Sweep + Advisor/Cutscene Polish"
type: feat
status: active
date: 2026-04-08
project: open-rebellion
tags: [knesset, swarm, story, events, advisor, cutscene, parity, sprint]
---

# Knesset Shamash-Bet — Story Events Sweep + Advisor/Cutscene Polish

*Named for **Shamash** (שמש), the Akkadian/Ugaritic sun god who illuminates what the night hid (the missing event handlers), paired with **Bet** (the second house) — a sprint that completes both Knesset Shamash's lapsed Phase 3b notification work and the four Kothar wa Khasis closing gaps from Knesset Resheph.*

## Enhancement Summary

**Deepened on:** 2026-04-08 by `/compound-engineering:deepen-plan`
**Sections enhanced:** 7 (Problem Statement, Proposed Solution, Technical Approach, Implementation Phases, Acceptance Criteria, Risk Analysis, Documentation Plan)
**Review agents used:** architecture-strategist, performance-oracle, data-integrity-guardian, silent-failure-hunter, code-simplicity-reviewer, quad-snd API verifier (6 parallel)
**Findings synthesized:** 41 across all reviewers (4 CRITICAL, 14 HIGH, 17 MEDIUM, 6 LOW)

### Key Improvements Folded In

1. **Save format bumps to v8.** Independently confirmed by both architecture and data-integrity reviewers via citation to the existing comment at `crates/rebellion-data/src/save.rs:367` ("bincode is positional — `#[serde(default)]` is inoperative"). Dabora 1 owns the `v7→v8` migration as its day-zero task. The bump bundles `economy: EconomyState` into `SaveState` (closes a pre-existing bug where post-load economy state silently re-fires incidents).
2. **Phase 1 LOC cut from ~600 to ~150** (75% reduction in new types). Dropped: `frozen_permanently` field (redundant with `is_captive` + `EventFired`), `CharacterFlag` enum, `EventCondition::CharacterFlag`, `EventAction::SetCharacterFlag`, `GameEffect::CharacterFlagSet`, `System::incident_flags`, `System::last_support_tier`, `ManufacturingState::was_empty`, `IncidentKind` enum, `GameEffect::SystemIncidentFired`, `BOUNTY_HUNTERS_DAT_ID` sentinel, `CutsceneKind` enum, `EVT_FINAL_BATTLE_KNOWN = 0x222` (split collapsed). Kept: `Character::heritage_known` (only genuinely new state), `GameEffect::SpecialForceSpawned`, `EventAction::SpawnSpecialForce`.
3. **Final Battle BMP variant moves to render layer.** A single triple-collapse fix kills three findings at once: arch-#9 same-tick cascade risk, perf-#3 `CharactersCoLocated` double-walk, and the previously-required predicate cache (perf-#S12). The render layer's `event_id_to_resource()` reads `Character::heritage_known` to pick the BMP. Audit-driven: existing `0x396 "Final Battle Imminent"` already narrates "father and son" so its action sets `heritage_known = true` BEFORE the co-location event fires.
4. **`apply_event_actions` consolidation via `pub` visibility, not extraction.** Combined fix from arch-CRITICAL-#2 and simplicity-M1: pub the integrator function, delete `main.rs:3271-3409`, route `EventAction::DisplayMessage` through a new `GameEffect::StoryMessageDisplayed` so main.rs drains UI messages from the effect stream (preserves the rebellion-data → rebellion-render layering). Saves ~80 LOC versus the original "extract a shared helper" approach.
5. **`is_story_event` filter replaced with `SystemTag` field on `GameEvent`.** Misrouting becomes a type error at the `define()` site instead of a brittle ID pattern match. Kills the #S7 list inconsistency entirely.
6. **6 daborot collapsed to 5.** Dabora 2 (Economy/Manufacturing) and Dabora 4 (Victory/Integrator/golden) merge — both touch integrator emission arms and are sequential anyway. Dabora 6a (BIN hex research) folds into Dabora 1's day-zero discovery work as inline xxd + Python instead of a committed script.
7. **Notification events emit at the point of decision, not via persistent bit-fields.** Drops 8 LOC of `incident_flags` set/clear plumbing per event. Direct `self.emit(SYS_ECONOMY, EVT_*, json)` from the discovery branches in `economy.rs` is identical in correctness and ~150 LOC simpler.
8. **`event_rolls` overflow becomes a hard `assert!` not `debug_assert!`.** Independent of this sprint's task list, `events.rs:553` currently has `unwrap_or(1.0)` which silently never-fires Random conditions in release builds — compromising the autoresearch fixed-eval-budget invariant. Fixing the upstream `unwrap_or` is ~2 LOC and protects every future sprint. Dabora 1 owns it.
9. **Save lockout covers all three entry points.** Plan originally only gated `Cmd+S`. Cockpit `SaveLoad` button (`main.rs:2011`) and context-menu `OpenSaveLoad` (`main.rs:2590`) were wide open. Belt-and-suspenders check inside `save_slot()` itself.
10. **quad-snd `set_volume` confirmed working in production.** `Sound::set_volume(&AudioContext, f32)` is called at `audio.rs:449-454` for looped music. `VideoPlayer::AudioTrack` already retains `_sound`. Phase 3 #V7 collapses to: rename `_sound → sound`, add 4-line `set_volume` method, add WASM no-op stub. No restart glitch, no documented limitation.
11. **Test count target revised from ≥470 to ≥447** after pruning ~18 redundant tests. Headless main-loop smoke test replaced with a pure `next_mode(mode, input) -> GameMode` unit test. Idempotency tests collapsed to one parameterized fixture per pattern.
12. **CI grep guard becomes a Rust unit test inside `story_events.rs`** instead of a shell script — co-located with the protected code, no shell, runs on every `cargo test`.

### New Considerations Discovered

- **Pre-existing bug exposed:** `EconomyState` is in `SimulationStates` but missing from `SaveState` at `crates/rebellion-data/src/save.rs:106-126`. Save mid-disaster, reload, the disaster re-fires on the first post-load tick because the bit was cleared. The v8 bump is the cheapest moment to close this — adds one struct field.
- **Pre-existing silent failure exposed:** `events.rs:553` `unwrap_or(1.0)` on `event_rolls` means any future Random condition that exceeds the 16-slot budget silently never fires. Autoresearch runs `--release`, where `debug_assert!` is a no-op. Fixed in Dabora 1 with a real `assert!`.
- **Existing 0x396 "Final Battle Imminent" already says "father and son"** — the new `heritage_known` flag must be flipped on this earlier event, not on `0x220`. Otherwise the narrative shows the paternity reveal first then displays the "Vader's Ultimatum" BMP, creating tonal whiplash. Audit-driven: `story_events.rs:215-269`.
- **`MovementState.orders` is keyed by `FleetKey`, not `CharacterKey`** (`movement.rs:206`). The original plan's "walk world.movement_orders for character == han" was the wrong data structure. Correct lookup: `han.current_fleet.and_then(|f| movement.orders().get(&f)).map(|o| o.destination)`.
- **`apply_event_actions` divergence is exactly one variant**: `DisplayMessage` is a no-op in `integrator.rs:913` but pushes to `MessageLog` at `main.rs:3279-3281`. The "byte-equivalence test" the original plan proposed would either fail or be a false test. Solved via `GameEffect::StoryMessageDisplayed`.
- **`Character` has no `Default` impl** — every construction site is explicit. Dabora 1 either greps and updates every site or adds a real `impl Default`. Plan now mandates the latter.
- **Three save-panel entry points exist, not one.** `Cmd+S` at `main.rs:579`, `CockpitButton::SaveLoad` at `main.rs:2011`, and `PanelAction::OpenSaveLoad` at `main.rs:2590`. The lockout must gate all three plus a defense-in-depth check inside `save_slot()`.
- **Slotmap key staleness in event payloads.** Telemetry payloads must serialize `character.dat_id: DatId` or `character.name: String`, never `CharacterKey`, because `EVT_CHARACTER_KILLED` runs after the character has been removed from the arena.

### Conflict Resolutions Across Reviewers

- **`last_support_tier` placement:** perf-oracle and DI both said keep on `world::System`; simplicity said intra-tick scratch in `EconomySystem::advance`. **Simplicity wins** because intra-tick scratch eliminates the save-format question entirely.
- **`incident_flags` reuse vs. delete:** perf-oracle/DI said reuse existing `SystemEconomy.incident_flags`; simplicity said delete and emit directly. **Hybrid wins:** delete `incident_flags` (simplicity) AND add `economy: EconomyState` to `SaveState` (DI's deeper bug fix), since the v8 bump pays for both.
- **`apply_event_actions` consolidation:** arch-CRITICAL-#2 said the duplicate is not byte-equivalent and proposed effect-based fix; simplicity-M1 said just `pub` and delete. **Combined wins:** `pub` the function AND route `DisplayMessage` through a new `GameEffect` variant.
- **`event_rolls` budget bump (#S8):** perf said dynamic helper, simplicity said cut entirely (no new Randoms). **Simplicity wins for sprint scope** (no new Random conditions land); BUT the upstream `unwrap_or(1.0)` silent-failure (SF-#9) is fixed independently as a Dabora 1 day-zero hardening.
- **CRITICAL findings 1+2** (phase ordering for `EVT_CHARACTER_KILLED` and `apply_event_actions` MessageLog drop) are both blocking but independent — both resolved in Phase 1.

### Verified Facts Now Encoded

- `quad_snd::Sound::set_volume(&AudioContext, f32)` works on in-flight playback. Cited at `audio.rs:449-454`.
- `crates/rebellion-data/src/save.rs:367` already carries the bincode-positional warning from a previous maintainer.
- `EffectPhase` has 8 variants. There is no `Story` variant. Story-adjacent effects use `Command`; economy-adjacent use `Economy`.
- Simulation phase order: Events=step 6, Death Star cleanup=step 11. `EVT_CHARACTER_KILLED` from cleanup is strictly **next-tick reactive**, not same-tick.
- `GameEffect::CharacterKilled` already exists at `effects.rs:148`. Reuse, don't duplicate.
- `cleanup_destroyed_system` has no `&mut PerceptionIntegrator` parameter. It must gain a `&mut Vec<GameEffect>` out-parameter to emit telemetry at all.
- `EVT_FORCE_DISCOVERED = 0x362` (generic) and `EVT_LEIA_FORCE = 0x363` are BOTH present in `events.rs:80, 100`. The "0x362 not implemented" claim in the cross-reference report is OUTDATED — that ID collision was fixed in Knesset Ereshkigal Phase 3.

---

## Overview

This sprint bundles two parallel parity tracks that share zero file ownership and can run as a single Minoan swarm:

- **Track A — Story & Events Sweep.** Wire the 6 partially-defined story event IDs (`0x128`, `0x200`, `0x231`, `0x306`, `0x361`, `0x386`) to actual `state.define()` registrations and to direct integrator emissions; add 9 missing notification events (`0x100`, `0x106`, `0x107`, `0x153`, `0x154`, `0x155`, `0x160`, `0x304`, `0x305`) as state-transition emissions in their owning systems (NEVER as `EventCondition::Random`); close Jabba's Palace case 4; add the `heritage_known` text-gate and the `FUN_00542050` Emperor combat-presence check for Final Battle; add a real `EventAction::SpawnSpecialForce` to make the Bounty Hunters chain create an actual entity. Lock the gains in `scripts/golden_values.json` under a new `story_events` section.
- **Track B — Advisor BIN + Cutscene Polish.** Decode at least one new advisor BIN header variant against a hex-corpus subtask; replace the BIN frame-ID → BMP modulo shortcut with a brute-force-discovered offset table patterned after `scripts/build-resource-entity-map.py`; chain victory/defeat cutscenes through the existing dead-coded `victory_screen.rs` egui modal; route `VideoPlayer` audio through `AudioVolumeState`.

This is the **final closing sprint for v0.20.x parity**. After this lands, the only remaining backlog items are P3 multiplayer foundations, Addon 1 (Create Your Own Character), and image-generation tasks.

## Problem Statement

### Track A — Story & Events Sweep

The community disassembly cross-reference (`docs/reports/2026-03-26-community-disassembly-cross-reference.md:201-247`) identified ~15 notification event IDs and 4 partial story chains as the largest remaining gap in Events & Story (currently 70% parity). Subsequent sprints (Knesset Ereshkigal Phase 3, Knesset Shamash Phase 3b, Knesset Hokhmah, Knesset Resheph) shipped:

- **Constants** for every event ID at `crates/rebellion-core/src/events.rs:67-102`.
- **Telemetry strings** for every event ID at `crates/rebellion-core/src/game_events.rs:67-130`.
- **A complete BMP catalog** at `crates/rebellion-render/src/bmp_cache.rs:178-319` (every story-relevant BMP has a named constant: `EVENT_LUKE_DISCOVERS_HERITAGE=1058`, `EVENT_VADER_VS_LEIA=1055`, `EVENT_EMPEROR_AND_VADER_VS_KNIGHT_LUKE=1064`, `EVENT_INFORMANTS_PROVIDE_INFORMATION=1000`, `EVENT_NATURAL_DISASTER=1003`, `EVENT_MAINTENANCE_SHORTFALL_ALLIANCE=1013`, `EVENT_CHARACTER_KILLED_ALLIANCE=1070`, etc.).
- **Existing partial chains** for Jabba's Palace (4 of 5 cases at `crates/rebellion-core/src/story_events.rs:392-679`), Final Battle co-location (`story_events.rs:271-321`), Bounty Hunters (`story_events.rs:1085`-region), Leia Force discovery (`story_events.rs:329`), and Emperor Arrival (`story_events.rs:359`).

What is **missing**, verified by direct grep on 2026-04-07:

1. **`state.define()` registrations** for `EVT_HQ_CAPTURED` (0x128), `EVT_HAN_RESCUE` (0x200), `EVT_JABBA_PRISONERS` (0x231), `EVT_CHARACTER_KILLED` (0x306), `EVT_TRAITOR_REVEALED` (0x361), `EVT_SIDE_CHANGE` (0x386). Constants exist; no event is registered.
2. **Direct integrator/system emissions** for the 9 notification events (`0x100`, `0x106`, `0x107`, `0x153`, `0x154`, `0x155`, `0x160`, `0x304`, `0x305`). No emission site exists anywhere — these have constants, telemetry strings, BMPs, and no producer.
3. **Jabba's Palace Case 4** (Han stays frozen forever). Cases 1-3 and 5 are wired; the bounded "5 escape rolls fail → permanent capture" terminal outcome is missing.
4. **`heritage_known: bool`** field on `world::Character`. The Final Battle event already uses `CharactersCoLocated{Luke, Vader, Emperor}` correctly, but the BMP screen always shows "Vader's Ultimatum" regardless of whether the player has already learned the paternity reveal. The flag does not exist anywhere in `rebellion-core`.
5. **`FUN_00542050_is_emperor_palpatine` combat-winner check** in `crates/rebellion-core/src/combat.rs`. Documented as missing in the cross-reference report; verified absent. Emperor presence should modify Final Battle outcomes.
6. **`EventAction::SpawnSpecialForce`** variant + handler. The Bounty Hunters chain currently fires events but never creates a `SpecialForce` instance at Han's system. The community cross-reference confirms the original game spawns a `BOUNTY_HUNTERS` entity from `SPECFCSD.DAT`.
7. **`is_story_event` telemetry filter** at `crates/rebellion-data/src/integrator.rs:1310-1314` only matches `0x210 | 0x212 | 0x220 | 0x221 | 0x380..=0x39A`. Any new event IDs outside this range will be silently misrouted to `SYS_EVENTS` instead of `SYS_STORY`, poisoning golden-value parity diffs.
8. **`apply_event_actions` duplication.** The function lives in BOTH `crates/rebellion-data/src/integrator.rs:910-982` (`apply_event_actions_to_world_inner`) AND `crates/rebellion-app/src/main.rs:3271-3409`. Documented as MEDIUM debt at `docs/plans/2026-03-26-001-feat-eval-driven-parity-open-souls-refactor-plan.md:167` but never consolidated. Every new `EventAction` variant requires touching both copies — high drift risk.
9. **`event_rolls` budget** at `crates/rebellion-app/src/main.rs:936` is hardcoded to 16 per tick. Adding new `Random` conditions will silently overflow.
10. **No `story_events` section** in `scripts/golden_values.json` to lock fire counts against autoresearch regression.

**Critical prior-art gotcha (must not be repeated):** A previous attempt added `0x100`/`0x153`/`0x154`/`0x155`/`0x160`/`0x304`/`0x305`/`0x361` as `EventCondition::Random`-driven per-tick rolls. They were **discarded in code review** when cross-reference against `FUN_0050a970` and `FUN_0050d720` revealed the original fires these on **state transitions** (bit-difference comparison against galaxy state), not random rolls. The lesson is recorded in the comment at `crates/rebellion-core/src/story_events.rs:689-702`. The plan must include a CI grep guard against this regression.

### Track B — Advisor BIN + Cutscene Polish

Knesset Resheph's task #132 (Knesset Kothar wa Khasis closing batch, 2026-04-07) closed two of four deferred items (cutscene playback U2 + advisor BIN simple-format C1) but explicitly left four gaps:

1. **Advisor BIN parser only handles ~24% of files** (183 of ~752 per faction). The simple `[u16 frame_count, u16 frame_id; frame_count]` format is the only variant decoded; the remaining ~76% declare inconsistent lengths and almost certainly use one or more undocumented header variants. Logged at `crates/rebellion-render/src/advisor.rs:557-593` per-faction summary.
2. **BIN frame-ID → BMP resource ID mapping is honest best-effort modulo** (`crates/rebellion-render/src/advisor.rs:486-596`, `frame_id as usize % self.primary_frame_pool_len`). The real DLL resource-index table is undocumented, but `scripts/build-resource-entity-map.py` solves the same class of problem via brute-force additive offset discovery. We can apply the same technique here.
3. **Victory/defeat cutscenes return directly to `MainMenu`** after `201.webm`/`202.webm`, bypassing the existing `crates/rebellion-render/src/victory_screen.rs` egui modal. Verified: `VictoryScreenState` is exported from the render lib but **never imported by `main.rs`** — the modal is dead code.
4. **`VideoPlayer` audio plays at fixed `volume: 1.0`** through its own `quad-snd AudioContext` (`crates/rebellion-render/src/video_player.rs:88-92, 322-347`), not respecting `AudioVolumeState` (`crates/rebellion-render/src/audio.rs:133-147`). Master/music sliders have no effect on cutscenes.

These four gaps are tracked in CLAUDE.md "Knesset Kothar wa Khasis — COMPLETE" "Known gaps left for future work" and in `agent_docs/roadmap.md` "Knesset Kothar wa Khasis — COMPLETE (2026-04-07)".

## Proposed Solution

A 6-dabora Minoan swarm with strict merge ordering. **Dabora 1 is a foundation pass that must merge before any other dabora can land**, because it consolidates `apply_event_actions` (eliminating the dual-write hazard for every subsequent task) and lands the new shared types (`EventAction::SpawnSpecialForce`, `Character::heritage_known`, `System::incident_flags`, `EventCondition::CharacterFlag` if needed, `EventAction::SetCharacterFlag`).

Dabora 6 (Advisor BIN research) is the only dabora that can usefully run **in parallel with Dabora 1** because its work is pure hex analysis on `.bin` files in `assets/references/ref-ui/07-droid-advisors/` plus a Python research script — it has no Rust dependencies until its implementation phase, which gates on Dabora 5.

Daborot 2-5 run in parallel after Dabora 1 lands, then Dabora 5 (cutscene chain) merges last because it touches the most volatile main.rs state machine.

## Technical Approach

### Architecture

The sprint respects the four cross-cutting bindings reaffirmed by `crates/rebellion-core/src/effects.rs`, `crates/rebellion-data/src/integrator.rs`, and CLAUDE.md:

1. **Effects are the only output (Knesset Ereshkigal Manifesto).** New world mutations land as `GameEffect` variants in `crates/rebellion-core/src/effects.rs`, systems return `Vec<GameEffect>`, the integrator's `apply_*` methods perform the mutations.
2. **`GameWorld` is read-only during system advance.** All mutation lives behind `apply_*` methods on `PerceptionIntegrator` in `crates/rebellion-data/src/integrator.rs:128-760`.
3. **Two-layer types.** Advisor and cutscene state never enter `rebellion-core`. Story flags live on `world::Character`, never on `dat::Character`.
4. **Never add rendering deps to `rebellion-core`.** Track B's audio plumbing stays in `rebellion-render` + `rebellion-app`.

The two tracks intersect only on `crates/rebellion-app/src/main.rs` (for state-machine wiring), `crates/rebellion-render/src/lib.rs` (for re-exports), and `crates/rebellion-data/src/integrator.rs` (Track A only). Daborot 5 and 6 share `crates/rebellion-render/src/`, so they merge serially.

### Type changes (Phase 1, Dabora 1)

```rust
// crates/rebellion-core/src/world/mod.rs
pub struct Character {
    // ... existing 27 fields ...
    #[serde(default)]
    pub heritage_known: bool,         // gates Final Battle BMP variant
    #[serde(default)]
    pub frozen_permanently: bool,     // Jabba case 4 terminal marker
}

pub struct System {
    // ... existing fields ...
    #[serde(default)]
    pub incident_flags: u8,           // bits 16-19 of original field_0x88
    #[serde(default)]
    pub last_support_tier: u8,        // for SupportChange edge detection
}

// crates/rebellion-core/src/effects.rs
pub enum GameEffect {
    // ... existing 33 variants ...
    SpecialForceSpawned {
        class_dat_id: DatId,
        system: SystemKey,
        is_alliance: bool,
    },
    CharacterFlagSet {
        character: CharacterKey,
        flag: CharacterFlag,
        value: bool,
    },
    SystemIncidentFired {
        system: SystemKey,
        kind: IncidentKind,            // Disaster, ResourceDiscovery, Informant, etc.
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CharacterFlag {
    HeritageKnown,
    FrozenPermanently,
}

// crates/rebellion-core/src/events.rs
pub enum EventAction {
    // ... existing 16 variants ...
    SpawnSpecialForce {
        class_dat_id: DatId,
        at_system: SystemKey,
        is_alliance: bool,
    },
    SetCharacterFlag {
        character: CharacterKey,
        flag: CharacterFlag,
        value: bool,
    },
}

pub enum EventCondition {
    // ... existing 16 variants ...
    CharacterFlag {
        character: CharacterKey,
        flag: CharacterFlag,
        expected: bool,
    },
}
```

`Character::heritage_known` is added with `#[serde(default)]` to remain compatible with save v7 — no migration needed because absent fields deserialize to `false`. `incident_flags` and `last_support_tier` follow the same pattern. The save format does NOT bump.

### Notification event emission sites (Phase 2, Daborot 2-4)

| Event ID | Constant | Owner system | Trigger | Idempotency mechanism |
|---|---|---|---|---|
| `0x100` | `EVT_SUPPORT_CHANGE` | `economy.rs` | Support crosses 20/40/60/80 tier boundary on either faction | `System.last_support_tier` |
| `0x105` | `EVT_FLEET_ARRIVE` | (already wired as `EVT_FLEET_ARRIVED` from `apply_arrivals`) | n/a — alias the constant | n/a |
| `0x106` | `EVT_CHARACTER_HEALTH` | `missions.rs` | `MissionEffect::CharacterInjured` resolution | one-shot per mission resolution |
| `0x107` | `EVT_UNITS_DEPLOYED` | `manufacturing.rs` | Build completion lands at destination system | one-shot per `BuildCompletion` |
| `0x128` | `EVT_HQ_CAPTURED` | `victory.rs` → `apply_victory` | `VictoryOutcome::HqCaptured` set | already gated by victory check |
| `0x153` | `EVT_INFORMANT_INTEL` | `missions.rs` | Successful Espionage mission with non-empty intel payload | one-shot per mission resolution |
| `0x154` | `EVT_NATURAL_DISASTER` | `economy.rs` | `incident_flags & DISASTER_BIT` and previously zero | bit clear after emission |
| `0x155` | `EVT_RESOURCE_DISCOVERY` | `economy.rs` | `incident_flags & DISCOVERY_BIT` and previously zero | bit clear after emission |
| `0x160` | `EVT_MANUFACTURING_IDLE` | `manufacturing.rs` | Queue draining to empty (was non-empty last tick) | per-system `was_empty: bool` |
| `0x200` | `EVT_HAN_RESCUE` | `story_events.rs` | Re-emit alongside `0x383` Jabba rescue | EventFired guard |
| `0x231` | `EVT_JABBA_PRISONERS` | `story_events.rs` | Consolidator: ANY of `0x385 \| 0x387 \| 0x399` fires | `EventFired` guard, defines AFTER captures |
| `0x304` | `EVT_MAINTENANCE_SHORTFALL_EVENT` | `economy.rs` | Costs > income on a per-faction 30-tick timer (`GNPRTB[7694]`) | per-faction timer |
| `0x305` | `EVT_SABOTEUR_DETECTED` | `missions.rs` | Enemy `MissionKind::Sabotage` resolves on player-controlled system | one-shot per resolution |
| `0x306` | `EVT_CHARACTER_KILLED` | `death_star.rs::cleanup_destroyed_system` AND `missions.rs` | Character removed from arena | one-shot per character removal |
| `0x361` | `EVT_TRAITOR_REVEALED` | `betrayal.rs` | Intel reveals a traitor BEFORE the flip (distinct from defection) | per-character one-shot |
| `0x386` | `EVT_SIDE_CHANGE` | `betrayal.rs` | Actual faction flip resolves | per-character one-shot |

All emissions go through the integrator's `emit(SYS_*, EVT_*, json)` path and are tagged via the (post-Phase-1) extended `is_story_event` filter for the 6 story IDs (0x128, 0x200, 0x231, 0x306, 0x361, 0x386); the remaining 9 stay tagged as `SYS_ECONOMY`/`SYS_MANUFACTURING`/`SYS_MISSIONS` per their owning system.

### Jabba's Palace Case 4 (Phase 2, Dabora 3)

Add a new event `EVT_HAN_PERMANENT_FREEZE` (suggest `0x39B` per the 0x380-0x39A continuation; reserve 0x39C+ for any follow-up):

```rust
state.define(GameEvent {
    id: EVT_HAN_PERMANENT_FREEZE,           // 0x39B
    conditions: vec![
        EventCondition::CharacterIsCaptive { character: han },
        EventCondition::CharacterFlag { character: han, flag: FrozenPermanently, expected: false },
        EventCondition::EventFired { id: EVT_HAN_CARBONITE_FAIL_5 },  // a counter-style escape-roll-failed event
        EventCondition::EventNotFired { id: EVT_HAN_RESCUE },
    ],
    actions: vec![
        EventAction::SetCharacterFlag { character: han, flag: FrozenPermanently, value: true },
        EventAction::DisplayMessage { resource_id: EVENT_JABBA_CAPTURES_SOLO },
    ],
    one_shot: true,
});
```

The escape-attempt counter is tracked via successive event firings (existing `EventFired` chaining); the terminal `frozen_permanently` flag prevents re-evaluation of any further escape conditions in the existing escape chain, which `EventCondition::CharacterFlag { ..., expected: false }` will gate.

### Final Battle heritage gate (Phase 2, Dabora 3)

Split the existing `0x220 EVT_FINAL_BATTLE` into two events, both with the `CharactersCoLocated{Luke, Vader, Emperor}` precondition:

- `0x220 EVT_FINAL_BATTLE_ULTIMATUM` — fires when `heritage_known == false`, displays `EVENT_VADER_VS_STUDENT_LUKE` (1059) → `EVENT_VADER_VS_KNIGHT_LUKE` (1060) sequence, sets `heritage_known = true` as side effect.
- `0x222 EVT_FINAL_BATTLE_KNOWN` — fires when `heritage_known == true`, displays `EVENT_EMPEROR_AND_VADER_VS_KNIGHT_LUKE` (1064).

Existing `0x220` consumers (golden_values, telemetry, ai-parity-tracker references) are preserved by making `0x220` the canonical "Final Battle started" identifier and `0x222` the chained follow-up.

### Bounty Hunters spawn entity (Phase 2, Dabora 3)

A `BOUNTY_HUNTERS_DAT_ID: DatId = DatId::new(0xFF000001)` constant is added in the reserved-mod range CLAUDE.md mentions for Addon 1 Phase 5, sentinel-flagged so that future SPECFCSD.DAT-derived classes can override it. The chain becomes:

```rust
state.define(GameEvent {
    id: EVT_BOUNTY_ATTACK,    // 0x212, existing
    conditions: /* unchanged */,
    actions: vec![
        EventAction::SpawnSpecialForce {
            class_dat_id: BOUNTY_HUNTERS_DAT_ID,
            at_system: /* Han.current_system if Some, else his transit destination */,
            is_alliance: false,
        },
        // existing display action
    ],
    one_shot: true,
});
```

The integrator handler creates a `SpecialForceUnit` and pushes its key onto the system's `special_forces` slot. If Han is in transit (`current_system.is_none()`), the integrator falls back to his pending destination from the active `MovementOrder` lookup.

### Emperor Palpatine combat check (Phase 2, Dabora 3)

Port `FUN_00542050_is_emperor_palpatine` as a `winner_resolution_emperor_modifier()` helper in `crates/rebellion-core/src/combat.rs`. The community cross-reference labels it but does not give the formula — the implementation lands as a documented best-available approximation (Emperor presence multiplies the winning faction's effective combat strength by `1.0 + GNPRTB[3076] / 100.0 = 1.5`, where 3076 is the existing `seat_of_power_bonus` constant in `golden_values.json:1032`). This is honest best-effort, mirrored after the Death Star "best available approximation" pattern Knesset Resheph established.

### Advisor BIN variant decoder (Phase 3, Dabora 6)

Two-phase work:

**Phase 6a — Hex research (parallel with Dabora 1).** A new helper at `scripts/dump_advisor_bins.py` walks every `.bin` file under `assets/references/ref-ui/07-droid-advisors/{alliance,empire}/`, runs the existing `parse_advisor_bin` logic, and dumps the first 32 bytes of any failing file in hex + decoded `u16` interpretation. Output goes to `agent_docs/research/2026-04-08-advisor-bin-variant-corpus.md` (committed as primary research artifact). The corpus is grouped by failure mode (`TruncatedHeader`, `TruncatedFrames`, `LengthMismatch`) and by the apparent value of bytes 0-3 to surface the discriminator.

**Phase 6b — Decoder + offset table (after Dabora 5).** Add `parse_advisor_bin_v2(&[u8]) -> Result<BinSequence, BinError>` that handles whichever variant the corpus reveals. Most likely candidates:

- `[u16 frame_count, u16 fps_hint, u16 frame_id; frame_count]` — 6-byte header with timing.
- `[u16 record_count, {u16 frame_id, u16 hold_ticks}; record_count]` — frame+timing pairs.
- `[u8 type_tag, u8 padding, u16 frame_count, u16 frame_id; frame_count]` — typed header.

The implementation lands as a `try_parse_v1()` → `try_parse_v2()` cascade, preserving the existing 24% success branch as the first attempt. Acceptance: parse rate rises from 24% to ≥50% across both factions (with the exact target locked in after Phase 6a's corpus analysis).

For the BIN frame-ID → BMP resource ID mapping, port `scripts/build-resource-entity-map.py`'s offset-discovery technique into a one-shot Python script `scripts/discover_advisor_bin_offset.py`:

1. For each successfully-parsed BIN file, record its declared frame IDs.
2. Cross-reference against the actual sorted BMP resource IDs (parsed from filename prefixes).
3. Try plausible additive offsets `[+0, +200, +700, +1000, +2000, +1801, +0x600, +0x800]`.
4. The offset that maximizes "declared frame ID + offset → real BMP exists" is the discovered table.
5. Bake the offset into a `const ADVISOR_BIN_BMP_OFFSET: i32` in `advisor.rs`.

If no single offset maps cleanly across both factions, fall back to two faction-specific offsets and document.

### VideoPlayer volume routing (Phase 3, Dabora 5)

`VideoPlayer::open` gains a `volume: f32` parameter. The default-1.0 path stays for the WASM stub. A new `VideoPlayer::set_volume(&mut self, vol: f32)` method calls into the underlying `quad_snd::Sound::set_volume(&self.audio.ctx, vol)` if `Playback` exposes it; if not, the fallback is to buffer the new volume and apply on next `play()` cycle (audible glitch acceptable as documented limitation). A pre-implementation research subtask reads quad-snd source to confirm the Playback API.

`main.rs` passes `audio_vol.master_volume * audio_vol.music_volume` (both already on `AudioVolumeState`) into every `open_cutscene()` call, and re-pushes the value once per frame if `audio_vol.dirty` while `cutscene_kind.is_some()`.

### Cutscene → VictoryModal chain (Phase 3, Dabora 5)

State machine recommendation: `Galaxy → VictoryModal (player dismisses) → Cutscene (player skips/finishes) → MainMenu`. This avoids overlaying egui on macroquad's fullscreen cutscene texture (which would re-introduce synchronization risk).

```rust
// crates/rebellion-app/src/main.rs (sketch, NOT FOR IMPLEMENTATION)
enum CutsceneKind { Intro, Victory, Defeat }

enum GameMode {
    MainMenu,
    GameSetup,
    Galaxy,
    VictoryModal { outcome: VictoryOutcome },     // NEW
    Cutscene { kind: CutsceneKind },              // EXTENDED — was unit
    // ...
}
```

`GameMode::VictoryModal` draws the existing `victory_screen.rs` modal over a frozen galaxy background. ESC or "Continue" → `GameMode::Cutscene { kind: CutsceneKind::Victory|Defeat }`. The `cutscene_kind` is persisted on the variant. After the cutscene finishes (or is skipped), the post-cutscene transition reads the variant: `Intro → MainMenu`, `Victory|Defeat → MainMenu` (the modal already shipped its narrative text). Save hotkey is disabled while `matches!(mode, GameMode::Cutscene { .. } | GameMode::VictoryModal { .. })`.

WASM behavior: `VideoPlayer` stub returns `is_finished() == true` immediately, so the WASM path collapses to `Galaxy → VictoryModal → MainMenu` automatically. No special-case code.

### Implementation Phases

#### Phase 1: Foundation (Dabora 1 — solo, blocks all others)

*Model: opus (consolidation work, type design, multi-file coordination)*

Tasks:
- **#S1 — `apply_event_actions` consolidation.** Extract the duplicated body from `crates/rebellion-app/src/main.rs:3271-3409` into a single shared `apply_event_action_to_world(action, world, log, tick) -> ()` helper exported from `rebellion-data::integrator` (public re-export of the existing private inner). main.rs's loop becomes a thin wrapper. Add a unit test in `integrator.rs` that asserts every `EventAction` variant matches in both call sites by pattern coverage.
- **#S2 — Closed-enum `EventAction` extension.** Add `SpawnSpecialForce` and `SetCharacterFlag` variants with full integrator + main.rs handler arms.
- **#S3 — `EventCondition::CharacterFlag` variant.** Add the predicate with evaluator at `events.rs:553`-region.
- **#S4 — `world::Character` flag fields.** Add `heritage_known: bool` and `frozen_permanently: bool` with `#[serde(default)]`. Update `Character::default()` if it exists.
- **#S5 — `world::System` incident fields.** Add `incident_flags: u8` and `last_support_tier: u8` with `#[serde(default)]`. Add `IncidentKind` enum + bit-mask constants.
- **#S6 — `GameEffect` variants.** Add `SpecialForceSpawned`, `CharacterFlagSet`, `SystemIncidentFired`. Tag with appropriate `EffectPhase` (Story for the first two, Economy for incident).
- **#S7 — `is_story_event` filter expansion** in `crates/rebellion-data/src/integrator.rs:1310-1314` to include `0x128 | 0x200 | 0x230 | 0x231 | 0x306 | 0x361 | 0x386 | 0x387` plus the new `0x222`/`0x39B` IDs introduced in Phase 2.
- **#S8 — `event_rolls` budget bump** at `crates/rebellion-app/src/main.rs:936`. Either expand to 32 (sufficient for all current + planned `Random` conditions) or compute dynamically via a `state.required_random_rolls()` helper.
- **#S9 — Sentinel `BOUNTY_HUNTERS_DAT_ID` constant** in `crates/rebellion-core/src/world/mod.rs` (next to the SPECFCSD area).
- **#S10 — CI grep guard** as a Rust integration test or `scripts/check_no_random_notification_events.sh` that fails if any of `EVT_SUPPORT_CHANGE`, `EVT_INFORMANT_INTEL`, `EVT_NATURAL_DISASTER`, `EVT_RESOURCE_DISCOVERY`, `EVT_MAINTENANCE_SHORTFALL_EVENT`, `EVT_SABOTEUR_DETECTED`, `EVT_TRAITOR_REVEALED`, `EVT_MANUFACTURING_IDLE` appears within 5 lines of `EventCondition::Random` in `story_events.rs`. Wire into `cargo test`.
- **#S11 — Stale comment cleanup** at `crates/rebellion-core/src/story_events.rs:1316` (the Leia 0x362→0x363 ID-collision note).

**Success criteria (Phase 1):** `cargo check` clean, all 417 existing tests pass, new types compile, the consolidation test asserts both apply sites are byte-equivalent on a synthetic action stream, the CI grep guard fails on a deliberate test offender and passes on the real codebase.

**Estimated effort:** ~600 LOC (mostly type plumbing).

#### Phase 2: Story & event emissions (Daborot 2, 3, 4 — parallel after Phase 1)

##### Dabora 2 — "Kothar-Gimel" (Economy + Manufacturing notification events)

*Model: sonnet (high-throughput pattern porting)*

Owns: `crates/rebellion-core/src/economy.rs`, `crates/rebellion-core/src/manufacturing.rs`.

Tasks:
- **#K1 — `EVT_SUPPORT_CHANGE` (0x100)** emission in `EconomySystem::advance` whenever a system's support tier crosses 20/40/60/80 (compared against `System.last_support_tier`). Updates `last_support_tier` post-emission.
- **#K2 — `EVT_NATURAL_DISASTER` (0x154)** emission when `incident_flags & DISASTER_BIT` is newly set. Bit clears after emission.
- **#K3 — `EVT_RESOURCE_DISCOVERY` (0x155)** emission, same pattern with `DISCOVERY_BIT`.
- **#K4 — `EVT_MAINTENANCE_SHORTFALL_EVENT` (0x304)** emission via per-faction 30-tick timer (`GNPRTB[7694]`). The timer state lives on `EconomySystem` (not on `GameWorld`) since it is intra-tick scratch.
- **#K5 — `EVT_UNITS_DEPLOYED` (0x107)** emission in `ManufacturingSystem::advance` when build completions arrive at their destination system (the `BuildCompletion` event is the trigger).
- **#K6 — `EVT_MANUFACTURING_IDLE` (0x160)** emission when a system's queue drains to empty. Per-system `was_empty: bool` lives on `ManufacturingState` (intra-system, persisted across ticks).
- **#K7 — Tests** for each event: minimal `GameWorld` fixture, trigger the state transition, assert exactly-one emission, assert idempotency.

**Files (OWN):** `economy.rs`, `manufacturing.rs`. **Read-only references:** `effects.rs`, `events.rs`, `world/mod.rs`, `game_events.rs`.

**Estimated effort:** ~400 LOC.

##### Dabora 3 — "Resheph-Dalet" (Story chains + Mission/Betrayal/Death Star events)

*Model: opus (story chain authoring + Final Battle/Bounty Hunter logic)*

Owns: `crates/rebellion-core/src/story_events.rs`, `crates/rebellion-core/src/missions.rs`, `crates/rebellion-core/src/betrayal.rs`, `crates/rebellion-core/src/death_star.rs`, `crates/rebellion-core/src/combat.rs`.

Tasks:
- **#R1 — `EVT_HAN_RESCUE` (0x200)** registered in `define_story_events()` to fire alongside the existing 0x383 Jabba rescue (chained via `EventFired { id: 0x383 }`).
- **#R2 — `EVT_JABBA_PRISONERS` (0x231)** consolidator event. Defined AFTER 0x385/0x387/0x399 in source order so same-tick chaining works. Conditions: `EventFired{0x385} | EventFired{0x387} | EventFired{0x399}` (requires either an OR condition variant or three separate consolidator events keyed off each capture).
- **#R3 — `EVT_HAN_PERMANENT_FREEZE` (0x39B)** event for Jabba Case 4 — terminal state with `SetCharacterFlag(FrozenPermanently)`.
- **#R4 — Final Battle heritage split.** Replace existing `0x220 EVT_FINAL_BATTLE` with two events: `0x220 EVT_FINAL_BATTLE_ULTIMATUM` (gates on `heritage_known == false`, sets it true) and `0x222 EVT_FINAL_BATTLE_KNOWN` (gates on `heritage_known == true`).
- **#R5 — `Bounty Hunters` real spawn.** Wire the existing `EVT_BOUNTY_ATTACK` (0x212) chain to add `EventAction::SpawnSpecialForce { class_dat_id: BOUNTY_HUNTERS_DAT_ID, at_system: han_system, is_alliance: false }`.
- **#R6 — `EVT_INFORMANT_INTEL` (0x153)** emission in `missions.rs` when an Espionage mission resolves successfully with a non-empty intel payload.
- **#R7 — `EVT_SABOTEUR_DETECTED` (0x305)** emission in `missions.rs` when an enemy Sabotage mission resolves on a player-controlled system.
- **#R8 — `EVT_CHARACTER_HEALTH` (0x106)** emission in `missions.rs` on `MissionEffect::CharacterInjured`.
- **#R9 — `EVT_TRAITOR_REVEALED` (0x361)** emission in `betrayal.rs` when intel reveals a traitor BEFORE the flip (new branch in betrayal evaluator).
- **#R10 — `EVT_SIDE_CHANGE` (0x386)** emission in `betrayal.rs` when the actual flip resolves (rename or wrap existing `BetrayalEvent::CharacterBetrayed`).
- **#R11 — `EVT_CHARACTER_KILLED` (0x306)** emission in `death_star.rs::cleanup_destroyed_system` AND in `missions.rs` for assassination resolutions.
- **#R12 — `FUN_00542050_is_emperor_palpatine` combat winner check** in `combat.rs` — `winner_resolution_emperor_modifier()` helper applied to `phase_who_won` when Emperor is co-located with the engagement.
- **#R13 — Tests** for every new chain. Use the existing `make_world_with_characters()` fixture pattern from `story_events.rs:764`.

**Files (OWN):** `story_events.rs`, `missions.rs`, `betrayal.rs`, `death_star.rs`, `combat.rs`. **Touches but does not own:** `events.rs` (new event ID constants if any beyond `0x222`/`0x39B`).

**Estimated effort:** ~900 LOC.

**Inter-dabora coordination note:** Dabora 3 is the largest dabora and touches `story_events.rs` exclusively. No other dabora modifies that file. The new event constants (`EVT_FINAL_BATTLE_KNOWN = 0x222`, `EVT_HAN_PERMANENT_FREEZE = 0x39B`) are added by Dabora 1 in Phase 1 to avoid late-stage `events.rs` collision.

##### Dabora 4 — "Athirat-He" (Victory + Integrator emissions + golden oracle)

*Model: opus (golden oracle authoring requires careful parity claims)*

Owns: `crates/rebellion-core/src/victory.rs`, `crates/rebellion-data/src/integrator.rs` (emission arms only — Phase 1 owned the consolidation), `scripts/golden_values.json`, `scripts/eval_parity.py` (extensions only).

Tasks:
- **#A1 — `EVT_HQ_CAPTURED` (0x128)** emission from `apply_victory` whenever `VictoryOutcome::HqCaptured` is set, BEFORE the victory modal trigger so telemetry consumers see the event before the run terminates.
- **#A2 — Integrator emission for `SpecialForceSpawned` `GameEffect`** — new arm in `apply_*` that creates the `SpecialForceUnit` in `world.special_forces` and pushes the key onto `system.special_forces`. Includes the in-transit fallback (use `MovementOrder` destination if `current_system.is_none()`).
- **#A3 — Integrator emission for `CharacterFlagSet`** — minimal arm that mutates the named flag on the named character.
- **#A4 — Integrator emission for `SystemIncidentFired`** — handler that clears the incident bit after emission.
- **#A5 — `story_events` section of `golden_values.json`.** A new top-level key with expected fire counts for a deterministic 5000-tick campaign at seed `42`. Counts come from a baseline run of `cargo run -p rebellion-playtest -- --seed 42 --ticks 5000 --jsonl > baseline.jsonl` followed by a histogram script. Lock fire counts (with ±10% tolerance) for: `EVT_DAGOBAH_COMPLETED`, `EVT_FINAL_BATTLE_ULTIMATUM`, `EVT_FINAL_BATTLE_KNOWN`, `EVT_LUKE_DAGOBAH`, `EVT_LEIA_FORCE`, `EVT_HQ_CAPTURED`, `EVT_HAN_RESCUE`, `EVT_JABBA_PRISONERS`, `EVT_BOUNTY_ATTACK`, `EVT_HAN_PERMANENT_FREEZE`, `EVT_EMPEROR_ARRIVAL`, `EVT_TRAITOR_REVEALED`, `EVT_SIDE_CHANGE`, `EVT_CHARACTER_KILLED`.
- **#A6 — `eval_parity.py` extension** to load the new `story_events` section and report pass/fail per event.
- **#A7 — Snapshot the OLD telemetry tag distribution** before #S7 (filter expansion) lands so the diff is intentional and documented in the merge commit.

**Files (OWN):** `victory.rs`, integrator emission arms, `golden_values.json`, `eval_parity.py`. **Touches but does not own:** `integrator.rs` (Phase 1 owns the consolidation).

**Estimated effort:** ~350 LOC + JSON.

#### Phase 3: Cutscene chain + Advisor BIN (Daborot 5 + 6, parallel — but Dabora 6 implementation gates on Dabora 5 merge)

##### Dabora 5 — "Kothar wa Khasis-Vav" (Cutscene state machine)

*Model: opus (state machine + audio bridge)*

Owns: `crates/rebellion-render/src/victory_screen.rs`, `crates/rebellion-render/src/video_player.rs`, `crates/rebellion-app/src/main.rs` (state machine + cutscene wiring sections only — Dabora 1 owns the apply_event_actions consolidation).

Tasks:
- **#V1 — `VictoryScreenState` wired into `main.rs`.** Add the field, populate it on victory detection, draw it in the new `GameMode::VictoryModal` arm.
- **#V2 — `GameMode::VictoryModal { outcome: VictoryOutcome }`** new variant. Renders the egui modal over a frozen galaxy background. ESC and "Continue" button transition to `GameMode::Cutscene { kind }`.
- **#V3 — `GameMode::Cutscene { kind: CutsceneKind }`** extension. Existing unit variant becomes a struct variant. Migration: any current cutscene trigger sets `kind: CutsceneKind::Intro` for `000.webm`, `Victory` for `201.webm`, `Defeat` for `202.webm`.
- **#V4 — Post-cutscene transition logic.** `Intro → MainMenu`, `Victory|Defeat → MainMenu` (modal already shipped before cutscene).
- **#V5 — Save lock-out** while `matches!(mode, Cutscene{..} | VictoryModal{..})` — gray out the save panel and disable the Cmd+S hotkey.
- **#V6 — `VideoPlayer::open(path, volume: f32)`** signature change. Default 1.0 in WASM stub.
- **#V7 — `VideoPlayer::set_volume(&mut self, vol: f32)`** new method. Pre-implementation research: read `quad-snd` source at `~/.cargo/registry/src/.../quad-snd-*/src/` to confirm whether `Playback` exposes runtime volume control. If not, document the audible-glitch fallback.
- **#V8 — `main.rs` audio bridge.** Pass `audio_vol.master_volume * audio_vol.music_volume` into every `open_cutscene()` call. While `cutscene_kind.is_some()`, re-push the value once per frame if `audio_vol.dirty`.
- **#V9 — Graceful WAV-missing branch.** `VideoPlayer` should treat a missing `.wav` sidecar as a silent cutscene (`has_audio: bool` field), not a hard error.
- **#V10 — Test:** headless smoke test asserting `Galaxy → VictoryModal → Cutscene → MainMenu` transition sequence using a fake `VictoryOutcome` and the `NotDecoded` cutscene path (which short-circuits but still drives the state machine).

**Files (OWN):** `victory_screen.rs`, `video_player.rs`, `main.rs` (cutscene/state machine sections). **Read-only:** `audio.rs`, `bmp_cache.rs`.

**Estimated effort:** ~500 LOC.

##### Dabora 6 — "Nabu-Zayin" (Advisor BIN research + decoder)

*Model: opus (RE pattern recognition + Python research scripts) — BIN research phase runs PARALLEL to Dabora 1 from day one, implementation phase runs AFTER Dabora 5.*

Owns: `crates/rebellion-render/src/advisor.rs`, new `scripts/dump_advisor_bins.py`, new `scripts/discover_advisor_bin_offset.py`, new `agent_docs/research/2026-04-08-advisor-bin-variant-corpus.md`.

Tasks (Phase 6a — research, parallel from day one):
- **#N1 — Hex corpus extraction.** Walk every `.bin` file under `assets/references/ref-ui/07-droid-advisors/{alliance,empire}/`. For each file: parse with current `parse_advisor_bin` logic (in Python, mirroring the Rust implementation), classify by failure mode, dump first 32 bytes in hex + decoded `u16`/`u8` interpretation.
- **#N2 — Variant signature discovery.** Group failing files by the value of bytes 0-3 to surface the discriminator. Annotate at least 20 representative samples manually with hypothesized field meanings.
- **#N3 — BIN→BMP offset discovery.** Cross-reference the 24% of successfully-parsed BIN files' declared frame IDs against actual sorted BMP filename prefixes. Try `[+0, +200, +700, +1000, +2000, +1801, +0x600, +0x800]` offsets. Report which offset (if any) maximizes coverage. Acceptance: ≥80% of declared frame IDs in the parsing-success corpus map to a real BMP under the chosen offset.
- **#N4 — Research artifact.** Commit `agent_docs/research/2026-04-08-advisor-bin-variant-corpus.md` with the corpus, hypotheses, and offset discovery results. This is the input to Phase 6b implementation.

Tasks (Phase 6b — implementation, after Dabora 5 merge):
- **#N5 — `parse_advisor_bin_v2`** implementing the variant decoder revealed by Phase 6a. Cascades from `try_parse_v1` → `try_parse_v2`. Acceptance: parse rate ≥50% across both factions (target locked after Phase 6a corpus).
- **#N6 — `ADVISOR_BIN_BMP_OFFSET` const** in `advisor.rs`, replacing the modulo shortcut at `:472-477`. Falls back to modulo for any frame ID that does not map under the discovered offset.
- **#N7 — Per-faction logging update** so the summary at `:584-593` reports `valid_v1`, `valid_v2`, `parse-failed`, `mapped`, `unmapped`.
- **#N8 — Tests** for: `parse_advisor_bin_v2` on a representative corpus sample committed to the test fixtures, BIN→BMP offset lookup, fallback behavior.

**Files (OWN):** `advisor.rs`, `scripts/dump_advisor_bins.py`, `scripts/discover_advisor_bin_offset.py`, `agent_docs/research/2026-04-08-advisor-bin-variant-corpus.md`.

**Estimated effort:** ~300 LOC + research scripts + corpus.

### Merge order

```
Dabora 1 (Foundation) ──┬─→ Dabora 2 (Economy/Manufacturing)  ─┐
                        ├─→ Dabora 3 (Story chains)            ├─→ Dabora 4 (Victory + golden) ─→ Dabora 5 (Cutscene) ─→ Dabora 6b (BIN impl)
                        └─→ Dabora 6a (BIN research, parallel from day one)
```

Dabora 4 lands AFTER Daborot 2-3 because the golden-value lock-in needs all new emissions in place. Dabora 5 lands AFTER Dabora 4 because both touch `main.rs` and the merge ordering reduces conflict. Dabora 6b lands LAST because its `advisor.rs` edits would otherwise conflict with any late `rebellion-render` changes from Dabora 5.

## Alternative Approaches Considered

### Alternative 1: Single-dabora sequential implementation

Run all tasks in one sub-agent serially. **Rejected**: prior knesset velocity data shows ~3× speedup with 4-6 daborot in parallel; this sprint has 25+ discrete tasks and serial execution would push the sprint into multiple sessions.

### Alternative 2: Land notification events as `EventCondition::Random`

The naive approach. **Rejected**: this is the exact regression Knesset Shamash discovered and discarded in 2026-03-26. Documented at `crates/rebellion-core/src/story_events.rs:689-702`. CI grep guard #S10 enforces.

### Alternative 3: Add per-character `MissionKind::PalaceRescue` as new mission type

The community cross-reference says the original game has "per-character dispatch (Leia, Luke, Chewbacca each get dedicated palace missions)" — this implies real `MissionKind` variants. **Rejected for this sprint**: adding a new mission type touches the entire mission probability table, missions panel UI, AI dispatch validation, and the mod loader's mission registry. Out of scope for a parity sprint. Track this as a follow-up sprint.

### Alternative 4: Wire the 8 unwired story event cutscenes (101-108) in this sprint

`agent_docs/game-media.md:67-81` lists 101-108 as story event cutscenes that are NOT wired. **Rejected for this sprint**: video player code paths are already in flight via Dabora 5; adding 8 new cutscene triggers with their own state-machine integration is a parallel feature, not a parity fix. Track as Knesset-N+1 follow-up.

### Alternative 5: Wait for new Ghidra RE work on the advisor BIN format

Instead of doing brute-force hex analysis on the 76% of rejected BIN files, schedule a Ghidra session against the advisor engine functions. **Rejected**: GhidraMCP work has higher coordination overhead than direct hex inspection of a small corpus, and the brute-force technique already worked for `scripts/build-resource-entity-map.py`. Falls back to "additional Ghidra work" only if Phase 6a research yields ambiguous results.

### Alternative 6: Defer golden-value oracle additions to a separate sprint

Treat #A5/#A6 as follow-up. **Rejected**: locking the gains in the same sprint that creates them is the entire point. Without a golden-value oracle, autoresearch can silently regress the new emissions.

## System-Wide Impact

### Interaction Graph

**New event registration** in `define_story_events()` creates a chain reaction:

1. `EventSystem::advance(state, world, &[TickEvent])` evaluates conditions in definition order.
2. A firing event pushes its `actions` into the integrator's effect queue.
3. `PerceptionIntegrator::apply_fired_events()` (`crates/rebellion-data/src/integrator.rs:471-495`) routes each action through `apply_event_actions_to_world_inner()` (`:910-982`).
4. Actions mutate `world` (`Character`, `System`, `SpecialForceUnit`).
5. Telemetry events emit via `self.emit(SYS_*, EVT_*, json)`.
6. The integrator's snapshot is consumed by:
   - `crates/rebellion-app/src/main.rs:935-1014` (interactive event-screen overlay path).
   - `crates/rebellion-render/src/message_log.rs` (message log entries with `MessageCategory::Event`).
   - `rebellion-playtest --jsonl` (campaign telemetry stream).
   - `scripts/eval_parity.py` (golden-value oracle).
7. Save/load preserves `EventState.fired_events` and `Character` flag fields via the existing `bincode` snapshot — `#[serde(default)]` on the new fields keeps save v7 compatible.

**New `EventAction::SpawnSpecialForce`** triggers a deeper mutation chain:
1. `apply_event_action_to_world` matches the variant.
2. Creates a `SpecialForceUnit { class_dat_id, is_alliance }` and inserts into `world.special_forces` (slotmap).
3. Pushes the resulting `SpecialForceKey` onto `system.special_forces`.
4. Emits `GameEffect::SpecialForceSpawned` (closed enum, propagated to telemetry).
5. AI dispatch validation in `crates/rebellion-core/src/ai.rs` may pick up the new unit on its next evaluation tick — verify by reading `evaluate_*` methods that touch `system.special_forces`.

**Cutscene → VictoryModal chain** (Dabora 5):
1. `VictorySystem::check()` returns `Some(outcome)`.
2. `apply_victory()` sets `world.victory_outcome = Some(outcome)` AND emits `EVT_HQ_CAPTURED` (Dabora 4 #A1).
3. Main loop detects `victory_outcome.is_some()`, sets `mode = GameMode::VictoryModal { outcome }`.
4. Egui draws the modal over a frozen galaxy background each frame.
5. Player ESC/Continue → `mode = GameMode::Cutscene { kind: CutsceneKind::Victory|Defeat }`.
6. `open_cutscene(VICTORY_CUTSCENE, audio_vol.effective_music_volume())` succeeds (or logs `NotDecoded` and short-circuits).
7. `VideoPlayer::advance(dt)` runs each frame; `audio_vol.dirty` triggers `set_volume`.
8. `is_finished() == true` → `mode = GameMode::MainMenu`.

### Error & Failure Propagation

- **`Character` slotmap key invalidation:** if Han is removed from the arena (assassination, Death Star cleanup) between the firing of `EVT_BOUNTY_ATTACK` and `apply_event_action_to_world::SpawnSpecialForce`, the `at_system: SystemKey` lookup uses Han's `current_system`. Edge case: Han is dead, `han.current_system` is invalid. Handler must guard with `world.characters.contains_key(han)` before computing `at_system`.
- **`MovementOrder` lookup for in-transit Han:** if `han.current_system.is_none()`, the spawn handler walks `world.movement_orders` for the active order with `character == han`. If none exists (race condition), it logs and skips the spawn.
- **`incident_flags` bit clearing:** the bit must clear AFTER emission, not before, or a re-entry into `economy.advance` in the same tick (impossible currently but defensive) would re-fire.
- **`set_volume` during cutscene:** if `quad_snd::Playback` does not expose runtime volume, the `set_volume` method buffers the value and applies on next `play()` cycle. Fallback documented; no propagation.
- **`VictoryModal → Cutscene` ESC propagation:** ESC during the modal advances to the cutscene (forward); ESC during the cutscene advances to MainMenu (forward). No "back" path from cutscene to modal — the game is over.
- **Save during VictoryModal:** save hotkey is disabled to prevent partial-state captures.

### State Lifecycle Risks

- **`heritage_known` set during `0x220` firing:** the action runs in the integrator's `apply_event_actions_to_world_inner` within the same tick the condition was evaluated. Same-tick re-evaluation of `0x222 EVT_FINAL_BATTLE_KNOWN` (which also gates on the all-three co-location) would see `heritage_known = true` and fire — but `0x222` is defined AFTER `0x220` and `0x220` is `one_shot: true`, so this is a clean cascade if `0x222` is also `one_shot: true`. **Verify in tests.**
- **Bounty Hunter spawn idempotency:** the chain has a `Random { 0.15 }` precondition; once `0x212` is `EventFired`, the spawn cannot re-fire (one_shot). However, if the player loads an older save where the chain fires fresh on a system that already has bounty hunters... the chain itself is one_shot per game so this is fine. **Save-load round-trip test required.**
- **`incident_flags` clearing across save/load:** the bit lives on `world::System` so it persists. If the bit is set at save time but the event hasn't fired yet, load → next economy tick → fires correctly.
- **`frozen_permanently` and the escape chain:** the existing escape chain must check `EventCondition::CharacterFlag { character: han, flag: FrozenPermanently, expected: false }` to short-circuit further escape rolls.
- **Cutscene state on save/load:** save is locked while in `Cutscene` or `VictoryModal` mode (#V5), so this risk is closed by construction.

### API Surface Parity

- **Two `apply_event_actions` copies** are consolidated to one in Phase 1 (#S1). After Phase 1, every new `EventAction` variant lands in exactly one place. **This is the highest-leverage debt fix in the sprint.**
- **`EventAction::SpawnSpecialForce` and `SetCharacterFlag`** must be handled in (a) the consolidated `apply_event_action_to_world`, (b) the WASM telemetry path if any. Verify with grep.
- **`VideoPlayer::open` signature change** (adds `volume: f32` parameter) propagates to the WASM stub at `video_player.rs:404-431`. The stub takes the parameter but ignores it.

### Integration Test Scenarios

1. **Full Jabba chain end-to-end:** seed a campaign with Han alive, advance ticks until `0x212` fires, assert `SpecialForceUnit` exists at Han's system; advance until Han is captured (`0x395` or random rolls), assert `0x383`/`0x384`/`0x385`/`0x387`/`0x399`/`0x39B` paths each fire under their respective preconditions in independent runs. Each path emits exactly one `EVT_JABBA_PRISONERS` consolidator.
2. **Final Battle heritage gate:** seed a campaign where Luke, Vader, Emperor end up co-located. Fresh save (no prior `0x394`/`0x395`): `0x220 EVT_FINAL_BATTLE_ULTIMATUM` fires with `EVENT_VADER_VS_STUDENT_LUKE` BMP; `heritage_known` flips to true. In a second co-location later in the same campaign (after a flee/regroup), `0x222 EVT_FINAL_BATTLE_KNOWN` fires with `EVENT_EMPEROR_AND_VADER_VS_KNIGHT_LUKE` BMP.
3. **Save/load round-trip across the Bounty Hunters chain:** save mid-chain, exit, reload, continue advance, assert chain completes correctly without re-spawning the bounty hunter unit.
4. **Cutscene → VictoryModal full flow:** trigger an HQ-capture victory, assert `VictoryModal` opens, ESC, assert cutscene starts, ESC again, assert MainMenu. Repeat with the cutscene assets in `NotDecoded` state and assert the same flow with a "graceful skip" log line.
5. **Audio volume during cutscene:** start a cutscene at master=1.0, drag the music slider to 0.0 mid-playback, assert the next frame's `set_volume(0.0)` is called.

## Acceptance Criteria

### Functional Requirements

- [ ] **#S1** `apply_event_actions` is a single function exported from `rebellion-data::integrator`. main.rs is a thin wrapper. Test asserts byte-equivalence on a synthetic action stream.
- [ ] **#S2-#S6** All Phase 1 type changes compile, all 417 existing tests pass.
- [ ] **#S7** `is_story_event` matches `0x128 | 0x200 | 0x222 | 0x230 | 0x231 | 0x306 | 0x361 | 0x386 | 0x387 | 0x39B` in addition to the existing range.
- [ ] **#S8** `event_rolls` budget is ≥32 OR dynamically sized.
- [ ] **#S9** `BOUNTY_HUNTERS_DAT_ID = DatId::new(0xFF000001)` defined.
- [ ] **#S10** CI grep guard fails on a deliberate offender, passes on the real codebase, runs as part of `cargo test`.
- [ ] **#S11** Stale comment at `story_events.rs:1316` removed or corrected.
- [ ] **#K1-#K6** All 6 economy/manufacturing notification events emit on the correct trigger, with idempotency tests passing.
- [ ] **#R1-#R11** All 11 story-track and mission/betrayal/death-star events register and emit correctly. Tests for each.
- [ ] **#R12** Emperor Palpatine combat-winner check applies a 1.5× multiplier (or chosen value) to the Emperor's faction in resolved combats where Emperor is co-located.
- [ ] **#R13** All new tests pass; total test count ≥ 470.
- [ ] **#A1-#A4** Integrator emission arms for all 3 new `GameEffect` variants.
- [ ] **#A5** `scripts/golden_values.json` has a populated `story_events` section with locked counts (±10% tolerance) for at least 14 events.
- [ ] **#A6** `scripts/eval_parity.py` reports pass/fail for the new section.
- [ ] **#A7** Telemetry tag distribution snapshot committed under `agent_docs/research/2026-04-08-pre-shamash-bet-telemetry-baseline.json`.
- [ ] **#V1-#V10** Cutscene state machine matches `Galaxy → VictoryModal → Cutscene → MainMenu`. Save lock-out enforced. Audio volume bridge functional. Headless smoke test passes.
- [ ] **#N1-#N4** Phase 6a research artifact committed at `agent_docs/research/2026-04-08-advisor-bin-variant-corpus.md`.
- [ ] **#N5-#N8** Phase 6b decoder lifts BIN parse rate from ~24% to ≥50% across both factions. BIN→BMP mapping uses discovered offset, falls back to modulo only for unmapped IDs.

### Non-Functional Requirements

- [ ] Zero new Clippy warnings (`PATH="/usr/bin:$PATH" cargo clippy --workspace -- -D warnings`).
- [ ] Zero `cargo check` failures across all targets.
- [ ] Total test count rises from 417 to ≥470.
- [ ] WASM build succeeds (`bash scripts/build-wasm.sh`).
- [ ] No new external dependencies.
- [ ] Save format stays at v7 (no migration).
- [ ] No changes to `tools/dat-dumper/` or DAT round-trip validation.
- [ ] `rebellion-playtest --seed 42 --ticks 5000` runs deterministically with the new emissions and matches the new `story_events` golden values within tolerance.

### Quality Gates

- [ ] All three pre-existing reviewer agents (`compound-engineering:review:dhh-rails-reviewer` style — adapted to Rust via `kieran-rust-reviewer` if available, otherwise `code-reviewer`) review each dabora's diff.
- [ ] `compound-engineering:review:performance-oracle` review on Dabora 2 (economy tick changes).
- [ ] `compound-engineering:review:data-integrity-guardian` review on Dabora 1 (`Character` field additions).
- [ ] All BLOCKING and HIGH lint violations from `~/.claude/skills/test-harness-auditor`'s lint pack resolved.
- [ ] Documentation updates land in the same PR as code:
  - `CLAUDE.md` "Knesset Shamash-Bet — COMPLETE" section added under "UI Rebuild" or as its own ordered entry.
  - `agent_docs/roadmap.md` updated to mark all Track A/B items as DONE.
  - `program.md` updated if eval criteria change.

## Success Metrics

- **Story event coverage:** 6 partial → 0 partial. 9 missing notification events → 9 emitting. Overall Events & Story parity 70% → ≥90%.
- **Combat parity:** 99% → 99.5% (Emperor Palpatine check is the only outstanding combat parity item from the cross-reference report).
- **Cutscene UX completeness:** 1 of 4 closing gaps → 4 of 4 closed.
- **Advisor BIN parse rate:** ~24% → ≥50% (target locked after Phase 6a research).
- **Test count:** 417 → ≥470.
- **Golden value oracle coverage:** 0 story events → 14 locked events.
- **Code debt:** dual `apply_event_actions` → consolidated.

## Dependencies & Prerequisites

- **None external.** Every input is in-repo.
- **`scripts/decode-cutscenes.sh`** must be runnable for any human QA testing of Dabora 5 (CI uses the `NotDecoded` graceful-skip path).
- **Dabora 1 must merge before Daborot 2-5.** Dabora 6a runs in parallel with everyone.
- **No new Cargo crates** introduced.

## Risk Analysis & Mitigation

| Risk | Severity | Mitigation |
|---|---|---|
| Re-introduction of `EventCondition::Random` for notification events | HIGH (regresses Knesset Shamash lesson) | CI grep guard #S10 fails the build if any of the 8 banned constants appear within 5 lines of `Random` in `story_events.rs`. |
| `apply_event_actions` drift between integrator and main.rs | HIGH (load-bearing across the entire sprint) | Phase 1 #S1 consolidates to one function. Test asserts byte-equivalence on synthetic action stream. |
| Golden-value tag-distribution shift breaks `eval_parity.py` | MEDIUM | Snapshot OLD distribution before #S7 lands (#A7). Diff is intentional and documented in merge commit. |
| `quad_snd::Playback` does not expose runtime volume | MEDIUM | Pre-implementation research subtask in #V7. Documented audible-glitch fallback (stop+restart) if absent. |
| Advisor BIN variant decoder reveals more than one variant | MEDIUM | Cascading parser pattern (`try_v1 → try_v2 → try_vN`); each variant lands as its own function. Acceptance threshold (≥50%) is conservative. |
| BIN→BMP offset discovery yields no clean offset | MEDIUM | Two-fallback strategy: faction-specific offsets, then modulo for unmapped IDs. Logged per-faction. |
| `Character::heritage_known` `#[serde(default)]` breaks save v7 | LOW | bincode handles `#[serde(default)]` correctly via field-by-field deserialization. Test: load a save v7 file pre-sprint, assert `heritage_known == false`. |
| Same-tick chaining of `0x220 → 0x222` mis-fires | MEDIUM | `one_shot: true` on both. Test: tick once with all three co-located, assert exactly one of the two fires; advance state, force `heritage_known = true`, tick again, assert `0x222` fires. |
| Bounty Hunter spawn race when Han is in transit | LOW | Handler walks `world.movement_orders` for the active order with `character == han`; logs and skips on no match. |
| `EVT_JABBA_PRISONERS` consolidator double-fires on simultaneous captures | LOW | Define AFTER captures with `EventNotFired { id: EVT_JABBA_PRISONERS }` self-guard. Test: synthetic state with two captures same tick. |
| `event_rolls` budget overflow when adding new `Random` conditions | LOW | #S8 bumps to 32 (sufficient ceiling). Add `debug_assert!(state.required_random_rolls() <= rolls.len())` at the top of `EventSystem::advance`. |
| `cleanup_destroyed_system` removes a character mid-`apply_event_actions` | MEDIUM | Death Star cleanup runs in its own integrator phase before any Phase 2-5 events. Verify phase ordering in integrator. |
| Telemetry consumers (autoresearch tracks) break on new `is_story_event` filter | MEDIUM | #A7 snapshot + intentional documented diff. Autoresearch baseline regenerated post-merge. |
| Dabora 3 + Dabora 6 conflict on `rebellion-render` | LOW | Strict merge ordering: Dabora 6b lands LAST. Dabora 3 doesn't touch rebellion-render at all. |
| WASM `localStorage` quota overflow from larger `EventState` | LOW | Add save size guard in `crates/rebellion-data/src/save.rs::save_slot` that warns if payload exceeds 4 MB. |
| Player skips cutscene with no input handler defined | LOW | Existing ESC/SPACE handlers from Knesset Kothar wa Khasis stay in place; #V4 confirms post-cutscene transition fires regardless of skip-vs-finish. |

## Resource Requirements

- **Team:** 6 daborot (1 lead + 5 parallel) via the `minoan-swarm` skill.
- **Models:** Dabora 1, 3, 4, 5, 6 use opus (complex coordination, type design, RE pattern recognition); Dabora 2 uses sonnet (high-throughput pattern porting).
- **Infrastructure:** local Mac Mini M4 / Tom's MBP. No remote compute. Mycelium spores for shared knowledge across daborot.
- **Testing:** `cargo test --workspace`, `bash scripts/build-wasm.sh`, `cargo run -p rebellion-playtest -- --seed 42 --ticks 5000 --jsonl > /tmp/baseline.jsonl` for golden-value lock.

## Future Considerations

After this sprint lands, the remaining backlog from `agent_docs/roadmap.md` and the community cross-reference report becomes:

- **Knesset N+1 candidate — Story Cutscenes:** wire 101-108 from `agent_docs/game-media.md:67-81` to story event triggers.
- **Knesset Nabu — Advisor BIN deep RE:** GhidraMCP session against the advisor engine to fully decode any remaining BIN format variants beyond what Phase 6a reveals.
- **Knesset N+2 — Per-character `MissionKind::PalaceRescue`:** add real mission types for Leia/Luke/Chewbacca palace rescues (currently `SetMandatoryMission` flag-only).
- **Knesset Nergal — AI parity closeout:** port the remaining 8 of 18 dispatch validators, AI resource rebalancing (`FUN_00558660`), Reconnaissance mission variant (`0x54`), Death Star multi-target selection.
- **Knesset Marduk — Multiplayer foundation:** port the 178 `net_notify_*` message types from the cross-reference report.
- **Addon 1 — Create Your Own Character (Demiurgos):** 5-phase plan from `docs/plans/2026-03-21-addon-create-your-own-character.md`.
- **Release packaging:** itch.io web release, Homebrew formula, GitHub Releases.

## Documentation Plan

- **`CLAUDE.md`** — add "Knesset Shamash-Bet — COMPLETE" section under the UI Rebuild block, mirroring the format of "Knesset Kothar wa Khasis — COMPLETE".
- **`agent_docs/roadmap.md`** — mark all Track A/B items as DONE; update parity counters (Combat 99% → 99.5%, Events & Story 70% → ≥90%).
- **`program.md`** — note new `story_events` section in golden_values.json under "Components".
- **`agent_docs/research/2026-04-08-advisor-bin-variant-corpus.md`** — Phase 6a artifact (BIN format research).
- **`agent_docs/research/2026-04-08-pre-shamash-bet-telemetry-baseline.json`** — Phase 1 #A7 artifact (telemetry tag distribution snapshot).
- **`docs/reports/2026-04-NN-knesset-shamash-bet-report.md`** — post-sprint summary, mirroring Knesset Resheph and Knesset Kothar wa Khasis reports.

## Sources & References

### Origin

This plan does NOT have a `docs/brainstorms/*-requirements.md` origin document. It was assembled from the broader Open Rebellion backlog enumerated on 2026-04-07 in response to Tom's request to plan "Story and Events sweep + Advisor bin and cutscene polish." The selection was made via `AskUserQuestion` in the planning session.

### Internal References

#### Track A — Story & Events

- `crates/rebellion-core/src/events.rs:67-102` — all event ID constants (existing).
- `crates/rebellion-core/src/events.rs:113-313` — `EventCondition` and `EventAction` enums.
- `crates/rebellion-core/src/story_events.rs:25-1402` — `define_story_events` registry, including all 4 partial story chains and the test fixtures.
- `crates/rebellion-core/src/story_events.rs:689-702` — **CRITICAL prior-art comment** on the discarded random-notification approach.
- `crates/rebellion-core/src/story_events.rs:1316` — stale Leia 0x362→0x363 comment to clean up.
- `crates/rebellion-core/src/effects.rs:32-236` — `EffectPhase` ordering and `GameEffect` closed enum.
- `crates/rebellion-core/src/world/mod.rs:469-554` — `Character` struct (27 existing fields).
- `crates/rebellion-core/src/world/mod.rs:894-899` — `SpecialForceUnit` struct.
- `crates/rebellion-core/src/world/mod.rs:971` — `special_forces` slotmap arena.
- `crates/rebellion-core/src/economy.rs` — economy tick (Phase 3b post-Shamash baseline).
- `crates/rebellion-core/src/manufacturing.rs` — manufacturing system.
- `crates/rebellion-core/src/missions.rs` — mission resolution.
- `crates/rebellion-core/src/betrayal.rs` — `BetrayalEvent::CharacterBetrayed`.
- `crates/rebellion-core/src/death_star.rs::cleanup_destroyed_system` — Knesset Ptah cleanup pass.
- `crates/rebellion-core/src/combat.rs` — combat resolver (`FUN_00542050` insertion site).
- `crates/rebellion-core/src/game_events.rs:67-130` — telemetry string constants for every event ID.
- `crates/rebellion-data/src/integrator.rs:128-760` — `PerceptionIntegrator::apply_*` methods.
- `crates/rebellion-data/src/integrator.rs:471-495` — `apply_fired_events`.
- `crates/rebellion-data/src/integrator.rs:910-982` — `apply_event_actions_to_world_inner` (consolidation target).
- `crates/rebellion-data/src/integrator.rs:1310-1314` — `is_story_event` filter (Phase 1 #S7).
- `crates/rebellion-app/src/main.rs:935-1014` — interactive event firing + screen overlay.
- `crates/rebellion-app/src/main.rs:936` — `event_rolls` budget (Phase 1 #S8).
- `crates/rebellion-app/src/main.rs:3271-3409` — duplicate `apply_event_actions` (consolidation target).
- `crates/rebellion-render/src/event_screen.rs:67-109` — `event_id_to_resource()` STRATEGY.DLL offset table.
- `crates/rebellion-render/src/bmp_cache.rs:178-319` — complete BMP catalog with named resource constants.
- `crates/rebellion-render/src/message_log.rs:35-74` — `MessageCategory` enum.

#### Track B — Advisor BIN + Cutscene

- `crates/rebellion-render/src/advisor.rs:104-171` — `parse_advisor_bin` and `BinError` (24% success branch).
- `crates/rebellion-render/src/advisor.rs:178-217` — `AdvisorState` and band-priority sequence picker.
- `crates/rebellion-render/src/advisor.rs:486-596` — `load_faction_frames` and the modulo BIN→BMP shortcut.
- `crates/rebellion-render/src/advisor.rs:557-593` — per-faction parse summary log.
- `crates/rebellion-render/src/video_player.rs:88-92` — `AudioTrack` (owned `AudioContext`).
- `crates/rebellion-render/src/video_player.rs:106-136` — `VideoPlayer::open`.
- `crates/rebellion-render/src/video_player.rs:322-347` — hardcoded `volume: 1.0`.
- `crates/rebellion-render/src/video_player.rs:404-431` — WASM stub.
- `crates/rebellion-render/src/victory_screen.rs:32-74` — dead-coded `VictoryScreenState` (wire-up target).
- `crates/rebellion-render/src/audio.rs:133-147` — `AudioVolumeState`.
- `crates/rebellion-render/src/lib.rs:66` — `victory_screen` re-export.
- `crates/rebellion-app/src/main.rs:74-76` — `INTRO/VICTORY/DEFEAT_CUTSCENE` path constants.
- `crates/rebellion-app/src/main.rs:481-490` — intro cutscene open call site.
- `crates/rebellion-app/src/main.rs:1475-1525` — `GameMode::Cutscene` arm and victory→cutscene transition.
- `crates/rebellion-app/src/main.rs:3705-3729` — `open_cutscene` helper.
- `crates/rebellion-app/src/audio.rs:449-454` — `AudioEngine::apply_volume` (precedent for `set_volume`).
- `assets/references/ref-ui/07-droid-advisors/{alliance,empire}/*.bin` — research corpus.
- `assets/references/ref-ui/07-droid-advisors/{alliance,empire}/*.bmp` — sorted BMP pool with resource ID prefixes.
- `scripts/build-resource-entity-map.py` — brute-force offset-discovery technique to mirror.
- `scripts/decode-cutscenes.sh` — cutscene pre-decode entry point.

#### Cross-cutting

- `CLAUDE.md` — Knesset Kothar wa Khasis "Known gaps" section, effect-only output rule, two-layer types, build prefix, integrator pattern.
- `agent_docs/architecture.md` — simulation loop, integrator pattern, type system layers.
- `agent_docs/roadmap.md` — full backlog, Knesset Resheph + Kothar wa Khasis closing notes.
- `agent_docs/dll-resource-catalog.md:106-146` — ALSPRITE/EMSPRITE inventory (74 BMPs + 716/713 BIN per faction).
- `agent_docs/game-media.md:67-81` — 8 unwired story event cutscenes (out of scope, follow-up).
- `agent_docs/seeding.md` — Coruscant/Yavin/Rebel HQ identity (relevant to `0x230` Coruscant gating if pursued as a stretch).
- `docs/reports/2026-03-26-community-disassembly-cross-reference.md:201-247` — canonical Events & Story gap inventory and `FUN_00542050_is_emperor_palpatine` reference.
- `docs/plans/2026-03-26-001-feat-eval-driven-parity-open-souls-refactor-plan.md:116, 167, 548-554` — Leia 0x362→0x363 collision history, `apply_event_actions` debt callout, "Bug 2: Notification Events Use Wrong Model" lesson.
- `docs/plans/2026-03-28-001-feat-knesset-ptah-todo-resolution-plan.md:135-163` — `cleanup_destroyed_system` design.
- `docs/plans/2026-04-03-001-feat-knesset-resheph-final-sprint-plan.md` — task #132 lineage, golden-oracle pattern from Sassuratu brief, 10-task prior sprint context.
- `scripts/golden_values.json` — current oracle (no `story_events` section yet).
- `scripts/eval_parity.py` — parity check entry point.
- `scripts/parity_tasks.json:84-91` — original task #132 acceptance criteria for the BIN format work.

### Related Work

- **Knesset Resheph** (2026-04-03) — 10-task final sprint that delivered the previous parity wave; deferred 4 tasks closed by Knesset Kothar wa Khasis (2 of 4) and now this sprint (the remaining 2 + the broader story sweep).
- **Knesset Kothar wa Khasis** (2026-04-07) — closed U2 (cutscene playback) + C1 (advisor BIN simple format).
- **Knesset Ereshkigal Phase 3** (2026-03-26 → 2026-03-27) — added story event constants and partial chains, attempted random notification events (discarded in Phase 3b).
- **Knesset Shamash Phase 3b** — discarded the random notification approach, left a comment as the load-bearing prior-art warning.
- **Knesset Hokhmah** — added the 8 story event chains in `0x390-0x39A`.
- **Knesset Ma'at** — combat formula corrections (shield absorption, fighter combat).
- **Knesset Athirat** — original Living Galaxy + War Room delivery.

---

*Plan authored 2026-04-08 by Claudicle, Artifex Maximus, against the Open Rebellion backlog enumerated 2026-04-07. Deepened 2026-04-08 via 6-agent parallel review producing 41 findings. Sprint name: Shamash-Bet. Originally six daborot / ~25 sub-tasks / ~3,050 LOC; after deepening: **five daborot / ~22 sub-tasks / ~1,750 LOC**. Completes Track A (story sweep) and Track B (advisor + cutscene polish) in a single coordinated knesset.*

---

# Research Insights — Section Amendments (2026-04-08 deepening)

The following amendments are the authoritative task specification. Where they conflict with earlier sections of this plan, these win. Each finding is tagged by source reviewer (ARCH/PERF/DI/SF/SIMP/QSND) and severity.

## Amended Phase 1 Task List (Dabora 1 — Foundation)

Phase 1 shrinks from ~600 LOC to ~150 LOC. The original tasks #S1–#S11 are superseded by this list.

- **#F1 — Save version bump v7 → v8.** *(ARCH-#8, DI-C1)* First task in the sprint, day zero. Bump `SAVE_VERSION` in `crates/rebellion-data/src/save.rs`. Add a v7 rejection arm in `load_slot()` mirroring v4/v5/v6 with message *"save version 7 is incompatible with this build (Character gained `heritage_known`; SaveState gained `economy`). Please start a new game."* Bump WASM localStorage meta key prefix from `rebellion_meta_` to `rebellion_meta_v8_` so stale browser entries get rejected cleanly. Update CLAUDE.md "Save v8 format" line and `agent_docs/architecture.md` save/load flow. **Acceptance:** existing v7 saves load as "rejected: version mismatch" with a clean error, not a corrupted deserialize.
- **#F2 — `economy: EconomyState` added to `SaveState`.** *(DI-H1)* Bundles into the v8 bump. Closes the pre-existing bug where post-load incidents silently re-fire because `SystemEconomy.incident_flags` was rebuilt from defaults. **Acceptance test:** save mid-disaster, reload, advance one tick, assert `EVT_NATURAL_DISASTER` does NOT re-emit.
- **#F3 — `Character::heritage_known: bool`** added to `world::Character` (no `#[serde(default)]` — inert under bincode). Also add `impl Default for Character` so new construction sites don't have to enumerate all fields (DI-M2). **Acceptance:** `cargo check` clean across the workspace; every existing `Character { ... }` literal in `seeds.rs` and test fixtures uses `..Default::default()` or explicit `heritage_known: false`.
- **#F4 — New `GameEffect` variants** (ARCH-#3 + DI-H3 phase fix):
  - `SpecialForceSpawned { at_system: SystemKey, is_alliance: bool }` — `EffectPhase::Command`.
  - `StoryMessageDisplayed { text: String, category: MessageCategoryTag }` — `EffectPhase::Command`. Introduces a bare `MessageCategoryTag` enum in `rebellion-core::effects` (no rebellion-render dep — the render layer maps the tag to its own `MessageCategory` color table). Closes ARCH-CRITICAL-#2 (message log layering).
  - **Explicitly NOT added:** `CharacterFlagSet`, `SystemIncidentFired`, new `CharacterKilled` variant. The first two are deleted per SIMP-H1/H4. The third already exists at `effects.rs:148` — reuse it (ARCH-CRITICAL-#1 + DI-H3).
- **#F5 — New `EventAction::SpawnSpecialForce { at_character: CharacterKey }`** *(ARCH-#4 + DI-H2 + SIMP-H6 + SF-#13)*. Note: field is `at_character`, NOT `at_system`. The action emits `GameEffect::SpecialForceSpawned` via the integrator, which resolves the system at fire time by reading `han.current_system` — falling back to `han.current_fleet.and_then(|f| movement.orders().get(&f)).map(|o| o.destination)` for the in-transit case. No `class_dat_id` field (SIMP-H6 YAGNI), no `0xFF000001` sentinel (ARCH-#4 burns the user-mod range). The integrator's `apply_special_force_spawn` constructs a `SpecialForceUnit` via a hardcoded `bounty_hunters()` constructor with `is_alliance: false` (aligned to Imperial assets per original game citation from cross-reference report; documented as best-available parity).
- **#F6 — `SystemTag` field on `GameEvent`** *(ARCH-#6)* replacing the `is_story_event` filter entirely. Enum variants: `Events`, `Story`, `Notification`. `state.define()` sets it explicitly. Integrator routing at `integrator.rs:1310` reads `fired.system_tag`, not a pattern match on `event_id`. Update every existing `state.define()` call in `story_events.rs` to carry the tag. Delete the `is_story_event` function. Misrouting becomes a type error at the define site. This also kills the internal #S7 inconsistency (ARCH-#10, now moot).
- **#F7 — `apply_event_actions` consolidation via `pub` visibility.** *(ARCH-CRITICAL-#2 + SIMP-M1 combined)*. Mark `apply_event_actions_to_world_inner` at `crates/rebellion-data/src/integrator.rs:910` as `pub` under the name `apply_event_action_to_world`. Add `#[inline]` (PERF-#10). Delete `crates/rebellion-app/src/main.rs:3271-3409` (the duplicate). main.rs line 941 becomes `apply_event_action_to_world(&fired.actions, &mut world, &mut effects_out, fired.tick);`. Inside the pub'd function, `EventAction::DisplayMessage { text, category }` now pushes `GameEffect::StoryMessageDisplayed { text, category }` into the effect queue instead of no-op. main.rs's per-tick post-processing drains story-message effects and pushes them into its `MessageLog`. `simulation.rs` drops them silently (headless). **No new helper extraction, no new trait, no byte-equivalence test** — the compiler's exhaustiveness check over the closed `EventAction` enum covers variant coverage for free.
- **#F8 — `event_rolls` unwrap_or(1.0) silent-failure fix.** *(SF-CRITICAL-#9)* At `crates/rebellion-core/src/events.rs:553`, replace `let roll = rng_rolls.get(*rng_cursor).copied().unwrap_or(1.0);` with `.expect("event_rolls budget exhausted")`. The `expect` panics in release builds (not `debug_assert!`). This sprint adds zero new `Random` conditions (per the banned-notification invariant), so the existing 16-slot budget stays. The fix protects every future sprint from silent Random never-fires poisoning autoresearch signal. ~2 LOC change.
- **#F9 — CI grep guard as a Rust `#[test]` inside `story_events.rs`.** *(SIMP-M3 + DI-L3)* Walks `state.events()` and asserts no event with ID in `[0x100, 0x153, 0x154, 0x155, 0x160, 0x304, 0x305, 0x361]` contains `EventCondition::Random`. No shell script. Runs on every `cargo test`. ~20 LOC.
- **#F10 — Save-panel lockout at all three entry points.** *(SF-#10)* Add `fn save_panel_gated(mode: &GameMode) -> bool { matches!(mode, GameMode::Cutscene | GameMode::VictoryModal { .. }) }` and apply at `main.rs:579` (Cmd+S hotkey), `main.rs:2011` (`CockpitButton::SaveLoad`), and `main.rs:2590` (`PanelAction::OpenSaveLoad`). Belt-and-suspenders check inside `save_slot()` in `rebellion-data/src/save.rs` refuses to serialize when `game_mode` is a locked variant. `log::warn!("save blocked: current game mode is {:?}", mode)` on any blocked attempt.

**Deleted from the original Phase 1** (~450 LOC saved):
- ~~#S1 extract a shared helper~~ → #F7 uses `pub` visibility instead.
- ~~#S2 `EventAction::SetCharacterFlag` + `SpawnSpecialForce` with class_dat_id~~ → #F5 (`SpawnSpecialForce` only, no class id).
- ~~#S3 `EventCondition::CharacterFlag`~~ → deleted entirely (SIMP-L4, SF-#6 moot).
- ~~#S4 `frozen_permanently: bool` + `CharacterFlag` enum~~ → deleted entirely (SIMP-H1). `heritage_known` stays in #F3.
- ~~#S5 `incident_flags: u8` + `last_support_tier: u8` on System~~ → deleted (SIMP-H3/H4). Intra-tick scratch replaces.
- ~~#S6 `GameEffect::CharacterFlagSet` + `SystemIncidentFired`~~ → deleted. Only `SpecialForceSpawned` + `StoryMessageDisplayed` survive.
- ~~#S7 `is_story_event` filter expansion~~ → replaced by #F6 `SystemTag` field.
- ~~#S8 `event_rolls` budget bump~~ → moot (no new Randoms). Replaced by #F8.
- ~~#S9 `BOUNTY_HUNTERS_DAT_ID` sentinel~~ → deleted (SIMP-H6).
- ~~#S10 shell script CI guard~~ → replaced by #F9 Rust test.
- ~~#S11 stale comment~~ → folded into Dabora 3.

## Amended Phase 2 — Merged Dabora 2/4 "Kothar-Anat" (Economy + Manufacturing + Victory + Integrator + Golden)

*(SIMP-M5+L3 dabora collapse.)* Owns: `economy.rs`, `manufacturing.rs`, `victory.rs`, integrator emission arms, `scripts/golden_values.json`, `scripts/eval_parity.py`.

All 6 notification events emit via **direct `self.emit(SYS_ECONOMY, EVT_*, json)` calls at the point of decision** inside their owning system's `advance()`, using intra-tick scratch for previous-value comparisons. No persistent fields in world types.

- **#K1 — `EVT_SUPPORT_CHANGE` (0x100).** Compute prev/new tier inside `EconomySystem::advance`. Emit if different. Zero new world state.
- **#K2 — `EVT_NATURAL_DISASTER` (0x154).** Direct emit at decision branch. Clear-before-emit ordering if any state flip (SF-#8 — clearing after risks infinite re-fire on panic).
- **#K3 — `EVT_RESOURCE_DISCOVERY` (0x155).** Same pattern.
- **#K4 — `EVT_MAINTENANCE_SHORTFALL_EVENT` (0x304).** Per-faction 30-tick timer (`GNPRTB[7694]`) as intra-tick scratch on `EconomySystem` itself.
- **#K5 — `EVT_UNITS_DEPLOYED` (0x107).** Emit from `ManufacturingSystem::advance` on `BuildCompletion` arrival.
- **#K6 — `EVT_MANUFACTURING_IDLE` (0x160).** Intra-tick prev/new queue length comparison.
- **#K7 — Tests.** ONE parameterized idempotency test per pattern (SIMP-L1).
- **#A1 — `EVT_HQ_CAPTURED` (0x128)** from `apply_victory`.
- **#A2 — Integrator arm for `SpecialForceSpawned`.** Resolves at-system via character lookup (DI-H2 correct data structure). Structured `log::warn!` on resolution failure. Gate the `EVT_BOUNTY_ATTACK` chain on `CharacterAtSystem OR CharacterHasActiveMovementOrder` so failure never fires (SF-#7).
- **#A3 — Integrator arm for `StoryMessageDisplayed`.** Pushes onto effect queue; main.rs drains post-tick into MessageLog.
- **#A4 — `scripts/golden_values.json` `story_events` section.** `{min, max}` bounds, NOT `{expected, tolerance}` (DI-M4). Counts via `PATH="/usr/bin:$PATH" cargo run -p rebellion-playtest -- --seed 42 --ticks 5000 --jsonl > baseline.jsonl` + histogram script (DI-M5).
- **#A5 — `eval_parity.py` grep + regeneration.** Before #F6 lands, grep for hardcoded `SYS_EVENTS`/`SYS_STORY` assertions. Regenerate as part of merge commit. Immediate fix if CI red (SF-#12).

Telemetry baseline snapshot: inline code block in Dabora 1 merge commit (SIMP-H7), no separate JSON file.

## Amended Phase 2 — Dabora 3 "Resheph" (Story chains + Missions + Betrayal + Death Star)

Owns `story_events.rs`, `missions.rs`, `betrayal.rs`, `death_star.rs`. Does NOT own `combat.rs` — Emperor Palpatine check cut (SIMP-H2), deferred to Knesset Nergal.

- **#R1 — `EVT_HAN_RESCUE` (0x200)** alongside existing 0x383 via `EventFired`.
- **#R2 — `EVT_JABBA_PRISONERS` (0x231)** consolidator. Three OR-branch variants (one per capture ID). Defined AFTER captures with self-guard. Acceptance tests (SF-#5): (a) two captures same tick → exactly one consolidator; (b) each capture ID triggers independently (parameterized); (c) self-guard prevents re-fire.
- **#R3 — `EVT_HAN_PERMANENT_FREEZE` (0x39B)** using EXISTING primitives only: `is_captive` + `EventFired { EVT_HAN_CARBONITE_FAIL_5 }`. No new fields, no new action variants (SIMP-H1).
- **#R4 — Final Battle heritage gate via RENDER LAYER, not event split.** *(SIMP-H5 + ARCH-#9 + PERF-#3 + SF-#11 triple-collapse.)* KEEP the single `0x220 EVT_FINAL_BATTLE`. DO NOT create `0x222`. Audit `story_events.rs:215-269`: `0x396 "Final Battle Imminent"` already narrates "father and son" — its action flips `heritage_known = true`. Also audit `0x394`/`0x395` during implementation. In `crates/rebellion-render/src/event_screen.rs:67-109`, `event_id_to_resource()` branches on `character.heritage_known` for Luke to pick between `EVENT_VADER_VS_STUDENT_LUKE` (1059) / `EVENT_VADER_VS_KNIGHT_LUKE` (1060) / `EVENT_EMPEROR_AND_VADER_VS_KNIGHT_LUKE` (1064). **Kills**: ARCH-#9 cascade risk, PERF-#3 double-walk, the formerly-required predicate cache (unnecessary because only one event walks the predicate).
- **#R5 — Bounty Hunters real spawn.** Wire existing `EVT_BOUNTY_ATTACK` (0x212) to add `EventAction::SpawnSpecialForce { at_character: han }`. Combined with SF-#7 precondition: add `CharacterExists { han } AND (CharacterAtSystem { han, .. } OR CharacterHasActiveMovementOrder { han })`. May require new `CharacterHasActiveMovementOrder` EventCondition variant.
- **#R6 — `EVT_INFORMANT_INTEL` (0x153)** in `missions.rs`.
- **#R7 — `EVT_SABOTEUR_DETECTED` (0x305)** in `missions.rs`.
- **#R8 — `EVT_CHARACTER_HEALTH` (0x106)** in `missions.rs`.
- **#R9 — `EVT_TRAITOR_REVEALED` (0x361)** in `betrayal.rs` reveal-before-flip.
- **#R10 — `EVT_SIDE_CHANGE` (0x386)** in `betrayal.rs` actual flip. Payload uses `dat_id` or `name`, NOT `CharacterKey` (DI-H2).
- **#R11 — `EVT_CHARACTER_KILLED` (0x306).** *(ARCH-CRITICAL-#1.)* `cleanup_destroyed_system()` gains `&mut Vec<GameEffect>` out-parameter. Emits the EXISTING `GameEffect::CharacterKilled` at `effects.rs:148`. Caller in `simulation.rs:280-284` drains. Also emit from `missions.rs` assassinations. **Documented:** character-death-triggered story events are STRICTLY next-tick reactive (Events=step 6, Death Star cleanup=step 11). Payload uses `dat_id` or `name`. Uniqueness via `Character::is_killed` flag, not `fired_ids` suffixing (DI-M3).
- **#R12 ~~Emperor Palpatine combat check~~ DELETED** (SIMP-H2). Deferred to Knesset Nergal.
- **#R13 — Stale comment at `story_events.rs:1316`** folds here (SIMP-L2).
- **#R14 — Tests.** ~30 new tests not ~53. `make_world_with_characters()` fixture. Final Battle heritage branch test. Three specific `EVT_JABBA_PRISONERS` consolidator tests from SF-#5.

## Amended Phase 3 — Dabora 4 "Kothar wa Khasis" (Cutscene chain)

Renumbered from Dabora 5. Owns `victory_screen.rs`, `video_player.rs`, `main.rs` (state machine).

- **#V1 — `VictoryScreenState` wired into main.rs** (new field, populate on victory).
- **#V2 — `GameMode::VictoryModal { outcome }`** new variant. Keep `GameMode::Cutscene` as unit variant (SIMP-M4) and use `VideoPlayer`'s existing path field to distinguish intro vs. victory/defeat. Transition-in reads `matches!(prev_mode, VictoryModal { .. })`. Intro: `Cutscene(INTRO) → MainMenu`. Victory/defeat: `VictoryModal → Cutscene → MainMenu`.
- **#V3 — Post-cutscene transition logic** driven by path inspection.
- **#V4 — Save lock-out at all three entry points** — #F10 (Dabora 1 lands helper; Dabora 4 ensures `VictoryModal` variant matches).
- **#V5 — `VideoPlayer::set_volume(&mut self, vol: f32) -> bool`** *(QSND verified)*. Rename `_sound → sound` at `video_player.rs:91`. Implementation: `if let Some(audio) = self.audio.as_mut() { audio.sound.set_volume(&audio.ctx, vol); true } else { log::debug!(...); false }`. Returns bool per SF-#14. WASM no-op stub returns `true`. NO buffered fallback, NO glitch docs. `Sound::set_volume` works in-flight — cited at `audio.rs:449-454`.
- **#V6 — `VideoPlayer::open(path, volume: f32)`** signature. Default callers pass `audio_vol.master_volume * audio_vol.music_volume`.
- **#V7 — main.rs audio bridge.** While in `GameMode::Cutscene`, if `audio_vol.dirty`: `if video_player.set_volume(effective_vol) { audio_vol.dirty = false; }` — dirty clears ONLY on successful apply (SF-#14).
- **#V8 — Graceful WAV-missing branch.** When `load_audio_track` returns `Ok(None)`, emit `log::info!("[cutscene] {} has no audio sidecar; playing silently", path)` exactly once per load (SF-#4).
- **#V9 — Pure-function state machine test.** Extract `next_mode(mode, input) -> GameMode`, unit test transitions without main-loop rig (SIMP-L1). Full cutscene smoke test dropped.

## Amended Phase 3 — Dabora 5 "Nabu" (Advisor BIN research + decoder)

Renumbered from Dabora 6. Owns `advisor.rs`, `scripts/discover_advisor_bin_offset.py` (only surviving Python helper — SIMP-M5 drops `dump_advisor_bins.py`).

- **Phase 5a — Inline hex corpus.** `for f in assets/references/ref-ui/07-droid-advisors/alliance/*.bin; do xxd -l 32 "$f"; done > /tmp/bin-corpus.txt`. Findings paste into post-sprint report appendix (SIMP-H7).
- **#N1 — `parse_advisor_bin_v2`** cascades `try_parse_v1 → try_parse_v2`. Acceptance: parse rate ≥ 50%.
- **#N2 — `ADVISOR_BIN_BMP_OFFSET`** const replaces the modulo shortcut at `advisor.rs:472-477`. Modulo fallback only for unresolved IDs.
- **#N3 — Per-faction summary log** *(SF-#1)*: `[advisor] {faction} BIN: total={N}, valid_v1={}, valid_v2={}, parse_failed={{truncated_header:N, truncated_frames:N, length_mismatch:N, other:N}}, empty={}, io_failed={}`. `log::warn!` when `valid_total / total` drops below `ADVISOR_BIN_MIN_PARSE_RATIO`.
- **#N4 — BIN→BMP coverage counters** *(SF-#2)*: `mapped_offset` and `mapped_modulo`. Rate-limited `log::warn!` when `mapped_modulo > mapped_offset * 0.1`. Integration test: `mapped_offset / total >= 0.8`.
- **#N5 — Tests** for v2 parser fixture, offset lookup, fallback behavior.

## Amended Acceptance Criteria

### Functional
- [ ] **#F1** Save version is v8; loading v7 returns clean rejection.
- [ ] **#F2** `SaveState.economy` exists; save-mid-disaster → reload → no re-emission.
- [ ] **#F3** `Character::heritage_known: bool` + `impl Default for Character` exist.
- [ ] **#F4** `GameEffect::SpecialForceSpawned` + `StoryMessageDisplayed` with `EffectPhase::Command`. No Story phase, no CharacterFlagSet, no SystemIncidentFired.
- [ ] **#F5** `EventAction::SpawnSpecialForce { at_character }` exists; integrator resolves via character lookup + movement.orders().
- [ ] **#F6** `GameEvent.system_tag` exists; `is_story_event` filter deleted.
- [ ] **#F7** `main.rs:3271-3409` deleted; `apply_event_action_to_world` is `pub #[inline]`; `DisplayMessage` routes through `StoryMessageDisplayed`.
- [ ] **#F8** `events.rs:553` uses `expect`; release panics on overflow.
- [ ] **#F9** Rust test guards banned IDs against `Random`.
- [ ] **#F10** Save lockout at all 3 entry points + belt-and-suspenders.
- [ ] **#K1–#K6** 6 notification events via intra-tick scratch. No new world fields.
- [ ] **#R1–#R11** 11 story events register/emit. EVT_CHARACTER_KILLED documented next-tick reactive. No frozen_permanently. No 0x222. No Emperor combat check.
- [ ] **#R4** First co-location triggers heritage flip on 0x396; second shows `EVENT_EMPEROR_AND_VADER_VS_KNIGHT_LUKE` via render-layer branch.
- [ ] **#A1–#A5** Integrator emissions; golden_values.json story_events section with `{min, max}`; eval_parity.py grep + regeneration.
- [ ] **#V1–#V9** `Galaxy → VictoryModal → Cutscene → MainMenu`. Save lockout. `Sound::set_volume`. WAV-missing log. `next_mode` unit test.
- [ ] **#N1–#N5** BIN v2 ≥50%; offset ≥80%; per-faction schema; rate-limited warns.

### Non-Functional (revised)
- [ ] Save format is **v8**, not v7. WASM meta key prefix bumped.
- [ ] Total test count 417 → **~447** (not 470).
- [ ] LOC estimate **~1,750** (not 3,050). New types: 2 + 1 helper effect, not 8.
- [ ] Dabora count: **5** (not 6).
- [ ] Zero new Clippy warnings; WASM build succeeds; deterministic playtest matches golden values.
- [ ] Golden regeneration diff documented in merge commit landing #F6.

## Amended Risk Analysis

| Risk | Severity | Mitigation |
|---|---|---|
| Notification events re-added as `Random` | HIGH | #F9 Rust unit test CI guard. |
| `apply_event_actions` MessageLog layering | CRITICAL → RESOLVED | #F4 `StoryMessageDisplayed` + #F7 pub. |
| `cleanup_destroyed_system` can't emit telemetry | CRITICAL → RESOLVED | #R11 `&mut Vec<GameEffect>`. Next-tick reactivity documented. |
| `#[serde(default)]` under bincode inoperative | CRITICAL → RESOLVED | #F1 v7→v8. `save.rs:367` cited. |
| `EconomyState` missing from SaveState | HIGH → RESOLVED | #F2 bundles into v8. |
| `event_rolls` silent never-fire in release | CRITICAL → RESOLVED | #F8 `expect` not `debug_assert!`. |
| Slotmap key staleness in telemetry | HIGH → RESOLVED | #R10/#R11 use `dat_id`/`name`. |
| `MovementState.orders` wrong lookup | HIGH → RESOLVED | #F5/#A2 use `current_fleet.and_then(...)`. |
| `is_story_event` filter drift | HIGH → RESOLVED | #F6 `SystemTag` field. |
| Save lockout incomplete | HIGH → RESOLVED | #F10 all 3 entry points. |
| 0x394/0x395/0x396 narrate paternity before 0x220 | HIGH → RESOLVED | #R4 audit; 0x396 flips heritage_known. |
| `frozen_permanently` invalid-state risk | HIGH → RESOLVED | #R3 uses existing primitives only. |
| Sentinel `DatId` burns user-mod range | HIGH → RESOLVED | #F5 no class_dat_id field. |
| `CharactersCoLocated` double-walk | HIGH → RESOLVED | #R4 triple-collapse (single event + render branch). |
| Advisor BIN cascade invisible regression | HIGH → RESOLVED | #N3 per-parser breakdown + warn threshold. |
| BIN→BMP modulo fallback silent | HIGH → RESOLVED | #N4 coverage counters. |
| 0x220→0x222 cascade semantics | MEDIUM → OBSOLETE | Event split cancelled. |
| `incident_flags` dual-write | MEDIUM → OBSOLETE | Bit-fields deleted. |
| `last_support_tier`/`was_empty` pollution | MEDIUM → OBSOLETE | Intra-tick scratch. |
| EVT_CHARACTER_KILLED unbounded growth | MEDIUM → RESOLVED | #R11 via `Character::is_killed`. |
| Golden ±10% meaningless for rare events | MEDIUM → RESOLVED | #A4 `{min, max}`. |
| `eval_parity.py` silent fail on filter expansion | MEDIUM → RESOLVED | #A5 grep + regeneration. |
| Bounty Hunter 3-case | MEDIUM → RESOLVED | #A2 documented parity citation. |
| `set_volume` silent on lazy audio init | LOW → RESOLVED | SF-#14: returns bool. |
| WAV missing no log | LOW → RESOLVED | #V8 one-shot `log::info!`. |
| `Character` no `Default` | MEDIUM → RESOLVED | #F3 adds impl. |
| Reserved DatId registry | LOW | Deferred — moot after #F5. |
| `special_forces` no despawn | LOW | Deferred. |

## Amended Documentation Plan

- **`CLAUDE.md`** — "Knesset Shamash-Bet — COMPLETE" section. Save format → v8.
- **`agent_docs/roadmap.md`** — Track A/B DONE. Save version, test count, parity counters updated.
- **`agent_docs/architecture.md`** — save/load flow with v8 migration arm.
- **`program.md`** — new `story_events` section reference.
- **Single post-sprint report** `docs/reports/2026-04-NN-knesset-shamash-bet-report.md` — includes BIN research appendix, telemetry baseline diff, 41-finding deepening summary.
- ~~`agent_docs/research/2026-04-08-advisor-bin-variant-corpus.md`~~ — DELETED per SIMP-H7.
- ~~`agent_docs/research/2026-04-08-pre-shamash-bet-telemetry-baseline.json`~~ — DELETED per SIMP-H7.

## Conflict Resolution Log

1. **`last_support_tier` placement** — PERF/DI (persistent) vs SIMP (intra-tick). **SIMP wins** — eliminates save-format burden.
2. **`incident_flags` reuse vs delete** — PERF/DI (reuse) vs SIMP (delete). **SIMP wins** — existing bit-field was a C-port artifact. DI's deeper `EconomyState` bug still lands as #F2.
3. **`apply_event_actions`** — ARCH-CRITICAL (effects-based) vs SIMP (pub visibility). **Both win**: #F7 uses pub AND routes DisplayMessage through new GameEffect.
4. **`event_rolls` budget** — PERF (bump) vs SIMP (cut). **SIMP wins** for sprint scope; SF-CRITICAL's underlying `unwrap_or` fix lands as #F8 independently.
5. **`GameMode::Cutscene` struct vs unit** — plan (struct) vs SIMP/ARCH-#7 (unit + path). **SIMP wins** for Cutscene; VictoryModal stays struct (outcome payload is load-bearing).
6. **`CaptivityState` enum vs bool** — DI/ARCH (enum) vs SIMP (existing primitives). **SIMP wins** — `is_captive` + `EventFired` already expresses terminal state.
7. **Dabora 2+4 merge** — SIMP said merge, others silent. **SIMP wins** — sequential anyway.
8. **Emperor Palpatine combat check (#R12)** — plan (include) vs SIMP (cut). **SIMP wins** — combat parity tacked onto story sprint; deferred.

## Final Metrics (post-deepening)

| Metric | Original plan | After deepening |
|---|---|---|
| LOC estimate | ~3,050 | **~1,750** (−43%) |
| New types in rebellion-core | 8 | **2 + 1 helper effect** (−63%) |
| Daborot | 6 | **5** |
| Phase 1 LOC | ~600 | **~150** (−75%) |
| Test count target | ≥470 | **≥447** |
| Save version | v7 (false) | **v8** |
| CRITICAL findings remaining | n/a | **0** (all resolved) |
| HIGH findings remaining | n/a | **0** (all resolved) |
| Review-agent coverage | — | 6 (arch, perf, DI, SF, simp, quad-snd) |
| Total findings synthesized | — | **41** |
