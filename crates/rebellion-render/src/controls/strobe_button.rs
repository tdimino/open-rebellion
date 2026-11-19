//! The original `CoolStrobeButton` and its `Checkbox` toggle variant.
//!
//! `FUN_00602150` owns the four bitmap slots, `FUN_006028c0` owns capture and
//! release, `FUN_00602d30` chooses the normal/held/disabled/selected image,
//! and `FUN_00603870` plus `WndProc` `0x006039a0` bind Checkbox toggling to the
//! same button state machine. Domain commands, tooltip text, faction choice,
//! and sounds remain with the feature that composes the control.

#![allow(
    clippy::module_name_repetitions,
    reason = "The source class name is the public renderer object name."
)]

use std::hash::Hash;

use egui_macroquad::egui;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::native_geometry::rect_contains;

/// One source bitmap used by a button state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonImage {
    pub source: DllSource,
    pub resource_id: u32,
}

impl ButtonImage {
    #[must_use]
    pub const fn new(source: DllSource, resource_id: u32) -> Self {
        Self {
            source,
            resource_id,
        }
    }
}

/// The four image slots configured through `FUN_00603150`.
///
/// Missing non-normal slots fall back to the nearest source-supported look:
/// held and selected use one another, while disabled ultimately uses normal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonArt {
    pub normal: ButtonImage,
    pub held: Option<ButtonImage>,
    pub selected: Option<ButtonImage>,
    pub disabled: Option<ButtonImage>,
    pub draw: ButtonDraw,
}

/// How `FUN_00602d30` presents the selected bitmap.
///
/// Most controls copy their saved background and key the bitmap's first
/// stored palette index over it. Controls configured with flag `0x80` draw
/// the bitmap directly instead; the strategic title-bar buttons are recovered
/// examples of that path.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonDraw {
    #[default]
    KeyedOverBackground,
    Direct,
}

impl ButtonArt {
    #[must_use]
    pub const fn new(normal: ButtonImage) -> Self {
        Self {
            normal,
            held: None,
            selected: None,
            disabled: None,
            draw: ButtonDraw::KeyedOverBackground,
        }
    }

    #[must_use]
    pub const fn two_state(source: DllSource, normal: u32, held: u32) -> Self {
        let normal = ButtonImage::new(source, normal);
        let held = ButtonImage::new(source, held);
        Self {
            normal,
            held: Some(held),
            selected: Some(held),
            disabled: None,
            draw: ButtonDraw::KeyedOverBackground,
        }
    }

    #[must_use]
    pub const fn with_selected(mut self, selected: ButtonImage) -> Self {
        self.selected = Some(selected);
        self
    }

    #[must_use]
    pub const fn with_disabled(mut self, disabled: ButtonImage) -> Self {
        self.disabled = Some(disabled);
        self
    }

    #[must_use]
    pub const fn with_draw(mut self, draw: ButtonDraw) -> Self {
        self.draw = draw;
        self
    }

    #[must_use]
    pub const fn image(self, visual: ButtonVisualState) -> ButtonImage {
        match visual {
            ButtonVisualState::Normal => self.normal,
            ButtonVisualState::Held => match self.held {
                Some(image) => image,
                None => match self.selected {
                    Some(image) => image,
                    None => self.normal,
                },
            },
            ButtonVisualState::Selected => match self.selected {
                Some(image) => image,
                None => match self.held {
                    Some(image) => image,
                    None => self.normal,
                },
            },
            ButtonVisualState::Disabled => match self.disabled {
                Some(image) => image,
                None => self.normal,
            },
        }
    }
}

/// The image state painted by `FUN_00602d30`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVisualState {
    Normal,
    Held,
    Selected,
    Disabled,
}

/// Optional repeat timing for source controls constructed with a repeat rate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonRepeat {
    pub delay_seconds: f64,
    pub interval_seconds: f64,
}

impl ButtonRepeat {
    #[must_use]
    pub const fn new(delay_seconds: f64, interval_seconds: f64) -> Self {
        Self {
            delay_seconds,
            interval_seconds,
        }
    }
}

/// Persistent control-local state. Feature meaning stays outside this record.
#[allow(
    clippy::struct_excessive_bools,
    reason = "Selected, disabled, armed and held are independent recovered button state bits."
)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ButtonState {
    pub selected: bool,
    pub disabled: bool,
    armed: bool,
    held: bool,
    repeat_deadline: Option<f64>,
}

impl ButtonState {
    #[must_use]
    pub const fn visual(selected: bool, disabled: bool, held: bool) -> Self {
        Self {
            selected,
            disabled,
            armed: held,
            held,
            repeat_deadline: None,
        }
    }

    #[must_use]
    pub const fn selected(selected: bool) -> Self {
        Self {
            selected,
            disabled: false,
            armed: false,
            held: false,
            repeat_deadline: None,
        }
    }

    #[must_use]
    pub const fn disabled(disabled: bool) -> Self {
        Self {
            selected: false,
            disabled,
            armed: false,
            held: false,
            repeat_deadline: None,
        }
    }

    #[must_use]
    pub const fn held(held: bool) -> Self {
        Self {
            selected: false,
            disabled: false,
            armed: held,
            held,
            repeat_deadline: None,
        }
    }
}

/// Current-frame pointer facts already admitted to the control by its window.
#[allow(
    clippy::struct_excessive_bools,
    reason = "Inside, press, hold and release are independent facts from one routed input frame."
)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ButtonInput {
    pub pointer_inside: bool,
    /// A separately hit-tested button-down position when motion shares its frame.
    /// None preserves the point-only input contract used by pure control callers.
    pub press_inside: Option<bool>,
    pub primary_pressed: bool,
    pub primary_down: bool,
    pub primary_released: bool,
    pub now_seconds: f64,
}

impl ButtonInput {
    /// Read the shared rectangular pointer contract for buttons and checkboxes.
    fn from_egui(ctx: &egui::Context, rect: egui::Rect) -> Self {
        ctx.input(|input| {
            let mut result = Self::for_rect(
                rect,
                input.pointer.interact_pos(),
                input.pointer.button_pressed(egui::PointerButton::Primary),
                input.pointer.button_down(egui::PointerButton::Primary),
                input.pointer.button_released(egui::PointerButton::Primary),
                input.time,
            );
            result.press_inside = input
                .pointer
                .press_origin()
                .map(|point| rect_contains(rect, point));
            result
        })
    }

    /// Build exact half-open geometry from the current pointer position.
    #[must_use]
    pub fn for_rect(
        rect: egui::Rect,
        pointer: Option<egui::Pos2>,
        primary_pressed: bool,
        primary_down: bool,
        primary_released: bool,
        now_seconds: f64,
    ) -> Self {
        Self {
            pointer_inside: pointer.is_some_and(|point| rect_contains(rect, point)),
            press_inside: None,
            primary_pressed,
            primary_down,
            primary_released,
            now_seconds,
        }
    }
}

/// One typed event emitted by the control state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonOutcome {
    Pressed,
    Released,
    Clicked,
    Repeated,
    Toggled(bool),
    Cancelled,
}

/// The ordered outcomes from one input frame, without allocating per control.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ButtonOutput {
    events: [Option<ButtonOutcome>; 4],
}

/// The typed control outcomes together with egui focus/accessibility state.
#[derive(Debug)]
pub struct StrobeButtonResponse {
    pub output: ButtonOutput,
    pub response: egui::Response,
}

impl StrobeButtonResponse {
    #[must_use]
    pub fn clicked(&self) -> bool {
        self.output.clicked()
    }

    #[must_use]
    pub fn toggled(&self) -> Option<bool> {
        self.output.toggled()
    }
}

impl ButtonOutput {
    fn push(&mut self, event: ButtonOutcome) {
        if let Some(slot) = self.events.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(event);
        }
    }

    #[must_use]
    pub fn contains(self, outcome: ButtonOutcome) -> bool {
        self.events.contains(&Some(outcome))
    }

    #[must_use]
    pub fn clicked(self) -> bool {
        self.contains(ButtonOutcome::Clicked)
    }

    #[must_use]
    pub fn toggled(self) -> Option<bool> {
        self.events
            .into_iter()
            .flatten()
            .find_map(|event| match event {
                ButtonOutcome::Toggled(value) => Some(value),
                _ => None,
            })
    }

    pub fn iter(self) -> impl Iterator<Item = ButtonOutcome> {
        self.events.into_iter().flatten()
    }
}

/// A source-shaped four-state bitmap button.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrobeButton {
    pub art: ButtonArt,
    pub state: ButtonState,
    pub repeat: Option<ButtonRepeat>,
}

impl StrobeButton {
    #[must_use]
    pub const fn new(art: ButtonArt) -> Self {
        Self {
            art,
            state: ButtonState::selected(false),
            repeat: None,
        }
    }

    #[must_use]
    pub const fn with_state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }

    #[must_use]
    pub const fn with_repeat(mut self, repeat: ButtonRepeat) -> Self {
        self.repeat = Some(repeat);
        self
    }

    #[must_use]
    pub const fn visual_state(&self) -> ButtonVisualState {
        if self.state.disabled {
            ButtonVisualState::Disabled
        } else if self.state.held {
            ButtonVisualState::Held
        } else if self.state.selected {
            ButtonVisualState::Selected
        } else {
            ButtonVisualState::Normal
        }
    }

    #[must_use]
    pub const fn image(&self) -> ButtonImage {
        self.art.image(self.visual_state())
    }

    /// Apply one routed pointer frame. A press that began elsewhere never
    /// arms this control; capture remains held while the pointer leaves, and
    /// an outside release cancels instead of clicking (`FUN_006028c0`).
    #[must_use]
    pub fn update(&mut self, input: ButtonInput) -> ButtonOutput {
        let mut output = ButtonOutput::default();
        if self.state.disabled {
            if self.state.armed {
                self.cancel(&mut output);
            }
            return output;
        }

        if input.primary_pressed && input.press_inside.unwrap_or(input.pointer_inside) {
            self.state.armed = true;
            self.state.held = true;
            self.state.repeat_deadline = self
                .repeat
                .map(|repeat| input.now_seconds + repeat.delay_seconds.max(0.0));
            output.push(ButtonOutcome::Pressed);
        }

        if self.state.armed && input.primary_released {
            self.state.armed = false;
            self.state.held = false;
            self.state.repeat_deadline = None;
            output.push(ButtonOutcome::Released);
            output.push(if input.pointer_inside {
                ButtonOutcome::Clicked
            } else {
                ButtonOutcome::Cancelled
            });
            return output;
        }

        if self.state.armed && !input.primary_down && !input.primary_pressed {
            self.cancel(&mut output);
            return output;
        }

        if self.state.armed && input.primary_down {
            // The native WndProc has no mouse-move branch: capture keeps the
            // held look until release even when the pointer leaves.
            self.state.held = true;
            if input.pointer_inside {
                if let (Some(repeat), Some(deadline)) = (self.repeat, self.state.repeat_deadline) {
                    if input.now_seconds >= deadline {
                        output.push(ButtonOutcome::Repeated);
                        self.state.repeat_deadline =
                            Some(input.now_seconds + repeat.interval_seconds.max(f64::EPSILON));
                    }
                }
            }
        }
        output
    }

    fn cancel(&mut self, output: &mut ButtonOutput) {
        self.state.armed = false;
        self.state.held = false;
        self.state.repeat_deadline = None;
        output.push(ButtonOutcome::Cancelled);
    }

    /// Run and paint an egui-composed button while retaining its control-local
    /// state in egui's typed temporary store under the supplied stable id.
    #[must_use]
    #[allow(
        clippy::too_many_arguments,
        reason = "The control boundary keeps geometry, identity, art, state and scale explicit."
    )]
    pub fn show(
        ui: &egui::Ui,
        cache: &mut BmpCache,
        rect: egui::Rect,
        id: impl Hash,
        art: ButtonArt,
        selected: bool,
        disabled: bool,
        repeat: Option<ButtonRepeat>,
        scale: f32,
    ) -> StrobeButtonResponse {
        let input = ButtonInput::from_egui(ui.ctx(), rect);
        Self::show_inner(
            ui, cache, rect, id, art, selected, disabled, repeat, scale, input, true,
        )
    }

    /// Variant for source controls with a bitmap hit mask. The caller derives
    /// `pointer_inside` from the admitted event and source pixels; capture,
    /// outcomes, art choice, and painting remain in this object.
    #[allow(
        clippy::too_many_arguments,
        reason = "The control boundary keeps geometry, art, state, timing and routed input explicit."
    )]
    #[must_use]
    pub fn show_with_input(
        ui: &egui::Ui,
        cache: &mut BmpCache,
        rect: egui::Rect,
        id: impl Hash,
        art: ButtonArt,
        selected: bool,
        disabled: bool,
        repeat: Option<ButtonRepeat>,
        scale: f32,
        input: ButtonInput,
    ) -> StrobeButtonResponse {
        Self::show_inner(
            ui, cache, rect, id, art, selected, disabled, repeat, scale, input, false,
        )
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "The internal boundary also carries whether egui must admit raw pointer input."
    )]
    fn show_inner(
        ui: &egui::Ui,
        cache: &mut BmpCache,
        rect: egui::Rect,
        id: impl Hash,
        art: ButtonArt,
        selected: bool,
        disabled: bool,
        repeat: Option<ButtonRepeat>,
        scale: f32,
        mut input: ButtonInput,
        require_response_ownership: bool,
    ) -> StrobeButtonResponse {
        let id = ui.id().with(id);
        let response = ui.interact(
            rect,
            id,
            if disabled {
                egui::Sense::hover()
            } else {
                egui::Sense::click()
            },
        );
        // Egui settles overlapping-window ownership before the source-shaped
        // state machine sees the press. This preserves native capture without
        // letting a popup click arm a control underneath it.
        if require_response_ownership
            && input.primary_pressed
            && !(response.is_pointer_button_down_on()
                || response.clicked_by(egui::PointerButton::Primary))
        {
            input.primary_pressed = false;
        }
        let state_id = id.with("strobe-button-state");
        let mut button = ui
            .data(|data| data.get_temp::<Self>(state_id))
            .unwrap_or_else(|| Self::new(art));
        button.art = art;
        button.state.selected = selected;
        button.state.disabled = disabled;
        button.repeat = repeat;
        let output = button.update(input);
        paint_button(
            ui.painter(),
            ui.ctx(),
            cache,
            button.image(),
            button.art.draw,
            rect,
            scale,
        );
        ui.data_mut(|data| data.insert_temp(state_id, button));

        StrobeButtonResponse { output, response }
    }

    /// Convenience for the common constructor shape whose selected state
    /// shares the held bitmap.
    #[must_use]
    #[allow(
        clippy::too_many_arguments,
        reason = "The convenience keeps the source, art pair and state explicit at migration sites."
    )]
    pub fn show_pair(
        ui: &egui::Ui,
        cache: &mut BmpCache,
        rect: egui::Rect,
        id: impl Hash,
        source: DllSource,
        art: (u32, u32),
        selected: bool,
        disabled: bool,
        scale: f32,
    ) -> StrobeButtonResponse {
        Self::show(
            ui,
            cache,
            rect,
            id,
            ButtonArt::two_state(source, art.0, art.1),
            selected,
            disabled,
            None,
            scale,
        )
    }
}

/// The authentic toggle variant, sharing `StrobeButton` capture and art rules.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Checkbox {
    pub button: StrobeButton,
    pub checked: bool,
}

impl Checkbox {
    #[must_use]
    pub const fn new(art: ButtonArt, checked: bool) -> Self {
        Self {
            button: StrobeButton::new(art).with_state(ButtonState::selected(checked)),
            checked,
        }
    }

    #[must_use]
    pub fn update(&mut self, input: ButtonInput) -> ButtonOutput {
        self.button.state.selected = self.checked;
        let output = self.button.update(input);
        if !output.clicked() {
            return output;
        }
        self.checked = !self.checked;
        self.button.state.selected = self.checked;
        let mut output = output;
        output.push(ButtonOutcome::Toggled(self.checked));
        output
    }

    #[must_use]
    pub const fn image(&self) -> ButtonImage {
        self.button.image()
    }

    #[must_use]
    #[allow(
        clippy::too_many_arguments,
        reason = "The checkbox keeps geometry, identity, art and externally owned checked state explicit."
    )]
    pub fn show(
        ui: &egui::Ui,
        cache: &mut BmpCache,
        rect: egui::Rect,
        id: impl Hash,
        art: ButtonArt,
        checked: bool,
        disabled: bool,
        scale: f32,
    ) -> StrobeButtonResponse {
        let id = ui.id().with(id);
        let response = ui.interact(
            rect,
            id,
            if disabled {
                egui::Sense::hover()
            } else {
                egui::Sense::click()
            },
        );
        let state_id = id.with("checkbox-state");
        let mut checkbox = ui
            .data(|data| data.get_temp::<Self>(state_id))
            .unwrap_or_else(|| Self::new(art, checked));
        checkbox.button.art = art;
        checkbox.button.state.disabled = disabled;
        checkbox.checked = checked;
        let mut input = ButtonInput::from_egui(ui.ctx(), rect);
        if input.primary_pressed
            && !(response.is_pointer_button_down_on()
                || response.clicked_by(egui::PointerButton::Primary))
        {
            input.primary_pressed = false;
        }
        let output = checkbox.update(input);
        paint_button(
            ui.painter(),
            ui.ctx(),
            cache,
            checkbox.image(),
            checkbox.button.art.draw,
            rect,
            scale,
        );
        ui.data_mut(|data| data.insert_temp(state_id, checkbox));
        StrobeButtonResponse { output, response }
    }
}

/// Paint one native-size, first-pixel-keyed state bitmap clipped to the
/// control window, as `FUN_00602d30` calls `FUN_005fc140`.
pub(crate) fn paint_button(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    image: ButtonImage,
    draw: ButtonDraw,
    rect: egui::Rect,
    scale: f32,
) {
    #[cfg(test)]
    crate::test_support::record_painted(image.resource_id, rect.min);
    let texture = match draw {
        ButtonDraw::KeyedOverBackground => cache.get_button(ctx, image.source, image.resource_id),
        ButtonDraw::Direct => cache.get(ctx, image.source, image.resource_id),
    };
    let Some(texture) = texture else {
        return;
    };
    let native = egui::Rect::from_min_size(rect.min, texture.size_vec2() * scale);
    painter
        .with_clip_rect(painter.clip_rect().intersect(rect))
        .image(
            texture.id(),
            native,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLIANCE_ART: ButtonArt = ButtonArt::two_state(DllSource::Strategy, 10_001, 10_002)
        .with_selected(ButtonImage::new(DllSource::Strategy, 10_003))
        .with_disabled(ButtonImage::new(DllSource::Strategy, 10_004));
    const EMPIRE_ART: ButtonArt = ButtonArt::two_state(DllSource::Strategy, 11_001, 11_002)
        .with_selected(ButtonImage::new(DllSource::Strategy, 11_003))
        .with_disabled(ButtonImage::new(DllSource::Strategy, 11_004));

    fn input(inside: bool, pressed: bool, down: bool, released: bool, now: f64) -> ButtonInput {
        ButtonInput {
            pointer_inside: inside,
            press_inside: None,
            primary_pressed: pressed,
            primary_down: down,
            primary_released: released,
            now_seconds: now,
        }
    }

    #[test]
    fn normal_held_selected_and_disabled_choose_each_configured_bitmap_for_both_factions() {
        // Source: FUN_00603150 maps states 0, 1, 2 and 4 to the four slots;
        // FUN_00602d30 paints disabled before held/selected.
        for art in [ALLIANCE_ART, EMPIRE_ART] {
            let mut button = StrobeButton::new(art);
            assert_eq!(button.image(), art.normal);
            let pressed = button.update(input(true, true, true, false, 0.0));
            assert!(pressed.contains(ButtonOutcome::Pressed));
            assert_eq!(button.image(), art.held.expect("held art"));
            let _ = button.update(input(true, false, false, true, 0.1));
            button.state.selected = true;
            assert_eq!(button.image(), art.selected.expect("selected art"));
            button.state.disabled = true;
            assert_eq!(button.image(), art.disabled.expect("disabled art"));
        }
    }

    #[test]
    fn a_press_that_starts_outside_never_arms_when_dragged_inside() {
        // Source: FUN_006028c0 captures only after FUN_006035f0 accepts the
        // button-down point. The former free helper treated any held pointer
        // that later entered the rectangle as held.
        let mut button = StrobeButton::new(ALLIANCE_ART);
        assert_eq!(
            button.update(input(false, true, true, false, 0.0)),
            ButtonOutput::default()
        );
        assert_eq!(
            button.update(input(true, false, true, false, 0.1)),
            ButtonOutput::default()
        );
        assert_eq!(
            button.update(input(true, false, false, true, 0.2)),
            ButtonOutput::default()
        );
        assert_eq!(button.visual_state(), ButtonVisualState::Normal);
    }

    #[test]
    fn an_armed_button_stays_held_outside_but_an_outside_release_cancels() {
        // Source: FUN_006028c0 has no mouse-move branch; release reruns
        // FUN_006035f0 and posts the command only when that point hits.
        let mut button = StrobeButton::new(ALLIANCE_ART);
        let _ = button.update(input(true, true, true, false, 0.0));
        let moved = button.update(input(false, false, true, false, 0.1));
        assert_eq!(moved, ButtonOutput::default());
        assert_eq!(button.visual_state(), ButtonVisualState::Held);
        let released = button.update(input(false, false, false, true, 0.2));
        assert!(released.contains(ButtonOutcome::Released));
        assert!(released.contains(ButtonOutcome::Cancelled));
        assert!(!released.clicked());
        assert_eq!(button.visual_state(), ButtonVisualState::Normal);
    }

    #[test]
    fn an_armed_button_cancels_when_capture_disappears_without_a_release() {
        // FUN_006028c0 clears capture and the held bit when the admitted
        // primary gesture vanishes before its button-up message arrives.
        let mut button = StrobeButton::new(ALLIANCE_ART);
        let _ = button.update(input(true, true, true, false, 0.0));
        let cancelled = button.update(input(true, false, false, false, 0.1));
        assert_eq!(
            cancelled.iter().collect::<Vec<_>>(),
            vec![ButtonOutcome::Cancelled]
        );
        assert_eq!(button.visual_state(), ButtonVisualState::Normal);
    }

    #[test]
    fn an_inside_release_reports_release_then_click() {
        let mut button = StrobeButton::new(EMPIRE_ART);
        let _ = button.update(input(true, true, true, false, 1.0));
        let released = button.update(input(true, false, false, true, 1.1));
        assert_eq!(
            released.iter().collect::<Vec<_>>(),
            vec![ButtonOutcome::Released, ButtonOutcome::Clicked]
        );
    }

    #[test]
    fn an_admitted_masked_click_does_not_require_a_second_egui_ownership_check() {
        // A source-shaped caller has already routed and hit-tested this input.
        // Requiring egui's rectangular response to own it again loses batched
        // browser press/release frames before FUN_006028c0 can emit the click.
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let mut clicked = false;
        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 200.0),
            )),
            ..Default::default()
        };

        let _ = ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                clicked = StrobeButton::show_with_input(
                    ui,
                    &mut cache,
                    egui::Rect::from_min_size(egui::pos2(20.0, 20.0), egui::vec2(24.0, 12.0)),
                    "masked-click",
                    ALLIANCE_ART,
                    false,
                    false,
                    None,
                    1.0,
                    input(true, true, false, true, 1.0),
                )
                .clicked();
            });
        });

        assert!(clicked);
    }

    #[test]
    fn a_disabled_button_uses_disabled_art_and_emits_no_pointer_outcome() {
        let mut button = StrobeButton::new(ALLIANCE_ART).with_state(ButtonState::disabled(true));
        assert_eq!(button.image(), ALLIANCE_ART.disabled.expect("disabled art"));
        assert_eq!(
            button.update(input(true, true, true, false, 0.0)),
            ButtonOutput::default()
        );
        assert_eq!(
            button.update(input(true, false, false, true, 0.1)),
            ButtonOutput::default()
        );
    }

    #[test]
    fn a_checkbox_toggles_only_after_an_inside_release() {
        // Source: Checkbox WndProc 0x006039a0 delegates capture/release to
        // FUN_006028c0 and binds the resulting state to its current bitmap.
        assert_eq!(
            Checkbox::new(ALLIANCE_ART, true).image(),
            ALLIANCE_ART.selected.expect("selected art")
        );
        let mut checkbox = Checkbox::new(ALLIANCE_ART, false);
        let _ = checkbox.update(input(true, true, true, false, 0.0));
        assert_eq!(
            checkbox
                .update(input(false, false, false, true, 0.1))
                .toggled(),
            None
        );
        assert!(!checkbox.checked);
        let _ = checkbox.update(input(true, true, true, false, 0.2));
        let toggled = checkbox.update(input(true, false, false, true, 0.3));
        assert!(toggled.clicked());
        assert_eq!(toggled.toggled(), Some(true));
        assert!(checkbox.checked);
        assert_eq!(
            checkbox.image(),
            ALLIANCE_ART.selected.expect("selected art")
        );
    }

    #[test]
    fn repeat_waits_for_its_delay_and_only_fires_while_the_pointer_is_inside() {
        // Source: FUN_00602150 stores repeat rate/deadline at +0xfc/+0x104;
        // FUN_006028c0 starts repeat only for an accepted press.
        let mut button = StrobeButton::new(ALLIANCE_ART).with_repeat(ButtonRepeat::new(0.25, 0.1));
        let _ = button.update(input(true, true, true, false, 2.0));
        assert_eq!(
            button.update(input(true, false, true, false, 2.24)),
            ButtonOutput::default()
        );
        assert!(button
            .update(input(true, false, true, false, 2.25))
            .contains(ButtonOutcome::Repeated));
        assert_eq!(
            button.update(input(true, false, true, false, 2.26)),
            ButtonOutput::default()
        );
        assert_eq!(
            button.update(input(false, false, true, false, 2.40)),
            ButtonOutput::default()
        );
        assert!(button
            .update(input(true, false, true, false, 2.41))
            .contains(ButtonOutcome::Repeated));
    }

    #[test]
    fn exact_hit_geometry_includes_top_left_and_excludes_bottom_right() {
        let rect = egui::Rect::from_min_max(egui::pos2(10.0, 20.0), egui::pos2(30.0, 40.0));
        for (point, inside) in [
            (egui::pos2(10.0, 20.0), true),
            (egui::pos2(29.999, 39.999), true),
            (egui::pos2(30.0, 39.0), false),
            (egui::pos2(29.0, 40.0), false),
        ] {
            assert_eq!(
                ButtonInput::for_rect(rect, Some(point), false, false, false, 0.0).pointer_inside,
                inside,
                "point={point:?}"
            );
        }
    }

    #[test]
    fn batched_press_and_motion_keep_capture_bound_to_the_press_coordinates() {
        // FUN_006028c0 receives button-down coordinates before later motion.
        let rect = egui::Rect::from_min_size(egui::pos2(20.0, 20.0), egui::vec2(20.0, 10.0));
        for art in [ALLIANCE_ART, EMPIRE_ART] {
            for (start, end, held) in [
                (egui::pos2(50.0, 25.0), egui::pos2(25.0, 25.0), false),
                (egui::pos2(25.0, 25.0), egui::pos2(50.0, 25.0), true),
            ] {
                let ctx = egui::Context::default();
                let mut button = StrobeButton::new(art);
                let events = vec![
                    egui::Event::PointerMoved(start),
                    crate::test_support::pointer_button(start, true),
                    egui::Event::PointerMoved(end),
                ];
                let _ = ctx.run(
                    crate::test_support::raw_input(events, egui::vec2(320.0, 200.0)),
                    |ctx| {
                        let _ = button.update(ButtonInput::from_egui(ctx, rect));
                    },
                );
                assert_eq!(button.state.armed, held);
                assert_eq!(button.state.held, held);
                let released = button.update(ButtonInput::for_rect(
                    rect,
                    Some(end),
                    false,
                    false,
                    true,
                    0.1,
                ));
                assert!(!released.clicked());
            }
        }
    }
}
