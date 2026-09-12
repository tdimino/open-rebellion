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

Campaign battles now enter through `tactical_flow::begin_player_battle`. The function rejects missing systems, duplicate or missing fleets, fleets not orbiting the requested system, and same-faction pairs before changing tactical state, cooldowns, messages, or game mode. Native tests cover both player sides and an invalid request with no mutation. The feature-gated WASM app compiles. The return path remains inline.

## Muted browser entry fixture

The test-only tactical fixture now constructs an opposing encounter and calls that same production entry function. Its browser launcher and scenario catalog are absent from the production build. On 2026-09-12, the isolated browser harness passed Alliance and Empire entries at 640×480 and 1280×800 letterboxed: four cases, four HTTP-200 startup requests each, muted launch, stable screenshots, no browser errors, and four closed browser processes. Production fixture-marker exclusion passed. Astra medium found no P0/P1 issue for this entry-only gate. The entry record confirms opposing ships and fighters. This is functional entry evidence only. The screenshots show the current replacement tactical HUD, with deployment instructions clipped by bottom controls at 640×480; no original tactical bitmap cell, battle command, or result/return journey has been accepted.
