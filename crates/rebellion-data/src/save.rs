//! Save / load for the full game state.
//!
//! # Format (`SAVE_VERSION`)
//!
//! Binary `bincode` encoding. A save file is:
//!
//! ```text
//! [magic: 8 bytes "OPENREB\0"]
//! [version: u32, little-endian]
//! [save_name: length-prefixed UTF-8 string]
//! [timestamp_secs: u64 Unix seconds]
//! [mod_count: u32]
//! for each mod:
//!   [mod_name: length-prefixed UTF-8]
//!   [mod_version: length-prefixed UTF-8]
//! [mod_hash: u64]
//! [fingerprint_version: u16]
//! [state_fingerprint: u64]                  // canonical logical state
//! [bincode-encoded SaveState]
//! ```
//!
//! Only the current version loads. The port has no released saves, so a
//! layout change bumps `SAVE_VERSION` and rejects older files instead of
//! migrating them.
//!
//! `SaveState` wraps all mutable simulation state, including the random-number
//! generator and tuning configuration needed to continue deterministically.
//! `GameWorld` (the entity
//! arenas) is included because fleet positions, popularity, etc. change during
//! play. Slotmap keys are stable across a session but are NOT portable across
//! different `load_game_data` calls — the save includes world state, not DAT
//! data. Loading always re-loads DAT files first, then applies the save on top.
//!
//! # WASM
//!
//! File IO is gated with `#[cfg(not(target_arch = "wasm32"))]`. On WASM,
//! saves are stored in browser localStorage through three small imports exposed
//! by the vendored miniquad `gl.js` loader. This keeps the binary compatible
//! with miniquad's raw WASM loader without requiring wasm-bindgen glue.
//!
//! # Save slots
//!
//! Saves live at `<saves_dir>/<slot_index>.reb`. The UI manages up to
//! `MAX_SAVE_SLOTS` named slots. `list_saves()` returns metadata for all
//! occupied slots.

use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};

use rebellion_core::ai::AIState;
use rebellion_core::betrayal::BetrayalState;
use rebellion_core::blockade::BlockadeState;
use rebellion_core::death_star::DeathStarState;
use rebellion_core::economy::EconomyState;
use rebellion_core::events::EventState;
use rebellion_core::fog::FogState;
use rebellion_core::ids::SystemKey;
use rebellion_core::jedi::JediState;
use rebellion_core::delivery::DeliveryState;
use rebellion_core::manufacturing::ManufacturingState;
use rebellion_core::missions::MissionState;
use rebellion_core::movement::MovementState;
use rebellion_core::repair::RepairState;
use rebellion_core::research::ResearchState;
use rebellion_core::tick::GameClock;
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::tuning::GameConfig;
use rebellion_core::uprising::UprisingState;
use rebellion_core::victory::VictoryState;
use rebellion_core::world::{CampaignConfig, GameWorld};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Binary magic at the start of every save file. 8 bytes.
pub const SAVE_MAGIC: &[u8; 8] = b"OPENREB\0";

/// Current save format version. Increment when `SaveState` layout changes;
/// saves of any other version are rejected.
pub const SAVE_VERSION: u32 = 32;

/// Current state-fingerprint algorithm version.
///
/// Version 1 is domain-separated FNV-1a over a canonical JSON projection of
/// the logical save state. JSON object keys and known set fields are sorted;
/// meaningful sequence order is preserved. It is an informational determinism
/// and corruption signal, not a cryptographic authentication mechanism.
pub const STATE_FINGERPRINT_VERSION: u16 = 1;

/// Maximum number of named save slots.
pub const MAX_SAVE_SLOTS: usize = 10;

// ---------------------------------------------------------------------------
// Mod hash
// ---------------------------------------------------------------------------

/// Compute a deterministic hash from sorted (name, version) pairs.
///
/// Uses FNV-1a (64-bit). The mod list is sorted before hashing so that
/// insertion order does not affect the result.
#[must_use]
pub fn compute_mod_hash(mods: &[(String, String)]) -> u64 {
    let mut sorted = mods.to_vec();
    sorted.sort();
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325; // FNV-1a offset basis
    for (name, version) in &sorted {
        for byte in name
            .bytes()
            .chain(b":".iter().copied())
            .chain(version.bytes())
        {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0100_0000_01b3); // FNV-1a prime
        }
        hash ^= 0xff; // separator between mod entries
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

// ---------------------------------------------------------------------------
// State fingerprint
// ---------------------------------------------------------------------------

/// Versioned fingerprint of a logical game-state snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateFingerprint {
    /// Fingerprint algorithm version, independent of [`SAVE_VERSION`].
    pub version: u16,
    /// Non-cryptographic 64-bit digest.
    pub value: u64,
}

impl std::fmt::Display for StateFingerprint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "v{}:{:016x}", self.version, self.value)
    }
}

const UNORDERED_SET_FIELDS: &[&str] = &["blockaded", "busy_characters", "fired_ids", "visible"];

fn canonicalize_fingerprint_value(
    value: &mut serde_json::Value,
    field_name: Option<&str>,
) -> anyhow::Result<()> {
    match value {
        serde_json::Value::Object(fields) => {
            for (name, child) in fields {
                canonicalize_fingerprint_value(child, Some(name))?;
            }
        }
        serde_json::Value::Array(items) => {
            for item in items.iter_mut() {
                canonicalize_fingerprint_value(item, None)?;
            }
            if field_name.is_some_and(|name| UNORDERED_SET_FIELDS.contains(&name)) {
                let mut keyed = items
                    .drain(..)
                    .map(|item| Ok((serde_json::to_vec(&item)?, item)))
                    .collect::<anyhow::Result<Vec<_>>>()?;
                keyed.sort_by(|left, right| left.0.cmp(&right.0));
                items.extend(keyed.into_iter().map(|(_, item)| item));
            }
        }
        _ => {}
    }
    Ok(())
}

/// Canonicalize a snapshot and return the fingerprint used by save files.
///
/// # Errors
/// Returns an error if the canonical save state cannot be serialized for hashing.
pub fn compute_state_fingerprint(state: &SaveState) -> anyhow::Result<StateFingerprint> {
    let mut canonical_state = serde_json::to_value(state)?;
    canonicalize_fingerprint_value(&mut canonical_state, None)?;
    let canonical_bytes = serde_json::to_vec(&canonical_state)?;

    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in b"OPENREB-STATE-FINGERPRINT\0"
        .iter()
        .copied()
        .chain(STATE_FINGERPRINT_VERSION.to_le_bytes())
        .chain(SAVE_VERSION.to_le_bytes())
        .chain(canonical_bytes)
    {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    Ok(StateFingerprint {
        version: STATE_FINGERPRINT_VERSION,
        value: hash,
    })
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
    // ── v10: deterministic continuation envelope ────────────────────────
    /// Exact simulation RNG position; restoring it prevents post-load rolls
    /// from diverging from an uninterrupted campaign.
    pub sim_rng: Xoshiro256PlusPlus,
    /// Optional AI controlling the player's nominal faction in dual-AI mode.
    pub ai2: Option<AIState>,
    pub repair: RepairState,
    /// Last automatic-combat tick per system.
    #[serde(
        serialize_with = "rebellion_core::serde_ordered::serialize_hash_map",
        deserialize_with = "rebellion_core::serde_ordered::deserialize_hash_map"
    )]
    pub combat_cooldowns: std::collections::HashMap<SystemKey, u64>,
    /// Tuning parameters used by the simulation that produced this state.
    pub game_config: GameConfig,
    /// Original difficulty, galaxy-size, faction, and victory-condition choices.
    pub campaign_config: CampaignConfig,
    /// Regiments embarked aboard capital-ship transports.
    pub troop_transport: TroopTransportState,
    // ── v16: en-route manufactured objects (F-030) ──────────────────────
    /// Manufactured objects travelling to their destination.
    pub deliveries: DeliveryState,
    // ── v29: the player's agent (Manage Garrisons / Manage Production) ──
    /// Saved with the game as the original saves its agent (`FUN_004397a0`).
    pub player_agent: rebellion_core::agent_automation::PlayerAgent,
}

// ---------------------------------------------------------------------------
// SaveMeta — slot metadata (no heavy world data)
// ---------------------------------------------------------------------------

/// Lightweight metadata for a save slot — used by the save/load UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMeta {
    /// Slot index (`0..MAX_SAVE_SLOTS`).
    pub slot: usize,
    /// Human-readable name provided by the player.
    pub name: String,
    /// Unix timestamp (seconds since epoch) of when the save was written.
    pub timestamp_secs: u64,
    /// Game tick at save time.
    pub game_tick: u64,
    /// Names of mods that were active when the save was written.
    pub mod_names: Vec<String>,
    /// Deterministic hash of the sorted (name, version) mod list.
    pub mod_hash: u64,
    /// Fingerprint of the canonical logical state, verified on load.
    pub state_fingerprint: StateFingerprint,
}

// ---------------------------------------------------------------------------
// IO functions (native only)
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::{
        compute_mod_hash, compute_state_fingerprint, SaveMeta, SaveState, StateFingerprint,
        MAX_SAVE_SLOTS, SAVE_MAGIC, SAVE_VERSION, STATE_FINGERPRINT_VERSION,
    };
    use std::io::{Read, Write};
    use std::path::{Path, PathBuf};

    use anyhow::Context;

    /// Default save directory: `<exe_dir>/saves/`.
    #[must_use]
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
    #[must_use]
    pub fn slot_path(saves_dir: &Path, slot: usize) -> PathBuf {
        saves_dir.join(format!("{slot}.reb"))
    }

    /// Write `state` to slot `slot` in `saves_dir`.
    ///
    /// `active_mods` is a list of `(name, version)` pairs for currently loaded
    /// mods. Pass an empty slice when no mods are active.
    ///
    /// Creates `saves_dir` if it does not exist.
    ///
    /// # Errors
    /// Returns an error if state serialization, fingerprinting, or creating/writing
    /// the save file fails.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Keep the existing fixed-width save encoding; changing overflow handling is outside this lint cleanup."
    )]
    pub fn save_slot(
        saves_dir: &Path,
        slot: usize,
        name: &str,
        state: &SaveState,
        active_mods: &[(String, String)],
    ) -> anyhow::Result<StateFingerprint> {
        std::fs::create_dir_all(saves_dir)
            .with_context(|| format!("creating saves directory {}", saves_dir.display()))?;

        let encoded = bincode::serialize(state).context("serializing save state")?;
        let state_fingerprint = compute_state_fingerprint(state)?;

        let path = slot_path(saves_dir, slot);
        let mut file = std::fs::File::create(&path)
            .with_context(|| format!("creating save file {}", path.display()))?;

        // ── Header ──────────────────────────────────────────────────────────
        file.write_all(SAVE_MAGIC).context("writing save magic")?;
        file.write_all(&SAVE_VERSION.to_le_bytes())
            .context("writing save version")?;

        // Name: u32 length + UTF-8 bytes
        let name_bytes = name.as_bytes();
        file.write_all(&(name_bytes.len() as u32).to_le_bytes())
            .context("writing name length")?;
        file.write_all(name_bytes).context("writing name")?;

        // Timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        file.write_all(&timestamp.to_le_bytes())
            .context("writing timestamp")?;

        // Mod metadata
        file.write_all(&(active_mods.len() as u32).to_le_bytes())
            .context("writing mod count")?;
        for (mod_name, mod_version) in active_mods {
            let nb = mod_name.as_bytes();
            file.write_all(&(nb.len() as u32).to_le_bytes())
                .context("writing mod name length")?;
            file.write_all(nb).context("writing mod name")?;
            let vb = mod_version.as_bytes();
            file.write_all(&(vb.len() as u32).to_le_bytes())
                .context("writing mod version length")?;
            file.write_all(vb).context("writing mod version")?;
        }
        let mod_hash = compute_mod_hash(active_mods);
        file.write_all(&mod_hash.to_le_bytes())
            .context("writing mod hash")?;

        // State fingerprint
        file.write_all(&state_fingerprint.version.to_le_bytes())
            .context("writing state fingerprint version")?;
        file.write_all(&state_fingerprint.value.to_le_bytes())
            .context("writing state fingerprint")?;

        // ── Body ────────────────────────────────────────────────────────────
        file.write_all(&encoded).context("writing save body")?;

        Ok(state_fingerprint)
    }

    /// Convenience wrapper: save with no active mods.
    ///
    /// # Errors
    /// Returns an error if state serialization, fingerprinting, or creating/writing
    /// the save file fails.
    pub fn save_slot_no_mods(
        saves_dir: &Path,
        slot: usize,
        name: &str,
        state: &SaveState,
    ) -> anyhow::Result<StateFingerprint> {
        save_slot(saves_dir, slot, name, state, &[])
    }

    /// Load `SaveState` from slot `slot` in `saves_dir`.
    ///
    /// Only `SAVE_VERSION` loads; any other version is rejected.
    ///
    /// # Errors
    /// Returns an error for unreadable, truncated, corrupt, or unsupported save data,
    /// including invalid text, deserialization failures, and fingerprint mismatches.
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
        file.read_exact(&mut version_buf)
            .context("reading version")?;
        let version = u32::from_le_bytes(version_buf);
        anyhow::ensure!(
            version == SAVE_VERSION,
            "save version {version} is not supported by this build (it reads only version \
             {SAVE_VERSION}). Please start a new game."
        );

        let mut name_len_buf = [0u8; 4];
        file.read_exact(&mut name_len_buf)
            .context("reading name length")?;
        let name_len = u32::from_le_bytes(name_len_buf) as usize;
        let mut name_bytes = vec![0u8; name_len];
        file.read_exact(&mut name_bytes).context("reading name")?;
        let name = String::from_utf8(name_bytes).context("invalid save name encoding")?;

        let mut ts_buf = [0u8; 8];
        file.read_exact(&mut ts_buf).context("reading timestamp")?;
        let timestamp_secs = u64::from_le_bytes(ts_buf);

        let mut count_buf = [0u8; 4];
        file.read_exact(&mut count_buf)
            .context("reading mod count")?;
        let mod_count = u32::from_le_bytes(count_buf) as usize;
        let mut mod_names = Vec::with_capacity(mod_count);
        for _ in 0..mod_count {
            let mut len_buf = [0u8; 4];
            file.read_exact(&mut len_buf)
                .context("reading mod name length")?;
            let len = u32::from_le_bytes(len_buf) as usize;
            let mut bytes = vec![0u8; len];
            file.read_exact(&mut bytes).context("reading mod name")?;
            let mod_name = String::from_utf8(bytes).context("invalid mod name encoding")?;

            file.read_exact(&mut len_buf)
                .context("reading mod version length")?;
            let vlen = u32::from_le_bytes(len_buf) as usize;
            let mut vbytes = vec![0u8; vlen];
            file.read_exact(&mut vbytes)
                .context("reading mod version")?;
            // We store name only in meta; version is folded into the hash.
            let _mod_version =
                String::from_utf8(vbytes).context("invalid mod version encoding")?;

            mod_names.push(mod_name);
        }
        let mut hash_buf = [0u8; 8];
        file.read_exact(&mut hash_buf).context("reading mod hash")?;
        let mod_hash = u64::from_le_bytes(hash_buf);

        let mut fingerprint_version_buf = [0u8; 2];
        file.read_exact(&mut fingerprint_version_buf)
            .context("reading state fingerprint version")?;
        let fingerprint_version = u16::from_le_bytes(fingerprint_version_buf);
        anyhow::ensure!(
            fingerprint_version == STATE_FINGERPRINT_VERSION,
            "unsupported state fingerprint version {fingerprint_version} (this build supports {STATE_FINGERPRINT_VERSION})"
        );
        let mut fingerprint_buf = [0u8; 8];
        file.read_exact(&mut fingerprint_buf)
            .context("reading state fingerprint")?;
        let expected = StateFingerprint {
            version: fingerprint_version,
            value: u64::from_le_bytes(fingerprint_buf),
        };

        // ── Body ────────────────────────────────────────────────────────────
        let mut body = Vec::new();
        file.read_to_end(&mut body).context("reading save body")?;
        let state: SaveState = bincode::deserialize(&body).context("deserializing save state")?;
        let state_fingerprint = compute_state_fingerprint(&state)?;
        anyhow::ensure!(
            expected == state_fingerprint,
            "save state fingerprint mismatch: expected {expected}, computed {state_fingerprint}"
        );

        let meta = SaveMeta {
            slot,
            name,
            timestamp_secs,
            game_tick: state.clock.tick,
            mod_names,
            mod_hash,
            state_fingerprint,
        };

        Ok((meta, state))
    }

    /// Detect occupancy independently of save decoding, so corrupt saves still
    /// require overwrite confirmation. An inaccessible path fails closed.
    #[must_use]
    pub fn slot_occupied(saves_dir: &Path, slot: usize) -> bool {
        slot_path(saves_dir, slot).try_exists().unwrap_or(true)
    }

    /// Return metadata for all occupied save slots in `saves_dir`.
    ///
    /// Slots without a file are silently skipped. Corrupt files are reported
    /// as `Err` entries in the returned vector.
    #[must_use]
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
    ///
    /// # Errors
    /// Returns an error if an existing save file cannot be deleted.
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
    default_saves_dir, delete_slot, list_saves, load_slot, save_slot, save_slot_no_mods,
    slot_occupied, slot_path,
};

// ---------------------------------------------------------------------------
// WASM browser storage
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
pub mod wasm_impl {
    use super::*;
    use std::path::{Path, PathBuf};

    const BROWSER_META_VERSION: u32 = 1;

    #[derive(Debug, Serialize, Deserialize)]
    struct BrowserStateFingerprint {
        version: u16,
        /// Decimal string avoids JavaScript's 53-bit safe-integer limit.
        value: String,
    }

    impl BrowserStateFingerprint {
        fn from_fingerprint(fingerprint: StateFingerprint) -> Self {
            Self {
                version: fingerprint.version,
                value: fingerprint.value.to_string(),
            }
        }

        fn to_fingerprint(&self) -> anyhow::Result<StateFingerprint> {
            anyhow::ensure!(
                self.version == STATE_FINGERPRINT_VERSION,
                "unsupported state fingerprint version {}",
                self.version
            );
            Ok(StateFingerprint {
                version: self.version,
                value: self.value.parse()?,
            })
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct BrowserSaveMeta {
        schema_version: u32,
        name: String,
        game_tick: u64,
        state_fingerprint: BrowserStateFingerprint,
    }

    #[link(wasm_import_module = "env")]
    extern "C" {
        fn rebellion_storage_set(
            key_ptr: *const u8,
            key_len: usize,
            value_ptr: *const u8,
            value_len: usize,
        ) -> i32;
        fn rebellion_storage_get(
            key_ptr: *const u8,
            key_len: usize,
            output_ptr: *mut u8,
            output_capacity: usize,
        ) -> i32;
        fn rebellion_storage_remove(key_ptr: *const u8, key_len: usize) -> i32;
    }

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
            if chunk.len() > 1 {
                out.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
            }
            if chunk.len() > 2 {
                out.push(CHARS[(triple & 0x3F) as usize] as char);
            }
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
            if chunk.len() < 2 {
                break;
            }
            let a = val(chunk[0])? as u32;
            let b = val(chunk[1])? as u32;
            let c = if chunk.len() > 2 {
                val(chunk[2])? as u32
            } else {
                0
            };
            let d = if chunk.len() > 3 {
                val(chunk[3])? as u32
            } else {
                0
            };
            let triple = (a << 18) | (b << 12) | (c << 6) | d;
            out.push((triple >> 16) as u8);
            if chunk.len() > 2 && chunk[2] != b'=' {
                out.push((triple >> 8) as u8);
            }
            if chunk.len() > 3 && chunk[3] != b'=' {
                out.push(triple as u8);
            }
        }
        Ok(out)
    }

    fn storage_set(key: &str, value: &str) -> anyhow::Result<()> {
        let status =
            unsafe { rebellion_storage_set(key.as_ptr(), key.len(), value.as_ptr(), value.len()) };
        if status == 0 {
            Ok(())
        } else {
            anyhow::bail!("localStorage.setItem failed (access denied or quota exceeded)")
        }
    }

    fn storage_get(key: &str) -> anyhow::Result<Option<String>> {
        let required =
            unsafe { rebellion_storage_get(key.as_ptr(), key.len(), std::ptr::null_mut(), 0) };
        match required {
            -1 => return Ok(None),
            n if n < 0 => anyhow::bail!("localStorage.getItem failed"),
            _ => {}
        }

        let mut bytes = vec![0; required as usize];
        let written = unsafe {
            rebellion_storage_get(key.as_ptr(), key.len(), bytes.as_mut_ptr(), bytes.len())
        };
        if written < 0 || written as usize != bytes.len() {
            anyhow::bail!("localStorage value changed while being read")
        }
        Ok(Some(String::from_utf8(bytes)?))
    }

    fn storage_remove(key: &str) -> anyhow::Result<()> {
        let status = unsafe { rebellion_storage_remove(key.as_ptr(), key.len()) };
        if status == 0 {
            Ok(())
        } else {
            anyhow::bail!("localStorage.removeItem failed")
        }
    }

    /// localStorage key for a save slot.
    ///
    /// The prefix carries the save format version, so entries from older
    /// builds are ignored instead of failing a bincode deserialize.
    fn slot_key(slot: usize) -> String {
        format!("rebellion_save_v{SAVE_VERSION}_{slot}")
    }

    /// localStorage key for versioned JSON save metadata (see [`slot_key`]).
    fn meta_key(slot: usize) -> String {
        format!("rebellion_meta_v{SAVE_VERSION}_{slot}")
    }

    /// The stored body and metadata of a slot, when both are present.
    ///
    /// A body without its metadata (an interrupted write) counts as no save.
    fn stored_save(slot: usize) -> anyhow::Result<Option<(String, String)>> {
        Ok(match (storage_get(&slot_key(slot))?, storage_get(&meta_key(slot))?) {
            (Some(body), Some(meta)) => Some((body, meta)),
            _ => None,
        })
    }

    fn parse_meta(encoded: &str) -> anyhow::Result<BrowserSaveMeta> {
        let meta: BrowserSaveMeta = serde_json::from_str(encoded)?;
        anyhow::ensure!(
            meta.schema_version == BROWSER_META_VERSION,
            "unsupported browser save metadata version {}",
            meta.schema_version
        );
        Ok(meta)
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
    ) -> anyhow::Result<StateFingerprint> {
        let encoded = bincode::serialize(state)?;
        let state_fingerprint = compute_state_fingerprint(state)?;
        let b64 = b64_encode(&encoded);

        storage_set(&slot_key(slot), &b64)?;

        // Store metadata separately (lightweight, for list_saves). JSON keeps
        // player-provided names lossless and makes the schema explicit.
        let meta = serde_json::to_string(&BrowserSaveMeta {
            schema_version: BROWSER_META_VERSION,
            name: name.to_string(),
            game_tick: state.clock.tick,
            state_fingerprint: BrowserStateFingerprint::from_fingerprint(state_fingerprint),
        })?;
        if let Err(error) = storage_set(&meta_key(slot), &meta) {
            // Drop the orphaned body so it is never paired with stale metadata.
            let _ = storage_remove(&slot_key(slot));
            return Err(error);
        }

        Ok(state_fingerprint)
    }

    pub fn save_slot_no_mods(
        saves_dir: &Path,
        slot: usize,
        name: &str,
        state: &SaveState,
    ) -> anyhow::Result<StateFingerprint> {
        save_slot(saves_dir, slot, name, state, &[])
    }

    pub fn load_slot(_saves_dir: &Path, slot: usize) -> anyhow::Result<(SaveMeta, SaveState)> {
        let Some((b64, meta_encoded)) = stored_save(slot)? else {
            anyhow::bail!("no save in slot {}", slot);
        };
        let bytes = b64_decode(&b64)?;
        let browser_meta = parse_meta(&meta_encoded)?;
        let expected = browser_meta.state_fingerprint.to_fingerprint()?;
        let state: SaveState = bincode::deserialize(&bytes)?;
        let state_fingerprint = compute_state_fingerprint(&state)?;
        anyhow::ensure!(
            expected == state_fingerprint,
            "save state fingerprint mismatch: expected {expected}, computed {state_fingerprint}"
        );

        let meta = SaveMeta {
            slot,
            name: browser_meta.name,
            timestamp_secs: 0, // no reliable clock in WASM
            game_tick: state.clock.tick,
            mod_names: vec![],
            mod_hash: compute_mod_hash(&[]),
            state_fingerprint,
        };

        Ok((meta, state))
    }

    /// Metadata and payload keys both count as occupied, including partially
    /// written saves. Storage access errors fail closed.
    pub fn slot_occupied(_saves_dir: &Path, slot: usize) -> bool {
        [slot_key(slot), meta_key(slot)]
            .iter()
            .any(|key| !matches!(storage_get(key), Ok(None)))
    }

    pub fn list_saves(_saves_dir: &Path) -> Vec<anyhow::Result<SaveMeta>> {
        (0..MAX_SAVE_SLOTS)
            .filter_map(|slot| match stored_save(slot) {
                Ok(None) => None,
                Ok(Some((_, encoded))) => Some(parse_meta(&encoded).and_then(|meta| {
                    Ok(SaveMeta {
                        slot,
                        name: meta.name,
                        timestamp_secs: 0,
                        game_tick: meta.game_tick,
                        mod_names: vec![],
                        mod_hash: compute_mod_hash(&[]),
                        state_fingerprint: meta.state_fingerprint.to_fingerprint()?,
                    })
                })),
                Err(error) => Some(Err(error)),
            })
            .collect()
    }

    pub fn delete_slot(_saves_dir: &Path, slot: usize) -> anyhow::Result<()> {
        storage_remove(&slot_key(slot))?;
        storage_remove(&meta_key(slot))
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_impl::{
    default_saves_dir, delete_slot, list_saves, load_slot, save_slot, save_slot_no_mods,
    slot_occupied,
};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use rand::{RngCore, SeedableRng};
    use rebellion_core::ai::{AIState, AiFaction};
    use rebellion_core::dat::Faction;
    use rebellion_core::world::ControlKind;

    fn minimal_save_state() -> SaveState {
        // Create a minimal world with two systems for VictoryState
        let mut world = GameWorld::default();
        let sector_key = world.sectors.insert(rebellion_core::world::Sector {
            dat_id: rebellion_core::ids::DatId::new(0x9200_0000),
            name: "Test".into(),
            group: rebellion_core::dat::SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let sys_a = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(0x9000_0000),
            name: "A".into(),
            sector: sector_key,
            x: 0,
            y: 0,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: true,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
            espionage_rating: 0.0,
        });
        let sys_b = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(0x9000_0001),
            name: "B".into(),
            sector: sector_key,
            x: 100,
            y: 100,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: true,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
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
            sim_rng: Xoshiro256PlusPlus::seed_from_u64(42),
            ai2: None,
            repair: RepairState::default(),
            combat_cooldowns: std::collections::HashMap::new(),
            game_config: GameConfig::default(),
            campaign_config: CampaignConfig::default(),
            troop_transport: TroopTransportState::default(),
            deliveries: DeliveryState::default(),
            player_agent: rebellion_core::agent_automation::PlayerAgent::default(),
        }
    }

    /// Create a unique temp directory scoped to this test.
    fn tmp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir()
            .join("open_rebellion_save_tests")
            .join(name);
        // Start empty: a file left by an earlier run must not stand in for
        // a fixture this run failed to write.
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create tmp dir");
        dir
    }

    // ── Existing tests (updated for new save_slot signature) ────────────────

    #[test]
    fn save_and_load_round_trip_preserves_state() {
        let saves_dir = tmp_dir("round_trip_v5");

        let state = minimal_save_state();
        save_slot(&saves_dir, 0, "Test Save", &state, &[]).expect("save should succeed");

        let (meta, loaded) = load_slot(&saves_dir, 0).expect("load should succeed");

        assert_eq!(meta.slot, 0);
        assert_eq!(meta.name, "Test Save");
        assert_eq!(meta.game_tick, loaded.clock.tick);
        assert!(meta.mod_names.is_empty());
        assert_eq!(meta.mod_hash, compute_mod_hash(&[]));
        assert_eq!(
            meta.state_fingerprint,
            compute_state_fingerprint(&loaded).expect("fingerprint loaded state")
        );
    }

    #[test]
    fn round_trip_preserves_deterministic_continuation_envelope() {
        let saves_dir = tmp_dir("continuation_envelope_v13");
        let mut state = minimal_save_state();
        state.sim_rng = Xoshiro256PlusPlus::seed_from_u64(0x5eed);
        state.ai2 = Some(AIState::new(AiFaction::Alliance));
        state.ai2.as_mut().unwrap().last_eval_tick = 77;
        let system = state.world.systems.keys().next().unwrap();
        state.combat_cooldowns.insert(system, 61);
        state.game_config.ai.tick_interval = 13;
        state.campaign_config.galaxy_size = rebellion_core::dat::GalaxySize::Huge;
        state.campaign_config.difficulty = rebellion_core::world::SeedDifficulty::Hard;
        state.campaign_config.victory_conditions =
            rebellion_core::world::VictoryConditions::HeadquartersOnly;

        let facility = state.world.manufacturing_facilities.insert(
            rebellion_core::world::ManufacturingFacilityInstance {
                class_dat_id: rebellion_core::ids::DatId::new(0x2800_0001),
                side: rebellion_core::dat::Faction::Empire,
                is_shipyard: true,
            },
        );
        state.world.systems[system]
            .manufacturing_facilities
            .push(facility);
        let class =
            state
                .world
                .capital_ship_classes
                .insert(rebellion_core::world::CapitalShipClass {
                    dat_id: rebellion_core::ids::DatId::new(0x1400_0001),
                    name: "Repair fixture".into(),
                    hull: 100,
                    damage_control: 5,
                    troop_capacity: 1,
                    ..Default::default()
                });
        let fleet = state.world.fleets.insert(rebellion_core::world::Fleet {
            location: system,
            capital_ships: vec![rebellion_core::world::ShipInstance::new(class, 75, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        state.world.systems[system].fleets.push(fleet);
        let repair_events = rebellion_core::repair::RepairSystem::advance(
            &mut state.repair,
            &state.world,
            &[rebellion_core::tick::TickEvent { tick: 1 }],
        );
        assert!(!repair_events.is_empty());
        assert!(state.repair.is_repairing(fleet));

        let troop = state.world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: rebellion_core::ids::DatId::new(0x1000_0008),
            is_alliance: false,
            regiment_strength: 100,
        });
        state.world.systems[system].ground_units.push(troop);
        state
            .troop_transport
            .embark(&mut state.world, fleet, &[troop])
            .unwrap();

        save_slot(&saves_dir, 0, "Continuation", &state, &[]).unwrap();
        let mut uninterrupted_rng = state.sim_rng.clone();
        let expected_rolls = (0..8)
            .map(|_| uninterrupted_rng.next_u64())
            .collect::<Vec<_>>();

        let (_, mut loaded) = load_slot(&saves_dir, 0).unwrap();
        let loaded_rolls = (0..8)
            .map(|_| loaded.sim_rng.next_u64())
            .collect::<Vec<_>>();

        assert_eq!(loaded_rolls, expected_rolls);
        assert_eq!(loaded.ai2.as_ref().unwrap().last_eval_tick, 77);
        assert_eq!(loaded.combat_cooldowns.get(&system), Some(&61));
        assert_eq!(loaded.game_config.ai.tick_interval, 13);
        assert_eq!(loaded.campaign_config, state.campaign_config);
        assert!(loaded.repair.is_repairing(fleet));
        assert_eq!(loaded.troop_transport.cargo(fleet), &[troop]);
    }

    #[test]
    fn repeated_snapshots_have_identical_fingerprints() {
        let first_run = minimal_save_state();
        let second_run = minimal_save_state();

        let first = compute_state_fingerprint(&first_run).expect("fingerprint first snapshot");
        let second = compute_state_fingerprint(&second_run).expect("fingerprint second snapshot");

        assert_eq!(first.version, STATE_FINGERPRINT_VERSION);
        assert_eq!(first, second);
        assert!(first.to_string().starts_with("v1:"));
    }

    #[test]
    fn fingerprint_normalizes_unordered_set_insertion() {
        let mut forward = minimal_save_state();
        let mut reverse = forward.clone();
        let system_keys = forward.world.systems.keys().collect::<Vec<_>>();

        forward.fog_alliance.visible.insert(system_keys[0]);
        forward.fog_alliance.visible.insert(system_keys[1]);
        reverse.fog_alliance.visible.insert(system_keys[1]);
        reverse.fog_alliance.visible.insert(system_keys[0]);

        assert_eq!(
            compute_state_fingerprint(&forward).unwrap(),
            compute_state_fingerprint(&reverse).unwrap()
        );
    }

    #[test]
    fn fingerprint_normalizes_typed_map_insertion() {
        let mut forward = minimal_save_state();
        let mut reverse = forward.clone();
        let system_keys = forward.world.systems.keys().collect::<Vec<_>>();

        forward.combat_cooldowns.insert(system_keys[0], 10);
        forward.combat_cooldowns.insert(system_keys[1], 20);
        reverse.combat_cooldowns.insert(system_keys[1], 20);
        reverse.combat_cooldowns.insert(system_keys[0], 10);

        assert_eq!(
            compute_state_fingerprint(&forward).unwrap(),
            compute_state_fingerprint(&reverse).unwrap()
        );
    }

    #[test]
    fn fingerprint_covers_rng_and_configuration() {
        let original = minimal_save_state();
        let mut rng_changed = original.clone();
        rng_changed.sim_rng.next_u64();
        let mut config_changed = original.clone();
        config_changed.game_config.ai.tick_interval += 1;
        let mut campaign_changed = original.clone();
        campaign_changed.campaign_config.victory_conditions =
            rebellion_core::world::VictoryConditions::HeadquartersOnly;

        let original_fingerprint = compute_state_fingerprint(&original).unwrap();
        assert_ne!(
            original_fingerprint,
            compute_state_fingerprint(&rng_changed).unwrap()
        );
        assert_ne!(
            original_fingerprint,
            compute_state_fingerprint(&config_changed).unwrap()
        );
        assert_ne!(
            original_fingerprint,
            compute_state_fingerprint(&campaign_changed).unwrap()
        );
    }

    #[test]
    fn fingerprint_mismatch_rejects_tampered_body() {
        const SAVE_NAME: &str = "Tamper Check";

        let saves_dir = tmp_dir("fingerprint_tamper_v9");
        let state = minimal_save_state();
        save_slot(&saves_dir, 0, SAVE_NAME, &state, &[]).unwrap();

        let path = slot_path(&saves_dir, 0);
        let mut bytes = std::fs::read(&path).unwrap();
        let body_offset = SAVE_MAGIC.len() + 4 + 4 + SAVE_NAME.len() + 8 + 4 + 8 + 2 + 8;
        let mut changed = state.clone();
        changed.clock.tick = 1;
        bytes.truncate(body_offset);
        bytes.extend(bincode::serialize(&changed).unwrap());
        std::fs::write(path, bytes).unwrap();

        let error = load_slot(&saves_dir, 0).expect_err("tampered body must be rejected");
        assert!(error.to_string().contains("fingerprint mismatch"));
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
            .filter_map(std::result::Result::ok)
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
    fn mod_hash_round_trip() {
        let saves_dir = tmp_dir("mod_hash_rt");
        let state = minimal_save_state();
        let mods = vec![("TestMod".to_string(), "1.0".to_string())];

        save_slot(&saves_dir, 0, "Modded", &state, &mods).expect("save with mods should succeed");

        let (meta, _) = load_slot(&saves_dir, 0).expect("load modded save should succeed");

        assert_eq!(meta.mod_names, vec!["TestMod".to_string()]);
        assert_eq!(meta.mod_hash, compute_mod_hash(&mods));
        // A different mod set yields a different hash, so callers can detect
        // a save made under other mods.
        let other_mods = vec![("OtherMod".to_string(), "2.0".to_string())];
        assert_ne!(meta.mod_hash, compute_mod_hash(&other_mods));
    }

    #[test]
    fn a_save_of_any_other_version_is_rejected_with_a_new_game_message() {
        let saves_dir = tmp_dir("other_version");
        save_slot(&saves_dir, 0, "Other", &minimal_save_state(), &[]).unwrap();
        let path = slot_path(&saves_dir, 0);
        let original = std::fs::read(&path).unwrap();

        for version in [SAVE_VERSION - 1, SAVE_VERSION + 1] {
            let mut bytes = original.clone();
            bytes[SAVE_MAGIC.len()..SAVE_MAGIC.len() + 4].copy_from_slice(&version.to_le_bytes());
            std::fs::write(&path, bytes).unwrap();

            let msg = load_slot(&saves_dir, 0).expect_err("only the current version loads").to_string();
            assert!(
                msg.contains(&format!("save version {version}")) && msg.contains("new game"),
                "error should name the version and suggest a new game: {msg}"
            );
        }
    }

    /// A save state with one regiment embarked on a fleet in orbit, already
    /// tracked by `BlockadeSystem::running_regiments` (F-021).
    fn state_with_tracked_regiment() -> (SaveState, rebellion_core::ids::TroopKey, SystemKey) {
        use rebellion_core::blockade::BlockadeSystem;
        use rebellion_core::movement::MovementState;

        let mut state = minimal_save_state();
        let system = state.world.systems.keys().next().unwrap();
        let class =
            state
                .world
                .capital_ship_classes
                .insert(rebellion_core::world::CapitalShipClass {
                    troop_capacity: 1,
                    ..Default::default()
                });
        let fleet = state.world.fleets.insert(rebellion_core::world::Fleet {
            location: system,
            capital_ships: vec![rebellion_core::world::ShipInstance::new(class, 100, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        state.world.systems[system].fleets.push(fleet);
        let troop = state.world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: rebellion_core::ids::DatId::new(0x1000_0008),
            is_alliance: false,
            regiment_strength: 100,
        });
        state.world.systems[system].ground_units.push(troop);
        state
            .troop_transport
            .embark(&mut state.world, fleet, &[troop])
            .unwrap();
        BlockadeSystem::running_regiments(
            &mut state.blockade,
            &state.world,
            &MovementState::new(),
            &state.troop_transport,
        );
        assert!(
            state.blockade.embarked_regiment(troop).is_some(),
            "the regiment is tracked before saving"
        );
        (state, troop, system)
    }

    /// A regiment's withdraw percent (`+0x60`, set by `FUN_0050b310` and rolled
    /// by `FUN_00504990` when it leaves) must survive a save, or a reload
    /// would let a regiment run a blockade without its roll (F-021).
    #[test]
    fn a_round_trip_preserves_an_embarked_regiments_orbit_and_withdraw_percent() {
        use rebellion_core::blockade::EmbarkedRegiment;

        let saves_dir = tmp_dir("v15_embarked_roundtrip");
        let (state, troop, system) = state_with_tracked_regiment();
        let tracked = state.blockade.embarked_regiment(troop).unwrap();

        save_slot(&saves_dir, 0, "Embarked", &state, &[]).unwrap();
        let (_, loaded) = load_slot(&saves_dir, 0).unwrap();

        assert_eq!(
            loaded.blockade.embarked_regiment(troop),
            Some(EmbarkedRegiment {
                orbit: Some(system),
                withdraw_percent: tracked.withdraw_percent,
            })
        );
    }

    #[test]
    fn a_round_trip_preserves_revolt_and_disaster_timers() {
        let saves_dir = tmp_dir("v15_uprising_round_trip");
        let mut state = minimal_save_state();
        let system = state.world.systems.keys().next().unwrap();
        state.uprising.active_uprisings.insert(
            system,
            rebellion_core::uprising::ActiveUprising {
                started_tick: 3,
                next_incident_tick: Some(45),
            },
        );
        state.uprising.next_disaster_tick = Some(250);
        save_slot(&saves_dir, 0, "V15 Save", &state, &[]).unwrap();

        let (_, loaded) = load_slot(&saves_dir, 0).expect("v15 save should load");

        let revolt = &loaded.uprising.active_uprisings[&system];
        assert_eq!(revolt.started_tick, 3);
        assert_eq!(revolt.next_incident_tick, Some(45));
        assert_eq!(loaded.uprising.next_disaster_tick, Some(250));
    }

    /// A save holding a remote build order and a delivery en route, for the
    /// F-030 layout tests.
    fn state_with_delivery() -> (SaveState, SystemKey, SystemKey) {
        let mut state = minimal_save_state();
        let mut systems = state.world.systems.keys();
        let (origin, destination) = (systems.next().unwrap(), systems.next().unwrap());
        let class = state
            .world
            .capital_ship_classes
            .insert(rebellion_core::world::CapitalShipClass::default());
        let kind = rebellion_core::manufacturing::BuildableKind::CapitalShip(class);
        state.manufacturing.enqueue(
            origin,
            rebellion_core::manufacturing::QueueItem::new(kind, 5, 10).delivered_to(destination),
        );
        state.deliveries.depart(
            &state.world,
            &[rebellion_core::manufacturing::Departure {
                origin,
                destination,
                tick: 7,
                kind,
            }],
        );
        (state, origin, destination)
    }

    #[test]
    fn a_round_trip_keeps_build_destinations_and_deliveries_en_route() {
        let saves_dir = tmp_dir("v16_delivery_round_trip");
        let (state, origin, destination) = state_with_delivery();
        save_slot(&saves_dir, 0, "V16 Save", &state, &[]).unwrap();

        let (_, loaded) = load_slot(&saves_dir, 0).expect("the save should load");

        assert_eq!(loaded.deliveries, state.deliveries);
        assert_eq!(loaded.deliveries.en_route().len(), 1);
        let queue = loaded
            .manufacturing
            .queue(
                origin,
                rebellion_core::manufacturing::ProductionArea::Shipyard,
            )
            .unwrap();
        assert_eq!(queue.active().unwrap().destination, Some(destination));
    }

    // Each production area keeps its own queue and Destination (v30,
    // FUN_00509670); both survive a save, whose fingerprint reads the state
    // as JSON.
    #[test]
    fn a_round_trip_keeps_each_production_areas_queue_and_destination() {
        use rebellion_core::manufacturing::ProductionArea;
        let saves_dir = tmp_dir("v30_area_round_trip");
        let (mut state, origin, destination) = state_with_delivery();
        state
            .manufacturing
            .set_destination(origin, ProductionArea::TrainingFacility, destination);
        state.manufacturing.enqueue(
            origin,
            rebellion_core::manufacturing::QueueItem::new(
                rebellion_core::manufacturing::BuildableKind::Troop(
                    rebellion_core::ids::DatId::new(0x1000_0001),
                ),
                4,
                4,
            ),
        );
        save_slot(&saves_dir, 0, "V30 Save", &state, &[]).unwrap();

        let (_, loaded) = load_slot(&saves_dir, 0).expect("the save should load");

        assert_eq!(
            loaded
                .manufacturing
                .destination(origin, ProductionArea::TrainingFacility),
            Some(destination)
        );
        let troops = loaded
            .manufacturing
            .queue(origin, ProductionArea::TrainingFacility)
            .unwrap();
        assert_eq!(troops.active().unwrap().destination, Some(destination));
        assert_eq!(loaded.manufacturing.queued_at(origin), 2);
    }

    #[test]
    fn a_round_trip_keeps_mission_members_and_special_force_skills() {
        use rebellion_core::missions::{
            MissionFaction, MissionKind, MissionMember, MissionRequest,
        };
        use rebellion_core::world::{Character, MissionRecord, SpecialForceUnit};
        let saves_dir = tmp_dir("v17_mission_members_round_trip");
        let mut state = minimal_save_state();
        let system = state.world.systems.keys().next().unwrap();
        let class_id = rebellion_core::ids::DatId::new(0x3c00_0001);
        let lead = state.world.characters.insert(Character {
            is_alliance: true,
            current_system: Some(system),
            ..Default::default()
        });
        let prisoner = state.world.characters.insert(Character {
            is_alliance: true,
            is_captive: true,
            current_system: Some(system),
            ..Default::default()
        });
        let unit = state.world.special_forces.insert(SpecialForceUnit {
            class_dat_id: class_id,
            is_alliance: true,
            skills: [1, 2, 3, 4, 5, 6, 7, 8],
            on_mission: false,
        });
        state.world.systems[system].special_forces.push(unit);
        state.world.mission_records.push(MissionRecord {
            dat_id: rebellion_core::ids::DatId::new(0x5100_0010),
            timer_min_days: 5,
            timer_spread_days: 10,
            repeats: true,
            hidden: false,
            detection_phases: true,
            can_resign: false,
            rules: rebellion_core::world::MissionTargetRules {
                container_loss_ends: true,
                target_loss_ends: true,
                ..Default::default()
            },
            members: rebellion_core::world::MissionMemberRules {
                alliance: true,
                empire: false,
                special_force_mask: 0x402,
                character_mask: 0x1_0000,
            },
        });
        let request = MissionRequest {
            kind: MissionKind::Diplomacy,
            faction: MissionFaction::Alliance,
            team: vec![MissionMember::Character(lead)],
            decoys: vec![
                MissionMember::SpecialForce(unit),
                MissionMember::Character(prisoner),
            ],
            target_system: system,
            target_character: None,
            target_object: None,
            tick: 0,
        };
        state
            .missions
            .dispatch_guarded(request, &mut state.world)
            .expect("the mission should dispatch");
        save_slot(&saves_dir, 0, "V17 Save", &state, &[]).unwrap();

        let (_, loaded) = load_slot(&saves_dir, 0).expect("the save should load");

        let mission = &loaded.missions.missions()[0];
        assert_eq!(mission.team, vec![MissionMember::Character(lead)]);
        assert_eq!(mission.decoys, vec![MissionMember::SpecialForce(unit)]);
        assert_eq!(mission.captured, vec![MissionMember::Character(prisoner)]);
        let loaded_unit = &loaded.world.special_forces[unit];
        assert_eq!(loaded_unit.skills, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(loaded_unit.on_mission);
        assert_eq!(loaded.world.mission_records, state.world.mission_records);
    }

    #[test]
    fn a_round_trip_keeps_resign_requests_and_troop_detection() {
        use rebellion_core::missions::{ActiveMission, MissionFaction, MissionKind, MissionMember};
        use rebellion_core::world::{Character, TroopClassDef};
        let saves_dir = tmp_dir("v20_resign_detection_round_trip");
        let mut state = minimal_save_state();
        let system = state.world.systems.keys().next().unwrap();
        let lead = MissionMember::Character(state.world.characters.insert(Character {
            is_alliance: true,
            current_system: Some(system),
            ..Default::default()
        }));
        let class = rebellion_core::ids::DatId::new(0x1000_0001);
        state.world.troop_classes.insert(
            class,
            TroopClassDef {
                attack_strength: 1,
                defense_strength: 2,
                detection: 7,
            },
        );
        let mut mission = ActiveMission::new(
            0,
            MissionKind::Sabotage,
            MissionFaction::Alliance,
            vec![lead],
            system,
            0,
        );
        mission.resigning = vec![lead];
        // MissionState exposes no mutable missions; build it from its serde form.
        let mut missions = serde_json::to_value(&state.missions).unwrap();
        missions["missions"] = serde_json::json!([mission]);
        state.missions = serde_json::from_value(missions).unwrap();
        save_slot(&saves_dir, 0, "V20 Save", &state, &[]).unwrap();

        let (_, loaded) = load_slot(&saves_dir, 0).expect("the save should load");

        assert_eq!(loaded.missions.missions()[0].resigning, vec![lead]);
        assert_eq!(loaded.world.troop_classes[&class].detection, 7);
    }

    #[test]
    fn a_round_trip_keeps_mission_phases_and_members_in_transit() {
        use rebellion_core::missions::{
            move_member, MissionEffect, MissionFaction, MissionKind, MissionMember, MissionRequest,
            MissionSystem, PHASE_TRANSIT,
        };
        use rebellion_core::tick::TickEvent;
        use rebellion_core::world::Character;
        let saves_dir = tmp_dir("v18_mission_transit_round_trip");
        let mut state = minimal_save_state();
        let mut systems = state.world.systems.keys();
        let (from, to) = (systems.next().unwrap(), systems.next().unwrap());
        let envoy = state.world.characters.insert(Character {
            is_alliance: true,
            current_system: Some(from),
            ..Default::default()
        });
        state
            .missions
            .dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Diplomacy,
                    MissionFaction::Alliance,
                    envoy,
                    to,
                    None,
                    0,
                ),
                &mut state.world,
            )
            .expect("the mission should dispatch");
        let advance = MissionSystem::advance(
            &mut state.missions,
            &state.world,
            &state.uprising,
            &[TickEvent { tick: 1 }],
            &[],
        );
        for effect in &advance.effects {
            if let MissionEffect::MemberMoved { member, to } = effect {
                move_member(&mut state.world, *member, *to);
            }
        }
        save_slot(&saves_dir, 0, "V18 Save", &state, &[]).unwrap();

        let (_, loaded) = load_slot(&saves_dir, 0).expect("the save should load");

        let (before, after) = (
            &state.missions.missions()[0],
            &loaded.missions.missions()[0],
        );
        assert_eq!(after.phase, PHASE_TRANSIT);
        assert_eq!(
            (
                after.ready,
                after.origin,
                after.speed,
                after.wait_start,
                after.timer_due
            ),
            (before.ready, Some(from), 100, 0, None)
        );
        assert_eq!(loaded.missions.en_route(), state.missions.en_route());
        assert!(loaded.missions.is_en_route(MissionMember::Character(envoy)));
        assert_eq!(loaded.world.characters[envoy].current_system, None);
    }

    #[test]
    fn a_round_trip_keeps_a_mission_s_armed_timer_and_its_seen_loss() {
        use rebellion_core::missions::{
            move_member, MissionEffect, MissionFaction, MissionKind, MissionRequest, MissionSystem,
            PHASE_TIMER,
        };
        use rebellion_core::tick::TickEvent;
        use rebellion_core::world::Character;
        let saves_dir = tmp_dir("mission_timer_round_trip");
        let mut state = minimal_save_state();
        let mut systems = state.world.systems.keys();
        let (from, to) = (systems.next().unwrap(), systems.next().unwrap());
        let envoy = state.world.characters.insert(Character {
            is_alliance: true,
            current_system: Some(from),
            ..Default::default()
        });
        state
            .missions
            .dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Diplomacy,
                    MissionFaction::Alliance,
                    envoy,
                    to,
                    None,
                    0,
                ),
                &mut state.world,
            )
            .expect("the mission should dispatch");
        let step = |state: &mut SaveState, tick: u64| {
            let advance = MissionSystem::advance(
                &mut state.missions,
                &state.world,
                &state.uprising,
                &[TickEvent { tick }],
                &[],
            );
            for effect in &advance.effects {
                if let MissionEffect::MemberMoved { member, to } = effect {
                    move_member(&mut state.world, *member, *to);
                }
            }
        };
        step(&mut state, 1);
        let arrival = state.missions.en_route()[0].arrival;
        state.world.systems[to].is_destroyed = true;
        step(&mut state, 2);
        step(&mut state, arrival);
        save_slot(&saves_dir, 0, "Timer Save", &state, &[]).unwrap();

        let (_, loaded) = load_slot(&saves_dir, 0).expect("the save should load");

        let (before, after) = (
            &state.missions.missions()[0],
            &loaded.missions.missions()[0],
        );
        assert_eq!(after.phase, PHASE_TIMER);
        assert!(before.timer_due.is_some());
        assert_eq!(after.timer_due, before.timer_due);
        assert!(after.container_loss_seen);
    }

    /// A recruit (`+0x50` bit 1) and an emptied pool (side `+0xb8`) must
    /// survive a save, or a reload would recruit the same character again
    /// and let Recruitment run past its end `0x10` (`FUN_0056b370`).
    #[test]
    fn a_round_trip_keeps_recruits_and_an_emptied_recruit_pool() {
        use rebellion_core::dat::Faction;
        use rebellion_core::world::Character;
        let saves_dir = tmp_dir("recruit_pool_round_trip");
        let mut state = minimal_save_state();
        let recruit = state.world.characters.insert(Character {
            is_empire: true,
            recruited: true,
            ..Default::default()
        });
        state.world.set_recruit_pool_empty(Faction::Empire);
        save_slot(&saves_dir, 0, "Recruit Save", &state, &[]).unwrap();

        let (_, loaded) = load_slot(&saves_dir, 0).expect("the save should load");

        assert!(loaded.world.characters[recruit].recruited);
        assert!(loaded.world.recruit_pool_empty(Faction::Empire));
        assert!(!loaded.world.recruit_pool_empty(Faction::Alliance));
    }

    /// A Sabotage order's object (`+0x4c`, read by `FUN_00521030`) must
    /// survive a save, or a reload would leave the mission nothing to
    /// destroy (`FUN_005746e0`).
    #[test]
    fn a_round_trip_keeps_a_sabotage_mission_s_target_object() {
        use rebellion_core::missions::{
            MissionFaction, MissionKind, MissionRequest, MissionTarget,
        };
        use rebellion_core::world::{Character, ManufacturingFacilityInstance};
        let saves_dir = tmp_dir("sabotage_target_round_trip");
        let mut state = minimal_save_state();
        let system = state.world.systems.keys().next().unwrap();
        let yard = state
            .world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: rebellion_core::ids::DatId::new(0x2800_0001),
                side: rebellion_core::dat::Faction::Empire,
                is_shipyard: true,
            });
        state.world.systems[system]
            .manufacturing_facilities
            .push(yard);
        let spy = state.world.characters.insert(Character {
            is_alliance: true,
            current_system: Some(system),
            recruited: true,
            ..Default::default()
        });
        let target = MissionTarget::ManufacturingFacility(yard);
        state
            .missions
            .dispatch_guarded(
                MissionRequest {
                    target_object: Some(target),
                    ..MissionRequest::single(
                        MissionKind::Sabotage,
                        MissionFaction::Alliance,
                        spy,
                        system,
                        None,
                        0,
                    )
                },
                &mut state.world,
            )
            .expect("the sabotage should dispatch");
        save_slot(&saves_dir, 0, "Sabotage Save", &state, &[]).unwrap();

        let (_, loaded) = load_slot(&saves_dir, 0).expect("the save should load");

        assert_eq!(loaded.missions.missions()[0].target_object, Some(target));
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
