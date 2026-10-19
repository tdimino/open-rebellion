//! The tactical Death Star station and the trench run
//! (`ghidra/notes/tactical-death-star.md`).

use super::test_support::{test_death_star, test_fighter, test_session, test_ship};
use super::*;
use crate::carriage::ProductRoom;
use crate::world::{CapitalShipClass, DefenseFacilityInstance, Fleet, ShipInstance, System};

/// A system holding a Death Star fleet of the side `owner_is_alliance` and an
/// opposing fleet; the Death Star ship has taken `damage`.
/// port: test fixture values, not rules.
fn station_world(owner_is_alliance: bool, damage: i32) -> (GameWorld, SystemKey, FleetKey, FleetKey) {
    let mut world = GameWorld::default();
    let system = world.systems.insert(System::default());
    let mut class = |family, is_alliance: bool, fighter_capacity| {
        world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(if family == 0x18 {
                crate::world::DEATH_STAR_CLASS_ID
            } else {
                0x1400_0001
            }),
            family,
            is_alliance,
            is_empire: !is_alliance,
            hull: 100,
            fighter_capacity,
            ..CapitalShipClass::default()
        })
    };
    let death_star = class(0x18, owner_is_alliance, 4);
    let destroyer = class(0x14, owner_is_alliance, 0);
    let cruiser = class(0x14, !owner_is_alliance, 0);
    let owner = world.fleets.insert(Fleet {
        location: system,
        capital_ships: vec![
            ShipInstance::new(destroyer, 100, owner_is_alliance),
            ShipInstance::new(death_star, 100 - damage, owner_is_alliance),
        ],
        is_alliance: owner_is_alliance,
        ..Fleet::default()
    });
    let enemy = world.fleets.insert(Fleet {
        location: system,
        capital_ships: vec![ShipInstance::new(cruiser, 100, !owner_is_alliance)],
        is_alliance: !owner_is_alliance,
        ..Fleet::default()
    });
    world.systems[system].fleets = vec![owner, enemy];
    world.systems[system].fleets.sort_unstable();
    (world, system, owner, enemy)
}

// FUN_0040b0e0.c:86-118 exports a family-0x18 ship only as the station's
// short record; FUN_0040bcd0 passes FUN_00595ef0 the class hull (+0x248),
// the remaining hull (+0x248 - +0x274), the shield (a family-0x25 child of
// the system, through FUN_0051c270) and the hangar (+0x26c, the class's
// fighter capacity); the charge starts full. The rule takes no side.
#[test]
fn the_station_is_raised_from_its_fleets_death_star_ship_and_is_no_capital_participant() {
    for owner_is_alliance in [false, true] {
        let (mut world, system, owner, enemy) = station_world(owner_is_alliance, 30);
        let session = BattleSession::new(&world, system, enemy, owner, true, 1, 0x5eed);
        let station = session.death_star.expect("the owner's station");
        assert!(!station.is_attacker);
        assert_eq!(station.is_alliance, owner_is_alliance);
        assert_eq!(station.hull_max, 100.0);
        assert_eq!(station.hull, 70.0);
        assert_eq!(station.laser_charge, ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE);
        assert!(!station.shielded);
        assert!(!station.not_completed);
        assert_eq!(station.hangar_capacity, 4);
        assert_eq!(station.ship, Some((owner, 1)));
        assert_eq!(session.ships.len(), 2, "the destroyer and the cruiser only");
        assert!(session
            .ships
            .iter()
            .all(|ship| ship.fleet != Some(owner) || ship.fleet_ship_index == 0));

        let shield = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: DatId::new(0x2500_0001),
            side: crate::dat::Faction::Empire,
        });
        world.systems[system].defense_facilities.push(shield);
        let session = BattleSession::new(&world, system, enemy, owner, true, 1, 0x5eed);
        let station = session.death_star.expect("the owner's station");
        assert!(station.shielded);
        assert!(station.operational());
        assert!(!station.attackable());
    }
}

// FUN_0040d170: the Death Star ship's hull becomes the station's remaining
// hull, truncated (__ftol); less is damage through FUN_00501ae0, which
// destroys it at none.
#[test]
fn the_death_star_ship_takes_the_stations_remaining_hull() {
    for owner_is_alliance in [false, true] {
        let (mut world, system, owner, enemy) = station_world(owner_is_alliance, 0);
        let mut session = BattleSession::new(&world, system, enemy, owner, true, 1, 0x5eed);
        session.death_star.as_mut().expect("the station").hull = 37.9;
        session.winner = Some(CombatWinner::Defender);
        let mut log = Vec::new();
        apply::apply_results(
            &session,
            &mut world,
            &mut crate::troop_transport::TroopTransportState::default(),
            &mut crate::movement::MovementState::new(),
            &ProductRoom::default(),
            &mut log,
        );
        assert!(world.fleet_has_death_star(owner), "{log:?}");
        assert_eq!(world.fleets[owner].capital_ships[1].hull_current, 37);
    }
}

// FUN_005d04e0 (0x5d0683-0x5d06e3): rand(1,100) <= trunc((FUN_005b23a0's
// hp * torpedoes * 0.53333336 / hp_max + min(rating, 9)) * 0.5).
#[test]
fn the_trench_chance_is_half_the_torpedo_term_plus_the_rating_truncated() {
    let mut fighter = test_fighter(3, true);
    fighter.torpedo_strength = 48;
    assert_eq!(trench_run_success_chance(&fighter, 9.0), 17);
    assert_eq!(trench_run_success_chance(&fighter, 12.0), 17, "the rating caps at 9");
    fighter.hull_current = 48.0;
    assert_eq!(trench_run_success_chance(&fighter, 1.0), 6);
    fighter.torpedo_strength = 0;
    assert_eq!(trench_run_success_chance(&fighter, 1.0), 0);
}

// FUN_005ad7e0(manager, 0): clamp(combat / 20 + 1, 1, 9) of the Alliance
// Commander while the ship carrying the character fights (FUN_005ad870);
// else 1.0.
#[test]
fn the_commander_rating_is_combat_over_twenty_plus_one_while_its_fleet_fights() {
    let fleet = FleetKey::default();
    let mut ship = test_ship(64, 0, true, true);
    ship.fleet = Some(fleet);
    let mut session = test_session(vec![ship], Vec::new(), true);
    assert_eq!(session.trench_run_rating(), 1.0);
    for (combat, rating) in [(0, 1.0), (60, 4.0), (70, 4.5), (160, 9.0)] {
        session.trench_run_commander = Some(TacticalTrenchCommander {
            combat,
            fleet: Some(fleet),
        });
        assert_eq!(session.trench_run_rating(), rating, "combat {combat}");
    }
    session.trench_run_commander = Some(TacticalTrenchCommander {
        combat: 60,
        fleet: None,
    });
    assert_eq!(session.trench_run_rating(), 1.0, "one the system holds rides no ship");
    session.trench_run_commander = Some(TacticalTrenchCommander {
        combat: 60,
        fleet: Some(fleet),
    });
    session.ships[0].alive = false;
    assert_eq!(session.trench_run_rating(), 1.0, "the carrying ship is gone");
}

// FUN_0040c200.c:68-70: the record's +0x50 is FUN_0040d680 (FILD the
// argument, FIDIV the integer 20 at DAT_006a80e8, a float quotient) of
// slot +0x1f0, FUN_004edc40, the effective combat (short +0x86);
// FUN_005afc00 adds one and caps it at 9 (_DAT_0066c3ec). The Commander
// is the Alliance's (FUN_0040b0e0.c:86-118) whichever side attacks.
#[test]
fn the_alliance_commander_is_rated_on_effective_combat_over_twenty_with_its_fraction() {
    use crate::officer_rank::CommandKind;
    use crate::world::{Character, SkillPair};
    for attacker_is_alliance in [true, false] {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        let mut side_fleets = [Vec::new(), Vec::new()];
        for is_alliance in [true, false] {
            let fleet = world.fleets.insert(Fleet {
                location: system,
                is_alliance,
                ..Fleet::default()
            });
            side_fleets[usize::from(is_alliance != attacker_is_alliance)].push(fleet);
            // Effective combat 70 for the Alliance's, 160 for the Empire's.
            world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                can_be_commander: true,
                command_kind: CommandKind::Commander,
                current_fleet: Some(fleet),
                combat: SkillPair {
                    base: if is_alliance { 60 } else { 150 },
                    variance: 20,
                },
                ..Character::default()
            });
        }
        let commander =
            TacticalTrenchCommander::of_battle(&world, system, &side_fleets, attacker_is_alliance)
                .unwrap();
        assert_eq!(commander.fleet, Some(side_fleets[usize::from(!attacker_is_alliance)][0]));
        assert_eq!(commander.combat, 70);
        assert_eq!(commander.rating(), 4.5);
    }
}

// FUN_0040b0e0.c:86-118: FUN_00509530 takes the system's Commander, then
// each fleet's, keeping the highest effective combat; a tie keeps the
// earlier.
#[test]
fn the_highest_alliance_commander_rates_the_run_and_a_tie_keeps_the_systems() {
    use crate::officer_rank::CommandKind;
    use crate::world::{Character, SkillPair};
    for attacker_is_alliance in [true, false] {
        for (at_system, in_fleet, system_wins) in [(50, 70, false), (70, 50, true), (60, 60, true)] {
            let mut world = GameWorld::default();
            let system = world.systems.insert(System::default());
            let fleet = world.fleets.insert(Fleet {
                location: system,
                is_alliance: true,
                ..Fleet::default()
            });
            let mut side_fleets = [Vec::new(), Vec::new()];
            side_fleets[usize::from(!attacker_is_alliance)].push(fleet);
            for (combat, current_fleet, current_system) in
                [(at_system, None, Some(system)), (in_fleet, Some(fleet), None)]
            {
                world.characters.insert(Character {
                    is_alliance: true,
                    can_be_commander: true,
                    command_kind: CommandKind::Commander,
                    current_fleet,
                    current_system,
                    combat: SkillPair {
                        base: combat,
                        variance: 0,
                    },
                    ..Character::default()
                });
            }
            let commander =
                TacticalTrenchCommander::of_battle(&world, system, &side_fleets, attacker_is_alliance)
                    .unwrap();
            let expected = if system_wins { (at_system, None) } else { (in_fleet, Some(fleet)) };
            assert_eq!((commander.combat, commander.fleet), expected);
        }
    }
}

// FUN_005afc00: the headless run takes the Commander's rating when
// FUN_005a6340 finds the carrying object among the battle's, with no
// FUN_005ad870 state check, so a destroyed carrier still counts; a
// Commander the system holds, or one riding a fleet outside the battle,
// rates 1.
#[test]
fn the_headless_rating_needs_the_commanders_carrier_in_the_battle_alive_or_not() {
    let fleet = FleetKey::default();
    let mut ship = test_ship(64, 0, true, true);
    ship.fleet = Some(fleet);
    let mut session = test_session(vec![ship], Vec::new(), true);
    let commander = |fleet| {
        Some(TacticalTrenchCommander {
            combat: 160,
            fleet,
        })
    };
    session.trench_run_commander = commander(Some(fleet));
    assert_eq!(session.headless_trench_rating(), 9.0);
    session.ships[0].alive = false;
    assert_eq!(
        session.headless_trench_rating(),
        9.0,
        "the carrier was destroyed"
    );
    session.trench_run_commander = commander(None);
    assert_eq!(session.headless_trench_rating(), 1.0, "the system holds it");
    let outside = slotmap::SlotMap::<FleetKey, ()>::with_key().insert(());
    session.trench_run_commander = commander(Some(outside));
    assert_eq!(session.headless_trench_rating(), 1.0, "its fleet is not in the battle");
}

/// A trench squadron of the Alliance group 0 with `craft` craft, eight hull
/// points each (`ORIGINAL_FIGHTER_HULL_PER_CRAFT`).
/// port: test fixture values, not rules.
fn trench_squadron(craft: u32) -> TacticalFighter {
    let mut fighter = test_fighter(3, true);
    fighter.order = TacticalOrder::AttackDeathStar;
    fighter.squad_count = craft;
    fighter.hull_current = craft as f32 * ORIGINAL_FIGHTER_HULL_PER_CRAFT;
    fighter.hull_max = fighter.hull_current;
    fighter
}

fn trench_session(fighters: Vec<TacticalFighter>, seed: u32) -> BattleSession {
    let mut session = test_session(Vec::new(), fighters, true);
    session.death_star = Some(test_death_star(false));
    session.trench_run_group = Some(0);
    session.tactical_rng = OriginalTacticalRng::new(seed);
    session
}

// FUN_005d04e0: a squadron that misses its maneuver (rand(1,10) > trunc(m),
// m = 1.0 with no sublight speed, FUN_005ba270) takes rand(1,n) craft of
// its n = trunc(hp / 8); with fewer than two craft the damage is 100000
// (0x5d05ea-0x5d0620), with no further draw. Only a live squadron rolls
// rand(1,100).
#[test]
fn a_maneuver_miss_takes_one_to_n_craft_and_a_last_craft_dies_without_a_draw() {
    let seed = (0..)
        .find(|&seed| OriginalTacticalRng::new(seed).inclusive(1, 10) >= 2)
        .unwrap();
    let mut single = trench_squadron(1);
    single.sublight_engine_power = 0.0;
    let mut session = trench_session(vec![single], seed);
    session.prepare_original_trench_run();
    let mut expected = OriginalTacticalRng::new(seed);
    let _ = expected.inclusive(1, 10);
    assert_eq!(session.tactical_rng, expected, "one draw, then the list is empty");
    assert!(!session.fighters[0].alive);
    assert_eq!(
        session.trench_run_pending_outcome,
        Some(TacticalTrenchRunOutcome::Failure)
    );

    // Two craft: rand(1,2) craft go; one left rolls rand(1,100) and pops
    // again, then dies outright on its next miss.
    let mut pair = trench_squadron(2);
    pair.sublight_engine_power = 0.0;
    let mut session = trench_session(vec![pair], seed);
    session.prepare_original_trench_run();
    let mut expected = OriginalTacticalRng::new(seed);
    let _ = expected.inclusive(1, 10);
    let mut hull = 16.0 - expected.inclusive(1, 2) as f32 * 8.0;
    while hull > 0.0 {
        let _ = expected.inclusive(1, 100);
        if expected.inclusive(1, 10) > 1 {
            hull = 0.0;
        }
    }
    assert_eq!(session.tactical_rng, expected);
    assert!(!session.fighters[0].alive);
}

// FUN_005d04e0 pops the lowest id (thunk_FUN_005f5060) and puts a survivor
// back under the same id (0x5d06f9), so the first squadron rolls until it
// dies before the second rolls once; a success restores every participant's
// hull from the state-5 snapshot (FUN_005d08d0, 0x5d07f0) and the damaged
// chatter is chosen.
#[test]
fn the_trench_roll_pops_the_lowest_squadron_first_and_a_success_restores_every_hull() {
    // The first squadron survives its first pop, and the second's maneuver
    // holds once the first is dead.
    let oracle = |seed: u32| {
        let mut rng = OriginalTacticalRng::new(seed);
        let mut first_pops = 0;
        loop {
            if rng.inclusive(1, 10) > 9 {
                break;
            }
            let _ = rng.inclusive(1, 100);
            first_pops += 1;
        }
        let second_holds = rng.inclusive(1, 10) <= 9;
        let _ = rng.inclusive(1, 100);
        (first_pops, second_holds, rng)
    };
    let seed = (0..)
        .find(|&seed| {
            let (first_pops, second_holds, _) = oracle(seed);
            first_pops > 0 && second_holds
        })
        .unwrap();
    let mut doomed = trench_squadron(1);
    doomed.maneuverability = 8;
    let mut striker = trench_squadron(1);
    striker.maneuverability = 8;
    striker.torpedo_strength = 1000;
    let mut session = trench_session(vec![doomed, striker], seed);
    session.prepare_original_trench_run();

    assert_eq!(session.tactical_rng, oracle(seed).2);
    assert_eq!(
        session.trench_run_pending_outcome,
        Some(TacticalTrenchRunOutcome::Success)
    );
    assert_eq!(session.trench_run_chatter, ORIGINAL_TRENCH_RUN_DAMAGED_SUCCESS_CHATTER);
    assert_eq!(session.fighters[0].hull_current, 8.0, "the snapshot is restored");
    assert!(session.fighters[0].alive);
    assert!(session.fighters.iter().all(|fighter| fighter.off_field));
}

// FUN_005d04e0 leaves for failure on the 1001st pop (CMP EAX,0x3e8; JA at
// 0x5d06ee), squadrons still flying.
#[test]
fn the_trench_loop_gives_up_after_the_thousand_and_first_pop() {
    let fighters = (0..8)
        .map(|_| {
            let mut fighter = trench_squadron(1_000_000);
            // No torpedoes and the base rating: trunc((0 + 1) * 0.5) = 0%.
            fighter.torpedo_strength = 0;
            fighter
        })
        .collect::<Vec<_>>();
    let maneuver = fighters[0].trench_maneuver().trunc() as u32;
    let mut session = trench_session(fighters, 7);
    let mut oracle = session.tactical_rng;
    session.prepare_original_trench_run();
    assert_eq!(
        session.trench_run_pending_outcome,
        Some(TacticalTrenchRunOutcome::Failure)
    );
    assert!(session.fighters.iter().any(|fighter| fighter.hull_current > 0.0));

    // The draws of exactly 1001 pops; a survivor pops again at once.
    let mut hull = 1_000_000.0 * ORIGINAL_FIGHTER_HULL_PER_CRAFT;
    for _ in 0..=1000 {
        if oracle.inclusive(1, 10) > maneuver {
            let craft = (hull / 8.0_f32).trunc() as u32;
            hull -= if craft < 2 {
                hull
            } else {
                oracle.inclusive(1, craft) as f32 * 8.0
            };
        }
        if hull > 0.0 {
            oracle.inclusive(1, 100);
        } else {
            hull = 1_000_000.0 * ORIGINAL_FIGHTER_HULL_PER_CRAFT;
        }
    }
    assert_eq!(session.tactical_rng, oracle, "the loop drew for 1001 pops");
}

// FUN_005ba890 (state 6) zeroes the station's hull (+0x68) and charge
// (+0x6c); it writes neither +0x74 nor +0x78.
#[test]
fn a_successful_run_zeroes_the_hull_and_charge_and_leaves_the_flags() {
    let mut session = trench_session(vec![trench_squadron(1)], 1);
    session.death_star.as_mut().unwrap().laser_charge = 60.0;
    session.trench_run_participants = vec![0];
    session.trench_run_pending_outcome = Some(TacticalTrenchRunOutcome::Success);
    session.resolve_original_trench_run();
    let station = session.death_star.unwrap();
    assert_eq!(station.hull, 0.0);
    assert_eq!(station.laser_charge, 0.0);
    assert!(!station.not_completed);
    assert!(!station.action_committed);
    assert!(!station.operational());
    assert!(!session.fighters[0].off_field, "the survivor is listed again");
}

// FUN_005d0ce0.c:78-103: an operational station adds 20000000.0 to the
// Empire, and to the Alliance when FUN_005afcc0 holds (attackable, and more
// listed Alliance squadrons than Imperial) or an Alliance group has order 6.
#[test]
fn the_alliance_takes_the_death_star_strength_with_more_squadrons_or_an_order_six_group() {
    let alliance_strength = |alliance: usize, empire: usize, shielded: bool, order_six: bool| {
        let mut fighters: Vec<_> = (0..alliance).map(|_| test_fighter(3, true)).collect();
        fighters.extend((0..empire).map(|_| test_fighter(5, false)));
        if order_six {
            fighters[0].order = TacticalOrder::AttackDeathStar;
        }
        let mut session = test_session(Vec::new(), fighters, true);
        let mut station = test_death_star(false);
        station.shielded = shielded;
        session.death_star = Some(station);
        (
            session.stalemate_strength(true) >= ORIGINAL_DEATH_STAR_STRENGTH,
            session.stalemate_strength(false) >= ORIGINAL_DEATH_STAR_STRENGTH,
        )
    };
    assert_eq!(alliance_strength(3, 2, false, false), (true, true));
    assert_eq!(alliance_strength(2, 2, false, false), (false, true));
    assert_eq!(alliance_strength(3, 2, true, false), (false, true));
    assert_eq!(alliance_strength(2, 2, true, true), (true, true));
}

// FUN_005d0f50.c:32-40: the Empire's station gives its side a way out when
// operational (FUN_005ba420) or not completed (+0x74); it tests no fleet.
#[test]
fn the_empire_may_retreat_on_its_station_while_operational_or_unfinished() {
    let mut session = test_session(Vec::new(), Vec::new(), true);
    let mut station = test_death_star(false);
    session.death_star = Some(station);
    assert!(session.death_star_allows_retreat(false));
    assert!(!session.death_star_allows_retreat(true));

    station.not_completed = true;
    session.death_star = Some(station);
    assert!(session.death_star_allows_retreat(false));

    station.not_completed = false;
    station.hull = 0.0;
    session.death_star = Some(station);
    assert!(!session.death_star_allows_retreat(false), "a destroyed station");

    let mut rebel = test_death_star(false);
    rebel.is_alliance = true;
    session.death_star = Some(rebel);
    assert!(!session.death_star_allows_retreat(false), "side 1 only");
}

// FUN_005c8780.c:136-176: the computer Empire decides every sixth update
// (countdown `param_1[0x10]` less 1.0, reset to 6.0) and, its station
// operational and charged to exactly 100.0, fires at the first enemy capital
// ship still in the battle (FUN_005ad870); the shot zeroes the charge
// (FUN_005ba720).
#[test]
fn the_computer_empire_fires_a_full_charge_at_the_first_live_enemy_ship_every_sixth_update() {
    let ships = vec![
        test_ship(0x1400_0001, 0, true, false),
        test_ship(0x1400_0001, 1, true, true),
        test_ship(0x1400_0002, 0, false, true),
    ];
    let mut session = test_session(ships, Vec::new(), true);
    let mut station = test_death_star(false);
    station.laser_charge = ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE;
    session.death_star = Some(station);
    session.death_star_ai.countdown = 6.0;
    for _ in 0..5 {
        session.advance_computer_controller();
    }
    assert!(session.death_star_beam.is_none(), "no decision before the sixth update");
    session.advance_computer_controller();
    assert_eq!(session.death_star_beam.map(|beam| beam.target), Some(1));
    assert_eq!(session.death_star.unwrap().laser_charge, 0.0);
    // FUN_005ba520.c:23-31: the local Alliance hears its warning 0x91 and
    // event 0x14; the targeting voice 0x111 is the player's (FUN_005a5740).
    assert_eq!(session.pending_voice_cues, vec![TacticalVoiceCue::death_star_fire_warning()]);
    assert_eq!(
        session.pending_audio_cues,
        vec![TacticalAudioCue::from_variant(TacticalAudioEvent::TorpedoImpact, 0)]
    );
    assert_eq!(session.pending_audio_cues[0].resource_id, 13_054);
    session.death_star_beam = None;
    session.death_star.as_mut().unwrap().laser_charge = ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE;
    for _ in 0..5 {
        session.advance_computer_controller();
    }
    assert!(session.death_star_beam.is_none(), "the countdown restarts at 6.0");
    session.advance_computer_controller();
    assert!(session.death_star_beam.is_some());

    let mut session = test_session(vec![test_ship(0x1400_0001, 0, true, true)], Vec::new(), true);
    let mut station = test_death_star(false);
    station.laser_charge = ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE - 1.0;
    session.death_star = Some(station);
    session.advance_computer_controller();
    assert!(session.death_star_beam.is_none(), "the charge must be exactly 100.0");
}

// FUN_005c8780.c:178-209: the computer Alliance gives one group order 6,
// once, when FUN_005ad930 holds.
#[test]
fn the_computer_alliance_orders_one_trench_run_when_the_station_is_open_to_it() {
    let mut fighters = vec![test_fighter(3, true), test_fighter(3, true), test_fighter(5, false)];
    fighters[0].fighter_group = 2;
    fighters[1].fighter_group = 1;
    let mut session = test_session(Vec::new(), fighters, false);
    session.death_star = Some(test_death_star(false));
    session.advance_computer_controller();
    assert_eq!(session.trench_run_group, Some(1), "the lowest group goes");
    assert_eq!(session.fighters[1].order, TacticalOrder::AttackDeathStar);
    assert_eq!(session.fighters[0].order, TacticalOrder::None);
    assert!(session.death_star_ai.ordered_run);

    let mut fighters = vec![test_fighter(3, true), test_fighter(5, false)];
    fighters[0].fighter_group = 0;
    let mut session = test_session(Vec::new(), fighters, false);
    session.death_star = Some(test_death_star(false));
    session.advance_computer_controller();
    assert_eq!(session.trench_run_group, None, "squadrons must outnumber the Empire's");
}

// port: the computer Alliance's head group (FUN_005c8780.c:198, slot 0x10 of
// param_1[0x19]) is the lowest of the four colour groups that still has
// squadrons; a spent group and the unassigned group 4 never go.
#[test]
fn the_computer_alliance_skips_a_spent_group_and_never_sends_the_unassigned_one() {
    let session_with = |groups: &[(u8, u32)]| {
        let mut fighters: Vec<_> = groups
            .iter()
            .map(|&(group, squads)| {
                let mut fighter = test_fighter(3, true);
                fighter.fighter_group = group;
                fighter.squad_count = squads;
                fighter
            })
            .collect();
        fighters.push(test_fighter(5, false));
        let mut session = test_session(Vec::new(), fighters, false);
        session.death_star = Some(test_death_star(false));
        session.advance_computer_controller();
        session
    };
    let spent = session_with(&[(0, 0), (2, 12), (3, 12)]);
    assert_eq!(spent.trench_run_group, Some(2), "group 0 has no squadrons left");

    let unassigned = session_with(&[(4, 12), (4, 12)]);
    assert_eq!(unassigned.trench_run_group, None);
    assert!(!unassigned.death_star_ai.ordered_run);
}

// FUN_005ad930.c:15-49: the station attackable (not shielded, operational),
// no run under way, more listed Alliance squadrons than Imperial, and no
// Alliance group under order 6.
#[test]
fn the_attack_order_needs_an_open_station_more_alliance_squadrons_and_no_run() {
    let session_with = |station: TacticalDeathStar, alliance: usize, empire: usize| {
        let mut fighters: Vec<_> = (0..alliance).map(|_| test_fighter(3, true)).collect();
        fighters.extend((0..empire).map(|_| test_fighter(5, false)));
        let mut session = test_session(Vec::new(), fighters, true);
        session.death_star = Some(station);
        session
    };
    let open = test_death_star(false);
    assert!(session_with(open, 2, 1).trench_run_order_available());
    assert!(!session_with(open, 1, 1).trench_run_order_available());
    let shielded = TacticalDeathStar { shielded: true, ..open };
    assert!(!session_with(shielded, 2, 1).trench_run_order_available());
    let unfinished = TacticalDeathStar { not_completed: true, ..open };
    assert!(!session_with(unfinished, 2, 1).trench_run_order_available());
    let mut ordered = session_with(open, 2, 1);
    ordered.fighters[0].order = TacticalOrder::AttackDeathStar;
    assert!(!ordered.trench_run_order_available());
    let mut running = session_with(open, 2, 1);
    running.trench_run_group = Some(0);
    assert!(!running.trench_run_order_available());
}

// FUN_005c6610.c:112-145: the group flies to 0.5 * battle+0x8cc on its own
// edge, (-50, 0, 0) for the Alliance at an extent of 100, at its speed times
// 0.95 (FUN_005eb1a0); within 0.2 the result enters state 5 and the
// 120000.0 timer starts.
#[test]
fn a_trench_group_flies_to_its_entry_point_and_starts_the_run_on_arrival() {
    let mut fighter = trench_squadron(4);
    fighter.fighter_group = 0;
    fighter.source_position = TacticalWorldPosition { x: -40.0, y: 0.0, z: 0.0 };
    fighter.sublight_engine_power = 10.0;
    let mut session = trench_session(vec![fighter], 1);
    session.source_layout.battle_extent = 100.0;
    assert_eq!(
        session.trench_run_entry_point(true),
        TacticalWorldPosition { x: -50.0, y: 0.0, z: 0.0 }
    );
    assert_eq!(session.trench_run_entry_point(false).x, 50.0);
    session.advance_trench_run_approach(1000.0);
    let position = session.fighters[0].source_position;
    assert!((position.x + 49.5).abs() < 1e-4, "10 * 0.95 per second: {position:?}");
    assert_eq!(session.trench_run_remaining_ms, None);
    session.advance_trench_run_approach(1000.0);
    assert_eq!(session.fighters[0].source_position.x, -50.0);
    assert!(session.trench_run_remaining_ms.is_some(), "arrival starts the state-5 timer");
}

// FUN_0040b0e0.c:184-203: with no Death Star in a fleet, a Death Star at
// queue position 0 still in production is exported as an uncompleted
// station (+0x74 = 1, FUN_0040bcd0.c:62): not operational, sprite 5020,
// charge 100.0, for the Empire whether it attacks or defends; a product
// behind another queue head is not.
#[test]
fn a_death_star_at_the_head_of_its_queue_is_an_uncompleted_station_for_either_role() {
    use crate::manufacturing::{BuildableKind, ManufacturingState, QueueItem};
    let (world, system, _, _) = station_world(false, 0);
    let death_star_class = world
        .capital_ship_classes
        .iter()
        .find(|(_, class)| class.is_death_star())
        .map(|(key, _)| key)
        .unwrap();
    let other_class = world
        .capital_ship_classes
        .iter()
        .find(|(_, class)| !class.is_death_star())
        .map(|(key, _)| key)
        .unwrap();
    for attacker_is_alliance in [true, false] {
        let mut manufacturing = ManufacturingState::default();
        manufacturing.enqueue(system, QueueItem::new(BuildableKind::CapitalShip(death_star_class), 10));
        let mut session = test_session(Vec::new(), Vec::new(), true);
        session.system = system;
        session.attacker_is_alliance = attacker_is_alliance;
        session.seat_unfinished_station(&world, &manufacturing);
        let station = session.death_star.expect("an uncompleted station");
        assert!(station.not_completed && !station.operational());
        assert_eq!(station.is_attacker, !attacker_is_alliance);
        assert!(!station.is_alliance);
        assert_eq!(station.resource_id(), 5020);
        assert_eq!(station.laser_charge, ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE);
        assert_eq!((station.hull, station.hull_max), (100.0, 100.0));
        assert_eq!(station.ship, None);
    }
    let mut manufacturing = ManufacturingState::default();
    manufacturing.enqueue(system, QueueItem::new(BuildableKind::CapitalShip(other_class), 10));
    manufacturing.enqueue(system, QueueItem::new(BuildableKind::CapitalShip(death_star_class), 10));
    let mut session = test_session(Vec::new(), Vec::new(), true);
    session.system = system;
    session.seat_unfinished_station(&world, &manufacturing);
    assert!(session.death_star.is_none(), "only queue position 0 counts");
}

/// `FUN_005f57a0` as the multiplicative congruence it computes:
/// `x * 0xa18e mod 0x7fffffff`.
fn lehmer(x: i64) -> i64 {
    x * 0xa18e % 0x7fff_ffff
}

/// The battle generator advanced by `draws` draws of any span above one
/// (`FUN_005f5700` steps once per draw whatever its span).
fn advanced(mut rng: OriginalBattleRng, draws: usize) -> OriginalBattleRng {
    for _ in 0..draws {
        rng.between(0, 100);
    }
    rng
}

// FUN_005af5f0.c:38 seeds the battle object's generator with 12345678 and
// FUN_005f56b0 steps it sixteen times; FUN_005af420(a, b) returns a plus
// state / (0x7ffffffe / (b - a)) of the next step (FUN_005f5700), so a draw
// never reaches b, and a span of one or less draws nothing.
#[test]
fn the_battle_generator_is_a_seeded_lehmer_stream_whose_draws_stop_short_of_their_top() {
    let mut state = 12_345_678_i64;
    for _ in 0..16 {
        state = lehmer(state);
    }
    let mut rng = OriginalBattleRng::default();
    for (minimum, maximum) in [(1, 10), (1, 100), (0, 100), (1, 4)] {
        state = lehmer(state);
        let span = i64::from(maximum - minimum);
        let expected = i64::from(minimum) + state / (0x7fff_fffe / span);
        assert_eq!(i64::from(rng.between(minimum, maximum)), expected);
        assert!(expected < i64::from(maximum));
    }
    let before = rng;
    assert_eq!(rng.between(1, 2), 1);
    assert_eq!(rng.between(3, 3), 3);
    assert_eq!(rng, before, "no draw for a span of one or less");
}

// FUN_005d14d0.c:41-60: the Empire side's headless step fires a station
// charged to exactly 100.0 at the first capital ship of its ranked
// opponents (FUN_005d1a70); FUN_005ba710 zeroes the charge alone (no
// message, voice or effect) and FUN_005af7a0 destroys the ship and its
// hangar. Short of 100.0 the step adds dt * (1/3) * 0.001 with dt = 1.0
// (FUN_005ba7f0, DAT_0066badc), so it never fires again in a battle.
#[test]
fn the_headless_empire_fires_its_full_charge_once_at_its_first_ranked_capital_ship() {
    let ships = vec![
        test_ship(0x1400_0001, 0, true, true),
        test_ship(0x1400_0002, 1, true, true),
        test_ship(0x1400_0003, 0, false, true),
    ];
    let mut docked = test_fighter(3, true);
    let mut flying = test_fighter(3, true);
    let mut session = test_session(ships, Vec::new(), true);
    let target = engagement_priorities(&session, false, false)
        .into_iter()
        .find_map(|unit| match unit {
            EngagementUnit::Capital(index) => Some(index),
            EngagementUnit::Fighter(_) => None,
        })
        .expect("an Alliance ship");
    docked.carrier = Some(TacticalBerth::Ship(target));
    docked.docked = Some(TacticalDock::Ready);
    flying.carrier = Some(TacticalBerth::Ship(target));
    session.fighters = vec![docked, flying];
    let mut station = test_death_star(false);
    station.laser_charge = ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE;
    // The shield keeps the Alliance from the trench run (FUN_005ba450);
    // the shot tests FUN_005ba420, which ignores it.
    station.shielded = true;
    session.death_star = Some(station);

    session.headless_death_star(true);
    assert!(session.ships[target].alive, "the Alliance side never fires it");
    assert!(session.fighters[1].alive);
    session.headless_death_star(false);

    assert!(!session.ships[target].alive);
    assert_eq!(session.ships[target].hull_current, 0);
    assert_eq!(session.ships.iter().filter(|ship| ship.alive).count(), 2);
    assert_eq!(session.fighters[0].squad_count, 0, "its hangar goes with it");
    assert!(session.fighters[1].alive, "a squadron out of it flies on");
    assert_eq!(session.death_star.unwrap().laser_charge, 0.0);
    assert!(session.death_star_beam.is_none());
    assert!(session.pending_voice_cues.is_empty() && session.pending_audio_cues.is_empty());

    session.headless_death_star(false);
    assert_eq!(session.death_star.unwrap().laser_charge, 1.0 / 3.0 * 0.001);
    assert_eq!(session.ships.iter().filter(|ship| ship.alive).count(), 2);

    let mut unfinished = station;
    unfinished.not_completed = true;
    session.death_star = Some(unfinished);
    session.headless_death_star(false);
    assert_eq!(session.ships.iter().filter(|ship| ship.alive).count(), 2);
}

/// A squadron that never misses its maneuver (`FUN_005ba270` gives 9 at
/// maneuverability 8) and, with no torpedoes, never succeeds.
/// port: test fixture values, not rules.
fn steady_squadron() -> TacticalFighter {
    let mut fighter = trench_squadron(4);
    fighter.maneuverability = 8;
    fighter.order = TacticalOrder::None;
    fighter
}

// FUN_005d14d0.c:20-40: the Alliance side's headless step sends its first
// fighter group on the trench run (FUN_005aefc0) while the station is
// attackable (FUN_005ba450) and the Empire lists strictly fewer squadrons
// (FUN_005c5eb0); every attempt draws from the battle generator.
#[test]
fn the_headless_alliance_runs_while_it_outnumbers_the_empires_squadrons_at_an_open_station() {
    let session_with = |alliance: usize, empire: usize, shielded: bool| {
        let mut fighters = vec![steady_squadron(); alliance];
        fighters.extend((0..empire).map(|_| test_fighter(4, false)));
        let mut session = test_session(Vec::new(), fighters, true);
        let mut station = test_death_star(false);
        station.shielded = shielded;
        session.death_star = Some(station);
        session
    };
    let ran = |mut session: BattleSession| {
        session.headless_death_star(true);
        session.battle_rng != OriginalBattleRng::default()
    };

    assert!(ran(session_with(2, 1, false)));
    assert!(!ran(session_with(1, 1, false)), "a strict majority");
    let mut session = session_with(2, 1, false);
    session.fighters[1].squad_count = 0;
    assert!(!ran(session), "a squadron with no craft is not listed");
    assert!(!ran(session_with(2, 1, true)), "the shield closes it");
    let mut session = session_with(2, 1, false);
    session.headless_death_star(false);
    assert_eq!(session.battle_rng, OriginalBattleRng::default(), "the Empire never runs");
}

// FUN_005d14d0.c:28-37 sends the side's first group of kind 4 (its +0x70
// list). port: the lowest fighter group holding a live squadron of the side
// in its battle lists.
#[test]
fn a_headless_trench_run_sends_the_sides_lowest_group_of_live_listed_squadrons() {
    let squadron = |group, edit: fn(&mut TacticalFighter)| {
        let mut fighter = steady_squadron();
        fighter.fighter_group = group;
        edit(&mut fighter);
        fighter
    };
    let fighters = vec![
        squadron(0, |fighter| fighter.is_attacker = false),
        squadron(0, |fighter| fighter.alive = false),
        squadron(0, |fighter| fighter.squad_count = 0),
        squadron(0, |fighter| fighter.off_field = true),
        squadron(2, |_| {}),
        squadron(1, |_| {}),
        squadron(1, |_| {}),
    ];
    let session = test_session(Vec::new(), fighters, true);

    assert_eq!(session.first_fighter_group(true), vec![5, 6]);
    assert_eq!(session.first_fighter_group(false), vec![0]);
}

// FUN_005aefc0: the walk visits the members round in id order, survivors
// keeping their place, and the hundred-and-first visit (CMP EDI,0x64 at
// 0x5af047) destroys every member left. A visit draws rand(1,10), then,
// alive, rand(1,100).
#[test]
fn a_headless_trench_run_ends_with_every_member_left_destroyed_after_a_hundred_visits() {
    let mut session = trench_session(vec![steady_squadron(), steady_squadron()], 1);

    session.headless_trench_run(vec![0, 1]);

    assert!(session.fighters.iter().all(|fighter| fighter.squad_count == 0 && !fighter.alive));
    assert!(session.death_star.unwrap().operational());
    assert_eq!(session.battle_rng, advanced(OriginalBattleRng::default(), 200));
}

// FUN_005aefc0 (0x5af0ba-0x5af0d8): a member that misses its maneuver with
// fewer than two craft is destroyed with no further draw, and the walk goes
// on to the next member.
#[test]
fn a_headless_member_of_one_craft_that_misses_dies_without_a_draw() {
    let mut lone = trench_squadron(1);
    lone.sublight_engine_power = 0.0;
    let mut session = trench_session(vec![lone, steady_squadron()], 1);
    assert!(OriginalBattleRng::default().between(1, 10) > 1, "the fixed stream misses");

    session.headless_trench_run(vec![0, 1]);

    assert!(session.fighters.iter().all(|fighter| fighter.squad_count == 0));
    assert_eq!(session.battle_rng, advanced(OriginalBattleRng::default(), 1 + 99 * 2));
}

// FUN_005aefc0: a miss takes rand(1,n) craft through FUN_005b54d0 (no
// shield, so one draw for the overflow); a survivor's rand(1,100) at or
// under the chance succeeds, and FUN_005ba8a0 zeroes the station's hull
// and charge, then the station's two hangar lists are destroyed
// (0x5af38b-0x5af3f4). The runners keep their damage.
#[test]
fn a_headless_success_zeroes_the_station_destroys_its_hangar_and_keeps_the_damage() {
    let mut runner = trench_squadron(4);
    runner.sublight_engine_power = 0.0;
    runner.shield = 0.0;
    runner.torpedo_strength = 10_000;
    let mut docked = test_fighter(4, false);
    docked.carrier = Some(TacticalBerth::Station);
    docked.docked = Some(TacticalDock::Recovered);
    let mut launched = test_fighter(4, false);
    launched.carrier = Some(TacticalBerth::Station);
    let mut session = trench_session(vec![runner, docked, launched], 1);
    let mut oracle = OriginalBattleRng::default();
    assert!(oracle.between(1, 10) > 1, "the fixed stream misses");
    let craft = oracle.between(1, 4);
    oracle.between(0, 100);
    oracle.between(1, 100);

    session.headless_trench_run(vec![0]);

    let station = session.death_star.unwrap();
    assert_eq!((station.hull, station.laser_charge), (0.0, 0.0));
    assert_eq!(session.fighters[0].hull_current, 32.0 - craft as f32 * 8.0);
    assert_eq!(session.fighters[1].squad_count, 0);
    assert!(session.fighters[2].alive);
    assert_eq!(session.battle_rng, oracle);
}

// FUN_005aefc0 (0x5af0ba-0x5af0d8): a member of two craft or more that
// misses takes rand(1, craft) * 8 shield first and, its hull left, rolls on.
#[test]
fn a_headless_member_of_two_craft_that_misses_takes_its_damage_and_rolls() {
    let mut runner = trench_squadron(2);
    runner.sublight_engine_power = 0.0;
    runner.shield = 0.0;
    runner.torpedo_strength = 10_000;
    let mut session = trench_session(vec![runner], 1);
    let mut oracle = OriginalBattleRng::default();
    assert!(oracle.between(1, 10) > 1, "the fixed stream misses");
    assert_eq!(oracle.between(1, 2), 1, "a span of one draws nothing");
    oracle.between(0, 100);
    oracle.between(1, 100);

    session.headless_trench_run(vec![0]);

    assert_eq!(session.death_star.unwrap().hull, 0.0);
    assert_eq!(session.fighters[0].hull_current, 8.0);
    assert_eq!(session.battle_rng, oracle);
}

// FUN_005aefc0 walks the members round in id order; a destroyed member's
// place goes to the one after it.
#[test]
fn the_headless_walk_goes_on_to_the_member_after_one_destroyed() {
    let mut lone = trench_squadron(1);
    lone.sublight_engine_power = 0.0;
    let mut sure = steady_squadron();
    sure.torpedo_strength = 10_000;
    let mut session = trench_session(vec![steady_squadron(), lone, sure], 1);
    let mut oracle = OriginalBattleRng::default();
    oracle.between(1, 10);
    oracle.between(1, 100);
    assert!(oracle.between(1, 10) > 1, "the fixed stream misses");
    oracle.between(1, 10);
    oracle.between(1, 100);

    session.headless_trench_run(vec![0, 1, 2]);

    assert_eq!(session.death_star.unwrap().hull, 0.0);
    assert!(!session.fighters[1].alive);
    assert!(session.fighters[0].alive);
    assert_eq!(session.battle_rng, oracle);
}

// FUN_005b54d0 on a squadron: the shield takes the damage first with a draw
// while it had any; an overflow, at least one point, takes the hull with a
// second draw. A squadron without hull takes nothing.
#[test]
fn a_headless_squadron_hit_draws_for_its_shield_and_for_its_overflow() {
    let hit = |shield: f32, hull: f32, damage: f32| {
        let mut fighter = steady_squadron();
        fighter.shield = shield;
        fighter.hull_current = hull;
        let mut session = trench_session(vec![fighter], 1);
        session.damage_squadron_shield_first(0, damage);
        let draws = (0..3)
            .find(|&n| advanced(OriginalBattleRng::default(), n) == session.battle_rng)
            .expect("at most two draws");
        (session.fighters[0].shield, session.fighters[0].hull_current, draws)
    };

    assert_eq!(hit(5.0, 32.0, 3.0), (2.0, 32.0, 1));
    assert_eq!(hit(5.0, 32.0, 5.0), (0.0, 32.0, 1));
    assert_eq!(hit(5.0, 32.0, 8.0), (0.0, 29.0, 2));
    assert_eq!(hit(0.0, 32.0, 3.0), (0.0, 29.0, 1));
    assert_eq!(hit(5.0, 0.0, 8.0), (5.0, 0.0, 0));
}

// FUN_005ae9c0 steps the Empire side (battle+0x250) first, and its
// FUN_005d14d0 fires the station before any damage (lines 41-60, then
// 84-100): the station's full charge takes a ship on the first step.
#[test]
fn a_battle_run_without_its_window_fires_the_station_on_its_first_step() {
    let ships = vec![
        test_ship(0x1400_0001, 0, true, true),
        test_ship(0x1400_0003, 0, false, true),
    ];
    let mut session = test_session(ships, Vec::new(), true);
    let mut station = test_death_star(false);
    station.laser_charge = ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE;
    session.death_star = Some(station);

    session.headless_step();

    assert!(!session.ships[0].alive);
    assert_eq!(
        session.death_star.unwrap().laser_charge,
        0.0,
        "the charge is added before the fire test (line 44, then 47)"
    );
}

/// `FUN_005d4d10`'s offset `rand(trunc(2 * extent * scale)) - extent * scale`
/// for the `draw`-th draw (1-based) of a generator seeded like the battle's
/// (12345678, sixteen warm-up steps): the station's `y` is draw 1, `z` draw 2.
fn surround_jitter(extent: f32, scale: f32, draw: usize) -> f32 {
    let mut state = 12_345_678_i64;
    for _ in 0..16 + draw {
        state = lehmer(state);
    }
    let half = extent * scale;
    let span = (2.0 * f64::from(extent) * f64::from(scale)).trunc() as i64;
    let roll = state / (0x7fff_fffe / span);
    (roll as f64 - f64::from(half)) as f32
}

// FUN_005d4d10 (0x5d5505-0x5d55a4, FUN_005d4d10.asm.txt): the station decal's
// frame sits in the scene at (-2.0 * extent, rand(0.4E) - 0.2E,
// rand(0.4E) - 0.2E) (_DAT_0066d248 = -2.0, _DAT_0066d244 = 0.2), whichever
// side owns or attacks it, off the face opposite the planet's. VM:
// alliance-day1169-deathstar-trench-run.sav, day 1173, zoomed out: the
// decal stands outside the battle box off one face, the planet off the other.
#[test]
fn the_station_stands_two_extents_out_on_negative_x_for_either_side() {
    let (world, system, _, _) = station_world(false, 0);
    for layout in [
        OriginalTacticalLayout::default(),
        OriginalTacticalLayout::from_active_counts(4, 0),
    ] {
        let extent = layout.battle_extent;
        let expected = TacticalWorldPosition {
            x: -2.0 * extent,
            y: surround_jitter(extent, 0.2, 1),
            z: surround_jitter(extent, 0.2, 2),
        };
        for is_alliance in [true, false] {
            for is_attacker in [true, false] {
                let station =
                    TacticalDeathStar::seated(&world, system, is_alliance, is_attacker, layout);
                assert_eq!(station.source_position, expected, "extent {extent}");
                assert!(expected.y.abs() <= 0.2 * extent && expected.z.abs() <= 0.2 * extent);
            }
        }
    }
}

// FUN_005ba520 runs the station actor's slot 8 through FUN_005c12d0, and
// FUN_005d59e0 builds the shot on the decal's child frame +0x4c, which
// FUN_005d4d10 sets at (0, 0.11 * extent, -0.09 * extent) from the station
// (_DAT_0066d258 = 0.11, _DAT_0066d250 = 0.09, 0x5d55c6-0x5d55f7).
#[test]
fn the_superlaser_leaves_from_the_stations_child_frame_for_either_side() {
    for player_is_attacker in [true, false] {
        let mut target = test_ship(64, 0, true, true);
        target.is_attacker = !player_is_attacker;
        let mut session = test_session(vec![target], Vec::new(), player_is_attacker);
        session.source_layout = OriginalTacticalLayout::from_active_counts(4, 0);
        let station = TacticalWorldPosition {
            x: -224.0,
            y: 3.0,
            z: -5.0,
        };
        session.death_star = Some(TacticalDeathStar {
            source_position: station,
            laser_charge: ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE,
            ..test_death_star(player_is_attacker)
        });
        assert!(session.commit_death_star_shot(0));
        let beam = session.death_star_beam.expect("the shot is under way");
        assert_eq!(
            beam.source_position,
            TacticalWorldPosition {
                x: -224.0,
                y: 3.0 + (112.0_f64 * 0.11) as f32,
                z: -5.0 - (112.0_f64 * 0.09) as f32,
            }
        );
    }
}
