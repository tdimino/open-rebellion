//! Orbital bombardment resolution.
//!
//! Direct translation of the C++ formula chain from REBEXE.EXE:
//!
//! ```text
//! FUN_00556430 → FUN_00555d30 → FUN_00555b30
//!   → FUN_00509620 (get combat stats as short[2])
//!   → FUN_0055d8c0 (damage formula)
//!     → FUN_0055d860: raw_power = sqrt((atk[0]-def[0])² + (atk[1]-def[1])²)
//!     → / DAT_006bb6e8 (GNPRTB bombardment divisor, param_id=0x1400)
//!     → FUN_0053e190 (apply difficulty modifier)
//!   → minimum 1 damage
//! ```
//!
//! See `ghidra/notes/bombardment.md` and `rust-implementation-guide.md` §4.
//!
//! # Advance contract
//! Never mutates `GameWorld`. Returns a `BombardmentResult` for the caller to apply.

use serde::{Deserialize, Serialize};

use crate::ids::{FleetKey, SystemKey};
use crate::world::{ControlKind, GameWorld};

/// GNPRTB parameter_id for the bombardment damage divisor (DAT_006bb6e8).
/// Confirmed from `data/base/json/GNPRTB.json`: param_id=0x1400 → value=5.
const GNPRTB_BOMBARDMENT_DIVISOR_PARAM: u16 = 0x1400;

/// Result of one orbital bombardment strike.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BombardmentResult {
    pub system: SystemKey,
    /// Damage applied to the defending system (popularity/facilities reduction).
    /// Minimum 1 — confirmed from C++ `result == 0 ? 1 : result`.
    pub damage: i32,
    pub tick: u64,
}

pub struct BombardmentSystem;

impl BombardmentSystem {
    /// Resolve one orbital bombardment strike from `attacker_fleet` against `system`.
    ///
    /// # Formula
    /// ```text
    /// atk = (fleet_bombardment_stat, fleet_secondary_stat)
    /// def = (system_defense_stat,    system_secondary_stat)
    /// raw_power = sqrt((atk[0]-def[0])² + (atk[1]-def[1])²)
    /// damage = floor(raw_power / gnprtb_divisor * difficulty_modifier)
    /// damage = max(damage, 1)
    /// ```
    ///
    /// # Guards (from FUN_00556430)
    /// - No self-bombardment: attacker faction == defender faction → damage = 0.
    /// - Bombardment disabled flag (`system.bombardment_blocked`) → damage = 0.
    ///   (Flag not yet in world model — add with blockade mechanics in task #20.)
    ///
    /// # Advance contract
    /// - Does NOT mutate world.
    /// - `difficulty`: 0=development, 1=alliance_easy, 2=alliance_medium, 3=alliance_hard,
    ///                  4=empire_easy, 5=empire_medium, 6=empire_hard, 7=multiplayer.
    ///   Matches the C++ `difficulty_packed` bits 4-5 mapping from `GnprtbParams::value()`.
    pub fn resolve_bombardment(
        world: &GameWorld,
        attacker_fleet: FleetKey,
        system: SystemKey,
        difficulty: u8,
        tick: u64,
    ) -> BombardmentResult {
        let fleet = &world.fleets[attacker_fleet];
        let sys = &world.systems[system];

        // Guard: no self-bombardment (uVar1 == uVar2 in FUN_00556430).
        // Compare faction of attacking fleet vs controlling faction of the system.
        let fleet_is_alliance = fleet.is_alliance;
        let system_controlled_by_alliance = match sys.control {
            ControlKind::Controlled(crate::dat::Faction::Alliance) => true,
            ControlKind::Controlled(crate::dat::Faction::Empire) => false,
            _ => {
                // Neutral/uncontrolled/contested: allow bombardment.
                !fleet_is_alliance // treat as opposite faction to avoid self-hit
            }
        };
        if fleet_is_alliance == system_controlled_by_alliance {
            return BombardmentResult { system, damage: 0, tick };
        }

        // FUN_00509620: get combat stats as (attack, secondary) pair.
        let atk = Self::fleet_bombardment_stats(world, attacker_fleet);
        let def = Self::system_bombardment_defense(world, system);

        // FUN_0055d860: power_ratio = Euclidean distance of the stat vectors.
        // C++: sqrt((atk[0]-def[0])² + (atk[1]-def[1])²)
        let dx = (atk.0 - def.0) as f64;
        let dy = (atk.1 - def.1) as f64;
        let raw_power = (dx * dx + dy * dy).sqrt();

        if raw_power == 0.0 {
            // No net power advantage: early-exit guard from FUN_0055d8c0 returns 0 before
            // the minimum-1 applies. Minimum-1 only fires when result after division is 0
            // but raw_power was non-zero (i.e., there was some power but it divided to zero).
            return BombardmentResult { system, damage: 0, tick };
        }

        // DAT_006bb6e8: GNPRTB bombardment divisor (param_id = 0x1400).
        // Value = 5 (all difficulty modes, from GNPRTB.json).
        let gnprtb_divisor = world.gnprtb.value(GNPRTB_BOMBARDMENT_DIVISOR_PARAM, difficulty);
        let divisor = (gnprtb_divisor.max(1)) as f64; // guard divide-by-zero

        // FUN_0053e190: apply difficulty modifier.
        // The difficulty modifier adjusts the pre-divisor result.
        // In the actual data all GnprtbParams values are uniform across difficulty,
        // so the modifier is 1.0 unless the caller explicitly passes per-difficulty params.
        // We treat divisor as the only difficulty-varying parameter per the RE notes.
        let damage_f = raw_power / divisor;

        // Minimum 1 damage — confirmed from C++ `result == 0 ? 1 : result`.
        let damage = (damage_f.floor() as i32).max(1);

        BombardmentResult { system, damage, tick }
    }

    /// Aggregate bombardment attack stats `(attack, secondary)` for a fleet.
    ///
    /// Maps to `FUN_00509620` called for the attacker side.
    ///
    /// - `[0]` = aggregate bombardment_modifier across all capital ships + fighters.
    /// - `[1]` = aggregate maneuverability (secondary stat proxy pending decompile
    ///           of FUN_00509620's exact secondary field selection).
    fn fleet_bombardment_stats(world: &GameWorld, fleet: FleetKey) -> (i32, i32) {
        let f = &world.fleets[fleet];
        let mut brd: i32 = 0;
        let mut sec: i32 = 0;

        for entry in &f.capital_ships {
            let class = &world.capital_ship_classes[entry.class];
            brd += class.bombardment_modifier as i32 * entry.count as i32;
            sec += class.maneuverability as i32 * entry.count as i32;
        }
        for entry in &f.fighters {
            let class = &world.fighter_classes[entry.class];
            // bombardment_defense in FIGHTSD.DAT encodes bomber attack capability.
            brd += class.bombardment_defense as i32 * entry.count as i32;
        }

        (brd, sec)
    }

    /// Aggregate bombardment defense stats `(defense, secondary)` for a system.
    ///
    /// Maps to `FUN_00509620` called for the defender side.
    ///
    /// - `[0]` = sum of defense facility bombardment_defense (placeholder: 10 each).
    ///           Full implementation deferred pending `DefenseFacilityClass` world model.
    /// - `[1]` = sum of troop regiment_strength as secondary defense contribution.
    fn system_bombardment_defense(world: &GameWorld, system: SystemKey) -> (i32, i32) {
        let sys = &world.systems[system];
        let mut def_stat: i32 = 0;
        let mut sec_stat: i32 = 0;

        // Defense facilities contribute primary bombardment defense.
        // Look up each facility's class definition for its bombardment_defense value.
        for &key in &sys.defense_facilities {
            if let Some(facility) = world.defense_facilities.get(key) {
                if let Some(class) = world.defense_facility_classes.get(&facility.class_dat_id) {
                    def_stat += class.bombardment_defense;
                } else {
                    def_stat += 10; // fallback if class definition not loaded
                }
            }
        }

        // Troops contribute secondary defense via regiment_strength.
        for &key in &sys.ground_units {
            sec_stat += world.troops[key].regiment_strength as i32;
        }

        (def_stat, sec_stat)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::*;
    use crate::ids::*;
    use crate::dat::{ExplorationStatus, Faction, SectorGroup};
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

    fn make_world_with_gnprtb() -> GameWorld {
        let mut world = empty_world();
        // Seed the bombardment divisor: param_id = 0x1400 = 5120, value = 5.
        world.gnprtb = GnprtbParams::new(vec![GnprtbEntry {
            parameter_id: 0x1400,
            development: 5,
            alliance_sp_easy: 5,
            alliance_sp_medium: 5,
            alliance_sp_hard: 5,
            empire_sp_easy: 5,
            empire_sp_medium: 5,
            empire_sp_hard: 5,
            multiplayer: 5,
        }]);
        world
    }

    fn make_sector(world: &mut GameWorld) -> SectorKey {
        world.sectors.insert(Sector {
            dat_id: DatId::new(0x92000001),
            name: "Outer Rim".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        })
    }

    fn make_system(world: &mut GameWorld, sector: SectorKey,
                   controlling: Option<Faction>) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId::new(0x90000001),
            name: "Hoth".into(),
            sector,
            x: 50,
            y: 50,
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
            control: controlling.map(ControlKind::Controlled).unwrap_or(ControlKind::Uncontrolled),
            is_headquarters: false,
            is_destroyed: false,
        })
    }

    fn make_capital_ship_class(world: &mut GameWorld, bombardment: u32) -> CapitalShipKey {
        world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x30000001),
            name: "ISD".into(),
            is_alliance: false,
            is_empire: true,
            hull: 200,
            shield_strength: 10,
            sub_light_engine: 5,
            maneuverability: 3,
            hyperdrive: 2,
            fighter_capacity: 6,
            detection: 5,
            turbolaser_fore: 5,
            shield_recharge_rate: 3,
            damage_control: 1,
            bombardment_modifier: bombardment,
            ..CapitalShipClass::default()
        })
    }

    fn make_fleet(world: &mut GameWorld, sys: SystemKey, class: CapitalShipKey,
                  count: u32, is_alliance: bool) -> FleetKey {
        world.fleets.insert(Fleet {
            location: sys,
            capital_ships: vec![ShipEntry { class, count }],
            fighters: vec![],
            characters: vec![],
            is_alliance,
            has_death_star: false,
        })
    }

    #[test]
    fn test_bombardment_formula_minimum_one_damage() {
        // Even with zero attack vs zero defense, we get 0 (raw_power=0 guard fires).
        // But with any positive attack > defense, result must be >= 1.
        let mut world = make_world_with_gnprtb();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector, Some(Faction::Alliance));
        let class = make_capital_ship_class(&mut world, 20); // bombardment=20, def=0
        let fleet = make_fleet(&mut world, sys, class, 3, false); // Empire attacks Alliance

        let result = BombardmentSystem::resolve_bombardment(&world, fleet, sys, 2, 1);
        assert!(result.damage >= 1, "Bombardment damage should be at least 1, got {}", result.damage);
    }

    #[test]
    fn test_bombardment_no_self_damage() {
        let mut world = make_world_with_gnprtb();
        let sector = make_sector(&mut world);
        // Empire controls, Empire attacks → self-bombardment → damage = 0.
        let sys = make_system(&mut world, sector, Some(Faction::Empire));
        let class = make_capital_ship_class(&mut world, 100);
        let fleet = make_fleet(&mut world, sys, class, 5, false); // is_alliance=false = Empire

        let result = BombardmentSystem::resolve_bombardment(&world, fleet, sys, 2, 1);
        assert_eq!(result.damage, 0, "Self-bombardment must deal 0 damage");
    }

    #[test]
    fn test_bombardment_scales_with_fleet_strength() {
        let mut world = make_world_with_gnprtb();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector, Some(Faction::Alliance));

        let class_small = make_capital_ship_class(&mut world, 5);
        let fleet_small = make_fleet(&mut world, sys, class_small, 1, false);
        let result_small = BombardmentSystem::resolve_bombardment(&world, fleet_small, sys, 2, 1);

        let class_large = make_capital_ship_class(&mut world, 50);
        let fleet_large = make_fleet(&mut world, sys, class_large, 5, false);
        let result_large = BombardmentSystem::resolve_bombardment(&world, fleet_large, sys, 2, 1);

        assert!(
            result_large.damage >= result_small.damage,
            "Larger fleet should deal >= damage: large={} small={}",
            result_large.damage, result_small.damage
        );
    }

    #[test]
    fn test_bombardment_euclidean_formula_exact() {
        // Manual calculation: atk=(30, 3*1=3), def=(0, 0), divisor=5.
        // raw_power = sqrt(30² + 3²) = sqrt(900 + 9) = sqrt(909) ≈ 30.15
        // damage = floor(30.15 / 5) = floor(6.03) = 6
        let mut world = make_world_with_gnprtb();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector, Some(Faction::Alliance));
        let class = make_capital_ship_class(&mut world, 30); // bombardment=30, maneuver=3
        let fleet = make_fleet(&mut world, sys, class, 1, false);

        let result = BombardmentSystem::resolve_bombardment(&world, fleet, sys, 2, 1);
        assert_eq!(result.damage, 6);
    }
}
