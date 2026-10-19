use std::path::PathBuf;

use rebellion_core::ids::DatId;

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("repository root")
        .join("data/base")
}

/// Setup leaves system `+0x88` bit 3 on every rim system (SYSTEMSD family
/// `0x92`) and on no core one (`0x90`): the base class's slot `+0xbc`
/// (`FUN_005081f0`) sets it, the core class's (`FUN_00566d40`) clears it,
/// the rim class's (`FUN_0056a550`) keeps it. SYSTEMSD holds 60 core and
/// 140 rim systems.
#[test]
#[ignore = "requires original data/base DAT files"]
fn rim_systems_start_out_of_every_sector_shift() {
    let world = rebellion_data::load_game_data(&data_dir()).expect("load original game data");
    let (mut core, mut rim) = (0, 0);
    for key in world.systems.keys() {
        match world.system_family(key) {
            0x90 => {
                core += 1;
                assert!(!world.shift_excluded.contains(&key));
            }
            0x92 => {
                rim += 1;
                assert!(world.shift_excluded.contains(&key));
            }
            other => panic!("unexpected system family {other:#x}"),
        }
    }
    assert!(core > 0 && rim > 0);
    assert_eq!(world.shift_excluded.len(), rim);
}

/// A regiment's slot `+0x1c0`, the blockade loss counters' value, is its
/// TROOPSD `uprising_defense` (class `+0x58`): 5 for record 1, 8 for
/// record 10.
#[test]
#[ignore = "requires original data/base DAT files"]
fn regiment_classes_keep_their_troopsd_uprising_defense() {
    let world = rebellion_data::load_game_data(&data_dir()).expect("load original game data");
    let value = |raw| rebellion_core::support_shift::regiment_value(&world, DatId::new(raw));
    assert_eq!(value(0x1000_0001), 5);
    assert_eq!(value(0x1000_000a), 8);
}

/// The loaded world holds SYSTEMSD and MJCHARSD ids without their family
/// byte, and the lookups by the original's compound ids still find their
/// records: `FUN_005180e0`'s Coruscant (`0x90000109`) and four leaders
/// (`0x30000240`, `0x32000242`, `0x34000280`, `0x35000281`), and
/// `FUN_005431e0`'s seats for a jailed escapee, Yavin (`0x92000121`) for
/// the Alliance and Coruscant for the Empire.
#[test]
#[ignore = "requires original data/base DAT files"]
fn the_loaded_world_resolves_the_victory_records_and_the_escapee_seats() {
    use rebellion_core::victory::{self, VictoryState, VictorySystem};
    use rebellion_core::world::{Abode, ControlKind, VictoryConditions};

    let mut world = rebellion_data::load_game_data(&data_dir()).expect("load original game data");
    let seat = |world: &rebellion_core::world::GameWorld, index| {
        world
            .systems
            .iter()
            .find(|(_, system)| system.dat_id.index() == index)
            .map(|(key, _)| key)
            .expect("SYSTEMSD holds the seat")
    };
    let (yavin, coruscant) = (seat(&world, 0x121), seat(&world, 0x109));
    let leader = |world: &rebellion_core::world::GameWorld, index| {
        world
            .characters
            .iter()
            .find(|(_, character)| character.dat_id.index() == index)
            .map(|(key, _)| key)
            .expect("MJCHARSD holds the leader")
    };

    for (index, is_alliance, home) in [(0x242, true, yavin), (0x281, false, coruscant)] {
        let key = leader(&world, index);
        world.systems[home].control = ControlKind::Controlled(if is_alliance {
            rebellion_core::dat::Faction::Alliance
        } else {
            rebellion_core::dat::Faction::Empire
        });
        let character = &mut world.characters[key];
        character.is_captive = true;
        character.current_system = None;
        character.current_fleet = None;
        character.abode = Some(Abode::jail_of(!is_alliance));
        rebellion_core::death_star::free_escapee(
            &mut world,
            &mut rebellion_core::missions::MissionState::new(),
            &rebellion_core::troop_transport::TroopTransportState::default(),
            &rebellion_core::movement::MovementState::new(),
            &rebellion_core::blockade::BlockadeState::new(),
            key,
            1,
        );
        assert_eq!(
            world.characters[key].current_system,
            Some(home),
            "{index:#x}"
        );
    }

    // Every leader held and Coruscant destroyed: each lookup sets its bit.
    for index in [0x240, 0x242, 0x280, 0x281] {
        let key = leader(&world, index);
        world.characters[key].is_captive = true;
    }
    world.systems[coruscant].is_destroyed = true;
    let state = VictoryState::new(yavin, coruscant);
    let found = victory::LC | victory::MC | victory::DC | victory::EC | victory::CD;
    for conditions in [
        VictoryConditions::Standard,
        VictoryConditions::HeadquartersOnly,
    ] {
        let word = VictorySystem::word(&state, &world, conditions);
        assert_eq!(word.map(|word| word & found), Some(found), "{conditions:?}");
    }
}
