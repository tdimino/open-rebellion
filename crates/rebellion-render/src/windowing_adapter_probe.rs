//! Test-only feasibility probe for realizing a prepared windowing snapshot in
//! locked `egui` 0.31.1. This deliberately does not introduce a second
//! production window manager.

use std::collections::{HashMap, HashSet};

use egui_macroquad::egui;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ProbeWindow {
    A,
    B,
    C,
}

impl ProbeWindow {
    const ALL: [Self; 3] = [Self::A, Self::B, Self::C];

    const fn name(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }

    const fn marker(self) -> egui::Color32 {
        match self {
            Self::A => egui::Color32::from_rgb(211, 43, 43),
            Self::B => egui::Color32::from_rgb(37, 151, 72),
            Self::C => egui::Color32::from_rgb(44, 92, 205),
        }
    }

    fn area_id(self) -> egui::Id {
        egui::Id::new(("windowing-adapter-rejected-area", self))
    }
}

const CANVAS: egui::Rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(320.0, 240.0));
const OVERLAP: egui::Rect =
    egui::Rect::from_min_max(egui::pos2(40.0, 40.0), egui::pos2(220.0, 180.0));
const HOST_ID: &str = "windowing-adapter-managed-host";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BridgeStrategy {
    ManagedHost,
    RepeatedMoveToTop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UnsupportedBridgeStrategy(BridgeStrategy);

#[derive(Clone, Copy, Debug, PartialEq)]
struct ProbeWindowSpec {
    id: ProbeWindow,
    rect: egui::Rect,
    clip_rect: egui::Rect,
}

impl ProbeWindowSpec {
    fn new(id: ProbeWindow, rect: egui::Rect) -> Self {
        Self {
            id,
            rect,
            clip_rect: rect,
        }
    }

    fn clipped(mut self, clip_rect: egui::Rect) -> Self {
        self.clip_rect = clip_rect;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct InputPermit {
    pointer: bool,
    keyboard: bool,
}

impl InputPermit {
    const BLOCKED: Self = Self {
        pointer: false,
        keyboard: false,
    };

    const ALLOWED: Self = Self {
        pointer: true,
        keyboard: true,
    };

    const fn interactive(self) -> bool {
        self.pointer || self.keyboard
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PreparedWindow {
    id: ProbeWindow,
    rect: egui::Rect,
    clip_rect: egui::Rect,
    permit: InputPermit,
}

#[derive(Clone, Debug, PartialEq)]
struct PreparedFrame {
    canvas: egui::Rect,
    bottom_to_top: Vec<PreparedWindow>,
    input_recipient: Option<ProbeWindow>,
}

#[derive(Debug)]
struct ProbeState {
    text: HashMap<ProbeWindow, String>,
    button_hits: Vec<ProbeWindow>,
    raw_row_hits: Vec<ProbeWindow>,
}

impl Default for ProbeState {
    fn default() -> Self {
        Self {
            text: ProbeWindow::ALL
                .into_iter()
                .map(|id| (id, id.name().to_owned()))
                .collect(),
            button_hits: Vec::new(),
            raw_row_hits: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct RealizedWindow {
    id: ProbeWindow,
    child_id: egui::Id,
    text_id: egui::Id,
    text_rect: egui::Rect,
    scroll_rect: egui::Rect,
    clip_rect: egui::Rect,
}

#[derive(Clone, Debug, PartialEq)]
struct RealizedFrame {
    bottom_to_top: Vec<RealizedWindow>,
    input_recipient: Option<ProbeWindow>,
}

/// Frozen candidate bridge step 1: consume an immutable manager snapshot,
/// perform current-frame hit routing, and issue permits before any widget runs.
fn prepare_adapter_frame(
    strategy: BridgeStrategy,
    canvas: egui::Rect,
    bottom_to_top: &[ProbeWindowSpec],
    pointer: Option<egui::Pos2>,
) -> Result<PreparedFrame, UnsupportedBridgeStrategy> {
    if strategy != BridgeStrategy::ManagedHost {
        return Err(UnsupportedBridgeStrategy(strategy));
    }

    let mut unique = HashSet::with_capacity(bottom_to_top.len());
    assert!(
        bottom_to_top.iter().all(|window| unique.insert(window.id)),
        "the prepared snapshot must contain each window exactly once"
    );

    let input_recipient = pointer.and_then(|point| {
        bottom_to_top
            .iter()
            .rev()
            .find(|window| {
                window.rect.contains(point) && window.clip_rect.intersect(canvas).contains(point)
            })
            .map(|window| window.id)
    });
    let bottom_to_top = bottom_to_top
        .iter()
        .map(|window| PreparedWindow {
            id: window.id,
            rect: window.rect,
            clip_rect: window.clip_rect.intersect(window.rect).intersect(canvas),
            permit: if input_recipient == Some(window.id) {
                InputPermit::ALLOWED
            } else {
                InputPermit::BLOCKED
            },
        })
        .collect();

    Ok(PreparedFrame {
        canvas,
        bottom_to_top,
        input_recipient,
    })
}

/// Frozen candidate bridge step 2: realize all children in one stable layer.
/// Window identity, not rank, salts widget IDs. Call order is paint order.
fn realize_adapter_frame(
    ctx: &egui::Context,
    frame: &PreparedFrame,
    state: &mut ProbeState,
) -> RealizedFrame {
    let layer = egui::LayerId::new(egui::Order::Middle, egui::Id::new(HOST_ID));
    let mut host = egui::Ui::new(
        ctx.clone(),
        egui::Id::new(HOST_ID),
        egui::UiBuilder::new()
            .layer_id(layer)
            .max_rect(frame.canvas)
            .sense(egui::Sense::empty()),
    );
    let mut realized = Vec::with_capacity(frame.bottom_to_top.len());

    for window in &frame.bottom_to_top {
        // Paint the window chrome from the host so an input-disabled child is
        // inert without visually fading the window beneath the active one.
        host.painter().with_clip_rect(window.clip_rect).rect_filled(
            window.rect,
            0.0,
            window.id.marker(),
        );
        let mut builder = egui::UiBuilder::new()
            .id_salt(window.id)
            .max_rect(window.rect)
            .layout(egui::Layout::top_down(egui::Align::Min))
            .sense(egui::Sense::hover());
        if !window.permit.interactive() {
            builder = builder.disabled();
        }
        let mut child = host.new_child(builder);
        child.set_clip_rect(window.clip_rect);

        let text_id = child.id().with("text-edit");
        let text_rect = egui::Rect::from_min_size(
            window.rect.min + egui::vec2(12.0, 12.0),
            egui::vec2(112.0, 24.0),
        );
        let text = state
            .text
            .get_mut(&window.id)
            .expect("all probe windows have text state");
        let _ = child.put(
            text_rect,
            egui::TextEdit::singleline(text)
                .id(text_id)
                .desired_width(112.0),
        );

        let scroll_rect = egui::Rect::from_min_size(
            window.rect.min + egui::vec2(12.0, 42.0),
            egui::vec2(112.0, 32.0),
        );
        let _ = child.scope_builder(
            egui::UiBuilder::new()
                .id_salt("scroll-scope")
                .max_rect(scroll_rect),
            |ui| {
                egui::ScrollArea::vertical()
                    .id_salt((window.id, "scroll"))
                    .max_height(scroll_rect.height())
                    .show(ui, |ui| {
                        ui.label("row 1");
                        ui.label("row 2");
                        ui.label("row 3");
                    });
            },
        );

        let button_rect = egui::Rect::from_min_size(
            window.rect.min + egui::vec2(12.0, 82.0),
            egui::vec2(112.0, 24.0),
        );
        if child
            .put(
                button_rect,
                egui::Button::new(format!("{} cockpit", window.id.name())),
            )
            .clicked()
        {
            state.button_hits.push(window.id);
        }

        let raw_row = egui::Rect::from_min_size(
            window.rect.min + egui::vec2(12.0, 112.0),
            egui::vec2(112.0, 18.0),
        );
        child
            .painter()
            .rect_filled(raw_row, 0.0, egui::Color32::from_gray(24));
        let raw_clicked = window.permit.pointer
            && child.input(|input| {
                input.pointer.primary_clicked()
                    && input
                        .pointer
                        .interact_pos()
                        .is_some_and(|point| raw_row.contains(point))
            });
        if raw_clicked {
            state.raw_row_hits.push(window.id);
        }

        realized.push(RealizedWindow {
            id: window.id,
            child_id: child.id(),
            text_id,
            text_rect,
            scroll_rect,
            clip_rect: child.clip_rect(),
        });
    }

    RealizedFrame {
        bottom_to_top: realized,
        input_recipient: frame.input_recipient,
    }
}

fn input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(CANVAS),
        events,
        ..Default::default()
    }
}

fn pointer_button(point: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: point,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

fn specs(order: &[ProbeWindow]) -> Vec<ProbeWindowSpec> {
    order
        .iter()
        .copied()
        .map(|id| ProbeWindowSpec::new(id, OVERLAP))
        .collect()
}

fn run_candidate(
    ctx: &egui::Context,
    state: &mut ProbeState,
    windows: &[ProbeWindowSpec],
    pointer: Option<egui::Pos2>,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, RealizedFrame) {
    let frame = prepare_adapter_frame(BridgeStrategy::ManagedHost, CANVAS, windows, pointer)
        .expect("managed host is the supported probe strategy");
    let mut realized = None;
    let output = ctx.run(input(events), |ctx| {
        realized = Some(realize_adapter_frame(ctx, &frame, state));
    });
    (output, realized.expect("the egui pass ran"))
}

fn marker_order(output: &egui::FullOutput) -> Vec<ProbeWindow> {
    let mut order = Vec::new();
    for clipped in &output.shapes {
        if let egui::Shape::Rect(rect) = &clipped.shape {
            if let Some(window) = ProbeWindow::ALL
                .into_iter()
                .find(|window| window.marker() == rect.fill)
            {
                order.push(window);
            }
        }
    }
    order
}

fn marker_clips(output: &egui::FullOutput) -> Vec<(ProbeWindow, egui::Rect)> {
    let mut clips = Vec::new();
    for clipped in &output.shapes {
        if let egui::Shape::Rect(rect) = &clipped.shape {
            if let Some(window) = ProbeWindow::ALL
                .into_iter()
                .find(|window| window.marker() == rect.fill)
            {
                clips.push((window, clipped.clip_rect));
            }
        }
    }
    clips
}

fn render_repeated_raise(ctx: &egui::Context, requested_bottom_to_top: &[ProbeWindow]) {
    let _ = ctx.run(input(Vec::new()), |ctx| {
        for window in requested_bottom_to_top {
            ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, window.area_id()));
            egui::Area::new(window.area_id())
                .order(egui::Order::Middle)
                .fixed_pos(OVERLAP.min)
                .show(ctx, |ui| {
                    ui.set_min_size(OVERLAP.size());
                    ui.label(window.name());
                });
        }
    });
}

fn observed_area(ctx: &egui::Context) -> Option<ProbeWindow> {
    let observed_id = ctx.layer_id_at(OVERLAP.center()).map(|layer| layer.id);
    ProbeWindow::ALL
        .into_iter()
        .find(|window| Some(window.area_id()) == observed_id)
}

fn text_id(realized: &RealizedFrame, window: ProbeWindow) -> egui::Id {
    realized
        .bottom_to_top
        .iter()
        .find(|realized| realized.id == window)
        .expect("window was realized")
        .text_id
}

fn focus_text_at_end(ctx: &egui::Context, id: egui::Id, char_count: usize) {
    ctx.memory_mut(|memory| memory.request_focus(id));
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::one(
            egui::text::CCursor::new(char_count),
        )));
    state.store(ctx, id);
}

#[test]
fn repeated_move_to_top_is_rejected_as_bridge_strategy() {
    assert_eq!(
        prepare_adapter_frame(
            BridgeStrategy::RepeatedMoveToTop,
            CANVAS,
            &specs(&ProbeWindow::ALL),
            None,
        ),
        Err(UnsupportedBridgeStrategy(BridgeStrategy::RepeatedMoveToTop))
    );

    let ctx = egui::Context::default();
    render_repeated_raise(&ctx, &ProbeWindow::ALL);
    render_repeated_raise(&ctx, &[ProbeWindow::C, ProbeWindow::A, ProbeWindow::B]);
    assert_eq!(observed_area(&ctx), Some(ProbeWindow::C));
    assert_ne!(observed_area(&ctx), Some(ProbeWindow::B));
}

#[test]
fn adapter_probe_reorders_first_frame() {
    let ctx = egui::Context::default();
    let mut state = ProbeState::default();
    let (output, realized) = run_candidate(
        &ctx,
        &mut state,
        &specs(&[ProbeWindow::C, ProbeWindow::A, ProbeWindow::B]),
        None,
        Vec::new(),
    );

    assert_eq!(
        marker_order(&output),
        vec![ProbeWindow::C, ProbeWindow::A, ProbeWindow::B]
    );
    assert_eq!(
        realized
            .bottom_to_top
            .iter()
            .map(|window| window.id)
            .collect::<Vec<_>>(),
        vec![ProbeWindow::C, ProbeWindow::A, ProbeWindow::B]
    );
    assert_eq!(
        realized.bottom_to_top[0].text_rect,
        egui::Rect::from_min_size(
            OVERLAP.min + egui::vec2(12.0, 12.0),
            egui::vec2(112.0, 24.0),
        )
    );
    assert_eq!(
        realized.bottom_to_top[0].scroll_rect,
        egui::Rect::from_min_size(
            OVERLAP.min + egui::vec2(12.0, 42.0),
            egui::vec2(112.0, 32.0),
        )
    );
}

#[test]
fn adapter_probe_blocks_lower_text_edit() {
    let ctx = egui::Context::default();
    let mut state = ProbeState::default();
    let (_, initial) = run_candidate(
        &ctx,
        &mut state,
        &specs(&[ProbeWindow::A]),
        Some(OVERLAP.center()),
        Vec::new(),
    );
    let id = text_id(&initial, ProbeWindow::A);
    focus_text_at_end(&ctx, id, state.text[&ProbeWindow::A].chars().count());

    let _ = run_candidate(
        &ctx,
        &mut state,
        &specs(&[ProbeWindow::A, ProbeWindow::C]),
        Some(OVERLAP.center()),
        vec![egui::Event::Text(" blocked".to_owned())],
    );

    assert_eq!(state.text[&ProbeWindow::A], "A");
    assert!(egui::TextEdit::load_state(&ctx, id).is_some());
}

#[test]
fn adapter_probe_preserves_widget_state_and_blank_map() {
    let ctx = egui::Context::default();
    let mut state = ProbeState::default();
    let (_, initial) = run_candidate(
        &ctx,
        &mut state,
        &specs(&ProbeWindow::ALL),
        Some(OVERLAP.center()),
        Vec::new(),
    );
    let initial_ids: HashMap<_, _> = initial
        .bottom_to_top
        .iter()
        .map(|window| (window.id, (window.child_id, window.text_id)))
        .collect();

    let blank = egui::pos2(300.0, 220.0);
    let (_, reordered) = run_candidate(
        &ctx,
        &mut state,
        &specs(&[ProbeWindow::C, ProbeWindow::A, ProbeWindow::B]),
        Some(blank),
        vec![egui::Event::PointerMoved(blank)],
    );
    let reordered_ids: HashMap<_, _> = reordered
        .bottom_to_top
        .iter()
        .map(|window| (window.id, (window.child_id, window.text_id)))
        .collect();

    assert_eq!(reordered_ids, initial_ids);
    assert_eq!(reordered.input_recipient, None);
    assert_eq!(ctx.screen_rect(), CANVAS);
    assert!(!ctx.wants_pointer_input());
}

#[test]
fn windowing_adapter_reorders_existing_windows_on_the_first_requested_frame() {
    const PERMUTATIONS: [[ProbeWindow; 3]; 6] = [
        [ProbeWindow::A, ProbeWindow::B, ProbeWindow::C],
        [ProbeWindow::A, ProbeWindow::C, ProbeWindow::B],
        [ProbeWindow::B, ProbeWindow::A, ProbeWindow::C],
        [ProbeWindow::B, ProbeWindow::C, ProbeWindow::A],
        [ProbeWindow::C, ProbeWindow::A, ProbeWindow::B],
        [ProbeWindow::C, ProbeWindow::B, ProbeWindow::A],
    ];

    for permutation in PERMUTATIONS {
        let ctx = egui::Context::default();
        let mut state = ProbeState::default();
        let (output, _) = run_candidate(&ctx, &mut state, &specs(&permutation), None, Vec::new());
        assert_eq!(marker_order(&output), permutation);
    }

    let ctx = egui::Context::default();
    let mut state = ProbeState::default();
    for permutation in PERMUTATIONS {
        let (output, _) = run_candidate(&ctx, &mut state, &specs(&permutation), None, Vec::new());
        assert_eq!(marker_order(&output), permutation);
    }
    for membership in [
        vec![ProbeWindow::C, ProbeWindow::A],
        vec![ProbeWindow::B, ProbeWindow::C, ProbeWindow::A],
    ] {
        let (output, _) = run_candidate(&ctx, &mut state, &specs(&membership), None, Vec::new());
        assert_eq!(marker_order(&output), membership);
    }
}

#[test]
fn windowing_adapter_keeps_widget_ids_and_text_state_across_raise() {
    let ctx = egui::Context::default();
    let mut state = ProbeState::default();
    let (_, first) = run_candidate(
        &ctx,
        &mut state,
        &specs(&ProbeWindow::ALL),
        Some(OVERLAP.center()),
        Vec::new(),
    );
    let id = text_id(&first, ProbeWindow::A);
    focus_text_at_end(&ctx, id, state.text[&ProbeWindow::A].chars().count());
    let (_, raised) = run_candidate(
        &ctx,
        &mut state,
        &specs(&[ProbeWindow::B, ProbeWindow::C, ProbeWindow::A]),
        Some(OVERLAP.center()),
        vec![egui::Event::Text("+".to_owned())],
    );

    assert_eq!(text_id(&raised, ProbeWindow::A), id);
    assert_eq!(state.text[&ProbeWindow::A], "A+");
    assert!(egui::TextEdit::load_state(&ctx, id).is_some());
}

#[test]
fn windowing_adapter_blocks_lower_text_edit_before_mutation() {
    let ctx = egui::Context::default();
    let mut state = ProbeState::default();
    let (_, first) = run_candidate(
        &ctx,
        &mut state,
        &specs(&[ProbeWindow::A]),
        Some(OVERLAP.center()),
        Vec::new(),
    );
    let id = text_id(&first, ProbeWindow::A);
    focus_text_at_end(&ctx, id, state.text[&ProbeWindow::A].chars().count());

    let frame = prepare_adapter_frame(
        BridgeStrategy::ManagedHost,
        CANVAS,
        &specs(&[ProbeWindow::A, ProbeWindow::B]),
        Some(OVERLAP.center()),
    )
    .unwrap();
    assert_eq!(frame.input_recipient, Some(ProbeWindow::B));
    assert_eq!(frame.bottom_to_top[0].permit, InputPermit::BLOCKED);
    assert!(InputPermit {
        pointer: true,
        keyboard: false,
    }
    .interactive());
    assert!(InputPermit {
        pointer: false,
        keyboard: true,
    }
    .interactive());
    let mut realized = None;
    let _ = ctx.run(input(vec![egui::Event::Text("!".to_owned())]), |ctx| {
        realized = Some(realize_adapter_frame(ctx, &frame, &mut state));
    });

    assert!(realized.is_some());
    assert_eq!(state.text[&ProbeWindow::A], "A");
}

#[test]
fn windowing_adapter_routes_overlap_to_exactly_one_button() {
    let ctx = egui::Context::default();
    let mut state = ProbeState::default();
    let button = OVERLAP.min + egui::vec2(30.0, 94.0);
    let windows = specs(&[ProbeWindow::C, ProbeWindow::A, ProbeWindow::B]);
    for events in [
        vec![egui::Event::PointerMoved(button)],
        vec![pointer_button(button, true)],
        vec![pointer_button(button, false)],
    ] {
        let _ = run_candidate(&ctx, &mut state, &windows, Some(button), events);
    }

    assert_eq!(state.button_hits, vec![ProbeWindow::B]);
    assert!(state.raw_row_hits.is_empty());

    let raw_row = OVERLAP.min + egui::vec2(30.0, 120.0);
    for events in [
        vec![egui::Event::PointerMoved(raw_row)],
        vec![pointer_button(raw_row, true)],
        vec![pointer_button(raw_row, false)],
    ] {
        let _ = run_candidate(&ctx, &mut state, &windows, Some(raw_row), events);
    }
    assert_eq!(state.button_hits, vec![ProbeWindow::B]);
    assert_eq!(state.raw_row_hits, vec![ProbeWindow::B]);
}

#[test]
fn windowing_adapter_preserves_scroll_clipping_and_root_hit_regions() {
    let clip = egui::Rect::from_min_max(OVERLAP.min, OVERLAP.max - egui::vec2(32.0, 36.0));
    let windows = vec![ProbeWindowSpec::new(ProbeWindow::A, OVERLAP).clipped(clip)];
    let inside = clip.center();
    let outside_clip = egui::pos2(210.0, 170.0);
    let blank = egui::pos2(300.0, 220.0);

    let inside_frame =
        prepare_adapter_frame(BridgeStrategy::ManagedHost, CANVAS, &windows, Some(inside)).unwrap();
    let clipped_frame = prepare_adapter_frame(
        BridgeStrategy::ManagedHost,
        CANVAS,
        &windows,
        Some(outside_clip),
    )
    .unwrap();
    let blank_frame =
        prepare_adapter_frame(BridgeStrategy::ManagedHost, CANVAS, &windows, Some(blank)).unwrap();
    assert_eq!(inside_frame.input_recipient, Some(ProbeWindow::A));
    assert_eq!(clipped_frame.input_recipient, None);
    assert_eq!(blank_frame.input_recipient, None);

    let ctx = egui::Context::default();
    let mut state = ProbeState::default();
    let (output, realized) = run_candidate(
        &ctx,
        &mut state,
        &windows,
        Some(inside),
        vec![egui::Event::PointerMoved(inside)],
    );
    assert_eq!(realized.bottom_to_top[0].clip_rect, clip);
    assert_eq!(marker_clips(&output), vec![(ProbeWindow::A, clip)]);
    assert!(ctx.wants_pointer_input());

    let _ = run_candidate(
        &ctx,
        &mut state,
        &windows,
        Some(blank),
        vec![egui::Event::PointerMoved(blank)],
    );
    assert!(!ctx.wants_pointer_input());
}

#[test]
fn windowing_adapter_cold_frame_matches_warm_frame_after_resize() {
    let resized_canvas = egui::Rect::from_min_max(
        egui::pos2(0.0, 0.0),
        egui::pos2(CANVAS.width() + 80.0, CANVAS.height() + 60.0),
    );
    let resized_windows = vec![
        ProbeWindowSpec::new(ProbeWindow::C, OVERLAP.translate(egui::vec2(70.0, 40.0))),
        ProbeWindowSpec::new(ProbeWindow::A, OVERLAP.translate(egui::vec2(30.0, 20.0))),
        ProbeWindowSpec::new(ProbeWindow::B, OVERLAP.translate(egui::vec2(50.0, 30.0))),
    ];
    let run_resized = |ctx: &egui::Context, state: &mut ProbeState| {
        let frame = prepare_adapter_frame(
            BridgeStrategy::ManagedHost,
            resized_canvas,
            &resized_windows,
            None,
        )
        .unwrap();
        let mut realized = None;
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(resized_canvas),
                ..Default::default()
            },
            |ctx| realized = Some(realize_adapter_frame(ctx, &frame, state)),
        );
        (output, realized.unwrap())
    };

    let cold_ctx = egui::Context::default();
    let (cold_output, cold_realized) = run_resized(&cold_ctx, &mut ProbeState::default());

    let warm_ctx = egui::Context::default();
    let mut warm_state = ProbeState::default();
    let _ = run_candidate(
        &warm_ctx,
        &mut warm_state,
        &specs(&ProbeWindow::ALL),
        None,
        Vec::new(),
    );
    let (warm_output, warm_realized) = run_resized(&warm_ctx, &mut warm_state);

    assert_eq!(marker_order(&cold_output), marker_order(&warm_output));
    assert_eq!(marker_clips(&cold_output), marker_clips(&warm_output));
    assert_eq!(cold_realized, warm_realized);
}
