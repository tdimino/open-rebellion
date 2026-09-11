//! Original strategic sector windows.
//!
//! `REBEXE.EXE` constructs these as modeless 235 by 360 child windows. The
//! window body is data-driven: each system uses its `SYSTEMSD.DAT` planet
//! picture, sector-relative coordinates, and three compact status tracks.
//! The close and side-switch controls use their original STRATEGY resources.

use egui_macroquad::egui;
use rebellion_core::dat::Faction;
use rebellion_core::ids::{DatId, SectorKey, SystemKey};
use rebellion_core::world::{ControlKind, GameWorld};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};

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
    /// Open the selected system's parent sector. Existing windows raise rather
    /// than duplicate, matching the original child-window lookup.
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
        let column = self
            .windows
            .last()
            .map_or(WindowColumn::Primary, |window| window.column.opposite());
        self.windows.push(OpenSectorWindow { sector, column });
        true
    }

    /// True when the pointer lies inside any visible window. Logical right and
    /// bottom edges remain exclusive, as in the recovered Win32 rectangles.
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.windows.iter().any(|window| {
            let rect = window_screen_rect(self.faction, window.column, layout);
            point.0 >= rect.min.x
                && point.0 < rect.max.x
                && point.1 >= rect.min.y
                && point.1 < rect.max.y
        })
    }

    pub fn window_count(&self) -> usize {
        self.windows.len()
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
}

#[derive(Default)]
struct WindowDrawResult {
    focus: bool,
    close: bool,
    switch_side: bool,
    selected: Option<SystemKey>,
    opened: Option<(SystemKey, (i16, i16))>,
}

/// Paint and operate all open sector windows using the recovered strategic
/// canvas. Window mutations are applied after the pass so area ordering stays
/// deterministic.
pub fn draw_sector_windows(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut SectorWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Vec<SectorWindowAction> {
    state.prepare_faction(faction);
    let windows = state.windows.clone();
    let mut actions = Vec::new();
    let mut focused = None;
    let mut closed = None;
    let mut switched = None;

    let focused_sector = windows.last().map(|window| window.sector);
    for window in windows {
        let result = draw_sector_window(
            ctx,
            world,
            window,
            focused_sector == Some(window.sector),
            faction,
            layout,
            cache,
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

fn draw_sector_window(
    ctx: &egui::Context,
    world: &GameWorld,
    window: OpenSectorWindow,
    focused: bool,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
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

    let area_id = egui::Id::new(("original-sector-window", window.sector));
    if focused {
        ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, area_id));
    }
    let area = egui::Area::new(area_id)
        .fixed_pos(screen_position)
        .order(egui::Order::Middle)
        .show(ctx, |ui| {
            let (pointer, primary_down) = ctx.input(|input| {
                (
                    input.pointer.interact_pos(),
                    input.pointer.button_down(egui::PointerButton::Primary),
                )
            });
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
            for system_key in &sector.systems {
                let Some(system) = world.systems.get(*system_key) else {
                    continue;
                };
                let (planet_x, planet_y) =
                    sector_planet_position(sector.x, sector.y, system.x, system.y);
                let planet_rect =
                    logical_rect(window_rect, layout.scale, planet_x, planet_y, 37.0, 37.0);
                let planet_response = ui.interact(
                    planet_rect,
                    ui.id().with((window.sector, *system_key)),
                    egui::Sense::click(),
                );
                let planet_hovered = pointer.is_some_and(|point| rect_contains(planet_rect, point));
                let planet_clicked = exact_clicked(&planet_response, planet_rect);
                let planet_double_clicked = planet_response.double_clicked() && planet_clicked;
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
                    system_name_color(system.control, player),
                );

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
    let relative_x = system_x.saturating_sub(sector_x) as f32;
    let relative_y = system_y.saturating_sub(sector_y) as f32;
    (
        (relative_x / 13.0 * 37.0).round(),
        (relative_y / 10.0 * 37.0).round(),
    )
}

fn planet_picture_id(dat_id: DatId) -> u8 {
    dat_id
        .index()
        .checked_sub(100)
        .and_then(|index| SYSTEM_PLANET_PICTURES.get(index as usize))
        .copied()
        .unwrap_or(1)
}

fn planet_resource_id(dat_id: DatId) -> u32 {
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

fn system_name_color(control: ControlKind, player: Faction) -> egui::Color32 {
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
    if friendly > hostile {
        egui::Color32::from_rgb(0, 255, 64)
    } else if hostile > friendly {
        egui::Color32::from_rgb(255, 32, 32)
    } else {
        egui::Color32::YELLOW
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
        .map(|texture| texture.id())
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
        .map(|texture| texture.id())
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
        .map(|texture| texture.id())
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
        assert_eq!(planet_resource_id(DatId::new(100)), 10212);
        assert_eq!(planet_resource_id(DatId::new(263)), 10224);
        assert_eq!(planet_resource_id(DatId::new(269)), 10215);
        assert_eq!(planet_resource_id(DatId::new(271)), 10239);
        assert_eq!(planet_resource_id(DatId::new(279)), 10237);
        assert_eq!(planet_resource_id(DatId::new(282)), 10238);
        assert_eq!(planet_resource_id(DatId::new(99)), 10212);
    }

    #[test]
    fn sector_relative_coordinates_match_recovered_divisors() {
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
}
