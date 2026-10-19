//! Staging a playtest from a save, and saving where it ends.
//!
//! `--load` starts from a save in the game's own format (a native
//! developer-command `Save:` or a slot file), and `--save-out` writes one at
//! the end of the run, so a state reached by playing, headless or native,
//! can seed the next run.

use std::path::Path;

use rand_xoshiro::Xoshiro256PlusPlus;

use rebellion_core::ai::AiFaction;
use rebellion_core::dat::Faction;
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::tuning::GameConfig;
use rebellion_core::world::GameWorld;
use rebellion_data::save::{load_file, save_file, SaveState};
use rebellion_data::simulation::SimulationStates;

/// A loaded save as the playtest holds it, beside its world.
pub struct Staged {
    pub states: SimulationStates,
    pub rng: Xoshiro256PlusPlus,
    pub game_config: GameConfig,
    pub ai_faction: AiFaction,
    pub player_is_alliance: bool,
}

/// Read the save at `path` into the playtest's state.
///
/// The playtest's fog is the player's side's. The save's story events are
/// already defined, and its RNG position continues, so `--seed` has no
/// effect on a loaded run.
///
/// # Errors
///
/// Returns the save's read, version or fingerprint error.
pub fn load(path: &Path) -> anyhow::Result<(GameWorld, Staged)> {
    let (_, save) = load_file(path, 0)?;
    let SaveState {
        world,
        clock,
        manufacturing,
        missions,
        events,
        ai,
        movement,
        fog_alliance,
        fog_empire,
        player_is_alliance,
        blockade,
        uprising,
        research,
        jedi,
        victory,
        betrayal,
        economy,
        sim_rng,
        ai2,
        repair,
        game_config,
        campaign_config,
        troop_transport,
        deliveries,
        player_agent: _,
        stockpiles,
    } = save;
    let ai_faction = ai.faction.unwrap_or(AiFaction::Empire);
    let fog = if player_is_alliance {
        fog_alliance
    } else {
        fog_empire
    };
    let staged = Staged {
        states: SimulationStates {
            clock,
            manufacturing,
            missions,
            events,
            ai,
            ai2,
            movement,
            fog,
            blockade,
            uprising,
            research,
            jedi,
            victory,
            betrayal,
            economy,
            repair,
            troop_transport,
            deliveries,
            stockpiles,
            campaign_config,
        },
        rng: sim_rng,
        game_config,
        ai_faction,
        player_is_alliance,
    };
    Ok((world, staged))
}

/// Write the run's state to `path` in the game's save format and return its
/// fingerprint. The side the playtest does not view keeps a freshly seeded
/// fog, and the player's agent is the default one.
///
/// # Errors
///
/// Returns the save's serialization or write error.
pub fn save_out(
    path: &Path,
    world: &GameWorld,
    states: &SimulationStates,
    rng: &Xoshiro256PlusPlus,
    game_config: &GameConfig,
    player_is_alliance: bool,
) -> anyhow::Result<String> {
    let other = |faction| {
        let mut fog = FogState::new(faction);
        FogSystem::seed(&mut fog, world);
        fog
    };
    let (fog_alliance, fog_empire) = if states.fog.faction == Faction::Alliance {
        (states.fog.clone(), other(Faction::Empire))
    } else {
        (other(Faction::Alliance), states.fog.clone())
    };
    let state = SaveState {
        world: world.clone(),
        clock: states.clock.clone(),
        manufacturing: states.manufacturing.clone(),
        missions: states.missions.clone(),
        events: states.events.clone(),
        ai: states.ai.clone(),
        movement: states.movement.clone(),
        fog_alliance,
        fog_empire,
        player_is_alliance,
        blockade: states.blockade.clone(),
        uprising: states.uprising.clone(),
        research: states.research.clone(),
        jedi: states.jedi.clone(),
        victory: states.victory.clone(),
        betrayal: states.betrayal.clone(),
        economy: states.economy.clone(),
        sim_rng: rng.clone(),
        ai2: states.ai2.clone(),
        repair: states.repair.clone(),
        game_config: game_config.clone(),
        campaign_config: states.campaign_config,
        troop_transport: states.troop_transport.clone(),
        deliveries: states.deliveries.clone(),
        player_agent: rebellion_core::agent_automation::PlayerAgent::default(),
        stockpiles: states.stockpiles.clone(),
    };
    let name = format!("Playtest day {}", states.clock.tick);
    let fingerprint = save_file(path, &name, &state, &[])?;
    Ok(fingerprint.to_string())
}
