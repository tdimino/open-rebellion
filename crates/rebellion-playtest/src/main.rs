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
use rebellion_core::commands::all_commands;
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

    /// Both factions controlled by AI
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

    /// Execute a single command from the shared registry and exit.
    /// Use --exec list to see available commands.
    #[arg(long)]
    exec: Option<String>,

    /// Interactive REPL mode: read commands from stdin, execute against
    /// persistent state, print JSON results. Enables LLM agent play.
    #[arg(long)]
    repl: bool,
}

/// Dispatch a single command against the current world/simulation state.
/// Returns a human-readable result string.
fn dispatch_command(
    cmd: &str,
    world: &mut rebellion_core::world::GameWorld,
    states: &mut SimulationStates,
    rng: &mut Xoshiro256PlusPlus,
    logger: &mut EventLogger,
    start: &std::time::Instant,
    ai_faction: AiFaction,
) -> String {
    match cmd {
        "show_game_stats" => {
            let a = world.systems.values().filter(|s| s.controlling_faction == Some(Faction::Alliance)).count();
            let e = world.systems.values().filter(|s| s.controlling_faction == Some(Faction::Empire)).count();
            format!("Stats: {} systems ({} Alliance, {} Empire), {} fleets, {} characters",
                world.systems.len(), a, e, world.fleets.len(), world.characters.len())
        }
        "list_active_missions" => {
            let missions = states.missions.missions();
            if missions.is_empty() {
                "No active missions".to_string()
            } else {
                missions.iter().map(|m| format!("{:?} by {:?}", m.kind, m.faction)).collect::<Vec<_>>().join("\n")
            }
        }
        "list_active_fleets" => {
            world.fleets.iter().map(|(_, f)| {
                let sys = world.systems.get(f.location).map(|s| s.name.as_str()).unwrap_or("?");
                let side = if f.is_alliance { "Alliance" } else { "Empire" };
                let ships = f.capital_ships.iter().map(|e| e.count as usize).sum::<usize>()
                    + f.fighters.iter().map(|e| e.count as usize).sum::<usize>();
                format!("{} fleet at {} — {} ships", side, sys, ships)
            }).collect::<Vec<_>>().join("\n")
        }
        "show_event_count" => {
            let total = states.events.events().len();
            let fired = states.events.events().iter().filter(|e| states.events.has_fired(e.id)).count();
            format!("Events: {} defined, {} fired", total, fired)
        }
        "toggle_dual_ai" => {
            if states.ai2.is_some() {
                states.ai2 = None;
                "Dual AI disabled".to_string()
            } else {
                let second = match ai_faction {
                    AiFaction::Empire => AiFaction::Alliance,
                    AiFaction::Alliance => AiFaction::Empire,
                };
                states.ai2 = Some(AIState::new(second));
                "Dual AI enabled".to_string()
            }
        }
        "reveal_all_systems" => {
            for (key, _) in world.systems.iter() {
                states.fog.reveal(key);
            }
            "All systems revealed".to_string()
        }
        "force_victory_check" => {
            let tick_ev = rebellion_core::tick::TickEvent { tick: 0 };
            let result = rebellion_core::victory::VictorySystem::check(&states.victory, world, &[tick_ev]);
            match result {
                Some(outcome) => format!("Victory: {:?}", outcome),
                None => "No winner yet".to_string(),
            }
        }
        id if id.starts_with("advance_") => {
            let n: u64 = id.trim_start_matches("advance_")
                .trim_end_matches("_tick").trim_end_matches("_ticks").trim_end_matches("s")
                .parse().unwrap_or(1);
            for t in 1..=n {
                let tick_events = vec![rebellion_core::tick::TickEvent { tick: t }];
                let rolls: Vec<f64> = (0..1024).map(|_| rng.gen::<f64>()).collect();
                let wall_ms = start.elapsed().as_millis() as u64;
                let evts = run_simulation_tick(world, states, &tick_events, &rolls, wall_ms);
                logger.extend(evts);
            }
            format!("Advanced {} ticks ({} events)", n, logger.len())
        }
        id if id.starts_with("speed_") => {
            let _ = id;
            "Speed commands are not applicable in headless mode".to_string()
        }
        "export_game_log" => {
            let path = std::path::PathBuf::from("playtest_exec.jsonl");
            match logger.export_jsonl(&path) {
                Ok(()) => format!("Exported to {}", path.display()),
                Err(e) => format!("Export failed: {}", e),
            }
        }
        _ => format!("Unknown command: '{}'. Use --exec list for available commands.", cmd),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let start = Instant::now();

    if args.dual_ai {
        eprintln!("Dual-AI mode: both factions AI-controlled");
    }

    // Handle --exec list (show available commands and exit)
    if let Some(ref cmd) = args.exec {
        if cmd == "list" {
            let commands = all_commands();
            eprintln!("Available commands:");
            for c in &commands {
                eprintln!("  {:30} [{}] {}", c.id, c.category, c.description);
            }
            return Ok(());
        }
        // Validate command exists
        let commands = all_commands();
        if !commands.iter().any(|c| c.id == cmd.as_str()) {
            eprintln!("Unknown command: '{}'. Use --exec list to see available commands.", cmd);
            std::process::exit(1);
        }
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

    // Determine second AI faction for dual-AI mode
    let ai2 = if args.dual_ai {
        let second_faction = match ai_faction {
            AiFaction::Empire => AiFaction::Alliance,
            AiFaction::Alliance => AiFaction::Empire,
        };
        Some(AIState::new(second_faction))
    } else {
        None
    };

    // Initialize simulation states
    let mut states = SimulationStates {
        clock: GameClock::new(),
        manufacturing: rebellion_core::manufacturing::ManufacturingState::new(),
        missions: rebellion_core::missions::MissionState::new(),
        events: rebellion_core::events::EventState::new(),
        ai: AIState::new(ai_faction),
        ai2,
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

    // Handle --exec: dispatch single command and exit
    if let Some(ref cmd) = args.exec {
        let output = dispatch_command(cmd, &mut world, &mut states, &mut rng, &mut logger, &start, ai_faction);
        println!("{}", output);
        return Ok(());
    }

    // Handle --repl: interactive command loop with persistent state
    if args.repl {
        eprintln!("REPL mode. Type commands (one per line). 'quit' to exit, 'help' for commands.");
        let stdin = std::io::stdin();
        let mut tick_counter: u64 = 0;
        loop {
            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    break;
                }
            }
            let cmd = line.trim();
            if cmd.is_empty() { continue; }
            if cmd == "quit" || cmd == "exit" { break; }
            if cmd == "help" {
                let commands = all_commands();
                for c in &commands {
                    println!("  {:30} [{}] {}", c.id, c.category, c.description);
                }
                println!("  {:30} Advance simulation and print events", "tick [N]");
                println!("  {:30} Exit REPL", "quit");
                continue;
            }
            // Special: "tick N" advances N ticks and prints new events
            if cmd.starts_with("tick") {
                let n: u64 = cmd.trim_start_matches("tick").trim().parse().unwrap_or(1);
                let pre_count = logger.len();
                for _ in 0..n {
                    tick_counter += 1;
                    let tick_events = vec![rebellion_core::tick::TickEvent { tick: tick_counter }];
                    let rolls: Vec<f64> = (0..1024).map(|_| rng.gen::<f64>()).collect();
                    let wall_ms = start.elapsed().as_millis() as u64;
                    let evts = run_simulation_tick(&mut world, &mut states, &tick_events, &rolls, wall_ms);
                    logger.extend(evts);
                }
                let new_events = logger.len() - pre_count;
                let json = serde_json::json!({
                    "tick": tick_counter,
                    "advanced": n,
                    "new_events": new_events,
                    "victory": states.victory.resolved,
                });
                println!("{}", json);
                continue;
            }
            let output = dispatch_command(cmd, &mut world, &mut states, &mut rng, &mut logger, &start, ai_faction);
            let json = serde_json::json!({
                "command": cmd,
                "tick": tick_counter,
                "result": output,
                "victory": states.victory.resolved,
                "events_total": logger.len(),
            });
            println!("{}", json);
        }
        return Ok(());
    }

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
