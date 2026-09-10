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

#[test]
fn original_troop_classes_use_compound_dat_ids() {
    let world = rebellion_data::load_game_data(&data_dir()).expect("load original game data");

    assert_eq!(world.troop_classes.len(), 10);
    for troop in world.troops.values() {
        assert_eq!(troop.class_dat_id.family(), 0x10);
        assert!(
            world.troop_classes.contains_key(&troop.class_dat_id),
            "seeded troop {:?} must resolve to its TROOPSD class",
            troop.class_dat_id
        );
    }

    let alliance_army = world
        .troop_classes
        .get(&DatId::new(0x1000_0002))
        .expect("Alliance Army compound class id");
    assert!(alliance_army.attack_strength > 0);
    assert!(alliance_army.defense_strength > 0);
}
