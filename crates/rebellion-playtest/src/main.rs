//! Headless playtest runner for Open Rebellion.
//!
//! Runs the full simulation without rendering. The AI controls one
//! faction; the other faction is idle (or both can be AI-controlled
//! once dual-AI support is wired).
//!
//! Usage:
//!   cargo run -p rebellion-playtest -- data/base --seed 42 --ticks 5000 --output playtest.jsonl

mod logger;
mod staging;

use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;
use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use rebellion_core::ai::{AIState, AiFaction};
use rebellion_core::commands::all_commands;
use rebellion_core::dat::Faction;
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::tick::{GameClock, GameSpeed};
use rebellion_core::world::{ControlKind, SeedOptions};
use rebellion_data::simulation::{run_simulation_tick, SimulationStates};

use logger::EventLogger;

#[derive(Parser)]
#[command(
    name = "rebellion-playtest",
    about = "Headless playtest runner for Open Rebellion"
)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "These independent flags preserve the existing state and serialization model."
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

    /// Stream raw JSONL events to stdout as they are generated.
    /// Incompatible with --repl and --exec. Progress/summary goes to stderr.
    #[arg(long, conflicts_with_all = ["repl", "exec"])]
    jsonl: bool,

    /// Path to a JSON game config file (tuning parameters for AI, movement, production).
    /// If omitted, uses built-in defaults.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Record decision traces for these subsystems (comma-separated:
    /// missions, ai, research, blockade, combat, control, or all). Traces
    /// join the event stream as `trace` records.
    #[arg(long)]
    trace: Option<String>,

    /// Start from a save in the game's own format instead of a new
    /// campaign. The save's RNG, AI sides and config continue, so --seed,
    /// --ai-faction and --config (unless given) do not apply.
    #[arg(long)]
    load: Option<PathBuf>,

    /// Write a save in the game's own format when the run ends.
    #[arg(long)]
    save_out: Option<PathBuf>,

    /// Game speed: very-slow, slow, medium or fast. It sets the rounds the
    /// AI runs a day (the per-day count, FUN_0051dea0). A new campaign
    /// starts at fast; --load keeps the save's speed unless given.
    #[arg(long, value_enum)]
    speed: Option<Speed>,
}

/// A game speed the `--speed` flag names.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Speed {
    VerySlow,
    Slow,
    Medium,
    Fast,
}

impl From<Speed> for GameSpeed {
    fn from(speed: Speed) -> Self {
        match speed {
            Speed::VerySlow => Self::VerySlow,
            Speed::Slow => Self::Slow,
            Speed::Medium => Self::Medium,
            Speed::Fast => Self::Fast,
        }
    }
}

/// Advance the clock one day and return the day's tick event.
fn next_day(states: &mut SimulationStates) -> Vec<rebellion_core::tick::TickEvent> {
    states.clock.tick += 1;
    vec![rebellion_core::tick::TickEvent {
        tick: states.clock.tick,
    }]
}

/// Write the `--save-out` save, if one was asked for.
fn write_save_out(
    args: &Args,
    world: &rebellion_core::world::GameWorld,
    states: &SimulationStates,
    rng: &Xoshiro256PlusPlus,
    game_config: &rebellion_core::tuning::GameConfig,
    player_is_alliance: bool,
) -> anyhow::Result<()> {
    if let Some(path) = &args.save_out {
        let fingerprint =
            staging::save_out(path, world, states, rng, game_config, player_is_alliance)?;
        eprintln!(
            "Saved day {} to {} (fingerprint {fingerprint})",
            states.clock.tick,
            path.display()
        );
    }
    Ok(())
}

/// Dispatch a single command against the current world/simulation state.
/// Returns a human-readable result string.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep the existing explicit simulation state inputs at this integration boundary."
)]
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "Elapsed milliseconds cannot approach the u64 limit during a playtest."
)]
fn dispatch_command(
    cmd: &str,
    world: &mut rebellion_core::world::GameWorld,
    states: &mut SimulationStates,
    rng: &mut Xoshiro256PlusPlus,
    logger: &mut EventLogger,
    start: &std::time::Instant,
    ai_faction: AiFaction,
    game_config: &rebellion_core::tuning::GameConfig,
) -> String {
    match cmd {
        "show_game_stats" => {
            let a = world
                .systems
                .values()
                .filter(|s| s.control.is_controlled_by(Faction::Alliance))
                .count();
            let e = world
                .systems
                .values()
                .filter(|s| s.control.is_controlled_by(Faction::Empire))
                .count();
            format!(
                "Stats: {} systems ({} Alliance, {} Empire), {} fleets, {} characters",
                world.systems.len(),
                a,
                e,
                world.fleets.len(),
                world.characters.len()
            )
        }
        "list_active_missions" => {
            let missions = states.missions.missions();
            if missions.is_empty() {
                "No active missions".to_string()
            } else {
                missions
                    .iter()
                    .map(|m| format!("{:?} by {:?}", m.kind, m.faction))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
        "list_active_fleets" => world
            .fleets
            .iter()
            .map(|(_, f)| {
                let sys = world
                    .systems
                    .get(f.location)
                    .map_or("?", |s| s.name.as_str());
                let side = if f.is_alliance { "Alliance" } else { "Empire" };
                let ships = f.ship_count() as usize
                    + f.fighters.iter().map(|e| e.count as usize).sum::<usize>();
                format!("{side} fleet at {sys} — {ships} ships")
            })
            .collect::<Vec<_>>()
            .join("\n"),
        "show_event_count" => {
            let total = states.events.events().len();
            let fired = states
                .events
                .events()
                .iter()
                .filter(|e| states.events.has_fired(e.id))
                .count();
            format!("Events: {total} defined, {fired} fired")
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
            for (key, _) in &world.systems {
                states.fog.reveal(key);
            }
            "All systems revealed".to_string()
        }
        "force_victory_check" => {
            let tick_ev = rebellion_core::tick::TickEvent { tick: 0 };
            let result = rebellion_core::victory::VictorySystem::check(
                &states.victory,
                world,
                &[tick_ev],
                states.campaign_config.victory_conditions,
            );
            match result {
                Some(outcome) => format!("Victory: {outcome:?}"),
                None => "No winner yet".to_string(),
            }
        }
        id if id.starts_with("advance_") => {
            let n: u64 = id
                .trim_start_matches("advance_")
                .trim_end_matches("_tick")
                .trim_end_matches("_ticks")
                .trim_end_matches('s')
                .parse()
                .unwrap_or(1);
            for _ in 0..n {
                let tick_events = next_day(states);
                let rolls: Vec<f64> = (0..1024).map(|_| rng.gen::<f64>()).collect();
                let wall_ms = start.elapsed().as_millis() as u64;
                let evts =
                    run_simulation_tick(world, states, &tick_events, &rolls, wall_ms, game_config);
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
                Err(e) => format!("Export failed: {e}"),
            }
        }
        _ => format!("Unknown command: '{cmd}'. Use --exec list for available commands."),
    }
}

/// The state readouts the native dev channel shares
/// (`rebellion_data::readouts`): `list characters [side]`, `list missions
/// [side]`, `show research`, `list production [side]` and `show side <side>`.
/// They see through fog of war: dev truth, not a player's view. `None` for
/// any other command.
fn readout(
    cmd: &str,
    world: &rebellion_core::world::GameWorld,
    states: &SimulationStates,
    tick: u64,
) -> Option<String> {
    use rebellion_data::readouts;
    let lower = cmd.trim().to_ascii_lowercase();
    let side = |prefix: &str| readouts::parse_side(&lower[prefix.len()..]);
    let text = if lower.starts_with("list characters") {
        side("list characters")
            .map(|side| readouts::list_characters(world, &states.missions, &states.movement, side))
    } else if lower.starts_with("list missions") {
        side("list missions")
            .map(|side| readouts::list_missions(world, &states.missions, tick, side))
    } else if lower.starts_with("list production") {
        side("list production")
            .map(|side| readouts::list_production(world, &states.manufacturing, tick, side))
    } else if lower == "show research" {
        Ok(readouts::show_research(world, &states.research))
    } else if lower.starts_with("show side") {
        side("show side").and_then(|side| {
            let side = side.ok_or_else(|| "show side takes alliance or empire".to_owned())?;
            Ok(readouts::show_side(
                world,
                &states.stockpiles,
                &states.manufacturing,
                &states.missions,
                side,
            ))
        })
    } else {
        return None;
    };
    Some(text.unwrap_or_else(|why| why))
}

#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "Elapsed milliseconds cannot approach the u64 limit during a playtest."
)]
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
            eprintln!("Unknown command: '{cmd}'. Use --exec list to see available commands.");
            std::process::exit(1);
        }
    }

    if let Some(spec) = &args.trace {
        let mask =
            rebellion_core::trace::TraceMask::parse(spec).map_err(|why| anyhow::anyhow!(why))?;
        rebellion_core::trace::set_mask(mask);
    }

    // Load game data with deterministic seed options, or stage from a save
    let (mut world, mut staged) = if let Some(path) = &args.load {
        eprintln!("Loading save {}...", path.display());
        let (world, staged) = staging::load(path)?;
        (world, Some(staged))
    } else {
        eprintln!("Loading game data from {}...", args.data_dir.display());
        let seed_options = SeedOptions {
            rng_seed: Some(args.seed),
            ..SeedOptions::default()
        };
        let world = rebellion_data::load_game_data_with_options(&args.data_dir, &seed_options)?;
        (world, None)
    };
    eprintln!(
        "Loaded: {} systems, {} characters, {} fleets",
        world.systems.len(),
        world.characters.len(),
        world.fleets.len()
    );

    // Initialize RNG
    let mut rng = staged.as_ref().map_or_else(
        || Xoshiro256PlusPlus::seed_from_u64(args.seed),
        |staged| staged.rng.clone(),
    );
    eprintln!("Seed: {}", args.seed);

    // Load game config (tuning parameters for AI, movement, production)
    let game_config: rebellion_core::tuning::GameConfig = if let Some(ref path) = args.config {
        let text = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read config {}: {}", path.display(), e))?;
        serde_json::from_str(&text)
            .map_err(|e| anyhow::anyhow!("Failed to parse config {}: {}", path.display(), e))?
    } else if let Some(staged) = &staged {
        staged.game_config.clone()
    } else {
        rebellion_core::tuning::GameConfig::default()
    };
    if let Some(config) = &args.config {
        eprintln!("Config: {}", config.display());
    }

    // Determine AI faction
    let ai_faction = if let Some(staged) = &staged {
        staged.ai_faction
    } else if args.ai_faction.to_lowercase() == "alliance" {
        AiFaction::Alliance
    } else {
        AiFaction::Empire
    };
    let player_is_alliance = staged
        .as_ref()
        .map_or(ai_faction != AiFaction::Alliance, |staged| {
            staged.player_is_alliance
        });
    let fog_faction = if player_is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    };

    // Find HQ systems for victory state
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
    let (victory_a, victory_e) = if let (Some(a), Some(e)) = (a_hq, e_hq) {
        (a, e)
    } else {
        let mut keys = world.systems.keys();
        let a = keys.next().expect("need at least 2 systems");
        let e = keys.next().expect("need at least 2 systems");
        (a, e)
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
        economy: rebellion_core::economy::EconomyState::default(),
        repair: rebellion_core::repair::RepairState::default(),
        troop_transport: rebellion_core::troop_transport::TroopTransportState::default(),
        deliveries: rebellion_core::delivery::DeliveryState::default(),
        stockpiles: rebellion_core::stockpiles::StockpileState::default(),
        campaign_config: rebellion_core::world::CampaignConfig::default(),
    };

    if let Some(staged) = staged.take() {
        // A save carries its own fog, story events and clock.
        states = staged.states;
        if args.dual_ai && states.ai2.is_none() {
            states.ai2 = Some(AIState::new(match ai_faction {
                AiFaction::Empire => AiFaction::Alliance,
                AiFaction::Alliance => AiFaction::Empire,
            }));
        }
    } else {
        // Seed fog
        FogSystem::seed(&mut states.fog, &world);

        // Register story events
        rebellion_core::story_events::define_story_events(&mut states.events, &world);

        // Set clock to maximum speed for headless
        states.clock.set_speed(GameSpeed::Fast);
    }
    if let Some(speed) = args.speed {
        states.clock.set_speed(speed.into());
    }

    let mut logger = EventLogger::new();
    let mut victory_reached = false;

    // Handle --exec: dispatch single command and exit
    if let Some(ref cmd) = args.exec {
        let output = dispatch_command(
            cmd,
            &mut world,
            &mut states,
            &mut rng,
            &mut logger,
            &start,
            ai_faction,
            &game_config,
        );
        println!("{output}");
        write_save_out(
            &args,
            &world,
            &states,
            &rng,
            &game_config,
            player_is_alliance,
        )?;
        return Ok(());
    }

    // Handle --repl: interactive command loop with persistent state
    if args.repl {
        eprintln!("REPL mode. Type commands (one per line). 'quit' to exit, 'help' for commands.");
        let stdin = std::io::stdin();
        loop {
            let tick_counter = states.clock.tick;
            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Read error: {e}");
                    break;
                }
            }
            let cmd = line.trim();
            if cmd.is_empty() {
                continue;
            }
            if cmd == "quit" || cmd == "exit" {
                break;
            }
            if cmd == "help" {
                let commands = all_commands();
                for c in &commands {
                    println!("  {:30} [{}] {}", c.id, c.category, c.description);
                }
                println!("  {:30} Advance simulation and print events", "tick [N]");
                println!(
                    "  {:30} State readouts (dev truth, no fog of war)",
                    "list characters|missions|production [side], show research|side <side>"
                );
                println!("  {:30} Exit REPL", "quit");
                continue;
            }
            // Special: "tick N" advances N ticks and prints new events
            if cmd.starts_with("tick") {
                let n: u64 = cmd.trim_start_matches("tick").trim().parse().unwrap_or(1);
                let pre_count = logger.len();
                for _ in 0..n {
                    let tick_events = next_day(&mut states);
                    let rolls: Vec<f64> = (0..1024).map(|_| rng.gen::<f64>()).collect();
                    let wall_ms = start.elapsed().as_millis() as u64;
                    let evts = run_simulation_tick(
                        &mut world,
                        &mut states,
                        &tick_events,
                        &rolls,
                        wall_ms,
                        &game_config,
                    );
                    logger.extend(evts);
                }
                let new_events = logger.len() - pre_count;
                let json = serde_json::json!({
                    "tick": states.clock.tick,
                    "advanced": n,
                    "new_events": new_events,
                    "victory": states.victory.resolved,
                });
                println!("{json}");
                continue;
            }
            // REPL-only inspection commands (not in shared command registry)
            if let Some(text) = readout(cmd, &world, &states, tick_counter) {
                println!("{text}");
                continue;
            }
            if cmd == "systems" {
                let mut alliance = Vec::new();
                let mut empire = Vec::new();
                let mut neutral = 0usize;
                for (_, sys) in &world.systems {
                    match sys.control {
                        ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => {
                            alliance.push(sys.name.as_str());
                        }
                        ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => {
                            empire.push(sys.name.as_str());
                        }
                        _ => neutral += 1,
                    }
                }
                let json = serde_json::json!({
                    "command": "systems",
                    "alliance": { "count": alliance.len(), "systems": alliance },
                    "empire": { "count": empire.len(), "systems": empire },
                    "neutral": neutral,
                    "tick": tick_counter,
                });
                println!("{json}");
                continue;
            }
            if cmd == "transit" {
                let orders: Vec<_> = states
                    .movement
                    .orders()
                    .values()
                    .map(|order| {
                        let origin = world
                            .systems
                            .get(order.origin)
                            .map_or("?", |s| s.name.as_str());
                        let dest = world
                            .systems
                            .get(order.destination)
                            .map_or("?", |s| s.name.as_str());
                        let faction = world.fleets.get(order.fleet).map_or("?", |f| {
                            if f.is_alliance {
                                "Alliance"
                            } else {
                                "Empire"
                            }
                        });
                        serde_json::json!({
                            "faction": faction,
                            "origin": origin,
                            "destination": dest,
                            "progress": format!("{:.0}%", order.progress() * 100.0),
                            "ticks_remaining": order.ticks_remaining(),
                        })
                    })
                    .collect();
                let json = serde_json::json!({
                    "command": "transit",
                    "count": orders.len(),
                    "orders": orders,
                    "tick": tick_counter,
                });
                println!("{json}");
                continue;
            }
            if cmd.starts_with("events") {
                let n: usize = cmd
                    .trim_start_matches("events")
                    .trim()
                    .parse()
                    .unwrap_or(10);
                let all = logger.events();
                let start_idx = all.len().saturating_sub(n);
                let recent: Vec<_> = all[start_idx..].iter().collect();
                let json = serde_json::json!({
                    "command": "events",
                    "count": recent.len(),
                    "events": recent,
                    "tick": tick_counter,
                });
                println!("{json}");
                continue;
            }
            let output = dispatch_command(
                cmd,
                &mut world,
                &mut states,
                &mut rng,
                &mut logger,
                &start,
                ai_faction,
                &game_config,
            );
            let json = serde_json::json!({
                "command": cmd,
                "tick": states.clock.tick,
                "result": output,
                "victory": states.victory.resolved,
                "events_total": logger.len(),
            });
            println!("{json}");
        }
        write_save_out(
            &args,
            &world,
            &states,
            &rng,
            &game_config,
            player_is_alliance,
        )?;
        return Ok(());
    }

    eprintln!("Running {} ticks...", args.ticks);

    for tick_num in 1..=args.ticks {
        // Generate tick events (force 1 tick per iteration in headless)
        let tick_events = next_day(&mut states);
        let wall_ms = start.elapsed().as_millis() as u64;

        // Generate rolls for this tick
        let rolls: Vec<f64> = (0..1024).map(|_| rng.gen::<f64>()).collect();

        // Run shared simulation tick
        let events = run_simulation_tick(
            &mut world,
            &mut states,
            &tick_events,
            &rolls,
            wall_ms,
            &game_config,
        );
        if args.jsonl {
            for event in &events {
                println!("{}", serde_json::to_string(event).unwrap_or_default());
            }
        }
        logger.extend(events);

        // Check victory
        if states.victory.resolved {
            eprintln!("Victory condition reached at tick {}!", states.clock.tick);
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

    write_save_out(
        &args,
        &world,
        &states,
        &rng,
        &game_config,
        player_is_alliance,
    )?;

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
        logger.print_summary(&world, &states.movement);
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
