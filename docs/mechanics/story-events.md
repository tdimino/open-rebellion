---
title: "Scripted Story Events"
description: "4 story chains (12 event nodes): Force milestones, Dagobah, Final Battle, Bounty Hunters, Jabba's Palace"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "story-events"
sources:
  - type: "code"
    file: "crates/rebellion-core/src/story_events.rs"
  - type: "code"
    file: "crates/rebellion-core/src/events.rs"
related:
  - "jedi"
  - "characters"
tags: ["story", "events", "chains", "scripted"]
---

# Scripted Story Events

The game features 4 scripted story chains containing 12 event nodes that fire when conditions are met. Events are registered at game start via `define_story_events()` and evaluated each tick by `EventSystem::advance`.

## What You See

- Story messages appear in the message log when conditions are met.
- Characters may be placed on mandatory missions, start Jedi training, or get captured.
- Events chain: one event's completion unlocks the next in sequence.

## How It Works

### Event System

Each `GameEvent` has:
- **id**: Unique event ID (from the RE event registry).
- **conditions**: All must be true for the event to fire (`EventCondition` variants).
- **actions**: Applied when the event fires (`EventAction` variants).
- **is_repeatable**: If false (most story events), fires only once.

### EventCondition Types

| Condition | Description |
|-----------|-------------|
| `TickAtLeast { tick }` | Game has reached at least this tick |
| `CharacterExists { character }` | Character is alive in the world |
| `CharacterHasForceLevel { character, min_tier }` | Force tier >= threshold |
| `CharacterIsJediTrainer { character }` | Character has `is_jedi_trainer = true` |
| `CharacterForceExperience { character, min_xp }` | Force XP >= threshold |
| `CharacterIsCaptive { character }` | Character is currently held captive |
| `EventFired { id }` | Another event has already fired |
| `EventNotFired { id }` | Another event has NOT fired (blocking condition) |
| `FactionControlsNSystems { faction, min_count }` | Faction controls at least N systems |
| `Random { probability }` | RNG roll against probability |

### EventAction Types

| Action | Description |
|--------|-------------|
| `DisplayMessage { text }` | Show message in the log |
| `SetMandatoryMission { character, mandatory }` | Lock/unlock character on mandatory mission |
| `StartJediTraining { character }` | Begin Force training |
| `AccumulateForceExperience { character, amount }` | Add XP directly |
| `CaptureCharacter { character, captor_faction }` | Capture a character |
| `SetCarboniteState { character, frozen }` | Freeze/unfreeze in carbonite |

### Event Chains

#### Chain 1: Luke Dagobah Training (multi-phase)

| Event ID | Name | Conditions | Actions |
|----------|------|------------|---------|
| `0x1e1` | Force Awakening | Luke is Aware | Display message |
| `0x1e5` | Force Training Milestone | Luke is Training | Display message |
| `0x390` | Dagobah Calls | Luke Aware, tick >= 30, Yoda exists | Display message |
| `0x392` | Yoda Agrees to Train | 0x390 fired, Yoda is trainer | Display message |
| `0x221` | Luke Dagobah Training | Luke exists + Aware, tick >= 50 | Mandatory mission, start training |
| `0x391` | First Training Day | 0x221 fired, tick >= 55 | +10 XP, display message |
| `0x210` | Dagobah Complete | 0x221 fired, Luke Experienced | End mandatory mission |

#### Chain 2: Final Battle (multi-phase)

| Event ID | Name | Conditions | Actions |
|----------|------|------------|---------|
| `0x393` | Vader Dispatched | Dagobah complete, Vader exists | Vader mandatory mission |
| `0x394` | Vader En Route | 0x393 fired, tick >= 70 | Display message |
| `0x395` | Vader Reports to Emperor | 0x393 fired, Empire >= 150 systems, 0x394 NOT fired | Alternate branch |
| `0x396` | Final Battle Imminent | 0x394 fired | Display message |
| `0x220` | The Final Battle | 0x396 fired, Luke + Vader exist | Both mandatory mission |

#### Chain 3: Bounty Hunters + Carbonite

| Event ID | Name | Conditions | Actions |
|----------|------|------------|---------|
| `0x397` | Bounty Hunters Active | Tick >= 80, Empire >= 50 systems, Han exists | Display message |
| `0x212` | Bounty Hunter Attack | 0x397 fired, tick >= 100, Han exists, 15% roll | Han mandatory mission |
| `0x398` | Han Frozen in Carbonite | 0x212 fired | Set carbonite frozen |

#### Chain 4: Jabba's Palace (captures + rescue)

| Event ID | Name | Conditions | Actions |
|----------|------|------------|---------|
| `0x380` | Luke Senses Han | 0x212 fired, Luke exists | Display message |
| `0x381` | Leia Plans Rescue | 0x212 fired, Leia exists | Display message |
| `0x382` | Chewbacca Joins | 0x381 fired, Chewbacca exists | Display message |
| `0x383` | Han Rescued | 0x380 + 0x381 fired | End Han mandatory mission |
| `0x399` | Luke Captured at Palace | 0x380 fired, tick >= 115, 0x39A NOT fired | Capture Luke |
| `0x39A` | Palace Rescue Complete | 0x383 fired | Unfreeze Han |

### Chaining Behavior

Events fire in definition order within a single `advance()` call. If event A fires and event B checks `EventFired { id: A }`, B can chain-fire in the same tick if B is defined after A. Events defined before A require the next tick to see A's fired status.

### Graceful Degradation

Characters are looked up by name. If a required character is not found (mods, alternate scenarios), that chain is silently skipped.

## Source Material

- **Code**: `crates/rebellion-core/src/story_events.rs` -- `define_story_events()`, all 12 chains
- **Code**: `crates/rebellion-core/src/events.rs` -- `EventSystem`, `EventCondition`, `EventAction`
- **Event IDs**: `0x1e1`--`0x39A` (see event registry in entity-system.md §6)

## Related

- [jedi.md](jedi.md) -- Force tier conditions and Jedi training actions
- [characters.md](characters.md) -- Character existence and captivity conditions
