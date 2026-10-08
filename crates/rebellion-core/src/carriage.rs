//! Which capital ship carries a fleet's squadrons, regiments and characters.
//!
//! The original's fleet holds only capital ships; squadrons (`FUN_005039d0`),
//! regiments (`FUN_00504c40`) and characters (`FUN_00536da0`) are children of
//! a ship (`ghidra/notes/fleet-window.md`, "Loading a regiment onto a
//! fleet"). The port keeps the rosters on the fleet and records each unit's
//! ship by its tag (`ShipInstance::tag`): `FighterEntry::carrier` and
//! `TroopTransportState`'s carriers. A tag goes with its ship through every
//! split, join and removal, so cargo whose tag is no longer a living ship of
//! the fleet was lost with that ship.

use crate::ids::{FighterKey, FleetKey};
use crate::troop_transport::TroopTransportState;
use crate::world::{CapitalShipClass, FighterEntry, Fleet, GameWorld};

/// Squadrons aboard the ship tagged `tag`.
#[must_use]
pub fn squadrons_aboard(fleet: &Fleet, tag: u32) -> u32 {
    if tag == 0 {
        return 0;
    }
    fleet
        .fighters
        .iter()
        .filter(|entry| entry.carrier == tag)
        .map(|entry| entry.count)
        .sum()
}

/// Of `fleet`'s living ships with room, the one with the most room left,
/// the earlier in the fleet on a tie: `FUN_00552300` walks the ships, and
/// `FUN_00552b10` keeps a candidate that `FUN_0054fbb0` finds less
/// constrained than the best so far. `room` is a ship's capacity less what
/// it carries (`FUN_00500b40`). `None` when no ship has room
/// (`FUN_00552300`, status `0x27`).
fn most_room(
    world: &GameWorld,
    fleet: FleetKey,
    room: impl Fn(&CapitalShipClass, u32) -> i64,
) -> Option<usize> {
    let value = world.fleets.get(fleet)?;
    let mut best: Option<(usize, i64)> = None;
    for (index, ship) in value.capital_ships.iter().enumerate() {
        if !ship.alive {
            continue;
        }
        let Some(class) = world.capital_ship_classes.get(ship.class) else {
            continue;
        };
        let left = room(class, ship.tag);
        if left > 0 && best.is_none_or(|(_, most)| left > most) {
            best = Some((index, left));
        }
    }
    best.map(|(index, _)| index)
}

/// The ship a squadron boards when it joins `fleet` ([`most_room`]; a
/// ship's room is its class's `+0x26c` less the squadrons aboard).
#[must_use]
pub fn ship_for_squadron(world: &GameWorld, fleet: FleetKey) -> Option<usize> {
    let value = world.fleets.get(fleet)?;
    most_room(world, fleet, |class, tag| {
        i64::from(class.fighter_capacity) - i64::from(squadrons_aboard(value, tag))
    })
}

/// The ship a regiment boards when it joins `fleet` ([`most_room`]; a
/// ship's room is its class's `+0x270` less the regiments aboard).
#[must_use]
pub fn ship_for_regiment(
    world: &GameWorld,
    transport: &TroopTransportState,
    fleet: FleetKey,
) -> Option<usize> {
    most_room(world, fleet, |class, tag| {
        let aboard = if tag == 0 {
            0
        } else {
            transport.regiments_aboard(fleet, tag)
        };
        i64::from(class.troop_capacity) - i64::try_from(aboard).unwrap_or(i64::MAX)
    })
}

/// The tag of the fleet's ship at `index`, giving it the next world tag if
/// it has none.
pub fn tag_ship(world: &mut GameWorld, fleet: FleetKey, index: usize) -> Option<u32> {
    let next = world.last_ship_tag + 1;
    let ship = world.fleets.get_mut(fleet)?.capital_ships.get_mut(index)?;
    if ship.tag == 0 {
        ship.tag = next;
        world.last_ship_tag = next;
    }
    Some(ship.tag)
}

/// Put `count` squadrons of `class` aboard `fleet`, one at a time, each on
/// the ship with the most room ([`ship_for_squadron`]). A squadron no ship
/// has room for stays on the fleet (port: the original refuses the move).
pub fn board_squadrons(world: &mut GameWorld, fleet: FleetKey, class: FighterKey, count: u32) {
    for _ in 0..count {
        let carrier = ship_for_squadron(world, fleet)
            .and_then(|index| tag_ship(world, fleet, index))
            .unwrap_or(0);
        let Some(value) = world.fleets.get_mut(fleet) else {
            return;
        };
        if let Some(entry) = value
            .fighters
            .iter_mut()
            .find(|entry| entry.class == class && entry.carrier == carrier)
        {
            entry.count += 1;
        } else {
            value.fighters.push(FighterEntry {
                class,
                count: 1,
                carrier,
            });
        }
    }
}

/// Put the fleet's squadrons held on the fleet itself (carrier 0) aboard
/// its ships ([`board_squadrons`]), in roster order.
pub fn board_held_squadrons(world: &mut GameWorld, fleet: FleetKey) {
    let Some(value) = world.fleets.get_mut(fleet) else {
        return;
    };
    let mut held = Vec::new();
    value.fighters.retain(|entry| {
        if entry.carrier == 0 {
            held.push((entry.class, entry.count));
        }
        entry.carrier != 0
    });
    for (class, count) in held {
        board_squadrons(world, fleet, class, count);
    }
}

/// Whether `tag` names a living ship of `fleet`.
#[must_use]
pub fn carried_by_living_ship(fleet: &Fleet, tag: u32) -> bool {
    tag != 0
        && fleet
            .capital_ships
            .iter()
            .any(|ship| ship.alive && ship.tag == tag)
}

/// Drop the squadrons whose ship was lost or left the fleet. hyp: they are
/// destroyed with their ship, as a destroyed container destroys what it
/// holds (`FUN_004f84e0`; `ghidra/notes/mission-lifecycle.md` leaves cargo
/// aboard a destroyed hull open). Returns the squadrons lost.
pub fn drop_lost_squadrons(fleet: &mut Fleet) -> u32 {
    let mut lost = 0;
    let living: Vec<u32> = fleet
        .capital_ships
        .iter()
        .filter(|ship| ship.alive)
        .map(|ship| ship.tag)
        .collect();
    fleet.fighters.retain(|entry| {
        let kept = entry.carrier == 0 || living.contains(&entry.carrier);
        if !kept {
            lost += entry.count;
        }
        kept
    });
    lost
}

/// The ship a fleet's characters ride aboard: its first living ship.
/// Characters have unlimited room (`FUN_00500b40`), so every candidate ties
/// and the fleet's first accepting ship wins (`FUN_00552b10`). port: the
/// port keeps no character's ship, so a character whose ship is lost moves
/// to the next one.
#[must_use]
pub fn character_ship(fleet: &Fleet) -> Option<usize> {
    fleet.capital_ships.iter().position(|ship| ship.alive)
}

/// The Fleet window's indicator flags for the fleet's ship at `index`
/// (`FUN_004a66a0`): 1 squadrons, 2 regiments, 4 characters aboard, `0x40`
/// no hyperdrive.
#[must_use]
pub fn ship_flags(
    world: &GameWorld,
    transport: &TroopTransportState,
    fleet: FleetKey,
    index: usize,
) -> u8 {
    let Some(value) = world.fleets.get(fleet) else {
        return 0;
    };
    let Some(ship) = value.capital_ships.get(index).filter(|ship| ship.alive) else {
        return 0;
    };
    let mut flags = 0;
    if squadrons_aboard(value, ship.tag) > 0 {
        flags |= 1;
    }
    if ship.tag != 0 && transport.regiments_aboard(fleet, ship.tag) > 0 {
        flags |= 2;
    }
    if !value.characters.is_empty() && character_ship(value) == Some(index) {
        flags |= 4;
    }
    if world
        .capital_ship_classes
        .get(ship.class)
        .is_some_and(|class| class.hyperdrive == 0)
    {
        flags |= 0x40;
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{CapitalShipClass, ShipInstance};

    /// A fleet of ships whose classes hold `capacities` squadrons each.
    fn fleet(world: &mut GameWorld, capacities: &[u32]) -> FleetKey {
        let ships = capacities
            .iter()
            .map(|&fighter_capacity| {
                let class = world.capital_ship_classes.insert(CapitalShipClass {
                    fighter_capacity,
                    ..CapitalShipClass::default()
                });
                ShipInstance::new(class, 10, true)
            })
            .collect();
        world.fleets.insert(Fleet {
            location: Default::default(),
            capital_ships: ships,
            fighters: Vec::new(),
            characters: Vec::new(),
            is_alliance: true,
            has_death_star: false,
        })
    }

    fn carriers(world: &GameWorld, fleet: FleetKey) -> Vec<(u32, u32)> {
        world.fleets[fleet]
            .fighters
            .iter()
            .map(|entry| (entry.carrier, entry.count))
            .collect()
    }

    // FUN_00552b10 keeps the candidate FUN_0054fbb0 finds less constrained:
    // the most room wins, the earlier ship on a tie.
    #[test]
    fn a_squadron_boards_the_ship_with_the_most_room() {
        let mut world = GameWorld::default();
        let key = fleet(&mut world, &[2, 3]);
        let class = world.fighter_classes.insert(Default::default());
        assert_eq!(ship_for_squadron(&world, key), Some(1));
        board_squadrons(&mut world, key, class, 1);
        assert_eq!(ship_for_squadron(&world, key), Some(0));
        board_squadrons(&mut world, key, class, 2);
        let tags: Vec<u32> = world.fleets[key]
            .capital_ships
            .iter()
            .map(|ship| ship.tag)
            .collect();
        assert_eq!(carriers(&world, key), [(tags[1], 2), (tags[0], 1)]);
    }

    // FUN_00552300 refuses with 0x27 when no ship has room. port: a
    // squadron placed anyway stays on the fleet.
    #[test]
    fn a_squadron_with_no_room_aboard_stays_on_the_fleet() {
        let mut world = GameWorld::default();
        let key = fleet(&mut world, &[1, 0]);
        let class = world.fighter_classes.insert(Default::default());
        board_squadrons(&mut world, key, class, 2);
        let tag = world.fleets[key].capital_ships[0].tag;
        assert_eq!(carriers(&world, key), [(tag, 1), (0, 1)]);
        assert_eq!(world.fleets[key].capital_ships[1].tag, 0);
    }

    // hyp: squadrons go with a lost ship; those held on the fleet stay.
    #[test]
    fn squadrons_aboard_a_lost_ship_are_lost_with_it() {
        let mut world = GameWorld::default();
        let key = fleet(&mut world, &[1, 1]);
        let class = world.fighter_classes.insert(Default::default());
        board_squadrons(&mut world, key, class, 3);
        world.fleets[key].capital_ships[0].alive = false;
        assert_eq!(drop_lost_squadrons(&mut world.fleets[key]), 1);
        let tag = world.fleets[key].capital_ships[1].tag;
        assert_eq!(carriers(&world, key), [(tag, 1), (0, 1)]);
    }

    // FUN_00500b40: characters have unlimited room, so the first living
    // ship takes them.
    #[test]
    fn characters_ride_the_first_living_ship() {
        let mut world = GameWorld::default();
        let key = fleet(&mut world, &[0, 0, 0]);
        assert_eq!(character_ship(&world.fleets[key]), Some(0));
        world.fleets[key].capital_ships[0].alive = false;
        assert_eq!(character_ship(&world.fleets[key]), Some(1));
    }
}
