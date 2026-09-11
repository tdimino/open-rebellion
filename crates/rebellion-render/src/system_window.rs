//! Original detailed system windows and their command-center reference rail.
//!
//! `REBEXE.EXE` creates a 226 by 304 client surface inside a 231-pixel title
//! frame. Six bitmap tabs select personnel, fleet, defense, manufacturing,
//! troop, and production content. Modeless windows can be minimized into the
//! faction-specific 12-slot rail and restored without creating duplicates.

use egui_macroquad::egui;
use rebellion_core::dat::{ExplorationStatus, Faction};
use rebellion_core::ids::SystemKey;
use rebellion_core::world::{ControlKind, GameWorld};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};

pub const SYSTEM_WINDOW_CLIENT_WIDTH: f32 = 226.0;
pub const SYSTEM_WINDOW_WIDTH: f32 = 231.0;
pub const SYSTEM_WINDOW_HEIGHT: f32 = 304.0;
pub const REFERENCE_RAIL_SLOTS: usize = 12;

const WINDOW_BACKGROUND: u32 = 10297;
const CLOSE_NORMAL: u32 = 10108;
const CLOSE_PRESSED: u32 = 10109;
const MINIMIZE_NORMAL: u32 = 10253;
const MINIMIZE_PRESSED: u32 = 10254;
const SECTOR_NORMAL: u32 = 10209;
const SECTOR_PRESSED: u32 = 10208;

const TITLE_HOSTILE_ACTIVE: u32 = 10299;
const TITLE_HOSTILE_INACTIVE: u32 = 10200;
const TITLE_FRIENDLY_ACTIVE: u32 = 10302;
const TITLE_FRIENDLY_INACTIVE: u32 = 10201;
const TITLE_NEUTRAL_ACTIVE: u32 = 10303;
const TITLE_NEUTRAL_INACTIVE: u32 = 10304;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemWindowTab {
    Personnel,
    Fleets,
    Defense,
    Manufacturing,
    Troops,
    Production,
}

impl SystemWindowTab {
    const ALL: [Self; 6] = [
        Self::Personnel,
        Self::Fleets,
        Self::Defense,
        Self::Manufacturing,
        Self::Troops,
        Self::Production,
    ];

    fn x(self) -> f32 {
        match self {
            Self::Personnel => 0.0,
            Self::Fleets => 39.0,
            Self::Defense => 77.0,
            Self::Manufacturing => 115.0,
            Self::Troops => 152.0,
            Self::Production => 190.0,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Personnel => "Personnel",
            Self::Fleets => "Fleets",
            Self::Defense => "Planetary Defenses",
            Self::Manufacturing => "Manufacturing Facilities",
            Self::Troops => "Trooper Regiments",
            Self::Production => "Mines and Refineries",
        }
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
    logical_position: (i16, i16),
    tab: SystemWindowTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RailEntry {
    window: OpenSystemWindow,
}

/// Mutable modeless-window and rail state. The last visible window is focused;
/// the first rail entry occupies slot one and is the oldest minimized window.
#[derive(Debug)]
pub struct SystemWindowState {
    faction: CockpitFaction,
    windows: Vec<OpenSystemWindow>,
    rail: Vec<RailEntry>,
}

impl Default for SystemWindowState {
    fn default() -> Self {
        Self {
            faction: CockpitFaction::Alliance,
            windows: Vec::new(),
            rail: Vec::new(),
        }
    }
}

impl SystemWindowState {
    /// Open at the original logical double-click point, clamped so the client
    /// surface remains inside the recovered galaxy aperture. Existing visible
    /// or minimized windows focus or restore instead of duplicating.
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
            logical_position,
            tab: SystemWindowTab::Personnel,
        });
        true
    }

    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.windows.iter().any(|window| {
            let rect = window_screen_rect(*window, layout);
            point.0 >= rect.min.x
                && point.0 < rect.max.x
                && point.1 >= rect.min.y
                && point.1 < rect.max.y
        })
    }

    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    pub fn rail_count(&self) -> usize {
        self.rail.len()
    }

    pub fn clear(&mut self) {
        self.windows.clear();
        self.rail.clear();
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

    fn close(&mut self, system: SystemKey) {
        self.windows.retain(|window| window.system != system);
    }

    fn minimize(&mut self, system: SystemKey) -> bool {
        let Some(index) = self
            .windows
            .iter()
            .position(|window| window.system == system)
        else {
            return false;
        };
        let window = self.windows.remove(index);
        self.rail.retain(|entry| entry.window.system != system);
        if self.rail.len() == REFERENCE_RAIL_SLOTS {
            self.rail.remove(0);
        }
        self.rail.push(RailEntry { window });
        true
    }

    fn restore(&mut self, system: SystemKey) -> bool {
        let Some(index) = self
            .rail
            .iter()
            .position(|entry| entry.window.system == system)
        else {
            return false;
        };
        let entry = self.rail.remove(index);
        self.windows.push(entry.window);
        true
    }

    fn select_tab(&mut self, system: SystemKey, tab: SystemWindowTab) {
        if let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.system == system)
        {
            window.tab = tab;
        }
        self.focus(system);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemWindowAction {
    FocusSector(SystemKey),
    SelectSystem(SystemKey),
}

#[derive(Default)]
struct WindowDrawResult {
    focus: bool,
    close: bool,
    minimize: bool,
    focus_sector: bool,
    tab: Option<SystemWindowTab>,
}

/// Draw the faction rail and every visible original detailed system window.
pub fn draw_system_windows(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut SystemWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Vec<SystemWindowAction> {
    state.prepare_faction(faction);
    draw_reference_rail(ctx, world, state, faction, layout, cache);

    let windows = state.windows.clone();
    let focused_system = windows.last().map(|window| window.system);
    let mut actions = Vec::new();
    let mut focused = None;
    let mut closed = None;
    let mut minimized = None;
    let mut selected_tab = None;

    for window in windows {
        let result = draw_system_window(
            ctx,
            world,
            window,
            focused_system == Some(window.system),
            faction,
            layout,
            cache,
        );
        if result.focus {
            focused = Some(window.system);
            actions.push(SystemWindowAction::SelectSystem(window.system));
        }
        if result.close {
            closed = Some(window.system);
        }
        if result.minimize {
            minimized = Some(window.system);
        }
        if result.focus_sector {
            actions.push(SystemWindowAction::FocusSector(window.system));
            closed = Some(window.system);
        }
        if let Some(tab) = result.tab {
            selected_tab = Some((window.system, tab));
        }
    }

    if let Some(system) = closed {
        state.close(system);
    } else if let Some(system) = minimized {
        state.minimize(system);
    } else if let Some((system, tab)) = selected_tab {
        state.select_tab(system, tab);
    } else if let Some(system) = focused {
        state.focus(system);
    }
    actions
}

fn draw_reference_rail(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut SystemWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) {
    let entries = state.rail.clone();
    let mut restored = None;
    for (index, entry) in entries.iter().enumerate() {
        let Some(system) = world.systems.get(entry.window.system) else {
            continue;
        };
        let logical = rail_slot_rect(faction, index);
        let screen_rect = cockpit_rect(layout, logical);
        egui::Area::new(egui::Id::new(("original-reference-rail", index)))
            .fixed_pos(screen_rect.min)
            .order(egui::Order::Middle)
            .show(ctx, |ui| {
                let (slot_rect, response) =
                    ui.allocate_exact_size(screen_rect.size(), egui::Sense::click());
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
                    relationship_color(relationship(system.control, cockpit_faction(faction))),
                );
                if exact_clicked(&response, slot_rect) {
                    restored = Some(entry.window.system);
                }
            });
    }
    if let Some(system) = restored {
        state.restore(system);
    }
}

fn draw_system_window(
    ctx: &egui::Context,
    world: &GameWorld,
    window: OpenSystemWindow,
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
    let screen_rect = window_screen_rect(window, layout);
    let relationship = relationship(system.control, cockpit_faction(faction));

    let area_id = egui::Id::new(("original-system-window", window.system));
    if focused {
        ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, area_id));
    }
    let area = egui::Area::new(area_id)
        .fixed_pos(screen_rect.min)
        .order(egui::Order::Middle)
        .show(ctx, |ui| {
            let (pointer, primary_down) = ctx.input(|input| {
                (
                    input.pointer.interact_pos(),
                    input.pointer.button_down(egui::PointerButton::Primary),
                )
            });
            let (local_window, window_response) =
                ui.allocate_exact_size(screen_rect.size(), egui::Sense::click());
            ui.painter()
                .rect_filled(local_window, 0.0, egui::Color32::BLACK);
            paint_resource(
                ui.painter(),
                ctx,
                cache,
                WINDOW_BACKGROUND,
                logical_rect(local_window, layout.scale, 2.0, 0.0, 226.0, 304.0),
            );
            paint_resource(
                ui.painter(),
                ctx,
                cache,
                title_resource(relationship, focused),
                logical_rect(local_window, layout.scale, 0.0, 0.0, 231.0, 16.0),
            );

            let sector_rect = logical_rect(local_window, layout.scale, 3.0, 3.0, 14.0, 14.0);
            let minimize_rect = logical_rect(local_window, layout.scale, 200.0, 3.0, 14.0, 14.0);
            let close_rect = logical_rect(local_window, layout.scale, 214.0, 3.0, 14.0, 14.0);
            let sector_response = ui.interact(
                sector_rect,
                ui.id().with((window.system, "sector")),
                egui::Sense::click(),
            );
            let minimize_response = ui.interact(
                minimize_rect,
                ui.id().with((window.system, "minimize")),
                egui::Sense::click(),
            );
            let close_response = ui.interact(
                close_rect,
                ui.id().with((window.system, "close")),
                egui::Sense::click(),
            );
            let sector_pressed =
                primary_down && pointer.is_some_and(|point| rect_contains(sector_rect, point));
            let minimize_pressed =
                primary_down && pointer.is_some_and(|point| rect_contains(minimize_rect, point));
            let close_pressed =
                primary_down && pointer.is_some_and(|point| rect_contains(close_rect, point));
            paint_resource(
                ui.painter(),
                ctx,
                cache,
                if sector_pressed {
                    SECTOR_PRESSED
                } else {
                    SECTOR_NORMAL
                },
                sector_rect,
            );
            paint_resource(
                ui.painter(),
                ctx,
                cache,
                if minimize_pressed {
                    MINIMIZE_PRESSED
                } else {
                    MINIMIZE_NORMAL
                },
                minimize_rect,
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

            ui.painter().text(
                logical_point(local_window, layout.scale, 115.5, 2.0),
                egui::Align2::CENTER_TOP,
                &system.name,
                egui::FontId::proportional((11.0 * layout.scale).max(7.0)),
                egui::Color32::BLACK,
            );

            for tab in SystemWindowTab::ALL {
                let available = tab_available(world, window.system, tab);
                let tab_rect =
                    logical_rect(local_window, layout.scale, 2.0 + tab.x(), 20.0, 36.0, 33.0);
                let response = ui.interact(
                    tab_rect,
                    ui.id().with((window.system, tab)),
                    if available {
                        egui::Sense::click()
                    } else {
                        egui::Sense::hover()
                    },
                );
                let pressed = available
                    && ((tab == window.tab)
                        || (primary_down
                            && pointer.is_some_and(|point| rect_contains(tab_rect, point))));
                paint_resource(
                    ui.painter(),
                    ctx,
                    cache,
                    tab_resource(tab, relationship, pressed, available),
                    tab_rect,
                );
                if available && exact_clicked(&response, tab_rect) {
                    result.tab = Some(tab);
                    result.focus = true;
                }
            }

            paint_tab_content(
                ui.painter(),
                world,
                window,
                relationship,
                layout.scale,
                local_window,
            );

            result.focus_sector = exact_clicked(&sector_response, sector_rect);
            result.minimize = exact_clicked(&minimize_response, minimize_rect);
            result.close = exact_clicked(&close_response, close_rect);
            if window_response.clicked() || result.focus_sector || result.minimize || result.close {
                result.focus = true;
            }
        });
    if area.response.clicked() {
        result.focus = true;
    }
    result
}

fn paint_tab_content(
    painter: &egui::Painter,
    world: &GameWorld,
    window: OpenSystemWindow,
    relationship: Relationship,
    scale: f32,
    parent: egui::Rect,
) {
    let Some(system) = world.systems.get(window.system) else {
        return;
    };
    let title = if system.exploration_status == ExplorationStatus::Unexplored {
        "Unknown"
    } else {
        window.tab.label()
    };
    painter.text(
        logical_point(parent, scale, 115.0, 58.0),
        egui::Align2::CENTER_TOP,
        title,
        egui::FontId::proportional((11.0 * scale).max(7.0)),
        egui::Color32::LIGHT_GRAY,
    );
    if system.exploration_status == ExplorationStatus::Unexplored {
        return;
    }

    let count = tab_count(world, window.system, window.tab);
    if count == 0 {
        return;
    }
    let summary = match window.tab {
        SystemWindowTab::Personnel => personnel_names(world, window.system),
        SystemWindowTab::Fleets => format!("{} fleet{}", count, plural(count)),
        SystemWindowTab::Defense => format!(
            "{} defense facilit{}",
            count,
            if count == 1 { "y" } else { "ies" }
        ),
        SystemWindowTab::Manufacturing => {
            format!(
                "{} manufacturing facilit{}",
                count,
                if count == 1 { "y" } else { "ies" }
            )
        }
        SystemWindowTab::Troops => format!("{} regiment{}", count, plural(count)),
        SystemWindowTab::Production => format!(
            "{} production facilit{}",
            count,
            if count == 1 { "y" } else { "ies" }
        ),
    };
    painter.text(
        logical_point(parent, scale, 58.0, 79.0),
        egui::Align2::LEFT_TOP,
        summary,
        egui::FontId::proportional((9.0 * scale).max(6.0)),
        relationship_color(relationship),
    );
}

fn personnel_names(world: &GameWorld, system: SystemKey) -> String {
    let mut names: Vec<&str> = world
        .characters
        .values()
        .filter(|character| !character.is_killed && character.current_system == Some(system))
        .map(|character| character.name.as_str())
        .collect();
    names.sort_unstable();
    names.into_iter().take(3).collect::<Vec<_>>().join(", ")
}

fn plural(count: usize) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

fn tab_available(world: &GameWorld, system: SystemKey, tab: SystemWindowTab) -> bool {
    if tab == SystemWindowTab::Personnel {
        return true;
    }
    tab_count(world, system, tab) > 0
}

fn tab_count(world: &GameWorld, system_key: SystemKey, tab: SystemWindowTab) -> usize {
    let Some(system) = world.systems.get(system_key) else {
        return 0;
    };
    if system.exploration_status == ExplorationStatus::Unexplored {
        return 0;
    }
    match tab {
        SystemWindowTab::Personnel => world
            .characters
            .values()
            .filter(|character| {
                !character.is_killed && character.current_system == Some(system_key)
            })
            .count(),
        SystemWindowTab::Fleets => system.fleets.len(),
        SystemWindowTab::Defense => system.defense_facilities.len(),
        SystemWindowTab::Manufacturing => system.manufacturing_facilities.len(),
        SystemWindowTab::Troops => system.ground_units.len() + system.special_forces.len(),
        SystemWindowTab::Production => system.production_facilities.len(),
    }
}

fn tab_resource(
    tab: SystemWindowTab,
    relationship: Relationship,
    pressed: bool,
    available: bool,
) -> u32 {
    if tab == SystemWindowTab::Personnel {
        return match (relationship, pressed) {
            (Relationship::Hostile, true) => 10311,
            (Relationship::Hostile, false) => 10312,
            (Relationship::Friendly, true) => 10314,
            (Relationship::Friendly, false) => 10315,
            (Relationship::Neutral, true) => 10317,
            (Relationship::Neutral, false) => 10318,
        };
    }
    let (pressed_resource, normal_resource, disabled_resource) = match tab {
        SystemWindowTab::Fleets => (10326, 10327, 10328),
        SystemWindowTab::Defense => (10329, 10330, 10331),
        SystemWindowTab::Manufacturing => (10332, 10333, 10334),
        SystemWindowTab::Troops => (10323, 10324, 10325),
        SystemWindowTab::Production => (10320, 10321, 10322),
        SystemWindowTab::Personnel => unreachable!(),
    };
    if !available {
        disabled_resource
    } else if pressed {
        pressed_resource
    } else {
        normal_resource
    }
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

fn title_resource(relationship: Relationship, focused: bool) -> u32 {
    match (relationship, focused) {
        (Relationship::Hostile, true) => TITLE_HOSTILE_ACTIVE,
        (Relationship::Hostile, false) => TITLE_HOSTILE_INACTIVE,
        (Relationship::Friendly, true) => TITLE_FRIENDLY_ACTIVE,
        (Relationship::Friendly, false) => TITLE_FRIENDLY_INACTIVE,
        (Relationship::Neutral, true) => TITLE_NEUTRAL_ACTIVE,
        (Relationship::Neutral, false) => TITLE_NEUTRAL_INACTIVE,
    }
}

fn cockpit_faction(faction: CockpitFaction) -> Faction {
    match faction {
        CockpitFaction::Alliance => Faction::Alliance,
        CockpitFaction::Empire => Faction::Empire,
    }
}

fn clamp_to_galaxy(position: (i16, i16), layout: CockpitLayout) -> (i16, i16) {
    let scale = layout.scale.max(f32::EPSILON);
    let min_x = ((layout.galaxy.x - layout.canvas.x) / scale).round();
    let min_y = ((layout.galaxy.y - layout.canvas.y) / scale).round();
    let max_x = (min_x + layout.galaxy.width / scale - SYSTEM_WINDOW_WIDTH).max(min_x);
    let max_y = (min_y + layout.galaxy.height / scale - SYSTEM_WINDOW_HEIGHT).max(min_y);
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

fn exact_clicked(response: &egui::Response, rect: egui::Rect) -> bool {
    response.clicked()
        && response
            .interact_pointer_pos()
            .is_some_and(|point| rect_contains(rect, point))
}

fn rect_contains(rect: egui::Rect, point: egui::Pos2) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
    use rebellion_core::dat::SectorGroup;
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{Sector, System};

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

    #[test]
    fn opening_clamps_to_faction_galaxy_and_deduplicates() {
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
        assert_eq!(state.windows[0].logical_position, (309, 86));
        assert!(state.open(&world, systems[0], (0, 0), CockpitFaction::Alliance, layout));
        assert_eq!(state.window_count(), 1);
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
        assert_eq!(state.rail[0].window.system, systems[1]);
        assert_eq!(state.rail[11].window.system, systems[12]);
    }

    #[test]
    fn restore_preserves_position_and_tab() {
        let (world, systems) = fixture_world(1);
        let mut state = SystemWindowState::default();
        let layout = layout(CockpitFaction::Alliance, 1.0);
        state.open(
            &world,
            systems[0],
            (100, 60),
            CockpitFaction::Alliance,
            layout,
        );
        state.select_tab(systems[0], SystemWindowTab::Production);
        state.minimize(systems[0]);
        assert!(state.restore(systems[0]));
        assert_eq!(state.window_count(), 1);
        assert_eq!(state.rail_count(), 0);
        assert_eq!(state.windows[0].logical_position, (100, 60));
        assert_eq!(state.windows[0].tab, SystemWindowTab::Production);
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
    fn tab_resources_match_recovered_pressed_normal_and_disabled_sets() {
        assert_eq!(
            tab_resource(
                SystemWindowTab::Personnel,
                Relationship::Hostile,
                true,
                true
            ),
            10311
        );
        assert_eq!(
            tab_resource(
                SystemWindowTab::Personnel,
                Relationship::Friendly,
                false,
                true
            ),
            10315
        );
        assert_eq!(
            tab_resource(
                SystemWindowTab::Personnel,
                Relationship::Neutral,
                true,
                true
            ),
            10317
        );
        assert_eq!(
            tab_resource(SystemWindowTab::Fleets, Relationship::Friendly, true, true),
            10326
        );
        assert_eq!(
            tab_resource(SystemWindowTab::Troops, Relationship::Friendly, false, true),
            10324
        );
        assert_eq!(
            tab_resource(
                SystemWindowTab::Production,
                Relationship::Friendly,
                false,
                false
            ),
            10322
        );
    }

    #[test]
    fn reference_rail_rectangles_match_both_original_factions() {
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
    fn system_window_occlusion_uses_scaled_exclusive_edges() {
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
        assert!(state.contains_screen_point(layout, (671.9, 747.9)));
        assert!(!state.contains_screen_point(layout, (672.0, 140.0)));
        assert!(!state.contains_screen_point(layout, (210.0, 748.0)));
    }
}
