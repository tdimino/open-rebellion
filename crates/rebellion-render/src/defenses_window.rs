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

use crate::fleet_window::{control_side, faction_side, fleet_side, mini_layers, MiniObject};
use crate::native_geometry::{exact_clicked, logical_rect, rect_contains};
use crate::object_menu::MenuObject;
use crate::panels::fleets::fighter_mini_id;
use crate::quadrant_icons::{mission_keys, system_members};
use crate::scroll_bar::{egui_pointer, ScrollBar, ScrollDrag, ScrollRange, LIST_BAR_29CC};
use crate::strategic_window_common;
#[allow(unused_imports)]
use crate::strategic_window_common::window_prelude::*;
use crate::system_window::{
    canvas_point, character_mini_resource_id, clamp_window_to_galaxy, defense_facility_mini,
    opposing_contents_visible, special_force_mini, surface_drag_grab, troop_mini, TitleDrag,
};
use crate::windowing::GalaxyChild;
use crate::windowing_input::{FrameInputRouter, InputEventId, PointerButtonId, PointerPhase};
use crate::windowing_policy::system_subject;
use egui_macroquad::egui;
use rebellion_core::dat::{ExplorationStatus, Faction};
use rebellion_core::economy::EconomyState;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{
    CharacterKey, DefenseFacilityKey, FighterKey, SpecialForceKey, SystemKey, TroopKey,
};
use rebellion_core::missions::{MissionMember, MissionState, MissionTarget};
use rebellion_core::saved_interface::{SavedWindow, SavedWindowKind};
use rebellion_core::world::GameWorld;
#[cfg(test)]
use rebellion_windowing::{ContentReport, WindowOwner};

pub const DEFENSES_WINDOW_WIDTH: f32 = 235.0;
pub const DEFENSES_WINDOW_HEIGHT: f32 = 304.0;

const BACKGROUND: u32 = 10577;
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
    /// Id 4: `GenCore` shields (`0x24`) and the Death Star Shield (`0x25`).
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
    const fn menu_object(self, system: SystemKey, side: u8) -> MenuObject {
        match self {
            Self::Character(key) => MenuObject::Character(key),
            Self::SpecialForce(key) => MenuObject::SpecialForce(key),
            Self::Troop(key) => MenuObject::Troop(key),
            Self::Squadron { class, .. } => MenuObject::SystemFighter {
                system,
                class,
                is_alliance: side == 1,
            },
            Self::Defense(key) => MenuObject::DefenseFacility(key),
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
    /// The cells as last painted from the list's top, for the release hit
    /// test.
    painted: Vec<DefensesItem>,
}

strategic_window_common::impl_strategic_window_record!(
    OpenDefensesWindow,
    area_id,
    window_screen_rect
);

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

impl strategic_window_common::RoutedDragPayload for RowDrag {
    fn press(&self) -> egui::Pos2 {
        self.press
    }

    fn list(&self) -> egui::Rect {
        self.list
    }
}

/// The open Defenses windows. The last is focused and paints on top.
#[derive(Debug)]
pub struct DefensesWindowState {
    faction: CockpitFaction,
    windows: Vec<OpenDefensesWindow>,
    drag: Option<RowDrag>,
    drag_owner: Option<WindowId>,
    /// A window drag in progress.
    title_drag: Option<TitleDrag>,
}

strategic_window_common::impl_strategic_window_state_core!(
    DefensesWindowState,
    OpenDefensesWindow,
    area_id,
    drag = None,
    drag_owner = None,
);
strategic_window_common::impl_strategic_window_collection_accessors!(
    DefensesWindowState,
    window_screen_rect
);
strategic_window_common::impl_system_keyed_window_routing!(
    DefensesWindowState,
    GalaxyChild::Defenses,
    area_id,
    window_screen_rect
);
strategic_window_common::impl_fixture_window_order!(DefensesWindowState, OpenDefensesWindow);
strategic_window_common::impl_admitted_release_target!(DefensesWindowState, area_id);
strategic_window_common::define_basic_prepared_detail!(
    /// Validated Defenses content held aside until the manager accepts its
    /// stable `(Defenses, system DAT ID)` identity.
    PreparedDefensesWindow for DefensesWindowState,
    kind: WindowKind::Defenses,
    size: (DEFENSES_WINDOW_WIDTH, DEFENSES_WINDOW_HEIGHT),
);
strategic_window_common::impl_system_content_reports!(DefensesWindowState, WindowKind::Defenses);
strategic_window_common::impl_saved_system_windows!(DefensesWindowState, |system| {
    SavedWindowKind::Defenses { system }
});

type EndDragResult = strategic_window_common::RoutedDragResult<DefensesWindowAction, RowDrag>;

impl DefensesWindowState {
    /// Publishes new Defenses content, or confirms the exact preserved
    /// content, only after a successful manager open commit.
    pub fn commit_open(
        &mut self,
        prepared: PreparedDefensesWindow,
        transition: &WindowTransition,
    ) -> Option<WindowId> {
        let subject = prepared.spec.key.subject?;
        // Dedicated interface fixtures predate manager publication and may
        // seed one exact Defenses record before the app queues its open.
        #[cfg(feature = "interface-test-fixtures")]
        if let WindowTransition::Opened { id } = *transition {
            if let Some(index) = self.windows.iter().position(|window| {
                window.id.is_none() && window.subject.is_none() && window.system == prepared.system
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
        }
        strategic_window_common::commit_open(
            &mut self.windows,
            &mut self.faction,
            prepared.faction,
            prepared.system,
            subject,
            transition,
            |id, subject| OpenDefensesWindow {
                id: Some(id),
                subject: Some(subject),
                system: prepared.system,
                logical_position: prepared.logical_position,
                page: DefensesPage::Personnel,
                selected: None,
                first_row: 0,
                scroll: ScrollDrag::default(),
                release_target_visible: false,
                painted: Vec::new(),
            },
            |_| {},
        )
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
        self.open_direct(world.systems.contains_key(system), system, faction, || {
            OpenDefensesWindow {
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
                painted: Vec::new(),
            }
        })
    }

    /// Whether exact Defenses content participates visibly in this frozen
    /// frame. Manager-owned content must retain its committed kind and DAT
    /// subject; direct fixture content remains visible while its system exists.
    #[must_use]
    pub fn has_visible_content(&self, snapshot: &RoutingSnapshot, world: &GameWorld) -> bool {
        self.windows
            .iter()
            .any(|window| window_is_visible(window, snapshot, world))
    }

    /// Removes only content named by manager teardown receipts.
    pub fn remove_content(&mut self, removed: &[WindowId]) {
        strategic_window_common::remove_content(&mut self.windows, removed);
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

    /// What `system`'s window shows, for the interface fixture.
    #[must_use]
    pub fn report(
        &self,
        world: &GameWorld,
        fog: &FogState,
        missions: &MissionState,
        system: SystemKey,
    ) -> Option<DefensesWindowReport> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        let player = self.faction.player();
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
            garrison: garrison_line(world, player, system, window.page),
        })
    }

    /// Refreshes which retained egui layers may resolve a gameplay release.
    /// The application calls this immediately after freezing the frame so a
    /// stale prior-frame area cannot retarget before Defenses paints.
    pub fn refresh_release_targets(&mut self, snapshot: &RoutingSnapshot, world: &GameWorld) {
        for window in &mut self.windows {
            window.release_target_visible = window_is_visible(window, snapshot, world);
        }
    }

    // The object a Mission order released over the Defenses window egui
    // draws as `layer` names (`+0x68`, `FUN_004aa380`): the row in the cell
    // under `point` (list `+0x164` at (7, 81), on any page), or `None`
    // where the window gives its system. port: a character or squadron row
    // names no `MissionTarget`, so it gives the system too.
    strategic_window_common::define_release_object_method!(|this, layout, layer, point| {
        let (window, local) = strategic_window_common::local_point_for_layer(
            &this.windows,
            layout,
            layer,
            point,
            area_id,
            window_screen_rect,
        )?;
        let index = crate::system_window::grid_cell(local, LIST, (CELL, CELL), COLUMNS)?;
        match *window.painted.get(index)? {
            DefensesItem::Defense(key) => Some(MissionTarget::DefenseFacility(key)),
            DefensesItem::Troop(key) => Some(MissionTarget::Troop(key)),
            DefensesItem::SpecialForce(key) => Some(MissionTarget::SpecialForce(key)),
            DefensesItem::Character(_) | DefensesItem::Squadron { .. } => None,
        }
    });

    pub fn clear(&mut self) {
        self.windows.clear();
        self.drag = None;
        self.drag_owner = None;
        self.title_drag = None;
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
        strategic_window_common::cancel_owned_capture(&mut self.drag, &mut self.drag_owner, window);
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
        mut frame: strategic_window_common::RoutedDragFrame<'_>,
    ) -> EndDragResult {
        let mut result = strategic_window_common::complete_window_drag!(
            self,
            frame,
            forward,
            DefensesWindowAction::ReleasePointer
        );
        let Some((drag, point, ctrl)) = result.take_drop() else {
            return result;
        };
        let action = match drag.item {
            DefensesItem::Squadron { class, .. } => DefensesWindowAction::DragSquadron {
                system: drag.system,
                class,
                is_alliance: drag.is_alliance,
                confirmed: ctrl,
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

    /// The open Defenses children; z-order and focus come from the manager.
    pub fn children(&self) -> impl Iterator<Item = GalaxyChild> + '_ {
        self.windows
            .iter()
            .map(|window| GalaxyChild::Defenses(window.system))
    }

    /// Rebuild a shown saved Defenses window (`FUN_00441190` type 10,
    /// `FUN_004a7790`) at its saved place, above the windows rebuilt before
    /// it, on the personnel page: the page is not saved (`FUN_004aa6d0`). A
    /// hidden one is a rail entry only (`SystemWindowState::restore_saved`).
    /// False for any other record, a hidden one, or a system not in the
    /// world.
    pub fn restore_saved(
        &mut self,
        world: &GameWorld,
        faction: CockpitFaction,
        saved: &SavedWindow,
    ) -> bool {
        let system = strategic_window_common::restore_shown_detail_system!(
            self, world, faction, saved, Defenses
        );
        self.windows.push(OpenDefensesWindow {
            id: None,
            subject: None,
            system,
            logical_position: saved.position,
            page: DefensesPage::Personnel,
            selected: None,
            first_row: 0,
            scroll: ScrollDrag::default(),
            release_target_visible: true,
            painted: Vec::new(),
        });
        true
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
    /// view hit-tests the screen point and issues `0x201`, or `0x202` with
    /// Ctrl (`TroopTransportState::move_squadron`).
    DragSquadron {
        system: SystemKey,
        class: FighterKey,
        is_alliance: bool,
        confirmed: bool,
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
    player: Faction,
    system: SystemKey,
    page: DefensesPage,
) -> Option<String> {
    if page != DefensesPage::Regiments || shown_side(world, system) != faction_side(player) {
        return None;
    }
    let requirement =
        rebellion_core::economy::stored_support_values(world, system).garrison_requirement;
    Some(format!("Garrison Requirement: {requirement}"))
}

fn area_id(window: &OpenDefensesWindow) -> egui::Id {
    egui::Id::new(("original-defenses-window", window.id, window.system))
}

fn window_is_visible(
    window: &OpenDefensesWindow,
    snapshot: &RoutingSnapshot,
    world: &GameWorld,
) -> bool {
    match window.id {
        Some(id) => {
            snapshot.is_visible(id)
                && snapshot.key(id).is_some_and(|key| {
                    key.kind == WindowKind::Defenses && key.subject == window.subject
                })
                && system_subject(world, window.system) == window.subject
        }
        None => world.systems.contains_key(window.system),
    }
}

strategic_window_common::define_record_screen_rect!(
    window_screen_rect,
    OpenDefensesWindow,
    DEFENSES_WINDOW_WIDTH,
    DEFENSES_WINDOW_HEIGHT
);

fn tab_rect(window: egui::Rect, scale: f32, page: DefensesPage) -> egui::Rect {
    strategic_window_common::tab_strip_rect(window, scale, TAB_STRIP, TAB_SIZE, page.x())
}

/// The `index`th grid cell (`FUN_00609ae0`): row-major from the list's
/// corner, three to a row.
fn cell_rect(window: egui::Rect, scale: f32, index: usize) -> egui::Rect {
    strategic_window_common::grid_cell_rect(
        window,
        scale,
        index,
        COLUMNS,
        (LIST.0, LIST.1),
        (CELL, CELL),
    )
}

strategic_window_common::impl_strategic_window_cell_rect!(
    DefensesWindowState,
    window_screen_rect,
    cell_rect
);

#[derive(Default)]
struct WindowDrawResult {
    /// Where a press on the window's own surface took hold ([`TitleDrag`]).
    drag_grab: Option<egui::Vec2>,
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
    /// The cells painted from the list's top.
    painted: Vec<DefensesItem>,
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
#[allow(
    clippy::too_many_arguments,
    reason = "Keep explicit state and rendering inputs at this UI boundary, as the Fleet window does."
)]
pub fn draw_defenses_windows(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    _economy: &EconomyState,
    state: &mut DefensesWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Vec<DefensesWindowAction> {
    draw_defenses_windows_with_snapshot(
        strategic_window_common::DetailWindowsDraw::unmanaged(
            ctx, world, fog, missions, faction, layout, cache,
        ),
        state,
    )
}

/// Draw manager-owned Defenses content in the frozen bottom-to-top paint
/// order and admit mutations only through routed pointer events.
#[allow(
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
    draw_managed_defenses_windows_shown(
        ManagedDefensesDraw {
            ctx,
            world,
            fog,
            missions,
            economy,
            state,
            faction,
            layout,
            cache,
            snapshot,
            pointer_events,
            router,
        },
        &|_| true,
    )
}

/// Inputs shared by the managed Defenses renderer and its visibility-aware
/// application boundary.
pub struct ManagedDefensesDraw<'a> {
    pub ctx: &'a egui::Context,
    pub world: &'a GameWorld,
    pub fog: &'a FogState,
    pub missions: &'a MissionState,
    pub economy: &'a EconomyState,
    pub state: &'a mut DefensesWindowState,
    pub faction: CockpitFaction,
    pub layout: CockpitLayout,
    pub cache: &'a mut BmpCache,
    pub snapshot: &'a RoutingSnapshot,
    pub pointer_events: &'a [(InputEventId, PointerPhase)],
    pub router: &'a mut FrameInputRouter,
}

/// [`draw_managed_defenses_windows`] with command-center GID visibility.
pub fn draw_managed_defenses_windows_shown(
    input: ManagedDefensesDraw<'_>,
    shown: &dyn Fn(GalaxyChild) -> bool,
) -> Vec<DefensesWindowAction> {
    let ManagedDefensesDraw {
        ctx,
        world,
        fog,
        missions,
        economy: _,
        state,
        faction,
        layout,
        cache,
        snapshot,
        pointer_events,
        router,
    } = input;
    strategic_window_common::draw_managed_detail_windows!(
        draw_defenses_windows_with_snapshot => ctx world fog missions faction layout cache
        snapshot pointer_events router shown state
    )
}

#[allow(
    clippy::too_many_arguments,
    reason = "Shared implementation for production snapshots and fixture-only direct content."
)]
fn draw_defenses_windows_with_snapshot(
    draw: strategic_window_common::DetailWindowsDraw<'_>,
    state: &mut DefensesWindowState,
) -> Vec<DefensesWindowAction> {
    strategic_window_common::unpack_detail_windows_draw!(
        draw => ctx world fog missions faction layout cache snapshot pointer_events router shown
    );
    state.update_title_drag(ctx, layout);
    if state.windows.iter().all(|window| window.id.is_none()) {
        state.prepare_faction(faction);
    }
    let strategic_window_common::WindowPaintSet {
        windows,
        stale_manager_windows,
        focused_manager,
        focused_fixture_system,
    } = strategic_window_common::refreshed_release_window_paint_set!(
        &mut state.windows,
        snapshot,
        WindowKind::Defenses,
        |system| shown(GalaxyChild::Defenses(system)),
        world,
        window_is_visible,
    );
    for &id in &stale_manager_windows {
        state.cancel_pointer_capture(id);
    }
    let ended_drag = state.end_drag(strategic_window_common::RoutedDragFrame::new(
        ctx,
        layout.scale,
        router.as_deref_mut(),
        pointer_events,
    ));
    let mut actions = stale_manager_windows
        .into_iter()
        .map(DefensesWindowAction::Close)
        .collect::<Vec<_>>();
    actions.extend(ended_drag.actions);
    for window in &windows {
        let focused =
            strategic_window_common::is_focused(window, focused_manager, focused_fixture_system);
        let result = strategic_window_common::record_title_drag(
            &mut state.title_drag,
            area_id(window),
            draw_defenses_window(
                strategic_window_common::DetailWindowDraw::new(
                    ctx, world, window, focused, faction, layout, cache,
                ),
                fog,
                missions,
            ),
            |result| result.drag_grab,
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
        let pointer_permitted = strategic_window_common::pointer_mutation_permitted(
            window.id,
            pointer_mutation,
            &result.causal_pointer_phases,
            router.as_deref_mut(),
            pointer_events,
            |phase| {
                phase == PointerPhase::Release(PointerButtonId::Primary)
                    && ended_drag.claimed_primary_release_by == window.id
            },
        );
        if pointer_permitted {
            if let Some(drag) = result.drag {
                state.drag = Some(drag);
                state.drag_owner = window.id;
                if let Some(id) = window.id {
                    actions.push(DefensesWindowAction::CapturePointer(id));
                }
            }
        }
        if strategic_window_common::apply_minimizable_detail_exit! {
            (state, actions, window, system),
            (
                result.close && pointer_permitted,
                result.minimize,
                pointer_permitted,
                world.systems.contains_key(system)
            ),
            DefensesWindowAction,
        } {
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
            open.painted = result.painted;
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
        strategic_window_common::apply_detail_focus! {
            (state, actions, window, system),
            (result.focus, pointer_permitted),
            DefensesWindowAction,
        };
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

#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "Keep the window's ordered paint and input pass together, as the Fleet window does."
)]
fn draw_defenses_window(
    draw: strategic_window_common::DetailWindowDraw<'_, OpenDefensesWindow>,
    fog: &FogState,
    missions: &MissionState,
) -> WindowDrawResult {
    strategic_window_common::begin_detail_window_draw!(
        draw => ctx world window focused faction layout cache result system scale screen_rect id player:
        WindowDrawResult
    );
    let side = shown_side(world, window.system);
    let rows = page_rows(world, fog, missions, player, window.system, window.page);
    let area = strategic_window_common::show_canvas(ctx, id, screen_rect, scale, |ui, canvas| {
        let (local, window_response, painter) = canvas.input_parts();
        // The title label `+0x49` starts after the sector button, in
        // font 5 with format `0x24`: one line, left-aligned and
        // vertically centred (`FUN_004a8790`).
        canvas.paint_background(cache, BACKGROUND);
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
            strategic_window_common::TITLE_BUTTONS,
            window.system,
            |index, rect| {
                crate::control_registry::register_screen(
                    "Defenses",
                    ["Sector", "Minimize", "Close"][index],
                    rect,
                );
            },
        );
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
        if let Some(line) = garrison_line(world, player, window.system, window.page) {
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
                (bar_left..LIST.0 + LIST.2).contains(&x) && (LIST.1..LIST.1 + LIST.3).contains(&y)
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
        result.painted = shown.clone().map(|(_, row)| row.item).collect();
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
            for (source, bitmap) in mini_layers(object, row.mini, en_route, None) {
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
            strategic_window_common::paint_grid_cell_label(
                &list_painter,
                cell,
                scale,
                &row.label,
                &font,
                color,
                CELL - TEXT_RIGHT_TRIM,
                TEXT_OFFSET,
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
            if let (true, Some(press)) =
                (draggable, crate::native_geometry::primary_press_origin(ctx))
            {
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
                if let Some(point) = response
                    .interact_pointer_pos()
                    .filter(|point| rect_contains(cell.intersect(list), *point))
                {
                    let object = row.item.menu_object(window.system, side);
                    result.object_menu = Some((Some(object), canvas_point(layout, point)));
                    result.note_pointer_phase(PointerPhase::Release(PointerButtonId::Secondary));
                }
            }
        }

        crate::scroll_bar::paint_egui(
            &painter,
            ctx,
            cache,
            local,
            scale,
            &bar.blits(range, scroll, pointer),
        );

        result.drag_grab = surface_drag_grab(ctx, &window_response, local, scale, &[]);
        if strategic_window_common::interaction_requests_focus(&window_response, &clicked) {
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
    use crate::test_support::{
        add_character, click_frames as click, hover_frames as hover, pointer_button as press,
        right_click_frames as right_click, scaled_full_layout as scaled, send_on_mission,
        title_drag_frames as title_drag, world,
    };
    use rebellion_core::economy::SupportValues;
    use rebellion_core::ids::{DatId, FighterKey};
    use rebellion_core::world::{ControlKind, DefenseFacilityInstance, System, TroopUnit};
    use rebellion_windowing::{ContentStatus, WindowCommand, WindowManager, WindowTransition};

    const KDY: u32 = 0x2200_0001;
    const LNR: u32 = 0x2300_0002;
    const GENCORE: u32 = 0x2400_0003;
    const DEATH_STAR_SHIELD: u32 = 0x2500_0004;
    const ALLIANCE_ARMY: u32 = 0x1000_0002;
    const STORMTROOPERS: u32 = 0x1000_0006;

    fn fighter_drag_fixture(
        faction: CockpitFaction,
    ) -> (GameWorld, SystemKey, FighterKey, MissionState) {
        let is_alliance = faction == CockpitFaction::Alliance;
        let (mut world, system) = world(ControlKind::Controlled(if is_alliance {
            Faction::Alliance
        } else {
            Faction::Empire
        }));
        let class = world
            .fighter_classes
            .insert(rebellion_core::world::FighterClass {
                dat_id: DatId::new(if is_alliance {
                    0x1c00_0001
                } else {
                    0x1c00_0005
                }),
                name: "Fighter".into(),
                ..Default::default()
            });
        world.systems[system].add_squadrons(class, 1, is_alliance);
        (world, system, class, MissionState::new())
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
        });
        world.systems[system].ground_units.push(key);
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
        crate::test_support::add_special_force(&mut world, system, 0x3c00_0001, true);
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
        assert_empty!(page(DefensesPage::Squadrons));
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

        assert_empty!(labels(
            &world,
            &unseen,
            &missions,
            system,
            DefensesPage::Batteries
        ));
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
        world.systems[system].control = ControlKind::Uncontrolled;
        for page in DefensesPage::ALL {
            assert_empty!(labels(&world, &fog, &missions, system, page));
        }
        world.systems[system].control = ControlKind::Controlled(Faction::Alliance);
        world.systems[system].exploration_status = ExplorationStatus::Unexplored;
        assert_empty!(labels(
            &world,
            &fog,
            &missions,
            system,
            DefensesPage::Batteries
        ));
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
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        world.support_values.insert(
            system,
            SupportValues {
                garrison_requirement: 3,
                ..SupportValues::default()
            },
        );
        let line = |player, page| garrison_line(&world, player, system, page);

        assert_eq!(
            line(Faction::Alliance, DefensesPage::Regiments).as_deref(),
            Some("Garrison Requirement: 3")
        );
        assert_eq!(line(Faction::Alliance, DefensesPage::Personnel), None);
        assert_eq!(line(Faction::Empire, DefensesPage::Regiments), None);
        world.support_values.clear();
        assert_eq!(
            garrison_line(&world, Faction::Alliance, system, DefensesPage::Regiments).as_deref(),
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
        world.systems[system].control = ControlKind::Uncontrolled;
        assert_eq!(rail_icon(&world, system), 11535);

        crate::test_support::assert_strategic_title_resources();
        assert_eq!(selected_color(1), egui::Color32::from_rgb(255, 0, 0));
        assert_eq!(selected_color(2), egui::Color32::from_rgb(0, 255, 0));
        assert_eq!(selected_color(0), egui::Color32::from_rgb(0, 255, 255));
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

    fn prepared_managed_window(
        world: &GameWorld,
        system: SystemKey,
    ) -> (WindowManager, DefensesWindowState, PreparedDefensesWindow) {
        let manager = WindowManager::new();
        let state = DefensesWindowState::default();
        let prepared = state
            .prepare_open(world, system, ORIGIN, CockpitFaction::Alliance, scaled())
            .expect("live system has a stable DAT subject");
        (manager, state, prepared)
    }

    fn opened_managed_window(
        world: &GameWorld,
        system: SystemKey,
    ) -> (WindowManager, DefensesWindowState) {
        let (mut manager, mut state, prepared) = prepared_managed_window(world, system);
        commit_prepared_defenses_window(&mut manager, &mut state, prepared);
        (manager, state)
    }

    fn assert_preserved_content(
        state: &mut DefensesWindowState,
        system: SystemKey,
        selected: DefensesItem,
    ) {
        assert_eq!(state.window_count(), 1);
        let content = state.window_mut(system).unwrap();
        assert_eq!(content.logical_position, ORIGIN);
        assert_eq!(content.page, DefensesPage::Batteries);
        assert_eq!(content.selected, Some(selected));
        assert_eq!(content.first_row, 1);
    }

    fn seed_preserved_content(
        world: &GameWorld,
        state: &mut DefensesWindowState,
        system: SystemKey,
    ) -> DefensesItem {
        let selected = DefensesItem::Defense(world.systems[system].defense_facilities[0]);
        let content = state.window_mut(system).unwrap();
        content.page = DefensesPage::Batteries;
        content.selected = Some(selected);
        content.first_row = 1;
        selected
    }

    #[test]
    fn defenses_window_shared_identity_preserves_content() {
        let (world, system, _) = stocked();
        let (mut manager, mut state, prepared) = prepared_managed_window(&world, system);
        assert_eq!(prepared.spec().key.kind, WindowKind::Defenses);
        assert_eq!(prepared.spec().owner, WindowOwner::StrategicRoot);
        let (id, opened) = commit_prepared_defenses_window(&mut manager, &mut state, prepared);
        assert_eq!(opened, WindowTransition::Opened { id });

        let selected = seed_preserved_content(&world, &mut state, system);

        let reopened = state
            .prepare_open(
                &world,
                system,
                (400, 300),
                CockpitFaction::Alliance,
                scaled(),
            )
            .unwrap();
        crate::test_support::assert_existing_detail_reopen(id, || {
            commit_prepared_defenses_window(&mut manager, &mut state, reopened)
        });
        assert_preserved_content(&mut state, system, selected);

        crate::test_support::assert_detail_minimized_reopen!(
            &mut manager,
            &mut state,
            id,
            system,
            |state: &mut DefensesWindowState| {
                state
                    .prepare_open(
                        &world,
                        system,
                        (500, 350),
                        CockpitFaction::Alliance,
                        scaled(),
                    )
                    .unwrap()
            },
            commit_prepared_defenses_window,
        );
        assert_preserved_content(&mut state, system, selected);
    }

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn defenses_fixture_legacy_open_adopts_manager_identity_without_resetting_content() {
        let (world, system, _) = stocked();
        let mut manager = WindowManager::new();
        let mut state = DefensesWindowState::default();
        assert!(state.open(&world, system, ORIGIN, CockpitFaction::Alliance, scaled(),));
        let selected = seed_preserved_content(&world, &mut state, system);

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
        assert_preserved_content(&mut state, system, selected);
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
        let valid = run_managed_idle(&world, &mut state, &manager);
        assert_empty!(valid.actions);
        assert_eq!(state.release_target(layer), Some(stale_system));

        world.systems[stale_system].dat_id = DatId::new(101);
        world.systems[stale_system].name = "Replacement".into();
        state.refresh_release_targets(&manager.begin_frame(), &world);
        assert_eq!(
            state.release_target(layer),
            None,
            "the app prepass must invalidate a stale layer before Fleet can resolve a drop"
        );
        let rendered = run_managed_idle(&world, &mut state, &manager);

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
        crate::test_support::assert_manager_activation_preserves_content_order(
            &mut manager,
            &ids,
            || {
                state
                    .windows
                    .iter()
                    .filter_map(|window| window.id)
                    .collect()
            },
        );
    }

    #[test]
    fn defenses_visible_content_follows_exact_frozen_identity() {
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let (mut manager, state) = opened_managed_window(&world, system);
        let id = state.window_id(system).unwrap();
        crate::test_support::assert_system_subject_visibility(
            &mut manager,
            &mut world,
            system,
            id,
            DatId::new(101),
            "Defenses",
            |world, snapshot| state.has_visible_content(snapshot, world),
        );
    }

    #[test]
    fn defenses_pointer_hits_only_its_visible_topmost_manager_layer() {
        let (world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let (manager, state) = opened_managed_window(&world, system);
        let id = state.window_id(system).unwrap();
        let snapshot = manager.begin_frame();
        let rect = state.screen_rect(scaled(), system).unwrap();
        let content = state
            .windows
            .iter()
            .find(|window| window.id == Some(id))
            .unwrap();
        let own = egui::LayerId::new(egui::Order::Foreground, area_id(content));
        crate::test_support::assert_detail_window_hits(id, rect, own, |point, layer| {
            state.hit_window_ids(&snapshot, scaled(), point, layer)
        });
    }

    crate::test_support::define_detail_window_point!(at, ORIGIN);

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

        crate::test_support::assert_screen_rect(
            state.tab_screen_rect(scaled(), system, DefensesPage::Shields),
            at(136.0, 20.0),
            egui::vec2(72.0, 66.0),
        );
        crate::test_support::assert_screen_rect(
            state.cell_screen_rect(scaled(), system, 4),
            at(77.0, 151.0),
            egui::vec2(140.0, 140.0),
        );
        let layer = crate::test_support::foreground_layer(area_id(
            state
                .windows
                .iter()
                .find(|window| window.system == system)
                .unwrap(),
        ));
        crate::test_support::assert_detail_canvas_routing(
            system,
            layer,
            at(234.0, 303.0),
            at(235.5, 150.0),
            |point| state.contains_screen_point(scaled(), point),
            |layer| state.release_target(layer),
        );

        crate::test_support::assert_detail_fixture_reset!(
            state,
            system,
            state.open(&world, system, ORIGIN, CockpitFaction::Empire, scaled()),
        );
    }

    type Run = crate::test_support::RenderRun<DefensesWindowAction>;

    fn raw_input(
        index: usize,
        events: Vec<egui::Event>,
        modifiers: egui::Modifiers,
    ) -> egui::RawInput {
        crate::test_support::timed_raw_input(index, events, modifiers, egui::vec2(1400.0, 1000.0))
    }

    /// Draw the windows once per frame of events, on [`scaled`] for the
    /// Alliance; the last frame's text and bitmaps are kept.
    fn run(
        world: &GameWorld,
        missions: &MissionState,
        state: &mut DefensesWindowState,
        frames: Vec<Vec<egui::Event>>,
    ) -> Run {
        run_as(
            world,
            missions,
            state,
            frames,
            (CockpitFaction::Alliance, egui::Modifiers::default()),
        )
    }

    /// [`run`] for `faction`, with `modifiers` held through every frame.
    fn run_as(
        world: &GameWorld,
        missions: &MissionState,
        state: &mut DefensesWindowState,
        frames: Vec<Vec<egui::Event>>,
        (faction, modifiers): (CockpitFaction, egui::Modifiers),
    ) -> Run {
        let mut cache = BmpCache::new();
        let side = if faction == CockpitFaction::Alliance {
            Faction::Alliance
        } else {
            Faction::Empire
        };
        let fog = world
            .systems
            .keys()
            .fold(FogState::new(side), |mut fog, key| {
                fog.reveal(key);
                fog
            });
        let mut world = world.clone();
        let keys: Vec<_> = world.systems.keys().collect();
        for key in keys {
            world.support_values.insert(
                key,
                SupportValues {
                    garrison_requirement: 2,
                    ..SupportValues::default()
                },
            );
        }
        crate::test_support::run_painted_render_frames(
            frames,
            |index, events| raw_input(index, events, modifiers),
            |ctx| {
                draw_defenses_windows(
                    ctx,
                    &world,
                    &fog,
                    missions,
                    &EconomyState::default(),
                    state,
                    faction,
                    scaled(),
                    &mut cache,
                )
            },
        )
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
        let mut cache = BmpCache::new();
        let fog = world
            .systems
            .keys()
            .fold(FogState::new(Faction::Alliance), |mut fog, key| {
                fog.reveal(key);
                fog
            });
        let economy = EconomyState::default();
        let window = state
            .windows
            .iter()
            .find_map(|window| window.id)
            .expect("managed test content has an identity");
        crate::test_support::run_painted_routed_render_frames(
            manager,
            window,
            frames,
            |index, events| raw_input(index, events, egui::Modifiers::default()),
            |ctx, snapshot, routed, router| {
                draw_managed_defenses_windows(
                    ctx,
                    world,
                    &fog,
                    missions,
                    &economy,
                    state,
                    CockpitFaction::Alliance,
                    scaled(),
                    &mut cache,
                    snapshot,
                    routed,
                    router,
                )
            },
        )
    }

    fn run_managed_idle(
        world: &GameWorld,
        state: &mut DefensesWindowState,
        manager: &WindowManager,
    ) -> Run {
        run_managed(
            world,
            &MissionState::new(),
            state,
            manager,
            vec![(vec![egui::Event::PointerMoved(at(100.0, 100.0))], vec![])],
        )
    }

    fn begin_squadron_drag(
        world: &GameWorld,
        missions: &MissionState,
        state: &mut DefensesWindowState,
        faction: CockpitFaction,
        start: egui::Pos2,
        end: egui::Pos2,
    ) {
        let plain = (faction, egui::Modifiers::default());
        let _ = run_as(world, missions, state, click(at(118.0, 36.0)), plain);
        let mut frames = hover(start);
        frames.extend([
            vec![press(start, true)],
            vec![egui::Event::PointerMoved(end)],
        ]);
        let _ = run_as(world, missions, state, frames, plain);
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

        crate::test_support::assert_detail_chrome(&run.painted, BACKGROUND, 10299, at);
        crate::test_support::assert_painted_at(
            &run.painted,
            [
                (10571, at(28.0, 20.0)),
                (10563, at(64.0, 20.0)),
                (10558, at(100.0, 20.0)),
                (10553, at(136.0, 20.0)),
                (10550, at(172.0, 20.0)),
                (18_176 + 832, at(7.0, 81.0)),
                (18_176 + 832, at(77.0, 81.0)),
                (17_728, at(147.0, 81.0)),
            ],
        );
        assert!(!run.painted.iter().any(|(id, _)| ROW_FRAME.contains(id)));
        let idle = run.text("Idle");
        assert_eq!(idle.pos, at(8.0, 108.0));
        // The cell less 3 (FUN_00609ae0's text rect), at scale 2.
        assert_eq!((idle.font, idle.wrap), (18.0, 134.0));
        assert_eq!(run.text("Hidden").color, egui::Color32::WHITE);
        // `+0x49` at (19, 2), 16 high: left-aligned, vertically centred.
        let title = run.text("Sluis Van");
        assert_eq!((title.pos.x, title.font), (at(19.0, 0.0).x, 22.0));
        assert_eq!(title.pos.y + title.size.y / 2.0, at(0.0, 10.0).y);
        // `+0x5b` at (2, 51), 231 by 16: centred both ways.
        let name = run.text("Personnel");
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
            selected.text("Hidden").color,
            egui::Color32::from_rgb(255, 0, 0)
        );
        assert_eq!(
            selected.actions,
            [DefensesWindowAction::SelectSystem(system)]
        );
        let report = state
            .report(&world, &seen(system), &missions, system)
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
        let garrison = regiments.text("Garrison Requirement: 2");
        assert_eq!(garrison.pos + garrison.size / 2.0, at(116.0, 71.5));
        assert!(regiments.painted.contains(&(10564, at(64.0, 20.0))));
        assert!(regiments.painted.contains(&(17_473, at(7.0, 81.0))));
    }

    #[test]
    fn the_title_buttons_open_the_sector_minimize_and_close() {
        // 0xca opens the sector window (FUN_00429ce0); 0xc9 posts 0x466;
        // 200 closes.
        let (world, system, missions) = stocked();
        crate::test_support::assert_detail_title_buttons!(
            at,
            || opened(&world, system),
            |state, frames| run(&world, &missions, state, frames),
            DefensesWindowAction,
            DefensesWindowState,
            system,
            ORIGIN,
        );
    }

    #[test]
    fn a_title_button_shows_its_pressed_art_only_while_the_button_is_held_over_it() {
        // FUN_004a8790's buttons swap to their pressed bitmaps under a held
        // pointer only.
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);
        crate::test_support::assert_strategic_title_button_press(at, |frames| {
            let result = run(&world, &missions, &mut state, frames);
            (result, state.window_count())
        });
    }

    // FUN_00422ce0 issues a type 10 (Defenses window) list drop as 0x202
    // when GetAsyncKeyState(0x11) finds Ctrl down, else 0x201
    // (move-order.md, "A drag is a move").
    #[test]
    fn a_squadron_dropped_with_ctrl_held_is_a_confirmed_move_for_either_side() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let (world, system, class, missions) = fighter_drag_fixture(faction);
            for (modifiers, confirmed) in [
                (egui::Modifiers::default(), false),
                (egui::Modifiers::CTRL, true),
            ] {
                let mut state = DefensesWindowState::default();
                assert!(state.open(&world, system, ORIGIN, faction, scaled()));
                let (start, end) = (at(30.0, 100.0), at(300.0, 100.0));
                begin_squadron_drag(&world, &missions, &mut state, faction, start, end);
                let released = run_as(
                    &world,
                    &missions,
                    &mut state,
                    vec![vec![egui::Event::PointerMoved(end), press(end, false)]],
                    (faction, modifiers),
                );
                let drops: Vec<_> = released
                    .actions
                    .iter()
                    .filter_map(|action| match *action {
                        DefensesWindowAction::DragSquadron {
                            class: dropped,
                            confirmed,
                            ..
                        } => Some((dropped, confirmed)),
                        _ => None,
                    })
                    .collect();
                assert_eq!(drops, [(class, confirmed)], "{faction:?}");
            }
        }
    }

    // CoolDragList (FUN_006083c0) posts 0x29a only on a release more than
    // sqrt(24) pixels from the press (squared distance above 0x18) and
    // outside the list's client rect (move-order.md, "Port notes").
    #[test]
    fn a_squadron_drop_needs_a_long_move_that_leaves_the_list_for_either_side() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let (world, system, _class, missions) = fighter_drag_fixture(faction);
            let plain = (faction, egui::Modifiers::default());
            let drops = |from: (f32, f32), to: (f32, f32)| {
                let mut state = DefensesWindowState::default();
                assert!(state.open(&world, system, ORIGIN, faction, scaled()));
                let (start, end) = (at(from.0, from.1), at(to.0, to.1));
                begin_squadron_drag(&world, &missions, &mut state, faction, start, end);
                run_as(
                    &world,
                    &missions,
                    &mut state,
                    vec![vec![egui::Event::PointerMoved(end), press(end, false)]],
                    plain,
                )
                .actions
                .iter()
                .filter(|action| matches!(action, DefensesWindowAction::DragSquadron { .. }))
                .count()
            };

            assert_eq!(
                drops((8.0, 100.0), (4.0, 100.0)),
                0,
                "{faction:?}: 4 pixels, outside the list"
            );
            assert_eq!(
                drops((30.0, 100.0), (100.0, 100.0)),
                0,
                "{faction:?}: inside the list"
            );
            assert_eq!(
                drops((30.0, 100.0), (300.0, 100.0)),
                1,
                "{faction:?}: far and outside"
            );
        }
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
    fn a_mission_released_on_a_row_names_its_regiment_or_facility() {
        // FUN_004aa380: the list item under the point (list +0x164 at
        // (7, 81)) on any page, else the subject (move-order.md, "Hit
        // tests").
        let (world, system, missions) = stocked();
        let mut state = opened(&world, system);
        let layer = egui::LayerId::new(egui::Order::Foreground, area_id(&state.windows[0]));
        let _ = run(&world, &missions, &mut state, click(at(80.0, 30.0)));
        let troop = world.systems[system].ground_units[0];
        assert_eq!(
            state.release_object(scaled(), layer, at(30.0, 100.0)),
            Some(MissionTarget::Troop(troop))
        );
        assert_eq!(state.release_object(scaled(), layer, at(100.0, 60.0)), None);

        let _ = run(&world, &missions, &mut state, click(at(150.0, 30.0)));
        let shield = world.systems[system]
            .defense_facilities
            .iter()
            .copied()
            .find(|&key| {
                state.windows[0]
                    .painted
                    .contains(&DefensesItem::Defense(key))
            });
        assert_eq!(
            state.release_object(scaled(), layer, at(30.0, 100.0)),
            shield.map(MissionTarget::DefenseFacility)
        );
        assert!(shield.is_some());
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
        assert_empty!(menus(&outside));
    }

    #[test]
    fn managed_secondary_press_selects_and_clears_with_its_exact_phase() {
        let (world, system, missions) = stocked();
        let (manager, mut state) = opened_managed_window(&world, system);
        let row = at(100.0, 100.0);
        run_managed(
            &world,
            &missions,
            &mut state,
            &manager,
            crate::test_support::routed_secondary_press_frames(row),
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
            crate::test_support::routed_secondary_press_frames(empty),
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
            add_character(&mut world, system, format!("Extra {index}"), true);
        }
        let (manager, mut state) = opened_managed_window(&world, system);
        let thumb = at(220.0, 100.0);
        let dragged = at(220.0, 200.0);
        run_managed(
            &world,
            &missions,
            &mut state,
            &manager,
            crate::test_support::routed_primary_drag_frames(thumb, dragged),
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
            add_character(&mut world, system, format!("Extra {index}"), true);
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

    #[test]
    fn a_saved_defenses_window_comes_back_in_place_on_the_personnel_page() {
        // FUN_004aa6d0 saves the system and no page; FUN_00441190 rebuilds
        // type 10 at the saved x and y (FUN_004a7790), which opens on the
        // personnel page.
        let (mut world, first) = world(ControlKind::Uncontrolled);
        let second = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0002),
            ..world.systems[first].clone()
        });
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut state = DefensesWindowState::default();
            assert!(state.open(&world, first, (40, 50), faction, scaled()));
            assert!(state.open(&world, second, ORIGIN, faction, scaled()));
            state.windows[0].page = DefensesPage::Regiments;
            let saved = state.saved_windows(&world);
            assert!(saved.iter().all(|window| window.shown));

            let restored = crate::test_support::restored_saved_windows(
                &saved,
                faction,
                |state: &mut DefensesWindowState, window| {
                    state.restore_saved(&world, faction, window)
                },
            );

            let back: Vec<_> = restored
                .windows
                .iter()
                .map(|window| (window.system, window.logical_position, window.page))
                .collect();
            assert_eq!(
                back,
                [
                    (first, (40, 50), DefensesPage::Personnel),
                    (second, ORIGIN, DefensesPage::Personnel),
                ],
                "{faction:?}"
            );
            let hidden = SavedWindow {
                shown: false,
                ..saved[0]
            };
            let mut none = DefensesWindowState::default();
            assert!(!none.restore_saved(&world, faction, &hidden));
            assert_eq!(none.window_count(), 0);
        }
    }

    #[test]
    fn a_keyed_close_closes_that_window_and_leaves_the_rest_for_either_side() {
        // FUN_004ac3a0: WM_KEYDOWN 0x1b calls slot +0x30 on the focused window.
        crate::test_support::assert_strategic_keyed_close(
            DefensesWindowState::default,
            |state, world, system, faction| state.open(world, system, ORIGIN, faction, scaled()),
            DefensesWindowState::close_window,
            DefensesWindowState::is_open,
            DefensesWindowState::window_count,
        );
    }

    #[test]
    fn the_view_raises_one_window_of_the_kind_above_the_rest_for_either_side() {
        // FUN_00422ce0 0x467: BringWindowToTop on the window it activates.
        crate::test_support::assert_strategic_raise(
            DefensesWindowState::default,
            |state, world, system, faction| state.open(world, system, ORIGIN, faction, scaled()),
            DefensesWindowState::raise,
            |state| state.children().count(),
            DefensesWindowState::window_count,
        );
    }

    #[test]
    fn a_press_on_the_windows_bare_surface_drags_it_but_not_on_its_list_or_tabs() {
        // FUN_004a7790 sets the movable bit (+0xb8 | 4) and FUN_004a86d0
        // passes every press on the window to the base (FUN_00606ee0), which
        // takes hold; FUN_006071e0 places it live and FUN_00607010 lets go.
        // The list (with its bar) and the tabs are controls of their own.
        let (world, system, missions) = stocked();
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let moves = |from: (f32, f32)| {
                let mut state = DefensesWindowState::default();
                assert!(state.open(&world, system, ORIGIN, faction, scaled()));
                let start = at(from.0, from.1);
                let _ = run_as(
                    &world,
                    &missions,
                    &mut state,
                    title_drag(start, start + egui::vec2(40.0, 40.0)),
                    (faction, egui::Modifiers::default()),
                );
                let moved = state.windows[0].logical_position;
                assert!(moved == ORIGIN || moved == (ORIGIN.0 + 20, ORIGIN.1 + 20));
                moved != ORIGIN
            };
            assert!(moves((100.0, 9.0)), "{faction:?}: title");
            assert!(moves((115.0, 297.0)), "{faction:?}: under the list");
            assert!(!moves((100.0, 150.0)), "{faction:?}: list");
            assert!(!moves((40.0, 35.0)), "{faction:?}: tab");
            assert!(!moves((210.0, 9.0)), "{faction:?}: minimize");
        }
    }
}
