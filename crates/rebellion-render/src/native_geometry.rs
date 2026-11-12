//! Shared geometry for controls authored in an original game's native canvas.

use egui_macroquad::egui;

/// Origin of this frame's primary-button press, if one began.
pub(crate) fn primary_press_origin(ctx: &egui::Context) -> Option<egui::Pos2> {
    ctx.input(|input| {
        input
            .pointer
            .button_pressed(egui::PointerButton::Primary)
            .then(|| input.pointer.press_origin())
            .flatten()
    })
}

/// A half-open rectangle in unscaled native-canvas coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct NativeRect {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl NativeRect {
    pub(crate) const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub(crate) fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

/// Scale a rectangle from window-local logical coordinates to screen space.
pub(crate) fn logical_rect(
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

/// Scale a point from window-local logical coordinates to screen space.
pub(crate) fn logical_point(parent: egui::Rect, scale: f32, x: f32, y: f32) -> egui::Pos2 {
    egui::pos2(parent.min.x + x * scale, parent.min.y + y * scale)
}

/// Report a click only when its interaction point is inside `rect`.
pub(crate) fn exact_clicked(response: &egui::Response, rect: egui::Rect) -> bool {
    response.clicked()
        && response
            .interact_pointer_pos()
            .is_some_and(|point| rect_contains(rect, point))
}

/// Half-open containment matching the original Win32 hit-test boundaries.
pub(crate) fn rect_contains(rect: egui::Rect, point: egui::Pos2) -> bool {
    point.x >= rect.min.x && point.x < rect.max.x && point.y >= rect.min.y && point.y < rect.max.y
}

/// Paint a cached texture over the full supplied rectangle.
pub(crate) fn paint_texture_rect(
    painter: &egui::Painter,
    texture_id: egui::TextureId,
    rect: egui::Rect,
) {
    painter.image(
        texture_id,
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
}
