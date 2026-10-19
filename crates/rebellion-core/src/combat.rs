//! The battle types shared with the tactical engine (`side_present`,
//! `CombatForce`, `CombatSide`).
//!
//! Space battles run through `crate::tactical` (played, simulated, or
//! headless when nobody watches; `ghidra/notes/simulate-runs-tactical-engine.md`).
//! Ground combat is the Planetary Assault order's run (`crate::assault`,
//! `ghidra/notes/ground-assault.md`).

use serde::{Deserialize, Serialize};

use crate::ids::{FleetKey, SystemKey};
use crate::world::GameWorld;

// ---------------------------------------------------------------------------
// Public result types
// ---------------------------------------------------------------------------

/// Whether a side is present at `system` for battle (`FUN_00509710`): it has
/// a usable fleet there (mode 1, [`Fleet::usable`](crate::world::Fleet::usable)),
/// or squadrons the system holds for it. Defense facilities do not count.
/// `FUN_0050b8e0` starts a battle when both sides are present
/// (`ghidra/notes/system-squadrons-combat.md`).
#[must_use]
pub fn side_present(world: &GameWorld, system: SystemKey, is_alliance: bool) -> bool {
    world
        .systems
        .get(system)
        .is_some_and(|value| side_present_in(world, value, is_alliance))
}

/// [`side_present`] for a system already in hand.
#[must_use]
pub fn side_present_in(world: &GameWorld, system: &crate::world::System, is_alliance: bool) -> bool {
    system.squadron_count(is_alliance) > 0
        || system.fleets.iter().any(|&fleet| {
            world
                .fleets
                .get(fleet)
                .is_some_and(|fleet| fleet.is_alliance == is_alliance && fleet.usable())
        })
}

/// The side's fleets at `system` that fight, in container order: the
/// usable ones (`FUN_004ffef0(system, 1, side)`, the mode-1 walk of the
/// Commander field `FUN_00509530` and the post-battle pass
/// `FUN_00516ef0`). `FUN_0040b0e0` exports only a fleet's usable children,
/// so a fleet holding only ships on their own legs puts nothing in the
/// battle. port: the original has no lead fleet; the port's lead is the
/// first of these.
#[must_use]
pub fn fighting_fleets(world: &GameWorld, system: SystemKey, is_alliance: bool) -> Vec<FleetKey> {
    world.systems.get(system).map_or_else(Vec::new, |value| {
        value
            .fleets
            .iter()
            .copied()
            .filter(|&fleet| {
                world
                    .fleets
                    .get(fleet)
                    .is_some_and(|fleet| fleet.is_alliance == is_alliance && fleet.usable())
            })
            .collect()
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // FUN_00509710 and FUN_0050b8e0 read a system's fleets in mode 1
    // (FUN_004ffef0, `+0x50` bit 0), and the fleet override FUN_004fe540
    // makes a fleet usable only while one of its capital ships is completed
    // and not in transit; a ship on its own leg (bit 5, FUN_00556430) is not.
    #[test]
    fn a_fleet_holding_only_a_ship_on_its_own_leg_is_not_present_for_either_side() {
        use crate::blockade::BlockadeSystem;
        use crate::world::{
            CapitalShipClass, ControlKind, Fleet, ShipInstance, ShipTransit, System,
        };
        for is_alliance in [true, false] {
            let holder = crate::dat::Faction::of_alliance(!is_alliance);
            let mut world = GameWorld::default();
            let origin = world.systems.insert(System::default());
            let system = world.systems.insert(System {
                control: ControlKind::Controlled(holder),
                is_populated: true,
                ..System::default()
            });
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..CapitalShipClass::default()
            });
            let mut ship = ShipInstance::new(class, 100, is_alliance);
            ship.transit = Some(ShipTransit {
                from: origin,
                to: system,
                arrival_tick: 9,
            });
            let fleet = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ship],
                is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets.push(fleet);
            let defender = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, !is_alliance)],
                is_alliance: !is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets.push(defender);
            assert!(!side_present(&world, system, is_alliance), "{is_alliance}");
            assert!(!BlockadeSystem::battle_bit(&world, system), "{is_alliance}");

            world.systems[system].fleets.retain(|&key| key != defender);
            assert!(
                !BlockadeSystem::system_is_blockaded(&world, &world.systems[system]),
                "{is_alliance}"
            );

            world.fleets[fleet].capital_ships[0].transit = None;
            assert!(side_present(&world, system, is_alliance), "{is_alliance}");
            assert!(
                BlockadeSystem::system_is_blockaded(&world, &world.systems[system]),
                "{is_alliance}"
            );
        }
    }

    // FUN_00509710 and FUN_0050b8e0 read a system's fleets in mode 1, and
    // FUN_004fe540.c:20-35 makes a fleet usable only through a completed
    // ship. An ordered ship sits in its destination's fleet uncompleted
    // from the order (FUN_004f7d50(.., 1, 0); the route-1 day-115 save:
    // ship +0x50 0x42 in a fleet), so it neither fights nor blockades.
    #[test]
    fn a_fleet_holding_only_an_unfinished_ship_is_not_present_for_either_side() {
        use crate::blockade::BlockadeSystem;
        use crate::world::{CapitalShipClass, ControlKind, Fleet, ShipInstance, System};
        for is_alliance in [true, false] {
            let holder = crate::dat::Faction::of_alliance(!is_alliance);
            let mut world = GameWorld::default();
            let system = world.systems.insert(System {
                control: ControlKind::Controlled(holder),
                is_populated: true,
                ..System::default()
            });
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..CapitalShipClass::default()
            });
            let fleet = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::ordered(class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets.push(fleet);
            let defender = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, !is_alliance)],
                is_alliance: !is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets.push(defender);
            assert!(!side_present(&world, system, is_alliance), "{is_alliance}");
            assert!(
                fighting_fleets(&world, system, is_alliance).is_empty(),
                "{is_alliance}"
            );
            assert!(!BlockadeSystem::battle_bit(&world, system), "{is_alliance}");

            world.systems[system].fleets.retain(|&key| key != defender);
            assert!(
                !BlockadeSystem::system_is_blockaded(&world, &world.systems[system]),
                "{is_alliance}"
            );

            // Completed in place (FUN_0052bee0): the same fleet is present.
            world.fleets[fleet].capital_ships[0].completed = true;
            assert!(side_present(&world, system, is_alliance), "{is_alliance}");
            assert!(
                BlockadeSystem::system_is_blockaded(&world, &world.systems[system]),
                "{is_alliance}"
            );
        }
    }

    // FUN_0040b0e0 exports only a fleet's usable children, and the
    // Commander field FUN_00509530 and the post-battle pass FUN_00516ef0
    // walk the side's fleets in mode 1 (FUN_004ffef0): a fleet listed first
    // whose only ship is on its own leg neither fights nor leads.
    #[test]
    fn a_fleet_whose_only_ship_is_on_its_own_leg_neither_fights_nor_leads_for_either_side() {
        use crate::world::{CapitalShipClass, Fleet, ShipInstance, ShipTransit, System};
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let yard = world.systems.insert(System::default());
            let system = world.systems.insert(System::default());
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..CapitalShipClass::default()
            });
            let mut travelling = ShipInstance::new(class, 100, is_alliance);
            travelling.transit = Some(ShipTransit {
                from: yard,
                to: system,
                arrival_tick: 9,
            });
            let new_hull = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![travelling],
                is_alliance,
                ..Fleet::default()
            });
            let standing = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            });
            let enemy = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, !is_alliance)],
                is_alliance: !is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets = vec![new_hull, enemy, standing];

            assert!(side_present(&world, system, is_alliance), "{is_alliance}");
            assert_eq!(
                fighting_fleets(&world, system, is_alliance),
                vec![standing],
                "{is_alliance}"
            );
            assert_eq!(fighting_fleets(&world, system, !is_alliance), vec![enemy]);
        }
    }

    // The Commander field (FUN_0040b0e0.c:86-118) is FUN_00509530's, which
    // walks the Alliance's fleets in mode 1 (FUN_004ffef0(system, 1, 1)):
    // a Commander aboard a fleet whose only ship is on its own leg does not
    // rate the battle, whichever side attacks.
    #[test]
    fn a_commander_aboard_a_fleet_whose_only_ship_is_on_its_own_leg_does_not_rate_the_battle() {
        use crate::officer_rank::CommandKind;
        use crate::tactical::BattleSession;
        use crate::world::{
            CapitalShipClass, Character, Fleet, ShipInstance, ShipTransit, SkillPair, System,
        };
        for attacker_is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let yard = world.systems.insert(System::default());
            let system = world.systems.insert(System::default());
            let fleet_of = |world: &mut GameWorld, is_alliance: bool| {
                let class = world.capital_ship_classes.insert(CapitalShipClass {
                    is_alliance,
                    is_empire: !is_alliance,
                    hull: 100,
                    ..CapitalShipClass::default()
                });
                let fleet = world.fleets.insert(Fleet {
                    location: system,
                    capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
                    is_alliance,
                    ..Fleet::default()
                });
                world.systems[system].fleets.push(fleet);
                fleet
            };
            let new_hull = fleet_of(&mut world, true);
            world.fleets[new_hull].capital_ships[0].transit = Some(ShipTransit {
                from: yard,
                to: system,
                arrival_tick: 9,
            });
            let standing = fleet_of(&mut world, true);
            let enemy = fleet_of(&mut world, false);
            for (fleet, combat) in [(new_hull, 150), (standing, 50)] {
                world.characters.insert(Character {
                    is_alliance: true,
                    can_be_commander: true,
                    command_kind: CommandKind::Commander,
                    current_fleet: Some(fleet),
                    combat: SkillPair {
                        base: combat,
                        variance: 0,
                    },
                    ..Character::default()
                });
            }
            let (attacker, defender) = if attacker_is_alliance {
                (standing, enemy)
            } else {
                (enemy, standing)
            };
            let session = BattleSession::new(&world, system, attacker, defender, true, 0, 1);

            let commander = session.trench_run_commander.unwrap();
            assert_eq!(
                (commander.fleet, commander.combat),
                (Some(standing), 50),
                "{attacker_is_alliance}"
            );
        }
    }

    // FUN_004fe540 walks the fleet's existing capital ships (FUN_00502e30,
    // mode 3) and sets usable only when one is completed (bit 2) and not in
    // transit (bit 5); a fleet with none is not usable, so it is absent from
    // the mode-1 presence walks (FUN_00509710, FUN_0050b8e0). Squadrons
    // never stand in for a ship (FUN_004fe160 admits only capital ships).
    #[test]
    fn a_fleet_without_a_usable_capital_ship_is_not_present_for_either_side() {
        use crate::blockade::BlockadeSystem;
        use crate::world::{
            CapitalShipClass, ControlKind, FighterClass, FighterEntry, Fleet, ShipInstance,
            System,
        };
        for is_alliance in [true, false] {
            let holder = crate::dat::Faction::of_alliance(!is_alliance);
            let mut world = GameWorld::default();
            let system = world.systems.insert(System {
                control: ControlKind::Controlled(holder),
                is_populated: true,
                ..System::default()
            });
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..CapitalShipClass::default()
            });
            let fighter = world.fighter_classes.insert(FighterClass::default());
            let mut destroyed = ShipInstance::new(class, 0, is_alliance);
            destroyed.alive = false;
            let fleet = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![destroyed],
                fighters: vec![FighterEntry {
                    class: fighter,
                    count: 3,
                    carrier: 0,
                }],
                is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets.push(fleet);
            let defender = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, !is_alliance)],
                is_alliance: !is_alliance,
                ..Fleet::default()
            });
            world.systems[system].fleets.push(defender);
            assert!(!side_present(&world, system, is_alliance), "{is_alliance}");
            assert!(!BlockadeSystem::battle_bit(&world, system), "{is_alliance}");

            world.systems[system].fleets.retain(|&key| key != defender);
            assert!(
                !BlockadeSystem::system_is_blockaded(&world, &world.systems[system]),
                "{is_alliance}"
            );

            world.fleets[fleet]
                .capital_ships
                .push(ShipInstance::new(class, 100, is_alliance));
            assert!(side_present(&world, system, is_alliance), "{is_alliance}");
            assert!(
                BlockadeSystem::system_is_blockaded(&world, &world.systems[system]),
                "{is_alliance}"
            );
        }
    }

    // FUN_00509710 counts a side present by a usable fleet or by the
    // squadrons the system holds for it (FUN_00503a50): squadrons alone
    // make a side present, and the other side's squadrons do not.
    #[test]
    fn a_side_holding_only_system_squadrons_is_present() {
        use crate::world::{FighterClass, System};
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let system = world.systems.insert(System::default());
            assert!(!side_present(&world, system, is_alliance));
            let class = world.fighter_classes.insert(FighterClass::default());
            world.systems[system].add_squadrons(class, 1, is_alliance);
            assert!(side_present(&world, system, is_alliance), "{is_alliance}");
            assert!(!side_present(&world, system, !is_alliance), "{is_alliance}");
        }
    }
}
