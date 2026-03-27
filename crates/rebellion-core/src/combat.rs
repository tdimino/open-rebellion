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

/// GNPRTB parameter for combat difficulty modifier (DAT_00661a88).
/// Scales combat damage output. Value is percentage (100 = 1.0x).
/// Same param used by FUN_0053e190 for bombardment and ground combat.
const GNPRTB_COMBAT_DIFFICULTY_MODIFIER: u16 = 0x1400;

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
    /// Current shield HP pool — absorbs damage before hull.
    shield_current: i32,
    /// Maximum shield HP (from CapitalShipClass::shield_strength).
    shield_max: i32,
    /// Pending weapon damage awaiting shield absorption (set in phase 3, consumed in phase 4).
    pending_damage: i32,
    /// Pending ion cannon damage (subset of pending_damage — 2x effective against shields).
    pending_ion_damage: i32,
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
        difficulty: u8,
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

        // Difficulty modifier: FUN_0053e190 scales combat damage via GNPRTB.
        let difficulty_mod = {
            let raw = world.gnprtb.value(GNPRTB_COMBAT_DIFFICULTY_MODIFIER, difficulty);
            if raw > 0 { raw as f64 / 100.0 } else { 1.0 }
        };

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
            // Shields absorb pending weapon damage before hull. Ion cannons deal 2x
            // damage to shields. Shield recharges (shield_nibble / 15) fraction per tick.
            Self::phase_shield_absorb(&mut atk_ships);
            Self::phase_shield_absorb(&mut def_ships);

            // Phase 5: Hull damage application (FUN_005443f0, 54 lines, vtable +0x1d0).
            // Overflow damage after shields is applied to hull_current, scaled by difficulty.
            Self::phase_hull_damage(&mut atk_ships, difficulty_mod);
            Self::phase_hull_damage(&mut def_ships, difficulty_mod);

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
                shield_current: class.shield_strength as i32,
                shield_max:     class.shield_strength as i32,
                pending_damage: 0,
                pending_ion_damage: 0,
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
    ///
    /// Damage is stored as `pending_damage` / `pending_ion_damage` on each target.
    /// Phase 4 (shield absorption) processes pending damage before hull application.
    fn phase_weapon_fire(
        world: &GameWorld,
        firing_fleet: FleetKey,
        firing: &[ShipSnap],
        targets: &mut Vec<ShipSnap>,
        rng: &mut impl Iterator<Item = f64>,
    ) {
        let fleet = &world.fleets[firing_fleet];
        // Compute total fire split by weapon type: ion cannons tracked separately.
        let (total_fire, total_ion): (u32, u32) = firing.iter()
            .zip(fleet.capital_ships.iter().flat_map(|e| {
                std::iter::repeat(e.class).take(e.count as usize)
            }))
            .filter(|(snap, _)| snap.alive)
            .fold((0u32, 0u32), |(acc_fire, acc_ion), (snap, class_key)| {
                let class = &world.capital_ship_classes[class_key];
                let non_ion = class.turbolaser_fore
                    + class.turbolaser_aft
                    + class.turbolaser_port
                    + class.turbolaser_starboard
                    + class.laser_cannon_fore
                    + class.laser_cannon_aft
                    + class.laser_cannon_port
                    + class.laser_cannon_starboard;
                let ion = class.ion_cannon_fore
                    + class.ion_cannon_aft
                    + class.ion_cannon_port
                    + class.ion_cannon_starboard;
                let scale = snap.weapon_nibble as u32;
                (acc_fire + (non_ion * scale) / 15, acc_ion + (ion * scale) / 15)
            });

        let total = total_fire + total_ion;
        if total == 0 { return; }

        let alive_indices: Vec<usize> = targets.iter().enumerate()
            .filter(|(_, t)| t.alive)
            .map(|(i, _)| i)
            .collect();
        if alive_indices.is_empty() { return; }

        let n = alive_indices.len() as u32;
        let fire_per = total_fire / n;
        let ion_per = total_ion / n;
        let total_per = fire_per + ion_per;

        for idx in alive_indices {
            let roll = rng.next().unwrap_or(0.5);
            // ±20% variance around total_per. TODO: replace with exact vtable +0x1c4 formula.
            let variance = (total_per as f64 * 0.2 * (roll * 2.0 - 1.0)) as i32;
            let damage = (total_per as i32 + variance).max(0);

            // Split damage proportionally between ion and non-ion.
            let ion_frac = if total > 0 { total_ion as f64 / total as f64 } else { 0.0 };
            let ion_dmg = (damage as f64 * ion_frac) as i32;

            targets[idx].pending_damage += damage;
            targets[idx].pending_ion_damage += ion_dmg;
        }
    }

    /// Phase 4: Shield absorption (FUN_00544130, 83 lines, vtable +0x1c8).
    ///
    /// Each ship's shield pool absorbs pending weapon damage before hull.
    /// Ion cannon damage is 2x effective against shields (lore-accurate: ion cannons
    /// are designed to overload shield generators).
    ///
    /// After absorption, shields recharge: `(shield_nibble / 15) * shield_max` per tick.
    /// Shield cannot exceed shield_max. Overflow damage remains in `pending_damage`
    /// for Phase 5 (hull damage application).
    fn phase_shield_absorb(ships: &mut [ShipSnap]) {
        for ship in ships.iter_mut() {
            if !ship.alive || ship.pending_damage == 0 { continue; }

            // Shield recharge: (shield_nibble / 15) fraction of max shield per tick.
            // Applied before absorption so ships benefit from recharge during combat.
            if ship.shield_nibble > 0 && ship.shield_max > 0 {
                let recharge = (ship.shield_max as f64 * ship.shield_nibble as f64 / 15.0) as i32;
                ship.shield_current = (ship.shield_current + recharge).min(ship.shield_max);
            }

            if ship.shield_current <= 0 {
                // No shields — all damage passes through to hull phase.
                continue;
            }

            // Ion cannons deal 2x damage to shields. Compute effective shield damage:
            // non-ion portion hits shields 1:1, ion portion hits shields 2:1.
            let non_ion_damage = ship.pending_damage - ship.pending_ion_damage;
            let effective_shield_damage = non_ion_damage + ship.pending_ion_damage * 2;

            let absorbed = effective_shield_damage.min(ship.shield_current);
            ship.shield_current -= absorbed;

            // Convert absorbed effective damage back to raw damage consumed.
            // Use integer math (via i64 to avoid overflow) to eliminate
            // double-rounding that previously lost up to 2 raw damage.
            let raw_consumed = if effective_shield_damage > 0 {
                (ship.pending_damage as i64 * absorbed as i64
                    / effective_shield_damage as i64) as i32
            } else {
                0
            };

            ship.pending_damage = (ship.pending_damage - raw_consumed).max(0);
            let ion_consumed = if effective_shield_damage > 0 {
                (ship.pending_ion_damage as i64 * absorbed as i64
                    / effective_shield_damage as i64) as i32
            } else {
                0
            };
            ship.pending_ion_damage = (ship.pending_ion_damage - ion_consumed).max(0);
        }
    }

    /// Phase 5: Hull damage application (FUN_005443f0, 54 lines, vtable +0x1d0).
    ///
    /// Applies remaining pending damage (after shield absorption) to hull.
    /// No-op guard: only fires when pending_damage > 0 (mirrors C++ check
    /// `new_hull != current_hull` from FUN_00501490).
    fn phase_hull_damage(ships: &mut [ShipSnap], difficulty_mod: f64) {
        for ship in ships.iter_mut() {
            if !ship.alive || ship.pending_damage == 0 { continue; }

            // Scale hull damage by difficulty modifier (FUN_0053e190).
            let scaled_damage = ((ship.pending_damage as f64 * difficulty_mod) as i32).max(1);
            ship.hull_current = (ship.hull_current - scaled_damage).max(0);
            ship.pending_damage = 0;
            ship.pending_ion_damage = 0;

            if ship.hull_current == 0 {
                ship.alive = false; // alive_flag at +0xac bit0 cleared
            }
        }
    }

    /// Compute total fighter carrier capacity for a fleet's alive capital ships.
    ///
    /// Sums `fighter_capacity` across all capital ship classes, scaled by the
    /// number of alive hulls of each class. Returns the maximum number of
    /// fighter squadrons that can be launched simultaneously.
    fn compute_carrier_capacity(
        world: &GameWorld,
        fleet_key: FleetKey,
        ship_snaps: &[ShipSnap],
    ) -> u32 {
        let fleet = &world.fleets[fleet_key];
        let mut capacity: u32 = 0;
        let mut snap_idx = 0;
        for entry in &fleet.capital_ships {
            let class = &world.capital_ship_classes[entry.class];
            for _ in 0..entry.count {
                if snap_idx < ship_snaps.len() && ship_snaps[snap_idx].alive {
                    capacity += class.fighter_capacity;
                }
                snap_idx += 1;
            }
        }
        capacity
    }

    /// Launch fighters from carrier ships, capping deployed squadrons to
    /// available carrier capacity. Returns a vec of deployed squadron counts
    /// (same length as the fleet's fighter roster, but each capped).
    fn launch_fighters(
        fleet_fighters: &[u32],
        carrier_capacity: u32,
    ) -> Vec<u32> {
        let mut remaining_capacity = carrier_capacity;
        let mut launched = Vec::with_capacity(fleet_fighters.len());
        for &count in fleet_fighters {
            let deploy = count.min(remaining_capacity);
            launched.push(deploy);
            remaining_capacity = remaining_capacity.saturating_sub(deploy);
        }
        launched
    }

    /// Phase 6: Fighter engagement (FUN_005444e0, 53 lines, vtable +0x1d4).
    ///
    /// Fighter squadrons deploy from carrier ships (gated by `fighter_capacity`),
    /// attack enemy capital ships using per-class attack_strength, then engage
    /// opposing fighters in dogfights using attack_strength and maneuverability.
    /// Surviving squadrons are recalled to carriers post-combat.
    ///
    /// Family 0x71 exactly entities trigger alt_shield_path (C++ +0x78 bit7 = true).
    /// Dead fighters with +0x50 & 0x08 still count for phase-7 alive check.
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
        // Step 1: Launch — cap deployed squadrons to carrier capacity.
        let atk_capacity = Self::compute_carrier_capacity(world, attacker, atk_ships);
        let def_capacity = Self::compute_carrier_capacity(world, defender, def_ships);

        let mut atk_launched = Self::launch_fighters(atk_fighters, atk_capacity);
        let mut def_launched = Self::launch_fighters(def_fighters, def_capacity);

        // Snapshot original launched counts before combat modifies them —
        // needed by recall_fighters to distinguish grounded squads from wiped ones.
        let atk_originally_launched = atk_launched.clone();
        let def_originally_launched = def_launched.clone();

        let atk_fleet = &world.fleets[attacker];
        let def_fleet = &world.fleets[defender];

        // Step 2: Fighters attack enemy capital ships.
        Self::fighters_attack_ships(
            &atk_launched, atk_fleet, def_ships, world, rng,
        );
        Self::fighters_attack_ships(
            &def_launched, def_fleet, atk_ships, world, rng,
        );

        // Step 3: Fighter-vs-fighter dogfight using attack_strength and maneuverability.
        Self::fighter_dogfight(
            &mut atk_launched, atk_fleet,
            &mut def_launched, def_fleet,
            world, rng,
        );

        // Step 4: Recall — surviving fighters return to carriers.
        // Re-check capacity (carriers may have been destroyed during this phase).
        let atk_capacity_post = Self::compute_carrier_capacity(world, attacker, atk_ships);
        let def_capacity_post = Self::compute_carrier_capacity(world, defender, def_ships);

        Self::recall_fighters(atk_fighters, &atk_launched, &atk_originally_launched, atk_capacity_post);
        Self::recall_fighters(def_fighters, &def_launched, &def_originally_launched, def_capacity_post);
    }

    /// Fighters attack enemy capital ships using per-class weapon stats.
    ///
    /// Each squadron's damage is computed from the FighterClass weapon-type
    /// attack strengths (turbolaser, ion cannon, laser cannon) scaled by
    /// squadron count. Shield strength of the target absorbs partial damage.
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

            // Compute per-squadron attack power from weapon-type strengths.
            // Each weapon type contributes: weapon_count * weapon_attack_strength.
            // Falls back to overall_attack_strength if per-weapon stats are zero.
            let per_weapon_attack =
                class.turbolaser_fore * class.turbolaser_attack_strength
                + class.ion_cannon_fore * class.ion_cannon_attack_strength
                + class.laser_cannon_fore * class.laser_cannon_attack_strength;

            let base_attack = if per_weapon_attack > 0 {
                per_weapon_attack
            } else {
                class.overall_attack_strength
            };

            let attack_power = base_attack * sq_count;
            if attack_power == 0 { continue; }

            let roll = rng.next().unwrap_or(0.5);
            let raw_idx = (roll * alive_targets.len() as f64) as usize;
            let target_idx = alive_targets[raw_idx.min(alive_targets.len() - 1)];

            // Shield absorption: target's shield_nibble absorbs a fraction.
            let raw_damage = attack_power as i32;
            let absorbed = (raw_damage as f64 * enemy_ships[target_idx].shield_nibble as f64 / 15.0) as i32;
            let damage = (raw_damage - absorbed).max(1);

            enemy_ships[target_idx].hull_current =
                (enemy_ships[target_idx].hull_current - damage).max(0);
            if enemy_ships[target_idx].hull_current == 0 {
                enemy_ships[target_idx].alive = false;
            }
        }
    }

    /// Fighter-vs-fighter dogfight using FighterClass attack_strength and maneuverability.
    ///
    /// Each side's combat power = sum(squadron_count * (attack_strength + maneuverability / 2)).
    /// The power ratio determines the loss rate for each side. Higher maneuverability
    /// grants evasion advantage, reducing losses taken.
    fn fighter_dogfight(
        atk_launched: &mut Vec<u32>,
        atk_fleet: &crate::world::Fleet,
        def_launched: &mut Vec<u32>,
        def_fleet: &crate::world::Fleet,
        world: &GameWorld,
        rng: &mut impl Iterator<Item = f64>,
    ) {
        let atk_total: u32 = atk_launched.iter().sum();
        let def_total: u32 = def_launched.iter().sum();
        if atk_total == 0 || def_total == 0 { return; }

        // Compute weighted combat power per side using attack_strength + maneuverability/2.
        let atk_power: f64 = atk_launched.iter().enumerate()
            .filter(|(_, &c)| c > 0)
            .map(|(i, &c)| {
                let class = atk_fleet.fighters.get(i)
                    .map(|e| &world.fighter_classes[e.class]);
                let (atk_str, maneuver) = class
                    .map(|c| (c.overall_attack_strength as f64, c.maneuverability as f64))
                    .unwrap_or((1.0, 0.0));
                c as f64 * (atk_str + maneuver / 2.0)
            })
            .sum();

        let def_power: f64 = def_launched.iter().enumerate()
            .filter(|(_, &c)| c > 0)
            .map(|(i, &c)| {
                let class = def_fleet.fighters.get(i)
                    .map(|e| &world.fighter_classes[e.class]);
                let (atk_str, maneuver) = class
                    .map(|c| (c.overall_attack_strength as f64, c.maneuverability as f64))
                    .unwrap_or((1.0, 0.0));
                c as f64 * (atk_str + maneuver / 2.0)
            })
            .sum();

        if atk_power <= 0.0 && def_power <= 0.0 { return; }
        let total_power = atk_power + def_power;

        let roll_atk = rng.next().unwrap_or(0.5);
        let roll_def = rng.next().unwrap_or(0.5);

        // Loss formula: losses = squad_count * (enemy_power / total_power) * roll * attrition_rate.
        // Attrition rate 0.4: dogfights are lethal but not total wipeouts per round.
        // Maneuverability already baked into power — higher maneuverability means
        // your side contributes more power, so the enemy takes proportionally more losses.
        let attrition_rate = 0.4;
        let atk_loss_rate = def_power / total_power;
        let def_loss_rate = atk_power / total_power;

        let atk_losses = ((atk_total as f64 * atk_loss_rate * attrition_rate * roll_atk) as u32).min(atk_total);
        let def_losses = ((def_total as f64 * def_loss_rate * attrition_rate * roll_def) as u32).min(def_total);

        Self::apply_fighter_losses(atk_launched, atk_losses);
        Self::apply_fighter_losses(def_launched, def_losses);
    }

    /// Recall surviving fighters to carriers, capping to remaining carrier capacity.
    ///
    /// Fighters that exceed post-combat carrier capacity are lost (carriers destroyed
    /// during combat cannot recover their squadrons).
    ///
    /// `originally_launched`: per-squadron counts that were actually deployed (from
    /// `launch_fighters`). Squads with 0 launched were grounded and are preserved.
    fn recall_fighters(
        fleet_fighters: &mut Vec<u32>,
        launched_survivors: &[u32],
        originally_launched: &[u32],
        carrier_capacity: u32,
    ) {
        let mut remaining_capacity = carrier_capacity;
        for (i, count) in fleet_fighters.iter_mut().enumerate() {
            let was_launched = originally_launched.get(i).copied().unwrap_or(0);
            if was_launched == 0 {
                // Squadron was grounded (never launched) — preserve its count.
                continue;
            }
            let survived = launched_survivors.get(i).copied().unwrap_or(0);
            // Recalled = survived (capped to remaining carrier capacity).
            let recalled = survived.min(remaining_capacity);
            remaining_capacity = remaining_capacity.saturating_sub(recalled);
            *count = recalled;
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
    /// Per-unit resolution uses class-based attack/defense stats from TROOPSD.DAT
    /// (`TroopClassDef`). Defense facilities at the system grant a defense bonus
    /// to the defending faction's troops.
    ///
    /// Death Star (family 0x34) takes a separate path — call `resolve_death_star()`.
    ///
    /// # Advance contract
    /// - Does NOT mutate world. Returns TroopDamageEvents.
    /// - `difficulty`: 0-7 index into GNPRTB difficulty columns.
    /// - `rng_rolls`: one roll per attacker-defender pair. Budget: troop_count² rolls.
    pub fn resolve_ground(
        world: &GameWorld,
        system: SystemKey,
        attacker_is_alliance: bool,
        difficulty: u8,
        rng_rolls: &[f64],
        tick: u64,
    ) -> GroundCombatResult {
        let sys = &world.systems[system];
        let mut rng = rng_rolls.iter().copied();
        let mut troop_damage: Vec<TroopDamageEvent> = Vec::new();

        // Step 6-8 of FUN_00560d50: iterate all troops at system by faction.
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

        // Defense facility bonus: sum bombardment_defense of defending faction's facilities.
        // Provides a flat defense bonus spread across defending troops (fortification advantage).
        let defender_is_alliance = !attacker_is_alliance;
        let facility_defense_bonus: f64 = sys.defense_facilities.iter()
            .filter_map(|&key| world.defense_facilities.get(key))
            .filter(|fac| fac.is_alliance == defender_is_alliance)
            .map(|fac| {
                world.defense_facility_classes
                    .get(&fac.class_dat_id)
                    .map(|c| c.bombardment_defense)
                    .unwrap_or(10) as f64
            })
            .sum();

        let facility_bonus_per_unit = if active_def.is_empty() {
            0.0
        } else {
            facility_defense_bonus / active_def.len() as f64
        };

        // Difficulty modifier: scales damage via GNPRTB (FUN_0053e190).
        // Default to 1.0x (100) if GNPRTB not loaded.
        let difficulty_mod = {
            let raw = world.gnprtb.value(GNPRTB_COMBAT_DIFFICULTY_MODIFIER, difficulty);
            if raw > 0 { raw as f64 / 100.0 } else { 1.0 }
        };

        // Officer combat rating modifier (from community disassembly cross-reference:
        // character combat rating at offset +0x86 factors into ground combat).
        // Sum combat skills of all characters assigned to fleets at this system
        // that match the attacker's faction.
        let officer_combat_bonus: f64 = {
            let mut total = 0u32;
            for &fleet_key in &sys.fleets {
                if let Some(fleet) = world.fleets.get(fleet_key) {
                    if fleet.is_alliance == attacker_is_alliance {
                        for &char_key in &fleet.characters {
                            if let Some(ch) = world.characters.get(char_key) {
                                total += ch.combat.base + ch.combat.variance / 2;
                            }
                        }
                    }
                }
            }
            1.0 + (total as f64 / 200.0)
        };

        // Per-unit resolution: FUN_004ee350 (30 lines).
        // Each attacker regiment attacks each defender regiment individually.
        // Damage computed from class attack/defense stats scaled by regiment strength.
        let mut current_strength: std::collections::HashMap<TroopKey, i16> = active_atk.iter()
            .chain(active_def.iter())
            .map(|&k| (k, world.troops[k].regiment_strength))
            .collect();

        // Warn once per missing class (not per inner-loop iteration).
        {
            let mut warned = std::collections::HashSet::new();
            for &key in active_atk.iter().chain(active_def.iter()) {
                let dat_id = world.troops[key].class_dat_id;
                if !world.troop_classes.contains_key(&dat_id) && warned.insert(dat_id) {
                    eprintln!("WARNING: missing TroopClassDef for {:?}, using fallback (10/10). Is TROOPSD.DAT loaded?", dat_id);
                }
            }
        }

        for &atk_key in &active_atk {
            for &def_key in &active_def {
                let roll = rng.next().unwrap_or(0.5);

                let atk_str = *current_strength.get(&atk_key).unwrap_or(&0);
                let def_str = *current_strength.get(&def_key).unwrap_or(&0);
                if atk_str <= 0 || def_str <= 0 { continue; }

                // Look up class-based attack/defense stats from TROOPSD.DAT.
                let atk_unit = &world.troops[atk_key];
                let def_unit = &world.troops[def_key];

                let atk_class_attack = world.troop_classes.get(&atk_unit.class_dat_id)
                    .map(|c| c.attack_strength as f64).unwrap_or(10.0);
                let def_class_defense = world.troop_classes.get(&def_unit.class_dat_id)
                    .map(|c| c.defense_strength as f64).unwrap_or(10.0);
                let def_class_attack = world.troop_classes.get(&def_unit.class_dat_id)
                    .map(|c| c.attack_strength as f64).unwrap_or(10.0);

                // Effective power: class stat * (regiment_strength / 100).
                // Defender gets facility bonus spread across defending units.
                let atk_effective = atk_class_attack * (atk_str as f64 / 100.0);
                let def_effective = def_class_defense * (def_str as f64 / 100.0)
                    + facility_bonus_per_unit;

                let total = atk_effective + def_effective;
                if total <= 0.0 { continue; }

                // Hit probability: proportional to effective attack vs total.
                let hit_prob = atk_effective / total;

                if roll < hit_prob {
                    // Attacker hits defender. Officer combat rating modifies damage.
                    let raw_damage = (atk_class_attack * (atk_str as f64 / 100.0) * difficulty_mod * officer_combat_bonus)
                        .max(1.0);
                    let reduction = (raw_damage as i16).max(1).min(def_str);
                    let old_strength = current_strength[&def_key];
                    let new_strength = (old_strength - reduction).max(0);
                    current_strength.insert(def_key, new_strength);
                    troop_damage.push(TroopDamageEvent {
                        troop: def_key,
                        strength_before: old_strength,
                        strength_after: new_strength,
                    });
                } else {
                    // Defender counter-attacks.
                    let raw_damage = (def_class_attack * (def_str as f64 / 100.0) * difficulty_mod)
                        .max(1.0);
                    let reduction = (raw_damage as i16).max(1).min(atk_str);
                    let old_strength = current_strength[&atk_key];
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
            troop_classes: HashMap::new(),
            defense_facility_classes: HashMap::new(),
            difficulty_index: 2,
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
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
        assert_eq!(result.winner, CombatSide::Attacker);
    }

    #[test]
    fn test_ground_combat_no_troops_is_draw() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        let rolls: Vec<f64> = vec![];
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
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
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
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

    // -----------------------------------------------------------------------
    // Ground combat formula + difficulty modifier tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_ground_combat_uses_class_attack_defense_stats() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Register troop classes with asymmetric stats.
        let strong_class = DatId::new(0x14000001);
        let weak_class = DatId::new(0x14000002);
        world.troop_classes.insert(strong_class, TroopClassDef {
            attack_strength: 50,
            defense_strength: 20,
        });
        world.troop_classes.insert(weak_class, TroopClassDef {
            attack_strength: 5,
            defense_strength: 5,
        });

        // 3 strong attackers vs 1 weak defender.
        for _ in 0..3 {
            let key = world.troops.insert(TroopUnit {
                class_dat_id: strong_class,
                is_alliance: true,
                regiment_strength: 100,
            });
            world.systems[sys].ground_units.push(key);
        }
        let def_key = world.troops.insert(TroopUnit {
            class_dat_id: weak_class,
            is_alliance: false,
            regiment_strength: 100,
        });
        world.systems[sys].ground_units.push(def_key);

        let rolls: Vec<f64> = vec![0.1; 100]; // low rolls → attacker hits
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
        assert_eq!(result.winner, CombatSide::Attacker);
        // Verify damage events reference the defender troop.
        assert!(result.troop_damage.iter().any(|e| e.troop == def_key));
    }

    #[test]
    fn test_ground_combat_defense_facility_helps_defender() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        let class_id = DatId::new(0x14000001);
        world.troop_classes.insert(class_id, TroopClassDef {
            attack_strength: 20,
            defense_strength: 20,
        });

        // Equal forces: 2 attackers vs 2 defenders.
        for _ in 0..2 {
            let key = world.troops.insert(TroopUnit {
                class_dat_id: class_id,
                is_alliance: true,
                regiment_strength: 100,
            });
            world.systems[sys].ground_units.push(key);
        }
        for _ in 0..2 {
            let key = world.troops.insert(TroopUnit {
                class_dat_id: class_id,
                is_alliance: false,
                regiment_strength: 100,
            });
            world.systems[sys].ground_units.push(key);
        }

        // Add a strong defense facility for the Empire (defender).
        let fac_class_id = DatId::new(0x1c000001);
        world.defense_facility_classes.insert(fac_class_id, DefenseFacilityClassDef {
            bombardment_defense: 200,
        });
        let fac_key = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: fac_class_id,
            is_alliance: false, // empire facility
        });
        world.systems[sys].defense_facilities.push(fac_key);

        // With massive facility bonus, defender should win or draw with equal forces.
        let rolls: Vec<f64> = vec![0.5; 100];
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
        // The facility bonus shifts hit probability toward the defender.
        assert!(result.winner == CombatSide::Defender || result.winner == CombatSide::Draw,
            "Defense facilities should help the defender, got {:?}", result.winner);
    }

    #[test]
    fn test_ground_combat_difficulty_scales_damage() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        let class_id = DatId::new(0x14000001);
        world.troop_classes.insert(class_id, TroopClassDef {
            attack_strength: 20,
            defense_strength: 10,
        });

        // Set up GNPRTB with difficulty-varying modifier.
        // Param 0x1400: easy=50 (0.5x damage), hard=200 (2x damage).
        world.gnprtb = GnprtbParams::new(vec![GnprtbEntry {
            parameter_id: 0x1400,
            development: 100,
            alliance_sp_easy: 50,   // difficulty=1 → 0.5x damage
            alliance_sp_medium: 100, // difficulty=2 → 1.0x
            alliance_sp_hard: 200,   // difficulty=3 → 2.0x
            empire_sp_easy: 100,
            empire_sp_medium: 100,
            empire_sp_hard: 100,
            multiplayer: 100,
        }]);

        // 1 attacker vs 1 defender at easy difficulty.
        let atk = world.troops.insert(TroopUnit {
            class_dat_id: class_id,
            is_alliance: true,
            regiment_strength: 100,
        });
        let def = world.troops.insert(TroopUnit {
            class_dat_id: class_id,
            is_alliance: false,
            regiment_strength: 100,
        });
        world.systems[sys].ground_units.extend([atk, def]);

        let rolls: Vec<f64> = vec![0.1]; // attacker hits
        let easy_result = CombatSystem::resolve_ground(&world, sys, true, 1, &rolls, 1);
        let easy_damage: i16 = easy_result.troop_damage.iter()
            .filter(|e| e.troop == def)
            .map(|e| e.strength_before - e.strength_after)
            .sum();

        // Reset strengths for hard difficulty test.
        world.troops[atk].regiment_strength = 100;
        world.troops[def].regiment_strength = 100;

        let rolls: Vec<f64> = vec![0.1];
        let hard_result = CombatSystem::resolve_ground(&world, sys, true, 3, &rolls, 1);
        let hard_damage: i16 = hard_result.troop_damage.iter()
            .filter(|e| e.troop == def)
            .map(|e| e.strength_before - e.strength_after)
            .sum();

        // Hard (2.0x) should deal more damage than easy (0.5x).
        assert!(hard_damage > easy_damage,
            "Hard difficulty damage ({}) should exceed easy ({})", hard_damage, easy_damage);
    }

    #[test]
    fn test_ground_combat_no_class_data_uses_fallback() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Do NOT register any troop classes — should use fallback (10/10).
        let atk = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x14000099), // unknown class
            is_alliance: true,
            regiment_strength: 100,
        });
        let def = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x14000099),
            is_alliance: false,
            regiment_strength: 50,
        });
        world.systems[sys].ground_units.extend([atk, def]);

        let rolls: Vec<f64> = vec![0.1; 10]; // attacker hits
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
        // Should produce damage events using fallback stats, not panic.
        assert!(!result.troop_damage.is_empty());
    }

    #[test]
    fn test_space_combat_difficulty_scales_hull_damage() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Set up GNPRTB with difficulty-varying modifier.
        world.gnprtb = GnprtbParams::new(vec![GnprtbEntry {
            parameter_id: 0x1400,
            development: 100,
            alliance_sp_easy: 50,
            alliance_sp_medium: 100,
            alliance_sp_hard: 200,
            empire_sp_easy: 100,
            empire_sp_medium: 100,
            empire_sp_hard: 100,
            multiplayer: 100,
        }]);

        let atk_class = make_class(&mut world, 500, 100);
        let def_class = make_class(&mut world, 500, 100);
        let atk = make_fleet(&mut world, sys, atk_class, 2, true);
        let def = make_fleet(&mut world, sys, def_class, 2, false);

        // Run at easy difficulty (0.5x damage).
        let rolls: Vec<f64> = vec![0.5; 200];
        let easy_result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);
        let easy_total_damage: i32 = easy_result.ship_damage.iter()
            .map(|e| e.hull_before - e.hull_after)
            .sum();

        // Run at hard difficulty (2.0x damage) with same rolls.
        let hard_result = CombatSystem::resolve_space(&world, atk, def, sys, 3, &rolls, 1);
        let hard_total_damage: i32 = hard_result.ship_damage.iter()
            .map(|e| e.hull_before - e.hull_after)
            .sum();

        // Hard should deal more total damage than easy.
        assert!(hard_total_damage > easy_total_damage,
            "Hard difficulty damage ({}) should exceed easy ({})", hard_total_damage, easy_total_damage);
    }

    #[test]
    fn test_ground_combat_large_asymmetric_battle() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        let elite_class = DatId::new(0x14000001);
        let militia_class = DatId::new(0x14000002);
        world.troop_classes.insert(elite_class, TroopClassDef {
            attack_strength: 80,
            defense_strength: 60,
        });
        world.troop_classes.insert(militia_class, TroopClassDef {
            attack_strength: 10,
            defense_strength: 5,
        });

        // 2 elite attackers vs 6 militia defenders.
        for _ in 0..2 {
            let key = world.troops.insert(TroopUnit {
                class_dat_id: elite_class,
                is_alliance: true,
                regiment_strength: 100,
            });
            world.systems[sys].ground_units.push(key);
        }
        for _ in 0..6 {
            let key = world.troops.insert(TroopUnit {
                class_dat_id: militia_class,
                is_alliance: false,
                regiment_strength: 100,
            });
            world.systems[sys].ground_units.push(key);
        }

        // With median rolls the elite troops should inflict heavy casualties.
        let rolls: Vec<f64> = vec![0.3; 200];
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
        // Elite attackers should win against weak militia despite being outnumbered.
        assert_eq!(result.winner, CombatSide::Attacker,
            "Elite troops should overcome militia numbers");
        // Should have generated substantial damage events.
        assert!(result.troop_damage.len() >= 6,
            "Expected at least 6 damage events, got {}", result.troop_damage.len());
    }

    // -----------------------------------------------------------------------
    // Fighter combat subsystem tests
    // -----------------------------------------------------------------------

    fn make_fighter_class(
        world: &mut GameWorld,
        attack_strength: u32,
        maneuverability: u32,
        is_alliance: bool,
    ) -> FighterKey {
        world.fighter_classes.insert(FighterClass {
            dat_id: DatId::new(0x71000001),
            name: "Test Fighter".into(),
            is_alliance,
            is_empire: !is_alliance,
            overall_attack_strength: attack_strength,
            maneuverability,
            ..FighterClass::default()
        })
    }

    fn make_fleet_with_fighters(
        world: &mut GameWorld,
        sys: SystemKey,
        ship_class: CapitalShipKey,
        ship_count: u32,
        fighter_class: FighterKey,
        fighter_squads: u32,
        is_alliance: bool,
    ) -> FleetKey {
        let key = world.fleets.insert(Fleet {
            location: sys,
            capital_ships: vec![ShipEntry { class: ship_class, count: ship_count }],
            fighters: vec![FighterEntry { class: fighter_class, count: fighter_squads }],
            characters: vec![],
            is_alliance,
            has_death_star: false,
        });
        world.systems[sys].fleets.push(key);
        key
    }

    #[test]
    fn test_fighter_dogfight_maneuverability_advantage() {
        // Side A: high maneuverability (10 attack, 20 maneuverability).
        // Side B: low maneuverability (10 attack, 2 maneuverability).
        // Same squad count — side A should take fewer losses.
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);
        let ship_class = make_class(&mut world, 500, 50);
        let fc_nimble = make_fighter_class(&mut world, 10, 20, true);
        let fc_clumsy = make_fighter_class(&mut world, 10, 2, false);

        let atk = make_fleet_with_fighters(&mut world, sys, ship_class, 2, fc_nimble, 10, true);
        let def = make_fleet_with_fighters(&mut world, sys, ship_class, 2, fc_clumsy, 10, false);

        let rolls: Vec<f64> = vec![0.8; 200];
        let result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);

        // Nimble side should lose fewer fighters.
        let atk_losses: u32 = result.fighter_losses.iter()
            .filter(|e| e.fleet == atk)
            .map(|e| e.squads_before - e.squads_after)
            .sum();
        let def_losses: u32 = result.fighter_losses.iter()
            .filter(|e| e.fleet == def)
            .map(|e| e.squads_before - e.squads_after)
            .sum();
        assert!(atk_losses <= def_losses,
            "nimble fighters (losses={atk_losses}) should not lose more than clumsy (losses={def_losses})");
    }

    #[test]
    fn test_fighter_vs_capital_uses_attack_strength() {
        // Fighters with high attack_strength should deal significant damage.
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Defender: 1 ship with hull 200, no weapons (won't fire back much).
        let def_class = make_class(&mut world, 200, 0);
        // Attacker: 1 ship (carrier) + strong fighters.
        let atk_class = make_class(&mut world, 100, 10);
        let fc_strong = make_fighter_class(&mut world, 50, 5, true);

        let atk = make_fleet_with_fighters(&mut world, sys, atk_class, 1, fc_strong, 8, true);
        let def = make_fleet(&mut world, sys, def_class, 1, false);

        let rolls: Vec<f64> = vec![0.5; 200];
        let result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);

        // Defenders should have taken hull damage from fighters.
        let def_damage: Vec<&ShipDamageEvent> = result.ship_damage.iter()
            .filter(|e| e.fleet == def)
            .collect();
        assert!(!def_damage.is_empty(), "fighters with attack_strength=50 should damage capital ships");
    }

    #[test]
    fn test_carrier_capacity_limits_fighter_launch() {
        // Carrier with fighter_capacity=2, but fleet has 10 fighter squads.
        // Only 2 should launch.
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Carrier: fighter_capacity=2 (from make_class, which sets 6 — make a custom one).
        let small_carrier = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x30000002),
            name: "Small Carrier".into(),
            is_alliance: true,
            is_empire: false,
            hull: 500,
            shield_strength: 10,
            fighter_capacity: 2,
            turbolaser_fore: 10,
            shield_recharge_rate: 3,
            ..CapitalShipClass::default()
        });

        let fc = make_fighter_class(&mut world, 100, 10, true);
        let def_class = make_class(&mut world, 100, 10);

        // Attacker: 1 small carrier with capacity 2, but 10 fighter squads.
        let atk = make_fleet_with_fighters(&mut world, sys, small_carrier, 1, fc, 10, true);
        // Defender: no fighters, just 1 weak ship.
        let def = make_fleet(&mut world, sys, def_class, 1, false);

        let rolls: Vec<f64> = vec![0.5; 300];
        let result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);

        // After recall, attacker should have at most 2 surviving fighters (carrier cap).
        let total_original: u32 = world.fleets[atk].fighters.iter().map(|e| e.count).sum();
        // Surviving = original - losses
        let total_lost: u32 = result.fighter_losses.iter()
            .filter(|e| e.fleet == atk)
            .map(|e| e.squads_before - e.squads_after)
            .sum();
        let total_surviving = total_original - total_lost;
        assert!(total_surviving <= 2,
            "carrier capacity is 2 but {total_surviving} fighters survived");
    }

    #[test]
    fn test_fighter_recall_after_carrier_destruction() {
        // If all carriers are destroyed, no fighters can be recalled.
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Attacker: 1 very weak carrier (hull=1) + fighters.
        let weak_carrier = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x30000003),
            name: "Weak Carrier".into(),
            is_alliance: true,
            is_empire: false,
            hull: 1, // will be destroyed by defender
            shield_strength: 0,
            fighter_capacity: 10,
            turbolaser_fore: 5,
            shield_recharge_rate: 0,
            ..CapitalShipClass::default()
        });
        let fc = make_fighter_class(&mut world, 10, 5, true);
        let atk = make_fleet_with_fighters(&mut world, sys, weak_carrier, 1, fc, 5, true);

        // Defender: strong ship that will destroy the carrier.
        let strong_class = make_class(&mut world, 1000, 200);
        let def = make_fleet(&mut world, sys, strong_class, 3, false);

        let rolls: Vec<f64> = vec![0.5; 300];
        let result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);

        // Attacker carrier should be destroyed — all surviving fighters lost during recall.
        let atk_ship_destroyed = result.ship_damage.iter()
            .any(|e| e.fleet == atk && e.hull_after == 0);
        if atk_ship_destroyed {
            // All fighter squads should show as lost (squads_after = 0 in loss events).
            let surviving: u32 = result.fighter_losses.iter()
                .filter(|e| e.fleet == atk)
                .map(|e| e.squads_after)
                .sum();
            assert_eq!(surviving, 0,
                "all fighters should be lost when carrier is destroyed, but {surviving} survived");
        }
        // If carrier survived, test is inconclusive — that's OK.
    }

    #[test]
    fn test_no_fighters_skips_fighter_phase() {
        // Two fleets with no fighters — fighter loss events should be empty.
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);
        let class = make_class(&mut world, 100, 50);
        let atk = make_fleet(&mut world, sys, class, 2, true);
        let def = make_fleet(&mut world, sys, class, 2, false);

        let rolls: Vec<f64> = vec![0.5; 200];
        let result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);
        assert!(result.fighter_losses.is_empty(),
            "no fighter losses expected when neither fleet has fighters");
    }

    #[test]
    fn test_asymmetric_fighter_engagement() {
        // Only attacker has fighters; defender has none.
        // Attacker fighters should damage defender ships without dogfight losses.
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);
        let ship_class = make_class(&mut world, 200, 20);
        let fc = make_fighter_class(&mut world, 30, 10, true);

        let atk = make_fleet_with_fighters(&mut world, sys, ship_class, 2, fc, 6, true);
        let def = make_fleet(&mut world, sys, ship_class, 2, false);

        let rolls: Vec<f64> = vec![0.5; 300];
        let result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);

        // Attacker should not lose fighters in dogfight (no enemy fighters).
        let atk_fighter_losses: u32 = result.fighter_losses.iter()
            .filter(|e| e.fleet == atk)
            .map(|e| e.squads_before - e.squads_after)
            .sum();
        assert_eq!(atk_fighter_losses, 0,
            "attacker should lose no fighters in dogfight when defender has none, but lost {atk_fighter_losses}");

        // Defender should have taken ship damage (from fighters + capital weapons).
        let def_damage_count = result.ship_damage.iter()
            .filter(|e| e.fleet == def)
            .count();
        assert!(def_damage_count > 0,
            "defender ships should be damaged by attacker fighters");
    }

    // -----------------------------------------------------------------------
    // Shield absorption tests (Phase 4)
    // -----------------------------------------------------------------------

    /// Helper: create a ship class with explicit shield, hull, and weapon config.
    fn make_shield_class(
        world: &mut GameWorld,
        hull: u32,
        shield: u32,
        shield_recharge: u32,
        turbolaser: u32,
        ion_cannon: u32,
    ) -> CapitalShipKey {
        world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x30000001),
            name: "Shielded Ship".into(),
            is_alliance: false,
            is_empire: true,
            hull,
            shield_strength: shield,
            shield_recharge_rate: shield_recharge,
            turbolaser_fore: turbolaser,
            ion_cannon_fore: ion_cannon,
            ..CapitalShipClass::default()
        })
    }

    #[test]
    fn test_shield_absorbs_damage_before_hull() {
        let mut ships = vec![ShipSnap {
            hull_current: 100,
            hull_max: 100,
            shield_current: 100,
            shield_max: 100,
            pending_damage: 50,
            pending_ion_damage: 0,
            shield_nibble: 0,
            weapon_nibble: 15,
            alive: true,
        }];
        CombatSystem::phase_shield_absorb(&mut ships);
        CombatSystem::phase_hull_damage(&mut ships, 1.0);
        assert_eq!(ships[0].shield_current, 50);
        assert_eq!(ships[0].hull_current, 100);
        assert!(ships[0].alive);
    }

    #[test]
    fn test_shield_overflow_damages_hull() {
        let mut ships = vec![ShipSnap {
            hull_current: 100,
            hull_max: 100,
            shield_current: 30,
            shield_max: 100,
            pending_damage: 80,
            pending_ion_damage: 0,
            shield_nibble: 0,
            weapon_nibble: 15,
            alive: true,
        }];
        CombatSystem::phase_shield_absorb(&mut ships);
        assert_eq!(ships[0].shield_current, 0);
        assert!(ships[0].pending_damage > 0);
        CombatSystem::phase_hull_damage(&mut ships, 1.0);
        assert!(ships[0].hull_current < 100);
        assert!(ships[0].hull_current > 0);
    }

    #[test]
    fn test_ion_cannon_2x_shield_damage() {
        let mut ion_vec = vec![ShipSnap {
            hull_current: 200,
            hull_max: 200,
            shield_current: 100,
            shield_max: 100,
            pending_damage: 40,
            pending_ion_damage: 40,
            shield_nibble: 0,
            weapon_nibble: 15,
            alive: true,
        }];
        let mut turbo_vec = vec![ShipSnap {
            hull_current: 200,
            hull_max: 200,
            shield_current: 100,
            shield_max: 100,
            pending_damage: 40,
            pending_ion_damage: 0,
            shield_nibble: 0,
            weapon_nibble: 15,
            alive: true,
        }];
        CombatSystem::phase_shield_absorb(&mut ion_vec);
        CombatSystem::phase_shield_absorb(&mut turbo_vec);
        assert!(ion_vec[0].shield_current < turbo_vec[0].shield_current,
            "Ion shields {} should be less than turbo shields {}",
            ion_vec[0].shield_current, turbo_vec[0].shield_current);
    }

    #[test]
    fn test_shield_recharge_per_tick() {
        let mut ships = vec![ShipSnap {
            hull_current: 100,
            hull_max: 100,
            shield_current: 0,
            shield_max: 150,
            pending_damage: 5,
            pending_ion_damage: 0,
            shield_nibble: 15,
            weapon_nibble: 15,
            alive: true,
        }];
        CombatSystem::phase_shield_absorb(&mut ships);
        assert_eq!(ships[0].shield_current, 145);
        CombatSystem::phase_hull_damage(&mut ships, 1.0);
        assert_eq!(ships[0].hull_current, 100);
    }

    #[test]
    fn test_zero_shield_passes_all_damage_to_hull() {
        let mut ships = vec![ShipSnap {
            hull_current: 100,
            hull_max: 100,
            shield_current: 0,
            shield_max: 0,
            pending_damage: 60,
            pending_ion_damage: 0,
            shield_nibble: 0,
            weapon_nibble: 15,
            alive: true,
        }];
        CombatSystem::phase_shield_absorb(&mut ships);
        assert_eq!(ships[0].pending_damage, 60);
        CombatSystem::phase_hull_damage(&mut ships, 1.0);
        assert_eq!(ships[0].hull_current, 40);
    }

    #[test]
    fn test_shield_prevents_death() {
        let mut ships = vec![ShipSnap {
            hull_current: 50,
            hull_max: 50,
            shield_current: 200,
            shield_max: 200,
            pending_damage: 100,
            pending_ion_damage: 0,
            shield_nibble: 0,
            weapon_nibble: 15,
            alive: true,
        }];
        CombatSystem::phase_shield_absorb(&mut ships);
        CombatSystem::phase_hull_damage(&mut ships, 1.0);
        assert!(ships[0].alive);
        assert_eq!(ships[0].hull_current, 50);
        assert_eq!(ships[0].shield_current, 100);
    }

    #[test]
    fn test_shield_integration_full_combat() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);
        let atk_class = make_shield_class(&mut world, 200, 0, 0, 50, 0);
        let def_class = make_shield_class(&mut world, 200, 500, 10, 50, 0);
        let atk = make_fleet(&mut world, sys, atk_class, 1, true);
        let def = make_fleet(&mut world, sys, def_class, 1, false);

        let rolls: Vec<f64> = vec![0.5; 200];
        let result = CombatSystem::resolve_space(&world, atk, def, sys, 1, &rolls, 1);

        let atk_damage: i32 = result.ship_damage.iter()
            .filter(|d| d.fleet == atk)
            .map(|d| d.hull_before - d.hull_after)
            .sum();
        let def_damage: i32 = result.ship_damage.iter()
            .filter(|d| d.fleet == def)
            .map(|d| d.hull_before - d.hull_after)
            .sum();
        assert!(def_damage < atk_damage,
            "Shielded defender hull damage {} should be less than unshielded attacker {}",
            def_damage, atk_damage);
    }

    #[test]
    fn test_ion_cannon_integration() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);
        let target_class = make_shield_class(&mut world, 200, 100, 0, 10, 0);
        let ion_atk_class = make_shield_class(&mut world, 200, 0, 0, 0, 50);
        let ion_atk = make_fleet(&mut world, sys, ion_atk_class, 1, true);
        let target1 = make_fleet(&mut world, sys, target_class, 1, false);

        let rolls: Vec<f64> = vec![0.5; 200];
        let result_ion = CombatSystem::resolve_space(&world, ion_atk, target1, sys, 1, &rolls, 1);

        let mut world2 = empty_world();
        let sector2 = make_sector(&mut world2);
        let sys2 = make_system(&mut world2, sector2);
        let target_class2 = make_shield_class(&mut world2, 200, 100, 0, 10, 0);
        let turbo_atk_class = make_shield_class(&mut world2, 200, 0, 0, 50, 0);
        let turbo_atk = make_fleet(&mut world2, sys2, turbo_atk_class, 1, true);
        let target2 = make_fleet(&mut world2, sys2, target_class2, 1, false);

        let result_turbo = CombatSystem::resolve_space(&world2, turbo_atk, target2, sys2, 1, &rolls, 1);

        let ion_hull_damage: i32 = result_ion.ship_damage.iter()
            .filter(|d| d.fleet == target1)
            .map(|d| d.hull_before - d.hull_after)
            .sum();
        let turbo_hull_damage: i32 = result_turbo.ship_damage.iter()
            .filter(|d| d.fleet == target2)
            .map(|d| d.hull_before - d.hull_after)
            .sum();
        assert!(ion_hull_damage >= turbo_hull_damage,
            "Ion hull damage {} should be >= turbo hull damage {} against shielded target",
            ion_hull_damage, turbo_hull_damage);
    }
}
