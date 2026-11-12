mod common;

use common::data_dir;
use rebellion_core::ids::DatId;
use rebellion_core::world::Skill;

#[test]
#[ignore = "requires original data/base DAT files"]
fn missnsd_records_carry_the_mission_timer_and_flags() {
    // FUN_005236e0 reads the timer from record +0x50/+0x54 and the flags
    // from +0x58..+0x64, the file fields after `target_flags`
    // (ghidra/notes/mission-lifecycle.md, "The mission record").
    let world = rebellion_data::load_game_data(&data_dir()).expect("load original game data");

    assert_eq!(world.mission_records.len(), 25);
    let diplomacy = world
        .mission_record(DatId::new(0x5100_0010))
        .expect("Diplomacy record");
    assert_eq!(diplomacy.dat_id, DatId::new(0x5100_0010));
    assert_eq!(
        (diplomacy.timer_min_days, diplomacy.timer_spread_days),
        (5, 10)
    );
    assert!(diplomacy.repeats && diplomacy.detection_phases && diplomacy.can_resign);
    assert!(!diplomacy.hidden);
    let sabotage = world
        .mission_record(DatId::new(0x6900_0012))
        .expect("Sabotage record");
    assert_eq!(
        (sabotage.timer_min_days, sabotage.timer_spread_days),
        (1, 2)
    );
    assert!(!sabotage.repeats);
}

#[test]
#[ignore = "requires original data/base DAT files"]
fn a_mission_family_can_hold_several_missnsd_records() {
    // MISSNSD ids 69 and 70 are both Vacation (family 0x72) with different
    // timers, so a record is found by its id (`GameWorld::mission_record`).
    let world = rebellion_data::load_game_data(&data_dir()).expect("load original game data");

    let vacation: Vec<_> = world
        .mission_records
        .iter()
        .filter(|record| record.dat_id.family() == 0x72)
        .map(|record| (record.dat_id, record.timer_min_days))
        .collect();
    assert_eq!(
        vacation,
        vec![
            (DatId::new(0x7200_0045), 60),
            (DatId::new(0x7200_0046), 1000)
        ]
    );
    let research = world
        .mission_records
        .iter()
        .filter(|record| record.dat_id.family() == 0x53)
        .count();
    assert_eq!(research, 3);
}

#[test]
#[ignore = "requires original data/base DAT files"]
fn seeded_special_forces_take_their_skills_from_specfcsd() {
    // FUN_00535e40 sets each skill to the class base + rand(0..=variance);
    // the shipped SPECFCSD variances are 0.
    let world = rebellion_data::load_game_data(&data_dir()).expect("load original game data");

    assert_eq!(world.special_force_classes.len(), 9);
    let class = &world.special_force_classes[&DatId::new(0x3c00_0001)];
    assert_eq!(class.skills[Skill::Espionage as usize].base, 55);
    assert_eq!(class.mission_mask, 0x1);
    assert!(!world.special_forces.is_empty());
    for unit in world.special_forces.values() {
        let class = &world.special_force_classes[&unit.class_dat_id];
        for (value, template) in unit.skills.iter().zip(class.skills) {
            assert_eq!(*value, template.base);
        }
    }
}

#[test]
#[ignore = "requires original data/base DAT files"]
fn every_port_mission_kind_names_its_shipped_record_and_target_columns() {
    // MISSNSD columns 11..21 (in-memory +0x6c..+0x94) read by FUN_00523450,
    // FUN_00592600, FUN_005868c0 and FUN_00586e20
    // (ghidra/notes/mission-lifecycle.md, "The running checks").
    use rebellion_core::missions::MissionKind;
    let world = rebellion_data::load_game_data(&data_dir()).expect("load original game data");
    let expected = [
        (
            MissionKind::Diplomacy,
            (5, 10, true),
            [1, 1, 1, 1, 1, 0, 0, 1, 0, 0],
        ),
        (
            MissionKind::Espionage,
            (1, 20, false),
            [0, 0, 0, 1, 1, 1, 1, 1, 0, 0],
        ),
        (
            MissionKind::Recruitment,
            (5, 20, false),
            [1, 1, 1, 1, 0, 0, 1, 1, 0, 0],
        ),
        (
            MissionKind::InciteUprising,
            (2, 10, true),
            [1, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        ),
        (
            MissionKind::SubdueUprising,
            (2, 10, true),
            [1, 1, 1, 1, 0, 0, 1, 0, 0, 0],
        ),
        (
            MissionKind::Rescue,
            (1, 6, false),
            [0, 0, 1, 0, 0, 1, 0, 0, 1, 0],
        ),
        (
            MissionKind::Abduction,
            (1, 2, false),
            [0, 0, 1, 0, 0, 1, 0, 0, 0, 1],
        ),
        (
            MissionKind::Assassination,
            (1, 1, false),
            [0, 0, 1, 0, 0, 1, 0, 0, 0, 1],
        ),
        (
            MissionKind::Sabotage,
            (1, 2, false),
            [0, 0, 1, 0, 1, 1, 0, 0, 0, 0],
        ),
        (
            MissionKind::DeathStarSabotage,
            (1, 1, false),
            [0, 0, 1, 0, 0, 1, 0, 0, 0, 0],
        ),
    ];
    for (kind, (min, spread, repeats), columns) in expected {
        let id = kind.record_id().expect("an original mission kind");
        let record = world.mission_record(id).expect("a shipped record");
        assert_eq!(
            (
                record.timer_min_days,
                record.timer_spread_days,
                record.repeats
            ),
            (min, spread, repeats),
            "{kind:?}"
        );
        let r = record.rules;
        let read = [
            r.container_loss_ends,
            r.needs_populated_container,
            r.target_loss_ends,
            r.own_side_target,
            r.other_side_target,
            r.opponent_target,
            r.revolting_target,
            r.calm_target,
            r.prisoner_target,
            r.free_target,
        ]
        .map(u8::from);
        assert_eq!(read, columns, "{kind:?}");
    }
    assert_eq!(MissionKind::Autoscrap.record_id(), None);
}

#[test]
#[ignore = "requires original data/base DAT files"]
fn characters_placed_at_game_start_are_recruited_and_the_rest_form_each_side_s_pool() {
    // FUN_0055fe70 places a recruit and sets +0x50 bit 1, so an unplaced
    // character is one Recruitment may still sign (FUN_0055ef30).
    let world = rebellion_data::load_game_data(&data_dir()).expect("load original game data");

    let placed = |c: &rebellion_core::world::Character| {
        c.current_system.is_some() || c.current_fleet.is_some()
    };
    assert!(world.characters.values().all(|c| c.recruited == placed(c)));
    for side in [true, false] {
        let pool = world
            .characters
            .values()
            .filter(|c| c.is_alliance == side && !c.recruited)
            .count();
        assert!(pool > 0, "side alliance={side} has a recruit pool");
    }
    assert!(!world.recruit_pool_empty(rebellion_core::dat::Faction::Alliance));
    assert!(!world.recruit_pool_empty(rebellion_core::dat::Faction::Empire));
}
