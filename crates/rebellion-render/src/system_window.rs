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
use rebellion_core::missions::MissionState;
use rebellion_core::world::{ControlKind, GameWorld};
use rebellion_windowing::{
    ContentReport, ContentStatus, RoutingSnapshot, SubjectId, WindowId, WindowKind, WindowOwner,
    WindowSpec, WindowTransition,
};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::{control_side, paint_native};
use crate::manufacturing_window::{
    band, band_strip, contents_visible, item_frame, page_cells, yard_count, yard_page,
    FacilityItem, FacilityPage, BAND_AREAS, BAND_DESTINATION, BAND_FRAME, BAND_MINI, BAND_RECTS,
    BAND_STATUS, BAND_TITLE, BAND_UNITS, CELL, COLUMNS, COUNT_ORIGINS, LIST, PROGRESS_RECTS,
    YARD_COLUMN,
};
use crate::object_menu::MenuObject;
use crate::windowing_input::{
    FrameInputRouter, InputEventId, InputRecipient, PointerButtonId, PointerPhase,
};
use crate::windowing_policy::{contextual_spec, system_subject};

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

/// Validated System content held aside until the manager accepts its stable
/// `(System, system DAT ID)` identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreparedSystemWindow {
    spec: WindowSpec,
    faction: CockpitFaction,
    system: SystemKey,
    logical_position: (i16, i16),
}

impl PreparedSystemWindow {
    /// Complete manager declaration for this content candidate.
    #[must_use]
    pub const fn spec(&self) -> WindowSpec {
        self.spec
    }

    /// Runtime system identity retained beside the stable DAT subject.
    #[must_use]
    pub const fn system(&self) -> SystemKey {
        self.system
    }
}

/// Mutable System-window content. Presentation history belongs exclusively to
/// the shared manager and is projected by [`reference_rail_slots`].
#[derive(Debug)]
pub struct SystemWindowState {
    faction: CockpitFaction,
    windows: Vec<OpenSystemWindow>,
}

impl Default for SystemWindowState {
    fn default() -> Self {
        Self {
            faction: CockpitFaction::Alliance,
            windows: Vec::new(),
        }
    }
}

impl SystemWindowState {
    /// Whether any window of this kind is open.
    #[must_use]
    pub fn any_open(&self) -> bool {
        !self.windows.is_empty()
    }

    /// Egui layer attached to one manager-issued System identity.
    #[must_use]
    pub fn managed_layer_id(&self, id: WindowId) -> Option<egui::LayerId> {
        self.windows
            .iter()
            .find(|window| window.id == Some(id))
            .copied()
            .map(|window| egui::LayerId::new(egui::Order::Foreground, window_area_id(window)))
    }

    /// Validates and prepares System content without mutating the live store.
    /// The stable subject is the system's original DAT identity.
    #[must_use]
    pub fn prepare_open(
        &self,
        world: &GameWorld,
        system: SystemKey,
        logical_position: (i16, i16),
        faction: CockpitFaction,
        layout: CockpitLayout,
    ) -> Option<PreparedSystemWindow> {
        let subject = system_subject(world, system)?;
        Some(PreparedSystemWindow {
            spec: contextual_spec(WindowKind::System, subject, WindowOwner::StrategicRoot),
            faction,
            system,
            logical_position: clamp_to_galaxy(logical_position, layout),
        })
    }

    /// Publishes new System content, or confirms the exact preserved content,
    /// only after a successful manager open commit.
    pub fn commit_open(
        &mut self,
        prepared: PreparedSystemWindow,
        transition: &WindowTransition,
    ) -> Option<WindowId> {
        let subject = prepared.spec.key.subject?;
        match *transition {
            WindowTransition::Opened { id } => {
                if self.windows.iter().any(|window| {
                    window.id == Some(id)
                        || window.subject == Some(subject)
                        || window.system == prepared.system
                }) {
                    return None;
                }
                if self.windows.is_empty() {
                    self.faction = prepared.faction;
                } else if self.faction != prepared.faction {
                    return None;
                }
                self.windows.push(OpenSystemWindow {
                    id: Some(id),
                    subject: Some(subject),
                    system: prepared.system,
                    logical_position: prepared.logical_position,
                    tab: SystemWindowTab::Overview,
                    selected_item: None,
                    selected_band: None,
                });
                Some(id)
            }
            WindowTransition::Existing { id, .. } => {
                if self.faction != prepared.faction {
                    return None;
                }
                self.windows
                    .iter()
                    .any(|window| {
                        window.id == Some(id)
                            && window.subject == Some(subject)
                            && window.system == prepared.system
                    })
                    .then_some(id)
            }
            _ => None,
        }
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
        let logical_position = clamp_to_galaxy(logical_position, layout);
        self.windows.push(OpenSystemWindow {
            id: None,
            subject: None,
            system,
            logical_position,
            tab: SystemWindowTab::Overview,
            selected_item: None,
            selected_band: None,
        });
        true
    }

    /// Exact manager identity attached to `system`'s committed content.
    #[must_use]
    pub fn window_id(&self, system: SystemKey) -> Option<WindowId> {
        self.windows
            .iter()
            .find(|window| window.system == system)
            .and_then(|window| window.id)
    }

    /// Whether `system`'s exact content identity participates in this frozen
    /// frame. Direct fixture content has no manager identity and stays visible.
    #[must_use]
    pub fn window_is_visible(&self, snapshot: &RoutingSnapshot, system: SystemKey) -> bool {
        self.windows.iter().any(|window| {
            window.system == system && window.id.is_none_or(|id| snapshot.is_visible(id))
        })
    }

    /// Whether exact manager-owned System content remains attached.
    #[must_use]
    pub fn contains_window(&self, id: WindowId) -> bool {
        self.windows.iter().any(|window| window.id == Some(id))
    }

    /// Removes only content named by manager teardown receipts.
    pub fn remove_content(&mut self, removed: &[WindowId]) {
        self.windows
            .retain(|window| window.id.is_none_or(|id| !removed.contains(&id)));
    }

    /// Reports every active System identity in the frozen snapshot, including
    /// minimized history entries whose content remains attached.
    #[must_use]
    pub fn content_reports(
        &self,
        snapshot: &RoutingSnapshot,
        world: &GameWorld,
    ) -> Vec<ContentReport> {
        snapshot
            .paint_order()
            .iter()
            .chain(snapshot.presentation_history())
            .copied()
            .filter(|&id| {
                snapshot
                    .key(id)
                    .is_some_and(|key| key.kind == WindowKind::System)
            })
            .map(|id| {
                let ready = self.windows.iter().any(|window| {
                    window.id == Some(id)
                        && world.systems.contains_key(window.system)
                        && system_subject(world, window.system) == window.subject
                });
                ContentReport {
                    window: id,
                    status: if ready {
                        ContentStatus::Ready
                    } else {
                        ContentStatus::Absent
                    },
                }
            })
            .collect()
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
        self.windows.iter().any(|window| {
            let rect = window_screen_rect(*window, layout);
            point.0 >= rect.min.x
                && point.0 < rect.max.x
                && point.1 >= rect.min.y
                && point.1 < rect.max.y
        })
    }

    /// Whether visible System content covers `point`. Minimized manager
    /// content remains attached but cannot block the strategic root.
    #[must_use]
    pub fn contains_visible_screen_point(
        &self,
        snapshot: &RoutingSnapshot,
        layout: CockpitLayout,
        point: (f32, f32),
    ) -> bool {
        self.windows.iter().any(|window| {
            window.id.is_none_or(|id| snapshot.is_visible(id))
                && rect_contains(
                    window_screen_rect(*window, layout),
                    egui::pos2(point.0, point.1),
                )
        })
    }

    /// Exact visible manager identity whose System rectangle covers `point`
    /// when egui reports that same content layer as topmost.
    #[must_use]
    pub fn hit_window_ids(
        &self,
        snapshot: &RoutingSnapshot,
        layout: CockpitLayout,
        point: (f32, f32),
        top_layer: Option<egui::LayerId>,
    ) -> std::collections::BTreeSet<WindowId> {
        let point = egui::pos2(point.0, point.1);
        self.windows
            .iter()
            .filter(|window| top_layer.is_some_and(|layer| layer.id == window_area_id(**window)))
            .filter_map(|window| {
                window.id.filter(|id| {
                    snapshot.is_visible(*id)
                        && rect_contains(window_screen_rect(*window, layout), point)
                })
            })
            .collect()
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

    pub fn clear(&mut self) {
        self.windows.clear();
    }

    fn prepare_faction(&mut self, faction: CockpitFaction) {
        if self.faction != faction {
            self.faction = faction;
            self.clear();
        }
    }

    fn focus(&mut self, system: SystemKey) -> bool {
        let Some(index) = self
            .windows
            .iter()
            .position(|window| window.id.is_none() && window.system == system)
        else {
            return false;
        };
        let window = self.windows.remove(index);
        self.windows.push(window);
        true
    }

    fn close(&mut self, system: SystemKey) {
        self.windows.retain(|window| window.system != system);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[expect(
    clippy::struct_excessive_bools,
    reason = "Each flag is one independent title-bar or list outcome of a frame."
)]
struct WindowDrawResult {
    focus: bool,
    close: bool,
    minimize: bool,
    focus_sector: bool,
    tab: Option<SystemWindowTab>,
    /// A press on a facility page's list: the item under it, or none.
    item: Option<Option<FacilityItem>>,
    band: Option<ProductionArea>,
    object_menu: Option<(Option<MenuObject>, (i16, i16))>,
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
) -> Vec<SystemWindowAction> {
    draw_system_windows_with_snapshot(
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
        None,
        &[],
        None,
    )
}

/// Draw manager-owned System content in the frozen manager order while the
/// existing reference rail remains a temporary presentation adapter.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit state and rendering inputs at this UI boundary; the rail's Missions icon reads the mission state."
)]
pub fn draw_managed_system_windows(
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
    snapshot: &RoutingSnapshot,
    pointer_events: &[SystemWindowInputEvent],
    router: &mut FrameInputRouter,
) -> Vec<SystemWindowAction> {
    draw_system_windows_with_snapshot(
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
        Some(snapshot),
        pointer_events,
        Some(router),
    )
}

fn claim_system_pointer_event(
    router: &mut FrameInputRouter,
    events: &[SystemWindowInputEvent],
    window: WindowId,
    phase: PointerPhase,
) -> Option<InputEventId> {
    events.iter().rev().find_map(|event| {
        (event.phase == phase
            && router.pointer_recipient(event.id) == InputRecipient::Window(window)
            && router.claim(event.id, InputRecipient::Window(window)))
        .then_some(event.id)
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "Shared implementation for production snapshots and fixture-only direct content."
)]
fn draw_system_windows_with_snapshot(
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
    snapshot: Option<&RoutingSnapshot>,
    pointer_events: &[SystemWindowInputEvent],
    mut router: Option<&mut FrameInputRouter>,
) -> Vec<SystemWindowAction> {
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
    let mut windows = Vec::new();
    if let Some(snapshot) = snapshot {
        windows.extend(snapshot.paint_order().iter().filter_map(|id| {
            if snapshot
                .key(*id)
                .is_some_and(|key| key.kind == WindowKind::System)
            {
                state
                    .windows
                    .iter()
                    .find(|window| window.id == Some(*id))
                    .copied()
            } else {
                None
            }
        }));
    }
    // Direct content exists only in renderer tests and explicit fixtures.
    windows.extend(
        state
            .windows
            .iter()
            .filter(|window| window.id.is_none())
            .copied(),
    );
    let focused_manager = snapshot.and_then(|frame| frame.focused().map(|focus| focus.window));
    let focused_fixture_system = windows
        .last()
        .filter(|window| window.id.is_none())
        .map(|window| window.system);
    for window in windows {
        let system = window.system;
        let focused = match window.id {
            Some(id) => focused_manager == Some(id),
            None => focused_fixture_system == Some(system),
        };
        let result = draw_system_window(ctx, sources, window, focused, faction, layout, cache);
        let pointer_mutation = result.focus
            || result.close
            || result.minimize
            || result.focus_sector
            || result.tab.is_some()
            || result.item.is_some()
            || result.band.is_some()
            || result.object_menu.is_some();
        let pointer_permitted = match (window.id, pointer_mutation, router.as_deref_mut()) {
            (Some(id), true, Some(router)) => {
                let phase = if result.object_menu.is_some() {
                    PointerPhase::Release(PointerButtonId::Secondary)
                } else {
                    PointerPhase::Release(PointerButtonId::Primary)
                };
                claim_system_pointer_event(router, pointer_events, id, phase)
                    .or_else(|| {
                        (phase == PointerPhase::Release(PointerButtonId::Primary))
                            .then(|| {
                                claim_system_pointer_event(
                                    router,
                                    pointer_events,
                                    id,
                                    PointerPhase::Press(PointerButtonId::Primary),
                                )
                            })
                            .flatten()
                    })
                    .is_some()
            }
            (Some(_), true, None) => false,
            _ => true,
        };
        if (result.close && pointer_permitted) || !world.systems.contains_key(system) {
            if let Some(id) = window.id {
                actions.push(SystemWindowAction::Close(id));
            } else {
                state.close(system);
            }
            continue;
        }
        if result.minimize && pointer_permitted {
            if let Some(id) = window.id {
                actions.push(SystemWindowAction::MinimizeManaged {
                    window: id,
                    system,
                    logical_position: window.logical_position,
                });
            } else {
                state.close(system);
            }
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
        if result.focus && pointer_permitted {
            if let Some(id) = window.id {
                actions.push(SystemWindowAction::Activate(id));
            } else {
                state.focus(system);
            }
            actions.push(SystemWindowAction::SelectSystem(system));
        }
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

#[expect(
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
        let color = relationship_color(relationship(system.control, cockpit_faction(faction)));
        egui::Area::new(egui::Id::new(("original-reference-rail", slot.id.get())))
            .fixed_pos(slot.rect.min)
            .order(egui::Order::Middle)
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
                            cockpit_faction(faction),
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

#[expect(
    clippy::too_many_lines,
    reason = "Keep the window's ordered paint and input pass together, as the Defenses window does."
)]
fn draw_system_window(
    ctx: &egui::Context,
    sources: Sources<'_>,
    window: OpenSystemWindow,
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
    let id = window_area_id(window);
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

            let font = egui::FontId::proportional((9.0 * scale).max(6.0));
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
                        let line = |text: &str, (x, y): (f32, f32)| {
                            painter.text(
                                logical_point(local, scale, bx + x, by + y),
                                egui::Align2::LEFT_TOP,
                                text,
                                font.clone(),
                                egui::Color32::WHITE,
                            );
                        };
                        line(crate::manufacturing_window::band_title(area), BAND_TITLE);
                        line(&shown.status, BAND_STATUS);
                        if let Some(mini) = shown.mini {
                            paint(
                                cache,
                                DllSource::Gokres,
                                mini,
                                bx + BAND_MINI.0,
                                by + BAND_MINI.1,
                            );
                        }
                        if let Some(units) = &shown.units {
                            line(units, BAND_UNITS);
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

fn window_area_id(window: OpenSystemWindow) -> egui::Id {
    window.id.map_or_else(
        || area_id(window.system),
        crate::windowing::managed_child_id,
    )
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

pub(crate) fn special_force_mini(dat_id: DatId) -> Option<(u32, &'static str)> {
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
        WindowCommand, WindowKind, WindowManager, WindowOwner, WindowTransition,
    };

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
        let (reopened_id, transition) =
            commit_prepared_system_window(&mut manager, &mut state, reopened);
        assert_eq!(reopened_id, id);
        assert!(matches!(
            transition,
            WindowTransition::Existing { id: existing, .. } if existing == id
        ));
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

        manager.apply(WindowCommand::Minimize(id)).unwrap();
        assert!(!state.window_is_visible(&manager.begin_frame(), system));
        let restored = state
            .prepare_open(&world, system, (500, 350), CockpitFaction::Alliance, layout)
            .unwrap();
        let (restored_id, transition) =
            commit_prepared_system_window(&mut manager, &mut state, restored);
        assert_eq!(restored_id, id);
        assert!(matches!(
            transition,
            WindowTransition::Existing { id: existing, .. } if existing == id
        ));
        assert!(state.window_is_visible(&manager.begin_frame(), system));
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

        PAINTED.with(|painted| painted.borrow_mut().clear());
        let ctx = egui::Context::default();
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &snapshot);
        let mut cache = BmpCache::new();
        let movement = rebellion_core::movement::MovementState::default();
        let fog = FogState::new(Faction::Alliance);
        let missions = MissionState::new();
        let mut actions = Vec::new();
        let raw_input = || egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1400.0, 1000.0),
            )),
            ..egui::RawInput::default()
        };
        let _ = ctx.run(raw_input(), |ctx| {
            let _ = draw_reference_rail(
                ctx,
                &world,
                &movement,
                &fog,
                &missions,
                CockpitFaction::Alliance,
                rail_layout,
                &mut cache,
                &snapshot,
                &[],
                &mut router,
            );
        });
        PAINTED.with(|painted| painted.borrow_mut().clear());
        router.begin_frame(2, &snapshot);
        let output = ctx.run(raw_input(), |ctx| {
            actions = draw_reference_rail(
                ctx,
                &world,
                &movement,
                &fog,
                &missions,
                CockpitFaction::Alliance,
                rail_layout,
                &mut cache,
                &snapshot,
                &[],
                &mut router,
            );
        });
        assert!(actions.is_empty());
        let painted = PAINTED.with(|painted| painted.take());
        let defenses_icon = crate::defenses_window::rail_icon(&world, systems[2]);
        assert!(
            painted
                .iter()
                .any(|(resource, _)| *resource == defenses_icon),
            "{painted:?}"
        );
        let missions_icon = crate::missions_window::rail_icon(
            &world,
            &fog,
            &missions,
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

    #[test]
    fn reference_rail_restore_removes_entry_without_recreating_content() {
        let (world, systems) = fixture_world(1);
        let system = systems[0];
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let mut manager = WindowManager::new();
        let mut state = SystemWindowState::default();
        let prepared = state
            .prepare_open(&world, system, ORIGIN, CockpitFaction::Alliance, layout)
            .unwrap();
        let (id, _) = commit_prepared_system_window(&mut manager, &mut state, prepared);
        manager.apply(WindowCommand::Minimize(id)).unwrap();
        assert_eq!(manager.begin_frame().presentation_history(), [id]);

        let point = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 0)).center();
        let pointer_button = |pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let frames = [
            (
                vec![egui::Event::PointerMoved(point)],
                PointerPhase::Hover,
                false,
            ),
            (
                vec![egui::Event::PointerMoved(point)],
                PointerPhase::Hover,
                false,
            ),
            (
                vec![pointer_button(true)],
                PointerPhase::Press(PointerButtonId::Primary),
                false,
            ),
            (
                vec![pointer_button(false)],
                PointerPhase::Release(PointerButtonId::Primary),
                true,
            ),
            (
                vec![egui::Event::PointerMoved(point)],
                PointerPhase::Hover,
                false,
            ),
            (
                vec![egui::Event::PointerMoved(point)],
                PointerPhase::Hover,
                false,
            ),
            (
                vec![pointer_button(true)],
                PointerPhase::Press(PointerButtonId::Primary),
                false,
            ),
            (
                vec![pointer_button(false)],
                PointerPhase::Release(PointerButtonId::Primary),
                false,
            ),
        ];
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
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1000.0),
                )),
                events,
                ..egui::RawInput::default()
            };
            let movement = rebellion_core::movement::MovementState::default();
            let fog = FogState::new(Faction::Alliance);
            let missions = MissionState::new();
            let manufacturing = ManufacturingState::new();
            let deliveries = DeliveryState::new();
            let mut frame_actions = Vec::new();
            let _ = ctx.run(input, |ctx| {
                frame_actions = draw_managed_system_windows(
                    ctx,
                    &world,
                    &movement,
                    &fog,
                    &missions,
                    &manufacturing,
                    &deliveries,
                    &mut state,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                    &snapshot,
                    &[SystemWindowInputEvent {
                        id: event,
                        point: (point.x, point.y),
                        phase,
                    }],
                    &mut router,
                );
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
        assert!(manager.begin_frame().presentation_history().is_empty());
        assert_eq!(manager.begin_frame().paint_order(), [id]);
        assert_eq!(state.window_id(system), Some(id));
        assert_eq!(state.window_count(), 1);
    }

    #[test]
    fn reference_rail_restore_claims_its_own_release_in_multi_release_frame() {
        let (world, systems) = fixture_world(1);
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let mut manager = WindowManager::new();
        let opened = manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::contextual_spec(
                    WindowKind::System,
                    system_subject(&world, systems[0]).unwrap(),
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let WindowTransition::Opened { id } = opened.transition else {
            panic!("expected detail open");
        };
        manager.apply(WindowCommand::Minimize(id)).unwrap();

        let rail_point = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 0)).center();
        let unrelated_point = egui::pos2(1300.0, 900.0);
        let pointer_button = |point, pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let snapshot = manager.begin_frame();
        let ctx = egui::Context::default();
        let mut router = FrameInputRouter::default();
        let mut cache = BmpCache::new();
        let movement = rebellion_core::movement::MovementState::default();
        let fog = FogState::new(Faction::Alliance);
        let missions = MissionState::new();

        for frame in 1..=2 {
            router.begin_frame(frame, &snapshot);
            let _ = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1400.0, 1000.0),
                    )),
                    events: vec![egui::Event::PointerMoved(rail_point)],
                    ..egui::RawInput::default()
                },
                |ctx| {
                    assert!(draw_reference_rail(
                        ctx,
                        &world,
                        &movement,
                        &fog,
                        &missions,
                        CockpitFaction::Alliance,
                        layout,
                        &mut cache,
                        &snapshot,
                        &[],
                        &mut router,
                    )
                    .is_empty());
                },
            );
        }

        let press = InputEventId {
            frame: 3,
            ordinal: 0,
        };
        router.begin_frame(3, &snapshot);
        assert_eq!(
            router.collect_pointer(
                press,
                &std::collections::BTreeSet::new(),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Root
        );
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1000.0),
                )),
                events: vec![pointer_button(rail_point, true)],
                ..egui::RawInput::default()
            },
            |ctx| {
                assert!(draw_reference_rail(
                    ctx,
                    &world,
                    &movement,
                    &fog,
                    &missions,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                    &snapshot,
                    &[SystemWindowInputEvent {
                        id: press,
                        point: (rail_point.x, rail_point.y),
                        phase: PointerPhase::Press(PointerButtonId::Primary),
                    }],
                    &mut router,
                )
                .is_empty());
            },
        );

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
        router.begin_frame(4, &snapshot);
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
                router.collect_pointer(event, &std::collections::BTreeSet::new(), phase),
                InputRecipient::Root
            );
        }
        let mut actions = Vec::new();
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1000.0),
                )),
                events: vec![pointer_button(rail_point, false)],
                ..egui::RawInput::default()
            },
            |ctx| {
                actions = draw_reference_rail(
                    ctx,
                    &world,
                    &movement,
                    &fog,
                    &missions,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
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
                    &mut router,
                );
            },
        );

        assert_eq!(actions, [SystemWindowAction::RestoreReferenceWindow(id)]);
        assert!(
            !router.claim_root(causal_release),
            "the rail must claim the release inside its exact slot"
        );
        assert!(
            router.claim_root(unrelated_release),
            "an unrelated root release must remain available"
        );
    }

    #[test]
    fn reference_rail_modal_blocks_restore_and_hover_actions() {
        let (world, systems) = fixture_world(1);
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let mut manager = WindowManager::new();
        let opened = manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::contextual_spec(
                    WindowKind::System,
                    system_subject(&world, systems[0]).unwrap(),
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let WindowTransition::Opened { id } = opened.transition else {
            panic!("expected detail open");
        };
        manager.apply(WindowCommand::Minimize(id)).unwrap();
        manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::singleton_spec(
                    WindowKind::SystemFinder,
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let point = cockpit_rect(layout, rail_slot_rect(CockpitFaction::Alliance, 0)).center();
        let pointer_button = |pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let frames = [
            (vec![egui::Event::PointerMoved(point)], PointerPhase::Hover),
            (vec![egui::Event::PointerMoved(point)], PointerPhase::Hover),
            (
                vec![pointer_button(true)],
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            (
                vec![pointer_button(false)],
                PointerPhase::Release(PointerButtonId::Primary),
            ),
        ];
        let ctx = egui::Context::default();
        let mut router = FrameInputRouter::default();
        let mut actions = Vec::new();
        let mut cache = BmpCache::new();
        let movement = rebellion_core::movement::MovementState::default();
        let fog = FogState::new(Faction::Alliance);
        let missions = MissionState::new();
        for (index, (events, phase)) in frames.into_iter().enumerate() {
            let frame = u64::try_from(index + 1).unwrap();
            let event = InputEventId { frame, ordinal: 0 };
            let snapshot = manager.begin_frame();
            router.begin_frame(frame, &snapshot);
            assert_eq!(
                router.collect_pointer(event, &std::collections::BTreeSet::new(), phase),
                InputRecipient::Blocked
            );
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1000.0),
                )),
                events,
                ..egui::RawInput::default()
            };
            let mut frame_actions = Vec::new();
            let _ = ctx.run(input, |ctx| {
                frame_actions = draw_reference_rail(
                    ctx,
                    &world,
                    &movement,
                    &fog,
                    &missions,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                    &snapshot,
                    &[SystemWindowInputEvent {
                        id: event,
                        point: (point.x, point.y),
                        phase,
                    }],
                    &mut router,
                );
            });
            actions.extend(frame_actions);
        }

        assert!(actions.is_empty(), "modal rail input leaked: {actions:?}");
        assert_eq!(manager.begin_frame().presentation_history(), [id]);
    }

    #[test]
    fn reference_rail_missing_art_does_not_delete_registered_detail() {
        let (world, systems) = fixture_world(1);
        let layout = layout(CockpitFaction::Alliance, 1.0);
        let mut manager = WindowManager::new();
        let opened = manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::contextual_spec(
                    WindowKind::Fleet,
                    system_subject(&world, systems[0]).unwrap(),
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let WindowTransition::Opened { id } = opened.transition else {
            panic!("expected detail open");
        };
        manager.apply(WindowCommand::Minimize(id)).unwrap();
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
                assert!(draw_reference_rail(
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
                )
                .is_empty());
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
        painted: Vec<(u32, egui::Pos2)>,
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
        let layout = layout(CockpitFaction::Alliance, 2.0);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let fog = FogState::new(Faction::Alliance);
        let mut run = Run::default();
        for events in frames {
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
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                ));
            });
            run.painted = PAINTED.with(|painted| painted.take());
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
    fn faction_change_clears_visible_windows() {
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
        assert_eq!((heading.1, heading.3), (at(60.0, 58.0), 18.0));
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
        // FUN_00457c90: the product's name, mini at (40, 15) and units at
        // (5, 47).
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
        let mini = crate::panels::fleets::capital_ship_mini_id(DatId::new(0x1400_0040)).unwrap();
        assert!(
            run.painted.contains(&(mini, at(95.0, 72.0))),
            "{:?}",
            run.painted
        );
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
}
