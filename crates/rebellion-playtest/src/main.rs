//! Headless playtest runner for Open Rebellion.
//!
//! Runs the full simulation without rendering. The AI controls one
//! faction; the other faction is idle (or both can be AI-controlled
//! once dual-AI support is wired).
//!
//! Usage:
//!   cargo run -p rebellion-playtest -- data/base --seed 42 --ticks 5000 --output playtest.jsonl

mod logger;

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;
use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use rebellion_core::ai::{AiFaction, AIState};
use rebellion_core::dat::Faction;
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::tick::{GameClock, GameSpeed};
use rebellion_data::simulation::{run_simulation_tick, SimulationStates};

use logger::EventLogger;

#[derive(Parser)]
#[command(
    name = "rebellion-playtest",
    about = "Headless playtest runner for Open Rebellion"
)]
struct Args {
    /// Path to game data directory (containing .DAT files)
    data_dir: PathBuf,

    /// RNG seed for deterministic simulation
    #[arg(long, default_value_t = 0)]
    seed: u64,

    /// Number of ticks to simulate
    #[arg(long, default_value_t = 5000)]
    ticks: u64,

    /// Both factions controlled by AI (not yet implemented)
    #[arg(long)]
    dual_ai: bool,

    /// Output JSONL file path
    #[arg(long)]
    output: Option<PathBuf>,

    /// Print summary statistics on exit
    #[arg(long)]
    summary: bool,

    /// AI faction: "empire" or "alliance" (default: empire)
    #[arg(long, default_value = "empire")]
    ai_faction: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let start = Instant::now();

    if args.dual_ai {
        eprintln!(
            "Warning: --dual-ai is accepted but not yet wired. \
             Only the AI faction ({}) will act.",
            args.ai_faction
        );
    }

    // Load game data
    eprintln!("Loading game data from {}...", args.data_dir.display());
    let mut world = rebellion_data::load_game_data(&args.data_dir)?;
    eprintln!(
        "Loaded: {} systems, {} characters, {} fleets",
        world.systems.len(),
        world.characters.len(),
        world.fleets.len()
    );

    // Initialize RNG
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(args.seed);
    eprintln!("Seed: {}", args.seed);

    // Determine AI faction
    let ai_faction = if args.ai_faction.to_lowercase() == "alliance" {
        AiFaction::Alliance
    } else {
        AiFaction::Empire
    };
    let player_is_alliance = ai_faction != AiFaction::Alliance;
    let fog_faction = if player_is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };

    // Find HQ systems for victory state
    let a_hq = world
        .systems
        .iter()
        .find(|(_, s)| s.is_headquarters && s.controlling_faction == Some(Faction::Alliance))
        .map(|(k, _)| k);
    let e_hq = world
        .systems
        .iter()
        .find(|(_, s)| s.is_headquarters && s.controlling_faction == Some(Faction::Empire))
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

    // Initialize simulation states
    let mut states = SimulationStates {
        clock: GameClock::new(),
        manufacturing: rebellion_core::manufacturing::ManufacturingState::new(),
        missions: rebellion_core::missions::MissionState::new(),
        events: rebellion_core::events::EventState::new(),
        ai: AIState::new(ai_faction),
        movement: rebellion_core::movement::MovementState::new(),
        fog: FogState::new(fog_faction),
        blockade: rebellion_core::blockade::BlockadeState::new(),
        uprising: rebellion_core::uprising::UprisingState::new(),
        death_star: rebellion_core::death_star::DeathStarState::default(),
        research: rebellion_core::research::ResearchState::new(),
        jedi: rebellion_core::jedi::JediState::new(),
        victory: rebellion_core::victory::VictoryState::new(victory_a, victory_e),
        betrayal: rebellion_core::betrayal::BetrayalState::new(),
        combat_cooldowns: HashMap::new(),
    };

    // Seed fog
    FogSystem::seed(&mut states.fog, &world);

    // Register story events
    rebellion_core::story_events::define_story_events(&mut states.events, &world);

    // Set clock to maximum speed for headless
    states.clock.set_speed(GameSpeed::Faster);

    let mut logger = EventLogger::new();
    let mut victory_reached = false;

    eprintln!("Running {} ticks...", args.ticks);

    for tick_num in 1..=args.ticks {
        // Generate tick events (force 1 tick per iteration in headless)
        let tick_events = vec![rebellion_core::tick::TickEvent { tick: tick_num }];
        let wall_ms = start.elapsed().as_millis() as u64;

        // Generate rolls for this tick
        let rolls: Vec<f64> = (0..1024).map(|_| rng.gen::<f64>()).collect();

        // Run shared simulation tick
        let events = run_simulation_tick(&mut world, &mut states, &tick_events, &rolls, wall_ms);
        logger.extend(events);

        // Check victory
        if states.victory.resolved {
            eprintln!("Victory condition reached at tick {}!", tick_num);
            victory_reached = true;
            break;
        }

        // Progress indicator
        if tick_num % 1000 == 0 {
            eprintln!(
                "  tick {}/{} ({} events so far)",
                tick_num,
                args.ticks,
                logger.len()
            );
        }
    }

    // Export results
    if let Some(ref output_path) = args.output {
        logger.export_jsonl(output_path)?;
        eprintln!(
            "Exported {} events to {}",
            logger.len(),
            output_path.display()
        );
    }

    if args.summary || args.output.is_none() {
        logger.print_summary();
    }

    let elapsed = start.elapsed();
    eprintln!(
        "\nCompleted in {:.1}s ({} ticks, {} events, victory: {})",
        elapsed.as_secs_f64(),
        if victory_reached {
            "early".to_string()
        } else {
            format!("{}", args.ticks)
        },
        logger.len(),
        victory_reached
    );

    Ok(())
}
