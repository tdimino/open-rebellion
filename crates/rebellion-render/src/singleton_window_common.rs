//! Shared manager routing for UI surfaces that publish one window at a time.

use egui_macroquad::egui;
use rebellion_windowing::{ContentReport, ContentStatus, RoutingSnapshot, WindowId, WindowKind};

pub(crate) struct PopupCanvas {
    pub(crate) frame: egui::Rect,
    pub(crate) painter: egui::Painter,
    pub(crate) scale: f32,
}

impl PopupCanvas {
    pub(crate) fn rect(&self, x: f32, y: f32, width: f32, height: f32) -> egui::Rect {
        crate::native_geometry::logical_rect(self.frame, self.scale, x, y, width, height)
    }
}

pub(crate) fn show_popup_canvas<R>(
    ctx: &egui::Context,
    area_id: egui::Id,
    rect: egui::Rect,
    scale: f32,
    draw: impl FnOnce(&mut egui::Ui, &PopupCanvas) -> R,
) -> egui::InnerResponse<R> {
    crate::native_geometry::show_area(
        ctx,
        area_id,
        rect,
        egui::Order::Tooltip,
        egui::Sense::hover(),
        |ui, frame, painter| {
            let canvas = PopupCanvas {
                frame,
                painter,
                scale,
            };
            draw(ui, &canvas)
        },
    )
}

#[must_use]
pub(crate) fn managed_layer_id(
    current: Option<WindowId>,
    id: WindowId,
    layer_name: &'static str,
) -> Option<egui::LayerId> {
    (current == Some(id))
        .then(|| egui::LayerId::new(egui::Order::Tooltip, egui::Id::new(layer_name)))
}

#[must_use]
pub(crate) fn hit_window(
    current: Option<WindowId>,
    frame: &RoutingSnapshot,
    kind: WindowKind,
    rect: egui::Rect,
    point: (f32, f32),
) -> Option<WindowId> {
    let id = current?;
    let point = egui::pos2(point.0, point.1);
    let contains = point.x >= rect.min.x
        && point.x < rect.max.x
        && point.y >= rect.min.y
        && point.y < rect.max.y;
    (frame.key(id).is_some_and(|key| key.kind == kind) && frame.allows_input(id) && contains)
        .then_some(id)
}

#[must_use]
pub(crate) fn keyboard_owner(
    current: Option<WindowId>,
    frame: &RoutingSnapshot,
) -> Option<WindowId> {
    frame.keyboard_owner_for(current)
}

/// Legacy compatibility hit test for ID-less singleton content.
#[must_use]
pub(crate) fn contains_screen_point(is_open: bool, rect: egui::Rect, point: (f32, f32)) -> bool {
    is_open && crate::native_geometry::rect_contains(rect, egui::pos2(point.0, point.1))
}

#[must_use]
pub(crate) fn content_reports(
    frame: &RoutingSnapshot,
    kind: WindowKind,
    is_ready: impl Fn(WindowId) -> bool,
) -> Vec<ContentReport> {
    frame.content_reports_for(kind, |id| {
        if is_ready(id) {
            ContentStatus::Ready
        } else {
            ContentStatus::Unpaintable
        }
    })
}

macro_rules! impl_singleton_window_routing {
    ($state:ty, $kind:path, $layer_name:literal, $rect:path) => {
        impl $state {
            /// Egui layer attached to the exact manager-issued window.
            #[must_use]
            pub fn managed_layer_id(
                &self,
                id: rebellion_windowing::WindowId,
            ) -> Option<egui_macroquad::egui::LayerId> {
                crate::singleton_window_common::managed_layer_id(self.window_id(), id, $layer_name)
            }

            /// Manager-owned window hit for event-local pointer routing.
            #[must_use]
            pub fn hit_window(
                &self,
                frame: &rebellion_windowing::RoutingSnapshot,
                layout: crate::cockpit::CockpitLayout,
                point: (f32, f32),
            ) -> Option<rebellion_windowing::WindowId> {
                crate::singleton_window_common::hit_window(
                    self.window_id(),
                    frame,
                    $kind,
                    $rect(layout),
                    point,
                )
            }

            /// Exact manager identity holding this frame's keyboard lease.
            #[must_use]
            pub fn keyboard_owner(
                &self,
                frame: &rebellion_windowing::RoutingSnapshot,
            ) -> Option<rebellion_windowing::WindowId> {
                crate::singleton_window_common::keyboard_owner(self.window_id(), frame)
            }
        }
    };
}

pub(crate) use impl_singleton_window_routing;

macro_rules! impl_singleton_contains_screen_point {
    ($state:ty, $rect:path) => {
        impl $state {
            /// Whether `point` lies inside this open singleton surface.
            #[must_use]
            pub fn contains_screen_point(
                &self,
                layout: crate::cockpit::CockpitLayout,
                point: (f32, f32),
            ) -> bool {
                crate::singleton_window_common::contains_screen_point(
                    self.window.is_some(),
                    $rect(layout),
                    point,
                )
            }
        }
    };
}

pub(crate) use impl_singleton_contains_screen_point;

macro_rules! impl_singleton_content_reports {
    ($state:ty, $kind:path) => {
        impl $state {
            /// Explicit content state for fail-closed manager reconciliation.
            #[must_use]
            pub fn content_reports(
                &self,
                frame: &rebellion_windowing::RoutingSnapshot,
            ) -> Vec<rebellion_windowing::ContentReport> {
                crate::singleton_window_common::content_reports(frame, $kind, |id| {
                    self.window_id() == Some(id)
                })
            }
        }
    };
}

pub(crate) use impl_singleton_content_reports;
