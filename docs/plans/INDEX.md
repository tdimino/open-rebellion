---
title: "Plans Index"
description: "All feature plans, ExecPlans, and design proposals for Open Rebellion"
category: plan
created: 2026-03-22
updated: 2026-10-15
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

Completed plans live in `completed/`; retired plans live in `archive/`. Frontmatter keys (`status`, `progress`, `superseded_by`, `merged_into`) are defined in [README.md](README.md).

## Active Plans

The [Batched Interface Parity Execution Plan](2026-09-11-feat-batched-interface-parity-plan.md)
is the umbrella interface plan; the interface audit remains the strict
acceptance authority. The
[Tactical 106-Cell Acceptance Train](2026-09-22-tooling-tactical-106-cell-acceptance.md)
owns the space battle. On 2026-10-15 an applicability review archived 17
plans whose work was done, moved or overtaken; see Archived Plans.

| Plan | Type | Status | Date |
|------|------|--------|------|
| [Original Research, the Strategic AI and the Open Items](2026-10-15-feat-original-ai-and-research.md) | feat | completed (100%); Phase B research, C5 production, the C2-C4 specs, Phase D (or-pfv4, native on both sides), the Personnel icon layers, the story overlay removal, C2 (module 13), C3 (module 8 records and postures), C4 (fleet tasks, class I officer tasks), the daily cadence (save v46), provenance citations, the golden, the review fixes and or-pfrm (the original control rule) the Death Star class fix and the blockade timing done, or-glv7 closed, uncommitted; or-lzz3 answered; Phase G tools built and checked live, review fixes done; the Empire research check passed; Phase H done (module order, repair, research planners, load determinism; golden for the four causes); production and rank-lapse checks passed, blue rectangle closed; the morale retreat (FUN_005d19a0) ported and checked on both sides; the engaged-fleet lock and the en-route mark (the seed-9 shuttle gone); save v50, v51; the AI round scheduler mapped; the posture trace and the native siege-hold check (both sides); the VM recheck (the winner leaves within four days); the blockaded-yard question settled; scheduler steps 1-3 and missions refused at creation (save v52); timer 0x386 checked in the VM; step 4 (native rounds, the advisor's slot, every dispatch validated, native checks on both sides); the support-holder, timer 0x386 and bit 11 fixes done (native on both sides); the neutral seeding fix (FUN_00559700); one golden for every cause since v50; provenance baseline empty; the scoped mutants closed or equivalent; fast-forwarded onto origin/main, every gate passing, one commit | 2026-10-15 |
| [Strategic Window Modality and Focus](2026-10-10-feat-strategic-window-modality-and-focus.md) | feat | active; 44% — workstreams 1-2 done; workstream 3 has the egui adapter and Fleet (PR #45, 9533b516); Sector, System, Defenses and Missions remain | 2026-10-10 |
| [Strategic Windowing Toolkit Implementation](2026-10-10-strategic-windowing-toolkit-implementation.md) | feat | active; 44% — tasks 0-5B merged; PR #45 (9533b516) migrated the Fleet family (its share of 7A-7C, capture cancellation including focus loss); task 6 Sector next | 2026-10-10 |
| [Resource Stockpiles and Top-Bar Counters](2026-10-08-feat-resource-stockpiles.md) | feat | active; 75% — phases 1-3 landed (d7119926, 4f280da8); narrowed to phase 4's both-faction native and browser monitor checks plus the known gaps | 2026-10-08 |
| [Tactical 106-Cell Acceptance Train](2026-09-22-tooling-tactical-106-cell-acceptance.md) | tooling | active; 57% — C1, C2, C5 and C6 landed (A0 sourced for all 106; C6 pass 11, fail 66, not-run 28, settled 1); C3, C4 and C7 open; strict acceptance 11 of 106 ([capture index](../qa/2026-09-10-interface-parity-audit/evidence/2026-10-13-tactical-a0-capture-index.md), [task graph](../../agent_docs/task-graph.md)) | 2026-09-22 |
| [Batched Interface Parity](2026-09-11-feat-batched-interface-parity-plan.md) | feat | active; 2% — 1 of 43 families strictly complete (OBJ-01); 15 partial, 27 failing; 13 of 627 cells accepted; no UIP-B01 to B07 exit gate met | 2026-09-11 |
| [Faithful HD Pipeline](2026-09-10-faithful-hd-pipeline/) | feat | partial (parked until original parity closes); 20% — P45A landed (b7c9811c); P45C advisor family gates offline only (7ce3a4c7); P45B, P45D and P45E pending; no HD work since 2026-09-14 | 2026-09-10 |
| [Create Your Own Character](2026-03-21-addon-create-your-own-character.md) | addon | planned; 0% — post-v1 design, absorbs Homeworld Recruitment Mode; must be revised against mod-surface rules 1-3 first; no homeworld field, Academy, creation UI, Force table or custom-portrait pipeline has landed | 2026-03-21 |

## Archived Plans

Plans that no longer drive work. Each keeps its last progress line and records why in `archive_reason`; `merged_into` names the plan that took its open items.

| Plan | Type | Last progress | Archived | Why |
|------|------|---------------|----------|-----|
| [Knesset Sassuratu](archive/2026-03-24-002-knesset-sassuratu-visual-qa-parity.md) | feat | 53% | 2026-10-15 | Tests egui panels and hotkeys the original-UI work removed; its SEO targets a Ghidra site that was never published |
| [Autoresearch Metrics](archive/2026-03-21-autoresearch-metrics-plan.md) | feat | 60% | 2026-10-15 | Tunes sourceless augmentation knobs the full-parity direction replaces; tooling idle since 2026-04-01 |
| [AI Fog of War Proposal](archive/2026-03-22-ai-fog-of-war-proposal.md) | design | 0% | 2026-10-15 | Omniscience premise refuted; AI visibility is or-q-port-observation-levels-jgy |
| [AI Fog of War — Nomos Blueprint](archive/2026-03-22-ai-fog-of-war-nomos-blueprint.md) | design | 0% | 2026-10-15 | Sourceless fog layer on AI scoring being replaced; premise refuted |
| [Homeworld Recruitment Mode](archive/2026-03-22-design-homeworld-recruitment-mode.md) | design | 0% | 2026-10-15 | Merged into Create Your Own Character |
| [Interface Parity Acceleration Sidecar](archive/2026-09-11-tooling-interface-parity-acceleration-sidecar.md) | tooling | 67% | 2026-10-15 | Tooling done; ACC-06 moved to Batched UIP-B02 |
| [P59 Parallel Strategic Lane](archive/2026-09-14-feat-p59-parallel-gid-message-index-lane.md) | feat | 25% | 2026-10-15 | Lane premise gone; GID claim table and guard moved to Batched UIP-B02 |
| [Tactical 3D Asset Pipeline](archive/2026-09-12-feat-tactical-3d-asset-pipeline.md) | feat | 80% | 2026-10-15 | Implemented through P58-B22; A0 view gates moved to the 106-cell plan and br |
| [Standalone Space-Battle Test Launcher](archive/2026-09-12-tooling-standalone-space-battle-launcher.md) | tooling | 67% | 2026-10-15 | Built; T3-T5 are the 106-cell plan's C3, C4 and C7 |
| [Complete Game UI Rebuild](archive/2026-03-22-001-feat-complete-game-ui-rebuild-plan.md) | feat | 92% | 2026-10-15 | Targets removed egui panels; unbuilt items are invented UI |
| [UI Parity ExecPlan](archive/2026-03-23-002-feat-ui-parity-execplan.md) | feat | 86% | 2026-10-15 | M1-M6 landed; M7 moved to the Faithful HD Pipeline |
| [100% Parity Plan](archive/2026-03-25-002-100-percent-parity-plan.md) | feat | 85% | 2026-10-15 | The September audit replaced its parity claim |
| [Upscale All DLL BMPs](archive/2026-03-25-003-upscale-all-dll-bmps-execplan.md) | feat | 50% | 2026-10-15 | Bulk upscaling abandoned for the Faithful HD Pipeline |
| [Final Parity Roadmap](archive/2026-03-31-001-final-parity-roadmap-execplan.md) | feat | 68% | 2026-10-15 | Duplicates the roadmap; acceptance moved to the audit |
| [Knesset Shamash-Bet](archive/2026-04-08-001-feat-knesset-shamash-bet-story-events-cutscene-plan.md) | feat | 85% | 2026-10-15 | Superseded by the completed Tammuz; targets gone or replaced |
| [Port Dispatch Validators](archive/2026-03-23-feat-port-dispatch-validators-execplan.md) | feat | 79% | 2026-10-15 | The 'validators' are resource adjusters, ported as economy steps |
| [Resolve AI Guesses via Ghidra RE](archive/2026-03-23-001-feat-resolve-ai-guesses-ghidra-re-plan.md) | feat | 78% | 2026-10-15 | Open tasks rest on misreadings; cadence moved to the original-AI plan |
| [Knesset Resheph](archive/2026-04-03-001-feat-knesset-resheph-final-sprint-plan.md) | feat | 92% | 2026-10-15 | P44 overturned its last task; its fleet heuristics are replaced |
| [Test, Lint and Build Gates](archive/2026-04-06-001-test-lint-build-infrastructure-execplan.md) | test | 10% | 2026-10-15 | GitHub Actions design no longer matches; F-009 and M5 own the gates |

## Completed Plans

| Plan | Type | Version | Date |
|------|------|---------|------|
| [Will Forster Encyclopedia Handoff Adaptation](completed/2026-10-02-feat-will-forster-encyclopedia-handoff-adaptation.md) | feat | W0-W7 | 2026-10-02 |
| [Encyclopedia Data Extraction, Modding, and Display](completed/2026-09-27-design-encyclopedia-data-pipeline.md) | design | E35 | 2026-09-27 |
| [Knesset Tammuz](completed/2026-04-12-001-feat-knesset-tammuz-full-parity-sprint-plan.md) | feat | — | 2026-04-12 |
| [Knesset Ptah](completed/2026-03-28-001-feat-knesset-ptah-todo-resolution-plan.md) | feat | — | 2026-03-28 |
| [PerceptionIntegrator Extraction](completed/2026-03-27-refactor-perception-integrator-extraction-plan.md) | refactor | — | 2026-03-27 |
| [Knesset Ereshkigal](completed/2026-03-26-001-feat-eval-driven-parity-open-souls-refactor-plan.md) | feat | — | 2026-03-26 |
| [Knesset Kothar](completed/2026-03-25-001-knesset-final-parity.md) | feat | — | 2026-03-25 |
| [Game Seeding Parity](completed/2026-03-24-003-game-seeding-parity-execplan.md) | feat | — | 2026-03-24 |
| [Knesset Demiurge](completed/2026-03-24-001-knesset-demiurge-ui-parity.md) | feat | — | 2026-03-24 |
| [AI Attack Target Variety](completed/2026-03-21-feat-ai-attack-target-variety-plan.md) | feat | — | 2026-03-21 |
| [War Machine](completed/2026-03-15-feat-war-machine-v0.4.0.md) | feat | v0.4.0 | 2026-03-15 |
| [Codex Review Fixes](completed/2026-03-15-fix-v0.4.1-codex-review-fixes.md) | fix | v0.4.1 | 2026-03-15 |
| [Mod Workshop](completed/2026-03-16-feat-v0.6.0-mod-workshop.md) | feat | v0.6.0 | 2026-03-16 |
| [Playtest Observability](completed/2026-03-20-feat-playtest-observability-execplan.md) | feat | v0.13.0 | 2026-03-20 |
