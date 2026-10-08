---
title: "Resource Stockpiles and Top-Bar Counters"
description: "Model each side's raw and refined materials and maintenance as the original does, and fill the command center's three monitors"
type: feat
status: active
created: 2026-10-08
updated: 2026-10-08
tags: [economy, manufacturing, resources, cockpit, parity]
---

# Resource Stockpiles and Top-Bar Counters

The command center's top strip shows three numbers: the Raw Materials,
Refined Materials and Maintenance monitors. The trace in
[`ghidra/notes/top-bar-resource-counters.md`](../../ghidra/notes/top-bar-resource-counters.md)
finds that the first two are per-side stockpiles that facility cycles change
one unit at a time, and the third is maintenance capacity less maintenance
used. The port has no stockpiles and no facility cycles: its builds run on
fixed day counts and never draw refined material.

## What the original does

- **Raw** (side `+0x78`): a mine adds 1 at the end of each cycle; a refinery
  takes 1 at the start of each cycle, or waits in a queue while none is left.
- **Refined** (side `+0x7c`): a refinery adds 1 at the end of each cycle; a
  yard takes 1 at the start of each cycle. Scrapping a capital ship refunds
  half its refined cost.
- **Maintenance**: `min(50 × mines, 50 × refineries)` less the class
  `maintenance_cost` of every object the side owns. Over-use locks objects
  (event `0x382`).
- **Cycle**: states 1 waiting → 2 working → 3 done → 1. The working delay
  of a mine or refinery is
  `(ceil(allocated / 10) + processing_rate) × efficiency / 100`, where
  `allocated` is its share of the maintenance load and `efficiency` the
  support-based rate the port already computes
  (`economy::calculate_collection_rate`). A facility's first cycle after it
  comes online is randomised: `rand(0..=d) + d / 2`. A yard's delay is its
  `processing_rate`.

## Phases

### Phase 1: Maintenance Monitor (done)

- `rebellion-core/src/resources.rs`: `maintenance_capacity`,
  `maintenance_used`, `maintenance_surplus`.
- The app draws the right counter; the other two stay blank (`port:`).
- hyp: queued units count from their order (manual p. 84).

### Phase 2: Stockpiles and mine and refinery cycles

- Trace how `FUN_0052f6b0`/`FUN_0052f8f0` spread the maintenance load over
  facilities (the `allocated` share) before coding.
- Per-side raw and refined counts. Per-facility cycle state, the ready day
  and the first-cycle flag, run in the tick after economy.
- The refinery's wait queue (`FUN_0052fbb0`, `FUN_0052fbf0`).
- Open: the enemy-diversion roll's source record (`FUN_005166a0`,
  `FUN_005185c0`) and the starting stockpiles.
- Bump `SAVE_VERSION`; regenerate the replay golden for this named cause.

### Phase 3: Yards draw refined material

- Each yard cycle takes 1 refined unit or waits; a build's progress is its
  completed cycles. This replaces `QueueItem::ticks_remaining`.
- The scrap refund (`FUN_00530270`).
- Check Build Selection's best-case times and the AI's build choices
  against the new flow.

### Phase 4: Raw and Refined monitors

- Wire the left and middle counters; verify natively for both factions
  against captures of the original.

## Known gaps

- **Seeding**: on seed 42 (Small, Medium) our Empire starts with 11 mines,
  11 refineries and 515 maintenance in use, a surplus of 35; the original's
  Empire showed 317 on day 10. The facility counts or unit mix differ.
- `seeds::compute_faction_maintenance` charges each regiment a flat 1; the
  original charges the class's `maintenance_cost` (TROOPSD 1..8). It sets
  the seeding budget.
