---
title: "Knesset Kothar — Final Parity Sprint"
date: 2026-03-25
team: kothar
agents: 5 (Resheph, Shapash, Anat, Pidray, Yarikh)
model: claude-opus-4-6 (lead + Resheph), claude-sonnet-4-6 (others)
status: planned
tags: [swarm, knesset, seeding, combat-parity, polish, droid-advisor]
---

# Knesset Kothar — Final Parity Sprint

## Name & Etymology

**Kothar** (כתר) — The divine craftsman of Ugaritic mythology, Kothar wa Khasis ("Skillful and Wise"), builder of Baal's palace and forger of the divine weapons. This Knesset completes the palace — the final 9 tasks that stand between Open Rebellion and feature-complete parity with the 1998 original. Named for the craftsman-god because this is finishing work: seeding the galaxy correctly, arming ships with their full stat blocks, polishing the chrome, and breathing life into the droid advisors.

Previous Knessets: **Athirat** (Living Galaxy + War Room, 18 tasks, 4 agents), **Shapash** (Full Parity, v0.5.0), **Elat** (Mod Workshop, v0.6.0), **Hokhmah** (Release, v0.7.0), **Demiurge** (UI Parity, 13 tasks, 5 agents), **Sassuratu** (Visual QA, 47 tests).

---

## Daborot (Work Teams)

### 1. Resheph — Seeding Forge

| Field | Value |
|-------|-------|
| **Greek/Semitic** | רשף — Resheph, god of plague and war. Chosen because seeding determines the opening state of every war. |
| **Model** | **Opus 4.6** — Sequential chain with complex parameter algebra, GNPRTB/SDPRTB cross-references, and save-format impact analysis. Sonnet-class risk of silent formula errors is too high. |
| **Tasks** | #17 (M5), #18 (M6), #19 (M7), #20 (M8) — sequential |
| **Start condition** | Immediate — no dependencies |
| **Budget** | 50 tool calls |
| **Specialty** | Procedural galaxy initialization, parameter table algebra, deterministic RNG |

**Task chain** (strictly sequential — each milestone builds on the previous):

| Order | Task | Description | Est. LOC | Tests |
|-------|------|-------------|----------|-------|
| 1 | **#17 — M5: Support/popularity init** | Map SDPRTB 7682-7685 + GNPRTB 7764-7765 to dual-popularity floats. Core/rim split. Controlled systems 20-100, neutral 41-59. | ~180 | 3 |
| 2 | **#18 — M6: Control buckets + facilities** | Procedural political control from GNPRTB 7680/7681. Energy/raw materials from GNPRTB 7721-7727. Facility generation from SYFCCRTB/SYFCRMTB. | ~250 | 4 |
| 3 | **#19 — M7: Maintenance-budget seeding** | SDPRTB 5168-5170 budget loop. CMUNALTB/CMUNEMTB unit bundles. Low-support garrison from GNPRTB 7761-7762. | ~300 | 3 |
| 4 | **#20 — M8: Integration + regression** | Wire SeedOptions through app/playtest entry points. Replace post-hoc control derivation. Save-version policy. Full test sweep. | ~100 | 5 |

**File ownership** (exclusive):

| File | Access | Why |
|------|--------|-----|
| `crates/rebellion-data/src/seeds.rs` | **Write** | All seeding logic lives here |
| `crates/rebellion-core/src/world/mod.rs` | **Write** (System struct only) | New seeding fields: support initialization, resource fields |
| `crates/rebellion-data/src/lib.rs` | **Write** (loader ordering only) | Move GNPRTB/SDPRTB load before seeding |
| `crates/rebellion-data/src/save.rs` | **Write** | Save-version bump for new System fields |
| `crates/rebellion-app/src/main.rs` | **Write** (SeedOptions threading) | Pass galaxy size + difficulty to loader |
| `crates/rebellion-playtest/src/main.rs` | **Write** (SeedOptions threading) | Same |
| `data/base/json/GNPRTB.json` | **Read** | Parameter reference |
| `data/base/json/SDPRTB.json` | **Read** | Parameter reference |

**Key constraints**:
- All new tests must use fixed RNG seeds (deterministic reproducibility)
- Dual-popularity floats preserved — seeding maps original 0-100 support into existing model
- `rebellion-core` stays headless-testable (no rendering deps)
- ExecPlan at `docs/plans/2026-03-24-003-game-seeding-parity-execplan.md` is the authoritative spec

---

### 2. Shapash — Combat Armorer

| Field | Value |
|-------|-------|
| **Semitic** | שפש — Shapash, the sun-goddess who sees all. Chosen because she illuminates what was hidden — the 15 DAT fields that exist in the binary but never reached the world model. |
| **Model** | **Sonnet 4.6** — Mechanical field promotion (read DAT struct → add to world struct → update conversion). Well-defined, low ambiguity. |
| **Tasks** | #21 (Ship class fields) |
| **Start condition** | Immediate — independent of all other daborot |
| **Budget** | 30 tool calls |
| **Specialty** | DAT-to-world type promotion, combat system enrichment |

**Task #21 — Promote 15 missing DAT fields**

The DAT parser (`CapitalShip` in `tools/dat-dumper/src/types/capital_ships.rs`) has 51 fields. The world model (`CapitalShipClass` in `world/mod.rs`) has ~27. The gap:

**CapitalShipClass missing fields** (from CAPSHPSD.DAT):

| # | DAT field | Purpose | Combat system usage |
|---|-----------|---------|---------------------|
| 1 | `uprising_defense` | Defense against uprisings when garrisoned | uprising.rs |
| 2 | `hyperdrive_if_damaged` | Fallback speed when damaged | movement.rs, combat retreat |
| 3 | `turbolaser_range` | Effective range for turbolasers | tactical_view.rs |
| 4 | `ion_cannon_range` | Effective range for ion cannons | tactical_view.rs |
| 5 | `laser_cannon_range` | Effective range for lasers | tactical_view.rs |
| 6 | `turbolaser_attack_strength` | Per-weapon-type attack power | combat.rs |
| 7 | `ion_cannon_attack_strength` | Per-weapon-type attack power | combat.rs |
| 8 | `laser_cannon_attack_strength` | Per-weapon-type attack power | combat.rs |
| 9 | `overall_attack_strength` | Aggregate attack stat | combat.rs (auto-resolve) |
| 10 | `tractor_beam_power` | Tractor beam strength | combat.rs (escape prevention) |
| 11 | `tractor_beam_range` | Tractor beam effective range | combat.rs |
| 12 | `gravity_well_projector` | Interdictor capability flag | movement.rs (interdiction) |
| 13 | `interdiction_strength` | Interdiction pull strength | movement.rs |
| 14 | `weapon_recharge_rate` | Weapons recharge per round | combat.rs (sustained fire) |
| 15 | `production_family` | Production chain grouping | manufacturing.rs |

**FighterClass missing fields** (from FIGHTSD.DAT):

| # | DAT field | Purpose |
|---|-----------|---------|
| 1 | `research_order` | Tech tree position |
| 2 | `research_difficulty` | Research cost |
| 3 | `uprising_defense` | Garrison defense |
| 4 | `detection` | Sensor range |
| 5 | `shield_strength` | Fighter shields |
| 6 | `sub_light_engine` | Speed |
| 7 | `maneuverability` | Dodge chance |
| 8 | `hyperdrive` | Hyperspace capability |
| 9 | `turbolaser_*` (12 fields) | Weapon arcs |
| 10 | `*_range` (3 fields) | Weapon ranges |
| 11 | `*_attack_strength` (3 fields) | Weapon power |
| 12 | `torpedoes_range` | Torpedo effective range |

**File ownership** (exclusive):

| File | Access |
|------|--------|
| `crates/rebellion-core/src/world/mod.rs` | **Write** (CapitalShipClass + FighterClass structs only — NO System struct, that's Resheph's) |
| `crates/rebellion-data/src/lib.rs` | **Write** (conversion code for new fields in `build_capital_ship_class` / `build_fighter_class` only) |
| `crates/rebellion-core/src/combat.rs` | **Read** (verify which fields combat already uses) |

**Overlap prevention**: Shapash touches `world/mod.rs` for ship/fighter structs only. Resheph touches it for System struct only. These are non-overlapping struct definitions within the same file. If merge conflicts arise, Resheph's changes take priority since they're on the critical path.

**Estimated**: ~120 LOC (struct additions + conversion), 2-3 tests verifying field promotion round-trips correctly.

---

### 3. Anat — Polish & QA

| Field | Value |
|-------|-------|
| **Semitic** | ענת — Anat, the warrior-goddess and quality enforcer. Chosen for mixed combat-and-craft work: WASM debugging, CI dependency fixing, HD art pipeline. |
| **Model** | **Sonnet 4.6** — Three independent, well-scoped tasks. No deep algorithm design. |
| **Tasks** | #22 (DEF-004 WASM timing), #23 (Ghidra site CI), #24 (HD visual polish) |
| **Start condition** | Immediate — all independent |
| **Budget** | 40 tool calls |
| **Specialty** | Cross-platform debugging, CI/CD, asset pipeline |

**Task #22 — DEF-004: Right-click context menu WASM timing**

Investigation task. The defect was found by automated browser QA (Knesset Sassuratu) — right-click context menus don't trigger in WASM. Native macOS works fine. Hypothesis: macroquad's `is_mouse_button_released(Right)` timing differs between native and WASM event handling.

Steps:
1. Read `crates/rebellion-render/src/lib.rs` around line 258 (right-click detection)
2. Check macroquad's WASM input handling for `MouseButtonDown`/`MouseButtonUp` event delivery
3. If timing issue: add `is_mouse_button_pressed` fallback or frame-delay detection
4. If not reproducible: document as browser-automation artifact, downgrade to P3
5. Build WASM and test: `bash scripts/build-wasm.sh`

**File ownership**:

| File | Access |
|------|--------|
| `crates/rebellion-render/src/lib.rs` | **Write** (input handling section only) |
| `scripts/build-wasm.sh` | **Read** |

**Task #23 — Ghidra site: social plugin Pillow/CairoSVG**

The mkdocs-material `social` plugin (line 61-65 in `ghidra-site/mkdocs.yml`) generates Open Graph social cards. It requires `Pillow` and `CairoSVG` Python packages. These are likely missing in CI.

Steps:
1. Check if `ghidra-site/requirements.txt` exists and includes these deps
2. Create or update requirements file with `mkdocs-material`, `pillow`, `cairosvg`
3. Verify `mkdocs build` succeeds locally in the ghidra-site directory
4. Check for GitHub Actions workflow that needs updating

**File ownership**:

| File | Access |
|------|--------|
| `ghidra-site/mkdocs.yml` | **Read** |
| `ghidra-site/requirements.txt` | **Write** (create/update) |
| `.github/workflows/*` | **Write** (if CI exists for ghidra-site) |

**Task #24 — HD visual polish: upscaled BMPs**

Asset generation task. Uses existing tools (waifu2x at `~/.local/bin/waifu2x`).

Steps:
1. Identify highest-impact BMPs: cockpit frames (STRATEGY.DLL), control buttons (COMMON.DLL), character portraits (GOKRES.DLL)
2. Batch upscale with waifu2x: `waifu2x -i input_dir/ -o output_dir/ -n 1 -s 4 -f png`
3. Place upscaled PNGs in `data/hd/` for BmpCache HD fallback path
4. Verify BmpCache loads HD PNGs when available (existing `DllSource` + HD fallback code)

**File ownership**:

| File | Access |
|------|--------|
| `data/hd/` | **Write** (new directory for upscaled assets) |
| `crates/rebellion-render/src/bmp_cache.rs` | **Read** (verify HD fallback path) |
| No Rust source changes expected — pipeline is already built |

---

### 4. Pidray — Droid Artificer

| Field | Value |
|-------|-------|
| **Semitic** | פדרי — Pidray ("Flashy One"), daughter of Baal, associated with light and manifestation. Chosen because the droid advisor must manifest visually — 148 animation frames brought to life from reverse-engineered BIN data. |
| **Model** | **Sonnet 4.6** with Opus escalation if BIN format proves complex |
| **Tasks** | #25 (Droid Advisor system, Phase 2.3) |
| **Start condition** | Immediate — independent, but largest standalone task |
| **Budget** | 50 tool calls |
| **Specialty** | Binary format RE, animation systems, egui integration |

**Task #25 — Droid Advisor System**

The original game has animated droid advisors: C-3PO + R2-D2 (Alliance), Imperial protocol droid (Empire). These are sprite animations stored across ALSPRITE.DLL (74 BMP + 716 BIN) and EMSPRITE.DLL (74 BMP + 713 BIN). The BIN resources contain animation control data (sequence definitions, frame timing, transition triggers).

**Phase 1: BIN Format Reverse Engineering** (~200 LOC)
1. Extract sample BIN resources from ALSPRITE.DLL using existing `scripts/extract-dll-resources.py`
2. Analyze BIN binary structure: look for frame indices, timing values, sequence markers
3. Cross-reference with known BMP resource IDs (2000-9000 range)
4. Document format in `agent_docs/droid-advisor-bin.md`

**Phase 2: Animation Runtime** (~150 LOC)
1. Define `DroidAdvisorState` in `rebellion-core` (current sequence, frame index, timer, faction)
2. Define animation sequence types (idle, speaking, reacting, celebrating, mourning)
3. BIN parser in dat-dumper or rebellion-data

**Phase 3: egui Integration** (~100 LOC)
1. Advisor panel in `rebellion-render/src/panels/advisor.rs`
2. Frame rendering via BmpCache (BMP sprites already extractable)
3. Trigger advisor animations on game events (battle won, system lost, research complete)
4. Keyboard shortcut (A) to toggle advisor panel

**File ownership** (exclusive):

| File | Access |
|------|--------|
| `crates/rebellion-core/src/advisor.rs` | **Write** (new — DroidAdvisorState, animation types) |
| `crates/rebellion-render/src/panels/advisor.rs` | **Write** (new — egui advisor panel) |
| `agent_docs/droid-advisor-bin.md` | **Write** (new — BIN format documentation) |
| `tools/dat-dumper/src/types/` | **Write** (new BIN parser if needed) |
| `scripts/extract-dll-resources.py` | **Read** |

**Risk**: BIN format is unresearched. If the format proves too complex for a single sprint (e.g., embedded bytecode or complex state machines), Pidray should implement a simplified version with static frame cycling and document the full format for a future pass.

**Estimated**: ~400-450 LOC total, 3-5 tests.

---

### 5. Yarikh — Verification Sentinel

| Field | Value |
|-------|-------|
| **Semitic** | ירח — Yarikh, the moon-god, patron of measurement and cycles. Chosen because verification is cyclical measurement — run, check, run again. |
| **Model** | **Sonnet 4.6** |
| **Tasks** | Post-implementation verification phase |
| **Start condition** | After ALL four implementation daborot complete |
| **Budget** | 30 tool calls |
| **Specialty** | Code review, silent failure detection, architecture validation |

Yarikh does not implement. Yarikh validates.

---

## Dependency & Parallelism Diagram

```
Time ──────────────────────────────────────────────────────────────────►

Resheph (Opus)   ┌─#17─M5──┐┌─#18─M6──┐┌─#19─M7──┐┌─#20─M8──┐
 (sequential)    └──────────┘└──────────┘└──────────┘└──────────┘
                                                                  ╲
Shapash (Sonnet) ┌──────#21─Ship Fields──────┐                    ╲
 (independent)   └───────────────────────────┘                     ╲
                                                                    ├── Yarikh
Anat (Sonnet)    ┌─#22─WASM─┐┌─#23─CI──┐┌──#24─HD──────┐         ╱   Verification
 (independent)   └──────────┘└─────────┘└──────────────┘         ╱
                                                                ╱
Pidray (Sonnet)  ┌──────────────#25─Droid Advisor──────────────┐
 (independent)   └─────────────────────────────────────────────┘
```

**Parallelism**: Resheph, Shapash, Anat, and Pidray all start simultaneously. Resheph runs its 4 tasks sequentially (M5→M6→M7→M8). The other three daborot run their tasks independently.

**Conflict prevention**: The only shared file is `crates/rebellion-core/src/world/mod.rs`, touched by both Resheph (System struct) and Shapash (CapitalShipClass + FighterClass structs). These are separate struct definitions — no line overlap. If a merge conflict occurs, resolve structurally (interleave the additions).

---

## File Ownership Matrix

| File | Resheph | Shapash | Anat | Pidray | Notes |
|------|---------|---------|------|--------|-------|
| `rebellion-core/src/world/mod.rs` | System | Ship/Fighter | — | — | Non-overlapping structs |
| `rebellion-data/src/seeds.rs` | **EXCL** | — | — | — | All seeding logic |
| `rebellion-data/src/lib.rs` | Loader ordering | Ship conversion | — | — | Different functions |
| `rebellion-data/src/save.rs` | **EXCL** | — | — | — | Version bump |
| `rebellion-app/src/main.rs` | SeedOptions | — | — | — | — |
| `rebellion-playtest/src/main.rs` | SeedOptions | — | — | — | — |
| `rebellion-core/src/combat.rs` | — | **Read** | — | — | Verify field usage |
| `rebellion-render/src/lib.rs` | — | — | Input fix | — | Right-click detection |
| `rebellion-render/src/bmp_cache.rs` | — | — | **Read** | — | Verify HD path |
| `rebellion-core/src/advisor.rs` | — | — | — | **NEW** | Droid state |
| `rebellion-render/src/panels/advisor.rs` | — | — | — | **NEW** | Advisor UI |
| `ghidra-site/requirements.txt` | — | — | **EXCL** | — | CI deps |
| `ghidra-site/mkdocs.yml` | — | — | **Read** | — | — |
| `data/hd/` | — | — | **EXCL** | — | Upscaled assets |
| `agent_docs/droid-advisor-bin.md` | — | — | — | **NEW** | BIN format docs |

---

## Verification Phase Protocol

After all four implementation daborot complete, Yarikh spawns **3 review subagents** in parallel:

### Review 1: Code Reviewer (Sonnet)

**Scope**: All files modified by Resheph, Shapash, Anat, Pidray.

**Checklist**:
- [ ] No `fieldN` names on semantically-known fields
- [ ] No rendering deps in rebellion-core
- [ ] All new public structs derive `Debug, Clone, Serialize, Deserialize`
- [ ] No slotmap keys serialized directly
- [ ] DatId preserved for all new entity references
- [ ] Code compiles: `PATH="/usr/bin:$PATH" cargo check`

### Review 2: Silent Failure Hunter (Sonnet)

**Scope**: Seeding (seeds.rs) + combat (combat.rs + world/mod.rs ship fields).

**Checklist**:
- [ ] No `unwrap()` on parameter lookups (GNPRTB/SDPRTB values may be absent)
- [ ] RNG seed determinism: same seed → identical galaxy state
- [ ] Budget loop terminates (no infinite loop if no legal placement exists)
- [ ] Popularity values clamped to [0.0, 1.0] (or [0, 100] depending on model)
- [ ] New ship fields default to 0 if DAT record is shorter than expected
- [ ] Save-version bump is paired with migration or intentional rejection

### Review 3: Architecture Strategist (Sonnet)

**Scope**: New modules (advisor.rs, droid-advisor-bin.md), modified interfaces.

**Checklist**:
- [ ] Advisor state is headless-testable (no rendering in rebellion-core)
- [ ] BIN parser follows DatRecord pattern (parse + write_bytes)
- [ ] SeedOptions flows cleanly from UI → loader → seeds without leaky abstractions
- [ ] No circular dependencies introduced
- [ ] New agent_docs files registered in agent_docs/INDEX.md

### Final Build Verification

After all 3 reviews complete:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion

# Full test suite (293 existing + new seeding/combat/advisor tests)
PATH="/usr/bin:$PATH" cargo test

# Type check
PATH="/usr/bin:$PATH" cargo check

# WASM build (verifies no WASM-incompatible deps added)
bash scripts/build-wasm.sh

# Seeding-specific regression
PATH="/usr/bin:$PATH" cargo test -p rebellion-data seeds::tests -- --nocapture
```

**All must pass.** If any fail, Yarikh files the failure back to the responsible dabora for immediate fix before the Knesset closes.

---

## Acceptance Criteria

### Seeding (Resheph)
- [ ] No system starts at (0.0, 0.0) popularity unless intentionally unpopulated + neutral
- [ ] Controlled core systems have support in SDPRTB 7682-7685 derived ranges
- [ ] Neutral core systems land in 41-59 window
- [ ] Maintenance-budget loop produces majority of starting forces
- [ ] Budget spent ≤ ceiling for chosen galaxy size + difficulty
- [ ] Luke + Leia at Yavin, Palpatine at Coruscant, Mon Mothma at Rebel HQ
- [ ] All 8+ existing seeding tests still pass
- [ ] 15+ new tests with fixed seeds

### Ship Class Fields (Shapash)
- [ ] CapitalShipClass has 15 new fields matching CAPSHPSD.DAT
- [ ] FighterClass has all DAT fields promoted to world model
- [ ] `cargo test` passes (no field-mismatch panics)
- [ ] Existing combat system continues to compile and pass

### WASM Timing (Anat)
- [ ] Right-click behavior diagnosed and documented
- [ ] Fix applied if genuine WASM issue found, or downgraded with evidence if browser-automation artifact
- [ ] Native build unaffected

### Ghidra Site CI (Anat)
- [ ] `mkdocs build` succeeds locally with social plugin
- [ ] requirements.txt includes Pillow + CairoSVG
- [ ] CI workflow updated if present

### HD Visual Polish (Anat)
- [ ] At least cockpit frames and character portraits upscaled
- [ ] HD PNGs placed in correct path for BmpCache fallback
- [ ] Game still renders correctly with HD assets

### Droid Advisor (Pidray)
- [ ] BIN format documented (even if partially)
- [ ] At minimum: static frame display in egui panel with faction-appropriate sprites
- [ ] At best: animated sequences with event-triggered transitions
- [ ] Advisor panel toggleable via keyboard shortcut

### Overall
- [ ] 293+ existing tests pass
- [ ] Zero new warnings in `cargo check`
- [ ] WASM build succeeds
- [ ] All new code reviewed by Yarikh's 3 subagents

---

## Estimated Wall-Clock Time

| Phase | Duration | Notes |
|-------|----------|-------|
| **Implementation** (parallel) | 30-45 min | Resheph is bottleneck (4 sequential seeding milestones). Others finish faster. |
| **Verification** (parallel reviews) | 10-15 min | 3 review subagents in parallel |
| **Final build** | 5 min | cargo test + cargo check + WASM build |
| **Total** | **45-65 min** | |

Resheph (seeding chain) is the critical path. Shapash, Anat, and Pidray will likely finish during Resheph's M6-M7 phase and idle until verification.

---

## Execution Checklist

1. [ ] Orchestrator creates 5 dabora tasks
2. [ ] Resheph, Shapash, Anat, Pidray launch simultaneously
3. [ ] Each dabora marks its task(s) complete via TaskUpdate
4. [ ] Orchestrator waits for all 4 daborot to complete
5. [ ] Yarikh launches 3 review subagents in parallel
6. [ ] Reviews complete — any defects filed back to responsible dabora
7. [ ] Defect fixes applied
8. [ ] Final build verification passes
9. [ ] CLAUDE.md updated: version bump, Known Limitations trimmed, Reports section updated
10. [ ] Knesset report written to `docs/reports/2026-03-25-knesset-kothar-swarm-report.md`
11. [ ] Commit

---

## Risk Register

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| BIN format too complex for single sprint | Medium | #25 delivers static frames only | Pidray documents format, implements simplified version, files follow-up task |
| world/mod.rs merge conflict (Resheph + Shapash) | Low | 5 min manual resolve | Non-overlapping struct regions; Resheph priority on conflict |
| SDPRTB parameter values missing or unexpected | Low | Seeding falls back to defaults | All param lookups use `.unwrap_or(default)` pattern |
| WASM build breaks from new deps | Low | Blocks final verification | Only Pidray adds new module; advisor.rs has no platform deps |
| HD asset upscaling quality poor | Low | Ship with lower quality, iterate later | Use cunet model (best for game art), denoise level 1 |

---

*Designed by Nomos (νόμος), soul architect of Claudicle.*
*"Let the palace be finished. Let the craftsman god rest."*
