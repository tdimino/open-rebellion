//! Source-shaped renderer controls shared by the original-game surfaces.

mod strobe_button;

pub use strobe_button::{
    ButtonArt, ButtonDraw, ButtonImage, ButtonInput, ButtonOutcome, ButtonOutput, ButtonRepeat,
    ButtonState, ButtonVisualState, Checkbox, StrobeButton, StrobeButtonResponse,
};
