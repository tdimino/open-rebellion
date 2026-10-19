//! The Operation Report window (window type `0x1c`: `FUN_00422ce0` case
//! `0x1c` -> ctor `FUN_0044c410`, vtable `0x00659c78`, builder
//! `FUN_0044c630`; `ghidra/notes/operation-report-window.md`).
//!
//! A 470 by 331 window over the galaxy view that shows one Operation Report
//! message (kind `0xf`): a rail of a Close button and four radio views, the
//! Summary (the message's picture, title and text), the fleet side's and
//! the system's forces (six category tabs over a two- or three-column list
//! of the report's objects) and Display (two buttons that go to the system).
//! The art set is the player's side (`+0x118 + 0x9c`). It plays no sound.

use std::collections::VecDeque;
use std::ops::Range;
use std::sync::Arc;

use egui_macroquad::egui;
use rebellion_core::dat::Faction;
use rebellion_core::ids::SystemKey;
use rebellion_core::movement::MovementState;
use rebellion_core::operation_report::{OperationReport, ReportObject, ReportUnit};
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::{mini_layers, paint_native, portrait_layers, MiniObject};
use crate::message_log::{GameMessage, MessageId};
use crate::mission_dialog::{button, galaxy_centered_rect, paint_button};
use crate::scroll_bar::{ScrollBar, ScrollDrag, ScrollRange, LIST_BAR_29FC};
use crate::system_window::{logical_rect, rect_contains};

pub const OPERATION_REPORT_WIDTH: f32 = 470.0;
pub const OPERATION_REPORT_HEIGHT: f32 = 331.0;

// STRATEGY bitmaps by the player's side, Alliance then Empire
// (FUN_0044c630.c:73-110).
const SHELL: [u32; 2] = [0x285f, 0x2860];
const RAIL_ART: [u32; 2] = [0x2a44, 0x2a45];
/// The rail art's x (`0x19c` / `0x19f`, FUN_0044c630.c:150-155).
const RAIL_X: [f32; 2] = [412.0, 415.0];
/// Every layer's picture is blitted at (12, 13) under the shell
/// (FUN_0044c630.c:149).
const PICTURE_AT: (f32, f32) = (12.0, 13.0);
/// Layer 3 (tabs `0x97`-`0x9b`), layer 4 (Personnel) and layer 5 (Display)
/// (FUN_0044c630.c:116-140).
const LAYER_TWO_COLUMN: u32 = 0x2a09;
const LAYER_THREE_COLUMN: u32 = 0x2a0a;
const LAYER_DISPLAY: u32 = 0x2a0b;

#[derive(Debug, Clone, Copy)]
struct Control {
    rect: (f32, f32, f32, f32),
    art: (u32, u32),
}

/// Close (`0x0a`), normal then pressed (FUN_0044c630.c:366-391).
const CLOSE: [Control; 2] = [
    Control {
        rect: (423.0, 25.0, 32.0, 31.0),
        art: (0x2882, 0x2883),
    },
    Control {
        rect: (426.0, 21.0, 44.0, 41.0),
        art: (0x2888, 0x2889),
    },
];

/// The rail group (`100`, radio) at A (0x1a2, 0x58), E (0x1aa, 0x59); its
/// buttons are 41 or 44 wide and 41 high (FUN_0044c630.c:400-420).
const RAIL_ORIGIN: [(f32, f32); 2] = [(418.0, 88.0), (426.0, 89.0)];
const RAIL_BUTTON: [(f32, f32); 2] = [(41.0, 41.0), (44.0, 41.0)];

/// The tab group (`0x96`) at (0x24, 0x3b); each tab 0x31 by 0x29
/// (FUN_0044c630.c:604-760).
const TABS_AT: (f32, f32) = (36.0, 59.0);
const TAB_SIZE: (f32, f32) = (49.0, 41.0);

/// The list (`FUN_00607ea0` at (0x25, 0x88), 0x15b by 0xa6;
/// FUN_0044c630.c:786-797).
const LIST: (f32, f32, f32, f32) = (37.0, 136.0, 347.0, 166.0);

/// The title: font 0xb, centred in 400 at (12, 13) (FUN_0044c630.c:178-184).
const TITLE_AT: (f32, f32) = (12.0, 13.0);
const TITLE_WIDTH: f32 = 400.0;
/// The Summary's text: font 6, 0x15e wide, at (0x19, 0xe1) over a black
/// copy at (0x1a, 0xe2) (FUN_0044c630.c:186-199).
const BODY_AT: (f32, f32) = (25.0, 225.0);
const TEXT_WIDTH: f32 = 350.0;
/// The Display layer's text `0x7013`: font 6, 0x15e wide, at x 0x1d with
/// its last line ending at y 0xb1 (FUN_0044c630.c:237-245).
const DISPLAY_TEXT_X: f32 = 29.0;
const DISPLAY_TEXT_BOTTOM: f32 = 177.0;
/// The TEXTSTRA entry the Display layer shows (FUN_0044c630.c:237).
pub const DISPLAY_TEXT_ID: u16 = 0x7013;
/// The column headers' y, font 4 (FUN_0044c630.c:264-328).
const HEADER_Y: f32 = 117.0;
/// The overlay (FUN_0044e290): the view's name centred in 0x15b at
/// (12, 0x24), font 6, and the tab's name at (0x26, 100), font 4.
const SUBTITLE_AT: (f32, f32) = (12.0, 36.0);
const SUBTITLE_WIDTH: f32 = 347.0;
const TAB_NAME_AT: (f32, f32) = (38.0, 100.0);

/// Display's buttons, 0xa9 by 0x60 at y 0xb6: the system at x 0x1d
/// (`0x0b`, `0x2a0c`) and the fleet at x 0xe2 (`0x0c`, `0x2a0d` Alliance,
/// `0x2a0e` Empire); pressed, the side's frame (`0x2a0f`, `0x2a10`) is
/// keyed over the picture (FUN_0044c630.c:804-931).
const GOTO_SYSTEM_RECT: (f32, f32, f32, f32) = (29.0, 182.0, 169.0, 96.0);
const GOTO_FLEET_RECT: (f32, f32, f32, f32) = (226.0, 182.0, 169.0, 96.0);
const GOTO_SYSTEM_ART: u32 = 0x2a0c;
const GOTO_FLEET_ART: [u32; 2] = [0x2a0d, 0x2a0e];
const GOTO_PRESSED_FRAME: [u32; 2] = [0x2a0f, 0x2a10];
/// The caption's offset in its button (`FUN_006027e0(btn, 0, h / 2 - 12)`).
const GOTO_CAPTION_Y: f32 = 36.0;
/// A font-4 line, 16 pixels (`message-index-rows.md`).
const CAPTION_LINE: f32 = 16.0;

/// The empty cells' texts (`FUN_0044e500`): RCDATA `0x8526` (TEXTSTRA
/// 34086) and `0x1941`.
const NONE_TEXT: &str = "None";
const NO_CASUALTIES_TEXT: &str = "No Casualties";
/// An empty cell's text sits 10 below the cell's top (`FUN_0044edb0.c:44`).
const EMPTY_TEXT_Y: f32 = 10.0;

/// The rail's radio views (`FUN_0044e3d0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReportView {
    /// `0x65`, layer 1: the picture, the title and the text.
    #[default]
    Summary,
    /// `0x66`: the fleet side's lists (`+0x78`, `+0x88`, `+0x90`).
    FleetForces,
    /// `0x67`: the system's lists (`+0x70`, `+0x80`, `+0x98`).
    SystemForces,
    /// `0x68`, layer 5: go to the system.
    Display,
}

impl ReportView {
    pub const ALL: [Self; 4] = [
        Self::Summary,
        Self::FleetForces,
        Self::SystemForces,
        Self::Display,
    ];

    /// The button's command id.
    #[must_use]
    pub const fn command(self) -> u16 {
        match self {
            Self::Summary => 0x65,
            Self::FleetForces => 0x66,
            Self::SystemForces => 0x67,
            Self::Display => 0x68,
        }
    }

    const fn index(self) -> usize {
        match self {
            Self::Summary => 0,
            Self::FleetForces => 1,
            Self::SystemForces => 2,
            Self::Display => 3,
        }
    }

    /// The button's y in the rail group, Alliance then Empire.
    const fn y(self, side: usize) -> f32 {
        const Y: [[f32; 4]; 2] = [[0.0, 53.0, 108.0, 162.0], [0.0, 59.0, 118.0, 177.0]];
        Y[side][self.index()]
    }

    const fn lists(self) -> bool {
        matches!(self, Self::FleetForces | Self::SystemForces)
    }

    /// The button's normal and pressed bitmaps: Forces by the fleet
    /// owner (`+0x6c`), System Forces by the holder (`+0x68`).
    fn art(self, side: usize, report: &OperationReport) -> (u32, u32) {
        let by = |faction: Faction, neutral: [(u32, u32); 2]| match faction {
            Faction::Alliance => [(0x29ea, 0x29eb), (0x29f4, 0x29f5)][side],
            Faction::Empire => [(0x29ec, 0x29ed), (0x29f6, 0x29f7)][side],
            Faction::Neutral => neutral[side],
        };
        match self {
            Self::Summary => [(0x29e8, 0x29e9), (0x29f2, 0x29f3)][side],
            Self::FleetForces => by(report.fleet_owner, [(0x29ec, 0x29ed), (0x29f6, 0x29f7)]),
            Self::SystemForces => by(report.holder, [(0x29f0, 0x29f1), (0x29fa, 0x29fb)]),
            Self::Display => [(0x2a27, 0x2a28), (0x2918, 0x2919)][side],
        }
    }

    /// The button's tooltip (`FUN_00600a40`), which the overlay shows over
    /// a list: `0x1937` Alliance Forces or `0x1938` Imperial Forces by the
    /// fleet owner (none for another), `0x1943`, `0x1944`, `0x1939`.
    const fn tooltip(self, report: &OperationReport) -> Option<&'static str> {
        match self {
            Self::Summary => Some("Summary"),
            Self::FleetForces => match report.fleet_owner {
                Faction::Alliance => Some("Alliance Forces"),
                Faction::Empire => Some("Imperial Forces"),
                Faction::Neutral => None,
            },
            Self::SystemForces => Some("System Forces"),
            Self::Display => Some("Display"),
        }
    }

    const fn control(self) -> &'static str {
        match self {
            Self::Summary => "Summary",
            Self::FleetForces => "FleetForces",
            Self::SystemForces => "SystemForces",
            Self::Display => "Display",
        }
    }
}

/// The six category tabs (`0x97`-`0x9c`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportTab {
    CapitalShips,
    FighterSquadrons,
    Manufacturing,
    DefenseFacilities,
    TrooperRegiments,
    Personnel,
}

impl ReportTab {
    pub const ALL: [Self; 6] = [
        Self::CapitalShips,
        Self::FighterSquadrons,
        Self::Manufacturing,
        Self::DefenseFacilities,
        Self::TrooperRegiments,
        Self::Personnel,
    ];

    /// The tab's command id.
    #[must_use]
    pub const fn command(self) -> u16 {
        match self {
            Self::CapitalShips => 0x97,
            Self::FighterSquadrons => 0x98,
            Self::Manufacturing => 0x99,
            Self::DefenseFacilities => 0x9a,
            Self::TrooperRegiments => 0x9b,
            Self::Personnel => 0x9c,
        }
    }

    pub(crate) const fn index(self) -> usize {
        match self {
            Self::CapitalShips => 0,
            Self::FighterSquadrons => 1,
            Self::Manufacturing => 2,
            Self::DefenseFacilities => 3,
            Self::TrooperRegiments => 4,
            Self::Personnel => 5,
        }
    }

    /// The families `[lo, hi)` each list is filtered by (`FUN_0044e500`).
    #[must_use]
    pub const fn families(self) -> Range<u8> {
        match self {
            Self::CapitalShips => 0x14..0x1c,
            Self::FighterSquadrons => 0x1c..0x20,
            Self::Manufacturing => 0x28..0x30,
            Self::DefenseFacilities => 0x22..0x28,
            Self::TrooperRegiments => 0x10..0x14,
            Self::Personnel => 0x30..0x40,
        }
    }

    /// The tab's x in its group (FUN_0044c630.c:604-760).
    const fn x(self) -> f32 {
        const X: [f32; 6] = [0.0, 62.0, 124.0, 186.0, 248.0, 312.0];
        X[self.index()]
    }

    /// The tab's normal and pressed bitmaps, Alliance then Empire.
    pub(crate) const fn art(self, side: usize) -> (u32, u32) {
        const ART: [[(u32, u32); 2]; 6] = [
            [(0x286c, 0x286b), (0x2878, 0x2877)],
            [(0x2872, 0x2871), (0x287c, 0x287b)],
            [(0x2868, 0x2867), (0x2874, 0x2873)],
            [(0x2866, 0x2865), (0x2866, 0x2865)],
            [(0x2870, 0x286f), (0x287a, 0x2879)],
            [(0x286a, 0x2869), (0x2876, 0x2875)],
        ];
        ART[self.index()][side]
    }

    /// The tooltip, the overlay's tab name: TEXTSTRA `0x1928`, `0x1929`,
    /// `0x1835`, `0x1946`, `0x1930`, `0x1931`.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::CapitalShips => "Capital Ships",
            Self::FighterSquadrons => "Fighter Squadrons",
            Self::Manufacturing => "Manufacturing",
            Self::DefenseFacilities => "Defense Facilities",
            Self::TrooperRegiments => "Trooper Regiments",
            Self::Personnel => "Personnel",
        }
    }

    const fn control(self) -> &'static str {
        match self {
            Self::CapitalShips => "CapitalShips",
            Self::FighterSquadrons => "FighterSquadrons",
            Self::Manufacturing => "Manufacturing",
            Self::DefenseFacilities => "DefenseFacilities",
            Self::TrooperRegiments => "TrooperRegiments",
            Self::Personnel => "Personnel",
        }
    }

    /// Personnel lists three columns of 110 by 50 cells; the others two of
    /// 165 by 70 (`FUN_006082c0(list, 0x6e, 0x32)` / `(0xa5, 0x46)`).
    pub(crate) const fn cell(self) -> (f32, f32) {
        match self {
            Self::Personnel => (110.0, 50.0),
            _ => (165.0, 70.0),
        }
    }

    pub(crate) const fn columns(self) -> usize {
        match self {
            Self::Personnel => 3,
            _ => 2,
        }
    }

    /// The column headers and their x, each 0xa5 or 0x6e wide
    /// (FUN_0044c630.c:264-328): `0x1932`/`0x1933`, or
    /// `0x1934`/`0x1935`/`0x1936`.
    const fn headers(self) -> &'static [(&'static str, f32)] {
        match self {
            Self::Personnel => &[("Survivors", 38.0), ("Captured", 165.0), ("Killed", 275.0)],
            _ => &[("Operational", 38.0), ("Destroyed", 204.0)],
        }
    }

    pub(crate) const fn layer(self) -> u32 {
        match self {
            Self::Personnel => LAYER_THREE_COLUMN,
            _ => LAYER_TWO_COLUMN,
        }
    }

    /// Left (`0x25`) and Right (`0x27`) step to the neighbouring tab
    /// (`FUN_0044e180`); Right past the last, or Left at the first, takes
    /// the group's first (`FUN_005f5060`).
    fn stepped(self, right: bool) -> Self {
        let index = self.index();
        let next = if right {
            index + 1
        } else {
            index.wrapping_sub(1)
        };
        Self::ALL.get(next).copied().unwrap_or(Self::CapitalShips)
    }
}

/// One cell of the list (`FUN_0044edb0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportCell<'a> {
    /// An object: its picture and name.
    Object(&'a ReportUnit),
    /// The first empty cell of a column: "None" or "No Casualties".
    Text(&'static str),
    Blank,
}

/// The list `FUN_0044e500` fills, row by row: the view's operational,
/// (for Personnel) captured and destroyed lists, each filtered to the tab's
/// families, side by side; a column out of objects shows its empty text in
/// its first row, then blanks while another column goes on.
#[must_use]
pub fn list_cells<'a>(
    report: &'a OperationReport,
    view: ReportView,
    tab: ReportTab,
) -> Vec<ReportCell<'a>> {
    let (operational, destroyed, captured) = match view {
        ReportView::FleetForces => (
            &report.fleet_operational,
            &report.fleet_destroyed,
            &report.fleet_captured,
        ),
        ReportView::SystemForces => (
            &report.system_operational,
            &report.system_destroyed,
            &report.system_captured,
        ),
        ReportView::Summary | ReportView::Display => return Vec::new(),
    };
    let families = tab.families();
    let filtered = |list: &'a [ReportUnit]| -> Vec<&'a ReportUnit> {
        list.iter()
            .filter(|unit| families.contains(&unit.family()))
            .collect()
    };
    let mut columns = vec![(filtered(operational), NONE_TEXT)];
    if tab == ReportTab::Personnel {
        columns.push((filtered(captured), NONE_TEXT));
    }
    columns.push((filtered(destroyed), NO_CASUALTIES_TEXT));
    let rows = columns
        .iter()
        .map(|(units, _)| units.len().max(1))
        .max()
        .unwrap_or(1);
    let mut cells = Vec::with_capacity(rows * columns.len());
    for row in 0..rows {
        for (units, empty) in &columns {
            cells.push(match units.get(row) {
                Some(unit) => ReportCell::Object(unit),
                None if row == 0 => ReportCell::Text(empty),
                None => ReportCell::Blank,
            });
        }
    }
    cells
}

/// A cell's object picture, bottom layer first: `FUN_0042c3b0(.., arg, 1)`
/// with arg 0 on the Personnel tab (the list mini over its backdrop,
/// `mini_layers`) and 1 on the others (the GOKRES portrait, the mini less
/// `0x4000`) (`FUN_0044e500.c:117-128`, `FUN_0044edb0.c:56`), without the
/// regiment backdrop and the status marks that [`cell_layers`] adds. port:
/// the battle summary's cells (`tactical_view`) draw this one.
#[must_use]
pub fn cell_picture(unit: &ReportUnit, tab: ReportTab) -> Vec<(DllSource, u32)> {
    let id = unit.dat_id;
    if tab == ReportTab::Personnel {
        return match id.family() {
            0x30..=0x3b => crate::system_window::character_mini_resource_id(id, unit.major)
                .map(|mini| mini_layers(MiniObject::Character, mini, false, None)),
            _ => crate::system_window::special_force_mini(id)
                .map(|(mini, _)| mini_layers(MiniObject::SpecialForce(id), mini, false, None)),
        }
        .unwrap_or_default();
    }
    let mini = match id.family() {
        0x14..=0x1b => crate::panels::fleets::capital_ship_mini_id(id),
        0x1c..=0x1f => crate::panels::fleets::fighter_mini_id(id),
        _ => crate::manufacturing_window::class_mini(id).map(|(mini, _)| mini),
    };
    mini.and_then(|mini| mini.checked_sub(0x4000))
        .map(|portrait| vec![(DllSource::Gokres, portrait)])
        .unwrap_or_default()
}

/// The object state `FUN_0042c3b0`'s status marks read.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CellStatus {
    /// `+0x50` bit 4 (en route) set and bit 3 (destroyed) clear.
    pub en_route: bool,
    /// `+0x50` bit 9, a capital ship's hull damage or damage counters
    /// (`FUN_00501c00`).
    pub damaged: bool,
    /// `+0xac` bit 0, a captive character.
    pub captive: bool,
}

/// `unit`'s object's state as the window draws it ([`ReportObject`]): an
/// object is en route while it or its fleet travels (`FUN_004f8240`) and it
/// is not destroyed. A tagged ship is read wherever it now is; one gone
/// from every fleet reads nothing; a destroyed object reads the damaged bit
/// it died with ([`ReportObject::Destroyed`]). port: a squadron's bit 9, its lost craft
/// (`FUN_00503340` from the squad-size damage `+0x60`), stays clear: the
/// strategic squadron keeps only a count
/// (`or-port-squadron-craft-losses-karc`). No state here sets a
/// character's or special force's bit 9 (STRATEGY 11502, from
/// `FUN_004f1720`, untraced). None clears bit 2 (11570-11573, under
/// construction): the report lists a fleet's ships in mode 1
/// (`FUN_00496770.c:56,157`), so no unfinished ship is listed.
#[must_use]
pub fn cell_status(world: &GameWorld, movement: &MovementState, unit: &ReportUnit) -> CellStatus {
    let travels = |fleet| movement.is_in_transit(fleet);
    match unit.object {
        None => CellStatus::default(),
        Some(ReportObject::Aboard(fleet) | ReportObject::Ship { fleet, tag: 0 }) => CellStatus {
            en_route: travels(fleet),
            ..CellStatus::default()
        },
        Some(ReportObject::Ship { tag, .. }) => world
            .fleets
            .iter()
            .find_map(|(fleet, value)| {
                value
                    .capital_ships
                    .iter()
                    .find(|ship| ship.tag == tag)
                    .map(|ship| (fleet, ship))
            })
            .map_or_else(CellStatus::default, |(fleet, ship)| CellStatus {
                // FUN_004f8240.c:17-31: its own leg (bit 5) or its fleet's.
                en_route: ship.alive && (travels(fleet) || ship.transit.is_some()),
                damaged: world
                    .capital_ship_classes
                    .get(ship.class)
                    .is_some_and(|class| ship.is_damaged(class.hull.cast_signed())),
                captive: false,
            }),
        Some(ReportObject::Character(key)) => {
            world
                .characters
                .get(key)
                .map_or_else(CellStatus::default, |value| CellStatus {
                    en_route: !value.is_killed
                        && (value.en_route || value.current_fleet.is_some_and(travels)),
                    damaged: false,
                    captive: value.is_captive,
                })
        }
        Some(ReportObject::SpecialForce(key)) => CellStatus {
            en_route: world
                .special_forces
                .get(key)
                .is_some_and(|value| value.en_route),
            ..CellStatus::default()
        },
        // FUN_0042c3b0 tests no column: a destroyed object shows the
        // damaged bit its view kept.
        Some(ReportObject::Destroyed { damaged }) => CellStatus {
            damaged,
            ..CellStatus::default()
        },
    }
}

/// A cell's picture with its status marks, bottom layer first
/// (`FUN_0042c3b0(.., arg, 1)`): on the Personnel tab the mini's stack
/// ([`mini_layers`]), then over a captive character (`+0xac` bit 0) its
/// GOKRES `(class & 0xfff) + 0x7000`, the mini plus `0x3000`
/// (`FUN_0042c3b0.c:454-461`); elsewhere the portrait's
/// ([`portrait_layers`]). A facility's portrait takes no mark: none of its
/// states is listed here.
#[must_use]
pub fn cell_layers(unit: &ReportUnit, tab: ReportTab, status: CellStatus) -> Vec<(DllSource, u32)> {
    let id = unit.dat_id;
    if tab == ReportTab::Personnel {
        return match id.family() {
            0x30..=0x3b => crate::system_window::character_mini_resource_id(id, unit.major).map(
                |mini| {
                    let mut layers = mini_layers(MiniObject::Character, mini, status.en_route, None);
                    if status.captive {
                        layers.push((DllSource::Gokres, mini + 0x3000));
                    }
                    layers
                },
            ),
            _ => crate::system_window::special_force_mini(id).map(|(mini, _)| {
                mini_layers(MiniObject::SpecialForce(id), mini, status.en_route, None)
            }),
        }
        .unwrap_or_default();
    }
    let object = match id.family() {
        0x10..=0x13 => Some(MiniObject::Regiment(id)),
        0x14..=0x1f => Some(MiniObject::Craft),
        _ => None,
    };
    match (object, cell_picture(unit, tab).first()) {
        (Some(object), Some(&(_, portrait))) => {
            portrait_layers(object, portrait, status.en_route, status.damaged, None)
        }
        (_, picture) => picture.copied().into_iter().collect(),
    }
}

/// The text colour by the fleet owner (`+0x6c`): `0x20000ff` red for side
/// 1, `0x200ff00` green for side 2, else `0x2ffff00` cyan
/// (FUN_0044c630.c:171-177, FUN_0044e290.c:19-28).
#[must_use]
pub fn side_color(fleet_owner: Faction) -> egui::Color32 {
    match fleet_owner {
        Faction::Alliance => egui::Color32::from_rgb(255, 0, 0),
        Faction::Empire => egui::Color32::from_rgb(0, 255, 0),
        Faction::Neutral => egui::Color32::from_rgb(0, 255, 255),
    }
}

/// What the report window asks of the galaxy view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationReportAction {
    /// Close (`0x0a`) or Escape: slot `+0x30`, then `FUN_00469d60`, which
    /// reopens the Message Index on `category` when the report came from it.
    Closed { reopen_index: Option<u16> },
    /// `0x0b`: close and open the system's Sector window (`FUN_00429ce0`).
    GotoSystem { system: SystemKey },
    /// `0x0c`: the same, then `FUN_0045c8e0(window, system, 4)`: the
    /// sector window's Fleet icon opens the system's Fleet window with
    /// nothing selected (`FUN_0045aac0`, `FUN_0044df90.c:20-35`).
    GotoFleet { system: SystemKey },
}

#[derive(Debug, Clone)]
struct OpenReport {
    message: MessageId,
    report: Arc<OperationReport>,
    title: String,
    body: String,
    /// The system button's caption, read as the window is built: the
    /// system's name (`FUN_004f62d0`), else `0x1945` "Goto System".
    system_caption: String,
    faction: CockpitFaction,
    view: ReportView,
    tab: ReportTab,
    first_row: usize,
    drag: ScrollDrag,
    /// `DAT_006b2910 == 0x1c`: the Message Index category to reopen on
    /// close (`DAT_006b290c`).
    reopen_index: Option<u16>,
}

/// The report window. port: one shows at a time; an alert filed while one
/// is open waits until it closes.
#[derive(Debug, Clone, Default)]
pub struct OperationReportState {
    window: Option<OpenReport>,
    waiting: VecDeque<OpenReport>,
}

impl OperationReportState {
    /// Open `message`'s report on the Summary view and its first tab
    /// (`0x97` for a bombardment, `0x9b` for an assault;
    /// FUN_0044c630.c:921-923), with the player's art. `reopen_index` is
    /// the Index category when the Index opened it. A message without a
    /// report opens nothing.
    pub fn open(
        &mut self,
        world: &GameWorld,
        message: &GameMessage,
        faction: CockpitFaction,
        reopen_index: Option<u16>,
    ) -> bool {
        let Some(report) = message.report.clone() else {
            return false;
        };
        let (title, body) = message
            .display
            .as_ref()
            .map_or((message.text.clone(), String::new()), |display| {
                (display.title.clone(), display.body.clone())
            });
        let system_caption = world
            .systems
            .get(report.system)
            .map_or_else(|| "Goto System".to_owned(), |system| system.name.clone());
        let tab = if report.assault {
            ReportTab::TrooperRegiments
        } else {
            ReportTab::CapitalShips
        };
        let open = OpenReport {
            message: message.id,
            report,
            title,
            body,
            system_caption,
            faction,
            view: ReportView::Summary,
            tab,
            first_row: 0,
            drag: ScrollDrag::default(),
            reopen_index,
        };
        if self.window.is_some() {
            self.waiting.push_back(open);
        } else {
            self.window = Some(open);
        }
        true
    }

    /// Close the shown report; the next waiting one shows.
    pub fn close(&mut self) {
        self.window = self.waiting.pop_front();
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.window.is_some()
    }

    /// The shown report's message.
    #[must_use]
    pub fn message(&self) -> Option<MessageId> {
        self.window.as_ref().map(|window| window.message)
    }

    /// The shown view and tab.
    #[must_use]
    pub fn view(&self) -> Option<(ReportView, ReportTab)> {
        self.window.as_ref().map(|window| (window.view, window.tab))
    }

    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.window.is_some() && rect_contains(window_rect(layout), egui::pos2(point.0, point.1))
    }

    /// `FUN_0044e3d0`: the rail's radio; the list keeps its tab and starts
    /// at its top (`FUN_0044e500(tab, 1)` refills it).
    pub fn set_view(&mut self, view: ReportView) {
        if let Some(window) = &mut self.window {
            if window.view != view {
                window.view = view;
                window.first_row = 0;
                window.drag = ScrollDrag::default();
            }
        }
    }

    /// `FUN_0044e500(tab, 0)`: a new tab refills the list.
    pub fn set_tab(&mut self, tab: ReportTab) {
        if let Some(window) = &mut self.window {
            if window.tab != tab {
                window.tab = tab;
                window.first_row = 0;
                window.drag = ScrollDrag::default();
            }
        }
    }
}

fn window_rect(layout: CockpitLayout) -> egui::Rect {
    galaxy_centered_rect(layout, OPERATION_REPORT_WIDTH, OPERATION_REPORT_HEIGHT)
}

const fn side(faction: CockpitFaction) -> usize {
    match faction {
        CockpitFaction::Alliance => 0,
        CockpitFaction::Empire => 1,
    }
}

/// Draw the open report, if any, in the galaxy view and report an action.
///
/// `display_text` is TEXTSTRA [`DISPLAY_TEXT_ID`], from the caller's text
/// templates, so a mod or language pack can replace it.
pub fn draw_operation_report(
    ctx: &egui::Context,
    state: &mut OperationReportState,
    layout: CockpitLayout,
    world: &GameWorld,
    movement: &MovementState,
    cache: &mut BmpCache,
    display_text: &str,
) -> Option<OperationReportAction> {
    let rect = window_rect(layout);
    draw_at(
        ctx,
        state,
        rect.min,
        layout.scale,
        world,
        movement,
        cache,
        display_text,
    )
}

/// The interface fixture's report: the window at (85, 55), scale 1, as the
/// Message Index fixture places its shell.
#[cfg(feature = "interface-test-fixtures")]
pub fn draw_operation_report_fixture(
    ctx: &egui::Context,
    state: &mut OperationReportState,
    world: &GameWorld,
    movement: &MovementState,
    cache: &mut BmpCache,
    display_text: &str,
) -> Option<OperationReportAction> {
    draw_at(
        ctx,
        state,
        egui::pos2(85.0, 55.0),
        1.0,
        world,
        movement,
        cache,
        display_text,
    )
}

/// Lay `text` out wrapped to `width` in `font`.
fn wrapped(
    painter: &egui::Painter,
    text: &str,
    font: egui::FontId,
    color: egui::Color32,
    width: f32,
) -> Arc<egui::Galley> {
    painter.layout(text.to_owned(), font, color, width)
}

#[expect(
    clippy::too_many_lines,
    reason = "The window paints in the original's layer and control order; splitting it hides that order."
)]
#[expect(
    clippy::too_many_arguments,
    reason = "The window reads the world, the moves and the Display text it is given."
)]
fn draw_at(
    ctx: &egui::Context,
    state: &mut OperationReportState,
    origin: egui::Pos2,
    scale: f32,
    world: &GameWorld,
    movement: &MovementState,
    cache: &mut BmpCache,
    display_text: &str,
) -> Option<OperationReportAction> {
    let window = state.window.clone()?;
    let side_idx = side(window.faction);
    let report = &window.report;
    let rect = egui::Rect::from_min_size(
        origin,
        egui::vec2(
            OPERATION_REPORT_WIDTH * scale,
            OPERATION_REPORT_HEIGHT * scale,
        ),
    );
    let color = side_color(report.fleet_owner);
    let cells = list_cells(report, window.view, window.tab);
    let (cell_w, cell_h) = window.tab.cell();
    let columns = window.tab.columns();
    let rows = cells.len().div_ceil(columns);
    let range = ScrollRange {
        first: window.first_row,
        visible: LIST.3 / cell_h,
        total: rows,
    };
    let bar = ScrollBar::new(LIST_BAR_29FC, LIST.0, LIST.1, LIST.2, LIST.3);

    let mut action = None;
    let mut new_view = None;
    let mut new_tab = None;
    let mut first_row = window.first_row;
    let mut drag = window.drag;

    let id = egui::Id::new("original-operation-report");
    ctx.move_to_top(egui::LayerId::new(egui::Order::Tooltip, id));
    egui::Area::new(id)
        .fixed_pos(rect.min)
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let (frame, _) = ui.allocate_exact_size(rect.size(), egui::Sense::click());
            let painter = ui.painter().with_clip_rect(frame);
            let at = |(x, y, w, h): (f32, f32, f32, f32)| logical_rect(frame, scale, x, y, w, h);
            let paint = |cache: &mut BmpCache, source: DllSource, id: u32, (x, y): (f32, f32)| {
                paint_native(&painter, ctx, cache, source, id, frame, scale, x, y);
            };
            let name = |control: &str, rect: egui::Rect| {
                crate::control_registry::register_screen("OperationReport", control, rect);
            };

            // The layer's bitmap (FUN_006075b0 / FUN_00607610): its picture
            // at (12, 13), the shell over it, the rail art at its edge.
            painter.rect_filled(frame, 0.0, egui::Color32::BLACK);
            let picture = match window.view {
                ReportView::Summary => u32::from(report.picture()),
                ReportView::FleetForces | ReportView::SystemForces => window.tab.layer(),
                ReportView::Display => LAYER_DISPLAY,
            };
            paint(cache, DllSource::Strategy, picture, PICTURE_AT);
            paint(cache, DllSource::Strategy, SHELL[side_idx], (0.0, 0.0));
            paint(
                cache,
                DllSource::Strategy,
                RAIL_ART[side_idx],
                (RAIL_X[side_idx], 0.0),
            );

            // Every layer carries the title.
            let title = theme_font(ctx, 0xb, scale);
            painter.text(
                at((TITLE_AT.0 + TITLE_WIDTH / 2.0, TITLE_AT.1, 0.0, 0.0)).min,
                egui::Align2::CENTER_TOP,
                &window.title,
                title,
                color,
            );
            let body_font = theme_font(ctx, 6, scale);
            match window.view {
                ReportView::Summary => {
                    for (dx, tint) in [(1.0, egui::Color32::BLACK), (0.0, color)] {
                        let galley = wrapped(
                            &painter,
                            &window.body,
                            body_font.clone(),
                            tint,
                            TEXT_WIDTH * scale,
                        );
                        painter.galley(
                            at((BODY_AT.0 + dx, BODY_AT.1 + dx, 0.0, 0.0)).min,
                            galley,
                            tint,
                        );
                    }
                }
                ReportView::Display => {
                    let galley = wrapped(
                        &painter,
                        display_text,
                        body_font.clone(),
                        color,
                        TEXT_WIDTH * scale,
                    );
                    let top = DISPLAY_TEXT_BOTTOM - galley.size().y / scale;
                    painter.galley(at((DISPLAY_TEXT_X, top, 0.0, 0.0)).min, galley, color);
                }
                ReportView::FleetForces | ReportView::SystemForces => {
                    let header = theme_font(ctx, 4, scale);
                    let (header_width, _) = window.tab.cell();
                    for &(text, x) in window.tab.headers() {
                        painter.text(
                            at((x + header_width / 2.0, HEADER_Y, 0.0, 0.0)).min,
                            egui::Align2::CENTER_TOP,
                            text,
                            header.clone(),
                            egui::Color32::WHITE,
                        );
                    }
                    // FUN_0044e290: the view's name and the tab's.
                    if let Some(subtitle) = window.view.tooltip(report) {
                        painter.text(
                            at((
                                SUBTITLE_AT.0 + SUBTITLE_WIDTH / 2.0,
                                SUBTITLE_AT.1,
                                0.0,
                                0.0,
                            ))
                            .min,
                            egui::Align2::CENTER_TOP,
                            subtitle,
                            body_font.clone(),
                            color,
                        );
                    }
                    painter.text(
                        at((TAB_NAME_AT.0, TAB_NAME_AT.1, 0.0, 0.0)).min,
                        egui::Align2::LEFT_TOP,
                        window.tab.label(),
                        header,
                        egui::Color32::WHITE,
                    );
                }
            }

            // Close (0x0a).
            let close = CLOSE[side_idx];
            let close_rect = at(close.rect);
            name("Close", close_rect);
            if button(
                ui,
                cache,
                close_rect,
                "report-close",
                close.art,
                false,
                scale,
            ) {
                action = Some(OperationReportAction::Closed {
                    reopen_index: window.reopen_index,
                });
            }

            // The rail's radio (group 100).
            let (rail_x, rail_y) = RAIL_ORIGIN[side_idx];
            let (button_w, button_h) = RAIL_BUTTON[side_idx];
            for view in ReportView::ALL {
                let view_rect = at((rail_x, rail_y + view.y(side_idx), button_w, button_h));
                name(view.control(), view_rect);
                if button(
                    ui,
                    cache,
                    view_rect,
                    ("report-view", view.index()),
                    view.art(side_idx, report),
                    view == window.view,
                    scale,
                ) {
                    new_view = Some(view);
                }
            }

            if window.view.lists() {
                // The tabs (group 0x96), shown only over a list.
                for tab in ReportTab::ALL {
                    let tab_rect = at((TABS_AT.0 + tab.x(), TABS_AT.1, TAB_SIZE.0, TAB_SIZE.1));
                    name(tab.control(), tab_rect);
                    if button(
                        ui,
                        cache,
                        tab_rect,
                        ("report-tab", tab.index()),
                        tab.art(side_idx),
                        tab == window.tab,
                        scale,
                    ) {
                        new_tab = Some(tab);
                    }
                }

                // The list: rows of cells, each the object's picture
                // centred with its name under it (FUN_0044edb0), white.
                let list_rect = at(LIST);
                let list_painter = painter.with_clip_rect(list_rect);
                let name_font = theme_font(ctx, 10, scale);
                for (index, cell) in cells.iter().enumerate().skip(first_row * columns) {
                    let row = index / columns - first_row;
                    if row as f32 * cell_h >= LIST.3 {
                        break;
                    }
                    let x = LIST.0 + cell_w * (index % columns) as f32;
                    let y = LIST.1 + cell_h * row as f32;
                    match cell {
                        ReportCell::Object(unit) => {
                            let layers = cell_layers(
                                unit,
                                window.tab,
                                cell_status(world, movement, unit),
                            );
                            let size = layers
                                .first()
                                .and_then(|&(source, id)| cache.original_resource_size(source, id))
                                .map_or((0.0, 0.0), |[w, h]| (w as f32, h as f32));
                            let left = ((cell_w - size.0) / 2.0).floor();
                            let top = ((cell_h - size.1) / 2.0).floor();
                            for (source, id) in layers {
                                paint_native(
                                    &list_painter,
                                    ctx,
                                    cache,
                                    source,
                                    id,
                                    frame,
                                    scale,
                                    x + left,
                                    y + top,
                                );
                            }
                            list_painter.text(
                                at((x + cell_w / 2.0, y + top + size.1, 0.0, 0.0)).min,
                                egui::Align2::CENTER_TOP,
                                &unit.name,
                                name_font.clone(),
                                egui::Color32::WHITE,
                            );
                        }
                        ReportCell::Text(text) => {
                            list_painter.text(
                                at((x + cell_w / 2.0, y + EMPTY_TEXT_Y, 0.0, 0.0)).min,
                                egui::Align2::CENTER_TOP,
                                *text,
                                body_font.clone(),
                                egui::Color32::WHITE,
                            );
                        }
                        ReportCell::Blank => {}
                    }
                }
                // The list's bar (RCDATA 0x29fc), one row a step.
                let pointer = crate::scroll_bar::egui_pointer(ctx, frame, scale);
                first_row = bar.update(&mut drag, range, range.visible.floor() as usize, pointer);
                crate::scroll_bar::paint_egui(
                    &painter,
                    ctx,
                    cache,
                    frame,
                    scale,
                    &bar.blits(
                        ScrollRange {
                            first: first_row,
                            ..range
                        },
                        &drag,
                        pointer,
                    ),
                );
                let hovered = ui
                    .input(|input| input.pointer.hover_pos())
                    .is_some_and(|point| rect_contains(list_rect, point));
                let wheel = ui.input(|input| input.raw_scroll_delta.y);
                if hovered && wheel != 0.0 {
                    first_row =
                        crate::mission_dialog::scrolled(first_row, range.max_first() + 1, wheel);
                }
            }

            if window.view == ReportView::Display {
                // 0x0b and 0x0c.
                let font = theme_font(ctx, 4, scale);
                for (control, rect, art, caption) in [
                    (
                        "GotoSystem",
                        GOTO_SYSTEM_RECT,
                        GOTO_SYSTEM_ART,
                        window.system_caption.as_str(),
                    ),
                    (
                        "GotoFleet",
                        GOTO_FLEET_RECT,
                        GOTO_FLEET_ART[side_idx],
                        "Fleet",
                    ),
                ] {
                    let button_rect = at(rect);
                    name(control, button_rect);
                    let response =
                        ui.interact(button_rect, ui.id().with(control), egui::Sense::click());
                    // A CoolStrobeButton (FUN_00602150, FUN_0044c630.c:804-931),
                    // keyed as FUN_00602d30 blits it.
                    paint_button(&painter, ctx, cache, art, button_rect, scale);
                    if response.is_pointer_button_down_on() {
                        paint(
                            cache,
                            DllSource::Strategy,
                            GOTO_PRESSED_FRAME[side_idx],
                            (rect.0, rect.1),
                        );
                    }
                    // Font 4 (app +0xb0, FUN_00413000.c:141), centred and
                    // word-wrapped across the button (DT_CENTER); FUN_006026e0
                    // centres the text's height in the button and
                    // FUN_006027e0 moves it down 36.
                    let mut job = egui::text::LayoutJob::simple(
                        caption.to_owned(),
                        font.clone(),
                        egui::Color32::WHITE,
                        rect.2 * scale,
                    );
                    job.halign = egui::Align::Center;
                    let galley = painter.layout_job(job);
                    let lines = galley.rows.len().max(1) as f32;
                    let top = ((rect.3 - CAPTION_LINE * lines) / 2.0).trunc() + GOTO_CAPTION_Y;
                    painter.galley(
                        at((rect.0 + rect.2 / 2.0, rect.1 + top, 0.0, 0.0)).min,
                        galley,
                        egui::Color32::WHITE,
                    );
                    if crate::system_window::exact_clicked(&response, button_rect) {
                        action = Some(if control == "GotoSystem" {
                            OperationReportAction::GotoSystem {
                                system: report.system,
                            }
                        } else {
                            OperationReportAction::GotoFleet {
                                system: report.system,
                            }
                        });
                    }
                }
            }
        });

    // FUN_0044e180: Escape closes; Left and Right step the tabs over a list.
    let (escape, left, right) = ctx.input(|input| {
        (
            input.key_pressed(egui::Key::Escape),
            input.key_pressed(egui::Key::ArrowLeft),
            input.key_pressed(egui::Key::ArrowRight),
        )
    });
    if escape && action.is_none() {
        action = Some(OperationReportAction::Closed {
            reopen_index: window.reopen_index,
        });
    }
    if window.view.lists() && new_tab.is_none() && (left || right) {
        new_tab = Some(window.tab.stepped(right));
    }
    if let Some(open) = state.window.as_mut() {
        open.first_row = first_row;
        open.drag = drag;
    }
    if let Some(view) = new_view {
        state.set_view(view);
    }
    if let Some(tab) = new_tab {
        state.set_tab(tab);
    }
    if action.is_some() {
        state.close();
    }
    action
}

fn theme_font(ctx: &egui::Context, entry: u8, scale: f32) -> egui::FontId {
    crate::theme::game_font_on(ctx, entry, scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fleet_window::tests::PAINTED;
    use crate::message_log::{MessageCategory, MessageDisplay, MessageRail, RailAudience};
    use rebellion_core::ids::DatId;
    use rebellion_core::world::System;

    const SIDES: [(CockpitFaction, Faction); 2] = [
        (CockpitFaction::Alliance, Faction::Alliance),
        (CockpitFaction::Empire, Faction::Empire),
    ];

    /// port: test fixture values, not rules.
    fn unit(raw: u32, name: &str) -> ReportUnit {
        ReportUnit {
            dat_id: DatId::new(raw),
            name: name.into(),
            major: false,
            object: None,
        }
    }

    /// A bombardment by `owner` of a system the other side holds: two
    /// ships and a squadron stand, one ship died; a regiment and a yard
    /// stand at the system, a regiment died there.
    ///
    /// port: test fixture values, not rules.
    fn report(owner: Faction) -> OperationReport {
        let holder = if owner == Faction::Alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        };
        let mut report = OperationReport::new(SystemKey::default(), false, holder, owner);
        report.fleet_operational = vec![
            unit(0x1400_0040, "First"),
            unit(0x1c00_0001, "Wing"),
            unit(0x1400_0041, "Second"),
            unit(0x3000_0240, "Officer"),
        ];
        report.fleet_destroyed = vec![unit(0x1400_0042, "Lost")];
        report.fleet_captured = vec![unit(0x3800_0340, "Prisoner")];
        report.system_operational = vec![unit(0x1000_0005, "Standing"), unit(0x2800_0001, "Yard")];
        report.system_destroyed = vec![unit(0x1000_0006, "Fallen")];
        report
    }

    fn opened(faction: CockpitFaction, owner: Faction) -> OperationReportState {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System {
            name: "Tokmia".into(),
            ..System::default()
        });
        let mut value = report(owner);
        value.system = system;
        let message = GameMessage::new(1, "Orbital bombardment of Tokmia", MessageCategory::Combat)
            .on_rail(MessageRail::Conflict, RailAudience::Both)
            .with_report(Arc::new(value))
            .with_display(MessageDisplay {
                title: "Orbital bombardment of Tokmia".into(),
                body: "Imperial ships have conducted an orbital strike".into(),
                ..MessageDisplay::default()
            });
        let mut state = OperationReportState::default();
        assert!(state.open(&world, &message, faction, Some(0x80)));
        state
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Text {
        text: String,
        pos: egui::Pos2,
        color: egui::Color32,
    }

    #[derive(Default)]
    struct Run {
        actions: Vec<OperationReportAction>,
        texts: Vec<Text>,
        /// Each text's wrapped line count.
        rows: Vec<(String, usize)>,
        painted: Vec<(u32, egui::Pos2)>,
    }

    /// Draw the window at the origin, scale 1, once per frame of events;
    /// the last frame's text and bitmaps are kept.
    fn run(state: &mut OperationReportState, frames: Vec<Vec<egui::Event>>) -> Run {
        run_in(&GameWorld::default(), &MovementState::new(), state, frames)
    }

    /// [`run`] over `world` and `movement`.
    fn run_in(
        world: &GameWorld,
        movement: &MovementState,
        state: &mut OperationReportState,
        frames: Vec<Vec<egui::Event>>,
    ) -> Run {
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let mut result = Run::default();
        for (index, events) in frames.into_iter().enumerate() {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                time: Some(index as f64 * 0.05),
                events,
                ..Default::default()
            };
            PAINTED.with(|painted| painted.borrow_mut().clear());
            let output = ctx.run(input, |ctx| {
                result.actions.extend(draw_at(
                    ctx,
                    state,
                    egui::Pos2::ZERO,
                    1.0,
                    world,
                    movement,
                    &mut cache,
                    DISPLAY_TEXT,
                ));
            });
            result.painted = PAINTED.with(|painted| painted.take());
            result.texts.clear();
            result.rows.clear();
            for clipped in output.shapes {
                if let egui::Shape::Text(text) = clipped.shape {
                    let format = &text.galley.job.sections[0].format;
                    result.texts.push(Text {
                        text: text.galley.text().to_owned(),
                        pos: text.pos,
                        color: format.color,
                    });
                    result
                        .rows
                        .push((text.galley.text().to_owned(), text.galley.rows.len()));
                }
            }
        }
        result
    }

    fn hover(point: (f32, f32)) -> Vec<Vec<egui::Event>> {
        let point = egui::pos2(point.0, point.1);
        vec![
            vec![egui::Event::PointerMoved(point)],
            vec![egui::Event::PointerMoved(point)],
        ]
    }

    fn press(point: (f32, f32), pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: egui::pos2(point.0, point.1),
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    fn click(point: (f32, f32)) -> Vec<Vec<egui::Event>> {
        let mut frames = hover(point);
        frames.extend([vec![press(point, true)], vec![press(point, false)], vec![]]);
        frames
    }

    fn key(key: egui::Key) -> Vec<Vec<egui::Event>> {
        vec![vec![egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }]]
    }

    /// The centre of the rail's button for `view` on `side`.
    fn rail(side: usize, view: ReportView) -> (f32, f32) {
        let (x, y) = RAIL_ORIGIN[side];
        let (w, h) = RAIL_BUTTON[side];
        (x + w / 2.0, y + view.y(side) + h / 2.0)
    }

    fn tab(tab: ReportTab) -> (f32, f32) {
        (
            TABS_AT.0 + tab.x() + TAB_SIZE.0 / 2.0,
            TABS_AT.1 + TAB_SIZE.1 / 2.0,
        )
    }

    fn painted(run: &Run, id: u32, (x, y): (f32, f32)) -> bool {
        run.painted.contains(&(id, egui::pos2(x, y)))
    }

    fn text<'a>(run: &'a Run, text: &str) -> &'a Text {
        run.texts
            .iter()
            .find(|value| value.text == text)
            .unwrap_or_else(|| panic!("no {text:?} in {:?}", run.texts))
    }

    #[test]
    fn each_tab_lists_its_families_side_by_side_with_the_empty_texts() {
        // FUN_0044e500: each list filtered by the tab's family range, rows
        // built pairwise; a column's first empty cell says "None" (0x8526)
        // or "No Casualties" (0x1941), later ones are blank; Personnel adds
        // the captured column between them.
        for (_, owner) in SIDES {
            let report = report(owner);
            let names = |view, tab| -> Vec<String> {
                list_cells(&report, view, tab)
                    .into_iter()
                    .map(|cell| match cell {
                        ReportCell::Object(unit) => unit.name.clone(),
                        ReportCell::Text(text) => format!("<{text}>"),
                        ReportCell::Blank => String::new(),
                    })
                    .collect()
            };
            let fleet = ReportView::FleetForces;
            assert_eq!(
                names(fleet, ReportTab::CapitalShips),
                ["First", "Lost", "Second", ""]
            );
            assert_eq!(
                names(fleet, ReportTab::FighterSquadrons),
                ["Wing", "<No Casualties>"]
            );
            assert_eq!(
                names(fleet, ReportTab::Manufacturing),
                ["<None>", "<No Casualties>"]
            );
            assert_eq!(
                names(fleet, ReportTab::Personnel),
                ["Officer", "Prisoner", "<No Casualties>"]
            );
            let system = ReportView::SystemForces;
            assert_eq!(
                names(system, ReportTab::TrooperRegiments),
                ["Standing", "Fallen"]
            );
            assert_eq!(
                names(system, ReportTab::Manufacturing),
                ["Yard", "<No Casualties>"]
            );
            assert!(names(ReportView::Summary, ReportTab::CapitalShips).is_empty());
            assert!(names(ReportView::Display, ReportTab::CapitalShips).is_empty());
        }
    }

    #[test]
    fn the_tab_families_are_the_six_half_open_ranges() {
        // FUN_0044e500 cases 0x97-0x9c.
        assert_eq!(
            ReportTab::ALL.map(ReportTab::families),
            [
                0x14..0x1c,
                0x1c..0x20,
                0x28..0x30,
                0x22..0x28,
                0x10..0x14,
                0x30..0x40
            ]
        );
        assert_eq!(
            ReportTab::ALL.map(ReportTab::command),
            [0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c]
        );
    }

    #[test]
    fn a_cell_shows_the_portrait_off_the_personnel_tab_and_the_mini_on_it() {
        // FUN_0044edb0 -> FUN_0042c3b0(.., param_2 == 0, 1): the portrait
        // (mini less 0x4000) for tabs 0x97-0x9b, the mini over its backdrop
        // for 0x9c.
        let ship = crate::panels::fleets::capital_ship_mini_id(DatId::new(0x1400_0040)).unwrap();
        assert_eq!(
            cell_picture(&unit(0x1400_0040, "Ship"), ReportTab::CapitalShips),
            [(DllSource::Gokres, ship - 0x4000)]
        );
        assert_eq!(
            cell_picture(&unit(0x1000_0005, "Regiment"), ReportTab::TrooperRegiments),
            [(DllSource::Gokres, 17_476 - 0x4000)]
        );
        assert_eq!(
            cell_picture(&unit(0x3800_0340, "Minor"), ReportTab::Personnel),
            [
                (DllSource::Strategy, 11_500),
                (DllSource::Gokres, 832 + 18_176)
            ]
        );
        assert_eq!(
            cell_picture(&unit(0x3c00_0001, "Guerrillas"), ReportTab::Personnel),
            [(DllSource::Strategy, 11_500), (DllSource::Gokres, 17_728)]
        );
    }

    #[test]
    fn a_cell_stacks_its_status_marks_under_the_object_and_the_captive_mark_over_it() {
        // FUN_0042c3b0(.., arg, 1), bottom first: the backdrop (11500, a
        // regiment's portrait 11514), the en route mark (+0x50 bit 4), the
        // burn marks (bit 9, GOKRES +0x2000 on a portrait), the object, and
        // last a captive character's GOKRES (class & 0xfff) + 0x7000
        // (+0xac bit 0, FUN_0042c3b0.c:454-461).
        let none = CellStatus::default();
        let en_route = CellStatus {
            en_route: true,
            ..none
        };
        let damaged = CellStatus {
            damaged: true,
            ..none
        };
        let captive = CellStatus {
            captive: true,
            ..none
        };
        let ship = crate::panels::fleets::capital_ship_mini_id(DatId::new(0x1400_0040)).unwrap()
            - 0x4000;
        let ship_unit = unit(0x1400_0040, "Ship");
        let tab = ReportTab::CapitalShips;
        assert_eq!(cell_layers(&ship_unit, tab, none), [(DllSource::Gokres, ship)]);
        assert_eq!(
            cell_layers(&ship_unit, tab, en_route),
            [(DllSource::Gokres, ship + 0x1000), (DllSource::Gokres, ship)]
        );
        assert_eq!(
            cell_layers(&ship_unit, tab, CellStatus { en_route: true, ..damaged }),
            [
                (DllSource::Gokres, ship + 0x1000),
                (DllSource::Gokres, ship + 0x2000),
                (DllSource::Gokres, ship)
            ]
        );
        let wing = crate::panels::fleets::fighter_mini_id(DatId::new(0x1c00_0001)).unwrap() - 0x4000;
        assert_eq!(
            cell_layers(&unit(0x1c00_0001, "Wing"), ReportTab::FighterSquadrons, damaged),
            [(DllSource::Gokres, wing + 0x2000), (DllSource::Gokres, wing)]
        );

        let troops = ReportTab::TrooperRegiments;
        let regiment = unit(0x1000_0005, "Regiment");
        let portrait = 17_476 - 0x4000;
        assert_eq!(
            cell_layers(&regiment, troops, none),
            [(DllSource::Strategy, 11_514), (DllSource::Gokres, portrait)]
        );
        assert_eq!(
            cell_layers(&regiment, troops, en_route),
            [
                (DllSource::Strategy, 11_514),
                (DllSource::Strategy, 11_516),
                (DllSource::Gokres, portrait)
            ]
        );
        // TROOPSD 2 keeps its own GOKRES mark (fleet-window.md, marks).
        let army = crate::manufacturing_window::class_mini(DatId::new(0x1000_0002))
            .unwrap()
            .0
            - 0x4000;
        assert_eq!(
            cell_layers(&unit(0x1000_0002, "Army"), troops, en_route),
            [
                (DllSource::Strategy, 11_514),
                (DllSource::Gokres, army + 0x1000),
                (DllSource::Gokres, army)
            ]
        );

        let personnel = ReportTab::Personnel;
        let minor = unit(0x3800_0340, "Minor");
        let mini = 832 + 18_176;
        assert_eq!(
            cell_layers(&minor, personnel, en_route),
            [
                (DllSource::Strategy, 11_500),
                (DllSource::Strategy, 11_501),
                (DllSource::Gokres, mini)
            ]
        );
        assert_eq!(
            cell_layers(&minor, personnel, captive),
            [
                (DllSource::Strategy, 11_500),
                (DllSource::Gokres, mini),
                (DllSource::Gokres, 0x7a40)
            ]
        );
        // A special force is never a captive (the 0x30-0x3b test) and its
        // burn marks on a mini wrap to 11502, which no state here sets.
        let guerrillas = unit(0x3c00_0001, "Guerrillas");
        assert_eq!(
            cell_layers(&guerrillas, personnel, CellStatus { en_route: true, ..captive }),
            [
                (DllSource::Strategy, 11_500),
                (DllSource::Strategy, 11_501),
                (DllSource::Gokres, 17_728)
            ]
        );
    }

    #[test]
    fn a_cell_reads_its_objects_state_as_the_window_draws() {
        // FUN_0044edb0 hands FUN_0042c3b0 the listed object itself, so the
        // marks follow its +0x50 and +0xac at each refill: en route while it
        // or its fleet travels (FUN_004f8240) and not destroyed (bit 3), a
        // ship's damage (FUN_00501c00), a character's captivity.
        use rebellion_core::world::{CapitalShipClass, Character, Fleet, ShipInstance};
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            hull: 10,
            ..CapitalShipClass::default()
        });
        let ship = |tag: u32, hull: i32| {
            let mut ship = ShipInstance::new(class, 10, true);
            ship.tag = tag;
            ship.hull_current = hull;
            ship
        };
        let mut dead = ship(9, 0);
        dead.alive = false;
        let moving = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ship(7, 5), ship(8, 10), ship(0, 10), dead],
            ..Fleet::default()
        });
        let still = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ship(0, 10)],
            ..Fleet::default()
        });
        let mut movement = MovementState::new();
        assert!(movement.order(moving, system, system, 5));
        let character = |world: &mut GameWorld, value: Character| world.characters.insert(value);
        let aboard = character(
            &mut world,
            Character {
                current_fleet: Some(moving),
                ..Character::default()
            },
        );
        let travelling = character(
            &mut world,
            Character {
                en_route: true,
                ..Character::default()
            },
        );
        let killed = character(
            &mut world,
            Character {
                en_route: true,
                is_killed: true,
                ..Character::default()
            },
        );
        let held = character(
            &mut world,
            Character {
                is_captive: true,
                current_fleet: Some(still),
                ..Character::default()
            },
        );
        let force = world
            .special_forces
            .insert(rebellion_core::world::SpecialForceUnit {
                en_route: true,
                ..Default::default()
            });
        let status = |object: Option<ReportObject>| {
            cell_status(
                &world,
                &movement,
                &ReportUnit {
                    object,
                    ..unit(0x1400_0040, "Object")
                },
            )
        };
        let none = CellStatus::default();
        let en_route = CellStatus {
            en_route: true,
            ..none
        };
        assert_eq!(status(None), none);
        assert_eq!(status(Some(ReportObject::Aboard(moving))), en_route);
        assert_eq!(status(Some(ReportObject::Aboard(still))), none);
        // A tagged ship is found wherever it now is.
        let ship_in = |fleet, tag| Some(ReportObject::Ship { fleet, tag });
        assert_eq!(
            status(ship_in(still, 7)),
            CellStatus {
                damaged: true,
                ..en_route
            }
        );
        assert_eq!(status(ship_in(moving, 8)), en_route);
        assert_eq!(status(ship_in(moving, 0)), en_route);
        assert_eq!(status(ship_in(still, 0)), none);
        assert_eq!(status(ship_in(moving, 99)), none);
        assert_eq!(
            status(ship_in(moving, 9)),
            CellStatus {
                damaged: true,
                ..none
            }
        );
        assert_eq!(status(Some(ReportObject::Character(aboard))), en_route);
        assert_eq!(status(Some(ReportObject::Character(travelling))), en_route);
        assert_eq!(status(Some(ReportObject::Character(killed))), none);
        assert_eq!(
            status(Some(ReportObject::Character(held))),
            CellStatus {
                captive: true,
                ..none
            }
        );
        assert_eq!(status(Some(ReportObject::SpecialForce(force))), en_route);
    }

    #[test]
    fn a_ship_on_its_own_leg_reads_en_route_in_a_standing_fleet_for_either_side() {
        // FUN_004f8240.c:17-31: a ship's en route bit 4 is its own leg (bit
        // 5, FUN_00556430) or its container's bit 4, and FUN_0044edb0 hands
        // FUN_0042c3b0 the ship itself.
        use rebellion_core::world::{CapitalShipClass, Fleet, ShipInstance, ShipTransit};
        for is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let system = world.systems.insert(System::default());
            let class = world.capital_ship_classes.insert(CapitalShipClass {
                hull: 10,
                ..CapitalShipClass::default()
            });
            let mut travelling = ShipInstance::new(class, 10, is_alliance);
            travelling.tag = 3;
            travelling.transit = Some(ShipTransit {
                from: system,
                to: system,
                arrival_tick: 9,
            });
            let mut standing = ShipInstance::new(class, 10, is_alliance);
            standing.tag = 4;
            world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![travelling, standing],
                is_alliance,
                ..Fleet::default()
            });
            let movement = MovementState::new();
            let status = |tag| {
                cell_status(
                    &world,
                    &movement,
                    &ReportUnit {
                        object: Some(ReportObject::Ship {
                            fleet: world.fleets.keys().next().unwrap(),
                            tag,
                        }),
                        ..unit(0x1400_0040, "Object")
                    },
                )
            };
            assert!(status(3).en_route, "{is_alliance}");
            assert!(!status(4).en_route, "{is_alliance}");
        }
    }

    #[test]
    fn the_list_paints_an_en_route_ships_mark_at_its_portrait() {
        // FUN_0044edb0 centres the marked picture in its cell; the marks
        // sit at the picture's origin (FUN_005fd0f0(.., 0, 0)).
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        let fleet = world.fleets.insert(rebellion_core::world::Fleet {
            location: system,
            ..Default::default()
        });
        let mut movement = MovementState::new();
        assert!(movement.order(fleet, system, system, 5));
        let portrait =
            crate::panels::fleets::capital_ship_mini_id(DatId::new(0x1400_0040)).unwrap() - 0x4000;
        for (side, (faction, owner)) in SIDES.into_iter().enumerate() {
            let mut state = opened(faction, owner);
            if let Some(window) = state.window.as_mut() {
                let mut value = (*window.report).clone();
                value.fleet_operational[0].object = Some(ReportObject::Aboard(fleet));
                window.report = Arc::new(value);
            }
            let list = run_in(
                &world,
                &movement,
                &mut state,
                click(rail(side, ReportView::FleetForces)),
            );
            let at = |id: u32| {
                list.painted
                    .iter()
                    .find(|(painted, _)| *painted == id)
                    .map(|&(_, pos)| pos)
            };
            assert!(at(portrait).is_some(), "{faction:?}");
            assert_eq!(at(portrait + 0x1000), at(portrait), "{faction:?}");
        }
    }

    #[test]
    fn the_destroyed_column_paints_the_damage_art_a_ship_died_with() {
        // FUN_0044e500 draws a destroyed entry through the same view lookup
        // and FUN_0044edb0 as an operational one, and FUN_0042c3b0 tests
        // only the object's +0x50 bit 9, never the column. The hull loss
        // sets the bit (FUN_00501ae0 -> FUN_00501490 -> FUN_00501c00)
        // before the destroy slot +0xac (FUN_004f71d0) writes the cause, so
        // a ship the batteries killed shows its damage art under Destroyed.
        let portrait =
            crate::panels::fleets::capital_ship_mini_id(DatId::new(0x1400_0042)).unwrap() - 0x4000;
        for (side, (faction, owner)) in SIDES.into_iter().enumerate() {
            for damaged in [true, false] {
                let mut state = opened(faction, owner);
                if let Some(window) = state.window.as_mut() {
                    let mut value = (*window.report).clone();
                    value.fleet_destroyed[0].object = Some(ReportObject::Destroyed { damaged });
                    window.report = Arc::new(value);
                }
                let list = run(&mut state, click(rail(side, ReportView::FleetForces)));
                let at = |id: u32| {
                    list.painted
                        .iter()
                        .find(|(painted, _)| *painted == id)
                        .map(|&(_, pos)| pos)
                };
                assert!(at(portrait).is_some(), "{faction:?}");
                assert_eq!(
                    at(portrait + 0x2000),
                    at(portrait).filter(|_| damaged),
                    "{faction:?} damaged {damaged}"
                );
            }
        }
    }

    #[test]
    fn the_rail_art_follows_the_fleet_owner_and_the_holder_on_both_art_sets() {
        // FUN_0044c630.c:457-560: 0x66 by +0x6c, 0x67 by +0x68 (a neutral
        // holder takes 0x29f0 / 0x29fa).
        let mut alliance = report(Faction::Alliance);
        assert_eq!(ReportView::FleetForces.art(0, &alliance), (0x29ea, 0x29eb));
        assert_eq!(ReportView::FleetForces.art(1, &alliance), (0x29f4, 0x29f5));
        assert_eq!(ReportView::SystemForces.art(0, &alliance), (0x29ec, 0x29ed));
        assert_eq!(ReportView::SystemForces.art(1, &alliance), (0x29f6, 0x29f7));
        alliance.holder = Faction::Neutral;
        assert_eq!(ReportView::SystemForces.art(0, &alliance), (0x29f0, 0x29f1));
        assert_eq!(ReportView::SystemForces.art(1, &alliance), (0x29fa, 0x29fb));
        let empire = report(Faction::Empire);
        assert_eq!(ReportView::FleetForces.art(0, &empire), (0x29ec, 0x29ed));
        assert_eq!(ReportView::SystemForces.art(0, &empire), (0x29ea, 0x29eb));
        assert_eq!(
            ReportView::FleetForces.tooltip(&empire),
            Some("Imperial Forces")
        );
        assert_eq!(
            ReportView::FleetForces.tooltip(&alliance),
            Some("Alliance Forces")
        );
    }

    #[test]
    fn the_summary_paints_the_picture_under_the_shell_and_the_title_in_the_owners_colour() {
        // FUN_0044c630 layer 1: the picture (+0x2e) at (12, 13), the shell
        // 0x285f / 0x2860 at (0, 0), the rail art 0x2a44 / 0x2a45 at x 0x19c
        // / 0x19f; Close and the four rail buttons, Summary held down; the
        // title and the text in red for side 1, green for side 2.
        for (side, (faction, owner)) in SIDES.into_iter().enumerate() {
            let mut state = opened(faction, owner);
            let run = run(&mut state, hover((5.0, 5.0)));
            assert!(painted(&run, 0x2b9b, PICTURE_AT), "{faction:?}");
            assert!(painted(&run, SHELL[side], (0.0, 0.0)));
            assert!(painted(&run, RAIL_ART[side], (RAIL_X[side], 0.0)));
            assert!(painted(
                &run,
                CLOSE[side].art.0,
                (CLOSE[side].rect.0, CLOSE[side].rect.1)
            ));
            let (x, y) = RAIL_ORIGIN[side];
            assert!(
                painted(&run, [0x29e9, 0x29f3][side], (x, y)),
                "Summary down"
            );
            assert!(!run
                .painted
                .iter()
                .any(|(id, _)| ReportTab::ALL.iter().any(|tab| tab.art(side).0 == *id)));
            let color = side_color(owner);
            assert_eq!(text(&run, "Orbital bombardment of Tokmia").color, color);
            let body = "Imperial ships have conducted an orbital strike";
            for (pos, tint) in [
                (
                    egui::pos2(BODY_AT.0 + 1.0, BODY_AT.1 + 1.0),
                    egui::Color32::BLACK,
                ),
                (egui::pos2(BODY_AT.0, BODY_AT.1), color),
            ] {
                assert!(
                    run.texts.contains(&Text {
                        text: body.into(),
                        pos,
                        color: tint
                    }),
                    "{body} at {pos:?} in {tint:?}: {:?}",
                    run.texts
                );
            }
        }
    }

    #[test]
    fn a_forces_view_shows_the_tabs_its_layer_and_the_overlay_and_keeps_the_tab() {
        // FUN_0044e3d0 0x66 / 0x67: the list layer 0x2a09, the tabs with
        // 0x97 down, the headers, the view's and the tab's names
        // (FUN_0044e290); Personnel takes layer 0x2a0a and three headers.
        for (side, (faction, owner)) in SIDES.into_iter().enumerate() {
            let mut state = opened(faction, owner);
            let fleet = run(&mut state, click(rail(side, ReportView::FleetForces)));
            assert_eq!(
                state.view(),
                Some((ReportView::FleetForces, ReportTab::CapitalShips))
            );
            assert!(painted(&fleet, LAYER_TWO_COLUMN, PICTURE_AT), "{faction:?}");
            assert!(painted(
                &fleet,
                ReportTab::CapitalShips.art(side).1,
                (TABS_AT.0, TABS_AT.1)
            ));
            let forces = if owner == Faction::Alliance {
                "Alliance Forces"
            } else {
                "Imperial Forces"
            };
            assert_eq!(text(&fleet, forces).color, side_color(owner));
            assert_eq!(text(&fleet, "Capital Ships").pos, egui::pos2(38.0, 100.0));
            text(&fleet, "Operational");
            text(&fleet, "Destroyed");
            text(&fleet, "First");
            text(&fleet, "Lost");

            let personnel = run(&mut state, click(tab(ReportTab::Personnel)));
            assert_eq!(
                state.view(),
                Some((ReportView::FleetForces, ReportTab::Personnel))
            );
            assert!(painted(&personnel, LAYER_THREE_COLUMN, PICTURE_AT));
            for header in ["Survivors", "Captured", "Killed", "Officer", "Prisoner"] {
                text(&personnel, header);
            }

            let system = run(&mut state, click(rail(side, ReportView::SystemForces)));
            assert_eq!(
                state.view(),
                Some((ReportView::SystemForces, ReportTab::Personnel))
            );
            text(&system, "System Forces");
            text(&system, "None");
        }
    }

    /// TEXTSTRA 0x7013 as the shipped game has it.
    const DISPLAY_TEXT: &str =
        "Select one of the following buttons to close this display and go directly to...";

    #[test]
    fn display_offers_the_system_and_the_fleet_and_each_goes_there() {
        // FUN_0044df90 0x0b / 0x0c close the window and open the system's
        // sector window; layer 5 is 0x2a0b with text 0x7013 and the two
        // 169 by 96 buttons, the fleet's art by the player's side.
        for (side, (faction, owner)) in SIDES.into_iter().enumerate() {
            let mut state = opened(faction, owner);
            let display = run(&mut state, click(rail(side, ReportView::Display)));
            assert!(painted(&display, LAYER_DISPLAY, PICTURE_AT));
            assert!(painted(&display, GOTO_SYSTEM_ART, (29.0, 182.0)));
            assert!(painted(&display, GOTO_FLEET_ART[side], (226.0, 182.0)));
            text(&display, "Tokmia");
            text(&display, "Fleet");
            text(&display, DISPLAY_TEXT);
            let system = state.window.as_ref().unwrap().report.system;
            let went = run(&mut state, click((100.0, 230.0)));
            assert_eq!(went.actions, [OperationReportAction::GotoSystem { system }]);
            assert!(!state.is_open());

            let mut state = opened(faction, owner);
            run(&mut state, click(rail(side, ReportView::Display)));
            let went = run(&mut state, click((300.0, 230.0)));
            assert_eq!(
                went.actions,
                [OperationReportAction::GotoFleet { system }],
                "{faction:?}"
            );
        }
    }

    #[test]
    fn a_display_caption_is_centred_down_its_button_and_wraps_to_its_width() {
        // FUN_006026e0 centres the DT_CALCRECT text rect in the 169 by 96
        // button ((h - text h) / 2, word-wrapped), then FUN_006027e0(btn, 0,
        // h / 2 - 12) moves it down 36; a font-4 line is 16 pixels.
        for (side, (faction, owner)) in SIDES.into_iter().enumerate() {
            let mut state = opened(faction, owner);
            if let Some(window) = state.window.as_mut() {
                window.system_caption = "Outer Rim Territories Shipping Lanes Beyond".into();
            }
            let display = run(&mut state, click(rail(side, ReportView::Display)));
            let rows = |caption: &str| {
                display
                    .rows
                    .iter()
                    .find(|(text, _)| text == caption)
                    .map_or(0, |&(_, rows)| rows)
            };
            assert_eq!(rows("Fleet"), 1);
            assert_eq!(text(&display, "Fleet").pos.y, 182.0 + 40.0 + 36.0, "{faction:?}");
            let caption = "Outer Rim Territories Shipping Lanes Beyond";
            let system = text(&display, caption);
            assert!(rows(caption) > 1, "{faction:?}");
            let lines = 16 * rows(caption) as i32;
            assert_eq!(
                system.pos.y,
                182.0 + ((96 - lines) / 2 + 36) as f32,
                "{faction:?}"
            );
        }
    }

    #[test]
    fn close_and_escape_go_back_to_the_index_the_report_came_from() {
        // FUN_0044df90 case 10 and FUN_0044e180 key 0x1b: slot +0x30, then
        // FUN_00469d60 posts the Index back on its route.
        for (side, (faction, owner)) in SIDES.into_iter().enumerate() {
            let mut state = opened(faction, owner);
            let close = CLOSE[side].rect;
            let closed = run(
                &mut state,
                click((close.0 + close.2 / 2.0, close.1 + close.3 / 2.0)),
            );
            assert_eq!(
                closed.actions,
                [OperationReportAction::Closed {
                    reopen_index: Some(0x80)
                }]
            );
            assert!(!state.is_open());
            let mut state = opened(faction, owner);
            let escaped = run(&mut state, key(egui::Key::Escape));
            assert_eq!(
                escaped.actions,
                [OperationReportAction::Closed {
                    reopen_index: Some(0x80)
                }],
                "{faction:?}"
            );
        }
    }

    #[test]
    fn left_and_right_step_the_tabs_over_a_list_and_wrap_to_the_first() {
        // FUN_0044e180 keys 0x25 / 0x27: the neighbouring visible tab;
        // Right past the last and Left at the first select the first.
        assert_eq!(ReportTab::Personnel.stepped(true), ReportTab::CapitalShips);
        assert_eq!(
            ReportTab::CapitalShips.stepped(false),
            ReportTab::CapitalShips
        );
        assert_eq!(
            ReportTab::Manufacturing.stepped(false),
            ReportTab::FighterSquadrons
        );
        for (side, (faction, owner)) in SIDES.into_iter().enumerate() {
            let mut state = opened(faction, owner);
            run(&mut state, key(egui::Key::ArrowRight));
            assert_eq!(
                state.view(),
                Some((ReportView::Summary, ReportTab::CapitalShips))
            );
            run(&mut state, click(rail(side, ReportView::SystemForces)));
            run(&mut state, key(egui::Key::ArrowRight));
            assert_eq!(
                state.view(),
                Some((ReportView::SystemForces, ReportTab::FighterSquadrons)),
                "{faction:?}"
            );
        }
    }

    #[test]
    fn a_report_opens_on_its_first_tab_and_a_second_waits_for_the_first_to_close() {
        // FUN_0044c630.c:921-923: +0x180 = 0x97, or 0x9b for an assault.
        let world = GameWorld::default();
        for (faction, owner) in SIDES {
            let mut assault = report(owner);
            assault.assault = true;
            let message = |report: OperationReport, id| {
                let mut message = GameMessage::new(1, "Report", MessageCategory::Combat)
                    .with_report(Arc::new(report));
                message.id = id;
                message
            };
            let mut state = OperationReportState::default();
            assert!(!state.open(
                &world,
                &GameMessage::new(1, "Plain", MessageCategory::Combat),
                faction,
                None
            ));
            assert!(state.open(&world, &message(report(owner), 1), faction, None));
            assert!(state.open(&world, &message(assault, 2), faction, None));
            assert_eq!(state.message(), Some(1));
            assert_eq!(
                state.view(),
                Some((ReportView::Summary, ReportTab::CapitalShips))
            );
            state.close();
            assert_eq!(state.message(), Some(2));
            assert_eq!(
                state.view(),
                Some((ReportView::Summary, ReportTab::TrooperRegiments))
            );
            state.close();
            assert!(!state.is_open());
        }
    }
}
