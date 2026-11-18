//! The original `CoolTabControl` composed from [`super::StrobeButton`] children.
//!
//! `FUN_0060d590` owns the child strip and `FUN_0060d860` changes the selected
//! child before notifying the feature window. Feature enums, resource lookup,
//! page contents, and commands remain with that feature.

use std::hash::Hash;

use egui_macroquad::egui;
use rebellion_windowing::ControlId;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::native_geometry::rect_contains;

use super::{
    draw_registered_tooltips, ButtonArt, ButtonInput, ButtonState, StrobeButton,
    TooltipPresentation,
};

/// Whether a tab is populated, empty-but-selectable, or disabled.
///
/// Empty tabs remain interactive. Their feature supplies the recovered empty
/// bitmap in [`TabSpec::art`], while this type keeps empty distinct from the
/// native disabled state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TabPresentation {
    #[default]
    Enabled,
    Empty,
    Disabled,
}

impl TabPresentation {
    #[must_use]
    pub const fn disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }
}

/// One feature-configured child of a [`TabControl`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TabSpec<T> {
    pub value: T,
    pub control_id: ControlId,
    pub rect: egui::Rect,
    pub art: ButtonArt,
    pub presentation: TabPresentation,
    pub tooltip: Option<&'static str>,
}

impl<T> TabSpec<T> {
    #[must_use]
    pub const fn new(value: T, control_id: ControlId, rect: egui::Rect, art: ButtonArt) -> Self {
        Self {
            value,
            control_id,
            rect,
            art,
            presentation: TabPresentation::Enabled,
            tooltip: None,
        }
    }

    /// Configure the common normal/held pair whose selected image is held.
    #[must_use]
    pub const fn two_state(
        value: T,
        control_id: ControlId,
        rect: egui::Rect,
        source: DllSource,
        normal: u32,
        held: u32,
    ) -> Self {
        Self::new(
            value,
            control_id,
            rect,
            ButtonArt::two_state(source, normal, held),
        )
    }

    #[must_use]
    pub const fn with_presentation(mut self, presentation: TabPresentation) -> Self {
        self.presentation = presentation;
        self
    }

    /// Register the source feature's TEXTSTRA help text with the shared stock
    /// tooltip control when this tab strip is painted.
    #[must_use]
    pub const fn with_tooltip(mut self, tooltip: &'static str) -> Self {
        self.tooltip = Some(tooltip);
        self
    }
}

/// Persistent selection and focus identity owned by the tab strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabControlState<T> {
    pub selected: T,
    pub focused: ControlId,
}

impl<T> TabControlState<T> {
    #[must_use]
    pub const fn new(selected: T, focused: ControlId) -> Self {
        Self { selected, focused }
    }
}

/// One typed parent notification, corresponding to the selected-child command
/// that `FUN_0060d860` sends after changing the native strip state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabControlOutput<T> {
    Selected(T),
}

impl<T: Copy> TabControlOutput<T> {
    #[must_use]
    pub const fn selected(self) -> T {
        match self {
            Self::Selected(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabDirection {
    Left,
    Right,
}

/// Recovered boundary behavior for feature key handlers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabTraversal {
    /// Keep the current enabled tab at the outer boundary.
    Clamp,
    /// The Report handlers return the first tab when stepping past either
    /// outer boundary instead of wrapping left to the final tab.
    ResetToFirst,
}

/// A typed tab strip whose children are source-shaped [`StrobeButton`]s.
pub struct TabControl<'a, T> {
    specs: &'a [TabSpec<T>],
    state: &'a mut TabControlState<T>,
}

impl<'a, T> TabControl<'a, T>
where
    T: Copy + PartialEq,
{
    #[must_use]
    pub const fn new(specs: &'a [TabSpec<T>], state: &'a mut TabControlState<T>) -> Self {
        Self { specs, state }
    }

    #[must_use]
    pub const fn state(&self) -> &TabControlState<T> {
        self.state
    }

    /// Paint a feature-owned selection and return the child's typed parent
    /// notification. This is the common per-frame lifecycle for strips whose
    /// authoritative selection remains in their feature window.
    #[must_use]
    pub fn paint_and_select<S: Copy + Hash>(
        specs: &[TabSpec<T>],
        mut state: TabControlState<T>,
        ui: &egui::Ui,
        cache: &mut BmpCache,
        strip_id: S,
        scale: f32,
    ) -> Option<T> {
        TabControl::new(specs, &mut state)
            .show(ui, cache, strip_id, scale)
            .map(TabControlOutput::selected)
    }

    /// Bitmap-mask counterpart to [`Self::paint_and_select`].
    #[must_use]
    pub fn paint_and_select_with_inputs<S: Copy + Hash>(
        specs: &[TabSpec<T>],
        mut state: TabControlState<T>,
        ui: &egui::Ui,
        cache: &mut BmpCache,
        strip_id: S,
        scale: f32,
        inputs: &[ButtonInput],
    ) -> Option<T> {
        TabControl::new(specs, &mut state)
            .show_with_inputs(ui, cache, strip_id, scale, inputs)
            .map(TabControlOutput::selected)
    }

    /// Resolve one exact point against a feature-owned selection.
    #[must_use]
    pub fn select_point(
        specs: &[TabSpec<T>],
        mut state: TabControlState<T>,
        point: egui::Pos2,
    ) -> Option<T> {
        TabControl::new(specs, &mut state)
            .select_at(point)
            .map(TabControlOutput::selected)
    }

    /// Resolve one recovered keyboard step against a feature-owned selection.
    #[must_use]
    pub fn step_selection(
        specs: &[TabSpec<T>],
        mut state: TabControlState<T>,
        direction: TabDirection,
        traversal: TabTraversal,
    ) -> Option<T> {
        TabControl::new(specs, &mut state)
            .traverse(direction, traversal)
            .map(TabControlOutput::selected)
    }

    /// Synchronize a feature-driven selection and its stable child focus.
    pub fn select(&mut self, value: T) {
        if let Some(spec) = self.specs.iter().find(|spec| spec.value == value) {
            self.state.selected = value;
            self.state.focused = spec.control_id;
        }
    }

    /// Compose the configured child into its exact visual button state.
    #[must_use]
    pub fn button(&self, value: T, held: bool) -> Option<StrobeButton> {
        self.specs
            .iter()
            .find(|spec| spec.value == value)
            .map(|spec| {
                StrobeButton::new(spec.art).with_state(ButtonState::visual(
                    value == self.state.selected,
                    spec.presentation.disabled(),
                    held,
                ))
            })
    }

    /// Select the enabled or empty tab under an exact half-open hit region.
    #[must_use]
    pub fn select_at(&mut self, point: egui::Pos2) -> Option<TabControlOutput<T>> {
        let spec = self
            .specs
            .iter()
            .find(|spec| !spec.presentation.disabled() && rect_contains(spec.rect, point))?;
        let value = spec.value;
        self.select(value);
        Some(TabControlOutput::Selected(value))
    }

    /// Move selection over enabled and empty tabs using a feature's recovered
    /// boundary rule. Disabled children never receive keyboard focus.
    #[must_use]
    pub fn traverse(
        &mut self,
        direction: TabDirection,
        traversal: TabTraversal,
    ) -> Option<TabControlOutput<T>> {
        let first_enabled = self
            .specs
            .iter()
            .position(|spec| !spec.presentation.disabled())?;
        let current = self
            .specs
            .iter()
            .position(|spec| spec.control_id == self.state.focused)
            .or_else(|| {
                self.specs
                    .iter()
                    .position(|spec| spec.value == self.state.selected)
            })
            .unwrap_or(first_enabled);
        let candidate = match direction {
            TabDirection::Left => (0..current)
                .rev()
                .find(|&index| !self.specs[index].presentation.disabled()),
            TabDirection::Right => (current + 1..self.specs.len())
                .find(|&index| !self.specs[index].presentation.disabled()),
        };
        let target = candidate.unwrap_or(match traversal {
            TabTraversal::Clamp => {
                if self.specs[current].presentation.disabled() {
                    first_enabled
                } else {
                    current
                }
            }
            TabTraversal::ResetToFirst => first_enabled,
        });
        let value = self.specs[target].value;
        self.select(value);
        Some(TabControlOutput::Selected(value))
    }

    /// Paint and run every child through [`StrobeButton`]. The strip id and
    /// stable [`ControlId`] jointly isolate persistent child capture state.
    #[must_use]
    pub fn show<S: Copy + Hash>(
        &mut self,
        ui: &egui::Ui,
        cache: &mut BmpCache,
        strip_id: S,
        scale: f32,
    ) -> Option<TabControlOutput<T>> {
        let mut selected = None;
        for spec in self.specs {
            if StrobeButton::show(
                ui,
                cache,
                spec.rect,
                (strip_id, spec.control_id),
                spec.art,
                spec.value == self.state.selected,
                spec.presentation.disabled(),
                None,
                scale,
            )
            .clicked()
            {
                selected = Some(spec.value);
            }
        }
        let tooltips = self
            .specs
            .iter()
            .filter_map(|spec| spec.tooltip.map(|text| (spec.rect, text)))
            .collect::<Vec<_>>();
        if !tooltips.is_empty() {
            draw_registered_tooltips(
                ui.ctx(),
                egui::Id::new((strip_id, "stock-tooltips")),
                &tooltips,
                scale,
                TooltipPresentation::stock(),
            );
        }
        selected.map(|value| {
            self.select(value);
            TabControlOutput::Selected(value)
        })
    }

    /// Bitmap-mask variant used by controls whose source artwork, rather than
    /// the bounding rectangle, admits the pointer. Input order matches specs.
    #[must_use]
    pub fn show_with_inputs<S: Copy + Hash>(
        &mut self,
        ui: &egui::Ui,
        cache: &mut BmpCache,
        strip_id: S,
        scale: f32,
        inputs: &[ButtonInput],
    ) -> Option<TabControlOutput<T>> {
        let mut selected = None;
        for (spec, input) in self.specs.iter().zip(inputs.iter().copied()) {
            if StrobeButton::show_with_input(
                ui,
                cache,
                spec.rect,
                (strip_id, spec.control_id),
                spec.art,
                spec.value == self.state.selected,
                spec.presentation.disabled(),
                None,
                scale,
                input,
            )
            .clicked()
            {
                selected = Some(spec.value);
            }
        }
        let tooltips = self
            .specs
            .iter()
            .filter_map(|spec| spec.tooltip.map(|text| (spec.rect, text)))
            .collect::<Vec<_>>();
        if !tooltips.is_empty() {
            draw_registered_tooltips(
                ui.ctx(),
                egui::Id::new((strip_id, "stock-tooltips")),
                &tooltips,
                scale,
                TooltipPresentation::stock(),
            );
        }
        selected.map(|value| {
            self.select(value);
            TabControlOutput::Selected(value)
        })
    }
}

#[cfg(test)]
mod tests {
    use egui_macroquad::egui;
    use rebellion_windowing::ControlId;

    use crate::bmp_cache::DllSource;
    use crate::controls::{ButtonArt, ButtonImage};

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Page {
        First,
        Empty,
        Disabled,
        Last,
    }

    const fn art(normal: u32, selected: u32, disabled: u32) -> ButtonArt {
        ButtonArt::new(ButtonImage::new(DllSource::Strategy, normal))
            .with_selected(ButtonImage::new(DllSource::Strategy, selected))
            .with_disabled(ButtonImage::new(DllSource::Strategy, disabled))
    }

    fn spec(page: Page, index: u32, presentation: TabPresentation) -> TabSpec<Page> {
        let normal = (index + 1) * 100;
        TabSpec::new(
            page,
            ControlId(u64::from(0x97 + index)),
            egui::Rect::from_min_size(egui::pos2(index as f32 * 10.0, 0.0), egui::vec2(10.0, 8.0)),
            art(normal, normal + 1, normal + 2),
        )
        .with_presentation(presentation)
    }

    fn specs() -> [TabSpec<Page>; 4] {
        [
            spec(Page::First, 0, TabPresentation::Enabled),
            spec(Page::Empty, 1, TabPresentation::Empty),
            spec(Page::Disabled, 2, TabPresentation::Disabled),
            spec(Page::Last, 3, TabPresentation::Enabled),
        ]
    }

    #[test]
    fn selected_empty_and_disabled_tabs_keep_their_configured_art_states() {
        let specs = specs();
        let mut state = TabControlState::new(Page::Empty, ControlId(0x98));
        let tabs = TabControl::new(&specs, &mut state);

        assert_eq!(
            tabs.button(Page::First, false).unwrap().image().resource_id,
            100
        );
        assert_eq!(
            tabs.button(Page::Empty, false).unwrap().image().resource_id,
            201
        );
        assert_eq!(
            tabs.button(Page::Disabled, false)
                .unwrap()
                .image()
                .resource_id,
            302
        );
    }

    #[test]
    fn source_help_text_is_configuration_not_tab_domain_policy() {
        let spec = spec(Page::First, 0, TabPresentation::Enabled).with_tooltip("First page");
        assert_eq!(spec.tooltip, Some("First page"));
        assert_eq!(spec.value, Page::First);
    }

    #[test]
    fn exact_tab_hits_include_top_left_and_exclude_bottom_right_edges() {
        let specs = specs();
        let mut state = TabControlState::new(Page::First, ControlId(0x97));
        let mut tabs = TabControl::new(&specs, &mut state);

        assert_eq!(
            tabs.select_at(egui::pos2(10.0, 0.0)),
            Some(TabControlOutput::Selected(Page::Empty))
        );
        assert_eq!(tabs.select_at(egui::pos2(20.0, 0.0)), None);
        assert_eq!(tabs.select_at(egui::pos2(40.0, 8.0)), None);
    }

    #[test]
    fn keyboard_traversal_skips_disabled_tabs_and_retains_stable_focus_ids() {
        let specs = specs();
        let mut state = TabControlState::new(Page::Empty, ControlId(0x98));
        let mut tabs = TabControl::new(&specs, &mut state);

        for (direction, page, focused) in [
            (TabDirection::Right, Page::Last, ControlId(0x9a)),
            (TabDirection::Left, Page::Empty, ControlId(0x98)),
        ] {
            assert_eq!(
                tabs.traverse(direction, TabTraversal::Clamp),
                Some(TabControlOutput::Selected(page))
            );
            assert_eq!(tabs.state().focused, focused);
        }
    }

    #[test]
    fn source_reset_traversal_returns_the_first_tab_at_either_outer_boundary() {
        for (selected, focused, direction) in [
            (Page::Last, ControlId(0x9a), TabDirection::Right),
            (Page::First, ControlId(0x97), TabDirection::Left),
        ] {
            let specs = specs();
            let mut state = TabControlState::new(selected, focused);
            assert_eq!(
                TabControl::new(&specs, &mut state).traverse(direction, TabTraversal::ResetToFirst),
                Some(TabControlOutput::Selected(Page::First))
            );
        }
    }

    #[test]
    fn faction_specific_art_is_configuration_not_tab_domain_policy() {
        for (normal, selected) in [(10_111, 10_112), (10_161, 10_162)] {
            let specs = [TabSpec::new(
                Page::First,
                ControlId(0x97),
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(10.0, 8.0)),
                ButtonArt::two_state(DllSource::Strategy, normal, selected),
            )];
            let mut state = TabControlState::new(Page::First, ControlId(0x97));
            let tabs = TabControl::new(&specs, &mut state);
            assert_eq!(
                tabs.button(Page::First, false).unwrap().image().resource_id,
                selected
            );
        }
    }
}
