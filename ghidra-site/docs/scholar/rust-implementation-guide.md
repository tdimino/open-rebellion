---
title: "Rust Implementation Guide: Combat & Mission Systems"
description: "Maps decompiled C++ from REBEXE.EXE to rebellion-core Rust implementation"
category: "ghidra"
created: 2026-03-15
updated: 2026-03-16
---

# Rust Implementation Guide: Combat & Mission Systems

*Maps decompiled C++ from REBEXE.EXE to `rebellion-core` Rust. Written for the Knesset Tiamat swarm.*

**Source notes**: `ghidra/notes/space-combat.md`, `ground-combat.md`, `bombardment.md`, `annotated-functions.md`, `COMBAT-SUMMARY.md`
**Target crate**: `crates/rebellion-core/src/`—zero IO, no rendering deps, all advance() pattern

---

## Index

1. [Game Object Layout → Rust Structs](#1-game-object-layout--rust-structs)
2. [Space Combat Auto-Resolve](#2-space-combat-auto-resolve)
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

**Add to `CapitalShipClass`** (class template—from CAPSHPSD.DAT):
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

**Add a new `ShipInstance` world type** (runtime per-hull state—replaces the count-only `ShipEntry`):
```rust
/// A single hull of a capital ship class—the runtime instance.
/// Distinct from ShipEntry which is a (class, count) summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipInstance {
    pub class: CapitalShipKey,
    /// Current hull. Starts at CapitalShipClass::hull, reduced by combat.
    pub hull_current: i32,
    /// Packed nibbles: bits 0-3 = shield_recharge_allocated, bits 4-7 = weapon_recharge_allocated.
    /// Mirrors C++ offset +0x64. Read via helpers shield_nibble() / weapon_nibble().
    /// Note: C++ writes via XOR-mask pattern `(new ^ old) & 0xf ^ old`—functionally equivalent
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

**Add to `FighterClass`** (template—from FIGHTSD.DAT):
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
    /// DAT_006bb6e8—bombardment damage divisor.
    /// Scales Euclidean-distance raw power to final damage.
    pub bombardment_divisor: [i32; 4],
    /// DAT_00661a88—difficulty modifier table (4 entries, one per difficulty).
    pub difficulty_modifier: [i32; 4],
}
```

Add `pub gnprtb: GnprtbParams` to `GameWorld`. Populated by `rebellion-data` during loading.

---

## 2. Space Combat Auto-Resolve

### 2.1 The 7-Phase C++ Pipeline → Rust Enum + Match

The original orchestrator (`FUN_00549910`, 77 lines, called from `FUN_005457f0`) drives combat through 7 phases using a state machine on `combat_phase_flags` at `this+0x58`. Each phase:

1. Checks a gate bit on the flags register
2. Retrieves per-side stat objects via a getter
3. Dispatches the actual resolver via vtable

In Rust, we replace the vtable dispatch with an enum + match pattern. The flags register becomes `CombatPhaseFlags`.

### 2.2 `CombatPhaseFlags` Bitfield

Direct translation of `this+0x58` (confirmed from space-combat.md):

```rust
use bitflags::bitflags;

bitflags! {
    /// Mirrors the C++ combat_phase_flags at object offset +0x58.
    /// Controls which phases are active and which type codes are set.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CombatPhaseFlags: u32 {
        /// Space combat active (outer wrapper gate, checked by FUN_00544030).
        const ACTIVE           = 0x0001;
        /// Weapon fire is in progress (attacker side).
        const WEAPON_FIRE      = 0x0002;
        /// Inner gate mask for weapon fire phase (0x04, checked by FUN_0053a640).
        /// Note: outer wrapper gates on ACTIVE & !PHASES_ENABLED; inner vtable dispatch uses this mask.
        const WEAPON_TYPE      = 0x0004;
        /// Type code: shield absorb phase (0x20).
        const SHIELD_TYPE      = 0x0020;
        /// All subsequent phases enabled (shields, hull, fighters).
        const PHASES_ENABLED   = 0x0040;
        /// Type code: hull damage phase (0x80).
        const HULL_TYPE        = 0x0080;
        /// Type code: fighter engagement phase (0x100).
        const FIGHTER_TYPE     = 0x0100;
    }
}
```

Gate logic (straight from the phase table in space-combat.md):

- Weapon fire runs when: `flags.contains(ACTIVE) && !flags.contains(PHASES_ENABLED)` → i.e., `0x58 & 0x01` is set AND `0x58 & 0x40` is clear
- Shield absorb, hull damage, fighter engagement all run when: `flags.contains(PHASES_ENABLED)`

### 2.3 `CombatSystem::resolve_space()`—Full Signature and Skeleton

```rust
// crates/rebellion-core/src/combat.rs

use crate::ids::{FleetKey, SystemKey};
use crate::world::GameWorld;
use crate::tick::TickEvent;

// ---------------------------------------------------------------------------
// Public result types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SpaceCombatResult {
    pub attacker_fleet: FleetKey,
    pub defender_fleet: FleetKey,
    pub system: SystemKey,
    pub winner: CombatSide,
    pub ship_damage: Vec<ShipDamageEvent>,
    pub fighter_losses: Vec<FighterLossEvent>,
    pub tick: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatSide { Attacker, Defender, Draw }

#[derive(Debug, Clone)]
pub struct ShipDamageEvent {
    pub fleet: FleetKey,
    /// Index into fleet.capital_ships (not a slotmap key—instances are transient).
    pub ship_index: usize,
    pub hull_before: i32,
    pub hull_after: i32,
}

#[derive(Debug, Clone)]
pub struct FighterLossEvent {
    pub fleet: FleetKey,
    pub fighter_index: usize,
    pub squads_before: u32,
    pub squads_after: u32,
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Transient per-battle combat state—alive for one resolve_space() call.
/// Never stored in GameWorld; emitted as result events.
struct SpaceCombatState {
    /// Mutable copies of the attacker's ship hulls for this resolution.
    attacker_ships: Vec<ShipInstanceSnapshot>,
    /// Mutable copies of the defender's ship hulls.
    defender_ships: Vec<ShipInstanceSnapshot>,
    /// Attacker fighter squadron counts (mutable for the engagement phase).
    attacker_fighters: Vec<u32>,
    defender_fighters: Vec<u32>,
    flags: CombatPhaseFlags,
    difficulty: u8,  // 0-3, extracted from C++ +0x24 bits 4-5
}

struct ShipInstanceSnapshot {
    class: CapitalShipKey,
    hull_current: i32,
    hull_max: i32,
    shield_nibble: u8,
    weapon_nibble: u8,
    alive: bool,
}

// ---------------------------------------------------------------------------
// System
// ---------------------------------------------------------------------------

pub struct CombatSystem;

impl CombatSystem {
    /// Auto-resolve space combat between two fleets at a system.
    ///
    /// Follows the 7-phase C++ pipeline from FUN_005457f0 → FUN_00549910.
    ///
    /// # Advance contract
    /// - Does NOT mutate `world`. All fleet state changes are returned as events.
    /// - `rng_rolls`: caller-provided uniform [0,1) values. Consumed in order.
    ///   Current budget: 4 rolls per ship per phase (weapon + shield + hull + fighter).
    ///   Pass a slice of at least `fleet_size * 4 * max_rounds` rolls.
    pub fn resolve_space(
        world: &GameWorld,
        attacker: FleetKey,
        defender: FleetKey,
        system: SystemKey,
        difficulty: u8,
        rng_rolls: &[f64],
        tick: u64,
    ) -> SpaceCombatResult {
        let mut rng = rng_rolls.iter().copied();

        // Phase 1: Initialize (FUN_005442f0)
        let mut state = Self::phase_init(world, attacker, defender, difficulty);

        // Phase 2: Fleet composition evaluation (FUN_00544da0, 96 lines)
        // Sets PHASES_ENABLED flag if either side has active weapons.
        Self::phase_fleet_eval(world, &mut state);

        // Phase 3: Weapon fire—gate: ACTIVE && !PHASES_ENABLED
        // Calls FUN_00543b60 for each side (attacker fires first).
        if state.flags.contains(CombatPhaseFlags::ACTIVE)
            && !state.flags.contains(CombatPhaseFlags::PHASES_ENABLED)
        {
            Self::phase_weapon_fire(world, &mut state, &mut rng);
        }

        // Phases 4-6 all gate on PHASES_ENABLED.
        if state.flags.contains(CombatPhaseFlags::PHASES_ENABLED) {
            // Phase 4: Shield absorption (FUN_00544130, 83 lines)
            Self::phase_shield_absorb(world, &mut state, &mut rng);

            // Phase 5: Hull damage application (FUN_005443f0, 54 lines)
            Self::phase_hull_damage(world, &mut state, &mut rng);

            // Phase 6: Fighter engagement (FUN_005444e0, 53 lines)
            Self::phase_fighter_engage(world, &mut state, &mut rng);
        }

        // Phase 7: Result determination (FUN_005445d0, 175 lines)
        // Counts survivors, checks alive_flag at +0xac bit0.
        // Exception: fighters with +0x50 & 0x08 count as ready even if hull=0.
        // Family 0x73/0x74 take a special path (FUN_00534640).
        let winner = Self::phase_determine_result(world, &state);

        // Phase 8: Post-combat cleanup (FUN_00544a20, 86 lines)—handled by caller
        // (apply damage events to GameWorld).

        Self::build_result(attacker, defender, system, &state, winner, tick)
    }

    // ------------------------------------------------------------------
    // Phase implementations
    // ------------------------------------------------------------------

    fn phase_init(
        world: &GameWorld,
        attacker: FleetKey,
        defender: FleetKey,
        difficulty: u8,
    ) -> SpaceCombatState {
        let a_fleet = &world.fleets[attacker];
        let d_fleet = &world.fleets[defender];

        let attacker_ships = a_fleet.capital_ships.iter().map(|entry| {
            let class = &world.capital_ship_classes[entry.class];
            ShipInstanceSnapshot {
                class: entry.class,
                hull_current: class.hull as i32,
                hull_max: class.hull as i32,
                // Initial nibbles: max recharge allocated to each
                shield_nibble: (class.shield_recharge.min(15)) as u8,
                weapon_nibble: 0x0f, // full weapon allocation at start
                alive: true,
            }
        }).collect();

        // Mirror for defender.
        let defender_ships = d_fleet.capital_ships.iter().map(|entry| {
            let class = &world.capital_ship_classes[entry.class];
            ShipInstanceSnapshot {
                class: entry.class,
                hull_current: class.hull as i32,
                hull_max: class.hull as i32,
                shield_nibble: (class.shield_recharge.min(15)) as u8,
                weapon_nibble: 0x0f,
                alive: true,
            }
        }).collect();

        let attacker_fighters = a_fleet.fighters.iter().map(|e| e.count).collect();
        let defender_fighters = d_fleet.fighters.iter().map(|e| e.count).collect();

        SpaceCombatState {
            attacker_ships,
            defender_ships,
            attacker_fighters,
            defender_fighters,
            // Start with ACTIVE; PHASES_ENABLED is set by fleet_eval.
            flags: CombatPhaseFlags::ACTIVE,
            difficulty,
        }
    }

    fn phase_fleet_eval(world: &GameWorld, state: &mut SpaceCombatState) {
        // FUN_00544da0 (96 lines): checks fleet composition.
        // Sets PHASES_ENABLED if either side has active weapons (weapon_nibble > 0).
        // In practice this is almost always true—guard kept for correctness.
        let any_attacker_armed = state.attacker_ships.iter()
            .any(|s| s.alive && s.weapon_nibble > 0);
        let any_defender_armed = state.defender_ships.iter()
            .any(|s| s.alive && s.weapon_nibble > 0);

        if any_attacker_armed || any_defender_armed {
            state.flags |= CombatPhaseFlags::PHASES_ENABLED;
        }
    }

    fn phase_weapon_fire(
        world: &GameWorld,
        state: &mut SpaceCombatState,
        rng: &mut impl Iterator<Item = f64>,
    ) {
        // FUN_00543b60—per-side weapon fire, vtable +0x1c4 dispatch.
        // Attacker fires first (side=1), then defender (side=0).
        // Weapon damage is aggregated per side, then distributed.
        //
        // Approximation (full formula behind vtable +0x1c4, not yet decompiled):
        // total_fire = sum of weapon_nibble * class.weapons_* for alive ships.
        // Pending: decompile vtable +0x1c4 handler for exact per-arc formula.
        Self::fire_side(world, &state.attacker_ships, &mut state.defender_ships, rng);
        Self::fire_side(world, &state.defender_ships, &mut state.attacker_ships, rng);
    }

    fn fire_side(
        world: &GameWorld,
        firing: &[ShipInstanceSnapshot],
        targets: &mut Vec<ShipInstanceSnapshot>,
        rng: &mut impl Iterator<Item = f64>,
    ) {
        // Aggregate weapon power across all alive firing ships.
        let total_fire: u32 = firing.iter()
            .filter(|s| s.alive)
            .map(|s| {
                let class = &world.capital_ship_classes[s.class];
                // Sum across all 4 arcs, all 3 weapon types, scaled by allocation nibble.
                let raw_weapons = class.weapons_fore_turbolaser
                    + class.weapons_fore_ion
                    + class.weapons_fore_laser
                    + class.weapons_aft_turbolaser
                    + class.weapons_aft_ion
                    + class.weapons_aft_laser
                    + class.weapons_port_turbolaser
                    + class.weapons_port_ion
                    + class.weapons_port_laser
                    + class.weapons_starboard_turbolaser
                    + class.weapons_starboard_ion
                    + class.weapons_starboard_laser;
                // weapon_nibble (0-15) scales output: 0=no fire, 15=full
                (raw_weapons * s.weapon_nibble as u32) / 15
            })
            .sum();

        if total_fire == 0 { return; }

        // Distribute fire across alive targets (round-robin weighted by hull fraction).
        // TODO: replace with exact formula from vtable +0x1c4 once decompiled.
        let alive_targets: Vec<usize> = targets.iter().enumerate()
            .filter(|(_, t)| t.alive)
            .map(|(i, _)| i)
            .collect();
        if alive_targets.is_empty() { return; }

        let fire_per_target = total_fire / alive_targets.len() as u32;
        for idx in alive_targets {
            let roll = rng.next().unwrap_or(0.5);
            // Apply variance: ±20% around mean fire_per_target.
            let variance = (fire_per_target as f64 * 0.2 * (roll * 2.0 - 1.0)) as i32;
            let damage = (fire_per_target as i32 + variance).max(0);
            let target = &mut targets[idx];
            // Raw fire goes into the shield/hull pipeline in phases 4-5.
            // Store as pending damage on hull_current for now; shield phase will absorb.
            target.hull_current = (target.hull_current - damage).max(0);
            if target.hull_current == 0 {
                target.alive = false; // alive_flag +0xac bit0 cleared
            }
        }
    }

    fn phase_shield_absorb(
        world: &GameWorld,
        state: &mut SpaceCombatState,
        rng: &mut impl Iterator<Item = f64>,
    ) {
        // FUN_00544130 (83 lines)—vtable +0x1c8 dispatch.
        // Shields absorb a portion of incoming weapon damage before hull is hit.
        //
        // Known: shield_nibble (bits 0-3 of +0x64) = allocated shield recharge.
        // Full formula not yet decompiled (pending Ghidra session).
        //
        // Approximation: shields absorb (shield_nibble / 15) fraction of damage.
        // This function modifies ship state in-place; no separate events returned.
        //
        // TODO: Replace with exact formula from FUN_00544130 decompile.
        let _ = (world, rng); // suppress unused until decompiled
    }

    fn phase_hull_damage(
        world: &GameWorld,
        state: &mut SpaceCombatState,
        rng: &mut impl Iterator<Item = f64>,
    ) {
        // FUN_005443f0 (54 lines)—vtable +0x1d0 dispatch.
        // Remaining damage after shield absorption is applied to hull_current.
        // Hull setter (FUN_00501490): range-checked [0, max_hull], notifies both sides.
        //
        // No-op guard: only fires if new_hull != current_hull (from FUN_00501490).
        //
        // TODO: Replace with exact formula once FUN_005443f0 is decompiled.
        let _ = (world, rng);
    }

    fn phase_fighter_engage(
        world: &GameWorld,
        state: &mut SpaceCombatState,
        rng: &mut impl Iterator<Item = f64>,
    ) {
        // FUN_005444e0 (53 lines)—vtable +0x1d4 dispatch.
        // Fighter engagement: squadrons attack each other and enemy capital ships.
        //
        // Entity path: family 0x71-0x72 triggers the alt_shield_path (+0x78 bit7).
        // Dead fighters with +0x50 & 0x08 still count for fleet eval in phase 7.
        //
        // TODO: Replace with exact formula once FUN_005444e0 is decompiled.
        let _ = (world, rng);
    }

    fn phase_determine_result(
        world: &GameWorld,
        state: &SpaceCombatState,
    ) -> CombatSide {
        // FUN_005445d0 (175 lines):
        // 1. Iterates all units, checks alive_flag at +0xac bit0.
        // 2. Fighter exception: +0x50 & 0x08 → still combat-ready even if hull=0.
        // 3. Family 0x73/0x74 (special entity, likely Death Star) → FUN_00534640 path.
        // 4. Calls FUN_00543d90 per side to determine winner.
        let attacker_alive = state.attacker_ships.iter().any(|s| s.alive)
            || state.attacker_fighters.iter().any(|&c| c > 0);
        let defender_alive = state.defender_ships.iter().any(|s| s.alive)
            || state.defender_fighters.iter().any(|&c| c > 0);

        match (attacker_alive, defender_alive) {
            (true, false)  => CombatSide::Attacker,
            (false, true)  => CombatSide::Defender,
            (true, true)   => CombatSide::Draw,
            (false, false) => CombatSide::Draw, // mutual annihilation
        }
    }

    fn build_result(
        attacker: FleetKey,
        defender: FleetKey,
        system: SystemKey,
        state: &SpaceCombatState,
        winner: CombatSide,
        tick: u64,
    ) -> SpaceCombatResult {
        let mut ship_damage = Vec::new();

        for (i, snap) in state.attacker_ships.iter().enumerate() {
            if snap.hull_current < snap.hull_max {
                ship_damage.push(ShipDamageEvent {
                    fleet: attacker,
                    ship_index: i,
                    hull_before: snap.hull_max,
                    hull_after: snap.hull_current,
                });
            }
        }
        for (i, snap) in state.defender_ships.iter().enumerate() {
            if snap.hull_current < snap.hull_max {
                ship_damage.push(ShipDamageEvent {
                    fleet: defender,
                    ship_index: i,
                    hull_before: snap.hull_max,
                    hull_after: snap.hull_current,
                });
            }
        }

        SpaceCombatResult {
            attacker_fleet: attacker,
            defender_fleet: defender,
            system,
            winner,
            ship_damage,
            fighter_losses: Vec::new(), // populated once phase_fighter_engage is complete
            tick,
        }
    }
}
```

### 2.4 Vtable Dispatch → Rust Entity Kind Enum

The C++ vtable at `+0x1c4`, `+0x1c8`, `+0x1d0`, `+0x1d4` dispatches different behaviors depending on entity class (capital ship vs fighter vs Death Star). In Rust, use an enum instead:

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
    /// - Death Star (family 0x34) takes a separate path—see resolve_death_star().
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
        // skip units with strength <= 0—they are destroyed / non-combat.
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
                // In Rust both are already in opposite partitions—guard is implicit.

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
    /// FUN_005617b0 (68 lines)—pending full decompile.
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

### 4.1 Direct Formula Translation

The full C++ call chain:

```
FUN_00556430 → FUN_00555d30 → FUN_00555b30
  → FUN_00509620 (get combat stats as short[2])
  → FUN_0055d8c0 (damage formula)
    → FUN_0055d860: raw_power = sqrt((atk[0]-def[0])² + (atk[1]-def[1])²)
    → / DAT_006bb6e8 (GNPRTB bombardment divisor)
    → FUN_0053e190 (apply difficulty modifier)
  → minimum 1 damage
```

The `short[2]` stats from `FUN_00509620`:
- `[0]` = bombardment attack strength (attacker) / bombardment defense rating (defender)
- `[1]` = secondary stat (likely detection or maneuverability modifier—confirm via Ghidra)

```rust
// crates/rebellion-core/src/combat.rs (continued)

#[derive(Debug, Clone)]
pub struct BombardmentResult {
    pub system: SystemKey,
    pub damage: i32,
    pub tick: u64,
}

impl CombatSystem {
    /// Orbital bombardment damage formula.
    ///
    /// Direct translation of FUN_0055d8c0 → FUN_0055d860 from REBEXE.EXE.
    ///
    /// # Formula
    /// ```text
    /// raw_power = sqrt((atk_bombardment - def_bombardment)² + (atk_secondary - def_secondary)²)
    /// damage = apply_modifier(raw_power / gnprtb_divisor, difficulty)
    /// if damage == 0: damage = 1  // minimum 1 (confirmed from C++)
    /// ```
    ///
    /// # Advance contract
    /// - Does NOT mutate world.
    /// - `gnprtb_divisor`: `world.gnprtb.bombardment_divisor[difficulty as usize]`
    /// - `difficulty_modifier`: `world.gnprtb.difficulty_modifier[difficulty as usize]`
    pub fn resolve_bombardment(
        world: &GameWorld,
        attacker_fleet: FleetKey,
        system: SystemKey,
        difficulty: u8,
        tick: u64,
    ) -> BombardmentResult {
        // Guard: bombardment disabled if system flag this[0x14] & 0x800 is set.
        // In Rust: check a future system.bombardment_blocked flag.
        // (Not yet in world model—add when implementing blockade mechanics.)

        // Guard: no self-bombardment (uVar1 == uVar2 check in FUN_00556430).
        let attacker_fleet_data = &world.fleets[attacker_fleet];
        let system_data = &world.systems[system];
        if attacker_fleet_data.is_alliance == system_data.is_alliance_controlled() {
            return BombardmentResult { system, damage: 0, tick };
        }

        // FUN_00509620: get combat stats as (attack, secondary) pair.
        // Attacker: aggregate bombardment stat across fleet.
        let atk = Self::fleet_bombardment_stats(world, attacker_fleet);

        // Defender: aggregate defense from defense facilities + troop bombardment_defense.
        let def = Self::system_bombardment_defense(world, system);

        // FUN_0055d860: power_ratio = Euclidean distance.
        // C++: sqrt((atk[0]-def[0])² + (atk[1]-def[1])²)
        let dx = (atk.0 - def.0) as f64;
        let dy = (atk.1 - def.1) as f64;
        let raw_power = (dx * dx + dy * dy).sqrt();

        if raw_power == 0.0 {
            return BombardmentResult { system, damage: 0, tick };
        }

        // DAT_006bb6e8: GNPRTB bombardment divisor (difficulty-indexed).
        let gnprtb_divisor = world.gnprtb.bombardment_divisor[difficulty as usize];
        let divisor = gnprtb_divisor.max(1) as f64; // guard against divide-by-zero

        // FUN_0053e190: apply difficulty modifier.
        let pre_mod = raw_power / divisor;
        let modifier = world.gnprtb.difficulty_modifier[difficulty as usize];
        let damage_f = pre_mod * (modifier as f64 / 100.0);

        // Minimum 1 damage (confirmed from C++ `result == 0 ? 1 : result`).
        let damage = (damage_f.floor() as i32).max(1);

        BombardmentResult { system, damage, tick }
    }

    /// Aggregate bombardment attack stats (short[2]) for a fleet.
    /// Returns (bombardment_strength, secondary_stat).
    /// Maps to FUN_00509620 attacker call.
    fn fleet_bombardment_stats(world: &GameWorld, fleet: FleetKey) -> (i32, i32) {
        let fleet_data = &world.fleets[fleet];
        let mut brd: i32 = 0;
        let mut sec: i32 = 0;

        for entry in &fleet_data.capital_ships {
            let class = &world.capital_ship_classes[entry.class];
            brd += class.bombardment as i32 * entry.count as i32;
            // Secondary: use maneuverability as proxy until FUN_00509620 is decompiled.
            sec += class.maneuverability as i32 * entry.count as i32;
        }
        for entry in &fleet_data.fighters {
            let class = &world.fighter_classes[entry.class];
            brd += class.bombardment as i32 * entry.count as i32;
        }

        (brd, sec)
    }

    /// Aggregate bombardment defense stats (short[2]) for a system.
    /// Returns (defense_rating, secondary_stat).
    /// Maps to FUN_00509620 defender call.
    fn system_bombardment_defense(world: &GameWorld, system: SystemKey) -> (i32, i32) {
        let system_data = &world.systems[system];
        let mut def: i32 = 0;
        let mut sec: i32 = 0;

        // Defense facilities contribute bombardment_defense.
        // (DefenseFacilityClass needs bombardment_defense field—add to world model.)
        for &key in &system_data.defense_facilities {
            // TODO: look up DefenseFacilityClass by class_dat_id and sum bombardment_defense.
            // Placeholder: each facility contributes 10 defense.
            let _ = key;
            def += 10;
        }

        // Troop regiment strength contributes secondary defense.
        for &key in &system_data.ground_units {
            let unit = &world.troops[key];
            sec += unit.regiment_strength as i32;
        }

        (def, sec)
    }
}
```

### 4.2 `world.gnprtb` Lookup Pattern

At every combat call site in `main.rs`, the difficulty-indexed lookup looks like:

```rust
// main.rs—before calling resolve_bombardment:
let difficulty = game_state.difficulty as u8; // 0=easy, 1=medium, 2=hard, 3=impossible
let result = CombatSystem::resolve_bombardment(&world, attacker, system, difficulty, tick);
```

Inside the function:
```rust
// The two confirmed global addresses from COMBAT-SUMMARY.md:
// DAT_006bb6e8 → world.gnprtb.bombardment_divisor[difficulty]
// DAT_00661a88 → world.gnprtb.difficulty_modifier[difficulty]
let divisor = world.gnprtb.bombardment_divisor[difficulty as usize];
let modifier = world.gnprtb.difficulty_modifier[difficulty as usize];
```

---

## 5. Mission System Expansion

### 5.1 All 9 Mission Types as an Enum

The current `MissionKind` in `missions.rs` only covers `Diplomacy` and `Recruitment`. MISSNSD.DAT defines 25 mission records; COMBAT-SUMMARY.md and the DAT format notes confirm at minimum these mission type codes:

| Code | Mission | *MSTB File | Skill Used |
|------|---------|------------|------------|
|—| Diplomacy | DIPLMSTB.DAT | `diplomacy` |
|—| Recruitment | RCRTMSTB.DAT | `leadership` |
| 6 | Sabotage | SBTGMSTB.DAT | `espionage` |
| 7 | Assassination | ASSNMSTB.DAT | `combat` |
|—| Espionage | ESPIMSTB.DAT | `espionage` |
|—| Rescue | RESCMSTB.DAT | `combat` |
|—| Abduction | ABDCMSTB.DAT | `espionage` |
|—| Incite Uprising | INCTMSTB.DAT | `diplomacy` |
| 21 | Autoscrap |—|—(auto, no character) |

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

Each `*MSTB.DAT` file is an `IntTableEntry` table (16B per entry): `id`, `field2`, `threshold` (i32 skill delta), `value` (probability 0-100). The quadratic formula from rebellion2 still applies—the coefficients are fit from these tables.

Until the *MSTB tables are fit, use placeholders that match the structural pattern:

```rust
impl MissionKind {
    pub fn coefficients(self) -> (f64, f64, f64) {
        match self {
            // Existing (from rebellion2 Mission.cs):
            MissionKind::Diplomacy    => (0.005558,  0.7656,  20.15),
            MissionKind::Recruitment  => (-0.001748, 0.8657,  11.923),

            // Placeholders—fit from *MSTB.DAT IntTableEntry tables:
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

/// A facility was sabotaged—reduce its remaining production ticks.
FacilitySabotaged {
    system: SystemKey,
    /// Index of the facility in system.manufacturing_facilities (or defense_facilities).
    facility_index: usize,
    /// Ticks of production lost.
    ticks_lost: u32,
},

/// A character was assassinated—remove from game.
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

/// Intelligence gathered on a system—reveal fog of war.
SystemIntelligenceGathered {
    system: SystemKey,
    faction: MissionFaction,
},

/// Uprising started on a system—shift popularity.
UprisingStarted {
    system: SystemKey,
    /// Popularity shift applied immediately.
    popularity_delta: f32,
},
```

---

## 6. Integration Order

### 6.1 New Module Registration

1. Create `crates/rebellion-core/src/combat.rs` with all types from §2-4
2. Add to `crates/rebellion-core/src/lib.rs`:
   ```rust
   pub mod combat;
   ```
3. Add `GnprtbParams` to `world/mod.rs` and `GameWorld`
4. Add `MstbTable` / `MstbEntry` to `missions.rs` (or a new `mstb.rs` if preferred)
5. Add `campaign combat` loading step to `rebellion-data/src/lib.rs`

### 6.2 Wire into `main.rs`

Combat fires on `ArrivalEvent`—when a fleet arrives at a system occupied by an enemy fleet. Ground combat fires after space combat resolves. Bombardment fires each tick if a blocking fleet has orbital bombardment capability:

```rust
// In main.rs, add after movement step:

// 1. Space combat: check each arrival for hostile fleet presence.
for arrival in &arrivals {
    if let Some(enemy_fleet) = find_hostile_fleet_at(arrival.system, &world) {
        let rolls = draw_rng_rolls(&mut rng, SPACE_COMBAT_RNG_BUDGET);
        let result = CombatSystem::resolve_space(
            &world,
            arrival.fleet,
            enemy_fleet,
            arrival.system,
            game_state.difficulty,
            &rolls,
            tick,
        );
        apply_space_combat_result(&mut world, result, &mut message_log);
    }
}

// 2. Ground combat: check systems where both factions have troops.
for system_key in systems_with_ground_conflict(&world) {
    let rolls = draw_rng_rolls(&mut rng, GROUND_COMBAT_RNG_BUDGET);
    let result = CombatSystem::resolve_ground(
        &world, system_key, attacker_is_alliance, &rolls, tick,
    );
    apply_ground_combat_result(&mut world, result, &mut message_log);
}

// 3. Bombardment: check systems under blockade.
for (fleet_key, system_key) in blockading_fleets(&world) {
    let result = CombatSystem::resolve_bombardment(
        &world, fleet_key, system_key, game_state.difficulty, tick,
    );
    apply_bombardment_result(&mut world, result, &mut message_log);
}
```

### 6.3 RNG Budget Constants

Define at the top of `combat.rs` for test harness use:

```rust
/// RNG rolls consumed per ship per space combat call.
/// Budget: weapon_fire + shield_absorb + hull_damage + fighter_engage = 4 phases.
/// Times max ships per side (30 classes × 4 phases = 120 minimum).
pub const SPACE_COMBAT_RNG_PER_SHIP: usize = 4;

/// RNG rolls consumed per troop pair in ground combat.
pub const GROUND_COMBAT_RNG_PER_PAIR: usize = 1;
```

### 6.4 What Remains Pending Decompile

These phases have confirmed C++ addresses but the exact formulas are behind vtable calls not yet decompiled. The stubs above are reasonable approximations for playable behavior. Priority order for next Ghidra session:

| Function | Address | Phase | Blocker |
|----------|---------|-------|---------|
| `FUN_00544130` | shield absorb | Phase 4 | 83 lines, vtable +0x1c8 |
| `FUN_005443f0` | hull damage apply | Phase 5 | 54 lines, vtable +0x1d0 |
| `FUN_005444e0` | fighter engagement | Phase 6 | 53 lines, vtable +0x1d4 |
| vtable `+0x1c4` handler | weapon fire calc | Phase 3 | dispatch target unknown |
| `FUN_004ee350` | per-unit ground resolve | Ground | 30 lines, vtable +0x330 |
| `FUN_005617b0` | Death Star handler | Ground | 68 lines |
| `FUN_00509620` | combat stat getter | Bombardment | secondary stat identity |

---

## Appendix: Entity Family Byte Quick Reference

For `CombatEntityKind::from_family_byte()` and routing in `resolve_ground()`:

| Family Range | Entity Type | Combat Path |
|---|---|---|
| `0x08-0x0f` | Characters | Side validation only (no direct combat) |
| `0x14-0x1b` | Troops / Special Forces | Ground combat—`FUN_00560d50` |
| `0x30-0x33`, `0x35-0x3b` | Capital Ships | Space combat—vtable +0x1c4 path |
| `0x34` | Death Star | `FUN_005617b0` special path |
| `0x71-0x72` | Fighter squadrons | Space combat—alt_shield_path (+0x78 bit7) |
| `0x73-0x74` | Special combat entity | Space result—`FUN_00534640` path |
| `0x90-0x98` | Star Systems | Not combat entities |
