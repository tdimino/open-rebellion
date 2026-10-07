//! Encyclopedia viewer — browse ships, characters, and systems with artwork.
//!
//! Displays a floating egui window with tabs for capital ships, fighters,
//! characters, and star systems.  Each entity shows its 400×200 BMP artwork
//! loaded from `EData`/ alongside stats pulled from `GameWorld`.
//!
//! # EDATA mapping
//!
//! The original game stores encyclopedia images in sequentially numbered BMP
//! files (`EData/EDATA.NNN`).  The C# editor (`SwRebellionEditor`) reveals the
//! direct index mapping for entity types that don't go through `ENCYBMAP.DLL`:
//!
//! | Entity type          | First EDATA index |
//! |----------------------|-------------------|
//! | Production facilities | 1               |
//! | Manufacturing facs   | 3                 |
//! | Troops               | 15                |
//! | Special forces       | 25                |
//! | Fighters             | 34                |
//! | Capital ships        | 42                |
//! | Major characters     | 72                |
//! | Minor characters     | 78                |
//!
//! Star systems use a two-level lookup via `ENCYBMAP.DLL` (not yet implemented
//! here — systems fall back to a placeholder image).
//!
//! # Integration
//!
//! ```ignore
//! egui_macroquad::ui(|ctx| {
//!     if let Some(action) = draw_encyclopedia(ctx, world, &mut enc_state) {
//!         // handle action (currently none, may add focus-system in future)
//!     }
//! });
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use egui_macroquad::egui::TextureOptions;
use egui_macroquad::egui::{self, Color32, RichText, ScrollArea, TextureHandle, Vec2};
use rebellion_core::ids::{CapitalShipKey, CharacterKey, FighterKey, SystemKey};
use rebellion_core::world::GameWorld;

#[cfg(not(target_arch = "wasm32"))]
use crate::bmp_cache::{load_approved_hd_assets, validated_hd_bytes};
use crate::bmp_cache::{ApprovedHdAsset, AssetRenderProfile, BmpCache, DllSource};
pub use crate::encyclopedia_surface::{
    encyclopedia_index_list_action, encyclopedia_keyboard_action, EncyclopediaArtworkView,
    EncyclopediaSurface, EncyclopediaSurfaceAction, EncyclopediaSurfaceAudience,
    EncyclopediaSurfaceAvailability, EncyclopediaSurfaceCategory, EncyclopediaSurfaceKey,
    EncyclopediaSurfaceMode, EncyclopediaSurfaceNavigation, EncyclopediaSurfaceTopicItem,
};
pub use crate::encyclopedia_textures::{
    EncyclopediaTextureBackend, EncyclopediaTextureSampling, EncyclopediaTextureUpload,
    EncyclopediaTopicTextureCache,
};

#[cfg(target_arch = "wasm32")]
static WASM_EDATA_CACHE: std::sync::LazyLock<std::sync::Mutex<HashMap<String, Vec<u8>>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// Install original encyclopedia artwork unpacked from the browser runtime pack.
///
/// Keys are original filenames such as `EDATA.042`. GPU textures remain lazy:
/// opening a topic decodes only that topic's source bitmap, then retains the
/// resulting egui texture in [`EncyclopediaState`].
#[cfg(target_arch = "wasm32")]
pub fn set_encyclopedia_asset_cache(cache: HashMap<String, Vec<u8>>) {
    *WASM_EDATA_CACHE.lock().unwrap() = cache;
}

fn edata_filename(edata_n: u16) -> String {
    format!("EDATA.{edata_n:03}")
}

#[cfg(target_arch = "wasm32")]
fn wasm_edata_bytes(edata_n: u16) -> Option<Vec<u8>> {
    WASM_EDATA_CACHE
        .lock()
        .unwrap()
        .get(&edata_filename(edata_n))
        .cloned()
}

// ---------------------------------------------------------------------------
// Tab selection
// ---------------------------------------------------------------------------

/// Which entity category is currently displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncyclopediaTab {
    #[default]
    CapitalShips,
    Fighters,
    Characters,
    Systems,
}

// ---------------------------------------------------------------------------
// EncyclopediaState
// ---------------------------------------------------------------------------

/// All mutable state for the encyclopedia panel.
pub struct EncyclopediaState {
    /// Whether the panel is open.
    pub open: bool,
    /// Active tab.
    pub tab: EncyclopediaTab,
    /// Selected entity within the current tab (list index).
    pub selected_index: usize,
    /// Path to the `EData`/ directory (original BMPs).
    pub edata_path: Option<PathBuf>,
    /// Path to HD upscaled PNGs directory, used only by the faithful-HD profile.
    pub hd_path: Option<PathBuf>,
    /// Explicit asset profile. Original parity is the default.
    pub asset_profile: AssetRenderProfile,
    /// Selected original index category command (`0x6f..=0x75`).
    ///
    /// The replacement panel does not consume this field. It is retained for
    /// the source-exact index renderer that will replace that panel.
    pub original_category_command: u16,
    /// Stable compound object identity selected in the original index.
    pub original_selected_object_id: Option<u32>,
    /// First catalog row shown by the original index list.
    pub original_scroll_row: usize,
    /// EDATA keys explicitly approved by the faithful-HD manifest.
    approved_hd_assets: HashMap<String, ApprovedHdAsset>,
    /// Cached textures keyed by EDATA file number (1-based).
    textures: HashMap<u16, Option<TextureHandle>>,
}

impl EncyclopediaState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure the `EData` directory.  Call before opening the encyclopedia.
    pub fn set_edata_path(&mut self, path: impl Into<PathBuf>) {
        self.edata_path = Some(path.into());
        self.textures.clear();
    }

    /// Configure the HD upscaled PNG directory. Expected naming:
    /// `EDATA_NNN.png`. Setting a path does not enable HD substitution.
    pub fn set_hd_path(&mut self, path: impl Into<PathBuf>) {
        let path = path.into();
        #[cfg(not(target_arch = "wasm32"))]
        {
            let manifest_root = path.parent().unwrap_or(&path);
            self.approved_hd_assets = load_approved_hd_assets(manifest_root);
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.approved_hd_assets.clear();
        }
        self.hd_path = Some(path);
        self.textures.clear();
    }

    /// Change the explicit render profile and invalidate prior textures.
    pub fn set_asset_profile(&mut self, profile: AssetRenderProfile) {
        if self.asset_profile != profile {
            self.asset_profile = profile;
            self.textures.clear();
        }
    }
}

impl Default for EncyclopediaState {
    fn default() -> Self {
        EncyclopediaState {
            open: false,
            tab: EncyclopediaTab::default(),
            selected_index: 0,
            edata_path: None,
            hd_path: None,
            asset_profile: AssetRenderProfile::OriginalParity,
            original_category_command: 0x6f,
            original_selected_object_id: None,
            original_scroll_row: 0,
            approved_hd_assets: HashMap::new(),
            textures: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Public draw function
// ---------------------------------------------------------------------------

/// Render the encyclopedia window inside an `egui_macroquad::ui` closure.
///
/// Returns `Some(SystemKey)` when the user clicks "Zoom to system" on a
/// star system entry (caller should pan the galaxy map to that system).
/// Returns `None` otherwise.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
pub fn draw_encyclopedia(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut EncyclopediaState,
    bmp_cache: &mut BmpCache,
) -> Option<SystemKey> {
    if !state.open {
        return None;
    }

    let mut focus_system: Option<SystemKey> = None;

    // Extract fields that the Window::open() needs to borrow independently
    // of the closure's mutable borrow of `state`.
    let mut window_open = state.open;

    egui::Window::new("Encyclopedia")
        .default_size([700.0, 520.0])
        .min_width(480.0)
        .min_height(360.0)
        .collapsible(true)
        .open(&mut window_open)
        .show(ctx, |ui| {
            // ── Tab bar ───────────────────────────────────────────────────
            ui.horizontal(|ui| {
                for (label, tab) in [
                    ("Capital Ships", EncyclopediaTab::CapitalShips),
                    ("Fighters", EncyclopediaTab::Fighters),
                    ("Characters", EncyclopediaTab::Characters),
                    ("Systems", EncyclopediaTab::Systems),
                ] {
                    if ui.selectable_label(state.tab == tab, label).clicked() && state.tab != tab {
                        state.tab = tab;
                        state.selected_index = 0;
                    }
                }
            });
            ui.separator();

            // ── Two-column layout: list (left) | detail (right) ───────────
            ui.columns(2, |cols| {
                // Left: scrollable entity list
                let list_ui = &mut cols[0];
                ScrollArea::vertical()
                    .id_salt("enc_list")
                    .show(list_ui, |ui| match state.tab {
                        EncyclopediaTab::CapitalShips => {
                            let keys: Vec<(CapitalShipKey, &str)> = world
                                .capital_ship_classes
                                .iter()
                                .map(|(k, c)| (k, c.name.as_str()))
                                .collect();
                            for (i, (_, name)) in keys.iter().enumerate() {
                                let sel = state.selected_index == i;
                                if ui.selectable_label(sel, *name).clicked() {
                                    state.selected_index = i;
                                }
                            }
                        }
                        EncyclopediaTab::Fighters => {
                            let keys: Vec<(FighterKey, &str)> = world
                                .fighter_classes
                                .iter()
                                .map(|(k, c)| (k, c.name.as_str()))
                                .collect();
                            for (i, (_, name)) in keys.iter().enumerate() {
                                let sel = state.selected_index == i;
                                if ui.selectable_label(sel, *name).clicked() {
                                    state.selected_index = i;
                                }
                            }
                        }
                        EncyclopediaTab::Characters => {
                            let chars: Vec<(CharacterKey, &str)> = world
                                .characters
                                .iter()
                                .map(|(k, c)| (k, c.name.as_str()))
                                .collect();
                            for (i, (_, name)) in chars.iter().enumerate() {
                                let sel = state.selected_index == i;
                                if ui.selectable_label(sel, *name).clicked() {
                                    state.selected_index = i;
                                }
                            }
                        }
                        EncyclopediaTab::Systems => {
                            let systems: Vec<(SystemKey, &str)> = world
                                .systems
                                .iter()
                                .map(|(k, s)| (k, s.name.as_str()))
                                .collect();
                            for (i, (_, name)) in systems.iter().enumerate() {
                                let sel = state.selected_index == i;
                                if ui.selectable_label(sel, *name).clicked() {
                                    state.selected_index = i;
                                }
                            }
                        }
                    });

                // Right: detail pane
                let detail_ui = &mut cols[1];
                ScrollArea::vertical()
                    .id_salt("enc_detail")
                    .show(detail_ui, |ui| {
                        match state.tab {
                            EncyclopediaTab::CapitalShips => {
                                let ships: Vec<CapitalShipKey> =
                                    world.capital_ship_classes.keys().collect();
                                if let Some(&key) = ships.get(state.selected_index) {
                                    if let Some(ship) = world.capital_ship_classes.get(key) {
                                        // GOKRES.DLL 122×50 ship status sprite.
                                        // Formula: resource_id = dat_id.raw() + 1024.
                                        // Ships without a sprite fall through to the EDATA image.
                                        let gokres_id = ship.dat_id.raw() + 1024;
                                        if let Some(tex) =
                                            bmp_cache.get(ctx, DllSource::Gokres, gokres_id)
                                        {
                                            ui.add(
                                                egui::Image::new(tex)
                                                    .fit_to_exact_size(Vec2::new(122.0, 50.0)),
                                            );
                                        }

                                        // EDATA offset for capital ships: 42 + 0-based index
                                        let edata_n = 42u16 + state.selected_index as u16;
                                        show_edata_image(ui, ctx, edata_n, state);
                                        ui.add_space(4.0);
                                        ui.heading(&ship.name);
                                        ui.separator();
                                        let faction = match (ship.is_alliance, ship.is_empire) {
                                            (true, false) => "Alliance",
                                            (false, true) => "Empire",
                                            _ => "Both",
                                        };
                                        stat_row(ui, "Faction", faction);
                                        stat_row(ui, "Hull", &ship.hull.to_string());
                                        stat_row(ui, "Shields", &ship.shield_strength.to_string());
                                        stat_row(
                                            ui,
                                            "Sublight",
                                            &ship.sub_light_engine.to_string(),
                                        );
                                        stat_row(ui, "Hyperdrive", &ship.hyperdrive.to_string());
                                        stat_row(ui, "Maneuver", &ship.maneuverability.to_string());
                                        stat_row(
                                            ui,
                                            "Fighters",
                                            &ship.fighter_capacity.to_string(),
                                        );
                                        stat_row(ui, "Troops", &ship.troop_capacity.to_string());
                                        stat_row(
                                            ui,
                                            "Build cost",
                                            &ship.refined_material_cost.to_string(),
                                        );
                                        stat_row(
                                            ui,
                                            "Maintenance",
                                            &ship.maintenance_cost.to_string(),
                                        );
                                        stat_row(
                                            ui,
                                            "Research order",
                                            &ship.research_order.to_string(),
                                        );
                                        stat_row(
                                            ui,
                                            "Build time",
                                            &ship.research_difficulty.to_string(),
                                        );
                                    }
                                }
                            }
                            EncyclopediaTab::Fighters => {
                                let fighters: Vec<FighterKey> =
                                    world.fighter_classes.keys().collect();
                                if let Some(&key) = fighters.get(state.selected_index) {
                                    if let Some(ftr) = world.fighter_classes.get(key) {
                                        // EDATA offset for fighters: 34 + 0-based index
                                        let edata_n = 34u16 + state.selected_index as u16;
                                        show_edata_image(ui, ctx, edata_n, state);
                                        ui.add_space(4.0);
                                        ui.heading(&ftr.name);
                                        ui.separator();
                                        let faction = match (ftr.is_alliance, ftr.is_empire) {
                                            (true, false) => "Alliance",
                                            (false, true) => "Empire",
                                            _ => "Both",
                                        };
                                        stat_row(ui, "Faction", faction);
                                        stat_row(
                                            ui,
                                            "Squadron size",
                                            &ftr.squadron_size.to_string(),
                                        );
                                        stat_row(ui, "Torpedoes", &ftr.torpedoes.to_string());
                                        stat_row(
                                            ui,
                                            "Build cost",
                                            &ftr.refined_material_cost.to_string(),
                                        );
                                        stat_row(
                                            ui,
                                            "Maintenance",
                                            &ftr.maintenance_cost.to_string(),
                                        );
                                    }
                                }
                            }
                            EncyclopediaTab::Characters => {
                                let chars: Vec<CharacterKey> = world.characters.keys().collect();
                                if let Some(&key) = chars.get(state.selected_index) {
                                    if let Some(chr) = world.characters.get(key) {
                                        // Major characters (0..5) → EDATA 72+; minor (6+) → 78+
                                        // We don't have a major/minor flag split by index here,
                                        // but world.characters stores major first (load order).
                                        // Use 72 for first 6, 78 for the rest.
                                        let edata_n = if state.selected_index < 6 {
                                            72u16 + state.selected_index as u16
                                        } else {
                                            78u16 + (state.selected_index - 6) as u16
                                        };
                                        show_edata_image(ui, ctx, edata_n, state);
                                        ui.add_space(4.0);
                                        ui.heading(&chr.name);
                                        ui.separator();
                                        let kind = if chr.is_major {
                                            "Major character"
                                        } else {
                                            "Minor character"
                                        };
                                        stat_row(ui, "Type", kind);
                                        stat_row_pair(
                                            ui,
                                            "Diplomacy",
                                            chr.diplomacy.base,
                                            chr.diplomacy.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Espionage",
                                            chr.espionage.base,
                                            chr.espionage.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Ship Design",
                                            chr.ship_design.base,
                                            chr.ship_design.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Troop Training",
                                            chr.troop_training.base,
                                            chr.troop_training.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Facility Design",
                                            chr.facility_design.base,
                                            chr.facility_design.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Combat",
                                            chr.combat.base,
                                            chr.combat.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Leadership",
                                            chr.leadership.base,
                                            chr.leadership.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Loyalty",
                                            chr.loyalty.base,
                                            chr.loyalty.variance,
                                        );
                                        if chr.jedi_probability > 0 {
                                            stat_row(
                                                ui,
                                                "Jedi probability",
                                                &format!("{}%", chr.jedi_probability),
                                            );
                                        }
                                        let mut roles = Vec::new();
                                        if chr.can_be_admiral {
                                            roles.push("Admiral");
                                        }
                                        if chr.can_be_general {
                                            roles.push("General");
                                        }
                                        if chr.can_be_commander {
                                            roles.push("Commander");
                                        }
                                        if !roles.is_empty() {
                                            stat_row(ui, "Roles", &roles.join(", "));
                                        }
                                    }
                                }
                            }
                            EncyclopediaTab::Systems => {
                                let systems: Vec<SystemKey> = world.systems.keys().collect();
                                if let Some(&key) = systems.get(state.selected_index) {
                                    if let Some(system) = world.systems.get(key) {
                                        // Systems use ENCYBMAP.DLL for their image index.
                                        // Until ENCYBMAP is parsed, show a placeholder.
                                        show_placeholder_image(ui);
                                        ui.add_space(4.0);
                                        ui.heading(&system.name);
                                        ui.separator();
                                        if let Some(sector) = world.sectors.get(system.sector) {
                                            stat_row(ui, "Sector", &sector.name);
                                            let region = match sector.group {
                                                rebellion_core::dat::SectorGroup::Core => "Core",
                                                rebellion_core::dat::SectorGroup::RimInner => {
                                                    "Inner Rim"
                                                }
                                                rebellion_core::dat::SectorGroup::RimOuter => {
                                                    "Outer Rim"
                                                }
                                            };
                                            stat_row(ui, "Region", region);
                                        }
                                        stat_row(
                                            ui,
                                            "Position",
                                            &format!("({}, {})", system.x, system.y),
                                        );
                                        stat_row(
                                            ui,
                                            "Alliance",
                                            &format!("{:.0}%", system.popularity_alliance * 100.0),
                                        );
                                        stat_row(
                                            ui,
                                            "Empire",
                                            &format!("{:.0}%", system.popularity_empire * 100.0),
                                        );
                                        stat_row(ui, "Fleets", &system.fleets.len().to_string());
                                        stat_row(
                                            ui,
                                            "Defenses",
                                            &system.defense_facilities.len().to_string(),
                                        );
                                        stat_row(
                                            ui,
                                            "Shipyards",
                                            &system.manufacturing_facilities.len().to_string(),
                                        );

                                        ui.add_space(6.0);
                                        if ui.small_button("Zoom to system on map").clicked() {
                                            focus_system = Some(key);
                                        }
                                    }
                                }
                            }
                        }
                    });
            });
        });

    // Write back the open flag (egui sets it to false when the X button is clicked).
    state.open = window_open;

    focus_system
}

// ---------------------------------------------------------------------------
// Original index shell
// ---------------------------------------------------------------------------

/// Native Galactic Encyclopedia window dimensions from `FUN_00429f30`.
pub const ENCYCLOPEDIA_INDEX_WIDTH: f32 = 470.0;
pub const ENCYCLOPEDIA_INDEX_HEIGHT: f32 = 330.0;

const ORIGINAL_INDEX_VISIBLE_ROWS: usize = 9;
const ORIGINAL_INDEX_ROW_HEIGHT: f32 = 18.0;

/// One immutable source object shown by the authentic index list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalEncyclopediaEntry {
    pub object_id: u32,
    pub name: String,
}

impl OriginalEncyclopediaEntry {
    #[must_use]
    pub const fn family(&self) -> u8 {
        (self.object_id >> 24) as u8
    }
}

/// Localized source catalog transported from the original DAT/TEXTSTRA data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalEncyclopediaCatalog {
    pub title: String,
    pub topic_label: String,
    category_labels: [String; 7],
    entries: Vec<OriginalEncyclopediaEntry>,
}

impl OriginalEncyclopediaCatalog {
    #[must_use]
    pub fn new(
        title: String,
        topic_label: String,
        category_labels: [String; 7],
        mut entries: Vec<OriginalEncyclopediaEntry>,
    ) -> Self {
        entries.sort_by(|left, right| {
            left.name
                .to_lowercase()
                .cmp(&right.name.to_lowercase())
                .then_with(|| left.object_id.cmp(&right.object_id))
        });
        entries.dedup_by_key(|entry| entry.object_id);
        Self {
            title,
            topic_label,
            category_labels,
            entries,
        }
    }

    #[must_use]
    pub fn category_label(&self, command_id: u16) -> Option<&str> {
        let index = usize::from(command_id.checked_sub(0x6f)?);
        self.category_labels.get(index).map(String::as_str)
    }

    #[must_use]
    pub fn entries_for(&self, command_id: u16) -> Vec<&OriginalEncyclopediaEntry> {
        if !(0x6f..=0x75).contains(&command_id) {
            return Vec::new();
        }
        self.entries
            .iter()
            .filter(|entry| original_category_contains(command_id, entry.family()))
            .collect()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

const fn original_category_contains(command_id: u16, family: u8) -> bool {
    match command_id {
        0x6f => true,
        0x70 => family >= 0x90 && family < 0x98,
        0x71 => family >= 0x14 && family < 0x20,
        0x72 => family >= 0x20 && family < 0x30,
        0x73 => family >= 0x40 && family < 0x80,
        0x74 => family >= 0x10 && family < 0x14,
        0x75 => family >= 0x30 && family < 0x40,
        _ => false,
    }
}

const INDEX_CONTENT: u32 = 10_338;
const TOPIC_CONTENT: u32 = 10_337;
const INDEX_CONTENT_X: f32 = 12.0;
const INDEX_CONTENT_Y: f32 = 13.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OriginalControlSpec {
    command_id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    normal_resource: u32,
    pressed_resource: u32,
    selected_in_index: bool,
}

const fn encyclopedia_category_controls(
    faction: crate::cockpit::CockpitFaction,
) -> [OriginalControlSpec; 7] {
    let (ships_normal, ships_pressed, facilities_normal, facilities_pressed) = match faction {
        crate::cockpit::CockpitFaction::Alliance => (10_348, 10_347, 10_344, 10_343),
        crate::cockpit::CockpitFaction::Empire => (10_360, 10_359, 10_356, 10_355),
    };
    let (personnel_normal, personnel_pressed, missions_normal, missions_pressed) = match faction {
        crate::cockpit::CockpitFaction::Alliance => (10_346, 10_345, 11_616, 11_615),
        crate::cockpit::CockpitFaction::Empire => (10_358, 10_357, 11_618, 11_617),
    };
    let (troops_normal, troops_pressed) = match faction {
        crate::cockpit::CockpitFaction::Alliance => (10_352, 10_351),
        crate::cockpit::CockpitFaction::Empire => (10_362, 10_361),
    };
    [
        OriginalControlSpec {
            command_id: 0x6f,
            x: 36,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: 10_340,
            pressed_resource: 10_339,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x70,
            x: 88,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: 10_350,
            pressed_resource: 10_349,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x71,
            x: 140,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: ships_normal,
            pressed_resource: ships_pressed,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x72,
            x: 192,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: facilities_normal,
            pressed_resource: facilities_pressed,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x73,
            x: 244,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: missions_normal,
            pressed_resource: missions_pressed,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x74,
            x: 296,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: troops_normal,
            pressed_resource: troops_pressed,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x75,
            x: 348,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: personnel_normal,
            pressed_resource: personnel_pressed,
            selected_in_index: false,
        },
    ]
}

const fn encyclopedia_rail_controls(
    faction: crate::cockpit::CockpitFaction,
) -> [OriginalControlSpec; 3] {
    match faction {
        crate::cockpit::CockpitFaction::Alliance => [
            OriginalControlSpec {
                command_id: 0xfb,
                x: 423,
                y: 25,
                width: 32,
                height: 31,
                normal_resource: 10_370,
                pressed_resource: 10_371,
                selected_in_index: false,
            },
            OriginalControlSpec {
                command_id: 0x67,
                x: 423,
                y: 93,
                width: 32,
                height: 31,
                normal_resource: 10_374,
                pressed_resource: 10_375,
                selected_in_index: false,
            },
            OriginalControlSpec {
                command_id: 0x68,
                x: 423,
                y: 147,
                width: 32,
                height: 31,
                normal_resource: 10_372,
                pressed_resource: 10_373,
                selected_in_index: true,
            },
        ],
        crate::cockpit::CockpitFaction::Empire => [
            OriginalControlSpec {
                command_id: 0xfb,
                x: 426,
                y: 21,
                width: 44,
                height: 41,
                normal_resource: 10_376,
                pressed_resource: 10_377,
                selected_in_index: false,
            },
            OriginalControlSpec {
                command_id: 0x67,
                x: 426,
                y: 89,
                width: 44,
                height: 41,
                normal_resource: 10_380,
                pressed_resource: 10_381,
                selected_in_index: false,
            },
            OriginalControlSpec {
                command_id: 0x68,
                x: 426,
                y: 143,
                width: 44,
                height: 41,
                normal_resource: 10_378,
                pressed_resource: 10_379,
                selected_in_index: true,
            },
        ],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SurfaceControlSpec {
    command_id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    normal_resource: u32,
    pressed_resource: u32,
    disabled_resource: Option<u32>,
    selected: bool,
}

const fn encyclopedia_rail_controls_for_mode(
    faction: crate::cockpit::CockpitFaction,
    mode: EncyclopediaSurfaceMode,
) -> [SurfaceControlSpec; 3] {
    let legacy = encyclopedia_rail_controls(faction);
    [
        surface_control(legacy[0], None, false),
        surface_control(
            legacy[1],
            None,
            matches!(mode, EncyclopediaSurfaceMode::Topic),
        ),
        surface_control(
            legacy[2],
            None,
            matches!(mode, EncyclopediaSurfaceMode::Index),
        ),
    ]
}

const fn surface_control(
    control: OriginalControlSpec,
    disabled_resource: Option<u32>,
    selected: bool,
) -> SurfaceControlSpec {
    SurfaceControlSpec {
        command_id: control.command_id,
        x: control.x,
        y: control.y,
        width: control.width,
        height: control.height,
        normal_resource: control.normal_resource,
        pressed_resource: control.pressed_resource,
        disabled_resource,
        selected,
    }
}

const fn encyclopedia_topic_controls(
    _faction: crate::cockpit::CockpitFaction,
) -> [SurfaceControlSpec; 2] {
    [
        SurfaceControlSpec {
            command_id: 0x83,
            x: 28,
            y: 14,
            width: 21,
            height: 17,
            normal_resource: 10_385,
            pressed_resource: 10_386,
            disabled_resource: Some(10_387),
            selected: false,
        },
        SurfaceControlSpec {
            command_id: 0x84,
            x: 380,
            y: 14,
            width: 21,
            height: 17,
            normal_resource: 10_382,
            pressed_resource: 10_383,
            disabled_resource: Some(10_384),
            selected: false,
        },
    ]
}

struct EguiEncyclopediaTextureBackend {
    context: egui::Context,
}

impl EncyclopediaTextureBackend for EguiEncyclopediaTextureBackend {
    type Texture = TextureHandle;

    fn upload(&mut self, upload: EncyclopediaTextureUpload<'_>) -> Result<Self::Texture, String> {
        let width = usize::try_from(upload.width)
            .map_err(|_| format!("{} width does not fit usize", upload.filename))?;
        let height = usize::try_from(upload.height)
            .map_err(|_| format!("{} height does not fit usize", upload.filename))?;
        let image = egui::ColorImage::from_rgba_unmultiplied([width, height], upload.rgba);
        Ok(self.context.load_texture(
            format!("encyclopedia:{}:{}", upload.filename, upload.digest),
            image,
            match upload.sampling {
                EncyclopediaTextureSampling::Nearest => TextureOptions::NEAREST,
                EncyclopediaTextureSampling::Linear => TextureOptions::LINEAR,
            },
        ))
    }

    fn release(&mut self, texture: Self::Texture) {
        drop(texture);
    }
}

/// Local view state for the authentic, graphics-neutral Encyclopedia surface.
///
/// Selection and routing remain application/presenter state. This object owns
/// only pixels and viewport offsets, and retains at most one topic texture.
pub struct EncyclopediaSurfaceState {
    pub scroll_row: usize,
    body_scroll: f32,
    captured_control: Option<u16>,
    focused_mode: Option<EncyclopediaSurfaceMode>,
    active_topic_key: Option<(u64, u32)>,
    topic_textures: Option<EncyclopediaTopicTextureCache<EguiEncyclopediaTextureBackend>>,
    last_texture_error: Option<String>,
}

impl Default for EncyclopediaSurfaceState {
    fn default() -> Self {
        Self {
            scroll_row: 0,
            body_scroll: 0.0,
            captured_control: None,
            focused_mode: None,
            active_topic_key: None,
            topic_textures: None,
            last_texture_error: None,
        }
    }
}

impl EncyclopediaSurfaceState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn reconcile_active_topic(&mut self, generation: u64, object_id: Option<u32>) {
        let next = object_id.map(|object_id| (generation, object_id));
        if self.active_topic_key != next {
            self.body_scroll = 0.0;
            self.active_topic_key = next;
        }
    }

    fn resolve_topic_texture(
        &mut self,
        ctx: &egui::Context,
        surface: &EncyclopediaSurface<'_>,
    ) -> Option<egui::TextureId> {
        let cache = self.topic_textures.get_or_insert_with(|| {
            EncyclopediaTopicTextureCache::new(EguiEncyclopediaTextureBackend {
                context: ctx.clone(),
            })
        });
        let artwork = surface
            .active_topic
            .as_ref()
            .and_then(|topic| topic.artwork.as_ref());
        match cache.resolve(surface.texture_generation, artwork) {
            Ok(resolution) => {
                self.last_texture_error = None;
                resolution.texture.map(TextureHandle::id)
            }
            Err(error) => {
                if self.last_texture_error.as_deref() != Some(&error) {
                    eprintln!("[encyclopedia] topic artwork unavailable: {error}");
                }
                self.last_texture_error = Some(error);
                None
            }
        }
    }
}

/// Paint one source-backed authentic index or topic frame.
///
/// The function consumes only the renderer-owned DTO and emits logical
/// presenter actions; it never reads campaign state, files, or source catalogs.
pub fn draw_encyclopedia_surface(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    origin: egui::Pos2,
    scale: f32,
    state: &mut EncyclopediaSurfaceState,
    surface: &EncyclopediaSurface<'_>,
) -> Option<EncyclopediaSurfaceAction> {
    if scale <= 0.0 || surface.categories.is_empty() {
        return None;
    }
    let faction = match surface.audience {
        EncyclopediaSurfaceAudience::Alliance => crate::cockpit::CockpitFaction::Alliance,
        EncyclopediaSurfaceAudience::Empire => crate::cockpit::CockpitFaction::Empire,
    };
    let effective_selection = surface
        .navigation
        .selected_object_id
        .or_else(|| surface.topics.first().map(|topic| topic.object_id));
    reconcile_surface_scroll(state, surface, effective_selection);
    state.reconcile_active_topic(
        surface.texture_generation,
        surface.active_topic.as_ref().map(|topic| topic.object_id),
    );
    let topic_texture = state.resolve_topic_texture(ctx, surface);
    if ctx.input(|input| input.pointer.primary_pressed()) {
        state.captured_control = None;
    }
    let mut action = None;
    let mut surface_rect = None;
    let keyboard_focus_id = surface_keyboard_focus_id(surface.mode);

    let surface_id = egui::Id::new("authentic-encyclopedia-surface");
    ctx.move_to_top(egui::LayerId::new(egui::Order::Foreground, surface_id));
    egui::Area::new(surface_id)
        .fixed_pos(origin)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let size = egui::vec2(
                ENCYCLOPEDIA_INDEX_WIDTH * scale,
                ENCYCLOPEDIA_INDEX_HEIGHT * scale,
            );
            let (window_rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
            surface_rect = Some(window_rect);
            let (base, rail) = match faction {
                crate::cockpit::CockpitFaction::Alliance => (10_335, 10_585),
                crate::cockpit::CockpitFaction::Empire => (10_336, 10_589),
            };
            paint_original_resource_native(
                ui.painter(),
                ctx,
                cache,
                base,
                window_rect.min,
                scale,
                window_rect,
            );
            paint_original_resource_native(
                ui.painter(),
                ctx,
                cache,
                rail,
                encyclopedia_point(window_rect, scale, 412.0, 0.0),
                scale,
                window_rect,
            );
            let content_rect = encyclopedia_rect(
                window_rect,
                scale,
                12.0,
                if surface.mode == EncyclopediaSurfaceMode::Index {
                    13.0
                } else {
                    14.0
                },
                400.0,
                306.0,
            );
            paint_original_resource_native(
                ui.painter(),
                ctx,
                cache,
                if surface.mode == EncyclopediaSurfaceMode::Index {
                    INDEX_CONTENT
                } else {
                    TOPIC_CONTENT
                },
                content_rect.min,
                scale,
                content_rect,
            );

            let focus_rect = if surface.mode == EncyclopediaSurfaceMode::Index {
                original_index_list_rect(window_rect, scale)
            } else {
                encyclopedia_rect(window_rect, scale, 17.0, 231.0, 395.0, 80.0)
            };
            ui.interact(
                focus_rect,
                keyboard_focus_id,
                egui::Sense::focusable_noninteractive(),
            );
            if state.focused_mode != Some(surface.mode) {
                ctx.memory_mut(|memory| memory.request_focus(keyboard_focus_id));
                state.focused_mode = Some(surface.mode);
            }
            ctx.memory_mut(|memory| {
                memory.set_focus_lock_filter(
                    keyboard_focus_id,
                    egui::EventFilter {
                        tab: true,
                        horizontal_arrows: true,
                        vertical_arrows: true,
                        escape: true,
                    },
                );
            });

            if surface.mode == EncyclopediaSurfaceMode::Topic {
                draw_surface_topic(ui, window_rect, scale, state, surface, topic_texture);
                for (index, control) in encyclopedia_topic_controls(faction).into_iter().enumerate()
                {
                    let destination = if index == 0 {
                        surface.navigation.previous_object_id
                    } else {
                        surface.navigation.next_object_id
                    };
                    if draw_surface_control(
                        ui,
                        ctx,
                        cache,
                        window_rect,
                        scale,
                        control,
                        destination.is_some(),
                        &mut state.captured_control,
                    ) {
                        action = destination.map(EncyclopediaSurfaceAction::OpenTopic);
                    }
                }
            } else {
                draw_surface_index(
                    ui,
                    window_rect,
                    scale,
                    state,
                    surface,
                    effective_selection,
                    &mut action,
                );
                for control in encyclopedia_category_controls(faction) {
                    if draw_original_control(
                        ui,
                        ctx,
                        cache,
                        window_rect,
                        scale,
                        control,
                        control.command_id == surface.category.command_id,
                    ) {
                        action = Some(EncyclopediaSurfaceAction::SelectCategory(
                            control.command_id,
                        ));
                    }
                }
            }

            for control in encyclopedia_rail_controls_for_mode(faction, surface.mode) {
                if draw_surface_control(
                    ui,
                    ctx,
                    cache,
                    window_rect,
                    scale,
                    control,
                    true,
                    &mut state.captured_control,
                ) {
                    action = match control.command_id {
                        0xfb => Some(EncyclopediaSurfaceAction::Close),
                        0x67 if surface.mode == EncyclopediaSurfaceMode::Index => {
                            effective_selection.map(EncyclopediaSurfaceAction::OpenTopic)
                        }
                        0x68 if surface.mode == EncyclopediaSurfaceMode::Topic => {
                            Some(EncyclopediaSurfaceAction::ShowIndex)
                        }
                        _ => action,
                    };
                }
            }
        });

    if ctx.input(|input| input.pointer.primary_pressed())
        && surface_rect.is_some_and(|rect| {
            ctx.pointer_latest_pos()
                .is_some_and(|point| rect.contains(point))
        })
    {
        ctx.memory_mut(|memory| memory.request_focus(keyboard_focus_id));
    }

    if ctx.input(|input| input.pointer.button_released(egui::PointerButton::Primary)) {
        state.captured_control = None;
    }

    action.or_else(|| {
        if ctx.memory(|memory| memory.has_focus(keyboard_focus_id)) {
            surface_keyboard_input(ctx, state, surface, effective_selection)
        } else {
            None
        }
    })
}

fn surface_keyboard_focus_id(mode: EncyclopediaSurfaceMode) -> egui::Id {
    egui::Id::new((
        "authentic-encyclopedia-keyboard-focus",
        match mode {
            EncyclopediaSurfaceMode::Index => 0_u8,
            EncyclopediaSurfaceMode::Topic => 1,
        },
    ))
}

fn reconcile_surface_scroll(
    state: &mut EncyclopediaSurfaceState,
    surface: &EncyclopediaSurface<'_>,
    selection: Option<u32>,
) {
    let max_scroll = surface
        .topics
        .len()
        .saturating_sub(ORIGINAL_INDEX_VISIBLE_ROWS);
    state.scroll_row = state.scroll_row.min(max_scroll);
    if let Some(position) = selection.and_then(|selected| {
        surface
            .topics
            .iter()
            .position(|topic| topic.object_id == selected)
    }) {
        if position < state.scroll_row {
            state.scroll_row = position;
        } else if position >= state.scroll_row + ORIGINAL_INDEX_VISIBLE_ROWS {
            state.scroll_row = position + 1 - ORIGINAL_INDEX_VISIBLE_ROWS;
        }
    }
}

fn draw_surface_index(
    ui: &mut egui::Ui,
    window_rect: egui::Rect,
    scale: f32,
    state: &mut EncyclopediaSurfaceState,
    surface: &EncyclopediaSurface<'_>,
    effective_selection: Option<u32>,
    action: &mut Option<EncyclopediaSurfaceAction>,
) {
    let title_font = egui::FontId::proportional(15.0 * scale);
    let list_font = egui::FontId::proportional(14.0 * scale);
    let selected_name = effective_selection
        .and_then(|selected| {
            surface
                .topics
                .iter()
                .find(|topic| topic.object_id == selected)
        })
        .map_or("", |topic| topic.title);
    ui.painter().text(
        encyclopedia_point(window_rect, scale, 211.0, 14.0),
        egui::Align2::CENTER_TOP,
        surface.title,
        title_font,
        Color32::WHITE,
    );
    ui.painter().text(
        encyclopedia_point(window_rect, scale, 36.0, 48.0),
        egui::Align2::LEFT_TOP,
        surface.topic_label,
        list_font.clone(),
        Color32::WHITE,
    );
    ui.painter().text(
        encyclopedia_point(window_rect, scale, 143.0, 47.0),
        egui::Align2::LEFT_TOP,
        selected_name,
        list_font.clone(),
        Color32::WHITE,
    );
    ui.painter().text(
        encyclopedia_point(window_rect, scale, 40.0, 120.0),
        egui::Align2::LEFT_TOP,
        surface.category.label,
        list_font.clone(),
        Color32::WHITE,
    );

    let max_scroll = surface
        .topics
        .len()
        .saturating_sub(ORIGINAL_INDEX_VISIBLE_ROWS);
    let list_rect = original_index_list_rect(window_rect, scale);
    let list_painter = ui.painter().with_clip_rect(list_rect);
    if ui.rect_contains_pointer(list_rect) {
        let wheel = ui.input(|input| input.raw_scroll_delta.y);
        if wheel.abs() > f32::EPSILON {
            let rows = ((wheel.abs() / 24.0).ceil() as usize).clamp(1, 3);
            state.scroll_row = if wheel < 0.0 {
                state.scroll_row.saturating_add(rows).min(max_scroll)
            } else {
                state.scroll_row.saturating_sub(rows)
            };
        }
    }
    let up_rect = encyclopedia_rect(window_rect, scale, 374.0, 137.0, 12.0, 13.0);
    let down_rect = encyclopedia_rect(window_rect, scale, 374.0, 284.0, 12.0, 13.0);
    if ui
        .interact(
            up_rect,
            ui.id().with("authentic-encyclopedia-scroll-up"),
            egui::Sense::click(),
        )
        .clicked()
    {
        state.scroll_row = state.scroll_row.saturating_sub(1);
    }
    if ui
        .interact(
            down_rect,
            ui.id().with("authentic-encyclopedia-scroll-down"),
            egui::Sense::click(),
        )
        .clicked()
    {
        state.scroll_row = state.scroll_row.saturating_add(1).min(max_scroll);
    }
    for (visible_row, topic) in surface
        .topics
        .iter()
        .skip(state.scroll_row)
        .take(ORIGINAL_INDEX_VISIBLE_ROWS)
        .enumerate()
    {
        let y = 137.0 + visible_row as f32 * ORIGINAL_INDEX_ROW_HEIGHT;
        let row_rect = original_index_row_rect(window_rect, scale, visible_row);
        if Some(topic.object_id) == effective_selection {
            list_painter.rect_filled(row_rect, 0.0, Color32::from_rgb(0, 0, 96));
        }
        let response = ui.interact(
            row_rect,
            ui.id()
                .with(("authentic-encyclopedia-row", topic.object_id)),
            egui::Sense::click(),
        );
        list_painter.text(
            encyclopedia_point(window_rect, scale, 40.0, y + 1.0),
            egui::Align2::LEFT_TOP,
            topic.title,
            list_font.clone(),
            Color32::WHITE,
        );
        if response.double_clicked() {
            *action = Some(EncyclopediaSurfaceAction::OpenTopic(topic.object_id));
        } else if response.clicked() {
            *action = Some(EncyclopediaSurfaceAction::SelectTopic(topic.object_id));
        }
    }
}

fn draw_surface_topic(
    ui: &mut egui::Ui,
    window_rect: egui::Rect,
    scale: f32,
    state: &mut EncyclopediaSurfaceState,
    surface: &EncyclopediaSurface<'_>,
    texture: Option<egui::TextureId>,
) {
    let Some(topic) = surface.active_topic.as_ref() else {
        return;
    };
    if let (Some(texture), Some(artwork)) = (texture, topic.artwork.as_ref()) {
        let rect = encyclopedia_rect(
            window_rect,
            scale,
            12.0,
            31.0,
            artwork.width as f32,
            artwork.height as f32,
        );
        ui.painter().image(
            texture,
            rect,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    }
    ui.painter().text(
        encyclopedia_point(window_rect, scale, 36.0, 14.0),
        egui::Align2::LEFT_TOP,
        topic.title,
        egui::FontId::proportional(15.0 * scale),
        Color32::WHITE,
    );
    let Some(description) = topic.description else {
        state.body_scroll = 0.0;
        return;
    };
    let body_rect = encyclopedia_rect(window_rect, scale, 17.0, 231.0, 395.0, 80.0);
    let galley = ui.painter().layout(
        description.to_owned(),
        egui::FontId::proportional(14.0 * scale),
        Color32::WHITE,
        body_rect.width(),
    );
    let max_scroll = (galley.size().y - body_rect.height()).max(0.0);
    if ui.rect_contains_pointer(body_rect) {
        let wheel = ui.input(|input| input.raw_scroll_delta.y);
        state.body_scroll = (state.body_scroll - wheel).clamp(0.0, max_scroll);
    } else {
        state.body_scroll = state.body_scroll.min(max_scroll);
    }
    ui.painter().with_clip_rect(body_rect).galley(
        body_rect.min - egui::vec2(0.0, state.body_scroll),
        galley,
        Color32::WHITE,
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "the recovered control geometry and renderer state stay explicit"
)]
fn draw_surface_control(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    window_rect: egui::Rect,
    scale: f32,
    control: SurfaceControlSpec,
    enabled: bool,
    captured_control: &mut Option<u16>,
) -> bool {
    let rect = encyclopedia_rect(
        window_rect,
        scale,
        f32::from(control.x),
        f32::from(control.y),
        f32::from(control.width),
        f32::from(control.height),
    );
    let control_id = ui
        .id()
        .with(("encyclopedia-surface-control", control.command_id));
    ui.interact(
        rect,
        control_id,
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    if enabled && ctx.input(|input| input.pointer.primary_pressed()) {
        let captured = ctx.pointer_latest_pos().is_some_and(|point| {
            original_control_contains(cache, control.normal_resource, rect, scale, point)
        });
        if captured {
            *captured_control = Some(control.command_id);
        }
    }
    let captured = enabled && *captured_control == Some(control.command_id);
    let (pointer, primary_down, primary_released) = ctx.input(|input| {
        (
            input.pointer.interact_pos(),
            input.pointer.button_down(egui::PointerButton::Primary),
            input.pointer.button_released(egui::PointerButton::Primary),
        )
    });
    let pointer_hits = pointer.is_some_and(|point| {
        original_control_contains(cache, control.normal_resource, rect, scale, point)
    });
    let pressed = control.selected || (enabled && primary_down && captured && pointer_hits);
    let resource = if !enabled {
        control.disabled_resource.unwrap_or(control.normal_resource)
    } else if pressed {
        control.pressed_resource
    } else {
        control.normal_resource
    };
    paint_original_resource_native(ui.painter(), ctx, cache, resource, rect.min, scale, rect);
    enabled && captured && primary_released && pointer_hits
}

fn surface_keyboard_input(
    ctx: &egui::Context,
    state: &mut EncyclopediaSurfaceState,
    surface: &EncyclopediaSurface<'_>,
    effective_selection: Option<u32>,
) -> Option<EncyclopediaSurfaceAction> {
    let pressed = |key| {
        let pressed = ctx.input(|input| input.key_pressed(key));
        if pressed {
            ctx.input_mut(|input| {
                input.consume_key(egui::Modifiers::NONE, key);
            });
        }
        pressed
    };
    if pressed(egui::Key::Tab) {
        return None;
    }
    if pressed(egui::Key::Escape) {
        return Some(EncyclopediaSurfaceAction::Close);
    }
    if surface.mode == EncyclopediaSurfaceMode::Topic {
        if pressed(egui::Key::ArrowUp) {
            state.body_scroll = (state.body_scroll - 14.0).max(0.0);
        } else if pressed(egui::Key::ArrowDown) {
            state.body_scroll += 14.0;
        } else if pressed(egui::Key::PageUp) {
            state.body_scroll = (state.body_scroll - 80.0).max(0.0);
        } else if pressed(egui::Key::PageDown) {
            state.body_scroll += 80.0;
        }
    }
    let (logical_key, list_key) = if pressed(egui::Key::ArrowLeft) {
        (Some(EncyclopediaSurfaceKey::Left), None)
    } else if pressed(egui::Key::ArrowRight) {
        (Some(EncyclopediaSurfaceKey::Right), None)
    } else if pressed(egui::Key::Enter) {
        (Some(EncyclopediaSurfaceKey::Enter), None)
    } else if pressed(egui::Key::ArrowUp) {
        (None, Some(EncyclopediaSurfaceKey::Up))
    } else if pressed(egui::Key::ArrowDown) {
        (None, Some(EncyclopediaSurfaceKey::Down))
    } else if pressed(egui::Key::PageUp) {
        (None, Some(EncyclopediaSurfaceKey::PageUp))
    } else if pressed(egui::Key::PageDown) {
        (None, Some(EncyclopediaSurfaceKey::PageDown))
    } else if pressed(egui::Key::Home) {
        (None, Some(EncyclopediaSurfaceKey::Home))
    } else if pressed(egui::Key::End) {
        (None, Some(EncyclopediaSurfaceKey::End))
    } else {
        (None, None)
    };
    if let Some(key) = logical_key {
        let categories = surface
            .categories
            .iter()
            .map(|category| category.command_id)
            .collect::<Vec<_>>();
        return encyclopedia_keyboard_action(
            surface.mode,
            key,
            &categories,
            surface.category.command_id,
            effective_selection,
            surface.navigation.previous_object_id,
            surface.navigation.next_object_id,
        );
    }
    if surface.mode == EncyclopediaSurfaceMode::Index {
        return list_key.and_then(|key| {
            let topics = surface
                .topics
                .iter()
                .map(|topic| topic.object_id)
                .collect::<Vec<_>>();
            encyclopedia_index_list_action(key, &topics, effective_selection)
        });
    }
    None
}

/// Paint the source-exact Galactic Encyclopedia index bitmap layers.
///
/// Text labels and the object list are intentionally absent until their
/// `TEXTSTRA` and native-list contracts are transported. Category resource
/// `0x75` is painted at native size and clipped to its 49-by-41 control, as
/// the original constructor does; it is never scaled from 57 pixels high.
pub fn draw_encyclopedia_index_shell(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: crate::cockpit::CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
    selected_category: u16,
) -> Option<u16> {
    draw_encyclopedia_index_layers(ctx, cache, faction, origin, scale, selected_category, None)
        .activated_command
}

/// Paint the authentic index shell together with localized source rows.
///
/// The catalog is immutable reference data and selection is stored by compound
/// object id, so category changes and source reordering cannot silently select
/// a different object. Topic composition remains a later parity checkpoint.
pub fn draw_encyclopedia_index_catalog(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: crate::cockpit::CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
    state: &mut EncyclopediaState,
    catalog: &OriginalEncyclopediaCatalog,
) -> Option<u16> {
    if scale <= 0.0 || catalog.is_empty() {
        return None;
    }
    reconcile_original_index_state(state, catalog, false);
    let entries = catalog.entries_for(state.original_category_command);
    let selected_name = entries
        .iter()
        .find(|entry| Some(entry.object_id) == state.original_selected_object_id)
        .map(|entry| entry.name.as_str())
        .unwrap_or("");
    let category_label = catalog
        .category_label(state.original_category_command)
        .unwrap_or("");
    let content = OriginalIndexContent {
        title: &catalog.title,
        topic_label: &catalog.topic_label,
        category_label,
        selected_name,
        entries: &entries,
        selected_object_id: state.original_selected_object_id,
        scroll_row: state.original_scroll_row,
    };
    let outcome = draw_encyclopedia_index_layers(
        ctx,
        cache,
        faction,
        origin,
        scale,
        state.original_category_command,
        Some(content),
    );
    if let Some(command) = outcome.activated_command {
        if (0x6f..=0x75).contains(&command) && command != state.original_category_command {
            state.original_category_command = command;
            reconcile_original_index_state(state, catalog, true);
        }
    }
    if let Some(scroll_row) = outcome.scroll_row {
        state.original_scroll_row = scroll_row;
        reconcile_original_index_state(state, catalog, false);
    }
    if let Some(object_id) = outcome.selected_object_id {
        state.original_selected_object_id = Some(object_id);
        reconcile_original_index_state(state, catalog, true);
    }
    outcome.activated_command
}

fn reconcile_original_index_state(
    state: &mut EncyclopediaState,
    catalog: &OriginalEncyclopediaCatalog,
    reveal_selection: bool,
) {
    if !(0x6f..=0x75).contains(&state.original_category_command) {
        state.original_category_command = 0x6f;
    }
    let entries = catalog.entries_for(state.original_category_command);
    if !entries
        .iter()
        .any(|entry| Some(entry.object_id) == state.original_selected_object_id)
    {
        state.original_selected_object_id = entries.first().map(|entry| entry.object_id);
        state.original_scroll_row = 0;
    }
    let selected_index = entries
        .iter()
        .position(|entry| Some(entry.object_id) == state.original_selected_object_id)
        .unwrap_or(0);
    let max_scroll = entries.len().saturating_sub(ORIGINAL_INDEX_VISIBLE_ROWS);
    state.original_scroll_row = state.original_scroll_row.min(max_scroll);
    if reveal_selection {
        if selected_index < state.original_scroll_row {
            state.original_scroll_row = selected_index;
        } else if selected_index >= state.original_scroll_row + ORIGINAL_INDEX_VISIBLE_ROWS {
            state.original_scroll_row = selected_index + 1 - ORIGINAL_INDEX_VISIBLE_ROWS;
        }
    }
}

#[derive(Default)]
struct OriginalIndexOutcome {
    activated_command: Option<u16>,
    selected_object_id: Option<u32>,
    scroll_row: Option<usize>,
}

struct OriginalIndexContent<'a> {
    title: &'a str,
    topic_label: &'a str,
    category_label: &'a str,
    selected_name: &'a str,
    entries: &'a [&'a OriginalEncyclopediaEntry],
    selected_object_id: Option<u32>,
    scroll_row: usize,
}

fn draw_encyclopedia_index_layers(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: crate::cockpit::CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
    selected_category: u16,
    content: Option<OriginalIndexContent<'_>>,
) -> OriginalIndexOutcome {
    if scale <= 0.0 || !(0x6f..=0x75).contains(&selected_category) {
        return OriginalIndexOutcome::default();
    }

    let mut outcome = OriginalIndexOutcome::default();
    egui::Area::new(egui::Id::new("original-encyclopedia-index-shell"))
        .fixed_pos(origin)
        .order(egui::Order::Middle)
        .show(ctx, |ui| {
            let size = egui::vec2(
                ENCYCLOPEDIA_INDEX_WIDTH * scale,
                ENCYCLOPEDIA_INDEX_HEIGHT * scale,
            );
            let (window_rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
            let (base, rail) = match faction {
                crate::cockpit::CockpitFaction::Alliance => (10_335, 10_585),
                crate::cockpit::CockpitFaction::Empire => (10_336, 10_589),
            };
            paint_original_resource_native(
                ui.painter(),
                ctx,
                cache,
                base,
                window_rect.min,
                scale,
                window_rect,
            );
            paint_original_resource_native(
                ui.painter(),
                ctx,
                cache,
                rail,
                encyclopedia_point(window_rect, scale, 412.0, 0.0),
                scale,
                window_rect,
            );
            let content_rect = encyclopedia_rect(
                window_rect,
                scale,
                INDEX_CONTENT_X,
                INDEX_CONTENT_Y,
                400.0,
                306.0,
            );
            paint_original_resource_native(
                ui.painter(),
                ctx,
                cache,
                INDEX_CONTENT,
                content_rect.min,
                scale,
                content_rect,
            );

            for control in encyclopedia_category_controls(faction) {
                if draw_original_control(
                    ui,
                    ctx,
                    cache,
                    window_rect,
                    scale,
                    control,
                    control.command_id == selected_category,
                ) {
                    outcome.activated_command = Some(control.command_id);
                }
            }
            for control in encyclopedia_rail_controls(faction) {
                if draw_original_control(
                    ui,
                    ctx,
                    cache,
                    window_rect,
                    scale,
                    control,
                    control.selected_in_index,
                ) {
                    outcome.activated_command = Some(control.command_id);
                }
            }
            if let Some(content) = content {
                draw_original_index_content(ui, window_rect, scale, content, &mut outcome);
            }
        });
    outcome
}

fn draw_original_index_content(
    ui: &mut egui::Ui,
    window_rect: egui::Rect,
    scale: f32,
    content: OriginalIndexContent<'_>,
    outcome: &mut OriginalIndexOutcome,
) {
    let painter = ui.painter();
    let title_font = egui::FontId::proportional(15.0 * scale);
    let list_font = egui::FontId::proportional(14.0 * scale);
    painter.text(
        encyclopedia_point(window_rect, scale, 211.0, 14.0),
        egui::Align2::CENTER_TOP,
        content.title,
        title_font,
        Color32::WHITE,
    );
    painter.text(
        encyclopedia_point(window_rect, scale, 36.0, 48.0),
        egui::Align2::LEFT_TOP,
        content.topic_label,
        list_font.clone(),
        Color32::WHITE,
    );
    painter.text(
        encyclopedia_point(window_rect, scale, 143.0, 47.0),
        egui::Align2::LEFT_TOP,
        content.selected_name,
        list_font.clone(),
        Color32::WHITE,
    );
    painter.text(
        encyclopedia_point(window_rect, scale, 40.0, 120.0),
        egui::Align2::LEFT_TOP,
        content.category_label,
        list_font.clone(),
        Color32::WHITE,
    );

    let max_scroll = content
        .entries
        .len()
        .saturating_sub(ORIGINAL_INDEX_VISIBLE_ROWS);
    let list_rect = original_index_list_rect(window_rect, scale);
    let list_painter = painter.with_clip_rect(list_rect);
    if ui.rect_contains_pointer(list_rect) {
        let wheel = ui.input(|input| input.raw_scroll_delta.y);
        if wheel.abs() > f32::EPSILON {
            let rows = ((wheel.abs() / 24.0).ceil() as usize).clamp(1, 3);
            outcome.scroll_row = Some(if wheel < 0.0 {
                content.scroll_row.saturating_add(rows).min(max_scroll)
            } else {
                content.scroll_row.saturating_sub(rows)
            });
        }
    }
    let up_rect = encyclopedia_rect(window_rect, scale, 374.0, 137.0, 12.0, 13.0);
    let down_rect = encyclopedia_rect(window_rect, scale, 374.0, 284.0, 12.0, 13.0);
    if ui
        .interact(
            up_rect,
            ui.id().with("encyclopedia-index-scroll-up"),
            egui::Sense::click(),
        )
        .clicked()
    {
        outcome.scroll_row = Some(content.scroll_row.saturating_sub(1));
    }
    if ui
        .interact(
            down_rect,
            ui.id().with("encyclopedia-index-scroll-down"),
            egui::Sense::click(),
        )
        .clicked()
    {
        outcome.scroll_row = Some(content.scroll_row.saturating_add(1).min(max_scroll));
    }

    for (visible_row, entry) in content
        .entries
        .iter()
        .skip(content.scroll_row)
        .take(ORIGINAL_INDEX_VISIBLE_ROWS)
        .enumerate()
    {
        let y = 137.0 + visible_row as f32 * ORIGINAL_INDEX_ROW_HEIGHT;
        let row_rect = original_index_row_rect(window_rect, scale, visible_row);
        let selected = Some(entry.object_id) == content.selected_object_id;
        if selected {
            list_painter.rect_filled(row_rect, 0.0, Color32::from_rgb(0, 0, 96));
        }
        let response = ui.interact(
            row_rect,
            ui.id().with(("encyclopedia-index-row", entry.object_id)),
            egui::Sense::click(),
        );
        list_painter.text(
            encyclopedia_point(window_rect, scale, 40.0, y + 1.0),
            egui::Align2::LEFT_TOP,
            &entry.name,
            list_font.clone(),
            Color32::WHITE,
        );
        if response.clicked() {
            outcome.selected_object_id = Some(entry.object_id);
        }
    }
}

fn draw_original_control(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    window_rect: egui::Rect,
    scale: f32,
    control: OriginalControlSpec,
    selected: bool,
) -> bool {
    let rect = encyclopedia_rect(
        window_rect,
        scale,
        f32::from(control.x),
        f32::from(control.y),
        f32::from(control.width),
        f32::from(control.height),
    );
    let control_id = ui
        .id()
        .with(("encyclopedia-original-control", control.command_id));
    let response = ui.interact(rect, control_id, egui::Sense::click());
    let capture_id = control_id.with("opaque-press-origin");
    if ctx.input(|input| input.pointer.primary_pressed()) {
        let captured = ctx.pointer_latest_pos().is_some_and(|point| {
            original_control_contains(cache, control.normal_resource, rect, scale, point)
        });
        ui.data_mut(|data| data.insert_temp(capture_id, captured));
    }
    let captured = ui.data(|data| data.get_temp::<bool>(capture_id).unwrap_or(false));
    let (pointer, primary_down) = ctx.input(|input| {
        (
            input.pointer.interact_pos(),
            input.pointer.button_down(egui::PointerButton::Primary),
        )
    });
    let pointer_hits = pointer.is_some_and(|point| {
        original_control_contains(cache, control.normal_resource, rect, scale, point)
    });
    let pressed = selected
        || (primary_down && response.is_pointer_button_down_on() && captured && pointer_hits);
    paint_original_resource_native(
        ui.painter(),
        ctx,
        cache,
        if pressed {
            control.pressed_resource
        } else {
            control.normal_resource
        },
        rect.min,
        scale,
        rect,
    );
    captured
        && response.clicked()
        && response.interact_pointer_pos().is_some_and(|point| {
            original_control_contains(cache, control.normal_resource, rect, scale, point)
        })
}

fn original_control_contains(
    cache: &mut BmpCache,
    resource_id: u32,
    rect: egui::Rect,
    scale: f32,
    point: egui::Pos2,
) -> bool {
    if point.x < rect.min.x
        || point.x >= rect.max.x
        || point.y < rect.min.y
        || point.y >= rect.max.y
    {
        return false;
    }
    let x = ((point.x - rect.min.x) / scale).floor() as usize;
    let y = ((point.y - rect.min.y) / scale).floor() as usize;
    cache.is_resource_hit(DllSource::Strategy, resource_id, x, y)
}

fn encyclopedia_point(parent: egui::Rect, scale: f32, x: f32, y: f32) -> egui::Pos2 {
    egui::pos2(parent.min.x + x * scale, parent.min.y + y * scale)
}

fn encyclopedia_rect(
    parent: egui::Rect,
    scale: f32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> egui::Rect {
    egui::Rect::from_min_size(
        encyclopedia_point(parent, scale, x, y),
        egui::vec2(width * scale, height * scale),
    )
}

fn original_index_list_rect(parent: egui::Rect, scale: f32) -> egui::Rect {
    encyclopedia_rect(parent, scale, 36.0, 137.0, 350.0, 160.0)
}

fn original_index_row_rect(parent: egui::Rect, scale: f32, visible_row: usize) -> egui::Rect {
    let y = 137.0 + visible_row as f32 * ORIGINAL_INDEX_ROW_HEIGHT;
    encyclopedia_rect(
        parent,
        scale,
        36.0,
        y,
        338.0,
        ORIGINAL_INDEX_ROW_HEIGHT,
    )
    .intersect(original_index_list_rect(parent, scale))
}

fn paint_original_resource_native(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    resource_id: u32,
    origin: egui::Pos2,
    scale: f32,
    clip_rect: egui::Rect,
) {
    let Some(texture) = cache.get(ctx, DllSource::Strategy, resource_id) else {
        return;
    };
    let rect = egui::Rect::from_min_size(origin, texture.size_vec2() * scale);
    painter.with_clip_rect(clip_rect).image(
        texture.id(),
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
}

/// Fixed-position test adapter for deterministic browser inspection.
#[cfg(feature = "interface-test-fixtures")]
pub fn draw_encyclopedia_index_fixture(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: crate::cockpit::CockpitFaction,
    selected_category: u16,
) -> Option<u16> {
    draw_encyclopedia_index_shell(
        ctx,
        cache,
        faction,
        egui::pos2(85.0, 55.0),
        1.0,
        selected_category,
    )
}

/// Fixed-position catalog adapter for deterministic browser inspection.
#[cfg(feature = "interface-test-fixtures")]
pub fn draw_encyclopedia_index_catalog_fixture(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: crate::cockpit::CockpitFaction,
    state: &mut EncyclopediaState,
    catalog: &OriginalEncyclopediaCatalog,
) -> Option<u16> {
    draw_encyclopedia_index_catalog(
        ctx,
        cache,
        faction,
        egui::pos2(85.0, 55.0),
        1.0,
        state,
        catalog,
    )
}

// ---------------------------------------------------------------------------
// Image helpers
// ---------------------------------------------------------------------------

/// Load and display an EDATA BMP image, caching the texture by EDATA number.
///
/// On first call for a given `edata_n`, reads the BMP file, decodes it via
/// the `image` crate, and registers it as an egui texture.  Subsequent calls
/// use the cached handle.  If the file is missing or fails to decode, shows
/// a gray placeholder rectangle.
fn show_edata_image(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    edata_n: u16,
    state: &mut EncyclopediaState,
) {
    // Lazy-load the texture if not yet cached. HD is an explicit enhancement
    // profile; original parity never probes the HD tree.
    if !state.textures.contains_key(&edata_n) {
        let handle = load_edata_texture(
            ctx,
            edata_n,
            state.hd_path.as_deref(),
            state.edata_path.as_deref(),
            state.asset_profile,
            state
                .approved_hd_assets
                .get(&format!("edata/EDATA_{edata_n:03}")),
        );
        if handle.is_none() {
            eprintln!(
                "[encyclopedia] original artwork unavailable asset={}",
                edata_filename(edata_n)
            );
        }
        state.textures.insert(edata_n, handle);
    }

    if let Some(Some(handle)) = state.textures.get(&edata_n) {
        let size = egui::vec2(400.0, 200.0);
        ui.add(egui::Image::from_texture((handle.id(), size)));
    } else {
        show_placeholder_image(ui);
    }
}

/// Paint one original EDATA image at a fixed native-size location for the
/// test-only browser transport gate.
///
/// This is not an encyclopedia UI and is absent from production builds. It
/// isolates the runtime-pack/cache/decode/render path so exact source pixels
/// can be compared without promoting the current replacement window.
#[cfg(feature = "interface-test-fixtures")]
pub fn draw_encyclopedia_artwork_fixture(
    ctx: &egui::Context,
    edata_n: u16,
    state: &mut EncyclopediaState,
) {
    egui::Area::new(egui::Id::new("encyclopedia_artwork_transport_fixture"))
        .fixed_pos(egui::pos2(120.0, 120.0))
        .show(ctx, |ui| show_edata_image(ui, ctx, edata_n, state));
}

/// Draw a gray placeholder rectangle when no image is available.
fn show_placeholder_image(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(400.0, 200.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 4.0, Color32::from_gray(40));
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "No image",
        egui::FontId::default(),
        Color32::from_gray(100),
    );
}

/// Load an EDATA image and register it as an egui texture.
///
/// In faithful-HD mode, checks for an approved PNG first (`EDATA_NNN.png` in
/// `hd_path`), then falls back to the original BMP (`EDATA.NNN` in
/// `edata_path`). Original-parity mode never probes the HD path.
/// Returns `None` if neither exists or decoding fails.
/// On WASM targets, always returns `None` (filesystem access not available).
#[cfg(not(target_arch = "wasm32"))]
fn load_edata_texture(
    ctx: &egui::Context,
    edata_n: u16,
    hd_path: Option<&Path>,
    edata_path: Option<&Path>,
    profile: AssetRenderProfile,
    approved_hd: Option<&ApprovedHdAsset>,
) -> Option<TextureHandle> {
    let dir = edata_path?;
    let bmp_file = dir.join(edata_filename(edata_n));

    if profile == AssetRenderProfile::FaithfulHd {
        if let Some(hd_dir) = hd_path {
            let hd_file = hd_dir.join(format!("EDATA_{edata_n:03}.png"));
            if let Some(bytes) =
                approved_hd.and_then(|approval| validated_hd_bytes(&bmp_file, &hd_file, approval))
            {
                if let Some(handle) = load_image_bytes(ctx, edata_n, &bytes, TextureOptions::LINEAR)
                {
                    return Some(handle);
                }
            } else if approved_hd.is_some() && hd_file.exists() {
                eprintln!(
                    "[encyclopedia] HD source/output digest mismatch for EDATA_{edata_n:03}; falling back to original"
                );
            }
        }
    }

    // Original data is authoritative and uses exact nearest sampling.
    if bmp_file.exists() {
        return load_image_file(ctx, edata_n, &bmp_file, TextureOptions::NEAREST);
    }

    None
}

/// Decode an image file (BMP or PNG) and register it as an egui texture.
#[cfg(not(target_arch = "wasm32"))]
fn load_image_file(
    ctx: &egui::Context,
    edata_n: u16,
    path: &Path,
    texture_options: TextureOptions,
) -> Option<TextureHandle> {
    let bytes = std::fs::read(path).ok()?;

    load_image_bytes(ctx, edata_n, &bytes, texture_options)
}

fn load_image_bytes(
    ctx: &egui::Context,
    edata_n: u16,
    bytes: &[u8],
    texture_options: TextureOptions,
) -> Option<TextureHandle> {
    // image crate auto-detects format from magic bytes.
    let img = image::load_from_memory(bytes).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let color_image =
        egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgba.as_raw());

    let handle = ctx.load_texture(format!("edata_{edata_n}"), color_image, texture_options);

    Some(handle)
}

#[cfg(target_arch = "wasm32")]
fn load_edata_texture(
    ctx: &egui::Context,
    edata_n: u16,
    _hd_path: Option<&Path>,
    _edata_path: Option<&Path>,
    _profile: AssetRenderProfile,
    _approved_hd: Option<&ApprovedHdAsset>,
) -> Option<TextureHandle> {
    let bytes = wasm_edata_bytes(edata_n)?;
    load_image_bytes(ctx, edata_n, &bytes, TextureOptions::NEAREST)
}

// ---------------------------------------------------------------------------
// Stat display helpers
// ---------------------------------------------------------------------------

/// Two-column stat row: label on left, value on right.
fn stat_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!("{label}:"))
                .small()
                .color(Color32::from_gray(160)),
        );
        ui.label(RichText::new(value).small());
    });
}

/// Two-column stat row for `SkillPair` values (base ± variance).
fn stat_row_pair(ui: &mut egui::Ui, label: &str, base: u32, variance: u32) {
    let value = if variance > 0 {
        format!("{base} ± {variance}")
    } else {
        base.to_string()
    };
    stat_row(ui, label, &value);
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    #[derive(Debug, Default, PartialEq, Eq)]
    struct TextureCounts {
        uploads: usize,
        releases: usize,
        last_dimensions: Option<(u32, u32)>,
        last_sampling: Option<EncyclopediaTextureSampling>,
    }

    #[derive(Clone)]
    struct CountingTextureBackend(Rc<RefCell<TextureCounts>>);

    impl EncyclopediaTextureBackend for CountingTextureBackend {
        type Texture = u64;

        fn upload(
            &mut self,
            upload: EncyclopediaTextureUpload<'_>,
        ) -> Result<Self::Texture, String> {
            let mut counts = self.0.borrow_mut();
            counts.uploads += 1;
            counts.last_dimensions = Some((upload.width, upload.height));
            counts.last_sampling = Some(upload.sampling);
            Ok(counts.uploads as u64)
        }

        fn release(&mut self, _texture: Self::Texture) {
            self.0.borrow_mut().releases += 1;
        }
    }

    fn synthetic_indexed_edata() -> Vec<u8> {
        const WIDTH: usize = 400;
        const HEIGHT: usize = 200;
        const PIXEL_OFFSET: usize = 1_078;
        let mut bytes = vec![0_u8; PIXEL_OFFSET + WIDTH * HEIGHT];
        let file_size = bytes.len() as u32;
        bytes[0..2].copy_from_slice(b"BM");
        bytes[2..6].copy_from_slice(&file_size.to_le_bytes());
        bytes[10..14].copy_from_slice(&(PIXEL_OFFSET as u32).to_le_bytes());
        bytes[14..18].copy_from_slice(&40_u32.to_le_bytes());
        bytes[18..22].copy_from_slice(&(WIDTH as i32).to_le_bytes());
        bytes[22..26].copy_from_slice(&(HEIGHT as i32).to_le_bytes());
        bytes[26..28].copy_from_slice(&1_u16.to_le_bytes());
        bytes[28..30].copy_from_slice(&8_u16.to_le_bytes());
        bytes[34..38].copy_from_slice(&((WIDTH * HEIGHT) as u32).to_le_bytes());
        bytes[46..50].copy_from_slice(&256_u32.to_le_bytes());
        bytes[54 + 4..54 + 8].copy_from_slice(&[0x20, 0x80, 0xe0, 0]);
        bytes[PIXEL_OFFSET..].fill(1);
        bytes
    }

    fn artwork_view<'a>(
        filename: &'a str,
        digest: &'a str,
        bytes: &'a [u8],
    ) -> EncyclopediaArtworkView<'a> {
        EncyclopediaArtworkView {
            resource_id: 0x2740,
            filename,
            digest,
            width: 400,
            height: 200,
            bytes,
            sampling: EncyclopediaTextureSampling::Nearest,
        }
    }

    #[test]
    fn topic_texture_cache_hits_once_and_releases_on_resource_or_generation_change() {
        let counts = Rc::new(RefCell::new(TextureCounts::default()));
        let backend = CountingTextureBackend(counts.clone());
        let mut cache = EncyclopediaTopicTextureCache::new(backend);
        let first_bytes = synthetic_indexed_edata();
        let second_bytes = synthetic_indexed_edata();
        let first = artwork_view("EDATA.014", "digest-14", &first_bytes);
        let second = artwork_view("EDATA.015", "digest-15", &second_bytes);

        assert!(!cache.resolve(7, Some(&first)).unwrap().cache_hit);
        assert!(cache.resolve(7, Some(&first)).unwrap().cache_hit);
        assert_eq!(
            *counts.borrow(),
            TextureCounts {
                uploads: 1,
                releases: 0,
                last_dimensions: Some((400, 200)),
                last_sampling: Some(EncyclopediaTextureSampling::Nearest),
            }
        );

        assert!(!cache.resolve(7, Some(&second)).unwrap().cache_hit);
        assert_eq!(counts.borrow().uploads, 2);
        assert_eq!(counts.borrow().releases, 1);

        assert!(!cache.resolve(8, Some(&second)).unwrap().cache_hit);
        assert_eq!(counts.borrow().uploads, 3);
        assert_eq!(counts.borrow().releases, 2);

        assert!(cache.resolve(8, None).unwrap().texture.is_none());
        assert_eq!(counts.borrow().releases, 3);
    }

    #[test]
    fn topic_texture_cache_invalidates_when_only_sampling_profile_changes() {
        let counts = Rc::new(RefCell::new(TextureCounts::default()));
        let backend = CountingTextureBackend(counts.clone());
        let mut cache = EncyclopediaTopicTextureCache::new(backend);
        let bytes = synthetic_indexed_edata();
        let nearest = artwork_view("EDATA.014", "same-digest", &bytes);
        let mut linear = nearest;
        linear.sampling = EncyclopediaTextureSampling::Linear;

        assert!(!cache.resolve(7, Some(&nearest)).unwrap().cache_hit);
        assert!(!cache.resolve(7, Some(&linear)).unwrap().cache_hit);
        assert_eq!(counts.borrow().uploads, 2);
        assert_eq!(counts.borrow().releases, 1);
        assert_eq!(
            counts.borrow().last_sampling,
            Some(EncyclopediaTextureSampling::Linear)
        );
    }

    #[test]
    fn topic_texture_cache_drop_releases_the_retained_handle() {
        let counts = Rc::new(RefCell::new(TextureCounts::default()));
        let bytes = synthetic_indexed_edata();
        let artwork = artwork_view("EDATA.014", "digest-14", &bytes);

        {
            let backend = CountingTextureBackend(counts.clone());
            let mut cache = EncyclopediaTopicTextureCache::new(backend);
            assert!(cache.resolve(7, Some(&artwork)).unwrap().texture.is_some());
            assert_eq!(counts.borrow().uploads, 1);
            assert_eq!(counts.borrow().releases, 0);
        }

        assert_eq!(counts.borrow().releases, 1);
    }

    #[test]
    fn topic_texture_cache_rejects_a_single_dimension_mismatch_before_upload() {
        let counts = Rc::new(RefCell::new(TextureCounts::default()));
        let backend = CountingTextureBackend(counts.clone());
        let mut cache = EncyclopediaTopicTextureCache::new(backend);
        let bytes = synthetic_indexed_edata();
        let mut artwork = artwork_view("EDATA.014", "digest-14", &bytes);
        artwork.height = 199;

        let error = match cache.resolve(7, Some(&artwork)) {
            Ok(_) => panic!("a height mismatch must be rejected"),
            Err(error) => error,
        };

        assert!(error.contains("400x200 do not match validated 400x199"));
        assert_eq!(counts.borrow().uploads, 0);
        assert_eq!(counts.borrow().releases, 0);
    }

    #[test]
    fn topic_changes_reset_body_scroll_while_stable_frames_preserve_it() {
        let mut state = EncyclopediaSurfaceState::new();
        state.body_scroll = 80.0;

        state.reconcile_active_topic(7, Some(11));
        assert_eq!(state.body_scroll, 0.0);
        state.body_scroll = 40.0;
        state.reconcile_active_topic(7, Some(11));
        assert_eq!(state.body_scroll, 40.0);

        state.reconcile_active_topic(7, Some(12));
        assert_eq!(state.body_scroll, 0.0);
        state.body_scroll = 20.0;
        state.reconcile_active_topic(8, Some(12));
        assert_eq!(state.body_scroll, 0.0);
    }

    #[test]
    fn authentic_surface_consumes_keys_only_while_its_mode_child_owns_focus() {
        let categories = vec![
            EncyclopediaSurfaceCategory {
                command_id: 0x6f,
                label_resource_id: 0x1850,
                label: "All",
                topic_count: 1,
            },
            EncyclopediaSurfaceCategory {
                command_id: 0x70,
                label_resource_id: 0x1855,
                label: "Systems",
                topic_count: 1,
            },
        ];
        let surface = EncyclopediaSurface {
            texture_generation: 7,
            title: "Galactic Encyclopedia",
            topic_label: "Topic",
            audience: EncyclopediaSurfaceAudience::Alliance,
            mode: EncyclopediaSurfaceMode::Index,
            category: categories[0],
            categories,
            topics: vec![EncyclopediaSurfaceTopicItem {
                object_id: 11,
                title: "Abregado-rae",
                availability: EncyclopediaSurfaceAvailability::Resolved,
            }],
            active_topic: None,
            navigation: EncyclopediaSurfaceNavigation {
                selected_object_id: Some(11),
                previous_object_id: None,
                next_object_id: None,
            },
        };
        let ctx = egui::Context::default();
        let mut state = EncyclopediaSurfaceState::new();
        let mut cache = BmpCache::new();
        let focus_id = surface_keyboard_focus_id(EncyclopediaSurfaceMode::Index);

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            assert_eq!(
                draw_encyclopedia_surface(
                    ctx,
                    &mut cache,
                    egui::Pos2::ZERO,
                    1.0,
                    &mut state,
                    &surface,
                ),
                None
            );
        });
        assert!(ctx.memory(|memory| memory.has_focus(focus_id)));

        let key_input = || egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::ArrowRight,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };
        let unrelated = egui::Id::new("unrelated-encyclopedia-test-widget");
        let mut unfocused_action = None;
        let mut unfocused_key_remained = false;
        let _ = ctx.run(key_input(), |ctx| {
            ctx.memory_mut(|memory| memory.request_focus(unrelated));
            unfocused_action = draw_encyclopedia_surface(
                ctx,
                &mut cache,
                egui::Pos2::ZERO,
                1.0,
                &mut state,
                &surface,
            );
            unfocused_key_remained = ctx.input(|input| input.key_pressed(egui::Key::ArrowRight));
        });
        assert_eq!(unfocused_action, None);
        assert!(unfocused_key_remained);
        assert!(ctx.memory(|memory| memory.has_focus(unrelated)));

        let mut focused_action = None;
        let mut focused_key_remained = true;
        let _ = ctx.run(key_input(), |ctx| {
            ctx.memory_mut(|memory| memory.request_focus(focus_id));
            focused_action = draw_encyclopedia_surface(
                ctx,
                &mut cache,
                egui::Pos2::ZERO,
                1.0,
                &mut state,
                &surface,
            );
            focused_key_remained = ctx.input(|input| input.key_pressed(egui::Key::ArrowRight));
        });
        assert_eq!(
            focused_action,
            Some(EncyclopediaSurfaceAction::SelectCategory(0x70))
        );
        assert!(!focused_key_remained);
    }

    #[test]
    fn authentic_keyboard_actions_keep_index_and_topic_edge_rules_distinct() {
        let categories = [0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75];

        assert_eq!(
            encyclopedia_keyboard_action(
                EncyclopediaSurfaceMode::Index,
                EncyclopediaSurfaceKey::Left,
                &categories,
                0x6f,
                Some(0x1400_0001),
                None,
                None,
            ),
            Some(EncyclopediaSurfaceAction::SelectCategory(0x6f))
        );
        assert_eq!(
            encyclopedia_keyboard_action(
                EncyclopediaSurfaceMode::Index,
                EncyclopediaSurfaceKey::Left,
                &categories,
                0x73,
                Some(0x1400_0001),
                None,
                None,
            ),
            Some(EncyclopediaSurfaceAction::SelectCategory(0x72))
        );
        assert_eq!(
            encyclopedia_keyboard_action(
                EncyclopediaSurfaceMode::Index,
                EncyclopediaSurfaceKey::Right,
                &categories,
                0x75,
                Some(0x1400_0001),
                None,
                None,
            ),
            Some(EncyclopediaSurfaceAction::SelectCategory(0x6f))
        );
        assert_eq!(
            encyclopedia_keyboard_action(
                EncyclopediaSurfaceMode::Index,
                EncyclopediaSurfaceKey::Enter,
                &categories,
                0x71,
                Some(0x1400_0001),
                None,
                None,
            ),
            Some(EncyclopediaSurfaceAction::OpenTopic(0x1400_0001))
        );
        assert_eq!(
            encyclopedia_keyboard_action(
                EncyclopediaSurfaceMode::Topic,
                EncyclopediaSurfaceKey::Left,
                &categories,
                0x71,
                Some(0x1400_0001),
                None,
                Some(0x1400_0002),
            ),
            None
        );
        assert_eq!(
            encyclopedia_keyboard_action(
                EncyclopediaSurfaceMode::Topic,
                EncyclopediaSurfaceKey::Right,
                &categories,
                0x71,
                Some(0x1400_0001),
                None,
                Some(0x1400_0002),
            ),
            Some(EncyclopediaSurfaceAction::OpenTopic(0x1400_0002))
        );
        assert_eq!(
            encyclopedia_keyboard_action(
                EncyclopediaSurfaceMode::Topic,
                EncyclopediaSurfaceKey::Escape,
                &categories,
                0x71,
                Some(0x1400_0001),
                None,
                None,
            ),
            Some(EncyclopediaSurfaceAction::Close)
        );
    }

    #[test]
    fn authentic_index_list_keys_are_bounded_and_page_by_visible_rows() {
        let topics = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21];

        assert_eq!(
            encyclopedia_index_list_action(EncyclopediaSurfaceKey::Up, &topics, Some(11)),
            Some(EncyclopediaSurfaceAction::SelectTopic(11))
        );
        assert_eq!(
            encyclopedia_index_list_action(EncyclopediaSurfaceKey::Down, &topics, Some(11)),
            Some(EncyclopediaSurfaceAction::SelectTopic(12))
        );
        assert_eq!(
            encyclopedia_index_list_action(EncyclopediaSurfaceKey::PageDown, &topics, Some(11)),
            Some(EncyclopediaSurfaceAction::SelectTopic(20))
        );
        assert_eq!(
            encyclopedia_index_list_action(EncyclopediaSurfaceKey::PageUp, &topics, Some(21)),
            Some(EncyclopediaSurfaceAction::SelectTopic(12))
        );
        assert_eq!(
            encyclopedia_index_list_action(EncyclopediaSurfaceKey::Home, &topics, Some(17)),
            Some(EncyclopediaSurfaceAction::SelectTopic(11))
        );
        assert_eq!(
            encyclopedia_index_list_action(EncyclopediaSurfaceKey::End, &topics, None),
            Some(EncyclopediaSurfaceAction::SelectTopic(21))
        );
    }

    #[test]
    fn authentic_topic_controls_use_recovered_faction_geometry_and_endpoint_resources() {
        let alliance = encyclopedia_topic_controls(crate::cockpit::CockpitFaction::Alliance);
        assert_eq!(
            alliance.map(|control| (
                control.command_id,
                control.x,
                control.y,
                control.width,
                control.height,
                control.normal_resource,
                control.pressed_resource,
                control.disabled_resource,
            )),
            [
                (0x83, 28, 14, 21, 17, 10_385, 10_386, Some(10_387)),
                (0x84, 380, 14, 21, 17, 10_382, 10_383, Some(10_384)),
            ]
        );
        let empire_rail = encyclopedia_rail_controls_for_mode(
            crate::cockpit::CockpitFaction::Empire,
            EncyclopediaSurfaceMode::Topic,
        );
        assert_eq!(
            empire_rail.map(|control| (control.command_id, control.x, control.y, control.selected)),
            [
                (0xfb, 426, 21, false),
                (0x67, 426, 89, true),
                (0x68, 426, 143, false)
            ]
        );
    }

    #[test]
    fn encyclopedia_defaults_to_original_parity() {
        let state = EncyclopediaState::new();
        assert_eq!(state.asset_profile, AssetRenderProfile::OriginalParity);
    }

    #[test]
    fn changing_asset_profile_invalidates_cached_images() {
        let mut state = EncyclopediaState::new();
        state.textures.insert(42, None);

        state.set_asset_profile(AssetRenderProfile::FaithfulHd);

        assert!(state.textures.is_empty());
        assert_eq!(state.asset_profile, AssetRenderProfile::FaithfulHd);
    }

    #[test]
    fn edata_filenames_preserve_the_original_three_digit_identity() {
        assert_eq!(edata_filename(1), "EDATA.001");
        assert_eq!(edata_filename(42), "EDATA.042");
        assert_eq!(edata_filename(192), "EDATA.192");
    }

    #[test]
    fn original_edata_bytes_decode_at_source_size() {
        let ctx = egui::Context::default();
        let texture = load_image_bytes(
            &ctx,
            42,
            &synthetic_indexed_edata(),
            TextureOptions::NEAREST,
        )
        .expect("synthetic original EDATA should decode");

        assert_eq!(texture.size(), [400, 200]);
    }

    #[test]
    fn original_index_categories_match_the_recovered_command_geometry() {
        let controls = encyclopedia_category_controls(crate::cockpit::CockpitFaction::Alliance);

        assert_eq!(
            controls.map(|control| (
                control.command_id,
                control.x,
                control.y,
                control.width,
                control.height,
            )),
            [
                (0x6f, 36, 78, 49, 41),
                (0x70, 88, 78, 49, 41),
                (0x71, 140, 78, 49, 41),
                (0x72, 192, 78, 49, 41),
                (0x73, 244, 78, 49, 41),
                (0x74, 296, 78, 49, 41),
                (0x75, 348, 78, 49, 41),
            ]
        );
    }

    #[test]
    fn original_index_category_resources_preserve_faction_variants() {
        let alliance = encyclopedia_category_controls(crate::cockpit::CockpitFaction::Alliance);
        let empire = encyclopedia_category_controls(crate::cockpit::CockpitFaction::Empire);

        assert_eq!(
            alliance.map(|control| (control.normal_resource, control.pressed_resource)),
            [
                (10_340, 10_339),
                (10_350, 10_349),
                (10_348, 10_347),
                (10_344, 10_343),
                (11_616, 11_615),
                (10_352, 10_351),
                (10_346, 10_345),
            ]
        );
        assert_eq!(
            empire.map(|control| (control.normal_resource, control.pressed_resource)),
            [
                (10_340, 10_339),
                (10_350, 10_349),
                (10_360, 10_359),
                (10_356, 10_355),
                (11_618, 11_617),
                (10_362, 10_361),
                (10_358, 10_357),
            ]
        );
    }

    #[test]
    fn original_index_rail_preserves_faction_geometry_and_selected_mode() {
        let alliance = encyclopedia_rail_controls(crate::cockpit::CockpitFaction::Alliance);
        let empire = encyclopedia_rail_controls(crate::cockpit::CockpitFaction::Empire);

        assert_eq!(
            alliance.map(|control| (
                control.command_id,
                control.x,
                control.y,
                control.width,
                control.height,
                control.normal_resource,
                control.pressed_resource,
                control.selected_in_index,
            )),
            [
                (0xfb, 423, 25, 32, 31, 10_370, 10_371, false),
                (0x67, 423, 93, 32, 31, 10_374, 10_375, false),
                (0x68, 423, 147, 32, 31, 10_372, 10_373, true),
            ]
        );
        assert_eq!(
            empire.map(|control| (
                control.command_id,
                control.x,
                control.y,
                control.width,
                control.height,
                control.normal_resource,
                control.pressed_resource,
                control.selected_in_index,
            )),
            [
                (0xfb, 426, 21, 44, 41, 10_376, 10_377, false),
                (0x67, 426, 89, 44, 41, 10_380, 10_381, false),
                (0x68, 426, 143, 44, 41, 10_378, 10_379, true),
            ]
        );
    }

    fn original_catalog_fixture() -> OriginalEncyclopediaCatalog {
        OriginalEncyclopediaCatalog::new(
            "Galactic Encyclopedia".into(),
            "Topic".into(),
            [
                "All Databases".into(),
                "System Database".into(),
                "Ship Database".into(),
                "Facilities Database".into(),
                "Missions Database".into(),
                "Troop Database".into(),
                "Personnel Database".into(),
            ],
            vec![
                OriginalEncyclopediaEntry {
                    object_id: 0x9000_0001,
                    name: "Allyuen".into(),
                },
                OriginalEncyclopediaEntry {
                    object_id: 0x1400_0001,
                    name: "Alliance Dreadnaught".into(),
                },
                OriginalEncyclopediaEntry {
                    object_id: 0x1000_0001,
                    name: "Alliance Army Regiment".into(),
                },
                OriginalEncyclopediaEntry {
                    object_id: 0x3000_0001,
                    name: "Ackbar".into(),
                },
            ],
        )
    }

    #[test]
    fn original_catalog_uses_source_labels_and_family_filters() {
        let catalog = original_catalog_fixture();

        assert_eq!(catalog.len(), 4);
        assert!(!catalog.is_empty());
        assert_eq!(catalog.category_label(0x6f), Some("All Databases"));
        assert_eq!(catalog.category_label(0x75), Some("Personnel Database"));
        assert_eq!(catalog.entries_for(0x6f).len(), 4);
        assert_eq!(catalog.entries_for(0x70)[0].name, "Allyuen");
        assert_eq!(catalog.entries_for(0x71)[0].name, "Alliance Dreadnaught");
        assert_eq!(catalog.entries_for(0x74)[0].name, "Alliance Army Regiment");
        assert_eq!(catalog.entries_for(0x75)[0].name, "Ackbar");
    }

    #[test]
    fn original_catalog_selection_preserves_identity_or_resets_to_first_match() {
        let catalog = original_catalog_fixture();
        let mut state = EncyclopediaState {
            original_selected_object_id: Some(0x1400_0001),
            ..EncyclopediaState::default()
        };

        reconcile_original_index_state(&mut state, &catalog, false);
        assert_eq!(state.original_selected_object_id, Some(0x1400_0001));

        state.original_category_command = 0x71;
        reconcile_original_index_state(&mut state, &catalog, true);
        assert_eq!(state.original_selected_object_id, Some(0x1400_0001));

        state.original_category_command = 0x70;
        reconcile_original_index_state(&mut state, &catalog, true);
        assert_eq!(state.original_selected_object_id, Some(0x9000_0001));
        assert_eq!(state.original_scroll_row, 0);
    }

    #[test]
    fn original_category_ranges_preserve_every_exclusive_boundary() {
        assert!(original_category_contains(0x6f, 0));
        for (command, start, end) in [
            (0x70, 0x90, 0x98),
            (0x71, 0x14, 0x20),
            (0x72, 0x20, 0x30),
            (0x73, 0x40, 0x80),
            (0x74, 0x10, 0x14),
            (0x75, 0x30, 0x40),
        ] {
            assert!(!original_category_contains(command, start - 1));
            assert!(original_category_contains(command, start));
            assert!(original_category_contains(command, end - 1));
            assert!(!original_category_contains(command, end));
        }
        assert!(!original_category_contains(0x76, 0x30));
    }

    #[test]
    fn original_catalog_reveal_and_clamp_keep_selection_visible() {
        let entries = (0..12)
            .map(|index| OriginalEncyclopediaEntry {
                object_id: 0x9000_0000 | index,
                name: format!("System {index:02}"),
            })
            .collect();
        let catalog = OriginalEncyclopediaCatalog::new(
            "Galactic Encyclopedia".into(),
            "Topic".into(),
            std::array::from_fn(|index| format!("Category {index}")),
            entries,
        );
        let mut state = EncyclopediaState {
            original_category_command: 0x70,
            original_selected_object_id: Some(0x9000_000b),
            original_scroll_row: 0,
            ..EncyclopediaState::default()
        };

        reconcile_original_index_state(&mut state, &catalog, true);
        assert_eq!(state.original_scroll_row, 3);

        state.original_selected_object_id = Some(0x9000_0000);
        reconcile_original_index_state(&mut state, &catalog, true);
        assert_eq!(state.original_scroll_row, 0);

        state.original_scroll_row = usize::MAX;
        reconcile_original_index_state(&mut state, &catalog, false);
        assert_eq!(state.original_scroll_row, 3);

        state.original_category_command = 0xffff;
        state.original_selected_object_id = Some(0xffff_ffff);
        reconcile_original_index_state(&mut state, &catalog, false);
        assert_eq!(state.original_category_command, 0x6f);
        assert_eq!(state.original_selected_object_id, Some(0x9000_0000));
        assert_eq!(state.original_scroll_row, 0);
    }

    #[test]
    fn original_index_ninth_row_is_clipped_to_the_native_list_boundary() {
        let parent = egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(470.0, 330.0));
        let list = original_index_list_rect(parent, 1.0);

        for row in 0..ORIGINAL_INDEX_VISIBLE_ROWS {
            let rect = original_index_row_rect(parent, 1.0, row);
            assert!(rect.min.y >= list.min.y);
            assert!(rect.max.y <= list.max.y);
            assert!(rect.height() > 0.0);
        }
        assert_eq!(original_index_row_rect(parent, 1.0, 8).height(), 16.0);
    }
}
