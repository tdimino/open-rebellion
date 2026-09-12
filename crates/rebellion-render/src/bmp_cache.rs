//! BMP texture cache for DLL-extracted UI assets.
//!
//! Provides `BmpCache`, a lazy-loading egui texture registry keyed by
//! `(DllSource, resource_id)`.  Callers request a texture by DLL source and
//! numeric resource ID; on first access the cache locates the BMP on disk,
//! decodes it via the `image` crate, and registers it as an egui texture.
//! Subsequent calls return the cached `TextureHandle` immediately.
//!
//! # Path convention
//!
//! Original BMPs are staged as:
//! ```text
//! {base_path}/{dll-name}-dll/BMP/{resource_id}.bmp
//! ```
//! e.g. `data/base/ui/strategy-dll/BMP/10553.bmp` on native and
//! `web/data/ui/strategy-dll/BMP/10553.bmp` on WASM (staged by `build-wasm.sh`)
//!
//! Approved HD PNGs (optional) live at:
//! ```text
//! {hd_path}/{dll-name}/{resource_id}.png
//! ```
//! They are considered only when [`AssetRenderProfile::FaithfulHd`] is selected.
//! The default [`AssetRenderProfile::OriginalParity`] always renders the
//! original staged bytes with nearest-neighbor sampling.
//!
//! # WASM
//!
//! On `wasm32` targets, BMP bytes are loaded from the single-request runtime
//! asset pack (or the loose-file development fallback) into a static
//! `WASM_BMP_CACHE`. The WASM `load_texture()` reads from this cache, decodes
//! only requested images via `image::load_from_memory()`, and registers the
//! result as an egui texture — identical to the native path minus filesystem
//! I/O.
//!
//! Call [`set_bmp_cache()`] from the app's WASM loading screen after unpacking
//! the runtime asset bytes, before the game loop starts.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use egui_macroquad::egui::{self, TextureHandle, TextureOptions};
use macroquad::prelude::{FilterMode, Texture2D};
#[cfg(not(target_arch = "wasm32"))]
use serde::Deserialize;
#[cfg(not(target_arch = "wasm32"))]
use sha2::{Digest, Sha256};

#[cfg(target_arch = "wasm32")]
const DATA_PREFIX: &str = "web/data/base";
#[cfg(not(target_arch = "wasm32"))]
const DATA_PREFIX: &str = "data/base";

#[cfg(target_arch = "wasm32")]
const HD_PREFIX: &str = "web/data/hd";
#[cfg(not(target_arch = "wasm32"))]
const HD_PREFIX: &str = "data/hd";

// ---------------------------------------------------------------------------
// WASM BMP byte cache (mirrors WASM_FILE_CACHE in rebellion-data/src/lib.rs)
// ---------------------------------------------------------------------------

/// Static cache of pre-fetched BMP/PNG bytes for WASM builds.
///
/// Keys are `"{dll-dir-name}/{resource_id}"` — e.g. `"strategy-dll/10553"`.
/// Values are the raw file bytes (BMP or PNG).
#[cfg(target_arch = "wasm32")]
static WASM_BMP_CACHE: std::sync::LazyLock<std::sync::Mutex<HashMap<String, Vec<u8>>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// Pre-load BMP/PNG bytes for WASM.  Call from the loading screen after
/// fetching UI assets via HTTP, before the game loop starts.
///
/// The map key format is `"{dll-dir-name}/{resource_id}"` — e.g.
/// `"strategy-dll/10553"` for a BMP, or `"hd/strategy-dll/10553"` for an
/// HD PNG override.
#[cfg(target_arch = "wasm32")]
pub fn set_bmp_cache(cache: HashMap<String, Vec<u8>>) {
    *WASM_BMP_CACHE.lock().unwrap() = cache;
}

/// Look up pre-fetched bytes for a single BMP/PNG resource on WASM.
#[cfg(target_arch = "wasm32")]
fn get_bmp_bytes(dll_dir: &str, resource_id: u32) -> Option<Vec<u8>> {
    let key = format!("{}/{}", dll_dir, resource_id);
    WASM_BMP_CACHE.lock().unwrap().get(&key).cloned()
}

/// Look up pre-fetched HD PNG override bytes on WASM.
#[cfg(target_arch = "wasm32")]
fn get_hd_bytes(dll_dir: &str, resource_id: u32) -> Option<Vec<u8>> {
    let key = format!("hd/{}/{}", dll_dir, resource_id);
    WASM_BMP_CACHE.lock().unwrap().get(&key).cloned()
}

// ---------------------------------------------------------------------------
// Render profile
// ---------------------------------------------------------------------------

/// Selects whether the renderer may substitute reviewed HD assets.
///
/// Original parity is intentionally the default. Merely placing a PNG in
/// `data/hd` must never change parity screenshots or release acceptance.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AssetRenderProfile {
    /// Decode the original resources and sample them without interpolation.
    #[default]
    OriginalParity,
    /// Prefer manifest-approved HD resources, falling back to the originals.
    FaithfulHd,
}

impl AssetRenderProfile {
    /// Stable configuration value used by the native environment variable and
    /// future browser-pack manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginalParity => "original-parity",
            Self::FaithfulHd => "faithful-hd",
        }
    }

    /// Parse a stable profile value. Unknown values fail closed at the caller.
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "original" | "original-parity" | "parity" => Some(Self::OriginalParity),
            "faithful-hd" | "hd" => Some(Self::FaithfulHd),
            _ => None,
        }
    }

    const fn allows_hd(self) -> bool {
        matches!(self, Self::FaithfulHd)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AssetVariant {
    Original,
    FaithfulHd,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Deserialize)]
struct HdApprovalManifest {
    schema_version: u32,
    profile: String,
    assets: HashMap<String, HdApprovalRecord>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Deserialize)]
struct HdApprovalRecord {
    approved: bool,
    review: Option<HdApprovalReview>,
    gates: Option<HdApprovalGates>,
    source: Option<HdApprovalSource>,
    output: Option<HdApprovalOutput>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Deserialize)]
struct HdApprovalReview {
    reviewer: String,
    evidence: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Deserialize)]
struct HdApprovalGates {
    human_review: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Deserialize)]
struct HdApprovalSource {
    sha256: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Deserialize)]
struct HdApprovalOutput {
    sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ApprovedHdAsset {
    source_sha256: String,
    output_sha256: String,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn approved_hd_assets_from_bytes(
    bytes: &[u8],
) -> Result<HashMap<String, ApprovedHdAsset>, String> {
    let manifest: HdApprovalManifest =
        serde_json::from_slice(bytes).map_err(|error| format!("invalid HD manifest: {error}"))?;
    if manifest.schema_version != 1 {
        return Err(format!(
            "unsupported HD manifest schema {}",
            manifest.schema_version
        ));
    }
    if manifest.profile != AssetRenderProfile::FaithfulHd.as_str() {
        return Err(format!(
            "HD manifest profile {:?} is not faithful-hd",
            manifest.profile
        ));
    }

    let mut approved = HashMap::new();
    for (key, record) in manifest.assets {
        if !record.approved {
            continue;
        }
        let review = record
            .review
            .ok_or_else(|| format!("approved HD asset {key:?} has no review record"))?;
        if review.reviewer.trim().is_empty() || review.evidence.trim().is_empty() {
            return Err(format!(
                "approved HD asset {key:?} has incomplete review metadata"
            ));
        }
        let gates = record
            .gates
            .ok_or_else(|| format!("approved HD asset {key:?} has no gate record"))?;
        if gates.human_review != "pass" {
            return Err(format!(
                "approved HD asset {key:?} has not passed human review"
            ));
        }
        let source = record
            .source
            .ok_or_else(|| format!("approved HD asset {key:?} has no source record"))?;
        let source_digest = source.sha256.to_ascii_lowercase();
        if !is_sha256(&source_digest) {
            return Err(format!(
                "approved HD asset {key:?} has an invalid source SHA-256"
            ));
        }
        let output = record
            .output
            .ok_or_else(|| format!("approved HD asset {key:?} has no output record"))?;
        let output_digest = output.sha256.to_ascii_lowercase();
        if !is_sha256(&output_digest) {
            return Err(format!(
                "approved HD asset {key:?} has an invalid output SHA-256"
            ));
        }
        approved.insert(
            key,
            ApprovedHdAsset {
                source_sha256: source_digest,
                output_sha256: output_digest,
            },
        );
    }
    Ok(approved)
}

#[cfg(not(target_arch = "wasm32"))]
fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn load_approved_hd_assets(hd_root: &Path) -> HashMap<String, ApprovedHdAsset> {
    let manifest_path = hd_root.join("manifest.json");
    match std::fs::read(&manifest_path) {
        Ok(bytes) => match approved_hd_assets_from_bytes(&bytes) {
            Ok(assets) => assets,
            Err(error) => {
                eprintln!(
                    "[bmp_cache] ignoring HD manifest path={} error={}",
                    manifest_path.display(),
                    error
                );
                HashMap::new()
            }
        },
        Err(_) => HashMap::new(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_verified_file(path: &Path, expected_sha256: &str) -> Option<Vec<u8>> {
    let Ok(bytes) = std::fs::read(path) else {
        return None;
    };
    let actual = format!("{:x}", Sha256::digest(&bytes));
    (actual == expected_sha256).then_some(bytes)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn validated_hd_bytes(
    source_path: &Path,
    output_path: &Path,
    approval: &ApprovedHdAsset,
) -> Option<Vec<u8>> {
    read_verified_file(source_path, &approval.source_sha256)?;
    read_verified_file(output_path, &approval.output_sha256)
}

impl AssetVariant {
    const fn texture_options(self) -> TextureOptions {
        match self {
            Self::Original => TextureOptions::NEAREST,
            Self::FaithfulHd => TextureOptions::LINEAR,
        }
    }
}

// ---------------------------------------------------------------------------
// DllSource
// ---------------------------------------------------------------------------

/// Which DLL a UI resource comes from.
///
/// Maps directly to the `{dll-name}-dll/` staging directory name used by
/// `scripts/stage-ui-assets.py`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DllSource {
    /// `STRATEGY.DLL` — galaxy map chrome, character panels, event screens
    Strategy,
    /// `COMMON.DLL` — global buttons, sliders, main-menu backgrounds
    Common,
    /// `TACTICAL.DLL` — combat HUD, ship sprites, squadron controls
    Tactical,
    /// `GOKRES.DLL` — entity status sprites, character portraits, ship icons
    Gokres,
}

impl DllSource {
    /// Lowercase DLL name used as the staging directory prefix.
    ///
    /// Staging layout: `{base_path}/{dll_dir_name}/BMP/{id}.bmp`
    pub fn dll_dir_name(self) -> &'static str {
        match self {
            DllSource::Strategy => "strategy-dll",
            DllSource::Common => "common-dll",
            DllSource::Tactical => "tactical-dll",
            DllSource::Gokres => "gokres-dll",
        }
    }

    /// Egui texture name prefix (for debug labels).
    pub fn texture_prefix(self) -> &'static str {
        match self {
            DllSource::Strategy => "strategy",
            DllSource::Common => "common",
            DllSource::Tactical => "tactical",
            DllSource::Gokres => "gokres",
        }
    }
}

// ---------------------------------------------------------------------------
// Named resource IDs
// ---------------------------------------------------------------------------

/// Named BMP resource IDs for frequently used DLL assets.
///
/// Names are derived from the extracted DLL inventories in
/// `agent_docs/dll-resource-catalog.md`, the full per-DLL indexes under
/// `assets/references/ref-ui-full/`, and the curated reference filenames under
/// `assets/references/ref-ui/`.
pub mod resources {
    /// Resource IDs for `COMMON.DLL` BMPs.
    ///
    /// Covers the global main-menu background and shared interface resources.
    pub mod common {
        /// Main title-screen background.
        pub const MAIN_MENU_BG: u32 = 20001;

        /// First animated shuttle-cockpit control frame.
        pub const MAIN_MENU_ANIMATION_FIRST: u32 = 11001;
        /// Last shuttle-cockpit control/selection frame.
        pub const MAIN_MENU_ANIMATION_LAST: u32 = 11275;

        /// Main-menu button: restart the game.
        pub const BTN_RESTART_GAME_NORMAL: u32 = 10035;
        /// Main-menu button: restart the game (pressed).
        pub const BTN_RESTART_GAME_PRESSED: u32 = 10036;
        /// Main-menu button: restart the game (disabled).
        pub const BTN_RESTART_GAME_DISABLED: u32 = 10037;
    }

    /// Resource IDs for `STRATEGY.DLL` BMPs.
    ///
    /// Covers galaxy-map backgrounds, panel chrome, and the most common event
    /// screens surfaced by the current render layer and curated reference set.
    pub mod strategy {
        /// Alliance strategic command-center shell.
        pub const ALLIANCE_COMMAND_CENTER_SHELL: u32 = 900;
        /// Imperial strategic command-center shell.
        pub const EMPIRE_COMMAND_CENTER_SHELL: u32 = 901;
        /// Bright authored galaxy starfield used only when GID display is off.
        pub const GALAXY_STARFIELD_BRIGHT: u32 = 902;
        /// Dim authored galaxy starfield used by every active GID mode.
        pub const GALAXY_STARFIELD_DIM: u32 = 903;

        /// Alliance System Finder, pressed.
        pub const ALLIANCE_SYSTEM_FINDER_PRESSED: u32 = 10001;
        /// Alliance System Finder, normal.
        pub const ALLIANCE_SYSTEM_FINDER_NORMAL: u32 = 10002;
        /// Alliance Fleet Finder, pressed.
        pub const ALLIANCE_FLEET_FINDER_PRESSED: u32 = 10003;
        /// Alliance Fleet Finder, normal.
        pub const ALLIANCE_FLEET_FINDER_NORMAL: u32 = 10004;
        /// Alliance Personnel Finder, pressed.
        pub const ALLIANCE_PERSONNEL_FINDER_PRESSED: u32 = 10005;
        /// Alliance Personnel Finder, normal.
        pub const ALLIANCE_PERSONNEL_FINDER_NORMAL: u32 = 10006;
        /// Alliance Troop Finder, pressed.
        pub const ALLIANCE_TROOP_FINDER_PRESSED: u32 = 10007;
        /// Alliance Troop Finder, normal.
        pub const ALLIANCE_TROOP_FINDER_NORMAL: u32 = 10008;
        /// Alliance Game Options, pressed.
        pub const ALLIANCE_GAME_OPTIONS_PRESSED: u32 = 10009;
        /// Alliance Game Options, normal.
        pub const ALLIANCE_GAME_OPTIONS_NORMAL: u32 = 10010;
        /// Alliance Encyclopedia, pressed.
        pub const ALLIANCE_ENCYCLOPEDIA_PRESSED: u32 = 10011;
        /// Alliance Encyclopedia, normal.
        pub const ALLIANCE_ENCYCLOPEDIA_NORMAL: u32 = 10012;
        /// Alliance Galactic Information Display, normal.
        pub const ALLIANCE_GID_NORMAL: u32 = 10013;
        /// Alliance Galactic Information Display, pressed.
        pub const ALLIANCE_GID_PRESSED: u32 = 10014;

        /// Imperial System Finder, pressed.
        pub const EMPIRE_SYSTEM_FINDER_PRESSED: u32 = 10015;
        /// Imperial System Finder, normal.
        pub const EMPIRE_SYSTEM_FINDER_NORMAL: u32 = 10016;
        /// Imperial Fleet Finder, pressed.
        pub const EMPIRE_FLEET_FINDER_PRESSED: u32 = 10017;
        /// Imperial Fleet Finder, normal.
        pub const EMPIRE_FLEET_FINDER_NORMAL: u32 = 10018;
        /// Imperial Personnel Finder, pressed.
        pub const EMPIRE_PERSONNEL_FINDER_PRESSED: u32 = 10019;
        /// Imperial Personnel Finder, normal.
        pub const EMPIRE_PERSONNEL_FINDER_NORMAL: u32 = 10020;
        /// Imperial Troop Finder, pressed.
        pub const EMPIRE_TROOP_FINDER_PRESSED: u32 = 10021;
        /// Imperial Troop Finder, normal.
        pub const EMPIRE_TROOP_FINDER_NORMAL: u32 = 10022;
        /// Imperial Game Options, pressed.
        pub const EMPIRE_GAME_OPTIONS_PRESSED: u32 = 10023;
        /// Imperial Game Options, normal.
        pub const EMPIRE_GAME_OPTIONS_NORMAL: u32 = 10024;
        /// Imperial Encyclopedia, pressed.
        pub const EMPIRE_ENCYCLOPEDIA_PRESSED: u32 = 10025;
        /// Imperial Encyclopedia, normal.
        pub const EMPIRE_ENCYCLOPEDIA_NORMAL: u32 = 10026;
        /// Imperial Galactic Information Display, normal.
        pub const EMPIRE_GID_NORMAL: u32 = 10027;
        /// Imperial Galactic Information Display, pressed.
        pub const EMPIRE_GID_PRESSED: u32 = 10028;

        /// Code-built GID menu frame tiles, checkmarks, category icons, and arrows.
        pub const GID_FRAME_FIRST: u32 = 10100;
        pub const GID_FRAME_LAST: u32 = 10107;
        pub const GID_CHECK_ALLIANCE: u32 = 10108;
        pub const GID_CHECK_EMPIRE: u32 = 10109;
        pub const GID_ALLIANCE_CATEGORY_FIRST: u32 = 10110;
        pub const GID_ALLIANCE_CATEGORY_LAST: u32 = 10115;
        pub const GID_ALLIANCE_ARROW: u32 = 10117;
        pub const GID_EMPIRE_CATEGORY_FIRST: u32 = 10119;
        pub const GID_EMPIRE_CATEGORY_LAST: u32 = 10124;
        pub const GID_EMPIRE_ARROW: u32 = 10128;
        pub const GID_SHIPYARDS: u32 = 10131;
        pub const GID_TRAINING_FACILITIES: u32 = 10132;
        pub const GID_CONSTRUCTION_YARDS: u32 = 10133;
        pub const GID_AVAILABLE_ENERGY: u32 = 10134;
        pub const GID_AVAILABLE_RAW_MATERIAL: u32 = 10135;
        pub const GID_MINES: u32 = 10136;
        pub const GID_REFINERIES: u32 = 10137;
        pub const GID_IDLE_PERSONNEL_ALLIANCE: u32 = 10138;
        pub const GID_ACTIVE_PERSONNEL_ALLIANCE: u32 = 10140;
        pub const GID_FIGHTER_SQUADRONS_ALLIANCE: u32 = 10141;
        pub const GID_DEATH_STAR_SHIELDS: u32 = 10142;
        pub const GID_PLANETARY_SHIELDS_ALLIANCE: u32 = 10143;

        /// Alliance GID system markers, largest through smallest.
        pub const GID_ALLIANCE_LARGEST: u32 = 10146;
        pub const GID_ALLIANCE_LARGE: u32 = 10147;
        pub const GID_ALLIANCE_MEDIUM: u32 = 10148;
        pub const GID_ALLIANCE_SMALLEST: u32 = 10149;
        /// Imperial GID system markers, smallest through largest.
        pub const GID_EMPIRE_SMALLEST: u32 = 10150;
        pub const GID_EMPIRE_MEDIUM: u32 = 10151;
        pub const GID_EMPIRE_LARGE: u32 = 10152;
        pub const GID_EMPIRE_LARGEST: u32 = 10153;
        /// Neutral GID system markers, smallest through largest.
        pub const GID_NEUTRAL_SMALLEST: u32 = 10154;
        pub const GID_NEUTRAL_MEDIUM: u32 = 10155;
        pub const GID_NEUTRAL_LARGE: u32 = 10156;
        pub const GID_NEUTRAL_LARGEST: u32 = 10157;
        /// Unexplored or unpopulated GID system marker.
        pub const GID_UNEXPLORED: u32 = 10158;
        /// Additional native GID selection/special marker, not yet assigned.
        pub const GID_SPECIAL: u32 = 10166;
        /// Compact strategic-map legend control.
        pub const GID_COMPACT_LEGEND: u32 = 10168;
        pub const GID_MULTI_FACTION_FLEET: u32 = 10160;
        pub const GID_SELECTION: u32 = 10166;
        pub const GID_HOVER: u32 = 10167;
        pub const GID_SPECIAL_ALLIANCE: u32 = 10169;
        pub const GID_SPECIAL_EMPIRE: u32 = 10170;
        pub const GID_UPRISING_ALLIANCE: u32 = 11608;
        pub const GID_UPRISING_EMPIRE: u32 = 11609;
        pub const GID_PLANETARY_BATTERIES: u32 = 11610;
        pub const GID_FLEETS_EN_ROUTE_ALLIANCE: u32 = 11613;
        pub const GID_FLEETS_EN_ROUTE_EMPIRE: u32 = 11614;

        /// Original sector-window planet pictures 1 through 23.
        pub const SECTOR_PLANET_FIRST: u32 = 10212;
        pub const SECTOR_PLANET_LAST: u32 = 10234;
        /// Non-contiguous sector-window planet pictures 25, 26, and 24.
        pub const SECTOR_PLANET_SPECIAL_FIRST: u32 = 10237;
        pub const SECTOR_PLANET_SPECIAL_LAST: u32 = 10239;

        /// Generic strategy UI frame variant A.
        pub const UI_PANEL_FRAME_A: u32 = 10553;
        /// Generic strategy UI frame variant B.
        pub const UI_PANEL_FRAME_B: u32 = 10554;
        /// Generic strategy UI frame variant C.
        pub const UI_PANEL_FRAME_C: u32 = 10555;

        /// Event screen: informants provide information.
        pub const EVENT_INFORMANTS_PROVIDE_INFORMATION: u32 = 1000;
        /// Event screen: natural disaster.
        pub const EVENT_NATURAL_DISASTER: u32 = 1003;
        /// Event screen: smuggling losses or benefits.
        pub const EVENT_SMUGGLING_LOSSES_OR_BENEFITS: u32 = 1004;
        /// Event screen: planet allegiance evolves (Alliance).
        pub const EVENT_PLANET_ALLEGIANCE_EVOLVES_ALLIANCE: u32 = 1005;
        /// Event screen: planet near uprising (Empire).
        pub const EVENT_PLANET_NEAR_UPRISING_EMPIRE: u32 = 1009;
        /// Event screen: uprising begins on planet.
        pub const EVENT_UPRISING_BEGINS_ON_PLANET: u32 = 1010;
        /// Event screen: uprising ends on planet (Empire).
        pub const EVENT_UPRISING_ENDS_ON_PLANET_EMPIRE: u32 = 1012;
        /// Event screen: maintenance shortfall / saboteurs strike (Alliance).
        pub const EVENT_MAINTENANCE_SHORTFALL_ALLIANCE: u32 = 1013;
        /// Event screen: maintenance shortfall / saboteurs strike (Empire).
        pub const EVENT_MAINTENANCE_SHORTFALL_EMPIRE: u32 = 1014;
        /// Event screen: fleet arrives at planet (Alliance).
        pub const EVENT_FLEET_ARRIVES_AT_PLANET_ALLIANCE: u32 = 1018;
        /// Event screen: fleet arrives at planet (Empire).
        pub const EVENT_FLEET_ARRIVES_AT_PLANET_EMPIRE: u32 = 1019;
        /// Event screen: units arrive (Alliance).
        pub const EVENT_UNITS_ARRIVE_ALLIANCE: u32 = 1020;
        /// Event screen: units arrive (Empire).
        pub const EVENT_UNITS_ARRIVE_EMPIRE: u32 = 1021;
        /// Event screen: headquarters arrive (Alliance).
        pub const EVENT_HEADQUARTERS_ARRIVE_ALLIANCE: u32 = 1022;
        /// Event screen: fleet initiates blockade of planet (Alliance).
        pub const EVENT_BLOCKADE_INITIATED_ALLIANCE: u32 = 1027;
        /// Event screen: fleet initiates blockade of planet (Empire).
        pub const EVENT_BLOCKADE_INITIATED_EMPIRE: u32 = 1028;
        /// Event screen: blockade breach fails (Empire).
        pub const EVENT_BLOCKADE_BREACH_FAILS_EMPIRE: u32 = 1030;
        /// Event screen: blockade breach fails (Alliance).
        pub const EVENT_BLOCKADE_BREACH_FAILS_ALLIANCE: u32 = 1031;
        /// Event screen: unit or facility decommissioned (Alliance).
        pub const EVENT_UNIT_FACILITY_DECOMMISSION_ALLIANCE: u32 = 1032;
        /// Event screen: unit or facility decommissioned (Empire).
        pub const EVENT_UNIT_FACILITY_DECOMMISSION_EMPIRE: u32 = 1033;
        /// Event screen: character retirement (Alliance).
        pub const EVENT_CHARACTER_RETIREMENT_ALLIANCE: u32 = 1034;
        /// Event screen: character retirement (Empire).
        pub const EVENT_CHARACTER_RETIREMENT_EMPIRE: u32 = 1035;
        /// Event screen: multiplayer chat (Alliance).
        pub const EVENT_MULTIPLAYER_CHAT_ALLIANCE: u32 = 1040;
        /// Event screen: multiplayer chat (Empire).
        pub const EVENT_MULTIPLAYER_CHAT_EMPIRE: u32 = 1041;
        /// Event screen: recruitment mission report / Jedi discovered (Alliance).
        pub const EVENT_RECRUITMENT_REPORT_ALLIANCE: u32 = 1042;
        /// Event screen: recruitment mission report (Empire).
        pub const EVENT_RECRUITMENT_REPORT_EMPIRE: u32 = 1043;
        /// Event screen: diplomacy mission report.
        pub const EVENT_DIPLOMACY_REPORT: u32 = 1044;
        /// Event screen: espionage mission report.
        pub const EVENT_ESPIONAGE_REPORT: u32 = 1045;
        /// Event screen: incite uprising foiled (Alliance).
        pub const EVENT_INCITE_UPRISING_FOILED_ALLIANCE: u32 = 1046;
        /// Event screen: incite uprising foiled (Empire).
        pub const EVENT_INCITE_UPRISING_FOILED_EMPIRE: u32 = 1047;
        /// Event screen: Rebel character captured.
        pub const EVENT_REBEL_CHARACTER_CAPTURED: u32 = 1048;
        /// Event screen: Empire character captured.
        pub const EVENT_EMPIRE_CHARACTER_CAPTURED: u32 = 1049;
        /// Event screen: character injured.
        pub const EVENT_CHARACTER_INJURED: u32 = 1050;
        /// Event screen: character recovered.
        pub const EVENT_CHARACTER_RECOVERED: u32 = 1051;
        /// Event screen: bounty hunters defeated.
        pub const EVENT_BOUNTY_HUNTERS_DEFEATED: u32 = 1052;
        /// Event screen: Jabba captures Solo.
        pub const EVENT_JABBA_CAPTURES_SOLO: u32 = 1053;
        /// Event screen: bounty hunters locate Solo.
        pub const EVENT_BOUNTY_HUNTERS_LOCATE_SOLO: u32 = 1054;
        /// Event screen: Vader vs Leia.
        pub const EVENT_VADER_VS_LEIA: u32 = 1055;
        /// Event screen: Emperor vs Leia.
        pub const EVENT_EMPEROR_VS_LEIA: u32 = 1056;
        /// Event screen: Luke travels to Dagobah.
        pub const EVENT_LUKE_TRAVELS_DAGOBAH: u32 = 1057;
        /// Event screen: Luke discovers his heritage.
        pub const EVENT_LUKE_DISCOVERS_HERITAGE: u32 = 1058;
        /// Event screen: Vader vs student Luke.
        pub const EVENT_VADER_VS_STUDENT_LUKE: u32 = 1059;
        /// Event screen: Vader vs Jedi Knight Luke.
        pub const EVENT_VADER_VS_KNIGHT_LUKE: u32 = 1060;
        /// Event screen: Emperor vs student Luke.
        pub const EVENT_EMPEROR_VS_STUDENT_LUKE: u32 = 1061;
        /// Event screen: Emperor vs Jedi Knight Luke.
        pub const EVENT_EMPEROR_VS_KNIGHT_LUKE: u32 = 1062;
        /// Event screen: Emperor and Vader vs Jedi Knight Luke.
        pub const EVENT_EMPEROR_AND_VADER_VS_KNIGHT_LUKE: u32 = 1064;
        /// Event screen: Jabba vs Luke.
        pub const EVENT_JABBA_VS_LUKE: u32 = 1065;
        /// Event screen: Emperor arrives on Coruscant.
        pub const EVENT_EMPEROR_ARRIVES_CORUSCANT: u32 = 1068;
        /// Event screen: character killed (Alliance).
        pub const EVENT_CHARACTER_KILLED_ALLIANCE: u32 = 1070;
        /// Event screen: enemy mission foiled (Alliance).
        pub const EVENT_ENEMY_MISSION_FOILED_ALLIANCE: u32 = 1073;
        /// Event screen: enemy mission foiled (Empire).
        pub const EVENT_ENEMY_MISSION_FOILED_EMPIRE: u32 = 1074;
        /// Event screen: character killed (Empire).
        pub const EVENT_CHARACTER_KILLED_EMPIRE: u32 = 1075;

        /// Event screen: battle at planet, Alliance fleet defeated.
        pub const EVENT_BATTLE_ALLIANCE_DEFEATED: u32 = 10757;
        /// Event screen: battle at planet, Empire fleet defeated.
        pub const EVENT_BATTLE_EMPIRE_DEFEATED: u32 = 10758;
        /// Event screen: battle at planet, Alliance fleet victorious.
        pub const EVENT_BATTLE_ALLIANCE_VICTORY: u32 = 10759;
        /// Event screen: battle at planet, Empire fleet victorious.
        pub const EVENT_BATTLE_EMPIRE_VICTORY: u32 = 10760;

        /// Event screen: assault on planet (Alliance).
        pub const EVENT_ASSAULT_ON_PLANET_ALLIANCE: u32 = 11160;
        /// Event screen: assault on planet (Empire).
        pub const EVENT_ASSAULT_ON_PLANET_EMPIRE: u32 = 11161;
        /// Event screen: orbital bombardment of planet, variant A.
        pub const EVENT_ORBITAL_BOMBARDMENT_A: u32 = 11162;
        /// Event screen: orbital bombardment of planet, variant B.
        pub const EVENT_ORBITAL_BOMBARDMENT_B: u32 = 11163;
    }

    /// Resource IDs for `TACTICAL.DLL` BMPs.
    ///
    /// Covers tactical HUD panels, command buttons, Death Star controls, and
    /// weapon recharge gauges.
    pub mod tactical {
        /// Full tactical background.
        pub const BACKGROUND: u32 = 1000;

        /// Task forces HUD panel (Alliance).
        pub const TASK_FORCES_ALLIANCE: u32 = 1001;
        /// Task forces HUD panel (Empire).
        pub const TASK_FORCES_EMPIRE: u32 = 1004;

        /// Task-force button, normal state.
        pub const BTN_TASK_FORCE_NORMAL: u32 = 1005;
        /// Task-force button, pressed state.
        pub const BTN_TASK_FORCE_PRESSED: u32 = 1006;
        /// Task-force button, unassigned state.
        pub const BTN_TASK_FORCE_UNASSIGNED: u32 = 1007;

        /// Fighter squadrons HUD panel (Alliance).
        pub const FIGHTER_SQUADRONS_ALLIANCE: u32 = 1008;
        /// Fighter squadrons HUD panel (Empire).
        pub const FIGHTER_SQUADRONS_EMPIRE: u32 = 1010;

        /// Squadron button: red, normal state.
        pub const BTN_RED_SQUADRON_NORMAL: u32 = 1012;
        /// Squadron button: blue, normal state.
        pub const BTN_BLUE_SQUADRON_NORMAL: u32 = 1013;
        /// Squadron button: green, normal state.
        pub const BTN_GREEN_SQUADRON_NORMAL: u32 = 1014;
        /// Squadron button: gold, normal state.
        pub const BTN_GOLD_SQUADRON_NORMAL: u32 = 1015;
        /// Squadron button: red, pressed state.
        pub const BTN_RED_SQUADRON_PRESSED: u32 = 1016;
        /// Squadron button: blue, pressed state.
        pub const BTN_BLUE_SQUADRON_PRESSED: u32 = 1017;
        /// Squadron button: green, pressed state.
        pub const BTN_GREEN_SQUADRON_PRESSED: u32 = 1018;
        /// Squadron button: gold, pressed state.
        pub const BTN_GOLD_SQUADRON_PRESSED: u32 = 1019;
        /// Squadron HUD marker: unassigned.
        pub const SQUADRON_UNASSIGNED: u32 = 1020;

        /// Death Star laser control: ready.
        pub const DEATH_STAR_LASER_READY: u32 = 1021;
        /// Death Star laser control: fired.
        pub const DEATH_STAR_LASER_FIRED: u32 = 1022;
        /// Death Star laser control: loading.
        pub const DEATH_STAR_LASER_LOADING: u32 = 1023;
        /// Death Star laser control: gauge.
        pub const DEATH_STAR_LASER_GAUGE: u32 = 1024;

        /// Highlight Alliance ships.
        pub const HIGHLIGHT_ALLIANCE_SHIPS: u32 = 1034;
        /// Dim Alliance ships.
        pub const DIM_ALLIANCE_SHIPS: u32 = 1035;
        /// Highlight Empire ships.
        pub const HIGHLIGHT_EMPIRE_SHIPS: u32 = 1036;
        /// Dim Empire ships.
        pub const DIM_EMPIRE_SHIPS: u32 = 1037;

        /// Tactical command button: Maneuvers/Tactics, normal state.
        pub const BTN_MANEUVERS_TACTICS_NORMAL: u32 = 1105;
        /// Tactical command button: Maneuvers/Tactics, pressed state.
        pub const BTN_MANEUVERS_TACTICS_PRESSED: u32 = 1106;
        /// Tactical command button: Missions, normal state.
        pub const BTN_MISSIONS_NORMAL: u32 = 1107;
        /// Tactical command button: Missions, pressed state.
        pub const BTN_MISSIONS_PRESSED: u32 = 1108;

        /// Tactical command button: withdraw from battle, normal state.
        pub const BTN_WITHDRAW_FROM_BATTLE_NORMAL: u32 = 1149;
        /// Tactical command button: withdraw from battle, pressed state.
        pub const BTN_WITHDRAW_FROM_BATTLE_PRESSED: u32 = 1150;

        /// Tactical command button: recover, Empire normal state.
        pub const BTN_RECOVER_EMPIRE_NORMAL: u32 = 1170;
        /// Tactical command button: recover, Empire pressed state.
        pub const BTN_RECOVER_EMPIRE_PRESSED: u32 = 1171;
        /// Tactical command button: recover, Empire disabled state.
        pub const BTN_RECOVER_EMPIRE_DISABLED: u32 = 1172;
        /// Tactical command button: recover, Alliance normal state.
        pub const BTN_RECOVER_ALLIANCE_NORMAL: u32 = 1173;
        /// Tactical command button: recover, Alliance pressed state.
        pub const BTN_RECOVER_ALLIANCE_PRESSED: u32 = 1174;
        /// Tactical command button: recover, Alliance disabled state.
        pub const BTN_RECOVER_ALLIANCE_DISABLED: u32 = 1175;

        /// Tactical command button: attack Death Star, normal state.
        pub const BTN_ATTACK_DEATH_STAR_NORMAL: u32 = 1176;
        /// Tactical command button: attack Death Star, pressed state.
        pub const BTN_ATTACK_DEATH_STAR_PRESSED: u32 = 1177;
        /// Tactical command button: attack Death Star, disabled state.
        pub const BTN_ATTACK_DEATH_STAR_DISABLED: u32 = 1178;

        /// Tactical command button: attack capital ships, Alliance normal state.
        pub const BTN_ATTACK_CAPITAL_SHIPS_ALLIANCE_NORMAL: u32 = 1179;
        /// Tactical command button: attack capital ships, Alliance pressed state.
        pub const BTN_ATTACK_CAPITAL_SHIPS_ALLIANCE_PRESSED: u32 = 1180;
        /// Tactical command button: attack capital ships, Empire normal state.
        pub const BTN_ATTACK_CAPITAL_SHIPS_EMPIRE_NORMAL: u32 = 1182;
        /// Tactical command button: attack capital ships, Empire pressed state.
        pub const BTN_ATTACK_CAPITAL_SHIPS_EMPIRE_PRESSED: u32 = 1183;

        /// Tactical command button: attack fighters, Alliance normal state.
        pub const BTN_ATTACK_FIGHTERS_ALLIANCE_NORMAL: u32 = 1191;
        /// Tactical command button: attack fighters, Alliance pressed state.
        pub const BTN_ATTACK_FIGHTERS_ALLIANCE_PRESSED: u32 = 1192;
        /// Tactical command button: attack fighters, Empire normal state.
        pub const BTN_ATTACK_FIGHTERS_EMPIRE_NORMAL: u32 = 1194;
        /// Tactical command button: attack fighters, Empire pressed state.
        pub const BTN_ATTACK_FIGHTERS_EMPIRE_PRESSED: u32 = 1195;

        /// Weapon recharge gauge: 0%.
        pub const WEAPON_RECHARGE_0_PCT: u32 = 1206;
        /// Weapon recharge gauge: 25%.
        pub const WEAPON_RECHARGE_25_PCT: u32 = 1207;
        /// Weapon recharge gauge: 50%.
        pub const WEAPON_RECHARGE_50_PCT: u32 = 1208;
        /// Weapon recharge gauge: 75%.
        pub const WEAPON_RECHARGE_75_PCT: u32 = 1209;
        /// Weapon recharge gauge: 100%.
        pub const WEAPON_RECHARGE_100_PCT: u32 = 1210;

        /// Right-side hull integrity and shield strength panel.
        pub const RIGHT_PANEL_HULL_AND_SHIELD: u32 = 1302;

        /// Mission HUD: attack capital ships (Alliance).
        pub const MISSIONS_HUD_ATTACK_CAPITAL_SHIPS_ALLIANCE: u32 = 2151;
        /// Mission HUD: attack fighters (Alliance).
        pub const MISSIONS_HUD_ATTACK_FIGHTERS_ALLIANCE: u32 = 2152;
        /// Mission HUD: recover (Alliance).
        pub const MISSIONS_HUD_RECOVER_ALLIANCE: u32 = 2153;
        /// Mission HUD: attack Death Star (Alliance).
        pub const MISSIONS_HUD_ATTACK_DEATH_STAR_ALLIANCE: u32 = 2154;
        /// Mission HUD: attack capital ships (Empire).
        pub const MISSIONS_HUD_ATTACK_CAPITAL_SHIPS_EMPIRE: u32 = 2155;
        /// Mission HUD: attack fighters (Empire).
        pub const MISSIONS_HUD_ATTACK_FIGHTERS_EMPIRE: u32 = 2156;
        /// Mission HUD: recover (Empire).
        pub const MISSIONS_HUD_RECOVER_EMPIRE: u32 = 2157;
        /// Mission HUD: empty state.
        pub const MISSIONS_HUD_EMPTY: u32 = 2158;

        /// Start of the tactical ship sprite block.
        pub const SHIP_SPRITE_START: u32 = 2001;
        /// End of the tactical ship sprite block.
        pub const SHIP_SPRITE_END: u32 = 2130;
    }

    /// Resource IDs for `GOKRES.DLL` BMPs.
    ///
    /// Covers high-value facility icons, officer portraits, and commonly used
    /// mini-icons for ships and fighter squadrons.
    pub mod gokres {
        /// Facility status: mine.
        pub const FACILITY_MINE: u32 = 1;
        /// Facility status: refinery.
        pub const FACILITY_REFINERY: u32 = 2;
        /// Facility status: orbital shipyard.
        pub const FACILITY_ORBITAL_SHIPYARD: u32 = 256;
        /// Facility status: advanced shipyard.
        pub const FACILITY_ADVANCED_SHIPYARD: u32 = 259;
        /// Facility status: KDY-150 shipyard.
        pub const FACILITY_KDY_150: u32 = 512;
        /// Facility status: LNR Series 1.
        pub const FACILITY_LNR_SERIES_1: u32 = 513;
        /// Facility status: Gencore level 1.
        pub const FACILITY_GENCORE_LEVEL_1: u32 = 514;
        /// Facility status: LNR Series 2.
        pub const FACILITY_LNR_SERIES_2: u32 = 515;
        /// Facility status: Gencore level 2.
        pub const FACILITY_GENCORE_LEVEL_2: u32 = 516;
        /// Facility status: Death Star shield.
        pub const FACILITY_DEATH_STAR_SHIELD: u32 = 640;
        /// Facility status: Alliance Headquarters.
        pub const FACILITY_ALLIANCE_HEADQUARTERS: u32 = 832;

        /// Portrait: Admiral Ackbar.
        pub const PORTRAIT_ACKBAR: u32 = 19008;
        /// Portrait: Wedge Antilles.
        pub const PORTRAIT_WEDGE_ANTILLES: u32 = 19009;
        /// Portrait: Lando Calrissian.
        pub const PORTRAIT_LANDO_CALRISSIAN: u32 = 19010;
        /// Portrait: Chewbacca.
        pub const PORTRAIT_CHEWBACCA: u32 = 19011;
        /// Portrait: Jan Dodonna.
        pub const PORTRAIT_JAN_DODONNA: u32 = 19012;
        /// Portrait: Crix Madine.
        pub const PORTRAIT_CRIX_MADINE: u32 = 19013;
        /// Portrait: Carlist Rieekan.
        pub const PORTRAIT_CARLIST_RIEEKAN: u32 = 19014;
        /// Portrait: Afyon.
        pub const PORTRAIT_AFYON: u32 = 19015;
        /// Portrait: Drayson.
        pub const PORTRAIT_DRAYSON: u32 = 19016;
        /// Portrait: Borsk Fey'lya.
        pub const PORTRAIT_BORSK_FEYLYA: u32 = 19017;
        /// Portrait: Tura Raftican.
        pub const PORTRAIT_TURA_RAFTICAN: u32 = 19018;
        /// Portrait: Bren Derlin.
        pub const PORTRAIT_BREN_DERLIN: u32 = 19019;
        /// Portrait: Garm Bel Iblis.
        pub const PORTRAIT_GARM_BEL_IBLIS: u32 = 19020;
        /// Portrait: Talon Karrde.
        pub const PORTRAIT_TALON_KARRDE: u32 = 19021;
        /// Portrait: Narra.
        pub const PORTRAIT_NARRA: u32 = 19022;
        /// Portrait: Huoba Neva.
        pub const PORTRAIT_HUOBA_NEVA: u32 = 19023;
        /// Portrait: Page.
        pub const PORTRAIT_PAGE: u32 = 19024;
        /// Portrait: Syub Snunb.
        pub const PORTRAIT_SYUB_SNUNB: u32 = 19025;
        /// Portrait: Adar Tallon.
        pub const PORTRAIT_ADAR_TALLON: u32 = 19026;
        /// Portrait: Sarin Virgilio.
        pub const PORTRAIT_SARIN_VIRGILIO: u32 = 19027;
        /// Portrait: Vanden Willard.
        pub const PORTRAIT_VANDEN_WILLARD: u32 = 19028;
        /// Portrait: Roget Jiriss.
        pub const PORTRAIT_ROGET_JIRISS: u32 = 19029;
        /// Portrait: Kaiya Andrimetrum.
        pub const PORTRAIT_KAIYA_ANDRIMETRUM: u32 = 19030;
        /// Portrait: Mazer Rackus.
        pub const PORTRAIT_MAZER_RACKUS: u32 = 19031;
        /// Portrait: Orrimaarko.
        pub const PORTRAIT_ORRIMAARKO: u32 = 19032;
        /// Portrait: Ma'w'shiye.
        pub const PORTRAIT_MAWSHIYE: u32 = 19033;

        /// Portrait: Governor Jerjerrod.
        pub const PORTRAIT_JERJERROD: u32 = 19072;
        /// Portrait: Admiral Ozzel.
        pub const PORTRAIT_OZZEL: u32 = 19073;
        /// Portrait: Admiral Piett.
        pub const PORTRAIT_PIETT: u32 = 19074;
        /// Portrait: General Veers.
        pub const PORTRAIT_VEERS: u32 = 19075;
        /// Portrait: Brandei.
        pub const PORTRAIT_BRANDEI: u32 = 19076;
        /// Portrait: Covell.
        pub const PORTRAIT_COVELL: u32 = 19077;
        /// Portrait: Dorja.
        pub const PORTRAIT_DORJA: u32 = 19078;
        /// Portrait: Bin Essada.
        pub const PORTRAIT_BIN_ESSADA: u32 = 19079;
        /// Portrait: Niles Ferrier.
        pub const PORTRAIT_NILES_FERRIER: u32 = 19080;
        /// Portrait: Grammel.
        pub const PORTRAIT_GRAMMEL: u32 = 19081;
        /// Portrait: Griff.
        pub const PORTRAIT_GRIFF: u32 = 19082;
        /// Portrait: Klev.
        pub const PORTRAIT_KLEV: u32 = 19083;
        /// Portrait: Needa.
        pub const PORTRAIT_NEEDA: u32 = 19084;
        /// Portrait: Bane Nothos.
        pub const PORTRAIT_BANE_NOTHOS: u32 = 19085;
        /// Portrait: Orlok.
        pub const PORTRAIT_ORLOK: u32 = 19086;
        /// Portrait: Pellaeon.
        pub const PORTRAIT_PELLAEON: u32 = 19087;
        /// Portrait: Screed.
        pub const PORTRAIT_SCREED: u32 = 19088;
        /// Portrait: Thrawn.
        pub const PORTRAIT_THRAWN: u32 = 19089;
        /// Portrait: Zuggs.
        pub const PORTRAIT_ZUGGS: u32 = 19090;
        /// Portrait: Daala.
        pub const PORTRAIT_DAALA: u32 = 19091;
        /// Portrait: Pter Thanas.
        pub const PORTRAIT_PTER_THANAS: u32 = 19092;
        /// Portrait: Bevel Lemelisk.
        pub const PORTRAIT_BEVEL_LEMELISK: u32 = 19093;
        /// Portrait: Shenir Rix.
        pub const PORTRAIT_SHENIR_RIX: u32 = 19094;
        /// Portrait: Noval Garaint.
        pub const PORTRAIT_NOVAL_GARAINT: u32 = 19095;
        /// Portrait: Garindan.
        pub const PORTRAIT_GARINDAN: u32 = 19096;
        /// Portrait: Menndo.
        pub const PORTRAIT_MENNDO: u32 = 19097;
        /// Portrait: Labansat.
        pub const PORTRAIT_LABANSAT: u32 = 19098;
        /// Portrait: Villar.
        pub const PORTRAIT_VILLAR: u32 = 19099;

        /// Portrait: Mon Mothma.
        pub const PORTRAIT_MON_MOTHMA: u32 = 18496;
        /// Portrait: Leia Organa.
        pub const PORTRAIT_LEIA_ORGANA: u32 = 18497;
        /// Portrait: Luke Skywalker.
        pub const PORTRAIT_LUKE_SKYWALKER: u32 = 18498;
        /// Portrait: Han Solo.
        pub const PORTRAIT_HAN_SOLO: u32 = 18499;
        /// Portrait: Luke Skywalker as Jedi Knight.
        pub const PORTRAIT_LUKE_SKYWALKER_JEDI_KNIGHT: u32 = 18512;
        /// Portrait: Emperor Palpatine.
        pub const PORTRAIT_EMPEROR_PALPATINE: u32 = 18560;
        /// Portrait: Darth Vader.
        pub const PORTRAIT_DARTH_VADER: u32 = 18561;

        /// Fighter mini-icon: A-wing.
        pub const MINI_FIGHTER_A_WING: u32 = 17984;
        /// Fighter mini-icon: B-wing.
        pub const MINI_FIGHTER_B_WING: u32 = 17985;
        /// Fighter mini-icon: X-wing.
        pub const MINI_FIGHTER_X_WING: u32 = 17986;
        /// Fighter mini-icon: Y-wing.
        pub const MINI_FIGHTER_Y_WING: u32 = 17987;
        /// Fighter mini-icon: TIE Fighter.
        pub const MINI_FIGHTER_TIE_FIGHTER: u32 = 18048;
        /// Fighter mini-icon: TIE Interceptor.
        pub const MINI_FIGHTER_TIE_INTERCEPTOR: u32 = 18049;
        /// Fighter mini-icon: TIE Bomber.
        pub const MINI_FIGHTER_TIE_BOMBER: u32 = 18050;
        /// Fighter mini-icon: TIE Defender.
        pub const MINI_FIGHTER_TIE_DEFENDER: u32 = 18051;

        /// Ship mini-icon: MC80 Liberty type cruiser.
        pub const MINI_SHIP_MC80_LIBERTY_CRUISER: u32 = 18240;
        /// Ship mini-icon: bulk cruiser.
        pub const MINI_SHIP_BULK_CRUISER: u32 = 18241;
        /// Ship mini-icon: assault frigate.
        pub const MINI_SHIP_ASSAULT_FRIGATE: u32 = 18242;
        /// Ship mini-icon: Nebulon-B frigate.
        pub const MINI_SHIP_NEBULON_B_FRIGATE: u32 = 18243;
        /// Ship mini-icon: Alliance escort carrier.
        pub const MINI_SHIP_ALLIANCE_ESCORT_CARRIER: u32 = 18244;
        /// Ship mini-icon: Corellian corvette.
        pub const MINI_SHIP_CORELLIAN_CORVETTE: u32 = 18245;
        /// Ship mini-icon: medium transport.
        pub const MINI_SHIP_MEDIUM_TRANSPORT: u32 = 18246;
        /// Ship mini-icon: bulk transport.
        pub const MINI_SHIP_BULK_TRANSPORT: u32 = 18247;
        /// Ship mini-icon: Corellian gunship.
        pub const MINI_SHIP_CORELLIAN_GUNSHIP: u32 = 18248;
        /// Ship mini-icon: Alliance dreadnaught / MC40A light cruiser.
        pub const MINI_SHIP_ALLIANCE_DREADNAUGHT: u32 = 18249;
        /// Ship mini-icon: CC-7700 frigate.
        pub const MINI_SHIP_CC_7700_FRIGATE: u32 = 18250;
        /// Ship mini-icon: Bulwark battlecruiser / Viscount Star Defender.
        pub const MINI_SHIP_VISCOUNT_STAR_DEFENDER: u32 = 18251;
        /// Ship mini-icon: Liberator cruiser.
        pub const MINI_SHIP_LIBERATOR_CRUISER: u32 = 18252;
        /// Ship mini-icon: CC-9600 frigate / MC30c frigate.
        pub const MINI_SHIP_MC30C_FRIGATE: u32 = 18253;
        /// Ship mini-icon: Dauntless cruiser / MC80A Home One type.
        pub const MINI_SHIP_MC80A_HOME_ONE_CRUISER: u32 = 18254;

        /// Ship mini-icon: Strike cruiser / Vindicator heavy cruiser.
        pub const MINI_SHIP_STRIKE_CRUISER: u32 = 18304;
        /// Ship mini-icon: Lancer frigate.
        pub const MINI_SHIP_LANCER_FRIGATE: u32 = 18305;
        /// Ship mini-icon: Interdictor cruiser / Immobilizer cruiser.
        pub const MINI_SHIP_INTERDICTOR_CRUISER: u32 = 18306;
        /// Ship mini-icon: Carrack light cruiser / Arquitens light cruiser.
        pub const MINI_SHIP_CARRACK_LIGHT_CRUISER: u32 = 18307;
        /// Ship mini-icon: Victory I Star Destroyer.
        pub const MINI_SHIP_VICTORY_I_STAR_DESTROYER: u32 = 18308;
        /// Ship mini-icon: Imperial I Star Destroyer.
        pub const MINI_SHIP_IMPERIAL_I_STAR_DESTROYER: u32 = 18309;
        /// Ship mini-icon: Super Star Destroyer.
        pub const MINI_SHIP_SUPER_STAR_DESTROYER: u32 = 18310;
        /// Ship mini-icon: assault transport / Gladiator Star Destroyer.
        pub const MINI_SHIP_GLADIATOR_STAR_DESTROYER: u32 = 18311;
        /// Ship mini-icon: Death Star.
        pub const MINI_SHIP_DEATH_STAR: u32 = 18312;
        /// Ship mini-icon: galleon / Acclamator drop ship.
        pub const MINI_SHIP_ACCLAMATOR_DROP_SHIP: u32 = 18313;
        /// Ship mini-icon: Victory II Star Destroyer.
        pub const MINI_SHIP_VICTORY_II_STAR_DESTROYER: u32 = 18314;
        /// Ship mini-icon: Imperial II Star Destroyer.
        pub const MINI_SHIP_IMPERIAL_II_STAR_DESTROYER: u32 = 18315;
        /// Ship mini-icon: Star Galleon frigate.
        pub const MINI_SHIP_STAR_GALLEON_FRIGATE: u32 = 18316;
        /// Ship mini-icon: Imperial escort carrier.
        pub const MINI_SHIP_IMPERIAL_ESCORT_CARRIER: u32 = 18317;
        /// Ship mini-icon: Imperial dreadnaught.
        pub const MINI_SHIP_IMPERIAL_DREADNOUGHT: u32 = 18318;
    }
}

// ---------------------------------------------------------------------------
// BmpCache
// ---------------------------------------------------------------------------

/// Lazy-loading texture cache for DLL-extracted BMP assets.
pub struct BmpCache {
    /// Root directory containing staged `{dll-name}-dll/BMP/` trees.
    base_path: Option<PathBuf>,
    /// Optional HD PNG directory, consulted only by the faithful-HD profile.
    hd_path: Option<PathBuf>,
    /// Explicit render profile. Original parity is the fail-closed default.
    profile: AssetRenderProfile,
    /// Asset keys explicitly approved by the faithful-HD manifest.
    approved_hd_assets: HashMap<String, ApprovedHdAsset>,
    /// Cached textures.  `None` value means "attempted load, file not found".
    textures: HashMap<(DllSource, u32), Option<TextureHandle>>,
    /// Macroquad textures used below the egui interface layers.
    macroquad_textures: HashMap<(DllSource, u32), Option<Texture2D>>,
    /// Native-style per-pixel hit masks decoded from the original BMPs.
    hit_masks: HashMap<(DllSource, u32), Option<BitmapHitMask>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BitmapHitMask {
    width: usize,
    height: usize,
    opaque: Vec<bool>,
}

impl BitmapHitMask {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.get(0..2)? != b"BM" {
            return None;
        }
        let u16_at = |offset: usize| {
            Some(u16::from_le_bytes(
                bytes.get(offset..offset + 2)?.try_into().ok()?,
            ))
        };
        let u32_at = |offset: usize| {
            Some(u32::from_le_bytes(
                bytes.get(offset..offset + 4)?.try_into().ok()?,
            ))
        };
        let i32_at = |offset: usize| {
            Some(i32::from_le_bytes(
                bytes.get(offset..offset + 4)?.try_into().ok()?,
            ))
        };

        let pixel_offset = u32_at(10)? as usize;
        let dib_size = u32_at(14)?;
        let signed_width = i32_at(18)?;
        let signed_height = i32_at(22)?;
        if dib_size < 40
            || signed_width <= 0
            || signed_height == 0
            || u16_at(26)? != 1
            || u16_at(28)? != 8
            || u32_at(30)? != 0
        {
            return None;
        }

        let width = usize::try_from(signed_width).ok()?;
        let height = usize::try_from(signed_height.unsigned_abs()).ok()?;
        let row_stride = width.checked_add(3)? & !3;
        let pixel_bytes = row_stride.checked_mul(height)?;
        pixel_offset
            .checked_add(pixel_bytes)
            .filter(|end| *end <= bytes.len())?;

        // The native control stores the first DIB pixel's palette index as its
        // transparent key. Positive-height BMP rows are stored bottom-up.
        let transparent_index = *bytes.get(pixel_offset)?;
        let bottom_up = signed_height > 0;
        let mut opaque = Vec::with_capacity(width.checked_mul(height)?);
        for y in 0..height {
            let stored_y = if bottom_up { height - 1 - y } else { y };
            let row_offset = pixel_offset + stored_y * row_stride;
            for x in 0..width {
                opaque.push(bytes[row_offset + x] != transparent_index);
            }
        }

        Some(Self {
            width,
            height,
            opaque,
        })
    }

    /// `FUN_005fca00` excludes every outer edge before consulting the BMP's
    /// palette-key mask. Its transparent palette index is the first stored
    /// pixel, which is the decoded image's bottom-left pixel for these BMPs.
    fn contains(&self, x: usize, y: usize) -> bool {
        x > 0 && y > 0 && x < self.width && y < self.height && self.opaque[y * self.width + x]
    }
}

impl BmpCache {
    /// Create an empty cache with no path configured.
    pub fn new() -> Self {
        Self {
            base_path: None,
            hd_path: None,
            profile: AssetRenderProfile::OriginalParity,
            approved_hd_assets: HashMap::new(),
            textures: HashMap::new(),
            macroquad_textures: HashMap::new(),
            hit_masks: HashMap::new(),
        }
    }

    /// Set the root directory that contains `{dll-name}-dll/BMP/` trees.
    ///
    /// Call before any `get()` or `preload_range()` invocations.
    pub fn set_base_path(&mut self, path: impl Into<PathBuf>) {
        self.base_path = Some(path.into());
        self.textures.clear();
        self.macroquad_textures.clear();
        self.hit_masks.clear();
    }

    /// Set an optional HD PNG directory.
    ///
    /// Expected layout: `{hd_path}/{dll-name}/{resource_id}.png`. Setting the
    /// path does not enable substitutions; call [`Self::set_render_profile`]
    /// with [`AssetRenderProfile::FaithfulHd`] explicitly.
    pub fn set_hd_path(&mut self, path: impl Into<PathBuf>) {
        let path = path.into();
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.approved_hd_assets = load_approved_hd_assets(&path);
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.approved_hd_assets.clear();
        }
        self.hd_path = Some(path);
        self.textures.clear();
    }

    /// Change the render profile and invalidate textures loaded by the prior
    /// profile.
    pub fn set_render_profile(&mut self, profile: AssetRenderProfile) {
        if self.profile != profile {
            self.profile = profile;
            self.textures.clear();
        }
    }

    /// Return the active render profile.
    pub const fn render_profile(&self) -> AssetRenderProfile {
        self.profile
    }

    fn hd_asset_approval(&self, source: DllSource, resource_id: u32) -> Option<&ApprovedHdAsset> {
        self.profile.allows_hd().then_some(())?;
        self.approved_hd_assets
            .get(&format!("{}/{}", source.dll_dir_name(), resource_id))
    }

    /// Retrieve a texture by source DLL and resource ID.
    ///
    /// On first call for a given `(source, id)` the BMP is loaded from disk
    /// and cached.  Returns `None` if the file is absent, unreadable, or this
    /// is a WASM build.
    pub fn get(
        &mut self,
        ctx: &egui::Context,
        source: DllSource,
        resource_id: u32,
    ) -> Option<&TextureHandle> {
        let key = (source, resource_id);

        if !self.textures.contains_key(&key) {
            let handle = self.load_texture(ctx, source, resource_id);
            if handle.is_none() {
                eprintln!(
                    "[bmp_cache] asset unavailable source={} resource_id={}",
                    source.dll_dir_name(),
                    resource_id
                );
            }
            self.textures.insert(key, handle);
        }

        self.textures.get(&key)?.as_ref()
    }

    /// Test a source pixel against the original bitmap's native hit mask.
    ///
    /// This intentionally ignores faithful-HD substitutions. Interaction
    /// geometry remains tied to the original resource even when reviewed HD
    /// artwork is selected for rendering.
    pub fn is_resource_hit(
        &mut self,
        source: DllSource,
        resource_id: u32,
        x: usize,
        y: usize,
    ) -> bool {
        self.ensure_hit_mask(source, resource_id);
        self.hit_masks
            .get(&(source, resource_id))
            .and_then(Option::as_ref)
            .is_some_and(|mask| mask.contains(x, y))
    }

    /// Return the original indexed bitmap dimensions used for logical paint.
    pub fn original_resource_size(
        &mut self,
        source: DllSource,
        resource_id: u32,
    ) -> Option<[usize; 2]> {
        self.ensure_hit_mask(source, resource_id);
        self.hit_masks
            .get(&(source, resource_id))
            .and_then(Option::as_ref)
            .map(|mask| [mask.width, mask.height])
    }

    /// Retrieve an original bitmap as a nearest-neighbor Macroquad texture.
    ///
    /// This path is for authored bitmap layers painted beneath Macroquad
    /// primitives before egui paints the original interface chrome. It uses
    /// the original resource even when the optional HD profile is active.
    pub fn get_macroquad_original(
        &mut self,
        source: DllSource,
        resource_id: u32,
    ) -> Option<&Texture2D> {
        let key = (source, resource_id);
        if !self.macroquad_textures.contains_key(&key) {
            let texture = self
                .load_original_bytes(source, resource_id)
                .and_then(|bytes| decode_macroquad_texture(&bytes, source, resource_id));
            if texture.is_none() {
                eprintln!(
                    "[bmp_cache] macroquad asset unavailable source={} resource_id={}",
                    source.dll_dir_name(),
                    resource_id
                );
            }
            self.macroquad_textures.insert(key, texture);
        }

        self.macroquad_textures.get(&key)?.as_ref()
    }

    fn ensure_hit_mask(&mut self, source: DllSource, resource_id: u32) {
        let key = (source, resource_id);
        if !self.hit_masks.contains_key(&key) {
            let mask = self
                .load_original_bytes(source, resource_id)
                .and_then(|bytes| BitmapHitMask::from_bytes(&bytes));
            self.hit_masks.insert(key, mask);
        }
    }

    /// Bulk-load all resources in `[start, end]` (inclusive) for one DLL.
    ///
    /// Useful for pre-warming the cache before the first frame that needs
    /// those textures, avoiding hitches. Missing files are logged once and
    /// negatively cached so later frames do not repeat the same lookup.
    pub fn preload_range(&mut self, ctx: &egui::Context, source: DllSource, start: u32, end: u32) {
        for id in start..=end {
            let key = (source, id);
            if !self.textures.contains_key(&key) {
                let handle = self.load_texture(ctx, source, id);
                if handle.is_none() {
                    eprintln!(
                        "[bmp_cache] asset unavailable source={} resource_id={}",
                        source.dll_dir_name(),
                        id
                    );
                }
                self.textures.insert(key, handle);
            }
        }
    }

    // ── Internal ────────────────────────────────────────────────────────────

    #[cfg(not(target_arch = "wasm32"))]
    fn load_original_bytes(&self, source: DllSource, resource_id: u32) -> Option<Vec<u8>> {
        let base = self.base_path.as_deref()?;
        let bmp_file = rebase_path_prefix(base, "data/base", DATA_PREFIX)
            .join(source.dll_dir_name())
            .join("BMP")
            .join(format!("{}.bmp", resource_id));
        std::fs::read(bmp_file).ok()
    }

    #[cfg(target_arch = "wasm32")]
    fn load_original_bytes(&self, source: DllSource, resource_id: u32) -> Option<Vec<u8>> {
        get_bmp_bytes(source.dll_dir_name(), resource_id)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_texture(
        &self,
        ctx: &egui::Context,
        source: DllSource,
        resource_id: u32,
    ) -> Option<TextureHandle> {
        let base = self.base_path.as_deref()?;
        let bmp_file = rebase_path_prefix(base, "data/base", DATA_PREFIX)
            .join(source.dll_dir_name())
            .join("BMP")
            .join(format!("{}.bmp", resource_id));

        // Faithful HD is opt-in. Original parity never probes the HD tree.
        if let Some(approval) = self.hd_asset_approval(source, resource_id) {
            if let Some(hd_dir) = &self.hd_path {
                let hd_file = rebase_path_prefix(hd_dir, "data/hd", HD_PREFIX)
                    .join(source.dll_dir_name())
                    .join(format!("{}.png", resource_id));
                if let Some(bytes) = validated_hd_bytes(&bmp_file, &hd_file, approval) {
                    if let Some(handle) = load_image_bytes_as_texture(
                        ctx,
                        source,
                        resource_id,
                        &bytes,
                        AssetVariant::FaithfulHd,
                    ) {
                        return Some(handle);
                    }
                } else if hd_file.exists() {
                    eprintln!(
                        "[bmp_cache] HD source/output digest mismatch for {}/{}; falling back to original",
                        source.dll_dir_name(),
                        resource_id
                    );
                }
            }
        }

        // Original staged BMP is authoritative and is always the fallback.
        if bmp_file.exists() {
            load_image_as_texture(ctx, source, resource_id, &bmp_file, AssetVariant::Original)
        } else {
            None
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn load_texture(
        &self,
        ctx: &egui::Context,
        source: DllSource,
        resource_id: u32,
    ) -> Option<TextureHandle> {
        let dll_dir = source.dll_dir_name();

        if self.hd_asset_approval(source, resource_id).is_some() {
            if let Some(bytes) = get_hd_bytes(dll_dir, resource_id) {
                match decode_color_image(&bytes, source, resource_id) {
                    Ok(color_image) => {
                        return Some(ctx.load_texture(
                            &format!("{}_{}_hd", source.texture_prefix(), resource_id),
                            color_image,
                            AssetVariant::FaithfulHd.texture_options(),
                        ));
                    }
                    Err(error) => {
                        macroquad::logging::warn!(
                            "[bmp_cache] WASM HD decode failed for {}/{}: {}; falling back to original",
                            dll_dir,
                            resource_id,
                            error
                        );
                    }
                }
            }
        }

        let bytes = get_bmp_bytes(dll_dir, resource_id)?;
        let color_image = match decode_color_image(&bytes, source, resource_id) {
            Ok(image) => image,
            Err(error) => {
                macroquad::logging::warn!(
                    "[bmp_cache] WASM original decode failed for {}/{}: {}",
                    dll_dir,
                    resource_id,
                    error
                );
                return None;
            }
        };

        Some(ctx.load_texture(
            &format!("{}_{}", source.texture_prefix(), resource_id),
            color_image,
            AssetVariant::Original.texture_options(),
        ))
    }
}

impl Default for BmpCache {
    fn default() -> Self {
        Self::new()
    }
}

fn rebase_path_prefix(path: &Path, from_prefix: &str, to_prefix: &str) -> PathBuf {
    path.strip_prefix(from_prefix)
        .map(|suffix| PathBuf::from(to_prefix).join(suffix))
        .unwrap_or_else(|_| path.to_path_buf())
}

/// Return whether a staged resource uses the original game's palette-blue
/// transparency matte.
fn uses_blue_screen_transparency(source: DllSource, resource_id: u32) -> bool {
    match source {
        DllSource::Strategy => matches!(
            resource_id,
            resources::strategy::ALLIANCE_COMMAND_CENTER_SHELL
                | resources::strategy::EMPIRE_COMMAND_CENTER_SHELL
                | resources::strategy::GID_ALLIANCE_LARGEST
                    ..=resources::strategy::GID_UNEXPLORED
                | 10159..=10167
                | 10169..=10170
                | resources::strategy::SECTOR_PLANET_FIRST
                    ..=resources::strategy::SECTOR_PLANET_LAST
                | resources::strategy::SECTOR_PLANET_SPECIAL_FIRST
                    ..=resources::strategy::SECTOR_PLANET_SPECIAL_LAST
        ),
        DllSource::Gokres => matches!(resource_id, 16_000..=19_999),
        DllSource::Common => matches!(
            resource_id,
            10001..=10003
                | 10005
                | 10007
                | 10009
                | 10011
                | 10013..=10015
                | 10017..=10019
                | 10158..=10159
                | resources::common::MAIN_MENU_ANIMATION_FIRST
                    ..=resources::common::MAIN_MENU_ANIMATION_LAST
        ),
        DllSource::Tactical => false,
    }
}

fn decode_macroquad_texture(
    bytes: &[u8],
    source: DllSource,
    resource_id: u32,
) -> Option<Texture2D> {
    let rgba = decode_rgba_image(bytes, source, resource_id).ok()?;
    let width = u16::try_from(rgba.width()).ok()?;
    let height = u16::try_from(rgba.height()).ok()?;
    let texture = Texture2D::from_rgba8(width, height, rgba.as_raw());
    texture.set_filter(FilterMode::Nearest);
    Some(texture)
}

/// Decode a staged image and apply the original game's palette-blue
/// transparency to resources whose extracted bitmaps contain that matte.
fn decode_color_image(
    bytes: &[u8],
    source: DllSource,
    resource_id: u32,
) -> image::ImageResult<egui::ColorImage> {
    let rgba = decode_rgba_image(bytes, source, resource_id)?;

    let (w, h) = rgba.dimensions();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        [w as usize, h as usize],
        rgba.as_raw(),
    ))
}

fn decode_rgba_image(
    bytes: &[u8],
    source: DllSource,
    resource_id: u32,
) -> image::ImageResult<image::RgbaImage> {
    let mut rgba = image::load_from_memory(bytes)?.to_rgba8();
    if uses_blue_screen_transparency(source, resource_id) {
        for pixel in rgba.pixels_mut() {
            if pixel[0] < 32 && pixel[1] < 32 && pixel[2] > 192 {
                pixel[3] = 0;
            }
        }
    }
    Ok(rgba)
}

// ---------------------------------------------------------------------------
// File loader (native only)
// ---------------------------------------------------------------------------

/// Decode an image file (BMP or PNG) and register it as an egui texture.
#[cfg(not(target_arch = "wasm32"))]
fn load_image_as_texture(
    ctx: &egui::Context,
    source: DllSource,
    resource_id: u32,
    path: &Path,
    variant: AssetVariant,
) -> Option<TextureHandle> {
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!(
                "[bmp_cache] read failed source={} resource_id={} path={} error={}",
                source.dll_dir_name(),
                resource_id,
                path.display(),
                error
            );
            return None;
        }
    };

    load_image_bytes_as_texture(ctx, source, resource_id, &bytes, variant)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_image_bytes_as_texture(
    ctx: &egui::Context,
    source: DllSource,
    resource_id: u32,
    bytes: &[u8],
    variant: AssetVariant,
) -> Option<TextureHandle> {
    // `image` crate auto-detects format from magic bytes — handles both BMP
    // (which may be palette-indexed) and PNG.
    let color_image = match decode_color_image(bytes, source, resource_id) {
        Ok(image) => image,
        Err(error) => {
            eprintln!(
                "[bmp_cache] decode failed source={} resource_id={} bytes=verified error={}",
                source.dll_dir_name(),
                resource_id,
                error
            );
            return None;
        }
    };

    let handle = ctx.load_texture(
        format!("{}_{}", source.texture_prefix(), resource_id),
        color_image,
        variant.texture_options(),
    );

    Some(handle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_resource_is_negatively_cached() {
        let mut cache = BmpCache::new();
        let ctx = egui::Context::default();
        let key = (DllSource::Common, 999_999);

        assert!(cache.get(&ctx, key.0, key.1).is_none());
        assert!(matches!(cache.textures.get(&key), Some(None)));
        assert!(cache.get(&ctx, key.0, key.1).is_none());
        assert_eq!(cache.textures.len(), 1);
    }

    #[test]
    fn missing_macroquad_resource_is_negatively_cached_without_a_context() {
        let mut cache = BmpCache::new();
        let key = (DllSource::Strategy, 999_998);

        assert!(cache.get_macroquad_original(key.0, key.1).is_none());
        assert!(matches!(cache.macroquad_textures.get(&key), Some(None)));
        assert!(cache.get_macroquad_original(key.0, key.1).is_none());
        assert_eq!(cache.macroquad_textures.len(), 1);
    }

    #[test]
    fn render_profile_defaults_to_original_and_parses_stable_names() {
        let cache = BmpCache::new();
        assert_eq!(cache.render_profile(), AssetRenderProfile::OriginalParity);
        assert_eq!(
            AssetRenderProfile::parse("original-parity"),
            Some(AssetRenderProfile::OriginalParity)
        );
        assert_eq!(
            AssetRenderProfile::parse("faithful-hd"),
            Some(AssetRenderProfile::FaithfulHd)
        );
        assert_eq!(AssetRenderProfile::parse("experimental-remaster"), None);
    }

    #[test]
    fn hd_manifest_accepts_only_explicit_faithful_approvals() {
        let manifest = br#"{
            "schema_version": 1,
            "profile": "faithful-hd",
            "assets": {
                "common-dll/10001": {
                    "approved": true,
                    "review": {"reviewer": "tester", "evidence": "test-proof"},
                    "gates": {"human_review": "pass"},
                    "source": {"sha256": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"},
                    "output": {"sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}
                },
                "common-dll/10002": {"approved": false}
            }
        }"#;

        let approved = approved_hd_assets_from_bytes(manifest).unwrap();
        assert_eq!(approved.len(), 1);
        assert_eq!(
            approved.get("common-dll/10001"),
            Some(&ApprovedHdAsset {
                source_sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                    .to_string(),
                output_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .to_string(),
            })
        );
        assert!(!approved.contains_key("common-dll/10002"));
    }

    #[test]
    fn hd_manifest_rejects_unknown_schema_and_profile() {
        let schema = br#"{"schema_version":2,"profile":"faithful-hd","assets":{}}"#;
        let profile = br#"{"schema_version":1,"profile":"experimental-remaster","assets":{}}"#;

        assert!(approved_hd_assets_from_bytes(schema).is_err());
        assert!(approved_hd_assets_from_bytes(profile).is_err());
    }

    #[test]
    fn hd_manifest_rejects_approval_without_review_and_digest() {
        let manifest = br#"{
            "schema_version": 1,
            "profile": "faithful-hd",
            "assets": {"common-dll/10001": {"approved": true}}
        }"#;

        assert!(approved_hd_assets_from_bytes(manifest).is_err());
    }

    #[test]
    fn changing_render_profile_invalidates_cached_textures() {
        let mut cache = BmpCache::new();
        cache.textures.insert((DllSource::Common, 123), None);

        cache.set_render_profile(AssetRenderProfile::FaithfulHd);

        assert!(cache.textures.is_empty());
        assert_eq!(cache.render_profile(), AssetRenderProfile::FaithfulHd);
    }

    #[test]
    fn hd_requires_both_explicit_profile_and_manifest_approval() {
        let mut cache = BmpCache::new();
        cache.approved_hd_assets.insert(
            "common-dll/123".to_string(),
            ApprovedHdAsset {
                source_sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                    .to_string(),
                output_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .to_string(),
            },
        );

        assert!(cache.hd_asset_approval(DllSource::Common, 123).is_none());
        cache.set_render_profile(AssetRenderProfile::FaithfulHd);
        assert!(cache.hd_asset_approval(DllSource::Common, 123).is_some());
        assert!(cache.hd_asset_approval(DllSource::Common, 124).is_none());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn changed_source_or_hd_bytes_fail_and_verified_bytes_are_returned() {
        let base = std::env::temp_dir().join(format!(
            "open-rebellion-hd-digests-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        std::fs::create_dir_all(&base).unwrap();
        let source = base.join("source.bmp");
        let output = base.join("output.png");
        std::fs::write(&source, b"abc").unwrap();
        std::fs::write(&output, b"xyz").unwrap();
        let approval = ApprovedHdAsset {
            source_sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
                .to_string(),
            output_sha256: "3608bca1e44ea6c4d268eb6db02260269892c0b42b86bbf1e77a6fa16c3c9282"
                .to_string(),
        };
        assert_eq!(
            validated_hd_bytes(&source, &output, &approval),
            Some(b"xyz".to_vec())
        );
        std::fs::write(&source, b"changed").unwrap();
        assert!(validated_hd_bytes(&source, &output, &approval).is_none());
        std::fs::write(&source, b"abc").unwrap();
        std::fs::write(&output, b"changed").unwrap();
        assert!(validated_hd_bytes(&source, &output, &approval).is_none());
        std::fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn original_and_hd_variants_use_explicit_sampling() {
        assert_eq!(
            AssetVariant::Original.texture_options(),
            TextureOptions::NEAREST
        );
        assert_eq!(
            AssetVariant::FaithfulHd.texture_options(),
            TextureOptions::LINEAR
        );
    }

    #[test]
    fn cockpit_background_blue_screen_becomes_transparent() {
        let mut image = image::RgbaImage::new(2, 1);
        image.put_pixel(0, 0, image::Rgba([0, 0, 255, 255]));
        image.put_pixel(1, 0, image::Rgba([80, 90, 100, 255]));

        let mut encoded = Vec::new();
        image::DynamicImage::ImageRgba8(image)
            .write_to(
                &mut std::io::Cursor::new(&mut encoded),
                image::ImageFormat::Png,
            )
            .unwrap();
        let decoded = decode_color_image(
            &encoded,
            DllSource::Strategy,
            resources::strategy::ALLIANCE_COMMAND_CENTER_SHELL,
        )
        .unwrap();

        assert_eq!(decoded.pixels[0].a(), 0);
        assert_eq!(decoded.pixels[1].a(), 255);
    }

    #[test]
    fn galaxy_starfield_preserves_deep_blue_pixels() {
        let mut image = image::RgbaImage::new(1, 1);
        image.put_pixel(0, 0, image::Rgba([0, 0, 255, 255]));

        let mut encoded = Vec::new();
        image::DynamicImage::ImageRgba8(image)
            .write_to(
                &mut std::io::Cursor::new(&mut encoded),
                image::ImageFormat::Png,
            )
            .unwrap();
        let decoded = decode_color_image(
            &encoded,
            DllSource::Strategy,
            resources::strategy::GALAXY_STARFIELD_BRIGHT,
        )
        .unwrap();

        assert_eq!(decoded.pixels[0].a(), 255);
    }

    #[test]
    fn gid_marker_blue_screen_becomes_transparent() {
        let mut image = image::RgbaImage::new(2, 1);
        image.put_pixel(0, 0, image::Rgba([0, 0, 255, 255]));
        image.put_pixel(1, 0, image::Rgba([225, 20, 20, 255]));

        let mut encoded = Vec::new();
        image::DynamicImage::ImageRgba8(image)
            .write_to(
                &mut std::io::Cursor::new(&mut encoded),
                image::ImageFormat::Png,
            )
            .unwrap();
        let decoded = decode_rgba_image(
            &encoded,
            DllSource::Strategy,
            resources::strategy::GID_ALLIANCE_LARGEST,
        )
        .unwrap();

        assert_eq!(decoded.get_pixel(0, 0)[3], 0);
        assert_eq!(decoded.get_pixel(1, 0)[3], 255);
    }

    #[test]
    fn gokres_miniature_blue_screen_becomes_transparent() {
        let mut image = image::RgbaImage::new(3, 1);
        image.put_pixel(0, 0, image::Rgba([0, 0, 255, 255]));
        image.put_pixel(1, 0, image::Rgba([20, 20, 220, 255]));
        image.put_pixel(2, 0, image::Rgba([90, 100, 180, 255]));

        let mut encoded = Vec::new();
        image::DynamicImage::ImageRgba8(image)
            .write_to(
                &mut std::io::Cursor::new(&mut encoded),
                image::ImageFormat::Png,
            )
            .unwrap();
        for resource_id in [
            16_385,
            16_640,
            16_896,
            17_472,
            17_728,
            resources::gokres::MINI_FIGHTER_X_WING,
            resources::gokres::MINI_SHIP_CORELLIAN_CORVETTE,
            19_008,
        ] {
            let decoded = decode_color_image(&encoded, DllSource::Gokres, resource_id).unwrap();
            assert_eq!(decoded.pixels[0].a(), 0);
            assert_eq!(decoded.pixels[1].a(), 0);
            assert_eq!(decoded.pixels[2].a(), 255);
        }
    }

    #[test]
    fn sector_planet_blue_screen_becomes_transparent() {
        let mut image = image::RgbaImage::new(2, 1);
        image.put_pixel(0, 0, image::Rgba([0, 0, 255, 255]));
        image.put_pixel(1, 0, image::Rgba([120, 140, 180, 255]));

        let mut encoded = Vec::new();
        image::DynamicImage::ImageRgba8(image)
            .write_to(
                &mut std::io::Cursor::new(&mut encoded),
                image::ImageFormat::Png,
            )
            .unwrap();
        for resource_id in [10212, 10234, 10237, 10239] {
            let decoded = decode_color_image(&encoded, DllSource::Strategy, resource_id).unwrap();
            assert_eq!(decoded.pixels[0].a(), 0);
            assert_eq!(decoded.pixels[1].a(), 255);
        }
    }

    #[test]
    fn unrelated_blue_resource_remains_opaque() {
        let mut image = image::RgbaImage::new(1, 1);
        image.put_pixel(0, 0, image::Rgba([0, 0, 255, 255]));

        let mut encoded = Vec::new();
        image::DynamicImage::ImageRgba8(image)
            .write_to(
                &mut std::io::Cursor::new(&mut encoded),
                image::ImageFormat::Png,
            )
            .unwrap();
        let decoded = decode_color_image(&encoded, DllSource::Gokres, 2112).unwrap();

        assert_eq!(decoded.pixels[0].a(), 255);
    }

    #[test]
    fn native_hit_mask_uses_bottom_left_palette_key_and_strict_edges() {
        let width = 4usize;
        let height = 4usize;
        let row_stride = 4usize;
        let pixel_offset = 14 + 40 + 256 * 4;
        let mut encoded = vec![0u8; pixel_offset + row_stride * height];
        encoded[0..2].copy_from_slice(b"BM");
        let encoded_len = encoded.len() as u32;
        encoded[2..6].copy_from_slice(&encoded_len.to_le_bytes());
        encoded[10..14].copy_from_slice(&(pixel_offset as u32).to_le_bytes());
        encoded[14..18].copy_from_slice(&40u32.to_le_bytes());
        encoded[18..22].copy_from_slice(&(width as i32).to_le_bytes());
        encoded[22..26].copy_from_slice(&(height as i32).to_le_bytes());
        encoded[26..28].copy_from_slice(&1u16.to_le_bytes());
        encoded[28..30].copy_from_slice(&8u16.to_le_bytes());
        encoded[34..38].copy_from_slice(&((row_stride * height) as u32).to_le_bytes());

        // Indices 3 and 7 deliberately share a palette color. The native mask
        // compares indices, not decoded RGBA values. The first stored pixel is
        // index 7, and an interior index-7 pixel is transparent too.
        encoded[14 + 40 + 3 * 4..14 + 40 + 3 * 4 + 4].copy_from_slice(&[0, 255, 0, 0]);
        encoded[14 + 40 + 7 * 4..14 + 40 + 7 * 4 + 4].copy_from_slice(&[0, 255, 0, 0]);
        for stored_y in 0..height {
            let row = pixel_offset + stored_y * row_stride;
            encoded[row..row + width].fill(3);
        }
        encoded[pixel_offset] = 7;
        let interior_stored_y = height - 1 - 2;
        encoded[pixel_offset + interior_stored_y * row_stride + 2] = 7;

        let mask = BitmapHitMask::from_bytes(&encoded).unwrap();

        assert!(!mask.contains(0, 1));
        assert!(!mask.contains(1, 0));
        assert!(mask.contains(1, 1));
        assert!(!mask.contains(2, 2));
        assert!(!mask.contains(4, 1));
        assert!(!mask.contains(1, 4));
    }
}
