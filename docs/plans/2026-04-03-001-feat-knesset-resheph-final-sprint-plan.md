---
title: "Knesset Resheph — Final Sprint"
type: feat
status: active
date: 2026-04-03
project: open-rebellion
tags: [knesset, swarm, combat, ai, wasm, polish, final-sprint]
---

# Knesset Resheph — Final Sprint

*Named after Resheph (רשף), Canaanite god of war and plague — the firebrand who finishes what remains.*

## Purpose

Clear all 12 remaining tasks from the Open Rebellion backlog in a single Minoan Swarm knesset. This pushes the project from **~97% parity** to **~99%** — the only gaps after this sprint will be Ghidra RE work (Death Star GNPRTB params) and media playback (video).

## Prior Art

| Knesset | Agents | Tasks | Duration | LOC |
|---------|--------|-------|----------|-----|
| Athirat | 4 | 18 | ~45 min | ~11.7K |
| Demiurge | 5 | 13 | ~3h | ~3.8K |
| Ma'at | 5 | ~10 | ~2h | combat+assets |
| Ptah | 1 (lead) | 13 | ~2h | TODO resolution |

## Task Inventory (12 remaining)

| # | Task ID | Domain | Description | Blocked By | Est LOC |
|---|---------|--------|-------------|------------|---------|
| 1 | #124 | Combat | Per-arc weapon fire (replace ±20% approx) | — (in progress) | ~80 |
| 2 | #125 | Combat | Death Star construction/warning exact values | #124 | ~30 |
| 3 | #126 | AI | Port remaining 14 AI dispatch validators | — | ~200 |
| 4 | #128 | AI | Faction-specific deployment budgets in AiConfig | — | ~60 |
| 5 | #129 | AI | Strategic AI deltas (uprising prevention, DS escort) | #126 | ~150 |
| 6 | #130 | UI/Data | DLL lookup tables for encyclopedia art, tactical sprites | — | ~200 |
| 7 | #131 | Media | Native video playback for decoded WebM cutscenes | — | ~300 |
| 8 | #132 | UI | BIN-driven advisor animation (replace frame-cycling) | — | ~150 |
| 9 | #133 | WASM | Save/load via IndexedDB/localStorage | — (in progress) | ~120 |
| 10 | #134 | WASM | BmpCache texture loading (cockpit, portraits, events) | — | ~100 |
| 11 | #135 | WASM | Audio byte loading in browser path | — | ~80 |
| 12 | #136 | Eval | Build golden-value oracle for eval_parity.py | — | ~200 |

## Scope Boundaries

- **In scope**: All 12 tasks above
- **Out of scope**: Ghidra RE for exact Death Star GNPRTB parameters (F4 uses best available approximations)
- **Out of scope**: HD asset upscaling (visual polish, separate effort)
- **Risk**: #131 (video playback) may require external crate evaluation — if blocked, defer to post-sprint

## 5 Daborot

Tasks cluster naturally into 5 non-overlapping domains by file ownership.

### Dabora 1: **Anat** (ענת Lady of Battle) — Combat & Gameplay

*Model: opus (complex formula work)*

| Task | Description | Files (OWN) | Blocked By |
|------|-------------|-------------|------------|
| #124 | Per-weapon-type damage (turbolaser/ion/laser attack strengths) | `combat.rs` | — |
| #125 | Death Star construction/warning best-available values | `death_star.rs` | #124 |

**Approach**: Wire `turbolaser_attack_strength`, `ion_cannon_attack_strength`, `laser_cannon_attack_strength` from `CapitalShipClass` into `phase_weapon_fire()`. Each weapon type's total damage = sum(arcs) * attack_strength * weapon_nibble / 15. For #125, review GNPRTB indices near 0x1400 range and update placeholders where data exists.

### Dabora 2: **Kaptaru** (כפתור Caphtor) — AI Parity

*Model: sonnet (pattern porting, high throughput)*

| Task | Description | Files (OWN) | Blocked By |
|------|-------------|-------------|------------|
| #126 | Port 14 AI dispatch validators from Ghidra decompilation | `ai.rs` | — |
| #128 | Add faction-specific deployment budgets to AiConfig | `ai.rs`, `tuning.rs` | — |
| #129 | Strategic AI deltas (uprising prevention, resource balancing, DS escort) | `ai.rs` | #126 |

**Approach**: #126 ports the capacity/composition checks from the 18 decoded sub-functions (4 already done, 14 remaining). Reference `docs/plans/2026-03-23-feat-port-dispatch-validators-execplan.md`. #128 adds `alliance_deploy_budget` / `empire_deploy_budget` to `AiConfig`. #129 adds uprising garrison reinforcement, DS escort logic, and resource-balancing heuristics.

**Note**: `tuning.rs` is in `forbidden_files` for autoresearch but NOT for direct development. Kaptaru may modify it.

### Dabora 3: **Shapash** (שפש Torch of the Gods) — UI & Polish

*Model: sonnet (UI rendering, BIN parsing)*

| Task | Description | Files (OWN) | Blocked By |
|------|-------------|-------------|------------|
| #130 | DLL resource lookup tables for encyclopedia/tactical art | `bmp_cache.rs`, `encyclopedia.rs`, `tactical_view.rs` | — |
| #132 | BIN-driven advisor animation (replace frame-cycling fallback) | `advisor.rs` | — |
| #131 | Native video playback for decoded WebM cutscenes | `video.rs` (NEW) | — |

**Approach**: #130 maps DLL resource IDs to BMP texture names using `agent_docs/dll-resource-catalog.md` as reference. #132 parses the advisor BIN format (partially decoded) to drive C-3PO/R2-D2 frame sequences. #131 evaluates `symphonia` or `gstreamer-rs` for WebM decoding — if blocked by dependency complexity, stub with a "cutscene skipped" message and defer.

### Dabora 4: **Mami** (ממי Mother of the Gods) — WASM Browser Path

*Model: sonnet (platform integration)*

| Task | Description | Files (OWN) | Blocked By |
|------|-------------|-------------|------------|
| #133 | Save/load via IndexedDB/localStorage | `save.rs` (WASM path) | — |
| #134 | BmpCache texture loading from staged web/data/ | `bmp_cache.rs` (WASM path) | — |
| #135 | Audio byte loading in browser path | `audio.rs` (WASM path) | — |

**Approach**: All three tasks add `#[cfg(target_arch = "wasm32")]` branches. #133 uses `web-sys` IndexedDB or localStorage for save slots. #134 loads BMPs from the `web/data/base/` staging directory via fetch API. #135 loads WAV/audio via the macroquad WASM audio subsystem (quad-snd already supports WebAudio).

**Constraint**: Mami ONLY touches WASM-gated code paths. Native paths owned by other daborot remain untouched.

### Dabora 5: **Sassuratu** (ששורתו Midwife) — Eval & Integration

*Model: sonnet (Python scripting, integration testing)*

| Task | Description | Files (OWN) | Blocked By |
|------|-------------|-------------|------------|
| #136 | Build golden-value oracle for eval_parity.py from community disassembly | `scripts/eval_parity.py`, `scripts/golden_values.json` (NEW) | ALL |

**Approach**: Cross-reference `docs/reports/2026-03-26-community-disassembly-cross-reference.md` against playtest JSONL output to build expected value ranges for each parity sub-metric. Run 3-seed validation after all other daborot complete. Update `eval_parity.py` to optionally validate against golden values.

**Starts**: After Anat, Kaptaru, and Mami complete (depends on gameplay + AI + WASM changes).

## File Ownership Matrix

| File | Anat | Kaptaru | Shapash | Mami | Sassuratu |
|------|------|---------|---------|------|-----------|
| `combat.rs` | **OWN** | — | — | — | read |
| `death_star.rs` | **OWN** | — | — | — | read |
| `ai.rs` | — | **OWN** | — | — | read |
| `tuning.rs` | — | **OWN** | — | — | read |
| `bmp_cache.rs` | — | — | **OWN** (native) | **OWN** (WASM) | read |
| `advisor.rs` | — | — | **OWN** | — | read |
| `encyclopedia.rs` | — | — | **OWN** | — | read |
| `tactical_view.rs` | — | — | **OWN** | — | read |
| `video.rs` (NEW) | — | — | **OWN** | — | read |
| `save.rs` | — | — | — | **OWN** (WASM) | read |
| `audio.rs` | — | — | — | **OWN** (WASM) | read |
| `scripts/eval_parity.py` | — | — | — | — | **OWN** |

**Conflict avoidance**: Mami and Shapash share `bmp_cache.rs` and `audio.rs` but are partitioned by `#[cfg]` gates. Mami writes only inside `#[cfg(target_arch = "wasm32")]` blocks; Shapash writes only in non-WASM code.

## Dependency Graph & Critical Path

```
Phase 1 (parallel, no deps):
  Anat #124 (per-arc weapons, ~1h) ─────────────────────┐
  Kaptaru #126 (14 validators, ~2h) ────────────────────┐│
  Kaptaru #128 (faction budgets, ~30m) ─────────────────┐││
  Shapash #130 (DLL lookups, ~1.5h) ───────────────────┐│││
  Shapash #132 (advisor BIN, ~1.5h) ──────────────────┐││││
  Shapash #131 (video playback, ~2h) ────────────────┐│││││
  Mami #133 (WASM save/load, ~1.5h) ───────────────┐││││││
  Mami #134 (WASM BmpCache, ~1h) ─────────────────┐│││││││
  Mami #135 (WASM audio, ~1h) ───────────────────┐││││││││
                                                  │││││││││
Phase 2 (after deps):                            │││││││││
  Anat #125 (DS values, ~30m, after #124) ──────┐│││││││││
  Kaptaru #129 (strategic AI, ~1.5h, after #126)┐││││││││││
                                                ││││││││││
Phase 3 (after all):                            ││││││││││
  Sassuratu #136 (golden oracle) ───────────────┘┘┘┘┘┘┘┘┘┘

Critical path: Kaptaru #126 (2h) → #129 (1.5h) → Sassuratu #136 (1h) = ~4.5h
```

## Execution Model

- **Lead**: Opus (this session) — orchestrates, reviews, merges
- **Daborot 1-4**: Sonnet subagents via Agent Teams (isolation: worktree for file safety)
- **Dabora 5**: Runs after Phase 1+2 complete, in main session or as final subagent
- **Build verification**: `PATH="/usr/bin:$PATH" cargo test -p rebellion-core -p rebellion-data` after each merge
- **Test floor**: Must maintain 405+ tests (currently 405)

## Acceptance Criteria

- [ ] All 12 tasks marked completed
- [ ] `cargo check` passes with zero errors
- [ ] `cargo test -p rebellion-core -p rebellion-data` passes (405+ tests)
- [ ] WASM build succeeds: `bash scripts/build-wasm.sh`
- [ ] Per-weapon-type damage differentiates turbolaser/ion/laser in combat
- [ ] Covert missions can be foiled by enemy counter-intelligence (F2, already done)
- [ ] AI uses faction-specific deployment budgets
- [ ] AI validates dispatch capacity before deploying fleets
- [ ] Advisor droid uses BIN-driven animation frames
- [ ] WASM save/load functional in browser
- [ ] eval_parity.py has golden-value oracle mode

## Success Metrics

- Task backlog: 12 → 0 pending
- Test count: 405 → 420+ (estimated ~15 new tests across all daborot)
- Overall parity: ~97% → ~99%
- WASM functional: save/load + textures + audio working in browser

## Risks & Mitigations

| Risk | Mitigation |
|------|-----------|
| Video playback (#131) blocked by crate deps | Stub with skip message, defer to post-sprint |
| Advisor BIN format (#132) insufficiently decoded | Fall back to improved frame-cycling with correct frame count |
| AI validators (#126) reference entity offsets not in our types | Port what maps cleanly, stub remainder with comments |
| WASM IndexedDB (#133) async complexity | Use localStorage as simpler fallback, upgrade later |
| File conflicts between Mami/Shapash on bmp_cache.rs | Strict cfg-gate partitioning, lead reviews before merge |

## Sources & References

### Internal
- Prior Knessets: [Athirat report](docs/reports/2026-03-13-knesset-athirat-swarm-report.md), [Demiurge plan](docs/plans/2026-03-24-001-knesset-demiurge-ui-parity.md), [Ptah plan](docs/plans/2026-03-28-001-feat-knesset-ptah-todo-resolution-plan.md)
- AI validators: [dispatch-validators-execplan](docs/plans/2026-03-23-feat-port-dispatch-validators-execplan.md)
- DLL resources: [dll-resource-catalog](agent_docs/dll-resource-catalog.md)
- Ghidra RE: [ghidra-re](agent_docs/ghidra-re.md)
- Community disassembly: [cross-reference](docs/reports/2026-03-26-community-disassembly-cross-reference.md)
- Architecture: [architecture](agent_docs/architecture.md)
