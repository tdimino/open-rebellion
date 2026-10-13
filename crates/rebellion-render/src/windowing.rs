//! Renderer binding for the SW-05A stable-host egui 0.31.1 mechanism.
//!
//! The kernel snapshot owns order, focus, capture and modal eligibility. This
//! adapter owns only current geometry, widget namespaces and paint accounting.
//! It does not normalize events, own gestures, or mutate content lifecycle.

use std::collections::{BTreeMap, BTreeSet};

use egui_macroquad::egui;
use rebellion_windowing::{ControlId, FocusTarget, RouteTarget, RoutingSnapshot, WindowId};

const HOST: &str = "strategic-window-surfaces";

/// Category of one dispatch supplied by the coordinator, independently of paint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SurfaceInput {
    /// Paint without mutating widget input.
    Paint,
    /// Pointer dispatch at its event-local position; capture can route without a hit.
    Pointer(Option<egui::Pos2>),
    /// Keyboard dispatch to the snapshot's focused window/control.
    Keyboard,
}

/// The geometry prepass must not apply feature actions or edit backing content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfacePhase {
    /// Register current widgets using temporary copies of mutable widget values.
    Geometry,
    /// Deliver this dispatch and record actual paint receipts.
    Interactive,
}

/// Permission for custom input reads; built-in widgets are gated before they run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfacePermit {
    /// This window owns the pointer dispatch.
    pub pointer: bool,
    /// This window owns the keyboard dispatch.
    pub keyboard: bool,
}

/// Callback phase and permission, frozen before either egui pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfacePass {
    /// Geometry projection or actual dispatch.
    pub phase: SurfacePhase,
    /// Custom input handlers must honor this permission as well as the phase.
    pub permit: SurfacePermit,
}

/// Invalid strategic canvas cannot safely realize an input barrier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceError {
    /// Canvas is empty, inverted or non-finite; the caller must stop dispatch.
    InvalidCanvas,
    /// Raw widget input disagrees with the category/position; the coordinator must split it.
    MismatchedInput,
}

#[derive(Clone, Copy, Debug)]
struct Geometry {
    rect: egui::Rect,
    clip: egui::Rect,
}

/// Current-frame renderer accounting, with no ownership of kernel records.
#[derive(Default, Debug)]
pub struct EguiWindowSurfaces {
    geometry: BTreeMap<WindowId, Geometry>,
    hits: BTreeMap<WindowId, Vec<egui::Rect>>,
    controls: BTreeMap<(WindowId, Option<ControlId>), egui::Id>,
    painted: BTreeSet<WindowId>,
    diagnostics: BTreeSet<WindowId>,
}

impl EguiWindowSurfaces {
    /// Discard previous geometry, control mappings and receipts before registration.
    pub fn begin_frame(&mut self) {
        self.geometry.clear();
        self.hits.clear();
        self.controls.clear();
        self.painted.clear();
        self.diagnostics.clear();
    }

    /// Register current layout/clip bounds separately from disjoint hit regions.
    /// Invalid replacement geometry cannot leave the previous rectangle authoritative.
    pub fn register_geometry(
        &mut self,
        window: WindowId,
        rect: egui::Rect,
        clip: egui::Rect,
    ) -> bool {
        if !valid_rect(rect) || !valid_rect(clip) {
            self.geometry.remove(&window);
            self.diagnostics.insert(window);
            return false;
        }
        self.geometry.insert(
            window,
            Geometry {
                rect,
                clip: rect.intersect(clip),
            },
        );
        true
    }

    /// Add one hit component; gaps between components remain transparent.
    pub fn register_hit_rect(&mut self, window: WindowId, rect: egui::Rect) -> bool {
        if !valid_rect(rect) {
            self.diagnostics.insert(window);
            return false;
        }
        self.hits.entry(window).or_default().push(rect);
        true
    }

    /// Return hits in current geometry, independent of registration/paint order.
    #[must_use]
    pub fn hit_windows(&self, point: egui::Pos2) -> BTreeSet<WindowId> {
        self.hits
            .iter()
            .filter_map(|(window, rects)| {
                self.geometry
                    .get(window)
                    .filter(|geometry| {
                        rects
                            .iter()
                            .any(|rect| rect.intersect(geometry.clip).contains(point))
                    })
                    .map(|_| *window)
            })
            .collect()
    }

    /// Stable namespace rooted in the allocated identity, never its current rank.
    #[must_use]
    pub fn widget_id(target: FocusTarget) -> egui::Id {
        let window = egui::Id::new((HOST, "window", target.window));
        target
            .control
            .map_or(window, |control| window.with(("control", control)))
    }

    /// Paint chrome on the shared layer without input-disabled widget fading.
    /// This is the host-painter path proven by SW-05A, with the child's current clip.
    #[must_use]
    pub fn chrome_painter(ui: &egui::Ui) -> egui::Painter {
        let mut painter = ui.painter().clone();
        painter.set_fade_to_color(None);
        painter
    }

    /// Map an actual response ID; callers register controls during geometry projection.
    pub fn register_control(&mut self, target: FocusTarget, widget: egui::Id) {
        self.controls
            .insert((target.window, target.control), widget);
    }

    /// Resolve child focus using current mappings, or fall back within its owner.
    #[must_use]
    pub fn resolved_focus(&self, frame: &RoutingSnapshot) -> Option<(FocusTarget, egui::Id)> {
        let target = frame
            .focused()
            .filter(|target| frame.allows_input(target.window))?;
        if let Some(widget) = self.controls.get(&(target.window, target.control)) {
            return Some((target, *widget));
        }
        let fallback = FocusTarget {
            window: target.window,
            control: None,
        };
        Some((fallback, Self::widget_id(fallback)))
    }

    /// Paint success is diagnostic; absence can never destroy retained content.
    pub fn record_painted(&mut self, id: WindowId) {
        self.painted.insert(id);
    }

    /// Actual receipts since the latest realization, never a content-presence set.
    #[must_use]
    pub fn rendered_ids(&self) -> BTreeSet<WindowId> {
        self.painted.clone()
    }

    /// Identities with invalid or missing current layout geometry.
    #[must_use]
    pub fn diagnostics(&self) -> &BTreeSet<WindowId> {
        &self.diagnostics
    }

    /// Snapshot eligibility; a dispatch still grants input to exactly one recipient.
    #[must_use]
    pub fn allows_input(&self, frame: &RoutingSnapshot, window: WindowId) -> bool {
        frame.allows_input(window)
    }

    /// Realize one coordinator-supplied input category using the proven shared layer.
    ///
    /// Call `begin_frame` and register *current* geometry/hits first. `input`
    /// must contain only this dispatch category; event normalization and gesture
    /// leases belong to the coordinator. Geometry callbacks must use copies for
    /// mutable built-in widget values and defer feature actions to Interactive.
    /// Custom raw-input reads additionally honor `pass.permit`. Register actual
    /// widget response IDs in both passes; record paint only in Interactive.
    /// Use `chrome_painter` for unchanged bitmap/chrome colors beneath a blocker.
    /// All preview texture/platform deltas are retained and final shapes win.
    pub fn run(
        &mut self,
        ctx: &egui::Context,
        frame: &RoutingSnapshot,
        canvas: egui::Rect,
        delivery: SurfaceInput,
        mut input: egui::RawInput,
        mut draw: impl FnMut(&mut egui::Ui, WindowId, SurfacePass, &mut Self),
    ) -> Result<egui::FullOutput, SurfaceError> {
        if !valid_rect(canvas) {
            return Err(SurfaceError::InvalidCanvas);
        }
        if matches!(delivery, SurfaceInput::Pointer(Some(point)) if !point.is_finite()) {
            return Err(SurfaceError::MismatchedInput);
        }
        if !input
            .events
            .iter()
            .all(|event| input_matches(delivery, event))
        {
            return Err(SurfaceError::MismatchedInput);
        }
        let route = match delivery {
            SurfaceInput::Pointer(point) => {
                frame
                    .pointer_route(
                        &point
                            .filter(|p| canvas.contains(*p))
                            .map_or_else(BTreeSet::new, |p| self.hit_windows(p)),
                    )
                    .target
            }
            SurfaceInput::Keyboard => frame.keyboard_route().target,
            SurfaceInput::Paint => RouteTarget::StrategicRoot,
        };
        let recipient = match route {
            RouteTarget::Window(target) => Some(target.window),
            _ => None,
        };
        // Port: one host realizes the boundary on the same layer as owned menus.
        // A distinct higher barrier Area would block legitimate modal descendants.
        let host_rect = if !frame.pointer_allows_root() {
            canvas
        } else if let SurfaceInput::Pointer(Some(point)) = delivery {
            recipient
                .and_then(|id| {
                    self.hits.get(&id).and_then(|rects| {
                        let clip = self.geometry.get(&id)?.clip.intersect(canvas);
                        rects
                            .iter()
                            .map(|r| r.intersect(clip))
                            .find(|r| r.contains(point))
                    })
                })
                .unwrap_or_else(|| egui::Rect::from_min_size(canvas.min, egui::Vec2::ZERO))
        } else {
            egui::Rect::from_min_size(canvas.min, egui::Vec2::ZERO)
        };
        let windows: Vec<_> = frame
            .paint_order()
            .iter()
            .filter_map(|id| {
                if let Some(geometry) = self.geometry.get(id) {
                    Some((*id, *geometry))
                } else {
                    self.diagnostics.insert(*id);
                    None
                }
            })
            .collect();
        self.painted.clear();
        self.controls.clear();
        input.screen_rect = Some(canvas);
        let mut preview = input.clone();
        preview.events = match delivery {
            SurfaceInput::Pointer(Some(point)) => vec![egui::Event::PointerMoved(point)],
            _ => Vec::new(),
        };
        let mut output = ctx.run(preview, |ctx| {
            self.realize(
                ctx,
                &windows,
                canvas,
                host_rect,
                delivery,
                recipient,
                SurfacePhase::Geometry,
                &mut draw,
            )
        });
        if delivery == SurfaceInput::Keyboard {
            if let Some((_, widget)) = self.resolved_focus(frame) {
                ctx.memory_mut(|memory| memory.request_focus(widget));
            }
        }
        let final_output = ctx.run(input, |ctx| {
            self.realize(
                ctx,
                &windows,
                canvas,
                host_rect,
                delivery,
                recipient,
                SurfacePhase::Interactive,
                &mut draw,
            )
        });
        output.append(final_output);
        Ok(output)
    }

    #[allow(clippy::too_many_arguments)] // One private projection of the frozen prepared dispatch.
    fn realize(
        &mut self,
        ctx: &egui::Context,
        windows: &[(WindowId, Geometry)],
        canvas: egui::Rect,
        host_rect: egui::Rect,
        delivery: SurfaceInput,
        recipient: Option<WindowId>,
        phase: SurfacePhase,
        draw: &mut impl FnMut(&mut egui::Ui, WindowId, SurfacePass, &mut Self),
    ) {
        let layer = egui::LayerId::new(egui::Order::Middle, egui::Id::new(HOST));
        egui::Area::new(egui::Id::new(HOST))
            .order(egui::Order::Middle)
            .fixed_pos(host_rect.min)
            .default_size(host_rect.size())
            .sense(egui::Sense::hover())
            .constrain(false)
            .fade_in(false)
            .show(ctx, |host| {
                host.set_min_size(host_rect.size());
                host.set_clip_rect(canvas);
                for (id, geometry) in windows {
                    let permit = SurfacePermit {
                        pointer: recipient == Some(*id)
                            && matches!(delivery, SurfaceInput::Pointer(_)),
                        keyboard: recipient == Some(*id) && delivery == SurfaceInput::Keyboard,
                    };
                    let mut builder = egui::UiBuilder::new()
                        .layer_id(layer)
                        .max_rect(geometry.rect)
                        .layout(egui::Layout::top_down(egui::Align::Min))
                        .sense(egui::Sense::hover());
                    if !permit.pointer && !permit.keyboard {
                        builder = builder.disabled();
                    }
                    let mut child = egui::Ui::new(
                        ctx.clone(),
                        Self::widget_id(FocusTarget {
                            window: *id,
                            control: None,
                        }),
                        builder,
                    );
                    child.set_clip_rect(geometry.clip.intersect(canvas));
                    draw(&mut child, *id, SurfacePass { phase, permit }, self);
                }
            });
    }
}

fn valid_rect(rect: egui::Rect) -> bool {
    rect.is_finite() && rect.is_positive()
}

fn input_matches(delivery: SurfaceInput, event: &egui::Event) -> bool {
    match event {
        // Metadata does not grant pointer or keyboard mutation to any child.
        egui::Event::WindowFocused(_) | egui::Event::Screenshot { .. } => true,
        egui::Event::Copy
        | egui::Event::Cut
        | egui::Event::Paste(_)
        | egui::Event::Text(_)
        | egui::Event::Key { .. }
        | egui::Event::Ime(_) => delivery == SurfaceInput::Keyboard,
        egui::Event::PointerMoved(pos)
        | egui::Event::PointerButton { pos, .. }
        | egui::Event::Touch { pos, .. } => {
            matches!(delivery, SurfaceInput::Pointer(Some(expected)) if expected == *pos)
        }
        egui::Event::MouseMoved(_)
        | egui::Event::PointerGone
        | egui::Event::MouseWheel { .. }
        | egui::Event::Zoom(_) => matches!(delivery, SurfaceInput::Pointer(_)),
    }
}

#[cfg(all(test, feature = "interface-test-fixtures"))]
mod tests {
    use super::*;
    use crate::windowing_policy::{contextual_spec, singleton_spec};
    use egui_macroquad::egui;
    use rebellion_windowing::{
        ControlId, SubjectId, WindowCommand, WindowKind, WindowManager, WindowOwner,
    };
    use rebellion_windowing::{FocusTarget, RoutingSnapshot, WindowId};
    use std::collections::BTreeMap;

    const CANVAS: egui::Rect =
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(320.0, 240.0));
    const BODY: egui::Rect =
        egui::Rect::from_min_max(egui::pos2(40.0, 40.0), egui::pos2(220.0, 200.0));

    fn opened(manager: &mut WindowManager, kind: WindowKind, subject: u64) -> WindowId {
        let spec = contextual_spec(kind, SubjectId::new(2, subject), WindowOwner::StrategicRoot);
        manager.apply(WindowCommand::Open(spec)).unwrap();
        manager.id_for(spec.key).unwrap()
    }

    fn trio() -> (WindowManager, [WindowId; 3]) {
        let mut manager = WindowManager::new();
        let ids = [1, 2, 3].map(|n| opened(&mut manager, WindowKind::System, n));
        (manager, ids)
    }

    fn geometry(
        s: &mut EguiWindowSurfaces,
        frame: &RoutingSnapshot,
        rect: egui::Rect,
        clip: egui::Rect,
    ) {
        s.begin_frame();
        for id in frame.paint_order() {
            assert!(s.register_geometry(*id, rect, clip));
            assert!(s.register_hit_rect(*id, rect));
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Widgets {
        child: egui::Id,
        text: egui::Id,
        text_rect: egui::Rect,
        button: egui::Id,
        scroll: egui::Id,
        scroll_rect: egui::Rect,
        content_size: egui::Vec2,
        offset: egui::Vec2,
        clip: egui::Rect,
    }

    #[derive(Default)]
    struct Content {
        text: BTreeMap<WindowId, String>,
        widgets: BTreeMap<WindowId, Widgets>,
        button_hits: Vec<WindowId>,
        raw_hits: Vec<WindowId>,
        keyboard_hits: Vec<WindowId>,
    }

    fn draw(
        ui: &mut egui::Ui,
        id: WindowId,
        pass: SurfacePass,
        surfaces: &mut EguiWindowSurfaces,
        state: &mut Content,
    ) {
        let rect = ui.max_rect();
        EguiWindowSurfaces::chrome_painter(ui).rect_filled(rect, 0.0, marker(id));
        let target = FocusTarget {
            window: id,
            control: Some(ControlId(1)),
        };
        let text_id = EguiWindowSurfaces::widget_id(target);
        let text = state.text.entry(id).or_insert_with(|| "seed".to_owned());
        let mut preview = text.clone();
        let value = if pass.phase == SurfacePhase::Geometry {
            &mut preview
        } else {
            text
        };
        let text_response = ui.put(
            egui::Rect::from_min_size(rect.min + egui::vec2(12.0, 12.0), egui::vec2(112.0, 24.0)),
            egui::TextEdit::singleline(value)
                .id(text_id)
                .desired_width(112.0),
        );
        surfaces.register_control(target, text_response.id);
        let scroll_rect =
            egui::Rect::from_min_size(rect.min + egui::vec2(12.0, 42.0), egui::vec2(112.0, 32.0));
        let scroll = ui
            .scope_builder(
                egui::UiBuilder::new()
                    .id_salt("scroll-scope")
                    .max_rect(scroll_rect),
                |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("scroll")
                        .max_height(32.0)
                        .show(ui, |ui| {
                            for n in 0..12 {
                                ui.label(format!("row {n}"));
                            }
                        })
                },
            )
            .inner;
        let button = ui.put(
            egui::Rect::from_min_size(rect.min + egui::vec2(12.0, 82.0), egui::vec2(112.0, 24.0)),
            egui::Button::new("cockpit"),
        );
        let raw_rect =
            egui::Rect::from_min_size(rect.min + egui::vec2(12.0, 112.0), egui::vec2(112.0, 18.0));
        let raw_clicked = pass.permit.pointer
            && ui.input(|i| {
                i.pointer.primary_clicked()
                    && i.pointer
                        .interact_pos()
                        .is_some_and(|p| raw_rect.contains(p))
            });
        if pass.phase == SurfacePhase::Interactive {
            if button.clicked() {
                state.button_hits.push(id);
            }
            if raw_clicked {
                state.raw_hits.push(id);
            }
            if pass.permit.keyboard && ui.input(|input| input.key_pressed(egui::Key::F2)) {
                state.keyboard_hits.push(id);
            }
            state.widgets.insert(
                id,
                Widgets {
                    child: ui.id(),
                    text: text_response.id,
                    text_rect: text_response.rect,
                    button: button.id,
                    scroll: scroll.id,
                    scroll_rect: scroll.inner_rect,
                    content_size: scroll.content_size,
                    offset: scroll.state.offset,
                    clip: ui.clip_rect(),
                },
            );
            surfaces.record_painted(id);
        }
    }

    fn run(
        ctx: &egui::Context,
        s: &mut EguiWindowSurfaces,
        frame: &RoutingSnapshot,
        state: &mut Content,
        delivery: SurfaceInput,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        s.run(
            ctx,
            frame,
            CANVAS,
            delivery,
            egui::RawInput {
                events,
                ..Default::default()
            },
            |ui, id, pass, s| draw(ui, id, pass, s, state),
        )
        .unwrap()
    }

    fn marker(id: WindowId) -> egui::Color32 {
        egui::Color32::from_rgb((id.get() * 35) as u8, 25, 150)
    }

    fn painted_order(output: &egui::FullOutput, ids: &[WindowId]) -> Vec<WindowId> {
        output
            .shapes
            .iter()
            .filter_map(|s| match &s.shape {
                egui::Shape::Rect(r) => ids.iter().copied().find(|id| marker(*id) == r.fill),
                _ => None,
            })
            .collect()
    }

    fn button(point: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    // Port: re-run the accepted egui 0.31.1 probe permutations against allocated
    // kernel identities. A set of repeated move_to_top calls loses this order.
    #[test]
    fn windowing_adapter_real_snapshot_matches_probe() {
        let (mut manager, ids) = trio();
        let warm = egui::Context::default();
        let mut surfaces = EguiWindowSurfaces::default();
        let mut state = Content::default();
        for order in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            for index in order {
                manager.apply(WindowCommand::Raise(ids[index])).unwrap();
            }
            let frame = manager.begin_frame();
            geometry(&mut surfaces, &frame, BODY, BODY);
            let output = run(
                &warm,
                &mut surfaces,
                &frame,
                &mut state,
                SurfaceInput::Paint,
                vec![],
            );
            assert_eq!(painted_order(&output, &ids), order.map(|i| ids[i]));
            let cold = egui::Context::default();
            geometry(&mut surfaces, &frame, BODY, BODY);
            let cold_output = run(
                &cold,
                &mut surfaces,
                &frame,
                &mut Content::default(),
                SurfaceInput::Paint,
                vec![],
            );
            assert_eq!(painted_order(&cold_output, &ids), order.map(|i| ids[i]));
        }
    }

    #[test]
    fn windowing_geometry_uses_current_rect_union() {
        let (mut manager, [a, _, _]) = trio();
        let mut s = EguiWindowSurfaces::default();
        let left = egui::Rect::from_min_size(egui::pos2(10.0, 10.0), egui::vec2(20.0, 20.0));
        let right = left.translate(egui::vec2(100.0, 0.0));
        assert!(s.register_geometry(a, left.union(right), CANVAS));
        assert!(s.register_hit_rect(a, left));
        assert!(s.register_hit_rect(a, right));
        assert_eq!(s.hit_windows(left.center()), [a].into_iter().collect());
        assert_eq!(s.hit_windows(right.center()), [a].into_iter().collect());
        assert!(s.hit_windows(egui::pos2(60.0, 20.0)).is_empty());
        let ctx = egui::Context::default();
        let _ = run(
            &ctx,
            &mut s,
            &manager.begin_frame(),
            &mut Content::default(),
            SurfaceInput::Pointer(Some(egui::pos2(60.0, 20.0))),
            vec![egui::Event::PointerMoved(egui::pos2(60.0, 20.0))],
        );
        assert!(
            !ctx.wants_pointer_input(),
            "the host must not blanket a disjoint hit gap"
        );
        manager.apply(WindowCommand::Minimize(a)).unwrap();
        s.begin_frame();
        assert!(s.hit_windows(left.center()).is_empty());
        manager.apply(WindowCommand::Restore(a)).unwrap();
        let moved = left.translate(egui::vec2(20.0, 70.0));
        assert!(s.register_geometry(a, moved, moved));
        assert!(s.register_hit_rect(a, moved));
        assert!(s.hit_windows(left.center()).is_empty());
        assert_eq!(s.hit_windows(moved.center()), [a].into_iter().collect());
    }

    #[test]
    fn windowing_adapter_delivers_one_button_and_raw_row_after_blank_and_geometry_change() {
        let (mut manager, ids) = trio();
        let ctx = egui::Context::default();
        let mut s = EguiWindowSurfaces::default();
        let mut state = Content::default();
        geometry(&mut s, &manager.begin_frame(), BODY, BODY);
        let blank = egui::pos2(300.0, 220.0);
        let _ = run(
            &ctx,
            &mut s,
            &manager.begin_frame(),
            &mut state,
            SurfaceInput::Pointer(Some(blank)),
            vec![egui::Event::PointerMoved(blank)],
        );
        assert!(!ctx.wants_pointer_input());
        manager.apply(WindowCommand::Raise(ids[0])).unwrap();
        let frame = manager.begin_frame();
        let moved = BODY.translate(egui::vec2(30.0, 15.0));
        geometry(&mut s, &frame, moved, moved);
        let point = moved.min + egui::vec2(30.0, 94.0);
        let _ = run(
            &ctx,
            &mut s,
            &frame,
            &mut state,
            SurfaceInput::Pointer(Some(point)),
            vec![egui::Event::PointerMoved(point), button(point, true)],
        );
        let _ = run(
            &ctx,
            &mut s,
            &frame,
            &mut state,
            SurfaceInput::Pointer(Some(point)),
            vec![button(point, false)],
        );
        assert_eq!(state.button_hits, vec![ids[0]]);
        assert!(state.raw_hits.is_empty());
        let point = moved.min + egui::vec2(30.0, 120.0);
        let _ = run(
            &ctx,
            &mut s,
            &frame,
            &mut state,
            SurfaceInput::Pointer(Some(point)),
            vec![egui::Event::PointerMoved(point), button(point, true)],
        );
        let _ = run(
            &ctx,
            &mut s,
            &frame,
            &mut state,
            SurfaceInput::Pointer(Some(point)),
            vec![button(point, false)],
        );
        assert_eq!(state.raw_hits, vec![ids[0]]);
    }

    // Original modal eligibility belongs to kernel policy (FUN_00606960).
    // Port adapter proof observes TextEdit's backing string before any action.
    #[test]
    fn windowing_adapter_blocks_text_before_mutation_and_retains_unpainted_barrier() {
        let (mut manager, [a, _, _]) = trio();
        let ctx = egui::Context::default();
        let mut s = EguiWindowSurfaces::default();
        let mut state = Content::default();
        manager
            .apply(WindowCommand::Focus(FocusTarget {
                window: a,
                control: Some(ControlId(1)),
            }))
            .unwrap();
        geometry(&mut s, &manager.begin_frame(), BODY, BODY);
        let _ = run(
            &ctx,
            &mut s,
            &manager.begin_frame(),
            &mut state,
            SurfaceInput::Keyboard,
            vec![],
        );
        let modal_spec = singleton_spec(WindowKind::Encyclopedia, WindowOwner::StrategicRoot);
        manager.apply(WindowCommand::Open(modal_spec)).unwrap();
        let modal = manager.id_for(modal_spec.key).unwrap();
        let frame = manager.begin_frame();
        s.begin_frame();
        assert!(!s.allows_input(&frame, a));
        assert!(s.allows_input(&frame, modal));
        assert!(s.register_geometry(a, BODY, BODY));
        assert!(s.register_hit_rect(a, BODY));
        let _ = run(
            &ctx,
            &mut s,
            &frame,
            &mut state,
            SurfaceInput::Keyboard,
            vec![egui::Event::Text("forbidden".into())],
        );
        assert_eq!(state.text[&a], "seed");
        assert!(egui::TextEdit::load_state(
            &ctx,
            EguiWindowSurfaces::widget_id(FocusTarget {
                window: a,
                control: Some(ControlId(1))
            })
        )
        .is_some());
        assert!(!s.rendered_ids().contains(&modal));
        assert!(
            manager.begin_frame().key(modal).is_some(),
            "paint accounting must not remove content"
        );
        assert!(!s.register_geometry(modal, egui::Rect::NAN, BODY));
        assert!(!s.diagnostics().is_empty());
        let blank = egui::pos2(300.0, 220.0);
        let _ = run(
            &ctx,
            &mut s,
            &frame,
            &mut state,
            SurfaceInput::Pointer(Some(blank)),
            vec![egui::Event::PointerMoved(blank)],
        );
        assert!(
            ctx.wants_pointer_input(),
            "missing geometry must not open the map beneath a modal"
        );
    }

    #[test]
    fn windowing_adapter_preserves_actual_widget_geometry_and_nonzero_scroll_across_rank() {
        let (mut manager, ids) = trio();
        let ctx = egui::Context::default();
        let mut s = EguiWindowSurfaces::default();
        let mut state = Content::default();
        geometry(&mut s, &manager.begin_frame(), BODY, BODY);
        let _ = run(
            &ctx,
            &mut s,
            &manager.begin_frame(),
            &mut state,
            SurfaceInput::Paint,
            vec![],
        );
        let before = state.widgets.clone();
        let a = ids[0];
        assert_ne!(before[&a].child, before[&ids[1]].child);
        assert_ne!(before[&a].text, before[&ids[1]].text);
        assert_ne!(before[&a].button, before[&ids[1]].button);
        let mut scroll = egui::scroll_area::State::load(&ctx, before[&a].scroll).unwrap();
        scroll.offset.y = 17.0;
        scroll.store(&ctx, before[&a].scroll);
        manager.apply(WindowCommand::Minimize(ids[1])).unwrap();
        manager.apply(WindowCommand::Raise(a)).unwrap();
        manager
            .apply(WindowCommand::Focus(FocusTarget {
                window: a,
                control: Some(ControlId(1)),
            }))
            .unwrap();
        geometry(&mut s, &manager.begin_frame(), BODY, BODY);
        let _ = run(
            &ctx,
            &mut s,
            &manager.begin_frame(),
            &mut state,
            SurfaceInput::Keyboard,
            vec![egui::Event::Text("+".into())],
        );
        let after = &state.widgets[&a];
        assert_eq!(
            (after.child, after.text, after.button, after.scroll),
            (
                before[&a].child,
                before[&a].text,
                before[&a].button,
                before[&a].scroll
            )
        );
        assert_eq!(after.offset.y, 17.0);
        assert!(after.content_size.y > after.scroll_rect.height());
        assert_eq!(after.text_rect, before[&a].text_rect);
        assert_eq!(after.clip, BODY);
        assert_ne!(
            state.text[&a], "seed",
            "keyboard must reach the mapped TextEdit"
        );
        assert_eq!(
            s.rendered_ids(),
            manager
                .begin_frame()
                .paint_order()
                .iter()
                .copied()
                .collect()
        );
    }

    #[test]
    fn windowing_adapter_missing_control_falls_back_locally_and_modal_owned_menu_is_usable() {
        let mut manager = WindowManager::new();
        let spec = singleton_spec(WindowKind::Encyclopedia, WindowOwner::StrategicRoot);
        manager.apply(WindowCommand::Open(spec)).unwrap();
        let modal = manager.id_for(spec.key).unwrap();
        let menu_spec = singleton_spec(WindowKind::Menu, WindowOwner::Window(modal));
        manager.apply(WindowCommand::Open(menu_spec)).unwrap();
        let menu = manager.id_for(menu_spec.key).unwrap();
        manager
            .apply(WindowCommand::Focus(FocusTarget {
                window: modal,
                control: Some(ControlId(9)),
            }))
            .unwrap();
        let mut s = EguiWindowSurfaces::default();
        assert!(s.allows_input(&manager.begin_frame(), menu));
        s.register_control(
            FocusTarget {
                window: menu,
                control: Some(ControlId(9)),
            },
            egui::Id::new("unrelated"),
        );
        assert_eq!(
            s.resolved_focus(&manager.begin_frame()),
            Some((
                FocusTarget {
                    window: modal,
                    control: None
                },
                EguiWindowSurfaces::widget_id(FocusTarget {
                    window: modal,
                    control: None
                })
            ))
        );
        let ctx = egui::Context::default();
        let mut state = Content::default();
        geometry(&mut s, &manager.begin_frame(), BODY, BODY);
        let point = BODY.min + egui::vec2(30.0, 94.0);
        let _ = run(
            &ctx,
            &mut s,
            &manager.begin_frame(),
            &mut state,
            SurfaceInput::Pointer(Some(point)),
            vec![egui::Event::PointerMoved(point), button(point, true)],
        );
        let _ = run(
            &ctx,
            &mut s,
            &manager.begin_frame(),
            &mut state,
            SurfaceInput::Pointer(Some(point)),
            vec![button(point, false)],
        );
        assert_eq!(state.button_hits, vec![menu]);
        s.begin_frame();
        assert!(s.rendered_ids().is_empty());
        assert!(
            manager.begin_frame().key(modal).is_some() && manager.begin_frame().key(menu).is_some()
        );
        assert_eq!(
            s.resolved_focus(&manager.begin_frame()).unwrap().0.control,
            None
        );
    }

    #[test]
    fn windowing_adapter_rejects_mismatched_input_before_any_widget_runs() {
        let (manager, _) = trio();
        let ctx = egui::Context::default();
        let mut s = EguiWindowSurfaces::default();
        geometry(&mut s, &manager.begin_frame(), BODY, BODY);
        for (delivery, event) in [
            (SurfaceInput::Keyboard, button(BODY.center(), true)),
            (
                SurfaceInput::Pointer(Some(BODY.center())),
                egui::Event::Text("wrong recipient".into()),
            ),
            (
                SurfaceInput::Paint,
                egui::Event::Text("paint cannot edit".into()),
            ),
            (
                SurfaceInput::Pointer(Some(BODY.center())),
                button(egui::pos2(300.0, 220.0), true),
            ),
        ] {
            let mut calls = 0;
            let result = s.run(
                &ctx,
                &manager.begin_frame(),
                CANVAS,
                delivery,
                egui::RawInput {
                    events: vec![event],
                    ..Default::default()
                },
                |_, _, _, _| calls += 1,
            );
            assert!(
                result.is_err(),
                "mixed dispatch must be split by the coordinator"
            );
            assert_eq!(calls, 0);
        }
        let mut calls = 0;
        assert!(s
            .run(
                &ctx,
                &manager.begin_frame(),
                CANVAS,
                SurfaceInput::Pointer(Some(egui::pos2(f32::NAN, 0.0))),
                egui::RawInput::default(),
                |_, _, _, _| calls += 1
            )
            .is_err());
        assert_eq!(calls, 0);
    }

    #[test]
    fn windowing_adapter_rebuilds_control_mapping_from_current_projection() {
        let (mut manager, [a, _, _]) = trio();
        manager
            .apply(WindowCommand::Focus(FocusTarget {
                window: a,
                control: Some(ControlId(1)),
            }))
            .unwrap();
        let frame = manager.begin_frame();
        let mut s = EguiWindowSurfaces::default();
        geometry(&mut s, &frame, BODY, BODY);
        s.register_control(
            FocusTarget {
                window: a,
                control: Some(ControlId(1)),
            },
            egui::Id::new("removed-control"),
        );
        let _ = s
            .run(
                &egui::Context::default(),
                &frame,
                CANVAS,
                SurfaceInput::Keyboard,
                egui::RawInput::default(),
                |_, _, _, _| {},
            )
            .unwrap();
        assert_eq!(
            s.resolved_focus(&frame).unwrap().0,
            FocusTarget {
                window: a,
                control: None
            }
        );
    }

    #[test]
    fn windowing_geometry_invalid_replacement_and_frame_reset_do_not_reuse_old_regions() {
        let (manager, [a, _, _]) = trio();
        let mut s = EguiWindowSurfaces::default();
        assert!(s.register_geometry(a, BODY, BODY));
        assert!(s.register_hit_rect(a, BODY));
        assert!(!s.register_geometry(a, egui::Rect::NAN, BODY));
        assert!(s.hit_windows(BODY.center()).is_empty());
        assert_eq!(s.diagnostics(), &[a].into_iter().collect());
        s.begin_frame();
        assert!(s.diagnostics().is_empty());
        assert!(!s.register_hit_rect(a, egui::Rect::NAN));
        assert!(!s.register_hit_rect(
            a,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::ZERO)
        ));
        assert!(!s.register_geometry(
            a,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(f32::INFINITY, 100.0)),
            BODY
        ));
        assert!(s.hit_windows(BODY.center()).is_empty());
        let ctx = egui::Context::default();
        let mut calls = 0;
        let result = s.run(
            &ctx,
            &manager.begin_frame(),
            egui::Rect::NAN,
            SurfaceInput::Paint,
            egui::RawInput::default(),
            |_, _, _, _| calls += 1,
        );
        assert!(result.is_err());
        assert_eq!(calls, 0);
    }

    #[test]
    fn windowing_geometry_resized_restore_matches_cold_geometry_and_actual_clipping() {
        let (mut manager, ids) = trio();
        let warm = egui::Context::default();
        let mut s = EguiWindowSurfaces::default();
        let mut state = Content::default();
        geometry(&mut s, &manager.begin_frame(), BODY, BODY);
        let _ = run(
            &warm,
            &mut s,
            &manager.begin_frame(),
            &mut state,
            SurfaceInput::Paint,
            vec![],
        );
        manager.apply(WindowCommand::Minimize(ids[0])).unwrap();
        manager.apply(WindowCommand::Restore(ids[0])).unwrap();
        let frame = manager.begin_frame();
        let canvas = CANVAS.expand(30.0);
        let moved = BODY.translate(egui::vec2(50.0, 20.0));
        let clip = egui::Rect::from_min_max(moved.min, moved.max - egui::vec2(32.0, 36.0));
        geometry(&mut s, &frame, moved, clip);
        let output = s
            .run(
                &warm,
                &frame,
                canvas,
                SurfaceInput::Paint,
                egui::RawInput::default(),
                |ui, id, pass, s| draw(ui, id, pass, s, &mut state),
            )
            .unwrap();
        let cold = egui::Context::default();
        let mut cold_state = Content::default();
        geometry(&mut s, &frame, moved, clip);
        let cold_output = s
            .run(
                &cold,
                &frame,
                canvas,
                SurfaceInput::Paint,
                egui::RawInput::default(),
                |ui, id, pass, s| draw(ui, id, pass, s, &mut cold_state),
            )
            .unwrap();
        assert_eq!(painted_order(&output, &ids), vec![ids[1], ids[2], ids[0]]);
        assert_eq!(
            painted_order(&cold_output, &ids),
            vec![ids[1], ids[2], ids[0]]
        );
        assert_eq!(state.widgets, cold_state.widgets);
        for widget in state.widgets.values() {
            assert_eq!(widget.clip, clip);
        }
        assert!(s.hit_windows(moved.max - egui::vec2(5.0, 5.0)).is_empty());
        assert!(s.hit_windows(BODY.min + egui::vec2(3.0, 3.0)).is_empty());
    }

    #[test]
    fn windowing_adapter_retains_preview_textures_and_final_shapes() {
        let (manager, [a, _, _]) = trio();
        let ctx = egui::Context::default();
        let frame = manager.begin_frame();
        let mut s = EguiWindowSurfaces::default();
        s.register_geometry(a, BODY, BODY);
        s.register_hit_rect(a, BODY);
        let output = s
            .run(
                &ctx,
                &frame,
                CANVAS,
                SurfaceInput::Paint,
                egui::RawInput::default(),
                |ui, _, pass, _| {
                    let color = if pass.phase == SurfacePhase::Geometry {
                        ui.label("preview allocates the font texture");
                        egui::Color32::RED
                    } else {
                        egui::Color32::GREEN
                    };
                    EguiWindowSurfaces::chrome_painter(ui).rect_filled(BODY, 0.0, color);
                },
            )
            .unwrap();
        assert!(!output.textures_delta.set.is_empty());
        let fills: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Rect(rect) => Some(rect.fill),
                _ => None,
            })
            .collect();
        assert!(fills.contains(&egui::Color32::GREEN));
        assert!(!fills.contains(&egui::Color32::RED));
    }

    #[test]
    fn windowing_adapter_grants_custom_keyboard_reads_to_only_the_frozen_owner() {
        let (mut manager, [a, _, _]) = trio();
        manager
            .apply(WindowCommand::Focus(FocusTarget {
                window: a,
                control: Some(ControlId(1)),
            }))
            .unwrap();
        let frame = manager.begin_frame();
        let mut s = EguiWindowSurfaces::default();
        geometry(&mut s, &frame, BODY, BODY);
        let mut content = Content::default();
        let event = egui::Event::Key {
            key: egui::Key::F2,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        let _ = run(
            &egui::Context::default(),
            &mut s,
            &frame,
            &mut content,
            SurfaceInput::Keyboard,
            vec![event],
        );
        assert_eq!(content.keyboard_hits, vec![a]);
    }

    #[test]
    fn windowing_adapter_primes_hover_dependent_geometry_before_a_new_press() {
        let (manager, [a, _, _]) = trio();
        let frame = manager.begin_frame();
        let ctx = egui::Context::default();
        let mut s = EguiWindowSurfaces::default();
        s.register_geometry(a, BODY, BODY);
        s.register_hit_rect(a, BODY);
        let blank = egui::pos2(300.0, 220.0);
        let point = BODY.min + egui::vec2(30.0, 94.0);
        let mut hits = Vec::new();
        let mut project = |ui: &mut egui::Ui, id, pass: SurfacePass, _: &mut EguiWindowSurfaces| {
            let hovered = ui.input(|i| i.pointer.hover_pos().is_some_and(|p| BODY.contains(p)));
            let offset = if hovered { 82.0 } else { 12.0 };
            let rect = egui::Rect::from_min_size(
                BODY.min + egui::vec2(12.0, offset),
                egui::vec2(112.0, 24.0),
            );
            let response = ui.put(rect, egui::Button::new("hover-dependent control"));
            if pass.phase == SurfacePhase::Interactive && response.clicked() {
                hits.push(id);
            }
        };
        for (position, events) in [
            (blank, vec![egui::Event::PointerMoved(blank)]),
            (
                point,
                vec![egui::Event::PointerMoved(point), button(point, true)],
            ),
            (point, vec![button(point, false)]),
        ] {
            s.run(
                &ctx,
                &frame,
                CANVAS,
                SurfaceInput::Pointer(Some(position)),
                egui::RawInput {
                    events,
                    ..Default::default()
                },
                &mut project,
            )
            .unwrap();
        }
        assert_eq!(hits, vec![a]);
    }
}
