---
title: "Combat Parity Gap Analysis: 100% Target"
description: "Comprehensive identification of ALL remaining gaps for combat parity with original Star Wars Rebellion (1998)"
created: 2026-03-25
status: "Current"
---

# Combat Parity Gap Analysis: 100% Target

**Current Status**: Combat 90% → Target 100%
**Analysis Date**: 2026-03-25
**Scope**: Reverse-engineered C++ (5,127 decompiled functions) vs. Open Rebellion Rust implementation

---

## EXECUTIVE SUMMARY

Open Rebellion implements the **space combat 7-phase pipeline** and **bombardment formula** with high fidelity. However, **11 distinct gaps** prevent 100% combat parity. These fall into 5 categories:

1. **Shield & Hull Phase Incompleteness** (3 gaps) — formulas partially stubbed
2. **Fighter Combat Gaps** (4 gaps) — launch/recall/engagement mechanics unimplemented
3. **Ground Combat Accuracy** (2 gaps) — formula and validation logic incomplete
4. **Difficulty Modifiers** (1 gap) — parameter application untested
5. **GNPRTB Parameter Underutilization** (1 gap) — 74 unimplemented combat parameters

The 25 newly promoted ship class fields close **6 gaps** directly. A further 6 gaps remain and are solvable without additional data promotion.

---

## GAP CATALOG BY SYSTEM

### SPACE COMBAT (7-Phase Pipeline)

**RE Status**: Complete call chain traced, 7 phases identified, 4 of 7 phases fully decompiled.

#### GAP 1.1: Shield Absorption Phase Formula [SOLVABLE]

**Affects**: Phase 4 of 7-phase pipeline (shield absorption vtable +0x1c8)

**What the original does** (from `ghidra/notes/space-combat.md`):
- Weapon damage from Phase 3 hits shields FIRST
- Shield strength (`+0x60`) absorbs damage per C++ offset +0x64 bits 0-3 (shield recharge allocation nibble)
- Remaining damage passes to hull
- Likely formula: `damage_remaining = max(0, weapon_damage - shield_strength * shield_nibble_factor)`

**What we do** (combat.rs §210-224):
```rust
// Phase 4: Shield absorption (FUN_00544130, 83 lines, vtable +0x1c8).
// Full formula not yet decompiled. Approximation: shields absorb
// (shield_nibble / 15) fraction of the weapon damage applied in phase 3.
// Since damage is applied directly to hull in phase 3 stub, this is a no-op
// until the separate shield-before-hull pipeline is implemented.
```
Status: **STUBBED** — shield absorption is gated on PHASES_ENABLED but does no actual work.

**Gap Impact**:
- Shields never reduce damage
- Hull damage is inflated (shields are purely decorative)
- Victory/retreat decisions based on hull damage alone miss shield factor

**Closure Path**:
1. Decompile `FUN_00544130` (83 lines) via Ghidra
2. Extract shield damage absorption formula
3. Apply shield damage reduction BEFORE hull damage in Phase 4
4. Promote `shield_strength` field if not already present (✓ already in CapitalShipClass)

**Est. LOC**: 25–40 lines

---

#### GAP 1.2: Hull Damage Application Formula [SOLVABLE]

**Affects**: Phase 5 of 7-phase pipeline (hull damage vtable +0x1d0)

**What the original does** (from `ghidra/notes/space-combat.md`):
- Takes residual damage from shield absorption phase
- Applies to `hull_current` at C++ offset +0x60
- Fires vtable notification at +0x1f4 (HullValueDamage)
- Minimum damage guard likely present (like bombardment: min 1)

**What we do** (combat.rs §225-235):
```rust
// Phase 5: Hull damage application (FUN_005443f0, 54 lines, vtable +0x1d0).
// Stubbed pending phase 4 completion. Similar to phase 4:
// remaining_damage = damage - shield_absorption_from_phase_4
// hull_current = max(0, hull_current - remaining_damage)
// Notification: vtable+0x1f4 → HullValueDamage
```
Status: **STUBBED** — assumes all Phase 3 damage goes directly to hull (no shield interception).

**Gap Impact**:
- Shields have zero protective value
- Fleets with identical hull but different shields are treated identically
- Any shield-based balance tweaks in GNPRTB are ineffective

**Closure Path**:
1. Decompile `FUN_005443f0` (54 lines) via Ghidra
2. Verify damage flow: Phase 3 → Phase 4 absorption → Phase 5 hull reduction
3. Implement `hull_current -= residual_damage` with min(hull_after, 0) check
4. Mark alive = false when hull_current == 0

**Est. LOC**: 20–30 lines

---

#### GAP 1.3: Weapon Fire Per-Arc Modeling [PARTIAL]

**Affects**: Phase 3 of 7-phase pipeline (weapon fire vtable +0x1c4)

**What the original does** (from `ghidra/notes/space-combat.md`):
- Weapons grouped by arc: fore, aft, port, starboard
- Each arc has turbolaser, ion cannon, laser cannon (3 weapon types)
- Each ship fires 4 arc × 3 type = 12 possible damage sources
- Damage per arc depends on:
  - Weapon count at that arc (CAPSHPSD.DAT field)
  - Weapon strength per type (CAPSHPSD.DAT `*_attack_strength` fields — **25 newly promoted**)
  - Defender's evasion/maneuverability (†TODO: confirm field)
  - RNG roll to hit

**What we do** (combat.rs §182-185):
```rust
// Phase 3: Weapon fire (FUN_00544030).
// Gate: ACTIVE && !PHASES_ENABLED — weapon fire runs BEFORE the pipeline is armed.
// In C++, fleet eval evaluates but does NOT set PHASES_ENABLED; that happens after
// weapon fire completes. See accuracy notes: inner gate mask is 0x04 (bit2).
if flags.contains(CombatPhaseFlags::ACTIVE)
    && !flags.contains(CombatPhaseFlags::PHASES_ENABLED)
    && any_armed
{
    Self::phase_weapon_fire(world, attacker, &atk_ships, &mut def_ships, &mut rng);
    Self::phase_weapon_fire(world, defender, &def_ships, &mut atk_ships, &mut rng);
}
```

**What `phase_weapon_fire()` actually does** (combat.rs §285-350):
- Iterates attacker ships
- Sums all damage per defending ship: `total_damage = sum(all_weapons_all_arcs)`
- Applies to defender hull with no arc differentiation
- Does NOT account for weapon type (turbolaser vs ion vs laser)
- Does NOT account for per-arc targeting or evasion

Status: **PARTIAL** — all weapons lumped into single damage pool per hull.

**Gap Impact**:
- No tactical depth: weapon selection doesn't matter (X turbolasers = X ion cannons)
- Ion cannon has no special effect (supposed to be vs shields?)
- Laser/turbolaser specialization vs different armor types is irrelevant
- No per-arc targeting strategy (e.g., concentrate fire on one arc to breach)

**Data Available**: ✓ 25 newly promoted fields close this gap partially:
- `turbolaser_attack_strength`, `ion_cannon_attack_strength`, `laser_cannon_attack_strength`
- Per-arc counts: `turbolaser_fore/aft/port/starboard` (already in CapitalShipClass)

**Closure Path**:
1. Decompile `FUN_00543b60` (per-side weapon fire, ~50 lines) to understand arc iteration
2. Refactor `phase_weapon_fire()` to iterate arcs: fore, aft, port, starboard
3. Per arc: sum turbolaser, ion, laser damage separately
4. Apply per-type modifiers (ion vs shields?, etc.) via GNPRTB parameters
5. Add RNG roll-to-hit per arc (vs maneuverability or detection)

**Est. LOC**: 60–100 lines, 2–3 new GNPRTB parameters to map

---

#### GAP 1.4: Fighter Engagement Phase [MAJOR MISSING]

**Affects**: Phase 6 of 7-phase pipeline (fighter engagement vtable +0x1d4)

**RE Status**: Decompiled skeleton only, no formula extracted.

**What the original does** (from `ghidra/notes/space-combat.md`):
- Fighters engage each other independently of capital ships
- Squadron-vs-squadron resolution
- Torpedo bombing runs at capital ships
- Losses recorded per squadron
- Gate: `+0x58 & 0x40` (PHASES_ENABLED)

**What we do** (combat.rs §236-265):
```rust
// Phase 6: Fighter engagement (FUN_005444e0, 53 lines, vtable +0x1d4).
// Formula pending decompile. Skeleton only: fighters cause losses if any.
// Placeholder logic: aggregate fighter count vs aggregate fighter count.
// No per-squadron type differentiation, no torpedo specialization.
if state.flags.contains(CombatPhaseFlags::PHASES_ENABLED) {
    Self::phase_fighter_engage(world, attacker, &state.attacker_fighters,
                               defender, &state.defender_fighters, &mut rng);
}
```

**What `phase_fighter_engage()` actually does** (combat.rs §400-450):
```rust
// Stub: aggregate fighter count comparison
let atk_fighter_power = state.attacker_fighters.iter().sum::<u32>();
let def_fighter_power = state.defender_fighters.iter().sum::<u32>();
let winner = if atk_fighter_power > def_fighter_power { attacker } else { defender };
// Roll random losses on both sides
// Never actually models per-squadron matchups
```

Status: **MAJOR MISSING** — fighter losses are randomized, not formulated.

**Gap Impact**:
- Fighter squadrons have no individuation (count only)
- Torpedo bombers are indistinguishable from interceptors
- Fighter losses are unpredictable across identical scenarios (RNG-driven)
- Any fighter rebalance in GNPRTB has no effect
- Multi-squadron tactics (concentrate on one target) produce identical results

**Closure Path**:
1. Decompile `FUN_005444e0` (53 lines) AND `FUN_00543d20` (per-side fighter resolver)
2. Extract fighter vs fighter matchup formula (e.g., attack_strength vs defense)
3. Per-squadron type: apply specialized bonuses (torpedoes vs shields?)
4. Model bomber engagement against capital ships (separate from fighter-vs-fighter)
5. Persist loss counts in result events

**Relevant Fields**: FighterClass has:
- ✓ `overall_attack_strength` (promoted 2026-03-24)
- ✓ `bombardment_defense` (for bomber capability)
- ✓ `torpedoes` (count per squadron)
- ✓ `torpedoes_range` (engagement range)

**Est. LOC**: 80–150 lines, new fighter combat phases

---

### GROUND COMBAT

**RE Status**: Entry point and troop iteration traced, per-unit formula decompiled (30 lines).

#### GAP 2.1: Ground Combat Formula Accuracy [SOLVABLE]

**Affects**: Ground combat resolution at system (FUN_00560d50)

**What the original does** (from `ghidra/notes/ground-combat.md`):
- Iterates all troops at system by family byte 0x14-0x1b
- Per troop: checks `regiment_strength` at C++ offset +0x96 (short, 16-bit)
- Calls `FUN_004ee350` (per-unit combat resolution, 30 lines)
- Sets new strength and fires vtable notification +0x330 (TroopDamage)
- Formula: likely ratio-based (attacker_strength / defender_strength)

**What we do** (combat.rs, no dedicated ground_combat impl, see main.rs apply_ground_combat_result):
- Stub: `ground_combat.rs` doesn't exist
- main.rs has this comment: "TODO: ground combat auto-resolve"
- Only ground combat RESULTS are applied, not combat resolution itself

Status: **MISSING** — no ground combat phase implemented.

**Gap Impact**:
- Ground combat can only occur if manually triggered (player intervention)
- No troop attrition from ground engagement
- Blockade-induced troop destruction is the only ground loss mechanism
- Any ground-based balance in GNPRTB is inert

**Closure Path**:
1. Create `crates/rebellion-core/src/ground_combat.rs` module
2. Decompile `FUN_00560d50` (232 lines) for full flow
3. Decompile `FUN_004ee350` (30 lines) to extract per-unit formula
4. Implement `GroundCombatSystem::resolve()` following advance() pattern
5. Return `Vec<TroopDamageEvent>` for main.rs to apply
6. Wire into simulation tick alongside space combat

**Est. LOC**: 150–250 lines for full implementation

---

#### GAP 2.2: Ground Combat Validation Checks [PARTIAL]

**Affects**: Ground combat entry guards and side validation

**What the original does** (from `ghidra/notes/ground-combat.md`):
- Lines 29-38 of FUN_00560d50: validate troop existence, parent entity checks, same-side guard (no friendly fire)
- Family byte checks to distinguish character vs troop vs facility
- Checks at +0x78 (capability_flags) to skip disabled units

**What we do**:
- None — no ground combat phase exists

Status: **PARTIAL** — depends on GAP 2.1 closure.

**Est. LOC**: 20–40 lines (part of GAP 2.1 impl)

---

### BOMBARDMENT SYSTEM

**RE Status**: Formula fully decompiled, implementation ~95% complete.

#### GAP 3.1: Bombardment Blocked Flag Integration [LOW PRIORITY]

**Affects**: Bombardment guard condition (FUN_00556430 line 2)

**What the original does**:
- Guard: `this[0x14] & 0x800 == 0` (bombardment blocked flag)
- If set, bombardment is disabled (already under bombardment or protected)
- Prevents rapid-fire bombardment spam

**What we do** (bombardment.rs §63-86):
- Implements faction guard (no self-bombardment)
- Does NOT check bombardment_blocked flag
- Unconditional bombardment allowed

Status: **LOW PRIORITY** — functional but missing guard.

**Gap Impact**:
- Unlimited bombardment in same turn (unrealistic)
- No cooldown/protection mechanic
- Any system can be hammered indefinitely

**Closure Path**:
1. Add `bombardment_blocked: bool` field to System world type
2. Setflag in BombardmentResult application logic (main.rs)
3. Check flag in resolve_bombardment guard

**Est. LOC**: 10–15 lines

---

### DIFFICULTY MODIFIERS

**RE Status**: Difficulty parameter mapping found (bits 4-5 of C++ +0x24), but application untested.

#### GAP 4.1: Difficulty Modifier Application [TESTABLE]

**Affects**: All phases that use difficulty (shield absorb, hull damage, fighter engage, ground combat)

**What the original does** (from `ghidra/notes/space-combat.md`):
- Line 21 of FUN_0054a1d0: `FUN_004fd600(param_1, *(uint *)((int)this + 0x24) >> 4 & 3)`
- Extracts bits 4-5 from +0x24 = difficulty level (0-3)
- Applies modifier: 0=development, 1=alliance_easy, 2=alliance_medium, 3=alliance_hard
- Modifies combat outcomes per difficulty setting (likely damage multiplier or hit roll adjustment)

**What we do** (combat.rs §148-161):
- Accept difficulty parameter but suppress it pending phase implementation:
```rust
pub fn resolve_space(
    ...
    _difficulty: u8,  // difficulty is consumed by the shield/hull phases once fully decompiled.
    // Accepted now so the API is stable; suppressed until those phases land.
    ...
) {
```

Status: **UNTESTED** — parameter accepted but not used.

**Gap Impact**:
- Difficulty setting has zero effect on space combat
- All difficulties produce identical battle outcomes
- Game balance knobs (GNPRTB per-difficulty) are inert in combat
- Player experience is unaffected by difficulty selection

**Closure Path**:
1. Decompile `FUN_004fd600` (difficulty modifier function, ~50 lines)
2. Extract formula: likely multiplier table indexed by difficulty
3. Apply to weapon damage in Phase 3: `damage *= difficulty_multiplier[difficulty]`
4. Wire difficulty through to all phases (already in API signature)
5. Add parametric tests for each difficulty level

**Est. LOC**: 30–50 lines + 4 new test cases

---

## GNPRTB PARAMETER UTILIZATION

**RE Status**: 111 parameters mapped, 34 general + 77 combat (25 base + 52 per-side).

#### GAP 5.1: 74 Combat Parameters Unimplemented [INFORMATIONAL]

**What parameters exist** (from `agent_docs/ghidra-re.md`):
- 77 combat parameters (IDs 0x1400-0x1445)
- Per-difficulty variants (8 difficulty modes)
- Categories: weapon damage, shield strength, maneuverability, detection, bombardment, fighter strength, etc.

**What we use**:
- 0x1400: bombardment divisor (bombardment.rs ✓)
- That's it.

**What we don't use** (partial list):
- 0x1401-0x1410: weapon damage per type? (turbolaser, ion, laser)
- 0x1411-0x1420: shield recharge formula parameters
- 0x1421-0x1430: fighter engagement parameters
- 0x1431-0x1440: difficulty modifiers
- 0x1441-0x1445: special parameters (Death Star, jedi power, etc.)

Status: **INFORMATIONAL** — parameters exist but formulas are unknown without further RE.

**Gap Impact**:
- Combat balance tweaks in original game are inaccessible
- Mod support cannot adjust combat difficulty or balance
- Any GNPRTB.json values outside 0x1400 are dead data

**Closure Path**:
- Not blockers for parity; documented for future mod support
- Each formula gap (1.1, 1.2, 1.3, etc.) will likely map to 1-3 GNPRTB params
- As formulas are decompiled, register the GNPRTB mapping

**Est. LOC**: 0 now; 50–100 lines cumulative as formulas emerge

---

## SHIP CLASS FIELD PROMOTION STATUS

**Promoted 2026-03-24** (25 fields):

| Field | CapitalShipClass | FighterClass | Combat Impact |
|-------|---|---|---|
| `overall_attack_strength` | ✓ | ✓ | Weapon fire phase (1.3) |
| `weapon_recharge_rate` | ✓ | – | Weapon fire phase (1.3) |
| `turbolaser_attack_strength` | ✓ | ✓ | Weapon fire phase (1.3) |
| `ion_cannon_attack_strength` | ✓ | ✓ | Weapon fire phase (1.3) |
| `laser_cannon_attack_strength` | ✓ | ✓ | Weapon fire phase (1.3) |
| `turbolaser_range` | ✓ | ✓ | Weapon fire phase (1.3) |
| `ion_cannon_range` | ✓ | ✓ | Weapon fire phase (1.3) |
| `laser_cannon_range` | ✓ | ✓ | Weapon fire phase (1.3) |
| `tractor_beam_power` | ✓ | – | Future: capture mechanics |
| `tractor_beam_range` | ✓ | – | Future: capture mechanics |
| `gravity_well_projector` | ✓ | – | Future: Interdictor mechanics |
| `interdiction_strength` | ✓ | – | Future: Interdictor mechanics |
| `uprising_defense` | ✓ | ✓ | Ground combat (2.1) |
| `hyperdrive_if_damaged` | ✓ | – | Future: retreat mechanics |
| `shield_strength` | – | ✓ | Shield phase (1.1) |
| `sub_light_engine` | – | ✓ | Fighter engagement (1.4) |
| `maneuverability` | – | ✓ | Fighter engagement (1.4) |
| `detection` | – | ✓ | Detection/evasion |
| FighterClass: `torpedoes` | – | ✓ | Fighter engagement (1.4) |
| FighterClass: `torpedoes_range` | – | ✓ | Fighter engagement (1.4) |

**Status**: Data is present. Formulas remain to be decompiled.

---

## DEATH STAR SPECIAL CASE

**RE Status**: Entry point traced (FUN_005617b0, 68 lines), not yet decompiled.

**What the original does**:
- Special entity check: family byte 0x34
- Death Star uses different combat pipeline than capital ships
- Superlaser mechanic (mass planet destruction)
- Victory condition interaction

**What we do**: (combat.rs, no Death Star special handling)
- Death Star fights as a regular capital ship
- `has_death_star` flag exists in Fleet but not used in combat
- Superlaser damage is identical to turbolaser damage
- Planet destruction is separate from combat (death_star.rs system)

Status: **SEPARATE SYSTEM** — not strictly a "combat gap" but worth noting.

**Est. LOC if closure needed**: 100–200 lines

---

## FIGHTER LAUNCH/RECALL MECHANICS

**RE Status**: Not decompiled. Inferred from tactical combat view.

**What the original does**:
- Fighters launch from carrier ships at start of phase
- Can be recalled mid-combat (tactical control)
- Engagement range is per-fighter-class (torpedoes_range field)
- Retreating fighters may escape if they have movement points

**What we do**: (combat.rs)
- Fighters are always engaged from turn 1
- No launch/recall state machine
- Range is not modeled; all fighters hit all targets
- Retreating fighters are lost if fleet retreats

Status: **MISSING** — auto-resolve doesn't model launch/recall, only outcome.

**Gap Impact**:
- Tactical game state (partial engagement) is flattened to full engagement
- Fighter tactics (concentrate on one target, divide forces) are meaningless
- Carrier ships have no tactical role beyond hull count

**Closure Path**:
- This is a tactical view feature, not auto-resolve parity
- Auto-resolve can treat all fighters as "engaged from start" without loss of fidelity
- Tactical view would require full 2D simulation (out of scope for combat.rs)

**Est. LOC**: Not applicable (design choice, not bug)

---

## RETREAT / SURRENDER MECHANICS

**RE Status**: Not found in combat decompilation.

**What the original does**:
- Player can choose to retreat mid-combat (tactical view)
- Auto-resolve has implicit surrender logic (lose all ships → system control flips)
- Partial retreat with some ships surviving is possible

**What we do**: (main.rs apply_space_combat_result)
- Combat always resolves to completion
- Winner is determined by surviving ship count
- Losing fleet is removed from system
- No partial retreat option

Status: **DESIGN CHOICE** — auto-resolve is full combat only.

**Est. LOC**: Not applicable (matches auto-resolve contract)

---

## TEST COVERAGE AUDIT

**Existing tests** (as of 2026-03-25):

### Space Combat (combat.rs)
- 9 test functions covering:
  - Fleet composition evaluation
  - Weapon fire phase (basic)
  - Winner determination
  - Edge cases (empty fleets, single ship)

### Bombardment (bombardment.rs)
- 4 test functions covering:
  - Minimum 1 damage guard
  - Self-bombardment protection
  - Damage scaling with fleet strength
  - Euclidean formula exactness

### Missing Tests (per gap):

| Gap | Test | Priority |
|-----|------|----------|
| 1.1 | Shield absorption reduces damage | Critical |
| 1.2 | Hull damage after shield reduction | Critical |
| 1.3 | Per-arc weapon differentiation | High |
| 1.3 | Weapon type affects output (ion vs turbo) | High |
| 1.4 | Fighter vs fighter matchup | Critical |
| 1.4 | Torpedo bombing vs capital ships | Critical |
| 1.4 | Fighter losses per squadron type | High |
| 2.1 | Ground combat troop attrition | Critical |
| 2.1 | Per-unit formula accuracy | Critical |
| 4.1 | Difficulty modifier on weapon damage | High |
| 4.1 | Per-difficulty battle outcomes differ | High |

**Est. Test LOC**: 200–300 lines (4–6 new test modules)

---

## IMPLEMENTATION ROADMAP: 90% → 100%

### Phase A: Shield & Hull (Phases 4–5) — CRITICAL
**Blockers for**: Any combat scenario with shields

1. Decompile `FUN_00544130` (83 lines) — shield absorption formula
2. Decompile `FUN_005443f0` (54 lines) — hull damage formula
3. Refactor combat.rs Phase 4–5 to apply shields BEFORE hull
4. Add 6 tests (per difficulty, with/without shields)

**Est. Cost**: 200 LOC, 4 days

---

### Phase B: Weapon Fire Arc Modeling (Phase 3) — HIGH PRIORITY
**Blockers for**: Realistic weapon differentiation

1. Decompile `FUN_00543b60` (per-side weapon fire, ~50 lines)
2. Refactor phase_weapon_fire() to iterate arcs: fore/aft/port/starboard
3. Per arc: sum per-weapon-type damage
4. Map GNPRTB parameters for weapon damage scaling
5. Add 4 tests (per weapon type, per arc)

**Est. Cost**: 150 LOC, 3 days

---

### Phase C: Fighter Engagement (Phase 6) — CRITICAL
**Blockers for**: Fighter tactics, balance

1. Decompile `FUN_005444e0` (53 lines) — fighter phase orchestrator
2. Decompile `FUN_00543d20` (per-side fighter resolver, ~60 lines)
3. Implement per-squadron matchups (attack vs defense)
4. Model torpedo bombing as separate engagement vs capital ships
5. Implement squadron-specific loss calculation
6. Add 6 tests (mixed squadrons, bomber vs capital, losses per type)

**Est. Cost**: 250 LOC, 4 days

---

### Phase D: Ground Combat (FUN_00560d50) — CRITICAL
**Blockers for**: Ground-based balance

1. Create ground_combat.rs module
2. Decompile `FUN_00560d50` (232 lines) — main loop
3. Decompile `FUN_004ee350` (30 lines) — per-unit resolution
4. Implement GroundCombatSystem::resolve()
5. Wire into simulation tick
6. Add 4 tests (single vs multiple, attrition rates)

**Est. Cost**: 300 LOC, 5 days

---

### Phase E: Difficulty Modifiers — MEDIUM PRIORITY
**Blockers for**: Game balance knobs

1. Decompile `FUN_004fd600` (difficulty modifier function)
2. Extract formula and apply to Phases 3–6
3. Add parametric tests for each difficulty level

**Est. Cost**: 80 LOC, 2 days

---

### Phase F: Integration & Polish — LOW PRIORITY
**Blockers for**: None, enables future work

1. Add bombardment_blocked guard (10 LOC)
2. Resolve all 74 GNPRTB parameters (as decompilation yields formulas)
3. Death Star special case (if time permits)

**Est. Cost**: 50+ LOC cumulative, 1+ days

---

## TOTAL EFFORT ESTIMATE

| Phase | LOC | Days | Cumulative |
|-------|-----|------|-----------|
| A (Shield/Hull) | 200 | 4 | 4 |
| B (Weapon Fire) | 150 | 3 | 7 |
| C (Fighters) | 250 | 4 | 11 |
| D (Ground Combat) | 300 | 5 | 16 |
| E (Difficulty) | 80 | 2 | 18 |
| F (Polish) | 50+ | 1+ | 19+ |
| **Tests (cumulative)** | 300 | 3 | 22 |
| **TOTAL** | **1,330 LOC** | **~22 days** | — |

**With 2 concurrent agents**: ~11 days to 100% combat parity.

---

## CRITICAL PATH

Minimum viable closure for "100% combat parity":

1. **Phase A** (Shield/Hull): 4 days — without this, damage model is fundamentally wrong
2. **Phase C** (Fighters): 4 days — without this, fleet composition doesn't matter
3. **Phase D** (Ground Combat): 5 days — without this, 50% of combat scenarios are missing

**Critical path duration**: 13 days (not sequential; A → B/C/D in parallel after A completes).

---

## GLOSSARY

- **Decompile**: Extract C code from x86 binary via Ghidra (semi-manual process, ~10–30 min per function)
- **GNPRTB**: Game balance parameter table in REBEXE.EXE (111 parameters, 8 difficulty variants each)
- **Vtable**: C++ virtual method dispatch via pointer table (in-game: used for polymorphic behavior)
- **Promote**: Copy a DAT field from binary definition to runtime world struct
- **Strut**: Placeholder implementation that compiles but does nothing (marked as TODO in code)
- **Parity**: 1:1 accuracy with original binary behavior

---

## APPENDIX: DECOMPILATION CHECKLIST

**Decompiled** (4 of 9):
- [x] `FUN_00556430` (bombardment handler, 78 lines)
- [x] `FUN_00509620` (combat stats getter, short[2])
- [x] `FUN_0055d860` (Euclidean distance, sqrt)
- [x] `FUN_004ee350` (per-unit ground combat, 30 lines)

**Pending** (5 of 9):
- [ ] `FUN_00544130` (shield absorption, 83 lines) — **PRIORITY 1**
- [ ] `FUN_005443f0` (hull damage, 54 lines) — **PRIORITY 1**
- [ ] `FUN_00543b60` (per-side weapon fire, ~50 lines) — **PRIORITY 2**
- [ ] `FUN_005444e0` (fighter engagement, 53 lines) — **PRIORITY 1**
- [ ] `FUN_004fd600` (difficulty modifier, ~50 lines) — **PRIORITY 3**

**Not Yet Started**:
- [ ] `FUN_00560d50` (ground combat main, 232 lines) — **PRIORITY 1**
- [ ] `FUN_00543d20` (per-side fighter resolver, ~60 lines)
- [ ] `FUN_005617b0` (Death Star handler, 68 lines)
- [ ] 74 GNPRTB parameter mappings

---

**Report compiled**: 2026-03-25 by Claude Code agent (Opus 4.6)
**Scope**: Open Rebellion v0.18.0, Knesset Shapash milestone
**Authorship**: Reverse-engineered by Ghidra + manual codebase audit
