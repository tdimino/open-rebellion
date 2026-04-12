---
title: "Betrayal System"
description: "Loyalty-driven character faction defection mechanics"
category: "agent-docs"
created: 2026-03-15
updated: 2026-04-12
tags: [betrayal, loyalty, defection, simulation]
---

# Betrayal System

`betrayal.rs` — Loyalty-driven character faction defection.

## Types

| Type | Purpose |
|------|---------|
| `BetrayalState` | Per-character cooldown timers (`HashMap<CharacterKey, u64>`) |
| `BetrayalEvent` | `CharacterBetrayed { character, defected_to_alliance }` |

## API

```rust
let events = BetrayalSystem::advance(&mut state, &world, &tick_events, &rolls, &loyalty_tb);
```

## Logic

For each character in `world.characters`:
1. Skip if `is_unable_to_betray == true` (Luke, Vader immune)
2. Skip if last check was < `BETRAYAL_CHECK_INTERVAL` (50) ticks ago
3. Compute loyalty score: `loyalty.base as i32 - 50` (same signed scale as uprising)
4. Skip if loyalty >= 0 (loyal enough)
5. Look up probability from UPRIS1TB: `loyalty_table.lookup(score) as f64 / 100.0`
6. Roll against probability — if passes, emit `CharacterBetrayed`

## Effect Application (integrator.rs)

For each `CharacterBetrayed` event:
1. Emit `EVT_TRAITOR_REVEALED` (0x361) — BEFORE flip, with `original_faction` payload
2. Flip `is_alliance` / `is_empire` on the character
3. Remove character from all fleet assignments
4. Emit `EVT_SIDE_CHANGE` (0x386) — AFTER flip, with `defected_to_alliance` payload (DI-H2: uses `char_name`, not `CharacterKey`)
5. Emit legacy `EVT_BETRAYAL` telemetry event

## Source

- `entity-system.md` — `not betray` string at `0x006ab3dc`, loyalty fields `+0x66` and `+0x8a`
- UPRIS1TB.DAT shared with uprising system for threshold lookup
