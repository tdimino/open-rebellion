---
title: "Story Events System"
description: "Scripted story chains, telemetry twins, and event chaining mechanics"
category: "agent-docs"
created: 2026-04-12
updated: 2026-04-12
tags: [story-events, event-chains, jabba, final-battle, bounty-hunters, dagobah]
---

# Story Events System

`story_events.rs` — Scripted story chains defined via `define_story_events()`.

## Architecture

Events are registered in `EventState` by `define_story_events(&mut state, &world)`. The function receives character keys by name-search (`find_character`), and all event definitions are gated by `if let Some(char)` to handle worlds where characters don't exist.

**Single-pass linear evaluation**: `EventSystem::advance()` evaluates events in definition order (vector index 0..n). When an event fires, its ID is inserted into `fired_ids` immediately inline, so events defined LATER in the same pass can chain off earlier events via `EventFired`. This is NOT a fixpoint loop.

**Same-tick chaining rule**: Events MUST be defined AFTER their prerequisites to chain in the same tick. Events defined BEFORE their prerequisite fire on the NEXT tick.

## Chains

### Chain 1: Luke Dagobah Training
`0x390` (prerequisite gate) → `0x392` (Yoda teacher) → `EVT_LUKE_DAGOBAH` (0x221) → `0x391` (first training) → `EVT_DAGOBAH_COMPLETED` (0x210)

### Chain 2: Final Battle
`0x393` (Vader dispatched) → `0x394` (Vader en route) / `0x395` (alt: reports to Emperor) → `0x396` (Final Battle Imminent, **flips heritage_known**) → `EVT_FINAL_BATTLE` (0x220)

Heritage gate (#R4): `0x396` action includes `SetHeritageKnown { character: luke }`. The render layer (`event_screen.rs`) branches on `heritage_known` to pick the correct BMP for 0x220 (Vader vs Student Luke or Emperor & Vader vs Knight Luke).

### Chain 3: Bounty Hunters → Jabba's Palace
`0x397` (gate) → `EVT_BOUNTY_ATTACK` (0x212, **SpawnSpecialForce + CharacterAssignedToFleet**) → `0x398` (carbonite)

5-case palace outcome switch:
1. `0x384` — Han self-escape (Random 0.10, tick ≥ 135)
2. `0x383` → `0x39A` → `EVT_HAN_RESCUE` (0x200) — rescue chain
3. `0x399` — Luke captured (Random 0.20, tick ≥ 115, **before 0x383 in source order**)
4. `0x385` — Leia captured (Random 0.15, tick ≥ 108)
5. `EVT_JABBA_CAPTURES_CHEWIE` (0x387) — Chewie captured (Random 0.12, tick ≥ 110)

**Source-order preemption**: 0x399 is defined BEFORE 0x383, so Luke capture can preempt rescue. 0x385 and 0x387 are AFTER 0x383 and cannot preempt rescue.

### Telemetry Twins
- `EVT_HAN_RESCUE` (0x200) — fires same-tick as 0x383 via `EventFired` chaining. Silent (no actions).
- `EVT_JABBA_PRISONERS` (0x231) — consolidator for any palace capture. Three OR-branch variants (one per capture ID) with `EventNotFired` self-guard ensuring exactly one fires.

### Carbonite Countdown → Permanent Freeze
`EVT_HAN_CARBONITE_FAIL_1` (0x39C, tick 145) → `FAIL_2` (0x39D, 160) → `FAIL_3` (0x39E, 175) → `FAIL_4` (0x39F, 190) → `FAIL_5` (0x3A0, 205) → `EVT_HAN_PERMANENT_FREEZE` (0x39B)

All fail stages guard against rescue (0x383) and self-escape (0x384). Terminal event also requires `CharacterIsCaptive`.

## Key Patterns

### OR-branch consolidator
Since `EventCondition` uses AND logic, OR semantics require multiple event definitions with the same `id`, each gated on a different trigger + `EventNotFired { id: SELF }` to prevent duplicates.

### Random roll budget
`EventSystem::advance()` panics if more `EventCondition::Random` conditions are evaluated than rolls provided. Conditions short-circuit on AND failure, so a failed `TickAtLeast` before `Random` means no roll consumed. Tests must provide enough rolls for all Random conditions actually reached.

### Scope nesting
The entire palace chain (lines ~394–860) is inside `if let Some(luke) = luke`. Removing Luke from the world removes ALL palace events. Test design must account for this.

## Constants (events.rs)

| Constant | Value | Purpose |
|----------|-------|---------|
| `EVT_HAN_RESCUE` | 0x200 | Telemetry twin of 0x383 |
| `EVT_JABBA_PRISONERS` | 0x231 | Palace capture consolidator |
| `EVT_JABBA_CAPTURES_CHEWIE` | 0x387 | Chewie captured at palace |
| `EVT_HAN_PERMANENT_FREEZE` | 0x39B | Terminal carbonite state |
| `EVT_HAN_CARBONITE_FAIL_1..5` | 0x39C–0x3A0 | Escape countdown stages |
