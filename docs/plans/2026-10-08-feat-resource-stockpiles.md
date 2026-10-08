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

### Phase 2: Stockpiles and mine and refinery cycles (done)

- `rebellion-core/src/stockpiles.rs`: per-side raw and refined counts and a
  cycle per facility (idle, waiting, working; ready day; first-cycle flag),
  run daily between the economy and manufacturing.
- The load spread (`FUN_0052f6b0`/`FUN_0052f8f0`, `FUN_0055a820`/`FUN_0055a960`)
  deals the change one unit at a time and takes from the fullest first; a
  new facility gets no share until the load changes.
- Refineries and yards that find their input empty wait in a FIFO queue
  (`FUN_0052fbb0`, `FUN_0052fbf0`).
- A finished unit goes to the other side with chance |system `+0x6c`|%
  (`FUN_005166a0`), the support drift `economy` now stores.
- Starting stockpiles are 0 (hyp). Save v33; the replay golden was
  regenerated for this cause.

### Phase 3: Yards draw refined material (done)

- A queue item's progress is units of work: each yard cycle takes 1 refined
  unit or waits, and a unit's cost is its class's refined cost. This replaces
  `QueueItem::ticks_remaining`.
- The yard manager (`FUN_00529dd0`) runs as many yards as work is left,
  starting the fastest first; blockade stops the draw.
- Overdrawn maintenance arms event `0x382` every 10 days (GNPRTB 7168) and
  scraps a random unit that costs maintenance (`FUN_00530350`).
- Scrapping (`rebellion-core/src/scrap.rs`): the player's order `0x200`
  with its confirmation (TEXTSTRA `0x7050`/`0x7054`, pictures 1032/1033)
  and the half refined refund (`FUN_00530270`).
- The AI orders no unit whose maintenance it cannot cover (hyp; manual p. 30).

### Phase 4: Raw and Refined monitors

- The left and middle counters show the side's stockpiles. Native and browser
  checks for both factions remain.

## Known gaps

- **Exploration is shared by both sides.** Seeding explores the rim system
  holding the Alliance headquarters, so an Empire player sees its name and
  support in the Alliance's colors (Hoth on seed 42), although its facility
  view no longer holds the headquarters. Per-side exploration is untraced.
- **Compact legend art.** STRATEGY 10168's white border shares palette
  index 255 with its text, so keying its first pixel would erase the text;
  the original shows no border. How the original draws it is untraced.
- **Flame marks.** `FUN_0045bbb0` composites STRATEGY 905/906 onto a planet
  when system `+0x88` bit 4 is set; what sets it is untraced, and the port
  draws neither.

- **Seeding**: on seed 42 (Small, Medium) our Empire starts with 11 mines,
  11 refineries and 515 maintenance in use, a surplus of 35; the original's
  Empire showed 317 on day 10. The facility counts or unit mix differ.
- `seeds::compute_faction_maintenance` charges each regiment a flat 1; the
  original charges the class's `maintenance_cost` (TROOPSD 1..8). It sets
  the seeding budget. On seed 42 the Alliance starts 16 over its capacity,
  so its overdraft timer arms on the first day.
- Untraced: what scrapping a fleet does to its members (hyp: they go with
  it), destruction reason `0x16`, whether an uprising halts facilities
  (manual says so), and whether the player's Build Selection checks
  maintenance.
