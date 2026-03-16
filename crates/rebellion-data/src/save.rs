//! Save / load for the full game state.
//!
//! # Format
//!
//! Binary `bincode` encoding. A save file is:
//!
//! ```text
//! [magic: 8 bytes "OPENREB\0"]
//! [version: u32, little-endian]
//! [save_name: length-prefixed UTF-8 string]
//! [timestamp_secs: u64 Unix seconds]
//! [bincode-encoded SaveState]
//! ```
//!
//! `SaveState` wraps all mutable simulation state. `GameWorld` (the entity
//! arenas) is included because fleet positions, popularity, etc. change during
//! play. Slotmap keys are stable across a session but are NOT portable across
//! different `load_game_data` calls — the save includes world state, not DAT
//! data. Loading always re-loads DAT files first, then applies the save on top.
//!
//! # WASM
//!
//! File IO is gated with `#[cfg(not(target_arch = "wasm32"))]`. On WASM the
//! module still compiles (for type-checking) but all IO functions are stubs
//! that return `Err("not supported on WASM")`.
//!
//! # Save slots
//!
//! Saves live at `<saves_dir>/<slot_index>.reb`. The UI manages up to
//! `MAX_SAVE_SLOTS` named slots. `list_saves()` returns metadata for all
//! occupied slots.

use serde::{Deserialize, Serialize};

use rebellion_core::ai::AIState;
use rebellion_core::betrayal::BetrayalState;
use rebellion_core::blockade::BlockadeState;
use rebellion_core::death_star::DeathStarState;
use rebellion_core::events::EventState;
use rebellion_core::fog::FogState;
use rebellion_core::jedi::JediState;
use rebellion_core::manufacturing::ManufacturingState;
use rebellion_core::missions::MissionState;
use rebellion_core::movement::MovementState;
use rebellion_core::research::ResearchState;
use rebellion_core::tick::GameClock;
use rebellion_core::uprising::UprisingState;
use rebellion_core::victory::VictoryState;
use rebellion_core::world::GameWorld;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Binary magic at the start of every save file. 8 bytes.
pub const SAVE_MAGIC: &[u8; 8] = b"OPENREB\0";

/// Current save format version. Increment when `SaveState` layout changes.
pub const SAVE_VERSION: u32 = 3;

/// Maximum number of named save slots.
pub const MAX_SAVE_SLOTS: usize = 10;

// ---------------------------------------------------------------------------
// SaveState — the full serializable snapshot
// ---------------------------------------------------------------------------

/// Complete serializable game state.
///
/// All fields are `#[serde(skip)]`-free — every field must survive a round-trip.
/// The `gnprtb` and `mission_tables` inside `world` are included; on load the
/// caller should re-populate them from DAT files if the saved values are empty
/// (forward-compat with saves from before those fields existed).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveState {
    pub world: GameWorld,
    pub clock: GameClock,
    pub manufacturing: ManufacturingState,
    pub missions: MissionState,
    pub events: EventState,
    pub ai: AIState,
    pub movement: MovementState,
    pub fog_alliance: FogState,
    pub fog_empire: FogState,
    /// Faction the player chose at game start (false = Empire).
    pub player_is_alliance: bool,
    // ── v0.4.1: new simulation states ────────────────────────────────────
    pub blockade: BlockadeState,
    pub uprising: UprisingState,
    pub death_star: DeathStarState,
    pub research: ResearchState,
    pub jedi: JediState,
    pub victory: VictoryState,
    pub betrayal: BetrayalState,
}

// ---------------------------------------------------------------------------
// SaveMeta — slot metadata (no heavy world data)
// ---------------------------------------------------------------------------

/// Lightweight metadata for a save slot — used by the save/load UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMeta {
    /// Slot index (0..MAX_SAVE_SLOTS).
    pub slot: usize,
    /// Human-readable name provided by the player.
    pub name: String,
    /// Unix timestamp (seconds since epoch) of when the save was written.
    pub timestamp_secs: u64,
    /// Game tick at save time.
    pub game_tick: u64,
}

// ---------------------------------------------------------------------------
// IO functions (native only)
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::*;
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};

    use anyhow::Context;

    /// Default save directory: `<exe_dir>/saves/`.
    pub fn default_saves_dir() -> PathBuf {
        // Prefer a directory relative to the executable.  Fall back to the
        // working directory if the executable path is unavailable.
        let base = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("saves")
    }

    /// Path for save slot `slot` under `saves_dir`.
    pub fn slot_path(saves_dir: &Path, slot: usize) -> PathBuf {
        saves_dir.join(format!("{}.reb", slot))
    }

    /// Write `state` to slot `slot` in `saves_dir`.
    ///
    /// Creates `saves_dir` if it does not exist.
    pub fn save_slot(
        saves_dir: &Path,
        slot: usize,
        name: &str,
        state: &SaveState,
    ) -> anyhow::Result<()> {
        std::fs::create_dir_all(saves_dir)
            .with_context(|| format!("creating saves directory {}", saves_dir.display()))?;

        let path = slot_path(saves_dir, slot);
        let mut file = std::fs::File::create(&path)
            .with_context(|| format!("creating save file {}", path.display()))?;

        // ── Header ──────────────────────────────────────────────────────────
        file.write_all(SAVE_MAGIC)
            .context("writing save magic")?;
        file.write_all(&SAVE_VERSION.to_le_bytes())
            .context("writing save version")?;

        // Name: u32 length + UTF-8 bytes
        let name_bytes = name.as_bytes();
        file.write_all(&(name_bytes.len() as u32).to_le_bytes())
            .context("writing name length")?;
        file.write_all(name_bytes)
            .context("writing name")?;

        // Timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        file.write_all(&timestamp.to_le_bytes())
            .context("writing timestamp")?;

        // ── Body ────────────────────────────────────────────────────────────
        let encoded = bincode::serialize(state)
            .context("serializing save state")?;
        file.write_all(&encoded)
            .context("writing save body")?;

        Ok(())
    }

    /// Load `SaveState` from slot `slot` in `saves_dir`.
    pub fn load_slot(saves_dir: &Path, slot: usize) -> anyhow::Result<(SaveMeta, SaveState)> {
        let path = slot_path(saves_dir, slot);
        let mut file = std::fs::File::open(&path)
            .with_context(|| format!("opening save file {}", path.display()))?;

        // ── Header ──────────────────────────────────────────────────────────
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic).context("reading magic")?;
        anyhow::ensure!(
            &magic == SAVE_MAGIC,
            "not a valid save file: bad magic in {}",
            path.display()
        );

        let mut version_buf = [0u8; 4];
        file.read_exact(&mut version_buf).context("reading version")?;
        let version = u32::from_le_bytes(version_buf);
        anyhow::ensure!(
            version == SAVE_VERSION,
            "incompatible save version {} (expected {})",
            version, SAVE_VERSION
        );

        let mut name_len_buf = [0u8; 4];
        file.read_exact(&mut name_len_buf).context("reading name length")?;
        let name_len = u32::from_le_bytes(name_len_buf) as usize;
        let mut name_bytes = vec![0u8; name_len];
        file.read_exact(&mut name_bytes).context("reading name")?;
        let name = String::from_utf8(name_bytes).context("invalid save name encoding")?;

        let mut ts_buf = [0u8; 8];
        file.read_exact(&mut ts_buf).context("reading timestamp")?;
        let timestamp_secs = u64::from_le_bytes(ts_buf);

        // ── Body ────────────────────────────────────────────────────────────
        let mut body = Vec::new();
        file.read_to_end(&mut body).context("reading save body")?;
        let state: SaveState = bincode::deserialize(&body).context("deserializing save state")?;

        let meta = SaveMeta {
            slot,
            name,
            timestamp_secs,
            game_tick: state.clock.tick,
        };

        Ok((meta, state))
    }

    /// Return metadata for all occupied save slots in `saves_dir`.
    ///
    /// Slots without a file are silently skipped. Corrupt files are reported
    /// as `Err` entries in the returned vector.
    pub fn list_saves(saves_dir: &Path) -> Vec<anyhow::Result<SaveMeta>> {
        (0..MAX_SAVE_SLOTS)
            .filter_map(|slot| {
                let path = slot_path(saves_dir, slot);
                if path.exists() {
                    Some(load_slot(saves_dir, slot).map(|(meta, _)| meta))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Delete a save slot file. No-op if the slot doesn't exist.
    pub fn delete_slot(saves_dir: &Path, slot: usize) -> anyhow::Result<()> {
        let path = slot_path(saves_dir, slot);
        if path.exists() {
            std::fs::remove_file(&path)
                .with_context(|| format!("deleting save file {}", path.display()))?;
        }
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::{default_saves_dir, delete_slot, list_saves, load_slot, save_slot, slot_path};

// ---------------------------------------------------------------------------
// WASM stubs
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
pub mod wasm_stubs {
    use super::*;
    use std::path::{Path, PathBuf};

    pub fn default_saves_dir() -> PathBuf {
        PathBuf::from("saves")
    }

    pub fn save_slot(
        _saves_dir: &Path,
        _slot: usize,
        _name: &str,
        _state: &SaveState,
    ) -> anyhow::Result<()> {
        anyhow::bail!("save/load is not supported in the browser build")
    }

    pub fn load_slot(
        _saves_dir: &Path,
        _slot: usize,
    ) -> anyhow::Result<(SaveMeta, SaveState)> {
        anyhow::bail!("save/load is not supported in the browser build")
    }

    pub fn list_saves(_saves_dir: &Path) -> Vec<anyhow::Result<SaveMeta>> {
        Vec::new()
    }

    pub fn delete_slot(_saves_dir: &Path, _slot: usize) -> anyhow::Result<()> {
        anyhow::bail!("save/load is not supported in the browser build")
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_stubs::{default_saves_dir, delete_slot, list_saves, load_slot, save_slot};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use rebellion_core::ai::{AIState, AiFaction};
    use rebellion_core::dat::Faction;

    fn minimal_save_state() -> SaveState {
        // Create a minimal world with two systems for VictoryState
        let mut world = GameWorld::default();
        let sector_key = world.sectors.insert(rebellion_core::world::Sector {
            dat_id: rebellion_core::ids::DatId::new(0x9200_0000),
            name: "Test".into(),
            group: rebellion_core::dat::SectorGroup::Core,
            x: 0, y: 0, systems: vec![],
        });
        let sys_a = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(0x9000_0000),
            name: "A".into(), sector: sector_key, x: 0, y: 0,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: true,
            is_destroyed: false, controlling_faction: Some(Faction::Alliance),
        });
        let sys_b = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(0x9000_0001),
            name: "B".into(), sector: sector_key, x: 100, y: 100,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: true,
            is_destroyed: false, controlling_faction: Some(Faction::Empire),
        });
        SaveState {
            world,
            clock: GameClock::default(),
            manufacturing: ManufacturingState::new(),
            missions: MissionState::new(),
            events: EventState::new(),
            ai: AIState::new(AiFaction::Empire),
            movement: MovementState::new(),
            fog_alliance: FogState::new(Faction::Alliance),
            fog_empire: FogState::new(Faction::Empire),
            player_is_alliance: false,
            blockade: BlockadeState::new(),
            uprising: UprisingState::new(),
            death_star: DeathStarState::default(),
            research: ResearchState::new(),
            jedi: JediState::new(),
            victory: rebellion_core::victory::VictoryState::new(sys_a, sys_b),
            betrayal: BetrayalState::new(),
        }
    }

    /// Create a unique temp directory scoped to this test.
    fn tmp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir()
            .join("open_rebellion_save_tests")
            .join(name);
        std::fs::create_dir_all(&dir).expect("create tmp dir");
        dir
    }

    #[test]
    fn round_trip_save_load() {
        let saves_dir = tmp_dir("round_trip");

        let state = minimal_save_state();
        save_slot(&saves_dir, 0, "Test Save", &state)
            .expect("save should succeed");

        let (meta, loaded) = load_slot(&saves_dir, 0)
            .expect("load should succeed");

        assert_eq!(meta.slot, 0);
        assert_eq!(meta.name, "Test Save");
        assert_eq!(meta.game_tick, loaded.clock.tick);
    }

    #[test]
    fn list_saves_empty_dir() {
        let saves_dir = tmp_dir("list_empty");
        let metas = list_saves(&saves_dir);
        assert!(metas.is_empty());
    }

    #[test]
    fn list_saves_after_write() {
        let saves_dir = tmp_dir("list_after_write");

        let state = minimal_save_state();
        save_slot(&saves_dir, 2, "Slot 2", &state).unwrap();
        save_slot(&saves_dir, 5, "Slot 5", &state).unwrap();

        let metas: Vec<_> = list_saves(&saves_dir)
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();

        assert_eq!(metas.len(), 2);
        assert!(metas.iter().any(|m| m.slot == 2 && m.name == "Slot 2"));
        assert!(metas.iter().any(|m| m.slot == 5 && m.name == "Slot 5"));
    }

    #[test]
    fn delete_slot_removes_file() {
        let saves_dir = tmp_dir("delete_slot");

        let state = minimal_save_state();
        save_slot(&saves_dir, 1, "To Delete", &state).unwrap();
        assert!(slot_path(&saves_dir, 1).exists());

        delete_slot(&saves_dir, 1).unwrap();
        assert!(!slot_path(&saves_dir, 1).exists());
    }
}
