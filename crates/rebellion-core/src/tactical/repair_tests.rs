//! Subsystem repair in a played battle (`FUN_005b0330`, `FUN_005b1490`;
//! `ghidra/notes/tactical-engagement-ai.md`, "Subsystem repair").

use super::*;
use crate::world::{CapitalShipClass, Fleet, ShipInstance, System};

/// One unarmed ship per side, damage control 100, each carrying `engines`
/// engine hits and `weapons` weapon hits from the strategic game; the Alliance
/// attacks and `player_is_alliance` picks the player's side.
/// port: test fixture values, not rules.
fn carried(player_is_alliance: bool, engines: u8, weapons: u8) -> BattleSession {
    let mut world = GameWorld::default();
    let system = world.systems.insert(System::default());
    let mut fleets = Vec::new();
    for is_alliance in [true, false] {
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1400_0001),
            family: 0x14,
            is_alliance,
            is_empire: !is_alliance,
            hull: 100,
            sub_light_engine: 4,
            damage_control: 100,
            ..CapitalShipClass::default()
        });
        let mut ship = ShipInstance::new(class, 100, is_alliance);
        ship.set_damage_counter(3, engines);
        ship.set_damage_counter(1, weapons);
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ship],
            is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);
        fleets.push(fleet);
    }
    let mut session = BattleSession::new(&world, system, fleets[0], fleets[1], player_is_alliance, 0, 1);
    session.phase = BattlePhase::Combat;
    session.paused = false;
    session
}

fn steps(session: &mut BattleSession, count: usize) {
    for _ in 0..count {
        session.step();
    }
}

/// The engine hits of the player's ship and of the computer's.
fn engines(session: &BattleSession) -> (u8, u8) {
    let of = |player: bool| {
        session
            .ships
            .iter()
            .find(|ship| (ship.is_attacker == session.player_is_attacker) == player)
            .unwrap()
            .subsystem_damage
            .engines
    };
    (of(true), of(false))
}

// FUN_005c0170 sets +0x124 on every unit not of the player's side, and
// FUN_005b0330.c:19-24 counts the repair timer +0x3f4 down only for a unit
// without it that has outstanding hits (+0x158); the timer starts at 0
// (FUN_005b05c0.c:142), so the player's ship rolls on the first unit pass
// (FUN_005a9de0, once per 1000 ms turn of FUN_005bea30) and the computer's
// never does.
#[test]
fn only_the_players_ships_repair_their_carried_hits() {
    for player_is_alliance in [true, false] {
        let mut session = carried(player_is_alliance, 1, 0);
        assert_eq!(engines(&session), (1, 1));
        steps(&mut session, 4);
        assert_eq!(engines(&session), (0, 1), "{player_is_alliance}");
        steps(&mut session, 400);
        assert_eq!(engines(&session), (0, 1), "{player_is_alliance}");
    }
}

// FUN_005b0330.c:22-24: a repair roll resets the timer to 50 s
// (_DAT_0066c46c × _DAT_0066c490), so a ship's next roll comes 50 s after
// its last.
#[test]
fn a_ship_repairs_again_50_seconds_after_its_last_roll() {
    for player_is_alliance in [true, false] {
        let mut session = carried(player_is_alliance, 2, 0);
        steps(&mut session, 4);
        assert_eq!(engines(&session).0, 1, "{player_is_alliance}");
        steps(&mut session, 199);
        assert_eq!(engines(&session).0, 1, "{player_is_alliance}");
        steps(&mut session, 1);
        assert_eq!(engines(&session).0, 0, "{player_is_alliance}");
    }
}

// FUN_005b05c0.c:150-157 seeds +0x158 from the carried engine, shield,
// hyperdrive and tractor hits, not the weapon hits (+0x37c), so a ship that
// carries only weapon damage has no outstanding hits and never rolls
// (FUN_005b0330.c:19-21).
#[test]
fn carried_weapon_hits_alone_start_no_repair() {
    for player_is_alliance in [true, false] {
        let mut session = carried(player_is_alliance, 0, 1);
        steps(&mut session, 404);
        let weapons: Vec<u8> = session.ships.iter().map(|ship| ship.subsystem_damage.weapons).collect();
        assert_eq!(weapons, vec![1, 1], "{player_is_alliance}");
    }
}

// FUN_005b1970 adds every new hit to +0x158, so a hit taken in the battle
// starts the player's repair timer on the next unit pass.
#[test]
fn a_hit_taken_in_battle_is_repaired_on_the_next_pass() {
    for player_is_alliance in [true, false] {
        let mut session = carried(player_is_alliance, 0, 0);
        steps(&mut session, 4);
        let player = session
            .ships
            .iter()
            .position(|ship| ship.is_attacker == session.player_is_attacker)
            .unwrap();
        session.ships[player].add_subsystem_hit(TacticalSubsystemKind::Engines);
        assert_eq!(engines(&session).0, 1);
        steps(&mut session, 4);
        assert_eq!(engines(&session).0, 0, "{player_is_alliance}");
    }
}

/// The player's ship index in `carried`.
fn player_ship(session: &BattleSession) -> usize {
    session
        .ships
        .iter()
        .position(|ship| ship.is_attacker == session.player_is_attacker)
        .unwrap()
}

/// One unit pass on the player's ship with `damage_control`, its timer
/// out; returns whether it repaired and the stream it left.
/// port: test fixture values, not rules.
fn one_pass(player_is_alliance: bool, engines: u8, damage_control: u8) -> (bool, BattleSession) {
    let mut session = carried(player_is_alliance, engines, 0);
    let player = player_ship(&session);
    session.ships[player].damage_control = damage_control;
    session.combat_tick = 4;
    session.advance_subsystem_repairs();
    let repaired = session.ships[player].subsystem_damage.engines < engines;
    (repaired, session)
}

// FUN_005b1490: the repair needs a 1-100 draw no greater than damage
// control (+0x3f0), the draw included.
#[test]
fn a_repair_needs_a_draw_no_greater_than_damage_control() {
    for player_is_alliance in [true, false] {
        let mut stream = carried(player_is_alliance, 1, 0).tactical_rng;
        let draw = u8::try_from(stream.inclusive(1, 100)).unwrap();
        assert!(one_pass(player_is_alliance, 1, draw).0, "{player_is_alliance}: at the draw");
        if draw > 1 {
            assert!(!one_pass(player_is_alliance, 1, draw - 1).0, "{player_is_alliance}: below it");
        }
    }
}

// FUN_005b1490 draws the hit to repair (FUN_005a8a70(1, +0x158)) only with
// two or more outstanding; with one it takes it without a draw.
#[test]
fn the_hit_to_repair_is_drawn_only_among_two_or_more() {
    for player_is_alliance in [true, false] {
        for (outstanding, draws) in [(1_u8, 1_usize), (2, 2), (3, 2)] {
            let mut expected = carried(player_is_alliance, outstanding, 0).tactical_rng;
            for _ in 0..draws {
                expected.inclusive(1, 100);
            }
            let (repaired, session) = one_pass(player_is_alliance, outstanding, 100);
            assert!(repaired);
            assert_eq!(session.tactical_rng, expected, "{player_is_alliance} {outstanding}");
        }
    }
}

// FUN_005b1ab0 takes a repaired hit off +0x158, so a ship whose last hit
// is repaired has none outstanding and stops counting.
#[test]
fn a_repair_takes_the_hit_off_the_outstanding_count() {
    for player_is_alliance in [true, false] {
        let (_, session) = one_pass(player_is_alliance, 1, 100);
        assert_eq!(session.ships[player_ship(&session)].outstanding_hits, 0);
        let (_, session) = one_pass(player_is_alliance, 3, 100);
        assert_eq!(session.ships[player_ship(&session)].outstanding_hits, 2);
    }
}

// hyp: a fourth weapon hit empties every armed arc (`disable_weapon_arcs`);
// the third leaves them armed.
#[test]
fn only_the_fourth_weapon_hit_empties_the_armed_arcs() {
    for is_alliance in [true, false] {
        let mut ship = super::test_support::test_ship(64, 0, is_alliance, true);
        ship.weapon_arcs[0] = TacticalWeaponArc::new(0, 0, 2);
        for _ in 0..3 {
            ship.add_subsystem_hit(TacticalSubsystemKind::Weapons);
        }
        assert!(ship.weapon_arcs[0].ready, "{is_alliance}");
        ship.add_subsystem_hit(TacticalSubsystemKind::Weapons);
        assert!(!ship.weapon_arcs[0].ready, "{is_alliance}");
        assert_eq!(ship.weapon_arcs[0].energy, 0.0);
    }
}

// `add_subsystem_hit` reports a tractor hit that leaves the tractor at no
// condition, so the caller drops the ship's tractor field; no other hit
// does.
#[test]
fn only_a_tractor_hit_that_empties_the_tractor_drops_its_field() {
    for is_alliance in [true, false] {
        let mut ship = super::test_support::test_ship(64, 0, is_alliance, true);
        ship.subsystem_capacity.tractor = true;
        ship.refresh_subsystem_condition();
        for _ in 0..3 {
            assert!(!ship.add_subsystem_hit(TacticalSubsystemKind::Tractor));
        }
        assert!(ship.add_subsystem_hit(TacticalSubsystemKind::Tractor), "{is_alliance}");
        assert!(!ship.add_subsystem_hit(TacticalSubsystemKind::Engines), "{is_alliance}");
    }
}
