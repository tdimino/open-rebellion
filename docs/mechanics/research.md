---
title: "Research & Tech Trees"
description: "3 independent tech trees driven by character skill assignments"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "research"
sources:
  - type: "code"
    file: "crates/rebellion-core/src/research.rs"
  - type: "prima"
    page: "Chapter 4: Production & Resources"
related:
  - "manufacturing"
tags: ["research", "technology", "unlock", "characters"]
---

# Research & Tech Trees

## What You See

Each faction has three tech trees: Ship, Troop, and Facility. Assigning a character with the right design skill to a research project advances that tree over time. As levels increase, more advanced unit classes become available for manufacturing. Higher-level units are more powerful but take longer to research.

## How It Works

### Three Tech Trees

| Tree | Driven By | Unlocks |
|------|-----------|---------|
| Ship (`TechType::Ship`) | `ship_design` skill | Capital ship and fighter classes |
| Troop (`TechType::Troop`) | `troop_training` skill | Advanced troop unit types |
| Facility (`TechType::Facility`) | `facility_design` skill | Advanced construction types |

Each tree has an integer level per faction, independently tracked. Maximum level is **15** (matches REBEXE.EXE's tech tier range).

### Class Availability Check

A class is buildable when:

```
class.research_order <= faction_tech_level[tree]
```

Classes with `research_order = 0` are available from the start. Each level advancement unlocks all classes at that order.

### Research Duration

The base cost in ticks to advance from level N to N+1:

```
ticks = max(
    max(research_difficulty for all classes at research_order == N+1),
    RESEARCH_MIN_TICKS  // 10
)
```

If no classes exist at the target order, the cost falls back to `RESEARCH_DEFAULT_TICKS` (30).

Currently only `TechType::Ship` has DAT-backed `research_difficulty` values from `CapitalShipClass`. Troop and Facility trees use the default fallback.

### Research Dispatch

- Only one project per (faction x TechType) is active at a time
- A new dispatch replaces any existing project for the same combination
- Projects can be cancelled, freeing the assigned character

### Advance Pattern

```rust
ResearchSystem::advance(state, world, tick_events) -> Vec<ResearchResult>
```

Each tick, active projects decrement `ticks_remaining`. When a project completes, it emits `ResearchResult::TechUnlocked`. The caller is responsible for applying the level-up — `advance()` itself does not mutate research levels.

### Fighter Availability

Fighter classes currently lack a `research_order` field. They are treated as available at any research level, consistent with the original game where fighters unlock alongside their parent capital ships.

## Source Material

- Rust: `crates/rebellion-core/src/research.rs` — `ResearchState`, `ResearchSystem`, `ticks_for_next_level()`

## Related

- [Manufacturing & Production](manufacturing.md)
