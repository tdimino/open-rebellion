//! Ship repair — timer `0x380` (`ghidra/notes/ship-damage-repair.md`).
//!
//! A damaged, usable ship not in hyperspace (`FUN_005578d0`) holds a repair
//! timer. It fires every GNPRTB `0x805` days (shipped 30), or every GNPRTB
//! `0x804` days (shipped 2) while the ship's system, held by the ship's
//! side, has a counted working shipyard of that side (`FUN_00557860`,
//! `FUN_00557c30`, `FUN_00528040`, `FUN_005010a0`). Each fire is `FUN_00501c50`: the hull gains a tenth of
//! its maximum, and the damage counters lose one point with the probability
//! of the class damage control percent.
//!
//! # Advance contract
//! `RepairSystem::advance()` never mutates `GameWorld`.
//! It returns `Vec<RepairEvent>` for the caller to apply with
//! [`RepairEvent::apply`].

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::{FleetKey, SystemKey};
use crate::research::ResearchSystem;
use crate::tick::TickEvent;
use crate::uprising::UprisingState;
use crate::world::{GameWorld, RepairTimer, ShipInstance};

/// GNPRTB `0x804`: days between repair steps with a shipyard at hand
/// (`FUN_005010a0`, `FUN_00557c30`).
pub const GNPRTB_FAST_REPAIR_DAYS: u16 = 0x804;
/// GNPRTB `0x805`: days between repair steps elsewhere (`FUN_005010a0`).
pub const GNPRTB_REPAIR_DAYS: u16 = 0x805;
/// GNPRTB `0x804` shipped value in every column (`FUN_005010a0`), used when
/// no GNPRTB table is loaded.
const SHIPPED_FAST_REPAIR_DAYS: i64 = 2;
/// GNPRTB `0x805` shipped value in every column (`FUN_005010a0`), used when
/// no GNPRTB table is loaded.
const SHIPPED_REPAIR_DAYS: i64 = 30;
/// The shipyard family byte (`FUN_00433e40`, family `0x28`).
const SHIPYARD_FAMILY: u8 = 0x28;

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

/// Events produced by the repair system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepairEvent {
    /// A repair step fired (`FUN_00501c50`) and the timer rearmed.
    ShipRepaired {
        fleet: FleetKey,
        ship_index: usize,
        hull_before: i32,
        hull_after: i32,
        /// The ship's six damage counters before and after.
        counters_before: u32,
        counters_after: u32,
        timer: RepairTimer,
    },
    /// A ship's timer was armed, rearmed for a changed interval, or
    /// cancelled (`None`) without a repair step.
    TimerSet {
        fleet: FleetKey,
        ship_index: usize,
        timer: Option<RepairTimer>,
    },
    /// Repair work started for damaged ships in one fleet.
    /// Emitted when a fleet's ships first hold a timer, never for a healthy
    /// fleet.
    RepairCheckPerformed {
        system: SystemKey,
        fleet: FleetKey,
        ships_checked: usize,
    },
}

impl RepairEvent {
    /// Write the event's effect on the ship into `world`: the hull and the
    /// six counters (`FUN_00501b70`, `FUN_00501510` and its siblings) and
    /// the timer (`FUN_0053fa60`).
    pub fn apply(&self, world: &mut GameWorld) {
        let (fleet, ship_index) = match self {
            Self::ShipRepaired {
                fleet, ship_index, ..
            }
            | Self::TimerSet {
                fleet, ship_index, ..
            } => (*fleet, *ship_index),
            Self::RepairCheckPerformed { .. } => return,
        };
        if let Some(ship) = world
            .fleets
            .get_mut(fleet)
            .and_then(|f| f.capital_ships.get_mut(ship_index))
        {
            self.apply_to_ship(ship);
        }
    }

    /// The event's effect on one ship (`FUN_00501b70`, `FUN_00501510`,
    /// `FUN_0053fa60`); [`Self::apply`] and the batch loop of
    /// [`RepairSystem::advance`] share it.
    fn apply_to_ship(&self, ship: &mut ShipInstance) {
        match self {
            Self::ShipRepaired {
                hull_after,
                counters_after,
                timer,
                ..
            } => {
                ship.hull_current = *hull_after;
                ship.damage_counters = (ship.damage_counters & !0x00ff_ffff) | counters_after;
                ship.repair_timer = Some(*timer);
            }
            Self::TimerSet { timer, .. } => ship.repair_timer = *timer,
            Self::RepairCheckPerformed { .. } => {}
        }
    }
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Repair-system state used to distinguish the beginning of a repair episode
/// from the repair steps that follow it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepairState {
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_set",
        deserialize_with = "crate::serde_ordered::deserialize_hash_set"
    )]
    active_fleets: HashSet<FleetKey>,
}

impl RepairState {
    /// Whether a fleet was undergoing repair at the preceding repair step.
    #[must_use]
    pub fn is_repairing(&self, fleet: FleetKey) -> bool {
        self.active_fleets.contains(&fleet)
    }
}

// ---------------------------------------------------------------------------
// System
// ---------------------------------------------------------------------------

pub struct RepairSystem;

impl RepairSystem {
    /// The rolls one `advance` over `ticks` days can use: two per damaged
    /// ship per day (the damage-control roll and the counter pick of
    /// `FUN_00501c50`), the most a ship fires.
    #[must_use]
    pub fn roll_budget(world: &GameWorld, ticks: usize) -> usize {
        2 * ticks
            * world
                .fleets
                .values()
                .flat_map(|fleet| &fleet.capital_ships)
                .filter(|ship| ship.repair_timer.is_some() || Self::damaged(world, ship))
                .count()
    }

    fn damaged(world: &GameWorld, ship: &ShipInstance) -> bool {
        ship.alive
            && world
                .capital_ship_classes
                .get(ship.class)
                .is_some_and(|class| ship.is_damaged(class.hull.cast_signed()))
    }

    /// The interval of a ship's timer (`FUN_005010a0`).
    fn interval(world: &GameWorld, fast: bool) -> u64 {
        let (id, shipped) = if fast {
            (GNPRTB_FAST_REPAIR_DAYS, SHIPPED_FAST_REPAIR_DAYS)
        } else {
            (GNPRTB_REPAIR_DAYS, SHIPPED_REPAIR_DAYS)
        };
        u64::try_from(crate::movement::gnprtb_or_shipped(world, id, shipped)).unwrap_or(1)
    }

    /// Whether the system holds a counted shipyard of `side`
    /// (`FUN_0050bb70`, `FUN_00528040`): the side controls the system
    /// (`+0x24` bits 6-7), the yard works (`+0x60` bit 0 clear: neither
    /// blockaded nor in uprising, `ResearchSystem::yards_work`), and it is
    /// of the shipyard family. hyp: a facility of family `0x28` in the
    /// system's list is counted.
    fn has_shipyard(
        world: &GameWorld,
        uprisings: &UprisingState,
        system: SystemKey,
        side: Faction,
    ) -> bool {
        world.systems.get(system).is_some_and(|sys| {
            sys.control.is_controlled_by(side)
                && ResearchSystem::yards_work(world, uprisings, system)
                && sys
                    .manufacturing_facilities
                    .iter()
                    .filter_map(|key| world.manufacturing_facilities.get(*key))
                    .any(|yard| yard.side == side && yard.class_dat_id.family() == SHIPYARD_FAMILY)
        })
    }

    /// `FUN_00501c50`: the repair step of one ship. The hull gains a tenth
    /// of its maximum; with a counter standing, `damage_roll` under the
    /// class damage control percent removes one point, chosen by
    /// `point_roll` over the counters in the order shield, weapon, bits
    /// 8-11, bits 12-15, primary drive, backup drive.
    fn step(
        ship: &ShipInstance,
        hull_max: i32,
        damage_control: u32,
        damage_roll: f64,
        point_roll: f64,
    ) -> (i32, u32) {
        let mut hull = ship.hull_current;
        if hull < hull_max {
            hull = (hull + hull_max / 10).min(hull_max);
        }
        let mut counters = ship.damage_counters & 0x00ff_ffff;
        let sum = ship.damage_counter_sum();
        if sum > 0 && (damage_roll * 100.0) < f64::from(damage_control) {
            // FUN_0053e290(sum - 1): uniform over 0..=sum-1.
            let mut point = ((point_roll * f64::from(sum)) as u32).min(sum - 1);
            for index in 0..6 {
                let held = u32::from(ship.damage_counter(index));
                if point < held {
                    counters -= 1 << (4 * index);
                    break;
                }
                point -= held;
            }
        }
        (hull, counters)
    }

    /// Advance ship repairs through every tick of `tick_events`. `rolls` are
    /// uniform draws in `[0, 1)` from the simulation's stream, two per ship
    /// that fires, consumed in tick order ([`Self::roll_budget`]).
    ///
    /// Each tick sees the ships as the earlier ticks of the batch left them,
    /// so a long frame equals the same ticks taken one call at a time.
    ///
    /// Returns repair events for the caller to apply to `GameWorld`.
    pub fn advance(
        state: &mut RepairState,
        world: &GameWorld,
        uprisings: &UprisingState,
        tick_events: &[TickEvent],
        rolls: &[f64],
    ) -> Vec<RepairEvent> {
        let mut events = Vec::new();
        let mut next_roll = 0;
        let mut draw = || {
            let roll = rolls.get(next_roll).copied().unwrap_or(1.0);
            next_roll += 1;
            roll
        };
        // Ships the batch has already changed, by fleet and ship index.
        let mut changed: HashMap<(FleetKey, usize), ShipInstance> = HashMap::new();

        for tick in tick_events {
            let now = tick.tick;
            let mut repairing_now = HashSet::new();
            for (fleet_key, fleet) in &world.fleets {
                // A fleet in hyperspace is in no system's list (`+0x50` bit 4).
                let at_system = world
                    .systems
                    .get(fleet.location)
                    .is_some_and(|sys| !sys.is_destroyed && sys.fleets.contains(&fleet_key));
                let side = if fleet.is_alliance {
                    Faction::Alliance
                } else {
                    Faction::Empire
                };
                let fast = at_system && Self::has_shipyard(world, uprisings, fleet.location, side);
                let mut ships_repaired = 0;
                for (ship_index, original) in fleet.capital_ships.iter().enumerate() {
                    let ship = changed.get(&(fleet_key, ship_index)).unwrap_or(original);
                    // FUN_005578d0: damaged and usable, not in hyperspace.
                    let under_repair = at_system && Self::damaged(world, ship);
                    let Some(class) = world.capital_ship_classes.get(ship.class) else {
                        continue;
                    };
                    let event = if !under_repair {
                        ship.repair_timer.map(|_| RepairEvent::TimerSet {
                            fleet: fleet_key,
                            ship_index,
                            timer: None,
                        })
                    } else {
                        ships_repaired += 1;
                        let rearmed = RepairTimer {
                            due: now + Self::interval(world, fast),
                            fast,
                        };
                        match ship.repair_timer {
                            // FUN_0053fa60: a change of the record arms the timer.
                            Some(timer) if timer.fast == fast && timer.due > now => None,
                            Some(timer) if timer.fast == fast => {
                                let hull_max = class.hull.cast_signed();
                                let (hull_after, counters_after) = Self::step(
                                    ship,
                                    hull_max,
                                    class.damage_control,
                                    draw(),
                                    draw(),
                                );
                                Some(RepairEvent::ShipRepaired {
                                    fleet: fleet_key,
                                    ship_index,
                                    hull_before: ship.hull_current,
                                    hull_after,
                                    counters_before: ship.damage_counters & 0x00ff_ffff,
                                    counters_after,
                                    timer: rearmed,
                                })
                            }
                            _ => Some(RepairEvent::TimerSet {
                                fleet: fleet_key,
                                ship_index,
                                timer: Some(rearmed),
                            }),
                        }
                    };
                    if let Some(event) = event {
                        let mut updated = ship.clone();
                        event.apply_to_ship(&mut updated);
                        changed.insert((fleet_key, ship_index), updated);
                        events.push(event);
                    }
                }

                if ships_repaired > 0 {
                    repairing_now.insert(fleet_key);
                    if state.active_fleets.insert(fleet_key) {
                        events.push(RepairEvent::RepairCheckPerformed {
                            system: fleet.location,
                            fleet: fleet_key,
                            ships_checked: ships_repaired,
                        });
                    }
                }
            }

            // A healthy fleet or a fleet that left port has ended its repair
            // episode. Later damage can therefore produce a fresh
            // repair-start notification.
            state
                .active_fleets
                .retain(|fleet| repairing_now.contains(fleet));
        }

        events
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::{ExplorationStatus, Faction};
    use crate::ids::DatId;
    use crate::world::*;

    fn make_shipyard_system(world: &mut GameWorld) -> SystemKey {
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Test Sector".into(),
            group: crate::dat::SectorGroup::Core,
            x: 100,
            y: 100,
            systems: vec![],
        });
        let mfg_key = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                // Family 0x28: a shipyard (FUN_00433e40).
                class_dat_id: DatId::new(0x2800_0000),
                side: crate::dat::Faction::Empire,
                is_shipyard: false,
            });

        world.systems.insert(System {
            dat_id: DatId::new(0x9000_0000),
            name: "Kuat".into(),
            sector,
            x: 100,
            y: 100,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.3,
            popularity_empire: 0.7,
            is_populated: true,
            total_energy: 10,
            raw_materials: 8,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![mfg_key],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        })
    }

    fn add_fleet_with_ship(world: &mut GameWorld, sys_key: SystemKey) -> FleetKey {
        let class_key = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x3000_0001),
            name: "Star Destroyer".into(),
            is_alliance: false,
            is_empire: true,
            refined_material_cost: 100,
            maintenance_cost: 10,
            research_order: 1,
            research_difficulty: 5,
            hull: 200,
            shield_strength: 100,
            sub_light_engine: 50,
            maneuverability: 30,
            hyperdrive: 40,
            fighter_capacity: 6,
            troop_capacity: 4,
            detection: 3,
            turbolaser_fore: 10,
            turbolaser_aft: 5,
            turbolaser_port: 8,
            turbolaser_starboard: 8,
            ion_cannon_fore: 5,
            ion_cannon_aft: 3,
            ion_cannon_port: 4,
            ion_cannon_starboard: 4,
            laser_cannon_fore: 3,
            laser_cannon_aft: 2,
            laser_cannon_port: 3,
            laser_cannon_starboard: 3,
            shield_recharge_rate: 5,
            damage_control: 10,
            bombardment_modifier: 50,
            ..Default::default()
        });
        let fleet_key = world.fleets.insert(Fleet {
            location: sys_key,
            capital_ships: ShipInstance::make(class_key, 200, false, 3),
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world
            .systems
            .get_mut(sys_key)
            .unwrap()
            .fleets
            .push(fleet_key);
        fleet_key
    }

    #[test]
    fn healthy_ships_do_not_emit_repair_started() {
        let mut world = GameWorld::default();
        let sys_key = make_shipyard_system(&mut world);
        add_fleet_with_ship(&mut world, sys_key);
        let mut state = RepairState::default();
        let tick_events = vec![crate::tick::TickEvent { tick: 1 }];

        let events = RepairSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &tick_events,
            &[],
        );
        assert!(
            events.is_empty(),
            "healthy ships must not start repair telemetry"
        );
    }

    #[test]
    fn a_ship_without_a_shipyard_repairs_every_thirty_days() {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Outer Rim".into(),
            group: crate::dat::SectorGroup::RimOuter,
            x: 500,
            y: 500,
            systems: vec![],
        });
        // System with NO manufacturing facilities
        let sys_key = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "Tatooine".into(),
            sector,
            x: 500,
            y: 500,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 3,
            raw_materials: 2,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![], // no shipyard!
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        });
        let fleet_key = add_fleet_with_ship(&mut world, sys_key);
        world.fleets.get_mut(fleet_key).unwrap().capital_ships[0].hull_current = 150;
        let mut state = RepairState::default();
        let tick_events = vec![crate::tick::TickEvent { tick: 1 }];

        // FUN_005010a0: with no shipyard of its side at hand the timer waits
        // GNPRTB 0x805 (30) days, then repairs; it is not barred.
        let first = RepairSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &tick_events,
            &[],
        );
        assert!(
            !first
                .iter()
                .any(|event| matches!(event, RepairEvent::ShipRepaired { .. })),
            "no repair step on the arming day"
        );
        for event in &first {
            event.apply(&mut world);
        }
        let due = world.fleets[fleet_key].capital_ships[0]
            .repair_timer
            .expect("timer armed")
            .due;
        assert_eq!(due, 31);
        let late = RepairSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &[crate::tick::TickEvent { tick: due }],
            &[1.0, 1.0],
        );
        assert!(late
            .iter()
            .any(|event| matches!(event, RepairEvent::ShipRepaired { .. })));
    }

    #[test]
    fn no_repair_without_ticks() {
        let mut world = GameWorld::default();
        let sys_key = make_shipyard_system(&mut world);
        let fleet_key = add_fleet_with_ship(&mut world, sys_key);
        world.fleets.get_mut(fleet_key).unwrap().capital_ships[0].hull_current = 150;
        let mut state = RepairState::default();

        let events = RepairSystem::advance(&mut state, &world, &UprisingState::default(), &[], &[]);
        assert!(events.is_empty(), "no repair without tick events");
    }

    #[test]
    fn no_repair_at_destroyed_system() {
        let mut world = GameWorld::default();
        let sys_key = make_shipyard_system(&mut world);
        let fleet_key = add_fleet_with_ship(&mut world, sys_key);
        world.fleets.get_mut(fleet_key).unwrap().capital_ships[0].hull_current = 150;
        world.systems.get_mut(sys_key).unwrap().is_destroyed = true;
        let mut state = RepairState::default();
        let tick_events = vec![crate::tick::TickEvent { tick: 1 }];

        let events = RepairSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &tick_events,
            &[],
        );
        assert!(events.is_empty(), "no repair at destroyed system");
    }

    #[test]
    fn damaged_ship_emits_ship_repaired() {
        let mut world = GameWorld::default();
        let sys_key = make_shipyard_system(&mut world);
        let fleet_key = add_fleet_with_ship(&mut world, sys_key);
        // Damage the first ship: hull 200 → 150
        world.fleets.get_mut(fleet_key).unwrap().capital_ships[0].hull_current = 150;
        let mut state = RepairState::default();
        let tick_events = vec![crate::tick::TickEvent { tick: 1 }];

        // FUN_005010a0: the arming day only sets the 2-day timer
        // (GNPRTB 0x804) of a ship at its own shipyard.
        let armed = RepairSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &tick_events,
            &[],
        );
        assert!(armed.iter().any(|event| matches!(
            event,
            RepairEvent::RepairCheckPerformed { system, fleet, ships_checked: 1 }
                if *system == sys_key && *fleet == fleet_key
        )));
        for event in &armed {
            event.apply(&mut world);
        }
        let events = RepairSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &[crate::tick::TickEvent { tick: 3 }],
            &[1.0, 1.0],
        );
        let repaired: Vec<_> = events
            .iter()
            .filter(|e| matches!(e, RepairEvent::ShipRepaired { .. }))
            .collect();
        assert_eq!(repaired.len(), 1, "one damaged ship fires one step");
        match repaired[0] {
            RepairEvent::ShipRepaired {
                fleet,
                ship_index,
                hull_before,
                hull_after,
                ..
            } => {
                assert_eq!(*fleet, fleet_key);
                assert_eq!(*ship_index, 0);
                assert_eq!(*hull_before, 150);
                // FUN_00501c50: a tenth of the maximum hull, 200 / 10.
                assert_eq!(*hull_after, 170);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn repair_started_is_emitted_once_per_continuous_episode() {
        let mut world = GameWorld::default();
        let sys_key = make_shipyard_system(&mut world);
        let fleet_key = add_fleet_with_ship(&mut world, sys_key);
        world.fleets.get_mut(fleet_key).unwrap().capital_ships[0].hull_current = 150;
        let mut state = RepairState::default();
        let tick_events = vec![crate::tick::TickEvent { tick: 1 }];

        let first = RepairSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &tick_events,
            &[],
        );
        assert!(first.iter().any(|event| matches!(
            event,
            RepairEvent::RepairCheckPerformed { fleet, .. } if *fleet == fleet_key
        )));
        assert!(state.is_repairing(fleet_key));
        for event in &first {
            event.apply(&mut world);
        }

        let encoded = serde_json::to_string(&state).expect("serialize repair state");
        let restored: RepairState =
            serde_json::from_str(&encoded).expect("deserialize repair state");
        assert!(restored.is_repairing(fleet_key));

        let second = RepairSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &[crate::tick::TickEvent { tick: 3 }],
            &[1.0, 1.0],
        );
        assert!(second.iter().any(|event| matches!(
            event,
            RepairEvent::ShipRepaired { fleet, .. } if *fleet == fleet_key
        )));
        assert!(
            !second.iter().any(|event| matches!(
                event,
                RepairEvent::RepairCheckPerformed { fleet, .. } if *fleet == fleet_key
            )),
            "ongoing work must not be reported as a second repair start"
        );

        world.fleets.get_mut(fleet_key).unwrap().capital_ships[0].hull_current = 200;
        let healthy = RepairSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &tick_events,
            &[],
        );
        assert!(!state.is_repairing(fleet_key));
        for event in &healthy {
            event.apply(&mut world);
        }

        world.fleets.get_mut(fleet_key).unwrap().capital_ships[0].hull_current = 180;
        let restarted = RepairSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &tick_events,
            &[],
        );
        assert!(
            restarted.iter().any(|event| matches!(
                event,
                RepairEvent::RepairCheckPerformed { fleet, .. } if *fleet == fleet_key
            )),
            "new damage after recovery must begin a new repair episode"
        );
    }
}
