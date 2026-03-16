# Uprising System

`uprising.rs` — Loyalty-driven control changes with UPRIS1TB/UPRIS2TB probability tables.

## Types

| Type | Purpose |
|------|---------|
| `UprisingState` | Active uprisings + per-system incident cooldowns |
| `UprisingEvent` | UprisingIncident / UprisingBegan / UprisingSubdued |
| `SystemUprisingExt` | Extension trait on `System`: `is_populated()`, `loyalty_value()` |

## API

```rust
// Each tick: evaluate all systems for uprising risk
let events = UprisingSystem::advance(&mut state, &world, &tick_events, &rolls, &upris1tb);

// When Subdue Uprising mission completes:
let subdued = UprisingSystem::try_subdue(&mut state, &world, sys, tick, roll, &upris2tb);
```

## Loyalty Scale

`loyalty_value() = (controlling_faction_popularity * 100) - 50` (signed integer).

- Positive = loyal (above neutral)
- Negative = disloyal (uprising risk)
- UPRIS1TB thresholds are all negative: `(-40, 90%)`, `(-20, 50%)`, `(-5, 10%)`

## Lifecycle

1. System loyalty drops below UPRIS1TB lowest threshold
2. `UprisingIncident` fires (warning) — 10-tick cooldown prevents spam; first incident always fires
3. RNG roll against `start_prob / 100.0` — if passes, `UprisingBegan` fires
4. Caller flips `System::controlling_faction`
5. System stays in `active_uprisings` until subdued via mission

## Source

- Ghidra RE: `economy-systems.md §3`
- Events: `0x14b` (Uprising), `0x151` (ControlKindUprising), `0x152` (UprisingIncident)
