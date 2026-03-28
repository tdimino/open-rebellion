//! Integration test: verify every SYS_* telemetry constant emits at least one
//! GameEventRecord during a 1000-tick dual-AI playtest.
//!
//! This test requires game data files in `data/base/`. It is `#[ignore]`d by
//! default so `cargo test` passes without DAT files. Run explicitly:
//!
//! ```bash
//! PATH="/usr/bin:$PATH" cargo test -p rebellion-data --test telemetry_coverage -- --ignored
//! ```

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use rebellion_core::ai::{AiFaction, AIState};
use rebellion_core::dat::Faction;
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::game_events::*;
use rebellion_core::tick::{GameClock, GameSpeed};
use rebellion_core::tuning::GameConfig;
use rebellion_core::world::SeedOptions;
use rebellion_data::simulation::{run_simulation_tick, SimulationStates};

/// All SYS_* constants that should emit at least one event.
const ALL_SYSTEMS: &[&str] = &[
    SYS_MANUFACTURING,
    SYS_MOVEMENT,
    SYS_COMBAT,
    SYS_FOG,
    SYS_MISSIONS,
    SYS_EVENTS,
    SYS_AI,
    SYS_BLOCKADE,
    SYS_UPRISING,
    SYS_DEATH_STAR,
    SYS_RESEARCH,
    SYS_JEDI,
    SYS_VICTORY,
    SYS_BETRAYAL,
    SYS_STORY,
    SYS_ECONOMY,
    SYS_REPAIR,
];

/// Systems that may legitimately not fire in a 1000-tick simulation.
/// These are checked but failures are reported as warnings, not errors.
const OPTIONAL_SYSTEMS: &[&str] = &[
    SYS_VICTORY,    // Victory requires HQ capture or Death Star events
    SYS_BETRAYAL,   // Betrayal requires specific loyalty + RNG conditions
    SYS_STORY,      // Story events require specific character co-location
    SYS_JEDI,       // Jedi training requires specific character + time
    SYS_REPAIR,     // Repair requires damaged ships at shipyard systems
    SYS_DEATH_STAR, // Death Star construction takes many ticks; game may end first
    SYS_UPRISING,   // Uprising requires specific popularity + RNG conditions
];

fn data_dir() -> PathBuf {
    // Walk up from the test binary location to find the workspace root
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.parent().unwrap().parent().unwrap().join("data/base")
}

#[test]
#[ignore] // requires data/base/ DAT files
fn telemetry_coverage_all_sys_constants_emit() {
    let data_path = data_dir();
    if !data_path.exists() {
        panic!(
            "Game data directory not found at {}. \
             This test requires the original DAT files in data/base/.",
            data_path.display()
        );
    }

    // Load game world with deterministic seed
    let seed_options = SeedOptions {
        rng_seed: Some(42),
        ..SeedOptions::default()
    };
    let mut world = rebellion_data::load_game_data_with_options(&data_path, &seed_options)
        .expect("Failed to load game data");

    // Set up dual-AI simulation
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    // Find HQ systems
    let a_hq = world
        .systems
        .iter()
        .find(|(_, s)| s.is_headquarters && s.control.is_controlled_by(Faction::Alliance))
        .map(|(k, _)| k);
    let e_hq = world
        .systems
        .iter()
        .find(|(_, s)| s.is_headquarters && s.control.is_controlled_by(Faction::Empire))
        .map(|(k, _)| k);
    let (victory_a, victory_e) = match (a_hq, e_hq) {
        (Some(a), Some(e)) => (a, e),
        _ => {
            let mut keys = world.systems.keys();
            let a = keys.next().expect("need at least 2 systems");
            let e = keys.next().expect("need at least 2 systems");
            (a, e)
        }
    };

    let mut states = SimulationStates {
        clock: GameClock::new(),
        manufacturing: rebellion_core::manufacturing::ManufacturingState::new(),
        missions: rebellion_core::missions::MissionState::new(),
        events: rebellion_core::events::EventState::new(),
        ai: AIState::new(AiFaction::Empire),
        ai2: Some(AIState::new(AiFaction::Alliance)),
        movement: rebellion_core::movement::MovementState::new(),
        fog: FogState::new(Faction::Alliance),
        blockade: rebellion_core::blockade::BlockadeState::new(),
        uprising: rebellion_core::uprising::UprisingState::new(),
        death_star: rebellion_core::death_star::DeathStarState::default(),
        research: rebellion_core::research::ResearchState::new(),
        jedi: rebellion_core::jedi::JediState::new(),
        victory: rebellion_core::victory::VictoryState::new(victory_a, victory_e),
        betrayal: rebellion_core::betrayal::BetrayalState::new(),
        economy: rebellion_core::economy::EconomyState::default(),
        repair: rebellion_core::repair::RepairState,
        combat_cooldowns: HashMap::new(),
    };

    // Seed fog and story events
    FogSystem::seed(&mut states.fog, &world);
    rebellion_core::story_events::define_story_events(&mut states.events, &world);
    states.clock.set_speed(GameSpeed::Faster);

    let config = GameConfig::default();

    // Run 1000 ticks, collect all events
    let mut system_counts: HashMap<&str, usize> = HashMap::new();
    let mut total_events = 0usize;

    for tick in 1..=1000u64 {
        let tick_events = vec![rebellion_core::tick::TickEvent { tick }];
        let rolls: Vec<f64> = (0..1024).map(|_| rng.gen::<f64>()).collect();
        let events = run_simulation_tick(&mut world, &mut states, &tick_events, &rolls, tick, &config);

        for evt in &events {
            *system_counts.entry(evt.system).or_insert(0) += 1;
        }
        total_events += events.len();

        // Early exit if victory is reached
        if states.victory.resolved {
            eprintln!("Victory reached at tick {} — stopping early", tick);
            break;
        }
    }

    // Report coverage
    eprintln!("\n=== Telemetry Coverage Report ({} total events) ===", total_events);
    let optional: HashSet<&str> = OPTIONAL_SYSTEMS.iter().copied().collect();
    let mut missing_required = Vec::new();
    let mut missing_optional = Vec::new();

    for sys in ALL_SYSTEMS {
        let count = system_counts.get(sys).copied().unwrap_or(0);
        let marker = if count > 0 { "✓" } else if optional.contains(sys) { "○" } else { "✗" };
        eprintln!("  {} {:20} {:>6} events", marker, sys, count);
        if count == 0 {
            if optional.contains(sys) {
                missing_optional.push(*sys);
            } else {
                missing_required.push(*sys);
            }
        }
    }

    if !missing_optional.is_empty() {
        eprintln!(
            "\nOptional systems with zero events (not a failure): {:?}",
            missing_optional
        );
    }

    assert!(
        missing_required.is_empty(),
        "Required systems with zero telemetry events: {:?}. \
         These systems must emit at least one GameEventRecord in a 1000-tick dual-AI playtest.",
        missing_required
    );
}
