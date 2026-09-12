---
title: "Tactical Result Identity Regression"
description: "Per-hull damage and duplicate-class fighter roster writeback in the interactive tactical path"
category: qa
created: 2026-09-12
updated: 2026-09-12
tags: [qa, tactical, combat, persistence]
---

# Tactical result identity regression

The interactive tactical result path now writes each surviving ship's final hull strength back to its original fleet hull and removes destroyed hulls. Fighter losses use the originating roster index, not the class key: two entries of the same fighter class can finish with different counts. Results are applied independently to both fleets.

The focused native regression `tactical_space_results_preserve_hull_damage_and_duplicate_fighter_roster_slots` covers a damaged Alliance survivor, a destroyed Alliance hull, two same-class fighter entries with different losses, and a damaged Imperial survivor. The app and render crates compile; their 158 unit tests pass. Formatting and Clippy pass with the repository's existing `disallowed-names` warnings in the DAT dumper excluded. This is an outcome-integrity tranche, not a claim that the original tactical interface, battle formulas, or browser journey is complete. The standalone battle launcher plan tracks the remaining shared entry, return, fixture, and visual work.

## Shared entry follow-up

Campaign battles now enter through `tactical_flow::begin_player_battle`. The function rejects missing systems, duplicate or missing fleets, fleets not orbiting the requested system, and same-faction pairs before changing tactical state, cooldowns, messages, or game mode. Native tests cover both player sides and an invalid request with no mutation. The full workspace test suite and all ten app tests pass; the feature-gated WASM app compiles. The fixture has not yet been connected to this function, and the return path remains inline; no browser or visual acceptance is claimed here.
