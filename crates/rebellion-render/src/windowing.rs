//! Production boundary between immutable window snapshots and egui surfaces.

use std::collections::{BTreeMap, BTreeSet};

use egui_macroquad::egui;
use rebellion_windowing::{FocusTarget, RoutingSnapshot, WindowId};

/// Stable, rank-independent child identity used by the managed host.
#[must_use]
pub fn managed_child_id(window: WindowId) -> egui::Id {
    egui::Id::new(("strategic-window-managed-child", window.get()))
}

/// Projects the manager's single focused identity into egui's persistent area
/// stack. Feature modules expose stable layer IDs but never raise themselves;
/// this adapter is the sole cross-family realization point.
pub fn realize_managed_focus(
    ctx: &egui::Context,
    frame: &RoutingSnapshot,
    mut layers_for: impl FnMut(WindowId) -> Option<Vec<egui::LayerId>>,
) {
    let Some(layers) = frame.focused().and_then(|focus| layers_for(focus.window)) else {
        return;
    };
    for layer in layers {
        ctx.move_to_top(layer);
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
    use rebellion_windowing::{
        Cardinality, ExistingOpenPolicy, FocusOnOpen, FocusRestoration, InputPolicy, OpenPlacement,
        PresentationPolicy, StackBand, SubjectId, WindowCommand, WindowKey, WindowKind,
        WindowManager, WindowOwner, WindowPolicy, WindowSpec, WindowTransition,
    };

    fn open(manager: &mut WindowManager, subject: u64) -> WindowId {
        let spec = WindowSpec {
            key: WindowKey::contextual(WindowKind::System, SubjectId::new(2, subject)),
            owner: WindowOwner::StrategicRoot,
            policy: WindowPolicy {
                cardinality: Cardinality::Subject { max_open: None },
                band: StackBand::Normal,
                placement: OpenPlacement::TopOfBand,
                existing: ExistingOpenPolicy::RaiseRestore,
                input: InputPolicy::Modeless,
                presentation: PresentationPolicy::History,
                focus_on_open: FocusOnOpen::Window,
                restoration: FocusRestoration::PreviousEligible,
            },
        };
        match manager.apply(WindowCommand::Open(spec)).unwrap().transition {
            WindowTransition::Opened { id } => id,
            other => panic!("expected open, got {other:?}"),
        }
    }

    #[test]
    fn surface_geometry_unions_regions_and_uses_event_local_coordinates() {
        let mut manager = WindowManager::new();
        let window = open(&mut manager, 1);
        let mut surfaces = EguiWindowSurfaces::default();
        surfaces.register_hit_rect(
            window,
            egui::Rect::from_min_max(egui::pos2(10.0, 10.0), egui::pos2(20.0, 20.0)),
        );
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
        let mut manager = WindowManager::new();
        let window = open(&mut manager, 1);
        let mut surfaces = EguiWindowSurfaces::default();
        surfaces.register_hit_rect(
            window,
            egui::Rect::from_min_max(egui::pos2(10.0, 10.0), egui::pos2(20.0, 20.0)),
        );
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
    fn focus_adapter_resolves_only_the_managers_exact_focused_identity() {
        let mut manager = WindowManager::new();
        let first = open(&mut manager, 1);
        let second = open(&mut manager, 2);
        manager.apply(WindowCommand::Raise(first)).unwrap();
        let frame = manager.begin_frame();
        let mut resolved = Vec::new();

        realize_managed_focus(&egui::Context::default(), &frame, |window| {
            resolved.push(window);
            Some(vec![egui::LayerId::new(
                egui::Order::Foreground,
                managed_child_id(window),
            )])
        });

        assert_eq!(resolved, [first]);
        assert_ne!(first, second);
    }
}
