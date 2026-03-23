---
title: "Port 14 Missing Dispatch Validators into the AI Dispatch Gate"
type: feat
status: planned
date: 2026-03-23
project: open-rebellion
tags: [ai, parity, ghidra, execplan]
---

# Port 14 Missing Dispatch Validators into the AI Dispatch Gate

This ExecPlan is a living document. Sections Progress, Surprises &
Discoveries, Decision Log, and Outcomes & Retrospective must be
kept up to date as work proceeds.

## Purpose / Big Picture

Open Rebellion currently labels `FUN_00508250` as the source for `can_dispatch()`, but the Rust function at `crates/rebellion-core/src/ai.rs` only performs six character-only checks. The original REBEXE.EXE gate is much broader. It rejects deployment when a target system is over its budget, when the wrong ship or troop family is being considered, when faction strength conditions are wrong, or when system status bits say the operation is not legal. Porting the remaining 14 validators will stop AI from issuing dispatches that the original game would have refused, which is the difference between “AI moves something” and “AI moves something that actually fits the system’s deployment rules.”

The result should be visible in two ways. First, focused unit tests should show specific systems flipping from `true` to `false` when energy, material, troop, shipyard, or strength conditions are violated. Second, the composite gate should stop accepting candidates in cases that are currently legal only because Open Rebellion does not model the original system-side dispatch state yet.

## Progress

- [x] (2026-03-23 18:33Z) Inspected the current Rust `can_dispatch()` implementation in `crates/rebellion-core/src/ai.rs` and confirmed it is character-only.
- [x] (2026-03-23 18:33Z) Inspected `crates/rebellion-core/src/world/mod.rs` and confirmed `System` does not yet model the loyalty, budget, allocation, or dispatch-flag fields read by the missing validators.
- [x] (2026-03-23 18:33Z) Read the 14 missing decompiled validators in `ghidra/notes/` plus the surrounding economy and AI parity notes needed to map offsets.
- [x] (2026-03-23 18:33Z) Built an offset-to-field ownership map and grouped validators by complexity and gameplay impact.
- [ ] Refactor the Rust dispatch gate so it can evaluate system-level validators instead of only character fields.
- [ ] Add the missing system dispatch state and any missing class-definition lookups required by the validators.
- [ ] Port validators in the order defined below, adding focused regression tests after each batch.
- [ ] Run full validation and update the AI parity tracker when the cascade is faithful enough to replace the current placeholder.

## Surprises & Discoveries

The first surprise is structural. The current `can_dispatch()` at `crates/rebellion-core/src/ai.rs:314-341` only receives `AIState`, `AiFaction`, `CharacterKey`, and `Character`. Most of the missing validators do not read character state at all. They read system resources, system status flags, fleet iterators, troop iterators, facility iterators, and derived faction-strength bits. That means the present Rust function signature cannot host a faithful port without first introducing a richer dispatch context.

The second surprise is that the missing validators are not pure boolean gates. Several of them write derived values back into system fields before returning. `FUN_0050ad80` writes the computed allocation to `+0x60` through `FUN_00509d40`. `FUN_0050b0b0` writes to `+0x68` through `FUN_00509e40`. `FUN_0050b230`, `FUN_0050b2c0`, and `FUN_0050b310` write scores into `+0x6c`, `+0x70`, and `+0x74`. `FUN_0050b800`, `FUN_0050bb00`, and `FUN_0050b8e0` write status bits. A faithful Rust port therefore needs stateful helper functions, not a single `return condition_a && condition_b`.

The third surprise is that the repo already contains one place where this missing system state is being approximated. `crates/rebellion-core/src/uprising.rs:265-296` currently infers “populated” from `popularity_alliance + popularity_empire > 0` and infers loyalty from popularity percentages because `System` does not store the original `base_loyalty` field. The missing dispatch validators read the real binary-backed loyalty and populated bits directly, so adding these fields now will also remove one existing approximation elsewhere in the simulation.

The fourth surprise is terminology. The parity tracker correctly says “2 no-ops,” but the decompiled evidence shows that this is one no-op helper, `FUN_0051ebb0`, called twice inside the 18-check chain. That matters because it means there are 16 meaningful call sites, 14 of which are still missing in Rust, not 15.

## Decision Log

The plan treats the missing fields as belonging to `System`, not to `Character`, `Fleet`, or `AIState`. The Ghidra evidence is direct: the missing validators call system stat setters (`FUN_00509d40`, `FUN_00509dc0`, `FUN_00509e40`, `FUN_00509ec0`, `FUN_00509f40`, `FUN_00509fc0`) and use the same offset block documented in `ghidra/notes/economy-systems.md`. The owning Rust type should therefore be `world::System`, with a nested struct used to keep the AI-only or still-uncertain fields contained.

The plan keeps the name `can_dispatch()` for the top-level parity function, because that name is already established in `ai.rs`, but it does not keep the current signature. The current character-only logic should be extracted into a helper such as `can_dispatch_character_basics()`, and `can_dispatch()` should become the orchestrator over a `DispatchContext`. This prevents the codebase from accreting a misleading function that claims parity with `FUN_00508250` while still only checking character state.

The plan intentionally uses pragmatic field names for the less-certain offsets instead of pretending we have exact LucasArts names. Fields backed by strong evidence, such as `base_loyalty`, `system_energy`, `system_raw_material`, `system_energy_allocated`, `system_raw_material_allocated`, `is_populated`, and `has_shipyard_or_repair_dock`, should use their semantic names. Fields backed only by validator usage, such as `+0x70`, `+0x74`, and some high bits in `+0x88`, should start as narrowly-scoped internal names inside a nested dispatch-state struct until stronger naming evidence appears.

The plan also treats troop, special-force, manufacturing-facility, and production-facility class metadata as a missing world-model dependency. The data layer already knows those DAT families exist, but `GameWorld` currently only promotes defense facility class definitions. The validator port should load only the minimum additional class fields needed for cost and compatibility checks, not every possible DAT field.

## Outcomes & Retrospective

Pending completion. At the end of implementation this section should summarize which validators were ported exactly, which field names remain provisional, whether `uprising.rs` was migrated to use the real loyalty and populated fields, and whether any original side effects from `FUN_0050ad80` were intentionally simplified.

## Context and Orientation

A validator in this plan means one helper from the original `FUN_00508250` AND-chain. A dispatch context means the Rust struct that carries every piece of information needed to evaluate those validators for one candidate operation. An iterator means the original game’s helper object used to walk fleets, troops, or facilities inside a system; in Rust these should become ordinary loops or iterator adapters over `GameWorld`.

The current Rust gate lives in `crates/rebellion-core/src/ai.rs`. The current world model lives in `crates/rebellion-core/src/world/mod.rs`. The loyalty approximation that should stop being necessary once the real system fields exist lives in `crates/rebellion-core/src/uprising.rs`. The binary-backed field map and system setter evidence live in `agent_docs/ghidra-re.md`, `ghidra/notes/economy-systems.md`, `ghidra/notes/ai-behavior-analysis.md`, and the individual `ghidra/notes/FUN_0050*.c` files for each validator.

The most important orientation fact is that `this` inside the missing validators is a system-like object, not a character. The evidence is the setter fan-out. `FUN_0050ad60` writes through `FUN_00509dc0`, which is the documented setter for the system field at `+0x64`. `FUN_0050ad80` writes through `FUN_00509d40`, the setter for `+0x60`. `FUN_0050b0b0` writes through `FUN_00509e40`, the setter for `+0x68`. `FUN_0050b230`, `FUN_0050b2c0`, and `FUN_0050b310` write to `+0x6c`, `+0x70`, and `+0x74`. That is why these validators belong in a system-aware dispatch layer.

The following table is the field map this plan should use unless later RE disproves it:

| C offset | Proposed Rust owner | Proposed Rust field name | Confidence | Evidence |
|---|---|---|---|---|
| `+0x58` | `System` | `base_loyalty: i32` | High | `ghidra/notes/economy-systems.md` documents `+0x58` as system base loyalty; `FUN_0050b610` reads it directly. |
| `+0x5c` | `SystemDispatchState` | `system_energy: i32` | High | `ghidra/notes/economy-systems.md` names `+0x5c` as system energy; `FUN_0050ad60` and `FUN_0050ad80` use it as the first capacity ceiling. |
| `+0x60` | `SystemDispatchState` | `system_energy_allocated: i32` | High | `FUN_00509d40` is the documented setter for `+0x60`; `FUN_0050ad80` writes it. |
| `+0x64` | `SystemDispatchState` | `system_raw_material: i32` | High | `ghidra/notes/economy-systems.md` names `+0x64` as system raw material; `FUN_0050ad60` and `FUN_0050b0b0` compare against it. |
| `+0x68` | `SystemDispatchState` | `system_raw_material_allocated: i32` | High | `FUN_00509e40` is the documented setter for `+0x68`; `FUN_0050b0b0` writes it. |
| `+0x6c` | `SystemDispatchState` | `production_modifier: i32` | Medium-high | `FUN_00509ec0` writes `+0x6c`; `FUN_0050b230` fills it from faction and scoring inputs. |
| `+0x70` | `SystemDispatchState` | `dispatch_loyalty_percent: i32` | Medium | `FUN_00509f40` writes `+0x70`; `FUN_0050b2c0` computes it from loyalty via `FUN_00559c10`. |
| `+0x74` | `SystemDispatchState` | `ship_compatibility_percent: i32` | Medium | `FUN_00509fc0` writes `+0x74`; `FUN_0050b310` fills it from fleet and facility counts through `FUN_0055a020`. |
| `+0x78 bits 4-5` | `SystemDispatchState` | `deployment_mode: u8` | Low-medium | `FUN_0050a130` writes a 2-bit field at `+0x78 >> 4`; `FUN_0050b800` drives it. |
| `+0x7c` | `SystemDispatchState` | `deployment_position_index: i32` | Medium | `FUN_0050b800`, `FUN_0050bb00`, and `FUN_0050f980` treat non-negative `+0x7c` as “position exists.” |
| `+0x80` | `SystemDispatchState` | `troop_allocation: i32` | Medium-high | `FUN_0050b500` subtracts `+0x80` from troop count; `FUN_0050a290` is the setter. |
| `+0x84` | `SystemDispatchState` | `troop_deployment_mask: u32` | Medium | `FUN_0050b610` copies `+0x84` into a local bitfield and derives troop placement decisions from it. |
| `+0x88` | `SystemDispatchState` | `dispatch_flags: DispatchFlags` | High as a container, mixed by bit | `FUN_0050b230`, `FUN_0050b310`, `FUN_0050b610`, `FUN_0050b800`, `FUN_0050b8e0`, and `FUN_0050bb00` all read or write bits here. |

The following `dispatch_flags` bits should be named immediately because their evidence is strong enough to stabilize the interface:

| Bit | Proposed name | Confidence | Evidence |
|---|---|---|---|
| `bit 0` | `is_populated` | High | `crates/rebellion-core/src/uprising.rs:268-282` already documents this from `FUN_00509c30`; `FUN_0050b610`, `FUN_0050b800`, and `FUN_0050bb00` also gate on it. |
| `bit 2` | `has_deployment_position` | Medium | `FUN_0050b800` and `FUN_0050bb00` pair this bit with `+0x7c >= 0`. |
| `bit 5` | `has_shipyard_or_repair_dock` | High | `ghidra/notes/economy-systems.md` and `FUN_00511300.c` confirm this gate. |
| `bit 11` | `loyalty_side_flag` | Low-medium | `FUN_0050b230`, `FUN_0050b5a0`, and `FUN_0050c9f0` feed it into loyalty or faction scoring. Keep the name provisional. |
| `bit 12` | `strength_mismatch_flag` | Low | `FUN_0050b8e0` compares it against computed faction-strength truth. |
| `bit 14` | `strength_ready_flag` | Low | `FUN_0050b8e0` selects it when another comparison succeeds. |

The missing validators split cleanly into four complexity bands:

| Band | Validators | Why this band exists |
|---|---|---|
| Simple field and bit gates | `FUN_0050ad60`, `FUN_0050ba90`, `FUN_0050bb70`, `FUN_0050b800`, `FUN_0050bb00` | One or two reads, no nested iteration, small write-back surface. |
| Single-pass counts and score writes | `FUN_0050b230`, `FUN_0050b2c0`, `FUN_0050b500`, `FUN_0050b610`, `FUN_0050b8e0` | Reads existing system state and one collection at a time, but no deep nested iterator tree. |
| Multi-iterator compatibility checks | `FUN_0050b0b0`, `FUN_0050b310`, `FUN_0050c350` | Require new helpers over fleets, facilities, or per-entity class metadata. |
| Highest complexity | `FUN_0050ad80` | Counts, compares against budget, builds a temporary collection, removes entities until within capacity, then writes the result. |

The gameplay impact is not uniform. `FUN_0050ad80`, `FUN_0050b0b0`, `FUN_0050b310`, `FUN_0050b610`, and `FUN_0050c350` are the highest-impact validators because they enforce actual capacity and compatibility. Without them the AI can ask a system to deploy units that do not fit its energy budget, material budget, shipyard state, troop rules, or per-entity compatibility constraints. `FUN_0050b500`, `FUN_0050ba90`, `FUN_0050bb70`, and `FUN_0050b8e0` are medium-impact because they affect whether there is a practical fleet or troop slot available and whether a system is considered strong enough to proceed. `FUN_0050ad60`, `FUN_0050b230`, `FUN_0050b2c0`, `FUN_0050b800`, and `FUN_0050bb00` are still necessary for parity, but they are more likely to refine downstream state than to prevent the most visibly illegal deployments by themselves.

## Plan of Work

Start by correcting the Rust abstraction, not by porting C line-for-line into the existing character-only function. `can_dispatch()` should become a thin parity wrapper over a new `DispatchContext`, and the current six character checks should move into a helper that remains useful for officer missions. This is the only sequence that avoids painting the implementation into a corner.

Once the gate has the right inputs, extend `System` with a nested `SystemDispatchState` and add the real `base_loyalty` field. Do not flatten every unknown offset into top-level `System`; use a nested struct so the uncertain names stay clearly internal. Every new field should be `#[serde(default)]` to preserve save compatibility and make the migration safe.

After the system state exists, add only the minimum missing class metadata needed for the validators. The world model already stores ship and fighter classes, and it already stores defense facility class definitions. What is still missing is the class-definition data needed to answer cost or compatibility queries for troop, special-force, manufacturing-facility, and production-facility instances. These should be loaded into `GameWorld` via small definition structs keyed by `DatId`, not by bloating the runtime instances.

Then port the validators in an order that builds reusable helpers before the hardest logic depends on them. The easiest batch is `FUN_0050ad60`, `FUN_0050ba90`, `FUN_0050bb70`, `FUN_0050b800`, and `FUN_0050bb00`, because these mostly exercise field access and one iterator helper at a time. The next batch is `FUN_0050b230`, `FUN_0050b2c0`, `FUN_0050b500`, `FUN_0050b610`, and `FUN_0050b8e0`, because they depend on the newly-added fields but still do not need the deepest nested fleet or facility traversal. After that port `FUN_0050b0b0`, `FUN_0050b310`, and `FUN_0050c350`, which are the first places where generic “count compatible entities in this system” helpers will pay off. Finish with `FUN_0050ad80`, because it is the one validator whose original behavior creates a temporary pool, trims it, and then applies side effects.

The final stage is cleanup and validation. Once the new fields exist, update `uprising.rs` so “populated” and loyalty read from the real system state instead of popularity heuristics. Then update the AI parity tracker to reflect the new status and document any remaining provisional names or intentionally simplified side effects.

## Concrete Steps

Work from the repository root, `/Users/tomdimino/Desktop/Programming/open-rebellion`. First confirm the current mismatch between the claimed parity point and the actual signature:

```bash
sed -n '310,341p' crates/rebellion-core/src/ai.rs
sed -n '80,140p' crates/rebellion-core/src/world/mod.rs
sed -n '259,300p' crates/rebellion-core/src/uprising.rs
```

The expected result is that `can_dispatch()` only sees character data, `System` has no loyalty or dispatch-state fields, and `uprising.rs` derives populated and loyalty heuristically.

Next add the system-side model changes in `crates/rebellion-core/src/world/mod.rs` and wire defaults through the data layer. After those edits, run:

```bash
cargo test -p rebellion-core world::tests -- --nocapture
cargo test -p rebellion-core uprising::tests -- --nocapture
```

The expected result is a clean compile, passing world serialization/default tests, and uprising tests updated to use the real system state instead of popularity-only inference where appropriate.

Then introduce the dispatch abstraction in `crates/rebellion-core/src/ai.rs`. This step should create `DispatchContext`, `DispatchCandidate`, a character-basics helper, and the new top-level `can_dispatch()` orchestration shell. Before porting any validator logic, run:

```bash
cargo test -p rebellion-core ai -- --nocapture
```

The expected result is that existing AI tests still pass, even if the new validators still return placeholder values.

After that, port the validators in three batches and add dedicated unit tests at the same time. Batch one is `FUN_0050ad60`, `FUN_0050ba90`, `FUN_0050bb70`, `FUN_0050b800`, and `FUN_0050bb00`. Batch two is `FUN_0050b230`, `FUN_0050b2c0`, `FUN_0050b500`, `FUN_0050b610`, and `FUN_0050b8e0`. Batch three is `FUN_0050b0b0`, `FUN_0050b310`, `FUN_0050c350`, and finally `FUN_0050ad80`. After each batch, run:

```bash
cargo test -p rebellion-core dispatch_validator -- --nocapture
```

The expected result is a growing suite of validator-specific tests whose names all include `dispatch_validator`, with each batch staying green before the next begins.

When all validators are ported, run the full local verification:

```bash
cargo test -p rebellion-core
cargo test
```

The expected result is a clean pass, no failing AI or uprising regressions, and no serialization breakage from the newly-added fields.

## Validation and Acceptance

Acceptance is not “the code compiles.” Acceptance means the Rust gate can now represent the same categories of rejection that the original gate represented. The minimum test suite should include one focused test per missing validator and at least one end-to-end cascade test where a single failing validator flips the whole decision to `false`.

For `FUN_0050ad60`, build a system where `system_energy < system_raw_material` and assert that the overflow adjustment is rejected or corrected exactly as the original helper requires. For `FUN_0050ad80`, create a system whose friendly fleet-side entity count exceeds `system_energy` and assert that the gate rejects the dispatch or trims the allocation in the same way your chosen Rust design preserves. For `FUN_0050b0b0`, create candidates whose material-cost sum exceeds `system_raw_material` and assert that the allocated-material field is updated and the gate fails when it should.

For `FUN_0050b310`, build one populated system with no shipyard flag and one with `has_shipyard_or_repair_dock` set, then verify that the compatible ship candidate only dispatches in the valid case. For `FUN_0050b610`, vary the populated bit, troop class, and troop-availability masks and assert that only legal troop deployments pass. For `FUN_0050c350`, create a system with mixed fleets and facilities and assert that one incompatible entity causes the composite nested check to fail.

For `FUN_0050b8e0`, construct systems where `FUN_00509710`-equivalent faction strengths produce both “ready” and “not ready” states, and assert that the correct `dispatch_flags` bits are written. For `FUN_0050b800` and `FUN_0050bb00`, vary `has_deployment_position` and `deployment_position_index` so that both the direct gate and the derived deployment flag can be tested independently.

The regression bar is that existing character-dispatch tests continue to pass, the new validator tests cover all 14 missing functions, and the AI parity tracker can truthfully move the dispatch-validation row from partial to either done or nearly done with any remaining simplifications called out explicitly.

## Idempotence and Recovery

Every edit in this plan should be safe to repeat. New fields on `System` and any new class-definition maps on `GameWorld` must be marked with `#[serde(default)]`, so re-running tests or reloading older saves does not require a one-time migration script. The validator helpers themselves should be pure or narrowly stateful, so rerunning the same tests should not leave the world in a different state unless the original C helper clearly did so and the test constructs a fresh world each time.

Recovery should happen at milestone boundaries. After the model-change milestone, after the dispatch-abstraction milestone, and after each validator batch, verify the worktree with `git diff -- crates/rebellion-core/src crates/rebellion-data/src`. If a milestone fails, revert only the specific files touched in that milestone after reviewing the diff. Do not use `git reset --hard` in this repository, because the worktree already contains unrelated user changes.

If `FUN_0050ad80` proves too stateful to preserve exactly on the first pass, the recovery path is to keep the helper behind a clearly-named Rust function such as `apply_energy_budget_validator()` and land a boolean-equivalent version first, with a follow-up note in this document describing what side effect is still approximate. That approach is safe because the rest of the cascade does not depend on the random collection object used internally by the original helper.

## Interfaces and Dependencies

At completion the Rust side should expose a dispatch abstraction rich enough to describe the original gate. The minimum interface is a `DispatchContext` that includes the AI state, game world, acting faction, target system, and the candidate being considered. The candidate itself should be an enum, because the original validators route differently for characters, troops, facilities, fleets, ships, and fighters.

The minimum type changes are as follows:

| Type | Required additions |
|---|---|
| `crates/rebellion-core/src/world/mod.rs::System` | Add `base_loyalty` and `dispatch: SystemDispatchState`, both with serde defaults. |
| `SystemDispatchState` | Add `system_energy`, `system_energy_allocated`, `system_raw_material`, `system_raw_material_allocated`, `production_modifier`, `dispatch_loyalty_percent`, `ship_compatibility_percent`, `deployment_mode`, `deployment_position_index`, `troop_allocation`, `troop_deployment_mask`, and `dispatch_flags`. |
| `DispatchFlags` | Represent at least bits 0, 2, 5, 11, 12, and 14 with named constants or bitflags. |
| `GameWorld` | Add minimal class-definition maps for troop, special-force, manufacturing-facility, and production-facility metadata if a validator needs them. |
| `crates/rebellion-core/src/ai.rs` | Add `DispatchContext`, `DispatchCandidate`, `can_dispatch_character_basics()`, and the new `can_dispatch()` orchestrator. |

The minimum helper surface should include functions or methods equivalent to “friendly entity count in system,” “friendly material-cost sum in system,” “friendly troop count,” “friendly facility count,” “candidate has legal shipyard/troop compatibility,” “candidate has legal ship capacity,” and “compute faction strength at this system.” The implementation can use ordinary Rust loops. It does not need to mimic the original iterator object layout as long as it produces the same observable boolean decisions and writes the same derived system fields.

The external dependencies are already in the repo. The authoritative RE inputs are `ghidra/notes/FUN_0050ad60.c`, `ghidra/notes/FUN_0050ad80.c`, `ghidra/notes/FUN_0050b0b0.c`, `ghidra/notes/FUN_0050b230.c`, `ghidra/notes/FUN_0050b2c0.c`, `ghidra/notes/FUN_0050b310.c`, `ghidra/notes/FUN_0050b610.c`, `ghidra/notes/FUN_0050b500.c`, `ghidra/notes/FUN_0050ba90.c`, `ghidra/notes/FUN_0050bb70.c`, `ghidra/notes/FUN_0050c350.c`, `ghidra/notes/FUN_0050b8e0.c`, `ghidra/notes/FUN_0050b800.c`, and `ghidra/notes/FUN_0050bb00.c`. The field ownership evidence is in `ghidra/notes/economy-systems.md`. The current parity summary is in `agent_docs/systems/ai-parity-tracker.md`.
