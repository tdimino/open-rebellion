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
use rebellion_data::save::{load_file, save_file, LoadedSimulation, SaveState};
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
    let LoadedSimulation {
        world,
        states,
        inactive_fog: _,
        player_is_alliance,
        sim_rng,
        game_config,
        player_agent: _,
    } = save.into_simulation();
    let ai_faction = states.ai.faction.unwrap_or(AiFaction::Empire);
    let staged = Staged {
        states,
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
    let player_faction = Faction::of_alliance(player_is_alliance);
    debug_assert_eq!(states.fog.faction, player_faction);
    let inactive_fog = other(Faction::of_alliance(!player_is_alliance));
    let state = SaveState::from_simulation(
        world.clone(),
        states.clone(),
        inactive_fog,
        player_is_alliance,
        rng.clone(),
        game_config.clone(),
        rebellion_core::agent_automation::PlayerAgent::default(),
    );
    let name = format!("Playtest day {}", states.clock.tick);
    let fingerprint = save_file(path, &name, &state, &[])?;
    Ok(fingerprint.to_string())
}
