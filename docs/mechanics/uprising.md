---
title: "Uprising & Loyalty"
description: "Loyalty-driven uprisings: UPRIS1TB/UPRIS2TB probability tables, incident cooldown, faction flip"
category: "mechanics"
created: 2026-03-16
updated: 2026-03-16
game_system: "uprising"
sources:
  - type: "ghidra"
    file: "ghidra/notes/economy-systems.md"
  - type: "code"
    file: "crates/rebellion-core/src/uprising.rs"
related:
  - "diplomacy"
  - "blockade"
tags: ["uprising", "loyalty", "probability", "faction"]
---

# Uprising & Loyalty

Systems with low loyalty can revolt against their controlling faction, flipping control to the other side. The uprising system uses probability tables for both triggering and subduing revolts.

## What You See

- An `UprisingIncident` warning fires when loyalty is dangerously low -- a window to respond.
- If the uprising roll succeeds, the system flips to the opposing faction.
- The Subdue Uprising mission can end an active revolt.

## How It Works

### Loyalty Scale

Loyalty is derived from the controlling faction's popularity fraction:

```
loyalty_value = (popularity * 100) - 50
```

Positive values mean above-neutral loyalty (safe). Negative values mean below-neutral (danger zone). The uprising tables use negative thresholds.

### UPRIS1TB -- Uprising Start Probability

3 entries mapping loyalty to uprising probability (0--100%):

| Threshold | Probability | Meaning |
|-----------|-------------|---------|
| -40 | 90% | Critically low loyalty |
| -20 | 50% | Low loyalty |
| -5 | 10% | Borderline loyalty |

Linear interpolation between entries via `MstbTable::lookup()`. Loyalty >= 0 produces 0% probability.

### UPRIS2TB -- Subdue Probability

4 entries mapping loyalty to subdue success probability:

| Threshold | Probability | Meaning |
|-----------|-------------|---------|
| -40 | 20% | Very hard to subdue at critical loyalty |
| -20 | 40% | Difficult |
| -5 | 70% | Moderate |
| +10 | 90% | Easy if loyalty has recovered |

### Uprising Flow

Each tick, for every populated system not already in revolt:

1. Compute `loyalty_value` from the controlling faction's popularity.
2. Look up start probability from UPRIS1TB.
3. If probability > 0, fire `UprisingIncident` (precursor warning) with a **10-tick cooldown** to prevent spam.
4. Roll RNG against `start_prob / 100`. On success, fire `UprisingBegan` and add the system to `active_uprisings`.
5. The caller flips `System::controlling_faction` to the opposing faction.

### Subduing

When a Subdue Uprising mission completes at an active uprising system:

1. Look up subdue probability from UPRIS2TB at the system's current loyalty.
2. Roll RNG. On success, remove the system from `active_uprisings` and fire `UprisingSubdued`.

### Population Gate

Unpopulated systems (`popularity_alliance + popularity_empire == 0`) are skipped entirely. This mirrors the C++ gate at `*(uint*)(this + 0x88) & 1`.

### Event IDs

| Event | ID | Meaning |
|-------|----|---------|
| `UprisingIncident` | `0x152` (338) | Precursor warning |
| `Uprising` | `0x14b` (331) | Uprising begins |
| `ControlKindUprising` | `0x151` (337) | Control kind flips |

## Source Material

- **Ghidra RE**: `ghidra/notes/economy-systems.md` §3 -- loyalty write gate, UPRIS1TB/UPRIS2TB, `FUN_005121e0` (SystemUprisingNotif)
- **Code**: `crates/rebellion-core/src/uprising.rs` -- `UprisingSystem::advance`, `UprisingSystem::try_subdue`
- **DAT files**: `UPRIS1TB.DAT` (3 entries), `UPRIS2TB.DAT` (4 entries)

## Related

- [diplomacy.md](diplomacy.md) -- Diplomacy missions affect system loyalty
- [blockade.md](blockade.md) -- Blockades can destabilize loyalty
