//! A played battle's side withdrawal (`ghidra/notes/morale-retreat.md`,
//! "Windowed withdrawal").

use super::test_support::{test_death_star, test_fighter, test_session, test_ship};
use super::*;

/// port: test fixture values, not rules.
const EXTENT: f32 = 100.0;

/// A ship of the side with a working hyperdrive, or none, at `z`.
/// port: test fixture values, not rules.
fn ship(is_alliance: bool, task_force: u8, hyperdrive: bool, z: f32) -> TacticalShip {
    let mut ship = test_ship(0x1400_0001, usize::from(task_force), is_alliance, true);
    ship.task_force = task_force;
    ship.hull_current = 100;
    ship.hull_max = 100;
    let drive = u8::from(hyperdrive);
    ship.subsystem_capacity.hyperdrive = drive;
    ship.subsystem_condition.hyperdrive = drive;
    ship.source_position = TacticalWorldPosition { x: 3.0, y: 2.0, z };
    ship
}

/// Alliance (the attacker) against the Empire, extent 100.
fn battle(ships: Vec<TacticalShip>, fighters: Vec<TacticalFighter>) -> BattleSession {
    let mut session = test_session(ships, fighters, true);
    session.source_layout.battle_extent = EXTENT;
    session
}

/// The side's own edge: side 0 (the Alliance) faces +z, side 1 -z
/// (`FUN_005be160.c:27-45`), so its line lies at -/+ 0.5 x extent.
fn line(is_alliance: bool) -> f32 {
    if is_alliance {
        -0.5 * EXTENT
    } else {
        0.5 * EXTENT
    }
}

fn steps(session: &mut BattleSession, milliseconds: f32) {
    let mut left = milliseconds;
    while left > 0.0 {
        session.advance_side_withdrawals(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        left -= ORIGINAL_TACTICAL_STEP_MILLISECONDS;
    }
}

// FUN_005ad530.c:20 and :83-100: the controller's +0x84 becomes 2 and its
// countdown +0x7c at least 30000.0 (DAT_0066c240) without an operational
// station on side 1; FUN_005c8780.c:28-31 counts it down by the update's
// milliseconds and moves to state 3 at 0.0 (_DAT_0066cbb4).
#[test]
fn a_side_withdrawal_counts_down_thirty_seconds_then_holds() {
    for is_alliance in [true, false] {
        let mut session = battle(vec![ship(is_alliance, 0, true, 0.0)], Vec::new());
        let side = usize::from(!is_alliance);
        session.begin_side_withdrawal(is_alliance);
        assert_eq!(
            session.morale.withdrawal[side],
            TacticalWithdrawal {
                state: TacticalWithdrawalState::CountingDown,
                countdown_milliseconds: 30_000.0,
            }
        );
        assert_eq!(
            session.morale.withdrawal[1 - side],
            TacticalWithdrawal::default()
        );
        steps(&mut session, 29_750.0);
        assert_eq!(
            session.morale.withdrawal[side].state,
            TacticalWithdrawalState::CountingDown
        );
        steps(&mut session, 250.0);
        assert_eq!(
            session.morale.withdrawal[side].state,
            TacticalWithdrawalState::Holding
        );
    }
}

// FUN_005ad530.c:83-100: side 1 (the Empire) with an operational station
// (FUN_005a9330, FUN_005ba420) waits at least 150000.0 ms (DAT_0066c234);
// a longer countdown already running is kept. FUN_005c8780.c:31-47: at the
// end of the countdown side 1 posts 0x45 (FUN_005ba5e0): the station departs
// (+0x78) and recalls the squadrons waiting in its hangar (FUN_005b8d10).
#[test]
fn the_empire_with_its_station_waits_150_seconds_and_the_station_departs() {
    let mut docked = test_fighter(0x1c00_0005, false);
    docked.carrier = Some(TacticalBerth::Station);
    docked.docked = Some(TacticalDock::Ready);
    let mut flying = test_fighter(0x1c00_0005, false);
    flying.carrier = Some(TacticalBerth::Station);
    let mut session = battle(vec![ship(false, 0, true, 0.0)], vec![docked, flying]);
    session.death_star = Some(test_death_star(false));
    session.begin_side_withdrawal(false);
    assert_eq!(
        session.morale.withdrawal[1].countdown_milliseconds,
        150_000.0
    );
    steps(&mut session, 149_750.0);
    assert!(!session.death_star.unwrap().action_committed);
    steps(&mut session, 250.0);
    assert!(
        session.death_star.unwrap().action_committed,
        "0x45 departure"
    );
    assert_eq!(session.fighters[0].docked, Some(TacticalDock::Recovered));
    assert_eq!(session.fighters[1].docked, None, "in flight, not recalled");

    // The Alliance's withdrawal waits 30 s whatever the station does.
    let mut session = battle(vec![ship(true, 0, true, 0.0)], Vec::new());
    session.death_star = Some(test_death_star(false));
    session.begin_side_withdrawal(true);
    assert_eq!(
        session.morale.withdrawal[0].countdown_milliseconds,
        30_000.0
    );
    steps(&mut session, 30_000.0);
    assert!(!session.death_star.unwrap().action_committed);

    // An uncompleted station is not operational: 30 s and no departure.
    let mut session = battle(vec![ship(false, 0, true, 0.0)], Vec::new());
    let mut station = test_death_star(false);
    station.not_completed = true;
    session.death_star = Some(station);
    session.begin_side_withdrawal(false);
    assert_eq!(
        session.morale.withdrawal[1].countdown_milliseconds,
        30_000.0
    );
    steps(&mut session, 30_000.0);
    assert!(!session.death_star.unwrap().action_committed);

    // A longer countdown stands.
    let mut session = battle(vec![ship(true, 0, true, 0.0)], Vec::new());
    session.morale.withdrawal[0].countdown_milliseconds = 200_000.0;
    session.begin_side_withdrawal(true);
    assert_eq!(
        session.morale.withdrawal[0].countdown_milliseconds,
        200_000.0
    );
}

// FUN_005ad530.c:32-81: every capital ship's group is ordered to Withdraw
// (3); a fighter group whose first squadron has a hyperdrive rating
// (FUN_005b1bc0) Withdraws too, one with none is ordered to Recover (2). The
// other side's orders stand.
#[test]
fn withdrawal_orders_ships_and_hyperdrive_groups_out_and_the_rest_to_recover() {
    for is_alliance in [true, false] {
        let mut jumper = test_fighter(0x1c00_0001, is_alliance);
        jumper.fighter_group = 0;
        let mut short = test_fighter(0x1c00_0002, is_alliance);
        short.fighter_group = 1;
        let mut enemy = test_fighter(0x1c00_0003, !is_alliance);
        enemy.order = TacticalOrder::AttackFighters;
        let mut carrier = ship(is_alliance, 0, true, 0.0);
        carrier.fighter_capacity = 1;
        let mut session = battle(
            vec![carrier, ship(!is_alliance, 0, true, 0.0)],
            vec![jumper, short, enemy],
        );
        session.morale.hyperdrive_squadrons = vec![0];
        session.begin_side_withdrawal(is_alliance);
        assert_eq!(session.ships[0].order, TacticalOrder::Withdraw);
        assert_eq!(session.ships[1].order, TacticalOrder::None);
        assert_eq!(session.fighters[0].order, TacticalOrder::Withdraw);
        assert_eq!(session.fighters[1].order, TacticalOrder::Recover);
        // FUN_005ca6d0 case 2 begins the recovery (begin_original_fighter_recovery).
        assert_eq!(session.fighters[1].recovery_target, Some(0));
        assert_eq!(session.fighters[0].recovery_target, None);
        assert_eq!(session.fighters[2].order, TacticalOrder::AttackFighters);
        assert_eq!(TacticalOrder::Withdraw.source_code(), 3);
        assert_eq!(TacticalOrder::Withdraw.label(), "Withdraw");
    }
}

// FUN_005cc300.c:48-145: the task force flies to its side's edge, z at
// 0.5 x battle+0x8cc on the side opposite its facing, x and y kept. It jumps
// only once the countdown is out (_DAT_0066d000) and its front ship is
// within 8.0 (_DAT_0066cff0) of the line; the jump (state 6, 0x2d,
// FUN_005b1e40) is state 7 for 4000.0 ms (FUN_005a6b40, _DAT_0066c24c),
// then state 8: the ship has left (0x005aa2ad).
#[test]
fn a_withdrawing_ship_flies_to_its_edge_and_jumps_after_the_countdown() {
    for is_alliance in [true, false] {
        let mut session = battle(vec![ship(is_alliance, 0, true, 0.0)], Vec::new());
        session.ships[0].navigation_route = vec![TacticalWorldPosition::ORIGIN];
        session.ships[0].source_waypoint = Some(TacticalWorldPosition::ORIGIN);
        session.begin_side_withdrawal(is_alliance);
        assert!(
            session.ships[0].navigation_route.is_empty(),
            "the old route is dropped"
        );
        steps(&mut session, 250.0);
        assert_eq!(
            session.ships[0].source_waypoint,
            Some(TacticalWorldPosition {
                x: 3.0,
                y: 2.0,
                z: line(is_alliance),
            })
        );
        session.ships[0].source_position.z = line(is_alliance);
        steps(&mut session, 10_000.0);
        assert_eq!(
            session.ships[0].source_waypoint, None,
            "it holds at the line"
        );
        assert!(
            !session.ships[0].retreating,
            "the countdown is still running"
        );
        steps(&mut session, 19_750.0);
        assert!(session.ships[0].retreating, "state 7");
        assert!(session.ships[0].alive);
        steps(&mut session, 3_750.0);
        assert!(session.ships[0].alive);
        steps(&mut session, 250.0);
        assert!(!session.ships[0].alive);
        assert!(session.ships[0].retreated);
        assert!(!session.ships[0].retreating);
        assert!(session.morale.jumped[usize::from(!is_alliance)]);
    }
}

// FUN_005cc300.c:84-101: only the task force's front ship (FUN_005c6070)
// may jump; while it is in state 7 the next waits.
#[test]
fn a_task_force_jumps_one_ship_at_a_time() {
    for is_alliance in [true, false] {
        let mut session = battle(
            vec![
                ship(is_alliance, 0, true, line(is_alliance)),
                ship(is_alliance, 0, true, line(is_alliance)),
            ],
            Vec::new(),
        );
        session.begin_side_withdrawal(is_alliance);
        steps(&mut session, 30_000.0);
        assert!(session.ships[0].retreating, "{is_alliance}");
        assert!(!session.ships[1].retreating, "{is_alliance}");
        steps(&mut session, 4_000.0);
        assert!(session.ships[0].retreated, "{is_alliance}");
        steps(&mut session, 250.0);
        assert!(session.ships[1].retreating, "{is_alliance}");
    }
}

// FUN_005c6610.c:44-110: a withdrawing fighter group flies to the line and,
// once the countdown is out, each squadron with a working hyperdrive within
// 8.0 (_DAT_0066cb0c) jumps; FUN_005c8780.c:72-125: once no unit of the side
// with a hyperdrive is left in play, state 4 sends 0x2f to the units
// without one (FUN_005b2720), which drop their orders and fight on.
#[test]
fn hyperdrive_squadrons_jump_and_the_units_left_behind_fight_on() {
    for is_alliance in [true, false] {
        let mut squadron = test_fighter(0x1c00_0001, is_alliance);
        squadron.source_position.z = line(is_alliance);
        let mut session = battle(
            vec![ship(is_alliance, 0, false, line(is_alliance))],
            vec![squadron],
        );
        session.morale.hyperdrive_squadrons = vec![0];
        session.begin_side_withdrawal(is_alliance);
        let side = usize::from(!is_alliance);
        steps(&mut session, 30_000.0);
        assert!(session.fighters[0].alive, "state 7");
        assert_eq!(
            session.morale.withdrawal[side].state,
            TacticalWithdrawalState::Holding
        );
        assert_eq!(session.ships[0].order, TacticalOrder::Withdraw);
        steps(&mut session, 4_000.0);
        assert!(!session.fighters[0].alive);
        assert_eq!(session.fighters[0].squad_count, 12, "it left, not lost");
        assert!(session.morale.jumped[side]);
        steps(&mut session, 250.0);
        assert_eq!(
            session.morale.withdrawal[side].state,
            TacticalWithdrawalState::Over
        );
        assert_eq!(session.ships[0].order, TacticalOrder::None, "0x2f");
        assert!(session.ships[0].alive);
        assert!(!session.ships[0].retreating);
    }
}

/// A withdrawing ship that cannot move (power 0) at `short` short of its
/// side's line, the countdown already out.
fn parked(is_alliance: bool, short: f32) -> BattleSession {
    let facing = if is_alliance { 1.0 } else { -1.0 };
    let mut stuck = ship(is_alliance, 0, true, line(is_alliance) + facing * short);
    stuck.sublight_engine_power = 0.0;
    let mut session = battle(vec![stuck], Vec::new());
    session.begin_side_withdrawal(is_alliance);
    session.morale.withdrawal[usize::from(!is_alliance)].countdown_milliseconds = 0.0;
    session
}

// FUN_005cc300.c:101 (FUN_005c6610.c:81 alike): the unit may jump while
// (z - line z) x facing <= 8.0 (_DAT_0066cff0): up to 8.0 short of the
// line, or beyond it.
#[test]
fn a_unit_jumps_only_within_eight_of_its_line() {
    for is_alliance in [true, false] {
        for (short, jumps) in [(9.0, false), (8.0, true), (7.0, true), (-30.0, true)] {
            let mut session = parked(is_alliance, short);
            session.advance_side_withdrawals(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
            assert_eq!(session.ships[0].retreating, jumps, "{is_alliance} {short}");
        }
    }
}

// The move call reports arrival within the tolerance, 1.0 for the Withdraw
// executors (FUN_005cc300.c:75); a ship short of its line or beyond it steers
// for it, one within 1.0 holds.
#[test]
fn a_withdrawing_ship_steers_for_its_line_and_holds_within_one() {
    for is_alliance in [true, false] {
        let facing = if is_alliance { 1.0 } else { -1.0 };
        for (start, steers) in [(-20.0, true), (30.0, true), (1.5, true), (1.0, false)] {
            let z = line(is_alliance) + facing * start;
            let mut session = battle(vec![ship(is_alliance, 0, true, z)], Vec::new());
            session.ships[0].source_desired_forward = TacticalWorldVector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
            session.begin_side_withdrawal(is_alliance);
            session.advance_side_withdrawals(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
            let expected = steers.then_some(TacticalWorldPosition {
                x: 3.0,
                y: 2.0,
                z: line(is_alliance),
            });
            assert_eq!(
                session.ships[0].source_waypoint, expected,
                "{is_alliance} {start}"
            );
            if !steers {
                assert_eq!(
                    session.ships[0].source_desired_forward,
                    TacticalWorldVector::ZERO
                );
            }
        }
    }
}

// The executor holds order 3 until the side's controller ends it (0x2f):
// reaching the line is no order completion.
#[test]
fn reaching_the_line_keeps_the_withdraw_order() {
    let mut session = battle(vec![ship(true, 0, true, 0.0)], Vec::new());
    session.ships[0].order = TacticalOrder::Withdraw;
    session.ships[0].source_waypoint = Some(session.ships[0].source_position);
    session.advance_original_tactical_movement(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
    assert_eq!(session.ships[0].source_waypoint, None);
    assert_eq!(session.ships[0].order, TacticalOrder::Withdraw);
}

// FUN_005ad530 orders only the withdrawing side; the executors move only
// living units of that side holding order 3.
#[test]
fn only_the_withdrawing_sides_living_units_under_withdraw_move() {
    for is_alliance in [true, false] {
        let mut idle = ship(is_alliance, 1, true, 0.0);
        idle.order = TacticalOrder::None;
        let dead = ship(is_alliance, 2, true, 0.0);
        let mut other = ship(!is_alliance, 3, true, 0.0);
        other.order = TacticalOrder::Withdraw;
        let mut idle_squadron = test_fighter(0x1c00_0001, is_alliance);
        idle_squadron.fighter_group = 1;
        let mut docked = test_fighter(0x1c00_0001, is_alliance);
        docked.docked = Some(TacticalDock::Ready);
        let mut other_squadron = test_fighter(0x1c00_0001, !is_alliance);
        other_squadron.order = TacticalOrder::Withdraw;
        let mut session = battle(
            vec![ship(is_alliance, 0, true, 0.0), idle, dead, other],
            vec![
                test_fighter(0x1c00_0001, is_alliance),
                idle_squadron,
                docked,
                other_squadron,
            ],
        );
        session.morale.hyperdrive_squadrons = vec![0, 1, 2, 3];
        session.begin_side_withdrawal(is_alliance);
        session.ships[1].order = TacticalOrder::None;
        session.ships[2].alive = false;
        session.fighters[1].order = TacticalOrder::None;
        // A docked squadron waits in its hangar whatever its order.
        session.fighters[2].order = TacticalOrder::Withdraw;
        session.advance_side_withdrawals(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        assert!(session.ships[0].source_waypoint.is_some());
        for index in 1..4 {
            assert_eq!(session.ships[index].source_waypoint, None, "ship {index}");
        }
        assert_ne!(session.fighters[0].source_position.z, 0.0);
        for index in 1..4 {
            assert_eq!(
                session.fighters[index].source_position.z, 0.0,
                "squadron {index}"
            );
        }
    }
}

// FUN_005a6b40: a squadron's jump lasts 4000.0 ms like a ship's; a squadron
// in its jump is not moved again.
#[test]
fn a_squadron_stays_in_its_jump_for_four_seconds() {
    for is_alliance in [true, false] {
        let mut squadron = test_fighter(0x1c00_0001, is_alliance);
        squadron.source_position.z = line(is_alliance);
        let mut session = battle(Vec::new(), vec![squadron]);
        session.morale.hyperdrive_squadrons = vec![0];
        session.begin_side_withdrawal(is_alliance);
        session.morale.withdrawal[usize::from(!is_alliance)].countdown_milliseconds = 0.0;
        session.advance_side_withdrawals(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        assert_eq!(session.morale.squadron_jumps, vec![(0, 0.0)]);
        session.fighters[0].source_position.z = 0.0;
        steps(&mut session, 3_750.0);
        assert!(session.fighters[0].alive);
        assert_eq!(
            session.fighters[0].source_position.z, 0.0,
            "not moved in its jump"
        );
        steps(&mut session, 250.0);
        assert!(!session.fighters[0].alive);
    }
}

// FUN_005c8780.c:72-101 counts only the side's own units: the other side's
// hyperdrive squadron does not hold this side's withdrawal.
#[test]
fn the_other_sides_units_do_not_hold_a_withdrawal() {
    for is_alliance in [true, false] {
        let mut session = battle(
            vec![ship(is_alliance, 0, false, 0.0)],
            vec![test_fighter(0x1c00_0001, !is_alliance)],
        );
        session.morale.hyperdrive_squadrons = vec![0];
        session.begin_side_withdrawal(is_alliance);
        steps(&mut session, 30_250.0);
        assert_eq!(
            session.morale.withdrawal[usize::from(!is_alliance)].state,
            TacticalWithdrawalState::Over
        );
    }
}

// FUN_005b2720 (0x2f): only a unit of the side, alive (FUN_005ad870) with
// hull above 0.0 and without a working hyperdrive, drops its orders.
#[test]
fn the_units_left_behind_are_the_sides_living_ones_without_a_hyperdrive() {
    for is_alliance in [true, false] {
        let mut ships = vec![
            ship(is_alliance, 0, false, 0.0),
            ship(is_alliance, 1, false, 0.0),
            ship(is_alliance, 2, false, 0.0),
            ship(is_alliance, 3, true, 0.0),
            ship(!is_alliance, 4, false, 0.0),
        ];
        ships[1].alive = false;
        ships[2].hull_current = 0;
        let mut squadrons = vec![
            test_fighter(0x1c00_0001, is_alliance),
            test_fighter(0x1c00_0001, is_alliance),
            test_fighter(0x1c00_0001, is_alliance),
            test_fighter(0x1c00_0001, is_alliance),
            test_fighter(0x1c00_0001, !is_alliance),
            test_fighter(0x1c00_0001, is_alliance),
        ];
        squadrons[1].alive = false;
        squadrons[2].docked = Some(TacticalDock::Ready);
        squadrons[5].hull_current = 0.0;
        let mut session = battle(ships, squadrons);
        session.morale.hyperdrive_squadrons = vec![3];
        for ship in &mut session.ships {
            ship.order = TacticalOrder::Withdraw;
        }
        for fighter in &mut session.fighters {
            fighter.order = TacticalOrder::Withdraw;
        }
        session.recall_units_left_behind(is_alliance);
        let ships: Vec<_> = session.ships.iter().map(|ship| ship.order).collect();
        assert_eq!(
            ships,
            [
                TacticalOrder::None,
                TacticalOrder::Withdraw,
                TacticalOrder::Withdraw,
                TacticalOrder::Withdraw,
                TacticalOrder::Withdraw,
            ]
        );
        let squadrons: Vec<_> = session
            .fighters
            .iter()
            .map(|fighter| fighter.order)
            .collect();
        assert_eq!(
            squadrons,
            [
                TacticalOrder::None,
                TacticalOrder::Withdraw,
                TacticalOrder::Withdraw,
                TacticalOrder::Withdraw,
                TacticalOrder::Withdraw,
                TacticalOrder::Withdraw,
            ]
        );
    }
}

// FUN_005c6610.c:44-75 -> FUN_005eb1a0: a withdrawing squadron flies along z
// to its line at its speed x 0.95 x 0.001 per ms, from either side of it,
// and holds within the tolerance 1.0.
#[test]
fn a_withdrawing_squadron_flies_to_its_line_and_holds_within_one() {
    for is_alliance in [true, false] {
        let facing = if is_alliance { 1.0 } else { -1.0 };
        let travel = 10.0 * 0.95 * 0.25;
        for (start, end) in [
            (30.0, 30.0 - travel),
            (-30.0, -30.0 + travel),
            (1.5, 0.0),
            (1.0, 1.0),
        ] {
            let mut squadron = test_fighter(0x1c00_0001, is_alliance);
            squadron.source_position = TacticalWorldPosition {
                x: 3.0,
                y: 2.0,
                z: line(is_alliance) + facing * start,
            };
            let mut session = battle(Vec::new(), vec![squadron]);
            session.morale.hyperdrive_squadrons = vec![0];
            session.begin_side_withdrawal(is_alliance);
            session.advance_side_withdrawals(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
            let position = session.fighters[0].source_position;
            assert_eq!((position.x, position.y), (3.0, 2.0));
            let left = (position.z - line(is_alliance)) * facing;
            assert!((left - end).abs() < 1e-4, "{is_alliance} {start}: {left}");
        }
    }
}

// FUN_005ba5e0 posts the station's departure only for side 1 (the Empire)
// and only while the station is operational (FUN_005ba420); a retreat by
// the side without it leaves it in place.
#[test]
fn a_retreat_takes_only_its_own_sides_operational_imperial_station_away() {
    let retreat = |station: TacticalDeathStar| {
        let mut session = battle(vec![ship(false, 0, false, 0.0)], Vec::new());
        session.death_star = Some(station);
        session.retreat_side(false);
        // The retreating side's own flag (+0xb0, FUN_005ae9c0.c:62-103).
        assert_eq!(session.morale.retreated, [false, true]);
        // The departure's +0x78 (FUN_005ba5e0).
        session.death_star.unwrap().action_committed
    };
    assert!(retreat(test_death_star(false)));
    let mut unfinished = test_death_star(false);
    unfinished.not_completed = true;
    assert!(!retreat(unfinished), "an unfinished station departed");
    assert!(!retreat(test_death_star(true)), "the other side's station departed");
    let mut rebel = test_death_star(false);
    rebel.is_alliance = true;
    assert!(!retreat(rebel), "an Alliance station departed");
}

// The countdown's start: FUN_00516730.c:185-196 measures the transit to the
// side's retreat system (FUN_0055d8a0, at the default speed), FUN_00595d60
// scales it by 0.4 (_DAT_0066babc) and FUN_005a8b10 by 1000.0
// (_DAT_0066c288) into the side's +0x7c; FUN_005ad530.c:84-100 only raises
// it to the floor. A side with no retreat system keeps p5's 0
// (FUN_00516730.c:46); the chain's stages 6 and 7 leave that to a map with
// no other system.
#[test]
fn a_side_withdrawal_counts_down_its_transit_to_the_retreat_system() {
    use crate::world::{CapitalShipClass, ControlKind, Fleet, ShipInstance, System};

    for refuge_side in [crate::dat::Faction::Alliance, crate::dat::Faction::Empire] {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        // port: test fixture values, not rules.
        let refuge = world.systems.insert(System {
            x: 500,
            control: ControlKind::Controlled(refuge_side),
            ..System::default()
        });
        let mut fleet = |is_alliance: bool| {
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(0x1400_0001),
                family: 0x14,
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..CapitalShipClass::default()
            });
            world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            })
        };
        let alliance = fleet(true);
        let empire = fleet(false);
        world.systems[system].fleets = vec![alliance, empire];
        world.systems[system].fleets.sort_unstable();

        let mut session = BattleSession::new(&world, system, alliance, empire, true, 1, 0x5eed);
        apply::seat_morale(
            &mut session,
            &world,
            &crate::troop_transport::TroopTransportState::default(),
            &crate::movement::MovementState::new(),
            Some(true),
        );
        let ticks = crate::movement::transit_ticks_between(
            &world,
            (world.systems[system].x, world.systems[system].y),
            (world.systems[refuge].x, world.systems[refuge].y),
            crate::movement::default_speed(&world),
        );
        assert_eq!(ticks, 100);
        let refuge_is_alliance = refuge_side == crate::dat::Faction::Alliance;
        // The Alliance attacks: side 0 is the Alliance's.
        let refuge_side_index = usize::from(!refuge_is_alliance);
        // The side that holds nothing retreats to the same system as the
        // other side's (stage 7, FUN_005552e0), so both countdowns start.
        let expected = [100.0 * 0.4 * 1000.0; 2];
        assert_eq!(
            session.morale.withdrawal.map(|withdrawal| withdrawal.countdown_milliseconds),
            expected,
            "{refuge_side:?}"
        );
        session.begin_side_withdrawal(refuge_is_alliance);
        assert_eq!(
            session.morale.withdrawal[refuge_side_index].countdown_milliseconds,
            40_000.0,
            "the transit outlasts the 30 s floor"
        );
    }
}

// FUN_005a6b40 on a unit's first update in state 7 (its jump time +0x50
// still 0.0, _DAT_0066c250) plays effect 0x1d for a squadron (vtable +0x20,
// FUN_0040f340, is 1) or 0x1f for a capital ship (FUN_006158b0, 0) through
// FUN_005ba980; FUN_005bae60 gives each one variant, WAVE 13063 and 13065.
#[test]
fn a_jump_sounds_once_as_it_begins() {
    for is_alliance in [true, false] {
        let mut squadron = test_fighter(0x1c00_0001, is_alliance);
        squadron.source_position.z = line(is_alliance);
        let mut session = battle(
            vec![ship(is_alliance, 0, true, line(is_alliance))],
            vec![squadron],
        );
        session.morale.hyperdrive_squadrons = vec![0];
        session.begin_side_withdrawal(is_alliance);
        steps(&mut session, 34_000.0);
        let played = |event| {
            session
                .pending_audio_cues
                .iter()
                .filter(|cue| cue.event == event)
                .map(|cue| cue.resource_id)
                .collect::<Vec<_>>()
        };
        assert_eq!(played(TacticalAudioEvent::SquadronJump), [13_063], "{is_alliance}");
        assert_eq!(played(TacticalAudioEvent::CapitalShipJump), [13_065], "{is_alliance}");
    }
}

// FUN_005a6b40 draws a unit in state 7 at its position moved against its
// side's facing (FUN_005a7490) by (t × 0.001)² × k (_DAT_0066c26c), t its
// jump time held at 4000.0 ms (_DAT_0066c24c). k is DAT_006bc47c, 488, 484
// or 494 by (ordinal +0x13c + object id +0xec) & 3; the static initialisers
// at 0x005a69b0-0x005a6a7c set each to a distance over a time squared:
// 400.0 by 4.0 s, 575.0 by 3.75 s, 650.0 by 3.925 s and 325.0 by 3.725 s
// (DAT_0066c258-0066c264, 4000.0 less DAT_0066c270-0066c278). The unit's
// own position is left where it jumped.
#[test]
fn a_jumping_unit_is_drawn_flying_out_behind_its_side() {
    let rate = [
        400.0 / (4.0 * 4.0),
        575.0 / (3.75 * 3.75),
        650.0 / (3.925 * 3.925),
        325.0 / (3.725 * 3.725),
    ];
    for is_alliance in [true, false] {
        let facing = if is_alliance { 1.0 } else { -1.0 };
        let mut squadron = test_fighter(0x1c00_0001, is_alliance);
        squadron.source_position.z = 7.0;
        let mut flagship = ship(is_alliance, 0, true, 5.0);
        // port: test fixture values, not rules (ordinal 1).
        flagship.tactical_resource =
            super::resource_ids::capital_ship_tactical_resource(DatId::new(0x1400_0041));
        let mut session = battle(vec![flagship], vec![squadron]);
        // The side's units take ids 1, 2 (Empire 1000001, 1000002) as they
        // are created, the ship before the squadron.
        let ship_ordinal = session.ships[0].tactical_resource.unwrap().tactical_ordinal;
        let squadron_ordinal = session.fighters[0].tactical_resource.unwrap().tactical_ordinal;
        let ship_rate = rate[usize::from(ship_ordinal.wrapping_add(1) & 3)];
        let squadron_rate = rate[usize::from(squadron_ordinal.wrapping_add(2) & 3)];

        assert_eq!(session.ship_display_position(0).z, 5.0, "not jumping");
        session.ships[0].retreating = true;
        session.ships[0].retreat_progress = 1_500.0;
        session.morale.squadron_jumps = vec![(0, 2_000.0)];
        let ship_z = session.ship_display_position(0).z;
        let squadron_z = session.fighter_display_position(0).z;
        assert!((ship_z - (5.0 - facing * ship_rate * 1.5 * 1.5)).abs() < 1e-3, "{ship_z}");
        assert!(
            (squadron_z - (7.0 - facing * squadron_rate * 2.0 * 2.0)).abs() < 1e-3,
            "{squadron_z}"
        );
        assert_eq!(session.ships[0].source_position.z, 5.0);
        session.ships[0].retreat_progress = 4_000.0;
        let held = session.ship_display_position(0).z;
        assert!((held - (5.0 - facing * ship_rate * 16.0)).abs() < 1e-3, "{held}");
    }
}

/// An Alliance trench group 0 of one squadron of four craft against an
/// operational Imperial station, the run past its approach when `rolled`.
/// port: test fixture values, not rules.
fn trench_battle(rolled: bool) -> BattleSession {
    let mut squadron = test_fighter(3, true);
    squadron.order = TacticalOrder::AttackDeathStar;
    squadron.squad_count = 4;
    squadron.hull_current = 4.0 * ORIGINAL_FIGHTER_HULL_PER_CRAFT;
    squadron.hull_max = squadron.hull_current;
    let mut session = battle(Vec::new(), vec![squadron]);
    session.death_star = Some(test_death_star(false));
    session.trench_run_group = Some(0);
    session.morale.hyperdrive_squadrons = vec![0];
    if rolled {
        // A seed whose first maneuver draw misses, so the roll takes craft.
        let seed = (0..)
            .find(|&seed| OriginalTacticalRng::new(seed).inclusive(1, 10) >= 2)
            .unwrap();
        session.tactical_rng = OriginalTacticalRng::new(seed);
        session.trench_run_remaining_ms = Some(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS);
        session.prepare_original_trench_run();
    }
    session
}

// FUN_005ad530.c:43-46: a withdrawing side's group under order 6 posts 0x3c
// state 8, which FUN_005cfec0.c:165-185 runs while the group still holds
// order 6 (FUN_005caf50): past the approach (state 5, 9 or 10) each
// participant takes back its state-5 snapshot (+0x3cc from +0x3d0, +0x3c0
// from +0x3c4), state 2 and its place in the battle (FUN_005a9380, +0x108
// cleared); the group then takes order 3, which drops the run
// (FUN_005ca6d0 deletes the result, FUN_005d0950 clears DAT_006bc978). No
// state 6 or 7 follows: no film, no outcome, the station stands.
#[test]
fn a_withdrawal_breaks_off_its_trench_run_and_restores_the_runners() {
    for rolled in [true, false] {
        let mut session = trench_battle(rolled);
        let snapshot = session.fighters[0].hull_max;
        if rolled {
            assert!(session.fighters[0].off_field);
            assert!(session.fighters[0].hull_current < snapshot, "the roll took craft");
        }
        session.begin_side_withdrawal(true);
        let squadron = &session.fighters[0];
        assert_eq!(squadron.hull_current, snapshot, "{rolled}");
        assert_eq!(squadron.squad_count, 4);
        assert!(!squadron.off_field);
        assert_eq!(squadron.order, TacticalOrder::Withdraw);
        assert_eq!(session.trench_run_group, None);
        assert_eq!(session.trench_run_remaining_ms, None);
        assert_eq!(session.trench_run_pending_outcome, None);
        session.advance_trench_run_approach(1_000.0);
        session.advance_original_trench_run(200_000.0);
        assert_eq!(session.trench_run_outcome, None, "{rolled}");
        assert_eq!(session.take_pending_trench_run_cinematic(), None);
        assert_eq!(session.death_star.unwrap().hull, 100.0);
    }
}

/// The player's armed ship against the computer side's unarmed one, both
/// able to jump, the computer side free to retreat: its strength is 0.0
/// against the player's.
/// port: test fixture values, not rules.
fn outgunned_computer(player_is_alliance: bool) -> BattleSession {
    let mut armed = ship(player_is_alliance, 0, true, 0.0);
    armed.weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 30);
    armed.weapon_recharge_rate = 8.0;
    let unarmed = ship(!player_is_alliance, 0, true, 0.0);
    // The Alliance attacks; the player is the Alliance when it attacks.
    let mut session = test_session(vec![armed, unarmed], Vec::new(), player_is_alliance);
    session.source_layout.battle_extent = EXTENT;
    session.morale.retreat_permitted = [true, true];
    session.morale.threshold = [0.5, 0.5];
    session
}

fn ai_updates(session: &mut BattleSession, updates: usize) {
    for _ in 0..updates {
        session.advance_computer_controller();
    }
}

// FUN_005c8780.c:133-165: the computer side's controller (+0x8 = 1), in
// state 0, counts its decision timer +0x40 down by 1.0 (_DAT_0066cbd4) each
// update; at 0.0 it resets it to 6.0 (DAT_0066cba4), recomputes both
// strengths (vtable +0x14, FUN_005c7f70; the opponent's through
// FUN_005a9610) and, the opponent's above 0.0, its own over the
// opponent's below its threshold +0x24 or exactly 0.0, it may retreat
// (vtable +0xc, FUN_005c7e80) and no trench run exists (FUN_005d0ae0),
// withdraws (FUN_005ad530). The player's side never does (:160).
#[test]
fn the_computer_side_withdraws_on_its_decision_pass_when_outgunned() {
    for player_is_alliance in [true, false] {
        let computer_is_attacker = !player_is_alliance;
        let computer = usize::from(!computer_is_attacker);
        let player = 1 - computer;
        let mut session = outgunned_computer(player_is_alliance);
        session.death_star_ai.countdown = 6.0;
        ai_updates(&mut session, 5);
        assert_eq!(
            session.morale.withdrawal[computer].state,
            TacticalWithdrawalState::Fighting,
            "the sixth update decides"
        );
        ai_updates(&mut session, 1);
        assert_eq!(
            session.morale.withdrawal[computer].state,
            TacticalWithdrawalState::CountingDown,
            "{player_is_alliance}"
        );
        assert_eq!(
            session.morale.withdrawal[player].state,
            TacticalWithdrawalState::Fighting
        );
        assert_eq!(session.death_star_ai.countdown, 6.0);
    }
}

// FUN_005c8780.c:153-164: the side stays while it holds its own (the ratio
// at or above the threshold), while it may not retreat (FUN_005c7e80: not
// permitted), and while a trench run exists (FUN_005d0ae0, DAT_006bc978).
#[test]
fn the_computer_side_stays_while_it_may_not_or_need_not_withdraw() {
    for player_is_alliance in [true, false] {
        let computer = usize::from(player_is_alliance);
        let stays = |session: &mut BattleSession, why: &str| {
            session.death_star_ai.countdown = 1.0;
            session.advance_computer_controller();
            assert_eq!(
                session.morale.withdrawal[computer].state,
                TacticalWithdrawalState::Fighting,
                "{why} ({player_is_alliance})"
            );
        };
        let mut session = outgunned_computer(player_is_alliance);
        session.morale.retreat_permitted[computer] = false;
        stays(&mut session, "not permitted");
        let mut session = outgunned_computer(player_is_alliance);
        session.trench_run_group = Some(0);
        stays(&mut session, "a trench run exists");
        let mut session = outgunned_computer(player_is_alliance);
        session.ships[1].weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 30);
        session.ships[1].weapon_recharge_rate = 8.0;
        stays(&mut session, "evenly matched");
    }
}

// FUN_005c8780.c:187-197: the computer Alliance facing an operational
// station, with no run ordered (+0x18), none under way, the run not
// warranted (FUN_005ad930) and no squadron left in its list +0x64,
// withdraws when it may.
#[test]
fn the_computer_alliance_without_squadrons_withdraws_from_an_operational_station() {
    let mut session = outgunned_computer(false);
    // Armed, with no threshold, so only the station rule applies.
    let alliance_ship = session.ships.iter().position(|ship| ship.identity.is_alliance).unwrap();
    session.ships[alliance_ship].weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 30);
    session.ships[alliance_ship].weapon_recharge_rate = 8.0;
    session.morale.threshold = [0.0, 0.0];
    session.death_star = Some(test_death_star(false));
    session.death_star_ai.countdown = 1.0;
    session.morale.retreat_permitted[0] = false;
    session.advance_computer_controller();
    assert_eq!(
        session.morale.withdrawal[0].state,
        TacticalWithdrawalState::Fighting,
        "it may not retreat"
    );
    session.morale.retreat_permitted[0] = true;
    session.death_star_ai.countdown = 1.0;
    session.advance_computer_controller();
    assert_eq!(
        session.morale.withdrawal[0].state,
        TacticalWithdrawalState::CountingDown
    );
}

// port: each jump's start and its unit's departure (state 8, 0x005aa29a)
// are logged with the unit's object id and the jump's time, so a native run
// can time the 4000 ms jump (FUN_005a6b40, _DAT_0066c24c).
#[test]
fn a_jump_logs_its_start_and_its_units_departure() {
    for is_alliance in [true, false] {
        let mut session = battle(vec![ship(is_alliance, 0, true, line(is_alliance))], Vec::new());
        session.begin_side_withdrawal(is_alliance);
        steps(&mut session, 34_250.0);
        let id = if is_alliance { 1 } else { 1_000_001 };
        let lines: Vec<_> = session
            .pending_log_lines
            .iter()
            .filter(|line| line.contains(&format!("unit={id} ")))
            .cloned()
            .collect();
        assert_eq!(lines.len(), 2, "{lines:?}");
        assert!(lines[0].contains("status=jumping"), "{lines:?}");
        assert!(lines[1].contains("status=jumped"), "{lines:?}");
        let at = |line: &str| -> f32 {
            line.split("t_ms=").nth(1).unwrap().split(' ').next().unwrap().parse().unwrap()
        };
        assert_eq!((at(&lines[0]), at(&lines[1])), (0.0, 4_000.0), "{lines:?}");
    }
}

/// `is_alliance`'s side withdraws and its ship (hyperdrive, at its line,
/// room for one squadron) has left; the other side's ship (none) stays.
/// `fighters` join the battle first.
/// port: test fixture values, not rules.
fn after_the_jump(is_alliance: bool, fighters: Vec<TacticalFighter>) -> BattleSession {
    after_the_jump_with(is_alliance, fighters, true)
}

/// [`after_the_jump`] with the Alliance attacking when `attacker_is_alliance`.
/// port: test fixture values, not rules.
fn after_the_jump_with(
    is_alliance: bool,
    mut fighters: Vec<TacticalFighter>,
    attacker_is_alliance: bool,
) -> BattleSession {
    let mut carrier = ship(is_alliance, 0, true, line(is_alliance));
    carrier.fighter_capacity = 1;
    let mut ships = vec![carrier, ship(!is_alliance, 1, false, 0.0)];
    for ship in &mut ships {
        ship.is_attacker = ship.identity.is_alliance == attacker_is_alliance;
    }
    for fighter in &mut fighters {
        fighter.is_attacker = fighter.identity.is_alliance == attacker_is_alliance;
    }
    let mut session = battle(ships, fighters);
    session.attacker_is_alliance = attacker_is_alliance;
    session.player_is_attacker = attacker_is_alliance;
    session.begin_side_withdrawal(is_alliance == attacker_is_alliance);
    steps(&mut session, 34_250.0);
    assert!(session.ships[0].retreated, "the ship has left");
    session
}

/// The winner when `is_alliance`'s side wins (the Alliance attacks).
fn win(is_alliance: bool) -> CombatWinner {
    if is_alliance {
        CombatWinner::Attacker
    } else {
        CombatWinner::Defender
    }
}

// FUN_005a9ec0.c:169-173 counts a side's units in states 10, 2, 4, 6, 7 and 8
// only: a squadron in a hangar (states 1 and 5) adds nothing, so a side
// whose last ship jumped with its squadrons docked in it has left, and
// :274-286 gives the side still fighting the battle. FUN_005bef90.c:72-96
// keeps the squadron docked in the jumped ship.
#[test]
fn a_side_with_only_docked_squadrons_in_a_jumped_carrier_has_left_the_battle() {
    for is_alliance in [true, false] {
        let mut docked = test_fighter(0x1c00_0002, is_alliance);
        docked.carrier = Some(TacticalBerth::Ship(0));
        docked.docked = Some(TacticalDock::Recovered);
        docked.alive = false;
        let mut session = after_the_jump(is_alliance, vec![docked]);
        assert_eq!(session.fighters[0].squad_count, 12, "docked, not lost");
        assert!(session.finish_if_decided());
        assert_eq!(session.winner, Some(win(!is_alliance)), "{is_alliance}");
        assert_eq!(session.end_berths(), vec![Some(TacticalBerth::Ship(0))]);
    }
}

// FUN_005a9ec0.c:274-290: a side whose count is 0 has lost, and both lost
// clears both; FUN_0040c780.c:60-69 then finds no loser and :82-93 gives the
// battle to the side that withdrew (+0x27c side 0, +0x27e side 1, copied to
// +0x66c by FUN_0059a530 and read through FUN_005969d0).
#[test]
fn when_both_sides_reach_zero_the_side_that_jumped_wins() {
    for attacker_is_alliance in [true, false] {
        for is_alliance in [true, false] {
            let mut session = after_the_jump_with(is_alliance, Vec::new(), attacker_is_alliance);
            session.ships[1].alive = false;
            session.ships[1].hull_current = 0;
            assert!(session.finish_if_decided());
            assert_eq!(
                session.winner_is_alliance(),
                Some(is_alliance),
                "{attacker_is_alliance} {is_alliance}"
            );
        }
    }
}

// FUN_005a9ec0.c:287-290 clears both lost flags and FUN_0040c780.c:82-93,
// with neither side withdrawn, records no winner.
#[test]
fn when_both_sides_reach_zero_and_neither_jumped_the_battle_is_a_draw() {
    for player_is_attacker in [true, false] {
        let mut session = battle(
            vec![ship(true, 0, true, 0.0), ship(false, 1, true, 0.0)],
            Vec::new(),
        );
        session.player_is_attacker = player_is_attacker;
        for ship in &mut session.ships {
            ship.alive = false;
            ship.hull_current = 0;
        }
        assert!(session.finish_if_decided());
        assert_eq!(session.winner, Some(CombatWinner::Draw));
    }
}

// FUN_005a9ec0.c:257-273: a side counts as withdrawn only when no unit of it
// is alive (FUN_005ad870: state 2, 4 or 6) or in state 7 or 8. A squadron
// left fighting after its carrier jumped keeps its side in the battle.
#[test]
fn a_side_with_a_live_squadron_does_not_count_as_withdrawn() {
    for is_alliance in [true, false] {
        let session_with = |flying: TacticalFighter| after_the_jump(is_alliance, vec![flying]);
        let mut session = session_with(test_fighter(0x1c00_0002, is_alliance));
        assert!(session.fighters[0].alive);
        assert!(!session.side_withdrew(is_alliance), "{is_alliance}");
        session.fighters[0].destroy();
        assert!(session.side_withdrew(is_alliance), "{is_alliance}");
        assert!(!session.side_withdrew(!is_alliance));
    }
}

// FUN_005a9ec0.c:245-273: a side whose controller is in state 3, 4 or 5 has
// withdrawn when one of its units reached state 9, a squadron as well as a
// ship (FUN_005c6610.c:76-108 jumps a squadron, FUN_005a90c0 sets state 9).
#[test]
fn a_side_whose_only_jumper_was_a_squadron_has_withdrawn() {
    for is_alliance in [true, false] {
        let mut squadron = test_fighter(0x1c00_0001, is_alliance);
        squadron.source_position.z = line(is_alliance);
        let mut session = battle(vec![ship(!is_alliance, 0, false, 0.0)], vec![squadron]);
        session.morale.hyperdrive_squadrons = vec![0];
        session.begin_side_withdrawal(is_alliance);
        steps(&mut session, 34_250.0);
        assert!(!session.fighters[0].alive, "it has left");
        assert!(session.side_withdrew(is_alliance), "{is_alliance}");
        assert!(!session.side_withdrew(!is_alliance));
    }
}

// FUN_005a9ec0.c:253-256 and :273: side 1, the Empire, has also withdrawn
// when its station departed (+0x78, FUN_005ba5e0 after FUN_005c8780.c:31-47);
// the Alliance has no station to carry its withdrawal, and a withdrawal none
// of whose units jumped is none.
#[test]
fn an_empire_whose_station_departed_has_withdrawn() {
    let mut session = battle(vec![ship(true, 0, false, 0.0)], Vec::new());
    session.death_star = Some(test_death_star(false));
    session.begin_side_withdrawal(false);
    session.begin_side_withdrawal(true);
    steps(&mut session, 150_000.0);
    assert!(session.death_star.unwrap().action_committed);
    assert!(session.side_withdrew(false));
    session.ships[0].alive = false;
    session.ships[0].hull_current = 0;
    assert!(!session.side_withdrew(true), "the Alliance's ship had no hyperdrive");
}

// FUN_005a9ec0.c:245-273 asks only that no unit of the side is alive or in
// its jump and that one reached state 9: a side whose capital ship was
// destroyed while its hyperdrive squadrons jumped has withdrawn, and, its
// count at 0 (:169-173, :274-286), lost to the side still fighting
// (FUN_0040c780.c:60-69).
#[test]
fn a_side_whose_ship_died_while_its_squadrons_jumped_has_withdrawn() {
    for is_alliance in [true, false] {
        let mut squadrons = Vec::new();
        for _ in 0..2 {
            let mut squadron = test_fighter(0x1c00_0001, is_alliance);
            squadron.source_position.z = line(is_alliance);
            squadrons.push(squadron);
        }
        let mut session = battle(
            vec![ship(is_alliance, 0, false, 0.0), ship(!is_alliance, 1, false, 0.0)],
            squadrons,
        );
        session.morale.hyperdrive_squadrons = vec![0, 1];
        session.begin_side_withdrawal(is_alliance);
        session.ships[0].alive = false;
        session.ships[0].hull_current = 0;
        steps(&mut session, 34_250.0);
        assert!(session.fighters.iter().all(|fighter| !fighter.alive), "both jumped");
        assert!(session.side_withdrew(is_alliance), "{is_alliance}");
        assert!(session.finish_if_decided());
        assert_eq!(session.winner, Some(win(!is_alliance)));
    }
}

// FUN_005c8780.c:28-47: the countdown alone ends state 2; at 0.0 side 1
// posts 0x45 for its operational station, and FUN_005ba5e0 only commits the
// station (+0x78), recalls its waiting squadrons (FUN_005b8d10) and drops its
// decal (FUN_005c1200). State 3 then waits for the side's hyperdrive units
// (:72-101), whose Withdraw executor jumps them at the line
// (FUN_005cc300.c:77-145): a ship with a working hyperdrive still leaves.
#[test]
fn a_departed_station_leaves_its_sides_ships_to_jump() {
    for is_alliance in [true, false] {
        let mut session = battle(
            vec![ship(is_alliance, 0, true, line(is_alliance)), ship(!is_alliance, 1, false, 0.0)],
            Vec::new(),
        );
        session.death_star = Some(test_death_star(false));
        let countdown = if is_alliance { 30_000.0 } else { 150_000.0 };
        session.begin_side_withdrawal(is_alliance);
        steps(&mut session, countdown);
        assert_eq!(session.death_star.unwrap().action_committed, !is_alliance);
        assert_eq!(
            session.morale.withdrawal[usize::from(!is_alliance)].state,
            TacticalWithdrawalState::Holding
        );
        steps(&mut session, 4_250.0);
        assert!(session.ships[0].retreated, "{is_alliance}");
        assert!(session.side_withdrew(is_alliance));
    }
}

/// Both sides withdraw (each ship with a hyperdrive at its line) and jump,
/// the Alliance attacking when `attacker_is_alliance`. Returns the session
/// and the Alliance's and the Empire's fleets.
/// port: test fixture values, not rules.
fn both_jumped(
    player_is_alliance: bool,
    attacker_is_alliance: bool,
) -> (BattleSession, [Option<FleetKey>; 2]) {
    let mut ships = vec![ship(true, 0, true, line(true)), ship(false, 1, true, line(false))];
    for ship in &mut ships {
        ship.is_attacker = ship.identity.is_alliance == attacker_is_alliance;
    }
    let mut session = battle(ships, Vec::new());
    session.attacker_is_alliance = attacker_is_alliance;
    session.player_is_attacker = player_is_alliance == attacker_is_alliance;
    let mut keys = slotmap::SlotMap::<FleetKey, ()>::with_key();
    let fleets = [Some(keys.insert(())), Some(keys.insert(()))];
    let (attacker, defender) = if attacker_is_alliance {
        (fleets[0], fleets[1])
    } else {
        (fleets[1], fleets[0])
    };
    session.attacker_fleet = attacker;
    session.defender_fleet = defender;
    session.begin_side_withdrawal(true);
    session.begin_side_withdrawal(false);
    steps(&mut session, 34_250.0);
    assert!(session.ships.iter().all(|ship| ship.retreated), "both have left");
    (session, fleets)
}

/// The local side's faction.
fn faction(is_alliance: bool) -> TacticalVoiceFaction {
    if is_alliance {
        TacticalVoiceFaction::Alliance
    } else {
        TacticalVoiceFaction::Empire
    }
}

// FUN_005a9ec0.c:287-290 clears both lost flags; with both sides withdrawn
// FUN_0040c780.c:95-101 records the winner from the system's +0x78 holder
// (the side that arrived last, FUN_0050a1b0): holder 1 (the Alliance)
// gives 2, the Empire; holders 2 and 0 give 1, the Alliance. The voice is
// the local side's own withdrawal (FUN_005a9ec0.c:291-328: 0x96, 0x118),
// and the results follow the winner's fleet.
#[test]
fn when_both_sides_withdraw_the_side_that_did_not_arrive_last_wins() {
    for attacker_is_alliance in [true, false] {
        for player_is_alliance in [true, false] {
            for (holder, alliance_wins) in [(Some(true), false), (Some(false), true), (None, true)] {
                let (mut session, fleets) = both_jumped(player_is_alliance, attacker_is_alliance);
                session.arrival_holder_is_alliance = holder;
                let case = format!("{attacker_is_alliance} {player_is_alliance} {holder:?}");
                assert!(session.side_withdrew(true) && session.side_withdrew(false));
                assert!(session.finish_if_decided());
                assert_eq!(session.winner_is_alliance(), Some(alliance_wins), "{case}");
                assert_eq!(
                    session.take_pending_voice_cues(),
                    vec![TacticalVoiceCue::withdrawal_complete(faction(player_is_alliance))],
                    "{case}"
                );
                let fleet = fleets[usize::from(!alliance_wins)];
                assert_eq!(apply::summarize_results(&session).winner_fleet, fleet, "{case}");
            }
        }
    }
}

// FUN_005a9ec0.c:291-328 picks the outcome voice from the end flags, not
// the recorded winner: with both sides at nothing their lost flags are
// cleared (:287-290), so the local side hears its withdrawal (0x96, 0x118)
// when it jumped and nothing when only the other side did.
#[test]
fn when_both_sides_reach_zero_only_a_local_withdrawal_is_voiced() {
    for player_is_alliance in [true, false] {
        for local_jumped in [true, false] {
            let jumper = player_is_alliance == local_jumped;
            let mut session = after_the_jump(jumper, Vec::new());
            session.player_is_attacker = player_is_alliance;
            session.ships[1].alive = false;
            session.ships[1].hull_current = 0;
            session.take_pending_voice_cues();
            assert!(session.finish_if_decided());
            assert_eq!(session.winner, Some(win(jumper)));
            let expected = if local_jumped {
                vec![TacticalVoiceCue::withdrawal_complete(faction(player_is_alliance))]
            } else {
                Vec::new()
            };
            assert_eq!(
                session.take_pending_voice_cues(),
                expected,
                "{player_is_alliance} {local_jumped}"
            );
        }
    }
}

/// `alliance` and `empire` squadrons of one class and no ships; neither side
/// may retreat (`side+0x80` clear).
/// port: test fixture values, not rules.
fn squadrons_only(alliance: usize, empire: usize) -> BattleSession {
    let mut fighters = Vec::new();
    for (is_alliance, count) in [(true, alliance), (false, empire)] {
        for _ in 0..count {
            fighters.push(test_fighter(0x1c00_0001, is_alliance));
        }
    }
    battle(Vec::new(), fighters)
}

/// Step the stalemate watch one second at a time `seconds` times, each
/// step still undecided.
fn quiet(session: &mut BattleSession, seconds: usize) {
    for second in 0..seconds {
        assert!(!session.advance_stalemate_watch(1.0), "decided at step {second}");
    }
}

// FUN_005ae9c0.c:29-48: a side's strength (FUN_005d0ce0) at a new low
// (+0x264, +0x268) restarts its quiet time (+0x26c, +0x270), which
// otherwise grows by the step; once both have stayed quiet 1200 s
// (_DAT_0066c3e0, :49-57) the weaker side leaves (:58-103), destroyed
// outright as it may not retreat (FUN_005af7a0), and the stronger wins.
#[test]
fn a_stalemate_ends_after_1200_quiet_seconds_with_the_weaker_side_gone() {
    for alliance_stronger in [true, false] {
        let (alliance, empire) = if alliance_stronger { (2, 1) } else { (1, 2) };
        let mut session = squadrons_only(alliance, empire);
        quiet(&mut session, 1200);
        assert!(session.advance_stalemate_watch(1.0));
        assert_eq!(session.winner, Some(win(alliance_stronger)), "{alliance_stronger}");
        for fighter in &session.fighters {
            assert_eq!(
                fighter.squad_count == 0,
                fighter.identity.is_alliance != alliance_stronger,
                "{alliance_stronger}"
            );
        }
    }
}

// FUN_005ae9c0.c:29-48: a loss that takes a side to a new low restarts
// that side's quiet time, so the 1200 s count starts again from it.
#[test]
fn a_new_low_in_a_sides_strength_restarts_its_stalemate_count() {
    for alliance_stronger in [true, false] {
        let (alliance, empire) = if alliance_stronger { (3, 1) } else { (1, 3) };
        let mut session = squadrons_only(alliance, empire);
        quiet(&mut session, 601);
        let lost = session
            .fighters
            .iter()
            .position(|fighter| fighter.identity.is_alliance == alliance_stronger)
            .unwrap();
        session.fighters[lost].destroy();
        quiet(&mut session, 1200);
        assert!(session.advance_stalemate_watch(1.0));
        assert_eq!(session.winner, Some(win(alliance_stronger)), "{alliance_stronger}");
    }
}

// FUN_005ae9c0.c:58-103: with equal strengths both sides leave; neither may
// retreat, so both are destroyed and neither withdrew: no winner
// (FUN_0040c780.c:82-101).
#[test]
fn an_even_stalemate_takes_both_sides_away() {
    for player_is_attacker in [true, false] {
        let mut session = squadrons_only(1, 1);
        session.player_is_attacker = player_is_attacker;
        quiet(&mut session, 1200);
        assert!(session.advance_stalemate_watch(1.0));
        assert_eq!(session.winner, Some(CombatWinner::Draw));
        assert!(session.fighters.iter().all(|fighter| fighter.squad_count == 0));
    }
}
