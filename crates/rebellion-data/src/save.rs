//! Save / load for the full game state.
//!
//! # Format (v13)
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
//! [fingerprint_version: u16]                // v9+
//! [state_fingerprint: u64]                  // v9+, canonical logical state
//! [bincode-encoded SaveState]
//! ```
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

use rand::SeedableRng;
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

/// Current save format version. Increment when `SaveState` layout changes.
///
/// v9: Header gained a versioned fingerprint of the logical `SaveState`.
/// v10: Body gained deterministic continuation state (RNG, second AI, repair,
/// combat cooldowns, and tuning configuration).
/// v11: Body gained the original new-game campaign configuration.
/// v12: Repair state gained persisted, per-fleet repair-episode tracking.
/// v13: Body gained regiment-to-fleet cargo state.
pub const SAVE_VERSION: u32 = 13;

/// Current state-fingerprint algorithm version.
///
/// Version 1 is domain-separated FNV-1a over a canonical JSON projection of
/// the logical save state. JSON object keys and known set fields are sorted;
/// meaningful sequence order is preserved. It is an informational determinism
/// and corruption signal, not a cryptographic authentication mechanism.
pub const STATE_FINGERPRINT_VERSION: u16 = 1;

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
        for byte in name
            .bytes()
            .chain(b":".iter().copied())
            .chain(version.bytes())
        {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3); // FNV-1a prime
        }
        hash ^= 0xff; // separator between mod entries
        hash = hash.wrapping_mul(0x100000001b3);
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

/// In v9 these typed-key maps used serde's native JSON map representation.
/// The only representable shape was an empty object; non-empty slotmap keys
/// made `save_slot` fail before writing. Preserve that exact empty-map shape
/// when validating fingerprints from an existing v9 file.
const LEGACY_V9_TYPED_MAP_FIELDS: &[&str] = &[
    "active_uprisings",
    "battle_cooldowns",
    "incident_cooldowns",
    "last_check",
    "orders",
    "per_system",
    "queues",
];

fn restore_legacy_v9_empty_map_shapes(value: &mut serde_json::Value, field_name: Option<&str>) {
    match value {
        serde_json::Value::Object(fields) => {
            for (name, child) in fields {
                restore_legacy_v9_empty_map_shapes(child, Some(name));
            }
        }
        serde_json::Value::Array(items)
            if items.is_empty()
                && field_name.is_some_and(|name| LEGACY_V9_TYPED_MAP_FIELDS.contains(&name)) =>
        {
            *value = serde_json::Value::Object(serde_json::Map::new());
        }
        serde_json::Value::Array(items) => {
            for item in items {
                restore_legacy_v9_empty_map_shapes(item, None);
            }
        }
        _ => {}
    }
}

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

fn compute_serializable_fingerprint_for_version<T: Serialize + ?Sized>(
    save_version: u32,
    state: &T,
) -> anyhow::Result<StateFingerprint> {
    let mut canonical_state = serde_json::to_value(state)?;
    if save_version == 9 {
        restore_legacy_v9_empty_map_shapes(&mut canonical_state, None);
    }
    canonicalize_fingerprint_value(&mut canonical_state, None)?;
    let canonical_bytes = serde_json::to_vec(&canonical_state)?;

    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in b"OPENREB-STATE-FINGERPRINT\0"
        .iter()
        .copied()
        .chain(STATE_FINGERPRINT_VERSION.to_le_bytes())
        .chain(save_version.to_le_bytes())
        .chain(canonical_bytes)
    {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(StateFingerprint {
        version: STATE_FINGERPRINT_VERSION,
        value: hash,
    })
}

/// Canonicalize a snapshot and return the fingerprint used by new save files.
pub fn compute_state_fingerprint(state: &SaveState) -> anyhow::Result<StateFingerprint> {
    compute_serializable_fingerprint_for_version(SAVE_VERSION, state)
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
}

/// Exact v12 body. The regiment cargo state was added in v13.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveStateV12 {
    world: GameWorld,
    clock: GameClock,
    manufacturing: ManufacturingState,
    missions: MissionState,
    events: EventState,
    ai: AIState,
    movement: MovementState,
    fog_alliance: FogState,
    fog_empire: FogState,
    player_is_alliance: bool,
    blockade: BlockadeState,
    uprising: UprisingState,
    death_star: DeathStarState,
    research: ResearchState,
    jedi: JediState,
    victory: VictoryState,
    betrayal: BetrayalState,
    economy: EconomyState,
    sim_rng: Xoshiro256PlusPlus,
    ai2: Option<AIState>,
    repair: RepairState,
    #[serde(
        serialize_with = "rebellion_core::serde_ordered::serialize_hash_map",
        deserialize_with = "rebellion_core::serde_ordered::deserialize_hash_map"
    )]
    combat_cooldowns: std::collections::HashMap<SystemKey, u64>,
    game_config: GameConfig,
    campaign_config: CampaignConfig,
}

impl From<SaveStateV12> for SaveState {
    fn from(legacy: SaveStateV12) -> Self {
        Self {
            world: legacy.world,
            clock: legacy.clock,
            manufacturing: legacy.manufacturing,
            missions: legacy.missions,
            events: legacy.events,
            ai: legacy.ai,
            movement: legacy.movement,
            fog_alliance: legacy.fog_alliance,
            fog_empire: legacy.fog_empire,
            player_is_alliance: legacy.player_is_alliance,
            blockade: legacy.blockade,
            uprising: legacy.uprising,
            death_star: legacy.death_star,
            research: legacy.research,
            jedi: legacy.jedi,
            victory: legacy.victory,
            betrayal: legacy.betrayal,
            economy: legacy.economy,
            sim_rng: legacy.sim_rng,
            ai2: legacy.ai2,
            repair: legacy.repair,
            combat_cooldowns: legacy.combat_cooldowns,
            game_config: legacy.game_config,
            campaign_config: legacy.campaign_config,
            troop_transport: TroopTransportState::default(),
        }
    }
}

impl From<&SaveState> for SaveStateV12 {
    fn from(current: &SaveState) -> Self {
        Self {
            world: current.world.clone(),
            clock: current.clock.clone(),
            manufacturing: current.manufacturing.clone(),
            missions: current.missions.clone(),
            events: current.events.clone(),
            ai: current.ai.clone(),
            movement: current.movement.clone(),
            fog_alliance: current.fog_alliance.clone(),
            fog_empire: current.fog_empire.clone(),
            player_is_alliance: current.player_is_alliance,
            blockade: current.blockade.clone(),
            uprising: current.uprising.clone(),
            death_star: current.death_star.clone(),
            research: current.research.clone(),
            jedi: current.jedi.clone(),
            victory: current.victory.clone(),
            betrayal: current.betrayal.clone(),
            economy: current.economy.clone(),
            sim_rng: current.sim_rng.clone(),
            ai2: current.ai2.clone(),
            repair: current.repair.clone(),
            combat_cooldowns: current.combat_cooldowns.clone(),
            game_config: current.game_config.clone(),
            campaign_config: current.campaign_config,
        }
    }
}

/// Exact v10 body. Keep this separate: bincode is positional, so appending a
/// field to `SaveState` cannot be migrated through serde defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveStateV10 {
    world: GameWorld,
    clock: GameClock,
    manufacturing: ManufacturingState,
    missions: MissionState,
    events: EventState,
    ai: AIState,
    movement: MovementState,
    fog_alliance: FogState,
    fog_empire: FogState,
    player_is_alliance: bool,
    blockade: BlockadeState,
    uprising: UprisingState,
    death_star: DeathStarState,
    research: ResearchState,
    jedi: JediState,
    victory: VictoryState,
    betrayal: BetrayalState,
    economy: EconomyState,
    sim_rng: Xoshiro256PlusPlus,
    ai2: Option<AIState>,
    // RepairState was a zero-field unit struct through v11.
    repair: (),
    #[serde(
        serialize_with = "rebellion_core::serde_ordered::serialize_hash_map",
        deserialize_with = "rebellion_core::serde_ordered::deserialize_hash_map"
    )]
    combat_cooldowns: std::collections::HashMap<SystemKey, u64>,
    game_config: GameConfig,
}

impl From<SaveStateV10> for SaveState {
    fn from(legacy: SaveStateV10) -> Self {
        let campaign_config =
            CampaignConfig::from_legacy_world(&legacy.world, legacy.player_is_alliance);
        Self {
            world: legacy.world,
            clock: legacy.clock,
            manufacturing: legacy.manufacturing,
            missions: legacy.missions,
            events: legacy.events,
            ai: legacy.ai,
            movement: legacy.movement,
            fog_alliance: legacy.fog_alliance,
            fog_empire: legacy.fog_empire,
            player_is_alliance: legacy.player_is_alliance,
            blockade: legacy.blockade,
            uprising: legacy.uprising,
            death_star: legacy.death_star,
            research: legacy.research,
            jedi: legacy.jedi,
            victory: legacy.victory,
            betrayal: legacy.betrayal,
            economy: legacy.economy,
            sim_rng: legacy.sim_rng,
            ai2: legacy.ai2,
            repair: RepairState::default(),
            combat_cooldowns: legacy.combat_cooldowns,
            game_config: legacy.game_config,
            campaign_config,
            troop_transport: TroopTransportState::default(),
        }
    }
}

impl From<&SaveState> for SaveStateV10 {
    fn from(current: &SaveState) -> Self {
        Self {
            world: current.world.clone(),
            clock: current.clock.clone(),
            manufacturing: current.manufacturing.clone(),
            missions: current.missions.clone(),
            events: current.events.clone(),
            ai: current.ai.clone(),
            movement: current.movement.clone(),
            fog_alliance: current.fog_alliance.clone(),
            fog_empire: current.fog_empire.clone(),
            player_is_alliance: current.player_is_alliance,
            blockade: current.blockade.clone(),
            uprising: current.uprising.clone(),
            death_star: current.death_star.clone(),
            research: current.research.clone(),
            jedi: current.jedi.clone(),
            victory: current.victory.clone(),
            betrayal: current.betrayal.clone(),
            economy: current.economy.clone(),
            sim_rng: current.sim_rng.clone(),
            ai2: current.ai2.clone(),
            repair: (),
            combat_cooldowns: current.combat_cooldowns.clone(),
            game_config: current.game_config.clone(),
        }
    }
}

/// Exact v11 body. Repair state was still a zero-field unit struct; the
/// campaign configuration field was appended after the v10 body.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveStateV11 {
    world: GameWorld,
    clock: GameClock,
    manufacturing: ManufacturingState,
    missions: MissionState,
    events: EventState,
    ai: AIState,
    movement: MovementState,
    fog_alliance: FogState,
    fog_empire: FogState,
    player_is_alliance: bool,
    blockade: BlockadeState,
    uprising: UprisingState,
    death_star: DeathStarState,
    research: ResearchState,
    jedi: JediState,
    victory: VictoryState,
    betrayal: BetrayalState,
    economy: EconomyState,
    sim_rng: Xoshiro256PlusPlus,
    ai2: Option<AIState>,
    repair: (),
    #[serde(
        serialize_with = "rebellion_core::serde_ordered::serialize_hash_map",
        deserialize_with = "rebellion_core::serde_ordered::deserialize_hash_map"
    )]
    combat_cooldowns: std::collections::HashMap<SystemKey, u64>,
    game_config: GameConfig,
    campaign_config: CampaignConfig,
}

impl From<SaveStateV11> for SaveState {
    fn from(legacy: SaveStateV11) -> Self {
        Self {
            world: legacy.world,
            clock: legacy.clock,
            manufacturing: legacy.manufacturing,
            missions: legacy.missions,
            events: legacy.events,
            ai: legacy.ai,
            movement: legacy.movement,
            fog_alliance: legacy.fog_alliance,
            fog_empire: legacy.fog_empire,
            player_is_alliance: legacy.player_is_alliance,
            blockade: legacy.blockade,
            uprising: legacy.uprising,
            death_star: legacy.death_star,
            research: legacy.research,
            jedi: legacy.jedi,
            victory: legacy.victory,
            betrayal: legacy.betrayal,
            economy: legacy.economy,
            sim_rng: legacy.sim_rng,
            ai2: legacy.ai2,
            repair: RepairState::default(),
            combat_cooldowns: legacy.combat_cooldowns,
            game_config: legacy.game_config,
            campaign_config: legacy.campaign_config,
            troop_transport: TroopTransportState::default(),
        }
    }
}

impl From<&SaveState> for SaveStateV11 {
    fn from(current: &SaveState) -> Self {
        Self {
            world: current.world.clone(),
            clock: current.clock.clone(),
            manufacturing: current.manufacturing.clone(),
            missions: current.missions.clone(),
            events: current.events.clone(),
            ai: current.ai.clone(),
            movement: current.movement.clone(),
            fog_alliance: current.fog_alliance.clone(),
            fog_empire: current.fog_empire.clone(),
            player_is_alliance: current.player_is_alliance,
            blockade: current.blockade.clone(),
            uprising: current.uprising.clone(),
            death_star: current.death_star.clone(),
            research: current.research.clone(),
            jedi: current.jedi.clone(),
            victory: current.victory.clone(),
            betrayal: current.betrayal.clone(),
            economy: current.economy.clone(),
            sim_rng: current.sim_rng.clone(),
            ai2: current.ai2.clone(),
            repair: (),
            combat_cooldowns: current.combat_cooldowns.clone(),
            game_config: current.game_config.clone(),
            campaign_config: current.campaign_config.clone(),
        }
    }
}

/// Body layout shared by v8 and v9 saves. Bincode is positional, so legacy
/// bodies must be decoded into their exact historical shape before migration.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveStateV9 {
    world: GameWorld,
    clock: GameClock,
    manufacturing: ManufacturingState,
    missions: MissionState,
    events: EventState,
    ai: AIState,
    movement: MovementState,
    fog_alliance: FogState,
    fog_empire: FogState,
    player_is_alliance: bool,
    blockade: BlockadeState,
    uprising: UprisingState,
    death_star: DeathStarState,
    research: ResearchState,
    jedi: JediState,
    victory: VictoryState,
    betrayal: BetrayalState,
    economy: EconomyState,
}

impl From<SaveStateV9> for SaveState {
    fn from(legacy: SaveStateV9) -> Self {
        let campaign_config =
            CampaignConfig::from_legacy_world(&legacy.world, legacy.player_is_alliance);
        Self {
            world: legacy.world,
            clock: legacy.clock,
            manufacturing: legacy.manufacturing,
            missions: legacy.missions,
            events: legacy.events,
            ai: legacy.ai,
            movement: legacy.movement,
            fog_alliance: legacy.fog_alliance,
            fog_empire: legacy.fog_empire,
            player_is_alliance: legacy.player_is_alliance,
            blockade: legacy.blockade,
            uprising: legacy.uprising,
            death_star: legacy.death_star,
            research: legacy.research,
            jedi: legacy.jedi,
            victory: legacy.victory,
            betrayal: legacy.betrayal,
            economy: legacy.economy,
            sim_rng: Xoshiro256PlusPlus::seed_from_u64(0),
            ai2: None,
            repair: RepairState::default(),
            combat_cooldowns: std::collections::HashMap::new(),
            game_config: GameConfig::default(),
            campaign_config,
            troop_transport: TroopTransportState::default(),
        }
    }
}

impl From<&SaveState> for SaveStateV9 {
    fn from(current: &SaveState) -> Self {
        Self {
            world: current.world.clone(),
            clock: current.clock.clone(),
            manufacturing: current.manufacturing.clone(),
            missions: current.missions.clone(),
            events: current.events.clone(),
            ai: current.ai.clone(),
            movement: current.movement.clone(),
            fog_alliance: current.fog_alliance.clone(),
            fog_empire: current.fog_empire.clone(),
            player_is_alliance: current.player_is_alliance,
            blockade: current.blockade.clone(),
            uprising: current.uprising.clone(),
            death_star: current.death_star.clone(),
            research: current.research.clone(),
            jedi: current.jedi.clone(),
            victory: current.victory.clone(),
            betrayal: current.betrayal.clone(),
            economy: current.economy.clone(),
        }
    }
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
    /// Fingerprint of the canonical logical state.
    pub state_fingerprint: StateFingerprint,
    /// Whether the fingerprint was persisted and verified while loading.
    ///
    /// This is false for compatible v8 native saves, whose fingerprints are
    /// computed during load because that format did not store one.
    pub fingerprint_verified: bool,
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
            file.write_all(nb).context("writing mod name")?;
            let vb = mod_version.as_bytes();
            file.write_all(&(vb.len() as u32).to_le_bytes())
                .context("writing mod version length")?;
            file.write_all(vb).context("writing mod version")?;
        }
        let mod_hash = compute_mod_hash(active_mods);
        file.write_all(&mod_hash.to_le_bytes())
            .context("writing mod hash")?;

        // State fingerprint (v9+)
        file.write_all(&state_fingerprint.version.to_le_bytes())
            .context("writing state fingerprint version")?;
        file.write_all(&state_fingerprint.value.to_le_bytes())
            .context("writing state fingerprint")?;

        // ── Body ────────────────────────────────────────────────────────────
        file.write_all(&encoded).context("writing save body")?;

        Ok(state_fingerprint)
    }

    /// Convenience wrapper: save with no active mods.
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
        file.read_exact(&mut version_buf)
            .context("reading version")?;
        let version = u32::from_le_bytes(version_buf);

        // ── Version gate ────────────────────────────────────────────────────
        if version > SAVE_VERSION {
            anyhow::bail!(
                "save version {} is from a newer build (this build supports up to {})",
                version,
                SAVE_VERSION
            );
        }
        if version < MIN_MIGRATABLE_VERSION {
            anyhow::bail!(
                "save version {} is too old to migrate (minimum supported: {})",
                version,
                MIN_MIGRATABLE_VERSION
            );
        }

        // ── Name + timestamp (present in all versions) ──────────────────────
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

        // ── Mod metadata (v4+) ──────────────────────────────────────────────
        let (mod_names, mod_hash) = if version >= 4 {
            let mut count_buf = [0u8; 4];
            file.read_exact(&mut count_buf)
                .context("reading mod count")?;
            let mod_count = u32::from_le_bytes(count_buf) as usize;

            let mut names = Vec::with_capacity(mod_count);
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

        // ── State fingerprint (v9+) ────────────────────────────────────────
        let expected_fingerprint = if version >= 9 {
            let mut fingerprint_version_buf = [0u8; 2];
            file.read_exact(&mut fingerprint_version_buf)
                .context("reading state fingerprint version")?;
            let fingerprint_version = u16::from_le_bytes(fingerprint_version_buf);
            anyhow::ensure!(
                fingerprint_version == STATE_FINGERPRINT_VERSION,
                "unsupported state fingerprint version {} (this build supports {})",
                fingerprint_version,
                STATE_FINGERPRINT_VERSION
            );

            let mut fingerprint_buf = [0u8; 8];
            file.read_exact(&mut fingerprint_buf)
                .context("reading state fingerprint")?;
            Some(StateFingerprint {
                version: fingerprint_version,
                value: u64::from_le_bytes(fingerprint_buf),
            })
        } else {
            None
        };

        // ── Body ────────────────────────────────────────────────────────────
        let mut body = Vec::new();
        file.read_to_end(&mut body).context("reading save body")?;

        let (state, state_fingerprint, fingerprint_verified) = match version {
            SAVE_VERSION => {
                let state: SaveState =
                    bincode::deserialize(&body).context("deserializing save state")?;
                let fingerprint = compute_serializable_fingerprint_for_version(version, &state)?;
                if let Some(expected) = expected_fingerprint {
                    anyhow::ensure!(
                        expected == fingerprint,
                        "save state fingerprint mismatch: expected {}, computed {}",
                        expected,
                        fingerprint
                    );
                }
                (state, fingerprint, expected_fingerprint.is_some())
            }
            12 => {
                let legacy: SaveStateV12 =
                    bincode::deserialize(&body).context("deserializing v12 save state")?;
                if let Some(expected) = expected_fingerprint {
                    let legacy_fingerprint =
                        compute_serializable_fingerprint_for_version(version, &legacy)?;
                    anyhow::ensure!(
                        expected == legacy_fingerprint,
                        "save state fingerprint mismatch: expected {}, computed {}",
                        expected,
                        legacy_fingerprint
                    );
                }
                let state = SaveState::from(legacy);
                let fingerprint = compute_state_fingerprint(&state)?;
                (state, fingerprint, false)
            }
            11 => {
                let legacy: SaveStateV11 =
                    bincode::deserialize(&body).context("deserializing v11 save state")?;
                if let Some(expected) = expected_fingerprint {
                    let legacy_fingerprint =
                        compute_serializable_fingerprint_for_version(version, &legacy)?;
                    anyhow::ensure!(
                        expected == legacy_fingerprint,
                        "save state fingerprint mismatch: expected {}, computed {}",
                        expected,
                        legacy_fingerprint
                    );
                }
                let state = SaveState::from(legacy);
                let fingerprint = compute_state_fingerprint(&state)?;
                (state, fingerprint, false)
            }
            10 => {
                let legacy: SaveStateV10 =
                    bincode::deserialize(&body).context("deserializing v10 save state")?;
                if let Some(expected) = expected_fingerprint {
                    let legacy_fingerprint =
                        compute_serializable_fingerprint_for_version(version, &legacy)?;
                    anyhow::ensure!(
                        expected == legacy_fingerprint,
                        "save state fingerprint mismatch: expected {}, computed {}",
                        expected,
                        legacy_fingerprint
                    );
                }
                let state = SaveState::from(legacy);
                let fingerprint = compute_state_fingerprint(&state)?;
                (state, fingerprint, false)
            }
            9 | 8 => {
                let legacy: SaveStateV9 =
                    bincode::deserialize(&body).context("deserializing legacy save state")?;
                if let Some(expected) = expected_fingerprint {
                    let legacy_fingerprint =
                        compute_serializable_fingerprint_for_version(version, &legacy)?;
                    anyhow::ensure!(
                        expected == legacy_fingerprint,
                        "save state fingerprint mismatch: expected {}, computed {}",
                        expected,
                        legacy_fingerprint
                    );
                }
                let state = SaveState::from(legacy);
                let fingerprint = compute_state_fingerprint(&state)?;
                // Legacy bodies never persisted the complete continuation
                // envelope, so their migrated current-state fingerprint is
                // intentionally reported as unverified.
                (state, fingerprint, false)
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
            state_fingerprint,
            fingerprint_verified,
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
    /// Prefix is bumped per save format version so that stale browser entries
    /// from older builds get rejected cleanly instead of attempting an
    /// inoperable bincode deserialize.
    fn slot_key(slot: usize) -> String {
        format!("rebellion_save_v13_{}", slot)
    }

    fn v12_slot_key(slot: usize) -> String {
        format!("rebellion_save_v12_{}", slot)
    }

    fn v11_slot_key(slot: usize) -> String {
        format!("rebellion_save_v11_{}", slot)
    }

    fn v10_slot_key(slot: usize) -> String {
        format!("rebellion_save_v10_{}", slot)
    }

    fn v9_slot_key(slot: usize) -> String {
        format!("rebellion_save_v9_{}", slot)
    }

    /// localStorage key for versioned JSON save metadata.
    ///
    /// Prefix is bumped per save format version (see [`slot_key`]).
    fn meta_key(slot: usize) -> String {
        format!("rebellion_meta_v13_{}", slot)
    }

    fn v12_meta_key(slot: usize) -> String {
        format!("rebellion_meta_v12_{}", slot)
    }

    fn v11_meta_key(slot: usize) -> String {
        format!("rebellion_meta_v11_{}", slot)
    }

    fn v10_meta_key(slot: usize) -> String {
        format!("rebellion_meta_v10_{}", slot)
    }

    fn v9_meta_key(slot: usize) -> String {
        format!("rebellion_meta_v9_{}", slot)
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
        storage_set(&meta_key(slot), &meta)?;

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
        let (save_version, b64, meta_encoded) = if let Some(body) = storage_get(&slot_key(slot))? {
            let meta = storage_get(&meta_key(slot))?
                .ok_or_else(|| anyhow::anyhow!("save metadata missing for slot {}", slot))?;
            (SAVE_VERSION, body, meta)
        } else if let Some(body) = storage_get(&v12_slot_key(slot))? {
            let meta = storage_get(&v12_meta_key(slot))?
                .ok_or_else(|| anyhow::anyhow!("v12 save metadata missing for slot {}", slot))?;
            (12, body, meta)
        } else if let Some(body) = storage_get(&v11_slot_key(slot))? {
            let meta = storage_get(&v11_meta_key(slot))?
                .ok_or_else(|| anyhow::anyhow!("v11 save metadata missing for slot {}", slot))?;
            (11, body, meta)
        } else if let Some(body) = storage_get(&v10_slot_key(slot))? {
            let meta = storage_get(&v10_meta_key(slot))?
                .ok_or_else(|| anyhow::anyhow!("v10 save metadata missing for slot {}", slot))?;
            (10, body, meta)
        } else if let Some(body) = storage_get(&v9_slot_key(slot))? {
            let meta = storage_get(&v9_meta_key(slot))?
                .ok_or_else(|| anyhow::anyhow!("v9 save metadata missing for slot {}", slot))?;
            (9, body, meta)
        } else {
            anyhow::bail!("no save in slot {}", slot);
        };
        let bytes = b64_decode(&b64)?;
        let browser_meta = parse_meta(&meta_encoded)?;
        let expected_fingerprint = browser_meta.state_fingerprint.to_fingerprint()?;
        let (state, state_fingerprint, fingerprint_verified) = if save_version == SAVE_VERSION {
            let state: SaveState = bincode::deserialize(&bytes)?;
            let fingerprint = compute_state_fingerprint(&state)?;
            anyhow::ensure!(
                expected_fingerprint == fingerprint,
                "save state fingerprint mismatch: expected {}, computed {}",
                expected_fingerprint,
                fingerprint
            );
            (state, fingerprint, true)
        } else if save_version == 12 {
            let legacy: SaveStateV12 = bincode::deserialize(&bytes)?;
            let legacy_fingerprint =
                compute_serializable_fingerprint_for_version(save_version, &legacy)?;
            anyhow::ensure!(
                expected_fingerprint == legacy_fingerprint,
                "save state fingerprint mismatch: expected {}, computed {}",
                expected_fingerprint,
                legacy_fingerprint
            );
            let state = SaveState::from(legacy);
            let fingerprint = compute_state_fingerprint(&state)?;
            (state, fingerprint, false)
        } else if save_version == 11 {
            let legacy: SaveStateV11 = bincode::deserialize(&bytes)?;
            let legacy_fingerprint =
                compute_serializable_fingerprint_for_version(save_version, &legacy)?;
            anyhow::ensure!(
                expected_fingerprint == legacy_fingerprint,
                "save state fingerprint mismatch: expected {}, computed {}",
                expected_fingerprint,
                legacy_fingerprint
            );
            let state = SaveState::from(legacy);
            let fingerprint = compute_state_fingerprint(&state)?;
            (state, fingerprint, false)
        } else if save_version == 10 {
            let legacy: SaveStateV10 = bincode::deserialize(&bytes)?;
            let legacy_fingerprint =
                compute_serializable_fingerprint_for_version(save_version, &legacy)?;
            anyhow::ensure!(
                expected_fingerprint == legacy_fingerprint,
                "save state fingerprint mismatch: expected {}, computed {}",
                expected_fingerprint,
                legacy_fingerprint
            );
            let state = SaveState::from(legacy);
            let fingerprint = compute_state_fingerprint(&state)?;
            (state, fingerprint, false)
        } else {
            let legacy: SaveStateV9 = bincode::deserialize(&bytes)?;
            let legacy_fingerprint =
                compute_serializable_fingerprint_for_version(save_version, &legacy)?;
            anyhow::ensure!(
                expected_fingerprint == legacy_fingerprint,
                "save state fingerprint mismatch: expected {}, computed {}",
                expected_fingerprint,
                legacy_fingerprint
            );
            let state = SaveState::from(legacy);
            let fingerprint = compute_state_fingerprint(&state)?;
            (state, fingerprint, false)
        };

        let meta = SaveMeta {
            slot,
            name: browser_meta.name,
            timestamp_secs: 0, // no reliable clock in WASM
            game_tick: state.clock.tick,
            mod_names: vec![],
            mod_hash: compute_mod_hash(&[]),
            state_fingerprint,
            fingerprint_verified,
        };

        Ok((meta, state))
    }

    pub fn list_saves(_saves_dir: &Path) -> Vec<anyhow::Result<SaveMeta>> {
        (0..MAX_SAVE_SLOTS)
            .filter_map(|slot| {
                let encoded = match storage_get(&meta_key(slot)) {
                    Ok(Some(encoded)) => Ok(Some((encoded, true))),
                    Ok(None) => match storage_get(&v12_meta_key(slot)) {
                        Ok(Some(encoded)) => Ok(Some((encoded, false))),
                        Ok(None) => match storage_get(&v11_meta_key(slot)) {
                            Ok(Some(encoded)) => Ok(Some((encoded, false))),
                            Ok(None) => match storage_get(&v10_meta_key(slot)) {
                                Ok(Some(encoded)) => Ok(Some((encoded, false))),
                                Ok(None) => storage_get(&v9_meta_key(slot))
                                    .map(|legacy| legacy.map(|encoded| (encoded, false))),
                                Err(error) => Err(error),
                            },
                            Err(error) => Err(error),
                        },
                        Err(error) => Err(error),
                    },
                    Err(error) => Err(error),
                };
                match encoded {
                    Ok(None) => None,
                    Ok(Some((encoded, current))) => Some(parse_meta(&encoded).and_then(|meta| {
                        let state_fingerprint = meta.state_fingerprint.to_fingerprint()?;
                        Ok(SaveMeta {
                            slot,
                            name: meta.name,
                            timestamp_secs: 0,
                            game_tick: meta.game_tick,
                            mod_names: vec![],
                            mod_hash: compute_mod_hash(&[]),
                            state_fingerprint,
                            fingerprint_verified: current,
                        })
                    })),
                    Err(error) => Some(Err(error)),
                }
            })
            .collect()
    }

    pub fn delete_slot(_saves_dir: &Path, slot: usize) -> anyhow::Result<()> {
        storage_remove(&slot_key(slot))?;
        storage_remove(&meta_key(slot))?;
        storage_remove(&v12_slot_key(slot))?;
        storage_remove(&v12_meta_key(slot))?;
        storage_remove(&v11_slot_key(slot))?;
        storage_remove(&v11_meta_key(slot))?;
        storage_remove(&v10_slot_key(slot))?;
        storage_remove(&v10_meta_key(slot))?;
        storage_remove(&v9_slot_key(slot))?;
        storage_remove(&v9_meta_key(slot))?;
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
    use rand::RngCore;
    use rebellion_core::ai::{AIState, AiFaction};
    use rebellion_core::dat::Faction;
    use rebellion_core::world::ControlKind;
    use std::io::Write;

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
        file.write_all(&(name_bytes.len() as u32).to_le_bytes())
            .unwrap();
        file.write_all(name_bytes).unwrap();
        let timestamp: u64 = 1700000000; // fixed timestamp for reproducibility
        file.write_all(&timestamp.to_le_bytes()).unwrap();
        // No mod metadata — that's the v3 format
        let encoded = bincode::serialize(state).expect("serialize v3 body");
        file.write_all(&encoded).unwrap();
    }

    /// Write a save file with an arbitrary version number (for rejection tests).
    fn write_versioned_fixture(
        path: &std::path::Path,
        version: u32,
        name: &str,
        state: &SaveState,
    ) {
        let mut file = std::fs::File::create(path).expect("create versioned fixture");
        file.write_all(SAVE_MAGIC).unwrap();
        file.write_all(&version.to_le_bytes()).unwrap();
        let name_bytes = name.as_bytes();
        file.write_all(&(name_bytes.len() as u32).to_le_bytes())
            .unwrap();
        file.write_all(name_bytes).unwrap();
        let timestamp: u64 = 1700000000;
        file.write_all(&timestamp.to_le_bytes()).unwrap();
        if version >= 4 {
            file.write_all(&0u32.to_le_bytes()).unwrap(); // empty mod list
            file.write_all(&compute_mod_hash(&[]).to_le_bytes())
                .unwrap();
        }
        if version == 12 {
            let legacy = SaveStateV12::from(state);
            let fingerprint = compute_serializable_fingerprint_for_version(version, &legacy)
                .expect("fingerprint v12 body");
            file.write_all(&fingerprint.version.to_le_bytes()).unwrap();
            file.write_all(&fingerprint.value.to_le_bytes()).unwrap();
            let encoded = bincode::serialize(&legacy).expect("serialize v12 body");
            file.write_all(&encoded).unwrap();
            return;
        }
        if version == 11 {
            let legacy = SaveStateV11::from(state);
            let fingerprint = compute_serializable_fingerprint_for_version(version, &legacy)
                .expect("fingerprint v11 body");
            file.write_all(&fingerprint.version.to_le_bytes()).unwrap();
            file.write_all(&fingerprint.value.to_le_bytes()).unwrap();
            let encoded = bincode::serialize(&legacy).expect("serialize v11 body");
            file.write_all(&encoded).unwrap();
            return;
        }
        if version == 10 {
            let legacy = SaveStateV10::from(state);
            let fingerprint = compute_serializable_fingerprint_for_version(version, &legacy)
                .expect("fingerprint v10 body");
            file.write_all(&fingerprint.version.to_le_bytes()).unwrap();
            file.write_all(&fingerprint.value.to_le_bytes()).unwrap();
            let encoded = bincode::serialize(&legacy).expect("serialize v10 body");
            file.write_all(&encoded).unwrap();
            return;
        }

        let legacy = SaveStateV9::from(state);
        if version >= 9 {
            let fingerprint = compute_serializable_fingerprint_for_version(version, &legacy)
                .expect("fingerprint legacy body");
            file.write_all(&fingerprint.version.to_le_bytes()).unwrap();
            file.write_all(&fingerprint.value.to_le_bytes()).unwrap();
        }
        let encoded = bincode::serialize(&legacy).expect("serialize legacy body");
        file.write_all(&encoded).unwrap();
    }

    // ── Existing tests (updated for new save_slot signature) ────────────────

    #[test]
    fn round_trip_save_load() {
        let saves_dir = tmp_dir("round_trip_v5");

        let state = minimal_save_state();
        save_slot(&saves_dir, 0, "Test Save", &state, &[]).expect("save should succeed");

        let (meta, loaded) = load_slot(&saves_dir, 0).expect("load should succeed");

        assert_eq!(meta.slot, 0);
        assert_eq!(meta.name, "Test Save");
        assert_eq!(meta.game_tick, loaded.clock.tick);
        assert!(meta.mod_names.is_empty());
        assert!(meta.fingerprint_verified);
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
                is_alliance: false,
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

        let (meta, mut loaded) = load_slot(&saves_dir, 0).unwrap();
        let loaded_rolls = (0..8)
            .map(|_| loaded.sim_rng.next_u64())
            .collect::<Vec<_>>();

        assert!(meta.fingerprint_verified);
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
    fn fingerprint_changes_when_state_changes() {
        let original = minimal_save_state();
        let mut changed = original.clone();
        changed.clock.tick = 1;

        assert_ne!(
            compute_state_fingerprint(&original).unwrap(),
            compute_state_fingerprint(&changed).unwrap()
        );
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
        let saves_dir = tmp_dir("fingerprint_tamper_v9");
        const SAVE_NAME: &str = "Tamper Check";
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
    fn v8_save_loads_with_unverified_computed_fingerprint() {
        let saves_dir = tmp_dir("v8_fingerprint_compatibility");
        let state = minimal_save_state();
        let path = slot_path(&saves_dir, 0);
        write_versioned_fixture(&path, 8, "V8 Save", &state);

        let (meta, loaded) = load_slot(&saves_dir, 0).expect("v8 save should remain compatible");
        assert_eq!(loaded.clock.tick, state.clock.tick);
        assert_eq!(meta.state_fingerprint.version, STATE_FINGERPRINT_VERSION);
        assert!(!meta.fingerprint_verified);
    }

    #[test]
    fn v9_save_migrates_with_safe_continuation_defaults() {
        let saves_dir = tmp_dir("v9_continuation_compatibility");
        let state = minimal_save_state();
        let path = slot_path(&saves_dir, 0);
        write_versioned_fixture(&path, 9, "V9 Save", &state);

        let (meta, mut loaded) = load_slot(&saves_dir, 0).expect("v9 save should migrate");
        let mut default_rng = Xoshiro256PlusPlus::seed_from_u64(0);

        assert_eq!(loaded.clock.tick, state.clock.tick);
        assert_eq!(loaded.sim_rng.next_u64(), default_rng.next_u64());
        assert!(loaded.ai2.is_none());
        assert!(loaded.combat_cooldowns.is_empty());
        assert!(!meta.fingerprint_verified);
    }

    #[test]
    fn v10_save_migrates_campaign_setup_from_world() {
        let saves_dir = tmp_dir("v10_campaign_config_compatibility");
        let mut state = minimal_save_state();
        state.player_is_alliance = false;
        state.world.difficulty_index = 6;
        let path = slot_path(&saves_dir, 0);
        write_versioned_fixture(&path, 10, "V10 Save", &state);

        let (meta, loaded) = load_slot(&saves_dir, 0).expect("v10 save should migrate");
        assert_eq!(
            loaded.campaign_config.player_faction,
            rebellion_core::dat::Faction::Empire
        );
        assert_eq!(
            loaded.campaign_config.difficulty,
            rebellion_core::world::SeedDifficulty::Hard
        );
        assert_eq!(
            loaded.campaign_config.galaxy_size,
            rebellion_core::dat::GalaxySize::Standard
        );
        assert_eq!(
            loaded.campaign_config.victory_conditions,
            rebellion_core::world::VictoryConditions::Standard
        );
        assert!(!meta.fingerprint_verified);
    }

    #[test]
    fn v11_save_migrates_with_empty_repair_episode_state() {
        let saves_dir = tmp_dir("v11_repair_state_compatibility");
        let mut state = minimal_save_state();
        state.campaign_config.galaxy_size = rebellion_core::dat::GalaxySize::Huge;
        let path = slot_path(&saves_dir, 0);
        write_versioned_fixture(&path, 11, "V11 Save", &state);

        let (meta, loaded) = load_slot(&saves_dir, 0).expect("v11 save should migrate");
        assert_eq!(loaded.campaign_config, state.campaign_config);
        assert!(!meta.fingerprint_verified);
    }

    #[test]
    fn v12_save_migrates_with_empty_troop_transport_state() {
        let saves_dir = tmp_dir("v12_troop_transport_compatibility");
        let state = minimal_save_state();
        let path = slot_path(&saves_dir, 0);
        write_versioned_fixture(&path, 12, "V12 Save", &state);

        let (meta, loaded) = load_slot(&saves_dir, 0).expect("v12 save should migrate");
        assert!(loaded.troop_transport.is_empty());
        assert!(!meta.fingerprint_verified);
    }

    #[test]
    fn loads_v9_artifact_written_by_previous_release() {
        let saves_dir = tmp_dir("v9_historical_artifact");
        let path = slot_path(&saves_dir, 0);
        // Generated by commit 355715b's real v9 writer. Keeping the binary
        // fixture catches accidental drift in both bincode layout and the
        // historical fingerprint algorithm.
        std::fs::write(
            &path,
            include_bytes!("../tests/fixtures/v9-minimal-save.reb"),
        )
        .unwrap();

        let (meta, mut loaded) =
            load_slot(&saves_dir, 0).expect("the previously released v9 artifact must migrate");
        let mut default_rng = Xoshiro256PlusPlus::seed_from_u64(0);

        assert_eq!(meta.name, "Test Save");
        assert_eq!(loaded.sim_rng.next_u64(), default_rng.next_u64());
        assert!(loaded.ai2.is_none());
        assert!(loaded.combat_cooldowns.is_empty());
        assert!(!meta.fingerprint_verified);
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

        save_slot(&saves_dir, 0, "Modded", &state, &mods).expect("save with mods should succeed");

        let (meta, _) = load_slot(&saves_dir, 0).expect("load modded save should succeed");

        assert_eq!(meta.mod_names, vec!["TestMod".to_string()]);
        assert_eq!(meta.mod_hash, compute_mod_hash(&mods));
    }

    #[test]
    fn empty_mod_list_round_trip() {
        let saves_dir = tmp_dir("empty_mods");
        let state = minimal_save_state();

        save_slot(&saves_dir, 0, "No Mods", &state, &[]).expect("save with no mods should succeed");

        let (meta, _) = load_slot(&saves_dir, 0).expect("load should succeed");

        assert!(meta.mod_names.is_empty());
        assert_eq!(meta.mod_hash, compute_mod_hash(&[]));
    }

    #[test]
    fn future_version_rejected() {
        let saves_dir = tmp_dir("future_version");
        let state = minimal_save_state();
        let path = slot_path(&saves_dir, 0);
        write_versioned_fixture(&path, 99, "Future", &state);

        let err = load_slot(&saves_dir, 0).expect_err("future version should be rejected");

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
        let (meta, _) = load_slot(&saves_dir, 0).expect("mismatched mods should still load");

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
