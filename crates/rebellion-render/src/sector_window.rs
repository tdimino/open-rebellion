//! Original strategic sector windows.
//!
//! `REBEXE.EXE` constructs these as modeless 235 by 360 child windows. The
//! window body is data-driven: each system uses its `SYSTEMSD.DAT` planet
//! picture, sector-relative coordinates, and three compact status tracks.
//! The close and side-switch controls use their original STRATEGY resources.
//! A system with fleets shows the fleet icon at its planet's top right; a
//! double click on it opens the Fleet window (`ghidra/notes/fleet-window.md`).

use egui_macroquad::egui;
use rebellion_core::dat::Faction;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{DatId, SectorKey, SystemKey};
use rebellion_core::world::{ControlKind, GameWorld};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::paint_native;
use crate::object_menu::MenuObject;
use crate::quadrant_icons::{quadrant_icon, Quadrant};

/// `DAT_00658bd8`, a static 1023: the galaxy's width, whose half
/// `FUN_00429ce0` compares a sector's x with (`FUN_00526560`, the record's
/// `+0x48`). hyp: that x is `Sector.x`, SECTORSD's map position.
const GALAXY_WIDTH: i32 = 1023;

pub const SECTOR_WINDOW_WIDTH: f32 = 235.0;
pub const SECTOR_WINDOW_HEIGHT: f32 = 360.0;

const CLOSE_NORMAL: u32 = 10108;
const CLOSE_PRESSED: u32 = 10109;
const SWITCH_SIDE_NORMAL: u32 = 10210;
const SWITCH_SIDE_PRESSED: u32 = 10211;

const BORDER_TOP_LEFT: u32 = 10100;
const BORDER_TOP_RIGHT: u32 = 10101;
const BORDER_BOTTOM_LEFT: u32 = 10102;
const BORDER_BOTTOM_RIGHT: u32 = 10103;
const BORDER_TOP: u32 = 10104;
const BORDER_LEFT: u32 = 10105;
const BORDER_RIGHT: u32 = 10106;
const BORDER_BOTTOM: u32 = 10107;

/// `SYSTEMSD.DAT` records 100 through 299 are contiguous. This table preserves
/// their original `picture_id` values without adding presentation-only state
/// to the serialized simulation world.
const SYSTEM_PLANET_PICTURES: [u8; 200] = [
    1, 2, 7, 8, 3, 4, 9, 10, 11, 12, 5, 13, 11, 14, 15, 8, 7, 11, 6, 7, 8, 7, 1, 13, 2, 16, 17, 3,
    4, 18, 9, 13, 15, 7, 14, 18, 19, 20, 8, 13, 5, 6, 1, 2, 3, 7, 4, 5, 6, 1, 2, 3, 11, 4, 11, 5,
    6, 1, 2, 3, 9, 4, 5, 11, 19, 6, 1, 13, 19, 11, 7, 2, 13, 3, 13, 8, 4, 5, 6, 1, 1, 8, 2, 3, 4,
    5, 11, 19, 11, 6, 13, 1, 2, 20, 21, 8, 7, 13, 11, 8, 11, 13, 3, 13, 11, 15, 7, 4, 5, 19, 10,
    20, 20, 11, 19, 6, 9, 1, 20, 2, 19, 14, 22, 9, 11, 13, 3, 11, 8, 18, 4, 20, 15, 8, 13, 8, 8,
    19, 13, 8, 5, 19, 13, 6, 1, 13, 20, 8, 19, 9, 11, 13, 2, 3, 13, 18, 4, 5, 7, 11, 19, 11, 6, 13,
    1, 23, 2, 3, 13, 4, 5, 24, 7, 6, 1, 8, 13, 2, 14, 25, 19, 3, 26, 13, 20, 4, 11, 19, 5, 20, 9,
    20, 9, 9, 19, 6, 8, 1, 19, 8,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WindowColumn {
    Primary,
    Secondary,
}

impl WindowColumn {
    fn opposite(self) -> Self {
        match self {
            Self::Primary => Self::Secondary,
            Self::Secondary => Self::Primary,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OpenSectorWindow {
    sector: SectorKey,
    column: WindowColumn,
    /// The icon a left or right press chose last (`+0x184`,
    /// `FUN_0045b1b0`): its system and quadrant.
    selection: Option<(SystemKey, Quadrant)>,
    /// Where a left press on an icon began a drag, as a canvas point
    /// (`+0x14c`/`+0x150`, set by `FUN_004593e0` case `0x201`).
    drag_from: Option<(i16, i16)>,
}

/// Mutable state for the modeless sector-window stack. The last entry is the
/// focused window and therefore paints above earlier entries.
#[derive(Debug)]
pub struct SectorWindowState {
    faction: CockpitFaction,
    windows: Vec<OpenSectorWindow>,
}

impl Default for SectorWindowState {
    fn default() -> Self {
        Self {
            faction: CockpitFaction::Alliance,
            windows: Vec::new(),
        }
    }
}

impl SectorWindowState {
    /// Open the selected system's parent sector (`FUN_00429ce0`;
    /// `ghidra/notes/sector-window-placement.md`). An open window raises
    /// rather than duplicate. At most two are open, one per column: a sector
    /// on the galaxy's right half takes the second column when both are
    /// free, the free column otherwise, and with both taken it replaces the
    /// window in its own half's column.
    pub fn open_for_system(
        &mut self,
        world: &GameWorld,
        system: SystemKey,
        faction: CockpitFaction,
    ) -> bool {
        self.prepare_faction(faction);
        let Some(sector) = world.systems.get(system).map(|system| system.sector) else {
            return false;
        };
        if self.focus(sector) {
            return true;
        }
        let right_half = world
            .sectors
            .get(sector)
            .is_some_and(|value| i32::from(value.x) >= GALAXY_WIDTH / 2);
        let half = if right_half {
            WindowColumn::Secondary
        } else {
            WindowColumn::Primary
        };
        let taken = |column| self.windows.iter().any(|window| window.column == column);
        let column = match (taken(WindowColumn::Primary), taken(WindowColumn::Secondary)) {
            (false, false) | (true, true) => half,
            (true, false) => WindowColumn::Secondary,
            (false, true) => WindowColumn::Primary,
        };
        // FUN_00600f90 destroys the window the new one replaces.
        self.windows.retain(|window| window.column != column);
        self.windows.push(OpenSectorWindow {
            sector,
            column,
            selection: None,
            drag_from: None,
        });
        true
    }

    /// True when the pointer lies inside any visible window. Logical right and
    /// bottom edges remain exclusive, as in the recovered Win32 rectangles.
    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.windows.iter().any(|window| {
            let rect = window_screen_rect(self.faction, window.column, layout);
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

    /// Where `system`'s planet is drawn, in screen pixels, when its sector's
    /// window is open.
    #[must_use]
    pub fn planet_screen_rect(
        &self,
        world: &GameWorld,
        layout: CockpitLayout,
        system: SystemKey,
    ) -> Option<egui::Rect> {
        let value = world.systems.get(system)?;
        let window = self
            .windows
            .iter()
            .find(|window| window.sector == value.sector)?;
        let sector = world.sectors.get(window.sector)?;
        let (x, y) = sector_planet_position(sector.x, sector.y, value.x, value.y);
        let window_rect = window_screen_rect(self.faction, window.column, layout);
        Some(planet_rect(window_rect, layout.scale, x, y))
    }

    /// Where `system`'s top-right quadrant overlay lies, in screen pixels,
    /// when its sector's window is open: 28 by 19 from one pixel right of
    /// the planet's center to its center row (`FUN_00459e30`, flag
    /// `0x100000`).
    #[must_use]
    pub fn fleet_icon_screen_rect(
        &self,
        world: &GameWorld,
        layout: CockpitLayout,
        system: SystemKey,
    ) -> Option<egui::Rect> {
        self.quadrant_screen_rect(world, layout, system, Quadrant::Fleets)
    }

    /// The logical point `system`'s Fleet window opens at from its open
    /// sector window: the fleet icon's (`FUN_0045c8e0` →
    /// `FUN_0045aac0(.., item +0x40, +0x44)`). port: the icon's center, as
    /// an icon opened without a double-click point.
    #[must_use]
    pub fn fleet_window_point(
        &self,
        world: &GameWorld,
        layout: CockpitLayout,
        system: SystemKey,
    ) -> Option<(i16, i16)> {
        self.quadrant_window_point(world, layout, system, Quadrant::Fleets)
    }

    /// The logical point the window behind one of `system`'s quadrant icons
    /// opens at, as [`fleet_window_point`](Self::fleet_window_point) does
    /// for the Fleet window: the icon's center.
    #[must_use]
    pub fn quadrant_window_point(
        &self,
        world: &GameWorld,
        layout: CockpitLayout,
        system: SystemKey,
        quadrant: Quadrant,
    ) -> Option<(i16, i16)> {
        let icon = self.quadrant_screen_rect(world, layout, system, quadrant)?;
        Some(screen_to_logical(layout, icon.center()))
    }

    /// Where one of `system`'s four quadrant overlays lies, in screen
    /// pixels, when its sector's window is open (`FUN_00459e30`).
    #[must_use]
    pub fn quadrant_screen_rect(
        &self,
        world: &GameWorld,
        layout: CockpitLayout,
        system: SystemKey,
        quadrant: Quadrant,
    ) -> Option<egui::Rect> {
        let planet = self.planet_screen_rect(world, layout, system)?;
        Some(quadrant_rect(planet, layout.scale, quadrant))
    }

    /// The destination a targeting release at `point` takes from the sector
    /// window egui draws as `layer`, or `None` when `layer` is none of them.
    ///
    /// The window's `+0x70` (`FUN_0045c830` → `FUN_0045c660`) gives the first
    /// planet whose rectangle holds the point, and no system between planets.
    /// `FUN_00459e30` sets each rectangle once to the 37 by 37 planet bitmap
    /// at its position (`ghidra/notes/sector-window-hit-test.md`). hyp: the
    /// items are listed in the sector's system order.
    #[must_use]
    pub fn release_target(
        &self,
        world: &GameWorld,
        layout: CockpitLayout,
        layer: egui::LayerId,
        point: egui::Pos2,
    ) -> Option<Option<SystemKey>> {
        let window = self
            .windows
            .iter()
            .find(|window| area_id(window.sector) == layer.id)?;
        let Some(sector) = world.sectors.get(window.sector) else {
            return Some(None);
        };
        let window_rect = window_screen_rect(self.faction, window.column, layout);
        Some(sector.systems.iter().copied().find(|key| {
            world.systems.get(*key).is_some_and(|system| {
                let (x, y) = sector_planet_position(sector.x, sector.y, system.x, system.y);
                rect_contains(planet_rect(window_rect, layout.scale, x, y), point)
            })
        }))
    }

    pub fn clear(&mut self) {
        self.windows.clear();
    }

    fn prepare_faction(&mut self, faction: CockpitFaction) {
        if self.faction != faction {
            self.faction = faction;
            self.windows.clear();
        }
    }

    fn focus(&mut self, sector: SectorKey) -> bool {
        let Some(index) = self
            .windows
            .iter()
            .position(|window| window.sector == sector)
        else {
            return false;
        };
        let window = self.windows.remove(index);
        self.windows.push(window);
        true
    }

    fn close(&mut self, sector: SectorKey) {
        self.windows.retain(|window| window.sector != sector);
    }

    fn switch_side(&mut self, sector: SectorKey) {
        if let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.sector == sector)
        {
            window.column = window.column.opposite();
        }
        self.focus(sector);
    }
}

/// Actions that leave the sector-window manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectorWindowAction {
    SelectSystem(SystemKey),
    OpenSystemWindow {
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A double click on a shown fleet icon (`FUN_004593e0` case `0x203` →
    /// `FUN_0045aac0`, kind `0x10`).
    OpenFleetWindow {
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A double click on a shown defenses icon (`FUN_0045aac0`, kind 8):
    /// the System Defenses window, type 10.
    OpenDefensesWindow {
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A double click on a shown missions icon (`FUN_0045aac0`, kind
    /// `0x40`): the Missions window, type 11.
    OpenMissionsWindow {
        system: SystemKey,
        logical_position: (i16, i16),
    },
    /// A right-button release opens the object pop-up menu (slot 7,
    /// `FUN_004ac5c0`) for the window's selection, at a 640 by 480 canvas
    /// point.
    OpenObjectMenu {
        selection: Option<MenuObject>,
        point: (i16, i16),
    },
    /// A left release more than four pixels from the press that chose an
    /// icon (`FUN_004593e0` case `0x202`) posts `0x29a`: the galaxy view
    /// moves the selection to what lies under the screen `point`, as a
    /// Confirmed Move when Ctrl is held (`FUN_00422ce0`).
    DropSelection {
        selection: MenuObject,
        point: egui::Pos2,
        confirmed: bool,
    },
}

#[derive(Default)]
struct WindowDrawResult {
    focus: bool,
    close: bool,
    switch_side: bool,
    selected: Option<SystemKey>,
    opened: Option<(SystemKey, (i16, i16))>,
    opened_fleet: Option<(SystemKey, (i16, i16))>,
    opened_defenses: Option<(SystemKey, (i16, i16))>,
    opened_missions: Option<(SystemKey, (i16, i16))>,
    /// A left or right press on a shown icon.
    icon_pressed: Option<(SystemKey, Quadrant)>,
    /// A right-button release in the window, as a canvas point.
    object_menu: Option<(i16, i16)>,
    /// A left press on a shown icon, as a canvas point.
    drag_from: Option<(i16, i16)>,
    /// A left release ended the window's drag: where, and whether Ctrl was
    /// held, when it went far enough to drop.
    drag_end: Option<Option<(egui::Pos2, bool)>>,
}

/// Paint and operate all open sector windows using the recovered strategic
/// canvas. Window mutations are applied after the pass so area ordering stays
/// deterministic.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit state and rendering inputs at this existing UI boundary."
)]
pub fn draw_sector_windows(
    ctx: &egui::Context,
    world: &GameWorld,
    movement: &rebellion_core::movement::MovementState,
    fog: &FogState,
    state: &mut SectorWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    uprisings: &rebellion_core::uprising::UprisingState,
    missions: &rebellion_core::missions::MissionState,
) -> Vec<SectorWindowAction> {
    state.prepare_faction(faction);
    let windows = state.windows.clone();
    let mut actions = Vec::new();
    let mut focused = None;
    let mut closed = None;
    let mut switched = None;
    let mut chosen = Vec::new();

    let focused_sector = windows.last().map(|window| window.sector);
    for window in windows {
        let result = draw_sector_window(
            ctx,
            world,
            movement,
            fog,
            window,
            focused_sector == Some(window.sector),
            faction,
            layout,
            cache,
            uprisings,
            missions,
        );
        if result.focus {
            focused = Some(window.sector);
        }
        if result.close {
            closed = Some(window.sector);
        }
        if result.switch_side {
            switched = Some(window.sector);
        }
        if let Some(system) = result.selected {
            actions.push(SectorWindowAction::SelectSystem(system));
        }
        if let Some((system, logical_position)) = result.opened {
            actions.push(SectorWindowAction::OpenSystemWindow {
                system,
                logical_position,
            });
        }
        if let Some((system, logical_position)) = result.opened_fleet {
            actions.push(SectorWindowAction::OpenFleetWindow {
                system,
                logical_position,
            });
        }
        if let Some((system, logical_position)) = result.opened_defenses {
            actions.push(SectorWindowAction::OpenDefensesWindow {
                system,
                logical_position,
            });
        }
        if let Some((system, logical_position)) = result.opened_missions {
            actions.push(SectorWindowAction::OpenMissionsWindow {
                system,
                logical_position,
            });
        }
        let selection = result.icon_pressed.or(window.selection);
        // A press on an icon sets the selection and the drag; a release
        // ends the drag.
        if result.icon_pressed.is_some() || result.drag_end.is_some() {
            chosen.push((window.sector, selection, result.drag_from));
        }
        if let (Some(Some((point, confirmed))), Some((system, quadrant))) =
            (result.drag_end, selection)
        {
            actions.push(SectorWindowAction::DropSelection {
                selection: MenuObject::SystemIcon { system, quadrant },
                point,
                confirmed,
            });
        }
        if let Some(point) = result.object_menu {
            actions.push(SectorWindowAction::OpenObjectMenu {
                selection: selection
                    .map(|(system, quadrant)| MenuObject::SystemIcon { system, quadrant }),
                point,
            });
        }
    }

    for (sector, selection, drag_from) in chosen {
        if let Some(window) = state
            .windows
            .iter_mut()
            .find(|window| window.sector == sector)
        {
            window.selection = selection;
            window.drag_from = drag_from;
        }
    }

    if let Some(sector) = closed {
        state.close(sector);
    } else if let Some(sector) = switched {
        state.switch_side(sector);
    } else if let Some(sector) = focused {
        state.focus(sector);
    }
    actions
}

#[expect(
    clippy::too_many_lines,
    clippy::too_many_arguments,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
fn draw_sector_window(
    ctx: &egui::Context,
    world: &GameWorld,
    movement: &rebellion_core::movement::MovementState,
    fog: &FogState,
    window: OpenSectorWindow,
    focused: bool,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    uprisings: &rebellion_core::uprising::UprisingState,
    missions: &rebellion_core::missions::MissionState,
) -> WindowDrawResult {
    let mut result = WindowDrawResult::default();
    let Some(sector) = world.sectors.get(window.sector) else {
        result.close = true;
        return result;
    };
    let position = window_logical_position(faction, window.column);
    let screen_position = egui::pos2(
        layout.canvas.x + position.0 * layout.scale,
        layout.canvas.y + position.1 * layout.scale,
    );
    let size = egui::vec2(
        SECTOR_WINDOW_WIDTH * layout.scale,
        SECTOR_WINDOW_HEIGHT * layout.scale,
    );

    let area_id = area_id(window.sector);
    if focused {
        ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, area_id));
    }
    let area = egui::Area::new(area_id)
        .fixed_pos(screen_position)
        .order(egui::Order::Middle)
        .show(ctx, |ui| {
            let (pointer, primary_down, primary_pressed, any_pressed, released, ctrl) =
                ctx.input(|input| {
                    (
                        input.pointer.interact_pos(),
                        input.pointer.button_down(egui::PointerButton::Primary),
                        input.pointer.button_pressed(egui::PointerButton::Primary),
                        input.pointer.button_pressed(egui::PointerButton::Primary)
                            || input.pointer.button_pressed(egui::PointerButton::Secondary),
                        (
                            input.pointer.button_released(egui::PointerButton::Primary),
                            input
                                .pointer
                                .button_released(egui::PointerButton::Secondary),
                        ),
                        input.modifiers.ctrl,
                    )
                });
            let (primary_released, secondary_released) = released;
            let (window_rect, window_response) = ui.allocate_exact_size(size, egui::Sense::click());
            ui.painter()
                .rect_filled(window_rect, 0.0, egui::Color32::from_rgb(42, 42, 42));
            paint_window_border(ui.painter(), ctx, cache, window_rect, layout.scale);

            let title_color = sector_title_color(world, window.sector, faction);
            ui.painter().text(
                logical_point(window_rect, layout.scale, 117.5, 2.0),
                egui::Align2::CENTER_TOP,
                &sector.name,
                egui::FontId::proportional((13.0 * layout.scale).max(8.0)),
                title_color,
            );

            let switch_rect = logical_rect(window_rect, layout.scale, 204.0, 2.0, 14.0, 14.0);
            let close_rect = logical_rect(window_rect, layout.scale, 218.0, 2.0, 14.0, 14.0);
            let switch_response = ui.interact(
                switch_rect,
                ui.id().with((window.sector, "switch")),
                egui::Sense::click(),
            );
            let close_response = ui.interact(
                close_rect,
                ui.id().with((window.sector, "close")),
                egui::Sense::click(),
            );
            let switch_pressed =
                primary_down && pointer.is_some_and(|point| rect_contains(switch_rect, point));
            let close_pressed =
                primary_down && pointer.is_some_and(|point| rect_contains(close_rect, point));
            let switch_clicked = exact_clicked(&switch_response, switch_rect);
            let close_clicked = exact_clicked(&close_response, close_rect);
            paint_resource(
                ui.painter(),
                ctx,
                cache,
                if switch_pressed {
                    SWITCH_SIDE_PRESSED
                } else {
                    SWITCH_SIDE_NORMAL
                },
                switch_rect,
            );
            paint_resource(
                ui.painter(),
                ctx,
                cache,
                if close_pressed {
                    CLOSE_PRESSED
                } else {
                    CLOSE_NORMAL
                },
                close_rect,
            );

            let player = cockpit_faction(faction);
            let layer = egui::LayerId::new(egui::Order::Middle, area_id);
            let double_click = ctx
                .input(|input| {
                    input
                        .pointer
                        .button_double_clicked(egui::PointerButton::Primary)
                })
                .then_some(pointer)
                .flatten()
                .filter(|point| ctx.layer_id_at(*point) == Some(layer));
            // The window's area is exactly its rect, so its layer under the
            // point also says no other window covers it.
            let on_window = |point: &egui::Pos2| ctx.layer_id_at(*point) == Some(layer);
            // FUN_004593e0 cases 0x201 and 0x204 hit-test the overlays
            // (FUN_0045cc10); a press that finds none keeps the selection.
            let press = any_pressed.then_some(pointer).flatten().filter(on_window);
            for system_key in &sector.systems {
                let Some(system) = world.systems.get(*system_key) else {
                    continue;
                };
                let (planet_x, planet_y) =
                    sector_planet_position(sector.x, sector.y, system.x, system.y);
                let planet_rect = planet_rect(window_rect, layout.scale, planet_x, planet_y);
                let planet_response = ui.interact(
                    planet_rect,
                    ui.id().with((window.sector, *system_key)),
                    egui::Sense::click(),
                );
                let planet_hovered = pointer.is_some_and(|point| rect_contains(planet_rect, point));
                let planet_clicked = exact_clicked(&planet_response, planet_rect);
                // FUN_0045d140 draws an overlay only while its rule has
                // something to show; a double click finds it first
                // (`FUN_0045cc10`).
                let icons: Vec<(Quadrant, egui::Rect, u32)> = Quadrant::ALL
                    .into_iter()
                    .filter_map(|quadrant| {
                        let (normal, _) = quadrant_icon(
                            world,
                            fog,
                            missions,
                            movement,
                            player,
                            *system_key,
                            quadrant,
                        )?;
                        Some((
                            quadrant,
                            quadrant_rect(planet_rect, layout.scale, quadrant),
                            normal,
                        ))
                    })
                    .collect();
                let icon_double_clicked = double_click.and_then(|point| {
                    icons
                        .iter()
                        .find(|(_, rect, _)| rect_contains(*rect, point))
                        .map(|(quadrant, rect, _)| (*quadrant, *rect))
                });
                if let Some((quadrant, _, _)) = press.and_then(|point| {
                    icons
                        .iter()
                        .find(|(_, rect, _)| rect_contains(*rect, point))
                }) {
                    result.icon_pressed = Some((*system_key, *quadrant));
                    if primary_pressed {
                        result.drag_from = press.map(|point| screen_to_logical(layout, point));
                    }
                }
                let planet_double_clicked = planet_response.double_clicked()
                    && planet_clicked
                    && icon_double_clicked.is_none();
                paint_resource(
                    ui.painter(),
                    ctx,
                    cache,
                    planet_resource_id(system.dat_id),
                    planet_rect,
                );
                paint_status_tracks(
                    ui.painter(),
                    window_rect,
                    layout.scale,
                    planet_x,
                    planet_y,
                    system.popularity_alliance,
                    system.popularity_empire,
                    system.total_energy,
                    system.raw_materials,
                );
                ui.painter().text(
                    logical_point(window_rect, layout.scale, planet_x + 18.5, planet_y + 37.0),
                    egui::Align2::CENTER_TOP,
                    &system.name,
                    egui::FontId::proportional((10.0 * layout.scale).max(7.0)),
                    system_name_color(system.control, uprisings.is_uprising(*system_key), player),
                );

                for (_, icon_rect, normal) in &icons {
                    // port: the second bitmap (hyp: the pressed state) is not
                    // drawn.
                    paint_native(
                        ui.painter(),
                        ctx,
                        cache,
                        DllSource::Strategy,
                        *normal,
                        *icon_rect,
                        layout.scale,
                        0.0,
                        0.0,
                    );
                }
                if let Some((quadrant, icon_rect)) = icon_double_clicked {
                    let open_point = double_click.unwrap_or_else(|| icon_rect.center());
                    let opened = Some((*system_key, screen_to_logical(layout, open_point)));
                    // FUN_0045aac0: kind 4 opens the System window, kind 8
                    // the Defenses window, kind 0x10 the Fleet window and
                    // kind 0x40 the Missions window.
                    match quadrant {
                        Quadrant::System => result.opened = opened,
                        Quadrant::Defenses => result.opened_defenses = opened,
                        Quadrant::Fleets => result.opened_fleet = opened,
                        Quadrant::Missions => result.opened_missions = opened,
                    }
                    result.focus = true;
                }

                if planet_hovered || planet_clicked {
                    paint_selection_brackets(ui.painter(), planet_rect, layout.scale);
                }
                if planet_clicked {
                    result.selected = Some(*system_key);
                    result.focus = true;
                }
                if planet_double_clicked {
                    let open_point = pointer.unwrap_or_else(|| planet_rect.center());
                    result.opened = Some((*system_key, screen_to_logical(layout, open_point)));
                }
            }

            // FUN_004593e0 case 0x202: the window holds the capture, so the
            // release ends the drag wherever it lands; it drops only past
            // four pixels from the press in x or y.
            if let (true, Some(from)) = (primary_released, window.drag_from) {
                result.drag_end = Some(pointer.and_then(|point| {
                    let to = screen_to_logical(layout, point);
                    ((to.0 - from.0).abs() > 4 || (to.1 - from.1).abs() > 4)
                        .then_some((point, ctrl))
                }));
            }
            // WM_RBUTTONUP: slot 7, FUN_004ac5c0, opens the menu for the
            // window's selection, whatever lies under the cursor.
            if let Some(point) = secondary_released
                .then_some(pointer)
                .flatten()
                .filter(on_window)
            {
                result.object_menu = Some(screen_to_logical(layout, point));
                result.focus = true;
            }
            if window_response.clicked() || switch_clicked || close_clicked {
                result.focus = true;
            }
            result.switch_side = switch_clicked;
            result.close = close_clicked;
        });

    if area.response.clicked() {
        result.focus = true;
    }
    result
}

fn window_logical_position(faction: CockpitFaction, column: WindowColumn) -> (f32, f32) {
    match (faction, column) {
        (CockpitFaction::Alliance, WindowColumn::Primary) => (60.0, 35.0),
        (CockpitFaction::Alliance, WindowColumn::Secondary) => (300.0, 35.0),
        (CockpitFaction::Empire, WindowColumn::Primary) => (120.0, 40.0),
        (CockpitFaction::Empire, WindowColumn::Secondary) => (365.0, 40.0),
    }
}

fn area_id(sector: SectorKey) -> egui::Id {
    egui::Id::new(("original-sector-window", sector))
}

/// A planet item's picture and hit rectangle: 37 by 37 at its position.
fn planet_rect(window_rect: egui::Rect, scale: f32, x: f32, y: f32) -> egui::Rect {
    logical_rect(window_rect, scale, x, y, 37.0, 37.0)
}

/// A quadrant overlay around a planet whose center is (x + 18, y + 18),
/// 28 by 19, the 27 by 18 icon 10771 plus one (`FUN_00459e30:369-447`):
/// the left ones end at `cx`, the right ones start at `cx + 1`; the top ones
/// end at `cy`, the bottom ones start at `cy + 1`.
fn quadrant_rect(planet: egui::Rect, scale: f32, quadrant: Quadrant) -> egui::Rect {
    let (x, y) = match quadrant {
        Quadrant::System => (-10.0, -1.0),
        Quadrant::Defenses => (-10.0, 19.0),
        Quadrant::Fleets => (19.0, -1.0),
        Quadrant::Missions => (19.0, 19.0),
    };
    egui::Rect::from_min_size(
        planet.min + egui::vec2(x, y) * scale,
        egui::vec2(28.0, 19.0) * scale,
    )
}

fn window_screen_rect(
    faction: CockpitFaction,
    column: WindowColumn,
    layout: CockpitLayout,
) -> egui::Rect {
    let position = window_logical_position(faction, column);
    egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + position.0 * layout.scale,
            layout.canvas.y + position.1 * layout.scale,
        ),
        egui::vec2(
            SECTOR_WINDOW_WIDTH * layout.scale,
            SECTOR_WINDOW_HEIGHT * layout.scale,
        ),
    )
}

fn logical_rect(
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

#[expect(
    clippy::cast_possible_truncation,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
fn screen_to_logical(layout: CockpitLayout, point: egui::Pos2) -> (i16, i16) {
    let scale = layout.scale.max(f32::EPSILON);
    (
        ((point.x - layout.canvas.x) / scale).round() as i16,
        ((point.y - layout.canvas.y) / scale).round() as i16,
    )
}

fn exact_clicked(response: &egui::Response, rect: egui::Rect) -> bool {
    response.clicked()
        && response
            .interact_pointer_pos()
            .is_some_and(|point| rect_contains(rect, point))
}

fn rect_contains(rect: egui::Rect, point: egui::Pos2) -> bool {
    point.x >= rect.min.x && point.x < rect.max.x && point.y >= rect.min.y && point.y < rect.max.y
}

fn sector_planet_position(
    sector_x: u16,
    sector_y: u16,
    system_x: u16,
    system_y: u16,
) -> (f32, f32) {
    let relative_x = f32::from(system_x.saturating_sub(sector_x));
    let relative_y = f32::from(system_y.saturating_sub(sector_y));
    (
        (relative_x / 13.0 * 37.0).round(),
        (relative_y / 10.0 * 37.0).round(),
    )
}

pub(crate) fn planet_picture_id(dat_id: DatId) -> u8 {
    dat_id
        .index()
        .checked_sub(100)
        .and_then(|index| SYSTEM_PLANET_PICTURES.get(index as usize))
        .copied()
        .unwrap_or(1)
}

pub(crate) fn planet_resource_id(dat_id: DatId) -> u32 {
    let picture = planet_picture_id(dat_id);
    match picture {
        1..=23 => 10211 + u32::from(picture),
        24 => 10239,
        25 => 10237,
        26 => 10238,
        _ => 10212,
    }
}

fn cockpit_faction(faction: CockpitFaction) -> Faction {
    match faction {
        CockpitFaction::Alliance => Faction::Alliance,
        CockpitFaction::Empire => Faction::Empire,
    }
}

fn system_name_color(control: ControlKind, in_revolt: bool, player: Faction) -> egui::Color32 {
    let control = match control {
        ControlKind::Controlled(owner) if in_revolt => ControlKind::Uprising(owner),
        other => other,
    };
    match control {
        ControlKind::Controlled(owner) if owner == player => egui::Color32::from_rgb(0, 255, 64),
        ControlKind::Uprising(owner) if owner == player => egui::Color32::from_rgb(255, 230, 0),
        ControlKind::Controlled(_) | ControlKind::Uprising(_) => {
            egui::Color32::from_rgb(255, 32, 32)
        }
        ControlKind::Contested => egui::Color32::from_rgb(255, 230, 0),
        ControlKind::Uncontrolled => egui::Color32::from_rgb(0, 255, 255),
    }
}

fn sector_title_color(
    world: &GameWorld,
    sector_key: SectorKey,
    faction: CockpitFaction,
) -> egui::Color32 {
    let Some(sector) = world.sectors.get(sector_key) else {
        return egui::Color32::YELLOW;
    };
    let player = cockpit_faction(faction);
    let (friendly, hostile) = sector.systems.iter().fold((0, 0), |counts, key| {
        let Some(system) = world.systems.get(*key) else {
            return counts;
        };
        match system.control.faction() {
            Some(owner) if owner == player => (counts.0 + 1, counts.1),
            Some(_) => (counts.0, counts.1 + 1),
            None => counts,
        }
    });
    match friendly.cmp(&hostile) {
        std::cmp::Ordering::Greater => egui::Color32::from_rgb(0, 255, 64),
        std::cmp::Ordering::Less => egui::Color32::from_rgb(255, 32, 32),
        std::cmp::Ordering::Equal => egui::Color32::YELLOW,
    }
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

fn paint_window_border(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    rect: egui::Rect,
    scale: f32,
) {
    paint_resource(
        painter,
        ctx,
        cache,
        BORDER_TOP_LEFT,
        logical_rect(rect, scale, 0.0, 0.0, 2.0, 2.0),
    );
    paint_resource(
        painter,
        ctx,
        cache,
        BORDER_TOP_RIGHT,
        logical_rect(rect, scale, SECTOR_WINDOW_WIDTH - 2.0, 0.0, 2.0, 2.0),
    );
    paint_resource(
        painter,
        ctx,
        cache,
        BORDER_BOTTOM_LEFT,
        logical_rect(rect, scale, 0.0, SECTOR_WINDOW_HEIGHT - 2.0, 2.0, 2.0),
    );
    paint_resource(
        painter,
        ctx,
        cache,
        BORDER_BOTTOM_RIGHT,
        logical_rect(
            rect,
            scale,
            SECTOR_WINDOW_WIDTH - 2.0,
            SECTOR_WINDOW_HEIGHT - 2.0,
            2.0,
            2.0,
        ),
    );
    paint_tiled_horizontal(
        painter,
        ctx,
        cache,
        BORDER_TOP,
        rect,
        scale,
        2.0,
        SECTOR_WINDOW_WIDTH - 2.0,
        0.0,
    );
    paint_tiled_horizontal(
        painter,
        ctx,
        cache,
        BORDER_BOTTOM,
        rect,
        scale,
        2.0,
        SECTOR_WINDOW_WIDTH - 2.0,
        SECTOR_WINDOW_HEIGHT - 1.0,
    );
    paint_tiled_vertical(
        painter,
        ctx,
        cache,
        BORDER_LEFT,
        rect,
        scale,
        0.0,
        2.0,
        SECTOR_WINDOW_HEIGHT - 2.0,
    );
    paint_tiled_vertical(
        painter,
        ctx,
        cache,
        BORDER_RIGHT,
        rect,
        scale,
        SECTOR_WINDOW_WIDTH - 1.0,
        2.0,
        SECTOR_WINDOW_HEIGHT - 2.0,
    );
}

#[allow(clippy::too_many_arguments)]
fn paint_tiled_horizontal(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    resource_id: u32,
    parent: egui::Rect,
    scale: f32,
    start: f32,
    end: f32,
    y: f32,
) {
    let Some(texture_id) = cache
        .get(ctx, DllSource::Strategy, resource_id)
        .map(egui_macroquad::egui::TextureHandle::id)
    else {
        return;
    };
    let mut mesh = egui::Mesh::with_texture(texture_id);
    let mut x = start;
    while x < end {
        let width = (end - x).min(2.0);
        mesh.add_rect_with_uv(
            logical_rect(parent, scale, x, y, width, 1.0),
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(width / 2.0, 1.0)),
            egui::Color32::WHITE,
        );
        x += 2.0;
    }
    painter.add(mesh);
}

#[allow(clippy::too_many_arguments)]
fn paint_tiled_vertical(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    resource_id: u32,
    parent: egui::Rect,
    scale: f32,
    x: f32,
    start: f32,
    end: f32,
) {
    let Some(texture_id) = cache
        .get(ctx, DllSource::Strategy, resource_id)
        .map(egui_macroquad::egui::TextureHandle::id)
    else {
        return;
    };
    let mut mesh = egui::Mesh::with_texture(texture_id);
    let mut y = start;
    while y < end {
        let height = (end - y).min(2.0);
        mesh.add_rect_with_uv(
            logical_rect(parent, scale, x, y, 1.0, height),
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, height / 2.0)),
            egui::Color32::WHITE,
        );
        y += 2.0;
    }
    painter.add(mesh);
}

#[allow(clippy::too_many_arguments)]
fn paint_status_tracks(
    painter: &egui::Painter,
    parent: egui::Rect,
    scale: f32,
    x: f32,
    y: f32,
    alliance: f32,
    empire: f32,
    energy: u8,
    raw_materials: u8,
) {
    let track_width = 37.0;
    let alliance_width = track_width * alliance.clamp(0.0, 1.0);
    let empire_width = track_width * empire.clamp(0.0, 1.0);
    painter.rect_filled(
        logical_rect(parent, scale, x, y + 48.0, track_width, 3.0),
        0.0,
        egui::Color32::from_rgb(22, 22, 22),
    );
    painter.rect_filled(
        logical_rect(parent, scale, x, y + 48.0, alliance_width, 3.0),
        0.0,
        egui::Color32::from_rgb(32, 112, 255),
    );
    painter.rect_filled(
        logical_rect(
            parent,
            scale,
            x + track_width - empire_width,
            y + 48.0,
            empire_width,
            3.0,
        ),
        0.0,
        egui::Color32::from_rgb(255, 32, 32),
    );
    painter.rect_filled(
        logical_rect(
            parent,
            scale,
            x,
            y + 52.0,
            track_width * (f32::from(energy) / 14.0).clamp(0.0, 1.0),
            3.0,
        ),
        0.0,
        egui::Color32::from_rgb(255, 220, 0),
    );
    painter.rect_filled(
        logical_rect(
            parent,
            scale,
            x,
            y + 56.0,
            track_width * (f32::from(raw_materials) / 14.0).clamp(0.0, 1.0),
            3.0,
        ),
        0.0,
        egui::Color32::from_rgb(0, 240, 240),
    );
}

fn paint_selection_brackets(painter: &egui::Painter, rect: egui::Rect, scale: f32) {
    let color = egui::Color32::from_rgb(255, 32, 32);
    let stroke = egui::Stroke::new(scale.max(1.0), color);
    let length = 5.0 * scale;
    for (corner, dx, dy) in [
        (rect.left_top(), 1.0, 1.0),
        (rect.right_top(), -1.0, 1.0),
        (rect.left_bottom(), 1.0, -1.0),
        (rect.right_bottom(), -1.0, -1.0),
    ] {
        painter.line_segment(
            [corner, egui::pos2(corner.x + dx * length, corner.y)],
            stroke,
        );
        painter.line_segment(
            [corner, egui::pos2(corner.x, corner.y + dy * length)],
            stroke,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::world::{Sector, System};

    #[test]
    fn a_system_in_revolt_takes_the_uprising_name_colour() {
        // UprisingState, not ControlKind, records a revolt (F-026), so a held
        // system in revolt must colour like ControlKind::Uprising.
        let held = ControlKind::Controlled(Faction::Alliance);
        assert_eq!(
            system_name_color(held, true, Faction::Alliance),
            system_name_color(
                ControlKind::Uprising(Faction::Alliance),
                false,
                Faction::Alliance
            )
        );
        assert_ne!(
            system_name_color(held, true, Faction::Alliance),
            system_name_color(held, false, Faction::Alliance)
        );
        assert_eq!(
            system_name_color(ControlKind::Contested, true, Faction::Alliance),
            system_name_color(ControlKind::Contested, false, Faction::Alliance)
        );
    }

    fn fixture_world() -> (GameWorld, SystemKey, SystemKey) {
        let mut world = GameWorld::default();
        let sector_a = world.sectors.insert(Sector {
            dat_id: DatId::new(36),
            name: "Sesswenna".into(),
            group: SectorGroup::Core,
            x: 317,
            y: 248,
            systems: Vec::new(),
        });
        let system_a = world.systems.insert(System {
            dat_id: DatId::new(263),
            name: "Chandrila".into(),
            sector: sector_a,
            x: 322,
            y: 260,
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
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        });
        world.sectors[sector_a].systems.push(system_a);

        let sector_b = world.sectors.insert(Sector {
            dat_id: DatId::new(37),
            name: "Sluis".into(),
            group: SectorGroup::RimInner,
            x: 526,
            y: 532,
            systems: Vec::new(),
        });
        let system_b = world.systems.insert(System {
            dat_id: DatId::new(270),
            name: "Bothawui".into(),
            sector: sector_b,
            x: 560,
            y: 548,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.0,
            popularity_empire: 1.0,
            is_populated: true,
            total_energy: 5,
            raw_materials: 9,
            espionage_rating: 0.0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        });
        world.sectors[sector_b].systems.push(system_b);
        (world, system_a, system_b)
    }

    fn layout(scale: f32) -> CockpitLayout {
        CockpitLayout {
            canvas: CockpitViewport {
                x: 10.0,
                y: 20.0,
                width: 640.0 * scale,
                height: 480.0 * scale,
            },
            galaxy: CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            scale,
        }
    }

    #[test]
    fn original_planet_mapping_covers_all_special_ids() {
        // Source: SYSTEMSD.DAT picture_id values and STRATEGY.DLL bitmap resources 10212-10239.
        assert_eq!(planet_resource_id(DatId::new(100)), 10212);
        assert_eq!(planet_resource_id(DatId::new(263)), 10224);
        assert_eq!(planet_resource_id(DatId::new(269)), 10215);
        assert_eq!(planet_resource_id(DatId::new(271)), 10239);
        assert_eq!(planet_resource_id(DatId::new(279)), 10237);
        assert_eq!(planet_resource_id(DatId::new(282)), 10238);
        assert_eq!(planet_resource_id(DatId::new(99)), 10212);
    }

    #[test]
    fn sector_relative_coordinates_scale_x_by_37_over_13_and_y_by_37_over_10() {
        // No recovered source: the 13, 10 and 37 scale constants, kept as a regression pin.
        assert_eq!(sector_planet_position(317, 248, 322, 260), (14.0, 44.0));
        assert_eq!(sector_planet_position(317, 248, 373, 272), (159.0, 89.0));
        assert_eq!(sector_planet_position(317, 248, 322, 333), (14.0, 315.0));
    }

    #[test]
    fn modeless_windows_alternate_columns_and_raise_without_duplicates() {
        let (world, first, second) = fixture_world();
        let mut state = SectorWindowState::default();
        assert!(state.open_for_system(&world, first, CockpitFaction::Alliance));
        assert!(state.open_for_system(&world, second, CockpitFaction::Alliance));
        assert_eq!(state.window_count(), 2);
        assert_eq!(state.windows[0].column, WindowColumn::Primary);
        assert_eq!(state.windows[1].column, WindowColumn::Secondary);
        assert!(state.open_for_system(&world, first, CockpitFaction::Alliance));
        assert_eq!(state.window_count(), 2);
        assert_eq!(
            state.windows.last().unwrap().sector,
            world.systems[first].sector
        );
    }

    /// Add a one-system sector at map x `x` and return its system.
    fn add_sector(world: &mut GameWorld, x: u16) -> SystemKey {
        let template = world.systems.values().next().unwrap().clone();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(38),
            name: "Added".into(),
            group: SectorGroup::Core,
            x,
            y: 300,
            systems: Vec::new(),
        });
        let system = world.systems.insert(System { sector, ..template });
        world.sectors[sector].systems.push(system);
        system
    }

    #[test]
    fn a_first_window_takes_the_column_of_its_sectors_half_of_the_galaxy() {
        // FUN_00429ce0: with both columns free, a sector at x >= 1023 / 2
        // (DAT_00658bd8) opens in the second column.
        let (world, left, right) = fixture_world();
        for (system, column) in [
            (left, WindowColumn::Primary),
            (right, WindowColumn::Secondary),
        ] {
            let mut state = SectorWindowState::default();
            assert!(state.open_for_system(&world, system, CockpitFaction::Empire));
            assert_eq!(state.windows[0].column, column);
        }
        // The other column is the free one, whatever the half.
        let mut state = SectorWindowState::default();
        state.open_for_system(&world, right, CockpitFaction::Alliance);
        let mut world = world;
        let also_right = add_sector(&mut world, 900);
        state.open_for_system(&world, also_right, CockpitFaction::Alliance);
        assert_eq!(state.windows[1].column, WindowColumn::Primary);
    }

    #[test]
    fn a_third_sector_window_replaces_the_one_on_its_half_of_the_galaxy() {
        // FUN_00429ce0 → FUN_00600f90: with both columns taken, the window
        // in the new sector's half's column is destroyed and replaced.
        let (mut world, left, right) = fixture_world();
        let far_left = add_sector(&mut world, 100);
        let far_right = add_sector(&mut world, 1000);
        let mut state = SectorWindowState::default();
        state.open_for_system(&world, left, CockpitFaction::Alliance);
        state.open_for_system(&world, right, CockpitFaction::Alliance);

        state.open_for_system(&world, far_left, CockpitFaction::Alliance);
        assert_eq!(state.window_count(), 2);
        let sectors = |state: &SectorWindowState| {
            let mut open: Vec<_> = state
                .windows
                .iter()
                .map(|window| (window.column == WindowColumn::Primary, window.sector))
                .collect();
            open.sort_by_key(|(primary, _)| !primary);
            open
        };
        assert_eq!(
            sectors(&state),
            [
                (true, world.systems[far_left].sector),
                (false, world.systems[right].sector)
            ]
        );

        state.open_for_system(&world, far_right, CockpitFaction::Alliance);
        assert_eq!(
            sectors(&state),
            [
                (true, world.systems[far_left].sector),
                (false, world.systems[far_right].sector)
            ]
        );
    }

    #[test]
    fn faction_change_clears_stale_windows() {
        let (world, first, _) = fixture_world();
        let mut state = SectorWindowState::default();
        state.open_for_system(&world, first, CockpitFaction::Alliance);
        state.prepare_faction(CockpitFaction::Empire);
        assert_eq!(state.window_count(), 0);
    }

    #[test]
    fn occlusion_uses_scaled_exclusive_rect_edges() {
        let (world, first, _) = fixture_world();
        let mut state = SectorWindowState::default();
        state.open_for_system(&world, first, CockpitFaction::Alliance);
        let layout = layout(2.0);
        assert!(state.contains_screen_point(layout, (130.0, 90.0)));
        assert!(state.contains_screen_point(layout, (599.9, 809.9)));
        assert!(!state.contains_screen_point(layout, (600.0, 90.0)));
        assert!(!state.contains_screen_point(layout, (130.0, 810.0)));
    }

    #[test]
    fn original_control_rects_exclude_right_and_bottom_edges() {
        let rect = egui::Rect::from_min_max(egui::pos2(10.0, 20.0), egui::pos2(24.0, 34.0));
        assert!(rect_contains(rect, egui::pos2(10.0, 20.0)));
        assert!(rect_contains(rect, egui::pos2(23.999, 33.999)));
        assert!(!rect_contains(rect, egui::pos2(24.0, 20.0)));
        assert!(!rect_contains(rect, egui::pos2(10.0, 34.0)));
    }

    /// Draw `system`'s sector window and double-click at the canvas point
    /// `at`, returning every action.
    fn double_click_at(
        world: &GameWorld,
        system: SystemKey,
        at: (f32, f32),
    ) -> Vec<SectorWindowAction> {
        double_click_with_missions(
            world,
            system,
            &rebellion_core::missions::MissionState::new(),
            at,
        )
    }

    fn double_click_with_missions(
        world: &GameWorld,
        system: SystemKey,
        missions: &rebellion_core::missions::MissionState,
        at: (f32, f32),
    ) -> Vec<SectorWindowAction> {
        let layout = layout(1.0);
        let mut state = SectorWindowState::default();
        state.open_for_system(world, system, CockpitFaction::Alliance);
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(system);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let uprisings = rebellion_core::uprising::UprisingState::default();
        let point = egui::pos2(layout.canvas.x + at.0, layout.canvas.y + at.1);
        let button = |pressed| egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        };
        let mut actions = Vec::new();
        let frames = [
            vec![egui::Event::PointerMoved(point)],
            vec![egui::Event::PointerMoved(point)],
            vec![button(true)],
            vec![button(false)],
            vec![button(true)],
            vec![button(false)],
            vec![],
        ];
        for (index, events) in frames.into_iter().enumerate() {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                time: Some(index as f64 * 0.05),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                actions.extend(draw_sector_windows(
                    ctx,
                    world,
                    &rebellion_core::movement::MovementState::default(),
                    &fog,
                    &mut state,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                    &uprisings,
                    missions,
                ));
            });
        }
        actions
    }

    /// Draw `system`'s sector window and click each `(button, point)` in
    /// turn, a press and a release apiece, in one window state.
    fn clicks_at(
        world: &GameWorld,
        system: SystemKey,
        clicks: &[(egui::PointerButton, (f32, f32))],
    ) -> Vec<SectorWindowAction> {
        let layout = layout(1.0);
        let mut state = SectorWindowState::default();
        state.open_for_system(world, system, CockpitFaction::Alliance);
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(system);
        let missions = rebellion_core::missions::MissionState::new();
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let uprisings = rebellion_core::uprising::UprisingState::default();
        let mut frames = Vec::new();
        for &(button, at) in clicks {
            let pos = egui::pos2(layout.canvas.x + at.0, layout.canvas.y + at.1);
            let event = |pressed| egui::Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers: egui::Modifiers::default(),
            };
            frames.push(vec![egui::Event::PointerMoved(pos)]);
            frames.push(vec![event(true)]);
            frames.push(vec![event(false)]);
            frames.push(vec![]);
        }
        let mut actions = Vec::new();
        for (index, events) in frames.into_iter().enumerate() {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                // Far enough apart that no two clicks make a double click.
                time: Some(index as f64 * 0.5),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                actions.extend(draw_sector_windows(
                    ctx,
                    world,
                    &rebellion_core::movement::MovementState::default(),
                    &fog,
                    &mut state,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                    &uprisings,
                    &missions,
                ));
            });
        }
        actions
    }

    fn menus(actions: &[SectorWindowAction]) -> Vec<(Option<MenuObject>, (i16, i16))> {
        actions
            .iter()
            .filter_map(|action| match *action {
                SectorWindowAction::OpenObjectMenu { selection, point } => Some((selection, point)),
                _ => None,
            })
            .collect()
    }

    const RIGHT: egui::PointerButton = egui::PointerButton::Secondary;
    const LEFT: egui::PointerButton = egui::PointerButton::Primary;

    #[test]
    fn a_right_click_on_the_fleet_icon_opens_its_menu_at_the_cursor() {
        // FUN_004593e0 case 0x204: FUN_0045cc10 finds the shown overlay and
        // FUN_0045b1b0 makes it the window's selection (FUN_004f5b10: the
        // system and the kind). WM_RBUTTONUP reaches slot 7, FUN_004ac5c0,
        // with the cursor. (115, 90) is on the icon, past the planet.
        let (mut world, system, _) = fixture_world();
        add_fleet(&mut world, system);
        let fleets = Some(MenuObject::SystemIcon {
            system,
            quadrant: Quadrant::Fleets,
        });
        assert_eq!(
            menus(&clicks_at(&world, system, &[(RIGHT, (115.0, 90.0))])),
            [(fleets, (115, 90))]
        );
        // On the planet's own picture, where the icon overlaps it.
        assert_eq!(
            menus(&clicks_at(&world, system, &[(RIGHT, (100.0, 85.0))])),
            [(fleets, (100, 85))]
        );
    }

    #[test]
    fn each_shown_icon_is_its_own_selection() {
        // FUN_00459e30 gives each quadrant its own item and kind.
        let (mut world, system, _) = fixture_world();
        add_fleet(&mut world, system);
        add_mine(&mut world, system);
        let troop = world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: DatId::new(0x1000_0002),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(troop);
        for (quadrant, at) in [
            (Quadrant::System, (66.0, 80.0)),
            (Quadrant::Defenses, (66.0, 110.0)),
            (Quadrant::Fleets, (115.0, 80.0)),
        ] {
            assert_eq!(
                menus(&clicks_at(&world, system, &[(RIGHT, at)])),
                [(
                    Some(MenuObject::SystemIcon { system, quadrant }),
                    (at.0 as i16, at.1 as i16)
                )],
                "{quadrant:?}"
            );
        }
    }

    #[test]
    fn a_press_off_every_icon_keeps_the_windows_selection() {
        // FUN_004593e0 cases 0x201 and 0x204 return before FUN_0045afc0 when
        // FUN_0045cc10 finds nothing, so the last icon chosen stays selected
        // and the next right release (FUN_004ac5c0) opens its menu.
        let (mut world, system, _) = fixture_world();
        add_fleet(&mut world, system);
        let fleets = Some(MenuObject::SystemIcon {
            system,
            quadrant: Quadrant::Fleets,
        });
        // (200, 250) is empty window space, far from any planet.
        assert_eq!(
            menus(&clicks_at(
                &world,
                system,
                &[(LEFT, (115.0, 90.0)), (RIGHT, (200.0, 250.0))]
            )),
            [(fleets, (200, 250))]
        );
        // With nothing chosen yet, the menu opens for an empty selection.
        assert_eq!(
            menus(&clicks_at(&world, system, &[(RIGHT, (200.0, 250.0))])),
            [(None, (200, 250))]
        );
    }

    #[test]
    fn a_right_release_outside_the_window_opens_no_menu() {
        // WM_RBUTTONUP reaches the window under the cursor; the galaxy view
        // beyond the window's right edge (60 + 235) is not this window.
        let (mut world, system, _) = fixture_world();
        add_fleet(&mut world, system);
        assert!(menus(&clicks_at(
            &world,
            system,
            &[(LEFT, (115.0, 90.0)), (RIGHT, (400.0, 200.0))]
        ))
        .is_empty());
    }

    /// Press the left button at canvas point `from`, move to `to` and
    /// release there, with `modifiers` held.
    fn drag_at(
        world: &GameWorld,
        system: SystemKey,
        from: (f32, f32),
        to: (f32, f32),
        modifiers: egui::Modifiers,
    ) -> Vec<SectorWindowAction> {
        drags_at(world, system, &[(from, to)], modifiers)
    }

    /// A left-button drag's canvas points: where it presses, where it
    /// releases.
    type Drag = ((f32, f32), (f32, f32));

    /// Each drag of `drags` in turn, in one window state.
    fn drags_at(
        world: &GameWorld,
        system: SystemKey,
        drags: &[Drag],
        modifiers: egui::Modifiers,
    ) -> Vec<SectorWindowAction> {
        let layout = layout(1.0);
        let mut state = SectorWindowState::default();
        state.open_for_system(world, system, CockpitFaction::Alliance);
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(system);
        let missions = rebellion_core::missions::MissionState::new();
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let uprisings = rebellion_core::uprising::UprisingState::default();
        let at =
            |point: (f32, f32)| egui::pos2(layout.canvas.x + point.0, layout.canvas.y + point.1);
        let button = |pos, pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers,
        };
        let frames = drags.iter().flat_map(|&(from, to)| {
            [
                vec![egui::Event::PointerMoved(at(from))],
                vec![button(at(from), true)],
                vec![egui::Event::PointerMoved(at(to))],
                vec![button(at(to), false)],
                vec![],
            ]
        });
        let mut actions = Vec::new();
        for (index, events) in frames.into_iter().enumerate() {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                time: Some(index as f64 * 0.05),
                modifiers,
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                actions.extend(draw_sector_windows(
                    ctx,
                    world,
                    &rebellion_core::movement::MovementState::default(),
                    &fog,
                    &mut state,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                    &uprisings,
                    &missions,
                ));
            });
        }
        actions
    }

    fn drops(actions: &[SectorWindowAction]) -> Vec<(MenuObject, (f32, f32), bool)> {
        actions
            .iter()
            .filter_map(|action| match *action {
                SectorWindowAction::DropSelection {
                    selection,
                    point,
                    confirmed,
                } => Some((selection, (point.x, point.y), confirmed)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_drag_from_the_fleet_icon_drops_its_selection_where_released() {
        // FUN_004593e0: case 0x201 selects the icon and keeps the press
        // point; case 0x202 past four pixels posts 0x29a with the cursor.
        let (mut world, system, _) = fixture_world();
        add_fleet(&mut world, system);
        let fleets = MenuObject::SystemIcon {
            system,
            quadrant: Quadrant::Fleets,
        };
        // Released at (200, 250) on the canvas, which sits at (10, 20).
        assert_eq!(
            drops(&drag_at(
                &world,
                system,
                (115.0, 90.0),
                (200.0, 250.0),
                egui::Modifiers::NONE
            )),
            [(fleets, (210.0, 270.0), false)]
        );
        // Ctrl held: FUN_00422ce0 issues 0x202 (GetAsyncKeyState(0x11)).
        assert_eq!(
            drops(&drag_at(
                &world,
                system,
                (115.0, 90.0),
                (200.0, 250.0),
                egui::Modifiers::CTRL
            )),
            [(fleets, (210.0, 270.0), true)]
        );
    }

    #[test]
    fn a_release_within_four_pixels_of_the_press_drops_nothing() {
        // FUN_004593e0 case 0x202: 4 < |dx| or 4 < |dy|.
        let (mut world, system, _) = fixture_world();
        add_fleet(&mut world, system);
        let none = egui::Modifiers::NONE;
        assert!(drops(&drag_at(&world, system, (110.0, 90.0), (114.0, 86.0), none)).is_empty());
        assert_eq!(
            drops(&drag_at(&world, system, (110.0, 90.0), (110.0, 95.0), none)).len(),
            1
        );
        assert_eq!(
            drops(&drag_at(&world, system, (110.0, 90.0), (115.0, 90.0), none)).len(),
            1
        );
    }

    #[test]
    fn a_dropped_selection_does_not_drop_again_on_the_next_release() {
        // FUN_004593e0 case 0x202 ends the drag it posts 0x29a for; a later
        // press off every icon (case 0x201 returns early) starts none.
        let (mut world, system, _) = fixture_world();
        add_fleet(&mut world, system);
        assert_eq!(
            drops(&drags_at(
                &world,
                system,
                &[
                    ((115.0, 90.0), (200.0, 250.0)),
                    ((200.0, 250.0), (150.0, 200.0))
                ],
                egui::Modifiers::NONE
            ))
            .len(),
            1
        );
    }

    #[test]
    fn a_drag_that_starts_off_every_icon_drops_nothing() {
        // FUN_004593e0 case 0x200 drags only an item the press chose
        // (+0x1a0); (200, 250) is empty window space.
        let (mut world, system, _) = fixture_world();
        add_fleet(&mut world, system);
        assert!(drops(&drag_at(
            &world,
            system,
            (200.0, 250.0),
            (115.0, 90.0),
            egui::Modifiers::NONE
        ))
        .is_empty());
    }

    #[test]
    fn a_hidden_icons_corner_selects_nothing() {
        // FUN_0045d140 removes an item with nothing to show, so a press on
        // its corner finds no overlay.
        let (world, system, _) = fixture_world();
        assert_eq!(
            menus(&clicks_at(&world, system, &[(RIGHT, (115.0, 90.0))])),
            [(None, (115, 90))]
        );
    }

    fn opened(actions: &[SectorWindowAction]) -> Vec<SectorWindowAction> {
        actions
            .iter()
            .copied()
            .filter(|action| !matches!(action, SectorWindowAction::SelectSystem(_)))
            .collect()
    }

    fn add_fleet(world: &mut GameWorld, system: SystemKey) {
        let fleet = world.fleets.insert(rebellion_core::world::Fleet {
            location: system,
            capital_ships: Vec::new(),
            fighters: Vec::new(),
            characters: Vec::new(),
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[system].fleets.push(fleet);
    }

    #[test]
    fn a_double_click_on_the_fleet_icon_opens_the_fleet_window() {
        // FUN_004593e0 case 0x203: the shown overlay under the point;
        // FUN_0045aac0 maps kind 0x10 to window type 4. Chandrila's planet
        // sits at (74, 79), so the icon spans (93, 78) to (121, 97).
        let (mut world, system, _) = fixture_world();
        add_fleet(&mut world, system);
        let actions = double_click_at(&world, system, (100.0, 85.0));

        assert_eq!(
            opened(&actions),
            [SectorWindowAction::OpenFleetWindow {
                system,
                logical_position: (100, 85),
            }]
        );
        // Past the planet's right edge, still on the icon.
        let actions = double_click_at(&world, system, (115.0, 90.0));
        assert!(matches!(
            opened(&actions)[..],
            [SectorWindowAction::OpenFleetWindow { .. }]
        ));
    }

    #[test]
    fn the_fleet_window_opens_from_the_fleet_icons_point() {
        // FUN_0045c8e0 opens the Fleet window at the sector item's stored
        // point; port: the center of the icon at (93, 78) to (121, 97).
        let (world, system, _) = fixture_world();
        let mut state = SectorWindowState::default();
        assert_eq!(state.fleet_window_point(&world, layout(1.0), system), None);
        state.open_for_system(&world, system, CockpitFaction::Alliance);

        assert_eq!(
            state.fleet_window_point(&world, layout(1.0), system),
            Some((107, 88))
        );
        assert_eq!(
            state.fleet_window_point(&world, layout(2.0), system),
            Some((107, 88))
        );
    }

    #[test]
    fn without_fleets_the_icons_corner_opens_the_system_window() {
        // FUN_0045d140 hides the overlay at a zero count, so the planet
        // answers.
        let (world, system, _) = fixture_world();
        let actions = double_click_at(&world, system, (100.0, 85.0));

        assert!(matches!(
            opened(&actions)[..],
            [SectorWindowAction::OpenSystemWindow { .. }]
        ));
    }

    #[test]
    fn the_fleet_icons_rect_is_the_planets_top_right_quadrant() {
        // FUN_00459e30, flag 0x100000: (cx + 1, cy - 19) to (cx + 29, cy).
        let (world, system, _) = fixture_world();
        let layout = layout(1.0);
        let mut state = SectorWindowState::default();
        state.open_for_system(&world, system, CockpitFaction::Alliance);

        assert_eq!(
            state.fleet_icon_screen_rect(&world, layout, system),
            Some(egui::Rect::from_min_size(
                egui::pos2(10.0 + 93.0, 20.0 + 78.0),
                egui::vec2(28.0, 19.0)
            ))
        );
        // Twice the size: the offsets and the size scale with the window.
        let doubled = CockpitLayout {
            scale: 2.0,
            ..layout
        };
        assert_eq!(
            state.fleet_icon_screen_rect(&world, doubled, system),
            Some(egui::Rect::from_min_size(
                egui::pos2(10.0 + 93.0 * 2.0, 20.0 + 78.0 * 2.0),
                egui::vec2(56.0, 38.0)
            ))
        );
    }

    #[test]
    fn each_quadrant_icon_sits_at_its_corner_of_the_planet() {
        // FUN_00459e30:369-447: 28 by 19 each; the left ones end at cx, the
        // right ones start at cx + 1; the top ones end at cy, the bottom ones
        // start at cy + 1. Chandrila's planet sits at (74, 79), so cx = 92
        // and cy = 97.
        let (world, system, _) = fixture_world();
        let mut state = SectorWindowState::default();
        state.open_for_system(&world, system, CockpitFaction::Alliance);
        let corners = [
            (Quadrant::System, (64.0, 78.0)),
            (Quadrant::Defenses, (64.0, 98.0)),
            (Quadrant::Fleets, (93.0, 78.0)),
            (Quadrant::Missions, (93.0, 98.0)),
        ];
        for scale in [1.0, 2.0] {
            for (quadrant, (x, y)) in corners {
                assert_eq!(
                    state.quadrant_screen_rect(&world, layout(scale), system, quadrant),
                    Some(egui::Rect::from_min_size(
                        egui::pos2(10.0 + x * scale, 20.0 + y * scale),
                        egui::vec2(28.0 * scale, 19.0 * scale)
                    )),
                    "{quadrant:?} at scale {scale}"
                );
            }
        }
    }

    fn add_mine(world: &mut GameWorld, system: SystemKey) {
        let mine =
            world
                .production_facilities
                .insert(rebellion_core::world::ProductionFacilityInstance {
                    class_dat_id: DatId::new(0x2c00_0001),
                    is_alliance: true,
                    is_mine: true,
                });
        world.systems[system].production_facilities.push(mine);
    }

    #[test]
    fn a_double_click_on_the_system_icon_opens_the_system_window() {
        // FUN_0045aac0 maps kind 4 to window type 9. (66, 80) lies on the
        // top-left icon, left of the planet's picture.
        let (mut world, system, _) = fixture_world();
        assert!(opened(&double_click_at(&world, system, (66.0, 80.0))).is_empty());

        add_mine(&mut world, system);
        assert_eq!(
            opened(&double_click_at(&world, system, (66.0, 80.0))),
            [SectorWindowAction::OpenSystemWindow {
                system,
                logical_position: (66, 80),
            }]
        );
    }

    #[test]
    fn a_double_click_on_the_defenses_icon_opens_the_defenses_window() {
        // FUN_0045aac0 maps kind 8 to window type 10. (66, 110) lies on the
        // bottom-left icon, (64, 98) to (92, 117), left of the planet.
        let (mut world, system, _) = fixture_world();
        assert!(opened(&double_click_at(&world, system, (66.0, 110.0))).is_empty());

        let troop = world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: DatId::new(0x1000_0002),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(troop);
        assert_eq!(
            opened(&double_click_at(&world, system, (66.0, 110.0))),
            [SectorWindowAction::OpenDefensesWindow {
                system,
                logical_position: (66, 110),
            }]
        );
    }

    #[test]
    fn a_double_click_on_the_missions_icon_opens_the_missions_window() {
        // FUN_0045aac0 maps kind 0x40 to window type 11. (119, 110) lies on
        // the bottom-right icon, (93, 98) to (121, 117), right of the
        // planet.
        let (mut world, system, _) = fixture_world();
        let agent = world.characters.insert(rebellion_core::world::Character {
            dat_id: DatId::new(832),
            name: "Agent".into(),
            is_alliance: true,
            current_system: Some(system),
            recruited: true,
            ..Default::default()
        });
        let mut missions = rebellion_core::missions::MissionState::new();
        assert!(opened(&double_click_with_missions(
            &world,
            system,
            &missions,
            (119.0, 110.0)
        ))
        .is_empty());

        missions.dispatch(rebellion_core::missions::MissionRequest::single(
            rebellion_core::missions::MissionKind::Diplomacy,
            rebellion_core::missions::MissionFaction::Alliance,
            agent,
            system,
            None,
            0,
        ));
        assert_eq!(
            opened(&double_click_with_missions(
                &world,
                system,
                &missions,
                (119.0, 110.0)
            )),
            [SectorWindowAction::OpenMissionsWindow {
                system,
                logical_position: (119, 110),
            }]
        );
    }

    /// Draw `system`'s sector window at `scale` for two frames (an Area
    /// paints nothing on its first) and return each quadrant bitmap painted
    /// and where.
    fn painted_quadrant_icons(
        world: &GameWorld,
        system: SystemKey,
        missions: &rebellion_core::missions::MissionState,
        scale: f32,
    ) -> Vec<(u32, egui::Pos2)> {
        let layout = layout(scale);
        let mut state = SectorWindowState::default();
        state.open_for_system(world, system, CockpitFaction::Alliance);
        let mut fog = FogState::new(Faction::Alliance);
        fog.reveal(system);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let uprisings = rebellion_core::uprising::UprisingState::default();
        for index in 0..2 {
            crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow_mut().clear());
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1040.0),
                )),
                time: Some(f64::from(index) * 0.05),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                let _ = draw_sector_windows(
                    ctx,
                    world,
                    &rebellion_core::movement::MovementState::default(),
                    &fog,
                    &mut state,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                    &uprisings,
                    missions,
                );
            });
        }
        crate::fleet_window::tests::PAINTED
            .with(|painted| painted.take())
            .into_iter()
            .filter(|(id, _)| (10771..=10790).contains(id))
            .collect()
    }

    #[test]
    fn the_sector_window_paints_each_shown_quadrant_icon_at_its_corner() {
        // FUN_0045d140 paints FUN_0045ca80's first bitmap for each enabled
        // item: a mine (kind 4), a regiment (kind 8), a fleet (kind 0x10)
        // and an agent on a mission (kind 0x40), all Alliance (side 1).
        let (mut world, system, _) = fixture_world();
        let empty = rebellion_core::missions::MissionState::new();
        assert!(painted_quadrant_icons(&world, system, &empty, 2.0).is_empty());

        add_mine(&mut world, system);
        add_fleet(&mut world, system);
        let regiment = world.troops.insert(rebellion_core::world::TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(regiment);
        let agent = world.characters.insert(rebellion_core::world::Character {
            dat_id: DatId::new(832),
            name: "Agent".into(),
            is_alliance: true,
            current_system: Some(system),
            recruited: true,
            ..Default::default()
        });
        let mut missions = rebellion_core::missions::MissionState::new();
        missions.dispatch(rebellion_core::missions::MissionRequest::single(
            rebellion_core::missions::MissionKind::Diplomacy,
            rebellion_core::missions::MissionFaction::Alliance,
            agent,
            system,
            None,
            0,
        ));

        let at = |x: f32, y: f32| egui::pos2(10.0 + x * 2.0, 20.0 + y * 2.0);
        assert_eq!(
            painted_quadrant_icons(&world, system, &missions, 2.0),
            [
                (10771, at(64.0, 78.0)),
                (10773, at(64.0, 98.0)),
                (10775, at(93.0, 78.0)),
                (10777, at(93.0, 98.0)),
            ]
        );
    }
}
