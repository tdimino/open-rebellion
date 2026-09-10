//! Determinism acceptance probe for versioned save-state fingerprints.
//!
//! The current tracked test covers the seeded initial campaign. Longer command
//! stream, native/WASM, save/load, and tactical-path comparisons remain F-011
//! roadmap work.

use std::path::PathBuf;

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use rebellion_core::ai::{AIState, AiFaction};
use rebellion_core::betrayal::BetrayalState;
use rebellion_core::blockade::BlockadeState;
use rebellion_core::dat::Faction;
use rebellion_core::death_star::DeathStarState;
use rebellion_core::economy::EconomyState;
use rebellion_core::events::EventState;
use rebellion_core::fog::FogState;
use rebellion_core::jedi::JediState;
use rebellion_core::manufacturing::ManufacturingState;
use rebellion_core::missions::MissionState;
use rebellion_core::movement::MovementState;
use rebellion_core::repair::RepairState;
use rebellion_core::research::ResearchState;
use rebellion_core::tick::GameClock;
use rebellion_core::tuning::GameConfig;
use rebellion_core::uprising::UprisingState;
use rebellion_core::victory::VictoryState;
use rebellion_core::world::{CampaignConfig, SeedOptions};
use rebellion_data::save::{compute_state_fingerprint, SaveState};

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("repository root")
        .join("data/base")
}

fn seeded_snapshot(seed: u64) -> SaveState {
    let options = SeedOptions {
        rng_seed: Some(seed),
        ..SeedOptions::default()
    };
    let world = rebellion_data::load_game_data_with_options(&data_dir(), &options)
        .expect("load deterministic campaign fixture");
    let alliance_hq = world
        .systems
        .iter()
        .find(|(_, system)| {
            system.is_headquarters && system.control.is_controlled_by(Faction::Alliance)
        })
        .map(|(key, _)| key)
        .expect("Alliance headquarters");
    let empire_hq = world
        .systems
        .iter()
        .find(|(_, system)| {
            system.is_headquarters && system.control.is_controlled_by(Faction::Empire)
        })
        .map(|(key, _)| key)
        .expect("Empire headquarters");

    SaveState {
        world,
        clock: GameClock::new(),
        manufacturing: ManufacturingState::new(),
        missions: MissionState::new(),
        events: EventState::new(),
        ai: AIState::new(AiFaction::Empire),
        movement: MovementState::new(),
        fog_alliance: FogState::new(Faction::Alliance),
        fog_empire: FogState::new(Faction::Empire),
        player_is_alliance: true,
        blockade: BlockadeState::new(),
        uprising: UprisingState::new(),
        death_star: DeathStarState::default(),
        research: ResearchState::new(),
        jedi: JediState::new(),
        victory: VictoryState::new(alliance_hq, empire_hq),
        betrayal: BetrayalState::new(),
        economy: EconomyState::default(),
        sim_rng: Xoshiro256PlusPlus::seed_from_u64(seed),
        ai2: None,
        repair: RepairState::default(),
        combat_cooldowns: std::collections::HashMap::new(),
        game_config: GameConfig::default(),
        campaign_config: CampaignConfig::from_seed_options(
            options,
            rebellion_core::world::VictoryConditions::Standard,
        ),
        troop_transport: rebellion_core::troop_transport::TroopTransportState::default(),
    }
}

#[test]
#[ignore = "requires original data/base DAT files"]
fn identical_seeded_campaigns_have_identical_initial_fingerprints() {
    let first = compute_state_fingerprint(&seeded_snapshot(42)).expect("first fingerprint");
    let second = compute_state_fingerprint(&seeded_snapshot(42)).expect("second fingerprint");

    assert_eq!(first, second);
}
