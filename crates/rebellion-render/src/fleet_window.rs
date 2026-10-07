//! The original Fleet window (window type 4, `FUN_004a2630`): a system's
//! fleets on the left, the selected fleet's contents under four tabs on the
//! right (manual Fig. 3.65, p. 122). Recovery notes:
//! `ghidra/notes/fleet-window.md`.
//!
//! It opens from the fleet icon a sector window shows at the top right of a
//! planet (`FUN_0045ccc0`, `FUN_0045aac0`), and a move released over it
//! targets the fleet under the point (`+0x70`, `FUN_004a3130`), which is how
//! a regiment boards a fleet. A regiment dragged out of the Troops tab moves
//! to where it is dropped (`0x201`, `ghidra/notes/regiment-unload.md`).

use egui_macroquad::egui;
use rebellion_core::dat::{ExplorationStatus, Faction};
use rebellion_core::fleet_join;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{DatId, FleetKey, SystemKey, TroopKey};
use rebellion_core::movement::MovementState;
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::world::{ControlKind, GameWorld};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::object_menu::MenuObject;
use crate::panels::fleets::{capital_ship_mini_id, fighter_mini_id};
use crate::quadrant_icons::{quadrant_art, Quadrant};
use crate::system_window::{
    canvas_point, character_mini_resource_id, clamp_window_to_galaxy, exact_clicked, fleet_label,
    logical_rect, opposing_contents_visible, rect_contains, troop_mini,
};
use crate::targeting::ReleaseTarget;

pub const FLEET_WINDOW_WIDTH: f32 = 235.0;
pub const FLEET_WINDOW_HEIGHT: f32 = 304.0;

const BACKGROUND: u32 = 10770;
const CLOSE_NORMAL: u32 = 10108;
const CLOSE_PRESSED: u32 = 10109;
const MINIMIZE_NORMAL: u32 = 10253;
const MINIMIZE_PRESSED: u32 = 10254;
const SECTOR_NORMAL: u32 = 10209;
const SECTOR_PRESSED: u32 = 10208;
/// Alliance art; the Empire's is always 50 higher (`(side != 1) * 0x32`).
const EMPIRE_ART_OFFSET: u32 = 50;
const FLEET_FRAME: u32 = 10400;
const FLEET_SELECTED: u32 = 10401;
const PANE_ART: u32 = 10407;
const TAB_BASE: u32 = 10409;
const RIGHT_ITEM_SELECTED: u32 = 10420;
const FLEET_PICTURE: u32 = 10425;
/// An en route fleet's overlays (`+0x50` bit 4): 10423 over its entry
/// (`FUN_004a37c0`) and 10426 over its picture (`FUN_004a5c00`).
const EN_ROUTE_ENTRY: u32 = 10423;
const EN_ROUTE_PICTURE: u32 = 10426;
const NO_HYPERDRIVE: u32 = 10430;
/// STRATEGY 11501 (`0x2ced`): the en route mark of a character's mini.
const EN_ROUTE_PERSONNEL: u32 = 11501;
/// STRATEGY 11515 (`0x2cfb`): the en route mark of a regiment's mini.
const EN_ROUTE_REGIMENT: u32 = 11515;

/// What a mini stands for, as `FUN_0042c3b0` picks its en route mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MiniObject {
    /// A capital ship or squadron.
    Craft,
    Regiment(DatId),
    Character,
    SpecialForce(DatId),
}

/// The mark `FUN_0042c3b0(.., 0, 1)` draws over an en route object's `mini`
/// (`+0x50` bit 4 set, bit 3 clear), at the mini's origin. Its default is the
/// object's own GOKRES mark `(class & 0xfff) + 0x5000`, the mini's id plus
/// `0x1000`; it stays for craft, regiment classes `0x10000002` and
/// `0x10000008`, and special force classes `0x3c000003` and `0x3c000005`.
/// Other regiments take 11515, and characters and other special forces
/// 11501. An object is en route while its container is (`FUN_004f8240`), so
/// everything aboard a travelling fleet carries its mark.
pub(crate) fn en_route_mark(object: MiniObject, mini: u32) -> (DllSource, u32) {
    let own = (DllSource::Gokres, mini + 0x1000);
    match object {
        MiniObject::Craft => own,
        MiniObject::Regiment(class) => match class.raw() {
            0x1000_0002 | 0x1000_0008 => own,
            _ => (DllSource::Strategy, EN_ROUTE_REGIMENT),
        },
        MiniObject::Character => (DllSource::Strategy, EN_ROUTE_PERSONNEL),
        MiniObject::SpecialForce(class) => match class.raw() {
            0x3c00_0003 | 0x3c00_0005 => own,
            _ => (DllSource::Strategy, EN_ROUTE_PERSONNEL),
        },
    }
}

/// A craft's GOKRES portrait (`FUN_0042c3b0(.., 1, ..)`: `class & 0xfff`),
/// its mini less `0x4000`.
const fn ship_portrait(mini: u32) -> u32 {
    mini - 0x4000
}

/// The en route mark `FUN_0042c3b0(.., 1, 1)` draws over a craft's
/// portrait: its own GOKRES mark, `(class & 0xfff) + 0x1000`.
const fn en_route_portrait_mark(portrait: u32) -> u32 {
    portrait + 0x1000
}

/// The galaxy view rail's icon for a minimized Fleet window (`FUN_004a76e0`).
const RAIL_ICONS: [u32; 3] = [11536, 11537, 11538];

const LEFT_LIST: (f32, f32, f32, f32) = (4.0, 29.0, 91.0, 266.0);
const RIGHT_LIST: (f32, f32, f32, f32) = (101.0, 127.0, 133.0, 164.0);
const PICTURE_PANEL: (f32, f32, f32, f32) = (100.0, 42.0, 125.0, 49.0);
const TAB_STRIP: (f32, f32) = (99.0, 96.0);
const TAB_X: [f32; 4] = [1.0, 33.0, 65.0, 97.0];
const TAB_SIZE: (f32, f32) = (31.0, 28.0);
const LEFT_ITEM_HEIGHT: f32 = 50.0;
const RIGHT_ITEM_HEIGHT: f32 = 50.0;

/// The four tabs (`0x66..0x69`), in strip order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FleetWindowTab {
    CapitalShips,
    Fighters,
    Troops,
    Personnel,
}

impl FleetWindowTab {
    pub const ALL: [Self; 4] = [
        Self::CapitalShips,
        Self::Fighters,
        Self::Troops,
        Self::Personnel,
    ];

    const fn index(self) -> u32 {
        match self {
            Self::CapitalShips => 0,
            Self::Fighters => 1,
            Self::Troops => 2,
            Self::Personnel => 3,
        }
    }
}

/// A left-list entry: a fleet, or one of its capital ships while the fleet
/// is expanded (`FUN_004a37c0`, `FUN_004a3d40`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FleetWindowEntry {
    Fleet(FleetKey),
    Ship { fleet: FleetKey, index: usize },
}

impl FleetWindowEntry {
    #[must_use]
    pub const fn fleet(self) -> FleetKey {
        match self {
            Self::Fleet(fleet) | Self::Ship { fleet, .. } => fleet,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenFleetWindow {
    system: SystemKey,
    logical_position: (i16, i16),
    /// port: one selected entry; the list's own selection rules
    /// (`FUN_00609410`) are untraced.
    selected: Option<FleetWindowEntry>,
    selected_item: Option<usize>,
    expanded: Vec<FleetKey>,
    tab: FleetWindowTab,
    /// The in-place name edit Rename (0x203) opens (`FUN_004ac7a0`).
    rename: Option<RenameEdit>,
}

/// A Rename's edit field (`+0x138`, a CoolStringField) over the entry it
/// renames, with the order pending at `+0x13c`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RenameEdit {
    entry: FleetWindowEntry,
    text: String,
    /// The first frame selects the whole name and takes the focus
    /// (`FUN_00605110(field, 0, -1)`, `SetFocus`).
    fresh: bool,
}

/// What a frame did to an open rename edit.
#[derive(Debug, Clone, PartialEq, Eq)]
enum RenameOutcome {
    Typed(String),
    Commit(String),
    Cancel,
}

/// `CoolDragList` posts `0x29a` only when the release lies more than this
/// squared distance, in list pixels, from the press (`FUN_006083c0`).
const DRAG_DISTANCE_SQUARED: f32 = 24.0;

/// A left press held on a list item until its release (`CoolDragList`,
/// `FUN_006083c0`).
#[derive(Debug, Clone, Copy, PartialEq)]
struct ItemDrag {
    object: ItemObject,
    press: egui::Pos2,
    list: egui::Rect,
}

/// The object a list item stands for, which a drag carries and a right
/// click opens a menu for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemObject {
    /// A left-list fleet entry.
    Fleet(FleetKey),
    Regiment(TroopKey),
    /// A capital ship, by its index in the fleet's `capital_ships` and the
    /// fleet's roster then (`fleet_join::roster`).
    Ship {
        fleet: FleetKey,
        index: usize,
        roster: u64,
    },
}

impl ItemObject {
    const fn menu_object(self) -> MenuObject {
        match self {
            Self::Fleet(fleet) => MenuObject::Fleet(fleet),
            Self::Regiment(troop) => MenuObject::Troop(troop),
            Self::Ship {
                fleet,
                index,
                roster,
            } => MenuObject::Ship {
                fleet,
                index,
                roster,
            },
        }
    }
}

/// The open Fleet windows. The last is focused and paints on top.
#[derive(Debug)]
pub struct FleetWindowState {
    faction: CockpitFaction,
    windows: Vec<OpenFleetWindow>,
    drag: Option<ItemDrag>,
}

impl Default for FleetWindowState {
    fn default() -> Self {
        Self {
            faction: CockpitFaction::Alliance,
            windows: Vec::new(),
            drag: None,
        }
    }
}

impl FleetWindowState {
    /// Open `system`'s Fleet window at a logical point, clamped into the
    /// galaxy view, or bring its open window to the front: one per system
    /// (`FUN_0045aac0`, id `(system & 0x3ff) << 6 | 4`).
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
        self.windows.push(OpenFleetWindow {
            system,
            logical_position: clamp_window_to_galaxy(
                logical_position,
                layout,
                FLEET_WINDOW_WIDTH,
                FLEET_WINDOW_HEIGHT,
            ),
            selected: None,
            selected_item: None,
            expanded: Vec::new(),
            tab: FleetWindowTab::CapitalShips,
            rename: None,
        });
        true
    }

    /// Select `entry` in `system`'s open window, expanding its fleet for a
    /// ship (slot `+0x6c`, as the Fleet Finder's open does, `FUN_00429440`).
    pub fn select(&mut self, system: SystemKey, entry: FleetWindowEntry) -> bool {
        let Some(window) = self.window_mut(system) else {
            return false;
        };
        if let FleetWindowEntry::Ship { fleet, .. } = entry {
            if !window.expanded.contains(&fleet) {
                window.expanded.push(fleet);
            }
        }
        window.selected = Some(entry);
        window.selected_item = None;
        true
    }

    /// Start Rename (0x203) on `entry` in `system`'s open window
    /// (`FUN_00429350` → `vtable+0x78`, `FUN_004ac7a0`): the entry is
    /// selected and an edit holding its name opens over it, all of it
    /// selected. A rename already open is dropped (`vtable+0x80`).
    pub fn begin_rename(
        &mut self,
        world: &GameWorld,
        system: SystemKey,
        entry: FleetWindowEntry,
    ) -> bool {
        let name = match entry {
            FleetWindowEntry::Fleet(fleet) => world.fleet_name(fleet),
            FleetWindowEntry::Ship { fleet, index } => world.ship_name(fleet, index),
        }
        .map(str::to_owned);
        let Some(text) = name else {
            return false;
        };
        if !self.select(system, entry) {
            return false;
        }
        for window in &mut self.windows {
            window.rename = None;
        }
        if let Some(window) = self.window_mut(system) {
            window.rename = Some(RenameEdit {
                entry,
                text,
                fresh: true,
            });
        }
        true
    }

    /// Whether a rename edit holds the keyboard.
    #[must_use]
    pub fn renaming(&self) -> bool {
        self.windows.iter().any(|window| window.rename.is_some())
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

    /// The screen rect of `system`'s window.
    #[must_use]
    pub fn screen_rect(&self, layout: CockpitLayout, system: SystemKey) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(window_screen_rect(window, layout))
    }

    /// The screen rect of the `row`th left-list entry of `system`'s window.
    #[must_use]
    pub fn entry_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        row: usize,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(logical_rect(
            window_screen_rect(window, layout),
            layout.scale,
            LEFT_LIST.0,
            LEFT_LIST.1 + LEFT_ITEM_HEIGHT * row as f32,
            LEFT_LIST.2,
            LEFT_ITEM_HEIGHT,
        ))
    }

    /// The screen rect of the `row`th right-list item of `system`'s window.
    #[must_use]
    pub fn item_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        row: usize,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(logical_rect(
            window_screen_rect(window, layout),
            layout.scale,
            RIGHT_LIST.0,
            RIGHT_LIST.1 + RIGHT_ITEM_HEIGHT * row as f32,
            125.0,
            RIGHT_ITEM_HEIGHT,
        ))
    }

    /// The screen rect of `tab`'s button in `system`'s window.
    #[must_use]
    pub fn tab_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        tab: FleetWindowTab,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(tab_rect(
            window_screen_rect(window, layout),
            layout.scale,
            tab,
        ))
    }

    /// The selected entry and tab of `system`'s window.
    #[must_use]
    pub fn selection(
        &self,
        system: SystemKey,
    ) -> Option<(Option<FleetWindowEntry>, FleetWindowTab)> {
        self.windows
            .iter()
            .find(|window| window.system == system)
            .map(|window| (window.selected, window.tab))
    }

    /// What `system`'s window shows, for the interface fixture.
    #[must_use]
    pub fn report(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        fog: &FogState,
        transport: &TroopTransportState,
        system: SystemKey,
    ) -> Option<FleetWindowReport> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        let entries = left_entries(world, movement, fog, cockpit_faction(self.faction), window);
        let selected = window.selected.filter(|entry| entries.contains(entry));
        Some(FleetWindowReport {
            origin: window.logical_position,
            entries: entries.len(),
            fleet_rows: entries
                .iter()
                .enumerate()
                .filter_map(|(row, entry)| match entry {
                    FleetWindowEntry::Fleet(fleet) => Some((*fleet, row)),
                    FleetWindowEntry::Ship { .. } => None,
                })
                .collect(),
            selected,
            tab: window.tab,
            enabled: FleetWindowTab::ALL.map(|tab| tab_enabled(world, transport, selected, tab)),
            items: right_items(world, transport, selected, window.tab)
                .into_iter()
                .map(|item| item.label)
                .collect(),
            counts: tab_counts(world, transport, selected, window.tab),
        })
    }

    /// The target a move released at `point` takes from the Fleet window egui
    /// draws as `layer` (`+0x70`, `FUN_004a3130`), or `None` when `layer` is
    /// none of them. The inner `None` is an empty target: the order is
    /// destroyed.
    #[must_use]
    pub fn release_target(
        &self,
        world: &GameWorld,
        movement: &MovementState,
        fog: &FogState,
        layout: CockpitLayout,
        layer: egui::LayerId,
        point: egui::Pos2,
    ) -> Option<Option<ReleaseTarget>> {
        let window = self
            .windows
            .iter()
            .find(|window| area_id(window.system) == layer.id)?;
        let rect = window_screen_rect(window, layout);
        let scale = layout.scale.max(f32::EPSILON);
        let local = (
            (point.x - rect.min.x) / scale,
            (point.y - rect.min.y) / scale,
        );
        let player = cockpit_faction(self.faction);
        let entries = left_entries(world, movement, fog, player, window);
        Some(drop_target(world, player, window, &entries, local))
    }

    pub fn clear(&mut self) {
        self.windows.clear();
        self.drag = None;
    }

    /// Whether a left press on a Troops tab regiment is held: the list has
    /// captured the mouse (`FUN_006083c0`), so nothing under the pointer
    /// answers it.
    #[must_use]
    pub fn is_dragging(&self) -> bool {
        self.drag.is_some()
    }

    /// End a held drag on the left release: far enough from the press and
    /// outside the list, it becomes the `0x29a` drop (`FUN_006083c0`), and
    /// the galaxy view moves the whole selection against the window under
    /// the point (`FUN_00422ce0`, window type 4: `0x201`).
    ///
    /// port: Ctrl's Confirmed Move (`0x202`) is not ported for a regiment.
    fn end_drag(&mut self, ctx: &egui::Context, scale: f32) -> Option<FleetWindowAction> {
        let (released, down, point) = ctx.input(|input| {
            (
                input.pointer.primary_released(),
                input.pointer.primary_down(),
                input.pointer.latest_pos(),
            )
        });
        if !released {
            if !down {
                self.drag = None;
            }
            return None;
        }
        let drag = self.drag.take()?;
        let point = point?;
        let moved = (point - drag.press) / scale;
        if moved.length_sq() <= DRAG_DISTANCE_SQUARED || rect_contains(drag.list, point) {
            return None;
        }
        Some(match drag.object {
            ItemObject::Fleet(fleet) => FleetWindowAction::DragFleet { fleet, point },
            ItemObject::Regiment(troop) => FleetWindowAction::DragRegiment { troop, point },
            ItemObject::Ship {
                fleet,
                index,
                roster,
            } => FleetWindowAction::DragShip {
                fleet,
                index,
                roster,
                point,
            },
        })
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

    fn close(&mut self, system: SystemKey) -> Option<OpenFleetWindow> {
        let index = self
            .windows
            .iter()
            .position(|window| window.system == system)?;
        Some(self.windows.remove(index))
    }

    fn window_mut(&mut self, system: SystemKey) -> Option<&mut OpenFleetWindow> {
        self.windows
            .iter_mut()
            .find(|window| window.system == system)
    }
}

/// What one Fleet window shows ([`FleetWindowState::report`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FleetWindowReport {
    pub origin: (i16, i16),
    pub entries: usize,
    /// Each listed fleet and its row in the left list.
    pub fleet_rows: Vec<(FleetKey, usize)>,
    pub selected: Option<FleetWindowEntry>,
    pub tab: FleetWindowTab,
    pub enabled: [bool; 4],
    pub items: Vec<String>,
    pub counts: Option<(u32, u32)>,
}

/// Actions that leave the Fleet window manager.
#[derive(Debug, Clone, PartialEq)]
pub enum FleetWindowAction {
    /// The restore-sector button (`0xca`) opens the subject's sector window
    /// (`FUN_00429ce0`).
    OpenSector(SystemKey),
    SelectSystem(SystemKey),
    /// The minimize button (`100`) posts `0x466`: the window goes to the
    /// galaxy view's rail.
    Minimize {
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A left-list fleet dragged out of its list (`0x29a`): the galaxy view
    /// hit-tests the screen point and issues `0x201` (`FUN_00422ce0`).
    DragFleet {
        fleet: FleetKey,
        point: egui::Pos2,
    },
    /// A Troops tab regiment dragged out of its list (`0x29a`): the galaxy
    /// view hit-tests the screen point and issues `0x201` (`FUN_00422ce0`).
    DragRegiment {
        troop: TroopKey,
        point: egui::Pos2,
    },
    /// A Capital Ships tab item dragged out of its list: the same `0x201`
    /// for the ship (`ghidra/notes/fleet-join-split.md`).
    DragShip {
        fleet: FleetKey,
        index: usize,
        roster: u64,
        point: egui::Pos2,
    },
    /// A right click on a list entry or item opens its object's pop-up menu
    /// (manual p. 120), at a 640 by 480 canvas point.
    OpenObjectMenu {
        selection: MenuObject,
        point: (i16, i16),
    },
    /// Enter in a Rename's edit with a name in it: `FUN_004ac950` puts the
    /// text in the order (`+0x44`) and issues it; the setter is
    /// `FUN_004f6e60`.
    Rename {
        entry: FleetWindowEntry,
        name: String,
    },
}

/// The side whose fleets a system's fleet icon and rail icon show
/// (`FUN_0045ccc0`, `FUN_004a76e0`): the player's when it has fleets there,
/// else the other side's, else the system's own side bits. Returns the side
/// (1 Alliance, 2 Empire, 3 contested, 0 none) and its fleet count.
///
/// port: the count is the system's fleets the player can see (the system
/// window's rule), and every fleet in the port is active (`+0x50` bit 6).
#[must_use]
pub fn icon_side(
    world: &GameWorld,
    movement: &MovementState,
    fog: &FogState,
    player: Faction,
    system: SystemKey,
) -> (u8, usize) {
    let fleets = visible_fleets(world, movement, fog, player, system);
    let count = |side: u8| {
        fleets
            .iter()
            .filter(|&&fleet| {
                world
                    .fleets
                    .get(fleet)
                    .is_some_and(|value| fleet_side(value.is_alliance) == side)
            })
            .count()
    };
    let own = faction_side(player);
    let other = match own {
        1 => 2,
        2 => 1,
        _ => 0,
    };
    for side in [own, other] {
        let found = count(side);
        if found > 0 {
            return (side, found);
        }
    }
    let side = world
        .systems
        .get(system)
        .map_or(0, |value| control_side(value.control));
    (side, count(side))
}

/// The fleet icon's two bitmaps, or `None` when it is hidden: no fleets, or a
/// side with no art (`FUN_0045d140`, `FUN_0045ca80` kind `0x10`).
#[must_use]
pub fn fleet_icon(
    world: &GameWorld,
    movement: &MovementState,
    fog: &FogState,
    player: Faction,
    system: SystemKey,
) -> Option<(u32, u32)> {
    match icon_side(world, movement, fog, player, system) {
        (_, 0) => None,
        (side, _) => quadrant_art(Quadrant::Fleets, side),
    }
}

/// A minimized Fleet window's rail icon (`FUN_004a76e0`): 11536 for side 1,
/// 11537 for side 2, 11538 otherwise.
#[must_use]
pub fn rail_icon(
    world: &GameWorld,
    movement: &MovementState,
    fog: &FogState,
    player: Faction,
    system: SystemKey,
) -> u32 {
    match icon_side(world, movement, fog, player, system).0 {
        1 => RAIL_ICONS[0],
        2 => RAIL_ICONS[1],
        _ => RAIL_ICONS[2],
    }
}

/// The title strip's side (`+0x16c`, `FUN_004a3340`): the system's side bits
/// with no fleets; otherwise the player's side when one of its fleets is
/// there, else the other side.
fn title_side(world: &GameWorld, fleets: &[FleetKey], player: Faction, system: SystemKey) -> u8 {
    let own = faction_side(player);
    if fleets.is_empty() {
        return world
            .systems
            .get(system)
            .map_or(0, |value| control_side(value.control));
    }
    let own_present = fleets.iter().any(|&fleet| {
        world
            .fleets
            .get(fleet)
            .is_some_and(|value| fleet_side(value.is_alliance) == own)
    });
    if own_present {
        own
    } else if own == 1 {
        2
    } else {
        1
    }
}

/// `FUN_004a3340`: side 1's strip is 10299 active and 10200 inactive, side
/// 2's 10201 and 10302, any other 10303 and 10304. Active is the galaxy
/// view's focused child (`+0xb8`).
fn title_resource(side: u8, focused: bool) -> u32 {
    match (side, focused) {
        (1, true) => 10299,
        (1, false) => 10200,
        (2, true) => 10201,
        (2, false) => 10302,
        (_, true) => 10303,
        (_, false) => 10304,
    }
}

/// The system's fleets in key order that the player can see. hyp: the
/// original's merge loop orders by object id (`+0x6c`); the port's fleets
/// are kept in key order.
fn visible_fleets(
    world: &GameWorld,
    movement: &MovementState,
    fog: &FogState,
    player: Faction,
    system: SystemKey,
) -> Vec<FleetKey> {
    let Some(value) = world.systems.get(system) else {
        return Vec::new();
    };
    if value.exploration_status == ExplorationStatus::Unexplored {
        return Vec::new();
    }
    let opposing_visible = opposing_contents_visible(world, fog, player, system);
    let player_is_alliance = player == Faction::Alliance;
    let mut fleets: Vec<FleetKey> =
        rebellion_core::movement::listed_fleets(movement, world, system)
            .into_iter()
            .filter(|&fleet| {
                world.fleets.get(fleet).is_some_and(|value| {
                    opposing_visible || value.is_alliance == player_is_alliance
                })
            })
            .collect();
    fleets.sort_unstable();
    fleets.dedup();
    fleets
}

fn left_entries(
    world: &GameWorld,
    movement: &MovementState,
    fog: &FogState,
    player: Faction,
    window: &OpenFleetWindow,
) -> Vec<FleetWindowEntry> {
    let mut entries = Vec::new();
    for fleet in visible_fleets(world, movement, fog, player, window.system) {
        entries.push(FleetWindowEntry::Fleet(fleet));
        if window.expanded.contains(&fleet) {
            if let Some(value) = world.fleets.get(fleet) {
                entries.extend(
                    value
                        .capital_ships
                        .iter()
                        .enumerate()
                        .filter(|(_, ship)| ship.alive)
                        .map(|(index, _)| FleetWindowEntry::Ship { fleet, index }),
                );
            }
        }
    }
    entries
}

/// `+0x70` (`FUN_004a3130`) at a window-local logical point.
///
/// 1. The default: the selected own-side fleet; none while own-side fleets
///    are listed but none is selected; the subject (the system) when the
///    list holds no own-side fleet.
/// 2. In the left list, the subject, or the entry under the point.
/// 3. In the right list on the Capital ships tab, the ship under the point.
///
/// port: a ship stands for its fleet, which carries the port's cargo.
fn drop_target(
    world: &GameWorld,
    player: Faction,
    window: &OpenFleetWindow,
    entries: &[FleetWindowEntry],
    local: (f32, f32),
) -> Option<ReleaseTarget> {
    let system = window.system;
    let own = |fleet: FleetKey| {
        world
            .fleets
            .get(fleet)
            .is_some_and(|value| fleet_side(value.is_alliance) == faction_side(player))
    };
    let fleet_target = |fleet: FleetKey| ReleaseTarget::Fleet { fleet, system };
    if in_rect(LEFT_LIST, local) {
        let row = ((local.1 - LEFT_LIST.1) / LEFT_ITEM_HEIGHT) as usize;
        return Some(
            entries
                .get(row)
                .map_or(ReleaseTarget::System(system), |entry| {
                    fleet_target(entry.fleet())
                }),
        );
    }
    if window.tab == FleetWindowTab::CapitalShips && in_rect(RIGHT_LIST, local) {
        if let Some(FleetWindowEntry::Fleet(fleet)) = window.selected {
            let row = ((local.1 - RIGHT_LIST.1) / RIGHT_ITEM_HEIGHT) as usize;
            let ships = world.fleets.get(fleet).map_or(0, |value| {
                value.capital_ships.iter().filter(|ship| ship.alive).count()
            });
            if row < ships {
                return Some(fleet_target(fleet));
            }
        }
    }
    let own_listed = entries
        .iter()
        .any(|entry| matches!(entry, FleetWindowEntry::Fleet(fleet) if own(*fleet)));
    if !own_listed {
        return Some(ReleaseTarget::System(system));
    }
    match window.selected {
        Some(FleetWindowEntry::Fleet(fleet)) if own(fleet) => Some(fleet_target(fleet)),
        _ => None,
    }
}

/// A tab's state for the selection: Capital ships is disabled for a ship;
/// the others unless the selection carries their contents (`FUN_004a5c00`,
/// flags 1, 2 and 4).
fn tab_enabled(
    world: &GameWorld,
    transport: &TroopTransportState,
    selected: Option<FleetWindowEntry>,
    tab: FleetWindowTab,
) -> bool {
    let Some(FleetWindowEntry::Fleet(fleet)) = selected else {
        // port: a ship's own contents are not modelled; the port's cargo,
        // fighters and characters belong to the fleet.
        return false;
    };
    let Some(value) = world.fleets.get(fleet) else {
        return false;
    };
    match tab {
        FleetWindowTab::CapitalShips => true,
        FleetWindowTab::Fighters => value.fighters.iter().any(|entry| entry.count > 0),
        FleetWindowTab::Troops => transport.carried_count(fleet) > 0,
        FleetWindowTab::Personnel => !value.characters.is_empty(),
    }
}

/// One right-list item: its GOKRES mini, name and whether it lacks a
/// hyperdrive (indicator 10430, flag `0x40`).
#[derive(Debug, Clone, PartialEq, Eq)]
struct RightItem {
    mini: Option<u32>,
    /// What the mini stands for, for its en route mark.
    kind: MiniObject,
    label: String,
    no_hyperdrive: bool,
    /// The regiment or ship the item stands for.
    object: Option<ItemObject>,
}

/// The current tab's objects aboard the selected fleet (`FUN_004a6be0`).
fn right_items(
    world: &GameWorld,
    transport: &TroopTransportState,
    selected: Option<FleetWindowEntry>,
    tab: FleetWindowTab,
) -> Vec<RightItem> {
    let Some(FleetWindowEntry::Fleet(fleet)) = selected else {
        return Vec::new();
    };
    let Some(value) = world.fleets.get(fleet) else {
        return Vec::new();
    };
    let roster = fleet_join::roster(world, fleet).unwrap_or_default();
    match tab {
        FleetWindowTab::CapitalShips => value
            .capital_ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| ship.alive)
            .filter_map(|(index, ship)| Some((index, world.capital_ship_classes.get(ship.class)?)))
            .map(|(index, class)| RightItem {
                mini: capital_ship_mini_id(class.dat_id),
                kind: MiniObject::Craft,
                // FUN_004f6270: the ship's own name, else its class's.
                label: world
                    .ship_name(fleet, index)
                    .unwrap_or(&class.name)
                    .to_owned(),
                no_hyperdrive: class.hyperdrive == 0,
                object: Some(ItemObject::Ship {
                    fleet,
                    index,
                    roster,
                }),
            })
            .collect(),
        // port: the port keeps squadrons as counts per class.
        FleetWindowTab::Fighters => value
            .fighters
            .iter()
            .filter_map(|entry| Some((world.fighter_classes.get(entry.class)?, entry.count)))
            .flat_map(|(class, count)| {
                (0..count).map(move |_| RightItem {
                    mini: fighter_mini_id(class.dat_id),
                    kind: MiniObject::Craft,
                    label: class.name.clone(),
                    no_hyperdrive: false,
                    object: None,
                })
            })
            .collect(),
        FleetWindowTab::Troops => transport
            .cargo(fleet)
            .iter()
            .filter_map(|&key| Some((key, world.troops.get(key)?)))
            .filter_map(|(key, troop)| {
                Some((key, troop.class_dat_id, troop_mini(troop.class_dat_id)?))
            })
            .map(|(key, class, (mini, label))| RightItem {
                mini: Some(mini),
                kind: MiniObject::Regiment(class),
                label: label.to_owned(),
                no_hyperdrive: false,
                object: Some(ItemObject::Regiment(key)),
            })
            .collect(),
        FleetWindowTab::Personnel => value
            .characters
            .iter()
            .filter_map(|&character| world.characters.get(character))
            .map(|character| RightItem {
                mini: character_mini_resource_id(character.dat_id, character.is_major),
                kind: MiniObject::Character,
                label: character.name.clone(),
                no_hyperdrive: false,
                object: None,
            })
            .collect(),
    }
}

/// The two numbers tabs `0x67` and `0x68` print into the picture panel: the
/// squadrons or regiments aboard and the capacity (`FUN_004a5c00`, ship
/// slots `+0x23c`/`+0x240` and `+0x26c`/`+0x270`, summed).
fn tab_counts(
    world: &GameWorld,
    transport: &TroopTransportState,
    selected: Option<FleetWindowEntry>,
    tab: FleetWindowTab,
) -> Option<(u32, u32)> {
    let Some(FleetWindowEntry::Fleet(fleet)) = selected else {
        return None;
    };
    let value = world.fleets.get(fleet)?;
    match tab {
        FleetWindowTab::Fighters => {
            let aboard = value.fighters.iter().map(|entry| entry.count).sum();
            let capacity = value
                .capital_ships
                .iter()
                .filter(|ship| ship.alive)
                .filter_map(|ship| world.capital_ship_classes.get(ship.class))
                .map(|class| class.fighter_capacity)
                .fold(0_u32, u32::saturating_add);
            Some((aboard, capacity))
        }
        FleetWindowTab::Troops => Some((
            u32::try_from(transport.carried_count(fleet)).unwrap_or(u32::MAX),
            TroopTransportState::fleet_capacity(world, fleet)?,
        )),
        FleetWindowTab::CapitalShips | FleetWindowTab::Personnel => None,
    }
}

/// The left edge of a picture `width` wide, centred on whole pixels in the
/// 125-wide picture panel (`FUN_004a5c00`).
fn picture_x(width: f32) -> f32 {
    PICTURE_PANEL.0 + ((PICTURE_PANEL.2 - width) / 2.0).floor()
}

/// Tab `k`'s bitmap from a strip's base: normal `b + k`, disabled
/// `b + 3 + k`, selected `b + 7 + k` (`FUN_004a4b10`).
fn tab_resource(base: u32, tab: FleetWindowTab, selected: bool, enabled: bool) -> u32 {
    let k = tab.index();
    if !enabled {
        base + 3 + k
    } else if selected {
        base + 7 + k
    } else {
        base + k
    }
}

fn side_art(base: u32, side: u8) -> u32 {
    if side == 2 {
        base + EMPIRE_ART_OFFSET
    } else {
        base
    }
}

pub(crate) const fn fleet_side(is_alliance: bool) -> u8 {
    if is_alliance {
        1
    } else {
        2
    }
}

pub(crate) fn faction_side(faction: Faction) -> u8 {
    match faction {
        Faction::Alliance => 1,
        Faction::Empire => 2,
        Faction::Neutral => 0,
    }
}

/// hyp: `Contested` is side 3, and an uprising keeps its holder's side
/// (`ghidra/notes/blockade-bit.md`, "Port").
pub(crate) fn control_side(control: ControlKind) -> u8 {
    match control {
        ControlKind::Contested => 3,
        control => control.faction().map_or(0, faction_side),
    }
}

fn cockpit_faction(faction: CockpitFaction) -> Faction {
    match faction {
        CockpitFaction::Alliance => Faction::Alliance,
        CockpitFaction::Empire => Faction::Empire,
    }
}

fn in_rect(rect: (f32, f32, f32, f32), point: (f32, f32)) -> bool {
    point.0 >= rect.0 && point.0 < rect.0 + rect.2 && point.1 >= rect.1 && point.1 < rect.1 + rect.3
}

fn area_id(system: SystemKey) -> egui::Id {
    egui::Id::new(("original-fleet-window", system))
}

fn window_screen_rect(window: &OpenFleetWindow, layout: CockpitLayout) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + f32::from(window.logical_position.0) * layout.scale,
            layout.canvas.y + f32::from(window.logical_position.1) * layout.scale,
        ),
        egui::vec2(
            FLEET_WINDOW_WIDTH * layout.scale,
            FLEET_WINDOW_HEIGHT * layout.scale,
        ),
    )
}

fn tab_rect(window: egui::Rect, scale: f32, tab: FleetWindowTab) -> egui::Rect {
    logical_rect(
        window,
        scale,
        TAB_STRIP.0 + TAB_X[tab.index() as usize],
        TAB_STRIP.1,
        TAB_SIZE.0,
        TAB_SIZE.1,
    )
}

#[derive(Default)]
struct WindowDrawResult {
    focus: bool,
    close: bool,
    minimize: bool,
    open_sector: bool,
    select: Option<Option<FleetWindowEntry>>,
    toggle: Option<FleetKey>,
    tab: Option<FleetWindowTab>,
    item: Option<usize>,
    drag: Option<ItemDrag>,
    object_menu: Option<(MenuObject, egui::Pos2)>,
    rename: Option<RenameOutcome>,
}

/// Draw every open Fleet window.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit state and rendering inputs at this UI boundary, as the system window does."
)]
pub fn draw_fleet_windows(
    ctx: &egui::Context,
    world: &GameWorld,
    movement: &MovementState,
    fog: &FogState,
    transport: &TroopTransportState,
    state: &mut FleetWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Vec<FleetWindowAction> {
    state.prepare_faction(faction);
    let windows = state.windows.clone();
    let focused_system = windows.last().map(|window| window.system);
    let mut actions: Vec<_> = state.end_drag(ctx, layout.scale).into_iter().collect();
    for window in &windows {
        let result = draw_fleet_window(
            ctx,
            world,
            movement,
            fog,
            transport,
            window,
            focused_system == Some(window.system),
            faction,
            layout,
            cache,
        );
        let system = window.system;
        if let Some(drag) = result.drag {
            state.drag = Some(drag);
        }
        if let Some((selection, point)) = result.object_menu {
            actions.push(FleetWindowAction::OpenObjectMenu {
                selection,
                point: canvas_point(layout, point),
            });
        }
        if result.close || !world.systems.contains_key(system) {
            state.close(system);
            continue;
        }
        if result.minimize {
            if let Some(closed) = state.close(system) {
                actions.push(FleetWindowAction::Minimize {
                    system,
                    logical_position: closed.logical_position,
                });
            }
            continue;
        }
        if result.open_sector {
            actions.push(FleetWindowAction::OpenSector(system));
        }
        if let Some(open) = state.window_mut(system) {
            if let Some(selected) = result.select {
                open.selected = selected;
                open.selected_item = None;
            }
            if let Some(fleet) = result.toggle {
                if let Some(index) = open.expanded.iter().position(|&key| key == fleet) {
                    open.expanded.remove(index);
                } else {
                    open.expanded.push(fleet);
                }
            }
            if let Some(tab) = result.tab {
                open.tab = tab;
                open.selected_item = None;
            }
            if let Some(item) = result.item {
                open.selected_item = Some(item);
            }
            match result.rename {
                Some(RenameOutcome::Typed(text)) => {
                    if let Some(edit) = &mut open.rename {
                        edit.text = text;
                        edit.fresh = false;
                    }
                }
                Some(RenameOutcome::Commit(name)) => {
                    if let Some(edit) = open.rename.take() {
                        actions.push(FleetWindowAction::Rename {
                            entry: edit.entry,
                            name,
                        });
                    }
                }
                Some(RenameOutcome::Cancel) => open.rename = None,
                None => {}
            }
        }
        if result.focus {
            state.focus(system);
            actions.push(FleetWindowAction::SelectSystem(system));
        }
    }
    actions
}

#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "Keep the window's ordered paint and input pass together, as the system window does."
)]
fn draw_fleet_window(
    ctx: &egui::Context,
    world: &GameWorld,
    movement: &MovementState,
    fog: &FogState,
    transport: &TroopTransportState,
    window: &OpenFleetWindow,
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
    let viewer = faction_side(player);
    let other = if viewer == 1 { 2 } else { 1 };
    let entries = left_entries(world, movement, fog, player, window);
    let fleets: Vec<FleetKey> = entries
        .iter()
        .filter_map(|entry| match entry {
            FleetWindowEntry::Fleet(fleet) => Some(*fleet),
            FleetWindowEntry::Ship { .. } => None,
        })
        .collect();
    let selected = window.selected.filter(|entry| entries.contains(entry));
    let selected_side = selected.and_then(|entry| {
        world
            .fleets
            .get(entry.fleet())
            .map(|value| fleet_side(value.is_alliance))
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
            paint(
                cache,
                title_resource(title_side(world, &fleets, player, window.system), focused),
                2.0,
                2.0,
            );
            // The title label starts after the restore-sector button: its
            // width plus 5 (`FUN_004a4b10`).
            painter.text(
                logical_rect(local, scale, 19.0, 3.0, 0.0, 0.0).min,
                egui::Align2::LEFT_TOP,
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

            // The right pane shows only while something is selected: the
            // player's art for its own side, the other art otherwise.
            if let Some(side) = selected_side {
                let art_side = if side == viewer { viewer } else { other };
                paint(cache, side_art(PANE_ART, art_side), 97.0, 29.0);
            }

            // The left list: fleet entries, and ships while expanded.
            let list = logical_rect(
                local,
                scale,
                LEFT_LIST.0,
                LEFT_LIST.1,
                LEFT_LIST.2,
                LEFT_LIST.3,
            );
            let list_painter = painter.with_clip_rect(list);
            let font = egui::FontId::proportional((9.0 * scale).max(6.0));
            for (row, entry) in entries.iter().enumerate() {
                let top = LEFT_LIST.1 + LEFT_ITEM_HEIGHT * row as f32;
                if top >= LEFT_LIST.1 + LEFT_LIST.3 {
                    break;
                }
                let item = logical_rect(
                    local,
                    scale,
                    LEFT_LIST.0,
                    top,
                    LEFT_LIST.2,
                    LEFT_ITEM_HEIGHT,
                );
                let side = world
                    .fleets
                    .get(entry.fleet())
                    .map_or(1, |value| fleet_side(value.is_alliance));
                let expanded = window.expanded.contains(&entry.fleet());
                match *entry {
                    FleetWindowEntry::Fleet(fleet) => {
                        if selected == Some(*entry) {
                            paint_native(
                                &list_painter,
                                ctx,
                                cache,
                                DllSource::Strategy,
                                side_art(FLEET_SELECTED, side),
                                item,
                                scale,
                                0.0,
                                0.0,
                            );
                        }
                        paint_native(
                            &list_painter,
                            ctx,
                            cache,
                            DllSource::Strategy,
                            side_art(FLEET_FRAME, side),
                            item,
                            scale,
                            5.0,
                            5.0,
                        );
                        if movement.is_in_transit(fleet) {
                            paint_native(
                                &list_painter,
                                ctx,
                                cache,
                                DllSource::Strategy,
                                side_art(EN_ROUTE_ENTRY, side),
                                item,
                                scale,
                                5.0,
                                5.0,
                            );
                        }
                        paint_dotted(&list_painter, item, scale, (2.0, 6.0), (5.0, 6.0));
                        if expanded {
                            paint_dotted(&list_painter, item, scale, (2.0, 6.0), (2.0, 50.0));
                        }
                        if let Some(label) = fleet_label(world, fleet) {
                            list_painter.text(
                                logical_rect(item, scale, 4.0, 5.0, 0.0, 0.0).min,
                                egui::Align2::LEFT_TOP,
                                label,
                                font.clone(),
                                egui::Color32::WHITE,
                            );
                        }
                    }
                    FleetWindowEntry::Ship { fleet, index } => {
                        let last = entries
                            .get(row + 1)
                            .is_none_or(|next| next.fleet() != fleet);
                        if last {
                            paint_dotted(&list_painter, item, scale, (2.0, 0.0), (2.0, 6.0));
                            paint_dotted(&list_painter, item, scale, (2.0, 6.0), (5.0, 6.0));
                        } else {
                            paint_dotted(&list_painter, item, scale, (2.0, 0.0), (2.0, 50.0));
                        }
                        let class = world
                            .fleets
                            .get(fleet)
                            .and_then(|value| value.capital_ships.get(index))
                            .and_then(|ship| world.capital_ship_classes.get(ship.class));
                        // port: a selected ship's look (`FUN_004c7e10`) is
                        // untraced, so its entry draws as unselected.
                        if let Some(class) = class {
                            if let Some(mini) = capital_ship_mini_id(class.dat_id) {
                                paint_native(
                                    &list_painter,
                                    ctx,
                                    cache,
                                    DllSource::Gokres,
                                    mini,
                                    item,
                                    scale,
                                    5.0,
                                    15.0,
                                );
                                if movement.is_in_transit(fleet) {
                                    let (source, mark) = en_route_mark(MiniObject::Craft, mini);
                                    paint_native(
                                        &list_painter,
                                        ctx,
                                        cache,
                                        source,
                                        mark,
                                        item,
                                        scale,
                                        5.0,
                                        15.0,
                                    );
                                }
                            }
                            if class.hyperdrive == 0 {
                                paint_native(
                                    &list_painter,
                                    ctx,
                                    cache,
                                    DllSource::Strategy,
                                    side_art(NO_HYPERDRIVE, side),
                                    item,
                                    scale,
                                    9.0,
                                    33.0,
                                );
                            }
                        }
                    }
                }
                let response = ui.interact(
                    item.intersect(list),
                    ui.id().with((window.system, "entry", row)),
                    egui::Sense::click(),
                );
                if exact_clicked(&response, item) {
                    result.select = Some(Some(*entry));
                    result.focus = true;
                }
                if response.double_clicked() {
                    if let FleetWindowEntry::Fleet(fleet) = entry {
                        result.toggle = Some(*fleet);
                    }
                }
                let object = match *entry {
                    FleetWindowEntry::Fleet(fleet) => ItemObject::Fleet(fleet),
                    FleetWindowEntry::Ship { fleet, index } => ItemObject::Ship {
                        fleet,
                        index,
                        roster: fleet_join::roster(world, fleet).unwrap_or_default(),
                    },
                };
                if let (true, Some(point)) = (
                    response.secondary_clicked(),
                    response.interact_pointer_pos(),
                ) {
                    result.object_menu = Some((object.menu_object(), point));
                }
                // A drag out of either list posts 0x29a (FUN_006083c0).
                if let Some(press) = ui.ctx().input(|input| {
                    input
                        .pointer
                        .button_pressed(egui::PointerButton::Primary)
                        .then(|| input.pointer.press_origin())
                        .flatten()
                }) {
                    if response.is_pointer_button_down_on() {
                        result.drag = Some(ItemDrag {
                            object,
                            press,
                            list,
                        });
                    }
                }
            }
            if let Some(edit) = &window.rename {
                result.rename =
                    draw_rename_edit(ui, window, edit, &entries, local, list, scale, &font);
            }

            if let Some(side) = selected_side {
                // The picture: one fleet's 10425 centered, keyed over the
                // en route mark 10426 drawn first at the same left edge; or
                // one ship's portrait at the panel's origin (`FUN_004a5c00`).
                if let Some(FleetWindowEntry::Fleet(fleet)) = selected {
                    let picture = side_art(FLEET_PICTURE, side);
                    let width = cache
                        .original_resource_size(DllSource::Strategy, picture)
                        .map_or(PICTURE_PANEL.2, |size| size[0] as f32);
                    if movement.is_in_transit(fleet) {
                        paint(
                            cache,
                            side_art(EN_ROUTE_PICTURE, side),
                            picture_x(width),
                            PICTURE_PANEL.1,
                        );
                    }
                    paint(cache, picture, picture_x(width), PICTURE_PANEL.1);
                    if let Some(label) = fleet_label(world, fleet) {
                        painter.text(
                            logical_rect(local, scale, 164.0, 29.0, 0.0, 0.0).min,
                            egui::Align2::CENTER_TOP,
                            label,
                            egui::FontId::proportional((10.0 * scale).max(7.0)),
                            label_color(viewer),
                        );
                    }
                }
                if let Some(FleetWindowEntry::Ship { fleet, index }) = selected {
                    let mini = world
                        .fleets
                        .get(fleet)
                        .and_then(|value| value.capital_ships.get(index))
                        .and_then(|ship| world.capital_ship_classes.get(ship.class))
                        .and_then(|class| capital_ship_mini_id(class.dat_id));
                    if let Some(mini) = mini {
                        let portrait = ship_portrait(mini);
                        let mut layers = vec![portrait];
                        if movement.is_in_transit(fleet) {
                            layers.push(en_route_portrait_mark(portrait));
                        }
                        // The panel is a 125 by 49 bitmap; the 50-row
                        // portrait loses its last row.
                        let panel = painter.with_clip_rect(logical_rect(
                            local,
                            scale,
                            PICTURE_PANEL.0,
                            PICTURE_PANEL.1,
                            PICTURE_PANEL.2,
                            PICTURE_PANEL.3,
                        ));
                        for id in layers {
                            paint_native(
                                &panel,
                                ctx,
                                cache,
                                DllSource::Gokres,
                                id,
                                local,
                                scale,
                                PICTURE_PANEL.0,
                                PICTURE_PANEL.1,
                            );
                        }
                    }
                }
                if let Some((aboard, capacity)) = tab_counts(world, transport, selected, window.tab)
                {
                    let count_font = egui::FontId::proportional((10.0 * scale).max(7.0));
                    painter.text(
                        logical_rect(
                            local,
                            scale,
                            PICTURE_PANEL.0 + 1.0,
                            PICTURE_PANEL.1 + 2.0,
                            0.0,
                            0.0,
                        )
                        .min,
                        egui::Align2::LEFT_TOP,
                        aboard.to_string(),
                        count_font.clone(),
                        egui::Color32::WHITE,
                    );
                    painter.text(
                        logical_rect(
                            local,
                            scale,
                            PICTURE_PANEL.0 + 123.0,
                            PICTURE_PANEL.1 + 2.0,
                            0.0,
                            0.0,
                        )
                        .min,
                        egui::Align2::RIGHT_TOP,
                        capacity.to_string(),
                        count_font,
                        egui::Color32::WHITE,
                    );
                }

                // The strip: the player's base for its own side, the other
                // base otherwise.
                let base = side_art(TAB_BASE, if side == viewer { viewer } else { other });
                for tab in FleetWindowTab::ALL {
                    let enabled = tab_enabled(world, transport, selected, tab);
                    let rect = tab_rect(local, scale, tab);
                    let id = tab_resource(base, tab, tab == window.tab, enabled);
                    // The control clips the 29-row art to 28 (`FUN_00602d30`).
                    let tab_painter = painter.with_clip_rect(rect);
                    paint_native(
                        &tab_painter,
                        ctx,
                        cache,
                        DllSource::Strategy,
                        id,
                        rect,
                        scale,
                        0.0,
                        0.0,
                    );
                    let response = ui.interact(
                        rect,
                        ui.id().with((window.system, tab)),
                        if enabled {
                            egui::Sense::click()
                        } else {
                            egui::Sense::hover()
                        },
                    );
                    if enabled && exact_clicked(&response, rect) {
                        result.tab = Some(tab);
                        result.focus = true;
                    }
                }

                let right = logical_rect(
                    local,
                    scale,
                    RIGHT_LIST.0,
                    RIGHT_LIST.1,
                    RIGHT_LIST.2,
                    RIGHT_LIST.3,
                );
                let right_painter = painter.with_clip_rect(right);
                // Everything aboard a travelling fleet is en route
                // (`FUN_004f8240`).
                let travelling = matches!(
                    selected,
                    Some(FleetWindowEntry::Fleet(fleet)) if movement.is_in_transit(fleet)
                );
                for (row, item) in right_items(world, transport, selected, window.tab)
                    .iter()
                    .enumerate()
                {
                    let top = RIGHT_LIST.1 + RIGHT_ITEM_HEIGHT * row as f32;
                    if top >= RIGHT_LIST.1 + RIGHT_LIST.3 {
                        break;
                    }
                    let cell =
                        logical_rect(local, scale, RIGHT_LIST.0, top, 125.0, RIGHT_ITEM_HEIGHT);
                    if window.selected_item == Some(row) {
                        paint_native(
                            &right_painter,
                            ctx,
                            cache,
                            DllSource::Strategy,
                            side_art(RIGHT_ITEM_SELECTED, side),
                            cell,
                            scale,
                            1.0,
                            1.0,
                        );
                    }
                    if let Some(mini) = item.mini {
                        paint_native(
                            &right_painter,
                            ctx,
                            cache,
                            DllSource::Gokres,
                            mini,
                            cell,
                            scale,
                            28.0,
                            4.0,
                        );
                        if travelling {
                            let (source, mark) = en_route_mark(item.kind, mini);
                            paint_native(
                                &right_painter,
                                ctx,
                                cache,
                                source,
                                mark,
                                cell,
                                scale,
                                28.0,
                                4.0,
                            );
                        }
                    }
                    if item.no_hyperdrive {
                        paint_native(
                            &right_painter,
                            ctx,
                            cache,
                            DllSource::Strategy,
                            side_art(NO_HYPERDRIVE, side),
                            cell,
                            scale,
                            9.0,
                            23.0,
                        );
                    }
                    right_painter.text(
                        logical_rect(cell, scale, 62.5, 37.0, 0.0, 0.0).min,
                        egui::Align2::CENTER_TOP,
                        &item.label,
                        font.clone(),
                        egui::Color32::WHITE,
                    );
                    let response = ui.interact(
                        cell.intersect(right),
                        ui.id().with((window.system, "item", row)),
                        egui::Sense::click(),
                    );
                    if exact_clicked(&response, cell) {
                        result.item = Some(row);
                        result.focus = true;
                    }
                    if response.secondary_clicked() {
                        if let (Some(object), Some(point)) =
                            (item.object, response.interact_pointer_pos())
                        {
                            result.object_menu = Some((object.menu_object(), point));
                        }
                    }
                    // A left press on a regiment or ship captures the mouse
                    // for a drag (FUN_006083c0).
                    if let (Some(object), Some(press)) = (
                        item.object,
                        ui.ctx().input(|input| {
                            input
                                .pointer
                                .button_pressed(egui::PointerButton::Primary)
                                .then(|| input.pointer.press_origin())
                                .flatten()
                        }),
                    ) {
                        if response.is_pointer_button_down_on() && rect_contains(cell, press) {
                            result.drag = Some(ItemDrag {
                                object,
                                press,
                                list: right,
                            });
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

/// The selected name's color: `0x20000ff` (red) for a side 1 viewer,
/// `0x200ff00` (green) otherwise (`FUN_004a4b10`).
fn label_color(viewer: u8) -> egui::Color32 {
    if viewer == 1 {
        egui::Color32::from_rgb(255, 0, 0)
    } else {
        egui::Color32::from_rgb(0, 255, 0)
    }
}

/// The rename edit over its entry: the entry's name line, white on the
/// list. hyp: the selection rectangle (`vtable+0x88`) is the entry's name
/// line; the field's font is font 10 (`vtable+0x18(10)`, unmapped).
#[expect(
    clippy::too_many_arguments,
    reason = "The edit needs the window's frame, list clip and font."
)]
fn draw_rename_edit(
    ui: &mut egui::Ui,
    window: &OpenFleetWindow,
    edit: &RenameEdit,
    entries: &[FleetWindowEntry],
    local: egui::Rect,
    list: egui::Rect,
    scale: f32,
    font: &egui::FontId,
) -> Option<RenameOutcome> {
    let Some(row) = entries.iter().position(|entry| *entry == edit.entry) else {
        // The entry left the list: the edit goes with it.
        return Some(RenameOutcome::Cancel);
    };
    let top = LEFT_LIST.1 + LEFT_ITEM_HEIGHT * row as f32;
    let rect = logical_rect(local, scale, LEFT_LIST.0 + 4.0, top + 3.0, 85.0, 15.0).intersect(list);
    let id = ui.id().with((window.system, "rename"));
    let mut text = edit.text.clone();
    ui.painter().rect_filled(rect, 0.0, egui::Color32::BLACK);
    let response = ui.put(
        rect,
        egui::TextEdit::singleline(&mut text)
            .id(id)
            .frame(false)
            .font(font.clone())
            .text_color(egui::Color32::WHITE)
            .desired_width(rect.width()),
    );
    if edit.fresh {
        response.request_focus();
        let mut state = egui::TextEdit::load_state(ui.ctx(), id).unwrap_or_default();
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(0),
                egui::text::CCursor::new(text.chars().count()),
            )));
        state.store(ui.ctx(), id);
        return Some(RenameOutcome::Typed(text));
    }
    let enter = ui.input(|input| input.key_pressed(egui::Key::Enter));
    let escape = ui.input(|input| input.key_pressed(egui::Key::Escape));
    if enter {
        // FUN_004ac950: an empty name keeps the field open.
        if text.is_empty() {
            response.request_focus();
            return Some(RenameOutcome::Typed(text));
        }
        return Some(RenameOutcome::Commit(text));
    }
    // hyp: Escape and a click elsewhere end the edit unissued
    // (`FUN_004aca40`); what sends them is untraced.
    if escape || response.lost_focus() {
        return Some(RenameOutcome::Cancel);
    }
    response.changed().then_some(RenameOutcome::Typed(text))
}

/// Blit a bitmap at its native size at a logical offset in `parent`
/// (`FUN_00602d30`).
#[expect(
    clippy::too_many_arguments,
    reason = "A blit names its painter, source, resource and position."
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "Bitmap sizes are small integers."
)]
pub(crate) fn paint_native(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    source: DllSource,
    resource_id: u32,
    parent: egui::Rect,
    scale: f32,
    x: f32,
    y: f32,
) {
    #[cfg(test)]
    tests::PAINTED.with(|painted| {
        painted
            .borrow_mut()
            .push((resource_id, logical_rect(parent, scale, x, y, 0.0, 0.0).min));
    });
    let Some([width, height]) = cache.original_resource_size(source, resource_id) else {
        return;
    };
    let Some(texture) = cache
        .get(ctx, source, resource_id)
        .map(egui::TextureHandle::id)
    else {
        return;
    };
    painter.image(
        texture,
        logical_rect(parent, scale, x, y, width as f32, height as f32),
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}

/// The white dotted tree line (`CreatePen(PS_DOT, 1, 0x2ffffff)`): one pixel
/// on, one off.
fn paint_dotted(
    painter: &egui::Painter,
    parent: egui::Rect,
    scale: f32,
    from: (f32, f32),
    to: (f32, f32),
) {
    let steps = (to.0 - from.0).abs().max((to.1 - from.1).abs()) as usize;
    for step in (0..steps).step_by(2) {
        let t = step as f32 / steps.max(1) as f32;
        let x = from.0 + (to.0 - from.0) * t;
        let y = from.1 + (to.1 - from.1) * t;
        painter.rect_filled(
            logical_rect(parent, scale, x, y, 1.0, 1.0),
            0.0,
            egui::Color32::WHITE,
        );
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
    use rebellion_core::dat::SectorGroup;
    use rebellion_core::ids::{DatId, TroopKey};
    use rebellion_core::world::{CapitalShipClass, Fleet, Sector, ShipInstance, System, TroopUnit};

    fn layout() -> CockpitLayout {
        let viewport = CockpitViewport {
            x: 0.0,
            y: 0.0,
            width: 640.0,
            height: 480.0,
        };
        CockpitLayout {
            canvas: viewport,
            galaxy: viewport,
            scale: 1.0,
        }
    }

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
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control,
        });
        (world, system)
    }

    fn add_fleet(
        world: &mut GameWorld,
        system: SystemKey,
        is_alliance: bool,
        troops: u32,
    ) -> FleetKey {
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            name: "Transport".into(),
            is_alliance,
            troop_capacity: troops,
            hull: 100,
            hyperdrive: 1,
            ..CapitalShipClass::default()
        });
        let fleet = world.insert_fleet(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, 100, true)],
            fighters: Vec::new(),
            characters: Vec::new(),
            is_alliance,
            has_death_star: false,
        });
        world.systems[system].fleets.push(fleet);
        fleet
    }

    fn add_troop(world: &mut GameWorld, system: SystemKey) -> TroopKey {
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(troop);
        troop
    }

    /// Seen by the Alliance: the system is visible.
    fn fog(system: SystemKey) -> FogState {
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(system);
        fog
    }

    fn window(system: SystemKey, selected: Option<FleetWindowEntry>) -> OpenFleetWindow {
        OpenFleetWindow {
            system,
            logical_position: (0, 0),
            selected,
            selected_item: None,
            expanded: Vec::new(),
            tab: FleetWindowTab::CapitalShips,
            rename: None,
        }
    }

    #[test]
    fn the_fleet_icon_shows_the_players_side_first_and_hides_without_fleets() {
        // FUN_0045ccc0 picks the player's side, then the other; FUN_0045d140
        // hides the overlay at a zero count; FUN_0045ca80 kind 0x10.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Empire));
        let fog = fog(system);
        assert_eq!(
            fleet_icon(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                Faction::Alliance,
                system
            ),
            None
        );

        add_fleet(&mut world, system, false, 0);
        assert_eq!(
            fleet_icon(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                Faction::Alliance,
                system
            ),
            Some((10783, 10784))
        );
        add_fleet(&mut world, system, true, 0);
        assert_eq!(
            fleet_icon(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                Faction::Alliance,
                system
            ),
            Some((10775, 10776))
        );
        assert_eq!(
            fleet_icon(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                Faction::Empire,
                system
            ),
            Some((10783, 10784))
        );
    }

    #[test]
    fn an_unseen_enemy_fleet_shows_no_icon() {
        // port: the count is the fleets the player can see.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Empire));
        add_fleet(&mut world, system, false, 0);
        let unseen = FogState::new(Faction::Alliance);

        assert_eq!(
            fleet_icon(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &unseen,
                Faction::Alliance,
                system
            ),
            None
        );
    }

    #[test]
    fn the_rail_icon_falls_back_to_the_systems_side() {
        // FUN_004a76e0: 0x2d10, 0x2d11, else 0x2d12.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Empire));
        let fog = fog(system);
        assert_eq!(
            rail_icon(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                Faction::Alliance,
                system
            ),
            11537
        );
        world.systems[system].control = ControlKind::Contested;
        assert_eq!(
            rail_icon(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                Faction::Alliance,
                system
            ),
            11538
        );
        add_fleet(&mut world, system, true, 0);
        assert_eq!(
            rail_icon(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                Faction::Alliance,
                system
            ),
            11536
        );
    }

    #[test]
    fn the_title_strip_follows_the_players_fleets_then_the_system() {
        // FUN_004a3340: with fleets, the player's side when it has one there,
        // else the other; without, the system's side bits. Side 1 is
        // 10299/10200, side 2 10201/10302, other 10303/10304.
        let (mut world, system) = world(ControlKind::Uncontrolled);
        let player = Faction::Alliance;
        assert_eq!(title_side(&world, &[], player, system), 0);
        let enemy = add_fleet(&mut world, system, false, 0);
        assert_eq!(title_side(&world, &[enemy], player, system), 2);
        let own = add_fleet(&mut world, system, true, 0);
        assert_eq!(title_side(&world, &[enemy, own], player, system), 1);
        assert_eq!(title_side(&world, &[own], Faction::Empire, system), 1);

        assert_eq!(
            [title_resource(1, true), title_resource(1, false)],
            [10299, 10200]
        );
        assert_eq!(
            [title_resource(2, true), title_resource(2, false)],
            [10201, 10302]
        );
        assert_eq!(
            [title_resource(0, true), title_resource(3, false)],
            [10303, 10304]
        );
    }

    #[test]
    fn tab_art_is_normal_disabled_or_selected_from_the_strips_base() {
        // FUN_004a4b10: b + k, b + 3 + k, b + 7 + k; the Empire's base is 50
        // higher.
        assert_eq!(
            tab_resource(10409, FleetWindowTab::CapitalShips, false, true),
            10409
        );
        assert_eq!(
            tab_resource(10409, FleetWindowTab::CapitalShips, true, true),
            10416
        );
        assert_eq!(
            tab_resource(10409, FleetWindowTab::Troops, false, false),
            10414
        );
        assert_eq!(
            tab_resource(10459, FleetWindowTab::Personnel, true, true),
            10469
        );
        assert_eq!(side_art(TAB_BASE, 2), 10459);
    }

    #[test]
    fn the_troops_tab_lights_only_with_regiments_aboard_and_counts_them() {
        // FUN_004a5c00: flag 2 enables the tab; the panel prints the
        // regiments aboard and the capacity (+0x240, +0x270).
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 2);
        let troop = add_troop(&mut world, system);
        let mut transport = TroopTransportState::default();
        let selected = Some(FleetWindowEntry::Fleet(fleet));
        assert!(!tab_enabled(
            &world,
            &transport,
            selected,
            FleetWindowTab::Troops
        ));
        assert!(tab_enabled(
            &world,
            &transport,
            selected,
            FleetWindowTab::CapitalShips
        ));

        transport.load(&mut world, fleet, &[troop]).unwrap();

        assert!(tab_enabled(
            &world,
            &transport,
            selected,
            FleetWindowTab::Troops
        ));
        assert_eq!(
            tab_counts(&world, &transport, selected, FleetWindowTab::Troops),
            Some((1, 2))
        );
        assert_eq!(
            right_items(&world, &transport, selected, FleetWindowTab::Troops),
            [RightItem {
                mini: Some(17_472),
                kind: MiniObject::Regiment(DatId::new(0x1000_0001)),
                label: "Alliance Fleet Regiment".into(),
                no_hyperdrive: false,
                object: Some(ItemObject::Regiment(troop)),
            }]
        );
        // A ship's own contents are not modelled, so its tabs stay dark.
        let ship = Some(FleetWindowEntry::Ship { fleet, index: 0 });
        assert!(!tab_enabled(
            &world,
            &transport,
            ship,
            FleetWindowTab::CapitalShips
        ));
    }

    #[test]
    fn a_drop_on_a_fleet_entry_targets_it_and_between_entries_the_system() {
        // FUN_004a3130 step 2.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let first = add_fleet(&mut world, system, true, 1);
        let second = add_fleet(&mut world, system, false, 1);
        let open = window(system, None);
        let entries = left_entries(
            &world,
            &rebellion_core::movement::MovementState::default(),
            &fog(system),
            Faction::Alliance,
            &open,
        );
        assert_eq!(
            entries,
            [
                FleetWindowEntry::Fleet(first),
                FleetWindowEntry::Fleet(second)
            ]
        );
        let target = |point| drop_target(&world, Faction::Alliance, &open, &entries, point);

        assert_eq!(
            target((10.0, 30.0)),
            Some(ReleaseTarget::Fleet {
                fleet: first,
                system
            })
        );
        assert_eq!(
            target((94.0, 128.0)),
            Some(ReleaseTarget::Fleet {
                fleet: second,
                system
            })
        );
        assert_eq!(target((10.0, 130.0)), Some(ReleaseTarget::System(system)));
    }

    #[test]
    fn a_drop_outside_the_lists_takes_the_selected_own_fleet_or_nothing() {
        // FUN_004a3130 step 1: the selected own-side fleet; none while own
        // fleets are listed but none is selected.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let own = add_fleet(&mut world, system, true, 1);
        let enemy = add_fleet(&mut world, system, false, 1);
        let fog = fog(system);
        let outside = (150.0, 20.0);
        let target = |selected| {
            let open = window(system, selected);
            let entries = left_entries(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                Faction::Alliance,
                &open,
            );
            drop_target(&world, Faction::Alliance, &open, &entries, outside)
        };

        assert_eq!(target(None), None);
        assert_eq!(target(Some(FleetWindowEntry::Fleet(enemy))), None);
        assert_eq!(
            target(Some(FleetWindowEntry::Fleet(own))),
            Some(ReleaseTarget::Fleet { fleet: own, system })
        );
    }

    #[test]
    fn a_drop_with_no_own_fleet_listed_targets_the_system() {
        // FUN_004a3130 step 1: the subject when no own-side entry exists.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Empire));
        let enemy = add_fleet(&mut world, system, false, 1);
        let open = window(system, Some(FleetWindowEntry::Fleet(enemy)));
        let entries = left_entries(
            &world,
            &rebellion_core::movement::MovementState::default(),
            &fog(system),
            Faction::Alliance,
            &open,
        );

        assert_eq!(
            drop_target(&world, Faction::Alliance, &open, &entries, (150.0, 20.0)),
            Some(ReleaseTarget::System(system))
        );
    }

    #[test]
    fn a_drop_on_a_listed_ship_targets_its_fleet_only_on_the_ships_tab() {
        // FUN_004a3130 step 3. port: a ship stands for its fleet.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let own = add_fleet(&mut world, system, true, 1);
        let mut open = window(system, Some(FleetWindowEntry::Fleet(own)));
        let fog = fog(system);
        let ship_row = (150.0, 140.0);
        let empty_row = (150.0, 190.0);
        let entries = left_entries(
            &world,
            &rebellion_core::movement::MovementState::default(),
            &fog,
            Faction::Alliance,
            &open,
        );
        let fleet = Some(ReleaseTarget::Fleet { fleet: own, system });

        assert_eq!(
            drop_target(&world, Faction::Alliance, &open, &entries, ship_row),
            fleet
        );
        // Below the only ship, the default: the selected own fleet.
        assert_eq!(
            drop_target(&world, Faction::Alliance, &open, &entries, empty_row),
            fleet
        );
        open.selected = None;
        assert_eq!(
            drop_target(&world, Faction::Alliance, &open, &entries, ship_row),
            None
        );
        open.tab = FleetWindowTab::Troops;
        open.selected = Some(FleetWindowEntry::Fleet(own));
        assert_eq!(
            drop_target(&world, Faction::Alliance, &open, &entries, ship_row),
            fleet
        );
    }

    #[test]
    fn expanding_a_fleet_lists_its_ships_under_it() {
        // FUN_004a3d40: one entry per living capital ship while expanded.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 1);
        let mut open = window(system, None);
        open.expanded.push(fleet);

        assert_eq!(
            left_entries(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog(system),
                Faction::Alliance,
                &open
            ),
            [
                FleetWindowEntry::Fleet(fleet),
                FleetWindowEntry::Ship { fleet, index: 0 }
            ]
        );
    }

    #[test]
    fn a_system_has_one_fleet_window_placed_inside_the_galaxy_view() {
        // FUN_0045aac0: an open window comes to the front; a new one is
        // clamped into the view's rect.
        let (world, system) = world(ControlKind::Uncontrolled);
        let mut state = FleetWindowState::default();
        assert!(state.open(
            &world,
            system,
            (600, 400),
            CockpitFaction::Alliance,
            layout()
        ));
        assert!(state.open(&world, system, (10, 10), CockpitFaction::Alliance, layout()));

        assert_eq!(state.window_count(), 1);
        assert_eq!(state.windows[0].logical_position, (405, 176));
    }

    #[test]
    fn selecting_a_ship_from_outside_expands_its_fleet() {
        // FUN_00429440 selects the Fleet Finder's object through slot +0x6c;
        // a ship's entry lists only while its fleet is expanded.
        let (mut world, system) = world(ControlKind::Uncontrolled);
        let fleet = add_fleet(&mut world, system, true, 0);
        let mut state = FleetWindowState::default();
        let ship = FleetWindowEntry::Ship { fleet, index: 0 };
        assert!(!state.select(system, ship));
        assert!(state.open(&world, system, (0, 0), CockpitFaction::Alliance, layout()));
        state.windows[0].selected_item = Some(0);

        assert!(state.select(system, ship));

        let window = &state.windows[0];
        assert_eq!(window.selected, Some(ship));
        assert_eq!(window.selected_item, None);
        assert!(left_entries(
            &world,
            &rebellion_core::movement::MovementState::default(),
            &fog(system),
            Faction::Alliance,
            window
        )
        .contains(&ship));
        assert!(state.select(system, ship));
        assert_eq!(state.windows[0].expanded, [fleet]);
    }

    /// The canvas 10 by 20 pixels in and twice the original size, so offsets
    /// and scale both show.
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

    /// A logical point in the window opened at `ORIGIN` on [`scaled`].
    fn at(x: f32, y: f32) -> egui::Pos2 {
        egui::pos2(
            10.0 + (f32::from(ORIGIN.0) + x) * 2.0,
            20.0 + (f32::from(ORIGIN.1) + y) * 2.0,
        )
    }

    const ORIGIN: (i16, i16) = (20, 30);

    #[derive(Debug, Clone, PartialEq)]
    struct Text {
        text: String,
        /// The top-left corner and size of the laid-out text.
        pos: egui::Pos2,
        width: f32,
        size: f32,
        color: egui::Color32,
    }

    impl Text {
        /// Whether `painter.text` placed this at `anchor` with `align`.
        fn anchored(&self, anchor: egui::Pos2, align: egui::Align) -> bool {
            let x = match align {
                egui::Align::Min => self.pos.x,
                egui::Align::Center => self.pos.x + self.width / 2.0,
                egui::Align::Max => self.pos.x + self.width,
            };
            (x - anchor.x).abs() < 0.01 && self.pos.y == anchor.y
        }
    }

    thread_local! {
        /// Each bitmap [`paint_native`] or the shared button art
        /// (`mission_dialog::paint`) was asked for and where, since the
        /// test cache holds no textures.
        pub(crate) static PAINTED: std::cell::RefCell<Vec<(u32, egui::Pos2)>> =
            const { std::cell::RefCell::new(Vec::new()) };
    }

    #[derive(Default)]
    struct Run {
        actions: Vec<FleetWindowAction>,
        texts: Vec<Text>,
        dots: Vec<egui::Pos2>,
        painted: Vec<(u32, egui::Pos2)>,
    }

    /// Draw the windows once per frame of events, on [`scaled`] for the
    /// Alliance; the last frame's text and dots are kept.
    fn run(
        world: &GameWorld,
        transport: &TroopTransportState,
        state: &mut FleetWindowState,
        faction: CockpitFaction,
        frames: Vec<Vec<egui::Event>>,
    ) -> Run {
        run_moving(
            world,
            &MovementState::default(),
            transport,
            state,
            faction,
            frames,
        )
    }

    /// [`run`] with fleets under `movement`'s orders.
    fn run_moving(
        world: &GameWorld,
        movement: &MovementState,
        transport: &TroopTransportState,
        state: &mut FleetWindowState,
        faction: CockpitFaction,
        frames: Vec<Vec<egui::Event>>,
    ) -> Run {
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let fog =
            world
                .systems
                .keys()
                .fold(FogState::new(cockpit_faction(faction)), |mut fog, key| {
                    fog.reveal(key);
                    fog
                });
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
                result.actions.extend(draw_fleet_windows(
                    ctx,
                    world,
                    movement,
                    &fog,
                    transport,
                    state,
                    faction,
                    scaled(),
                    &mut cache,
                ));
            });
            result.painted = PAINTED.with(|painted| painted.take());
            result.texts.clear();
            result.dots.clear();
            for clipped in output.shapes {
                match clipped.shape {
                    egui::Shape::Text(text) => {
                        let format = &text.galley.job.sections[0].format;
                        result.texts.push(Text {
                            text: text.galley.text().to_owned(),
                            pos: text.pos,
                            width: text.galley.size().x,
                            size: format.font_id.size,
                            color: format.color,
                        });
                    }
                    // White, faded in with its area (premultiplied).
                    egui::Shape::Rect(rect)
                        if rect.fill.a() > 0
                            && [rect.fill.r(), rect.fill.g(), rect.fill.b()]
                                == [rect.fill.a(); 3]
                            && rect.rect.size() == egui::vec2(2.0, 2.0) =>
                    {
                        result.dots.push(rect.rect.min);
                    }
                    _ => {}
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

    fn double_click(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
        let mut frames = hover(point);
        frames.extend([
            vec![press(point, true)],
            vec![press(point, false)],
            vec![press(point, true)],
            vec![press(point, false)],
            vec![],
        ]);
        frames
    }

    fn opened(world: &GameWorld, system: SystemKey) -> FleetWindowState {
        let mut state = FleetWindowState::default();
        assert!(state.open(world, system, ORIGIN, CockpitFaction::Alliance, scaled()));
        state
    }

    /// Open `system`'s window with `fleet` selected on `tab`, press at
    /// window pixel `from`, move to `to` and release there. Returns the
    /// actions, the release point, and whether a drag was held before the
    /// release.
    fn drag_from_tab(
        world: &GameWorld,
        transport: &TroopTransportState,
        system: SystemKey,
        fleet: FleetKey,
        tab: FleetWindowTab,
        from: (f32, f32),
        to: (f32, f32),
    ) -> (Vec<FleetWindowAction>, egui::Pos2, bool) {
        let mut state = opened(world, system);
        if let Some(window) = state.window_mut(system) {
            window.selected = Some(FleetWindowEntry::Fleet(fleet));
            window.tab = tab;
        }
        let (start, end) = (at(from.0, from.1), at(to.0, to.1));
        let mut frames = hover(start);
        frames.extend([
            vec![press(start, true)],
            vec![egui::Event::PointerMoved(end)],
        ]);
        let held = run(
            world,
            transport,
            &mut state,
            CockpitFaction::Alliance,
            frames,
        );
        let dragging = state.is_dragging();
        let released = run(
            world,
            transport,
            &mut state,
            CockpitFaction::Alliance,
            vec![vec![egui::Event::PointerMoved(end), press(end, false)]],
        );
        let mut actions = held.actions;
        actions.extend(released.actions);
        (actions, end, dragging)
    }

    fn regiment_drops(actions: &[FleetWindowAction]) -> Vec<(TroopKey, egui::Pos2)> {
        actions
            .iter()
            .filter_map(|action| match *action {
                FleetWindowAction::DragRegiment { troop, point } => Some((troop, point)),
                _ => None,
            })
            .collect()
    }

    fn fleet_with_regiment() -> (
        GameWorld,
        TroopTransportState,
        SystemKey,
        FleetKey,
        TroopKey,
    ) {
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 2);
        let troop = add_troop(&mut world, system);
        let mut transport = TroopTransportState::default();
        transport.load(&mut world, fleet, &[troop]).unwrap();
        (world, transport, system, fleet, troop)
    }

    /// Inside the first right-list item.
    const FIRST_RIGHT_ITEM: (f32, f32) = (160.0, 150.0);

    #[test]
    fn a_regiment_dragged_out_of_the_troops_tab_drops_where_the_button_comes_up() {
        // FUN_006083c0 captures the mouse on the press and posts 0x29a with
        // the release point; FUN_00422ce0 moves a type 4 selection with
        // 0x201 against the window under it.
        let (world, transport, system, fleet, troop) = fleet_with_regiment();

        let (actions, release, held) = drag_from_tab(
            &world,
            &transport,
            system,
            fleet,
            FleetWindowTab::Troops,
            FIRST_RIGHT_ITEM,
            (40.0, 200.0),
        );

        assert!(held);
        assert_eq!(regiment_drops(&actions), [(troop, release)]);
    }

    #[test]
    fn a_regiment_released_inside_its_list_or_near_the_press_drops_nothing() {
        // FUN_006083c0: the squared distance must exceed 0x18 and the
        // release must leave the list's client rect.
        let (world, transport, system, fleet, _) = fleet_with_regiment();
        // Down the list; and 4 pixels up out of it from its top edge.
        for (from, to) in [
            (FIRST_RIGHT_ITEM, (160.0, 200.0)),
            ((160.0, 128.0), (160.0, 124.0)),
        ] {
            let (actions, _, held) = drag_from_tab(
                &world,
                &transport,
                system,
                fleet,
                FleetWindowTab::Troops,
                from,
                to,
            );
            assert!(held);
            assert!(regiment_drops(&actions).is_empty());
        }
    }

    #[test]
    fn a_press_on_a_regiments_hidden_part_below_the_list_starts_no_drag() {
        // The fourth row starts 150 below the list's top and the list is 164
        // tall, so a press at row offset 23 lies in its cell but under the
        // list's bottom edge.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 4);
        let troops: Vec<_> = (0..4).map(|_| add_troop(&mut world, system)).collect();
        let mut transport = TroopTransportState::default();
        transport.load(&mut world, fleet, &troops).unwrap();

        let (actions, _, held) = drag_from_tab(
            &world,
            &transport,
            system,
            fleet,
            FleetWindowTab::Troops,
            (160.0, 127.0 + 150.0 + 23.0),
            (40.0, 200.0),
        );

        assert!(!held);
        assert!(regiment_drops(&actions).is_empty());
    }

    #[test]
    fn a_right_list_items_rect_steps_down_one_row_height() {
        // Our own layout: 50-row items 125 wide from (101, 127).
        let (world, _, system, _, _) = fleet_with_regiment();
        let state = opened(&world, system);

        assert_eq!(
            state.item_screen_rect(scaled(), system, 1),
            Some(egui::Rect::from_min_max(at(101.0, 177.0), at(226.0, 227.0)))
        );
        assert_eq!(
            state.item_screen_rect(scaled(), system, 2),
            Some(egui::Rect::from_min_max(at(101.0, 227.0), at(226.0, 277.0)))
        );
        let mut other = world.clone();
        let missing = other.systems.insert(world.systems[system].clone());
        assert_eq!(state.item_screen_rect(scaled(), missing, 0), None);
    }

    #[test]
    fn a_capital_ship_dragged_out_of_its_tab_drops_that_ship_where_the_button_comes_up() {
        // FUN_00422ce0 moves a type 4 selection with 0x201; manual p. 120:
        // "To move ships [...] from one fleet to another, simply drag the
        // item to their new destinations."
        let (world, transport, system, fleet, _) = fleet_with_regiment();

        let (actions, release, held) = drag_from_tab(
            &world,
            &transport,
            system,
            fleet,
            FleetWindowTab::CapitalShips,
            FIRST_RIGHT_ITEM,
            (40.0, 200.0),
        );

        assert!(held);
        assert!(regiment_drops(&actions).is_empty());
        assert_eq!(
            actions
                .iter()
                .filter(|action| matches!(action, FleetWindowAction::DragShip { .. }))
                .collect::<Vec<_>>(),
            [&FleetWindowAction::DragShip {
                fleet,
                index: 0,
                roster: fleet_join::roster(&world, fleet).unwrap(),
                point: release,
            }]
        );
    }

    #[test]
    fn a_fleet_dragged_out_of_the_left_list_drops_that_fleet_where_the_button_comes_up() {
        // fleet-window.md: a drag out of either list posts 0x29a, and
        // FUN_00422ce0 moves the type 4 selection with 0x201.
        let (world, transport, system, fleet, _) = fleet_with_regiment();
        let (actions, release, held) = drag_from_tab(
            &world,
            &transport,
            system,
            fleet,
            FleetWindowTab::CapitalShips,
            (40.0, 40.0),
            (300.0, 200.0),
        );
        assert!(held);
        assert_eq!(
            actions
                .iter()
                .filter(|action| matches!(action, FleetWindowAction::DragFleet { .. }))
                .collect::<Vec<_>>(),
            [&FleetWindowAction::DragFleet {
                fleet,
                point: release,
            }]
        );

        // A release inside the list it left drops nothing.
        let (inside, _, _) = drag_from_tab(
            &world,
            &transport,
            system,
            fleet,
            FleetWindowTab::CapitalShips,
            (40.0, 40.0),
            (40.0, 200.0),
        );
        assert!(!inside
            .iter()
            .any(|action| matches!(action, FleetWindowAction::DragFleet { .. })));
    }

    #[test]
    fn a_capital_ship_item_stands_for_its_index_among_every_ship_of_the_fleet() {
        // Our own list: a destroyed ship is not listed but keeps its index.
        let (mut world, transport, _, fleet, _) = fleet_with_regiment();
        let class = world.fleets[fleet].capital_ships[0].class;
        world.fleets[fleet]
            .capital_ships
            .insert(0, ShipInstance::new(class, 100, true));
        world.fleets[fleet].capital_ships[0].alive = false;

        let items = right_items(
            &world,
            &transport,
            Some(FleetWindowEntry::Fleet(fleet)),
            FleetWindowTab::CapitalShips,
        );

        assert_eq!(
            items.iter().map(|item| item.object).collect::<Vec<_>>(),
            [Some(ItemObject::Ship {
                fleet,
                index: 1,
                roster: fleet_join::roster(&world, fleet).unwrap(),
            })]
        );
    }

    fn right_click(
        world: &GameWorld,
        transport: &TroopTransportState,
        state: &mut FleetWindowState,
        point: egui::Pos2,
    ) -> Vec<FleetWindowAction> {
        let button = |pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Secondary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let mut frames = hover(point);
        frames.extend([vec![button(true)], vec![button(false)], vec![]]);
        run(world, transport, state, CockpitFaction::Alliance, frames).actions
    }

    fn menus(actions: &[FleetWindowAction]) -> Vec<(MenuObject, (i16, i16))> {
        actions
            .iter()
            .filter_map(|action| match *action {
                FleetWindowAction::OpenObjectMenu { selection, point } => Some((selection, point)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_right_click_on_a_ship_or_regiment_item_opens_its_menu_where_it_was_clicked() {
        // Manual p. 120: "Right-click on a ship to bring up its menu";
        // FUN_004ac5c0 opens the menu at the canvas point.
        let (world, transport, system, fleet, troop) = fleet_with_regiment();
        let mut state = opened(&world, system);
        if let Some(window) = state.window_mut(system) {
            window.selected = Some(FleetWindowEntry::Fleet(fleet));
            window.tab = FleetWindowTab::CapitalShips;
        }

        let ship = right_click(&world, &transport, &mut state, at(160.0, 150.0));
        assert_eq!(
            menus(&ship),
            [(
                MenuObject::Ship {
                    fleet,
                    index: 0,
                    roster: fleet_join::roster(&world, fleet).unwrap(),
                },
                (180, 180)
            )]
        );

        if let Some(window) = state.window_mut(system) {
            window.tab = FleetWindowTab::Troops;
        }
        let regiment = right_click(&world, &transport, &mut state, at(160.0, 150.0));
        assert_eq!(menus(&regiment), [(MenuObject::Troop(troop), (180, 180))]);
    }

    #[test]
    fn a_right_click_on_a_fleet_entry_opens_the_fleets_menu_and_on_a_fighter_none() {
        // FUN_004ff8e0's fleet menu; port: a squadron has no menu yet.
        let (mut world, transport, system, fleet, _) = fleet_with_regiment();
        let fighter = world.fighter_classes.insert(Default::default());
        world.fleets[fleet]
            .fighters
            .push(rebellion_core::world::FighterEntry {
                class: fighter,
                count: 1,
            });
        let mut state = opened(&world, system);
        let entry = state
            .entry_screen_rect(scaled(), system, 0)
            .unwrap()
            .center();

        let actions = right_click(&world, &transport, &mut state, entry);
        assert_eq!(
            menus(&actions)
                .into_iter()
                .map(|(selection, _)| selection)
                .collect::<Vec<_>>(),
            [MenuObject::Fleet(fleet)]
        );

        if let Some(window) = state.window_mut(system) {
            window.selected = Some(FleetWindowEntry::Fleet(fleet));
            window.tab = FleetWindowTab::Fighters;
        }
        let actions = right_click(&world, &transport, &mut state, at(160.0, 150.0));
        assert!(menus(&actions).is_empty());
    }

    #[test]
    fn the_windows_rects_follow_the_canvas_offset_and_scale() {
        // Our own layout: the list's 50-row items (FUN_004a3d40) and the tab
        // strip at (99 + x, 96), 31 by 28.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 1);
        let mut other_system = world.systems[system].clone();
        other_system.fleets.clear();
        let other = world.systems.insert(other_system);
        let missing = world.systems.insert(world.systems[other].clone());
        world.systems.remove(missing);
        let mut state = opened(&world, system);
        assert!(state.open(
            &world,
            other,
            (300, 100),
            CockpitFaction::Alliance,
            scaled()
        ));

        assert_eq!(state.window_count(), 2);
        assert!(state.is_open(system) && state.is_open(other) && !state.is_open(missing));
        assert_eq!(
            state.entry_screen_rect(scaled(), system, 2),
            Some(egui::Rect::from_min_size(
                at(4.0, 129.0),
                egui::vec2(182.0, 100.0)
            ))
        );
        assert_eq!(
            state.tab_screen_rect(scaled(), system, FleetWindowTab::Troops),
            Some(egui::Rect::from_min_size(
                at(164.0, 96.0),
                egui::vec2(62.0, 56.0)
            ))
        );
        assert_eq!(
            state.screen_rect(scaled(), system),
            Some(egui::Rect::from_min_size(
                at(0.0, 0.0),
                egui::vec2(FLEET_WINDOW_WIDTH * 2.0, FLEET_WINDOW_HEIGHT * 2.0)
            ))
        );
        assert_eq!(state.screen_rect(scaled(), missing), None);
        assert_eq!(state.entry_screen_rect(scaled(), missing, 0), None);
        assert_eq!(
            state.tab_screen_rect(scaled(), missing, FleetWindowTab::Troops),
            None
        );
        assert_eq!(state.selection(missing), None);
        assert_eq!(
            state.selection(system),
            Some((None, FleetWindowTab::CapitalShips))
        );
        let point = |p: egui::Pos2| (p.x, p.y);
        assert!(state.contains_screen_point(scaled(), point(at(234.0, 303.0))));
        assert!(!state.contains_screen_point(scaled(), point(at(235.5, 150.0))));
        assert!(!state.contains_screen_point(scaled(), point(at(-0.5, 150.0))));

        let fog = fog(system);
        let layer = egui::LayerId::new(egui::Order::Foreground, area_id(system));
        assert_eq!(
            state.release_target(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                scaled(),
                layer,
                at(60.0, 40.0)
            ),
            Some(Some(ReleaseTarget::Fleet { fleet, system }))
        );
        // Below the only entry: the system.
        assert_eq!(
            state.release_target(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                scaled(),
                layer,
                at(10.0, 90.0)
            ),
            Some(Some(ReleaseTarget::System(system)))
        );
        let elsewhere = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("elsewhere"));
        assert_eq!(
            state.release_target(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                scaled(),
                elsewhere,
                at(10.0, 40.0)
            ),
            None
        );

        state.clear();
        assert_eq!(state.window_count(), 0);
    }

    #[test]
    fn opening_a_window_for_the_other_side_closes_the_first_sides_windows() {
        // port: the port keeps one player's windows at a time.
        let (world, system) = world(ControlKind::Uncontrolled);
        let mut state = opened(&world, system);
        let other = world.systems.keys().next().unwrap();
        assert!(state.open(&world, other, ORIGIN, CockpitFaction::Empire, scaled()));
        assert_eq!(state.window_count(), 1);
        assert_eq!(state.faction, CockpitFaction::Empire);
    }

    #[test]
    fn a_click_selects_a_fleet_then_a_lit_tab_and_the_close_button_closes_the_window() {
        // FUN_004a4b10: the list selection fills the right pane; a disabled
        // tab takes no click; control 0x277c closes.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 2);
        let troop = add_troop(&mut world, system);
        let mut transport = TroopTransportState::default();
        transport.load(&mut world, fleet, &[troop]).unwrap();
        let mut state = opened(&world, system);

        let selected = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            click(at(40.0, 50.0)),
        );
        assert_eq!(
            state.selection(system),
            Some((
                Some(FleetWindowEntry::Fleet(fleet)),
                FleetWindowTab::CapitalShips
            ))
        );
        assert!(selected
            .actions
            .contains(&FleetWindowAction::SelectSystem(system)));

        run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            click(at(115.0, 110.0)),
        );
        assert_eq!(
            state.selection(system).unwrap().1,
            FleetWindowTab::CapitalShips
        );
        run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            click(at(180.0, 110.0)),
        );
        assert_eq!(state.selection(system).unwrap().1, FleetWindowTab::Troops);
        assert_eq!(
            state
                .report(
                    &world,
                    &rebellion_core::movement::MovementState::default(),
                    &fog(system),
                    &transport,
                    system
                )
                .unwrap()
                .items,
            ["Alliance Fleet Regiment"]
        );

        run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            click(at(225.0, 10.0)),
        );
        assert!(!state.is_open(system));
        assert_eq!(
            state.report(
                &world,
                &rebellion_core::movement::MovementState::default(),
                &fog(system),
                &transport,
                system
            ),
            None
        );
    }

    #[test]
    fn the_title_buttons_minimize_the_window_and_open_its_sector() {
        // FUN_004a4b10: 0x280d minimizes to the rail (FUN_004a76e0); 0xca
        // restores the sector window.
        let (world, system) = world(ControlKind::Uncontrolled);
        let transport = TroopTransportState::default();
        let mut state = opened(&world, system);
        let sector = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            click(at(10.0, 10.0)),
        );
        assert!(sector
            .actions
            .contains(&FleetWindowAction::OpenSector(system)));
        assert!(state.is_open(system));

        let minimized = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            click(at(211.0, 10.0)),
        );
        assert!(minimized.actions.contains(&FleetWindowAction::Minimize {
            system,
            logical_position: ORIGIN,
        }));
        assert!(!state.is_open(system));
    }

    #[test]
    fn a_click_on_a_back_windows_background_brings_it_forward() {
        // FUN_0045aac0's focus: the clicked child becomes the active one.
        let (mut world, system) = world(ControlKind::Uncontrolled);
        let copy = world.systems[system].clone();
        let other = world.systems.insert(copy);
        let transport = TroopTransportState::default();
        let mut state = opened(&world, system);
        assert!(state.open(
            &world,
            other,
            (400, 300),
            CockpitFaction::Alliance,
            scaled()
        ));
        assert_eq!(state.windows.last().unwrap().system, other);

        let focused = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            click(at(150.0, 200.0)),
        );
        assert_eq!(state.windows.last().unwrap().system, system);
        assert!(focused
            .actions
            .contains(&FleetWindowAction::SelectSystem(system)));
    }

    #[test]
    fn a_double_click_on_a_fleet_lists_its_ships_and_another_hides_them() {
        // FUN_004a3d40: a fleet's entry expands into its capital ships.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        add_fleet(&mut world, system, true, 1);
        let transport = TroopTransportState::default();
        let mut state = opened(&world, system);
        let entries = |state: &FleetWindowState| {
            state
                .report(
                    &world,
                    &rebellion_core::movement::MovementState::default(),
                    &fog(system),
                    &transport,
                    system,
                )
                .unwrap()
                .entries
        };

        run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            double_click(at(40.0, 50.0)),
        );
        assert_eq!(entries(&state), 2);
        run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            double_click(at(40.0, 50.0)),
        );
        assert_eq!(entries(&state), 1);
    }

    #[test]
    fn the_window_writes_its_names_and_counts_where_the_original_does() {
        // FUN_004a4b10: the title label 5 past the restore button, black; the
        // selected name centred at x 164, red (0x20000ff) for side 1 and green
        // otherwise; FUN_004a5c00's counts in the picture panel. Our own: the
        // list label's place and the right list's centred names.
        for (faction, color) in [
            (CockpitFaction::Alliance, egui::Color32::from_rgb(255, 0, 0)),
            (CockpitFaction::Empire, egui::Color32::from_rgb(0, 255, 0)),
        ] {
            let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
            let fleet = add_fleet(&mut world, system, true, 2);
            let troop = add_troop(&mut world, system);
            let mut transport = TroopTransportState::default();
            transport.load(&mut world, fleet, &[troop]).unwrap();
            let mut state = FleetWindowState::default();
            assert!(state.open(&world, system, ORIGIN, faction, scaled()));
            if let Some(window) = state.window_mut(system) {
                window.selected = Some(FleetWindowEntry::Fleet(fleet));
                window.tab = FleetWindowTab::Troops;
            }

            let texts = run(
                &world,
                &transport,
                &mut state,
                faction,
                hover(at(-50.0, 0.0)),
            )
            .texts;
            let white = egui::Color32::WHITE;
            let (left, centre, right) = (egui::Align::Min, egui::Align::Center, egui::Align::Max);
            for (label, anchor, align, size, color) in [
                ("Sluis Van", at(19.0, 3.0), left, 22.0, egui::Color32::BLACK),
                ("Fleet 1", at(8.0, 34.0), left, 18.0, white),
                ("Fleet 1", at(164.0, 29.0), centre, 20.0, color),
                ("1", at(101.0, 44.0), left, 20.0, white),
                ("2", at(223.0, 44.0), right, 20.0, white),
                (
                    "Alliance Fleet Regiment",
                    at(163.5, 164.0),
                    centre,
                    18.0,
                    white,
                ),
            ] {
                assert!(
                    texts.iter().any(|text| text.text == label
                        && text.anchored(anchor, align)
                        && text.size == size
                        && text.color == color),
                    "{faction:?}: {label} at {anchor:?} in {texts:?}"
                );
            }
        }
    }

    #[test]
    fn an_enemy_fleet_selected_shows_the_other_sides_art() {
        // FUN_004a4b10 / FUN_004a3340: the title strip, pane art, selection
        // frame, picture and tab strip of the other side (Alliance + 50) when
        // the viewer's side has no fleet listed; the no-hyperdrive mark 10430
        // and the selected item 10420 take the fleet's side too.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Empire));
        let enemy = add_fleet(&mut world, system, false, 0);
        let class = world.fleets[enemy].capital_ships[0].class;
        world.capital_ship_classes[class].hyperdrive = 0;
        let transport = TroopTransportState::default();
        let mut state = opened(&world, system);
        if let Some(window) = state.window_mut(system) {
            window.selected = Some(FleetWindowEntry::Fleet(enemy));
            window.selected_item = Some(0);
            window.expanded.push(enemy);
        }

        let painted = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            hover(at(-50.0, 0.0)),
        )
        .painted;
        for (id, x, y) in [
            (10201, 2.0, 2.0),
            (10457, 97.0, 29.0),
            (10451, 4.0, 29.0),
            (10450, 9.0, 34.0),
            (10475, 100.0, 42.0),
            (10466, 100.0, 96.0),
            (10463, 132.0, 96.0),
            (10480, 13.0, 112.0),
            (10470, 102.0, 128.0),
            (10480, 110.0, 150.0),
            (CLOSE_NORMAL, 218.0, 3.0),
        ] {
            assert!(
                painted.contains(&(id, at(x, y))),
                "{id} at ({x}, {y}) in {painted:?}"
            );
        }
        assert!(
            !painted.iter().any(|(id, _)| *id == 10302),
            "the window is focused"
        );
    }

    #[test]
    fn a_fleet_in_hyperspace_shows_its_overlays_where_it_is_bound() {
        // move-order.md: the fleet joins its destination at once; there
        // FUN_004a37c0 draws 10423 at the entry's (5, 5) on +0x50 bit 4 and
        // FUN_004a5c00 10426 over the one selected fleet's picture. Its ships
        // are en route with it (FUN_004f8240), so FUN_0042c3b0 draws each
        // mini's own mark (+0x1000) over the ship entry's mini at (5, 15)
        // (FUN_004a3d40) and the right item's at (28, 4) (FUN_004a6e70).
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 0);
        let class = world.fleets[fleet].capital_ships[0].class;
        // The MC80 Liberty cruiser's class.
        world.capital_ship_classes[class].dat_id = DatId::new(0x1400_0040);
        let mini = capital_ship_mini_id(world.capital_ship_classes[class].dat_id).unwrap();
        let transport = TroopTransportState::default();
        let mut state = opened(&world, system);
        if let Some(window) = state.window_mut(system) {
            window.selected = Some(FleetWindowEntry::Fleet(fleet));
            window.expanded.push(fleet);
        }
        let mut movement = MovementState::default();
        let paint = |state: &mut FleetWindowState, movement: &MovementState| {
            run_moving(
                &world,
                movement,
                &transport,
                state,
                CockpitFaction::Alliance,
                hover(at(-50.0, 0.0)),
            )
            .painted
        };

        let orbiting = paint(&mut state, &movement);
        assert!(orbiting.contains(&(10400, at(9.0, 34.0))));
        assert!(!orbiting
            .iter()
            .any(|(id, _)| *id == 10423 || *id == 10426 || *id == mini + 0x1000));

        // Bound here from elsewhere: out of the orbit index, under an order.
        assert!(movement.order(fleet, system, system, 5));
        let en_route = paint(&mut state, &movement);
        assert!(en_route.contains(&(10423, at(9.0, 34.0))), "{en_route:?}");
        assert!(en_route.contains(&(10426, at(100.0, 42.0))), "{en_route:?}");
        assert!(
            en_route.contains(&(mini + 0x1000, at(9.0, 94.0))),
            "{en_route:?}"
        );
        assert!(
            en_route.contains(&(mini + 0x1000, at(129.0, 131.0))),
            "{en_route:?}"
        );
        // FUN_004a5c00 blits 10426 into the panel first, then the picture
        // keyed over it.
        let order = |painted: &[(u32, egui::Pos2)], id| painted.iter().position(|(x, _)| *x == id);
        assert!(
            order(&en_route, 10426) < order(&en_route, 10425),
            "{en_route:?}"
        );

        // One ship selected: its portrait (FUN_0042c3b0(.., 1, 1), the mini
        // less 0x4000) at the panel's origin, and its own mark (+0x1000)
        // over it while en route.
        if let Some(window) = state.window_mut(system) {
            window.selected = Some(FleetWindowEntry::Ship { fleet, index: 0 });
        }
        let portrait = mini - 0x4000;
        let ship = paint(&mut state, &movement);
        let panel = at(100.0, 42.0);
        assert!(ship.contains(&(portrait, panel)), "{ship:?}");
        assert!(ship.contains(&(portrait + 0x1000, panel)), "{ship:?}");
        assert!(order(&ship, portrait) < order(&ship, portrait + 0x1000));
        let orbiting = paint(&mut state, &MovementState::default());
        assert!(orbiting.contains(&(portrait, panel)), "{orbiting:?}");
        assert!(!orbiting.iter().any(|(id, _)| *id == portrait + 0x1000));
    }

    #[test]
    fn each_kind_of_mini_takes_its_own_en_route_mark() {
        // FUN_0042c3b0: GOKRES (class & 0xfff) + 0x5000 by default, kept for
        // craft, regiments 0x10000002/0x10000008 and special forces
        // 0x3c000003/0x3c000005; 0x2cfb (11515) for other regiments and
        // 0x2ced (11501) for characters and other special forces.
        let mark = |object| en_route_mark(object, 17_473);
        let own = (DllSource::Gokres, 21_569);
        let regiment = |raw| MiniObject::Regiment(DatId::new(raw));
        let force = |raw| MiniObject::SpecialForce(DatId::new(raw));
        assert_eq!(mark(MiniObject::Craft), own);
        assert_eq!(mark(regiment(0x1000_0002)), own);
        assert_eq!(mark(regiment(0x1000_0008)), own);
        assert_eq!(mark(regiment(0x1000_0001)), (DllSource::Strategy, 11_515));
        assert_eq!(mark(MiniObject::Character), (DllSource::Strategy, 11_501));
        assert_eq!(mark(force(0x3c00_0003)), own);
        assert_eq!(mark(force(0x3c00_0005)), own);
        assert_eq!(mark(force(0x3c00_0001)), (DllSource::Strategy, 11_501));
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

    fn renames(actions: &[FleetWindowAction]) -> Vec<(FleetWindowEntry, String)> {
        actions
            .iter()
            .filter_map(|action| match action {
                FleetWindowAction::Rename { entry, name } => Some((*entry, name.clone())),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn rename_edits_the_name_in_place_and_enter_issues_it() {
        // FUN_004ac7a0 opens the field holding the name, all selected, so
        // typing replaces it; FUN_004ac950 issues it on Enter.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 0);
        let transport = TroopTransportState::default();
        let mut state = opened(&world, system);
        let entry = FleetWindowEntry::Fleet(fleet);
        assert!(state.begin_rename(&world, system, entry));
        assert!(state.renaming());
        assert_eq!(state.selection(system).unwrap().0, Some(entry));

        let mut frames = hover(at(-50.0, 0.0));
        frames.push(vec![egui::Event::Text("Rogue".into())]);
        frames.push(vec![key(egui::Key::Enter)]);
        frames.push(vec![]);
        let run = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            frames,
        );
        assert_eq!(renames(&run.actions), [(entry, "Rogue".to_owned())]);
        assert!(!state.renaming());
    }

    #[test]
    fn an_empty_rename_keeps_its_field_and_escape_drops_it_unissued() {
        // FUN_004ac950 issues nothing for an empty field and leaves it open.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 0);
        let transport = TroopTransportState::default();
        let mut state = opened(&world, system);
        assert!(state.begin_rename(&world, system, FleetWindowEntry::Fleet(fleet)));

        let mut frames = hover(at(-50.0, 0.0));
        frames.push(vec![key(egui::Key::Backspace)]);
        frames.push(vec![key(egui::Key::Enter)]);
        frames.push(vec![]);
        let run = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            frames,
        );
        assert!(renames(&run.actions).is_empty());
        assert!(state.renaming(), "the field stays open");

        let mut frames = hover(at(-50.0, 0.0));
        frames.push(vec![key(egui::Key::Escape)]);
        frames.push(vec![]);
        let run_escape = super::tests::run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            frames,
        );
        assert!(renames(&run_escape.actions).is_empty());
        assert!(!state.renaming());
    }

    #[test]
    fn a_button_draws_pressed_only_while_the_mouse_is_down_on_it() {
        // FUN_00602d30's button states: 0x277c's 10108 normal, 10109 pressed.
        let (world, system) = world(ControlKind::Uncontrolled);
        let transport = TroopTransportState::default();
        let close = at(225.0, 10.0);
        let mut state = opened(&world, system);
        let hovered = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            hover(close),
        )
        .painted;
        assert!(hovered.contains(&(CLOSE_NORMAL, at(218.0, 3.0))));
        assert!(!hovered.iter().any(|(id, _)| *id == CLOSE_PRESSED));

        let mut frames = hover(close);
        frames.push(vec![press(close, true)]);
        frames.push(vec![]);
        let held = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            frames,
        )
        .painted;
        assert!(held.contains(&(CLOSE_PRESSED, at(218.0, 3.0))));
    }

    #[test]
    fn the_list_draws_the_rows_that_fit_and_the_right_list_too() {
        // Our own: the 266-high list holds six 50-high rows, the 164-high right
        // list four.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleets: Vec<_> = (0..7)
            .map(|_| add_fleet(&mut world, system, true, 5))
            .collect();
        let troops: Vec<_> = (0..5).map(|_| add_troop(&mut world, system)).collect();
        let mut transport = TroopTransportState::default();
        transport.load(&mut world, fleets[0], &troops).unwrap();
        let mut state = opened(&world, system);
        if let Some(window) = state.window_mut(system) {
            window.selected = Some(FleetWindowEntry::Fleet(fleets[0]));
            window.tab = FleetWindowTab::Troops;
        }

        let texts = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            hover(at(-50.0, 0.0)),
        )
        .texts;
        let count = |prefix: &str, size: f32| {
            texts
                .iter()
                .filter(|text| text.text.starts_with(prefix) && text.size == size)
                .count()
        };
        assert_eq!(count("Fleet ", 18.0), 6);
        assert_eq!(count("Alliance Fleet Regiment", 18.0), 4);
    }

    #[test]
    fn the_tree_is_dotted_one_pixel_on_and_one_off() {
        // CreatePen(PS_DOT, ...): an expanded fleet's line runs down past its
        // ships; the last ship's stops at its branch.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 1);
        let class = world.fleets[fleet].capital_ships[0].class;
        world.fleets[fleet]
            .capital_ships
            .push(ShipInstance::new(class, 100, true));
        let transport = TroopTransportState::default();
        let mut state = opened(&world, system);
        state.window_mut(system).unwrap().expanded.push(fleet);

        let dots = run(
            &world,
            &transport,
            &mut state,
            CockpitFaction::Alliance,
            hover(at(-50.0, 0.0)),
        )
        .dots;
        let dot = |x: f32, y: f32| dots.contains(&at(x, y));
        // The fleet's branch and its line down (item 0 at y 29).
        assert!(dot(6.0, 35.0) && dot(8.0, 35.0) && !dot(7.0, 35.0));
        assert!(dot(6.0, 77.0) && !dot(6.0, 78.0));
        // The first ship's line through its row (item 1 at y 79).
        assert!(dot(6.0, 79.0) && dot(6.0, 127.0));
        // The last ship's line stops at its branch (item 2 at y 129).
        assert!(dot(6.0, 129.0) && dot(6.0, 133.0) && !dot(6.0, 137.0));
        assert!(dot(6.0, 135.0) && dot(8.0, 135.0));
    }

    #[test]
    fn the_other_tabs_light_with_squadrons_and_characters_aboard() {
        // FUN_004a5c00: flags 1 (squadrons) and 3 (characters).
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 1);
        let transport = TroopTransportState::default();
        let selected = Some(FleetWindowEntry::Fleet(fleet));
        let lit = |world: &GameWorld, tab| tab_enabled(world, &transport, selected, tab);
        assert!(!lit(&world, FleetWindowTab::Fighters));
        assert!(!lit(&world, FleetWindowTab::Personnel));

        let squadron = world.fighter_classes.insert(Default::default());
        world.fleets[fleet]
            .fighters
            .push(rebellion_core::world::FighterEntry {
                class: squadron,
                count: 0,
            });
        assert!(!lit(&world, FleetWindowTab::Fighters));
        world.fleets[fleet].fighters[0].count = 1;
        assert!(lit(&world, FleetWindowTab::Fighters));
        let character = world.characters.insert(Default::default());
        world.fleets[fleet].characters.push(character);
        assert!(lit(&world, FleetWindowTab::Personnel));
    }

    #[test]
    fn a_renamed_ship_is_listed_by_its_own_name() {
        // FUN_004f6270: the ship's +0x34 name, else its class's.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 1);
        let transport = TroopTransportState::default();
        let selected = Some(FleetWindowEntry::Fleet(fleet));
        let labels = |world: &GameWorld| {
            right_items(world, &transport, selected, FleetWindowTab::CapitalShips)
                .into_iter()
                .map(|item| item.label)
                .collect::<Vec<_>>()
        };
        let class = world.fleets[fleet].capital_ships[0].class;
        assert_eq!(
            labels(&world),
            [world.capital_ship_classes[class].name.clone()]
        );
        assert!(world.rename_ship(fleet, 0, "Liberty"));
        assert_eq!(labels(&world), ["Liberty"]);
    }

    #[test]
    fn a_ship_without_a_hyperdrive_carries_the_indicator() {
        // FUN_004a6be0: flag 0x40 marks a ship with no hyperdrive (10430).
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let fleet = add_fleet(&mut world, system, true, 1);
        let transport = TroopTransportState::default();
        let selected = Some(FleetWindowEntry::Fleet(fleet));
        let flags = |world: &GameWorld| {
            right_items(world, &transport, selected, FleetWindowTab::CapitalShips)
                .iter()
                .map(|item| item.no_hyperdrive)
                .collect::<Vec<_>>()
        };
        assert_eq!(flags(&world), [false]);
        let class = world.fleets[fleet].capital_ships[0].class;
        world.capital_ship_classes[class].hyperdrive = 0;
        assert_eq!(flags(&world), [true]);
    }

    #[test]
    fn a_drop_on_an_enemy_fleets_listed_ship_targets_that_fleet() {
        // FUN_004a3130 step 3 on tab 0x66: the right list's item, whatever
        // its side; elsewhere the default refuses (an own fleet is listed).
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        add_fleet(&mut world, system, true, 1);
        let enemy = add_fleet(&mut world, system, false, 1);
        let mut open = window(system, Some(FleetWindowEntry::Fleet(enemy)));
        let entries = left_entries(
            &world,
            &rebellion_core::movement::MovementState::default(),
            &fog(system),
            Faction::Alliance,
            &open,
        );
        let target = |open: &OpenFleetWindow, point| {
            drop_target(&world, Faction::Alliance, open, &entries, point)
        };
        let enemy_fleet = Some(ReleaseTarget::Fleet {
            fleet: enemy,
            system,
        });

        assert_eq!(target(&open, (150.0, 127.0)), enemy_fleet);
        assert_eq!(target(&open, (150.0, 176.9)), enemy_fleet);
        assert_eq!(target(&open, (150.0, 177.0)), None);
        assert_eq!(target(&open, (150.0, 126.9)), None);
        open.tab = FleetWindowTab::Troops;
        assert_eq!(target(&open, (150.0, 140.0)), None);
    }

    #[test]
    fn the_hit_rectangles_exclude_their_right_and_bottom_edges() {
        // Win32 PtInRect.
        assert!(in_rect(LEFT_LIST, (4.0, 29.0)));
        assert!(in_rect(LEFT_LIST, (94.9, 294.9)));
        assert!(!in_rect(LEFT_LIST, (95.0, 100.0)));
        assert!(!in_rect(LEFT_LIST, (50.0, 295.0)));
        assert!(!in_rect(LEFT_LIST, (3.9, 100.0)));
        assert!(!in_rect(LEFT_LIST, (50.0, 28.9)));
    }

    #[test]
    fn the_fleet_picture_is_centred_on_whole_pixels_in_its_panel() {
        // FUN_004a5c00: 10425 is 122 wide in the 125-wide panel at x 100.
        assert_eq!(picture_x(122.0), 101.0);
        assert_eq!(picture_x(119.0), 103.0);
        assert_eq!(picture_x(125.0), 100.0);
    }

    #[test]
    fn the_normal_tab_art_counts_up_from_the_base() {
        // FUN_004a4b10: normal b + k.
        assert_eq!(
            tab_resource(10409, FleetWindowTab::Fighters, false, true),
            10410
        );
        assert_eq!(
            tab_resource(10459, FleetWindowTab::Personnel, false, true),
            10462
        );
    }

    #[test]
    fn the_fleet_icon_falls_back_to_the_other_side_at_an_unheld_system() {
        // FUN_0045ccc0: the player's side, then the other side, before the
        // system's own side bits.
        for (player, enemy_is_alliance, side) in
            [(Faction::Alliance, false, 2), (Faction::Empire, true, 1)]
        {
            let (mut world, system) = world(ControlKind::Uncontrolled);
            add_fleet(&mut world, system, enemy_is_alliance, 0);
            let mut fog = FogState::new(player);
            fog.reveal(system);
            assert_eq!(
                icon_side(
                    &world,
                    &rebellion_core::movement::MovementState::default(),
                    &fog,
                    player,
                    system
                ),
                (side, 1),
                "{player:?}"
            );
        }
    }
}
