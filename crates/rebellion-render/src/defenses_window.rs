//! The original System Defenses window (window type 10, `FUN_004a7790`): a
//! system's batteries, shields, squadrons, regiments and personnel under five
//! tabs (manual Fig. 3.73, p. 125). Recovery notes:
//! `ghidra/notes/sector-quadrants.md`, "Type 10".
//!
//! It opens from the icon a sector window shows at the bottom left of a
//! planet (`FUN_0045ce80`, `FUN_0045aac0` kind 8). Every page lists only the
//! side that holds the system (`+0x148`), and a move released over the
//! window targets its system (`+0x70`, `FUN_004aa470`). A right release
//! opens the object pop-up menu for the list's selection: the class keeps
//! the base dialog's slots 7 and 8 (`FUN_004ac5c0`, `FUN_004ac730`), and
//! slot 22 (`FUN_004a7a20`) gives the list's selected items. A squadron
//! dragged out of the list moves to where it is dropped (`0x201`), as one
//! out of the Fleet window does.

use egui_macroquad::egui;
use rebellion_core::dat::{ExplorationStatus, Faction};
use rebellion_core::economy::EconomyState;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{
    CharacterKey, DefenseFacilityKey, FighterKey, SpecialForceKey, SystemKey, TroopKey,
};
use rebellion_core::missions::{MissionMember, MissionState};
use rebellion_core::world::GameWorld;
use rebellion_windowing::{
    ContentReport, ContentStatus, RoutingSnapshot, SubjectId, WindowId, WindowKind, WindowOwner,
    WindowSpec, WindowTransition,
};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::{
    control_side, faction_side, fleet_side, mini_layers, paint_native, MiniObject,
    DRAG_DISTANCE_SQUARED,
};
use crate::object_menu::MenuObject;
use crate::panels::fleets::fighter_mini_id;
use crate::quadrant_icons::{mission_keys, system_members};
use crate::scroll_bar::{egui_pointer, ScrollBar, ScrollDrag, ScrollRange, LIST_BAR_29CC};
use crate::system_window::{
    canvas_point, character_mini_resource_id, clamp_window_to_galaxy, defense_facility_mini,
    exact_clicked, logical_rect, opposing_contents_visible, rect_contains, special_force_mini,
    troop_mini,
};
use crate::windowing_input::{
    FrameInputRouter, InputEventId, InputRecipient, PointerButtonId, PointerPhase,
};
use crate::windowing_policy::{contextual_spec, system_subject};

pub const DEFENSES_WINDOW_WIDTH: f32 = 235.0;
pub const DEFENSES_WINDOW_HEIGHT: f32 = 304.0;

const BACKGROUND: u32 = 10577;
const CLOSE_NORMAL: u32 = 10108;
const CLOSE_PRESSED: u32 = 10109;
const MINIMIZE_NORMAL: u32 = 10253;
const MINIMIZE_PRESSED: u32 = 10254;
const SECTOR_NORMAL: u32 = 10209;
const SECTOR_PRESSED: u32 = 10208;
/// The row frame `+0x5d` keyed over a selected row's mini (`FUN_004a9ce0`):
/// 10578 for side 1, 10579 otherwise.
const ROW_FRAME: [u32; 2] = [10578, 10579];
/// A minimized Defenses window's rail icon (`FUN_004aa4a0`).
const RAIL_ICONS: [u32; 3] = [11533, 11534, 11535];

/// The tab strip (`FUN_0060d590`) at (0, 20); each button is 36 by 33.
const TAB_STRIP: (f32, f32) = (0.0, 20.0);
const TAB_SIZE: (f32, f32) = (36.0, 33.0);
/// The list (`FUN_00607ea0`, id `0xcb`) and its 70 by 70 grid cells.
const LIST: (f32, f32, f32, f32) = (7.0, 81.0, 222.0, 210.0);
const CELL: f32 = 70.0;
const COLUMNS: usize = 3;
const VISIBLE_ROWS: usize = 3;
/// The text rect's offset in a cell (`+0xe8`, `+0xec`) and its right trim.
const TEXT_OFFSET: (f32, f32) = (1.0, 27.0);
const TEXT_RIGHT_TRIM: f32 = 3.0;
/// The title label `+0x49` (`FUN_004a8790`): from the sector button's width
/// plus 5, 16 high.
const TITLE: (f32, f32, f32) = (19.0, 2.0, 16.0);
/// The tab name `+0x5b` and the garrison line `+0x5c` (`FUN_004a8790`): x, y,
/// width and height.
const TAB_NAME: (f32, f32, f32, f32) = (2.0, 51.0, 231.0, 16.0);
const GARRISON: (f32, f32, f32, f32) = (2.0, 63.0, 228.0, 17.0);

/// The five pages (`FUN_004a90d0`), by tab id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefensesPage {
    /// Id 1: personnel not on a visible mission.
    Personnel,
    /// Id 2: regiments (`0x10..0x14`).
    Regiments,
    /// Id 3: fighter squadrons (`0x1c..0x20`).
    Squadrons,
    /// Id 4: GenCore shields (`0x24`) and the Death Star Shield (`0x25`).
    Shields,
    /// Id 5: KDY-150 (`0x22`) and LNR batteries (`0x23`).
    Batteries,
}

impl DefensesPage {
    pub const ALL: [Self; 5] = [
        Self::Personnel,
        Self::Regiments,
        Self::Squadrons,
        Self::Shields,
        Self::Batteries,
    ];

    /// The button's x in the strip (`FUN_004a8790`).
    const fn x(self) -> f32 {
        match self {
            Self::Personnel => 28.0,
            Self::Regiments => 64.0,
            Self::Squadrons => 100.0,
            Self::Shields => 136.0,
            Self::Batteries => 172.0,
        }
    }

    /// The help message `0x1740 + (5 - id)`: TEXTSTRA 5952..5956.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Batteries => "Planetary Batteries",
            Self::Shields => "Planetary Shields",
            Self::Squadrons => "Fighter Squadrons",
            Self::Regiments => "Trooper Regiments",
            Self::Personnel => "Personnel",
        }
    }
}

/// The button's bitmap (`FUN_004a8790`, `FUN_004a9ce0`): the normal id when
/// the page lists something, the empty id two higher otherwise, and the
/// middle id while selected. Squadrons, regiments and personnel have art per
/// side; any side but 1 and 2 shows one gray bitmap. Batteries and shields
/// share their art across sides, and `FUN_0060d700` copies the pressed
/// bitmap into the selected state they leave unset.
#[must_use]
pub const fn tab_resource(page: DefensesPage, side: u8, selected: bool, empty: bool) -> u32 {
    let base = match (page, side) {
        (DefensesPage::Batteries, _) => 10550,
        (DefensesPage::Shields, _) => 10553,
        (DefensesPage::Squadrons, 1) => 10556,
        (DefensesPage::Squadrons, 2) => 10559,
        (DefensesPage::Squadrons, _) => return 10562,
        (DefensesPage::Regiments, 1) => 10563,
        (DefensesPage::Regiments, 2) => 10566,
        (DefensesPage::Regiments, _) => return 10569,
        (DefensesPage::Personnel, 1) => 10570,
        (DefensesPage::Personnel, 2) => 10573,
        (DefensesPage::Personnel, _) => return 10576,
    };
    if selected {
        base + 1
    } else if empty {
        base + 2
    } else {
        base
    }
}

/// An object a page lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefensesItem {
    Character(CharacterKey),
    SpecialForce(SpecialForceKey),
    Troop(TroopKey),
    /// The `nth` squadron of a class the system holds for the shown side.
    Squadron {
        class: FighterKey,
        nth: u32,
    },
    Defense(DefenseFacilityKey),
}

impl DefensesItem {
    /// The object its pop-up menu opens for: a character (`FUN_004ed350`),
    /// a special force (`FUN_00503b50`), a regiment (`FUN_00504b30`) or a
    /// defense facility, whose menu holds Encyclopedia and Status
    /// (`FUN_0051d990`).
    const fn menu_object(self, system: SystemKey, side: u8) -> Option<MenuObject> {
        match self {
            Self::Character(key) => Some(MenuObject::Character(key)),
            Self::SpecialForce(key) => Some(MenuObject::SpecialForce(key)),
            Self::Troop(key) => Some(MenuObject::Troop(key)),
            Self::Squadron { class, .. } => Some(MenuObject::SystemFighter {
                system,
                class,
                is_alliance: side == 1,
            }),
            Self::Defense(key) => Some(MenuObject::DefenseFacility(key)),
        }
    }
}

/// One list row: the object, its GOKRES mini and its name (`FUN_004a9ab0`).
#[derive(Debug, Clone, PartialEq, Eq)]
struct DefensesRow {
    item: DefensesItem,
    mini: u32,
    label: String,
}

#[derive(Debug, Clone, PartialEq)]
struct OpenDefensesWindow {
    /// Opaque session identity allocated by the shared window manager.
    /// Fixture-only direct opens deliberately have no production identity.
    id: Option<WindowId>,
    subject: Option<SubjectId>,
    system: SystemKey,
    logical_position: (i16, i16),
    page: DefensesPage,
    /// port: one selected row; the list's own selection rules
    /// (`FUN_00609410`) are untraced.
    selected: Option<DefensesItem>,
    /// The list's first shown row of three cells, and its scroll bar's
    /// press (`FUN_0060a490`, RCDATA `0x29cc`).
    first_row: usize,
    scroll: ScrollDrag,
    /// Whether this exact content identity was admitted for release targeting
    /// by the current frozen presentation snapshot.
    release_target_visible: bool,
}

/// Validated Defenses content held aside until the manager accepts its stable
/// `(Defenses, system DAT ID)` identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreparedDefensesWindow {
    spec: WindowSpec,
    faction: CockpitFaction,
    system: SystemKey,
    logical_position: (i16, i16),
}

impl PreparedDefensesWindow {
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

/// A left press held on a squadron, character or special-force row until
/// its release (`CoolDragList`, `FUN_006083c0`).
#[derive(Debug, Clone, Copy, PartialEq)]
struct RowDrag {
    system: SystemKey,
    item: DefensesItem,
    is_alliance: bool,
    press: egui::Pos2,
    list: egui::Rect,
}

/// The open Defenses windows. The last is focused and paints on top.
#[derive(Debug)]
pub struct DefensesWindowState {
    faction: CockpitFaction,
    windows: Vec<OpenDefensesWindow>,
    drag: Option<RowDrag>,
    drag_owner: Option<WindowId>,
}

impl Default for DefensesWindowState {
    fn default() -> Self {
        Self {
            faction: CockpitFaction::Alliance,
            windows: Vec::new(),
            drag: None,
            drag_owner: None,
        }
    }
}

#[derive(Default)]
struct EndDragResult {
    actions: Vec<DefensesWindowAction>,
    claimed_primary_release_by: Option<WindowId>,
}

impl DefensesWindowState {
    /// Whether any window of this kind is open.
    #[must_use]
    pub fn any_open(&self) -> bool {
        !self.windows.is_empty()
    }

    /// Egui layer attached to one manager-issued Defenses identity.
    #[must_use]
    pub fn managed_layer_id(&self, id: WindowId) -> Option<egui::LayerId> {
        self.windows
            .iter()
            .find(|window| window.id == Some(id))
            .map(|window| egui::LayerId::new(egui::Order::Foreground, area_id(window)))
    }

    /// Validates and prepares Defenses content without mutating the live
    /// store. The stable subject is the system's original DAT identity.
    #[must_use]
    pub fn prepare_open(
        &self,
        world: &GameWorld,
        system: SystemKey,
        logical_position: (i16, i16),
        faction: CockpitFaction,
        layout: CockpitLayout,
    ) -> Option<PreparedDefensesWindow> {
        let subject = system_subject(world, system)?;
        Some(PreparedDefensesWindow {
            spec: contextual_spec(WindowKind::Defenses, subject, WindowOwner::StrategicRoot),
            faction,
            system,
            logical_position: clamp_window_to_galaxy(
                logical_position,
                layout,
                DEFENSES_WINDOW_WIDTH,
                DEFENSES_WINDOW_HEIGHT,
            ),
        })
    }

    /// Publishes new Defenses content, or confirms the exact preserved
    /// content, only after a successful manager open commit.
    pub fn commit_open(
        &mut self,
        prepared: PreparedDefensesWindow,
        transition: &WindowTransition,
    ) -> Option<WindowId> {
        let subject = prepared.spec.key.subject?;
        match *transition {
            WindowTransition::Opened { id } => {
                // Dedicated interface fixtures predate manager publication and
                // may seed one exact Defenses content record before the app
                // queues its semantic open. Attach only that unmanaged record
                // after the manager accepts the identity; preserve every
                // content field and reject any ambiguous or conflicting state.
                #[cfg(feature = "interface-test-fixtures")]
                if let Some(index) = self.windows.iter().position(|window| {
                    window.id.is_none()
                        && window.subject.is_none()
                        && window.system == prepared.system
                }) {
                    if self.faction != prepared.faction
                        || self.windows.iter().enumerate().any(|(candidate, window)| {
                            candidate != index
                                && (window.id == Some(id)
                                    || window.subject == Some(subject)
                                    || window.system == prepared.system)
                        })
                    {
                        return None;
                    }
                    let window = &mut self.windows[index];
                    window.id = Some(id);
                    window.subject = Some(subject);
                    window.release_target_visible = false;
                    return Some(id);
                }
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
                self.windows.push(OpenDefensesWindow {
                    id: Some(id),
                    subject: Some(subject),
                    system: prepared.system,
                    logical_position: prepared.logical_position,
                    page: DefensesPage::Personnel,
                    selected: None,
                    first_row: 0,
                    scroll: ScrollDrag::default(),
                    release_target_visible: false,
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

    /// Open `system`'s Defenses window at a logical point, clamped into the
    /// galaxy view, on the personnel page (`FUN_004a8790` selects tab 1), or
    /// bring its open window to the front: one per system (`FUN_0045aac0`).
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
        self.windows.push(OpenDefensesWindow {
            id: None,
            subject: None,
            system,
            logical_position: clamp_window_to_galaxy(
                logical_position,
                layout,
                DEFENSES_WINDOW_WIDTH,
                DEFENSES_WINDOW_HEIGHT,
            ),
            page: DefensesPage::Personnel,
            selected: None,
            first_row: 0,
            scroll: ScrollDrag::default(),
            release_target_visible: true,
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

    /// Whether exact Defenses content participates visibly in this frozen
    /// frame. Manager-owned content must retain its committed kind and DAT
    /// subject; direct fixture content remains visible while its system exists.
    #[must_use]
    pub fn has_visible_content(&self, snapshot: &RoutingSnapshot, world: &GameWorld) -> bool {
        self.windows.iter().any(|window| match window.id {
            Some(id) => {
                snapshot.is_visible(id)
                    && snapshot.key(id).is_some_and(|key| {
                        key.kind == WindowKind::Defenses && key.subject == window.subject
                    })
                    && system_subject(world, window.system) == window.subject
            }
            None => world.systems.contains_key(window.system),
        })
    }

    /// Whether exact manager-owned Defenses content remains attached.
    #[must_use]
    pub fn contains_window(&self, id: WindowId) -> bool {
        self.windows.iter().any(|window| window.id == Some(id))
    }

    /// Removes only content named by manager teardown receipts.
    pub fn remove_content(&mut self, removed: &[WindowId]) {
        self.windows
            .retain(|window| window.id.is_none_or(|id| !removed.contains(&id)));
        if self.drag.as_ref().is_some_and(|drag| {
            !self
                .windows
                .iter()
                .any(|window| window.system == drag.system)
        }) {
            self.drag = None;
            self.drag_owner = None;
        }
    }

    /// Reports every active Defenses identity in the frozen snapshot,
    /// including minimized history entries whose content remains attached.
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
                    .is_some_and(|key| key.kind == WindowKind::Defenses)
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

    #[must_use]
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    #[must_use]
    pub fn is_open(&self, system: SystemKey) -> bool {
        self.windows.iter().any(|window| window.system == system)
    }

    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        let point = egui::pos2(point.0, point.1);
        self.windows
            .iter()
            .any(|window| rect_contains(window_screen_rect(window, layout), point))
    }

    /// Whether visible Defenses content covers `point`. Minimized manager
    /// content stays attached but cannot block the strategic root.
    #[must_use]
    pub fn contains_visible_screen_point(
        &self,
        snapshot: &RoutingSnapshot,
        layout: CockpitLayout,
        point: (f32, f32),
    ) -> bool {
        let point = egui::pos2(point.0, point.1);
        self.windows.iter().any(|window| {
            window.id.is_none_or(|id| snapshot.is_visible(id))
                && rect_contains(window_screen_rect(window, layout), point)
        })
    }

    /// Exact visible manager identity whose Defenses rectangle covers
    /// `point` when egui reports that content layer as topmost.
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
            .filter(|window| top_layer.is_some_and(|layer| layer.id == area_id(window)))
            .filter_map(|window| {
                window.id.filter(|id| {
                    snapshot.is_visible(*id)
                        && rect_contains(window_screen_rect(window, layout), point)
                })
            })
            .collect()
    }

    /// The screen rect of `system`'s window.
    #[must_use]
    pub fn screen_rect(&self, layout: CockpitLayout, system: SystemKey) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(window_screen_rect(window, layout))
    }

    /// Show `page` in `system`'s open window, as a click on its tab does
    /// (`FUN_004a90d0` refills only for a different page).
    pub fn show_page(&mut self, system: SystemKey, page: DefensesPage) -> bool {
        let Some(window) = self.window_mut(system) else {
            return false;
        };
        if window.page != page {
            window.page = page;
            window.selected = None;
            window.first_row = 0;
        }
        true
    }

    /// The screen rect of `page`'s button in `system`'s window.
    #[must_use]
    pub fn tab_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        page: DefensesPage,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(tab_rect(
            window_screen_rect(window, layout),
            layout.scale,
            page,
        ))
    }

    /// The screen rect of the `index`th cell of `system`'s list.
    #[must_use]
    pub fn cell_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        index: usize,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(cell_rect(
            window_screen_rect(window, layout),
            layout.scale,
            index,
        ))
    }

    /// What `system`'s window shows, for the interface fixture.
    #[must_use]
    pub fn report(
        &self,
        world: &GameWorld,
        fog: &FogState,
        missions: &MissionState,
        economy: &EconomyState,
        system: SystemKey,
    ) -> Option<DefensesWindowReport> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        let player = cockpit_faction(self.faction);
        let rows = page_rows(world, fog, missions, player, system, window.page);
        let selected = window
            .selected
            .and_then(|item| rows.iter().position(|row| row.item == item));
        Some(DefensesWindowReport {
            origin: window.logical_position,
            side: shown_side(world, system),
            page: window.page,
            first_row: window.first_row,
            counts: DefensesPage::ALL
                .map(|page| page_rows(world, fog, missions, player, system, page).len()),
            rows: rows.into_iter().map(|row| row.label).collect(),
            selected,
            garrison: garrison_line(world, economy, player, system, window.page),
        })
    }

    /// The system a move released over the Defenses window egui draws as
    /// `layer` takes (`+0x70`, `FUN_004aa470`: the subject wherever the point
    /// is), or `None` when `layer` is none of them.
    #[must_use]
    pub fn release_target(&self, layer: egui::LayerId) -> Option<SystemKey> {
        self.windows
            .iter()
            .find(|window| window.release_target_visible && area_id(window) == layer.id)
            .map(|window| window.system)
    }

    /// Refreshes which retained egui layers may resolve a gameplay release.
    /// The application calls this immediately after freezing the frame so a
    /// stale prior-frame area cannot retarget before Defenses paints.
    pub fn refresh_release_targets(&mut self, snapshot: &RoutingSnapshot, world: &GameWorld) {
        for window in &mut self.windows {
            window.release_target_visible = match window.id {
                Some(id) => {
                    snapshot.is_visible(id)
                        && snapshot.key(id).is_some_and(|key| {
                            key.kind == WindowKind::Defenses && key.subject == window.subject
                        })
                        && system_subject(world, window.system) == window.subject
                }
                None => world.systems.contains_key(window.system),
            };
        }
    }

    pub fn clear(&mut self) {
        self.windows.clear();
        self.drag = None;
        self.drag_owner = None;
    }

    /// Whether a left press on a squadron row is held: the list has
    /// captured the mouse (`FUN_006083c0`).
    #[must_use]
    pub fn is_dragging(&self) -> bool {
        self.drag.is_some()
    }

    /// Exact manager capture owner of the active Defenses drag, if migrated.
    #[must_use]
    pub const fn drag_owner(&self) -> Option<WindowId> {
        self.drag_owner
    }

    /// Seeds the renderer side of an exact manager-owned drag for production
    /// boundary tests without exposing mutable capture internals in release
    /// builds.
    #[cfg(feature = "interface-test-fixtures")]
    #[doc(hidden)]
    pub fn seed_managed_drag_for_test(&mut self, window: WindowId) {
        self.drag = Some(RowDrag {
            system: SystemKey::default(),
            item: DefensesItem::Character(CharacterKey::default()),
            is_alliance: true,
            press: egui::Pos2::ZERO,
            list: egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        });
        self.drag_owner = Some(window);
    }

    /// Applies a manager capture-cancellation receipt to renderer payload.
    pub fn cancel_pointer_capture(&mut self, window: WindowId) {
        if self.drag_owner == Some(window) {
            self.drag = None;
            self.drag_owner = None;
        }
    }

    /// End a held drag on the left release: far enough from the press and
    /// outside the list, it becomes the `0x29a` drop, as in the Fleet
    /// window (`FUN_006083c0`).
    ///
    /// Ctrl held at the drop makes it a Confirmed Move (`0x202`,
    /// `GetAsyncKeyState(0x11)`) for characters and special forces.
    ///
    /// port: regiments and defenses do not drag; they keep their pop-up
    /// Move.
    fn end_drag(
        &mut self,
        ctx: &egui::Context,
        scale: f32,
        router: Option<&mut FrameInputRouter>,
        pointer_events: &[(InputEventId, PointerPhase)],
    ) -> EndDragResult {
        let (released, down, point, ctrl) = ctx.input(|input| {
            (
                input.pointer.primary_released(),
                input.pointer.primary_down(),
                input.pointer.latest_pos(),
                input.modifiers.ctrl,
            )
        });
        if !released {
            if !down && self.drag_owner.is_none() {
                self.drag = None;
            }
            return EndDragResult::default();
        }
        let mut result = EndDragResult::default();
        if let (Some(owner), Some(router)) = (self.drag_owner, router) {
            if claim_defenses_pointer_event(
                router,
                pointer_events,
                owner,
                PointerPhase::Release(PointerButtonId::Primary),
            )
            .is_none()
            {
                return result;
            }
            result.claimed_primary_release_by = Some(owner);
        }
        let Some(drag) = self.drag.take() else {
            return result;
        };
        result.actions.extend(
            self.drag_owner
                .take()
                .map(DefensesWindowAction::ReleasePointer),
        );
        let Some(point) = point else {
            return result;
        };
        let moved = (point - drag.press) / scale;
        if moved.length_sq() <= DRAG_DISTANCE_SQUARED || rect_contains(drag.list, point) {
            return result;
        }
        let action = match drag.item {
            DefensesItem::Squadron { class, .. } => DefensesWindowAction::DragSquadron {
                system: drag.system,
                class,
                is_alliance: drag.is_alliance,
                point,
            },
            DefensesItem::Character(key) => DefensesWindowAction::DragMember {
                member: MissionMember::Character(key),
                confirmed: ctrl,
                point,
            },
            DefensesItem::SpecialForce(key) => DefensesWindowAction::DragMember {
                member: MissionMember::SpecialForce(key),
                confirmed: ctrl,
                point,
            },
            DefensesItem::Troop(_) | DefensesItem::Defense(_) => return result,
        };
        result.actions.push(action);
        result
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
            .position(|window| window.system == system)
        else {
            return false;
        };
        let window = self.windows.remove(index);
        self.windows.push(window);
        true
    }

    fn close(&mut self, system: SystemKey) -> Option<OpenDefensesWindow> {
        let index = self
            .windows
            .iter()
            .position(|window| window.system == system)?;
        Some(self.windows.remove(index))
    }

    fn window_mut(&mut self, system: SystemKey) -> Option<&mut OpenDefensesWindow> {
        self.windows
            .iter_mut()
            .find(|window| window.system == system)
    }
}

/// What one Defenses window shows ([`DefensesWindowState::report`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefensesWindowReport {
    pub origin: (i16, i16),
    pub side: u8,
    pub page: DefensesPage,
    /// First visible grid row retained by the native scroll bar.
    pub first_row: usize,
    /// Each page's row count, in [`DefensesPage::ALL`] order.
    pub counts: [usize; 5],
    pub rows: Vec<String>,
    pub selected: Option<usize>,
    pub garrison: Option<String>,
}

/// Actions that leave the Defenses window manager.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DefensesWindowAction {
    /// The sector button (`0xca`) opens the subject's sector window
    /// (`FUN_00429ce0`).
    OpenSector(SystemKey),
    SelectSystem(SystemKey),
    /// A manager-owned Defenses window took pointer activation.
    Activate(WindowId),
    /// A routed primary press began a drag owned by this exact window.
    CapturePointer(WindowId),
    /// The matching routed primary release completed that capture.
    ReleasePointer(WindowId),
    /// The close button or stale renderer content requests exact teardown.
    Close(WindowId),
    /// The minimize button (`0xc9`) posts `0x466`: the window goes to the
    /// galaxy view's rail.
    Minimize {
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// Production minimize intent for manager-owned Defenses content. The
    /// rail adapter observes it only after the manager accepts the command.
    MinimizeManaged {
        window: WindowId,
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A right release on the list opens the object pop-up menu
    /// (`FUN_004ac5c0`) for the selection, at a 640 by 480 canvas point.
    OpenObjectMenu {
        /// Exact registered invoker when this is manager-owned content.
        owner: Option<WindowId>,
        system: SystemKey,
        selection: Option<MenuObject>,
        point: (i16, i16),
    },
    /// A Squadrons page row dragged out of the list (`0x29a`): the galaxy
    /// view hit-tests the screen point and issues `0x201`
    /// (`TroopTransportState::move_squadron`).
    DragSquadron {
        system: SystemKey,
        class: FighterKey,
        is_alliance: bool,
        point: egui::Pos2,
    },
    /// A Personnel page row dragged out of the list: the galaxy view
    /// issues `0x201`, or `0x202` with Ctrl, and the character or special
    /// force goes on a Move mission (`0x241`).
    DragMember {
        member: MissionMember,
        confirmed: bool,
        point: egui::Pos2,
    },
}

/// The side the window shows (`+0x148`): the system's side bits
/// (`FUN_004a8790`), refreshed when they change (`FUN_004a7f50`).
fn shown_side(world: &GameWorld, system: SystemKey) -> u8 {
    world
        .systems
        .get(system)
        .map_or(0, |value| control_side(value.control))
}

/// A minimized Defenses window's rail icon (`FUN_004aa4a0`): 11533 for side
/// 1, 11534 for side 2, 11535 otherwise, by the system's side bits.
#[must_use]
pub fn rail_icon(world: &GameWorld, system: SystemKey) -> u32 {
    match shown_side(world, system) {
        1 => RAIL_ICONS[0],
        2 => RAIL_ICONS[1],
        _ => RAIL_ICONS[2],
    }
}

/// `FUN_004a9800`/`FUN_004a9ce0`: the title strips, by the shown side and
/// whether the window is the galaxy view's focused child. The Fleet window
/// and the Missions window (`FUN_004a2200`) use the same art.
pub(crate) fn title_resource(side: u8, focused: bool) -> u32 {
    match (side, focused) {
        (1, true) => 10299,
        (1, false) => 10200,
        (2, true) => 10201,
        (2, false) => 10302,
        (_, true) => 10303,
        (_, false) => 10304,
    }
}

/// A selected row's text colour (`+0xdc`, `FUN_004a9ce0`): `0x20000ff`
/// (red) for side 1, `0x200ff00` (green) for side 2, `0x2ffff00` (cyan)
/// otherwise. Unselected rows are white (`+0xd8`, `0x2ffffff`).
fn selected_color(side: u8) -> egui::Color32 {
    match side {
        1 => egui::Color32::from_rgb(255, 0, 0),
        2 => egui::Color32::from_rgb(0, 255, 0),
        _ => egui::Color32::from_rgb(0, 255, 255),
    }
}

/// `FUN_004a90d0`'s rows for `page`, each filtered to the shown side, in the
/// order the page's walk yields them (the list's sort mode 4 keeps it).
///
/// port: the player sees the other side's objects only where the System
/// window shows them (`opposing_contents_visible`). The walks follow the
/// port's lists: characters in key order, then special forces, regiments
/// and facilities in their system's order; squadrons by class in the
/// system's order, one row each.
fn page_rows(
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    player: Faction,
    system: SystemKey,
    page: DefensesPage,
) -> Vec<DefensesRow> {
    let Some(value) = world.systems.get(system) else {
        return Vec::new();
    };
    if value.exploration_status == ExplorationStatus::Unexplored {
        return Vec::new();
    }
    let side = shown_side(world, system);
    if side != faction_side(player) && !opposing_contents_visible(world, fog, player, system) {
        return Vec::new();
    }
    match page {
        // FUN_00536e20: members with no mission key, or on a hidden mission
        // (`+0x78` bit 8).
        DefensesPage::Personnel => {
            let keys = mission_keys(missions);
            system_members(world, system)
                .filter(|(member, member_side, hidden)| {
                    *member_side == side && (*hidden || !keys.contains_key(member))
                })
                .filter_map(|(member, _, _)| match member {
                    MissionMember::Character(key) => {
                        let character = world.characters.get(key)?;
                        Some(DefensesRow {
                            item: DefensesItem::Character(key),
                            mini: character_mini_resource_id(character.dat_id, character.is_major)?,
                            label: character.display_name().to_owned(),
                        })
                    }
                    MissionMember::SpecialForce(key) => {
                        let force = world.special_forces.get(key)?;
                        let (mini, label) = special_force_mini(force.class_dat_id)?;
                        Some(DefensesRow {
                            item: DefensesItem::SpecialForce(key),
                            mini,
                            label: label.into(),
                        })
                    }
                })
                .collect()
        }
        // FUN_00504cc0.
        DefensesPage::Regiments => value
            .ground_units
            .iter()
            .filter_map(|&key| {
                let troop = world.troops.get(key)?;
                if fleet_side(troop.is_alliance) != side {
                    return None;
                }
                let (mini, label) = troop_mini(troop.class_dat_id)?;
                Some(DefensesRow {
                    item: DefensesItem::Troop(key),
                    mini,
                    label: label.into(),
                })
            })
            .collect(),
        // FUN_00503a50: the system's own squadrons, one item each.
        DefensesPage::Squadrons => value
            .fighters
            .iter()
            .filter(|entry| fleet_side(entry.is_alliance) == side)
            .filter_map(|entry| {
                let class = world.fighter_classes.get(entry.class)?;
                let mini = fighter_mini_id(class.dat_id)?;
                Some((0..entry.count).map(move |nth| DefensesRow {
                    item: DefensesItem::Squadron {
                        class: entry.class,
                        nth,
                    },
                    mini,
                    label: class.name.clone(),
                }))
            })
            .flatten()
            .collect(),
        // FUN_00527050, keeping families 0x24 and 0x25 (page 4) or 0x22 and
        // 0x23 (page 5).
        DefensesPage::Shields | DefensesPage::Batteries => value
            .defense_facilities
            .iter()
            .filter_map(|&key| {
                let facility = world.defense_facilities.get(key)?;
                let family = facility.class_dat_id.family();
                let wanted = if page == DefensesPage::Shields {
                    matches!(family, 0x24 | 0x25)
                } else {
                    matches!(family, 0x22 | 0x23)
                };
                if !wanted || crate::fleet_window::faction_side(facility.side) != side {
                    return None;
                }
                let (mini, label) = defense_facility_mini(facility.class_dat_id)?;
                Some(DefensesRow {
                    item: DefensesItem::Defense(key),
                    mini,
                    label: label.into(),
                })
            })
            .collect(),
    }
}

/// The shared tail of `FUN_004a90d0`: on the regiment page, when the shown
/// side is the player's (`FUN_0041cdb0`), TEXTSTRA 6471 "Garrison
/// Requirement: " and the system's stored requirement (`+0x80`).
fn garrison_line(
    world: &GameWorld,
    economy: &EconomyState,
    player: Faction,
    system: SystemKey,
    page: DefensesPage,
) -> Option<String> {
    if page != DefensesPage::Regiments || shown_side(world, system) != faction_side(player) {
        return None;
    }
    let requirement = economy
        .per_system
        .get(&system)
        .map_or(0, |value| value.garrison_requirement);
    Some(format!("Garrison Requirement: {requirement}"))
}

fn cockpit_faction(faction: CockpitFaction) -> Faction {
    match faction {
        CockpitFaction::Alliance => Faction::Alliance,
        CockpitFaction::Empire => Faction::Empire,
    }
}

fn area_id(window: &OpenDefensesWindow) -> egui::Id {
    egui::Id::new(("original-defenses-window", window.id, window.system))
}

fn window_screen_rect(window: &OpenDefensesWindow, layout: CockpitLayout) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + f32::from(window.logical_position.0) * layout.scale,
            layout.canvas.y + f32::from(window.logical_position.1) * layout.scale,
        ),
        egui::vec2(
            DEFENSES_WINDOW_WIDTH * layout.scale,
            DEFENSES_WINDOW_HEIGHT * layout.scale,
        ),
    )
}

fn tab_rect(window: egui::Rect, scale: f32, page: DefensesPage) -> egui::Rect {
    logical_rect(
        window,
        scale,
        TAB_STRIP.0 + page.x(),
        TAB_STRIP.1,
        TAB_SIZE.0,
        TAB_SIZE.1,
    )
}

/// The `index`th grid cell (`FUN_00609ae0`): row-major from the list's
/// corner, three to a row.
#[expect(clippy::cast_precision_loss, reason = "Cell indexes are small.")]
fn cell_rect(window: egui::Rect, scale: f32, index: usize) -> egui::Rect {
    let column = (index % COLUMNS) as f32;
    let row = (index / COLUMNS) as f32;
    logical_rect(
        window,
        scale,
        LIST.0 + column * CELL,
        LIST.1 + row * CELL,
        CELL,
        CELL,
    )
}

#[derive(Default)]
struct WindowDrawResult {
    focus: bool,
    close: bool,
    minimize: bool,
    open_sector: bool,
    page: Option<DefensesPage>,
    select: Option<DefensesItem>,
    /// A press on the list's empty space clears the selection
    /// (`FUN_006094b0`).
    deselect: bool,
    /// The selection and canvas point of a right release on the list.
    object_menu: Option<(Option<MenuObject>, (i16, i16))>,
    drag: Option<RowDrag>,
    /// The list's first row and bar press after this frame.
    scroll: Option<(usize, ScrollDrag)>,
    /// Normalized physical phases that causally produced this frame's
    /// mutations, in backend order with duplicates removed.
    causal_pointer_phases: Vec<PointerPhase>,
}

impl WindowDrawResult {
    fn note_pointer_phase(&mut self, phase: PointerPhase) {
        if !self.causal_pointer_phases.contains(&phase) {
            self.causal_pointer_phases.push(phase);
        }
    }
}

/// Draw every open Defenses window.
///
/// port: the window's keys (`FUN_004aa2b0`: Escape closes, left and right
/// step the tabs) are not ported; the galaxy view's windows keep no shared
/// keyboard focus to route them by.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit state and rendering inputs at this UI boundary, as the Fleet window does."
)]
pub fn draw_defenses_windows(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    economy: &EconomyState,
    state: &mut DefensesWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Vec<DefensesWindowAction> {
    draw_defenses_windows_with_snapshot(
        ctx,
        world,
        fog,
        missions,
        economy,
        state,
        faction,
        layout,
        cache,
        None,
        &[],
        None,
    )
}

/// Draw manager-owned Defenses content in the frozen bottom-to-top paint
/// order and admit mutations only through routed pointer events.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit state and rendering inputs at this UI boundary, as the Fleet window does."
)]
pub fn draw_managed_defenses_windows(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    economy: &EconomyState,
    state: &mut DefensesWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    snapshot: &RoutingSnapshot,
    pointer_events: &[(InputEventId, PointerPhase)],
    router: &mut FrameInputRouter,
) -> Vec<DefensesWindowAction> {
    draw_defenses_windows_with_snapshot(
        ctx,
        world,
        fog,
        missions,
        economy,
        state,
        faction,
        layout,
        cache,
        Some(snapshot),
        pointer_events,
        Some(router),
    )
}

fn claim_defenses_pointer_event(
    router: &mut FrameInputRouter,
    events: &[(InputEventId, PointerPhase)],
    window: WindowId,
    phase: PointerPhase,
) -> Option<InputEventId> {
    events.iter().find_map(|(event, candidate)| {
        (*candidate == phase
            && router.pointer_recipient(*event) == InputRecipient::Window(window)
            && router.claim(*event, InputRecipient::Window(window)))
        .then_some(*event)
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "Shared implementation for production snapshots and fixture-only direct content."
)]
fn draw_defenses_windows_with_snapshot(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    economy: &EconomyState,
    state: &mut DefensesWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    snapshot: Option<&RoutingSnapshot>,
    pointer_events: &[(InputEventId, PointerPhase)],
    mut router: Option<&mut FrameInputRouter>,
) -> Vec<DefensesWindowAction> {
    if state.windows.iter().all(|window| window.id.is_none()) {
        state.prepare_faction(faction);
    }
    if let Some(snapshot) = snapshot {
        state.refresh_release_targets(snapshot, world);
    } else {
        for window in &mut state.windows {
            window.release_target_visible =
                window.id.is_none() && world.systems.contains_key(window.system);
        }
    }
    let mut windows = Vec::new();
    let mut stale_manager_windows = Vec::new();
    if let Some(snapshot) = snapshot {
        for &id in snapshot.paint_order() {
            let Some(key) = snapshot
                .key(id)
                .filter(|key| key.kind == WindowKind::Defenses)
            else {
                continue;
            };
            match state
                .windows
                .iter()
                .find(|window| window.id == Some(id))
                .cloned()
            {
                Some(window) if window.release_target_visible && key.subject == window.subject => {
                    windows.push(window);
                }
                Some(_) | None => stale_manager_windows.push(id),
            }
        }
    }
    // Direct content exists only in renderer tests and explicit fixtures.
    windows.extend(
        state
            .windows
            .iter()
            .filter(|window| window.id.is_none())
            .cloned(),
    );
    let focused_manager = snapshot.and_then(|frame| frame.focused().map(|focus| focus.window));
    let focused_fixture_system = windows
        .last()
        .filter(|window| window.id.is_none())
        .map(|window| window.system);
    for &id in &stale_manager_windows {
        state.cancel_pointer_capture(id);
    }
    let ended_drag = state.end_drag(ctx, layout.scale, router.as_deref_mut(), pointer_events);
    let mut actions = stale_manager_windows
        .into_iter()
        .map(DefensesWindowAction::Close)
        .collect::<Vec<_>>();
    actions.extend(ended_drag.actions);
    for window in &windows {
        let focused = match window.id {
            Some(id) => focused_manager == Some(id),
            None => focused_fixture_system == Some(window.system),
        };
        let result = draw_defenses_window(
            ctx, world, fog, missions, economy, window, focused, faction, layout, cache,
        );
        let system = window.system;
        let scroll_changed = result
            .scroll
            .as_ref()
            .is_some_and(|(first, scroll)| *first != window.first_row || *scroll != window.scroll);
        let pointer_mutation = result.focus
            || result.close
            || result.minimize
            || result.open_sector
            || result.page.is_some()
            || result.select.is_some()
            || result.deselect
            || result.object_menu.is_some()
            || result.drag.is_some()
            || scroll_changed;
        let pointer_permitted = match (window.id, pointer_mutation, router.as_deref_mut()) {
            (Some(id), true, Some(router)) => {
                !result.causal_pointer_phases.is_empty()
                    && result.causal_pointer_phases.iter().all(|&phase| {
                        (phase == PointerPhase::Release(PointerButtonId::Primary)
                            && ended_drag.claimed_primary_release_by == Some(id))
                            || claim_defenses_pointer_event(router, pointer_events, id, phase)
                                .is_some()
                    })
            }
            (Some(_), true, None) => false,
            _ => true,
        };
        if pointer_permitted {
            if let Some(drag) = result.drag {
                state.drag = Some(drag);
                state.drag_owner = window.id;
                if let Some(id) = window.id {
                    actions.push(DefensesWindowAction::CapturePointer(id));
                }
            }
        }
        if (result.close && pointer_permitted) || !world.systems.contains_key(system) {
            if let Some(id) = window.id {
                actions.push(DefensesWindowAction::Close(id));
            } else {
                state.close(system);
            }
            continue;
        }
        if result.minimize && pointer_permitted {
            if let Some(id) = window.id {
                actions.push(DefensesWindowAction::MinimizeManaged {
                    window: id,
                    system,
                    logical_position: window.logical_position,
                });
            } else if let Some(closed) = state.close(system) {
                actions.push(DefensesWindowAction::Minimize {
                    system,
                    logical_position: closed.logical_position,
                });
            }
            continue;
        }
        if result.open_sector && pointer_permitted {
            actions.push(DefensesWindowAction::OpenSector(system));
        }
        if pointer_permitted {
            let Some(open) = state.window_mut(system) else {
                continue;
            };
            // FUN_004a90d0 refills only for a different page.
            if let Some((first_row, scroll)) = result.scroll {
                open.first_row = first_row;
                open.scroll = scroll;
            }
            // hyp: a refill shows the new page from its top.
            if let Some(page) = result.page.filter(|&page| page != open.page) {
                open.page = page;
                open.selected = None;
                open.first_row = 0;
            }
            if result.deselect {
                open.selected = None;
            }
            if let Some(item) = result.select {
                open.selected = Some(item);
            }
        }
        if result.focus && pointer_permitted {
            if let Some(id) = window.id {
                actions.push(DefensesWindowAction::Activate(id));
            } else {
                state.focus(system);
            }
            actions.push(DefensesWindowAction::SelectSystem(system));
        }
        if pointer_permitted {
            if let Some((selection, point)) = result.object_menu {
                actions.push(DefensesWindowAction::OpenObjectMenu {
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
    clippy::too_many_lines,
    reason = "Keep the window's ordered paint and input pass together, as the Fleet window does."
)]
fn draw_defenses_window(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    economy: &EconomyState,
    window: &OpenDefensesWindow,
    focused: bool,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> WindowDrawResult {
    let mut result = WindowDrawResult::default();
    let Some(system) = world.systems.get(window.system) else {
        result.close = true;
        return result;
    };
    let player = cockpit_faction(faction);
    let side = shown_side(world, window.system);
    let rows = page_rows(world, fog, missions, player, window.system, window.page);
    let scale = layout.scale;
    let screen_rect = window_screen_rect(window, layout);
    let id = area_id(window);
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
            let paint = |cache: &mut BmpCache, id: u32, x: f32, y: f32| {
                paint_native(
                    &painter,
                    ctx,
                    cache,
                    DllSource::Strategy,
                    id,
                    local,
                    scale,
                    x,
                    y,
                );
            };

            painter.rect_filled(local, 0.0, egui::Color32::BLACK);
            paint(cache, BACKGROUND, 0.0, 0.0);
            paint(cache, title_resource(side, focused), 2.0, 2.0);
            // The title label `+0x49` starts after the sector button, in
            // font 5 with format `0x24`: one line, left-aligned and
            // vertically centred (`FUN_004a8790`).
            painter.text(
                logical_rect(local, scale, TITLE.0, TITLE.1 + TITLE.2 / 2.0, 0.0, 0.0).min,
                egui::Align2::LEFT_CENTER,
                &system.name,
                egui::FontId::proportional((11.0 * scale).max(7.0)),
                egui::Color32::BLACK,
            );

            let buttons = [
                (SECTOR_NORMAL, SECTOR_PRESSED, 3.0, "sector"),
                (MINIMIZE_NORMAL, MINIMIZE_PRESSED, 204.0, "minimize"),
                (CLOSE_NORMAL, CLOSE_PRESSED, 218.0, "close"),
            ];
            let mut clicked = [false; 3];
            for (index, (normal, pressed, x, name)) in buttons.into_iter().enumerate() {
                let rect = logical_rect(local, scale, x, 3.0, 14.0, 14.0);
                crate::control_registry::register_screen(
                    "Defenses",
                    ["Sector", "Minimize", "Close"][index],
                    rect,
                );
                let response = ui.interact(
                    rect,
                    ui.id().with((window.system, name)),
                    egui::Sense::click(),
                );
                let down = primary_down && pointer.is_some_and(|point| rect_contains(rect, point));
                paint(cache, if down { pressed } else { normal }, x, 3.0);
                clicked[index] = exact_clicked(&response, rect);
            }
            result.open_sector = clicked[0];
            result.minimize = clicked[1];
            result.close = clicked[2];
            if clicked.into_iter().any(|clicked| clicked) {
                result.note_pointer_phase(PointerPhase::Release(PointerButtonId::Primary));
            }

            // The tabs. No tab is ever disabled; an empty one opens its
            // empty page. port: the help messages (`FUN_00600a40`) are not
            // shown.
            for page in DefensesPage::ALL {
                let rect = tab_rect(local, scale, page);
                let empty = page_rows(world, fog, missions, player, window.system, page).is_empty();
                crate::control_registry::register_screen("Defenses", page.name(), rect);
                paint_native(
                    &painter,
                    ctx,
                    cache,
                    DllSource::Strategy,
                    tab_resource(page, side, page == window.page, empty),
                    rect,
                    scale,
                    0.0,
                    0.0,
                );
                let response = ui.interact(
                    rect,
                    ui.id().with((window.system, page)),
                    egui::Sense::click(),
                );
                if exact_clicked(&response, rect) {
                    result.page = Some(page);
                    result.focus = true;
                    result.note_pointer_phase(PointerPhase::Release(PointerButtonId::Primary));
                }
            }

            // `+0x5b` and `+0x5c`: white, font 4, format `0x25`: one line,
            // centred both ways in its rect.
            let label_font = egui::FontId::proportional((10.0 * scale).max(7.0));
            painter.text(
                logical_rect(
                    local,
                    scale,
                    TAB_NAME.0 + TAB_NAME.2 / 2.0,
                    TAB_NAME.1 + TAB_NAME.3 / 2.0,
                    0.0,
                    0.0,
                )
                .min,
                egui::Align2::CENTER_CENTER,
                window.page.name(),
                label_font.clone(),
                egui::Color32::WHITE,
            );
            if let Some(line) = garrison_line(world, economy, player, window.system, window.page) {
                painter.text(
                    logical_rect(
                        local,
                        scale,
                        GARRISON.0 + GARRISON.2 / 2.0,
                        GARRISON.1 + GARRISON.3 / 2.0,
                        0.0,
                        0.0,
                    )
                    .min,
                    egui::Align2::CENTER_CENTER,
                    line,
                    label_font,
                    egui::Color32::WHITE,
                );
            }

            // The list: each row's image keyed at its cell's corner, the
            // frame over the mini while selected (`FUN_0060bd00`), and its
            // name word-wrapped from (1, 27). FUN_004a8790 gives it the bar
            // of RCDATA 0x29cc (FUN_0060a490); a unit is a row of three
            // cells. port: the list's wheel is untraced and not handled.
            let list = logical_rect(local, scale, LIST.0, LIST.1, LIST.2, LIST.3);
            let bar = ScrollBar::new(LIST_BAR_29CC, LIST.0, LIST.1, LIST.2, LIST.3);
            let range = ScrollRange {
                first: window.first_row,
                visible: VISIBLE_ROWS as f32,
                total: rows.len().div_ceil(COLUMNS),
            };
            let pointer = egui_pointer(ctx, local, scale);
            let mut scroll = window.scroll;
            let first_row = bar.update(&mut scroll, range, VISIBLE_ROWS, pointer);
            result.scroll = Some((first_row, scroll));
            if first_row != window.first_row || scroll != window.scroll {
                result.focus = true;
                if pointer.pressed {
                    result.note_pointer_phase(PointerPhase::Press(PointerButtonId::Primary));
                }
                if pointer.down && !pointer.pressed {
                    result.note_pointer_phase(PointerPhase::Motion(PointerButtonId::Primary));
                }
                if pointer.released {
                    result.note_pointer_phase(PointerPhase::Release(PointerButtonId::Primary));
                }
            }
            let range = scroll_range_after_input(range, first_row);
            let bar_left = LIST.0 + LIST.2 - bar.width();
            let on_bar = range.overflows()
                && pointer.at.is_some_and(|(x, y)| {
                    (bar_left..LIST.0 + LIST.2).contains(&x)
                        && (LIST.1..LIST.1 + LIST.3).contains(&y)
                });
            let list_painter = painter.with_clip_rect(list);
            // The list control under the rows (FUN_006083c0): a left or
            // right press on its empty space clears the selection
            // (FUN_006094b0), and a right release there opens the menu for
            // the empty selection.
            let list_response = ui.interact(
                list,
                ui.id().with((window.system, "list")),
                egui::Sense::click(),
            );
            let (primary_pressed, right_pressed, release_point) = ctx.input(|input| {
                (
                    input.pointer.button_pressed(egui::PointerButton::Primary),
                    input.pointer.button_pressed(egui::PointerButton::Secondary),
                    input.pointer.interact_pos(),
                )
            });
            let any_pressed = primary_pressed || right_pressed;
            if any_pressed && list_response.is_pointer_button_down_on() && !on_bar {
                result.deselect = true;
                result.focus = true;
                result.note_pointer_phase(PointerPhase::Press(if right_pressed {
                    PointerButtonId::Secondary
                } else {
                    PointerButtonId::Primary
                }));
            }
            if list_response.secondary_clicked() {
                if let Some(point) = release_point.filter(|point| rect_contains(list, *point)) {
                    result.object_menu = Some((None, canvas_point(layout, point)));
                    result.note_pointer_phase(PointerPhase::Release(PointerButtonId::Secondary));
                }
            }
            let font = egui::FontId::proportional((9.0 * scale).max(6.0));
            let frame = if side == 1 {
                ROW_FRAME[0]
            } else {
                ROW_FRAME[1]
            };
            let shown = rows
                .iter()
                .enumerate()
                .skip(first_row * COLUMNS)
                .take(COLUMNS * VISIBLE_ROWS);
            for (cell_index, (index, row)) in shown.enumerate() {
                let cell = cell_rect(local, scale, cell_index);
                if let (DefensesItem::Character(key), true) =
                    (row.item, crate::control_registry::enabled())
                {
                    if let Some(character) = world.characters.get(key) {
                        crate::control_registry::register_screen(
                            "Personnel",
                            &character.name,
                            cell.intersect(list),
                        );
                    }
                }
                let selected = window.selected == Some(row.item);
                // The list image (`FUN_004a9ab0` -> `FUN_0042c3b0(.., 0, 1)`):
                // the backdrop, a member's en route mark, then the mini.
                let (object, en_route) = match row.item {
                    DefensesItem::Character(key) => (
                        MiniObject::Character,
                        world.characters.get(key).is_some_and(|c| c.en_route),
                    ),
                    DefensesItem::SpecialForce(key) => world
                        .special_forces
                        .get(key)
                        .map_or((MiniObject::Craft, false), |force| {
                            (MiniObject::SpecialForce(force.class_dat_id), force.en_route)
                        }),
                    DefensesItem::Troop(key) => (
                        world.troops.get(key).map_or(MiniObject::Craft, |troop| {
                            MiniObject::Regiment(troop.class_dat_id)
                        }),
                        false,
                    ),
                    DefensesItem::Squadron { .. } | DefensesItem::Defense(_) => {
                        (MiniObject::Craft, false)
                    }
                };
                for (source, bitmap) in mini_layers(object, row.mini, en_route) {
                    paint_native(
                        &list_painter,
                        ctx,
                        cache,
                        source,
                        bitmap,
                        cell,
                        scale,
                        0.0,
                        0.0,
                    );
                }
                if selected {
                    paint_native(
                        &list_painter,
                        ctx,
                        cache,
                        DllSource::Strategy,
                        frame,
                        cell,
                        scale,
                        0.0,
                        0.0,
                    );
                }
                let color = if selected {
                    selected_color(side)
                } else {
                    egui::Color32::WHITE
                };
                let galley = list_painter.layout(
                    row.label.clone(),
                    font.clone(),
                    color,
                    (CELL - TEXT_RIGHT_TRIM) * scale,
                );
                list_painter.galley(
                    logical_rect(cell, scale, TEXT_OFFSET.0, TEXT_OFFSET.1, 0.0, 0.0).min,
                    galley,
                    color,
                );
                let response = ui.interact(
                    cell.intersect(list),
                    ui.id().with((window.system, "row", index)),
                    egui::Sense::click(),
                );
                if exact_clicked(&response, cell) {
                    result.select = Some(row.item);
                    result.focus = true;
                    result.note_pointer_phase(PointerPhase::Release(PointerButtonId::Primary));
                }
                // A right press selects the row as a left press does
                // (FUN_006083c0 shares the WM_LBUTTONDOWN path); the release
                // opens its menu.
                if right_pressed && response.is_pointer_button_down_on() {
                    result.select = Some(row.item);
                    result.focus = true;
                    result.note_pointer_phase(PointerPhase::Press(PointerButtonId::Secondary));
                }
                // A left press on a squadron, character or special force
                // captures the mouse for a drag (FUN_006083c0).
                let draggable = matches!(
                    row.item,
                    DefensesItem::Squadron { .. }
                        | DefensesItem::Character(_)
                        | DefensesItem::SpecialForce(_)
                );
                if let (true, Some(press)) = (
                    draggable,
                    ctx.input(|input| {
                        input
                            .pointer
                            .button_pressed(egui::PointerButton::Primary)
                            .then(|| input.pointer.press_origin())
                            .flatten()
                    }),
                ) {
                    if response.is_pointer_button_down_on() && rect_contains(cell, press) {
                        result.drag = Some(RowDrag {
                            system: window.system,
                            item: row.item,
                            is_alliance: side == 1,
                            press,
                            list,
                        });
                        result.note_pointer_phase(PointerPhase::Press(PointerButtonId::Primary));
                    }
                }
                if response.secondary_clicked() {
                    if let (Some(object), Some(point)) = (
                        row.item.menu_object(window.system, side),
                        response
                            .interact_pointer_pos()
                            .filter(|point| rect_contains(cell.intersect(list), *point)),
                    ) {
                        result.object_menu = Some((Some(object), canvas_point(layout, point)));
                        result
                            .note_pointer_phase(PointerPhase::Release(PointerButtonId::Secondary));
                    }
                }
            }

            crate::scroll_bar::paint_egui(
                &painter,
                ctx,
                cache,
                local,
                scale,
                &bar.blits(range, &scroll, pointer),
            );

            if window_response.clicked() || clicked.iter().any(|&value| value) {
                result.focus = true;
                result.note_pointer_phase(PointerPhase::Release(PointerButtonId::Primary));
            }
        });
    if area.response.clicked() {
        result.focus = true;
        result.note_pointer_phase(PointerPhase::Release(PointerButtonId::Primary));
    }
    result
}

/// Keep the painted thumb in the same committed row as the list contents.
fn scroll_range_after_input(range: ScrollRange, first_row: usize) -> ScrollRange {
    ScrollRange {
        first: first_row,
        ..range
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
    use crate::fleet_window::tests::PAINTED;
    use rebellion_core::dat::SectorGroup;
    use rebellion_core::economy::SystemEconomy;
    use rebellion_core::ids::DatId;
    use rebellion_core::missions::{MissionFaction, MissionKind, MissionRequest};
    use rebellion_core::world::{
        Character, ControlKind, DefenseFacilityInstance, Sector, SpecialForceUnit, System,
        TroopUnit,
    };
    use rebellion_windowing::{WindowCommand, WindowManager, WindowTransition};

    const KDY: u32 = 0x2200_0001;
    const LNR: u32 = 0x2300_0002;
    const GENCORE: u32 = 0x2400_0003;
    const DEATH_STAR_SHIELD: u32 = 0x2500_0004;
    const ALLIANCE_ARMY: u32 = 0x1000_0002;
    const STORMTROOPERS: u32 = 0x1000_0006;

    fn world(control: ControlKind) -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(36),
            name: "Sesswenna".into(),
            group: SectorGroup::Core,
            x: 317,
            y: 248,
            systems: Vec::new(),
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(100),
            name: "Sluis Van".into(),
            sector,
            x: 320,
            y: 250,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
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
            control,
        });
        (world, system)
    }

    fn add_defense(world: &mut GameWorld, system: SystemKey, dat_id: u32, is_alliance: bool) {
        let key = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: DatId::new(dat_id),
            side: rebellion_core::dat::Faction::of_alliance(is_alliance),
        });
        world.systems[system].defense_facilities.push(key);
    }

    fn add_troop(world: &mut GameWorld, system: SystemKey, dat_id: u32, is_alliance: bool) {
        let key = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(dat_id),
            is_alliance,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(key);
    }

    fn add_character(
        world: &mut GameWorld,
        system: SystemKey,
        name: &str,
        is_alliance: bool,
    ) -> CharacterKey {
        world.characters.insert(Character {
            dat_id: DatId::new(832),
            name: name.into(),
            is_alliance,
            is_empire: !is_alliance,
            current_system: Some(system),
            recruited: true,
            ..Default::default()
        })
    }

    fn send_on_mission(missions: &mut MissionState, member: CharacterKey, system: SystemKey) {
        missions.dispatch(MissionRequest::single(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            member,
            system,
            None,
            0,
        ));
    }

    fn seen(system: SystemKey) -> FogState {
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(system);
        fog
    }

    fn labels(
        world: &GameWorld,
        fog: &FogState,
        missions: &MissionState,
        system: SystemKey,
        page: DefensesPage,
    ) -> Vec<String> {
        page_rows(world, fog, missions, Faction::Alliance, system, page)
            .into_iter()
            .map(|row| row.label)
            .collect()
    }

    /// An Alliance system holding one of everything, beside the Empire's
    /// equivalents, which the window never lists there.
    fn stocked() -> (GameWorld, SystemKey, MissionState) {
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        for dat_id in [KDY, GENCORE, LNR, DEATH_STAR_SHIELD] {
            add_defense(&mut world, system, dat_id, true);
            add_defense(&mut world, system, dat_id, false);
        }
        add_troop(&mut world, system, ALLIANCE_ARMY, true);
        add_troop(&mut world, system, STORMTROOPERS, false);
        add_character(&mut world, system, "Idle", true);
        let away = add_character(&mut world, system, "Away", true);
        let hidden = add_character(&mut world, system, "Hidden", true);
        world.characters[hidden].on_hidden_mission = true;
        add_character(&mut world, system, "Enemy", false);
        let force = world.special_forces.insert(SpecialForceUnit {
            class_dat_id: DatId::new(0x3c00_0001),
            is_alliance: true,
            skills: [0; 8],
            on_mission: false,
            en_route: false,
            on_hidden_mission: false,
        });
        world.systems[system].special_forces.push(force);
        let mut missions = MissionState::new();
        send_on_mission(&mut missions, away, system);
        send_on_mission(&mut missions, hidden, system);
        (world, system, missions)
    }

    #[test]
    fn each_page_lists_the_holding_sides_objects_of_its_families_in_walk_order() {
        // FUN_004a90d0: pages 5 and 4 keep families 0x22/0x23 and 0x24/0x25
        // of the FUN_00527050 walk; page 2 is FUN_00504cc0; page 1 keeps
        // members with no mission key or on a hidden mission (+0x78 bit 8);
        // all filtered to the system's side (+0x148).
        let (world, system, missions) = stocked();
        let fog = seen(system);
        let page = |page| labels(&world, &fog, &missions, system, page);

        assert_eq!(page(DefensesPage::Batteries), ["KDY-150", "LNR Series I"]);
        assert_eq!(
            page(DefensesPage::Shields),
            ["GenCore Level I", "Death Star Shield"]
        );
        assert_eq!(page(DefensesPage::Regiments), ["Alliance Army Regiment"]);
        // port: squadrons live only aboard fleets.
        assert!(page(DefensesPage::Squadrons).is_empty());
        assert_eq!(
            page(DefensesPage::Personnel),
            ["Idle", "Hidden", "Guerrillas"]
        );
    }

    #[test]
    fn an_enemy_held_system_lists_the_enemys_objects_only_while_the_player_sees_them() {
        // +0x148 is the system's side, not the player's. port: the other
        // side's objects show only where the System window shows them.
        let (mut world, system, missions) = stocked();
        world.systems[system].control = ControlKind::Controlled(Faction::Empire);
        let unseen = FogState::new(Faction::Alliance);

        assert!(labels(&world, &unseen, &missions, system, DefensesPage::Batteries).is_empty());
        assert_eq!(
            labels(
                &world,
                &seen(system),
                &missions,
                system,
                DefensesPage::Regiments
            ),
            ["Stormtrooper Regiment"]
        );
        assert_eq!(
            labels(
                &world,
                &seen(system),
                &missions,
                system,
                DefensesPage::Personnel
            ),
            ["Enemy"]
        );
    }

    #[test]
    fn an_unheld_or_unexplored_system_lists_nothing_of_either_side() {
        // Side bits 0 or 3 match neither side's objects.
        let (mut world, system, missions) = stocked();
        let fog = seen(system);
        for control in [ControlKind::Uncontrolled, ControlKind::Contested] {
            world.systems[system].control = control;
            for page in DefensesPage::ALL {
                assert!(labels(&world, &fog, &missions, system, page).is_empty());
            }
        }
        world.systems[system].control = ControlKind::Controlled(Faction::Alliance);
        world.systems[system].exploration_status = ExplorationStatus::Unexplored;
        assert!(labels(&world, &fog, &missions, system, DefensesPage::Batteries).is_empty());
    }

    #[test]
    fn each_tab_shows_its_side_art_selected_or_empty() {
        // FUN_004a8790 and FUN_004a9ce0: normal, the empty id two higher,
        // the middle id selected; FUN_0060d700 copies the pressed bitmap
        // into the unset selected state of tabs 4 and 5.
        let cases = [
            (DefensesPage::Batteries, 1, [10550, 10551, 10552]),
            (DefensesPage::Batteries, 3, [10550, 10551, 10552]),
            (DefensesPage::Shields, 2, [10553, 10554, 10555]),
            (DefensesPage::Squadrons, 1, [10556, 10557, 10558]),
            (DefensesPage::Squadrons, 2, [10559, 10560, 10561]),
            (DefensesPage::Squadrons, 0, [10562; 3]),
            (DefensesPage::Regiments, 1, [10563, 10564, 10565]),
            (DefensesPage::Regiments, 2, [10566, 10567, 10568]),
            (DefensesPage::Regiments, 3, [10569; 3]),
            (DefensesPage::Personnel, 1, [10570, 10571, 10572]),
            (DefensesPage::Personnel, 2, [10573, 10574, 10575]),
            (DefensesPage::Personnel, 0, [10576; 3]),
        ];
        for (page, side, [normal, selected, empty]) in cases {
            assert_eq!(
                [
                    tab_resource(page, side, false, false),
                    tab_resource(page, side, true, true),
                    tab_resource(page, side, false, true),
                ],
                [normal, selected, empty],
                "{page:?} side {side}"
            );
        }
        // TEXTSTRA 5952..5956.
        assert_eq!(
            DefensesPage::ALL.map(DefensesPage::name),
            [
                "Personnel",
                "Trooper Regiments",
                "Fighter Squadrons",
                "Planetary Shields",
                "Planetary Batteries",
            ]
        );
    }

    #[test]
    fn the_garrison_line_shows_on_the_regiment_page_of_the_players_own_system() {
        // FUN_004a90d0's tail: TEXTSTRA 6471 and +0x80 on page 2 when the
        // side bits are the player's (FUN_0041cdb0).
        let (world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let mut economy = EconomyState::default();
        economy.per_system.insert(
            system,
            SystemEconomy {
                garrison_requirement: 3,
                ..SystemEconomy::default()
            },
        );
        let line = |player, page| garrison_line(&world, &economy, player, system, page);

        assert_eq!(
            line(Faction::Alliance, DefensesPage::Regiments).as_deref(),
            Some("Garrison Requirement: 3")
        );
        assert_eq!(line(Faction::Alliance, DefensesPage::Personnel), None);
        assert_eq!(line(Faction::Empire, DefensesPage::Regiments), None);
        assert_eq!(
            garrison_line(
                &world,
                &EconomyState::default(),
                Faction::Alliance,
                system,
                DefensesPage::Regiments
            )
            .as_deref(),
            Some("Garrison Requirement: 0")
        );
    }

    #[test]
    fn the_rail_icon_title_strip_and_selected_color_follow_the_systems_side() {
        // FUN_004aa4a0: 11533, 11534, else 11535; FUN_004a9800's strips;
        // FUN_004a9ce0's +0xdc.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Empire));
        assert_eq!(rail_icon(&world, system), 11534);
        world.systems[system].control = ControlKind::Controlled(Faction::Alliance);
        assert_eq!(rail_icon(&world, system), 11533);
        world.systems[system].control = ControlKind::Contested;
        assert_eq!(rail_icon(&world, system), 11535);

        assert_eq!(
            [1, 2, 3].map(|side| [title_resource(side, true), title_resource(side, false)]),
            [[10299, 10200], [10201, 10302], [10303, 10304]]
        );
        assert_eq!(selected_color(1), egui::Color32::from_rgb(255, 0, 0));
        assert_eq!(selected_color(2), egui::Color32::from_rgb(0, 255, 0));
        assert_eq!(selected_color(0), egui::Color32::from_rgb(0, 255, 255));
    }

    /// The canvas 10 by 20 pixels in and twice the original size.
    fn scaled() -> CockpitLayout {
        let viewport = CockpitViewport {
            x: 10.0,
            y: 20.0,
            width: 1280.0,
            height: 960.0,
        };
        CockpitLayout {
            canvas: viewport,
            galaxy: viewport,
            scale: 2.0,
        }
    }

    const ORIGIN: (i16, i16) = (20, 30);

    fn commit_prepared_defenses_window(
        manager: &mut WindowManager,
        state: &mut DefensesWindowState,
        prepared: PreparedDefensesWindow,
    ) -> (WindowId, WindowTransition) {
        let commit = manager
            .apply(WindowCommand::Open(prepared.spec()))
            .expect("prepared Defenses identity is accepted");
        let id = state
            .commit_open(prepared, &commit.transition)
            .expect("matching manager receipt publishes Defenses content");
        (id, commit.transition)
    }

    #[test]
    fn defenses_window_shared_identity_preserves_content() {
        let (world, system, _) = stocked();
        let mut manager = WindowManager::new();
        let mut state = DefensesWindowState::default();
        let prepared = state
            .prepare_open(&world, system, ORIGIN, CockpitFaction::Alliance, scaled())
            .expect("live system has a stable DAT subject");
        assert_eq!(prepared.spec().key.kind, WindowKind::Defenses);
        assert_eq!(prepared.spec().owner, WindowOwner::StrategicRoot);
        let (id, opened) = commit_prepared_defenses_window(&mut manager, &mut state, prepared);
        assert_eq!(opened, WindowTransition::Opened { id });

        let selected = DefensesItem::Defense(world.systems[system].defense_facilities[0]);
        let content = state.window_mut(system).unwrap();
        content.page = DefensesPage::Batteries;
        content.selected = Some(selected);
        content.first_row = 1;

        let reopened = state
            .prepare_open(
                &world,
                system,
                (400, 300),
                CockpitFaction::Alliance,
                scaled(),
            )
            .unwrap();
        let (reopened_id, transition) =
            commit_prepared_defenses_window(&mut manager, &mut state, reopened);
        assert_eq!(reopened_id, id);
        assert!(matches!(
            transition,
            WindowTransition::Existing { id: existing, .. } if existing == id
        ));
        assert_eq!(state.window_count(), 1);
        let content = state.window_mut(system).unwrap();
        assert_eq!(content.logical_position, ORIGIN);
        assert_eq!(content.page, DefensesPage::Batteries);
        assert_eq!(content.selected, Some(selected));
        assert_eq!(content.first_row, 1);

        manager.apply(WindowCommand::Minimize(id)).unwrap();
        assert!(!state.window_is_visible(&manager.begin_frame(), system));
        let restored = state
            .prepare_open(
                &world,
                system,
                (500, 350),
                CockpitFaction::Alliance,
                scaled(),
            )
            .unwrap();
        let (restored_id, transition) =
            commit_prepared_defenses_window(&mut manager, &mut state, restored);
        assert_eq!(restored_id, id);
        assert!(matches!(
            transition,
            WindowTransition::Existing { id: existing, .. } if existing == id
        ));
        assert!(state.window_is_visible(&manager.begin_frame(), system));
        let content = state.window_mut(system).unwrap();
        assert_eq!(content.logical_position, ORIGIN);
        assert_eq!(content.page, DefensesPage::Batteries);
        assert_eq!(content.selected, Some(selected));
        assert_eq!(content.first_row, 1);
    }

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn defenses_fixture_legacy_open_adopts_manager_identity_without_resetting_content() {
        let (world, system, _) = stocked();
        let mut manager = WindowManager::new();
        let mut state = DefensesWindowState::default();
        assert!(state.open(&world, system, ORIGIN, CockpitFaction::Alliance, scaled(),));
        let selected = DefensesItem::Defense(world.systems[system].defense_facilities[0]);
        let content = state.window_mut(system).unwrap();
        content.page = DefensesPage::Batteries;
        content.selected = Some(selected);
        content.first_row = 1;

        let prepared = state
            .prepare_open(
                &world,
                system,
                (400, 300),
                CockpitFaction::Alliance,
                scaled(),
            )
            .unwrap();
        let commit = manager.apply(WindowCommand::Open(prepared.spec())).unwrap();
        let id = match commit.transition {
            WindowTransition::Opened { id } => id,
            ref transition => panic!("expected Defenses open, got {transition:?}"),
        };

        assert_eq!(
            state.commit_open(prepared, &commit.transition),
            Some(id),
            "the accepted manager identity must attach to the exact fixture content"
        );
        assert_eq!(state.window_id(system), Some(id));
        assert_eq!(state.window_count(), 1);
        let content = state.window_mut(system).unwrap();
        assert_eq!(content.logical_position, ORIGIN);
        assert_eq!(content.page, DefensesPage::Batteries);
        assert_eq!(content.selected, Some(selected));
        assert_eq!(content.first_row, 1);
    }

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn defenses_fixture_legacy_open_rejects_wrong_faction_and_ambiguous_content() {
        let (world, system, _) = stocked();

        let mut wrong_faction_manager = WindowManager::new();
        let mut wrong_faction_state = DefensesWindowState::default();
        assert!(wrong_faction_state.open(
            &world,
            system,
            ORIGIN,
            CockpitFaction::Alliance,
            scaled(),
        ));
        let wrong_faction = wrong_faction_state
            .prepare_open(&world, system, ORIGIN, CockpitFaction::Empire, scaled())
            .unwrap();
        let wrong_faction_commit = wrong_faction_manager
            .apply(WindowCommand::Open(wrong_faction.spec()))
            .unwrap();
        assert_eq!(
            wrong_faction_state.commit_open(wrong_faction, &wrong_faction_commit.transition),
            None
        );
        assert_eq!(wrong_faction_state.window_id(system), None);
        assert_eq!(wrong_faction_state.window_count(), 1);

        let mut ambiguous_manager = WindowManager::new();
        let mut ambiguous_state = DefensesWindowState::default();
        assert!(ambiguous_state.open(&world, system, ORIGIN, CockpitFaction::Alliance, scaled(),));
        ambiguous_state
            .windows
            .push(ambiguous_state.windows[0].clone());
        let ambiguous = ambiguous_state
            .prepare_open(&world, system, ORIGIN, CockpitFaction::Alliance, scaled())
            .unwrap();
        let ambiguous_commit = ambiguous_manager
            .apply(WindowCommand::Open(ambiguous.spec()))
            .unwrap();
        assert_eq!(
            ambiguous_state.commit_open(ambiguous, &ambiguous_commit.transition),
            None
        );
        assert!(ambiguous_state
            .windows
            .iter()
            .all(|window| window.id.is_none() && window.subject.is_none()));
        assert_eq!(ambiguous_state.window_count(), 2);
    }

    #[test]
    fn defenses_window_stale_subject_does_not_retarget() {
        let (mut world, stale_system) = world(ControlKind::Controlled(Faction::Alliance));
        let mut manager = WindowManager::new();
        let mut state = DefensesWindowState::default();
        let prepared = state
            .prepare_open(
                &world,
                stale_system,
                ORIGIN,
                CockpitFaction::Alliance,
                scaled(),
            )
            .unwrap();
        let (id, _) = commit_prepared_defenses_window(&mut manager, &mut state, prepared);
        state.show_page(stale_system, DefensesPage::Shields);
        let layer = egui::LayerId::new(
            egui::Order::Foreground,
            area_id(
                state
                    .windows
                    .iter()
                    .find(|window| window.id == Some(id))
                    .unwrap(),
            ),
        );
        let valid = run_managed(
            &world,
            &MissionState::new(),
            &mut state,
            &manager,
            vec![(vec![egui::Event::PointerMoved(at(100.0, 100.0))], vec![])],
        );
        assert!(valid.actions.is_empty());
        assert_eq!(state.release_target(layer), Some(stale_system));

        world.systems[stale_system].dat_id = DatId::new(101);
        world.systems[stale_system].name = "Replacement".into();
        state.refresh_release_targets(&manager.begin_frame(), &world);
        assert_eq!(
            state.release_target(layer),
            None,
            "the app prepass must invalidate a stale layer before Fleet can resolve a drop"
        );
        let rendered = run_managed(
            &world,
            &MissionState::new(),
            &mut state,
            &manager,
            vec![(vec![egui::Event::PointerMoved(at(100.0, 100.0))], vec![])],
        );

        assert_eq!(rendered.actions, [DefensesWindowAction::Close(id)]);
        assert!(
            !rendered
                .painted
                .iter()
                .any(|(resource, _)| *resource == BACKGROUND),
            "stale content must close before it paints the replacement subject"
        );
        assert_eq!(state.window_id(stale_system), Some(id));
        assert_eq!(
            state.release_target(layer),
            None,
            "a drop or targeting release cannot resolve a stale same-key DAT replacement"
        );
        assert_eq!(
            state.window_mut(stale_system).unwrap().page,
            DefensesPage::Shields
        );
        assert_eq!(
            state.content_reports(&manager.begin_frame(), &world),
            [ContentReport {
                window: id,
                status: ContentStatus::Absent,
            }]
        );
    }

    #[test]
    fn defenses_window_manager_order_is_independent_of_content_vector() {
        let (mut world, first_system) = world(ControlKind::Controlled(Faction::Alliance));
        let mut second = world.systems[first_system].clone();
        second.dat_id = DatId::new(101);
        second.name = "Coruscant".into();
        let second_system = world.systems.insert(second);
        let systems = [first_system, second_system];
        let mut manager = WindowManager::new();
        let mut state = DefensesWindowState::default();
        let mut ids = Vec::new();
        for system in systems {
            let prepared = state
                .prepare_open(&world, system, ORIGIN, CockpitFaction::Alliance, scaled())
                .unwrap();
            ids.push(commit_prepared_defenses_window(&mut manager, &mut state, prepared).0);
        }
        assert_eq!(
            state
                .windows
                .iter()
                .filter_map(|window| window.id)
                .collect::<Vec<_>>(),
            ids
        );

        manager.apply(WindowCommand::Activate(ids[0])).unwrap();

        assert_eq!(
            state
                .windows
                .iter()
                .filter_map(|window| window.id)
                .collect::<Vec<_>>(),
            ids,
            "local content order must not emulate a manager raise"
        );
        assert_eq!(manager.begin_frame().paint_order(), &[ids[1], ids[0]]);
    }

    #[test]
    fn defenses_visible_content_follows_exact_frozen_identity() {
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let mut manager = WindowManager::new();
        let mut state = DefensesWindowState::default();
        let prepared = state
            .prepare_open(&world, system, ORIGIN, CockpitFaction::Alliance, scaled())
            .unwrap();
        let (id, _) = commit_prepared_defenses_window(&mut manager, &mut state, prepared);
        assert!(state.has_visible_content(&manager.begin_frame(), &world));

        manager.apply(WindowCommand::Minimize(id)).unwrap();
        assert!(!state.has_visible_content(&manager.begin_frame(), &world));

        manager.apply(WindowCommand::Restore(id)).unwrap();
        assert!(state.has_visible_content(&manager.begin_frame(), &world));

        world.systems[system].dat_id = DatId::new(101);
        assert!(
            !state.has_visible_content(&manager.begin_frame(), &world),
            "same-key replacement content is not the committed Defenses subject"
        );

        world.systems[system].dat_id = DatId::new(100);
        assert!(state.has_visible_content(&manager.begin_frame(), &world));
        manager.apply(WindowCommand::Close(id)).unwrap();
        assert!(
            !state.has_visible_content(&manager.begin_frame(), &world),
            "attached content stops contributing when its manager identity closes"
        );
    }

    #[test]
    fn defenses_pointer_hits_only_its_visible_topmost_manager_layer() {
        let (world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let mut manager = WindowManager::new();
        let mut state = DefensesWindowState::default();
        let prepared = state
            .prepare_open(&world, system, ORIGIN, CockpitFaction::Alliance, scaled())
            .unwrap();
        let (id, _) = commit_prepared_defenses_window(&mut manager, &mut state, prepared);
        let snapshot = manager.begin_frame();
        let rect = state.screen_rect(scaled(), system).unwrap();
        let inside = (rect.center().x, rect.center().y);
        let outside = (rect.max.x + 1.0, rect.max.y + 1.0);
        let content = state
            .windows
            .iter()
            .find(|window| window.id == Some(id))
            .unwrap();
        let own = egui::LayerId::new(egui::Order::Foreground, area_id(content));
        let other = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("other-window"));

        assert_eq!(
            state.hit_window_ids(&snapshot, scaled(), inside, Some(own)),
            std::collections::BTreeSet::from([id])
        );
        assert!(state
            .hit_window_ids(&snapshot, scaled(), inside, Some(other))
            .is_empty());
        assert!(state
            .hit_window_ids(&snapshot, scaled(), inside, None)
            .is_empty());
        assert!(state
            .hit_window_ids(&snapshot, scaled(), outside, Some(own))
            .is_empty());
    }

    /// A logical point in the window opened at `ORIGIN` on [`scaled`].
    fn at(x: f32, y: f32) -> egui::Pos2 {
        egui::pos2(
            10.0 + (f32::from(ORIGIN.0) + x) * 2.0,
            20.0 + (f32::from(ORIGIN.1) + y) * 2.0,
        )
    }

    fn opened(world: &GameWorld, system: SystemKey) -> DefensesWindowState {
        let mut state = DefensesWindowState::default();
        assert!(state.open(world, system, ORIGIN, CockpitFaction::Alliance, scaled()));
        state
    }

    #[test]
    fn the_windows_rects_and_release_target_follow_the_canvas_offset_and_scale() {
        // The strip at (0, 20) with 36 by 33 buttons; the list at (7, 81)
        // with 70 by 70 cells (FUN_004a8790, FUN_00609ae0); +0x70 gives the
        // subject (FUN_004aa470).
        let (world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let mut state = opened(&world, system);
        assert!(state.open(
            &world,
            system,
            (300, 100),
            CockpitFaction::Alliance,
            scaled()
        ));
        assert_eq!(state.window_count(), 1);
        assert!(state.is_open(system));

        assert_eq!(
            state.tab_screen_rect(scaled(), system, DefensesPage::Shields),
            Some(egui::Rect::from_min_size(
                at(136.0, 20.0),
                egui::vec2(72.0, 66.0)
            ))
        );
        assert_eq!(
            state.cell_screen_rect(scaled(), system, 4),
            Some(egui::Rect::from_min_size(
                at(77.0, 151.0),
                egui::vec2(140.0, 140.0)
            ))
        );
        let point = |p: egui::Pos2| (p.x, p.y);
        assert!(state.contains_screen_point(scaled(), point(at(234.0, 303.0))));
        assert!(!state.contains_screen_point(scaled(), point(at(235.5, 150.0))));

        let layer = egui::LayerId::new(
            egui::Order::Foreground,
            area_id(
                state
                    .windows
                    .iter()
                    .find(|window| window.system == system)
                    .unwrap(),
            ),
        );
        assert_eq!(state.release_target(layer), Some(system));
        let elsewhere = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("elsewhere"));
        assert_eq!(state.release_target(elsewhere), None);

        assert!(state.open(&world, system, ORIGIN, CockpitFaction::Empire, scaled()));
        assert_eq!(state.faction, CockpitFaction::Empire);
        state.clear();
        assert_eq!(state.window_count(), 0);
        assert!(!state.is_open(system));
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Text {
        text: String,
        pos: egui::Pos2,
        color: egui::Color32,
        font: f32,
        size: egui::Vec2,
        wrap: f32,
    }

    #[derive(Default)]
    struct Run {
        actions: Vec<DefensesWindowAction>,
        texts: Vec<Text>,
        painted: Vec<(u32, egui::Pos2)>,
    }

    /// Draw the windows once per frame of events, on [`scaled`] for the
    /// Alliance; the last frame's text and bitmaps are kept.
    fn run(
        world: &GameWorld,
        missions: &MissionState,
        state: &mut DefensesWindowState,
        frames: Vec<Vec<egui::Event>>,
    ) -> Run {
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let fog = world
            .systems
            .keys()
            .fold(FogState::new(Faction::Alliance), |mut fog, key| {
                fog.reveal(key);
                fog
            });
        let mut economy = EconomyState::default();
        for key in world.systems.keys() {
            economy.per_system.insert(
                key,
                SystemEconomy {
                    garrison_requirement: 2,
                    ..SystemEconomy::default()
                },
            );
        }
        let mut result = Run::default();
        for (index, events) in frames.into_iter().enumerate() {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1000.0),
                )),
                time: Some(index as f64 * 0.05),
                events,
                ..Default::default()
            };
            PAINTED.with(|painted| painted.borrow_mut().clear());
            let output = ctx.run(input, |ctx| {
                result.actions.extend(draw_defenses_windows(
                    ctx,
                    world,
                    &fog,
                    missions,
                    &economy,
                    state,
                    CockpitFaction::Alliance,
                    scaled(),
                    &mut cache,
                ));
            });
            result.painted = PAINTED.with(|painted| painted.take());
            result.texts.clear();
            for clipped in output.shapes {
                if let egui::Shape::Text(text) = clipped.shape {
                    let format = &text.galley.job.sections[0].format;
                    result.texts.push(Text {
                        text: text.galley.text().to_owned(),
                        pos: text.pos,
                        color: format.color,
                        font: format.font_id.size,
                        size: text.galley.size(),
                        wrap: text.galley.job.wrap.max_width,
                    });
                }
            }
        }
        result
    }

    /// Draw manager-owned content once per frame while feeding the exact
    /// normalized pointer phases that produced each egui input batch.
    fn run_managed(
        world: &GameWorld,
        missions: &MissionState,
        state: &mut DefensesWindowState,
        manager: &WindowManager,
        frames: Vec<(Vec<egui::Event>, Vec<PointerPhase>)>,
    ) -> Run {
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let fog = world
            .systems
            .keys()
            .fold(FogState::new(Faction::Alliance), |mut fog, key| {
                fog.reveal(key);
                fog
            });
        let mut economy = EconomyState::default();
        for key in world.systems.keys() {
            economy.per_system.insert(
                key,
                SystemEconomy {
                    garrison_requirement: 2,
                    ..SystemEconomy::default()
                },
            );
        }
        let window = state
            .windows
            .iter()
            .find_map(|window| window.id)
            .expect("managed test content has an identity");
        let mut router = FrameInputRouter::default();
        let mut result = Run::default();
        for (index, (events, phases)) in frames.into_iter().enumerate() {
            let frame = u64::try_from(index + 1).unwrap();
            let snapshot = manager.begin_frame();
            router.begin_frame(frame, &snapshot);
            let mut routed = Vec::new();
            for (ordinal, phase) in phases.into_iter().enumerate() {
                let event = InputEventId {
                    frame,
                    ordinal: u32::try_from(ordinal).unwrap(),
                };
                assert_eq!(
                    router.collect_pointer(
                        event,
                        &std::collections::BTreeSet::from([window]),
                        phase,
                    ),
                    InputRecipient::Window(window)
                );
                routed.push((event, phase));
            }
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1000.0),
                )),
                time: Some(index as f64 * 0.05),
                events,
                ..Default::default()
            };
            PAINTED.with(|painted| painted.borrow_mut().clear());
            let output = ctx.run(input, |ctx| {
                result.actions.extend(draw_managed_defenses_windows(
                    ctx,
                    world,
                    &fog,
                    missions,
                    &economy,
                    state,
                    CockpitFaction::Alliance,
                    scaled(),
                    &mut cache,
                    &snapshot,
                    &routed,
                    &mut router,
                ));
            });
            result.painted = PAINTED.with(|painted| painted.take());
            result.texts.clear();
            for clipped in output.shapes {
                if let egui::Shape::Text(text) = clipped.shape {
                    let format = &text.galley.job.sections[0].format;
                    result.texts.push(Text {
                        text: text.galley.text().to_owned(),
                        pos: text.pos,
                        color: format.color,
                        font: format.font_id.size,
                        size: text.galley.size(),
                        wrap: text.galley.job.wrap.max_width,
                    });
                }
            }
        }
        result
    }

    fn hover(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
        vec![
            vec![egui::Event::PointerMoved(point)],
            vec![egui::Event::PointerMoved(point)],
        ]
    }

    fn press(point: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    fn click(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
        let mut frames = hover(point);
        frames.extend([vec![press(point, true)], vec![press(point, false)], vec![]]);
        frames
    }

    fn text<'a>(run: &'a Run, text: &str) -> &'a Text {
        run.texts
            .iter()
            .find(|value| value.text == text)
            .unwrap_or_else(|| panic!("no {text:?} in {:?}", run.texts))
    }

    #[test]
    fn the_window_paints_its_chrome_tabs_and_the_personnel_page_first() {
        // FUN_004a8790 selects tab 1; 10577 at (0, 0), the focused side 1
        // strip at (2, 2), the five buttons at (x, 20), each page's empty
        // art where it lists nothing (FUN_004a9ce0), the rows' minis at
        // their cells' corners and their names from (1, 27).
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);
        let run = run(&world, &missions, &mut state, hover(at(150.0, 10.0)));

        for (id, point) in [
            (BACKGROUND, at(0.0, 0.0)),
            (10299, at(2.0, 2.0)),
            (SECTOR_NORMAL, at(3.0, 3.0)),
            (MINIMIZE_NORMAL, at(204.0, 3.0)),
            (CLOSE_NORMAL, at(218.0, 3.0)),
            (10571, at(28.0, 20.0)),
            (10563, at(64.0, 20.0)),
            (10558, at(100.0, 20.0)),
            (10553, at(136.0, 20.0)),
            (10550, at(172.0, 20.0)),
            (18_176 + 832, at(7.0, 81.0)),
            (18_176 + 832, at(77.0, 81.0)),
            (17_728, at(147.0, 81.0)),
        ] {
            assert!(
                run.painted.contains(&(id, point)),
                "{id} at {point:?} in {:?}",
                run.painted
            );
        }
        assert!(!run.painted.iter().any(|(id, _)| ROW_FRAME.contains(id)));
        let idle = text(&run, "Idle");
        assert_eq!(idle.pos, at(8.0, 108.0));
        // The cell less 3 (FUN_00609ae0's text rect), at scale 2.
        assert_eq!((idle.font, idle.wrap), (18.0, 134.0));
        assert_eq!(text(&run, "Hidden").color, egui::Color32::WHITE);
        // `+0x49` at (19, 2), 16 high: left-aligned, vertically centred.
        let title = text(&run, "Sluis Van");
        assert_eq!((title.pos.x, title.font), (at(19.0, 0.0).x, 22.0));
        assert_eq!(title.pos.y + title.size.y / 2.0, at(0.0, 10.0).y);
        // `+0x5b` at (2, 51), 231 by 16: centred both ways.
        let name = text(&run, "Personnel");
        assert_eq!(name.font, 20.0);
        assert_eq!(name.pos + name.size / 2.0, at(117.5, 59.0));
        assert!(!run
            .texts
            .iter()
            .any(|value| value.text.starts_with("Garrison")));
    }

    #[test]
    fn a_click_on_a_row_frames_it_in_the_sides_color_and_a_new_tab_clears_it() {
        // FUN_0060bd00 paints the selected image, the mini with the side
        // frame keyed over it; +0xdc colours its text. FUN_004a90d0 refills
        // only for a different page.
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);

        let selected = run(&world, &missions, &mut state, click(at(100.0, 100.0)));
        assert!(selected.painted.contains(&(ROW_FRAME[0], at(77.0, 81.0))));
        assert_eq!(
            text(&selected, "Hidden").color,
            egui::Color32::from_rgb(255, 0, 0)
        );
        assert_eq!(
            selected.actions,
            [DefensesWindowAction::SelectSystem(system)]
        );
        let report = state
            .report(
                &world,
                &seen(system),
                &missions,
                &EconomyState::default(),
                system,
            )
            .unwrap();
        assert_eq!(report.selected, Some(1));
        assert_eq!(report.counts, [3, 1, 0, 2, 2]);

        // The same tab keeps the selection; another clears it.
        let _ = run(&world, &missions, &mut state, click(at(40.0, 30.0)));
        assert_eq!(
            state.windows[0].selected,
            Some(DefensesItem::Character(
                world
                    .characters
                    .iter()
                    .find(|(_, value)| value.name == "Hidden")
                    .unwrap()
                    .0
            ))
        );
        let regiments = run(&world, &missions, &mut state, click(at(80.0, 30.0)));
        assert_eq!(state.windows[0].page, DefensesPage::Regiments);
        assert_eq!(state.windows[0].selected, None);
        // The regiment page of the player's own system shows the garrison.
        // `+0x5c` at (2, 63), 228 by 17: centred both ways.
        let garrison = text(&regiments, "Garrison Requirement: 2");
        assert_eq!(garrison.pos + garrison.size / 2.0, at(116.0, 71.5));
        assert!(regiments.painted.contains(&(10564, at(64.0, 20.0))));
        assert!(regiments.painted.contains(&(17_473, at(7.0, 81.0))));
    }

    #[test]
    fn the_title_buttons_open_the_sector_minimize_and_close() {
        // 0xca opens the sector window (FUN_00429ce0); 0xc9 posts 0x466;
        // 200 closes.
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);
        let sector = run(&world, &missions, &mut state, click(at(9.0, 9.0)));
        assert_eq!(
            sector.actions,
            [
                DefensesWindowAction::OpenSector(system),
                DefensesWindowAction::SelectSystem(system)
            ]
        );

        let minimized = run(&world, &missions, &mut state, click(at(210.0, 9.0)));
        assert_eq!(
            minimized.actions,
            [DefensesWindowAction::Minimize {
                system,
                logical_position: ORIGIN
            }]
        );
        assert_eq!(state.window_count(), 0);

        let mut state = opened(&world, system);
        let closed = run(&world, &missions, &mut state, click(at(224.0, 9.0)));
        assert!(closed.actions.is_empty());
        assert_eq!(state.window_count(), 0);
    }

    #[test]
    fn a_title_button_shows_its_pressed_art_only_while_the_button_is_held_over_it() {
        // FUN_004a8790's buttons swap to their pressed bitmaps under a held
        // pointer only.
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);
        let hovered = run(&world, &missions, &mut state, hover(at(210.0, 9.0)));
        assert!(hovered.painted.contains(&(MINIMIZE_NORMAL, at(204.0, 3.0))));

        let mut frames = hover(at(224.0, 9.0));
        frames.push(vec![press(at(224.0, 9.0), true)]);
        let held = run(&world, &missions, &mut state, frames);
        assert!(held.painted.contains(&(CLOSE_PRESSED, at(218.0, 3.0))));
        assert!(held.painted.contains(&(MINIMIZE_NORMAL, at(204.0, 3.0))));
        assert_eq!(state.window_count(), 1);
    }

    fn right_click(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
        let button = |pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Secondary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let mut frames = hover(point);
        frames.extend([vec![button(true)], vec![button(false)], vec![]]);
        frames
    }

    fn menus(run: &Run) -> Vec<(Option<MenuObject>, (i16, i16))> {
        run.actions
            .iter()
            .filter_map(|action| match *action {
                DefensesWindowAction::OpenObjectMenu {
                    selection, point, ..
                } => Some((selection, point)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_right_click_on_a_character_selects_it_and_opens_its_menu_at_the_cursor() {
        // FUN_004ac5c0 (the class's slot 7) opens the menu for slot 22's
        // selection (FUN_004a7a20), the list's selected rows, at the
        // release point in 640 by 480 canvas pixels.
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);
        let hidden = world
            .characters
            .iter()
            .find(|(_, value)| value.name == "Hidden")
            .unwrap()
            .0;

        let opened = run(&world, &missions, &mut state, right_click(at(100.0, 100.0)));
        assert_eq!(
            state.windows[0].selected,
            Some(DefensesItem::Character(hidden))
        );
        assert_eq!(
            menus(&opened),
            [(Some(MenuObject::Character(hidden)), (120, 130))]
        );
    }

    #[test]
    fn a_right_click_on_a_regiment_opens_its_menu() {
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);
        let _ = run(&world, &missions, &mut state, click(at(80.0, 30.0)));
        let troop = world.systems[system].ground_units[0];
        let opened = run(&world, &missions, &mut state, right_click(at(30.0, 100.0)));
        assert_eq!(
            menus(&opened),
            [(Some(MenuObject::Troop(troop)), (50, 130))]
        );
    }

    #[test]
    fn a_right_click_on_a_defense_facility_opens_its_menu() {
        // FUN_004ac5c0 opens the pop-up for slot 22's selection
        // (FUN_004a7a20); FUN_0051d990 gives it Encyclopedia and Status.
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);
        let _ = run(&world, &missions, &mut state, click(at(150.0, 30.0)));
        assert_eq!(state.windows[0].page, DefensesPage::Shields);
        let opened = run(&world, &missions, &mut state, right_click(at(30.0, 100.0)));
        let Some(DefensesItem::Defense(shield)) = state.windows[0].selected else {
            panic!("the right press selects the shield");
        };
        assert_eq!(
            menus(&opened),
            [(Some(MenuObject::DefenseFacility(shield)), (50, 130))]
        );
    }

    #[test]
    fn a_right_click_on_empty_list_space_clears_the_selection_and_opens_an_empty_menu() {
        // FUN_006094b0 clears the selection on a press over no item; the
        // release then opens the menu for nothing (Encyclopedia and Status,
        // both disabled).
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);
        let _ = run(&world, &missions, &mut state, click(at(100.0, 100.0)));
        assert!(state.windows[0].selected.is_some());

        let empty = run(&world, &missions, &mut state, right_click(at(100.0, 250.0)));
        assert_eq!(state.windows[0].selected, None);
        assert_eq!(menus(&empty), [(None, (120, 280))]);

        // Above the list, a right click opens nothing.
        let outside = run(&world, &missions, &mut state, right_click(at(100.0, 60.0)));
        assert!(menus(&outside).is_empty());
    }

    #[test]
    fn managed_secondary_press_selects_and_clears_with_its_exact_phase() {
        let (world, system, missions) = stocked();
        let mut manager = WindowManager::new();
        let mut state = DefensesWindowState::default();
        let prepared = state
            .prepare_open(&world, system, ORIGIN, CockpitFaction::Alliance, scaled())
            .unwrap();
        commit_prepared_defenses_window(&mut manager, &mut state, prepared);
        let secondary = |point, pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Secondary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let row = at(100.0, 100.0);
        run_managed(
            &world,
            &missions,
            &mut state,
            &manager,
            vec![
                (vec![egui::Event::PointerMoved(row)], vec![]),
                (vec![egui::Event::PointerMoved(row)], vec![]),
                (
                    vec![secondary(row, true)],
                    vec![PointerPhase::Press(PointerButtonId::Secondary)],
                ),
            ],
        );
        let hidden = world
            .characters
            .iter()
            .find(|(_, value)| value.name == "Hidden")
            .unwrap()
            .0;
        assert_eq!(
            state.windows[0].selected,
            Some(DefensesItem::Character(hidden)),
            "a routed secondary press selects before its later menu release"
        );

        let empty = at(100.0, 250.0);
        run_managed(
            &world,
            &missions,
            &mut state,
            &manager,
            vec![
                (vec![egui::Event::PointerMoved(empty)], vec![]),
                (vec![egui::Event::PointerMoved(empty)], vec![]),
                (
                    vec![secondary(empty, true)],
                    vec![PointerPhase::Press(PointerButtonId::Secondary)],
                ),
            ],
        );
        assert_eq!(
            state.windows[0].selected, None,
            "the same routed secondary press clears empty list space"
        );
    }

    #[test]
    fn managed_scroll_thumb_motion_commits_with_motion_phase() {
        let (mut world, system, missions) = stocked();
        for index in 0..8 {
            add_character(&mut world, system, &format!("Extra {index}"), true);
        }
        let mut manager = WindowManager::new();
        let mut state = DefensesWindowState::default();
        let prepared = state
            .prepare_open(&world, system, ORIGIN, CockpitFaction::Alliance, scaled())
            .unwrap();
        commit_prepared_defenses_window(&mut manager, &mut state, prepared);
        let thumb = at(220.0, 100.0);
        let dragged = at(220.0, 200.0);
        run_managed(
            &world,
            &missions,
            &mut state,
            &manager,
            vec![
                (vec![egui::Event::PointerMoved(thumb)], vec![]),
                (vec![egui::Event::PointerMoved(thumb)], vec![]),
                (
                    vec![press(thumb, true)],
                    vec![PointerPhase::Press(PointerButtonId::Primary)],
                ),
                (
                    vec![egui::Event::PointerMoved(dragged)],
                    vec![PointerPhase::Motion(PointerButtonId::Primary)],
                ),
            ],
        );
        assert_eq!(state.windows[0].first_row, 1);
    }

    #[test]
    fn scroll_thumb_paints_the_row_committed_by_this_input_frame() {
        let before = ScrollRange {
            first: 1,
            visible: 3.0,
            total: 8,
        };

        assert_eq!(scroll_range_after_input(before, 4).first, 4);
    }

    #[test]
    fn showing_a_page_clears_the_selection_only_when_the_page_changes() {
        // FUN_004a90d0 refills only for a different page, as a tab click.
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);
        let _ = run(&world, &missions, &mut state, click(at(100.0, 100.0)));
        assert!(state.show_page(system, DefensesPage::Personnel));
        assert!(state.windows[0].selected.is_some());
        assert!(state.show_page(system, DefensesPage::Regiments));
        assert_eq!(state.windows[0].page, DefensesPage::Regiments);
        assert_eq!(state.windows[0].selected, None);

        let mut other = world.clone();
        let missing = other.systems.insert(world.systems[system].clone());
        assert!(!state.show_page(missing, DefensesPage::Shields));
        assert_eq!(
            state.screen_rect(scaled(), system),
            Some(egui::Rect::from_min_size(
                at(0.0, 0.0),
                egui::vec2(470.0, 608.0)
            ))
        );
        assert_eq!(state.screen_rect(scaled(), missing), None);
    }

    #[test]
    fn a_page_shows_its_first_nine_cells_until_scrolled() {
        // Three rows of three cells show (the list is 210 high, a row 70);
        // FUN_0060a490's bar (RCDATA 0x29cc) scrolls the rest. The ninth
        // sits at (147, 221).
        let (mut world, system, missions) = stocked();
        for index in 0..8 {
            add_character(&mut world, system, &format!("Extra {index}"), true);
        }
        let mut state = opened(&world, system);
        let run = run(&world, &missions, &mut state, hover(at(150.0, 10.0)));
        let minis: Vec<_> = run
            .painted
            .iter()
            .filter(|(id, _)| *id == 18_176 + 832)
            .collect();
        assert_eq!(minis.len(), 9);
        assert!(run.painted.contains(&(18_176 + 832, at(147.0, 221.0))));
    }
}
