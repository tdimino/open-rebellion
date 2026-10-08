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
//! slot 22 (`FUN_004a7a20`) gives the list's selected items.

use egui_macroquad::egui;
use rebellion_core::dat::{ExplorationStatus, Faction};
use rebellion_core::economy::EconomyState;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{CharacterKey, DefenseFacilityKey, SpecialForceKey, SystemKey, TroopKey};
use rebellion_core::missions::{MissionMember, MissionState};
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::{control_side, faction_side, fleet_side, paint_native};
use crate::object_menu::MenuObject;
use crate::quadrant_icons::{mission_keys, system_members};
use crate::system_window::{
    canvas_point, character_mini_resource_id, clamp_window_to_galaxy, defense_facility_mini,
    exact_clicked, logical_rect, opposing_contents_visible, rect_contains, special_force_mini,
    troop_mini,
};

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
    Defense(DefenseFacilityKey),
}

impl DefensesItem {
    /// The object its pop-up menu opens for: a character (`FUN_004ed350`),
    /// a special force (`FUN_00503b50`), a regiment (`FUN_00504b30`) or a
    /// defense facility, whose menu holds Encyclopedia and Status
    /// (`FUN_0051d990`).
    const fn menu_object(self) -> Option<MenuObject> {
        match self {
            Self::Character(key) => Some(MenuObject::Character(key)),
            Self::SpecialForce(key) => Some(MenuObject::SpecialForce(key)),
            Self::Troop(key) => Some(MenuObject::Troop(key)),
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenDefensesWindow {
    system: SystemKey,
    logical_position: (i16, i16),
    page: DefensesPage,
    /// port: one selected row; the list's own selection rules
    /// (`FUN_00609410`) are untraced.
    selected: Option<DefensesItem>,
}

/// The open Defenses windows. The last is focused and paints on top.
#[derive(Debug)]
pub struct DefensesWindowState {
    faction: CockpitFaction,
    windows: Vec<OpenDefensesWindow>,
}

impl Default for DefensesWindowState {
    fn default() -> Self {
        Self {
            faction: CockpitFaction::Alliance,
            windows: Vec::new(),
        }
    }
}

impl DefensesWindowState {
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
            system,
            logical_position: clamp_window_to_galaxy(
                logical_position,
                layout,
                DEFENSES_WINDOW_WIDTH,
                DEFENSES_WINDOW_HEIGHT,
            ),
            page: DefensesPage::Personnel,
            selected: None,
        });
        true
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

    /// Show `page` in `system`'s open window, as a click on its tab does
    /// (`FUN_004a90d0` refills only for a different page).
    pub fn show_page(&mut self, system: SystemKey, page: DefensesPage) -> bool {
        let Some(window) = self.window_mut(system) else {
            return false;
        };
        if window.page != page {
            window.page = page;
            window.selected = None;
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
            .find(|window| area_id(window.system) == layer.id)
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
    /// Each page's row count, in [`DefensesPage::ALL`] order.
    pub counts: [usize; 5],
    pub rows: Vec<String>,
    pub selected: Option<usize>,
    pub garrison: Option<String>,
}

/// Actions that leave the Defenses window manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefensesWindowAction {
    /// The sector button (`0xca`) opens the subject's sector window
    /// (`FUN_00429ce0`).
    OpenSector(SystemKey),
    SelectSystem(SystemKey),
    /// The minimize button (`0xc9`) posts `0x466`: the window goes to the
    /// galaxy view's rail.
    Minimize {
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A right release on the list opens the object pop-up menu
    /// (`FUN_004ac5c0`) for the selection, at a 640 by 480 canvas point.
    OpenObjectMenu {
        system: SystemKey,
        selection: Option<MenuObject>,
        point: (i16, i16),
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
/// and facilities in their system's order. Squadrons live only aboard
/// fleets in the port, so the squadron page is always empty.
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
                            label: character.name.clone(),
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
        DefensesPage::Squadrons => Vec::new(),
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

fn area_id(system: SystemKey) -> egui::Id {
    egui::Id::new(("original-defenses-window", system))
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
    state.prepare_faction(faction);
    let windows = state.windows.clone();
    let focused_system = windows.last().map(|window| window.system);
    let mut actions = Vec::new();
    for window in &windows {
        let result = draw_defenses_window(
            ctx,
            world,
            fog,
            missions,
            economy,
            window,
            focused_system == Some(window.system),
            faction,
            layout,
            cache,
        );
        let system = window.system;
        if result.close || !world.systems.contains_key(system) {
            state.close(system);
            continue;
        }
        if result.minimize {
            if let Some(closed) = state.close(system) {
                actions.push(DefensesWindowAction::Minimize {
                    system,
                    logical_position: closed.logical_position,
                });
            }
            continue;
        }
        if result.open_sector {
            actions.push(DefensesWindowAction::OpenSector(system));
        }
        if let Some(open) = state.window_mut(system) {
            // FUN_004a90d0 refills only for a different page.
            if let Some(page) = result.page.filter(|&page| page != open.page) {
                open.page = page;
                open.selected = None;
            }
            if result.deselect {
                open.selected = None;
            }
            if let Some(item) = result.select {
                open.selected = Some(item);
            }
        }
        if result.focus {
            state.focus(system);
            actions.push(DefensesWindowAction::SelectSystem(system));
        }
        if let Some((selection, point)) = result.object_menu {
            actions.push(DefensesWindowAction::OpenObjectMenu {
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

            // The tabs. No tab is ever disabled; an empty one opens its
            // empty page. port: the help messages (`FUN_00600a40`) are not
            // shown.
            for page in DefensesPage::ALL {
                let rect = tab_rect(local, scale, page);
                let empty = page_rows(world, fog, missions, player, window.system, page).is_empty();
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
            // name word-wrapped from (1, 27). port: the scroll bar
            // (`FUN_0060a490`) is not drawn, so only the first nine cells show.
            let list = logical_rect(local, scale, LIST.0, LIST.1, LIST.2, LIST.3);
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
            let (any_pressed, right_pressed, release_point) = ctx.input(|input| {
                (
                    input.pointer.button_pressed(egui::PointerButton::Primary)
                        || input.pointer.button_pressed(egui::PointerButton::Secondary),
                    input.pointer.button_pressed(egui::PointerButton::Secondary),
                    input.pointer.interact_pos(),
                )
            });
            if any_pressed && list_response.is_pointer_button_down_on() {
                result.deselect = true;
                result.focus = true;
            }
            if list_response.secondary_clicked() {
                if let Some(point) = release_point.filter(|point| rect_contains(list, *point)) {
                    result.object_menu = Some((None, canvas_point(layout, point)));
                }
            }
            let font = egui::FontId::proportional((9.0 * scale).max(6.0));
            let frame = if side == 1 {
                ROW_FRAME[0]
            } else {
                ROW_FRAME[1]
            };
            for (index, row) in rows.iter().take(COLUMNS * VISIBLE_ROWS).enumerate() {
                let cell = cell_rect(local, scale, index);
                let selected = window.selected == Some(row.item);
                paint_native(
                    &list_painter,
                    ctx,
                    cache,
                    DllSource::Gokres,
                    row.mini,
                    cell,
                    scale,
                    0.0,
                    0.0,
                );
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
                }
                // A right press selects the row as a left press does
                // (FUN_006083c0 shares the WM_LBUTTONDOWN path); the release
                // opens its menu.
                if right_pressed && response.is_pointer_button_down_on() {
                    result.select = Some(row.item);
                    result.focus = true;
                }
                if response.secondary_clicked() {
                    if let (Some(object), Some(point)) = (
                        row.item.menu_object(),
                        response
                            .interact_pointer_pos()
                            .filter(|point| rect_contains(cell.intersect(list), *point)),
                    ) {
                        result.object_menu = Some((Some(object), canvas_point(layout, point)));
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

        let layer = egui::LayerId::new(egui::Order::Foreground, area_id(system));
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
    fn a_page_shows_only_its_first_nine_cells_without_the_scroll_bar() {
        // port: FUN_0060a490's scroll bar is not drawn, so three rows of
        // three cells show; the ninth sits at (147, 221).
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
