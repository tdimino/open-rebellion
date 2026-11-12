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
use rebellion_windowing::{RoutingSnapshot, SubjectId, WindowId, WindowKind, WindowTransition};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::{control_side, paint_native};
use crate::manufacturing_window::{
    band, band_strip, contents_visible, item_frame, page_cells, yard_count, yard_page,
    FacilityItem, FacilityPage, BAND_AREAS, BAND_DESTINATION, BAND_FRAME, BAND_PORTRAIT,
    BAND_RECTS, BAND_STATUS, BAND_TITLE, BAND_UNITS, CELL, COLUMNS, COUNT_ORIGINS, LIST,
    PROGRESS_RECTS, YARD_COLUMN,
};
use crate::native_geometry::{
    exact_clicked, logical_point, logical_rect, paint_texture_rect, rect_contains,
};
use crate::object_menu::MenuObject;
use crate::strategic_window_common;
use crate::windowing::GalaxyChild;
use crate::windowing_input::{
    FrameInputRouter, InputEventId, InputRecipient, PointerButtonId, PointerPhase,
};
#[cfg(test)]
use crate::windowing_policy::contextual_spec;
use crate::windowing_policy::system_subject;

pub const SYSTEM_WINDOW_CLIENT_WIDTH: f32 = 226.0;
/// `FUN_0045aac0` creates type 9 at 226 by 304 (`0xe2` by `0x130`).
pub const SYSTEM_WINDOW_WIDTH: f32 = 226.0;
pub const SYSTEM_WINDOW_HEIGHT: f32 = 304.0;
pub const REFERENCE_RAIL_SLOTS: usize = 12;

const WINDOW_BACKGROUND: u32 = 10297;
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

/// One immutable reference-rail slot projected from the manager's shared
/// presentation history. Content stores supply presentation data but never
/// own slot order or identity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReferenceRailSlot {
    id: WindowId,
    kind: WindowKind,
    system: SystemKey,
    rect: egui::Rect,
}

impl ReferenceRailSlot {
    /// Exact manager identity restored by this slot.
    #[must_use]
    pub const fn id(self) -> WindowId {
        self.id
    }

    /// Detail family used to choose the existing label/icon presenter.
    #[must_use]
    pub const fn kind(self) -> WindowKind {
        self.kind
    }

    /// Runtime system resolved from the stable manager subject.
    #[must_use]
    pub const fn system(self) -> SystemKey {
        self.system
    }

    /// Current faction-scaled slot rectangle.
    #[must_use]
    pub const fn rect(self) -> egui::Rect {
        self.rect
    }
}

/// Projects the manager's oldest-first minimized detail history into the
/// faction-specific twelve-slot rail without creating a second history.
#[must_use]
pub fn reference_rail_slots(
    snapshot: &RoutingSnapshot,
    world: &GameWorld,
    faction: CockpitFaction,
    layout: CockpitLayout,
) -> Vec<ReferenceRailSlot> {
    snapshot
        .presentation_history()
        .iter()
        .take(REFERENCE_RAIL_SLOTS)
        .enumerate()
        .filter_map(|(index, &id)| {
            let key = snapshot.key(id)?;
            if !matches!(
                key.kind,
                WindowKind::System
                    | WindowKind::Fleet
                    | WindowKind::Defenses
                    | WindowKind::Missions
            ) {
                return None;
            }
            let subject = key.subject?;
            let system = world.systems.iter().find_map(|(system, _)| {
                (system_subject(world, system) == Some(subject)).then_some(system)
            })?;
            Some(ReferenceRailSlot {
                id,
                kind: key.kind,
                system,
                rect: cockpit_rect(layout, rail_slot_rect(faction, index)),
            })
        })
        .collect()
}

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
    /// Opaque session identity allocated by the shared window manager.
    /// Fixture-only direct opens deliberately have no production identity.
    id: Option<WindowId>,
    subject: Option<SubjectId>,
    system: SystemKey,
    logical_position: (i16, i16),
    tab: SystemWindowTab,
    /// A facility page's selected item.
    selected_item: Option<FacilityItem>,
    /// The overview's selected band (band `+0x30` bit 0). port: one band;
    /// Ctrl's toggle is not ported.
    selected_band: Option<ProductionArea>,
}

strategic_window_common::impl_strategic_window_record!(
    OpenSystemWindow,
    open_window_area_id,
    open_window_screen_rect
);

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
    },
    /// A Missions window (type 11); its icon comes from `FUN_004a21c0`.
    Missions {
        system: SystemKey,
        logical_position: (i16, i16),
    },
}

impl RailEntry {
    fn system(self) -> SystemKey {
        match self {
            Self::System(window) => window.system,
            Self::Fleet { system, .. }
            | Self::Defenses { system, .. }
            | Self::Missions { system, .. } => system,
        }
    }

    fn is_system_window(self) -> bool {
        matches!(self, Self::System(_))
    }

    fn same_window(self, other: &Self) -> bool {
        match (self, *other) {
            (Self::System(left), Self::System(right)) => left.system == right.system,
            (Self::Fleet { id: left, .. }, Self::Fleet { id: right, .. }) => left == right,
            (Self::Defenses { system: left, .. }, Self::Defenses { system: right, .. })
            | (Self::Missions { system: left, .. }, Self::Missions { system: right, .. }) => {
                left == right
            }
            _ => false,
        }
    }
}

/// Mutable System-window content. Presentation history belongs exclusively to
/// the shared manager and is projected by [`reference_rail_slots`].
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

strategic_window_common::impl_strategic_window_state_core!(
    SystemWindowState,
    OpenSystemWindow,
    open_window_area_id,
    rail = Vec::new(),
    painted = Vec::new(),
    band_rename = None,
);
strategic_window_common::impl_system_keyed_window_routing!(
    SystemWindowState,
    GalaxyChild::System,
    open_window_area_id,
    open_window_screen_rect
);
strategic_window_common::define_basic_prepared_detail!(
    /// Validated System content held aside until the manager accepts its
    /// stable `(System, system DAT ID)` identity.
    PreparedSystemWindow for SystemWindowState,
    kind: WindowKind::System,
    size: (SYSTEM_WINDOW_WIDTH, SYSTEM_WINDOW_HEIGHT),
);
strategic_window_common::impl_system_content_reports!(SystemWindowState, WindowKind::System);
strategic_window_common::impl_saved_system_windows!(SystemWindowState, |system| {
    SavedWindowKind::System { system }
});

/// A band's in-place name edit (window `+0x170`, a `CoolStringField`), which
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
    /// Publishes new System content, or confirms the exact preserved content,
    /// only after a successful manager open commit.
    pub fn commit_open(
        &mut self,
        prepared: PreparedSystemWindow,
        transition: &WindowTransition,
    ) -> Option<WindowId> {
        strategic_window_common::commit_prepared_detail!(
            self,
            prepared,
            transition,
            |id, subject| OpenSystemWindow {
                id: Some(id),
                subject: Some(subject),
                system: prepared.system,
                logical_position: prepared.logical_position,
                tab: SystemWindowTab::Overview,
                selected_item: None,
                selected_band: None,
            },
            |_| {},
        )
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
        strategic_window_common::open_direct_system(
            &mut self.windows,
            world,
            system,
            |window| window.system == system,
            || OpenSystemWindow {
                id: None,
                subject: None,
                system,
                logical_position: clamp_to_galaxy(logical_position, layout),
                tab: SystemWindowTab::Overview,
                selected_item: None,
                selected_band: None,
            },
        )
    }

    /// Removes only content named by manager teardown receipts.
    pub fn remove_content(&mut self, removed: &[WindowId]) {
        strategic_window_common::remove_content(&mut self.windows, removed);
    }

    /// Reports every active System identity in the frozen snapshot, including
    /// minimized history entries whose content remains attached.
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
        Some(tab_rect(
            self.detail_screen_rect(layout, system)?,
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
        Some(band_rect(
            self.detail_screen_rect(layout, system)?,
            layout.scale,
            area,
        ))
    }

    fn detail_screen_rect(&self, layout: CockpitLayout, system: SystemKey) -> Option<egui::Rect> {
        strategic_window_common::screen_rect(&self.windows, system, |window| {
            window_screen_rect(*window, layout)
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
            .find(|window| window_area_id(**window) == layer.id)
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
        let (window, local) = strategic_window_common::local_point_for_layer(
            &self.windows,
            layout,
            layer,
            point,
            |window| area_id(window.system),
            |window, layout| window_screen_rect(*window, layout),
        )?;
        window.tab.page()?;
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
        self.band_rename = None;
        self.title_drag = None;
    }

    fn prepare_faction(&mut self, faction: CockpitFaction) {
        if self.faction != faction {
            self.faction = faction;
            self.clear();
        }
    }

    /// The open System children; z-order and focus come from the manager.
    pub fn children(&self) -> impl Iterator<Item = GalaxyChild> + '_ {
        self.windows
            .iter()
            .map(|window| GalaxyChild::System(window.system))
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

    /// Whether the Fleet window `id` is minimized on the rail.
    #[must_use]
    pub fn rail_holds_fleet(&self, id: WindowId) -> bool {
        self.rail
            .iter()
            .any(|entry| matches!(entry, RailEntry::Fleet { id: held, .. } if *held == id))
    }

    /// The rail's hidden content. Save ordering comes from the manager's
    /// presentation history, while this state supplies kind and position.
    #[must_use]
    pub fn saved_rail(
        &self,
        world: &GameWorld,
        fleet: impl Fn(WindowId) -> Option<u32>,
    ) -> Vec<SavedWindow> {
        self.rail
            .iter()
            .filter_map(|entry| {
                let system = world.systems.get(entry.system())?.dat_id;
                let (kind, position) = match *entry {
                    RailEntry::System(window) => {
                        (SavedWindowKind::System { system }, window.logical_position)
                    }
                    RailEntry::Fleet {
                        id,
                        logical_position,
                        ..
                    } => {
                        let tab = fleet(id)?;
                        (SavedWindowKind::Fleet { system, tab }, logical_position)
                    }
                    RailEntry::Defenses {
                        logical_position, ..
                    } => (SavedWindowKind::Defenses { system }, logical_position),
                    RailEntry::Missions {
                        logical_position, ..
                    } => (SavedWindowKind::Missions { system }, logical_position),
                };
                Some(SavedWindow {
                    kind,
                    position,
                    shown: false,
                })
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
                    id: None,
                    subject: None,
                    system,
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
                });
                true
            }
            SavedWindowKind::Missions { .. } if !saved.shown => {
                self.push_rail(RailEntry::Missions {
                    system,
                    logical_position: position,
                });
                true
            }
            _ => false,
        }
    }

    fn focus(&mut self, system: SystemKey) -> bool {
        strategic_window_common::focus_where(&mut self.windows, |window| {
            window.id.is_none() && window.system == system
        })
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

    /// The manager window whose band name is being edited, if any.
    #[must_use]
    pub fn band_rename_window(&self) -> Option<WindowId> {
        self.band_rename
            .as_ref()
            .and_then(|edit| self.window_id(edit.system))
    }

    #[cfg(test)]
    fn minimize(&mut self, system: SystemKey) -> bool {
        self.minimize_with_eviction(system).0
    }

    #[cfg(test)]
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
        if self.rail_holds_fleet(id) {
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
        self.minimize_defenses_window_at(system, logical_position)
    }

    /// Put a minimized Defenses window on the rail (`0x466`).
    pub fn minimize_defenses_window_at(
        &mut self,
        system: SystemKey,
        logical_position: (i16, i16),
    ) -> Option<WindowId> {
        Self::evicted_fleet(self.push_rail(RailEntry::Defenses {
            system,
            logical_position,
        }))
    }

    /// Put a minimized Missions window on the rail (`0x466`), above every
    /// window in the z-order.
    pub fn minimize_missions_window(
        &mut self,
        system: SystemKey,
        logical_position: (i16, i16),
    ) -> Option<WindowId> {
        self.minimize_missions_window_at(system, logical_position)
    }

    /// Put a minimized Missions window on the rail (`0x466`).
    pub fn minimize_missions_window_at(
        &mut self,
        system: SystemKey,
        logical_position: (i16, i16),
    ) -> Option<WindowId> {
        Self::evicted_fleet(self.push_rail(RailEntry::Missions {
            system,
            logical_position,
        }))
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

    #[cfg(test)]
    fn restore(&mut self, system: SystemKey) -> bool {
        let Some(index) = self
            .rail
            .iter()
            .position(|entry| entry.is_system_window() && entry.system() == system)
        else {
            return false;
        };
        if let RailEntry::System(window) = self.rail.remove(index) {
            self.windows.push(window);
        }
        true
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
    /// A pointer activation requests shared raise/focus for this identity.
    Activate(WindowId),
    /// Close or stale content requests exact manager teardown.
    Close(WindowId),
    /// Manager-owned System content requests minimization. Content stays
    /// attached until an explicit close/eviction receipt removes it.
    MinimizeManaged {
        window: WindowId,
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A click on the shared reference rail restores this exact manager ID.
    RestoreReferenceWindow(WindowId),
    /// A right-button release on a band opens the object pop-up menu
    /// (`FUN_004ac5c0`) for the selection, at a 640 by 480 canvas point.
    OpenObjectMenu {
        /// Exact registered invoker when this is manager-owned content.
        owner: Option<WindowId>,
        system: SystemKey,
        selection: Option<MenuObject>,
        point: (i16, i16),
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

/// One ordered, position-bearing pointer event routed from the frozen frame
/// snapshot to System content or the shared reference rail.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SystemWindowInputEvent {
    pub id: InputEventId,
    pub point: (f32, f32),
    pub phase: PointerPhase,
}

#[derive(Default)]
#[allow(
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
    /// The live edit's screen rectangle, used to distinguish its own press
    /// from an owning-window press admitted by [`FrameInputRouter`].
    rename_field: Option<egui::Rect>,
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

struct SystemDraw<'a> {
    ctx: &'a egui::Context,
    world: &'a GameWorld,
    movement: &'a rebellion_core::movement::MovementState,
    fog: &'a FogState,
    missions: &'a MissionState,
    manufacturing: &'a ManufacturingState,
    deliveries: &'a DeliveryState,
    state: &'a mut SystemWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &'a mut BmpCache,
}

/// Draw the faction rail and every visible Manufacturing and Production
/// window.
///
/// port: the window's keys (`FUN_00458980`) are not ported; the galaxy
/// view's windows keep no shared keyboard focus to route them by.
#[allow(
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
    draw_system_windows_with_snapshot(
        SystemDraw {
            ctx,
            world,
            movement,
            fog,
            missions,
            manufacturing,
            deliveries,
            state,
            faction,
            layout,
            cache,
        },
        None,
        &[],
        None,
        front,
        shown,
    )
}

/// Draw manager-owned System content in the frozen manager order while the
/// existing reference rail remains a temporary presentation adapter.
pub struct ManagedSystemDraw<'a> {
    pub ctx: &'a egui::Context,
    pub world: &'a GameWorld,
    pub movement: &'a rebellion_core::movement::MovementState,
    pub fog: &'a FogState,
    pub missions: &'a MissionState,
    pub manufacturing: &'a ManufacturingState,
    pub deliveries: &'a DeliveryState,
    pub state: &'a mut SystemWindowState,
    pub faction: CockpitFaction,
    pub layout: CockpitLayout,
    pub cache: &'a mut BmpCache,
    pub snapshot: &'a RoutingSnapshot,
    pub pointer_events: &'a [SystemWindowInputEvent],
    pub router: &'a mut FrameInputRouter,
    pub front: bool,
    pub shown: &'a dyn Fn(GalaxyChild) -> bool,
}

#[must_use]
pub fn draw_managed_system_windows(draw: ManagedSystemDraw<'_>) -> Vec<SystemWindowAction> {
    let ManagedSystemDraw {
        ctx,
        world,
        movement,
        fog,
        missions,
        manufacturing,
        deliveries,
        state,
        faction,
        layout,
        cache,
        snapshot,
        pointer_events,
        router,
        front,
        shown,
    } = draw;
    draw_system_windows_with_snapshot(
        SystemDraw {
            ctx,
            world,
            movement,
            fog,
            missions,
            manufacturing,
            deliveries,
            state,
            faction,
            layout,
            cache,
        },
        Some(snapshot),
        pointer_events,
        Some(router),
        front,
        shown,
    )
}

/// Claim the exact press that the owning System window routes through the
/// live band-name edit (`FUN_004534f0`, 0x201/0x204 -> `FUN_00458780`).
/// Raw egui pointer state is only a presentation copy; manager-owned content
/// may publish the rename only after this routed event is admitted and
/// claimed for the window identity.
fn claim_band_rename_press(
    router: &mut FrameInputRouter,
    events: &[SystemWindowInputEvent],
    window: OpenSystemWindow,
    layout: CockpitLayout,
    field: egui::Rect,
) -> Option<InputEventId> {
    let id = window.id?;
    let screen = window_screen_rect(window, layout);
    events.iter().rev().find_map(|event| {
        let point = egui::pos2(event.point.0, event.point.1);
        (matches!(
            event.phase,
            PointerPhase::Press(PointerButtonId::Primary | PointerButtonId::Secondary)
        ) && rect_contains(screen, point)
            && !rect_contains(field, point)
            && router.pointer_recipient(event.id) == InputRecipient::Window(id)
            && router.claim(event.id, InputRecipient::Window(id)))
        .then_some(event.id)
    })
}

fn draw_system_windows_with_snapshot(
    draw: SystemDraw<'_>,
    snapshot: Option<&RoutingSnapshot>,
    pointer_events: &[SystemWindowInputEvent],
    mut router: Option<&mut FrameInputRouter>,
    front: bool,
    shown: &dyn Fn(GalaxyChild) -> bool,
) -> Vec<SystemWindowAction> {
    let SystemDraw {
        ctx,
        world,
        movement,
        fog,
        missions,
        manufacturing,
        deliveries,
        state,
        faction,
        layout,
        cache,
    } = draw;
    if state.windows.iter().all(|window| window.id.is_none()) {
        state.prepare_faction(faction);
    }
    let mut actions: Vec<SystemWindowAction> = match (snapshot, router.as_deref_mut()) {
        (Some(snapshot), Some(router)) => draw_reference_rail(
            ctx,
            world,
            movement,
            fog,
            missions,
            faction,
            layout,
            cache,
            snapshot,
            pointer_events,
            router,
        ),
        _ => Vec::new(),
    };
    let sources = Sources {
        world,
        manufacturing,
        deliveries,
        fog,
    };
    state.update_title_drag(ctx, layout);
    let windows = strategic_window_common::presentation_windows(
        &state.windows,
        snapshot,
        WindowKind::System,
        |window| shown(GalaxyChild::System(window.system)),
    );
    let (focused_manager, focused_fixture_system) =
        strategic_window_common::presentation_focus(&windows, snapshot, false);
    for window in windows {
        let system = window.system;
        let rename = state
            .band_rename
            .clone()
            .filter(|edit| edit.system == system);
        let focused =
            strategic_window_common::is_focused(&window, focused_manager, focused_fixture_system)
                && front;
        let mut result = draw_system_window(
            ctx,
            sources,
            window,
            rename.as_ref(),
            focused,
            faction,
            layout,
            cache,
        );
        let rename_press_text = match (&result.rename, &rename) {
            (None, Some(edit)) => Some(edit.text.clone()),
            (Some(BandRenameOutcome::Typed(text)), Some(_)) => Some(text.clone()),
            _ => None,
        };
        // FUN_004534f0 0x201/0x204 finishes the edit (FUN_00458780) and then
        // handles the same press: it selects the band under it or passes it
        // on to FUN_004ac3a0, so the press is not consumed by the rename.
        let rename_press = match (
            rename_press_text,
            result.rename_field,
            router.as_deref_mut(),
        ) {
            (Some(text), Some(field), Some(router)) => {
                let press = claim_band_rename_press(router, pointer_events, window, layout, field);
                if press.is_some() {
                    result.rename = Some(BandRenameOutcome::Commit(text));
                }
                press
            }
            _ => None,
        };
        strategic_window_common::start_title_drag(
            &mut state.title_drag,
            area_id(window.system),
            result.drag_grab,
        );
        // Escape goes to the focused window: the field passes it to its own
        // System window (FUN_00605160 -> FUN_00458980) only while that window
        // holds the keyboard focus (gid-legend.md, "Escape and the keyboard
        // focus"), so an unfocused window's open edit ignores this frame's Escape.
        if !focused && matches!(result.rename, Some(BandRenameOutcome::Escape)) {
            result.rename = None;
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
                if !text.is_empty() && text != edit.original {
                    actions.push(SystemWindowAction::RenameProduct {
                        system,
                        area: edit.area,
                        name: text,
                    });
                }
            }
            (Some(BandRenameOutcome::Escape), Some(_)) => {
                state.band_rename = None;
                if let Some(window) = window.id {
                    // FUN_004ac950/FUN_004aca40: Escape abandons the edit and
                    // closes the containing System window. Manager-owned
                    // content must close through its exact identity so the
                    // renderer cannot orphan a live registry entry.
                    actions.push(SystemWindowAction::Close(window));
                } else {
                    state.close(system);
                }
                continue;
            }
            _ => {}
        }
        state.painted.retain(|(painted, _)| *painted != system);
        state.painted.push((system, result.painted.clone()));
        let pointer_mutation = result.focus
            || result.close
            || result.minimize
            || result.focus_sector
            || result.tab.is_some()
            || result.item.is_some()
            || result.band.is_some()
            || result.object_menu.is_some();
        let phase = strategic_window_common::pointer_mutation_phase([(
            result.object_menu.is_some(),
            PointerPhase::Release(PointerButtonId::Secondary),
        )]);
        let pointer_permitted = strategic_window_common::single_phase_pointer_mutation_permitted(
            window.id,
            pointer_mutation,
            router.as_deref_mut(),
            phase,
            false,
            |router, id, phase| {
                strategic_window_common::claim_pointer_event(
                    router,
                    pointer_events
                        .iter()
                        .rev()
                        .map(|event| (event.id, event.phase)),
                    id,
                    phase,
                )
                // The rename already claimed this window's press; the
                // same press drives the rest (FUN_004534f0 0x201/0x204).
                .or(rename_press)
                .is_some()
            },
        );
        if strategic_window_common::apply_detail_window_exit(
            strategic_window_common::DetailWindowExit {
                id: window.id,
                system,
                logical_position: window.logical_position,
                close_requested: result.close && pointer_permitted,
                minimize_requested: result.minimize,
                pointer_permitted,
                system_exists: world.systems.contains_key(system),
            },
            &mut actions,
            |system| {
                state.close(system);
                Some(window.logical_position)
            },
            SystemWindowAction::Close,
            |window, system, logical_position| SystemWindowAction::MinimizeManaged {
                window,
                system,
                logical_position,
            },
            |_, _| None,
        ) {
            continue;
        }
        if result.focus_sector && pointer_permitted {
            actions.push(SystemWindowAction::FocusSector(system));
            if let Some(id) = window.id {
                actions.push(SystemWindowAction::Close(id));
            } else {
                state.close(system);
            }
            continue;
        }
        if pointer_permitted {
            if let Some(tab) = result.tab {
                state.select_tab(system, tab);
            }
            if let Some(item) = result.item {
                state.select_item(system, item);
            }
            if let Some(area) = result.band {
                state.select_band(system, area);
            }
        }
        strategic_window_common::apply_detail_focus! {
            (state, actions, window, system),
            (result.focus, pointer_permitted),
            SystemWindowAction,
        };
        if pointer_permitted {
            if let Some((selection, point)) = result.object_menu {
                actions.push(SystemWindowAction::OpenObjectMenu {
                    owner: window.id,
                    system,
                    selection,
                    point,
                });
            }
        }
    }
    actions
}

#[allow(
    clippy::too_many_arguments,
    reason = "The shared rail presents four existing detail families and consumes the frame router's root lease."
)]
fn draw_reference_rail(
    ctx: &egui::Context,
    world: &GameWorld,
    movement: &rebellion_core::movement::MovementState,
    fog: &FogState,
    missions: &MissionState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    snapshot: &RoutingSnapshot,
    pointer_events: &[SystemWindowInputEvent],
    router: &mut FrameInputRouter,
) -> Vec<SystemWindowAction> {
    let mut actions = Vec::new();
    for slot in reference_rail_slots(snapshot, world, faction, layout) {
        let system = &world.systems[slot.system];
        let color = relationship_color(relationship(system.control, faction.player()));
        egui::Area::new(egui::Id::new(("original-reference-rail", slot.id.get())))
            .fixed_pos(slot.rect.min)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let (slot_rect, response) =
                    ui.allocate_exact_size(slot.rect.size(), egui::Sense::click());
                if slot.kind == WindowKind::System {
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
                    let icon = match slot.kind {
                        WindowKind::Fleet => crate::fleet_window::rail_icon(
                            world,
                            movement,
                            fog,
                            faction.player(),
                            slot.system,
                        ),
                        WindowKind::Defenses => {
                            crate::defenses_window::rail_icon(world, slot.system)
                        }
                        WindowKind::Missions => crate::missions_window::rail_icon(
                            world,
                            fog,
                            missions,
                            faction,
                            slot.system,
                        ),
                        _ => unreachable!("reference rail projects only detail families"),
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
                if exact_clicked(&response, slot_rect)
                    && pointer_events.iter().rev().any(|event| {
                        event.phase == PointerPhase::Release(PointerButtonId::Primary)
                            && rect_contains(slot_rect, egui::pos2(event.point.0, event.point.1))
                            && router.pointer_recipient(event.id) == InputRecipient::Root
                            && router.claim_root(event.id)
                    })
                {
                    actions.push(SystemWindowAction::RestoreReferenceWindow(slot.id));
                }
            });
    }
    actions
}

#[allow(
    clippy::too_many_lines,
    reason = "Keep the window's ordered paint and input pass together, as the Defenses window does."
)]
#[allow(
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
    let Some(system) =
        strategic_window_common::system_or_close(world, window.system, &mut result.close)
    else {
        return result;
    };
    let player = faction.player();
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
    let id = window_area_id(window);
    let area = strategic_window_common::show_canvas(ctx, id, screen_rect, scale, |ui, canvas| {
        let pointer = canvas.pointer;
        let primary_down = canvas.primary_down;
        let local = canvas.local;
        let window_response = canvas.response.clone();
        let painter = canvas.painter.clone();
        let paint = |cache: &mut BmpCache, source: DllSource, id: u32, x: f32, y: f32| {
            canvas.paint(cache, source, id, x, y);
        };

        canvas.paint_background(cache, WINDOW_BACKGROUND);
        let clicked = canvas.paint_title_bar(
            ui,
            cache,
            side,
            focused,
            strategic_window_common::TitleLabel::left_centered(
                &system.name,
                TITLE.0,
                TITLE.1,
                TITLE.2,
            ),
            strategic_window_common::title_buttons([SECTOR_X, MINIMIZE_X, CLOSE_X]),
            window.system,
            |_, _| {},
        );
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
        if window.tab.page().is_none() {
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
                let shown = band(world, sources.manufacturing, visible, window.system, area);
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
                        let field =
                            band_rename_rect(ui, local, scale, (bx, by), &edit.original, &font);
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
                result.rename_field = Some(field);
                result.rename = draw_band_rename(ui, field, edit, &font);
                let elsewhere = pointer.is_some_and(|point| {
                    rect_contains(local, point) && !rect_contains(field, point)
                });
                // Direct renderer fixtures have no manager identity or
                // routed event stream. Production content must claim
                // its owning window's exact admitted press above.
                if window.id.is_none() && any_pressed && elsewhere {
                    result.rename = Some(BandRenameOutcome::Commit(match result.rename {
                        Some(BandRenameOutcome::Typed(ref text)) => text.clone(),
                        _ => edit.text.clone(),
                    }));
                }
            }
        } else {
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
            let list = logical_rect(local, scale, LIST.0, LIST.1, LIST.2, LIST.3).intersect(local);
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

        result.drag_grab = surface_drag_grab(ctx, &window_response, local, scale, &[]);
        result.focus |=
            strategic_window_common::interaction_requests_focus(&window_response, &clicked);
    });
    result.focus |= area.response.clicked();
    result
}

fn area_id(system: SystemKey) -> egui::Id {
    egui::Id::new(("original-system-window", system))
}

fn window_area_id(window: OpenSystemWindow) -> egui::Id {
    window.id.map_or_else(
        || area_id(window.system),
        crate::windowing::managed_child_id,
    )
}

fn open_window_area_id(window: &OpenSystemWindow) -> egui::Id {
    window_area_id(*window)
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
#[allow(
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
fn cell_rect(window: egui::Rect, scale: f32, index: usize) -> egui::Rect {
    strategic_window_common::grid_cell_rect(window, scale, index, COLUMNS, (LIST.0, LIST.1), CELL)
}

strategic_window_common::impl_strategic_window_cell_rect!(
    SystemWindowState,
    |window, layout| window_screen_rect(*window, layout),
    cell_rect
);

/// A screen point in 640 by 480 canvas coordinates, truncated as a Win32
/// `POINT` is.
#[allow(
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
        strategic_window_common::select_all_text(ui.ctx(), id, count);
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
#[allow(
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
    open_window_screen_rect(&window, layout)
}

strategic_window_common::define_record_screen_rect!(
    open_window_screen_rect,
    OpenSystemWindow,
    SYSTEM_WINDOW_WIDTH,
    SYSTEM_WINDOW_HEIGHT
);

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
    paint_texture_rect(painter, texture_id, rect);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
    use crate::test_support::{
        click_frames as click, full_raw_input as frame_input, hover_frames as hover,
        key_press as key, right_click_frames as right_click, title_drag_frames as drag,
    };
    use rebellion_core::dat::Faction;
    use rebellion_core::world::{ManufacturingFacilityInstance, ProductionFacilityInstance};
    use rebellion_windowing::{
        SubjectId, WindowCommand, WindowId, WindowKind, WindowManager, WindowOwner,
        WindowTransition,
    };

    fn fleet_window_id(value: u64) -> WindowId {
        fleet_window_ids(&[value])[0]
    }

    fn fleet_window_ids(values: &[u64]) -> Vec<WindowId> {
        let mut manager = WindowManager::new();
        values
            .iter()
            .map(|value| {
                let spec = crate::windowing_policy::contextual_spec(
                    WindowKind::Fleet,
                    SubjectId::new(crate::windowing_policy::SUBJECT_SYSTEM, *value),
                    WindowOwner::StrategicRoot,
                );
                let receipt = manager.apply(WindowCommand::Open(spec)).unwrap();
                let WindowTransition::Opened { id } = receipt.transition else {
                    panic!("fixture Fleet subject must allocate a fresh identity");
                };
                id
            })
            .collect()
    }

    fn fleet_window_pair(subjects: [u64; 2]) -> (WindowId, WindowId) {
        let fleets = fleet_window_ids(&subjects);
        let [first, second] = fleets.as_slice() else {
            unreachable!("a two-subject fixture must create two Fleet windows")
        };
        (*first, *second)
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

    #[allow(
        clippy::cast_possible_truncation,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn fixture_world(count: usize) -> (GameWorld, Vec<SystemKey>) {
        assert!(count > 0, "fixture needs at least one system");
        let (mut world, first) =
            crate::test_support::world(ControlKind::Controlled(Faction::Alliance));
        let sector = world.systems[first].sector;
        let mut systems = vec![first];
        for index in 0..count {
            let system = if index == 0 {
                first
            } else {
                let system = world.systems[first].clone();
                let key = world.systems.insert(system);
                world.sectors[sector].systems.push(key);
                systems.push(key);
                key
            };
            let fixture = &mut world.systems[system];
            fixture.dat_id = DatId::new(100 + index as u32);
            fixture.name = format!("System {index}");
            fixture.x = 320 + index as u16;
            fixture.popularity_alliance = 1.0;
            fixture.popularity_empire = 0.0;
            fixture.total_energy = 8;
            fixture.raw_materials = 6;
        }
        (world, systems)
    }

    fn controlled_fixture_world(
        count: usize,
        faction: CockpitFaction,
    ) -> (GameWorld, Vec<SystemKey>) {
        let (mut world, systems) = fixture_world(count);
        for system in &systems {
            world.systems[*system].control = ControlKind::Controlled(faction.player());
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

    fn capital_ship_build(name: &str, count: u32) -> (GameWorld, SystemKey, ManufacturingState) {
        let (mut world, systems) = fixture_world(1);
        let system = systems[0];
        let mut manufacturing = ManufacturingState::new();
        rebellion_core::test_support::queue_capital_ship_build(
            &mut world,
            &mut manufacturing,
            system,
            name,
            false,
            false,
            count,
        );
        (world, system, manufacturing)
    }

    fn commit_prepared_system_window(
        manager: &mut WindowManager,
        state: &mut SystemWindowState,
        prepared: PreparedSystemWindow,
    ) -> (WindowId, WindowTransition) {
        let commit = manager
            .apply(WindowCommand::Open(prepared.spec()))
            .expect("prepared System window should satisfy its declared policy");
        let id = state
            .commit_open(prepared, &commit.transition)
            .expect("successful System open should publish or preserve content");
        (id, commit.transition)
    }

    fn prepare_test_window(
        state: &SystemWindowState,
        world: &GameWorld,
        system: SystemKey,
        position: (i16, i16),
        faction: CockpitFaction,
        layout: CockpitLayout,
    ) -> PreparedSystemWindow {
        state
            .prepare_open(world, system, position, faction, layout)
            .expect("live system has a stable DAT subject")
    }

    fn managed_test_window(
        world: &GameWorld,
        system: SystemKey,
        layout: CockpitLayout,
    ) -> (WindowManager, SystemWindowState, WindowId) {
        let mut manager = WindowManager::new();
        let mut state = SystemWindowState::default();
        let prepared = prepare_test_window(
            &state,
            world,
            system,
            ORIGIN,
            CockpitFaction::Alliance,
            layout,
        );
        let (id, _) = commit_prepared_system_window(&mut manager, &mut state, prepared);
        (manager, state, id)
    }

    #[test]
    fn system_window_shared_identity_preserves_content() {
        // `strategic-windowing-toolkit.md`, Window-family matrix: type 9 is a
        // subject-keyed Strategic-root detail whose reopen restores/raises.
        let (mut world, systems) = fixture_world(1);
        let system = systems[0];
        let mine = add_mine(&mut world, system);
        let mut manager = WindowManager::new();
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Alliance, 1.0);

        let first = state
            .prepare_open(&world, system, ORIGIN, CockpitFaction::Alliance, layout)
            .expect("live system has a stable DAT subject");
        assert_eq!(first.spec().key.kind, WindowKind::System);
        assert_eq!(first.spec().owner, WindowOwner::StrategicRoot);
        let (id, opened) = commit_prepared_system_window(&mut manager, &mut state, first);
        assert_eq!(opened, WindowTransition::Opened { id });

        let content = state.window_mut(system).unwrap();
        content.tab = SystemWindowTab::Mines;
        content.selected_item = Some(mine);
        content.selected_band = Some(ProductionArea::ConstructionYard);

        let reopened = state
            .prepare_open(&world, system, (400, 300), CockpitFaction::Alliance, layout)
            .unwrap();
        crate::test_support::assert_existing_detail_reopen(id, || {
            commit_prepared_system_window(&mut manager, &mut state, reopened)
        });
        assert_eq!(state.window_count(), 1);
        assert_eq!(
            state.selection(system),
            Some((
                SystemWindowTab::Mines,
                Some(ProductionArea::ConstructionYard)
            ))
        );
        let content = state.window_mut(system).unwrap();
        assert_eq!(content.logical_position, ORIGIN);
        assert_eq!(content.selected_item, Some(mine));

        crate::test_support::assert_simple_minimized_reopen(
            &mut manager,
            &mut state,
            id,
            |state, frame| state.window_is_visible(frame, system),
            |manager, state| {
                let restored = state
                    .prepare_open(&world, system, (500, 350), CockpitFaction::Alliance, layout)
                    .unwrap();
                commit_prepared_system_window(manager, state, restored)
            },
        );
        assert_eq!(state.window_count(), 1);
        assert_eq!(state.window_mut(system).unwrap().selected_item, Some(mine));
    }

    #[test]
    fn system_window_stale_subject_does_not_retarget() {
        // A stale runtime key must not acquire another system's DAT subject or
        // mutate the content already attached to the manager-issued identity.
        let (mut world, systems) = fixture_world(1);
        let stale_system = systems[0];
        let mut manager = WindowManager::new();
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Alliance, 1.0);
        let prepared = state
            .prepare_open(
                &world,
                stale_system,
                ORIGIN,
                CockpitFaction::Alliance,
                layout,
            )
            .unwrap();
        let (id, _) = commit_prepared_system_window(&mut manager, &mut state, prepared);
        state.select_tab(stale_system, SystemWindowTab::Refineries);

        let mut replacement = world.systems.remove(stale_system).unwrap();
        replacement.dat_id = DatId::new(101);
        replacement.name = "Replacement".into();
        let replacement_system = world.systems.insert(replacement);
        assert_ne!(replacement_system, stale_system);

        assert!(state
            .prepare_open(
                &world,
                stale_system,
                (400, 300),
                CockpitFaction::Alliance,
                layout,
            )
            .is_none());
        assert_eq!(state.window_id(stale_system), Some(id));
        assert_eq!(
            state.selection(stale_system),
            Some((SystemWindowTab::Refineries, None))
        );
        assert_eq!(state.window_id(replacement_system), None);
    }

    #[test]
    fn system_window_manager_order_is_independent_of_content_vector() {
        // The System store retains content lookup order only; shared manager
        // activation is the sole cross-family stack authority.
        let (world, systems) = fixture_world(2);
        let mut manager = WindowManager::new();
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Alliance, 1.0);
        let mut ids = Vec::new();
        for system in &systems {
            let prepared = state
                .prepare_open(&world, *system, ORIGIN, CockpitFaction::Alliance, layout)
                .unwrap();
            ids.push(commit_prepared_system_window(&mut manager, &mut state, prepared).0);
        }
        assert_eq!(
            state
                .windows
                .iter()
                .filter_map(|window| window.id)
                .collect::<Vec<_>>(),
            ids
        );

        state.select_tab(systems[0], SystemWindowTab::Mines);
        manager.apply(WindowCommand::Activate(ids[0])).unwrap();

        assert_eq!(
            state
                .windows
                .iter()
                .filter_map(|window| window.id)
                .collect::<Vec<_>>(),
            ids,
            "local content mutation must not emulate a raise"
        );
        assert_eq!(manager.begin_frame().paint_order(), &[ids[1], ids[0]]);
    }

    #[test]
    fn reference_rail_displays_manager_history_across_detail_families() {
        // FUN_00428b40/FUN_00429020: one oldest-first presentation history
        // supplies the twelve-slot rail independently of the content stores.
        let (world, systems) = fixture_world(4);
        let mut manager = WindowManager::new();
        let kinds = [
            WindowKind::System,
            WindowKind::Fleet,
            WindowKind::Defenses,
            WindowKind::Missions,
        ];
        let mut ids = Vec::new();
        for (kind, system) in kinds.into_iter().zip(systems.iter().copied()) {
            let subject = system_subject(&world, system).unwrap();
            let opened = manager
                .apply(WindowCommand::Open(
                    crate::windowing_policy::contextual_spec(
                        kind,
                        subject,
                        WindowOwner::StrategicRoot,
                    ),
                ))
                .unwrap();
            let WindowTransition::Opened { id } = opened.transition else {
                panic!("expected detail open");
            };
            manager.apply(WindowCommand::Minimize(id)).unwrap();
            ids.push(id);
        }

        let rail_layout = layout(CockpitFaction::Alliance, 2.0);
        let snapshot = manager.begin_frame();
        let slots = reference_rail_slots(&snapshot, &world, CockpitFaction::Alliance, rail_layout);
        assert_eq!(
            slots
                .iter()
                .map(|slot| (slot.id(), slot.kind(), slot.system()))
                .collect::<Vec<_>>(),
            ids.into_iter()
                .zip(kinds)
                .zip(systems.iter().copied())
                .map(|((id, kind), system)| (id, kind, system))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            slots[0].rect(),
            cockpit_rect(rail_layout, rail_slot_rect(CockpitFaction::Alliance, 0))
        );
        assert_eq!(
            slots[3].rect(),
            cockpit_rect(rail_layout, rail_slot_rect(CockpitFaction::Alliance, 3))
        );

        crate::test_support::clear_painted_trace();
        let ctx = egui::Context::default();
        let mut rail = RailTestState::new(Faction::Alliance);
        rail.router.begin_frame(1, &snapshot);
        let mut actions = Vec::new();
        let raw_input = || frame_input(Vec::new());
        let _ = ctx.run(raw_input(), |ctx| {
            let _ = rail.draw(
                ctx,
                &world,
                CockpitFaction::Alliance,
                rail_layout,
                &snapshot,
                &[],
            );
        });
        crate::test_support::clear_painted_trace();
        rail.router.begin_frame(2, &snapshot);
        let output = ctx.run(raw_input(), |ctx| {
            actions = rail.draw(
                ctx,
                &world,
                CockpitFaction::Alliance,
                rail_layout,
                &snapshot,
                &[],
            );
        });
        assert_empty!(actions);
        let painted = crate::test_support::take_painted_trace();
        let defenses_icon = crate::defenses_window::rail_icon(&world, systems[2]);
        assert!(
            painted
                .iter()
                .any(|(resource, _)| *resource == defenses_icon),
            "{painted:?}"
        );
        let missions_icon = crate::missions_window::rail_icon(
            &world,
            &rail.fog,
            &rail.missions,
            CockpitFaction::Alliance,
            systems[3],
        );
        assert!(
            painted
                .iter()
                .any(|(resource, _)| *resource == missions_icon),
            "{painted:?}"
        );
        let labels = output
            .shapes
            .into_iter()
            .filter_map(|shape| match shape.shape {
                egui::Shape::Text(text) => Some((
                    text.galley.text().to_owned(),
                    text.galley.job.sections[0].format.font_id.size,
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(
            labels.contains(&("System 0".to_owned(), 14.0)),
            "{labels:?}"
        );
        for name in ["System 1", "System 2", "System 3"] {
            assert!(labels.contains(&(name.to_owned(), 18.0)), "{labels:?}");
        }
    }

    struct ManagedSystemFixture {
        manager: WindowManager,
        state: SystemWindowState,
        ctx: egui::Context,
        router: FrameInputRouter,
        cache: BmpCache,
        movement: rebellion_core::movement::MovementState,
        fog: FogState,
        missions: MissionState,
        manufacturing: ManufacturingState,
        deliveries: DeliveryState,
        faction: CockpitFaction,
        layout: CockpitLayout,
    }

    impl ManagedSystemFixture {
        fn new(faction: CockpitFaction) -> Self {
            Self {
                manager: WindowManager::new(),
                state: SystemWindowState::default(),
                ctx: egui::Context::default(),
                router: FrameInputRouter::default(),
                cache: BmpCache::new(),
                movement: rebellion_core::movement::MovementState::default(),
                fog: FogState::new(faction.player()),
                missions: MissionState::new(),
                manufacturing: ManufacturingState::new(),
                deliveries: DeliveryState::new(),
                faction,
                layout: layout(faction, 2.0),
            }
        }

        fn open(&mut self, world: &GameWorld, system: SystemKey, position: (i16, i16)) -> WindowId {
            let prepared = self
                .state
                .prepare_open(world, system, position, self.faction, self.layout)
                .unwrap();
            commit_prepared_system_window(&mut self.manager, &mut self.state, prepared).0
        }

        fn open_active_rename(&mut self, world: &GameWorld, system: SystemKey) -> WindowId {
            let id = self.open(world, system, ORIGIN);
            self.manager.apply(WindowCommand::Activate(id)).unwrap();
            assert!(self
                .state
                .begin_band_rename(system, ProductionArea::Shipyard, "Death Star 1",));
            id
        }

        fn begin_frame(&mut self, frame: u64) -> RoutingSnapshot {
            let snapshot = self.manager.begin_frame();
            self.router.begin_frame(frame, &snapshot);
            snapshot
        }

        fn draw_started_frame(
            &mut self,
            world: &GameWorld,
            snapshot: &RoutingSnapshot,
            events: Vec<egui::Event>,
            pointer_events: &[SystemWindowInputEvent],
        ) -> Vec<SystemWindowAction> {
            self.draw_started_input(world, snapshot, frame_input(events), pointer_events)
        }

        fn draw_started_input(
            &mut self,
            world: &GameWorld,
            snapshot: &RoutingSnapshot,
            input: egui::RawInput,
            pointer_events: &[SystemWindowInputEvent],
        ) -> Vec<SystemWindowAction> {
            let mut actions = Vec::new();
            let _ = self.ctx.run(input, |ctx| {
                actions = draw_managed_system_windows(ManagedSystemDraw {
                    ctx,
                    world,
                    movement: &self.movement,
                    fog: &self.fog,
                    missions: &self.missions,
                    manufacturing: &self.manufacturing,
                    deliveries: &self.deliveries,
                    state: &mut self.state,
                    faction: self.faction,
                    layout: self.layout,
                    cache: &mut self.cache,
                    snapshot,
                    pointer_events,
                    router: &mut self.router,
                    front: true,
                    shown: &|_| true,
                });
            });
            actions
        }

        fn draw_frame(
            &mut self,
            world: &GameWorld,
            frame: u64,
            events: Vec<egui::Event>,
            pointer_events: &[SystemWindowInputEvent],
        ) -> Vec<SystemWindowAction> {
            let snapshot = self.begin_frame(frame);
            self.draw_started_frame(world, &snapshot, events, pointer_events)
        }

        fn draw_frames(
            &mut self,
            world: &GameWorld,
            frames: Vec<Vec<egui::Event>>,
        ) -> Vec<SystemWindowAction> {
            let mut actions = Vec::new();
            for (index, events) in frames.into_iter().enumerate() {
                let frame = u64::try_from(index + 1).unwrap();
                actions.extend(self.draw_frame(world, frame, events, &[]));
            }
            actions
        }
    }

    fn managed_rename_fixture(
        faction: CockpitFaction,
    ) -> (GameWorld, SystemKey, ManagedSystemFixture, WindowId) {
        let (world, systems) = controlled_fixture_world(1, faction);
        let system = systems[0];
        let mut fixture = ManagedSystemFixture::new(faction);
        let id = fixture.open_active_rename(&world, system);
        (world, system, fixture, id)
    }

    fn minimized_detail(
        world: &GameWorld,
        system: SystemKey,
        kind: WindowKind,
    ) -> (WindowManager, WindowId) {
        let mut manager = WindowManager::new();
        let opened = manager
            .apply(WindowCommand::Open(contextual_spec(
                kind,
                system_subject(world, system).unwrap(),
                WindowOwner::StrategicRoot,
            )))
            .unwrap();
        let WindowTransition::Opened { id } = opened.transition else {
            panic!("expected detail open");
        };
        manager.apply(WindowCommand::Minimize(id)).unwrap();
        (manager, id)
    }

    fn primary_pointer(point: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    fn system_input_event(
        id: InputEventId,
        point: egui::Pos2,
        phase: PointerPhase,
    ) -> SystemWindowInputEvent {
        SystemWindowInputEvent {
            id,
            point: (point.x, point.y),
            phase,
        }
    }

    fn rail_click_frames(point: egui::Pos2) -> Vec<(Vec<egui::Event>, PointerPhase)> {
        vec![
            (vec![egui::Event::PointerMoved(point)], PointerPhase::Hover),
            (vec![egui::Event::PointerMoved(point)], PointerPhase::Hover),
            (
                vec![primary_pointer(point, true)],
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            (
                vec![primary_pointer(point, false)],
                PointerPhase::Release(PointerButtonId::Primary),
            ),
        ]
    }

    #[derive(Clone, Copy)]
    struct RailTestFrame<'a> {
        faction: CockpitFaction,
        layout: CockpitLayout,
        snapshot: &'a RoutingSnapshot,
        event: InputEventId,
        point: egui::Pos2,
        phase: PointerPhase,
        overlay: Option<egui::Rect>,
    }

    #[derive(Clone, Copy)]
    struct RailClickTest {
        layout: CockpitLayout,
        point: egui::Pos2,
        expected_recipient: InputRecipient,
        overlay: Option<egui::Rect>,
    }

    fn run_alliance_rail_click(
        world: &GameWorld,
        test: RailClickTest,
        mut snapshot_for_frame: impl FnMut() -> RoutingSnapshot,
    ) -> Vec<SystemWindowAction> {
        let ctx = egui::Context::default();
        let mut rail = RailTestState::new(Faction::Alliance);
        let mut actions = Vec::new();
        for (index, (events, phase)) in rail_click_frames(test.point).into_iter().enumerate() {
            let frame = u64::try_from(index + 1).unwrap();
            let event = InputEventId { frame, ordinal: 0 };
            let snapshot = snapshot_for_frame();
            rail.router.begin_frame(frame, &snapshot);
            assert_eq!(
                rail.router
                    .collect_pointer(event, &std::collections::BTreeSet::new(), phase),
                test.expected_recipient
            );
            actions.extend(rail.run_frame(
                &ctx,
                frame_input(events),
                world,
                RailTestFrame {
                    faction: CockpitFaction::Alliance,
                    layout: test.layout,
                    snapshot: &snapshot,
                    event,
                    point: test.point,
                    phase,
                    overlay: test.overlay,
                },
            ));
        }
        actions
    }

    struct RailTestState {
        router: FrameInputRouter,
        cache: BmpCache,
        movement: rebellion_core::movement::MovementState,
        fog: FogState,
        missions: MissionState,
    }

    impl RailTestState {
        fn new(faction: Faction) -> Self {
            Self {
                router: FrameInputRouter::default(),
                cache: BmpCache::new(),
                movement: rebellion_core::movement::MovementState::default(),
                fog: FogState::new(faction),
                missions: MissionState::new(),
            }
        }

        fn draw(
            &mut self,
            ctx: &egui::Context,
            world: &GameWorld,
            faction: CockpitFaction,
            layout: CockpitLayout,
            snapshot: &RoutingSnapshot,
            pointer_events: &[SystemWindowInputEvent],
        ) -> Vec<SystemWindowAction> {
            draw_reference_rail(
                ctx,
                world,
                &self.movement,
                &self.fog,
                &self.missions,
                faction,
                layout,
                &mut self.cache,
                snapshot,
                pointer_events,
                &mut self.router,
            )
        }

        fn run_frame(
            &mut self,
            ctx: &egui::Context,
            input: egui::RawInput,
            world: &GameWorld,
            frame: RailTestFrame<'_>,
        ) -> Vec<SystemWindowAction> {
            let mut actions = Vec::new();
            let _ = ctx.run(input, |ctx| {
                if let Some(rect) = frame.overlay {
                    egui::Area::new(egui::Id::new("root-cockpit-overlay"))
                        .fixed_pos(rect.min)
                        .order(egui::Order::Foreground)
                        .show(ctx, |ui| {
                            let _ = ui.allocate_exact_size(rect.size(), egui::Sense::hover());
                        });
                }
                actions = self.draw(
                    ctx,
                    world,
                    frame.faction,
                    frame.layout,
                    frame.snapshot,
                    &[system_input_event(frame.event, frame.point, frame.phase)],
                );
            });
            actions
        }
    }

    #[test]
    fn reference_rail_restore_removes_entry_without_recreating_content() {
        let (world, systems) = fixture_world(1);
        let system = systems[0];
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let (mut manager, mut state, id) = managed_test_window(&world, system, layout);
        manager.apply(WindowCommand::Minimize(id)).unwrap();
        assert_eq!(manager.begin_frame().presentation_history(), [id]);

        let point = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 0)).center();
        let mut frames = rail_click_frames(point)
            .into_iter()
            .map(|(events, phase)| (events, phase, false))
            .collect::<Vec<_>>();
        frames[3].2 = true;
        frames.extend(
            rail_click_frames(point)
                .into_iter()
                .map(|(events, phase)| (events, phase, false)),
        );
        let ctx = egui::Context::default();
        let mut router = FrameInputRouter::default();
        let mut actions = Vec::new();
        let mut cache = BmpCache::new();
        for (index, (events, phase, preclaimed)) in frames.into_iter().enumerate() {
            let frame = u64::try_from(index + 1).unwrap();
            let event = InputEventId { frame, ordinal: 0 };
            let snapshot = manager.begin_frame();
            router.begin_frame(frame, &snapshot);
            assert_eq!(
                router.collect_pointer(event, &std::collections::BTreeSet::new(), phase),
                InputRecipient::Root
            );
            if preclaimed {
                assert!(router.claim_root(event));
            }
            let input = frame_input(events);
            let movement = rebellion_core::movement::MovementState::default();
            let fog = FogState::new(Faction::Alliance);
            let missions = MissionState::new();
            let manufacturing = ManufacturingState::new();
            let deliveries = DeliveryState::new();
            let mut frame_actions = Vec::new();
            let _ = ctx.run(input, |ctx| {
                frame_actions = draw_managed_system_windows(ManagedSystemDraw {
                    ctx,
                    world: &world,
                    movement: &movement,
                    fog: &fog,
                    missions: &missions,
                    manufacturing: &manufacturing,
                    deliveries: &deliveries,
                    state: &mut state,
                    faction: CockpitFaction::Alliance,
                    layout,
                    cache: &mut cache,
                    snapshot: &snapshot,
                    pointer_events: &[system_input_event(event, point, phase)],
                    router: &mut router,
                    front: true,
                    shown: &|_| true,
                });
            });
            actions.extend(frame_actions);
        }

        assert_eq!(
            actions,
            [SystemWindowAction::RestoreReferenceWindow(id)],
            "one physical release may restore at most once"
        );
        let restored = manager.apply(WindowCommand::Restore(id)).unwrap();
        assert_eq!(restored.transition, WindowTransition::Restored { id });
        assert_empty!(manager.begin_frame().presentation_history());
        assert_eq!(manager.begin_frame().paint_order(), [id]);
        assert_eq!(state.window_id(system), Some(id));
        assert_eq!(state.window_count(), 1);
    }

    #[test]
    fn reference_rail_restores_when_a_root_overlay_shares_its_slot() {
        let (world, systems) = fixture_world(1);
        let system = systems[0];
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let (manager, id) = minimized_detail(&world, system, WindowKind::Missions);

        let snapshot = manager.begin_frame();
        let slot = reference_rail_slots(&snapshot, &world, CockpitFaction::Alliance, layout)[0];
        let point = slot.rect().center();
        let actions = run_alliance_rail_click(
            &world,
            RailClickTest {
                layout,
                point,
                expected_recipient: InputRecipient::Root,
                overlay: Some(slot.rect()),
            },
            || snapshot.clone(),
        );

        assert_eq!(actions, [SystemWindowAction::RestoreReferenceWindow(id)]);
    }

    #[test]
    fn reference_rail_restore_claims_its_own_release_in_multi_release_frame() {
        let (world, systems) = fixture_world(1);
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let (manager, id) = minimized_detail(&world, systems[0], WindowKind::System);

        let rail_point = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 0)).center();
        let unrelated_point = egui::pos2(1300.0, 900.0);
        let pointer_button = primary_pointer;
        let snapshot = manager.begin_frame();
        let ctx = egui::Context::default();
        let mut rail = RailTestState::new(Faction::Alliance);

        for frame in 1..=2 {
            rail.router.begin_frame(frame, &snapshot);
            let _ = ctx.run(
                frame_input(vec![egui::Event::PointerMoved(rail_point)]),
                |ctx| {
                    assert_empty!(rail.draw(
                        ctx,
                        &world,
                        CockpitFaction::Alliance,
                        layout,
                        &snapshot,
                        &[],
                    ));
                },
            );
        }

        let press = InputEventId {
            frame: 3,
            ordinal: 0,
        };
        rail.router.begin_frame(3, &snapshot);
        assert_eq!(
            rail.router.collect_pointer(
                press,
                &std::collections::BTreeSet::new(),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Root
        );
        let _ = ctx.run(frame_input(vec![pointer_button(rail_point, true)]), |ctx| {
            assert_empty!(rail.draw(
                ctx,
                &world,
                CockpitFaction::Alliance,
                layout,
                &snapshot,
                &[SystemWindowInputEvent {
                    id: press,
                    point: (rail_point.x, rail_point.y),
                    phase: PointerPhase::Press(PointerButtonId::Primary),
                }],
            ));
        });

        let causal_release = InputEventId {
            frame: 4,
            ordinal: 0,
        };
        let unrelated_press = InputEventId {
            frame: 4,
            ordinal: 1,
        };
        let unrelated_release = InputEventId {
            frame: 4,
            ordinal: 2,
        };
        rail.router.begin_frame(4, &snapshot);
        for (event, phase) in [
            (
                causal_release,
                PointerPhase::Release(PointerButtonId::Primary),
            ),
            (
                unrelated_press,
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            (
                unrelated_release,
                PointerPhase::Release(PointerButtonId::Primary),
            ),
        ] {
            assert_eq!(
                rail.router
                    .collect_pointer(event, &std::collections::BTreeSet::new(), phase),
                InputRecipient::Root
            );
        }
        let mut actions = Vec::new();
        let _ = ctx.run(
            frame_input(vec![pointer_button(rail_point, false)]),
            |ctx| {
                actions = rail.draw(
                    ctx,
                    &world,
                    CockpitFaction::Alliance,
                    layout,
                    &snapshot,
                    &[
                        SystemWindowInputEvent {
                            id: causal_release,
                            point: (rail_point.x, rail_point.y),
                            phase: PointerPhase::Release(PointerButtonId::Primary),
                        },
                        SystemWindowInputEvent {
                            id: unrelated_press,
                            point: (unrelated_point.x, unrelated_point.y),
                            phase: PointerPhase::Press(PointerButtonId::Primary),
                        },
                        SystemWindowInputEvent {
                            id: unrelated_release,
                            point: (unrelated_point.x, unrelated_point.y),
                            phase: PointerPhase::Release(PointerButtonId::Primary),
                        },
                    ],
                );
            },
        );

        assert_eq!(actions, [SystemWindowAction::RestoreReferenceWindow(id)]);
        assert!(
            !rail.router.claim_root(causal_release),
            "the rail must claim the release inside its exact slot"
        );
        assert!(
            rail.router.claim_root(unrelated_release),
            "an unrelated root release must remain available"
        );
    }

    #[test]
    fn reference_rail_modal_blocks_restore_and_hover_actions() {
        let (world, systems) = fixture_world(1);
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let (mut manager, id) = minimized_detail(&world, systems[0], WindowKind::System);
        manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::singleton_spec(
                    WindowKind::SystemFinder,
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let point = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 0)).center();
        let actions = run_alliance_rail_click(
            &world,
            RailClickTest {
                layout,
                point,
                expected_recipient: InputRecipient::Blocked,
                overlay: None,
            },
            || manager.begin_frame(),
        );

        assert!(actions.is_empty(), "modal rail input leaked: {actions:?}");
        assert_eq!(manager.begin_frame().presentation_history(), [id]);
    }

    #[test]
    fn reference_rail_missing_art_does_not_delete_registered_detail() {
        let (world, systems) = fixture_world(1);
        let layout = layout(CockpitFaction::Alliance, 1.0);
        let (manager, id) = minimized_detail(&world, systems[0], WindowKind::Fleet);
        let snapshot = manager.begin_frame();
        let ctx = egui::Context::default();
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &snapshot);
        let mut cache = BmpCache::new();
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 500.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                assert_empty!(draw_reference_rail(
                    ctx,
                    &world,
                    &rebellion_core::movement::MovementState::default(),
                    &FogState::new(Faction::Alliance),
                    &MissionState::new(),
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                    &snapshot,
                    &[],
                    &mut router,
                ));
            },
        );

        let after = manager.begin_frame();
        assert_eq!(after.presentation_history(), [id]);
        assert!(after.key(id).is_some());
        assert_eq!(
            reference_rail_slots(&after, &world, CockpitFaction::Alliance, layout)[0].id(),
            id
        );
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
            crate::test_support::clear_painted_trace();
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
            run.painted = crate::test_support::take_painted_trace();
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
            run.texts = crate::test_support::rendered_texts(output)
                .into_iter()
                .map(|text| (text.text, text.pos, text.size.x, text.font))
                .collect();
        }
        run
    }

    const ORIGIN: (i16, i16) = (100, 60);

    /// A logical point in the window opened at `ORIGIN` on the scale 2
    /// layout.
    fn at(x: f32, y: f32) -> egui::Pos2 {
        crate::test_support::window_point(layout(CockpitFaction::Alliance, 2.0), ORIGIN, (x, y))
    }

    fn opened(world: &GameWorld, system: SystemKey) -> SystemWindowState {
        opened_as(world, system, CockpitFaction::Alliance)
    }

    fn opened_as(
        world: &GameWorld,
        system: SystemKey,
        faction: CockpitFaction,
    ) -> SystemWindowState {
        let mut state = SystemWindowState::default();
        assert!(state.open(world, system, ORIGIN, faction, layout(faction, 2.0)));
        state
    }

    fn text<'a>(run: &'a Run, words: &str) -> &'a RailText {
        run.texts
            .iter()
            .find(|value| value.0 == words)
            .unwrap_or_else(|| panic!("no {words:?} in {:?}", run.texts))
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

    fn run_rename_key(
        world: &GameWorld,
        system: SystemKey,
        pressed: egui::Key,
    ) -> (Run, SystemWindowState) {
        let mut state = renaming(world, system);
        let run = run_with(
            world,
            &ManufacturingState::new(),
            &mut state,
            after_opening(vec![
                vec![egui::Event::Text("Tarkin".into())],
                vec![key(pressed)],
            ]),
        );
        (run, state)
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
        let (run, state) = run_rename_key(&world, systems[0], egui::Key::Enter);
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
        let (run, state) = run_rename_key(&world, systems[0], egui::Key::Escape);
        assert_eq!(renamed(&run.actions), []);
        assert_empty!(state.windows);
        assert!(!state.band_rename_open());
    }

    #[test]
    fn manager_owned_band_rename_escape_requests_and_commits_exact_close() {
        // The field passes Escape to the containing System window
        // (FUN_00605160 -> FUN_00458980). Production content must keep its
        // registry identity until the manager accepts that exact Close.
        let (world, systems) = fixture_world(1);
        let system = systems[0];
        let mut fixture = ManagedSystemFixture::new(CockpitFaction::Alliance);
        let id = fixture.open_active_rename(&world, system);
        let actions = fixture.draw_frames(
            &world,
            after_opening(vec![
                vec![egui::Event::Text("Tarkin".into())],
                vec![key(egui::Key::Escape)],
            ]),
        );

        assert_eq!(actions, [SystemWindowAction::Close(id)]);
        assert!(fixture.state.contains_window(id));
        assert!(!fixture.state.band_rename_open());
        let closed = fixture.manager.apply(WindowCommand::Close(id)).unwrap();
        let WindowTransition::Closed { closed, .. } = closed.transition else {
            panic!("expected exact System close receipt");
        };
        fixture.state.remove_content(&closed);
        assert!(!fixture.state.contains_window(id));
    }

    #[test]
    fn the_press_that_ends_a_band_rename_also_selects_the_pressed_band_for_both_factions() {
        // FUN_004534f0 0x201/0x204 first finishes the edit (FUN_00458780), posts
        // 0x467, then hit-tests the bands with the same press and selects the
        // band it hits (param_1[0x92]), else passes the press on to
        // FUN_004ac3a0 (FUN_004534f0.c:57-100). The release comes in a later
        // frame, as in an ordinary click.
        for (faction, button, egui_button) in [
            (
                CockpitFaction::Alliance,
                PointerButtonId::Primary,
                egui::PointerButton::Primary,
            ),
            (
                CockpitFaction::Empire,
                PointerButtonId::Secondary,
                egui::PointerButton::Secondary,
            ),
        ] {
            let (world, system, mut fixture, id) = managed_rename_fixture(faction);
            assert_ne!(BAND_AREAS[1], ProductionArea::Shipyard);
            let mut actions = Vec::new();

            let (bx, by, bw, bh) = BAND_RECTS[1];
            let point = at(bx + bw / 2.0, by + bh / 2.0);
            let mut frames = after_opening(vec![vec![egui::Event::Text("Tarkin".into())]]);
            frames.push(vec![egui::Event::PointerMoved(point)]);
            frames.push(vec![egui::Event::PointerButton {
                pos: point,
                button: egui_button,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            }]);
            let press_frame = frames.len();
            for (index, events) in frames.into_iter().enumerate() {
                let frame = u64::try_from(index + 1).unwrap();
                let snapshot = fixture.begin_frame(frame);
                let routed = if index + 1 == press_frame {
                    let press = InputEventId { frame, ordinal: 0 };
                    let phase = PointerPhase::Press(button);
                    assert_eq!(
                        fixture.router.collect_pointer(
                            press,
                            &std::collections::BTreeSet::from([id]),
                            phase,
                        ),
                        InputRecipient::Window(id)
                    );
                    vec![SystemWindowInputEvent {
                        id: press,
                        point: (point.x, point.y),
                        phase,
                    }]
                } else {
                    Vec::new()
                };
                actions.extend(fixture.draw_started_frame(&world, &snapshot, events, &routed));
            }

            assert!(
                actions.contains(&SystemWindowAction::RenameProduct {
                    system,
                    area: ProductionArea::Shipyard,
                    name: "Tarkin".into(),
                }),
                "the press ends the edit for {faction:?}: {actions:?}"
            );
            assert!(!fixture.state.band_rename_open(), "{faction:?}");
            assert_eq!(
                fixture.state.selection(system).and_then(|(_, band)| band),
                Some(BAND_AREAS[1]),
                "FUN_004534f0 selects the pressed band with the same press for {faction:?}"
            );
        }
    }

    #[test]
    fn a_band_rename_names_the_window_it_edits() {
        // The rename field belongs to its own System window (window +0x170,
        // FUN_00458b50): the app routes that window's Escape to it, and only
        // that window's.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let (world, systems) = controlled_fixture_world(2, faction);
            let mut fixture = ManagedSystemFixture::new(faction);
            let first = fixture.open(&world, systems[0], ORIGIN);
            let second = fixture.open(&world, systems[1], (ORIGIN.0 + 200, ORIGIN.1));
            assert_eq!(fixture.state.band_rename_window(), None, "{faction:?}");
            assert!(fixture.state.begin_band_rename(
                systems[1],
                ProductionArea::Shipyard,
                "Death Star 1",
            ));
            assert_eq!(
                fixture.state.band_rename_window(),
                Some(second),
                "{faction:?}"
            );
            assert_ne!(
                fixture.state.band_rename_window(),
                Some(first),
                "{faction:?}"
            );
        }
    }

    #[test]
    fn band_rename_escape_leaves_the_window_open_while_another_window_has_the_focus() {
        // Escape goes to the focused window: the field passes it to its own
        // System window (FUN_00605160 -> FUN_00458980) only while that window
        // holds the keyboard focus (gid-legend.md, "Escape and the keyboard
        // focus"). A click on another window moves the focus there first.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let (world, systems) = controlled_fixture_world(2, faction);
            let mut fixture = ManagedSystemFixture::new(faction);
            let renaming = fixture.open_active_rename(&world, systems[0]);
            let other = fixture.open(&world, systems[1], (ORIGIN.0 + 200, ORIGIN.1));
            let mut actions = Vec::new();
            let frames = after_opening(vec![
                vec![egui::Event::Text("Tarkin".into())],
                vec![key(egui::Key::Escape)],
            ]);
            let escape_frame = frames.len();
            for (index, events) in frames.into_iter().enumerate() {
                if index + 1 == escape_frame {
                    fixture
                        .manager
                        .apply(WindowCommand::Activate(other))
                        .unwrap();
                }
                let frame = u64::try_from(index + 1).unwrap();
                actions.extend(fixture.draw_frame(&world, frame, events, &[]));
            }

            assert!(
                !actions.contains(&SystemWindowAction::Close(renaming)),
                "an unfocused window's rename closed it on Escape for {faction:?}: {actions:?}"
            );
            assert!(fixture.state.contains_window(renaming), "{faction:?}");
        }
    }

    #[test]
    fn a_managed_press_in_the_owning_system_commits_the_band_rename_once_for_both_factions() {
        // FUN_004534f0 routes the owning window's WM_LBUTTONDOWN (0x201)
        // and WM_RBUTTONDOWN (0x204) through FUN_00458780 to finish the
        // edit. The production-routed event, rather than an unowned egui
        // pointer copy, is the authority for that commit.
        for (faction, button) in [
            (CockpitFaction::Alliance, PointerButtonId::Primary),
            (CockpitFaction::Empire, PointerButtonId::Secondary),
        ] {
            let (world, system, mut fixture, id) = managed_rename_fixture(faction);
            let mut actions = fixture.draw_frames(
                &world,
                after_opening(vec![vec![egui::Event::Text("Tarkin".into())]]),
            );

            let point = at(20.0, 100.0);
            let raw_button = match button {
                PointerButtonId::Primary => egui::PointerButton::Primary,
                PointerButtonId::Secondary => egui::PointerButton::Secondary,
                PointerButtonId::Middle | PointerButtonId::Extra(_) => unreachable!(),
            };
            let snapshot = fixture.begin_frame(4);
            let input = crate::test_support::pointer_raw_input(
                point,
                vec![
                    crate::test_support::pointer_button_with(point, raw_button, true),
                    crate::test_support::pointer_button_with(point, raw_button, false),
                ],
                egui::vec2(1400.0, 1000.0),
            );
            actions.extend(fixture.draw_started_input(&world, &snapshot, input, &[]));
            assert!(actions.is_empty(), "an unrouted raw press must not publish");
            assert!(fixture.state.band_rename_open());
            assert!(fixture.state.contains_window(id));

            let frame = 5;
            let press = InputEventId { frame, ordinal: 0 };
            let phase = PointerPhase::Press(button);
            let snapshot = fixture.begin_frame(frame);
            assert_eq!(
                fixture.router.collect_pointer(
                    press,
                    &std::collections::BTreeSet::from([id]),
                    phase,
                ),
                InputRecipient::Window(id)
            );
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1000.0),
                )),
                ..egui::RawInput::default()
            };
            actions.extend(fixture.draw_started_input(
                &world,
                &snapshot,
                input,
                &[SystemWindowInputEvent {
                    id: press,
                    point: (point.x, point.y),
                    phase,
                }],
            ));

            assert_eq!(
                actions,
                [SystemWindowAction::RenameProduct {
                    system,
                    area: ProductionArea::Shipyard,
                    name: "Tarkin".into(),
                }]
            );
            assert!(fixture.state.contains_window(id));
            assert!(!fixture.state.band_rename_open());
            assert!(fixture.manager.begin_frame().key(id).is_some());
            assert!(
                !fixture.router.claim(press, InputRecipient::Window(id)),
                "the rename owner must consume the exact admitted press"
            );
        }
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
            let saved = state.saved_windows(&world);
            let placed: Vec<_> = state
                .windows
                .iter()
                .map(|window| (window.system, window.logical_position))
                .collect();

            let mut restored = SystemWindowState::default();
            crate::test_support::assert_saved_windows_restore(&saved, faction, |window| {
                restored.restore_saved(&world, faction, window)
            });

            let back: Vec<_> = restored
                .windows
                .iter()
                .map(|window| (window.system, window.logical_position))
                .collect();
            assert_eq!(back, placed, "{faction:?}");
            assert!(restored
                .windows
                .iter()
                .all(|window| window.tab == SystemWindowTab::Overview));
            assert_eq!(restored.faction, faction);
        }
    }

    #[test]
    fn saved_minimized_content_round_trips_without_renderer_z_serials() {
        let (world, systems) = fixture_world(3);
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut state = SystemWindowState::default();
            let layout = layout(faction, 1.0);
            state.open(&world, systems[0], (120, 70), faction, layout);
            assert!(state.minimize(systems[0]));
            let opened_first = state.windows.is_empty();
            assert!(opened_first);
            state.minimize_defenses_window_at(systems[1], (130, 75));
            state.minimize_missions_window_at(systems[2], (140, 85));
            let saved = state.saved_rail(&world, |_| None);
            assert!(saved.iter().all(|window| !window.shown));
            assert_eq!(
                saved.iter().map(|window| window.kind).collect::<Vec<_>>(),
                [
                    SavedWindowKind::System {
                        system: DatId::new(100)
                    },
                    SavedWindowKind::Defenses {
                        system: DatId::new(101)
                    },
                    SavedWindowKind::Missions {
                        system: DatId::new(102)
                    },
                ],
                "{faction:?}"
            );

            let mut restored = SystemWindowState::default();
            crate::test_support::assert_saved_windows_restore(&saved, faction, |window| {
                restored.restore_saved(&world, faction, window)
            });

            assert!(restored.windows.is_empty(), "{faction:?}");
            let back: Vec<_> = restored.saved_rail(&world, |_| None).into_iter().collect();
            assert_eq!(back, saved, "{faction:?}");
        }
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
        state.minimize_defenses_window_at(systems[1], (130, 75));
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
            state.saved_windows(&world)[0],
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
            state.saved_rail(&world, |id| (id == fleet).then_some(0x68)),
            [SavedWindow {
                kind: SavedWindowKind::Fleet {
                    system: DatId::new(100),
                    tab: 0x68,
                },
                position: (160, 90),
                shown: false,
            }]
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
        assert_empty!(other.windows);
        assert_empty!(other.rail);
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
            .map(|window| window.kind)
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
        state.prepare_faction(CockpitFaction::Empire);
        assert_eq!(state.window_count(), 0);
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
            (strategic_window_common::SECTOR_ART.0, at(3.0, 3.0)),
            (strategic_window_common::MINIMIZE_ART.0, at(195.0, 3.0)),
            (strategic_window_common::CLOSE_ART.0, at(209.0, 3.0)),
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
        let (world, system, manufacturing) = capital_ship_build("Corellian Corvette", 2);
        let mut state = opened(&world, system);
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
        let bmp = crate::test_support::bitmap_fixture(width, height, stride, offset, 8, 3);
        let mini = crate::panels::fleets::capital_ship_mini_id(DatId::new(0x1400_0040)).unwrap();
        std::fs::write(dir.join(format!("{}.bmp", mini - 0x4000)), &bmp).unwrap();
        let mut cache = BmpCache::new();
        cache.set_base_path(&root);

        let (world, system, manufacturing) = capital_ship_build("Corellian Corvette", 2);
        let mut state = opened(&world, system);
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
            owner: None,
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
            let mut state = opened_as(&world, systems[0], faction);
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
        let mut state = opened_as(world, system, faction);
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
            let mut state = opened_as(&world, systems[0], faction);
            let _ = run_as(
                &world,
                &manufacturing,
                &mut state,
                drag(at(130.0, 9.0), at(150.0, 59.0)),
                faction,
            );
            let dragged = state.windows[0].logical_position;
            let mut restored = SystemWindowState::default();
            for window in state.saved_windows(&world) {
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
        assert_eq!(state.window_count(), 0);

        let mut state = opened(&world, systems[0]);
        let _ = run_with(&world, &manufacturing, &mut state, click(at(215.0, 9.0)));
        assert_eq!(state.window_count(), 0);
    }

    #[test]
    fn the_other_sides_production_shows_only_where_its_contents_show() {
        // hyp: as the Defenses window's pages, the other side's facilities
        // and production show only where the player sees its objects.
        let (mut world, system, manufacturing) = capital_ship_build("Star Destroyer", 1);
        world.systems[system].control = ControlKind::Controlled(Faction::Empire);
        let mut state = opened(&world, system);
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
    fn fixture_defenses_rail_projection_keeps_its_own_slot() {
        // FUN_004aa4a0: a Defenses window minimizes (0x466) to its own rail
        // entry beside the same system's Fleet window, one per window.
        let (_world, systems) = fixture_world(1);
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
        assert_eq!(state.rail_slot_screen_rect(layout, 1), Some(slot));
        assert_eq!(state.rail[1].system(), systems[0]);
        // Painting and restore routing are exercised with real manager IDs
        // by `reference_rail_displays_manager_history_across_detail_families`.
    }

    #[test]
    fn fixture_missions_rail_projection_keeps_its_own_slot() {
        // FUN_004a21c0: a Missions window minimizes (0x466) to its own rail
        // entry beside the same system's Defenses window, one per window;
        // with no mission there its icon is side 3's 11541.
        let (_world, systems) = fixture_world(1);
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
        assert_eq!(state.rail_slot_screen_rect(layout, 1), Some(slot));
        assert_eq!(state.rail[1].system(), systems[0]);
        // The manager-owned renderer test covers icon and activation behavior.
    }

    #[test]
    fn the_rail_holds_a_minimized_fleet_window_and_no_other_for_either_side() {
        // FUN_004a76e0: a minimized Fleet window is the rail's own entry; a
        // shown Fleet window, an unknown one and a minimized System window
        // are not Fleet entries.
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let (world, systems) = fixture_world(2);
            let mut state = SystemWindowState::default();
            let fleets = fleet_window_ids(&[201, 202, 203]);
            let [minimized, shown, unknown] = fleets.as_slice() else {
                panic!()
            };
            // The restore takes the side first: a new side clears the rail
            // (`prepare_faction`).
            assert!(state.restore_saved(
                &world,
                faction,
                &SavedWindow {
                    kind: SavedWindowKind::System {
                        system: world.systems[systems[1]].dat_id,
                    },
                    position: (60, 40),
                    shown: false,
                },
            ));
            state.minimize_fleet_window(*minimized, systems[0], (100, 80));
            assert_eq!(state.rail_count(), 2, "{faction:?}");
            assert!(state.rail_holds_fleet(*minimized), "{faction:?}");
            assert!(!state.rail_holds_fleet(*shown), "{faction:?}");
            assert!(!state.rail_holds_fleet(*unknown), "{faction:?}");
        }
    }

    #[test]
    fn fixture_fleet_rail_projection_keeps_each_window_slot() {
        // FUN_004a76e0: a Fleet window minimizes to the rail as its own
        // entry; the rail holds one entry per window.
        let (world, systems) = fixture_world(2);
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let mut state = SystemWindowState::default();
        let (fleet0, fleet1) = fleet_window_pair([201, 202]);
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
        assert_eq!(state.rail_slot_screen_rect(layout, 0), Some(slot));
        let second = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 1));
        assert_eq!(state.rail_slot_screen_rect(layout, 1), Some(second));

        // Identity-based restoration is covered by the manager-owned path;
        // this content projection retains the slot's subject and position.
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
        let (fleet0, fleet1) = fleet_window_pair([301, 302]);
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
            assert_eq!(state.children().count(), 2, "{faction:?}");
            assert!(state.raise(first), "{faction:?}");
            assert_eq!(state.windows.len(), 2, "{faction:?}");
            assert!(!state.raise(missing), "{faction:?}");
        }
    }
}
