//! War Room player-facing UI panels.
//!
//! Each panel is a standalone function that accepts the egui context, read-only
//! world data, and mutable panel-local state.  Every panel returns
//! `Option<PanelAction>` — the caller applies the action to game state rather
//! than letting the panel borrow mutable world references, which would conflict
//! with egui's `FnMut` closure requirements.
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

pub mod bombardment;
pub mod death_star;
pub mod fleets;
pub mod game_setup;
pub mod jedi;
pub mod loyalty;
pub mod missions;
pub mod mod_manager;
pub mod officers;
pub mod research;
pub mod save_load;

#[cfg(any(debug_assertions, not(target_arch = "wasm32")))]
pub mod command_palette;

pub use fleets::{draw_fleets, FleetsState};
pub use missions::draw_missions;
pub use mod_manager::{draw_mod_manager, ModInfo, ModManagerAction, ModManagerState};
pub use officers::{draw_officers, OfficersState};
pub use save_load::{draw_save_load, SaveLoadPanelState, SaveSlotInfo};

use rebellion_core::fleet_join::FleetMover;
use rebellion_core::ids::{CharacterKey, FleetKey, SystemKey, TroopKey};
use rebellion_core::manufacturing::BuildableKind;
use rebellion_core::missions::{MissionFaction, MissionKind, MissionMember};
use rebellion_core::research::TechType;
use rebellion_core::troop_transport::RegimentTarget;

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
    AssignCharacterToFleet {
        character: CharacterKey,
        fleet: FleetKey,
    },
    /// Remove a character from a fleet.
    RemoveCharacterFromFleet {
        character: CharacterKey,
        fleet: FleetKey,
    },
    /// A fleet's or capital ships' Move (`0x201`) released on a Fleet window
    /// (`fleet_join::join_fleet`).
    JoinFleet { mover: FleetMover, target: FleetKey },
    /// Destination (`0x214`): `system`'s production areas, or the one
    /// given, deliver to `destination` (`ManufacturingState::set_destination`).
    SetDestination {
        system: SystemKey,
        area: Option<rebellion_core::manufacturing::ProductionArea>,
        destination: SystemKey,
    },
    /// Rename (`0x203`) issued from its edit: `FUN_004f6e60` sets a fleet's
    /// or, with `ship`, one of its capital ships' name.
    Rename {
        fleet: FleetKey,
        ship: Option<usize>,
        name: String,
    },
    /// A capital ship's Create Fleet (`0x270`, `fleet_join::create_fleet`).
    CreateFleet {
        fleet: FleetKey,
        ships: Vec<usize>,
        roster: u64,
    },
    /// Capital ships' Move (`0x201`) released on a system
    /// (`fleet_join::move_ships_to_system`).
    MoveShips {
        fleet: FleetKey,
        ships: Vec<usize>,
        roster: u64,
        system: SystemKey,
    },
    /// Dispatch one player-controlled fleet to a selected destination.
    DispatchFleet {
        fleet: FleetKey,
        destination: SystemKey,
        troops: Vec<TroopKey>,
    },
    /// A regiment's Move (`0x201`) released on a system or fleet
    /// (`TroopTransportState::move_regiment`).
    MoveRegiment {
        troop: TroopKey,
        target: RegimentTarget,
    },

    // ── Manufacturing ─────────────────────────────────────────────────────────
    /// Stop (`0x213`): `system`'s `area` drops every unit it was building
    /// (`ManufacturingState::stop`).
    StopProduction {
        system: SystemKey,
        area: rebellion_core::manufacturing::ProductionArea,
    },
    /// Build Selection's Confirm (`FUN_00438980`): `count` units of `kind`
    /// replace what `system`'s `area` was building.
    BuildProduction {
        system: SystemKey,
        area: rebellion_core::manufacturing::ProductionArea,
        kind: BuildableKind,
        count: u32,
    },

    // ── Missions ──────────────────────────────────────────────────────────────
    /// Dispatch a mission ordered on day `tick` with its agents and decoys
    /// (the dialog's "Begin Mission", `FUN_0046c3c0` case `0x66`).
    DispatchMission {
        kind: MissionKind,
        faction: MissionFaction,
        team: Vec<MissionMember>,
        decoys: Vec<MissionMember>,
        target: SystemKey,
        target_character: Option<CharacterKey>,
        tick: u64,
    },
    /// Cancel a mission that is currently in progress.
    CancelMission(u64),

    // ── Save / Load ───────────────────────────────────────────────────────────
    /// Open (toggle) the save/load panel.
    OpenSaveLoad,
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
    StopJediTraining { character: CharacterKey },

    // ── Bombardment ──────────────────────────────────────────────────
    /// Order orbital bombardment from a fleet against its current system.
    OrderBombardment { fleet: FleetKey, system: SystemKey },

    // ── Death Star ───────────────────────────────────────────────────
    /// Fire the Death Star superlaser at a system.
    FireDeathStar { system: SystemKey },
    /// Move the Death Star fleet to a target system.
    MoveDeathStar { system: SystemKey },

    // ── Play-testing (command palette) ────────────────────────────────
    /// Advance simulation by N ticks immediately.
    AdvanceTicks(u64),
    /// Set one of the original Game Speed menu's choices.
    SetGameSpeed(rebellion_core::tick::GameSpeed),
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
