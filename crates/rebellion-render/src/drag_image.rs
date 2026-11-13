//! The picture a held drag carries under the pointer.
//!
//! The original draws the dragged item off screen as the drag begins and
//! moves that copy with the cursor, keeping the point where the item was
//! grabbed under the cursor. `CoolDragList` (`FUN_006083c0`, the Fleet and
//! Defenses lists) and the Sector window (`FUN_004593e0`) share one helper:
//! on the first `WM_MOUSEMOVE` with the button held they copy the item
//! (`FUN_0060dbb0`), set the image's hotspot to the press point
//! (`FUN_0060dcb0`) and show it (`FUN_0060dce0`); every later move redraws it
//! at the cursor less the hotspot (`FUN_0060df00` → `FUN_0060e120`); the
//! release hides it (`FUN_0060de50`).

use egui_macroquad::egui;

use crate::bmp_cache::{BmpCache, DllSource};

/// The most bitmap layers one drag image carries.
pub const MAX_LAYERS: usize = 8;

/// One bitmap of a drag image, at its offset from the item's top-left in
/// 640 by 480 canvas points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragLayer {
    pub source: DllSource,
    pub bitmap: u32,
    pub x: f32,
    pub y: f32,
}

/// The dragged item's bitmaps and where the press grabbed it.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DragImage {
    layers: [Option<DragLayer>; MAX_LAYERS],
    /// The item's top-left less the press, in canvas points: the hotspot,
    /// negated (`FUN_0060e120`).
    offset: egui::Vec2,
    /// The press, in screen points.
    press: egui::Pos2,
}

impl DragImage {
    /// A drag image for an item whose top-left lies at `item_min`, pressed
    /// at `press`, both in screen points, on a canvas drawn at `scale`.
    #[must_use]
    pub fn new(item_min: egui::Pos2, press: egui::Pos2, scale: f32) -> Self {
        let scale = scale.max(f32::EPSILON);
        Self {
            layers: [None; MAX_LAYERS],
            offset: (item_min - press) / scale,
            press,
        }
    }

    /// Add a bitmap at canvas offset (`x`, `y`) from the item's top-left, as
    /// the item itself paints it. Layers past [`MAX_LAYERS`] are dropped.
    #[must_use]
    pub fn with_layer(mut self, source: DllSource, bitmap: u32, x: f32, y: f32) -> Self {
        if let Some(slot) = self.layers.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(DragLayer {
                source,
                bitmap,
                x,
                y,
            });
        }
        self
    }

    /// The image's bitmaps, bottom first.
    pub fn layers(&self) -> impl Iterator<Item = DragLayer> + '_ {
        self.layers.iter().flatten().copied()
    }

    /// Where the item's top-left follows the pointer, in screen points.
    #[must_use]
    pub fn top_left(&self, pointer: egui::Pos2, scale: f32) -> egui::Pos2 {
        pointer + self.offset * scale
    }

    /// Whether the image shows with the pointer at `pointer`: from the first
    /// move after the press (`FUN_006083c0`, `FUN_004593e0` on
    /// `WM_MOUSEMOVE`).
    #[must_use]
    pub fn shown(&self, pointer: egui::Pos2) -> bool {
        pointer != self.press
    }

    /// Paint the image under the pointer, above every window.
    pub fn paint(
        &self,
        ctx: &egui::Context,
        cache: &mut BmpCache,
        scale: f32,
        pointer: egui::Pos2,
    ) {
        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Tooltip,
            egui::Id::new("drag_image"),
        ));
        let at = egui::Rect::from_min_size(self.top_left(pointer, scale), egui::Vec2::ZERO);
        for layer in self.layers() {
            crate::fleet_window::paint_native(
                &painter,
                ctx,
                cache,
                layer.source,
                layer.bitmap,
                at,
                scale,
                layer.x,
                layer.y,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_item_keeps_the_point_where_it_was_grabbed_under_the_pointer() {
        // FUN_0060dcb0 sets the hotspot to the press; FUN_0060e120 draws at
        // the cursor less it. An item at (100, 50) pressed at (110, 58) on a
        // canvas at scale 2 sits at (190, 92) with the pointer at (200, 100).
        let image = DragImage::new(egui::pos2(100.0, 50.0), egui::pos2(110.0, 58.0), 2.0);
        assert_eq!(
            image.top_left(egui::pos2(200.0, 100.0), 2.0),
            egui::pos2(190.0, 92.0)
        );
        assert_eq!(
            image.top_left(egui::pos2(110.0, 58.0), 2.0),
            egui::pos2(100.0, 50.0)
        );
    }

    #[test]
    fn the_image_shows_from_the_first_move_after_the_press() {
        // FUN_006083c0 and FUN_004593e0 start the image on WM_MOUSEMOVE.
        let image = DragImage::new(egui::pos2(0.0, 0.0), egui::pos2(10.0, 10.0), 1.0);
        assert!(!image.shown(egui::pos2(10.0, 10.0)));
        assert!(image.shown(egui::pos2(11.0, 10.0)));
    }

    #[test]
    fn layers_keep_their_order_and_stop_at_the_limit() {
        let mut image = DragImage::new(egui::Pos2::ZERO, egui::Pos2::ZERO, 1.0);
        for bitmap in 0..10 {
            image = image.with_layer(DllSource::Strategy, bitmap, 1.0, 2.0);
        }
        let bitmaps: Vec<_> = image.layers().map(|layer| layer.bitmap).collect();
        assert_eq!(bitmaps, (0..8).collect::<Vec<_>>());
        assert_eq!(
            image.layers().next().map(|layer| (layer.x, layer.y)),
            Some((1.0, 2.0))
        );
    }
}
