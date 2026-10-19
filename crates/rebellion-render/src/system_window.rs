//! The original Manufacturing and Production window (window type 9,
//! `FUN_00452fc0`), which the port calls the System window, and the galaxy
//! view's reference rail of minimized windows.
//!
//! `REBEXE.EXE` creates it 226 by 304 (`FUN_0045aac0`). Six bitmap tabs show
//! the overview's three producer bands (ships, troops, facilities) or one of
//! five facility pages (shipyards, training facilities, construction yards,
//! refineries, mines). Manual pp. 82–86, Figs. 2.10, 3.24, 3.27. Recovery:
//! `ghidra/notes/manufacturing-build-selection.md`, "Window composition".
//! What the pages show is `manufacturing_window.rs`. Modeless windows can be
//! minimized into the faction-specific 12-slot rail and restored without
//! creating duplicates.

use egui_macroquad::egui;
use rebellion_core::dat::Faction;
use rebellion_core::delivery::DeliveryState;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{DatId, FleetKey, SystemKey};
use rebellion_core::manufacturing::{ManufacturingState, ProductionArea};
use rebellion_core::missions::{MissionState, MissionTarget};
use rebellion_core::saved_interface::{SavedWindow, SavedWindowKind};
use rebellion_core::world::{ControlKind, GameWorld};
use rebellion_windowing::WindowId;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::{control_side, paint_native};
use crate::manufacturing_window::{
    band, band_strip, contents_visible, item_frame, page_cells, yard_count, yard_page,
    FacilityItem, FacilityPage, BAND_AREAS, BAND_DESTINATION, BAND_FRAME, BAND_PORTRAIT,
    BAND_RECTS, BAND_STATUS, BAND_TITLE, BAND_UNITS, CELL, COLUMNS, COUNT_ORIGINS, LIST,
    PROGRESS_RECTS, YARD_COLUMN,
};
use crate::object_menu::MenuObject;
use crate::window_focus::GalaxyChild;

pub const SYSTEM_WINDOW_CLIENT_WIDTH: f32 = 226.0;
/// `FUN_0045aac0` creates type 9 at 226 by 304 (`0xe2` by `0x130`).
pub const SYSTEM_WINDOW_WIDTH: f32 = 226.0;
pub const SYSTEM_WINDOW_HEIGHT: f32 = 304.0;
pub const REFERENCE_RAIL_SLOTS: usize = 12;

const WINDOW_BACKGROUND: u32 = 10297;
const CLOSE_NORMAL: u32 = 10108;
const CLOSE_PRESSED: u32 = 10109;
const MINIMIZE_NORMAL: u32 = 10253;
const MINIMIZE_PRESSED: u32 = 10254;
const SECTOR_NORMAL: u32 = 10209;
const SECTOR_PRESSED: u32 = 10208;
/// The title buttons (`FUN_00455060`): the sector button at (3, 3); close
/// right-aligned 3 in (226 - 14 - 3), minimize just left of it.
const SECTOR_X: f32 = 3.0;
const MINIMIZE_X: f32 = 195.0;
const CLOSE_X: f32 = 209.0;
/// The title label `+0x49`: from the sector button's width plus 5, 179 by
/// 16, font 5, black, format `0x24` (left-aligned, vertically centred).
const TITLE: (f32, f32, f32) = (19.0, 2.0, 16.0);
/// The tab strip (`FUN_0060d590`) at (0, 20); each button is 36 by 33.
const TAB_STRIP_Y: f32 = 20.0;
const TAB_SIZE: (f32, f32) = (36.0, 33.0);
/// The page label `+0x5f`: font 4, white, format 1 (`DT_CENTER`), the
/// window's width wide at (0, 58).
const PAGE_LABEL_Y: f32 = 58.0;
/// Each yard count label `+0x19c`: format 1, 46 wide from x 6.
const COUNT_WIDTH: f32 = 46.0;
/// A facility picture sits 1 by 2 into its 69 by 40 cell.
const PICTURE_OFFSET: (f32, f32) = (1.0, 2.0);

/// The six tabs (`FUN_00455060`, command `0x70`, pages `0x67..0x6c`), in
/// strip order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemWindowTab {
    /// `0x67`: the three producer bands.
    Overview,
    Shipyards,
    TrainingFacilities,
    ConstructionYards,
    Refineries,
    Mines,
}

impl SystemWindowTab {
    pub const ALL: [Self; 6] = [
        Self::Overview,
        Self::Shipyards,
        Self::TrainingFacilities,
        Self::ConstructionYards,
        Self::Refineries,
        Self::Mines,
    ];

    /// The button's x in the strip.
    #[must_use]
    pub const fn x(self) -> f32 {
        match self {
            Self::Overview => 0.0,
            Self::Shipyards => 39.0,
            Self::TrainingFacilities => 77.0,
            Self::ConstructionYards => 115.0,
            Self::Refineries => 152.0,
            Self::Mines => 190.0,
        }
    }

    /// The facility page the tab shows, or `None` for the overview.
    #[must_use]
    pub const fn page(self) -> Option<FacilityPage> {
        match self {
            Self::Overview => None,
            Self::Shipyards => Some(FacilityPage::Shipyards),
            Self::TrainingFacilities => Some(FacilityPage::TrainingFacilities),
            Self::ConstructionYards => Some(FacilityPage::ConstructionYards),
            Self::Refineries => Some(FacilityPage::Refineries),
            Self::Mines => Some(FacilityPage::Mines),
        }
    }

    /// The button's help message, which the page label shows: TEXTSTRA
    /// 6197, 6184, 6192, 6194, 6183, 6182.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Overview => "Manufacturing",
            Self::Shipyards => "Shipyards",
            Self::TrainingFacilities => "Training Facilities",
            Self::ConstructionYards => "Construction Yards",
            Self::Refineries => "Refineries",
            Self::Mines => "Mines",
        }
    }
}

/// A tab's bitmap (`FUN_00455060`, `FUN_00456230`): the overview's by the
/// shown side, normal or pressed; a page's normal, pressed, or its empty art
/// in place of the normal one when the page lists no facility. A selected
/// button shows its pressed art (`FUN_0060d700` copies it into the selected
/// state).
#[must_use]
pub const fn tab_resource(tab: SystemWindowTab, side: u8, pressed: bool, empty: bool) -> u32 {
    let (normal, down, none) = match tab {
        SystemWindowTab::Overview => {
            let (normal, down) = match side {
                1 => (10_312, 10_311),
                2 => (10_315, 10_314),
                _ => (10_318, 10_317),
            };
            return if pressed { down } else { normal };
        }
        SystemWindowTab::Shipyards => (10_327, 10_326, 10_328),
        SystemWindowTab::TrainingFacilities => (10_330, 10_329, 10_331),
        SystemWindowTab::ConstructionYards => (10_333, 10_332, 10_334),
        SystemWindowTab::Refineries => (10_324, 10_323, 10_325),
        SystemWindowTab::Mines => (10_321, 10_320, 10_322),
    };
    if pressed {
        down
    } else if empty {
        none
    } else {
        normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Relationship {
    Friendly,
    Hostile,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OpenSystemWindow {
    system: SystemKey,
    /// When this window last took focus (`window_focus`).
    focus_serial: u64,
    logical_position: (i16, i16),
    tab: SystemWindowTab,
    /// A facility page's selected item.
    selected_item: Option<FacilityItem>,
    /// The overview's selected band (band `+0x30` bit 0). port: one band;
    /// Ctrl's toggle is not ported.
    selected_band: Option<ProductionArea>,
}

/// A minimized window on the galaxy view's rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RailEntry {
    System(OpenSystemWindow),
    /// A Fleet window (type 4); its icon comes from `FUN_004a76e0`.
    Fleet {
        id: WindowId,
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A System Defenses window (type 10); its icon comes from
    /// `FUN_004aa4a0`.
    Defenses {
        system: SystemKey,
        logical_position: (i16, i16),
        /// Its place in the z-order, kept while hidden.
        serial: u64,
    },
    /// A Missions window (type 11); its icon comes from `FUN_004a21c0`.
    Missions {
        system: SystemKey,
        logical_position: (i16, i16),
        /// Its place in the z-order, kept while hidden.
        serial: u64,
    },
}

impl RailEntry {
    const fn system(&self) -> SystemKey {
        match self {
            Self::System(window) => window.system,
            Self::Fleet { system, .. }
            | Self::Defenses { system, .. }
            | Self::Missions { system, .. } => *system,
        }
    }

    const fn is_system_window(&self) -> bool {
        matches!(self, Self::System(_))
    }

    /// One slot per window: the window type and its system.
    fn same_window(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fleet { id: left, .. }, Self::Fleet { id: right, .. }) => left == right,
            _ => {
                self.system() == other.system()
                    && std::mem::discriminant(self) == std::mem::discriminant(other)
            }
        }
    }
}

/// Mutable modeless-window and rail state. The last visible window is focused;
/// the first rail entry occupies slot one and is the oldest minimized window.
#[derive(Debug)]
pub struct SystemWindowState {
    faction: CockpitFaction,
    windows: Vec<OpenSystemWindow>,
    rail: Vec<RailEntry>,
    /// Each window's facility-page cells as last painted, for the release
    /// hit test.
    painted: Vec<(SystemKey, Vec<FacilityItem>)>,
    /// The band's name edit a Rename (0x215) opened, if any.
    band_rename: Option<BandRename>,
    /// A window drag in progress.
    title_drag: Option<TitleDrag>,
}

impl Default for SystemWindowState {
    fn default() -> Self {
        Self {
            faction: CockpitFaction::Alliance,
            windows: Vec::new(),
            rail: Vec::new(),
            painted: Vec::new(),
            band_rename: None,
            title_drag: None,
        }
    }
}

/// A band's in-place name edit (window `+0x170`, a CoolStringField), which
/// the band menu's Rename opens over the band's product name
/// (`FUN_00458b50` → `FUN_00458640`), the Rename order pending at `+0x268`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BandRename {
    system: SystemKey,
    area: ProductionArea,
    /// The name line's text when the edit opened (`FUN_00601b20`), which
    /// the commit compares with (`FUN_00458780.c:27-34`).
    original: String,
    text: String,
    /// The first frame selects the whole name and takes the focus
    /// (`FUN_00605110(field, 0, -1)`, `SetFocus`).
    fresh: bool,
}

/// What a frame did to an open band name edit.
#[derive(Debug, Clone, PartialEq, Eq)]
enum BandRenameOutcome {
    Typed(String),
    /// Enter (the field's 0x407) or a press elsewhere in the window: the
    /// edit closes, and its name applies when it changed (`FUN_00458780`).
    Commit(String),
    /// Escape closes the window: under Wine the route-1 Empire save's
    /// Coruscant System window closed with its band's edit open, the name
    /// unchanged (day 127 played on to day 140).
    Escape,
}

impl SystemWindowState {
    /// Whether any window of this kind is open.
    #[must_use]
    pub fn any_open(&self) -> bool {
        !self.windows.is_empty()
    }

    /// Open at the original logical double-click point, clamped so the client
    /// surface remains inside the recovered galaxy aperture, on the overview
    /// (`FUN_00452fc0` starts on page `0x67`). Existing visible or minimized
    /// windows focus or restore instead of duplicating.
    pub fn open(
        &mut self,
        world: &GameWorld,
        system: SystemKey,
        logical_position: (i16, i16),
        faction: CockpitFaction,
        layout: CockpitLayout,
    ) -> bool {
        self.prepare_faction(faction);
        if !world.systems.contains_key(system) {
            return false;
        }
        if self.focus(system) {
            return true;
        }
        if self.restore(system) {
            return true;
        }
        let logical_position = clamp_to_galaxy(logical_position, layout);
        self.windows.push(OpenSystemWindow {
            system,
            focus_serial: crate::window_focus::next_serial(),
            logical_position,
            tab: SystemWindowTab::Overview,
            selected_item: None,
            selected_band: None,
        });
        true
    }

    /// Open `system`'s window on `tab`.
    pub fn open_tab(
        &mut self,
        world: &GameWorld,
        system: SystemKey,
        tab: SystemWindowTab,
        logical_position: (i16, i16),
        faction: CockpitFaction,
        layout: CockpitLayout,
    ) -> bool {
        if !self.open(world, system, logical_position, faction, layout) {
            return false;
        }
        self.select_tab(system, tab);
        true
    }

    /// The screen rect of `tab`'s button in `system`'s visible window.
    #[must_use]
    pub fn tab_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        tab: SystemWindowTab,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(tab_rect(
            window_screen_rect(*window, layout),
            layout.scale,
            tab,
        ))
    }

    /// The screen rect of `area`'s overview band in `system`'s visible
    /// window.
    #[must_use]
    pub fn band_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        area: ProductionArea,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(band_rect(
            window_screen_rect(*window, layout),
            layout.scale,
            area,
        ))
    }

    /// The screen rect of the `index`th cell of `system`'s facility page.
    #[must_use]
    pub fn cell_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        index: usize,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(cell_rect(
            window_screen_rect(*window, layout),
            layout.scale,
            index,
        ))
    }

    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.contains_shown_screen_point(layout, point, &|_| true)
    }

    /// [`contains_screen_point`](Self::contains_screen_point) over the
    /// windows `shown` passes.
    #[must_use]
    pub fn contains_shown_screen_point(
        &self,
        layout: CockpitLayout,
        point: (f32, f32),
        shown: &dyn Fn(GalaxyChild) -> bool,
    ) -> bool {
        self.windows
            .iter()
            .filter(|window| shown(GalaxyChild::System(window.system)))
            .any(|window| {
                let rect = window_screen_rect(*window, layout);
                point.0 >= rect.min.x
                    && point.0 < rect.max.x
                    && point.1 >= rect.min.y
                    && point.1 < rect.max.y
            })
    }

    #[must_use]
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Each visible window's system and logical position, back to front.
    pub fn open_windows(&self) -> impl Iterator<Item = (SystemKey, (i16, i16))> + '_ {
        self.windows
            .iter()
            .map(|window| (window.system, window.logical_position))
    }

    /// The tab and selected band of `system`'s visible window.
    #[must_use]
    pub fn selection(
        &self,
        system: SystemKey,
    ) -> Option<(SystemWindowTab, Option<ProductionArea>)> {
        self.windows
            .iter()
            .find(|window| window.system == system)
            .map(|window| (window.tab, window.selected_band))
    }

    /// The destination a targeting release takes from the system window
    /// egui draws as `layer`: its own system wherever the point lies
    /// (`+0x70`, `FUN_004aa470`). `None` when `layer` is no open system
    /// window.
    #[must_use]
    pub fn release_target(&self, layer: egui::LayerId) -> Option<SystemKey> {
        self.windows
            .iter()
            .find(|window| area_id(window.system) == layer.id)
            .map(|window| window.system)
    }

    /// The object a Mission order released over the system window egui
    /// draws as `layer` names (`+0x68`, `FUN_00458b80`): on a facility page,
    /// the facility in the cell under `point` (list `+0x174` at (8, 77),
    /// `FUN_00609ae0`'s grid); `None` on the overview, off the cells, or on
    /// a pending unit or empty deposit, where the window gives its system.
    #[must_use]
    pub fn release_object(
        &self,
        layout: CockpitLayout,
        layer: egui::LayerId,
        point: egui::Pos2,
    ) -> Option<MissionTarget> {
        let window = self
            .windows
            .iter()
            .find(|window| area_id(window.system) == layer.id)?;
        window.tab.page()?;
        let rect = window_screen_rect(*window, layout);
        let local = (
            (point.x - rect.min.x) / layout.scale,
            (point.y - rect.min.y) / layout.scale,
        );
        let index = grid_cell(local, LIST, CELL, COLUMNS)?;
        let (_, cells) = self
            .painted
            .iter()
            .find(|(system, _)| *system == window.system)?;
        match cells.get(index)? {
            FacilityItem::Manufacturing(key) => Some(MissionTarget::ManufacturingFacility(*key)),
            FacilityItem::Production(key) => Some(MissionTarget::ProductionFacility(*key)),
            FacilityItem::Pending(_) | FacilityItem::Deposit(_) => None,
        }
    }

    #[must_use]
    pub fn rail_count(&self) -> usize {
        self.rail.len()
    }

    /// Each rail entry, oldest first: its system and its window, "system",
    /// "fleet", "defenses" or "missions".
    pub fn rail_entries(&self) -> impl Iterator<Item = (SystemKey, &'static str)> + '_ {
        self.rail.iter().map(|entry| {
            let kind = match entry {
                RailEntry::System(_) => "system",
                RailEntry::Fleet { .. } => "fleet",
                RailEntry::Defenses { .. } => "defenses",
                RailEntry::Missions { .. } => "missions",
            };
            (entry.system(), kind)
        })
    }

    /// The screen rect of the `index`th rail slot while it holds an entry.
    #[must_use]
    pub fn rail_slot_screen_rect(&self, layout: CockpitLayout, index: usize) -> Option<egui::Rect> {
        (index < self.rail.len()).then(|| cockpit_rect(layout, rail_slot_rect(self.faction, index)))
    }

    pub fn clear(&mut self) {
        self.windows.clear();
        self.rail.clear();
        self.painted.clear();
    }

    fn prepare_faction(&mut self, faction: CockpitFaction) {
        if self.faction != faction {
            self.faction = faction;
            self.clear();
        }
    }

    /// When the window in front took focus (`window_focus`); 0 with none
    /// open. Each window keeps its own, so after a close the window behind
    /// ranks where it took focus, as the original's z-order does.
    #[must_use]
    pub fn focus_serial(&self) -> u64 {
        self.windows.last().map_or(0, |window| window.focus_serial)
    }

    /// The open windows as galaxy children, with their z serials; a
    /// minimized window is on the rail, not among them.
    pub fn children(&self) -> impl Iterator<Item = (u64, GalaxyChild)> + '_ {
        self.windows
            .iter()
            .map(|window| (window.focus_serial, GalaxyChild::System(window.system)))
    }

    /// Bring `system`'s window to the top of the galaxy view's z-order, as
    /// its activation does (`FUN_00422ce0` 0x467: `BringWindowToTop`).
    /// False when it is not open.
    pub fn raise(&mut self, system: SystemKey) -> bool {
        self.focus(system)
    }

    /// Close `system`'s window, as its keyed close does (slot `+0x30`).
    /// False when it is not open.
    pub fn close_window(&mut self, system: SystemKey) -> bool {
        let open = self.windows.iter().any(|window| window.system == system);
        self.close(system);
        open
    }

    /// The shown System windows as a save keeps them, each with its z
    /// serial: the system by its DAT id (`FUN_00458ea0`) and the window's
    /// top-left. Minimized windows are in [`Self::saved_rail`].
    #[must_use]
    pub fn saved_windows(&self, world: &GameWorld) -> Vec<(u64, SavedWindow)> {
        self.windows
            .iter()
            .filter_map(|window| {
                Some((
                    window.focus_serial,
                    SavedWindow {
                        kind: SavedWindowKind::System {
                            system: world.systems.get(window.system)?.dat_id,
                        },
                        position: window.logical_position,
                        shown: true,
                    },
                ))
            })
            .collect()
    }

    /// The rail's windows as a save keeps them, each hidden (`+0x110` is 0
    /// on the rail, `FUN_00607d30`) with its place in the z-order: a hidden
    /// window stays in the view's child list, which the save walks bottom to
    /// top (`FUN_00440ea0.c:19-24`). A Fleet window's tab and place come
    /// from `fleet`.
    #[must_use]
    pub fn saved_rail(
        &self,
        world: &GameWorld,
        fleet: impl Fn(WindowId) -> Option<(u32, u64)>,
    ) -> Vec<(u64, SavedWindow)> {
        self.rail
            .iter()
            .filter_map(|entry| {
                let system = world.systems.get(entry.system())?.dat_id;
                let (serial, kind, position) = match *entry {
                    RailEntry::System(window) => (
                        window.focus_serial,
                        SavedWindowKind::System { system },
                        window.logical_position,
                    ),
                    RailEntry::Fleet {
                        id,
                        logical_position,
                        ..
                    } => {
                        let (tab, serial) = fleet(id)?;
                        (
                            serial,
                            SavedWindowKind::Fleet { system, tab },
                            logical_position,
                        )
                    }
                    RailEntry::Defenses {
                        logical_position,
                        serial,
                        ..
                    } => (
                        serial,
                        SavedWindowKind::Defenses { system },
                        logical_position,
                    ),
                    RailEntry::Missions {
                        logical_position,
                        serial,
                        ..
                    } => (
                        serial,
                        SavedWindowKind::Missions { system },
                        logical_position,
                    ),
                };
                Some((
                    serial,
                    SavedWindow {
                        kind,
                        position,
                        shown: false,
                    },
                ))
            })
            .collect()
    }

    /// Rebuild a saved System window (`FUN_00441190` type 9,
    /// `FUN_00452fc0`) at its saved place, above the windows rebuilt before
    /// it, on the overview: the page is not saved (`FUN_00458ea0`), and
    /// under Wine a window saved on Shipyards reopened on the overview
    /// (Empire day-2 start, played on to day 6). A hidden one goes back on
    /// the rail instead (slot `+0x08`, then `FUN_00428b40`), and so does a
    /// hidden Defenses or Missions window, whose rail entry is all it is
    /// while minimized. False for any other record, or a system not in the
    /// world.
    pub fn restore_saved(
        &mut self,
        world: &GameWorld,
        faction: CockpitFaction,
        saved: &SavedWindow,
    ) -> bool {
        let (SavedWindowKind::System { system }
        | SavedWindowKind::Defenses { system }
        | SavedWindowKind::Missions { system }) = saved.kind
        else {
            return false;
        };
        let Some(system) = system_by_dat_id(world, system) else {
            return false;
        };
        self.prepare_faction(faction);
        let position = saved.position;
        match saved.kind {
            SavedWindowKind::System { .. } => {
                self.close(system);
                self.rail
                    .retain(|entry| !(entry.is_system_window() && entry.system() == system));
                let window = OpenSystemWindow {
                    system,
                    focus_serial: crate::window_focus::next_serial(),
                    logical_position: position,
                    tab: SystemWindowTab::Overview,
                    selected_item: None,
                    selected_band: None,
                };
                if saved.shown {
                    self.windows.push(window);
                } else {
                    self.push_rail(RailEntry::System(window));
                }
                true
            }
            SavedWindowKind::Defenses { .. } if !saved.shown => {
                self.push_rail(RailEntry::Defenses {
                    system,
                    logical_position: position,
                    serial: crate::window_focus::next_serial(),
                });
                true
            }
            SavedWindowKind::Missions { .. } if !saved.shown => {
                self.push_rail(RailEntry::Missions {
                    system,
                    logical_position: position,
                    serial: crate::window_focus::next_serial(),
                });
                true
            }
            _ => false,
        }
    }

    fn focus(&mut self, system: SystemKey) -> bool {
        let Some(index) = self
            .windows
            .iter()
            .position(|window| window.system == system)
        else {
            return false;
        };
        let mut window = self.windows.remove(index);
        window.focus_serial = crate::window_focus::next_serial();
        self.windows.push(window);
        true
    }

    fn close(&mut self, system: SystemKey) {
        self.windows.retain(|window| window.system != system);
        self.band_rename.take_if(|edit| edit.system == system);
    }

    /// Open the band's name edit over `area`'s product name in `system`'s
    /// window, as the band menu's Rename does (`FUN_00458b50` →
    /// `FUN_00458640`): the name selected, the window focused and the band
    /// selected. Any edit already open closes unapplied. False when the
    /// window is not open.
    pub fn begin_band_rename(
        &mut self,
        system: SystemKey,
        area: ProductionArea,
        name: &str,
    ) -> bool {
        if !self.windows.iter().any(|window| window.system == system) {
            return false;
        }
        self.select_band(system, area);
        self.band_rename = Some(BandRename {
            system,
            area,
            original: name.to_owned(),
            text: name.to_owned(),
            fresh: true,
        });
        true
    }

    /// Whether a band's name edit holds the keyboard.
    #[must_use]
    pub fn band_rename_open(&self) -> bool {
        self.band_rename.is_some()
    }

    #[cfg(test)]
    fn minimize(&mut self, system: SystemKey) -> bool {
        self.minimize_with_eviction(system).0
    }

    fn minimize_with_eviction(&mut self, system: SystemKey) -> (bool, Option<WindowId>) {
        let Some(index) = self
            .windows
            .iter()
            .position(|window| window.system == system)
        else {
            return (false, None);
        };
        let window = self.windows.remove(index);
        self.band_rename.take_if(|edit| edit.system == system);
        let evicted = Self::evicted_fleet(self.push_rail(RailEntry::System(window)));
        (true, evicted)
    }

    /// Put a minimized Fleet window on the rail (`0x466`).
    pub fn minimize_fleet_window(
        &mut self,
        id: WindowId,
        system: SystemKey,
        logical_position: (i16, i16),
    ) -> Option<WindowId> {
        // A load's hidden Fleet window already holds its slot.
        if self
            .rail
            .iter()
            .any(|entry| matches!(entry, RailEntry::Fleet { id: held, .. } if *held == id))
        {
            return None;
        }
        Self::evicted_fleet(self.push_rail(RailEntry::Fleet {
            id,
            system,
            logical_position,
        }))
    }

    /// Retire the exact temporary Fleet rail projection after the shared
    /// manager and Fleet content have both committed a restore.
    pub fn restore_fleet_window(&mut self, id: WindowId) -> bool {
        let Some(index) = self
            .rail
            .iter()
            .position(|entry| matches!(entry, RailEntry::Fleet { id: held, .. } if *held == id))
        else {
            return false;
        };
        self.rail.remove(index);
        true
    }

    /// Put a minimized Defenses window on the rail (`0x466`), above every
    /// window in the z-order.
    pub fn minimize_defenses_window(
        &mut self,
        system: SystemKey,
        logical_position: (i16, i16),
    ) -> Option<WindowId> {
        self.minimize_defenses_window_at(
            system,
            logical_position,
            crate::window_focus::next_serial(),
        )
    }

    /// Put a minimized Defenses window on the rail (`0x466`), keeping its
    /// place `serial` in the z-order: a hidden window stays where it was in
    /// the view's child list, which a save walks (`FUN_00440ea0.c:19-24`).
    pub fn minimize_defenses_window_at(
        &mut self,
        system: SystemKey,
        logical_position: (i16, i16),
        serial: u64,
    ) -> Option<WindowId> {
        Self::evicted_fleet(self.push_rail(RailEntry::Defenses {
            system,
            logical_position,
            serial,
        }))
    }

    /// Put a minimized Missions window on the rail (`0x466`), above every
    /// window in the z-order.
    pub fn minimize_missions_window(
        &mut self,
        system: SystemKey,
        logical_position: (i16, i16),
    ) -> Option<WindowId> {
        self.minimize_missions_window_at(
            system,
            logical_position,
            crate::window_focus::next_serial(),
        )
    }

    /// Put a minimized Missions window on the rail (`0x466`), keeping its
    /// place `serial` in the z-order (`FUN_00440ea0.c:19-24`).
    pub fn minimize_missions_window_at(
        &mut self,
        system: SystemKey,
        logical_position: (i16, i16),
        serial: u64,
    ) -> Option<WindowId> {
        Self::evicted_fleet(self.push_rail(RailEntry::Missions {
            system,
            logical_position,
            serial,
        }))
    }

    /// Give `child`'s rail entry, a minimized System, Defenses or Missions
    /// window, the next place at the top of the z-order: a load's hidden
    /// window keeps its place among the windows rebuilt around it. False
    /// when `child` has no such entry.
    pub fn lift_hidden(&mut self, child: GalaxyChild) -> bool {
        let serial = crate::window_focus::next_serial();
        let entry = self.rail.iter_mut().find(|entry| match (child, &**entry) {
            (GalaxyChild::System(system), RailEntry::System(window)) => window.system == system,
            (GalaxyChild::Defenses(system), RailEntry::Defenses { system: held, .. })
            | (GalaxyChild::Missions(system), RailEntry::Missions { system: held, .. }) => {
                *held == system
            }
            _ => false,
        });
        match entry {
            Some(RailEntry::System(window)) => window.focus_serial = serial,
            Some(
                RailEntry::Defenses { serial: held, .. } | RailEntry::Missions { serial: held, .. },
            ) => {
                *held = serial;
            }
            Some(RailEntry::Fleet { .. }) | None => return false,
        }
        true
    }

    /// Put a load's hidden Fleet window on the rail at `index`, its place
    /// among the save's hidden windows: `FUN_00441190` appends each hidden
    /// window to the rail as the loader meets it (`FUN_00428b40`). The port
    /// commits Fleet windows after the others, so the slot is taken here
    /// and the manager's minimize keeps it ([`Self::minimize_fleet_window`]).
    pub fn insert_fleet_rail(
        &mut self,
        index: usize,
        id: WindowId,
        system: SystemKey,
        logical_position: (i16, i16),
    ) {
        let entry = RailEntry::Fleet {
            id,
            system,
            logical_position,
        };
        self.rail.retain(|held| !held.same_window(&entry));
        self.rail.insert(index.min(self.rail.len()), entry);
    }

    fn evicted_fleet(entry: Option<RailEntry>) -> Option<WindowId> {
        entry.and_then(|entry| match entry {
            RailEntry::Fleet { id, .. } => Some(id),
            RailEntry::System(_) | RailEntry::Defenses { .. } | RailEntry::Missions { .. } => None,
        })
    }

    fn push_rail(&mut self, entry: RailEntry) -> Option<RailEntry> {
        self.rail.retain(|held| !held.same_window(&entry));
        let evicted = (self.rail.len() == REFERENCE_RAIL_SLOTS).then(|| self.rail.remove(0));
        self.rail.push(entry);
        evicted
    }

    fn restore(&mut self, system: SystemKey) -> bool {
        let Some(index) = self
            .rail
            .iter()
            .position(|entry| entry.is_system_window() && entry.system() == system)
        else {
            return false;
        };
        if let RailEntry::System(mut window) = self.rail.remove(index) {
            window.focus_serial = crate::window_focus::next_serial();
            self.windows.push(window);
        }
        true
    }

    /// Carry a window drag one frame ([`TitleDrag`]): the window follows
    /// the pointer while the button is down, and the release lets go.
    fn update_title_drag(&mut self, ctx: &egui::Context, layout: CockpitLayout) {
        if let Some(drag) = self.title_drag {
            let window = self
                .windows
                .iter_mut()
                .find(|window| area_id(window.system) == drag.area);
            match (window, title_drag_step(ctx, layout, drag)) {
                (Some(window), Ok(place)) => {
                    if let Some(place) = place {
                        window.logical_position = place;
                    }
                }
                _ => self.title_drag = None,
            }
        }
    }

    fn window_mut(&mut self, system: SystemKey) -> Option<&mut OpenSystemWindow> {
        self.windows
            .iter_mut()
            .find(|window| window.system == system)
    }

    /// Show `tab`'s page; a new page clears the item selection
    /// (`FUN_004568a0` refills the list).
    fn select_tab(&mut self, system: SystemKey, tab: SystemWindowTab) {
        if let Some(window) = self.window_mut(system) {
            if window.tab != tab {
                window.tab = tab;
                window.selected_item = None;
            }
        }
        self.focus(system);
    }

    fn select_item(&mut self, system: SystemKey, item: Option<FacilityItem>) {
        if let Some(window) = self.window_mut(system) {
            window.selected_item = item;
        }
        self.focus(system);
    }

    fn select_band(&mut self, system: SystemKey, area: ProductionArea) {
        if let Some(window) = self.window_mut(system) {
            window.selected_band = Some(area);
        }
        self.focus(system);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemWindowAction {
    FocusSector(SystemKey),
    SelectSystem(SystemKey),
    /// A legacy System minimize displaced this exact manager-owned Fleet
    /// projection from the shared twelve-slot rail.
    EvictFleetWindow(WindowId),
    /// A right-button release on a band opens the object pop-up menu
    /// (`FUN_004ac5c0`) for the selection, at a 640 by 480 canvas point.
    OpenObjectMenu {
        system: SystemKey,
        selection: Option<MenuObject>,
        point: (i16, i16),
    },
    /// A click on a minimized Fleet window's rail slot restores it.
    RestoreFleetWindow {
        id: WindowId,
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A click on a minimized Defenses window's rail slot restores it.
    RestoreDefensesWindow {
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A click on a minimized Missions window's rail slot restores it.
    RestoreMissionsWindow {
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// The band's name edit applied a new name: the band's Rename (0x215)
    /// names the manager's current product (`FUN_0057e870` → manager
    /// `+0x1f4` → `FUN_00529980`, whose notifier `FUN_0052ba00` renames the
    /// product through `FUN_004f6e60`).
    RenameProduct {
        system: SystemKey,
        area: ProductionArea,
        name: String,
    },
}

#[derive(Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Each flag is one independent title-bar or list outcome of a frame."
)]
struct WindowDrawResult {
    /// Where a press on the window's own surface took hold ([`TitleDrag`]).
    drag_grab: Option<egui::Vec2>,
    focus: bool,
    close: bool,
    minimize: bool,
    focus_sector: bool,
    tab: Option<SystemWindowTab>,
    /// A press on a facility page's list: the item under it, or none.
    item: Option<Option<FacilityItem>>,
    /// The facility page's cells as painted.
    painted: Vec<FacilityItem>,
    band: Option<ProductionArea>,
    object_menu: Option<(Option<MenuObject>, (i16, i16))>,
    rename: Option<BandRenameOutcome>,
}

/// What a frame's drawing reads.
#[derive(Clone, Copy)]
struct Sources<'a> {
    world: &'a GameWorld,
    manufacturing: &'a ManufacturingState,
    deliveries: &'a DeliveryState,
    fog: &'a FogState,
}

/// Draw the faction rail and every visible Manufacturing and Production
/// window.
///
/// port: the window's keys (`FUN_00458980`) are not ported; the galaxy
/// view's windows keep no shared keyboard focus to route them by.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit state and rendering inputs at this UI boundary; the rail's Missions icon reads the mission state."
)]
pub fn draw_system_windows(
    ctx: &egui::Context,
    world: &GameWorld,
    movement: &rebellion_core::movement::MovementState,
    fog: &FogState,
    missions: &MissionState,
    manufacturing: &ManufacturingState,
    deliveries: &DeliveryState,
    state: &mut SystemWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    front: bool,
    shown: &dyn Fn(GalaxyChild) -> bool,
) -> Vec<SystemWindowAction> {
    state.prepare_faction(faction);
    let mut actions: Vec<SystemWindowAction> = draw_reference_rail(
        ctx, world, movement, fog, missions, state, faction, layout, cache,
    )
    .into_iter()
    .collect();
    let sources = Sources {
        world,
        manufacturing,
        deliveries,
        fog,
    };
    state.update_title_drag(ctx, layout);
    // A window the GID menu hides is neither painted nor live.
    let windows: Vec<_> = state
        .windows
        .iter()
        .filter(|window| shown(GalaxyChild::System(window.system)))
        .cloned()
        .collect();
    let focused_system = windows.last().map(|window| window.system);
    for window in windows {
        let system = window.system;
        let rename = state
            .band_rename
            .clone()
            .filter(|edit| edit.system == system);
        let result = draw_system_window(
            ctx,
            sources,
            window,
            rename.as_ref(),
            front && focused_system == Some(system),
            faction,
            layout,
            cache,
        );
        if let Some(grab) = result.drag_grab {
            state.title_drag = Some(TitleDrag {
                area: area_id(window.system),
                grab,
            });
        }
        match (result.rename.clone(), rename) {
            (Some(BandRenameOutcome::Typed(text)), Some(_)) => {
                if let Some(edit) = state.band_rename.as_mut() {
                    edit.text = text;
                    edit.fresh = false;
                }
            }
            (Some(BandRenameOutcome::Commit(text)), Some(edit)) => {
                state.band_rename = None;
                // FUN_00458780: a changed, non-empty name applies; any other
                // drops the order.
                if !text.is_empty() && text != edit.original {
                    actions.push(SystemWindowAction::RenameProduct {
                        system,
                        area: edit.area,
                        name: text,
                    });
                }
            }
            (Some(BandRenameOutcome::Escape), Some(_)) => {
                state.close(system);
                continue;
            }
            _ => {}
        }
        state.painted.retain(|(painted, _)| *painted != system);
        state.painted.push((system, result.painted));
        if result.close {
            state.close(system);
            continue;
        }
        if result.minimize {
            let (_, evicted) = state.minimize_with_eviction(system);
            actions.extend(evicted.map(SystemWindowAction::EvictFleetWindow));
            continue;
        }
        if result.focus_sector {
            actions.push(SystemWindowAction::FocusSector(system));
            state.close(system);
            continue;
        }
        if let Some(tab) = result.tab {
            state.select_tab(system, tab);
        }
        if let Some(item) = result.item {
            state.select_item(system, item);
        }
        if let Some(area) = result.band {
            state.select_band(system, area);
        }
        if result.focus {
            state.focus(system);
            actions.push(SystemWindowAction::SelectSystem(system));
        }
        if let Some((selection, point)) = result.object_menu {
            actions.push(SystemWindowAction::OpenObjectMenu {
                system,
                selection,
                point,
            });
        }
    }
    actions
}

#[expect(
    clippy::too_many_arguments,
    reason = "The rail's icons read the world, fog and mission state."
)]
fn draw_reference_rail(
    ctx: &egui::Context,
    world: &GameWorld,
    movement: &rebellion_core::movement::MovementState,
    fog: &FogState,
    missions: &MissionState,
    state: &mut SystemWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<SystemWindowAction> {
    let entries = state.rail.clone();
    let mut restored = None;
    for (index, entry) in entries.iter().enumerate() {
        let Some(system) = world.systems.get(entry.system()) else {
            continue;
        };
        let logical = rail_slot_rect(faction, index);
        let screen_rect = cockpit_rect(layout, logical);
        let color = relationship_color(relationship(system.control, cockpit_faction(faction)));
        egui::Area::new(egui::Id::new(("original-reference-rail", index)))
            .fixed_pos(screen_rect.min)
            .order(egui::Order::Middle)
            .show(ctx, |ui| {
                let (slot_rect, response) =
                    ui.allocate_exact_size(screen_rect.size(), egui::Sense::click());
                if entry.is_system_window() {
                    paint_resource(ui.painter(), ctx, cache, WINDOW_BACKGROUND, slot_rect);
                    ui.painter().rect_filled(
                        slot_rect,
                        0.0,
                        egui::Color32::from_rgba_unmultiplied(0, 0, 12, 122),
                    );
                    ui.painter().text(
                        slot_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &system.name,
                        egui::FontId::proportional((7.0 * layout.scale).max(5.0)),
                        color,
                    );
                } else {
                    // A Fleet window's rail icon (`FUN_004a76e0`) sits at the
                    // slot's corner with the system's name after it, as the
                    // reference capture shows. port: a Defenses window's
                    // (`FUN_004aa4a0`) and a Missions window's
                    // (`FUN_004a21c0`) are laid out the same way.
                    let icon = match entry {
                        RailEntry::Defenses { system, .. } => {
                            crate::defenses_window::rail_icon(world, *system)
                        }
                        RailEntry::Missions { system, .. } => crate::missions_window::rail_icon(
                            world, fog, missions, faction, *system,
                        ),
                        _ => crate::fleet_window::rail_icon(
                            world,
                            movement,
                            fog,
                            cockpit_faction(faction),
                            entry.system(),
                        ),
                    };
                    paint_native(
                        ui.painter(),
                        ctx,
                        cache,
                        DllSource::Strategy,
                        icon,
                        slot_rect,
                        layout.scale,
                        0.0,
                        0.0,
                    );
                    ui.painter().text(
                        logical_point(slot_rect, layout.scale, 12.0, 2.0),
                        egui::Align2::LEFT_TOP,
                        &system.name,
                        egui::FontId::proportional((9.0 * layout.scale).max(6.0)),
                        color,
                    );
                }
                if exact_clicked(&response, slot_rect) {
                    restored = Some(*entry);
                }
            });
    }
    match restored? {
        RailEntry::System(window) => {
            state.restore(window.system);
            None
        }
        RailEntry::Fleet {
            id,
            system,
            logical_position,
        } => Some(SystemWindowAction::RestoreFleetWindow {
            id,
            system,
            logical_position,
        }),
        entry @ RailEntry::Defenses {
            system,
            logical_position,
            ..
        } => {
            state.rail.retain(|held| !held.same_window(&entry));
            Some(SystemWindowAction::RestoreDefensesWindow {
                system,
                logical_position,
            })
        }
        entry @ RailEntry::Missions {
            system,
            logical_position,
            ..
        } => {
            state.rail.retain(|held| !held.same_window(&entry));
            Some(SystemWindowAction::RestoreMissionsWindow {
                system,
                logical_position,
            })
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "Keep the window's ordered paint and input pass together, as the Defenses window does."
)]
#[expect(
    clippy::too_many_arguments,
    reason = "The window reads its sources, its edit and the frame's layout."
)]
fn draw_system_window(
    ctx: &egui::Context,
    sources: Sources<'_>,
    window: OpenSystemWindow,
    rename: Option<&BandRename>,
    focused: bool,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> WindowDrawResult {
    let mut result = WindowDrawResult::default();
    let world = sources.world;
    let Some(system) = world.systems.get(window.system) else {
        result.close = true;
        return result;
    };
    let player = cockpit_faction(faction);
    let side = control_side(system.control);
    let visible = contents_visible(world, sources.fog, player, window.system);
    let known = if sources.fog.faction == player {
        sources.fog.known_facilities(window.system)
    } else {
        &[]
    };
    let pages = SystemWindowTab::ALL.map(|tab| {
        tab.page().map(|page| {
            page_cells(
                world,
                sources.manufacturing,
                sources.deliveries,
                visible,
                known,
                window.system,
                page,
            )
        })
    });
    let scale = layout.scale;
    let screen_rect = window_screen_rect(window, layout);
    let id = area_id(window.system);
    if focused {
        ctx.move_to_top(egui::LayerId::new(egui::Order::Foreground, id));
    }
    let area = egui::Area::new(id)
        .fixed_pos(screen_rect.min)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let (pointer, primary_down) = ctx.input(|input| {
                (
                    input.pointer.interact_pos(),
                    input.pointer.button_down(egui::PointerButton::Primary),
                )
            });
            let (local, window_response) =
                ui.allocate_exact_size(screen_rect.size(), egui::Sense::click());
            let painter = ui.painter().clone();
            let paint = |cache: &mut BmpCache, source: DllSource, id: u32, x: f32, y: f32| {
                paint_native(&painter, ctx, cache, source, id, local, scale, x, y);
            };

            painter.rect_filled(local, 0.0, egui::Color32::BLACK);
            paint(cache, DllSource::Strategy, WINDOW_BACKGROUND, 0.0, 0.0);
            paint(
                cache,
                DllSource::Strategy,
                crate::defenses_window::title_resource(side, focused),
                2.0,
                2.0,
            );
            painter.text(
                logical_rect(local, scale, TITLE.0, TITLE.1 + TITLE.2 / 2.0, 0.0, 0.0).min,
                egui::Align2::LEFT_CENTER,
                &system.name,
                egui::FontId::proportional((11.0 * scale).max(7.0)),
                egui::Color32::BLACK,
            );

            let buttons = [
                (SECTOR_NORMAL, SECTOR_PRESSED, SECTOR_X, "sector"),
                (MINIMIZE_NORMAL, MINIMIZE_PRESSED, MINIMIZE_X, "minimize"),
                (CLOSE_NORMAL, CLOSE_PRESSED, CLOSE_X, "close"),
            ];
            let mut clicked = [false; 3];
            for (index, (normal, pressed, x, name)) in buttons.into_iter().enumerate() {
                let rect = logical_rect(local, scale, x, 3.0, 14.0, 14.0);
                let response = ui.interact(
                    rect,
                    ui.id().with((window.system, name)),
                    egui::Sense::click(),
                );
                let down = primary_down && pointer.is_some_and(|point| rect_contains(rect, point));
                paint(
                    cache,
                    DllSource::Strategy,
                    if down { pressed } else { normal },
                    x,
                    3.0,
                );
                clicked[index] = exact_clicked(&response, rect);
            }
            result.focus_sector = clicked[0];
            result.minimize = clicked[1];
            result.close = clicked[2];

            // The tabs. port: an empty page's tab still opens it, as the
            // Defenses window's do; its help message is not shown.
            for (tab, cells) in SystemWindowTab::ALL.into_iter().zip(&pages) {
                let rect = tab_rect(local, scale, tab);
                let empty = cells
                    .as_deref()
                    .is_some_and(crate::manufacturing_window::page_is_empty);
                let response = ui.interact(
                    rect,
                    ui.id().with((window.system, tab)),
                    egui::Sense::click(),
                );
                let pressed = tab == window.tab
                    || (primary_down && pointer.is_some_and(|point| rect_contains(rect, point)));
                paint(
                    cache,
                    DllSource::Strategy,
                    tab_resource(tab, side, pressed, empty),
                    tab.x(),
                    TAB_STRIP_Y,
                );
                if exact_clicked(&response, rect) {
                    result.tab = Some(tab);
                    result.focus = true;
                }
            }

            let font = egui::FontId::proportional(OVERVIEW_FONT_EM * scale);
            let mut rename_field = None;
            let (any_pressed, right_pressed) = ctx.input(|input| {
                (
                    input.pointer.button_pressed(egui::PointerButton::Primary)
                        || input.pointer.button_pressed(egui::PointerButton::Secondary),
                    input.pointer.button_pressed(egui::PointerButton::Secondary),
                )
            });
            match window.tab.page() {
                None => {
                    // FUN_00457690: the yard column and its counts.
                    paint(
                        cache,
                        DllSource::Strategy,
                        YARD_COLUMN.0,
                        YARD_COLUMN.1,
                        YARD_COLUMN.2,
                    );
                    for (index, area) in BAND_AREAS.into_iter().enumerate() {
                        let cells = pages[SystemWindowTab::ALL
                            .iter()
                            .position(|tab| tab.page() == Some(yard_page(area)))
                            .unwrap_or_default()]
                        .as_deref()
                        .unwrap_or_default();
                        let (x, y) = COUNT_ORIGINS[index];
                        painter.text(
                            logical_point(local, scale, x + COUNT_WIDTH / 2.0, y),
                            egui::Align2::CENTER_TOP,
                            yard_count(cells),
                            font.clone(),
                            egui::Color32::WHITE,
                        );

                        // FUN_00458080: the band's frame, its strip and its
                        // lines.
                        let shown =
                            band(world, sources.manufacturing, visible, window.system, area);
                        let (bx, by, _, _) = BAND_RECTS[index];
                        let selected = window.selected_band == Some(area);
                        paint(cache, DllSource::Strategy, BAND_FRAME, bx, by);
                        paint(
                            cache,
                            DllSource::Strategy,
                            band_strip(side, selected),
                            bx,
                            by,
                        );
                        let line_in = |text: &str, (x, y): (f32, f32), color| {
                            painter.text(
                                logical_point(local, scale, bx + x, by + y),
                                egui::Align2::LEFT_TOP,
                                text,
                                font.clone(),
                                color,
                            );
                        };
                        let line = |text: &str, at: (f32, f32)| {
                            line_in(text, at, egui::Color32::WHITE);
                        };
                        // The title keeps the label's default colour, black
                        // (no FUN_00601c90, FUN_00458080.c:76-78).
                        line_in(
                            crate::manufacturing_window::band_title(area),
                            BAND_TITLE,
                            egui::Color32::BLACK,
                        );
                        // The portrait, keyed, then the units, the name
                        // and the destination over it (FUN_00457690.c:76-82).
                        if let Some(portrait) = shown.portrait {
                            paint(
                                cache,
                                DllSource::Gokres,
                                portrait,
                                bx + BAND_PORTRAIT.0,
                                by + BAND_PORTRAIT.1,
                            );
                        }
                        if let Some(units) = &shown.units {
                            line(units, BAND_UNITS);
                        }
                        match rename.filter(|edit| edit.area == area) {
                            Some(edit) => {
                                let field = band_rename_rect(
                                    ui,
                                    local,
                                    scale,
                                    (bx, by),
                                    &edit.original,
                                    &font,
                                );
                                rename_field = Some((field, edit));
                            }
                            None => line(&shown.status, BAND_STATUS),
                        }
                        line(&shown.destination, BAND_DESTINATION);

                        // FUN_004acec0: the progress bar, light gray over
                        // black.
                        let (px, py, width, height) = PROGRESS_RECTS[index];
                        let bar = logical_rect(local, scale, px, py, width, height);
                        painter.rect_filled(bar, 0.0, egui::Color32::BLACK);
                        if let Some(progress) = shown.progress {
                            let mut done = bar;
                            done.set_width(bar.width() * progress.clamp(0.0, 1.0));
                            painter.rect_filled(done, 0.0, egui::Color32::from_gray(240));
                        }

                        // A press selects the band; a right release opens
                        // its manager's menu (FUN_00453ee0, FUN_004ac5c0).
                        let rect = band_rect(local, scale, area);
                        let response = ui.interact(
                            rect,
                            ui.id().with((window.system, "band", index)),
                            egui::Sense::click(),
                        );
                        if any_pressed && response.is_pointer_button_down_on() {
                            result.band = Some(area);
                            result.focus = true;
                        }
                        if response.secondary_clicked() {
                            if let Some(point) = response
                                .interact_pointer_pos()
                                .filter(|point| rect_contains(rect, *point))
                            {
                                result.object_menu = Some((
                                    Some(MenuObject::Producer {
                                        system: window.system,
                                        area,
                                    }),
                                    canvas_point(layout, point),
                                ));
                            }
                        }
                    }
                    // The edit lies over its band and takes the pointer
                    // there. A press anywhere else in the window ends it as
                    // Enter does (FUN_004534f0, 0x201 and 0x204 →
                    // FUN_00458780).
                    if let Some((field, edit)) = rename_field {
                        result.rename = draw_band_rename(ui, field, edit, &font);
                        let elsewhere = pointer.is_some_and(|point| {
                            rect_contains(local, point) && !rect_contains(field, point)
                        });
                        if any_pressed && elsewhere {
                            result.rename = Some(BandRenameOutcome::Commit(match result.rename {
                                Some(BandRenameOutcome::Typed(ref text)) => text.clone(),
                                _ => edit.text.clone(),
                            }));
                        }
                    }
                }
                Some(_) => {
                    // The page label, then the list (FUN_004568a0).
                    painter.text(
                        logical_point(local, scale, SYSTEM_WINDOW_WIDTH / 2.0, PAGE_LABEL_Y),
                        egui::Align2::CENTER_TOP,
                        window.tab.name(),
                        egui::FontId::proportional((10.0 * scale).max(7.0)),
                        egui::Color32::WHITE,
                    );
                    let cells = pages[SystemWindowTab::ALL
                        .iter()
                        .position(|tab| *tab == window.tab)
                        .unwrap_or_default()]
                    .as_deref()
                    .unwrap_or_default();
                    let list =
                        logical_rect(local, scale, LIST.0, LIST.1, LIST.2, LIST.3).intersect(local);
                    let list_painter = painter.with_clip_rect(list);
                    // A press on the list's empty space clears the
                    // selection (FUN_006094b0).
                    let list_response = ui.interact(
                        list,
                        ui.id().with((window.system, "list")),
                        egui::Sense::click(),
                    );
                    if any_pressed && list_response.is_pointer_button_down_on() {
                        result.item = Some(None);
                        result.focus = true;
                    }
                    result.painted = cells.iter().map(|cell| cell.item).collect();
                    for (index, cell) in cells.iter().enumerate() {
                        let rect = cell_rect(local, scale, index);
                        if !rect.intersects(list) {
                            break;
                        }
                        paint_native(
                            &list_painter,
                            ctx,
                            cache,
                            DllSource::Strategy,
                            cell.picture,
                            rect,
                            scale,
                            PICTURE_OFFSET.0,
                            PICTURE_OFFSET.1,
                        );
                        if window.selected_item == Some(cell.item) {
                            paint_native(
                                &list_painter,
                                ctx,
                                cache,
                                DllSource::Strategy,
                                item_frame(side),
                                rect,
                                scale,
                                PICTURE_OFFSET.0,
                                PICTURE_OFFSET.1,
                            );
                        }
                        // A left or right press selects the item
                        // (FUN_006083c0). port: a facility's class menu
                        // (Encyclopedia, Status, Scrap) is not ported, so a
                        // right release opens none.
                        let response = ui.interact(
                            rect.intersect(list),
                            ui.id().with((window.system, "cell", index)),
                            egui::Sense::click(),
                        );
                        if (exact_clicked(&response, rect)
                            || (right_pressed && response.is_pointer_button_down_on()))
                            && result.item.is_none()
                        {
                            result.item = Some(Some(cell.item));
                            result.focus = true;
                        }
                    }
                }
            }

            result.drag_grab = surface_drag_grab(ctx, &window_response, local, scale, &[]);
            if window_response.clicked() || clicked.iter().any(|&value| value) {
                result.focus = true;
            }
        });
    if area.response.clicked() {
        result.focus = true;
    }
    result
}

fn area_id(system: SystemKey) -> egui::Id {
    egui::Id::new(("original-system-window", system))
}

fn tab_rect(window: egui::Rect, scale: f32, tab: SystemWindowTab) -> egui::Rect {
    logical_rect(window, scale, tab.x(), TAB_STRIP_Y, TAB_SIZE.0, TAB_SIZE.1)
}

fn band_rect(window: egui::Rect, scale: f32, area: ProductionArea) -> egui::Rect {
    let index = BAND_AREAS
        .iter()
        .position(|value| *value == area)
        .unwrap_or_default();
    let (x, y, width, height) = BAND_RECTS[index];
    logical_rect(window, scale, x, y, width, height)
}

/// The index of the grid cell (`FUN_00609ae0`) holding a window-local
/// point: `list` is the list's rect, `cell` each cell's size, row-major
/// with `columns` to a row from the list's corner. `None` off the list.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "The point lies inside the list, so the column and row are small and non-negative."
)]
pub(crate) fn grid_cell(
    point: (f32, f32),
    list: (f32, f32, f32, f32),
    cell: (f32, f32),
    columns: usize,
) -> Option<usize> {
    let (x, y) = (point.0 - list.0, point.1 - list.1);
    if !(0.0..list.2).contains(&x) || !(0.0..list.3).contains(&y) {
        return None;
    }
    let column = (x / cell.0) as usize;
    (column < columns).then(|| (y / cell.1) as usize * columns + column)
}

/// The `index`th cell of a facility page (`FUN_00609ae0`): row-major from
/// the list's corner, three to a row.
#[expect(clippy::cast_precision_loss, reason = "Cell indexes are small.")]
fn cell_rect(window: egui::Rect, scale: f32, index: usize) -> egui::Rect {
    let column = (index % COLUMNS) as f32;
    let row = (index / COLUMNS) as f32;
    logical_rect(
        window,
        scale,
        LIST.0 + column * CELL.0,
        LIST.1 + row * CELL.1,
        CELL.0,
        CELL.1,
    )
}

/// A screen point in 640 by 480 canvas coordinates, truncated as a Win32
/// `POINT` is.
#[expect(
    clippy::cast_possible_truncation,
    reason = "Canvas coordinates fit an i16, as the original's POINT words do."
)]
pub(crate) fn canvas_point(layout: CockpitLayout, point: egui::Pos2) -> (i16, i16) {
    let scale = layout.scale.max(f32::EPSILON);
    (
        ((point.x - layout.canvas.x) / scale).floor() as i16,
        ((point.y - layout.canvas.y) / scale).floor() as i16,
    )
}

/// Whether the player sees the other side's objects at `system`: it holds
/// the system, or its fog shows the system now.
pub(crate) fn opposing_contents_visible(
    world: &GameWorld,
    fog: &FogState,
    player: Faction,
    system: SystemKey,
) -> bool {
    world
        .systems
        .get(system)
        .is_some_and(|value| value.control.is_controlled_by(player))
        || (fog.faction == player && fog.is_visible(system))
}

/// A fleet's name, as its windows list it (`FUN_004f6270`): "Fleet N"
/// from its side's counter (`GameWorld::fleet_name`). Rename is not ported.
#[must_use]
pub fn fleet_label(world: &GameWorld, fleet: FleetKey) -> Option<String> {
    world.fleet_name(fleet).map(str::to_owned)
}

pub(crate) use rebellion_core::tactical::character_mini_resource_id;

pub(crate) fn manufacturing_facility_mini(dat_id: DatId) -> Option<(u32, &'static str)> {
    let label = match dat_id.index() {
        1 => "Orbital Shipyard",
        2 => "Training Facility",
        3 => "Construction Yard",
        4 => "Advanced Shipyard",
        5 => "Advanced Training Facility",
        6 => "Advanced Construction Yard",
        _ => return None,
    };
    matches!(dat_id.family(), 0x20 | 0x28..=0x2a).then_some(if dat_id.family() == 0x20 {
        (17_216, "Alliance Headquarters")
    } else {
        (16_639 + dat_id.index(), label)
    })
}

pub(crate) fn defense_facility_mini(dat_id: DatId) -> Option<(u32, &'static str)> {
    let (resource_id, label) = match dat_id.index() {
        1 => (16_896, "KDY-150"),
        2 => (16_897, "LNR Series I"),
        3 => (16_898, "GenCore Level I"),
        4 => (17_024, "Death Star Shield"),
        5 => (16_899, "LNR Series II"),
        6 => (16_900, "GenCore Level II"),
        _ => return None,
    };
    matches!(dat_id.family(), 0x22..=0x25).then_some((resource_id, label))
}

pub(crate) fn production_facility_mini(dat_id: DatId) -> Option<(u32, &'static str)> {
    if !matches!(dat_id.family(), 0x2c..=0x2d) {
        return None;
    }
    match dat_id.index() {
        1 => Some((16_385, "Mine")),
        2 => Some((16_386, "Refinery")),
        _ => None,
    }
}

pub(crate) fn troop_mini(dat_id: DatId) -> Option<(u32, &'static str)> {
    if dat_id.family() != 0x10 {
        return None;
    }
    let (resource_id, label) = match dat_id.index() {
        1 => (17_472, "Alliance Fleet Regiment"),
        2 => (17_473, "Alliance Army Regiment"),
        3 => (17_474, "Sullustan Regiment"),
        4 => (17_475, "Wookiee Regiment"),
        5 => (17_476, "Mon Calamari Regiment"),
        6 => (17_536, "Stormtrooper Regiment"),
        7 => (17_537, "Imperial Fleet Regiment"),
        8 => (17_538, "Imperial Army Regiment"),
        9 => (17_539, "War Droid Regiment"),
        10 => (17_540, "Dark Trooper Regiment"),
        _ => return None,
    };
    Some((resource_id, label))
}

/// A special force's GOKRES mini, `(TEXTSTRA id & 0xfff) + 0x4000`
/// (`FUN_0042c3b0`, slot `+0x30` = `FUN_004f63d0`), and its class name.
#[must_use]
pub fn special_force_mini(dat_id: DatId) -> Option<(u32, &'static str)> {
    if dat_id.family() != 0x3c {
        return None;
    }
    let (resource_id, label) = match dat_id.index() {
        1 => (17_728, "Guerrillas"),
        2 => (17_729, "Infiltrators"),
        3 => (17_730, "Longprobe Y-wing Recon Team"),
        4 => (17_731, "Bothan Spies"),
        5 => (17_792, "Imperial Probe Droid"),
        6 => (17_793, "Imperial Espionage Droid"),
        7 => (17_794, "Imperial Commandos"),
        8 => (17_795, "Noghri Death Commandos"),
        9 => (17_796, "Bounty Hunters"),
        _ => return None,
    };
    Some((resource_id, label))
}

fn relationship(control: ControlKind, player: Faction) -> Relationship {
    match control.faction() {
        Some(owner) if owner == player => Relationship::Friendly,
        Some(_) => Relationship::Hostile,
        None => Relationship::Neutral,
    }
}

fn relationship_color(relationship: Relationship) -> egui::Color32 {
    match relationship {
        Relationship::Friendly => egui::Color32::from_rgb(0, 255, 64),
        Relationship::Hostile => egui::Color32::from_rgb(255, 32, 32),
        Relationship::Neutral => egui::Color32::from_rgb(0, 240, 240),
    }
}

fn cockpit_faction(faction: CockpitFaction) -> Faction {
    match faction {
        CockpitFaction::Alliance => Faction::Alliance,
        CockpitFaction::Empire => Faction::Empire,
    }
}

/// Where a band's name edit lies: over the band's product-name line, as
/// wide as its text plus 2 and 13 high (`FUN_00458640` takes the label's
/// rect, `+0x14..+0x20`, two pixels wider). hyp (measured): under Wine the
/// selected "Death Star 1" filled 61 by 13 pixels from the line's top-left
/// (route-1 Empire save, day 123 played on).
fn band_rename_rect(
    ui: &egui::Ui,
    local: egui::Rect,
    scale: f32,
    (bx, by): (f32, f32),
    original: &str,
    font: &egui::FontId,
) -> egui::Rect {
    let width = ui.fonts(|fonts| {
        fonts
            .layout_no_wrap(original.to_owned(), font.clone(), egui::Color32::WHITE)
            .size()
            .x
    }) / scale;
    logical_rect(
        local,
        scale,
        bx + BAND_STATUS.0,
        by + BAND_STATUS.1,
        width + 2.0,
        BAND_RENAME_HEIGHT,
    )
}

/// The overview's text size: font 10 for the band labels (`FUN_00458080`,
/// `FUN_00601c60(label, 10)`) and the yard counts, `FUN_0060eed0` case 10:
/// Arial with a 14-pixel cell at weight 400. hyp (measured): under Wine its
/// glyphs are Arial's hinted at an 11-pixel em ("No Troops in training"
/// and "No Facilities are being built" 98 and 126 pixels of ink, route-1
/// Empire save, day 123 played on), which egui's outline advances meet at
/// 10.6.
const OVERVIEW_FONT_EM: f32 = 10.6;

/// The band name edit's height (hyp, measured; see [`band_rename_rect`]).
const BAND_RENAME_HEIGHT: f32 = 13.0;

/// Draw a band's name edit and read what the frame did to it. The field
/// draws its text white on the band; the selected text shows black on
/// white, and a whole selected name fills the field white: under Wine the
/// field opened as a white box with "Death Star 1" in black, and the typed
/// "Tarkin" showed white on the band (route-1 Empire save, day 123 played
/// on).
fn draw_band_rename(
    ui: &mut egui::Ui,
    field: egui::Rect,
    edit: &BandRename,
    font: &egui::FontId,
) -> Option<BandRenameOutcome> {
    let id = ui.id().with(("band-rename", edit.system, edit.area));
    let count = edit.text.chars().count();
    let selected = if edit.fresh {
        Some(0..count)
    } else {
        egui::TextEdit::load_state(ui.ctx(), id)
            .and_then(|state| state.cursor.char_range())
            .map(|range| {
                let [start, end] = range.sorted();
                start.index..end.index
            })
            .filter(|range| !range.is_empty())
    };
    if count > 0 && selected == Some(0..count) {
        ui.painter().rect_filled(field, 0.0, egui::Color32::WHITE);
    }
    let mut layouter = |ui: &egui::Ui, value: &str, _wrap: f32| {
        let mut job = egui::text::LayoutJob::default();
        for (index, character) in value.chars().enumerate() {
            let color = if selected
                .as_ref()
                .is_some_and(|range| range.contains(&index))
            {
                egui::Color32::BLACK
            } else {
                egui::Color32::WHITE
            };
            job.append(
                character.encode_utf8(&mut [0; 4]),
                0.0,
                egui::TextFormat::simple(font.clone(), color),
            );
        }
        ui.fonts(|fonts| fonts.layout_job(job))
    };
    let mut text = edit.text.clone();
    let response = ui
        .scope(|ui| {
            let visuals = ui.visuals_mut();
            visuals.selection.bg_fill = egui::Color32::WHITE;
            visuals.text_cursor.stroke.color = egui::Color32::WHITE;
            ui.put(
                field,
                egui::TextEdit::singleline(&mut text)
                    .id(id)
                    .frame(false)
                    .margin(egui::Margin::ZERO)
                    .desired_width(field.width())
                    .layouter(&mut layouter),
            )
        })
        .inner;
    if edit.fresh {
        response.request_focus();
        let mut state = egui::TextEdit::load_state(ui.ctx(), id).unwrap_or_default();
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(0),
                egui::text::CCursor::new(count),
            )));
        state.store(ui.ctx(), id);
        return Some(BandRenameOutcome::Typed(edit.text.clone()));
    }
    let (enter, escape) = ui.input(|input| {
        (
            input.key_pressed(egui::Key::Enter),
            input.key_pressed(egui::Key::Escape),
        )
    });
    if escape {
        return Some(BandRenameOutcome::Escape);
    }
    if enter {
        return Some(BandRenameOutcome::Commit(text));
    }
    response.changed().then_some(BandRenameOutcome::Typed(text))
}

/// The system whose DAT id is `id`: how a save names a window's subject.
pub(crate) fn system_by_dat_id(world: &GameWorld, id: DatId) -> Option<SystemKey> {
    world
        .systems
        .iter()
        .find(|(_, system)| system.dat_id == id)
        .map(|(key, _)| key)
}

fn clamp_to_galaxy(position: (i16, i16), layout: CockpitLayout) -> (i16, i16) {
    clamp_window_to_galaxy(position, layout, SYSTEM_WINDOW_WIDTH, SYSTEM_WINDOW_HEIGHT)
}

/// Clamp a window of `width` by `height` logical pixels into the galaxy
/// view (`FUN_0045aac0`, the view's `+0xcc..+0xd8` rect).
#[expect(
    clippy::cast_possible_truncation,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
pub(crate) fn clamp_window_to_galaxy(
    position: (i16, i16),
    layout: CockpitLayout,
    width: f32,
    height: f32,
) -> (i16, i16) {
    let scale = layout.scale.max(f32::EPSILON);
    let min_x = ((layout.galaxy.x - layout.canvas.x) / scale).round();
    let min_y = ((layout.galaxy.y - layout.canvas.y) / scale).round();
    let max_x = (min_x + layout.galaxy.width / scale - width).max(min_x);
    let max_y = (min_y + layout.galaxy.height / scale - height).max(min_y);
    (
        f32::from(position.0).clamp(min_x, max_x).round() as i16,
        f32::from(position.1).clamp(min_y, max_y).round() as i16,
    )
}

fn window_screen_rect(window: OpenSystemWindow, layout: CockpitLayout) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + f32::from(window.logical_position.0) * layout.scale,
            layout.canvas.y + f32::from(window.logical_position.1) * layout.scale,
        ),
        egui::vec2(
            SYSTEM_WINDOW_WIDTH * layout.scale,
            SYSTEM_WINDOW_HEIGHT * layout.scale,
        ),
    )
}

fn rail_slot_rect(faction: CockpitFaction, index: usize) -> (f32, f32, f32, f32) {
    const ALLIANCE_Y: [f32; 12] = [
        61.0, 83.0, 105.0, 127.0, 149.0, 171.0, 193.0, 214.0, 236.0, 258.0, 279.0, 301.0,
    ];
    const EMPIRE_Y: [f32; 12] = [
        48.0, 72.0, 98.0, 122.0, 147.0, 172.0, 196.0, 221.0, 245.0, 270.0, 295.0, 319.0,
    ];
    match faction {
        CockpitFaction::Alliance => (544.0, ALLIANCE_Y[index], 62.0, 18.0),
        CockpitFaction::Empire => (21.0, EMPIRE_Y[index], 54.0, 18.0),
    }
}

fn cockpit_rect(layout: CockpitLayout, logical: (f32, f32, f32, f32)) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + logical.0 * layout.scale,
            layout.canvas.y + logical.1 * layout.scale,
        ),
        egui::vec2(logical.2 * layout.scale, logical.3 * layout.scale),
    )
}

pub(crate) fn logical_rect(
    parent: egui::Rect,
    scale: f32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> egui::Rect {
    egui::Rect::from_min_size(
        logical_point(parent, scale, x, y),
        egui::vec2(width * scale, height * scale),
    )
}

fn logical_point(parent: egui::Rect, scale: f32, x: f32, y: f32) -> egui::Pos2 {
    egui::pos2(parent.min.x + x * scale, parent.min.y + y * scale)
}

pub(crate) fn exact_clicked(response: &egui::Response, rect: egui::Rect) -> bool {
    response.clicked()
        && response
            .interact_pointer_pos()
            .is_some_and(|point| rect_contains(rect, point))
}

pub(crate) fn rect_contains(rect: egui::Rect, point: egui::Pos2) -> bool {
    point.x >= rect.min.x && point.x < rect.max.x && point.y >= rect.min.y && point.y < rect.max.y
}

/// A drag of a movable detail window by its own surface (the title bar or
/// any bare part of it): its area and where in the window, in logical
/// pixels, the pointer took hold.
///
/// The System, Fleet, Defenses and Missions windows set the movable bit
/// (`+0xb8 | 4` in `FUN_00452fc0`, `FUN_004a2630`, `FUN_004a7790` and
/// `FUN_0049f130`); a sector window's base takes policy 0 (`FUN_004591d0` →
/// `FUN_004ac120` → `FUN_00606380`) and stays put, as a title drag left
/// the Dolomar sector window under Wine
/// (alliance-day1169-deathstar-trench-run played on to day 1170). A left press the window
/// passes to its base (`FUN_00606650` → `FUN_00606ee0`) captures the
/// pointer, each move while the button is down sets the window's place at
/// once (`FUN_006071e0`; `FUN_00606380` sets `+0xf4` to 1, the live mode),
/// with no clamp, and the release ends it (`FUN_00607010`). Every press on
/// the window's own surface reaches the base: the Missions (`FUN_0049f4b0`),
/// Fleet (`FUN_004a29c0`) and Defenses (`FUN_004a86d0`) procedures pass it
/// on, and the System procedure (`FUN_004534f0`) does unless it lands on an
/// overview band; presses on child controls (buttons, tabs, lists) go to
/// them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TitleDrag {
    pub area: egui::Id,
    pub grab: egui::Vec2,
}

/// The pointer as a 640 by 480 canvas point.
fn canvas_pointer(layout: CockpitLayout, pointer: egui::Pos2) -> egui::Vec2 {
    egui::vec2(
        (pointer.x - layout.canvas.x) / layout.scale,
        (pointer.y - layout.canvas.y) / layout.scale,
    )
}

/// Where a primary press this frame takes hold of a movable window's own
/// surface ([`TitleDrag`]), in window pixels: it landed on the window's
/// background widget `window`, which lies under every control widget, and
/// on none of `controls`, the child controls whose presses the port tests
/// by rectangle (`x, y, width, height`).
pub(crate) fn surface_drag_grab(
    ctx: &egui::Context,
    window: &egui::Response,
    local: egui::Rect,
    scale: f32,
    controls: &[(f32, f32, f32, f32)],
) -> Option<egui::Vec2> {
    let (pressed, origin) = ctx.input(|input| {
        (
            input.pointer.button_pressed(egui::PointerButton::Primary),
            input.pointer.press_origin(),
        )
    });
    if !pressed || !window.is_pointer_button_down_on() {
        return None;
    }
    let at = (origin? - local.min) / scale;
    let on_control = controls.iter().any(|&(x, y, width, height)| {
        (x..x + width).contains(&at.x) && (y..y + height).contains(&at.y)
    });
    (!on_control).then_some(at)
}

/// A held drag's frame: the window's new place, or `Err(())` once the
/// button is up and the drag is over; `Ok(None)` with no pointer.
pub(crate) fn title_drag_step(
    ctx: &egui::Context,
    layout: CockpitLayout,
    drag: TitleDrag,
) -> Result<Option<(i16, i16)>, ()> {
    let (down, pointer) = ctx.input(|input| {
        (
            input.pointer.button_down(egui::PointerButton::Primary),
            input.pointer.interact_pos(),
        )
    });
    if !down {
        return Err(());
    }
    Ok(pointer.map(|pointer| {
        let at = canvas_pointer(layout, pointer) - drag.grab;
        (at.x.round() as i16, at.y.round() as i16)
    }))
}

fn paint_resource(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    resource_id: u32,
    rect: egui::Rect,
) {
    let Some(texture_id) = cache
        .get(ctx, DllSource::Strategy, resource_id)
        .map(egui_macroquad::egui::TextureHandle::id)
    else {
        return;
    };
    painter.image(
        texture_id,
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
    use crate::fleet_window::tests::PAINTED;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::manufacturing::{BuildableKind, QueueItem};
    use rebellion_core::world::{
        CapitalShipClass, ManufacturingFacilityInstance, ProductionFacilityInstance, Sector, System,
    };
    use rebellion_windowing::{
        SubjectId, WindowCommand, WindowKind, WindowManager, WindowOwner, WindowTransition,
    };

    fn fleet_window_ids(values: &[u64]) -> Vec<WindowId> {
        let mut manager = WindowManager::new();
        values
            .iter()
            .map(|value| {
                let commit = manager
                    .apply(WindowCommand::Open(
                        crate::windowing_policy::contextual_spec(
                            WindowKind::Fleet,
                            SubjectId::new(2, *value),
                            WindowOwner::StrategicRoot,
                        ),
                    ))
                    .unwrap();
                let WindowTransition::Opened { id } = commit.transition else {
                    panic!("expected Fleet open");
                };
                id
            })
            .collect()
    }

    fn fleet_window_id(value: u64) -> WindowId {
        fleet_window_ids(&[value])[0]
    }

    fn layout(faction: CockpitFaction, scale: f32) -> CockpitLayout {
        let (x, width, height) = match faction {
            CockpitFaction::Alliance => (55.0, 485.0, 350.0),
            CockpitFaction::Empire => (120.0, 480.0, 355.0),
        };
        CockpitLayout {
            canvas: CockpitViewport {
                x: 10.0,
                y: 20.0,
                width: 640.0 * scale,
                height: 480.0 * scale,
            },
            galaxy: CockpitViewport {
                x: 10.0 + x * scale,
                y: 20.0 + 40.0 * scale,
                width: width * scale,
                height: height * scale,
            },
            scale,
        }
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn fixture_world(count: usize) -> (GameWorld, Vec<SystemKey>) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(36),
            name: "Sesswenna".into(),
            group: SectorGroup::Core,
            x: 317,
            y: 248,
            systems: Vec::new(),
        });
        let mut systems = Vec::new();
        for index in 0..count {
            let system = world.systems.insert(System {
                dat_id: DatId::new(100 + index as u32),
                name: format!("System {index}"),
                sector,
                x: 320 + index as u16,
                y: 250,
                exploration_status: ExplorationStatus::Explored,
                popularity_alliance: 1.0,
                popularity_empire: 0.0,
                is_populated: true,
                total_energy: 8,
                raw_materials: 6,
                espionage_rating: 0.0,
                fleets: Vec::new(),
                ground_units: Vec::new(),
                special_forces: Vec::new(),
                fighters: Vec::new(),
                defense_facilities: Vec::new(),
                manufacturing_facilities: Vec::new(),
                production_facilities: Vec::new(),
                is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Controlled(Faction::Alliance),
            });
            world.sectors[sector].systems.push(system);
            systems.push(system);
        }
        (world, systems)
    }

    fn add_yard(world: &mut GameWorld, system: SystemKey, class: u32) -> FacilityItem {
        let key = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(class),
                side: rebellion_core::dat::Faction::Alliance,
                is_shipyard: class >> 24 == 0x28,
            });
        world.systems[system].manufacturing_facilities.push(key);
        FacilityItem::Manufacturing(key)
    }

    fn add_mine(world: &mut GameWorld, system: SystemKey) -> FacilityItem {
        let key = world
            .production_facilities
            .insert(ProductionFacilityInstance {
                class_dat_id: DatId::new(0x2c00_0001),
                side: rebellion_core::dat::Faction::Alliance,
                is_mine: true,
            });
        world.systems[system].production_facilities.push(key);
        FacilityItem::Production(key)
    }

    /// A painted text's words, top-left corner, width and font size.
    type RailText = (String, egui::Pos2, f32, f32);

    #[derive(Default)]
    struct Run {
        actions: Vec<SystemWindowAction>,
        texts: Vec<RailText>,
        /// The last frame's filled rectangles and their colours.
        rects: Vec<(egui::Rect, egui::Color32)>,
        /// The last frame's texts and their colours.
        colors: Vec<(String, egui::Color32)>,
        painted: Vec<(u32, egui::Pos2)>,
        /// The last frame's shapes, bottom first.
        shapes: Vec<egui::Shape>,
    }

    /// Draw the windows and rail on the Alliance's layout at scale 2, one
    /// frame per entry of `frames`, the Alliance seeing every system. Keeps
    /// every action and the last frame's texts and bitmaps.
    fn run_with(
        world: &GameWorld,
        manufacturing: &ManufacturingState,
        state: &mut SystemWindowState,
        frames: Vec<Vec<egui::Event>>,
    ) -> Run {
        run_as(
            world,
            manufacturing,
            state,
            frames,
            CockpitFaction::Alliance,
        )
    }

    fn run_as(
        world: &GameWorld,
        manufacturing: &ManufacturingState,
        state: &mut SystemWindowState,
        frames: Vec<Vec<egui::Event>>,
        faction: CockpitFaction,
    ) -> Run {
        run_cached(
            world,
            manufacturing,
            state,
            frames,
            faction,
            BmpCache::new(),
            &mut |_, _| {},
        )
    }

    fn run_cached(
        world: &GameWorld,
        manufacturing: &ManufacturingState,
        state: &mut SystemWindowState,
        frames: Vec<Vec<egui::Event>>,
        faction: CockpitFaction,
        mut cache: BmpCache,
        before: &mut dyn FnMut(usize, &mut SystemWindowState),
    ) -> Run {
        let layout = layout(faction, 2.0);
        let ctx = egui::Context::default();
        let fog = FogState::new(Faction::Alliance);
        let mut run = Run::default();
        for (index, events) in frames.into_iter().enumerate() {
            before(index, state);
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1000.0),
                )),
                events,
                ..Default::default()
            };
            PAINTED.with(|painted| painted.borrow_mut().clear());
            let output = ctx.run(input, |ctx| {
                run.actions.extend(draw_system_windows(
                    ctx,
                    world,
                    &rebellion_core::movement::MovementState::default(),
                    &fog,
                    &MissionState::new(),
                    manufacturing,
                    &DeliveryState::new(),
                    state,
                    faction,
                    layout,
                    &mut cache,
                    true,
                    &|_| true,
                ));
            });
            run.painted = PAINTED.with(|painted| painted.take());
            run.shapes = output
                .shapes
                .iter()
                .map(|clipped| clipped.shape.clone())
                .collect();
            run.rects = output
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Rect(rect) => Some((rect.rect, rect.fill)),
                    _ => None,
                })
                .collect();
            run.colors = output
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) => Some((
                        text.galley.text().to_owned(),
                        text.galley.job.sections[0].format.color,
                    )),
                    _ => None,
                })
                .collect();
            run.texts = output
                .shapes
                .into_iter()
                .filter_map(|clipped| match clipped.shape {
                    egui::Shape::Text(text) => Some((
                        text.galley.text().to_owned(),
                        text.pos,
                        text.galley.size().x,
                        text.galley.job.sections[0].format.font_id.size,
                    )),
                    _ => None,
                })
                .collect();
        }
        run
    }

    fn run_rail(
        world: &GameWorld,
        state: &mut SystemWindowState,
        frames: Vec<Vec<egui::Event>>,
    ) -> (Vec<SystemWindowAction>, Vec<RailText>) {
        let run = run_with(world, &ManufacturingState::new(), state, frames);
        // The rail tests read the last frame's bitmaps from the record.
        PAINTED.with(|painted| *painted.borrow_mut() = run.painted);
        (run.actions, run.texts)
    }

    const ORIGIN: (i16, i16) = (100, 60);

    /// A logical point in the window opened at `ORIGIN` on the scale 2
    /// layout.
    fn at(x: f32, y: f32) -> egui::Pos2 {
        egui::pos2(
            10.0 + (f32::from(ORIGIN.0) + x) * 2.0,
            20.0 + (f32::from(ORIGIN.1) + y) * 2.0,
        )
    }

    fn opened(world: &GameWorld, system: SystemKey) -> SystemWindowState {
        let mut state = SystemWindowState::default();
        assert!(state.open(
            world,
            system,
            ORIGIN,
            CockpitFaction::Alliance,
            layout(CockpitFaction::Alliance, 2.0)
        ));
        state
    }

    fn hover(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
        vec![
            vec![egui::Event::PointerMoved(point)],
            vec![egui::Event::PointerMoved(point)],
        ]
    }

    fn click_with(point: egui::Pos2, button: egui::PointerButton) -> Vec<Vec<egui::Event>> {
        let event = |pressed| egui::Event::PointerButton {
            pos: point,
            button,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let mut frames = hover(point);
        frames.extend([vec![event(true)], vec![event(false)], vec![]]);
        frames
    }

    fn click(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
        click_with(point, egui::PointerButton::Primary)
    }

    fn right_click(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
        click_with(point, egui::PointerButton::Secondary)
    }

    fn text<'a>(run: &'a Run, words: &str) -> &'a RailText {
        run.texts
            .iter()
            .find(|value| value.0 == words)
            .unwrap_or_else(|| panic!("no {words:?} in {:?}", run.texts))
    }

    fn key(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }
    }

    /// `system`'s window with the ships band's name edit open on "Death
    /// Star 1".
    fn renaming(world: &GameWorld, system: SystemKey) -> SystemWindowState {
        let mut state = opened(world, system);
        assert!(state.begin_band_rename(system, ProductionArea::Shipyard, "Death Star 1"));
        state
    }

    /// Two frames for the edit to appear and take the focus, then `frames`.
    fn after_opening(frames: Vec<Vec<egui::Event>>) -> Vec<Vec<egui::Event>> {
        let mut all = vec![vec![], vec![]];
        all.extend(frames);
        all
    }

    fn renamed(actions: &[SystemWindowAction]) -> Vec<(ProductionArea, &str)> {
        actions
            .iter()
            .filter_map(|action| match action {
                SystemWindowAction::RenameProduct { area, name, .. } => {
                    Some((*area, name.as_str()))
                }
                _ => None,
            })
            .collect()
    }

    #[test]
    fn the_band_rename_opens_over_the_product_name_with_the_window_focused() {
        // FUN_00458b50 → FUN_00458640: the edit lies over the band's name
        // line (band (5, 16)), the band selected and the window focused.
        let (world, systems) = fixture_world(2);
        let mut state = opened(&world, systems[0]);
        let layout = layout(CockpitFaction::Alliance, 2.0);
        state.open(&world, systems[1], ORIGIN, CockpitFaction::Alliance, layout);
        assert!(!state.begin_band_rename(SystemKey::default(), ProductionArea::Shipyard, "X"));
        assert!(state.begin_band_rename(systems[0], ProductionArea::Shipyard, "Death Star 1"));
        assert!(state.band_rename_open());
        assert_eq!(state.windows.last().unwrap().system, systems[0]);
        assert_eq!(
            state.windows.last().unwrap().selected_band,
            Some(ProductionArea::Shipyard)
        );

        let run = run_with(
            &world,
            &ManufacturingState::new(),
            &mut state,
            after_opening(Vec::new()),
        );
        let (x, y, _, _) = BAND_RECTS[0];
        let field = text(&run, "Death Star 1");
        assert!(
            (field.1 - at(x + BAND_STATUS.0, y + BAND_STATUS.1)).length() < 4.0,
            "{field:?}"
        );
        // The other window's ships band still shows its line.
        assert_eq!(
            run.texts
                .iter()
                .filter(|text| text.0 == "No Ships are being built")
                .count(),
            1
        );
    }

    #[test]
    fn enter_applies_a_typed_name_to_the_bands_product() {
        // FUN_00458780: Enter (0x407) with a changed, non-empty name sets the
        // pending Rename's name and submits it. Under Wine "Tarkin" typed
        // over the selected "Death Star 1" renamed the unfinished ship
        // (route-1 Empire save, day 123 played on).
        let (world, systems) = fixture_world(1);
        let mut state = renaming(&world, systems[0]);
        let run = run_with(
            &world,
            &ManufacturingState::new(),
            &mut state,
            after_opening(vec![
                vec![egui::Event::Text("Tarkin".into())],
                vec![key(egui::Key::Enter)],
            ]),
        );
        assert_eq!(
            renamed(&run.actions),
            [(ProductionArea::Shipyard, "Tarkin")]
        );
        assert!(!state.band_rename_open());
        assert_eq!(state.windows.len(), 1);
    }

    /// The field where the band's name edit lies, at scale 2: the name's
    /// width in the band's font plus 2, 13 high, at the name line.
    fn band_field(name: &str) -> egui::Rect {
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |_| {});
        let width = ctx.fonts(|fonts| {
            fonts
                .layout_no_wrap(
                    name.to_owned(),
                    egui::FontId::proportional(OVERVIEW_FONT_EM * 2.0),
                    egui::Color32::WHITE,
                )
                .size()
                .x
        });
        let (x, y, _, _) = BAND_RECTS[0];
        egui::Rect::from_min_size(
            at(x + BAND_STATUS.0, y + BAND_STATUS.1),
            egui::vec2(width + 4.0, BAND_RENAME_HEIGHT * 2.0),
        )
    }

    #[test]
    fn the_overview_bands_take_font_10_at_the_originals_size() {
        // FUN_00458080 gives every band label font 10 (FUN_00601c60, lines
        // 77, 91, 106 and 121): FUN_0060eed0 case 10, Arial with a 14-pixel
        // cell at weight 400. The title (line 76-78) keeps the label's
        // default colour, which shows black; the other lines are white
        // (FUN_00601c90 0x2ffffff). hyp (measured): under Wine the idle
        // lines' ink is 98 ("No Troops in training") and 126 ("No Facilities
        // are being built") pixels wide, Arial's hinted advances at an
        // 11-pixel em (route-1 Empire save, day 123 played on).
        let (world, systems) = fixture_world(1);
        let mut state = opened(&world, systems[0]);
        let run = run_with(
            &world,
            &ManufacturingState::new(),
            &mut state,
            after_opening(Vec::new()),
        );
        for (line, original) in [
            ("No Troops in training", 98.0),
            ("No Facilities are being built", 126.0),
        ] {
            let width = text(&run, line).2 / 2.0;
            assert!(
                (width - original).abs() <= 4.0,
                "{line:?} is {width} wide, the original {original}"
            );
        }
        let color = |words: &str| {
            run.colors
                .iter()
                .find(|(text, _)| text == words)
                .map(|(_, color)| *color)
        };
        assert_eq!(color("Ship Construction"), Some(egui::Color32::BLACK));
        assert_eq!(color("No Troops in training"), Some(egui::Color32::WHITE));
    }

    #[test]
    fn the_whole_selected_name_fills_the_field_white_until_typing() {
        // FUN_00458640 selects the whole name: under Wine "Death Star 1"
        // showed black on a white box over the line, the label's rect two
        // pixels wider; the typed "Tarkin" showed white on the band
        // (route-1 Empire save, day 123 played on).
        let (world, systems) = fixture_world(1);
        let mut state = renaming(&world, systems[0]);
        let field = band_field("Death Star 1");
        let white = |run: &Run| {
            // White, faded in with its new window (premultiplied: every
            // channel equals alpha).
            run.rects.iter().any(|(rect, fill)| {
                fill.a() > 0
                    && [fill.r(), fill.g(), fill.b()] == [fill.a(); 3]
                    && (rect.min - field.min).length() < 0.01
                    && (rect.size() - field.size()).length() < 0.01
            })
        };
        let opened = run_with(
            &world,
            &ManufacturingState::new(),
            &mut state,
            after_opening(Vec::new()),
        );
        assert!(white(&opened), "{:?} not in {:?}", field, opened.rects);
        let typed = run_with(
            &world,
            &ManufacturingState::new(),
            &mut state,
            vec![vec![egui::Event::Text("Tarkin".into())], vec![]],
        );
        assert!(!white(&typed));
    }

    #[test]
    fn an_empty_name_opens_with_no_white_box() {
        // Selecting the whole of an empty name selects nothing, so nothing
        // shows selected. The edit selects the name on the frame it opens
        // in, over a window already drawn, and keeps it after; a whole name
        // shows its white box on both frames, an empty one on neither.
        let (world, systems) = fixture_world(1);
        let boxed = |name: &str, frames: usize| {
            let mut state = opened(&world, systems[0]);
            let run = run_cached(
                &world,
                &ManufacturingState::new(),
                &mut state,
                vec![Vec::new(); frames],
                CockpitFaction::Alliance,
                BmpCache::new(),
                &mut |index, state| {
                    if index == 2 {
                        assert!(state.begin_band_rename(
                            systems[0],
                            ProductionArea::Shipyard,
                            name
                        ));
                    }
                },
            );
            let field = band_field(name);
            run.rects.iter().any(|(rect, fill)| {
                fill.a() > 0
                    && [fill.r(), fill.g(), fill.b()] == [fill.a(); 3]
                    && (rect.min - field.min).length() < 0.01
            })
        };
        for frames in [3, 4] {
            assert!(boxed("Death Star 1", frames), "{frames}");
            assert!(!boxed("", frames), "{frames}");
        }
    }

    #[test]
    fn an_unchanged_or_emptied_name_applies_nothing() {
        // FUN_00458780.c:33-37: an unchanged or empty name drops the order.
        let (world, systems) = fixture_world(1);
        for (frames, typed) in [
            (Vec::new(), "Death Star 1"),
            (vec![vec![key(egui::Key::Backspace)]], ""),
        ] {
            let mut state = renaming(&world, systems[0]);
            run_with(
                &world,
                &ManufacturingState::new(),
                &mut state,
                after_opening(frames),
            );
            assert_eq!(
                state.band_rename.as_ref().map(|edit| edit.text.as_str()),
                Some(typed)
            );
            let run = run_with(
                &world,
                &ManufacturingState::new(),
                &mut state,
                vec![vec![key(egui::Key::Enter)]],
            );
            assert_eq!(renamed(&run.actions), []);
            assert!(!state.band_rename_open());
        }
    }

    #[test]
    fn escape_in_the_band_rename_closes_the_window_unrenamed() {
        // The field passes Escape to the window (FUN_00605160), whose
        // FUN_00458980 closes it. Under Wine the Coruscant System window
        // closed with the name unchanged (route-1 Empire save, day 127
        // played on to day 140).
        let (world, systems) = fixture_world(1);
        let mut state = renaming(&world, systems[0]);
        let run = run_with(
            &world,
            &ManufacturingState::new(),
            &mut state,
            after_opening(vec![
                vec![egui::Event::Text("Tarkin".into())],
                vec![key(egui::Key::Escape)],
            ]),
        );
        assert_eq!(renamed(&run.actions), []);
        assert!(state.windows.is_empty());
        assert!(!state.band_rename_open());
    }

    #[test]
    fn a_press_elsewhere_in_the_window_applies_the_typed_name() {
        // FUN_004534f0: a press (0x201, 0x204) in the window ends the edit
        // through FUN_00458780, as Enter does.
        let (world, systems) = fixture_world(1);
        let mut state = renaming(&world, systems[0]);
        let mut frames = vec![vec![egui::Event::Text("Tarkin".into())]];
        frames.extend(click(at(20.0, 100.0)));
        let run = run_with(
            &world,
            &ManufacturingState::new(),
            &mut state,
            after_opening(frames),
        );
        assert_eq!(
            renamed(&run.actions),
            [(ProductionArea::Shipyard, "Tarkin")]
        );
        assert!(!state.band_rename_open());
    }

    #[test]
    fn closing_or_minimizing_the_window_drops_its_band_rename() {
        let (world, systems) = fixture_world(1);
        let mut state = renaming(&world, systems[0]);
        assert!(state.close_window(systems[0]));
        assert!(!state.band_rename_open());
        let mut state = renaming(&world, systems[0]);
        assert!(state.minimize(systems[0]));
        assert!(!state.band_rename_open());
    }

    #[test]
    fn opening_clamps_to_faction_galaxy_and_deduplicates() {
        // FUN_0045aac0 clamps the 226 by 304 window into the galaxy view:
        // 55 + 485 - 226 and 40 + 350 - 304.
        let (world, systems) = fixture_world(1);
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Alliance, 2.0);
        assert!(state.open(
            &world,
            systems[0],
            (620, 470),
            CockpitFaction::Alliance,
            layout
        ));
        assert_eq!(state.windows[0].logical_position, (314, 86));
        assert_eq!(state.windows[0].tab, SystemWindowTab::Overview);
        assert!(state.open(&world, systems[0], (0, 0), CockpitFaction::Alliance, layout));
        assert_eq!(state.window_count(), 1);
    }

    #[test]
    fn a_keyed_close_closes_that_window_and_leaves_the_rest_for_either_side() {
        // FUN_004ac3a0: WM_KEYDOWN 0x1b calls slot +0x30 on the focused window.
        let (world, systems) = fixture_world(2);
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut state = SystemWindowState::default();
            let layout = layout(faction, 1.0);
            for system in &systems {
                assert!(state.open(&world, *system, (100, 50), faction, layout));
            }
            assert!(state.close_window(systems[1]));
            let open: Vec<_> = state.windows.iter().map(|window| window.system).collect();
            assert_eq!(open, [systems[0]], "{faction:?}");
            assert!(state.close_window(systems[0]));
            assert!(!state.any_open());
            assert!(!state.close_window(systems[0]));
        }
    }

    #[test]
    fn after_an_escape_the_window_behind_ranks_where_it_took_focus_for_either_side() {
        // The keyed close posts 0x467(0,0), which activates the topmost
        // remaining child in z-order (FUN_00422ce0); a window of another
        // kind focused between this kind's two stays above the one behind.
        let (world, systems) = fixture_world(2);
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut state = SystemWindowState::default();
            let layout = layout(faction, 1.0);
            assert!(state.open(&world, systems[0], (100, 50), faction, layout));
            let between = crate::window_focus::next_serial();
            assert!(state.open(&world, systems[1], (100, 50), faction, layout));
            assert!(state.focus_serial() > between, "{faction:?}");
            assert!(state.close_window(systems[1]));
            assert!(
                state.focus_serial() < between,
                "{faction:?}: the window behind keeps the place it took focus at"
            );
        }
    }

    #[test]
    fn a_saved_system_window_comes_back_in_place_on_the_overview() {
        // FUN_00458ea0 saves the system and no page; FUN_00441190 rebuilds
        // type 9 at the saved x and y. Under Wine a System window switched
        // to Shipyards and dragged to (322, 92) reopened there on the
        // overview (Empire day-2 start, played on to day 6).
        let (world, systems) = fixture_world(2);
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut state = SystemWindowState::default();
            let layout = layout(faction, 1.0);
            state.open_tab(
                &world,
                systems[0],
                SystemWindowTab::Shipyards,
                (322, 92),
                faction,
                layout,
            );
            state.open(&world, systems[1], (150, 80), faction, layout);
            let mut saved = state.saved_windows(&world);
            saved.sort_by_key(|(z, _)| *z);
            let placed: Vec<_> = state
                .windows
                .iter()
                .map(|window| (window.system, window.logical_position))
                .collect();

            let mut restored = SystemWindowState::default();
            let floor = crate::window_focus::next_serial();
            for (_, window) in &saved {
                assert!(
                    restored.restore_saved(&world, faction, window),
                    "{faction:?}"
                );
            }

            let back: Vec<_> = restored
                .windows
                .iter()
                .map(|window| (window.system, window.logical_position))
                .collect();
            assert_eq!(back, placed, "{faction:?}");
            assert!(restored.windows.iter().all(
                |window| window.tab == SystemWindowTab::Overview && window.focus_serial > floor
            ));
            assert_eq!(restored.faction, faction);
        }
    }

    #[test]
    fn a_saved_minimized_window_comes_back_on_the_rail_in_order() {
        // Each rail window saves hidden (+0x110 is 0 there) at its place in
        // the z-order (FUN_00440ea0.c:19-24), which it keeps from before it
        // was minimized; FUN_00441190 appends each hidden window back to the
        // rail as it meets it (FUN_00428b40). Under Wine a minimized System
        // window came back on the rail (Empire day-2 start, played on to
        // day 6).
        let (world, systems) = fixture_world(3);
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut state = SystemWindowState::default();
            let layout = layout(faction, 1.0);
            state.open(&world, systems[0], (120, 70), faction, layout);
            assert!(state.minimize(systems[0]));
            let opened_first = state.windows.is_empty();
            assert!(opened_first);
            state.minimize_defenses_window_at(systems[1], (130, 75), 7);
            state.minimize_missions_window_at(systems[2], (140, 85), 3);
            let mut saved = state.saved_rail(&world, |_| None);
            assert!(saved.iter().all(|(_, window)| !window.shown));
            let serials: Vec<u64> = saved.iter().map(|(serial, _)| *serial).collect();
            assert_eq!(serials[1..], [7, 3], "{faction:?}");
            saved.sort_by_key(|(serial, _)| *serial);

            let mut restored = SystemWindowState::default();
            for (_, window) in &saved {
                assert!(
                    restored.restore_saved(&world, faction, window),
                    "{faction:?}"
                );
            }

            assert!(restored.windows.is_empty(), "{faction:?}");
            let back: Vec<_> = restored
                .saved_rail(&world, |_| None)
                .into_iter()
                .map(|(_, window)| window)
                .collect();
            let order: Vec<_> = saved.iter().map(|(_, window)| *window).collect();
            assert_eq!(back, order, "{faction:?}");
        }
    }

    #[test]
    fn a_loads_hidden_window_takes_the_next_place_in_the_z_order() {
        // The save lists the windows bottom to top (FUN_00440ea0.c:19-24)
        // and FUN_00441190 makes them in that order, hidden ones included,
        // so a hidden System, Defenses or Missions window takes the next
        // place when its turn comes.
        let (world, systems) = fixture_world(3);
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Alliance, 1.0);
        state.open(
            &world,
            systems[0],
            (120, 70),
            CockpitFaction::Alliance,
            layout,
        );
        assert!(state.minimize(systems[0]));
        // Both below the System window, which took a fresh serial.
        state.minimize_defenses_window_at(systems[1], (130, 75), 0);
        state.minimize_missions_window_at(systems[2], (140, 85), 0);
        let top = |state: &SystemWindowState| {
            state
                .saved_rail(&world, |_| None)
                .into_iter()
                .max_by_key(|(serial, _)| *serial)
                .map(|(_, window)| window.kind)
        };
        let dat = |index: usize| world.systems[systems[index]].dat_id;

        // Only the window the child names lifts.
        assert!(!state.lift_hidden(GalaxyChild::Defenses(systems[2])));
        assert!(!state.lift_hidden(GalaxyChild::Missions(systems[1])));
        assert!(!state.lift_hidden(GalaxyChild::System(systems[1])));
        assert_eq!(
            top(&state),
            Some(SavedWindowKind::System { system: dat(0) })
        );

        assert!(state.lift_hidden(GalaxyChild::Missions(systems[2])));
        assert_eq!(
            top(&state),
            Some(SavedWindowKind::Missions { system: dat(2) })
        );
        assert!(state.lift_hidden(GalaxyChild::Defenses(systems[1])));
        assert_eq!(
            top(&state),
            Some(SavedWindowKind::Defenses { system: dat(1) })
        );
        assert!(state.lift_hidden(GalaxyChild::System(systems[0])));
        assert_eq!(
            top(&state),
            Some(SavedWindowKind::System { system: dat(0) })
        );
    }

    #[test]
    fn a_loads_hidden_fleet_window_takes_its_saved_rail_slot() {
        // FUN_00441190 appends each hidden window to the rail as it meets
        // it (FUN_00428b40); the port commits Fleet windows last, so one
        // goes back into its slot among the others.
        let (world, systems) = fixture_world(3);
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Empire, 1.0);
        state.open(
            &world,
            systems[0],
            (120, 70),
            CockpitFaction::Empire,
            layout,
        );
        assert!(state.minimize(systems[0]));
        state.minimize_defenses_window_at(systems[1], (130, 75), 7);
        let fleets = fleet_window_ids(&[301]);
        state.insert_fleet_rail(1, fleets[0], systems[2], (150, 90));
        assert_eq!(
            state.rail_entries().collect::<Vec<_>>(),
            [
                (systems[0], "system"),
                (systems[2], "fleet"),
                (systems[1], "defenses")
            ]
        );
        // The same window again moves rather than doubles.
        state.insert_fleet_rail(0, fleets[0], systems[2], (150, 90));
        assert_eq!(
            state.rail_entries().collect::<Vec<_>>(),
            [
                (systems[2], "fleet"),
                (systems[0], "system"),
                (systems[1], "defenses")
            ]
        );
    }

    #[test]
    fn a_saved_window_names_its_system_by_dat_id() {
        let (world, systems) = fixture_world(1);
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Empire, 1.0);
        state.open(
            &world,
            systems[0],
            (150, 80),
            CockpitFaction::Empire,
            layout,
        );
        assert_eq!(
            state.saved_windows(&world)[0].1,
            SavedWindow {
                kind: SavedWindowKind::System {
                    system: DatId::new(100),
                },
                position: (150, 80),
                shown: true,
            }
        );
        let fleet = fleet_window_id(7);
        state.minimize_fleet_window(fleet, systems[0], (160, 90));
        assert_eq!(state.saved_rail(&world, |_| None), []);
        assert_eq!(
            state.saved_rail(&world, |id| (id == fleet).then_some((0x68, 5))),
            [(
                5,
                SavedWindow {
                    kind: SavedWindowKind::Fleet {
                        system: DatId::new(100),
                        tab: 0x68,
                    },
                    position: (160, 90),
                    shown: false,
                }
            )]
        );
        let elsewhere = SavedWindow {
            kind: SavedWindowKind::System {
                system: DatId::new(0x9000_ffff),
            },
            position: (150, 80),
            shown: true,
        };
        let mut other = SystemWindowState::default();
        assert!(!other.restore_saved(&world, CockpitFaction::Empire, &elsewhere));
        let shown_defenses = SavedWindow {
            kind: SavedWindowKind::Defenses {
                system: DatId::new(100),
            },
            ..elsewhere
        };
        assert!(!other.restore_saved(&world, CockpitFaction::Empire, &shown_defenses));
        let shown_missions = SavedWindow {
            kind: SavedWindowKind::Missions {
                system: DatId::new(100),
            },
            ..elsewhere
        };
        assert!(!other.restore_saved(&world, CockpitFaction::Empire, &shown_missions));
        assert!(other.windows.is_empty() && other.rail.is_empty());
    }

    #[test]
    fn a_restored_system_window_replaces_its_own_rail_entry() {
        // One window per system (FUN_0045aac0): a System window restored
        // while its system is on the rail leaves one copy, the restored one.
        let (world, systems) = fixture_world(2);
        let layout = layout(CockpitFaction::Alliance, 1.0);
        let mut state = SystemWindowState::default();
        state.open(
            &world,
            systems[0],
            (120, 70),
            CockpitFaction::Alliance,
            layout,
        );
        assert!(state.minimize(systems[0]));
        state.minimize_defenses_window(systems[0], (130, 75));
        state.open(
            &world,
            systems[1],
            (140, 80),
            CockpitFaction::Alliance,
            layout,
        );
        assert!(state.minimize(systems[1]));
        let shown = SavedWindow {
            kind: SavedWindowKind::System {
                system: DatId::new(100),
            },
            position: (150, 90),
            shown: true,
        };
        assert!(state.restore_saved(&world, CockpitFaction::Alliance, &shown));
        assert_eq!(state.windows.len(), 1);
        assert_eq!(state.windows[0].logical_position, (150, 90));
        let rail: Vec<_> = state
            .saved_rail(&world, |_| None)
            .into_iter()
            .map(|(_, window)| window.kind)
            .collect();
        assert_eq!(
            rail,
            [
                SavedWindowKind::Defenses {
                    system: DatId::new(100),
                },
                SavedWindowKind::System {
                    system: DatId::new(101),
                },
            ]
        );
    }

    #[test]
    fn minimize_caps_at_twelve_and_evicts_oldest() {
        let (world, systems) = fixture_world(13);
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Alliance, 1.0);
        for system in &systems {
            state.open(&world, *system, (100, 50), CockpitFaction::Alliance, layout);
            assert!(state.minimize(*system));
        }
        assert_eq!(state.rail_count(), REFERENCE_RAIL_SLOTS);
        assert_eq!(state.rail[0].system(), systems[1]);
        assert_eq!(state.rail[11].system(), systems[12]);
    }

    #[test]
    fn mixed_rail_overflow_returns_the_exact_managed_fleet_victim() {
        let (_, systems) = fixture_world(13);
        let mut manager = WindowManager::new();
        let mut fleet_ids = Vec::new();
        for index in 0..2 {
            let commit = manager
                .apply(WindowCommand::Open(
                    crate::windowing_policy::contextual_spec(
                        WindowKind::Fleet,
                        SubjectId::new(2, 10_000 + index),
                        WindowOwner::StrategicRoot,
                    ),
                ))
                .unwrap();
            let WindowTransition::Opened { id } = commit.transition else {
                panic!("expected Fleet open");
            };
            fleet_ids.push(id);
        }

        let mut state = SystemWindowState::default();
        assert_eq!(
            state.minimize_fleet_window(fleet_ids[0], systems[0], (100, 80)),
            None
        );
        for system in &systems[1..12] {
            assert_eq!(state.minimize_defenses_window(*system, (90, 70)), None);
        }
        assert_eq!(state.rail_count(), REFERENCE_RAIL_SLOTS);
        assert_eq!(
            state.minimize_fleet_window(fleet_ids[1], systems[12], (120, 90)),
            Some(fleet_ids[0]),
        );
        assert_eq!(state.rail_count(), REFERENCE_RAIL_SLOTS);
        assert!(state.restore_fleet_window(fleet_ids[1]));
        assert!(!state.restore_fleet_window(fleet_ids[0]));
    }

    #[test]
    fn legacy_rail_overflow_returns_the_exact_managed_fleet_victim() {
        let (_, systems) = fixture_world(13);
        let fleet_id = fleet_window_id(10_100);
        let mut state = SystemWindowState::default();
        assert_eq!(
            state.minimize_fleet_window(fleet_id, systems[0], (100, 80)),
            None,
        );
        for system in &systems[1..12] {
            assert_eq!(state.minimize_defenses_window(*system, (90, 70)), None);
        }

        assert_eq!(state.rail_count(), REFERENCE_RAIL_SLOTS);
        assert_eq!(
            state.minimize_missions_window(systems[12], (60, 40)),
            Some(fleet_id),
        );
        assert_eq!(state.rail_count(), REFERENCE_RAIL_SLOTS);
        assert!(!state.restore_fleet_window(fleet_id));
    }

    #[test]
    fn restore_preserves_position_tab_and_selection() {
        let (mut world, systems) = fixture_world(1);
        let mine = add_mine(&mut world, systems[0]);
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Alliance, 1.0);
        state.open(
            &world,
            systems[0],
            (100, 60),
            CockpitFaction::Alliance,
            layout,
        );
        state.select_tab(systems[0], SystemWindowTab::Mines);
        state.select_item(systems[0], Some(mine));
        state.minimize(systems[0]);
        assert!(state.restore(systems[0]));
        assert_eq!(state.window_count(), 1);
        assert_eq!(state.rail_count(), 0);
        assert_eq!(state.windows[0].logical_position, (100, 60));
        assert_eq!(state.windows[0].tab, SystemWindowTab::Mines);
        assert_eq!(state.windows[0].selected_item, Some(mine));

        // The same page keeps the selection; another clears it.
        state.select_tab(systems[0], SystemWindowTab::Mines);
        assert_eq!(state.windows[0].selected_item, Some(mine));
        state.select_tab(systems[0], SystemWindowTab::Refineries);
        assert_eq!(state.windows[0].selected_item, None);
    }

    #[test]
    fn faction_change_clears_visible_and_minimized_windows() {
        let (world, systems) = fixture_world(2);
        let mut state = SystemWindowState::default();
        let alliance = layout(CockpitFaction::Alliance, 1.0);
        state.open(
            &world,
            systems[0],
            (100, 60),
            CockpitFaction::Alliance,
            alliance,
        );
        state.open(
            &world,
            systems[1],
            (100, 60),
            CockpitFaction::Alliance,
            alliance,
        );
        state.minimize(systems[0]);
        state.prepare_faction(CockpitFaction::Empire);
        assert_eq!(state.window_count(), 0);
        assert_eq!(state.rail_count(), 0);
    }

    #[test]
    fn each_tab_shows_its_side_art_pressed_or_empty() {
        // FUN_00455060: the overview's art by side; a page's normal and
        // pressed art, its empty art in place of the normal one.
        assert_eq!(
            [1, 2, 3, 0].map(|side| tab_resource(SystemWindowTab::Overview, side, false, true)),
            [10_312, 10_315, 10_318, 10_318]
        );
        assert_eq!(
            [1, 2, 3].map(|side| tab_resource(SystemWindowTab::Overview, side, true, false)),
            [10_311, 10_314, 10_317]
        );
        let pages = [
            (SystemWindowTab::Shipyards, [10_327, 10_326, 10_328]),
            (
                SystemWindowTab::TrainingFacilities,
                [10_330, 10_329, 10_331],
            ),
            (SystemWindowTab::ConstructionYards, [10_333, 10_332, 10_334]),
            (SystemWindowTab::Refineries, [10_324, 10_323, 10_325]),
            (SystemWindowTab::Mines, [10_321, 10_320, 10_322]),
        ];
        for (tab, [normal, pressed, empty]) in pages {
            assert_eq!(tab_resource(tab, 1, false, false), normal, "{tab:?}");
            assert_eq!(tab_resource(tab, 2, true, true), pressed, "{tab:?}");
            assert_eq!(tab_resource(tab, 3, false, true), empty, "{tab:?}");
        }
        // The help messages (TEXTSTRA 6197, 6184, 6192, 6194, 6183, 6182)
        // and the strip's x positions.
        assert_eq!(
            SystemWindowTab::ALL.map(SystemWindowTab::name),
            [
                "Manufacturing",
                "Shipyards",
                "Training Facilities",
                "Construction Yards",
                "Refineries",
                "Mines"
            ]
        );
        assert_eq!(
            SystemWindowTab::ALL.map(SystemWindowTab::x),
            [0.0, 39.0, 77.0, 115.0, 152.0, 190.0]
        );
    }

    #[test]
    fn maps_original_system_item_resource_families_without_fallbacks() {
        // Source: GOKRES.DLL mini-icon resource blocks and DAT record families.
        assert_eq!(
            character_mini_resource_id(DatId::new(576), true),
            Some(18_496)
        );
        assert_eq!(
            character_mini_resource_id(DatId::new(641), true),
            Some(18_561)
        );
        assert_eq!(
            character_mini_resource_id(DatId::new(832), false),
            Some(19_008)
        );
        assert_eq!(
            character_mini_resource_id(DatId::new(923), false),
            Some(19_099)
        );

        assert_eq!(
            manufacturing_facility_mini(DatId::new(0x2800_0001)),
            Some((16_640, "Orbital Shipyard"))
        );
        assert_eq!(
            manufacturing_facility_mini(DatId::new(0x2a00_0006)),
            Some((16_645, "Advanced Construction Yard"))
        );
        assert_eq!(
            manufacturing_facility_mini(DatId::new(0x2000_0001)),
            Some((17_216, "Alliance Headquarters"))
        );
        assert_eq!(
            defense_facility_mini(DatId::new(0x2400_0003)),
            Some((16_898, "GenCore Level I"))
        );
        assert_eq!(
            defense_facility_mini(DatId::new(0x2500_0004)),
            Some((17_024, "Death Star Shield"))
        );
        assert_eq!(
            production_facility_mini(DatId::new(0x2c00_0001)),
            Some((16_385, "Mine"))
        );
        assert_eq!(
            production_facility_mini(DatId::new(0x2d00_0002)),
            Some((16_386, "Refinery"))
        );
        assert_eq!(
            troop_mini(DatId::new(0x1000_0001)),
            Some((17_472, "Alliance Fleet Regiment"))
        );
        assert_eq!(
            troop_mini(DatId::new(0x1000_000a)),
            Some((17_540, "Dark Trooper Regiment"))
        );
        assert_eq!(
            special_force_mini(DatId::new(0x3c00_0001)),
            Some((17_728, "Guerrillas"))
        );
        assert_eq!(
            special_force_mini(DatId::new(0x3c00_0009)),
            Some((17_796, "Bounty Hunters"))
        );

        assert_eq!(character_mini_resource_id(DatId::new(575), true), None);
        assert_eq!(manufacturing_facility_mini(DatId::new(0x2700_0001)), None);
        assert_eq!(defense_facility_mini(DatId::new(0x2400_0007)), None);
        assert_eq!(production_facility_mini(DatId::new(0x2d00_0003)), None);
        assert_eq!(troop_mini(DatId::new(0x1000_000b)), None);
        assert_eq!(special_force_mini(DatId::new(0x3c00_000a)), None);
    }

    #[test]
    fn the_window_paints_its_chrome_tabs_and_the_overview_first() {
        // FUN_00455060 / FUN_00456230: 10297 at (0, 0), the focused side 1
        // strip at (2, 2), the title buttons at 3, 195 and 209, the six tabs
        // at (x, 20), the yard column at (6, 71), and each band's frame and
        // unselected side 1 strip at its corner (FUN_00458080).
        let (mut world, systems) = fixture_world(1);
        add_yard(&mut world, systems[0], 0x2800_0001);
        let mut state = opened(&world, systems[0]);
        let run = run_with(
            &world,
            &ManufacturingState::new(),
            &mut state,
            hover(at(150.0, 10.0)),
        );
        for (id, point) in [
            (WINDOW_BACKGROUND, at(0.0, 0.0)),
            (10_299, at(2.0, 2.0)),
            (SECTOR_NORMAL, at(3.0, 3.0)),
            (MINIMIZE_NORMAL, at(195.0, 3.0)),
            (CLOSE_NORMAL, at(209.0, 3.0)),
            (10_311, at(0.0, 20.0)),
            (10_327, at(39.0, 20.0)),
            (10_331, at(77.0, 20.0)),
            (10_334, at(115.0, 20.0)),
            (10_325, at(152.0, 20.0)),
            (10_322, at(190.0, 20.0)),
            (10_298, at(6.0, 71.0)),
            (10_290, at(55.0, 57.0)),
            (10_292, at(55.0, 57.0)),
            (10_292, at(55.0, 138.0)),
            (10_292, at(55.0, 219.0)),
        ] {
            assert!(
                run.painted.contains(&(id, point)),
                "{id} at {point:?} in {:?}",
                run.painted
            );
        }
        // The title at (19, 2), font 5; the bands' lines in font 10 from
        // (5, 1) and (5, 16) and (5, 57); the counts centred in the yard
        // column.
        let title = text(&run, "System 0");
        assert_eq!((title.1.x, title.3), (at(19.0, 0.0).x, 22.0));
        let heading = text(&run, "Ship Construction");
        assert_eq!(
            (heading.1, heading.3),
            (at(60.0, 58.0), OVERVIEW_FONT_EM * 2.0)
        );
        assert_eq!(text(&run, "No Ships are being built").1, at(60.0, 73.0));
        assert_eq!(text(&run, "No Troops in training").1, at(60.0, 154.0));
        assert_eq!(
            text(&run, "Facilities Under Construction").1,
            at(60.0, 220.0)
        );
        assert_eq!(
            run.texts
                .iter()
                .filter(|value| value.0 == "Destination: System 0")
                .count(),
            3
        );
        let ships = run
            .texts
            .iter()
            .find(|value| value.0 == "1:1")
            .unwrap_or_else(|| panic!("{:?}", run.texts));
        assert_eq!(ships.1.y, at(0.0, 119.0).y);
        assert!((ships.1.x + ships.2 / 2.0 - at(29.0, 0.0).x).abs() < 0.01);
        assert_eq!(run.texts.iter().filter(|value| value.0 == "0:0").count(), 2);
        assert!(!run.texts.iter().any(|value| value.0 == "Manufacturing"));
    }

    #[test]
    fn a_building_band_shows_its_product_and_units() {
        // FUN_00457c90: the product's name, portrait at (40, 15) and units
        // at (5, 47).
        let (mut world, systems) = fixture_world(1);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1400_0040),
            name: "Corellian Corvette".into(),
            ..CapitalShipClass::default()
        });
        let mut manufacturing = ManufacturingState::new();
        manufacturing.build(
            systems[0],
            &QueueItem::new(BuildableKind::CapitalShip(class), 10),
            2,
        );
        let mut state = opened(&world, systems[0]);
        let run = run_with(&world, &manufacturing, &mut state, hover(at(150.0, 10.0)));
        assert_eq!(text(&run, "Corellian Corvette").1, at(60.0, 73.0));
        assert_eq!(text(&run, "Building: 2").1, at(60.0, 104.0));
        let portrait =
            crate::panels::fleets::capital_ship_mini_id(DatId::new(0x1400_0040)).unwrap() - 0x4000;
        assert!(
            run.painted.contains(&(portrait, at(95.0, 72.0))),
            "{:?}",
            run.painted
        );
    }

    #[test]
    fn a_bands_name_lies_over_its_portrait() {
        // FUN_00457690.c:76-82: the band blits the portrait keyed at
        // (40, 15), then draws the units line, then the name and the
        // destination, so the name stays readable over a wide portrait
        // (under Wine the route-1 Empire save's "Death Star 1" reads whole
        // over the Death Star).
        let root = std::env::temp_dir().join(format!("band-portrait-{}", std::process::id()));
        let dir = root.join("gokres-dll/BMP");
        std::fs::create_dir_all(&dir).unwrap();
        // A 122 by 50 portrait, its rows padded to 124 bytes.
        let (width, height, stride, offset) = (122usize, 50usize, 124usize, 1078usize);
        let mut bmp = vec![0u8; offset + stride * height];
        bmp[..2].copy_from_slice(b"BM");
        bmp[10..14].copy_from_slice(&u32::try_from(offset).unwrap().to_le_bytes());
        bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
        bmp[18..22].copy_from_slice(&i32::try_from(width).unwrap().to_le_bytes());
        bmp[22..26].copy_from_slice(&i32::try_from(height).unwrap().to_le_bytes());
        bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
        bmp[28..30].copy_from_slice(&8u16.to_le_bytes());
        bmp[offset..].fill(3);
        let mini = crate::panels::fleets::capital_ship_mini_id(DatId::new(0x1400_0040)).unwrap();
        std::fs::write(dir.join(format!("{}.bmp", mini - 0x4000)), &bmp).unwrap();
        let mut cache = BmpCache::new();
        cache.set_base_path(&root);

        let (mut world, systems) = fixture_world(1);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1400_0040),
            name: "Corellian Corvette".into(),
            ..CapitalShipClass::default()
        });
        let mut manufacturing = ManufacturingState::new();
        manufacturing.build(
            systems[0],
            &QueueItem::new(BuildableKind::CapitalShip(class), 10),
            2,
        );
        let mut state = opened(&world, systems[0]);
        let run = run_cached(
            &world,
            &manufacturing,
            &mut state,
            hover(at(150.0, 10.0)),
            CockpitFaction::Alliance,
            cache,
            &mut |_, _| {},
        );
        let _ = std::fs::remove_dir_all(&root);
        let portrait = run
            .shapes
            .iter()
            .position(|shape| {
                matches!(shape, egui::Shape::Mesh(mesh)
                    if mesh.calc_bounds().min == at(95.0, 72.0))
            })
            .unwrap_or_else(|| panic!("{:?}", run.painted));
        let line = |words: &str| {
            run.shapes
                .iter()
                .position(
                    |shape| matches!(shape, egui::Shape::Text(text) if text.galley.text() == words),
                )
                .unwrap()
        };
        assert!(line("Building: 2") > portrait);
        assert!(line("Corellian Corvette") > portrait);
        assert!(line("Destination: System 0") > portrait);
    }

    #[test]
    fn a_press_on_a_band_selects_it_and_a_right_click_opens_its_menu() {
        // FUN_00458480's band rects; a press sets band +0x30 bit 0 and swaps
        // its strip (FUN_00456230); a right release opens the menu for the
        // selected managers (FUN_00453ee0, FUN_004ac5c0).
        let (world, systems) = fixture_world(1);
        let mut state = opened(&world, systems[0]);
        let manufacturing = ManufacturingState::new();
        let run = run_with(&world, &manufacturing, &mut state, click(at(100.0, 170.0)));
        assert_eq!(
            state.selection(systems[0]),
            Some((
                SystemWindowTab::Overview,
                Some(ProductionArea::TrainingFacility)
            ))
        );
        assert!(run
            .actions
            .contains(&SystemWindowAction::SelectSystem(systems[0])));
        assert!(!run
            .actions
            .iter()
            .any(|action| matches!(action, SystemWindowAction::OpenObjectMenu { .. })));
        let after = run_with(&world, &manufacturing, &mut state, hover(at(150.0, 10.0)));
        assert!(after.painted.contains(&(10_291, at(55.0, 138.0))));
        assert!(after.painted.contains(&(10_292, at(55.0, 57.0))));

        let menu = run_with(
            &world,
            &manufacturing,
            &mut state,
            right_click(at(100.0, 250.0)),
        );
        assert_eq!(
            state.selection(systems[0]),
            Some((
                SystemWindowTab::Overview,
                Some(ProductionArea::ConstructionYard)
            ))
        );
        assert!(menu.actions.contains(&SystemWindowAction::OpenObjectMenu {
            system: systems[0],
            selection: Some(MenuObject::Producer {
                system: systems[0],
                area: ProductionArea::ConstructionYard,
            }),
            point: (200, 310),
        }));
        assert_eq!(
            state.band_screen_rect(
                layout(CockpitFaction::Alliance, 2.0),
                systems[0],
                ProductionArea::Shipyard
            ),
            Some(egui::Rect::from_min_max(at(55.0, 57.0), at(221.0, 136.0)))
        );
    }

    #[test]
    fn a_mission_released_on_a_facility_cell_names_the_facility() {
        // FUN_00458b80: on a facility page, the list item under the point
        // (list +0x174 at (8, 77)); off an item, or on the overview, the
        // system (move-order.md, "Hit tests").
        let (mut world, systems) = fixture_world(1);
        world.systems[systems[0]].raw_materials = 2;
        let FacilityItem::Production(mine) = add_mine(&mut world, systems[0]) else {
            panic!("a mine is a production facility");
        };
        let mut state = opened(&world, systems[0]);
        let manufacturing = ManufacturingState::new();
        let layer = egui::LayerId::new(egui::Order::Foreground, area_id(systems[0]));
        let layout = layout(CockpitFaction::Alliance, 2.0);
        assert_eq!(state.release_object(layout, layer, at(42.0, 97.0)), None);

        let _ = run_with(&world, &manufacturing, &mut state, click(at(200.0, 30.0)));

        assert_eq!(
            state.release_object(layout, layer, at(42.0, 97.0)),
            Some(MissionTarget::ProductionFacility(mine))
        );
        assert_eq!(state.release_object(layout, layer, at(111.0, 97.0)), None);
        assert_eq!(state.release_object(layout, layer, at(4.0, 97.0)), None);
    }

    #[test]
    fn a_facility_page_shows_its_label_pictures_and_selected_frame() {
        // FUN_004568a0: the page label at (0, 58), centred; the list at
        // (8, 77) in 69 by 40 cells, each picture 1 by 2 in; a selected
        // item adds its side frame (+0x15c). The mines page pads the
        // system's deposits with 9005.
        let (mut world, systems) = fixture_world(1);
        world.systems[systems[0]].raw_materials = 2;
        let mine = add_mine(&mut world, systems[0]);
        let mut state = opened(&world, systems[0]);
        let manufacturing = ManufacturingState::new();
        let _ = run_with(&world, &manufacturing, &mut state, click(at(200.0, 30.0)));
        assert_eq!(state.windows[0].tab, SystemWindowTab::Mines);

        let run = run_with(&world, &manufacturing, &mut state, click(at(20.0, 90.0)));
        assert_eq!(state.windows[0].selected_item, Some(mine));
        let after = run_with(&world, &manufacturing, &mut state, hover(at(150.0, 10.0)));
        assert!(
            after.painted.contains(&(9_001, at(9.0, 79.0))),
            "{:?}",
            after.painted
        );
        assert!(after.painted.contains(&(9_005, at(78.0, 79.0))));
        assert!(after.painted.contains(&(10_262, at(9.0, 79.0))));
        assert!(after.painted.contains(&(10_320, at(190.0, 20.0))));
        let label = text(&after, "Mines");
        assert!((label.1.x + label.2 / 2.0 - at(113.0, 0.0).x).abs() < 0.01);
        assert_eq!((label.1.y, label.3), (at(0.0, 58.0).y, 20.0));
        assert!(run
            .actions
            .contains(&SystemWindowAction::SelectSystem(systems[0])));

        // A right press selects too and opens nothing; a press on empty list
        // space clears the selection.
        let right = run_with(
            &world,
            &manufacturing,
            &mut state,
            right_click(at(80.0, 90.0)),
        );
        assert_eq!(
            state.windows[0].selected_item,
            Some(FacilityItem::Deposit(1))
        );
        assert!(!right
            .actions
            .iter()
            .any(|action| matches!(action, SystemWindowAction::OpenObjectMenu { .. })));
        let _ = run_with(&world, &manufacturing, &mut state, click(at(100.0, 250.0)));
        assert_eq!(state.windows[0].selected_item, None);
        assert_eq!(
            state.cell_screen_rect(layout(CockpitFaction::Alliance, 2.0), systems[0], 4),
            Some(egui::Rect::from_min_max(at(77.0, 117.0), at(146.0, 157.0)))
        );
    }

    /// Press at `from`, move through `to`'s midpoint to `to` with the
    /// button held, and release there.
    fn drag(from: egui::Pos2, to: egui::Pos2) -> Vec<Vec<egui::Event>> {
        let button = |pos, pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let mut frames = hover(from);
        frames.extend([
            vec![button(from, true)],
            vec![egui::Event::PointerMoved(from.lerp(to, 0.5))],
            vec![egui::Event::PointerMoved(to)],
            vec![button(to, false)],
            vec![egui::Event::PointerMoved(to + egui::vec2(30.0, 30.0))],
        ]);
        frames
    }

    #[test]
    fn a_drag_on_the_title_moves_the_window_by_the_pointers_travel_for_either_side() {
        // FUN_00452fc0 sets the movable bit (+0xb8 | 4): a press the window
        // passes to its base (FUN_00606650 → FUN_00606ee0) takes hold, each
        // move with the button down places it (FUN_006071e0, live as
        // FUN_00606380 sets +0xf4 to 1) and the release lets go
        // (FUN_00607010). Under Wine, on the Empire assassin-combat save at
        // day 416, Carida's window moved 80 pixels left and Berchest's 120
        // right, each by the pointer's travel, with no snap.
        let (world, systems) = fixture_world(1);
        let manufacturing = ManufacturingState::new();
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut state = SystemWindowState::default();
            assert!(state.open(&world, systems[0], ORIGIN, faction, layout(faction, 2.0)));
            // The Empire's galaxy view starts further right, so the window
            // opens clamped into it.
            let (x, y) = state.windows[0].logical_position;
            let _ = run_as(
                &world,
                &manufacturing,
                &mut state,
                drag(at(130.0, 9.0), at(170.0, -11.0)),
                faction,
            );
            assert_eq!(
                state.windows[0].logical_position,
                (x + 40, y - 20),
                "{faction:?}"
            );
        }
    }

    /// Whether a drag of 20 by 20 from window pixel `from` on `tab` moves
    /// the window by that much, for `faction`.
    fn surface_drag_moves(
        world: &GameWorld,
        system: SystemKey,
        tab: SystemWindowTab,
        from: (f32, f32),
        faction: CockpitFaction,
    ) -> bool {
        let manufacturing = ManufacturingState::new();
        let mut state = SystemWindowState::default();
        assert!(state.open(world, system, ORIGIN, faction, layout(faction, 2.0)));
        state.select_tab(system, tab);
        let (x, y) = state.windows[0].logical_position;
        let point = |dx: f32, dy: f32| {
            egui::pos2(
                10.0 + (f32::from(x) + from.0 + dx) * 2.0,
                20.0 + (f32::from(y) + from.1 + dy) * 2.0,
            )
        };
        let _ = run_as(
            world,
            &manufacturing,
            &mut state,
            drag(point(0.0, 0.0), point(20.0, 20.0)),
            faction,
        );
        let moved = state.windows[0].logical_position;
        assert!(moved == (x, y) || moved == (x + 20, y + 20), "{moved:?}");
        moved != (x, y)
    }

    #[test]
    fn a_press_on_the_windows_bare_surface_drags_it_but_not_on_a_band_or_the_list() {
        // FUN_004534f0 passes a press to the base (FUN_00606ee0) unless it
        // lands on an overview band; the facility list is a list control
        // of its own.
        let (world, systems) = fixture_world(1);
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let surface = |tab, from| surface_drag_moves(&world, systems[0], tab, from, faction);
            assert!(
                surface(SystemWindowTab::Overview, (3.0, 150.0)),
                "{faction:?}: bare"
            );
            assert!(
                surface(SystemWindowTab::Overview, (100.0, 9.0)),
                "{faction:?}: title"
            );
            assert!(
                !surface(SystemWindowTab::Overview, (100.0, 100.0)),
                "{faction:?}: band"
            );
            assert!(
                !surface(SystemWindowTab::Shipyards, (100.0, 150.0)),
                "{faction:?}: list"
            );
            assert!(
                !surface(SystemWindowTab::Overview, (10.0, 30.0)),
                "{faction:?}: tab"
            );
        }
    }

    #[test]
    fn a_dragged_window_is_saved_and_restored_where_it_was_left() {
        // FUN_00441190 rebuilds a System window at its saved x and y; under
        // Wine a dragged System window came back in place.
        let (world, systems) = fixture_world(1);
        let manufacturing = ManufacturingState::new();
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut state = SystemWindowState::default();
            assert!(state.open(&world, systems[0], ORIGIN, faction, layout(faction, 2.0)));
            let _ = run_as(
                &world,
                &manufacturing,
                &mut state,
                drag(at(130.0, 9.0), at(150.0, 59.0)),
                faction,
            );
            let dragged = state.windows[0].logical_position;
            let mut restored = SystemWindowState::default();
            for (_, window) in state.saved_windows(&world) {
                assert!(restored.restore_saved(&world, faction, &window));
            }
            assert_eq!(restored.windows[0].logical_position, dragged, "{faction:?}");
        }
    }

    #[test]
    fn a_press_off_the_title_strip_or_on_a_title_button_moves_nothing() {
        // The title buttons and the tabs take their own presses
        // (FUN_004534f0), so FUN_00606ee0 never sees them.
        let (world, systems) = fixture_world(1);
        let manufacturing = ManufacturingState::new();
        for from in [at(201.0, 9.0), at(10.0, 30.0)] {
            let mut state = opened(&world, systems[0]);
            let _ = run_with(
                &world,
                &manufacturing,
                &mut state,
                drag(from, from + egui::vec2(80.0, 80.0)),
            );
            assert_eq!(state.windows[0].logical_position, ORIGIN, "{from:?}");
        }
    }

    #[test]
    fn the_title_buttons_open_the_sector_minimize_and_close() {
        let (world, systems) = fixture_world(1);
        let manufacturing = ManufacturingState::new();
        let mut state = opened(&world, systems[0]);
        let sector = run_with(&world, &manufacturing, &mut state, click(at(9.0, 9.0)));
        assert!(sector
            .actions
            .contains(&SystemWindowAction::FocusSector(systems[0])));
        assert_eq!(state.window_count(), 0);

        let mut state = opened(&world, systems[0]);
        let _ = run_with(&world, &manufacturing, &mut state, click(at(201.0, 9.0)));
        assert_eq!((state.window_count(), state.rail_count()), (0, 1));

        let mut state = opened(&world, systems[0]);
        let _ = run_with(&world, &manufacturing, &mut state, click(at(215.0, 9.0)));
        assert_eq!((state.window_count(), state.rail_count()), (0, 0));
    }

    #[test]
    fn the_other_sides_production_shows_only_where_its_contents_show() {
        // hyp: as the Defenses window's pages, the other side's facilities
        // and production show only where the player sees its objects.
        let (mut world, systems) = fixture_world(1);
        world.systems[systems[0]].control = ControlKind::Controlled(Faction::Empire);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1400_0040),
            name: "Star Destroyer".into(),
            ..CapitalShipClass::default()
        });
        let mut manufacturing = ManufacturingState::new();
        manufacturing.build(
            systems[0],
            &QueueItem::new(BuildableKind::CapitalShip(class), 10),
            1,
        );
        let mut state = opened(&world, systems[0]);
        let run = run_with(&world, &manufacturing, &mut state, hover(at(150.0, 10.0)));
        assert!(run
            .texts
            .iter()
            .any(|value| value.0 == "No Ships are being built"));
        assert!(!run.texts.iter().any(|value| value.0 == "Star Destroyer"));
        // The side 2 strip and tab art.
        assert!(run.painted.contains(&(10_294, at(55.0, 57.0))));
        assert!(run.painted.contains(&(10_314, at(0.0, 20.0))));
    }

    #[test]
    fn a_minimized_defenses_window_keeps_its_own_rail_slot_and_restores_from_it() {
        // FUN_004aa4a0: a Defenses window minimizes (0x466) to its own rail
        // entry beside the same system's Fleet window, one per window.
        let (world, systems) = fixture_world(1);
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let mut state = SystemWindowState::default();
        let _ = state.minimize_defenses_window(systems[0], (90, 70));
        state.minimize_fleet_window(fleet_window_id(101), systems[0], (100, 80));
        let _ = state.minimize_defenses_window(systems[0], (90, 70));
        assert_eq!(state.rail_count(), 2);
        assert_eq!(
            state.rail_entries().collect::<Vec<_>>(),
            [(systems[0], "fleet"), (systems[0], "defenses")]
        );

        let slot = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 1));
        assert_eq!(state.rail_slot_screen_rect(layout, 1), Some(slot));
        assert_eq!(state.rail_slot_screen_rect(layout, 2), None);
        let point = slot.center();
        let press = |pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow_mut().clear());
        let (actions, _) = run_rail(
            &world,
            &mut state,
            vec![
                vec![egui::Event::PointerMoved(point)],
                vec![egui::Event::PointerMoved(point)],
                vec![press(true)],
                vec![press(false)],
            ],
        );
        let painted = crate::fleet_window::tests::PAINTED.with(|painted| painted.take());
        assert!(
            painted.contains(&(
                crate::defenses_window::rail_icon(&world, systems[0]),
                slot.min
            )),
            "{painted:?}"
        );
        assert!(
            actions.contains(&SystemWindowAction::RestoreDefensesWindow {
                system: systems[0],
                logical_position: (90, 70),
            })
        );
        assert_eq!(state.rail_count(), 1);
        assert!(matches!(state.rail[0], RailEntry::Fleet { .. }));
    }

    #[test]
    fn a_minimized_missions_window_keeps_its_own_rail_slot_and_restores_from_it() {
        // FUN_004a21c0: a Missions window minimizes (0x466) to its own rail
        // entry beside the same system's Defenses window, one per window;
        // with no mission there its icon is side 3's 11541.
        let (world, systems) = fixture_world(1);
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let mut state = SystemWindowState::default();
        let _ = state.minimize_defenses_window(systems[0], (90, 70));
        let _ = state.minimize_missions_window(systems[0], (60, 40));
        let _ = state.minimize_missions_window(systems[0], (60, 40));
        assert_eq!(
            state.rail_entries().collect::<Vec<_>>(),
            [(systems[0], "defenses"), (systems[0], "missions")]
        );

        let slot = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 1));
        let point = slot.center();
        let press = |pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow_mut().clear());
        let (actions, _) = run_rail(
            &world,
            &mut state,
            vec![
                vec![egui::Event::PointerMoved(point)],
                vec![egui::Event::PointerMoved(point)],
                vec![press(true)],
                vec![press(false)],
            ],
        );
        let painted = crate::fleet_window::tests::PAINTED.with(|painted| painted.take());
        assert!(painted.contains(&(11541, slot.min)), "{painted:?}");
        assert!(
            actions.contains(&SystemWindowAction::RestoreMissionsWindow {
                system: systems[0],
                logical_position: (60, 40),
            })
        );
        assert_eq!(state.rail_count(), 1);
        assert!(matches!(state.rail[0], RailEntry::Defenses { .. }));
    }

    #[test]
    fn a_minimized_fleet_window_keeps_one_rail_slot_beside_its_systems_window() {
        // FUN_004a76e0: a Fleet window minimizes to the rail as its own
        // entry; the rail holds one entry per window.
        let (world, systems) = fixture_world(2);
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let mut state = SystemWindowState::default();
        let fleets = fleet_window_ids(&[201, 202]);
        let [fleet0, fleet1] = fleets.as_slice() else {
            panic!()
        };
        let (fleet0, fleet1) = (*fleet0, *fleet1);
        state.minimize_fleet_window(fleet0, systems[0], (100, 80));
        state.minimize_fleet_window(fleet0, systems[0], (100, 80));
        assert_eq!(state.rail_count(), 1);
        state.open(
            &world,
            systems[0],
            (60, 40),
            CockpitFaction::Alliance,
            layout,
        );
        assert!(state.minimize(systems[0]));
        state.minimize_fleet_window(fleet1, systems[1], (120, 90));
        assert_eq!(state.rail_count(), 3);
        assert_eq!(
            state
                .rail_entries()
                .map(|(_, kind)| kind)
                .collect::<Vec<_>>(),
            ["fleet", "system", "fleet"]
        );

        // The first slot, the Fleet window's: its icon at the corner and the
        // name at (12, 2), 9 points; a system window's name is 7 points.
        let slot = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 0));
        let away = egui::pos2(2.0, 2.0);
        let (_, texts) = run_rail(
            &world,
            &mut state,
            vec![
                vec![egui::Event::PointerMoved(away)],
                vec![egui::Event::PointerMoved(away)],
            ],
        );
        let name = slot.min + egui::vec2(12.0, 2.0) * 2.0;
        assert!(
            texts
                .iter()
                .any(|(text, pos, _, size)| text == "System 0" && *pos == name && *size == 18.0),
            "{texts:?}"
        );
        let second = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 1));
        assert!(
            texts
                .iter()
                .any(|(text, pos, width, size)| text == "System 0"
                    && (pos.x + width / 2.0 - second.center().x).abs() < 0.01
                    && *size == 14.0),
            "{texts:?}"
        );

        let point = slot.center();
        let press = |pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let (actions, _) = run_rail(
            &world,
            &mut state,
            vec![
                vec![egui::Event::PointerMoved(point)],
                vec![egui::Event::PointerMoved(point)],
                vec![press(true)],
                vec![press(false)],
            ],
        );
        assert!(actions.contains(&SystemWindowAction::RestoreFleetWindow {
            id: fleet0,
            system: systems[0],
            logical_position: (100, 80),
        }));
        // Emitting the request is not a restore commit. If validation or the
        // shared manager later rejects it, every projection stays intact.
        assert_eq!(state.rail_count(), 3);
        assert!(matches!(state.rail[0], RailEntry::Fleet { .. }));
        assert_eq!(state.rail[0].system(), systems[0]);
        assert!(state.rail[1].is_system_window());
        assert_eq!(state.rail[1].system(), systems[0]);
        assert_eq!(state.rail[2].system(), systems[1]);
    }

    #[test]
    fn a_committed_fleet_restore_retires_only_its_exact_rail_projection() {
        let (_, systems) = fixture_world(2);
        let mut state = SystemWindowState::default();
        let _ = state.minimize_defenses_window(systems[0], (90, 70));
        let fleets = fleet_window_ids(&[301, 302]);
        let [fleet0, fleet1] = fleets.as_slice() else {
            panic!()
        };
        let (fleet0, fleet1) = (*fleet0, *fleet1);
        state.minimize_fleet_window(fleet0, systems[0], (100, 80));
        state.minimize_fleet_window(fleet1, systems[1], (120, 90));

        assert!(state.restore_fleet_window(fleet0));
        assert_eq!(
            state.rail_entries().collect::<Vec<_>>(),
            [(systems[0], "defenses"), (systems[1], "fleet")]
        );
        assert!(!state.restore_fleet_window(fleet0));
    }

    #[test]
    fn reference_rail_rectangles_differ_by_faction() {
        // No recovered source: rail slot Y coordinates per faction, kept as a regression pin.
        assert_eq!(
            rail_slot_rect(CockpitFaction::Alliance, 0),
            (544.0, 61.0, 62.0, 18.0)
        );
        assert_eq!(
            rail_slot_rect(CockpitFaction::Alliance, 11),
            (544.0, 301.0, 62.0, 18.0)
        );
        assert_eq!(
            rail_slot_rect(CockpitFaction::Empire, 0),
            (21.0, 48.0, 54.0, 18.0)
        );
        assert_eq!(
            rail_slot_rect(CockpitFaction::Empire, 11),
            (21.0, 319.0, 54.0, 18.0)
        );
    }

    #[test]
    fn a_window_opens_on_the_tab_it_is_asked_for() {
        // Our own: a window opened on a page.
        let (mut world, systems) = fixture_world(2);
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let mut state = SystemWindowState::default();
        let missing = systems[1];
        world.systems.remove(missing);

        assert!(state.open_tab(
            &world,
            systems[0],
            SystemWindowTab::Refineries,
            (60, 40),
            CockpitFaction::Alliance,
            layout
        ));
        assert_eq!(state.windows[0].tab, SystemWindowTab::Refineries);
        assert!(!state.open_tab(
            &world,
            missing,
            SystemWindowTab::Refineries,
            (60, 40),
            CockpitFaction::Alliance,
            layout
        ));
        assert_eq!(state.windows.len(), 1);

        // FUN_0060d590: the tab buttons sit at (x, 20), 36 by 33.
        let origin = window_screen_rect(state.windows[0], layout).min;
        assert_eq!(
            state.tab_screen_rect(layout, systems[0], SystemWindowTab::Refineries),
            Some(egui::Rect::from_min_size(
                origin + egui::vec2(152.0, 20.0) * 2.0,
                egui::vec2(72.0, 66.0)
            ))
        );
        assert_eq!(
            state.tab_screen_rect(layout, missing, SystemWindowTab::Refineries),
            None
        );
    }

    #[test]
    fn system_window_occlusion_uses_scaled_exclusive_edges() {
        // FUN_0045aac0: 226 by 304 at (100, 60), scale 2, canvas at (10, 20).
        let (world, systems) = fixture_world(1);
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Alliance, 2.0);
        state.open(
            &world,
            systems[0],
            (100, 60),
            CockpitFaction::Alliance,
            layout,
        );
        assert!(state.contains_screen_point(layout, (210.0, 140.0)));
        assert!(state.contains_screen_point(layout, (661.9, 747.9)));
        assert!(!state.contains_screen_point(layout, (662.0, 140.0)));
        assert!(!state.contains_screen_point(layout, (210.0, 748.0)));
        let layer = egui::LayerId::new(egui::Order::Foreground, area_id(systems[0]));
        assert_eq!(state.release_target(layer), Some(systems[0]));
    }

    #[test]
    fn the_view_raises_one_window_of_the_kind_above_the_rest_for_either_side() {
        // FUN_00422ce0 0x467: BringWindowToTop on the window it activates.
        let (world, systems) = fixture_world(3);
        let (first, second, missing) = (systems[0], systems[1], systems[2]);
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut state = SystemWindowState::default();
            let layout = layout(faction, 1.0);
            assert!(state.open(&world, first, (100, 50), faction, layout));
            assert!(state.open(&world, second, (100, 50), faction, layout));
            let raised = |state: &SystemWindowState| crate::window_focus::topmost(state.children());
            assert_eq!(state.children().count(), 2, "{faction:?}");
            assert_eq!(raised(&state), Some(GalaxyChild::System(second)));
            assert!(state.raise(first), "{faction:?}");
            assert_eq!(raised(&state), Some(GalaxyChild::System(first)));
            assert!(!state.raise(missing), "{faction:?}");
        }
    }
}
