//! Carrier and station hangars: docking at the start, launches, retreat
//! docking and the squadrons' carriers after the battle.

use super::test_support::{test_death_star, test_fighter, test_session, test_ship};
use super::*;
use crate::carriage::ProductRoom;
use crate::world::{FighterEntry, Fleet, ShipInstance, System};

/// A battle with a carrier of each side (`is_alliance` true is the
/// attacker): ship 0 the Alliance's, ship 1 the Empire's, each holding
/// `capacity` squadrons, and `squadrons` docked-ready fighters per side.
/// port: test fixture values, not rules.
fn carrier_battle(capacity: u32, squadrons: usize, player_is_attacker: bool) -> BattleSession {
    let mut ships = vec![test_ship(0x1400_0001, 0, true, true), test_ship(0x1400_0002, 0, false, true)];
    for ship in &mut ships {
        ship.fighter_capacity = capacity;
    }
    ships[0].source_position = TacticalWorldPosition { x: -10.0, y: 0.0, z: 5.0 };
    ships[1].source_position = TacticalWorldPosition { x: 10.0, y: 0.0, z: -5.0 };
    let mut fighters = Vec::new();
    for (carrier, is_alliance) in [(0, true), (1, false)] {
        for _ in 0..squadrons {
            let mut fighter = test_fighter(if is_alliance { 3 } else { 5 }, is_alliance);
            fighter.carrier = Some(TacticalBerth::Ship(carrier));
            fighter.docked = Some(TacticalDock::Ready);
            fighter.alive = false;
            fighters.push(fighter);
        }
    }
    let mut session = test_session(ships, fighters, player_is_attacker);
    session.hangars.step = 3;
    session
}

/// A world with one fleet per side at one system: each fleet's ship 0
/// (tag 1 or 2) carries one squadron of a class without a hyperdrive and
/// one of a class with one; the Alliance fleet also holds one whose ship,
/// tag 9, is not in the battle, so its carrier does not resolve.
/// port: test fixture values, not rules.
pub(super) fn carried_world() -> (GameWorld, SystemKey) {
    let mut world = GameWorld::default();
    let system = world.systems.insert(System::default());
    for (is_alliance, tag) in [(true, 1), (false, 2)] {
        let ship_class = world.capital_ship_classes.insert(crate::world::CapitalShipClass {
            dat_id: DatId::new(0x1400_0001 + tag),
            family: 0x14,
            is_alliance,
            is_empire: !is_alliance,
            hull: 100,
            fighter_capacity: 4,
            ..crate::world::CapitalShipClass::default()
        });
        let grounded = world.fighter_classes.insert(crate::world::FighterClass {
            dat_id: DatId::new(0x1c00_0001 + tag),
            squadron_size: 6,
            is_alliance,
            ..crate::world::FighterClass::default()
        });
        let jumping = world.fighter_classes.insert(crate::world::FighterClass {
            dat_id: DatId::new(0x1c00_0010 + tag),
            squadron_size: 6,
            hyperdrive: 1,
            is_alliance,
            ..crate::world::FighterClass::default()
        });
        let mut ship = ShipInstance::new(ship_class, 100, is_alliance);
        ship.tag = tag;
        let mut fighters = vec![
            FighterEntry {
                class: grounded,
                count: 1,
                carrier: tag,
            },
            FighterEntry {
                class: jumping,
                count: 1,
                carrier: tag,
            },
        ];
        if is_alliance {
            fighters.push(FighterEntry {
                class: grounded,
                count: 1,
                carrier: 9,
            });
        }
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ship],
            fighters,
            is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);
    }
    world.last_ship_tag = 2;
    (world, system)
}

pub(super) fn battle_of(world: &GameWorld, system: SystemKey) -> BattleSession {
    let fleets: Vec<FleetKey> = world.systems[system].fleets.clone();
    BattleSession::new(world, system, fleets[0], fleets[1], true, 0, 1)
}

// FUN_005964c0.c:133-219: a fleet's squadron without a working hyperdrive
// (FUN_005b1b70) starts docked and ready (state 1) in its carrier's hangar
// (FUN_005a6360 resolves the carrier id +0x164); one with a hyperdrive, or
// one whose carrier does not resolve and with no station, starts in flight.
// The hangars count launch steps from 3 (battle+0x8ac, FUN_005be160 at
// 0x5be233).
#[test]
fn a_squadron_without_a_hyperdrive_starts_docked_in_its_carrier_for_either_side() {
    let (world, system) = carried_world();
    let session = battle_of(&world, system);
    assert_eq!(session.hangars.step, 3);
    // FUN_0040be90 passes the squadrons whose class has a hyperdrive to the
    // morale constructor (FUN_005b1bc0, +0x3ec).
    let mut jumping: Vec<usize> = [true, false]
        .into_iter()
        .map(|is_alliance| {
            (0..session.fighters.len())
                .filter(|&index| session.fighters[index].identity.is_alliance == is_alliance)
                .nth(1)
                .unwrap()
        })
        .collect();
    jumping.sort_unstable();
    assert_eq!(session.morale.hyperdrive_squadrons, jumping);
    // A damaged-hyperdrive rating alone counts too (morale-retreat.md, the
    // fighter factory FUN_0040be90): the Empire's jumping class keeps only it.
    let mut damaged = world.clone();
    let empire_jumping = damaged
        .fighter_classes
        .values_mut()
        .find(|class| class.dat_id == DatId::new(0x1c00_0012))
        .unwrap();
    empire_jumping.hyperdrive = 0;
    empire_jumping.hyperdrive_if_damaged = 1;
    assert_eq!(battle_of(&damaged, system).morale.hyperdrive_squadrons, jumping);
    for is_alliance in [true, false] {
        let ship = session
            .ships
            .iter()
            .position(|ship| ship.identity.is_alliance == is_alliance)
            .unwrap();
        let squadrons: Vec<&TacticalFighter> = session
            .fighters
            .iter()
            .filter(|fighter| fighter.identity.is_alliance == is_alliance)
            .collect();
        assert_eq!(squadrons[0].carrier, Some(TacticalBerth::Ship(ship)));
        assert_eq!(squadrons[0].docked, Some(TacticalDock::Ready));
        assert!(!squadrons[0].alive && !squadrons[0].listed());
        assert_eq!(squadrons[1].carrier, Some(TacticalBerth::Ship(ship)));
        assert_eq!(squadrons[1].docked, None, "a hyperdrive squadron flies");
        assert!(squadrons[1].alive);
    }
    let held = session
        .fighters
        .iter()
        .filter(|fighter| fighter.identity.is_alliance)
        .nth(2)
        .unwrap();
    assert_eq!(
        (held.carrier, held.docked),
        (None, None),
        "no carrier resolves"
    );
}

// FUN_005ab650.c:148-153: a squadron joins its side's placement list
// (+0x64) only when FUN_005b69f0 (+0x170 == 1, set by FUN_005b69d0) says
// it is not docked, so the placement loops (:254, :275, FUN_005a9030) give a
// docked one no X-slot and no position; :196-210 grow the battle extent only
// for objects that are not docked.
#[test]
fn docked_squadrons_take_no_slot_and_no_share_of_the_battle_extent_for_either_side() {
    let (world, system) = carried_world();
    let session = battle_of(&world, system);
    // Alliance: its carrier, the jumping squadron and the unresolved one;
    // Empire: its carrier and the jumping squadron.
    assert_eq!(session.source_layout.first_active_objects, 3);
    assert_eq!(session.source_layout.second_active_objects, 2);
    assert_eq!(session.source_layout.battle_extent, 109.0);
    for is_alliance in [true, false] {
        let squadrons: Vec<&TacticalFighter> = session
            .fighters
            .iter()
            .filter(|fighter| fighter.identity.is_alliance == is_alliance)
            .collect();
        assert_eq!(squadrons[0].docked, Some(TacticalDock::Ready));
        assert_eq!(squadrons[0].source_position, TacticalWorldPosition::ORIGIN);
        let lane = if is_alliance {
            session.source_layout.inner_negative_z
        } else {
            session.source_layout.inner_positive_z
        };
        assert_eq!(
            squadrons[1].source_position,
            TacticalWorldPosition { x: 0.0, y: 0.0, z: lane },
            "the first flying squadron takes the first slot"
        );
    }
}

// FUN_005ab650.c:297-320 (Alliance) and :383-406 (Empire) count the fighter
// types over the all-object list mgr+0x9a0, and FUN_005adfa0.c:22-26 groups
// every object of a type, with no docked test: a docked squadron is grouped.
#[test]
fn docked_squadrons_join_their_classes_fighter_group_for_either_side() {
    let (world, system) = carried_world();
    let session = battle_of(&world, system);
    for (is_alliance, group) in [(true, 3), (false, 0)] {
        let docked = session
            .fighters
            .iter()
            .find(|fighter| fighter.identity.is_alliance == is_alliance)
            .unwrap();
        assert_eq!(docked.docked, Some(TacticalDock::Ready));
        assert_eq!(docked.fighter_group, group);
    }
}

// FUN_005a6360.c:15-31: an unresolved carrier id falls back to the station's
// hangar, whichever side the squadron is on; FUN_005b8570.c:13 docks only
// while the hangar's reserved count is below its capacity.
#[test]
fn an_unresolved_carrier_is_the_station_and_a_full_hangar_never_launches() {
    let (world, system) = carried_world();
    let mut ships = Vec::new();
    let mut fighters: Vec<TacticalFighter> = (0..3)
        .map(|index| {
            let mut fighter = test_fighter(3, true);
            fighter.fleet = Some(world.systems[system].fleets[0]);
            fighter.fleet_fighter_index = 2;
            fighter.fleet_squadron_index = index;
            fighter
        })
        .collect();
    BattleSession::seat_carried_squadrons(&world, &ships, &mut fighters, &[], Some(2));
    assert!(fighters
        .iter()
        .all(|fighter| fighter.carrier == Some(TacticalBerth::Station)));
    assert_eq!(fighters[0].docked, Some(TacticalDock::Ready));
    assert_eq!(fighters[1].docked, Some(TacticalDock::Ready));
    assert_eq!(fighters[2].docked, Some(TacticalDock::Recovered), "no room left");

    ships.push(test_ship(0x1400_0001, 0, true, true));
    let mut fighters = vec![test_fighter(3, true)];
    fighters[0].fleet = Some(world.systems[system].fleets[0]);
    fighters[0].fleet_fighter_index = 2;
    BattleSession::seat_carried_squadrons(&world, &ships, &mut fighters, &[], None);
    assert_eq!(fighters[0].docked, None, "no station: it flies");
}

// FUN_005b8800.c:23-72: once per 1000 ms step an unarmed hangar arms at
// 5.0 + rand(0,8) + the step (battle+0x87c, from 3); when the step reaches
// it, the front squadron launches (FUN_005b8c70, state 2) and the hangar
// re-arms.
#[test]
fn a_carrier_launches_its_front_squadron_when_the_step_reaches_its_armed_time() {
    let mut session = carrier_battle(4, 2, true);
    let mut oracle = session.tactical_rng;
    session.advance_hangars(999.0);
    assert_eq!(session.hangars.step, 3, "no whole step yet");
    session.advance_hangars(1.0);
    let alliance_armed = 5.0 + oracle.inclusive(0, 8) as f32 + 3.0;
    let empire_armed = 5.0 + oracle.inclusive(0, 8) as f32 + 3.0;
    assert_eq!(session.hangars.ships, vec![alliance_armed, empire_armed]);
    assert_eq!(session.hangars.step, 4);
    assert!(session.fighters.iter().all(|fighter| fighter.docked.is_some()));

    while (session.hangars.step as f32) < alliance_armed {
        session.advance_hangars(1000.0);
        assert_eq!(session.fighters[0].docked, Some(TacticalDock::Ready));
    }
    let launch_step = session.hangars.step;
    let rng = session.tactical_rng;
    session.advance_hangars(1000.0);
    assert_eq!(session.fighters[0].docked, None);
    assert!(session.fighters[0].alive && session.fighters[0].listed());
    assert_eq!(session.fighters[0].source_position, session.ships[0].source_position);
    assert_eq!(session.fighters[1].docked, Some(TacticalDock::Ready), "one per expiry");
    let mut oracle = rng;
    let rearmed = 5.0 + oracle.inclusive(0, 8) as f32 + launch_step as f32;
    assert_eq!(session.hangars.ships[0], rearmed);
}

// FUN_005964c0.c:149-153 -> FUN_005b8570.c:13-30: in a battle with its
// window (engine +0x668 == 0) a squadron docking into an unarmed hangar's
// launch list arms it at 5.0 + rand(0,8) (FUN_005a8a70 on the CRT rand,
// FUN_0061a310) + battle+0x87c, which the battle constructor sets to 3
// (FUN_005a7500.c:551); an armed hangar, or a full one (+0xc not below
// +0x10), draws nothing.
#[test]
fn taking_command_arms_each_hangar_a_squadron_docked_in_from_step_three_for_either_side() {
    let (world, system) = carried_world();
    let mut session = battle_of(&world, system);
    let mut oracle = session.tactical_rng;
    session.arm_docked_hangars();
    let alliance = 5.0 + oracle.inclusive(0, 8) as f32 + 3.0;
    let empire = 5.0 + oracle.inclusive(0, 8) as f32 + 3.0;
    assert_eq!(session.hangars.ships, vec![alliance, empire]);
    assert_eq!(session.tactical_rng, oracle);
    session.arm_docked_hangars();
    assert_eq!(session.tactical_rng, oracle, "both hangars are armed");

    // The station's hangar arms once for squadrons of either side; a
    // squadron kept out of a full hangar arms nothing.
    let docked = |is_alliance: bool, berth: TacticalBerth, dock: TacticalDock| {
        let mut fighter = test_fighter(if is_alliance { 3 } else { 5 }, is_alliance);
        fighter.carrier = Some(berth);
        fighter.docked = Some(dock);
        fighter.alive = false;
        fighter
    };
    let mut session = test_session(
        vec![test_ship(0x1400_0001, 0, true, true)],
        vec![
            docked(true, TacticalBerth::Ship(0), TacticalDock::Recovered),
            docked(true, TacticalBerth::Station, TacticalDock::Ready),
            docked(false, TacticalBerth::Station, TacticalDock::Ready),
        ],
        false,
    );
    session.death_star = Some(test_death_star(false));
    session.hangars.step = 3;
    let mut oracle = session.tactical_rng;
    session.arm_docked_hangars();
    assert_eq!(session.hangars.station, 5.0 + oracle.inclusive(0, 8) as f32 + 3.0);
    assert_eq!(session.hangars.ships, vec![0.0], "a full hangar's squadron arms nothing");
    assert_eq!(session.tactical_rng, oracle);
}

// FUN_005bea30.c:33,121-125: a step runs every 1000 ms the session has
// gathered, the remainder kept for the next. FUN_005a9de0.c:34-36 runs only
// the hangar of a capital ship still in the battle, and FUN_005b8800.c:23-27
// launches only from that hangar's own list.
#[test]
fn each_carrier_launches_only_its_own_squadrons_while_it_stays_in_the_battle() {
    for is_alliance in [true, false] {
        let mut session = carrier_battle(4, 1, is_alliance);
        let (own, other) = if is_alliance { (0, 1) } else { (1, 0) };
        // Only the other carrier holds a squadron; the own carrier's hangar
        // is armed and due.
        session
            .fighters
            .retain(|fighter| fighter.carrier == Some(TacticalBerth::Ship(other)));
        session.hangars.ships = vec![0.0; 2];
        session.hangars.ships[own] = 3.0;
        session.ships[other].retreating = true;
        session.hangars.ships[other] = 3.0;
        session.advance_hangars(2500.0);
        assert_eq!((session.hangars.step, session.hangars.elapsed_ms), (5, 500.0));
        assert_eq!(session.fighters[0].docked, Some(TacticalDock::Ready), "side {is_alliance}");
        assert_eq!(session.hangars.ships[own], 3.0, "an empty list neither launches nor arms");
        session.ships[other].retreating = false;
        session.advance_hangars(500.0);
        assert_eq!(session.fighters[0].docked, None, "side {is_alliance}");
    }
}

// FUN_005a9de0.c:14-17: the station's hangar runs only for a local Empire
// (battle+0x9ec == 1) and while the station is operational (FUN_005ba420).
#[test]
fn the_station_launches_only_for_a_local_empire_while_operational() {
    let launched = |player_is_attacker: bool, station: TacticalDeathStar| {
        let mut fighter = test_fighter(5, false);
        fighter.carrier = Some(TacticalBerth::Station);
        fighter.docked = Some(TacticalDock::Ready);
        fighter.alive = false;
        let mut session = test_session(Vec::new(), vec![fighter], player_is_attacker);
        session.death_star = Some(station);
        session.hangars.step = 3;
        session.hangars.station = 4.0;
        session.advance_hangars(1000.0);
        session.advance_hangars(1000.0);
        session.fighters[0].docked.is_none()
    };
    let station = test_death_star(false);
    assert!(launched(false, station), "the player is the Empire defender");
    assert!(!launched(true, station), "an Alliance player's battle");
    assert!(!launched(false, TacticalDeathStar { hull: 0.0, ..station }));
}

// FUN_005d14d0.c:62-82: the Empire side's headless step launches every
// squadron docked at the start before any target or damage.
#[test]
fn a_battle_run_without_its_window_launches_every_docked_squadron_on_its_first_step() {
    let mut session = carrier_battle(4, 2, true);
    session.phase = BattlePhase::Combat;
    session.headless_step();
    for fighter in &session.fighters {
        assert_eq!(fighter.docked, None);
        assert!(fighter.alive);
    }
    assert_eq!(session.fighters[2].source_position, session.ships[1].source_position);
}

// FUN_005ba5e0.c:11-15 -> FUN_005b8d10.c:10-22: a departing station moves
// the squadrons still waiting to launch into its recovered list; those in
// flight stay out (FUN_005b8ce0.c:10).
#[test]
fn a_departing_station_recalls_its_waiting_squadrons_but_not_flying_ones() {
    let mut waiting = test_fighter(5, false);
    waiting.carrier = Some(TacticalBerth::Station);
    waiting.docked = Some(TacticalDock::Ready);
    waiting.alive = false;
    let mut flying = test_fighter(5, false);
    flying.carrier = Some(TacticalBerth::Station);
    let mut session = test_session(Vec::new(), vec![waiting, flying], true);
    session.death_star = Some(test_death_star(false));
    session.morale.retreat_permitted = [true, true];
    session.retreat_side(false);
    assert_eq!(session.fighters[0].docked, Some(TacticalDock::Recovered));
    assert!(session.death_star.unwrap().action_committed);
}

// FUN_005ba890.c:5-7 -> FUN_005b8d80.c:8-19: the station's destruction
// destroys every squadron in its launch and recovered lists
// (FUN_005b2d20.c:20-43); squadrons it launched fly on.
#[test]
fn a_destroyed_station_destroys_its_docked_squadrons_only() {
    let mut fighters = Vec::new();
    for docked in [Some(TacticalDock::Ready), Some(TacticalDock::Recovered), None] {
        let mut fighter = test_fighter(5, false);
        fighter.carrier = Some(TacticalBerth::Station);
        fighter.docked = docked;
        fighter.alive = docked.is_none();
        fighters.push(fighter);
    }
    let mut session = test_session(Vec::new(), fighters, true);
    session.death_star = Some(test_death_star(false));
    session.trench_run_pending_outcome = Some(TacticalTrenchRunOutcome::Success);
    session.resolve_original_trench_run();
    assert_eq!(session.fighters[0].squad_count, 0);
    assert_eq!(session.fighters[1].squad_count, 0);
    assert_eq!(session.fighters[2].squad_count, 12);
    assert!(session.fighters[2].alive);
}

// hyp: a squadron docked in a capital ship destroyed in the battle is lost
// with it, as the station's hangar is (FUN_005b8d80.c:8-19).
#[test]
fn squadrons_docked_in_a_lost_ship_are_lost_with_it_and_flying_ones_are_not() {
    let mut session = carrier_battle(4, 1, true);
    session.fighters[0].docked = None;
    session.fighters[0].alive = true;
    session.ships[0].alive = false;
    session.ships[1].alive = false;
    session.ships[1].retreated = true;
    session.destroy_squadrons_docked_in_lost_ships();
    assert_eq!(session.fighters[0].squad_count, 12, "in flight");
    assert_eq!(session.fighters[1].squad_count, 12, "its ship jumped");

    let mut session = carrier_battle(4, 1, true);
    session.ships[0].alive = false;
    session.destroy_squadrons_docked_in_lost_ships();
    assert_eq!(session.fighters[0].squad_count, 0);
}

// FUN_005afe40: a retreating squadron in flight docks in its own carrier,
// else the first ship of its side in the battle with room
// (FUN_005afc80, FUN_005b84a0), else, for the Empire only (:59), in its
// operational station; else FUN_005d14d0.c:107-116 destroys it.
#[test]
fn a_retreating_squadron_docks_in_its_carrier_then_any_ship_then_the_empire_station() {
    let flying = |is_alliance: bool, carrier: usize| {
        let mut fighter = test_fighter(if is_alliance { 3 } else { 5 }, is_alliance);
        fighter.carrier = Some(TacticalBerth::Ship(carrier));
        fighter
    };
    for is_alliance in [true, false] {
        let mut ships = vec![
            test_ship(0x1400_0001, 0, is_alliance, true),
            test_ship(0x1400_0001, 1, is_alliance, true),
        ];
        ships[0].fighter_capacity = 1;
        ships[1].fighter_capacity = 1;
        let fighters = vec![flying(is_alliance, 1), flying(is_alliance, 1), flying(is_alliance, 1)];
        let mut session = test_session(ships, fighters, true);
        let mut station = test_death_star(!is_alliance);
        station.is_attacker = is_alliance;
        station.hangar_capacity = 1;
        session.death_star = Some(station);
        assert_eq!(
            session.retreat_berth(0),
            Some(TacticalBerth::Ship(1)),
            "its own carrier"
        );
        session.fighters[0].carrier = Some(TacticalBerth::Ship(1));
        session.fighters[0].docked = Some(TacticalDock::Recovered);
        assert_eq!(session.retreat_berth(1), Some(TacticalBerth::Ship(0)), "the first with room");
        session.fighters[1].carrier = Some(TacticalBerth::Ship(0));
        session.fighters[1].docked = Some(TacticalDock::Recovered);
        let last = session.retreat_berth(2);
        if is_alliance {
            assert_eq!(last, None, "the Alliance never docks in the station");
        } else {
            assert_eq!(last, Some(TacticalBerth::Station));
        }
    }
}

// FUN_005d14d0.c:102-146: the retreating side's squadrons go first (dock or
// die), its ships after: a ship without a working hyperdrive is destroyed
// and the squadrons it took are lost with it (hyp, as above).
#[test]
fn a_retreat_docks_squadrons_before_its_ships_jump_or_die() {
    let mut session = carrier_battle(1, 1, true);
    session.fighters[0].docked = None;
    session.fighters[0].alive = true;
    session.fighters[0].carrier = None;
    session.ships[0].subsystem_capacity.hyperdrive = 1;
    session.ships[0].subsystem_condition.hyperdrive = 100;
    session.stalemate_resolve_side(true);
    assert!(session.ships[0].retreated);
    assert_eq!(session.fighters[0].carrier, Some(TacticalBerth::Ship(0)));
    assert_eq!(session.fighters[0].docked, Some(TacticalDock::Recovered));
    assert_eq!(session.fighters[0].squad_count, 12);

    let mut session = carrier_battle(1, 1, true);
    session.fighters[0].docked = None;
    session.fighters[0].alive = true;
    session.stalemate_resolve_side(true);
    assert!(!session.ships[0].retreated);
    assert_eq!(session.fighters[0].squad_count, 0, "docked in a ship that was destroyed");
}

// FUN_005bef90.c:72-96 (FUN_005c0b60.c:57-83, headless FUN_005aee90.c:20-47):
// each squadron still in the battle docks again in the first ship of its
// side with room, counting those already docked (FUN_005b84a0 adds without
// leaving the old hangar), else, for the Empire, in the station while
// operational or departed (+0x78); with no room a docked squadron keeps
// its carrier and a flying one has none.
#[test]
fn at_the_end_each_squadron_docks_in_the_first_ship_with_room_or_the_empire_station() {
    let mut session = carrier_battle(2, 2, true);
    for fighter in &mut session.fighters {
        fighter.docked = None;
        fighter.alive = true;
    }
    session.fighters[1].docked = Some(TacticalDock::Ready);
    let alliance_extra = test_fighter(3, true);
    session.fighters.push(alliance_extra);
    let mut station = test_death_star(false);
    station.hangar_capacity = 4;
    station.hull = 0.0;
    station.action_committed = false;
    session.death_star = Some(station);
    let berths = session.end_berths();
    assert_eq!(berths[0], Some(TacticalBerth::Ship(0)), "the docked one holds the other berth");
    assert_eq!(
        berths[1],
        Some(TacticalBerth::Ship(0)),
        "counted already, it finds no room and keeps its carrier"
    );
    assert_eq!(berths[2], Some(TacticalBerth::Ship(1)));
    assert_eq!(berths[3], Some(TacticalBerth::Ship(1)));
    assert_eq!(berths[4], None);

    let mut empire = test_fighter(5, false);
    empire.carrier = None;
    session.fighters.push(empire);
    assert_eq!(session.end_berths()[5], None, "a destroyed station takes none");
    session.death_star.as_mut().unwrap().hull = 100.0;
    session.death_star.as_mut().unwrap().action_committed = true;
    assert_eq!(session.end_berths()[5], Some(TacticalBerth::Station), "departed");
}

// FUN_0040d320.c:54-85: a squadron docked at the end becomes a child of its
// carrier's strategic ship (the station's: its Death Star ship,
// FUN_005ba670.c:15-17); one in no hangar becomes a child of the system.
#[test]
fn after_the_battle_a_squadron_rides_the_ship_it_docked_in_or_falls_to_the_system() {
    let (mut world, system) = carried_world();
    let mut session = battle_of(&world, system);
    let alliance_fleet = world.systems[system].fleets[0];
    let empire_fleet = world.systems[system].fleets[1];
    // The Alliance carrier is lost; its squadrons fly free at the end.
    let alliance_ship = session
        .ships
        .iter()
        .position(|ship| ship.identity.is_alliance)
        .unwrap();
    session.ships[alliance_ship].alive = false;
    session.ships[alliance_ship].hull_current = 0;
    for fighter in session.fighters.iter_mut().filter(|fighter| fighter.identity.is_alliance) {
        fighter.docked = None;
        fighter.alive = true;
    }
    session.winner = Some(CombatWinner::Defender);
    session.phase = BattlePhase::Results;
    let homes = apply::squadron_homes(&session, &mut world.clone());
    for (fighter, home) in session.fighters.iter().zip(&homes) {
        if fighter.identity.is_alliance {
            assert_eq!(*home, Some(None), "an Alliance squadron falls to the system");
        } else {
            assert_eq!(*home, Some(Some((empire_fleet, 2))));
        }
    }
    let mut troops = crate::troop_transport::TroopTransportState::default();
    let mut movement = crate::movement::MovementState::default();
    let mut log = Vec::new();
    apply::apply_results(
        &session,
        &mut world,
        &mut troops,
        &mut movement,
        &ProductRoom::default(),
        &mut log,
    );
    assert!(world
        .fleets
        .get(alliance_fleet)
        .is_none_or(|fleet| fleet.fighters.is_empty()));
    let empire = &world.fleets[empire_fleet];
    assert_eq!(
        empire
            .fighters
            .iter()
            .map(|entry| (entry.count, entry.carrier))
            .collect::<Vec<_>>(),
        vec![(1, 2), (1, 2)],
        "both Imperial squadrons dock in their carrier again"
    );
}

// FUN_005ae9c0.c:62-103: a stalemate loser that may not retreat (an enemy
// gravity well, FUN_005d0f50) loses every ship and squadron (FUN_005af7a0);
// a squadron recovered into one of its carriers goes with the carrier
// (tactical-death-star.md, section 18).
#[test]
fn a_stalemate_loser_under_a_gravity_well_loses_its_docked_squadrons_for_either_side() {
    for is_attacker in [true, false] {
        let mut session = carrier_battle(2, 1, true);
        session.morale.retreat_permitted = [true, true];
        let well = usize::from(is_attacker);
        session.morale.gravity_wells = vec![well];
        for fighter in &mut session.fighters {
            fighter.docked = Some(TacticalDock::Recovered);
        }
        for is_alliance in [true, false] {
            let mut flying = test_fighter(if is_alliance { 3 } else { 5 }, is_alliance);
            flying.is_attacker = is_alliance;
            session.fighters.push(flying);
        }
        session.stalemate_withdraw(is_attacker);
        for fighter in &session.fighters {
            let lost = fighter.is_attacker == is_attacker;
            assert_eq!(fighter.squad_count == 0, lost, "{is_attacker}");
        }
    }
}
