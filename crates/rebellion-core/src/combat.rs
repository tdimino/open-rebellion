//! Ground combat auto-resolve and the battle types shared with the tactical
//! engine (`side_present`, `CombatForce`, `CombatSide`).
//!
//! Space battles run through `crate::tactical` (played, simulated, or
//! headless when nobody watches; `ghidra/notes/simulate-runs-tactical-engine.md`).
//! See `ghidra/notes/ground-combat.md` for ground combat.
//!
//! # Advance contract
//! All public functions follow the simulation `advance()` pattern:
//! - Never mutate `GameWorld` — return result events for the caller to apply.
//! - Accept caller-provided RNG as `&[f64]` slices for deterministic tests.
//! - No IO.

use serde::{Deserialize, Serialize};

use crate::ids::{FleetKey, SystemKey, TroopKey};
use crate::world::GameWorld;

/// GNPRTB parameter 0x1400 for combat difficulty scaling.
/// FUN_0053e190(a,b) computes a*b/100 (DAT_00661a88 is the constant 100).
/// Value is percentage (100 = 1.0x).
const GNPRTB_COMBAT_DIFFICULTY_MODIFIER: u16 = 0x1400;

// ---------------------------------------------------------------------------
// Public result types
// ---------------------------------------------------------------------------

/// Whether a side is present at `system` for battle (`FUN_00509710`): it has
/// a fleet there, or squadrons the system holds for it. Defense facilities
/// do not count. `FUN_0050b8e0` starts a battle when both sides are present
/// (`ghidra/notes/system-squadrons-combat.md`).
#[must_use]
pub fn side_present(world: &GameWorld, system: SystemKey, is_alliance: bool) -> bool {
    world.systems.get(system).is_some_and(|value| {
        value.squadron_count(is_alliance) > 0
            || value.fleets.iter().any(|&fleet| {
                world
                    .fleets
                    .get(fleet)
                    .is_some_and(|fleet| fleet.is_alliance == is_alliance)
            })
    })
}

/// One side of a space battle: its fleet, if any, and the squadrons the
/// side holds at the battle system. `FUN_0040b0e0` sends the system's own
/// squadrons into the battle after every fleet's units, through the same
/// record builder `FUN_0040be90`, so they fight without a carrier
/// (`ghidra/notes/system-squadrons-combat.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatForce {
    pub fleet: Option<FleetKey>,
    pub is_alliance: bool,
}

impl CombatForce {
    /// `fleet` and its side's squadrons at the battle system.
    #[must_use]
    pub fn of_fleet(world: &GameWorld, fleet: FleetKey) -> Self {
        Self {
            fleet: Some(fleet),
            is_alliance: world.fleets[fleet].is_alliance,
        }
    }
}

/// Which side won a combat engagement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatSide {
    Attacker,
    Defender,
    Draw,
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
/// port: no recovered source; the original's planetary assault is open
/// (`or-q-ground-resolution-nms`). `FUN_004ee350` sets a character's
/// CommandKind at +0x96, not a regiment's strength (`ghidra/notes/ground-combat.md`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TroopDamageEvent {
    pub troop: TroopKey,
    pub strength_before: i16,
    pub strength_after: i16,
}

// ---------------------------------------------------------------------------
// CombatSystem
// ---------------------------------------------------------------------------

pub struct CombatSystem;

impl CombatSystem {
    // -----------------------------------------------------------------------
    // Ground combat — port: no recovered source (or-q-ground-resolution-nms)
    // -----------------------------------------------------------------------

    /// Auto-resolve ground combat at a system between all opposing troop units.
    ///
    /// port: no recovered source. The original's resolution is open
    /// (`or-q-ground-resolution-nms`, `or-q-regiment-combat-eaxn`; replacement
    /// `or-p-gnd-ykv`). `FUN_00560d50` and `FUN_004ee350` are the character
    /// CommandKind hook and setter, not ground combat (`ghidra/notes/ground-combat.md`).
    ///
    /// Per-unit resolution uses class-based attack/defense stats from TROOPSD.DAT
    /// (`TroopClassDef`). Defense facilities at the system grant a defense bonus
    /// to the defending faction's troops.
    ///
    /// The Death Star is capital-ship family 0x18
    /// (`ghidra/notes/death-star-strategic.md`); Destroy System runs in
    /// `DeathStarSystem::fire()`.
    ///
    /// # Advance contract
    /// - Does NOT mutate world. Returns `TroopDamageEvents`.
    /// - `difficulty`: 0-7 index into GNPRTB difficulty columns.
    /// - `rng_rolls`: one roll per attacker-defender pair. Budget: `troop_count²` rolls.
    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
    )]
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
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

        // port: iterate all troops at the system by faction.
        let (atk_keys, def_keys): (Vec<TroopKey>, Vec<TroopKey>) = sys
            .ground_units
            .iter()
            .copied()
            .partition(|&key| world.troops[key].is_alliance == attacker_is_alliance);

        // Regiment strength check (C++ offset +0x96, short):
        // units with strength <= 0 are destroyed — skip them.
        let active_atk: Vec<TroopKey> = atk_keys
            .iter()
            .copied()
            .filter(|&k| world.troops[k].regiment_strength > 0)
            .collect();
        let active_def: Vec<TroopKey> = def_keys
            .iter()
            .copied()
            .filter(|&k| world.troops[k].regiment_strength > 0)
            .collect();

        // Defense facility bonus: sum bombardment_defense of defending faction's facilities.
        // Provides a flat defense bonus spread across defending troops (fortification advantage).
        let defender_is_alliance = !attacker_is_alliance;
        let facility_defense_bonus: f64 = sys
            .defense_facilities
            .iter()
            .filter_map(|&key| world.defense_facilities.get(key))
            .filter(|fac| fac.side == crate::dat::Faction::of_alliance(defender_is_alliance))
            .map(|fac| {
                f64::from(
                    world
                        .defense_facility_classes
                        .get(&fac.class_dat_id)
                        .map_or(10, |c| c.bombardment_defense),
                )
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
            let raw = world
                .gnprtb
                .value(GNPRTB_COMBAT_DIFFICULTY_MODIFIER, difficulty);
            if raw > 0 {
                f64::from(raw) / 100.0
            } else {
                1.0
            }
        };

        // Officer combat rating modifier (from community disassembly cross-reference:
        // character combat rating at offset +0x86 factors into ground combat).
        // Sums the combat ratings of the attacker's characters in fleets at this
        // system. Uses base stat only (not variance midpoint) — the original reads the
        // rolled-concrete stat at +0x86, which is set to base during seeding.
        // Applied to attacker damage only (defender officers not factored — consistent
        // with the original's attacker-side damage modifier pattern).
        let officer_combat_bonus: f64 = {
            let mut total = 0u32;
            for &fleet_key in &sys.fleets {
                if let Some(fleet) = world.fleets.get(fleet_key) {
                    if fleet.is_alliance == attacker_is_alliance {
                        for &char_key in &fleet.characters {
                            // A character en route aboard has not arrived.
                            if let Some(ch) =
                                world.characters.get(char_key).filter(|ch| !ch.en_route)
                            {
                                total += ch.combat.base;
                            }
                        }
                    }
                }
            }
            1.0 + (f64::from(total) / 200.0)
        };

        // port: per-unit resolution.
        // Each attacker regiment attacks each defender regiment individually.
        // Damage computed from class attack/defense stats scaled by regiment strength.
        let mut current_strength: std::collections::HashMap<TroopKey, i16> = active_atk
            .iter()
            .chain(active_def.iter())
            .map(|&k| (k, world.troops[k].regiment_strength))
            .collect();

        // Warn once per missing class (not per inner-loop iteration).
        {
            let mut warned = std::collections::HashSet::new();
            for &key in active_atk.iter().chain(active_def.iter()) {
                let dat_id = world.troops[key].class_dat_id;
                if !world.troop_classes.contains_key(&dat_id) && warned.insert(dat_id) {
                    eprintln!("WARNING: missing TroopClassDef for {dat_id:?}, using fallback (10/10). Is TROOPSD.DAT loaded?");
                }
            }
        }

        for &atk_key in &active_atk {
            for &def_key in &active_def {
                let roll = rng.next().unwrap_or(0.5);

                let atk_str = *current_strength.get(&atk_key).unwrap_or(&0);
                let def_str = *current_strength.get(&def_key).unwrap_or(&0);
                if atk_str <= 0 || def_str <= 0 {
                    continue;
                }

                // Look up class-based attack/defense stats from TROOPSD.DAT.
                let atk_unit = &world.troops[atk_key];
                let def_unit = &world.troops[def_key];

                let atk_class_attack = world
                    .troop_classes
                    .get(&atk_unit.class_dat_id)
                    .map_or(10.0, |c| f64::from(c.attack_strength));
                let def_class_defense = world
                    .troop_classes
                    .get(&def_unit.class_dat_id)
                    .map_or(10.0, |c| f64::from(c.defense_strength));
                let def_class_attack = world
                    .troop_classes
                    .get(&def_unit.class_dat_id)
                    .map_or(10.0, |c| f64::from(c.attack_strength));

                // Effective power: class stat * (regiment_strength / 100).
                // Defender gets facility bonus spread across defending units.
                let atk_effective = atk_class_attack * (f64::from(atk_str) / 100.0);
                let def_effective =
                    def_class_defense * (f64::from(def_str) / 100.0) + facility_bonus_per_unit;

                let total = atk_effective + def_effective;
                if total <= 0.0 {
                    continue;
                }

                // Hit probability: proportional to effective attack vs total.
                let hit_prob = atk_effective / total;

                if roll < hit_prob {
                    // Attacker hits defender. Officer combat rating modifies damage.
                    let raw_damage = (atk_class_attack
                        * (f64::from(atk_str) / 100.0)
                        * difficulty_mod
                        * officer_combat_bonus)
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
                    let raw_damage =
                        (def_class_attack * (f64::from(def_str) / 100.0) * difficulty_mod).max(1.0);
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
            _ => CombatSide::Draw,
        };

        GroundCombatResult {
            system,
            winner,
            troop_damage,
            tick,
        }
    }

    fn count_ground_survivors(
        troops: &[TroopKey],
        damage: &[TroopDamageEvent],
        world: &GameWorld,
    ) -> usize {
        troops
            .iter()
            .filter(|&&key| {
                // Use the latest damage event for this key if present, else world state.
                if let Some(evt) = damage.iter().rev().find(|e| e.troop == key) {
                    evt.strength_after > 0
                } else {
                    world.troops[key].regiment_strength > 0
                }
            })
            .count()
    }

    // Death Star superlaser resolution is NOT handled by the combat system.
    // Planet destruction goes through DeathStarSystem::fire() in death_star.rs.
    // FUN_005617b0 recomputes CharacterMgr SeatOfPower (Emperor Palpatine,
    // 0x34000280, alive and active at Coruscant held by the Empire); it is not a
    // superlaser fire check. The original fires through order 0x223 (gate
    // FUN_00582440, run FUN_0058d800; ghidra/notes/bombardment.md,
    // or-port-bombardment-8usw).
}

/// Extract bits 4-5 of a packed field. `FUN_0054a1d0` reads
/// `*(uint *)((int)this + 0x24) >> 4 & 3` as the side it passes to
/// `FUN_004fd600`, not a difficulty; nothing in the simulation calls this.
#[must_use]
pub fn extract_difficulty(packed: u32) -> u8 {
    ((packed >> 4) & 3) as u8
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::{ExplorationStatus, SectorGroup};
    use crate::ids::*;
    use crate::world::ControlKind;
    use crate::world::*;
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
            mission_records: Vec::new(),
            special_force_classes: HashMap::new(),
            buildable_classes: HashMap::new(),
            troop_classes: HashMap::new(),
            defense_facility_classes: HashMap::new(),
            difficulty_index: 2,
            recruit_pool_empty: [false; 2],
            research_exhausted: [[false; 3]; 2],
            fleet_names: crate::world::FleetNames::default(),
            last_ship_tag: 0,
            squadron_transits: Vec::new(),
            arrival_holders: std::collections::HashMap::new(),
            death_star_sectors: std::collections::HashSet::new(),
        }
    }

    fn make_sector(world: &mut GameWorld) -> SectorKey {
        world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0001),
            name: "Test Sector".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        })
    }

    fn make_system(world: &mut GameWorld, sector: SectorKey) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
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
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
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
            dat_id: DatId::new(0x3000_0001),
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

    #[test]
    fn extract_difficulty_reads_packed_bits_four_and_five() {
        // No recovered source for difficulty here: FUN_0054a1d0 reads
        // `+0x24 >> 4 & 3` as the side passed to FUN_004fd600.
        // difficulty bits 4-5: value 2 = hard → packed = 0x20
        assert_eq!(extract_difficulty(0x20), 2);
        assert_eq!(extract_difficulty(0x00), 0);
        assert_eq!(extract_difficulty(0x10), 1);
        assert_eq!(extract_difficulty(0x30), 3);
    }

    #[test]
    fn ground_combat_attacker_wins_with_superior_strength() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Attacker: 5 strong alliance troops. Defender: 1 weak empire troop.
        for _ in 0..5 {
            let key = world.troops.insert(TroopUnit {
                class_dat_id: DatId::new(0x1400_0001),
                is_alliance: true,
                regiment_strength: 100,
            });
            world.systems[sys].ground_units.push(key);
        }
        let def_key = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1400_0002),
            is_alliance: false,
            regiment_strength: 4, // 5 attackers each deal 1 damage: 4→3→2→1→0 (dead)
        });
        world.systems[sys].ground_units.push(def_key);

        let rolls: Vec<f64> = vec![0.1; 100]; // low rolls → attacker almost always hits
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
        assert_eq!(result.winner, CombatSide::Attacker);
    }

    #[test]
    fn ground_combat_with_no_troops_is_a_draw() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        let rolls: Vec<f64> = vec![];
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
        assert_eq!(result.winner, CombatSide::Draw);
        assert!(result.troop_damage.is_empty());
    }

    #[test]
    fn ground_combat_skips_zero_strength_troops() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Both troops have strength 0 — both filtered out, should be a draw.
        let k1 = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1400_0001),
            is_alliance: true,
            regiment_strength: 0,
        });
        let k2 = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1400_0002),
            is_alliance: false,
            regiment_strength: 0,
        });
        world.systems[sys].ground_units.extend([k1, k2]);

        let rolls: Vec<f64> = vec![0.5; 10];
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
        assert_eq!(result.winner, CombatSide::Draw);
    }

    // -----------------------------------------------------------------------
    // Ground combat formula + difficulty modifier tests
    // -----------------------------------------------------------------------

    #[test]
    fn ground_combat_uses_class_attack_and_defense_stats() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Register troop classes with asymmetric stats.
        let strong_class = DatId::new(0x1400_0001);
        let weak_class = DatId::new(0x1400_0002);
        world.troop_classes.insert(
            strong_class,
            TroopClassDef {
                attack_strength: 50,
                defense_strength: 20,
                detection: 0,
            },
        );
        world.troop_classes.insert(
            weak_class,
            TroopClassDef {
                attack_strength: 5,
                defense_strength: 5,
                detection: 0,
            },
        );

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
    fn ground_combat_defense_facility_helps_the_defender() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        let class_id = DatId::new(0x1400_0001);
        world.troop_classes.insert(
            class_id,
            TroopClassDef {
                attack_strength: 20,
                defense_strength: 20,
                detection: 0,
            },
        );

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
        let fac_class_id = DatId::new(0x1c00_0001);
        world.defense_facility_classes.insert(
            fac_class_id,
            DefenseFacilityClassDef {
                bombardment_defense: 200,
            },
        );
        let fac_key = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: fac_class_id,
            side: crate::dat::Faction::Empire, // empire facility
        });
        world.systems[sys].defense_facilities.push(fac_key);

        // With massive facility bonus, defender should win or draw with equal forces.
        let rolls: Vec<f64> = vec![0.5; 100];
        let result = CombatSystem::resolve_ground(&world, sys, true, 2, &rolls, 1);
        // The facility bonus shifts hit probability toward the defender.
        assert!(
            result.winner == CombatSide::Defender || result.winner == CombatSide::Draw,
            "Defense facilities should help the defender, got {:?}",
            result.winner
        );
    }

    #[test]
    fn ground_combat_difficulty_scales_damage() {
        // FUN_0053e190(a, b) = a * b / 100 (DAT_00661a88 is 100); GNPRTB
        // 0x1400 as its second argument is not yet traced to a caller.
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        let class_id = DatId::new(0x1400_0001);
        world.troop_classes.insert(
            class_id,
            TroopClassDef {
                attack_strength: 20,
                defense_strength: 10,
                detection: 0,
            },
        );

        // Set up GNPRTB with difficulty-varying modifier.
        // Param 0x1400: easy=50 (0.5x damage), hard=200 (2x damage).
        world.gnprtb = GnprtbParams::new(vec![GnprtbEntry {
            parameter_id: 0x1400,
            development: 100,
            alliance_sp_easy: 50,    // difficulty=1 → 0.5x damage
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
        let easy_damage: i16 = easy_result
            .troop_damage
            .iter()
            .filter(|e| e.troop == def)
            .map(|e| e.strength_before - e.strength_after)
            .sum();

        // Reset strengths for hard difficulty test.
        world.troops[atk].regiment_strength = 100;
        world.troops[def].regiment_strength = 100;

        let rolls: Vec<f64> = vec![0.1];
        let hard_result = CombatSystem::resolve_ground(&world, sys, true, 3, &rolls, 1);
        let hard_damage: i16 = hard_result
            .troop_damage
            .iter()
            .filter(|e| e.troop == def)
            .map(|e| e.strength_before - e.strength_after)
            .sum();

        // Hard (2.0x) should deal more damage than easy (0.5x).
        assert!(
            hard_damage > easy_damage,
            "Hard difficulty damage ({hard_damage}) should exceed easy ({easy_damage})"
        );
    }

    #[test]
    fn elite_troops_overcome_numerically_superior_militia() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        let elite_class = DatId::new(0x1400_0001);
        let militia_class = DatId::new(0x1400_0002);
        world.troop_classes.insert(
            elite_class,
            TroopClassDef {
                attack_strength: 80,
                defense_strength: 60,
                detection: 0,
            },
        );
        world.troop_classes.insert(
            militia_class,
            TroopClassDef {
                attack_strength: 10,
                defense_strength: 5,
                detection: 0,
            },
        );

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
        assert_eq!(
            result.winner,
            CombatSide::Attacker,
            "Elite troops should overcome militia numbers"
        );
        // Should have generated substantial damage events.
        assert!(
            result.troop_damage.len() >= 6,
            "Expected at least 6 damage events, got {}",
            result.troop_damage.len()
        );
    }

    // -----------------------------------------------------------------------
    // Fighter combat subsystem tests
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Shield absorption tests (Phase 4)
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Phase 2 pending tests: DS shield hull-damage path
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Phase 2 pending tests: Officer combat rating in ground combat
    // -----------------------------------------------------------------------

    fn make_troop(world: &mut GameWorld, sys: SystemKey, is_alliance: bool) -> TroopKey {
        let key = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1400_0100),
            is_alliance,
            regiment_strength: 100,
        });
        world.systems[sys].ground_units.push(key);
        key
    }

    #[test]
    fn officer_combat_bonus_increases_ground_damage() {
        let mut world = empty_world();
        let sector = make_sector(&mut world);
        let sys = make_system(&mut world, sector);

        // Add troop classes so combat works
        world.troop_classes.insert(
            DatId::new(0x1400_0100),
            TroopClassDef {
                attack_strength: 30,
                defense_strength: 20,
                detection: 0,
            },
        );

        // Add troops: 2 attacker, 2 defender
        let _atk1 = make_troop(&mut world, sys, true);
        let _atk2 = make_troop(&mut world, sys, true);
        let def1 = make_troop(&mut world, sys, false);
        let def2 = make_troop(&mut world, sys, false);

        // Run ground combat WITHOUT officer
        let rolls: Vec<f64> = vec![0.3; 100];
        let result_no_officer = CombatSystem::resolve_ground(&world, sys, true, 1, &rolls, 1);

        // Sum damage to defender troops
        let no_officer_damage: i16 = result_no_officer
            .troop_damage
            .iter()
            .filter(|d| d.troop == def1 || d.troop == def2)
            .map(|d| d.strength_before - d.strength_after)
            .sum();

        // Now set up a fresh world with an officer
        let mut world2 = empty_world();
        let sector2 = make_sector(&mut world2);
        let sys2 = make_system(&mut world2, sector2);
        world2.troop_classes.insert(
            DatId::new(0x1400_0100),
            TroopClassDef {
                attack_strength: 30,
                defense_strength: 20,
                detection: 0,
            },
        );

        let _atk2_1 = make_troop(&mut world2, sys2, true);
        let _atk2_2 = make_troop(&mut world2, sys2, true);
        let def2_1 = make_troop(&mut world2, sys2, false);
        let def2_2 = make_troop(&mut world2, sys2, false);

        // Add a fleet with an officer (combat.base = 80)
        let officer = world2.characters.insert(Character {
            dat_id: DatId::new(0x0800_0001),
            name: "Admiral Ackbar".into(),
            is_alliance: true,
            is_major: true,
            combat: SkillPair {
                base: 80,
                variance: 0,
            },
            can_be_admiral: true,
            ..Default::default()
        });
        let atk_class = make_class(&mut world2, 100, 10);
        let fleet = world2.fleets.insert(Fleet {
            location: sys2,
            capital_ships: vec![ShipInstance::new(
                atk_class,
                world2.capital_ship_classes[atk_class].hull.cast_signed(),
                true,
            )],
            fighters: vec![],
            characters: vec![officer],
            is_alliance: true,
            has_death_star: false,
        });
        world2.systems[sys2].fleets.push(fleet);

        let result_with_officer = CombatSystem::resolve_ground(&world2, sys2, true, 1, &rolls, 1);

        // Officer with combat=80 gives 1.0 + (80/200) = 1.4x multiplier
        let with_officer_damage: i16 = result_with_officer
            .troop_damage
            .iter()
            .filter(|d| d.troop == def2_1 || d.troop == def2_2)
            .map(|d| d.strength_before - d.strength_after)
            .sum();

        assert!(with_officer_damage > no_officer_damage,
            "Officer (combat=80, 1.4x multiplier) should strictly increase damage: with={with_officer_damage}, without={no_officer_damage}");
    }
}
