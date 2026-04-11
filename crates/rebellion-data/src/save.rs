//! Save / load for the full game state.
//!
//! # Format (v8)
//!
//! Binary `bincode` encoding. A save file is:
//!
//! ```text
//! [magic: 8 bytes "OPENREB\0"]
//! [version: u32, little-endian]
//! [save_name: length-prefixed UTF-8 string]
//! [timestamp_secs: u64 Unix seconds]
//! [mod_count: u32]                          // v4+
//! for each mod:
//!   [mod_name: length-prefixed UTF-8]       // v4+
//!   [mod_version: length-prefixed UTF-8]    // v4+
//! [mod_hash: u64]                           // v4+
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
use rebellion_core::economy::EconomyState;
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
use rebellion_core::world::{ControlKind, GameWorld};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Binary magic at the start of every save file. 8 bytes.
pub const SAVE_MAGIC: &[u8; 8] = b"OPENREB\0";

/// Current save format version. Increment when `SaveState` layout changes.
///
/// v8: Character gained `heritage_known` + `is_killed`, SaveState gained `economy`.
pub const SAVE_VERSION: u32 = 8;

/// Minimum save version we can migrate from.
const MIN_MIGRATABLE_VERSION: u32 = 3;

/// Maximum number of named save slots.
pub const MAX_SAVE_SLOTS: usize = 10;

// ---------------------------------------------------------------------------
// Mod hash
// ---------------------------------------------------------------------------

/// Compute a deterministic hash from sorted (name, version) pairs.
///
/// Uses FNV-1a (64-bit). The mod list is sorted before hashing so that
/// insertion order does not affect the result.
pub fn compute_mod_hash(mods: &[(String, String)]) -> u64 {
    let mut sorted = mods.to_vec();
    sorted.sort();
    let mut hash: u64 = 0xcbf29ce484222325; // FNV-1a offset basis
    for (name, version) in &sorted {
        for byte in name.bytes().chain(b":".iter().copied()).chain(version.bytes()) {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3); // FNV-1a prime
        }
        hash ^= 0xff; // separator between mod entries
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

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
    // ── v8: economy state (closes incident re-fire on reload bug) ────────
    pub economy: EconomyState,
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
    /// Names of mods that were active when the save was written (v4+).
    pub mod_names: Vec<String>,
    /// Deterministic hash of the sorted (name, version) mod list (v4+).
    pub mod_hash: u64,
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
    /// `active_mods` is a list of `(name, version)` pairs for currently loaded
    /// mods. Pass an empty slice when no mods are active.
    ///
    /// Creates `saves_dir` if it does not exist.
    pub fn save_slot(
        saves_dir: &Path,
        slot: usize,
        name: &str,
        state: &SaveState,
        active_mods: &[(String, String)],
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

        // Mod metadata (v4+)
        file.write_all(&(active_mods.len() as u32).to_le_bytes())
            .context("writing mod count")?;
        for (mod_name, mod_version) in active_mods {
            let nb = mod_name.as_bytes();
            file.write_all(&(nb.len() as u32).to_le_bytes())
                .context("writing mod name length")?;
            file.write_all(nb)
                .context("writing mod name")?;
            let vb = mod_version.as_bytes();
            file.write_all(&(vb.len() as u32).to_le_bytes())
                .context("writing mod version length")?;
            file.write_all(vb)
                .context("writing mod version")?;
        }
        let mod_hash = compute_mod_hash(active_mods);
        file.write_all(&mod_hash.to_le_bytes())
            .context("writing mod hash")?;

        // ── Body ────────────────────────────────────────────────────────────
        let encoded = bincode::serialize(state)
            .context("serializing save state")?;
        file.write_all(&encoded)
            .context("writing save body")?;

        Ok(())
    }

    /// Convenience wrapper: save with no active mods.
    pub fn save_slot_no_mods(
        saves_dir: &Path,
        slot: usize,
        name: &str,
        state: &SaveState,
    ) -> anyhow::Result<()> {
        save_slot(saves_dir, slot, name, state, &[])
    }

    /// Load `SaveState` from slot `slot` in `saves_dir`.
    ///
    /// Supports migrating saves from older versions (minimum v3). Saves from
    /// future versions are rejected.
    pub fn load_slot(saves_dir: &Path, slot: usize) -> anyhow::Result<(SaveMeta, SaveState)> {
        let path = slot_path(saves_dir, slot);
        let mut file = std::fs::File::open(&path)
            .with_context(|| format!("opening save file {}", path.display()))?;

        // ── Header (common) ─────────────────────────────────────────────────
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

        // ── Version gate ────────────────────────────────────────────────────
        if version > SAVE_VERSION {
            anyhow::bail!(
                "save version {} is from a newer build (this build supports up to {})",
                version, SAVE_VERSION
            );
        }
        if version < MIN_MIGRATABLE_VERSION {
            anyhow::bail!(
                "save version {} is too old to migrate (minimum supported: {})",
                version, MIN_MIGRATABLE_VERSION
            );
        }

        // ── Name + timestamp (present in all versions) ──────────────────────
        let mut name_len_buf = [0u8; 4];
        file.read_exact(&mut name_len_buf).context("reading name length")?;
        let name_len = u32::from_le_bytes(name_len_buf) as usize;
        let mut name_bytes = vec![0u8; name_len];
        file.read_exact(&mut name_bytes).context("reading name")?;
        let name = String::from_utf8(name_bytes).context("invalid save name encoding")?;

        let mut ts_buf = [0u8; 8];
        file.read_exact(&mut ts_buf).context("reading timestamp")?;
        let timestamp_secs = u64::from_le_bytes(ts_buf);

        // ── Mod metadata (v4+) ──────────────────────────────────────────────
        let (mod_names, mod_hash) = if version >= 4 {
            let mut count_buf = [0u8; 4];
            file.read_exact(&mut count_buf).context("reading mod count")?;
            let mod_count = u32::from_le_bytes(count_buf) as usize;

            let mut names = Vec::with_capacity(mod_count);
            for _ in 0..mod_count {
                let mut len_buf = [0u8; 4];
                file.read_exact(&mut len_buf).context("reading mod name length")?;
                let len = u32::from_le_bytes(len_buf) as usize;
                let mut bytes = vec![0u8; len];
                file.read_exact(&mut bytes).context("reading mod name")?;
                let mod_name = String::from_utf8(bytes).context("invalid mod name encoding")?;

                file.read_exact(&mut len_buf).context("reading mod version length")?;
                let vlen = u32::from_le_bytes(len_buf) as usize;
                let mut vbytes = vec![0u8; vlen];
                file.read_exact(&mut vbytes).context("reading mod version")?;
                // We store name only in meta; version is folded into the hash.
                let _mod_version = String::from_utf8(vbytes)
                    .context("invalid mod version encoding")?;

                names.push(mod_name);
            }

            let mut hash_buf = [0u8; 8];
            file.read_exact(&mut hash_buf).context("reading mod hash")?;
            let hash = u64::from_le_bytes(hash_buf);

            (names, hash)
        } else {
            // v3 saves have no mod metadata — default to empty
            (Vec::new(), compute_mod_hash(&[]))
        };

        // ── Body ────────────────────────────────────────────────────────────
        let mut body = Vec::new();
        file.read_to_end(&mut body).context("reading save body")?;

        let state: SaveState = match version {
            SAVE_VERSION => {
                bincode::deserialize(&body).context("deserializing save state")?
            }
            7 => {
                anyhow::bail!(
                    "save version 7 is incompatible with this build (Character gained `heritage_known`; SaveState gained `economy`). \
                     Please start a new game."
                );
            }
            6 => {
                anyhow::bail!(
                    "save version 6 is incompatible with this build (Fleet.capital_ships changed from ShipEntry to ShipInstance). \
                     Please start a new game."
                );
            }
            5 => {
                anyhow::bail!(
                    "save version 5 is incompatible with this build (espionage_rating + facility type fields added). \
                     Please start a new game."
                );
            }
            4 => {
                anyhow::bail!(
                    "save version 4 is incompatible with this build (System seeding fields changed). \
                     Please start a new game."
                );
            }
            3 => {
                // v3 saves used a different Character struct layout (no captivity fields)
                // and bincode is positional — #[serde(default)] is inoperative.
                // True migration would require a SaveStateV3 struct. Since no v3 saves
                // are in the wild (v3 existed only during development), reject cleanly.
                anyhow::bail!(
                    "save version 3 is incompatible with this build (Character struct changed). \
                     Please start a new game."
                );
            }
            _ => unreachable!("version range already validated above"),
        };

        let meta = SaveMeta {
            slot,
            name,
            timestamp_secs,
            game_tick: state.clock.tick,
            mod_names,
            mod_hash,
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
pub use native::{
    default_saves_dir, delete_slot, list_saves, load_slot, save_slot, save_slot_no_mods, slot_path,
};

// ---------------------------------------------------------------------------
// WASM stubs
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
pub mod wasm_impl {
    use super::*;
    use std::path::{Path, PathBuf};

    /// Base64 encode (standard alphabet, no padding).
    fn b64_encode(data: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
        for chunk in data.chunks(3) {
            let b0 = chunk[0] as u32;
            let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
            let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
            let triple = (b0 << 16) | (b1 << 8) | b2;
            out.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
            out.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
            if chunk.len() > 1 { out.push(CHARS[((triple >> 6) & 0x3F) as usize] as char); }
            if chunk.len() > 2 { out.push(CHARS[(triple & 0x3F) as usize] as char); }
        }
        out
    }

    /// Base64 decode (standard alphabet, tolerates missing padding).
    fn b64_decode(s: &str) -> anyhow::Result<Vec<u8>> {
        fn val(c: u8) -> anyhow::Result<u8> {
            match c {
                b'A'..=b'Z' => Ok(c - b'A'),
                b'a'..=b'z' => Ok(c - b'a' + 26),
                b'0'..=b'9' => Ok(c - b'0' + 52),
                b'+' => Ok(62),
                b'/' => Ok(63),
                b'=' => Ok(0),
                _ => anyhow::bail!("invalid base64 character: {}", c as char),
            }
        }
        let bytes: Vec<u8> = s.bytes().filter(|b| *b != b'\n' && *b != b'\r').collect();
        let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
        for chunk in bytes.chunks(4) {
            if chunk.len() < 2 { break; }
            let a = val(chunk[0])? as u32;
            let b = val(chunk[1])? as u32;
            let c = if chunk.len() > 2 { val(chunk[2])? as u32 } else { 0 };
            let d = if chunk.len() > 3 { val(chunk[3])? as u32 } else { 0 };
            let triple = (a << 18) | (b << 12) | (c << 6) | d;
            out.push((triple >> 16) as u8);
            if chunk.len() > 2 && chunk[2] != b'=' { out.push((triple >> 8) as u8); }
            if chunk.len() > 3 && chunk[3] != b'=' { out.push(triple as u8); }
        }
        Ok(out)
    }

    /// Get the browser's localStorage.
    fn local_storage() -> anyhow::Result<web_sys::Storage> {
        let window = web_sys::window().ok_or_else(|| anyhow::anyhow!("no global window"))?;
        window
            .local_storage()
            .map_err(|_| anyhow::anyhow!("localStorage access denied"))?
            .ok_or_else(|| anyhow::anyhow!("localStorage not available"))
    }

    /// localStorage key for a save slot.
    ///
    /// Prefix is bumped per save format version so that stale browser entries
    /// from older builds get rejected cleanly instead of attempting an
    /// inoperable bincode deserialize.
    fn slot_key(slot: usize) -> String {
        format!("rebellion_save_v8_{}", slot)
    }

    /// localStorage key for save metadata (name + tick).
    ///
    /// Prefix is bumped per save format version (see [`slot_key`]).
    fn meta_key(slot: usize) -> String {
        format!("rebellion_meta_v8_{}", slot)
    }

    pub fn default_saves_dir() -> PathBuf {
        PathBuf::from("saves")
    }

    pub fn save_slot(
        _saves_dir: &Path,
        slot: usize,
        name: &str,
        state: &SaveState,
        _active_mods: &[(String, String)],
    ) -> anyhow::Result<()> {
        let storage = local_storage()?;
        let encoded = bincode::serialize(state)?;
        let b64 = b64_encode(&encoded);

        storage
            .set_item(&slot_key(slot), &b64)
            .map_err(|_| anyhow::anyhow!("localStorage.setItem failed (quota exceeded?)"))?;

        // Store metadata separately (lightweight, for list_saves).
        let meta = format!("{}|{}", name, state.clock.tick);
        storage
            .set_item(&meta_key(slot), &meta)
            .map_err(|_| anyhow::anyhow!("localStorage.setItem failed for metadata"))?;

        Ok(())
    }

    pub fn save_slot_no_mods(
        saves_dir: &Path,
        slot: usize,
        name: &str,
        state: &SaveState,
    ) -> anyhow::Result<()> {
        save_slot(saves_dir, slot, name, state, &[])
    }

    pub fn load_slot(
        _saves_dir: &Path,
        slot: usize,
    ) -> anyhow::Result<(SaveMeta, SaveState)> {
        let storage = local_storage()?;

        let b64 = storage
            .get_item(&slot_key(slot))
            .map_err(|_| anyhow::anyhow!("localStorage.getItem failed"))?
            .ok_or_else(|| anyhow::anyhow!("no save in slot {}", slot))?;

        let bytes = b64_decode(&b64)?;
        let state: SaveState = bincode::deserialize(&bytes)?;

        // Read metadata.
        let meta_str = storage
            .get_item(&meta_key(slot))
            .ok()
            .flatten()
            .unwrap_or_default();
        let (name, _tick) = meta_str.split_once('|').unwrap_or(("Unnamed", "0"));

        let meta = SaveMeta {
            slot,
            name: name.to_string(),
            timestamp_secs: 0, // no reliable clock in WASM
            game_tick: state.clock.tick,
            mod_names: vec![],
            mod_hash: 0,
        };

        Ok((meta, state))
    }

    pub fn list_saves(_saves_dir: &Path) -> Vec<anyhow::Result<SaveMeta>> {
        let storage = match local_storage() {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        (0..MAX_SAVE_SLOTS)
            .filter_map(|slot| {
                // Check if metadata exists for this slot.
                let meta_str = storage.get_item(&meta_key(slot)).ok()??;
                let (name, tick_str) = meta_str.split_once('|').unwrap_or(("Unnamed", "0"));
                let tick: u64 = tick_str.parse().unwrap_or(0);
                Some(Ok(SaveMeta {
                    slot,
                    name: name.to_string(),
                    timestamp_secs: 0,
                    game_tick: tick,
                    mod_names: vec![],
                    mod_hash: 0,
                }))
            })
            .collect()
    }

    pub fn delete_slot(_saves_dir: &Path, slot: usize) -> anyhow::Result<()> {
        let storage = local_storage()?;
        let _ = storage.remove_item(&slot_key(slot));
        let _ = storage.remove_item(&meta_key(slot));
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_impl::{
    default_saves_dir, delete_slot, list_saves, load_slot, save_slot, save_slot_no_mods,
};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use rebellion_core::ai::{AIState, AiFaction};
    use rebellion_core::dat::Faction;
    use std::io::Write;

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
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: true,
            is_destroyed: false, control: ControlKind::Controlled(Faction::Alliance),
            espionage_rating: 0.0,
        });
        let sys_b = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(0x9000_0001),
            name: "B".into(), sector: sector_key, x: 100, y: 100,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5, popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![], ground_units: vec![], special_forces: vec![],
            defense_facilities: vec![], manufacturing_facilities: vec![],
            production_facilities: vec![], is_headquarters: true,
            is_destroyed: false, control: ControlKind::Controlled(Faction::Empire),
            espionage_rating: 0.0,
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
            economy: EconomyState::default(),
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

    /// Write a v3-format save file (no mod metadata in header).
    /// Used as a fixture for migration tests.
    fn write_v3_fixture(path: &std::path::Path, name: &str, state: &SaveState) {
        let mut file = std::fs::File::create(path).expect("create v3 fixture");
        file.write_all(SAVE_MAGIC).unwrap();
        file.write_all(&3u32.to_le_bytes()).unwrap(); // version = 3
        let name_bytes = name.as_bytes();
        file.write_all(&(name_bytes.len() as u32).to_le_bytes()).unwrap();
        file.write_all(name_bytes).unwrap();
        let timestamp: u64 = 1700000000; // fixed timestamp for reproducibility
        file.write_all(&timestamp.to_le_bytes()).unwrap();
        // No mod metadata — that's the v3 format
        let encoded = bincode::serialize(state).expect("serialize v3 body");
        file.write_all(&encoded).unwrap();
    }

    /// Write a save file with an arbitrary version number (for rejection tests).
    fn write_versioned_fixture(path: &std::path::Path, version: u32, name: &str, state: &SaveState) {
        let mut file = std::fs::File::create(path).expect("create versioned fixture");
        file.write_all(SAVE_MAGIC).unwrap();
        file.write_all(&version.to_le_bytes()).unwrap();
        let name_bytes = name.as_bytes();
        file.write_all(&(name_bytes.len() as u32).to_le_bytes()).unwrap();
        file.write_all(name_bytes).unwrap();
        let timestamp: u64 = 1700000000;
        file.write_all(&timestamp.to_le_bytes()).unwrap();
        if version >= 4 {
            file.write_all(&0u32.to_le_bytes()).unwrap(); // empty mod list
            file.write_all(&compute_mod_hash(&[]).to_le_bytes()).unwrap();
        }
        let encoded = bincode::serialize(state).expect("serialize body");
        file.write_all(&encoded).unwrap();
    }

    // ── Existing tests (updated for new save_slot signature) ────────────────

    #[test]
    fn round_trip_save_load() {
        let saves_dir = tmp_dir("round_trip_v5");

        let state = minimal_save_state();
        save_slot(&saves_dir, 0, "Test Save", &state, &[])
            .expect("save should succeed");

        let (meta, loaded) = load_slot(&saves_dir, 0)
            .expect("load should succeed");

        assert_eq!(meta.slot, 0);
        assert_eq!(meta.name, "Test Save");
        assert_eq!(meta.game_tick, loaded.clock.tick);
        assert!(meta.mod_names.is_empty());
    }

    #[test]
    fn list_saves_empty_dir() {
        let saves_dir = tmp_dir("list_empty");
        let metas = list_saves(&saves_dir);
        assert!(metas.is_empty());
    }

    #[test]
    fn list_saves_after_write() {
        let saves_dir = tmp_dir("list_after_write_v5");

        let state = minimal_save_state();
        save_slot(&saves_dir, 2, "Slot 2", &state, &[]).unwrap();
        save_slot(&saves_dir, 5, "Slot 5", &state, &[]).unwrap();

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
        let saves_dir = tmp_dir("delete_slot_v5");

        let state = minimal_save_state();
        save_slot(&saves_dir, 1, "To Delete", &state, &[]).unwrap();
        assert!(slot_path(&saves_dir, 1).exists());

        delete_slot(&saves_dir, 1).unwrap();
        assert!(!slot_path(&saves_dir, 1).exists());
    }

    // ── New tests (Tasks 3–5) ───────────────────────────────────────────────

    #[test]
    fn v3_save_rejected_with_clear_message() {
        let saves_dir = tmp_dir("v3_migration");
        let state = minimal_save_state();
        let path = slot_path(&saves_dir, 0);
        write_v3_fixture(&path, "V3 Save", &state);

        // v3 saves are incompatible (bincode layout changed with captivity fields).
        let result = load_slot(&saves_dir, 0);
        assert!(result.is_err(), "v3 saves should be rejected");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("incompatible") || err_msg.contains("version 3"),
            "error should mention incompatibility: {err_msg}"
        );
    }

    #[test]
    fn v4_or_v5_compatibility_path() {
        let saves_dir = tmp_dir("v4_rejected");
        let state = minimal_save_state();
        let path = slot_path(&saves_dir, 0);
        write_versioned_fixture(&path, 4, "V4 Save", &state);

        let err = load_slot(&saves_dir, 0)
            .expect_err("v4 saves should be rejected after the v5 layout change");
        let msg = err.to_string();
        assert!(
            msg.contains("version 4") && msg.contains("incompatible"),
            "error should explain the v4 rejection path: {msg}"
        );
    }

    #[test]
    fn mod_hash_round_trip() {
        let saves_dir = tmp_dir("mod_hash_rt");
        let state = minimal_save_state();
        let mods = vec![("TestMod".to_string(), "1.0".to_string())];

        save_slot(&saves_dir, 0, "Modded", &state, &mods)
            .expect("save with mods should succeed");

        let (meta, _) = load_slot(&saves_dir, 0)
            .expect("load modded save should succeed");

        assert_eq!(meta.mod_names, vec!["TestMod".to_string()]);
        assert_eq!(meta.mod_hash, compute_mod_hash(&mods));
    }

    #[test]
    fn empty_mod_list_round_trip() {
        let saves_dir = tmp_dir("empty_mods");
        let state = minimal_save_state();

        save_slot(&saves_dir, 0, "No Mods", &state, &[])
            .expect("save with no mods should succeed");

        let (meta, _) = load_slot(&saves_dir, 0)
            .expect("load should succeed");

        assert!(meta.mod_names.is_empty());
        assert_eq!(meta.mod_hash, compute_mod_hash(&[]));
    }

    #[test]
    fn future_version_rejected() {
        let saves_dir = tmp_dir("future_version");
        let state = minimal_save_state();
        let path = slot_path(&saves_dir, 0);
        write_versioned_fixture(&path, 99, "Future", &state);

        let err = load_slot(&saves_dir, 0)
            .expect_err("future version should be rejected");

        let msg = err.to_string();
        assert!(
            msg.contains("newer build"),
            "error should mention 'newer build', got: {msg}"
        );
    }

    #[test]
    fn mod_hash_mismatch_still_loads() {
        let saves_dir = tmp_dir("mod_mismatch");
        let state = minimal_save_state();
        let mods_a = vec![("ModA".to_string(), "1.0".to_string())];
        let mods_b = vec![("ModB".to_string(), "2.0".to_string())];

        save_slot(&saves_dir, 0, "Mods A", &state, &mods_a).unwrap();

        // Load succeeds even though our "current" mods differ.
        let (meta, _) = load_slot(&saves_dir, 0)
            .expect("mismatched mods should still load");

        // The meta records what was saved, not what's current.
        assert_eq!(meta.mod_names, vec!["ModA".to_string()]);
        assert_eq!(meta.mod_hash, compute_mod_hash(&mods_a));
        // The caller can compare meta.mod_hash != compute_mod_hash(&mods_b)
        assert_ne!(meta.mod_hash, compute_mod_hash(&mods_b));
    }

    #[test]
    fn deterministic_hash_order_independent() {
        let mods_forward = vec![
            ("Alpha".to_string(), "1.0".to_string()),
            ("Beta".to_string(), "2.0".to_string()),
        ];
        let mods_reverse = vec![
            ("Beta".to_string(), "2.0".to_string()),
            ("Alpha".to_string(), "1.0".to_string()),
        ];

        assert_eq!(
            compute_mod_hash(&mods_forward),
            compute_mod_hash(&mods_reverse),
            "hash should be order-independent"
        );
    }
}
