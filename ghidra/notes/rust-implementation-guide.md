---
title: "Rust Implementation Guide: Combat & Mission Systems"
description: "Maps decompiled C++ from REBEXE.EXE to rebellion-core Rust implementation"
category: "ghidra"
created: 2026-03-15
updated: 2026-10-14
---

# Rust Implementation Guide: Combat & Mission Systems

*Maps decompiled C++ from REBEXE.EXE to `rebellion-core` Rust. Written for the Knesset Tiamat swarm.*

**Source notes**: `ghidra/notes/space-combat.md`, `ground-combat.md`, `bombardment.md`, `annotated-functions.md`, `COMBAT-SUMMARY.md`
**Target crate**: `crates/rebellion-core/src/` — zero IO, no rendering deps, all advance() pattern

---

## Index

1. [Game Object Layout → Rust Structs](#1-game-object-layout--rust-structs)
2. [Space Battles](#2-space-battles)
3. [Ground Combat](#3-ground-combat)
4. [Bombardment](#4-bombardment)
5. [Mission System Expansion](#5-mission-system-expansion)
6. [Integration Order](#6-integration-order)

---

## 1. Game Object Layout → Rust Structs

### 1.1 The C++ Entity Base Object

Every combat entity in REBEXE.EXE shares one heap-allocated C++ object layout. The fields below are confirmed from Ghidra across multiple functions:

```
+0x00   void*   vtable
+0x50   uint    status_flags         bit0=active, bit3=fighter_combat_eligible, bit12=special_disabled
+0x58   uint    combat_phase_flags   space combat state machine (see §2.2)
+0x60   int     hull_current         current hull damage value (ships); squad size (squadrons)
+0x64   uint    shield_weapon_packed bits 0-3 = shield_recharge_allocated, bits 4-7 = weapon_recharge_allocated
+0x66   short   loyalty_current      characters: base loyalty 0-100
+0x78   byte    capability_flags     bit6+7=disabled, bit4=special_entity_flag, bit7=alt_shield_path
+0x8a   short   enhanced_loyalty     mission bonus, raw 0-0x7fff, clamped 0-100
+0x96   short   regiment_strength    troops/ships: current regiment strength
+0x9a   short   hyperdrive_modifier  mission speed bonus (Han Solo: +hyperdrive_mod)
+0xac   byte    alive_flag           bit0=alive/combat-eligible
+0xb0   byte    combat_ready_flags   bit1=combat_ready_for_fleet_eval
```

The space combat state object (`this` in `FUN_00549910` and its callees) adds:

```
+0x24   uint    difficulty_packed    bits 4-5 = difficulty level 0-3
+0x1c4  code*   vtable_weapon_fire
+0x1c8  code*   vtable_shield_absorb
+0x1d0  code*   vtable_hull_damage
+0x1d4  code*   vtable_fighter_engage
+0x1d8  code*   vtable_who_won
```

### 1.2 What This Means for `CapitalShipClass` and Instance Records

The current `CapitalShipClass` in `world/mod.rs` carries only ~10 of ~50 DAT fields. Combat needs these additions promoted to the world model:

**Add to `CapitalShipClass`** (class template — from CAPSHPSD.DAT):
```rust
// Weapons (4 arcs × 3 types = 12 values; DAT fields already parsed by dat-dumper)
pub weapons_fore_turbolaser: u32,
pub weapons_fore_ion: u32,
pub weapons_fore_laser: u32,
pub weapons_aft_turbolaser: u32,
pub weapons_aft_ion: u32,
pub weapons_aft_laser: u32,
pub weapons_port_turbolaser: u32,
pub weapons_port_ion: u32,
pub weapons_port_laser: u32,
pub weapons_starboard_turbolaser: u32,
pub weapons_starboard_ion: u32,
pub weapons_starboard_laser: u32,

// Defense
pub shield_recharge: u32,       // max shield recharge per round
pub damage_control: u32,        // hull repair rate
pub bombardment: u32,           // bombardment attack stat (used in §4)
```

**Add a new `ShipInstance` world type** (runtime per-hull state — replaces the count-only `ShipEntry`):
```rust
/// A single hull of a capital ship class — the runtime instance.
/// Distinct from ShipEntry which is a (class, count) summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipInstance {
    pub class: CapitalShipKey,
    /// Current hull. Starts at CapitalShipClass::hull, reduced by combat.
    pub hull_current: i32,
    /// Packed nibbles: bits 0-3 = shield_recharge_allocated, bits 4-7 = weapon_recharge_allocated.
    /// Mirrors C++ offset +0x64. Read via helpers shield_nibble() / weapon_nibble().
    /// Note: C++ writes via XOR-mask pattern `(new ^ old) & 0xf ^ old` — functionally equivalent
    /// to simple mask-OR for in-range values, but the XOR is what the original binary does.
    pub shield_weapon_packed: u8,
    /// bit0 = alive; cleared when hull_current reaches 0.
    pub alive: bool,
    pub faction_is_alliance: bool,
}

impl ShipInstance {
    pub fn shield_nibble(&self) -> u8 { self.shield_weapon_packed & 0x0f }
    pub fn weapon_nibble(&self) -> u8 { (self.shield_weapon_packed >> 4) & 0x0f }
}
```

**Add to `TroopUnit`** (already exists, but needs strength):
```rust
/// Current regiment strength (C++ offset +0x96). Starts at class max, reduced by ground combat.
pub regiment_strength: i16,
```

**Add to `FighterClass`** (template — from FIGHTSD.DAT):
```rust
pub attack: u32,
pub defense: u32,
pub bombardment: u32,
```

### 1.3 GNPRTB Fields Needed in the World Model

Rather than hardcoding constants, these GNPRTB parameter values should be loaded into `GameWorld` at startup (or passed directly to `advance()`). The two confirmed combat parameters:

```rust
/// Game-balance parameters from GNPRTB.DAT, indexed by difficulty (0-3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnprtbParams {
    /// DAT_006bb6e8 — bombardment damage divisor.
    /// Scales Euclidean-distance raw power to final damage.
    pub bombardment_divisor: [i32; 4],
    /// DAT_00661a88 — difficulty modifier table (4 entries, one per difficulty).
    pub difficulty_modifier: [i32; 4],
}
```

Add `pub gnprtb: GnprtbParams` to `GameWorld`. Populated by `rebellion-data` during loading.

---

## 2. Space Battles

Every space battle runs the tactical engine, `crates/rebellion-core/src/tactical.rs`:
played in its window, simulated from the Battle Alert, or headless when no
one watches (`FUN_0040a050` state 8). The headless step (`FUN_005ae9c0`)
ranks opponents, assigns targets and applies each group's damage rate once a
second; the 1200 s stalemate rule ends every battle. See
`simulate-runs-tactical-engine.md`, `tactical-engagement-ai.md` and
`tactical-weapon-loop.md`.

`FUN_00549910` and the flag word at `+0x58` are the MissionMgr story checks
(`story-missions.md`), not a combat pipeline; no combat damage passes through
them.

| Step | Port |
|---|---|
| Both sides present (`FUN_00509710`, `FUN_0050b8e0`) | `combat::side_present` |
| One session per contested system (`FUN_00516ef0` seats every fleet and the system's squadrons) | `BattleSession::new_with_forces` |
| Headless run | `BattleSession::simulate_to_end` |
| Write-back (`FUN_0040cfc0`, `FUN_005032c0`) | `tactical::apply::apply_results` |
| A battle nobody watches | `rebellion-data` `integrator::resolve_system_space_combat` |

### 2.1 Entity Kind Enum

`combat.rs` keeps a family-byte classification (`CombatEntityKind`) and the
packed-difficulty helper:

```rust
/// Mirrors the C++ family-byte classification for combat dispatch.
/// Derived from `entity.id >> 24` in C++; in Rust, from the DatId family byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatEntityKind {
    CapitalShip,             // family 0x30-0x33, 0x35-0x3b
    DeathStar,               // family 0x34 → FUN_005617b0 / FUN_00534640
    FighterSquadron,         // family 0x71-0x72 (alt_shield_path)
    SpecialCombatEntity,     // family 0x73-0x74 (FUN_00534640 result path)
    Troop,                   // family 0x14-0x1b
    Character,               // family 0x08-0x0f
}

impl CombatEntityKind {
    pub fn from_family_byte(family: u8) -> Self {
        match family {
            0x08..=0x0f => Self::Character,
            0x14..=0x1b => Self::Troop,
            0x30..=0x33 | 0x35..=0x3b => Self::CapitalShip,
            0x34 => Self::DeathStar,
            0x71..=0x72 => Self::FighterSquadron,
            0x73..=0x74 => Self::SpecialCombatEntity,
            _ => Self::CapitalShip, // safe default
        }
    }
}
```

The difficulty modifier (bits 4-5 of `+0x24`) extracted from C++ line 21 of `FUN_0054a1d0`:

```rust
/// Extract difficulty level (0-3) from the packed difficulty field.
/// C++: `*(uint *)((int)this + 0x24) >> 4 & 3`
pub fn extract_difficulty(packed: u32) -> u8 {
    ((packed >> 4) & 3) as u8
}
```

---

## 3. Ground Combat

### 3.1 The C++ Flow → Rust Iterator

`FUN_00560d50` (232 lines) iterates all troops at a system, checks regiment strength, and calls per-unit resolution. Direct Rust translation:

```rust
// crates/rebellion-core/src/combat.rs (continued)

pub struct GroundCombatResult {
    pub system: SystemKey,
    pub winner: CombatSide,
    pub troop_damage: Vec<TroopDamageEvent>,
    pub tick: u64,
}

#[derive(Debug, Clone)]
pub struct TroopDamageEvent {
    pub troop: TroopKey,
    pub strength_before: i16,
    pub strength_after: i16,
}

impl CombatSystem {
    /// Auto-resolve ground combat at a system between all opposing troop units.
    ///
    /// Mirrors FUN_00560d50 (232 lines): iterates troops at system, checks
    /// regiment_strength at +0x96, calls per-unit resolution (FUN_004ee350).
    ///
    /// # Advance contract
    /// - Does NOT mutate world. Returns TroopDamageEvents.
    /// - Death Star (family 0x34) takes a separate path — see resolve_death_star().
    /// - `rng_rolls`: one roll per troop pair engagement. Budget: troop_count² rolls.
    pub fn resolve_ground(
        world: &GameWorld,
        system: SystemKey,
        attacker_is_alliance: bool,
        rng_rolls: &[f64],
        tick: u64,
    ) -> GroundCombatResult {
        let system_data = &world.systems[system];
        let mut rng = rng_rolls.iter().copied();
        let mut troop_damage: Vec<TroopDamageEvent> = Vec::new();

        // FUN_00560d50 step 6-8: iterate all troops at system, filter by strength.
        // C++: `FUN_004f25c0(type=3)` iterates troops (family 0x14-0x1b).
        // In Rust: walk system.ground_units, look up TroopUnit in world.troops.
        let (attacker_troops, defender_troops): (Vec<TroopKey>, Vec<TroopKey>) =
            system_data.ground_units.iter()
                .copied()
                .partition(|&key| world.troops[key].is_alliance == attacker_is_alliance);

        // Regiment strength check (C++ offset +0x96, short):
        // skip units with strength <= 0 — they are destroyed / non-combat.
        let active_attackers: Vec<TroopKey> = attacker_troops.iter()
            .copied()
            .filter(|&key| world.troops[key].regiment_strength > 0)
            .collect();

        let active_defenders: Vec<TroopKey> = defender_troops.iter()
            .copied()
            .filter(|&key| world.troops[key].regiment_strength > 0)
            .collect();

        // Per-unit resolution (FUN_004ee350, 30 lines):
        // For each attacker-defender pair, resolve combat and emit damage events.
        // No friendly fire: same-side check mirrors C++ `param_2 == param_1 → skip`.
        for &atk_key in &active_attackers {
            for &def_key in &active_defenders {
                let atk = &world.troops[atk_key];
                let def = &world.troops[def_key];

                // Same-side guard (C++: `if (param_2 == param_1) continue`).
                // In Rust both are already in opposite partitions — guard is implicit.

                let roll = rng.next().unwrap_or(0.5);

                // FUN_004ee350 pseudocode:
                //   short old_strength = unit->regiment_strength;  // +0x96
                //   if (old_strength != param_1) {
                //       unit->regiment_strength = new_value;
                //       notify_side(unit, 1, ctx); notify_side(unit, 2, ctx);
                //       vtable[unit, +0x330](old, new, ctx);  // HullValueDamage etc.
                //   }
                //
                // Approximation (attack vs defense ratio, pending vtable +0x330 decompile):
                let atk_power = atk.regiment_strength as f64;
                let def_power = def.regiment_strength as f64;
                let hit_prob = atk_power / (atk_power + def_power);

                if roll < hit_prob {
                    // Attacker scores a hit on defender.
                    let old_strength = def.regiment_strength;
                    let reduction = (def.regiment_strength / 4).max(1);
                    let new_strength = (def.regiment_strength - reduction).max(0);
                    troop_damage.push(TroopDamageEvent {
                        troop: def_key,
                        strength_before: old_strength,
                        strength_after: new_strength,
                    });
                } else {
                    // Defender scores a hit on attacker.
                    let old_strength = atk.regiment_strength;
                    let reduction = (atk.regiment_strength / 4).max(1);
                    let new_strength = (atk.regiment_strength - reduction).max(0);
                    troop_damage.push(TroopDamageEvent {
                        troop: atk_key,
                        strength_before: old_strength,
                        strength_after: new_strength,
                    });
                }
            }
        }

        // Determine winner by counting surviving units after applying damage.
        // Build temporary strength maps from the damage events.
        let surviving_attackers = Self::count_survivors(
            &active_attackers, &troop_damage, world);
        let surviving_defenders = Self::count_survivors(
            &active_defenders, &troop_damage, world);

        let winner = match (surviving_attackers > 0, surviving_defenders > 0) {
            (true, false)  => CombatSide::Attacker,
            (false, true)  => CombatSide::Defender,
            _              => CombatSide::Draw,
        };

        GroundCombatResult { system, winner, troop_damage, tick }
    }

    fn count_survivors(
        troops: &[TroopKey],
        damage: &[TroopDamageEvent],
        world: &GameWorld,
    ) -> usize {
        troops.iter().filter(|&&key| {
            // Check if any damage event sets this unit to 0.
            if let Some(evt) = damage.iter().rev().find(|e| e.troop == key) {
                evt.strength_after > 0
            } else {
                world.troops[key].regiment_strength > 0
            }
        }).count()
    }
}
```

### 3.2 Applying Ground Combat Results

The caller in `main.rs` applies `TroopDamageEvent` to `GameWorld`:

```rust
// In main.rs apply_ground_combat_result():
for evt in result.troop_damage {
    if let Some(unit) = world.troops.get_mut(evt.troop) {
        unit.regiment_strength = evt.strength_after;
    }
}
// Remove destroyed units (strength == 0).
world.systems[result.system].ground_units.retain(|&key| {
    world.troops.get(key).map_or(false, |u| u.regiment_strength > 0)
});
```

### 3.3 Death Star Handler Stub

Family `0x34` triggers `FUN_005617b0` (68 lines, not yet fully decompiled):

```rust
impl CombatSystem {
    /// Death Star superlaser resolution.
    ///
    /// Called when a Death Star (family 0x34) is present at the system.
    /// FUN_005617b0 (68 lines) — pending full decompile.
    ///
    /// Stub: marks the system as destroyed and returns attacker victory.
    pub fn resolve_death_star(
        world: &GameWorld,
        system: SystemKey,
        tick: u64,
    ) -> GroundCombatResult {
        // TODO: decompile FUN_005617b0 for exact superlaser mechanics.
        GroundCombatResult {
            system,
            winner: CombatSide::Attacker, // Death Star always wins
            troop_damage: Vec::new(),
            tick,
        }
    }
}
```

---

## 4. Bombardment

### 4.1 The Original's Bombardment

Orders `0x220`-`0x223` pass the gate `FUN_00582440` and run
`FUN_0058d800`: a Death Star phase for Destroy System, KDY/LNR defence fire
at the bombarding ships, per-hit target rolls against each target's value,
and support shifts (`bombardment.md`). The chain
`FUN_00556430` → `FUN_00555d30` → `FUN_00555b30` → `FUN_0055d8c0` is the
transit-time distance (`build-delivery.md`), and `FUN_00509620` returns a
system's coordinates. The port's `bombardment.rs` still uses that distance;
task-graph node `or-port-bombardment-8usw` replaces it.

## 5. Mission System Expansion

### 5.1 All 9 Mission Types as an Enum

The current `MissionKind` in `missions.rs` only covers `Diplomacy` and `Recruitment`. MISSNSD.DAT defines 25 mission records; COMBAT-SUMMARY.md and the DAT format notes confirm at minimum these mission type codes:

| Code | Mission | *MSTB File | Skill Used |
|------|---------|------------|------------|
| — | Diplomacy | DIPLMSTB.DAT | `diplomacy` |
| — | Recruitment | RCRTMSTB.DAT | `leadership` |
| 6 | Sabotage | SBTGMSTB.DAT | `espionage` |
| 7 | Assassination | ASSNMSTB.DAT | `combat` |
| — | Espionage | ESPIMSTB.DAT | `espionage` |
| — | Rescue | RESCMSTB.DAT | `combat` |
| — | Abduction | ABDCMSTB.DAT | `espionage` |
| — | Incite Uprising | INCTMSTB.DAT | `diplomacy` |
| 21 | Autoscrap | — | — (auto, no character) |

```rust
// Extend MissionKind in crates/rebellion-core/src/missions.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionKind {
    // --- Implemented ---
    Diplomacy,
    Recruitment,

    // --- War Machine phase ---
    /// Destroy an enemy facility. SBTGMSTB.DAT. Code 6.
    Sabotage,
    /// Kill an enemy character. ASSNMSTB.DAT. Code 7.
    Assassination,
    /// Gather intelligence on an enemy system. ESPIMSTB.DAT.
    Espionage,
    /// Free a captured character. RESCMSTB.DAT.
    Rescue,
    /// Capture an enemy character. ABDCMSTB.DAT.
    Abduction,
    /// Trigger a planetary uprising. INCTMSTB.DAT.
    InciteUprising,
    /// Automatic scrapping of obsolete units. Code 21. No character required.
    Autoscrap,
}
```

### 5.2 Extending `coefficients()` for New Mission Types

Each `*MSTB.DAT` file is an `IntTableEntry` table (16B per entry): `id`, `field2`, `threshold` (i32 skill delta), `value` (probability 0-100). The quadratic formula from rebellion2 still applies — the coefficients are fit from these tables.

Until the *MSTB tables are fit, use placeholders that match the structural pattern:

```rust
impl MissionKind {
    pub fn coefficients(self) -> (f64, f64, f64) {
        match self {
            // Existing (from rebellion2 Mission.cs):
            MissionKind::Diplomacy    => (0.005558,  0.7656,  20.15),
            MissionKind::Recruitment  => (-0.001748, 0.8657,  11.923),

            // Placeholders — fit from *MSTB.DAT IntTableEntry tables:
            MissionKind::Sabotage     => (-0.002,    0.75,    15.0),  // SBTGMSTB.DAT
            MissionKind::Assassination=> (-0.003,    0.80,    10.0),  // ASSNMSTB.DAT
            MissionKind::Espionage    => (-0.002,    0.78,    12.0),  // ESPIMSTB.DAT
            MissionKind::Rescue       => (-0.002,    0.72,    10.0),  // RESCMSTB.DAT
            MissionKind::Abduction    => (-0.002,    0.70,     8.0),  // ABDCMSTB.DAT
            MissionKind::InciteUprising=>(-0.003,    0.65,    18.0),  // INCTMSTB.DAT
            MissionKind::Autoscrap    => (0.0,       0.0,   100.0),  // always succeeds
        }
    }

    /// Which character skill score determines success probability.
    pub fn skill(self) -> fn(&crate::world::Character) -> u32 {
        match self {
            MissionKind::Diplomacy      => |c| c.diplomacy.base,
            MissionKind::Recruitment    => |c| c.leadership.base,
            MissionKind::Sabotage       => |c| c.espionage.base,
            MissionKind::Assassination  => |c| c.combat.base,
            MissionKind::Espionage      => |c| c.espionage.base,
            MissionKind::Rescue         => |c| c.combat.base,
            MissionKind::Abduction      => |c| c.espionage.base,
            MissionKind::InciteUprising => |c| c.diplomacy.base,
            MissionKind::Autoscrap      => |_| 100, // always max
        }
    }
}
```

### 5.3 *MSTB Probability Tables as Rust Lookup Tables

The `IntTableEntry` format: `threshold` is a signed skill delta; `value` is probability at that threshold. These should be loaded from DAT at startup and used for linear interpolation, replacing the fitted quadratic coefficients eventually.

```rust
/// One entry in a *MSTB probability table.
/// Mirrors the IntTableEntry layout (16B): id, field2, threshold(i32), value(u32).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MstbEntry {
    pub threshold: i32,  // signed skill delta (negative = below average)
    pub value: u32,      // probability 0-100 at this threshold
}

/// A complete *MSTB table for one mission type.
/// Loaded from DIPLMSTB.DAT, SBTGMSTB.DAT, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MstbTable {
    pub entries: Vec<MstbEntry>,
}

impl MstbTable {
    /// Interpolate probability for a given skill score.
    /// Skill score is mapped to a threshold delta before lookup.
    pub fn lookup(&self, skill_score: u32) -> f64 {
        // Convert skill score (0-100) to threshold delta relative to midpoint (50).
        let threshold = skill_score as i32 - 50;

        // Find bracketing entries and linearly interpolate.
        let entries = &self.entries;
        if entries.is_empty() { return 50.0; }

        // Clamp to table bounds.
        if threshold <= entries.first().unwrap().threshold as i32 {
            return entries.first().unwrap().value as f64;
        }
        if threshold >= entries.last().unwrap().threshold as i32 {
            return entries.last().unwrap().value as f64;
        }

        // Linear interpolation between bracketing entries.
        for window in entries.windows(2) {
            let lo = &window[0];
            let hi = &window[1];
            if threshold >= lo.threshold && threshold <= hi.threshold {
                let t = (threshold - lo.threshold) as f64
                    / (hi.threshold - lo.threshold) as f64;
                return lo.value as f64 + t * (hi.value as f64 - lo.value as f64);
            }
        }

        50.0 // fallback
    }
}
```

Add to `GameWorld`:
```rust
/// Mission probability tables, one per MissionKind.
/// Loaded from *MSTB.DAT files by rebellion-data.
pub mission_tables: std::collections::HashMap<String, MstbTable>,
```

### 5.4 Extending `MissionEffect` for Combat Mission Outcomes

Current `MissionEffect` only covers `PopularityShifted` and `CharacterRecruited`. Add:

```rust
// In missions.rs MissionEffect enum:

/// A facility was sabotaged — reduce its remaining production ticks.
FacilitySabotaged {
    system: SystemKey,
    /// Index of the facility in system.manufacturing_facilities (or defense_facilities).
    facility_index: usize,
    /// Ticks of production lost.
    ticks_lost: u32,
},

/// A character was assassinated — remove from game.
CharacterKilled {
    character: CharacterKey,
    faction: MissionFaction,
},

/// A character was captured by the opposing faction.
CharacterCaptured {
    character: CharacterKey,
    captured_by: MissionFaction,
    at_system: SystemKey,
},

/// Intelligence gathered on a system — reveal fog of war.
SystemIntelligenceGathered {
    system: SystemKey,
    faction: MissionFaction,
},

/// Uprising started on a system — shift popularity.
UprisingStarted {
    system: SystemKey,
    /// Popularity shift applied immediately.
    popularity_delta: f32,
},
```

---

## 6. Integration Order

`rebellion-data/src/simulation.rs` runs combat after movement each tick
(`agent_docs/simulation.md`), and `rebellion-app` `main.rs` does the same for
battles no window opens:

1. Space: for each system where both sides are present,
   `integrator::resolve_system_space_combat(world, troop_transport,
   movement, system, rng_seed, tick)` runs one headless tactical session
   seeded from one roll, then the caller reconciles a destroyed Death Star
   (`tactical::apply::reconcile_death_star_result`).
2. Ground: `CombatSystem::resolve_ground(&world, system, alliance_attacks,
   &rolls, tick)`, applied by `apply_ground_combat_result_inner`.
3. Bombardment: `BombardmentSystem::resolve_bombardment`, the port formula
   (§4).

Pending decompile for ground combat: the per-unit resolve `FUN_004ee350`
(vtable `+0x330`). `FUN_005617b0` is the Palpatine-at-Coruscant flag, not a
Death Star handler (`or-ch-combat-family-notes-yfx`).

---

## Appendix: Entity Family Byte Quick Reference

For `CombatEntityKind::from_family_byte()` and routing in `resolve_ground()`:

| Family Range | Entity Type | Combat Path |
|---|---|---|
| `0x08-0x0f` | Characters | Side validation only (no direct combat) |
| `0x14-0x1b` | Troops / Special Forces | Ground combat — `FUN_00560d50` |
| `0x30-0x33`, `0x35-0x3b` | Capital Ships | Space combat — vtable +0x1c4 path |
| `0x34` | Death Star | `FUN_005617b0` special path |
| `0x71-0x72` | Fighter squadrons | Space combat — alt_shield_path (+0x78 bit7) |
| `0x73-0x74` | Special combat entity | Space result — `FUN_00534640` path |
| `0x90-0x98` | Star Systems | Not combat entities |
