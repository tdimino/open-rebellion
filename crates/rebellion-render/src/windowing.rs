//! Production boundary between immutable window snapshots and egui surfaces.

use std::collections::{BTreeMap, BTreeSet};

use egui_macroquad::egui;
use rebellion_windowing::{FocusTarget, RoutingSnapshot, WindowId};

/// A presentation-family key for cockpit rules that refer to a saved child
/// window. Identity, visibility, focus, and order remain manager-owned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GalaxyChild {
    Fleet(WindowId),
    Sector(rebellion_core::ids::SectorKey),
    System(rebellion_core::ids::SystemKey),
    Defenses(rebellion_core::ids::SystemKey),
    Missions(rebellion_core::ids::SystemKey),
}

/// Stable, rank-independent child identity used by the managed host.
#[must_use]
pub fn managed_child_id(window: WindowId) -> egui::Id {
    egui::Id::new(("strategic-window-managed-child", window.get()))
}

/// Incremental projection of the manager's immutable bottom-to-top order into
/// egui's persistent area stack.
///
/// Egui batches every [`egui::Context::move_to_top`] request made during one
/// pass and deliberately preserves the requested layers' previous relative
/// order. Raising every managed layer in one pass therefore cannot impose a
/// new order. This projector first raises only the manager's top window, then
/// grows an already-correct suffix by one window per pass. The authoritative
/// top is correct after the first pass and remains top while the rest of the
/// stack converges.
#[derive(Debug, Default)]
pub struct ManagedOrderProjector {
    target: Vec<WindowId>,
    ordered_suffix: usize,
}

impl ManagedOrderProjector {
    fn suffix_for<'a>(&mut self, frame: &'a RoutingSnapshot) -> &'a [WindowId] {
        if self.target != frame.paint_order() {
            self.target = frame.paint_order().to_vec();
            self.ordered_suffix = 0;
        }
        self.ordered_suffix = self
            .ordered_suffix
            .saturating_add(1)
            .min(frame.paint_order().len());
        &frame.paint_order()[frame.paint_order().len() - self.ordered_suffix..]
    }
}

/// Projects the manager's order without allowing keyboard focus to raise a
/// layer. Only `Activate` changes both focus and z-order in the manager.
pub fn realize_managed_order(
    ctx: &egui::Context,
    frame: &RoutingSnapshot,
    projector: &mut ManagedOrderProjector,
    mut layers_for: impl FnMut(WindowId) -> Option<Vec<egui::LayerId>>,
) {
    for &window in projector.suffix_for(frame) {
        if let Some(layers) = layers_for(window) {
            for layer in layers {
                ctx.move_to_top(layer);
            }
        }
    }
}

/// One prepared child in immutable bottom-to-top paint order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManagedSurface {
    /// Session identity of the content surface.
    pub window: WindowId,
    /// Stable egui namespace, independent of rank and frame number.
    pub child_id: egui::Id,
    /// Modal and scene eligibility for this snapshot.
    pub allows_input: bool,
}

/// Current-frame geometry, control mappings, and diagnostic paint receipts.
#[derive(Debug, Default)]
pub struct EguiWindowSurfaces {
    hit_rects: BTreeMap<WindowId, Vec<egui::Rect>>,
    controls: Vec<(FocusTarget, egui::Id)>,
    painted: BTreeSet<WindowId>,
    invalid_geometry: BTreeSet<WindowId>,
}

impl EguiWindowSurfaces {
    /// Clears all frame-local geometry and paint receipts.
    pub fn begin_frame(&mut self) {
        self.hit_rects.clear();
        self.controls.clear();
        self.painted.clear();
        self.invalid_geometry.clear();
    }

    /// Adds one current-frame hit region. Disjoint regions are retained.
    pub fn register_hit_rect(&mut self, window: WindowId, rect: egui::Rect) {
        if !rect.is_finite() || rect.is_negative() {
            self.invalid_geometry.insert(window);
            return;
        }
        self.hit_rects.entry(window).or_default().push(rect);
    }

    /// Associates one semantic focus target with its stable egui widget ID.
    pub fn register_control(&mut self, target: FocusTarget, widget: egui::Id) {
        if let Some((_, registered)) = self
            .controls
            .iter_mut()
            .find(|(registered, _)| *registered == target)
        {
            *registered = widget;
        } else {
            self.controls.push((target, widget));
        }
    }

    /// Looks up a registered egui control without exposing mutable storage.
    #[must_use]
    pub fn control_widget(&self, target: FocusTarget) -> Option<egui::Id> {
        self.controls
            .iter()
            .find_map(|(registered, widget)| (*registered == target).then_some(*widget))
    }

    /// Returns every current-frame surface containing the event-local point.
    #[must_use]
    pub fn hit_windows(&self, point: egui::Pos2) -> BTreeSet<WindowId> {
        self.hit_rects
            .iter()
            .filter_map(|(window, rects)| {
                (!self.invalid_geometry.contains(window)
                    && rects.iter().any(|rect| rect.contains(point)))
                .then_some(*window)
            })
            .collect()
    }

    /// Records a diagnostic paint receipt, separate from content lifecycle.
    pub fn record_painted(&mut self, id: WindowId) {
        self.painted.insert(id);
    }

    /// Returns the diagnostic paint receipts for this frame.
    #[must_use]
    pub fn rendered_ids(&self) -> BTreeSet<WindowId> {
        self.painted.clone()
    }

    /// Returns identities whose reported geometry was invalid and ignored.
    #[must_use]
    pub fn invalid_geometry(&self) -> &BTreeSet<WindowId> {
        &self.invalid_geometry
    }

    /// Tests modal and scene eligibility, not event ownership.
    #[must_use]
    pub fn allows_input(&self, frame: &RoutingSnapshot, window: WindowId) -> bool {
        frame.allows_input(window) && !self.invalid_geometry.contains(&window)
    }

    /// Prepares the stable managed-host child sequence for this snapshot.
    #[must_use]
    pub fn realization_plan(&self, frame: &RoutingSnapshot) -> Vec<ManagedSurface> {
        frame
            .paint_order()
            .iter()
            .copied()
            .map(|window| ManagedSurface {
                window,
                child_id: managed_child_id(window),
                allows_input: self.allows_input(frame, window),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_windowing::{InputPolicy, WindowCommand, WindowManager};

    fn open(manager: &mut WindowManager, subject: u64) -> WindowId {
        crate::test_support::open_window(
            manager,
            crate::test_support::routing_window_spec(subject, InputPolicy::Modeless),
            "system",
        )
    }

    fn surfaces_with_hit_rect(window: WindowId) -> EguiWindowSurfaces {
        let mut surfaces = EguiWindowSurfaces::default();
        surfaces.register_hit_rect(
            window,
            egui::Rect::from_min_max(egui::pos2(10.0, 10.0), egui::pos2(20.0, 20.0)),
        );
        surfaces
    }

    fn surface_fixture() -> (WindowManager, WindowId, EguiWindowSurfaces) {
        let mut manager = WindowManager::new();
        let window = open(&mut manager, 1);
        let surfaces = surfaces_with_hit_rect(window);
        (manager, window, surfaces)
    }

    fn assert_layers(
        ctx: &egui::Context,
        ids: &[(WindowId, egui::Id, egui::Vec2); 3],
        outer: usize,
        inner: usize,
    ) {
        let layer = |id| egui::LayerId::new(egui::Order::Foreground, id);
        assert_eq!(
            ctx.layer_id_at(egui::pos2(20.0, 20.0)),
            Some(layer(ids[outer].1))
        );
        assert_eq!(
            ctx.layer_id_at(egui::pos2(50.0, 50.0)),
            Some(layer(ids[inner].1))
        );
    }

    #[test]
    fn surface_geometry_unions_regions_and_uses_event_local_coordinates() {
        let (_, window, mut surfaces) = surface_fixture();
        surfaces.register_hit_rect(
            window,
            egui::Rect::from_min_max(egui::pos2(30.0, 30.0), egui::pos2(40.0, 40.0)),
        );
        assert!(surfaces
            .hit_windows(egui::pos2(15.0, 15.0))
            .contains(&window));
        assert!(surfaces
            .hit_windows(egui::pos2(35.0, 35.0))
            .contains(&window));
        assert!(!surfaces
            .hit_windows(egui::pos2(25.0, 25.0))
            .contains(&window));
    }

    #[test]
    fn managed_child_identity_survives_rank_changes() {
        let mut manager = WindowManager::new();
        let first = open(&mut manager, 1);
        let second = open(&mut manager, 2);
        let surfaces = EguiWindowSurfaces::default();
        let before = surfaces.realization_plan(&manager.begin_frame());
        manager.apply(WindowCommand::Raise(first)).unwrap();
        let after = surfaces.realization_plan(&manager.begin_frame());
        let id = |plan: &[ManagedSurface], window| {
            plan.iter()
                .find(|item| item.window == window)
                .unwrap()
                .child_id
        };
        assert_eq!(id(&before, first), id(&after, first));
        assert_eq!(id(&before, second), id(&after, second));
    }

    #[test]
    fn invalid_geometry_quarantines_all_hit_regions_for_that_window() {
        let (manager, window, mut surfaces) = surface_fixture();
        surfaces.register_hit_rect(
            window,
            egui::Rect::from_min_max(egui::pos2(f32::NAN, 0.0), egui::pos2(1.0, 1.0)),
        );
        assert!(!surfaces
            .hit_windows(egui::pos2(15.0, 15.0))
            .contains(&window));
        assert!(!surfaces.allows_input(&manager.begin_frame(), window));
    }

    #[test]
    fn order_adapter_resolves_the_managers_exact_bottom_to_top_walk() {
        let mut manager = WindowManager::new();
        let first = open(&mut manager, 1);
        let second = open(&mut manager, 2);
        manager.apply(WindowCommand::Activate(first)).unwrap();
        let frame = manager.begin_frame();
        let mut resolved = Vec::new();
        let mut projector = ManagedOrderProjector::default();

        realize_managed_order(
            &egui::Context::default(),
            &frame,
            &mut projector,
            |window| {
                resolved.push(window);
                Some(vec![egui::LayerId::new(
                    egui::Order::Foreground,
                    managed_child_id(window),
                )])
            },
        );

        assert_eq!(resolved, [first]);
        assert_ne!(first, second);
    }

    #[test]
    fn order_projection_corrects_a_stale_overlap_without_displacing_the_top() {
        let mut manager = WindowManager::new();
        let bottom = open(&mut manager, 1);
        let middle = open(&mut manager, 2);
        let top = open(&mut manager, 3);
        let frame = manager.begin_frame();
        let ctx = egui::Context::default();
        let mut projector = ManagedOrderProjector::default();
        let ids = [
            (bottom, managed_child_id(bottom), egui::vec2(80.0, 80.0)),
            (middle, managed_child_id(middle), egui::vec2(60.0, 60.0)),
            (top, managed_child_id(top), egui::vec2(40.0, 40.0)),
        ];
        let layer = |id| egui::LayerId::new(egui::Order::Foreground, id);
        let input = || egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(100.0, 100.0),
            )),
            ..Default::default()
        };
        let draw_reverse = |ctx: &egui::Context| {
            for (_, id, size) in ids.into_iter().rev() {
                egui::Area::new(id)
                    .order(egui::Order::Foreground)
                    .fixed_pos(egui::Pos2::ZERO)
                    .show(ctx, |ui| {
                        ui.allocate_response(size, egui::Sense::click());
                    });
            }
        };
        let realize = |ctx: &egui::Context, projector: &mut ManagedOrderProjector| {
            draw_reverse(ctx);
            realize_managed_order(ctx, &frame, projector, |window| {
                ids.iter()
                    .find_map(|(candidate, id, _)| (*candidate == window).then(|| vec![layer(*id)]))
            });
        };

        let _ = ctx.run(input(), |ctx| draw_reverse(ctx));
        assert_eq!(
            ctx.layer_id_at(egui::pos2(20.0, 20.0)),
            Some(layer(ids[0].1))
        );

        let _ = ctx.run(input(), |ctx| {
            realize(ctx, &mut projector);
        });
        assert_layers(&ctx, &ids, 2, 0);

        let _ = ctx.run(input(), |ctx| {
            realize(ctx, &mut projector);
        });
        assert_layers(&ctx, &ids, 2, 1);
    }
}
