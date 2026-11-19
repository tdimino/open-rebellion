//! The original `CoolStringField` text, selection, focus, and key contract.
//!
//! `FUN_00604cf0` constructs the buffer, owner, bounds, and unlimited default;
//! `FUN_00604f70` installs its font/layout; `FUN_006050c0` installs an optional
//! maximum length; `FUN_00604f90` replaces and truncates its value;
//! `FUN_00605110` owns the selection; and `FUN_00605160` owns focus, caret,
//! overwrite, accepted text, editing keys, Enter notification, and owner-key
//! forwarding. Feature validation and the meaning of submit/cancel stay in
//! the parent window.

use std::ops::Deref;
use std::ops::Range;

use egui_macroquad::egui;
use rebellion_windowing::ControlId;

/// Unicode counterpart to the original `FUN_0061a050` printable-character
/// predicate. Numeric interpretation remains a Build Selection parent rule.
fn accepts_character(character: char) -> bool {
    !character.is_control()
}

/// Explicit paint configuration; feature art and layout remain outside.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StringFieldStyle {
    pub(crate) font: egui::FontId,
    pub(crate) text_color: egui::Color32,
    pub(crate) selection_background: egui::Color32,
    pub(crate) selected_text_color: egui::Color32,
    pub(crate) cursor_color: egui::Color32,
    pub(crate) frame: bool,
    pub(crate) margin: egui::Margin,
}

impl Default for StringFieldStyle {
    fn default() -> Self {
        Self {
            font: egui::FontId::proportional(14.0),
            text_color: egui::Color32::WHITE,
            selection_background: egui::Color32::WHITE,
            selected_text_color: egui::Color32::BLACK,
            cursor_color: egui::Color32::WHITE,
            frame: false,
            margin: egui::Margin::ZERO,
        }
    }
}

/// Geometry, stable identities, length/filter policy, and paint configuration.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StringFieldSpec {
    pub(crate) id: egui::Id,
    pub(crate) control_id: ControlId,
    pub(crate) rect: egui::Rect,
    pub(crate) max_length: Option<usize>,
    pub(crate) style: StringFieldStyle,
}

impl StringFieldSpec {
    #[must_use]
    pub(crate) fn new(
        id: egui::Id,
        control_id: ControlId,
        rect: egui::Rect,
        max_length: Option<usize>,
        style: StringFieldStyle,
    ) -> Self {
        Self {
            id,
            control_id,
            rect,
            max_length,
            style,
        }
    }
}

/// Input permission leased by the parent after top-level routing policy.
///
/// This object never decides which window owns focus. It only applies the
/// already-admitted lease and reports focus changes back to that owner.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct StringFieldLease {
    pub(crate) interactive: bool,
    pub(crate) handles_input: bool,
    pub(crate) requests_focus: bool,
    pub(crate) select_all: bool,
}

impl StringFieldLease {
    #[must_use]
    pub(crate) const fn acquire(select_all: bool) -> Self {
        Self {
            interactive: true,
            handles_input: true,
            requests_focus: true,
            select_all,
        }
    }

    /// Ask the widget layer to install focus while keyboard admission remains
    /// with the parent router. This is the handoff frame used by managed
    /// windows before the following frame leases the control its keys.
    #[must_use]
    pub(crate) const fn request_focus(select_all: bool) -> Self {
        Self {
            interactive: false,
            handles_input: false,
            requests_focus: true,
            select_all,
        }
    }

    #[must_use]
    pub(crate) const fn focused() -> Self {
        Self {
            interactive: true,
            handles_input: true,
            requests_focus: false,
            select_all: false,
        }
    }

    #[must_use]
    pub(crate) const fn blocked() -> Self {
        Self {
            interactive: false,
            handles_input: false,
            requests_focus: false,
            select_all: false,
        }
    }

    /// Allow pointer focus while the parent has leased keys to a sibling.
    #[must_use]
    pub(crate) const fn clickable() -> Self {
        Self {
            interactive: true,
            handles_input: false,
            requests_focus: false,
            select_all: false,
        }
    }
}

/// Persistent control-local text, selection/caret, overwrite, and focus state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringFieldState {
    text: String,
    anchor: usize,
    caret: usize,
    overwrite: bool,
    focused_control: Option<ControlId>,
}

impl Default for StringFieldState {
    fn default() -> Self {
        Self::new("")
    }
}

impl StringFieldState {
    #[must_use]
    pub(crate) fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let caret = text.chars().count();
        Self {
            text,
            anchor: caret,
            caret,
            overwrite: false,
            focused_control: None,
        }
    }

    #[must_use]
    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.caret = self.text.chars().count();
        self.anchor = self.caret;
    }

    pub(crate) fn clear(&mut self) {
        self.set_text("");
    }

    #[must_use]
    #[cfg(test)]
    pub(crate) const fn caret(&self) -> usize {
        self.caret
    }

    #[must_use]
    pub(crate) fn selection(&self) -> Option<Range<usize>> {
        (self.anchor != self.caret).then(|| {
            let start = self.anchor.min(self.caret);
            let end = self.anchor.max(self.caret);
            start..end
        })
    }

    pub(crate) fn select_all(&mut self) {
        self.anchor = 0;
        self.caret = self.text.chars().count();
    }

    /// Set the source selection endpoints (`FUN_00605110`), clamped to the
    /// current character count. Equal endpoints are a caret-only selection.
    #[cfg(test)]
    pub(crate) fn set_selection(&mut self, anchor: usize, caret: usize) {
        self.anchor = anchor;
        self.caret = caret;
        self.clamp_cursor();
    }

    #[must_use]
    #[cfg(test)]
    pub(crate) const fn overwrite(&self) -> bool {
        self.overwrite
    }

    #[must_use]
    pub(crate) const fn focused_control(&self) -> Option<ControlId> {
        self.focused_control
    }

    fn clamp_cursor(&mut self) {
        let length = self.text.chars().count();
        self.anchor = self.anchor.min(length);
        self.caret = self.caret.min(length);
    }

    fn truncate(&mut self, max_length: Option<usize>) {
        if let Some(max_length) = max_length {
            if let Some((byte_index, _)) = self.text.char_indices().nth(max_length) {
                self.text.truncate(byte_index);
                self.clamp_cursor();
            }
        }
    }

    fn store_cursor(&mut self, context: &egui::Context, id: egui::Id) {
        self.clamp_cursor();
        let mut edit = egui::TextEdit::load_state(context, id).unwrap_or_default();
        edit.cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(self.anchor),
                egui::text::CCursor::new(self.caret),
            )));
        edit.store(context, id);
    }

    fn load_cursor(&mut self, context: &egui::Context, id: egui::Id) {
        if let Some(range) =
            egui::TextEdit::load_state(context, id).and_then(|edit| edit.cursor.char_range())
        {
            self.anchor = range.secondary.index;
            self.caret = range.primary.index;
        }
        self.clamp_cursor();
    }

    fn insert_fallback_text(&mut self, text: &str, max_length: Option<usize>) {
        let range = self.selection().unwrap_or(self.caret..self.caret);
        let start = range.start;
        let mut buffer = FilteredBuffer {
            text: &mut self.text,
            max_length,
            overwrite: self.overwrite,
        };
        <FilteredBuffer<'_> as egui::TextBuffer>::delete_char_range(&mut buffer, range);
        let inserted =
            <FilteredBuffer<'_> as egui::TextBuffer>::insert_text(&mut buffer, text, start);
        self.anchor = start + inserted;
        self.caret = self.anchor;
    }
}

impl Deref for StringFieldState {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text()
    }
}

impl From<&str> for StringFieldState {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl PartialEq<&str> for StringFieldState {
    fn eq(&self, other: &&str) -> bool {
        self.text() == *other
    }
}

/// One typed parent outcome from `FUN_00605160`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StringFieldEvent {
    Changed,
    Submitted,
    Cancelled,
    FocusChanged(bool),
    UnhandledKey(egui::Key),
}

/// Ordered events from one frame. A frame may both change and submit text.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct StringFieldOutput {
    events: Vec<StringFieldEvent>,
}

impl StringFieldOutput {
    fn push(&mut self, event: StringFieldEvent) {
        self.events.push(event);
    }

    #[must_use]
    pub(crate) fn contains(&self, event: StringFieldEvent) -> bool {
        self.events.contains(&event)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = StringFieldEvent> + '_ {
        self.events.iter().copied()
    }
}

struct FilteredBuffer<'a> {
    text: &'a mut String,
    max_length: Option<usize>,
    overwrite: bool,
}

impl egui::TextBuffer for FilteredBuffer<'_> {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.text
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let accepted: String = text
            .chars()
            .filter(|&character| accepts_character(character))
            .collect();
        let length = self.text.chars().count();
        let index = char_index.min(length);
        let replace = if self.overwrite {
            accepted.chars().count().min(length.saturating_sub(index))
        } else {
            0
        };
        let capacity = self.max_length.map_or(usize::MAX, |max_length| {
            max_length.saturating_sub(length.saturating_sub(replace))
        });
        let inserted: String = accepted.chars().take(capacity).collect();
        <String as egui::TextBuffer>::delete_char_range(self.text, index..index + replace);
        let inserted_count = inserted.chars().count();
        <String as egui::TextBuffer>::insert_text(self.text, &inserted, index);
        inserted_count
    }

    fn delete_char_range(&mut self, range: Range<usize>) {
        let length = self.text.chars().count();
        let start = range.start.min(length);
        let end = range.end.min(length).max(start);
        <String as egui::TextBuffer>::delete_char_range(self.text, start..end);
    }
}

/// One reusable source-shaped string field.
pub(crate) struct StringField<'a> {
    state: &'a mut StringFieldState,
    spec: StringFieldSpec,
    lease: StringFieldLease,
}

fn key_has_text_event(key: egui::Key) -> bool {
    key == egui::Key::Space || (egui::Key::Colon..=egui::Key::Z).contains(&key)
}

impl<'a> StringField<'a> {
    #[must_use]
    pub(crate) const fn new(
        state: &'a mut StringFieldState,
        spec: StringFieldSpec,
        lease: StringFieldLease,
    ) -> Self {
        Self { state, spec, lease }
    }

    /// Apply the parent-issued focus lease, render/edit, and return only
    /// source control outcomes. The parent decides all feature commands.
    #[must_use]
    pub(crate) fn show(self, ui: &mut egui::Ui) -> StringFieldOutput {
        let Self { state, spec, lease } = self;
        let state_had_focus = state.focused_control == Some(spec.control_id);
        let widget_had_focus = ui.memory(|memory| memory.has_focus(spec.id));
        let key_events = ui.input(|input| {
            input
                .events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } => Some((*key, *modifiers)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        });
        let has_text_event = ui.input(|input| {
            input.events.iter().any(|event| {
                matches!(event, egui::Event::Text(text) | egui::Event::Paste(text) if !text.is_empty())
            })
        });
        let accepts_keys =
            lease.handles_input && (state_had_focus || widget_had_focus || lease.requests_focus);
        // egui clears Escape focus before this widget is drawn. Reinstalling
        // the parent's admitted lease here also covers that case; a second
        // post-draw Escape request would be redundant.
        let install_focus =
            lease.requests_focus || (lease.handles_input && state_had_focus && !widget_had_focus);

        if accepts_keys && key_events.iter().any(|(key, _)| *key == egui::Key::Insert) {
            state.overwrite = !state.overwrite;
        }
        if install_focus {
            if lease.select_all {
                state.select_all();
            }
            ui.memory_mut(|memory| memory.request_focus(spec.id));
        }
        let before = state.text.clone();
        state.truncate(spec.max_length);
        state.store_cursor(ui.ctx(), spec.id);

        let selected = state.selection();
        let whole_value_selected = !state.text.is_empty()
            && selected
                .as_ref()
                .is_some_and(|range| range.start == 0 && range.end == state.text.chars().count());
        if whole_value_selected {
            ui.painter()
                .rect_filled(spec.rect, 0.0, spec.style.selection_background);
        }
        let mut buffer = FilteredBuffer {
            text: &mut state.text,
            max_length: spec.max_length,
            overwrite: state.overwrite,
        };
        let style = &spec.style;
        let mut layouter = |ui: &egui::Ui, value: &str, _wrap_width: f32| {
            let mut job = egui::text::LayoutJob::default();
            for (index, character) in value.chars().enumerate() {
                let color = if selected
                    .as_ref()
                    .is_some_and(|range| range.contains(&index))
                {
                    style.selected_text_color
                } else {
                    style.text_color
                };
                job.append(
                    character.encode_utf8(&mut [0; 4]),
                    0.0,
                    egui::TextFormat::simple(style.font.clone(), color),
                );
            }
            ui.fonts(|fonts| fonts.layout_job(job))
        };
        let response = ui
            .scope(|ui| {
                let visuals = ui.visuals_mut();
                visuals.selection.bg_fill = style.selection_background;
                visuals.selection.stroke.color = style.selected_text_color;
                visuals.text_cursor.stroke.color = style.cursor_color;
                ui.set_clip_rect(ui.clip_rect().intersect(spec.rect));
                ui.put(
                    spec.rect,
                    egui::TextEdit::singleline(&mut buffer)
                        .id(spec.id)
                        .interactive(lease.interactive)
                        .frame(style.frame)
                        .margin(style.margin)
                        .font(style.font.clone())
                        .text_color(style.text_color)
                        .layouter(&mut layouter)
                        .desired_width(spec.rect.width())
                        .return_key(None),
                )
            })
            .inner;
        // Controls inside an `Area` may not be registered when the early
        // memory request runs. Repeat the same lease through the registered
        // response so central panels and layered original-game windows share
        // one focus-acquisition contract.
        if install_focus {
            response.request_focus();
        }
        state.load_cursor(ui.ctx(), spec.id);
        if lease.handles_input && install_focus && !widget_had_focus && state.text == before {
            let fallback = ui.input(|input| {
                input
                    .events
                    .iter()
                    .filter_map(|event| match event {
                        egui::Event::Text(text) | egui::Event::Paste(text) => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<String>()
            });
            if !fallback.is_empty() {
                state.insert_fallback_text(&fallback, spec.max_length);
                state.store_cursor(ui.ctx(), spec.id);
            }
        }

        let has_focus = response.has_focus();
        state.focused_control = has_focus.then_some(spec.control_id);
        let mut output = StringFieldOutput::default();
        if before != state.text {
            output.push(StringFieldEvent::Changed);
        }
        if has_focus && (!state_had_focus || !widget_had_focus) {
            output.push(StringFieldEvent::FocusChanged(true));
        } else if !has_focus && (state_had_focus || widget_had_focus) {
            output.push(StringFieldEvent::FocusChanged(false));
        }
        if accepts_keys {
            for (key, modifiers) in key_events {
                match key {
                    egui::Key::Enter => output.push(StringFieldEvent::Submitted),
                    egui::Key::Escape => output.push(StringFieldEvent::Cancelled),
                    egui::Key::Insert
                    | egui::Key::Backspace
                    | egui::Key::Delete
                    | egui::Key::Home
                    | egui::Key::End
                    | egui::Key::ArrowLeft
                    | egui::Key::ArrowRight => {}
                    _ if modifiers.command || modifiers.ctrl => {}
                    _ if has_text_event && key_has_text_event(key) => {}
                    _ => output.push(StringFieldEvent::UnhandledKey(key)),
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use egui_macroquad::egui;
    use rebellion_windowing::ControlId;

    use super::*;

    const CONTROL: ControlId = ControlId(0x203);

    fn spec(id: &'static str, max_length: usize) -> StringFieldSpec {
        StringFieldSpec {
            id: egui::Id::new(id),
            control_id: CONTROL,
            rect: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 20.0)),
            max_length: Some(max_length),
            style: StringFieldStyle::default(),
        }
    }

    fn key(key: egui::Key) -> egui::Event {
        modified_key(key, egui::Modifiers::NONE)
    }

    fn modified_key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    #[allow(
        clippy::needless_pass_by_value,
        reason = "test call sites construct specs inline while egui's FnMut runner requires a clone"
    )]
    fn show_once(
        ctx: &egui::Context,
        state: &mut StringFieldState,
        spec: StringFieldSpec,
        lease: StringFieldLease,
        events: Vec<egui::Event>,
    ) -> StringFieldOutput {
        let mut output = StringFieldOutput::default();
        let raw = egui::RawInput {
            events,
            ..Default::default()
        };
        let _ = ctx.run(raw, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                output = StringField::new(state, spec.clone(), lease).show(ui);
            });
        });
        output
    }

    #[test]
    fn a_new_field_can_select_its_entire_initial_value() {
        let mut state = StringFieldState::new("Rogue Squadron");
        state.select_all();

        assert_eq!(state.selection(), Some(0..14));
        assert_eq!(state.caret(), 14);
    }

    #[test]
    fn source_selection_endpoints_stop_at_the_current_character_boundary() {
        let mut state = StringFieldState::new("Rogue");
        state.set_selection(usize::MAX, usize::MAX);

        assert_eq!(state.selection(), None);
        assert_eq!(state.caret(), 5);
    }

    #[test]
    fn borrowed_text_conversion_and_comparison_preserve_the_value() {
        let state = StringFieldState::from("Rogue");

        assert_eq!(state, "Rogue");
        assert_ne!(state, "Wedge");
    }

    #[test]
    fn same_frame_text_at_a_nonzero_caret_advances_past_every_inserted_character() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue");
        state.set_selection(1, 1);
        let output = show_once(
            &ctx,
            &mut state,
            spec("same-frame-field", 20),
            StringFieldLease::acquire(false),
            vec![egui::Event::Text("XY".into())],
        );

        assert_eq!(state.text(), "RXYogue");
        assert_eq!(state.caret(), 3);
        assert!(output.contains(StringFieldEvent::Changed));
    }

    #[test]
    fn fallback_insertion_advances_a_nonzero_caret_by_the_accepted_characters() {
        let mut state = StringFieldState::new("Rogue");
        state.set_selection(1, 1);
        state.insert_fallback_text("XY", Some(20));

        assert_eq!(state.text(), "RXYogue");
        assert_eq!(state.caret(), 3);
    }

    #[test]
    fn the_filtered_buffer_exposes_an_editable_text_contract() {
        let mut text = String::new();
        let buffer = FilteredBuffer {
            text: &mut text,
            max_length: None,
            overwrite: false,
        };

        assert!(egui::TextBuffer::is_mutable(&buffer));
    }

    #[test]
    fn a_partial_selection_does_not_fill_the_whole_field() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue");
        state.set_selection(0, 2);
        let field = spec("partial-selection-field", 20);
        let rect = field.rect;
        let output = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ =
                    StringField::new(&mut state, field.clone(), StringFieldLease::acquire(false))
                        .show(ui);
            });
        });

        assert!(!output.shapes.iter().any(|clipped| {
            matches!(
                &clipped.shape,
                egui::Shape::Rect(shape)
                    if shape.rect == rect && shape.fill == egui::Color32::WHITE
            )
        }));
    }

    #[test]
    fn focus_acquisition_preserves_select_all_and_reports_the_control_lease() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue");
        let output = show_once(
            &ctx,
            &mut state,
            spec("focus-field", 20),
            StringFieldLease::acquire(true),
            Vec::new(),
        );

        assert!(output.contains(StringFieldEvent::FocusChanged(true)));
        assert_eq!(state.selection(), Some(0..5));
        assert_eq!(state.focused_control(), Some(CONTROL));
    }

    #[test]
    fn a_manager_focus_handoff_selects_text_without_admitting_same_frame_keys() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue");
        let output = show_once(
            &ctx,
            &mut state,
            spec("handoff-field", 20),
            StringFieldLease::request_focus(true),
            vec![egui::Event::Text("Blocked".into()), key(egui::Key::Enter)],
        );

        assert_eq!(state.text(), "Rogue");
        assert_eq!(state.selection(), Some(0..5));
        assert!(!output.contains(StringFieldEvent::Submitted));
    }

    #[test]
    fn accepted_text_replaces_the_selection_and_stops_at_the_character_limit() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Old");
        show_once(
            &ctx,
            &mut state,
            spec("limited-field", 5),
            StringFieldLease::acquire(true),
            Vec::new(),
        );
        let output = show_once(
            &ctx,
            &mut state,
            spec("limited-field", 5),
            StringFieldLease::focused(),
            vec![egui::Event::Text("Falcon".into())],
        );

        assert_eq!(state.text(), "Falco");
        assert!(output.contains(StringFieldEvent::Changed));
    }

    #[test]
    fn an_initial_value_is_truncated_to_the_source_maximum() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue Squadron");
        let output = show_once(
            &ctx,
            &mut state,
            spec("initial-limit-field", 5),
            StringFieldLease::focused(),
            Vec::new(),
        );

        assert_eq!(state.text(), "Rogue");
        assert!(output.contains(StringFieldEvent::Changed));
    }

    #[test]
    fn the_source_character_filter_keeps_printable_text_and_rejects_controls() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("");
        show_once(
            &ctx,
            &mut state,
            spec("filtered-field", 3),
            StringFieldLease::acquire(false),
            Vec::new(),
        );
        let output = show_once(
            &ctx,
            &mut state,
            spec("filtered-field", 3),
            StringFieldLease::focused(),
            vec![egui::Event::Text("1\n2\t3".into())],
        );

        assert_eq!(state.text(), "123");
        assert!(output.contains(StringFieldEvent::Changed));
    }

    #[test]
    fn insert_toggles_overwrite_and_replaces_from_the_caret() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue");
        state.set_selection(1, 1);
        show_once(
            &ctx,
            &mut state,
            spec("overwrite-field", 5),
            StringFieldLease::acquire(false),
            Vec::new(),
        );
        let output = show_once(
            &ctx,
            &mut state,
            spec("overwrite-field", 5),
            StringFieldLease::focused(),
            vec![key(egui::Key::Insert), egui::Event::Text("X".into())],
        );

        assert!(state.overwrite());
        assert_eq!(state.text(), "RXgue");
        assert!(output.contains(StringFieldEvent::Changed));
        assert!(!output.contains(StringFieldEvent::UnhandledKey(egui::Key::Insert)));
    }

    #[test]
    fn enter_escape_and_unhandled_navigation_are_typed_owner_outcomes() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue");
        show_once(
            &ctx,
            &mut state,
            spec("key-field", 20),
            StringFieldLease::acquire(false),
            Vec::new(),
        );

        let submit = show_once(
            &ctx,
            &mut state,
            spec("key-field", 20),
            StringFieldLease::focused(),
            vec![key(egui::Key::Enter)],
        );
        assert!(submit.contains(StringFieldEvent::Submitted));

        let cancel = show_once(
            &ctx,
            &mut state,
            spec("key-field", 20),
            StringFieldLease::focused(),
            vec![key(egui::Key::Escape)],
        );
        assert!(cancel.contains(StringFieldEvent::Cancelled));
        assert_eq!(state.focused_control(), Some(CONTROL));

        let forwarded = show_once(
            &ctx,
            &mut state,
            spec("key-field", 20),
            StringFieldLease::focused(),
            vec![
                key(egui::Key::ArrowDown),
                key(egui::Key::A),
                egui::Event::Text("a".into()),
            ],
        );
        assert!(forwarded.contains(StringFieldEvent::UnhandledKey(egui::Key::ArrowDown)));
        assert!(!forwarded.contains(StringFieldEvent::UnhandledKey(egui::Key::A)));
    }

    #[test]
    fn a_blocked_field_neither_consumes_escape_nor_uses_it_to_take_focus() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue");
        let output = show_once(
            &ctx,
            &mut state,
            spec("blocked-escape-field", 20),
            StringFieldLease::blocked(),
            vec![key(egui::Key::Escape)],
        );

        assert!(!output.contains(StringFieldEvent::Cancelled));
        assert_eq!(state.focused_control(), None);
    }

    #[test]
    fn control_and_command_shortcuts_stay_inside_the_field() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue");
        show_once(
            &ctx,
            &mut state,
            spec("shortcut-field", 20),
            StringFieldLease::acquire(false),
            Vec::new(),
        );

        for (key, modifiers) in [
            (egui::Key::A, egui::Modifiers::CTRL),
            (egui::Key::C, egui::Modifiers::COMMAND),
        ] {
            let output = show_once(
                &ctx,
                &mut state,
                spec("shortcut-field", 20),
                StringFieldLease::focused(),
                vec![modified_key(key, modifiers)],
            );
            assert!(!output.contains(StringFieldEvent::UnhandledKey(key)));
        }
    }

    #[test]
    fn an_empty_submit_is_reported_without_imposing_feature_validation() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("");
        show_once(
            &ctx,
            &mut state,
            spec("empty-field", 20),
            StringFieldLease::acquire(false),
            Vec::new(),
        );
        let output = show_once(
            &ctx,
            &mut state,
            spec("empty-field", 20),
            StringFieldLease::focused(),
            vec![key(egui::Key::Enter)],
        );

        assert!(output.contains(StringFieldEvent::Submitted));
        assert_eq!(state.text(), "");
    }

    #[test]
    fn losing_widget_focus_reports_release_without_deciding_parent_policy() {
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue");
        show_once(
            &ctx,
            &mut state,
            spec("loss-field", 20),
            StringFieldLease::acquire(false),
            Vec::new(),
        );
        let mut output = StringFieldOutput::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.memory_mut(|memory| memory.request_focus(egui::Id::new("other-field")));
                output = StringField::new(
                    &mut state,
                    spec("loss-field", 20),
                    StringFieldLease::blocked(),
                )
                .show(ui);
                let mut other = String::new();
                ui.add(egui::TextEdit::singleline(&mut other).id(egui::Id::new("other-field")));
            });
        });

        assert!(output.contains(StringFieldEvent::FocusChanged(false)));
        assert_eq!(state.focused_control(), None);
    }

    #[test]
    fn escape_keeps_widget_focus_until_the_owner_revokes_the_field_lease() {
        // FUN_00605160 forwards Escape to the owner without releasing focus.
        let ctx = egui::Context::default();
        let mut state = StringFieldState::new("Rogue");
        let field = spec("escape-widget-focus", 20);
        show_once(
            &ctx,
            &mut state,
            field.clone(),
            StringFieldLease::acquire(false),
            Vec::new(),
        );
        let output = show_once(
            &ctx,
            &mut state,
            field.clone(),
            StringFieldLease::focused(),
            vec![key(egui::Key::Escape)],
        );
        assert!(output.contains(StringFieldEvent::Cancelled));
        assert!(ctx.memory(|memory| memory.has_focus(field.id)));
    }
}
