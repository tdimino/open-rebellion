# Dabora 3 R11 Foundation — Review Synthesis

**Reviewed commits:** `ac306e7..f15365c` (Dabora 1 foundation → Dabora 3 R11 foundation)
**Status:** 425 tests pass, build clean, 3 of 4 reviewers reported (Codex still running)
**Reviewers:** code-reviewer, silent-failure-hunter, code-simplifier + Codex pending

## The Core Finding (CRITICAL — C1, echoed by silent-failure-hunter and code-reviewer)

The commit message and plan (line 850) assert:

> Character-death-triggered story events are **STRICTLY next-tick reactive** (Events=step 6, Death Star cleanup=step 11).

**This invariant holds for Death Star kills but NOT for mission assassinations.** Mission assassinations run at step 5 and flip `is_killed=true` via `apply_mission_effects_inner`. Step 6 is `EventSystem::advance` — it can evaluate the new `EventCondition::CharacterIsKilled` against the freshly-flipped flag and fire EVT_CHARACTER_KILLED story events **same-tick as the kill**.

**Fix options:**
1. (Recommended) Weaken the invariant: mission kills fire same-tick; Death Star kills next-tick. Document it. Test both paths.
2. Defer mission-kill `is_killed` flips to an after-events step. Complicates ordering and creates its own set of questions.

I'm going with option 1 — the single-tick reactivity is actually correct for mission kills (player sees assassination and death event simultaneously), and Death Star deaths are unavoidably next-tick because cleanup runs at step 11.

## CRITICAL C2 — "Killed but still alive to every other system"

**Biggest architectural gap.** R11 changed cleanup from `world.characters.remove(ck)` to `is_killed=true, keep in arena`, but **no other system filters on `is_killed`**.

| File | Line | Bug |
|---|---|---|
| `ai.rs:330-358` | `can_dispatch` | AI dispatches corpses on missions |
| `betrayal.rs:84` | betrayal loop | Dead Mon Mothma can defect |
| `jedi.rs:193-198` | training | **Stale comment says "removed from world"** — killed Luke keeps accumulating XP |
| `missions.rs:717,1068,1104` | mission target selection | Can target dead characters |
| `events.rs:703-727` | `CharactersCoLocated` | `current_system` never cleared — killed Luke can co-locate with Vader at destroyed Yavin |
| `events.rs:737` | `CharacterHasActiveMovementOrder` | `current_fleet` never cleared — killed Han still has "active movement order", triggers R5 bounty hunters |

**Fix:** Add `mark_character_killed(&mut Character)` helper that clears: `current_system`, `current_fleet`, `on_mission`, `on_mandatory_mission`, `is_captive`. Call from BOTH `cleanup_destroyed_system` AND `apply_mission_effects_inner::CharacterKilled`. Add `is_killed` short-circuit at the top of character-iterating systems.

Currently the mission arm (integrator.rs:970-974) clears `on_mission` + `on_mandatory_mission` but not the rest. The DS cleanup clears nothing. Asymmetric.

## CRITICAL C3 — Interactive AI-driven DS fire doesn't call cleanup_destroyed_system

**File:** `crates/rebellion-app/src/main.rs:1400-1441`

The `DeathStarSystem::advance → PlanetDestroyed` branch sets `is_destroyed=true` and pushes a log message but **never calls `cleanup_destroyed_system`**. Only the user-initiated `PanelAction::FireDeathStar` (main.rs:2781-2824) calls it.

**Pre-existing bug, but R11 now makes it actively broken** because the commit message advertises "interactive Death Star fire path" as covered for telemetry. AI-driven DS kills silently drop all telemetry and never mark characters `is_killed`.

The silent-failure-hunter (LOW finding) noted this path is currently dead code because `DeathStarSystem::advance()` never actually emits `PlanetDestroyed` — only `ConstructionCompleted` and `NearbyWarning`. That softens the priority but does NOT remove the gap. Fix: wire `cleanup_destroyed_system` into main.rs:1419 mirroring the panel handler OR delete the dead branch entirely.

## HIGH — CharacterIsKilled / CharacterHasActiveMovementOrder silent failures

Both from silent-failure-hunter:

### `CharacterIsKilled.unwrap_or(false)` — `events.rs:741-745`
Missing character silently returns "not killed" → story chain silently never fires → invariant break invisible.
**Fix:** `debug_assert!(world.characters.contains_key(*character))` in debug.

### `CharacterHasActiveMovementOrder` semantic lie — `events.rs:729-739`
Returns true for **stationary-fleet** characters too. SF-#7 wanted "in transit". Stationary character passes BOTH this AND `CharacterAtSystem`, making the OR gate vacuous.
**Fix:** rename to `CharacterAssignedToFleet` (honest) OR tighten to check `MovementState::get(c.current_fleet?).is_some()`. Honest rename is cleaner.

### `SetHeritageKnown` silent-drop — `integrator.rs:1229-1242`
No `else` branch, no warn. Dropped flip = wrong 0x220 cutscene. Should match Dabora 2's `SpawnSpecialForce` pattern and `eprintln!`.

## HIGH H2 — R6/R7/R8 route through SYS_STORY but should be SYS_MISSIONS

**File:** `simulation.rs:228, 261, 278, 300`

Mission-side emissions use `SYS_STORY` while every other mission-side emission uses `SYS_MISSIONS` (integrator.rs:512, 520, 551). The plan's `SystemTag` scheme is supposed to handle routing at the `state.define()` site, not at ad-hoc emit points.

**Fix (simpler):** Switch to `SYS_MISSIONS`. That means golden_values.json and eval_parity.py need no change (they look at event IDs, not system tags).

## MEDIUM — Golden values regeneration pending

R11 adds `character_killed`, `character_health`, `informant_intel`, `saboteur_detected` emissions. Golden values cover only Dabora 2 events. Not blocking R1-R10 work but IS a subtask of the Dabora 3 verification task (#84).

## MEDIUM — R11 adds primitives with zero tests

`CharacterIsKilled`, `CharacterHasActiveMovementOrder`, `SetHeritageKnown`, `cleanup_destroyed_system` out-param, mission-side EVT_CHARACTER_KILLED — all untested. `CharacterIsKilled` test would have caught C1 immediately.

**Fix:** These land as part of task #83 (R14 tests).

## Simplification Wins (code-simplifier)

### Worth collapsing

**K6 Manufacturing two-pass → one-pass** — `manufacturing.rs:362-403`
`pre_lengths: HashMap<SystemKey, usize>` scratch allocation is avoidable. Capture `queue.len()` inside the iter loop. ~25 LOC → ~10 LOC, HashMap allocation removed per tick. Tests cover the path.

**Death Star `killed_characters` Vec → single pass** — `death_star.rs:351-374`
The two-pass comment "borrow checker issues" is wrong — split borrows through distinct struct fields permit `world.fleets.get()` + `world.characters.get_mut()` simultaneously. Clone the fleet.characters, release the fleets borrow, then mutate. ~10 LOC saved.

Both should land as part of R11 cleanup, BEFORE the simpler collapse proves wrong under C1/C2 mutex changes.

### Not worth collapsing (documented)

- K4 two u32 cooldowns — per-faction required (one masks the other)
- K3 `resource_discovery_armed` gate — needed; first-tick seed would false-fire
- `SetHeritageKnown` comment block — WHY comment, plan decision
- R6/R7/R8 match arms — three arms, 60 LOC, not duplication

## Dabora 2 Boundary Audit (code-reviewer L1)

**Clean.** `apply_event_action_to_world` retains full Dabora 2 SpawnSpecialForce resolution (integrator.rs:1152-1228), `SetHeritageKnown` cleanly appended. main.rs deletion marker at 3405-3410 preserved. R6/R7/R8 block sits inside mission step 5, doesn't disturb step 4 fog tick. No regressions.

## NOT findings (explicitly cleared)

- `event_rolls.expect(...)` still loud in `events.rs` — Dabora 1 guarantee holds
- Dabora 2's `SpecialForceSpawned` structured `eprintln!` still present at `integrator.rs:1218-1225`
- Save v8 bump with no `#[serde(default)]` is deliberate — v7 loads fail at version check, not at bincode
- `EVT_TRAITOR_REVEALED` / `EVT_SIDE_CHANGE` constants unused but not dangling — deferred to R9/R10

## Action Plan

### Fix-before-R1 commits (blocking next task):

**Commit A — "fix(knesset-shamash-bet): R11 review — is_killed filtering + mark_character_killed"**
- Add `Character::mark_killed()` inherent method clearing `current_system`, `current_fleet`, `on_mission`, `on_mandatory_mission`, `is_captive`, setting `is_killed=true`
- Call from `cleanup_destroyed_system` and `apply_mission_effects_inner::CharacterKilled`
- Add `is_killed` short-circuit in: `ai.rs:can_dispatch`, `betrayal.rs:advance`, `jedi.rs:advance`, `missions.rs:1068,1104,717` target selection
- Fix jedi.rs:194-198 stale comment + add explicit `is_killed` check
- Tests: one unit test per short-circuit site (~6 tests)

**Commit B — "fix(knesset-shamash-bet): R11 review — debug_asserts + honest condition naming"**
- `CharacterIsKilled`: `debug_assert!(world.characters.contains_key(*character))` in debug
- Rename `CharacterHasActiveMovementOrder` → `CharacterAssignedToFleet` (honest) OR add `movement` param (messier)
- `SetHeritageKnown`: `eprintln!` on missing character, matching SpawnSpecialForce pattern
- R6/R7/R8: switch `SYS_STORY` → `SYS_MISSIONS`
- Tests: `evaluate_condition` unit tests for `CharacterIsKilled` + renamed condition (~3 tests)

**Commit C — "refactor(knesset-shamash-bet): Dabora 2 collapse — K6 + cleanup single-pass"**
- K6 HashMap → inline `pre_len` capture
- `killed_characters` Vec → single-pass loop
- Zero test count delta — pure refactor

**Commit D — "fix(knesset-shamash-bet): interactive AI-driven Death Star fire cleanup"**
- Either wire `cleanup_destroyed_system` into main.rs:1419 OR delete the dead branch with `unreachable!`
- Silent-failure-hunter says it's dead code today — prefer `unreachable!` and leave the fix for when `DeathStarSystem::advance` actually emits `PlanetDestroyed`

### After review fixes: R1-R14 in order

Proceed with tasks #75–#84 per the established Dabora 3 breakdown.

## Test Count Trajectory

- Current: 425
- After fix-before-R1 (Commits A+B): ~434
- After R1-R11 wiring (with R14 mini-tests sprinkled as they land): ~445
- After R14 full suite (#83): ~455
- Dabora 3 target: ~447 (plan), ~455 (with review fixes included)
