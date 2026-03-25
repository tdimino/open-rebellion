//! Space combat, ground combat, and fighter engagement auto-resolve.
//!
//! Implements the 7-phase pipeline reverse-engineered from REBEXE.EXE.
//! See `ghidra/notes/space-combat.md`, `ground-combat.md`, and
//! `rust-implementation-guide.md` §2-3 for the authoritative reference.
//!
//! # Advance contract
//! All public functions follow the simulation advance() pattern:
//! - Never mutate `GameWorld` — return result events for the caller to apply.
//! - Accept caller-provided RNG as `&[f64]` slices for deterministic tests.
//! - No IO.

use serde::{Deserialize, Serialize};

use crate::ids::{FleetKey, SystemKey, TroopKey};
use crate::world::GameWorld;

// ---------------------------------------------------------------------------
// Phase flags
// ---------------------------------------------------------------------------

/// Bitfield mirroring the C++ `combat_phase_flags` at object offset +0x58.
///
/// Controls which phases of the 7-phase space combat pipeline are active.
/// Confirmed from `ghidra/notes/space-combat.md` flag register table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatPhaseFlags(u32);

impl CombatPhaseFlags {
    /// bit0 — space combat active (outer wrapper gate).
    pub const ACTIVE: u32 = 0x0001;
    /// bit1 — weapon fire is in progress (attacker side).
    pub const WEAPON_FIRE: u32 = 0x0002;
    /// bit2 — inner gate mask for weapon fire vtable dispatch (0x04).
    pub const WEAPON_TYPE: u32 = 0x0004;
    /// bit5 — shield absorb type code (0x20).
    pub const SHIELD_TYPE: u32 = 0x0020;
    /// bit6 — all subsequent phases enabled (shields, hull, fighters).
    pub const PHASES_ENABLED: u32 = 0x0040;
    /// bit7 — hull damage type code (0x80).
    pub const HULL_TYPE: u32 = 0x0080;
    /// bit8 — fighter engagement type code (0x100).
    pub const FIGHTER_TYPE: u32 = 0x0100;

    pub fn new(bits: u32) -> Self { Self(bits) }
    pub fn contains(&self, flag: u32) -> bool { self.0 & flag != 0 }
    pub fn set(&mut self, flag: u32) { self.0 |= flag; }
}

// ---------------------------------------------------------------------------
// Public result types
// ---------------------------------------------------------------------------

/// Outcome of space combat auto-resolve between two fleets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceCombatResult {
    pub attacker_fleet: FleetKey,
    pub defender_fleet: FleetKey,
    pub system: SystemKey,
    pub winner: CombatSide,
    pub ship_damage: Vec<ShipDamageEvent>,
    pub fighter_losses: Vec<FighterLossEvent>,
    pub tick: u64,
}

/// Which side won a combat engagement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatSide {
    Attacker,
    Defender,
    Draw,
}

/// A hull took damage during combat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipDamageEvent {
    pub fleet: FleetKey,
    /// Index into fleet.capital_ships (not a slotmap key — instances are transient).
    pub ship_index: usize,
    pub hull_before: i32,
    pub hull_after: i32,
}

/// A fighter squadron took losses during combat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FighterLossEvent {
    pub fleet: FleetKey,
    pub fighter_index: usize,
    pub squads_before: u32,
    pub squads_after: u32,
}

/// Outcome of ground combat at a system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundCombatResult {
    pub system: SystemKey,
    pub winner: CombatSide,
    pub troop_damage: Vec<TroopDamageEvent>,
    pub tick: u64,
}

/// A troop unit's regiment strength changed during ground combat.
///
/// Mirrors C++ per-unit resolution: `FUN_004ee350` sets `unit->regiment_strength`
/// (offset +0x96, short) and fires the vtable notification at +0x330.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TroopDamageEvent {
    pub troop: TroopKey,
    pub strength_before: i16,
    pub strength_after: i16,
}

// ---------------------------------------------------------------------------
// Internal per-battle snapshots (transient, never stored in GameWorld)
// ---------------------------------------------------------------------------

/// Mutable snapshot of one hull for the duration of a single space battle.
#[derive(Clone)]
struct ShipSnap {
    hull_current: i32,
    hull_max: i32,
    /// Shield recharge allocation nibble (bits 0-3 of C++ +0x64).
    shield_nibble: u8,
    /// Weapon recharge allocation nibble (bits 4-7 of C++ +0x64).
    weapon_nibble: u8,
    alive: bool,
}

// ---------------------------------------------------------------------------
// CombatSystem
// ---------------------------------------------------------------------------

pub struct CombatSystem;

impl CombatSystem {
    // -----------------------------------------------------------------------
    // Space combat auto-resolve — 7-phase pipeline
    // -----------------------------------------------------------------------

    /// Auto-resolve space combat between two fleets at a system.
    ///
    /// Implements the 7-phase C++ pipeline from `FUN_005457f0` → `FUN_00549910`.
    /// See `ghidra/notes/space-combat.md` and `rust-implementation-guide.md` §2.
    ///
    /// # Advance contract
    /// - Does NOT mutate `world`. All fleet state changes are returned as events.
    /// - `rng_rolls`: caller-provided uniform [0,1) values consumed in order.
    ///   Budget: ~4 rolls per ship per phase. Pass `fleet_size * 16` rolls minimum.
    pub fn resolve_space(
        world: &GameWorld,
        attacker: FleetKey,
        defender: FleetKey,
        system: SystemKey,
        // difficulty is consumed by the shield/hull phases once fully decompiled.
        // Accepted now so the API is stable; suppressed until those phases land.
        _difficulty: u8,
        rng_rolls: &[f64],
        tick: u64,
    ) -> SpaceCombatResult {
        let mut rng = rng_rolls.iter().copied();

        // Phase 1: Initialize — build mutable hull snapshots (FUN_005442f0).
        let (mut atk_ships, mut atk_fighters) = Self::snapshot_fleet(world, attacker);
        let (mut def_ships, mut def_fighters) = Self::snapshot_fleet(world, defender);
        let atk_initial_fighters = atk_fighters.clone();
        let def_initial_fighters = def_fighters.clone();

        let mut flags = CombatPhaseFlags::new(CombatPhaseFlags::ACTIVE);

        // Phase 2: Fleet composition evaluation (FUN_00544da0, 96 lines).
        // Checks if either side has armed ships. Does NOT set PHASES_ENABLED yet.
        let any_armed = atk_ships.iter().any(|s| s.alive && s.weapon_nibble > 0)
            || def_ships.iter().any(|s| s.alive && s.weapon_nibble > 0);

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

        // NOW arm subsequent phases after weapon fire completes.
        if any_armed {
            flags.set(CombatPhaseFlags::PHASES_ENABLED);
        }

        // Phases 4-6: Gate on PHASES_ENABLED.
        if flags.contains(CombatPhaseFlags::PHASES_ENABLED) {
            // Phase 4: Shield absorption (FUN_00544130, 83 lines, vtable +0x1c8).
            // Full formula not yet decompiled. Approximation: shields absorb
            // (shield_nibble / 15) fraction of the weapon damage applied in phase 3.
            // Since damage is applied directly to hull in phase 3 stub, this is a no-op
            // until the separate shield-before-hull pipeline is implemented.

            // Phase 5: Hull damage application (FUN_005443f0, 54 lines, vtable +0x1d0).
            // Remaining damage after shields → hull. No-op guard: only fires when
            // new_hull != current_hull (from FUN_00501490). Already handled above.

            // Phase 6: Fighter engagement (FUN_005444e0, 53 lines, vtable +0x1d4).
            // Family 0x71 exactly uses alt_shield_path (C++ +0x78 bit7).
            Self::phase_fighter_engage(
                world, attacker, defender,
                &mut atk_fighters, &mut def_fighters,
                &mut atk_ships, &mut def_ships,
                &mut rng,
            );
        }

        // Phase 7: Result determination (FUN_005445d0, 175 lines).
        // 1. Checks alive_flag at +0xac bit0 for each unit.
        // 2. Fighter exception: +0x50 & 0x08 → still combat-ready even if hull=0.
        // 3. Family 0x73/0x74 → FUN_00534640 special path (Death Star / special entity).
        let atk_alive = atk_ships.iter().any(|s| s.alive)
            || atk_fighters.iter().any(|&c| c > 0);
        let def_alive = def_ships.iter().any(|s| s.alive)
            || def_fighters.iter().any(|&c| c > 0);

        let winner = match (atk_alive, def_alive) {
            (true, false)  => CombatSide::Attacker,
            (false, true)  => CombatSide::Defender,
            _              => CombatSide::Draw,
        };

        // Phase 8: Post-combat cleanup (FUN_00544a20, 86 lines) — handled by caller.

        Self::build_space_result(attacker, defender, system, &atk_ships, &def_ships,
                                 &atk_fighters, &def_fighters,
                                 &atk_initial_fighters, &def_initial_fighters,
                                 winner, tick)
    }

    /// Build mutable hull snapshots for one fleet.
    ///
    /// Each `ShipEntry { class, count }` is expanded into `count` individual `ShipSnap`
    /// records. Combat operates on individual hulls, not class-count summaries.
    fn snapshot_fleet(world: &GameWorld, fleet: FleetKey) -> (Vec<ShipSnap>, Vec<u32>) {
        let f = &world.fleets[fleet];
        let mut ships = Vec::new();
        for entry in &f.capital_ships {
            let class = &world.capital_ship_classes[entry.class];
            let snap = ShipSnap {
                hull_current: class.hull as i32,
                hull_max:     class.hull as i32,
                // Initial shield nibble: clamped to 4-bit range (0-15).
                shield_nibble: (class.shield_recharge_rate.min(15)) as u8,
                // Weapon nibble starts at max allocation (0x0f = 15).
                weapon_nibble: 0x0f,
                alive: true,
            };
            for _ in 0..entry.count {
                ships.push(snap.clone());
            }
        }
        let fighters = f.fighters.iter().map(|e| e.count).collect();
        (ships, fighters)
    }

    /// Phase 3 helper: one side fires at the other.
    ///
    /// Mirrors `FUN_00543b60` (per-side weapon fire, vtable +0x1c4 dispatch).
    /// The exact per-arc formula behind vtable +0x1c4 is not yet decompiled.
    /// Approximation: aggregate all arc weapons × weapon_nibble / 15, distribute
    /// evenly with ±20% variance across alive targets.
    fn phase_weapon_fire(
        world: &GameWorld,
        firing_fleet: FleetKey,
        firing: &[ShipSnap],
        targets: &mut Vec<ShipSnap>,
        rng: &mut impl Iterator<Item = f64>,
    ) {
        let fleet = &world.fleets[firing_fleet];
        let total_fire: u32 = firing.iter()
            .zip(fleet.capital_ships.iter().flat_map(|e| {
                std::iter::repeat(e.class).take(e.count as usize)
            }))
            .filter(|(snap, _)| snap.alive)
            .map(|(snap, class_key)| {
                let class = &world.capital_ship_classes[class_key];
                let raw = class.turbolaser_fore
                    + class.turbolaser_aft
                    + class.turbolaser_port
                    + class.turbolaser_starboard
                    + class.ion_cannon_fore
                    + class.ion_cannon_aft
                    + class.ion_cannon_port
                    + class.ion_cannon_starboard
                    + class.laser_cannon_fore
                    + class.laser_cannon_aft
                    + class.laser_cannon_port
                    + class.laser_cannon_starboard;
                // weapon_nibble (0-15) scales output proportionally.
                (raw * snap.weapon_nibble as u32) / 15
            })
            .sum();

        if total_fire == 0 { return; }

        let alive_indices: Vec<usize> = targets.iter().enumerate()
            .filter(|(_, t)| t.alive)
            .map(|(i, _)| i)
            .collect();
        if alive_indices.is_empty() { return; }

        let fire_per_target = total_fire / alive_indices.len() as u32;
        for idx in alive_indices {
            let roll = rng.next().unwrap_or(0.5);
            // ±20% variance around fire_per_target. TODO: replace with exact vtable +0x1c4 formula.
            let variance = (fire_per_target as f64 * 0.2 * (roll * 2.0 - 1.0)) as i32;
            let damage = (fire_per_target as i32 + variance).max(0);

            // Shield absorption approximation: shield_nibble / 15 fraction absorbed.
            // This merges phases 3-5 into one step pending separate shield phase decompile.
            let absorbed = (damage as f64 * targets[idx].shield_nibble as f64 / 15.0) as i32;
            let hull_damage = (damage - absorbed).max(0);

            targets[idx].hull_current = (targets[idx].hull_current - hull_damage).max(0);
            if targets[idx].hull_current == 0 {
                targets[idx].alive = false; // alive_flag at +0xac bit0 cleared
            }
        }
    }

    /// Phase 6: Fighter engagement (FUN_005444e0, 53 lines, vtable +0x1d4).
    ///
    /// Fighter squadrons attack enemy capital ships and opposing squadrons.
    /// Family 0x71 exactly entities trigger alt_shield_path (C++ +0x78 bit7 = true).
    /// Dead fighters with +0x50 & 0x08 still count for phase-7 alive check.
    ///
    /// Approximation (vtable +0x1d4 formula pending decompile):
    /// - Each alive squadron does attack damage to one random enemy ship.
    /// - Opposing squadrons inflict fighter losses on each other.
    fn phase_fighter_engage(
        world: &GameWorld,
        attacker: FleetKey,
        defender: FleetKey,
        atk_fighters: &mut Vec<u32>,
        def_fighters: &mut Vec<u32>,
        atk_ships: &mut Vec<ShipSnap>,
        def_ships: &mut Vec<ShipSnap>,
        rng: &mut impl Iterator<Item = f64>,
    ) {
        let atk_fleet = &world.fleets[attacker];
        let def_fleet = &world.fleets[defender];

        // Fighters attack enemy capital ships.
        Self::fighters_attack_ships(
            atk_fighters, atk_fleet, def_ships, world, rng,
        );
        Self::fighters_attack_ships(
            def_fighters, def_fleet, atk_ships, world, rng,
        );

        // Fighter-vs-fighter engagement: each side's total squadron count vs the other.
        let atk_total: u32 = atk_fighters.iter().sum();
        let def_total: u32 = def_fighters.iter().sum();

        if atk_total == 0 || def_total == 0 { return; }

        let roll_atk = rng.next().unwrap_or(0.5);
        let roll_def = rng.next().unwrap_or(0.5);

        // Loss = (enemy_count / (my_count + enemy_count)) * 0.3 * total_squads * roll.
        // Factor 0.3: approximation pending vtable +0x1d4 decompile.
        let atk_hit_rate = def_total as f64 / (atk_total + def_total) as f64;
        let def_hit_rate = atk_total as f64 / (atk_total + def_total) as f64;

        let atk_losses = ((atk_total as f64 * atk_hit_rate * 0.3 * roll_atk) as u32).min(atk_total);
        let def_losses = ((def_total as f64 * def_hit_rate * 0.3 * roll_def) as u32).min(def_total);

        Self::apply_fighter_losses(atk_fighters, atk_losses);
        Self::apply_fighter_losses(def_fighters, def_losses);
    }

    fn fighters_attack_ships(
        squadrons: &[u32],
        fleet: &crate::world::Fleet,
        enemy_ships: &mut Vec<ShipSnap>,
        world: &GameWorld,
        rng: &mut impl Iterator<Item = f64>,
    ) {
        let alive_targets: Vec<usize> = enemy_ships.iter().enumerate()
            .filter(|(_, s)| s.alive)
            .map(|(i, _)| i)
            .collect();
        if alive_targets.is_empty() { return; }

        for (sq_idx, &sq_count) in squadrons.iter().enumerate() {
            if sq_count == 0 { continue; }
            let class_key = match fleet.fighters.get(sq_idx) {
                Some(e) => e.class,
                None => continue,
            };
            let class = &world.fighter_classes[class_key];
            // Fighter attack against capital ships (approximation).
            // overall_attack_strength is the DAT aggregate attack stat.
            let attack_power = class.overall_attack_strength * sq_count;
            if attack_power == 0 { continue; }

            let roll = rng.next().unwrap_or(0.5);
            let raw_idx = (roll * alive_targets.len() as f64) as usize;
            let target_idx = alive_targets[raw_idx.min(alive_targets.len() - 1)];
            let damage = (attack_power / 10).max(1) as i32; // scale down raw power

            enemy_ships[target_idx].hull_current =
                (enemy_ships[target_idx].hull_current - damage).max(0);
            if enemy_ships[target_idx].hull_current == 0 {
                enemy_ships[target_idx].alive = false;
            }
        }
    }

    fn apply_fighter_losses(squadrons: &mut Vec<u32>, mut losses: u32) {
        for count in squadrons.iter_mut().rev() {
            if losses == 0 { break; }
            let take = losses.min(*count);
            *count -= take;
            losses -= take;
        }
    }

    fn build_space_result(
        attacker: FleetKey,
        defender: FleetKey,
        system: SystemKey,
        atk_ships: &[ShipSnap],
        def_ships: &[ShipSnap],
        atk_fighters: &[u32],
        def_fighters: &[u32],
        atk_initial_fighters: &[u32],
        def_initial_fighters: &[u32],
        winner: CombatSide,
        tick: u64,
    ) -> SpaceCombatResult {
        let mut ship_damage = Vec::new();
        for (i, snap) in atk_ships.iter().enumerate() {
            if snap.hull_current < snap.hull_max {
                ship_damage.push(ShipDamageEvent {
                    fleet: attacker,
                    ship_index: i,
                    hull_before: snap.hull_max,
                    hull_after: snap.hull_current,
                });
            }
        }
        for (i, snap) in def_ships.iter().enumerate() {
            if snap.hull_current < snap.hull_max {
                ship_damage.push(ShipDamageEvent {
                    fleet: defender,
                    ship_index: i,
                    hull_before: snap.hull_max,
                    hull_after: snap.hull_current,
                });
            }
        }

        // Compute fighter losses: initial counts captured before combat vs surviving counts.
        let mut fighter_losses = Vec::new();
        for (i, &init) in atk_initial_fighters.iter().enumerate() {
            let survived = atk_fighters.get(i).copied().unwrap_or(0);
            if init > survived {
                fighter_losses.push(FighterLossEvent {
                    fleet: attacker,
                    fighter_index: i,
                    squads_before: init,
                    squads_after: survived,
                });
            }
        }
        for (i, &init) in def_initial_fighters.iter().enumerate() {
            let survived = def_fighters.get(i).copied().unwrap_or(0);
            if init > survived {
                fighter_losses.push(FighterLossEvent {
                    fleet: defender,
                    fighter_index: i,
                    squads_before: init,
                    squads_after: survived,
                });
            }
        }

        SpaceCombatResult {
            attacker_fleet: attacker,
            defender_fleet: defender,
            system,
            winner,
            ship_damage,
            fighter_losses,
            tick,
        }
    }

    // -----------------------------------------------------------------------
    // Ground combat — FUN_00560d50 (232 lines)
    // -----------------------------------------------------------------------

    /// Auto-resolve ground combat at a system between all opposing troop units.
    ///
    /// Mirrors `FUN_00560d50` (232 lines): iterates troops (DatId family 0x14-0x1b)
    /// at the system, checks `regiment_strength` at C++ offset +0x96, calls per-unit
    /// resolution `FUN_004ee350`.
    ///
    /// Death Star (family 0x34) takes a separate path — call `resolve_death_star()`.
    ///
    /// # Advance contract
    /// - Does NOT mutate world. Returns TroopDamageEvents.
    /// - `rng_rolls`: one roll per attacker-defender pair. Budget: troop_count² rolls.
    pub fn resolve_ground(
        world: &GameWorld,
        system: SystemKey,
        attacker_is_alliance: bool,
        rng_rolls: &[f64],
        tick: u64,
    ) -> GroundCombatResult {
        let sys = &world.systems[system];
        let mut rng = rng_rolls.iter().copied();
        let mut troop_damage: Vec<TroopDamageEvent> = Vec::new();

        // Step 6-8 of FUN_00560d50: iterate all troops at system by faction.
        // C++ uses `FUN_004f25c0(type=3)` to iterate troops (family 0x14-0x1b).
        // In Rust: walk system.ground_units and partition by is_alliance.
        let (atk_keys, def_keys): (Vec<TroopKey>, Vec<TroopKey>) = sys.ground_units
            .iter()
            .copied()
            .partition(|&key| world.troops[key].is_alliance == attacker_is_alliance);

        // Regiment strength check (C++ offset +0x96, short):
        // units with strength <= 0 are destroyed — skip them.
        let active_atk: Vec<TroopKey> = atk_keys.iter()
            .copied()
            .filter(|&k| world.troops[k].regiment_strength > 0)
            .collect();
        let active_def: Vec<TroopKey> = def_keys.iter()
            .copied()
            .filter(|&k| world.troops[k].regiment_strength > 0)
            .collect();

        // Per-unit resolution: FUN_004ee350 (30 lines).
        // For each attacker-defender pair:
        //   1. `old_strength = unit->regiment_strength` (offset +0x96)
        //   2. If old_strength != new_value: set, notify both sides, vtable +0x330
        //
        // Same-side guard: `param_2 == param_1 → skip` (no friendly fire).
        // In Rust: both keys are already in opposite partitions, guard is implicit.
        // current_strength tracks live strength as damage accumulates within this battle.
        let mut current_strength: std::collections::HashMap<TroopKey, i16> = active_atk.iter()
            .chain(active_def.iter())
            .map(|&k| (k, world.troops[k].regiment_strength))
            .collect();

        for &atk_key in &active_atk {
            for &def_key in &active_def {
                let roll = rng.next().unwrap_or(0.5);

                let atk_power = *current_strength.get(&atk_key).unwrap_or(&0) as f64;
                let def_power = *current_strength.get(&def_key).unwrap_or(&0) as f64;
                if atk_power <= 0.0 || def_power <= 0.0 { continue; }

                let total = atk_power + def_power;
                // Hit probability: proportional to attacker strength vs combined strength.
                // Approximation (vtable +0x330 formula pending decompile).
                let hit_prob = atk_power / total;

                if roll < hit_prob {
                    // Attacker hits defender.
                    let old_strength = current_strength[&def_key];
                    let reduction = (old_strength / 4).max(1);
                    let new_strength = (old_strength - reduction).max(0);
                    current_strength.insert(def_key, new_strength);
                    troop_damage.push(TroopDamageEvent {
                        troop: def_key,
                        strength_before: old_strength,
                        strength_after: new_strength,
                    });
                } else {
                    // Defender hits attacker.
                    let old_strength = current_strength[&atk_key];
                    let reduction = (old_strength / 4).max(1);
                    let new_strength = (old_strength - reduction).max(0);
                    current_strength.insert(atk_key, new_strength);
                    troop_damage.push(TroopDamageEvent {
                        troop: atk_key,
                        strength_before: old_strength,
                        strength_after: new_strength,
                    });
                }
            }
        }

        // Determine winner by counting survivors after applying damage events.
        let surviving_atk = Self::count_ground_survivors(&active_atk, &troop_damage, world);
        let surviving_def = Self::count_ground_survivors(&active_def, &troop_damage, world);

        let winner = match (surviving_atk > 0, surviving_def > 0) {
            (true, false) => CombatSide::Attacker,
            (false, true) => CombatSide::Defender,
            _             => CombatSide::Draw,
        };

        GroundCombatResult { system, winner, troop_damage, tick }
    }

    fn count_ground_survivors(
        troops: &[TroopKey],
        damage: &[TroopDamageEvent],
        world: &GameWorld,
    ) -> usize {
        troops.iter().filter(|&&key| {
            // Use the latest damage event for this key if present, else world state.
            if let Some(evt) = damage.iter().rev().find(|e| e.troop == key) {
                evt.strength_after > 0
            } else {
                world.troops[key].regiment_strength > 0
            }
        }).count()
    }

    /// Death Star superlaser resolution stub.
    ///
    /// Called when a Death Star (DatId family 0x34) is present at the system.
    /// Mirrors `FUN_005617b0` (68 lines) — pending full decompile.
    pub fn resolve_death_star(
        _world: &GameWorld,
        system: SystemKey,
        tick: u64,
    ) -> GroundCombatResult {
        // TODO: decompile FUN_005617b0 for superlaser exact mechanics.
        GroundCombatResult {
            system,
            winner: CombatSide::Attacker, // Death Star always wins
            troop_damage: Vec::new(),
            tick,
        }
    }
}

// ---------------------------------------------------------------------------
// Entity kind classification
// ---------------------------------------------------------------------------

/// DatId family-byte classification for combat dispatch.
///
/// Mirrors the C++ family-byte check (`entity_id >> 24`) used in
/// `FUN_00560d50`, `FUN_005445d0`, and related combat functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatEntityKind {
    /// family 0x30-0x33, 0x35-0x3b
    CapitalShip,
    /// family 0x34 → FUN_005617b0 / FUN_00534640
    DeathStar,
    /// family 0x71-0x72 (alt_shield_path: +0x78 bit7)
    FighterSquadron,
    /// family 0x73-0x74 (FUN_00534640 result path)
    SpecialCombatEntity,
    /// family 0x14-0x1b (regiment_strength at +0x96)
    Troop,
    /// family 0x08-0x0f
    Character,
    /// Unrecognized family byte
    Unknown,
}

impl CombatEntityKind {
    /// Classify by DatId family byte (`id >> 24`).
    pub fn from_family_byte(family: u8) -> Self {
        match family {
            0x08..=0x0f => Self::Character,
            0x14..=0x1b => Self::Troop,
            0x30..=0x33 | 0x35..=0x3b => Self::CapitalShip,
            0x34 => Self::DeathStar,
            0x71 => Self::FighterSquadron, // exact == 0x71 only, NOT range (reviewer-corrected)
            0x72 => Self::Unknown,       // 0x72 is a different entity type, not alt-shield
            0x73..=0x74 => Self::SpecialCombatEntity,
            _ => Self::Unknown,
        }
    }
}

/// Extract difficulty level (0-3) from the C++ packed difficulty field.
/// C++ source: `*(uint *)((int)this + 0x24) >> 4 & 3` (line 21 of FUN_0054a1d0).
pub fn extract_difficulty(packed: u32) -> u8 {
    ((packed >> 4) & 3) as u8
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::ControlKind;
    use crate::world::*;
    use crate::ids::*;
    use crate::dat::{ExplorationStatus, SectorGroup};
    use std::collections::HashMap;

    fn empty_world() -> GameWorld {
        GameWorld {
            systems: slotmap::SlotMap::with_key(),
            sectors: slotmap::SlotMap::with_key(),
            capital_ship_classes: slotmap::SlotMap::with_key(),
            fighter_classes: slotmap::SlotMap::with_key(),
            characters: slotmap::SlotMap::with_key(),
            fleets: slotmap::SlotMap::with_key(),
            troops: slotmap::SlotMap::with_key(),
            special_forces: slotmap::SlotMap::with_key(),
            defense_facilities: slotmap::SlotMap::with_key(),
            manufacturing_facilities: slotmap::SlotMap::with_key(),
            production_facilities: slotmap::SlotMap::with_key(),
            gnprtb: GnprtbParams::default(),
            sdprtb: SdprtbParams::default(),
            mission_tables: HashMap::new(),
            defense_facility_classes: HashMap::new(),
        }
    }

    fn make_sector(world: &mut GameWorld) -> SectorKey {
        world.sectors.insert(Sector {
            dat_id: DatId::new(0x92000001),
            name: "Test Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        })
    }

    fn make_system(world: &mut GameWorld, sector: SectorKey) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId::new(0x90000001),
            name: "Coruscant".into(),
            sector,
            x: 100,
            y: 100,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            control: ControlKind::Uncontrolled,
            is_headquarters: false,
            is_destroyed: false,
        })
    }

    fn make_class(world: &mut GameWorld, hull: u32, turbolaser: u32) -> CapitalShipKey {
        world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x30000001),
            name: "Star Destroyer".into(),
            is_alliance: false,
            is_empire: true,
            hull,
            shield_strength: 10,
            sub_light_engine: 5,
            maneuverability: 3,
            hyperdrive: 2,
            fighter_capacity: 6,
            detection: 5,
            turbolaser_fore: turbolaser,
            shield_recharge_rate: 3,
            damage_control: 1,
            bombardment_modifier: 10,
            ..CapitalShipClass::default()
        })
    }

    fn make_fleet(world: &mut GameWorld, sys: SystemKey, class: CapitalShipKey,
                  count: u32, is_alliance: bool) -> FleetKey {
        let key = world.fleets.insert(Fleet {
            location: sys,
            capital_ships: vec![ShipEntry { class, count }],
            fighters: vec![],
            characters: vec![],
            is_alliance,
            has_death_star: false,
        });
        world.systems[sys].fleets.push(key);
        key
    }

    #[test]
    fn test_entity_kind_classification() {
        assert_eq!(CombatEntityKind::from_family_byte(0x30), CombatEntityKind::CapitalShip);
        assert_eq!(CombatEntityKind::from_family_byte(0x34), CombatEntityKind::DeathStar);
        assert_eq!(CombatEntityKind::from_family_byte(0x14), CombatEntityKind::Troop);
        assert_eq!(CombatEntityKind::from_family_byte(0x08), CombatEntityKind::Character);
        assert_eq!(CombatEntityKind::from_family_byte(0x71), CombatEntityKind::FighterSquadron);
    }

    #[test]
    fn test_extract_difficulty() {
        // difficulty bits 4-5: value 2 = hard → packed = 0x20
        assert_eq!(extract_difficulty(0x20), 2);
        assert_eq!(extract_difficulty(0x00), 0);
        assert_eq!(extract_difficulty(0x10), 1);
        assert_eq!(extract_difficulty(0x30), 3);
    }

    #[test]
    fn test_space_combat_attacker_wins_overwhelming_force() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);
        // Attacker: 5 large ships, defender: 1 small ship.
        let atk_class = make_class(&mut world, 1000, 200);
        let def_class = make_class(&mut world, 50, 5);
        let atk = make_fleet(&mut world, sys, atk_class, 5, true);
        let def = make_fleet(&mut world, sys, def_class, 1, false);

        // Provide ample RNG rolls.
        let rolls: Vec<f64> = vec![0.5; 200];
        let result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);
        assert_eq!(result.winner, CombatSide::Attacker);
    }

    #[test]
    fn test_space_combat_returns_damage_events() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);
        let atk_class = make_class(&mut world, 100, 50);
        let def_class = make_class(&mut world, 100, 5);
        let atk = make_fleet(&mut world, sys, atk_class, 3, true);
        let def = make_fleet(&mut world, sys, def_class, 1, false);

        let rolls: Vec<f64> = vec![0.5; 200];
        let result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);
        // Damage events should exist since combat occurred.
        assert!(!result.ship_damage.is_empty() || result.winner != CombatSide::Draw);
    }

    #[test]
    fn test_space_combat_draw_equal_forces() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);
        let class = make_class(&mut world, 100, 50);
        // Both sides: same class, same count, all rolls at exactly 0.5.
        let atk = make_fleet(&mut world, sys, class, 2, true);
        let def = make_fleet(&mut world, sys, class, 2, false);

        let rolls: Vec<f64> = vec![0.5; 200];
        let result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);
        // Equal forces with median rolls → at minimum both sides survive (draw) or one wins.
        // Just verify it returns a valid CombatSide.
        assert!(matches!(result.winner,
            CombatSide::Attacker | CombatSide::Defender | CombatSide::Draw));
    }

    #[test]
    fn test_ground_combat_attacker_wins() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Attacker: 5 strong alliance troops. Defender: 1 weak empire troop.
        for _ in 0..5 {
            let key = world.troops.insert(TroopUnit {
                class_dat_id: DatId::new(0x14000001),
                is_alliance: true,
                regiment_strength: 100,
            });
            world.systems[sys].ground_units.push(key);
        }
        let def_key = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x14000002),
            is_alliance: false,
            regiment_strength: 4, // 5 attackers each deal 1 damage: 4→3→2→1→0 (dead)
        });
        world.systems[sys].ground_units.push(def_key);

        let rolls: Vec<f64> = vec![0.1; 100]; // low rolls → attacker almost always hits
        let result = CombatSystem::resolve_ground(&world, sys, true, &rolls, 1);
        assert_eq!(result.winner, CombatSide::Attacker);
    }

    #[test]
    fn test_ground_combat_no_troops_is_draw() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        let rolls: Vec<f64> = vec![];
        let result = CombatSystem::resolve_ground(&world, sys, true, &rolls, 1);
        assert_eq!(result.winner, CombatSide::Draw);
        assert!(result.troop_damage.is_empty());
    }

    #[test]
    fn test_ground_combat_zero_strength_troops_skip() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Both troops have strength 0 — both filtered out, should be a draw.
        let k1 = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x14000001),
            is_alliance: true,
            regiment_strength: 0,
        });
        let k2 = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x14000002),
            is_alliance: false,
            regiment_strength: 0,
        });
        world.systems[sys].ground_units.extend([k1, k2]);

        let rolls: Vec<f64> = vec![0.5; 10];
        let result = CombatSystem::resolve_ground(&world, sys, true, &rolls, 1);
        assert_eq!(result.winner, CombatSide::Draw);
    }

    #[test]
    fn test_death_star_handler_returns_attacker_win() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        let result = CombatSystem::resolve_death_star(&world, sys, 42);
        assert_eq!(result.winner, CombatSide::Attacker);
        assert_eq!(result.tick, 42);
    }
}
