---
title: "Plans Index"
description: "All feature plans, ExecPlans, and design proposals for Open Rebellion"
category: plan
created: 2026-03-22
updated: 2026-09-12
tags: [index, plans]
---

# Plans Index

## Naming Convention

All plans use: `YYYY-MM-DD-{type}-{descriptive-name}.md`

| Type | Meaning |
|------|---------|
| `feat` | New feature implementation |
| `fix` | Bug fix |
| `refactor` | Code improvement without behavior change |
| `design` | Architecture/design proposal (not yet scheduled) |
| `addon` | Post-release addon feature |
| `tooling` | Test, evidence, and developer-workflow implementation |

Completed plans live in `completed/`.

## Active Plans

The [Batched Interface Parity Execution Plan](2026-09-11-feat-batched-interface-parity-plan.md)
is the current primary implementation plan. It groups related original-game
surfaces into browser-verifiable bundles while the interface audit remains the
strict acceptance authority. Its
[acceleration sidecar](2026-09-11-tooling-interface-parity-acceleration-sidecar.md)
defines the deterministic harness and one-gate family cadence.

| Plan | Type | Status | Date |
|------|------|--------|------|
| [Batched Interface Parity](2026-09-11-feat-batched-interface-parity-plan.md) | feat | active, P55 tactical runtime conversion complete; P56 render proof next | 2026-09-11 |
| [Interface Parity Acceleration Sidecar](2026-09-11-tooling-interface-parity-acceleration-sidecar.md) | tooling | active, GID pilot next | 2026-09-11 |
| [Standalone Space-Battle Test Launcher](2026-09-12-tooling-standalone-space-battle-launcher.md) | tooling | active, T0 result identity verified; entry and visuals open | 2026-09-12 |
| [Tactical 3D Asset Pipeline](2026-09-12-feat-tactical-3d-asset-pipeline.md) | feat | active, P55 runtime store complete; P56 render proof next | 2026-09-12 |
| [AI Attack Target Variety](2026-03-21-feat-ai-attack-target-variety-plan.md) | feat | complete | 2026-03-21 |
| [Autoresearch Metrics](2026-03-21-autoresearch-metrics-plan.md) | feat | planned | 2026-03-21 |
| [Create Your Own Character](2026-03-21-addon-create-your-own-character.md) | addon | planned | 2026-03-21 |
| [AI Fog of War Proposal](2026-03-22-ai-fog-of-war-proposal.md) | design | draft | 2026-03-22 |
| [AI Fog of War — Nomos Blueprint](2026-03-22-ai-fog-of-war-nomos-blueprint.md) | design | planned | 2026-03-22 |
| [Faithful HD Pipeline](2026-09-10-faithful-hd-pipeline/) | feat | foundation complete; P45B–P45E pending | 2026-09-10 |

## Completed Plans

| Plan | Type | Version | Date |
|------|------|---------|------|
| [War Machine](completed/2026-03-15-feat-war-machine-v0.4.0.md) | feat | v0.4.0 | 2026-03-15 |
| [Codex Review Fixes](completed/2026-03-15-fix-v0.4.1-codex-review-fixes.md) | fix | v0.4.1 | 2026-03-15 |
| [Mod Workshop](completed/2026-03-16-feat-v0.6.0-mod-workshop.md) | feat | v0.6.0 | 2026-03-16 |
| [Playtest Observability](completed/2026-03-20-feat-playtest-observability-execplan.md) | feat | v0.13.0 | 2026-03-20 |
