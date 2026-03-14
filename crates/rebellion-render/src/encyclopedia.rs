//! Encyclopedia viewer — browse ships, characters, and systems with artwork.
//!
//! Displays a floating egui window with tabs for capital ships, fighters,
//! characters, and star systems.  Each entity shows its 400×200 BMP artwork
//! loaded from EData/ alongside stats pulled from `GameWorld`.
//!
//! # EDATA mapping
//!
//! The original game stores encyclopedia images in sequentially numbered BMP
//! files (`EData/EDATA.NNN`).  The C# editor (SwRebellionEditor) reveals the
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

use egui_macroquad::egui::{self, Color32, RichText, ScrollArea, TextureHandle, TextureOptions};
use rebellion_core::ids::{CapitalShipKey, CharacterKey, FighterKey, SystemKey};
use rebellion_core::world::GameWorld;

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
    /// Path to the EData/ directory.  Set by the caller before first open.
    pub edata_path: Option<PathBuf>,
    /// Cached textures keyed by EDATA file number (1-based).
    textures: HashMap<u16, Option<TextureHandle>>,
}

impl EncyclopediaState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure the EData directory.  Call before opening the encyclopedia.
    pub fn set_edata_path(&mut self, path: impl Into<PathBuf>) {
        self.edata_path = Some(path.into());
    }
}

impl Default for EncyclopediaState {
    fn default() -> Self {
        EncyclopediaState {
            open: false,
            tab: EncyclopediaTab::default(),
            selected_index: 0,
            edata_path: None,
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
pub fn draw_encyclopedia(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut EncyclopediaState,
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
                    if ui.selectable_label(state.tab == tab, label).clicked() {
                        if state.tab != tab {
                            state.tab = tab;
                            state.selected_index = 0;
                        }
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
                    .show(list_ui, |ui| {
                        match state.tab {
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
                                        stat_row(ui, "Sublight", &ship.sub_light_engine.to_string());
                                        stat_row(ui, "Hyperdrive", &ship.hyperdrive.to_string());
                                        stat_row(ui, "Maneuver", &ship.maneuverability.to_string());
                                        stat_row(ui, "Fighters", &ship.fighter_capacity.to_string());
                                        stat_row(ui, "Troops", &ship.troop_capacity.to_string());
                                        stat_row(ui, "Build cost", &ship.refined_material_cost.to_string());
                                        stat_row(ui, "Maintenance", &ship.maintenance_cost.to_string());
                                        stat_row(ui, "Research order", &ship.research_order.to_string());
                                        stat_row(ui, "Build time", &ship.research_difficulty.to_string());
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
                                        stat_row(ui, "Squadron size", &ftr.squadron_size.to_string());
                                        stat_row(ui, "Torpedoes", &ftr.torpedoes.to_string());
                                        stat_row(ui, "Build cost", &ftr.refined_material_cost.to_string());
                                        stat_row(ui, "Maintenance", &ftr.maintenance_cost.to_string());
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
                                        let kind = if chr.is_major { "Major character" } else { "Minor character" };
                                        stat_row(ui, "Type", kind);
                                        stat_row_pair(ui, "Diplomacy", chr.diplomacy.base, chr.diplomacy.variance);
                                        stat_row_pair(ui, "Espionage", chr.espionage.base, chr.espionage.variance);
                                        stat_row_pair(ui, "Ship Design", chr.ship_design.base, chr.ship_design.variance);
                                        stat_row_pair(ui, "Troop Training", chr.troop_training.base, chr.troop_training.variance);
                                        stat_row_pair(ui, "Facility Design", chr.facility_design.base, chr.facility_design.variance);
                                        stat_row_pair(ui, "Combat", chr.combat.base, chr.combat.variance);
                                        stat_row_pair(ui, "Leadership", chr.leadership.base, chr.leadership.variance);
                                        stat_row_pair(ui, "Loyalty", chr.loyalty.base, chr.loyalty.variance);
                                        if chr.jedi_probability > 0 {
                                            stat_row(ui, "Jedi probability", &format!("{}%", chr.jedi_probability));
                                        }
                                        let mut roles = Vec::new();
                                        if chr.can_be_admiral { roles.push("Admiral"); }
                                        if chr.can_be_general { roles.push("General"); }
                                        if chr.can_be_commander { roles.push("Commander"); }
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
                                                rebellion_core::dat::SectorGroup::RimInner => "Inner Rim",
                                                rebellion_core::dat::SectorGroup::RimOuter => "Outer Rim",
                                            };
                                            stat_row(ui, "Region", region);
                                        }
                                        stat_row(ui, "Position", &format!("({}, {})", system.x, system.y));
                                        stat_row(ui, "Alliance", &format!("{:.0}%", system.popularity_alliance * 100.0));
                                        stat_row(ui, "Empire", &format!("{:.0}%", system.popularity_empire * 100.0));
                                        stat_row(ui, "Fleets", &system.fleets.len().to_string());
                                        stat_row(ui, "Defenses", &system.defense_facilities.len().to_string());
                                        stat_row(ui, "Shipyards", &system.manufacturing_facilities.len().to_string());

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
    // Lazy-load the texture if not yet cached.
    if !state.textures.contains_key(&edata_n) {
        let handle = load_edata_texture(ctx, edata_n, state.edata_path.as_deref());
        state.textures.insert(edata_n, handle);
    }

    if let Some(Some(handle)) = state.textures.get(&edata_n) {
        let size = egui::vec2(400.0, 200.0);
        ui.add(egui::Image::from_texture((handle.id(), size)));
    } else {
        show_placeholder_image(ui);
    }
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

/// Load an EDATA BMP file and register it as an egui texture.
///
/// Returns `None` if the file doesn't exist, can't be read, or fails to decode.
/// On WASM targets, always returns `None` (filesystem access not available).
#[cfg(not(target_arch = "wasm32"))]
fn load_edata_texture(
    ctx: &egui::Context,
    edata_n: u16,
    edata_path: Option<&Path>,
) -> Option<TextureHandle> {
    let dir = edata_path?;
    let file_name = format!("EDATA.{:03}", edata_n);
    let path = dir.join(&file_name);

    if !path.exists() {
        return None;
    }

    let bytes = std::fs::read(&path).ok()?;

    // Decode BMP (8-bit indexed) → RGBA8.
    let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Bmp).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [w as usize, h as usize],
        rgba.as_raw(),
    );

    let handle = ctx.load_texture(
        &format!("edata_{}", edata_n),
        color_image,
        TextureOptions::default(),
    );

    Some(handle)
}

#[cfg(target_arch = "wasm32")]
fn load_edata_texture(
    _ctx: &egui::Context,
    _edata_n: u16,
    _edata_path: Option<&Path>,
) -> Option<TextureHandle> {
    None
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

/// Two-column stat row for SkillPair values (base ± variance).
fn stat_row_pair(ui: &mut egui::Ui, label: &str, base: u32, variance: u32) {
    let value = if variance > 0 {
        format!("{base} ± {variance}")
    } else {
        base.to_string()
    };
    stat_row(ui, label, &value);
}
