//! War Room player-facing UI panels.
//!
//! Each panel is a standalone function that accepts the egui context, read-only
//! world data, and mutable panel-local state.  Every panel returns
//! `Option<PanelAction>` — the caller applies the action to game state rather
//! than letting the panel borrow mutable world references, which would conflict
//! with egui's FnMut closure requirements.
//!
//! # Integration
//!
//! Call panel draw functions inside the `egui_macroquad::ui` closure:
//!
//! ```ignore
//! egui_macroquad::ui(|ctx| {
//!     if let Some(action) = panels::draw_officers(ctx, world, &mut officer_state) {
//!         apply_officer_action(&mut world, &mut missions, action);
//!     }
//! });
//! egui_macroquad::draw();
//! ```

pub mod fleets;
pub mod game_setup;
pub mod jedi;
pub mod manufacturing;
pub mod missions;
pub mod mod_manager;
pub mod officers;
pub mod research;
pub mod save_load;

#[cfg(debug_assertions)]
pub mod command_palette;

pub use fleets::{draw_fleets, FleetsState};
pub use manufacturing::{draw_manufacturing, ManufacturingPanelState};
pub use missions::{draw_missions, MissionsPanelState};
pub use mod_manager::{draw_mod_manager, ModInfo, ModManagerAction, ModManagerState};
pub use officers::{draw_officers, OfficersState};
pub use save_load::{draw_save_load, SaveLoadPanelState, SaveSlotInfo};

use rebellion_core::ids::{CharacterKey, FleetKey, SystemKey};
use rebellion_core::manufacturing::BuildableKind;
use rebellion_core::missions::{MissionFaction, MissionKind};
use rebellion_core::research::TechType;

/// Player-initiated actions returned by War Room panels.
///
/// The caller applies these to `GameWorld`, `ManufacturingState`, and
/// `MissionState` rather than the panels borrowing mutable world refs.
#[derive(Debug, Clone)]
pub enum PanelAction {
    // ── Faction Selection ─────────────────────────────────────────────────────
    /// Player has chosen a starting faction.  Gates all other panels.
    SelectFaction(MissionFaction),

    // ── Officers ──────────────────────────────────────────────────────────────
    /// Show detailed stats for the selected character (sets panel focus).
    FocusCharacter(CharacterKey),

    // ── Fleets ────────────────────────────────────────────────────────────────
    /// Focus the galaxy map on the system where a fleet is located.
    FocusFleetSystem(SystemKey),
    /// Assign a character to a fleet as commander.
    AssignCharacterToFleet { character: CharacterKey, fleet: FleetKey },

    // ── Manufacturing ─────────────────────────────────────────────────────────
    /// Add a buildable to the production queue at a system.
    Enqueue { system: SystemKey, kind: BuildableKind, cost: u32, ticks: u32 },
    /// Cancel the queue item at `index` in a system's production queue.
    CancelQueueItem { system: SystemKey, index: usize },
    /// Move queue item at `index` to the front (prioritize).
    PrioritizeQueueItem { system: SystemKey, index: usize },

    // ── Missions ──────────────────────────────────────────────────────────────
    /// Dispatch a mission.  `duration_roll` is a pre-supplied [0,1) random value.
    DispatchMission {
        kind: MissionKind,
        faction: MissionFaction,
        character: CharacterKey,
        target: SystemKey,
        duration_roll: f64,
    },
    /// Cancel a mission that is currently in progress.
    CancelMission(u64),

    // ── Save / Load ───────────────────────────────────────────────────────────
    /// Player confirmed a save to the given slot with the given name.
    SaveGame { slot: usize, name: String },
    /// Player confirmed a load from the given slot.
    LoadGame { slot: usize },
    /// Player deleted the save in the given slot.
    DeleteSave { slot: usize },
    /// Player closed the save/load panel without taking an action.
    CloseSaveLoadPanel,

    // ── Mod Manager ─────────────────────────────────────────────────────
    /// Open/close the mod manager panel.
    OpenModManager,
    /// Toggle a mod's enabled state.
    ToggleMod { name: String },
    /// Reload all mods from disk.
    ReloadMods,

    // ── Research ──────────────────────────────────────────────────────────
    /// Assign a character to research a tech tree.
    DispatchResearch {
        character: CharacterKey,
        tech_type: TechType,
        faction: MissionFaction,
    },
    /// Cancel an active research project.
    CancelResearch {
        tech_type: TechType,
        faction: MissionFaction,
    },

    // ── Jedi Training ──────────────────────────────────────────────────────
    /// Start Force training for a character.
    StartJediTraining {
        character: CharacterKey,
        faction: MissionFaction,
    },
    /// Stop Force training for a character.
    StopJediTraining {
        character: CharacterKey,
    },

    // ── Context Menu Actions ───────────────────────────────────────────
    /// Open mission panel pre-targeted to a system with a specific mission kind.
    OpenMissionTo {
        target: SystemKey,
        kind: MissionKind,
        faction: MissionFaction,
    },
    /// Start fleet movement selection — player picks which fleet to move.
    InitiateFleetMove {
        destination: SystemKey,
    },

    // ── Play-testing (command palette) ────────────────────────────────
    /// Advance simulation by N ticks immediately.
    AdvanceTicks(u64),
    /// Set game speed (0=paused, 1=normal, 2=fast, 4=faster).
    SetGameSpeed(u32),
    /// Toggle AI control for both factions.
    ToggleDualAI,
    /// Immediately evaluate victory conditions.
    ForceVictoryCheck,
    /// Remove fog of war from all systems.
    RevealAllFog,
    /// Export message log to file.
    ExportGameLog,
    /// Display current game statistics overlay.
    ShowGameStats,
    /// List all currently active missions.
    ListActiveMissions,
    /// List all fleet positions and compositions.
    ListActiveFleets,
    /// Show count of triggered events.
    ShowEventCount,
}
