//! The original bitmap slider (`FUN_00604570`, `WndProc` `0x00604b80`).
//!
//! The control owns pointer capture and maps the horizontal rail to its
//! integer range. Feature modules retain bitmap selection and the meaning of
//! the resulting value.

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SliderSpec {
    minimum: u32,
    maximum: u32,
    travel: f32,
    pointer_offset: f32,
    handle_offset: f32,
}

impl SliderSpec {
    pub(crate) const fn horizontal(minimum: u32, maximum: u32, travel: f32) -> Self {
        Self {
            minimum,
            maximum,
            travel,
            pointer_offset: 0.0,
            handle_offset: 0.0,
        }
    }

    pub(crate) const fn with_offsets(mut self, pointer: f32, handle: f32) -> Self {
        self.pointer_offset = pointer;
        self.handle_offset = handle;
        self
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct SliderState {
    captured: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct SliderInput {
    pub pointer: Option<f32>,
    pub over: bool,
    pub pressed: bool,
    pub down: bool,
    pub released: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SliderOutput {
    pub value: u32,
    pub changed: bool,
    pub committed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Slider {
    spec: SliderSpec,
}

impl Slider {
    pub(crate) const fn new(spec: SliderSpec) -> Self {
        Self { spec }
    }

    fn bounded_value(self, value: u32) -> u32 {
        value.clamp(self.spec.minimum, self.spec.maximum)
    }

    fn value_at(self, pointer: f32) -> u32 {
        let span = self.spec.maximum.saturating_sub(self.spec.minimum);
        if span == 0 || self.spec.travel <= 0.0 {
            return self.spec.minimum;
        }
        let share = ((pointer - self.spec.pointer_offset) / self.spec.travel).clamp(0.0, 1.0);
        self.spec.minimum + (share * span as f32).round() as u32
    }

    pub(crate) fn handle_offset(self, value: u32) -> f32 {
        let span = self.spec.maximum.saturating_sub(self.spec.minimum);
        if span == 0 || self.spec.travel <= 0.0 {
            return self.spec.handle_offset;
        }
        let value = self.bounded_value(value) - self.spec.minimum;
        // The original constructor and painter use integer arithmetic for
        // the bitmap's whole-pixel displacement.
        self.spec.handle_offset + (self.spec.travel * value as f32 / span as f32).floor()
    }

    pub(crate) fn update(
        self,
        state: &mut SliderState,
        value: u32,
        input: SliderInput,
    ) -> SliderOutput {
        let value = self.bounded_value(value);
        if !input.enabled {
            state.captured = false;
            return SliderOutput {
                value,
                changed: false,
                committed: false,
            };
        }
        if input.pressed && input.over {
            state.captured = true;
        }
        let next = if state.captured && (input.pressed || input.down || input.released) {
            input
                .pointer
                .map_or(value, |pointer| self.value_at(pointer))
        } else {
            value
        };
        let committed = input.released && state.captured;
        if input.released {
            state.captured = false;
        }
        SliderOutput {
            value: next,
            changed: next != value,
            committed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: SliderSpec = SliderSpec::horizontal(0, 100, 199.0).with_offsets(5.0, 1.0);

    fn input(pointer: f32) -> SliderInput {
        SliderInput {
            pointer: Some(pointer),
            over: true,
            enabled: true,
            ..SliderInput::default()
        }
    }

    #[test]
    fn slider_captures_updates_outside_and_commits_the_integer_value() {
        // WndProc 0x00604b80 captures on press, updates while held even after
        // leaving the rail, and notifies the owner with the value on release.
        let slider = Slider::new(SPEC);
        let mut state = SliderState::default();
        let pressed = slider.update(
            &mut state,
            0,
            SliderInput {
                pressed: true,
                ..input(104.5)
            },
        );
        assert_eq!(pressed.value, 50);
        assert!(pressed.changed);
        assert!(!pressed.committed);

        let dragged = slider.update(
            &mut state,
            pressed.value,
            SliderInput {
                pointer: Some(400.0),
                down: true,
                enabled: true,
                ..SliderInput::default()
            },
        );
        assert_eq!(dragged.value, 100);

        let released = slider.update(
            &mut state,
            dragged.value,
            SliderInput {
                pointer: Some(400.0),
                released: true,
                enabled: true,
                ..SliderInput::default()
            },
        );
        assert_eq!(released.value, 100);
        assert!(released.committed);
    }

    #[test]
    fn slider_ignores_an_outside_press_and_places_the_handle_on_whole_pixels() {
        let slider = Slider::new(SPEC);
        let mut state = SliderState::default();
        let output = slider.update(
            &mut state,
            40,
            SliderInput {
                pointer: Some(104.5),
                pressed: true,
                enabled: true,
                ..SliderInput::default()
            },
        );
        assert_eq!(output.value, 40);
        assert!(!output.changed);
        assert!(!output.committed);
        assert_eq!(slider.handle_offset(50), 100.0);
    }

    #[test]
    fn disabling_a_captured_slider_cancels_the_drag_without_committing() {
        let slider = Slider::new(SPEC);
        let mut state = SliderState::default();
        let pressed = slider.update(
            &mut state,
            10,
            SliderInput {
                pressed: true,
                ..input(44.8)
            },
        );
        assert_eq!(pressed.value, 20);

        let disabled = slider.update(
            &mut state,
            pressed.value,
            SliderInput {
                pointer: Some(204.0),
                released: true,
                ..SliderInput::default()
            },
        );
        assert_eq!(disabled.value, 20);
        assert!(!disabled.changed);
        assert!(!disabled.committed);

        let later_release = slider.update(
            &mut state,
            disabled.value,
            SliderInput {
                pointer: Some(204.0),
                released: true,
                enabled: true,
                ..SliderInput::default()
            },
        );
        assert_eq!(later_release.value, 20);
        assert!(!later_release.committed);
    }

    #[test]
    fn fixed_ranges_and_zero_travel_keep_the_handle_at_its_configured_origin() {
        let fixed = Slider::new(SliderSpec::horizontal(25, 25, 199.0).with_offsets(5.0, 3.0));
        assert_eq!(fixed.handle_offset(25), 3.0);

        let stationary = Slider::new(SliderSpec::horizontal(0, 100, 0.0).with_offsets(5.0, 7.0));
        assert_eq!(stationary.handle_offset(50), 7.0);
        let shifted = Slider::new(SliderSpec::horizontal(25, 125, 200.0).with_offsets(5.0, 3.0));
        assert_eq!(shifted.handle_offset(75), 103.0);

        let mut fixed_state = SliderState::default();
        assert_eq!(
            fixed
                .update(
                    &mut fixed_state,
                    25,
                    SliderInput {
                        pressed: true,
                        ..input(104.5)
                    },
                )
                .value,
            25
        );
        let mut stationary_state = SliderState::default();
        assert_eq!(
            stationary
                .update(
                    &mut stationary_state,
                    25,
                    SliderInput {
                        pressed: true,
                        ..input(104.5)
                    },
                )
                .value,
            0
        );
        let mut shifted_state = SliderState::default();
        assert_eq!(
            shifted
                .update(
                    &mut shifted_state,
                    25,
                    SliderInput {
                        pressed: true,
                        ..input(105.0)
                    },
                )
                .value,
            75
        );
    }
}
