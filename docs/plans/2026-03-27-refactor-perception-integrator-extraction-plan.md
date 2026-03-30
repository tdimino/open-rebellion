# Extract PerceptionIntegrator From simulation.rs

This ExecPlan is a living document. Sections Progress, Surprises &
Discoveries, Decision Log, and Outcomes & Retrospective must be
kept up to date as work proceeds.

## Purpose / Big Picture

The goal is to turn `crates/rebellion-data/src/simulation.rs` into a small tick
orchestrator that does three things only: define `SimulationStates`, allocate
rolls and call `advance()` on each system in canonical order, and keep the
existing smoke tests. All world mutation and all `GameEventRecord` emission move
into `crates/rebellion-data/src/integrator.rs` on `PerceptionIntegrator`. When
this refactor is done, reviewers should be able to read `run_simulation_tick()`
top to bottom as a sequencing function instead of a 1,691 line mixed-control and
mutation file. The observable proof is that each work package leaves behavior
unchanged, `cargo check` stays green, the non-doctest workspace suite stays
green, and `simulation.rs` shrinks toward roughly 200 lines.

## Progress

- [x] (2026-03-28 03:20Z) Inspected `crates/rebellion-data/src/simulation.rs` and `crates/rebellion-data/src/integrator.rs`, confirmed the section boundaries and the existing scaffold.
- [x] (2026-03-28 03:20Z) Verified the current compile baseline with `cargo check`.
- [x] (2026-03-28 03:20Z) Verified the current non-doctest test baseline with `cargo test --workspace --lib --tests --bins` and counted 393 passing tests.
- [x] (2026-03-28 03:20Z) Captured the pre-existing doctest failure on `cargo test --workspace` so package gates can be stated precisely.
- [ ] Work Package 1 implemented.
- [ ] Work Package 2 implemented.
- [ ] Work Package 3 implemented.
- [ ] Work Package 4 implemented.
- [ ] Work Package 5 implemented.
- [ ] Work Package 6 implemented.
- [ ] Work Package 7 implemented.
- [ ] Work Package 8 implemented.
- [ ] Work Package 9 implemented.

## Surprises & Discoveries

- Observation: The literal workspace command `cargo test --workspace` is already red before any refactor work because three existing doctests fail in `crates/rebellion-core/src/tick.rs`, `crates/rebellion-core/src/manufacturing.rs`, and `crates/rebellion-core/src/movement.rs`.
  Evidence: `cargo test --workspace` finished with code 101 and reported 3 doctest failures caused by `No such file or directory (os error 2)`.

- Observation: The non-doctest workspace suite is green and currently covers 393 tests, not 396.
  Evidence: `cargo test --workspace --lib --tests --bins` listed and ran 393 tests successfully across `rebellion-core`, `rebellion-data`, and `rebellion-render`.

- Observation: `crates/rebellion-data/src/integrator.rs` already owns `sys_name()` and `char_name()`, so `simulation.rs` currently contains duplicated name-resolution logic.
  Evidence: `integrator.rs` lines 19-34 and `simulation.rs` lines 37-62 implement the same helper behavior.

## Decision Log

- Decision: Use nine independently shippable work packages instead of one large extraction.
  Rationale: The file mixes three kinds of responsibilities: pure orchestration, state mutation, and telemetry formatting. Smaller packages let each responsibility move with a stable compile and test gate.
  Date/Author: 2026-03-28 / Codex

- Decision: Keep combat trigger selection, roll allocation, and other system-to-system sequencing logic inside `run_simulation_tick()`.
  Rationale: Those parts are orchestration, not effect application. Moving them would make `PerceptionIntegrator` an alternate simulation driver instead of an effect and telemetry sink.
  Date/Author: 2026-03-28 / Codex

- Decision: Treat `cargo check` and `cargo test --workspace --lib --tests --bins` as the package gate unless the doctest failures are fixed first.
  Rationale: The requested literal `cargo test --workspace` gate is currently impossible to satisfy without unrelated doctest repair work.
  Date/Author: 2026-03-28 / Codex

- Decision: Prefer one integrator method per system section or per extracted helper family, not one giant `apply_one_tick()` method.
  Rationale: The package boundary should mirror the existing section comments in `simulation.rs`, keep review diff sizes small, and preserve one-to-one traceability from old section to new method.
  Date/Author: 2026-03-28 / Codex

## Outcomes & Retrospective

Implementation has not started. The useful outcome of this planning pass is that
the extraction order, the concrete method signatures, the package-level gates,
and the only current test-gate mismatch are all documented in one place. The
main remaining risk is not technical complexity inside the helper bodies; it is
borrow-shape churn while `run_simulation_tick()` is converted from a local
`Vec<GameEventRecord>` builder into an orchestrator around
`PerceptionIntegrator`.

## Context and Orientation

The refactor touches two files. `crates/rebellion-data/src/simulation.rs` is
currently 1,691 lines and contains imports and local helpers at lines 1-114,
`SimulationStates` at lines 116-138, the main tick driver at lines 140-895,
five large apply-helper families at lines 901-1537, and two tests plus a test
fixture at lines 1539-1691. `crates/rebellion-data/src/integrator.rs` is
currently 93 lines and already defines `PerceptionIntegrator`, `sys_name()`,
`char_name()`, `new()`, `push()`, `emit()`, and `finish()`.

In this plan, “orchestration” means deciding system order, consuming random
rolls, updating cooldown and availability bookkeeping that belongs to
`SimulationStates`, and choosing when to call one system after another.
“Effect application” means mutating `GameWorld` or a sibling simulation state in
response to a returned event or result. “Telemetry emission” means creating
`GameEventRecord` values.

The final shape should leave `crates/rebellion-data/src/simulation.rs` with
imports, `SimulationStates`, a much shorter `run_simulation_tick()` that creates
one `PerceptionIntegrator`, and the tests. The final shape should leave
`crates/rebellion-data/src/integrator.rs` as the single home for helper payload
formatters, world mutation helpers, and telemetry emission methods.

## Plan of Work

Work Package 1 establishes the new ownership boundary. Move the duplicated
payload helpers from `crates/rebellion-data/src/simulation.rs` lines 37-114 into
`crates/rebellion-data/src/integrator.rs`, then switch `run_simulation_tick()`
from a local `Vec<GameEventRecord>` to a `PerceptionIntegrator`. To prove the
plumbing works without changing any complex control flow, also move the campaign
snapshot section from `simulation.rs` lines 864-894 into the integrator. The
new signatures are:

```rust
pub fn sys_json(world: &GameWorld, key: SystemKey) -> serde_json::Value
pub fn ai_action_json(action: &AIAction, world: &GameWorld) -> serde_json::Value
pub fn emit_campaign_snapshot(
    &mut self,
    world: &GameWorld,
    movement_state: &MovementState,
)
```

Work Package 2 moves the economy section from `crates/rebellion-data/src/simulation.rs`
lines 180-265. This package is low risk because `EconomySystem::advance()` is
already separate and the integrator only needs to replay the existing match over
`EconomyEvent`. The new signature is:

```rust
pub fn apply_economy_events(
    &mut self,
    world: &mut GameWorld,
    economy_events: &[EconomyEvent],
)
```

This method owns all world mutation for `SupportDrifted` and `ControlResolved`
and all telemetry for `SupportDrifted`, `CollectionRateChanged`,
`GarrisonRequirementChanged`, `IncidentTriggered`, `ControlResolved`,
`EnergyOvercapped`, and `RawMaterialOvercapped`.

Work Package 3 moves manufacturing and movement together because both are
straight-line “apply return values, then emit telemetry” sections and both
reduce the largest amount of repetitive tick-body code. Move manufacturing from
`simulation.rs` lines 267-286, movement from lines 288-314, and the build helper
from lines 1299-1444. The new signatures are:

```rust
pub fn apply_build_completions(
    &mut self,
    world: &mut GameWorld,
    completions: &[rebellion_core::manufacturing::CompletionEvent],
)

pub fn apply_arrivals(
    &mut self,
    world: &mut GameWorld,
    arrivals: &[rebellion_core::movement::ArrivalEvent],
)
```

Inside `integrator.rs`, keep a private helper with the existing body shape:

```rust
fn apply_build_completion(
    world: &mut GameWorld,
    completion: &rebellion_core::manufacturing::CompletionEvent,
)
```

Work Package 4 moves combat application and combat telemetry. Keep combat target
selection, roll allocation, cooldown updates, and `AISystem::record_battle()`
inside `run_simulation_tick()`, but move the world mutation and
`GameEventRecord` creation out. Move the section body from
`simulation.rs` lines 316-430 and the helper bodies from lines 1450-1537. The
new signatures are:

```rust
pub fn apply_space_combat(
    &mut self,
    world: &mut GameWorld,
    system: SystemKey,
    result: &rebellion_core::combat::SpaceCombatResult,
)

pub fn apply_ground_combat(
    &mut self,
    world: &mut GameWorld,
    result: &rebellion_core::combat::GroundCombatResult,
)

pub fn emit_bombardment(
    &mut self,
    result: &rebellion_core::bombardment::BombardmentResult,
)
```

`integrator.rs` should also gain private helpers that preserve the existing
mutation logic exactly:

```rust
fn apply_space_combat_result(
    world: &mut GameWorld,
    result: &rebellion_core::combat::SpaceCombatResult,
)

fn apply_ground_combat_result(
    world: &mut GameWorld,
    result: &rebellion_core::combat::GroundCombatResult,
)
```

Work Package 5 moves mission resolution and escape handling. Move the missions
section from `simulation.rs` lines 445-466, the escape section from lines
468-498, and the helper family from lines 901-1076. This is the first package
that touches both `GameWorld` and sibling state, so keep the API explicit. The
new signatures are:

```rust
pub fn apply_mission_result(
    &mut self,
    world: &mut GameWorld,
    result: &rebellion_core::missions::MissionResult,
    uprising_state: &mut rebellion_core::uprising::UprisingState,
    death_star_state: &mut rebellion_core::death_star::DeathStarState,
    primary_ai: &mut AIState,
    secondary_ai: Option<&mut AIState>,
)

pub fn apply_escape_effects(
    &mut self,
    world: &mut GameWorld,
    effects: &[MissionEffect],
)
```

Inside `integrator.rs`, preserve the current helper shape as a private method so
the mission result method can stay thin:

```rust
fn apply_mission_effects(
    world: &mut GameWorld,
    effects: &[MissionEffect],
    uprising_state: &mut rebellion_core::uprising::UprisingState,
    death_star_state: &mut rebellion_core::death_star::DeathStarState,
)
```

Work Package 6 moves story-event action application. Move the events section
from `simulation.rs` lines 500-519 and the helper family from lines 1078-1207.
Also move the Jedi-training extraction loop at lines 514-519 because it is part
of event action handling, even though it mutates `JediState` instead of
`GameWorld`. The new signatures are:

```rust
pub fn apply_fired_event(
    &mut self,
    world: &mut GameWorld,
    fired: &rebellion_core::events::FiredEvent,
)

pub fn apply_event_jedi_training(
    &mut self,
    world: &GameWorld,
    jedi_state: &mut JediState,
    fired_events: &[rebellion_core::events::FiredEvent],
)
```

Keep the existing helper body as a private function:

```rust
fn apply_event_actions_to_world(
    world: &mut GameWorld,
    actions: &[EventAction],
    tick: u64,
)
```

Work Package 7 moves both AI sections and the AI action helper. Move
`simulation.rs` lines 520-599 and lines 1209-1293. This method should own the
state mutation across `MissionState`, `ManufacturingState`, `MovementState`, and
`ResearchState`, and it should also emit `EVT_AI_ACTION` using the shared
`ai_action_json()` formatter. The new signature is:

```rust
pub fn apply_ai_actions(
    &mut self,
    actions: &[AIAction],
    rolls: &[f64],
    ai_state: &mut AIState,
    mission_state: &mut MissionState,
    manufacturing_state: &mut ManufacturingState,
    movement_state: &mut MovementState,
    research_state: &mut rebellion_core::research::ResearchState,
    world: &GameWorld,
    config: &rebellion_core::tuning::GameConfig,
    dual_ai: bool,
)
```

This package is accepted only if single-AI and dual-AI behavior both stay
unchanged. The `dual_ai` flag exists solely to preserve the current payload
difference where the second AI gets `"dual_ai": true`.

Work Package 8 moves the remaining mid-tick systems that return event enums and
have no additional orchestration beyond iteration. Move blockade from
`simulation.rs` lines 601-634, uprising from lines 636-704, betrayal from lines
706-741, and Death Star from lines 743-772. The new signatures are:

```rust
pub fn apply_blockade_events(
    &mut self,
    world: &mut GameWorld,
    blockade_events: &[BlockadeEvent],
)

pub fn apply_uprising_events(
    &mut self,
    world: &mut GameWorld,
    uprising_events: &[UprisingEvent],
)

pub fn apply_betrayal_events(
    &mut self,
    world: &mut GameWorld,
    betrayal_events: &[BetrayalEvent],
)

pub fn apply_death_star_events(
    &mut self,
    world: &mut GameWorld,
    death_star_events: &[DeathStarEvent],
)
```

Work Package 9 moves the remaining simple sections and then performs the cleanup
pass that gets `simulation.rs` down to the target size. Move fog from
`simulation.rs` lines 433-443, research from lines 774-799, Jedi from lines
801-850, victory from lines 852-862, and then delete any now-dead helpers and
imports from `simulation.rs`. The new signatures are:

```rust
pub fn apply_fog_reveals(
    &mut self,
    world: &GameWorld,
    reveals: &[rebellion_core::fog::RevealEvent],
)

pub fn apply_research_results(
    &mut self,
    research_state: &mut ResearchState,
    research_results: &[ResearchResult],
)

pub fn apply_jedi_events(
    &mut self,
    world: &mut GameWorld,
    jedi_state: &mut JediState,
    jedi_events: &[JediEvent],
)

pub fn apply_victory_outcome(
    &mut self,
    victory_state: &mut VictoryState,
    outcome: &rebellion_core::victory::VictoryOutcome,
)
```

At the end of Work Package 9, `crates/rebellion-data/src/simulation.rs` should
contain only `SimulationStates`, `run_simulation_tick()`, and the existing test
module. `crates/rebellion-data/src/integrator.rs` should contain every helper
that mutates the world or emits tick telemetry.

## Concrete Steps

For every package, work from the repository root:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
```

After each package, run:

```bash
cargo check
cargo test --workspace --lib --tests --bins
```

Expected output is a successful `cargo check` and a successful non-doctest test
run. If the team requires the literal workspace command, run this as a separate
precondition before Work Package 1:

```bash
cargo test --workspace --doc
```

Expected output for that command is currently failure, not success, because of
the three existing doctest issues documented above. Either fix those doctests
first as unrelated preparatory work or explicitly exempt doctests from this
refactor’s acceptance gate.

During implementation, measure file shrinkage after Packages 3, 5, 7, and 9:

```bash
wc -l crates/rebellion-data/src/simulation.rs crates/rebellion-data/src/integrator.rs
```

Expected output is a monotonic decrease in `simulation.rs` and a corresponding
increase in `integrator.rs`, with the final `simulation.rs` count landing near
200 lines.

## Validation and Acceptance

Each package is independently acceptable when the code still compiles, the
non-doctest workspace suite still passes, and the moved section in
`crates/rebellion-data/src/simulation.rs` has been replaced by a method call on
`PerceptionIntegrator` rather than duplicated logic. The package is not accepted
if helper logic is copied instead of moved, or if telemetry starts being emitted
from both files.

The final package is accepted when `crates/rebellion-data/src/simulation.rs`
contains orchestration only, `crates/rebellion-data/src/integrator.rs` owns all
effect application and telemetry emission, and the remaining tests in
`simulation.rs` still pass unchanged. The final review should confirm that every
former inline section has a direct `integrator.apply_*()` or `integrator.emit_*()`
call in `run_simulation_tick()`.

## Idempotence and Recovery

Each package is safe to do in isolation because the boundary is method-extraction
based, not schema-changing. If a package goes wrong, revert only the edits in
`crates/rebellion-data/src/simulation.rs` and `crates/rebellion-data/src/integrator.rs`
for that package and return to the previous green commit. There is no save-file
format change, module rename, or cross-crate API migration required for this
refactor.

If borrow-checker pressure appears during a package, do not widen the scope.
Instead, reduce the extracted method from a batch signature to a per-item
signature for that package, keep behavior identical, and continue. The package
sequence is designed so that such narrowing still preserves the overall target.

## Interfaces and Dependencies

At completion, `crates/rebellion-data/src/integrator.rs` must still expose
`PerceptionIntegrator::new()`, `push()`, `emit()`, and `finish()`, and should
also expose the new public methods introduced in the nine work packages above.
The file must remain the only place that formats AI payload JSON or resolves
human-readable names for systems and characters.

`crates/rebellion-data/src/simulation.rs` must continue to expose:

```rust
pub struct SimulationStates

pub fn run_simulation_tick(
    world: &mut GameWorld,
    states: &mut SimulationStates,
    tick_events: &[TickEvent],
    rolls: &[f64],
    wall_ms: u64,
    config: &rebellion_core::tuning::GameConfig,
) -> Vec<GameEventRecord>
```

No work package should change the public signature of `run_simulation_tick()` or
`SimulationStates`. No work package should move the existing tests out of
`crates/rebellion-data/src/simulation.rs`. No work package should add new
cross-crate dependencies; all moved code stays inside `rebellion-data`.
