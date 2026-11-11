//! Shared collection mechanics for the strategic detail-window families.
//!
//! The four detail families own different content but share manager identity,
//! z-order, visibility, teardown and pointer-routing rules.

use std::collections::BTreeSet;

use egui_macroquad::egui;
use rebellion_core::ids::SystemKey;
use rebellion_windowing::{
    ContentReport, ContentStatus, RoutingSnapshot, SubjectId, WindowId, WindowKind,
    WindowTransition,
};

use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::system_window::{title_drag_step, TitleDrag};

pub(crate) type WindowIdentity = (Option<WindowId>, Option<SubjectId>, SystemKey);

pub(crate) trait StrategicWindowRecord {
    fn identity(&self) -> WindowIdentity;
    fn set_logical_position(&mut self, position: (i16, i16));
}

macro_rules! impl_strategic_window_record {
    ($record:ty) => {
        impl $crate::strategic_window_common::StrategicWindowRecord for $record {
            fn identity(&self) -> $crate::strategic_window_common::WindowIdentity {
                (self.id, self.subject, self.system)
            }

            fn set_logical_position(&mut self, position: (i16, i16)) {
                self.logical_position = position;
            }
        }
    };
}

pub(crate) use impl_strategic_window_record;

pub(crate) fn managed_layer_id<T: StrategicWindowRecord>(
    windows: &[T],
    id: WindowId,
    area_id: impl Fn(&T) -> egui::Id,
) -> Option<egui::LayerId> {
    windows
        .iter()
        .find(|window| window.identity().0 == Some(id))
        .map(|window| egui::LayerId::new(egui::Order::Foreground, area_id(window)))
}

pub(crate) fn update_title_drag<T: StrategicWindowRecord>(
    windows: &mut [T],
    active: &mut Option<TitleDrag>,
    ctx: &egui::Context,
    layout: CockpitLayout,
    area_id: impl Fn(&T) -> egui::Id,
) {
    let Some(drag) = *active else {
        return;
    };
    let window = windows
        .iter_mut()
        .find(|window| area_id(window) == drag.area);
    match (window, title_drag_step(ctx, layout, drag)) {
        (Some(window), Ok(Some(position))) => window.set_logical_position(position),
        (Some(_), Ok(None)) => {}
        _ => *active = None,
    }
}

pub(crate) fn commit_open<T: StrategicWindowRecord>(
    windows: &mut Vec<T>,
    faction: &mut CockpitFaction,
    requested_faction: CockpitFaction,
    system: SystemKey,
    subject: SubjectId,
    transition: &WindowTransition,
    make_window: impl FnOnce(WindowId, SubjectId) -> T,
    update_existing: impl FnOnce(&mut T),
) -> Option<WindowId> {
    match *transition {
        WindowTransition::Opened { id } => {
            if windows.iter().any(|window| {
                let identity = window.identity();
                identity.0 == Some(id) || identity.1 == Some(subject) || identity.2 == system
            }) {
                return None;
            }
            if windows.is_empty() {
                *faction = requested_faction;
            } else if *faction != requested_faction {
                return None;
            }
            windows.push(make_window(id, subject));
            Some(id)
        }
        WindowTransition::Existing { id, .. } => {
            if *faction != requested_faction {
                return None;
            }
            let window = windows.iter_mut().find(|window| {
                let identity = window.identity();
                identity.0 == Some(id) && identity.1 == Some(subject) && identity.2 == system
            })?;
            update_existing(window);
            Some(id)
        }
        _ => None,
    }
}

pub(crate) fn window_id<T: StrategicWindowRecord>(
    windows: &[T],
    system: SystemKey,
) -> Option<WindowId> {
    windows
        .iter()
        .find(|window| window.identity().2 == system)
        .and_then(|window| window.identity().0)
}

pub(crate) fn window_is_visible<T: StrategicWindowRecord>(
    windows: &[T],
    snapshot: &RoutingSnapshot,
    system: SystemKey,
) -> bool {
    windows.iter().any(|window| {
        let identity = window.identity();
        identity.2 == system && identity.0.is_none_or(|id| snapshot.is_visible(id))
    })
}

pub(crate) fn remove_content<T: StrategicWindowRecord>(windows: &mut Vec<T>, removed: &[WindowId]) {
    windows.retain(|window| window.identity().0.is_none_or(|id| !removed.contains(&id)));
}

pub(crate) fn content_reports<T: StrategicWindowRecord>(
    windows: &[T],
    snapshot: &RoutingSnapshot,
    kind: WindowKind,
    ready: impl Fn(&T, WindowId) -> bool,
) -> Vec<ContentReport> {
    snapshot
        .paint_order()
        .iter()
        .chain(snapshot.presentation_history())
        .copied()
        .filter(|id| snapshot.key(*id).is_some_and(|key| key.kind == kind))
        .map(|id| ContentReport {
            window: id,
            status: if windows
                .iter()
                .any(|window| window.identity().0 == Some(id) && ready(window, id))
            {
                ContentStatus::Ready
            } else {
                ContentStatus::Absent
            },
        })
        .collect()
}

pub(crate) fn contains_shown_screen_point<T: StrategicWindowRecord>(
    windows: &[T],
    point: (f32, f32),
    shown: impl Fn(SystemKey) -> bool,
    screen_rect: impl Fn(&T) -> egui::Rect,
) -> bool {
    let point = egui::pos2(point.0, point.1);
    windows
        .iter()
        .filter(|window| shown(window.identity().2))
        .any(|window| rect_contains(screen_rect(window), point))
}

pub(crate) fn contains_visible_screen_point_where<T: StrategicWindowRecord>(
    windows: &[T],
    snapshot: &RoutingSnapshot,
    point: (f32, f32),
    shown: impl Fn(&T) -> bool,
    screen_rect: impl Fn(&T) -> egui::Rect,
) -> bool {
    let point = egui::pos2(point.0, point.1);
    windows.iter().any(|window| {
        window.identity().0.is_none_or(|id| snapshot.is_visible(id))
            && shown(window)
            && rect_contains(screen_rect(window), point)
    })
}

pub(crate) fn hit_window_ids_where<T: StrategicWindowRecord>(
    windows: &[T],
    snapshot: &RoutingSnapshot,
    point: (f32, f32),
    top_layer: Option<egui::LayerId>,
    shown: impl Fn(WindowId) -> bool,
    area_id: impl Fn(&T) -> egui::Id,
    screen_rect: impl Fn(&T) -> egui::Rect,
) -> BTreeSet<WindowId> {
    let point = egui::pos2(point.0, point.1);
    windows
        .iter()
        .filter(|window| top_layer.is_some_and(|layer| layer.id == area_id(window)))
        .filter_map(|window| {
            window.identity().0.filter(|id| {
                snapshot.is_visible(*id) && shown(*id) && rect_contains(screen_rect(window), point)
            })
        })
        .collect()
}

pub(crate) fn focus<T: StrategicWindowRecord>(windows: &mut Vec<T>, system: SystemKey) -> bool {
    focus_where(windows, |window| window.identity().2 == system)
}

pub(crate) fn focus_where<T>(windows: &mut Vec<T>, matches: impl Fn(&T) -> bool) -> bool {
    let Some(index) = windows.iter().position(matches) else {
        return false;
    };
    let window = windows.remove(index);
    windows.push(window);
    true
}

pub(crate) fn close<T: StrategicWindowRecord>(
    windows: &mut Vec<T>,
    system: SystemKey,
) -> Option<T> {
    let index = windows
        .iter()
        .position(|window| window.identity().2 == system)?;
    Some(windows.remove(index))
}

pub(crate) fn window_mut<T: StrategicWindowRecord>(
    windows: &mut [T],
    system: SystemKey,
) -> Option<&mut T> {
    windows
        .iter_mut()
        .find(|window| window.identity().2 == system)
}

fn rect_contains(rect: egui::Rect, point: egui::Pos2) -> bool {
    point.x >= rect.min.x && point.x < rect.max.x && point.y >= rect.min.y && point.y < rect.max.y
}
