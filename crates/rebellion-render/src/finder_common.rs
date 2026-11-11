//! Shared mechanics for the original strategic Finder windows.
//!
//! Fleet, Personnel and Troop Finder have different rows, tabs, resources and
//! destinations, but the original gives them the same 470 by 330 strategic
//! window frame, name-prefix selection, manager routing and list navigation.

use egui_macroquad::egui;
use rebellion_core::dat::Faction;
use rebellion_windowing::{
    ContentReport, ContentStatus, ControlId, ExistingEffect, FocusTarget, RoutingSnapshot,
    WindowId, WindowKind, WindowTransition,
};

use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::mission_dialog::galaxy_centered_rect;
use crate::system_window::{logical_rect, rect_contains};

pub(crate) const WIDTH: f32 = 470.0;
pub(crate) const HEIGHT: f32 = 330.0;
pub(crate) const BACKGROUND: [u32; 2] = [10335, 10336];
pub(crate) const PANEL_AT: (f32, f32) = (12.0, 13.0);
pub(crate) const RAIL_AT: (f32, f32) = (412.0, 0.0);

pub(crate) type LogicalRect = (f32, f32, f32, f32);

/// One original bitmap control: its logical rectangle and normal/pressed art.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BitmapControl {
    pub(crate) rect: LogicalRect,
    pub(crate) art: (u32, u32),
}

/// The row text used by the original name-prefix selection routine.
pub(crate) trait FinderRowName {
    fn finder_name(&self) -> &str;
}

/// State shared by all three original Finder window families.
#[derive(Debug, Clone)]
pub(crate) struct FinderUiState {
    /// Strategic-window identity accepted at the app commit boundary.
    pub(crate) id: Option<WindowId>,
    pub(crate) faction: CockpitFaction,
    pub(crate) name: String,
    pub(crate) first_row: usize,
    pub(crate) focus_name: bool,
    pub(crate) pending_focus: Option<ControlId>,
    pub(crate) last_clicked: Option<(usize, f64)>,
}

impl FinderUiState {
    pub(crate) fn new(id: Option<WindowId>, faction: CockpitFaction) -> Self {
        Self {
            id,
            faction,
            name: String::new(),
            first_row: 0,
            focus_name: true,
            pending_focus: None,
            last_clicked: None,
        }
    }

    pub(crate) const fn window_id(&self) -> Option<WindowId> {
        self.id
    }

    pub(crate) fn managed_layer_id(
        &self,
        id: WindowId,
        layer_name: &'static str,
    ) -> Option<egui::LayerId> {
        (self.id == Some(id))
            .then(|| egui::LayerId::new(egui::Order::Tooltip, egui::Id::new(layer_name)))
    }

    pub(crate) fn hit_window(
        &self,
        frame: &RoutingSnapshot,
        kind: WindowKind,
        layout: CockpitLayout,
        point: (f32, f32),
    ) -> Option<WindowId> {
        let id = self.id?;
        (frame.key(id).is_some_and(|key| key.kind == kind)
            && frame.allows_input(id)
            && rect_contains(window_rect(layout), egui::pos2(point.0, point.1)))
        .then_some(id)
    }

    pub(crate) fn keyboard_owner(&self, frame: &RoutingSnapshot) -> Option<WindowId> {
        let id = self.id?;
        matches!(
            frame.keyboard_route().target,
            rebellion_windowing::RouteTarget::Window(target) if target.window == id
        )
        .then_some(id)
    }

    pub(crate) fn take_focus_request(&mut self) -> Option<FocusTarget> {
        Some(FocusTarget {
            window: self.id?,
            control: Some(self.pending_focus.take()?),
        })
    }

    pub(crate) fn contains_screen_point(layout: CockpitLayout, point: (f32, f32)) -> bool {
        rect_contains(window_rect(layout), egui::pos2(point.0, point.1))
    }

    pub(crate) fn reset_list_navigation(&mut self) {
        self.first_row = 0;
        self.last_clicked = None;
    }

    pub(crate) fn repeats_click(&self, row: usize, now: f64, delay: f64) -> bool {
        self.last_clicked
            .is_some_and(|(last, at)| last == row && now - at <= delay)
    }

    pub(crate) fn commit_focus_and_wheel(
        &mut self,
        focus_request: Option<ControlId>,
        wheel: f32,
        row_count: usize,
        visible_rows: usize,
    ) {
        if let Some(control) = focus_request {
            self.pending_focus = Some(control);
        }
        if wheel != 0.0 {
            let last = row_count.saturating_sub(visible_rows);
            self.first_row = if wheel > 0.0 {
                self.first_row.saturating_sub(1)
            } else {
                (self.first_row + 1).min(last)
            };
        }
    }
}

pub(crate) fn content_reports(
    frame: &RoutingSnapshot,
    kind: WindowKind,
    current: Option<WindowId>,
) -> Vec<ContentReport> {
    frame
        .paint_order()
        .iter()
        .copied()
        .filter(|id| frame.key(*id).is_some_and(|key| key.kind == kind))
        .map(|id| ContentReport {
            window: id,
            status: if current == Some(id) {
                ContentStatus::Ready
            } else {
                ContentStatus::Unpaintable
            },
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OpenCommit {
    Publish(WindowId),
    Existing(WindowId),
}

pub(crate) fn accept_singleton_open(
    transition: &WindowTransition,
    current: Option<WindowId>,
) -> Option<OpenCommit> {
    match *transition {
        WindowTransition::Opened { id } if current.is_none() => Some(OpenCommit::Publish(id)),
        WindowTransition::Existing {
            id,
            effect: ExistingEffect::NoOp,
        } if current == Some(id) => Some(OpenCommit::Existing(id)),
        _ => None,
    }
}

pub(crate) fn prefix_choice<R: FinderRowName>(rows: &[R], text: &str) -> Option<usize> {
    let typed: Vec<char> = text.chars().map(|c| c.to_ascii_lowercase()).collect();
    let mut best: Option<(usize, usize)> = None;
    for (index, row) in rows.iter().enumerate() {
        let common = row
            .finder_name()
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .zip(&typed)
            .take_while(|(a, b)| a == *b)
            .count();
        if common > 0 && best.is_none_or(|(_, length)| common > length) {
            best = Some((index, common));
        }
    }
    best.map(|(index, _)| index)
}

#[derive(Clone, Copy)]
pub(crate) struct ListInput<'a> {
    pub(crate) name: &'a str,
    pub(crate) name_changed: bool,
    pub(crate) clicked_row: Option<(usize, f64)>,
    pub(crate) focused_control: Option<ControlId>,
    pub(crate) name_control: ControlId,
    pub(crate) list_control: ControlId,
    pub(crate) arrow_up: bool,
    pub(crate) arrow_down: bool,
}

pub(crate) fn apply_list_input<R: FinderRowName>(
    ui: &mut FinderUiState,
    chosen: &mut Option<usize>,
    rows: &[R],
    input: ListInput<'_>,
    visible_rows: usize,
    focus_request: &mut Option<ControlId>,
) {
    ui.focus_name = false;
    if input.name_changed {
        ui.name.clear();
        ui.name.push_str(input.name);
        *chosen = prefix_choice(rows, input.name);
        if let Some(index) = *chosen {
            ui.first_row = scrolled_to(ui.first_row, index, visible_rows);
        }
    }
    if let Some((row, at)) = input.clicked_row {
        ui.last_clicked = Some((row, at));
        *chosen = Some(row);
        set_name(ui, rows[row].finder_name());
    }
    if input.focused_control == Some(input.name_control) && input.arrow_down && !rows.is_empty() {
        let row = (*chosen).unwrap_or(0).min(rows.len() - 1);
        *chosen = Some(row);
        set_name(ui, rows[row].finder_name());
        ui.first_row = scrolled_to(ui.first_row, row, visible_rows);
        *focus_request = Some(input.list_control);
    }
    if input.focused_control == Some(input.list_control) {
        if rows.is_empty() {
            if input.arrow_up {
                *focus_request = Some(input.name_control);
            }
            return;
        }
        let current = (*chosen).unwrap_or(0).min(rows.len() - 1);
        let next = if input.arrow_down {
            (current + 1).min(rows.len() - 1)
        } else if input.arrow_up && current > 0 {
            current - 1
        } else {
            current
        };
        if input.arrow_up && current == 0 {
            *focus_request = Some(input.name_control);
        } else if input.arrow_up || input.arrow_down {
            *chosen = Some(next);
            set_name(ui, rows[next].finder_name());
            ui.first_row = scrolled_to(ui.first_row, next, visible_rows);
        }
    }
}

fn set_name(ui: &mut FinderUiState, name: &str) {
    ui.name.clear();
    ui.name.push_str(name);
}

pub(crate) const fn side(faction: CockpitFaction) -> usize {
    match faction {
        CockpitFaction::Alliance => 0,
        CockpitFaction::Empire => 1,
    }
}

pub(crate) const fn player_side(faction: CockpitFaction) -> Faction {
    match faction {
        CockpitFaction::Alliance => Faction::Alliance,
        CockpitFaction::Empire => Faction::Empire,
    }
}

/// `FUN_00606980` places each Finder in the galaxy view; the port centers it.
pub(crate) fn window_rect(layout: CockpitLayout) -> egui::Rect {
    galaxy_centered_rect(layout, WIDTH, HEIGHT)
}

pub(crate) fn at(frame: egui::Rect, scale: f32, (x, y, w, h): LogicalRect) -> egui::Rect {
    logical_rect(frame, scale, x, y, w, h)
}

/// The first row that shows `row`, moving the list as little as possible.
pub(crate) const fn scrolled_to(first_row: usize, row: usize, visible_rows: usize) -> usize {
    if row < first_row {
        row
    } else if row >= first_row + visible_rows {
        row + 1 - visible_rows
    } else {
        first_row
    }
}

pub(crate) fn should_remove(id: Option<WindowId>, removed: &[WindowId]) -> bool {
    id.is_some_and(|id| removed.contains(&id))
}
