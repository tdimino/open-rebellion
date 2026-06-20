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
//! HD override PNGs (optional) live at:
//! ```text
//! {hd_path}/{dll-name}/{resource_id}.png
//! ```
//! HD PNGs take priority over original BMPs when present.
//!
//! # WASM
//!
//! On `wasm32` targets, BMP bytes are pre-fetched via HTTP during the loading
//! screen and stored in a static `WASM_BMP_CACHE`.  The WASM `load_texture()`
//! reads from this cache, decodes via `image::load_from_memory()`, and
//! registers the result as an egui texture — identical to the native path
//! minus filesystem I/O.
//!
//! Call [`set_bmp_cache()`] from the app's WASM loading screen after fetching
//! all BMP bytes, before the game loop starts.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use egui_macroquad::egui::{self, TextureHandle, TextureOptions};

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
    /// `EData` — planet assets and system backgrounds
    EData,
}

impl DllSource {
    /// Lowercase DLL name used as the staging directory prefix.
    ///
    /// Staging layout: `{base_path}/{dll_dir_name}/BMP/{id}.bmp`
    pub fn dll_dir_name(self) -> &'static str {
        match self {
            DllSource::Strategy => "strategy-dll",
            DllSource::Common   => "common-dll",
            DllSource::Tactical => "tactical-dll",
            DllSource::Gokres   => "gokres-dll",
            DllSource::EData    => "EData",
        }
    }

    /// Egui texture name prefix (for debug labels).
    pub fn texture_prefix(self) -> &'static str {
        match self {
            DllSource::Strategy => "strategy",
            DllSource::Common   => "common",
            DllSource::Tactical => "tactical",
            DllSource::Gokres   => "gokres",
            DllSource::EData    => "edata",
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
    /// Covers the global main-menu background and the standard strategy-view
    /// button library used by the cockpit controls.
    pub mod common {
        /// Main title-screen background.
        pub const MAIN_MENU_BG: u32 = 20001;

        /// Cockpit button: Officers.
        pub const BTN_OFFICERS_NORMAL: u32 = 11001;
        /// Cockpit button: Officers (pressed/active).
        pub const BTN_OFFICERS_PRESSED: u32 = 11002;
        /// Cockpit button: Officers (disabled).
        pub const BTN_OFFICERS_DISABLED: u32 = 11003;

        /// Cockpit button: Fleets.
        pub const BTN_FLEETS_NORMAL: u32 = 11004;
        /// Cockpit button: Fleets (pressed/active).
        pub const BTN_FLEETS_PRESSED: u32 = 11005;
        /// Cockpit button: Fleets (disabled).
        pub const BTN_FLEETS_DISABLED: u32 = 11006;

        /// Cockpit button: Manufacturing.
        pub const BTN_MANUFACTURING_NORMAL: u32 = 11007;
        /// Cockpit button: Manufacturing (pressed/active).
        pub const BTN_MANUFACTURING_PRESSED: u32 = 11008;
        /// Cockpit button: Manufacturing (disabled).
        pub const BTN_MANUFACTURING_DISABLED: u32 = 11009;

        /// Cockpit button: Missions.
        pub const BTN_MISSIONS_NORMAL: u32 = 11010;
        /// Cockpit button: Missions (pressed/active).
        pub const BTN_MISSIONS_PRESSED: u32 = 11011;
        /// Cockpit button: Missions (disabled).
        pub const BTN_MISSIONS_DISABLED: u32 = 11012;

        /// Cockpit button: Research.
        pub const BTN_RESEARCH_NORMAL: u32 = 11013;
        /// Cockpit button: Research (pressed/active).
        pub const BTN_RESEARCH_PRESSED: u32 = 11014;
        /// Cockpit button: Research (disabled).
        pub const BTN_RESEARCH_DISABLED: u32 = 11015;

        /// Cockpit button: Encyclopedia.
        pub const BTN_ENCYCLOPEDIA_NORMAL: u32 = 11016;
        /// Cockpit button: Encyclopedia (pressed/active).
        pub const BTN_ENCYCLOPEDIA_PRESSED: u32 = 11017;
        /// Cockpit button: Encyclopedia (disabled).
        pub const BTN_ENCYCLOPEDIA_DISABLED: u32 = 11018;

        /// Cockpit button: Save/Load.
        pub const BTN_SAVE_LOAD_NORMAL: u32 = 11019;
        /// Cockpit button: Save/Load (pressed/active).
        pub const BTN_SAVE_LOAD_PRESSED: u32 = 11020;
        /// Cockpit button: Save/Load (disabled).
        pub const BTN_SAVE_LOAD_DISABLED: u32 = 11021;

        /// Cockpit button: decrease game speed.
        pub const BTN_SPEED_DOWN_NORMAL: u32 = 11022;
        /// Cockpit button: decrease game speed (pressed/active).
        pub const BTN_SPEED_DOWN_PRESSED: u32 = 11023;
        /// Cockpit button: decrease game speed (disabled).
        pub const BTN_SPEED_DOWN_DISABLED: u32 = 11024;

        /// Cockpit button: increase game speed.
        pub const BTN_SPEED_UP_NORMAL: u32 = 11025;
        /// Cockpit button: increase game speed (pressed/active).
        pub const BTN_SPEED_UP_PRESSED: u32 = 11026;
        /// Cockpit button: increase game speed (disabled).
        pub const BTN_SPEED_UP_DISABLED: u32 = 11027;

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
        /// Main galaxy map starfield background.
        pub const GALAXY_BACKGROUND: u32 = 900;
        /// Galaxy display toggle: off.
        pub const GALAXY_DISPLAY_OFF: u32 = 902;
        /// Galaxy display toggle: on.
        pub const GALAXY_DISPLAY_ON: u32 = 903;

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
    /// Optional HD PNG override directory.  If `Some`, checked before `base_path`.
    hd_path: Option<PathBuf>,
    /// Cached textures for egui.
    textures: HashMap<(DllSource, u32), Option<TextureHandle>>,
    /// Cached textures for macroquad native rendering.
    mq_textures: HashMap<(DllSource, u32), Option<macroquad::prelude::Texture2D>>,
}

impl BmpCache {
    /// Create an empty cache with no path configured.
    pub fn new() -> Self {
        Self {
            base_path: None,
            hd_path: None,
            textures: HashMap::new(),
            mq_textures: HashMap::new(),
        }
    }

    /// Set the root directory that contains `{dll-name}-dll/BMP/` trees.
    ///
    /// Call before any `get()` or `preload_range()` invocations.
    pub fn set_base_path(&mut self, path: impl Into<PathBuf>) {
        self.base_path = Some(path.into());
    }

    /// Set an optional HD PNG override directory.
    ///
    /// Expected layout: `{hd_path}/{dll-name}/{resource_id}.png`
    pub fn set_hd_path(&mut self, path: impl Into<PathBuf>) {
        self.hd_path = Some(path.into());
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
            eprintln!("[bmp_cache] REQUEST: {:?} id {}", source, resource_id);
            let handle = self.load_texture(ctx, source, resource_id);
            if handle.is_some() {
                eprintln!("[bmp_cache] SUCCESS: {:?} id {}", source, resource_id);
            } else {
                eprintln!("[bmp_cache] FAILED: {:?} id {}", source, resource_id);
            }
            self.textures.insert(key, handle);
        }

        self.textures.get(&key)?.as_ref()
    }

    /// Retrieve a macroquad Texture2D by source DLL and resource ID.
    pub fn get_mq(
        &mut self,
        source: DllSource,
        resource_id: u32,
    ) -> Option<macroquad::prelude::Texture2D> {
        let key = (source, resource_id);

        if !self.mq_textures.contains_key(&key) {
            let tex = self.load_mq_texture(source, resource_id);
            self.mq_textures.insert(key, tex);
        }

        self.mq_textures.get(&key)?.clone()
    }

    /// Bulk-load all resources in `[start, end]` (inclusive) for one DLL.
    ///
    /// Useful for pre-warming the cache before the first frame that needs
    /// those textures, avoiding hitches.  Missing files are silently skipped.
    pub fn preload_range(
        &mut self,
        ctx: &egui::Context,
        source: DllSource,
        start: u32,
        end: u32,
    ) {
        for id in start..=end {
            let key = (source, id);
            if !self.textures.contains_key(&key) {
                let handle = self.load_texture(ctx, source, id);
                self.textures.insert(key, handle);
            }
        }
    }

    // ── Internal ────────────────────────────────────────────────────────────

    #[cfg(not(target_arch = "wasm32"))]
    fn load_texture(
        &self,
        ctx: &egui::Context,
        source: DllSource,
        resource_id: u32,
    ) -> Option<TextureHandle> {
        // 1. Check HD PNG override first.
        if let Some(hd_dir) = &self.hd_path {
            let hd_file = rebase_path_prefix(hd_dir, "data/hd", HD_PREFIX)
                .join(source.dll_dir_name())
                .join(format!("{}.png", resource_id));
            if hd_file.exists() {
                if let Some(handle) = load_image_as_texture(ctx, source, resource_id, &hd_file) {
                    return Some(handle);
                }
            }
        }

        // 2. Fall back to original staged BMP.
        let base = self.base_path.as_deref()?;
        let base_dir = rebase_path_prefix(base, "data/base", DATA_PREFIX);
        let bmp_file = if source == DllSource::EData {
            let root = base_dir.parent().unwrap_or(&base_dir);
            root.join(source.dll_dir_name()).join(format!("EDATA.{:03}", resource_id))
        } else {
            base_dir.join(source.dll_dir_name()).join("BMP").join(format!("{}.bmp", resource_id))
        };

        if bmp_file.exists() {
            load_image_as_texture(ctx, source, resource_id, &bmp_file)
        } else {
            eprintln!("[bmp_cache] WARNING: BMP not found at {:?}", bmp_file);
            None
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_mq_texture(
        &self,
        source: DllSource,
        resource_id: u32,
    ) -> Option<macroquad::prelude::Texture2D> {
        // 1. Check HD PNG override first.
        if let Some(hd_dir) = &self.hd_path {
            let hd_file = rebase_path_prefix(hd_dir, "data/hd", HD_PREFIX)
                .join(source.dll_dir_name())
                .join(format!("{}.png", resource_id));
            if hd_file.exists() {
                if let Some(tex) = load_image_as_mq_texture(&hd_file) {
                    return Some(tex);
                }
            }
        }

        // 2. Fall back to standard BMP.
        let base = self.base_path.as_deref()?;
        let base_dir = rebase_path_prefix(base, "data/base", DATA_PREFIX);
        let bmp_file = if source == DllSource::EData {
            let root = base_dir.parent().unwrap_or(&base_dir);
            root.join(source.dll_dir_name()).join(format!("EDATA.{:03}", resource_id))
        } else {
            base_dir.join(source.dll_dir_name()).join("BMP").join(format!("{}.bmp", resource_id))
        };

        if bmp_file.exists() {
            load_image_as_mq_texture(&bmp_file)
        } else {
            eprintln!("[bmp_cache] WARNING: MQ BMP not found at {:?}", bmp_file);
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

        // 1. Check HD PNG override first.
        let bytes = get_hd_bytes(dll_dir, resource_id)
            .or_else(|| get_bmp_bytes(dll_dir, resource_id))?;

        // 2. Decode and register as egui texture (same as native path).
        let img = match image::load_from_memory(&bytes) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("[bmp_cache] WASM decode failed for {}/{}: {}", dll_dir, resource_id, e);
                return None;
            }
        };
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [w as usize, h as usize],
            rgba.as_raw(),
        );

        Some(ctx.load_texture(
            &format!("{}_{}", source.texture_prefix(), resource_id),
            color_image,
            TextureOptions::default(),
        ))
    }

    #[cfg(target_arch = "wasm32")]
    fn load_mq_texture(
        &self,
        source: DllSource,
        resource_id: u32,
    ) -> Option<macroquad::prelude::Texture2D> {
        // Not implemented for WASM yet
        None
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
) -> Option<TextureHandle> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[bmp_cache] ERROR: Failed to read {:?}: {}", path, e);
            return None;
        }
    };

    // `image` crate auto-detects format from magic bytes — handles both BMP
    // (which may be palette-indexed) and PNG.
    let img = match image::load_from_memory(&bytes) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("[bmp_cache] ERROR: Failed to decode image from {:?}: {}", path, e);
            return None;
        }
    };
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [w as usize, h as usize],
        rgba.as_raw(),
    );

    let handle = ctx.load_texture(
        &format!("{}_{}", source.texture_prefix(), resource_id),
        color_image,
        TextureOptions::default(),
    );

    Some(handle)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_image_as_mq_texture(path: &Path) -> Option<macroquad::prelude::Texture2D> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[bmp_cache] ERROR: Failed to read {:?}: {}", path, e);
            return None;
        }
    };

    let img = match image::load_from_memory(&bytes) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("[bmp_cache] ERROR: Failed to decode image from {:?}: {}", path, e);
            return None;
        }
    };
    let rgba = img.to_rgba8();
    let width = rgba.width() as u16;
    let height = rgba.height() as u16;

    let tex = macroquad::prelude::Texture2D::from_rgba8(width, height, rgba.as_raw());
    tex.set_filter(macroquad::prelude::FilterMode::Linear);
    Some(tex)
}
