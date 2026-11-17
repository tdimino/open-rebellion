//! Source-shaped renderer controls shared by the original-game surfaces.

mod drag_list;
mod strobe_button;
mod tab_control;

pub(crate) use drag_list::{
    paint_label, DragList, DragListEvent, DragListLayout, DragListSpec, DragListState, DragPolicy,
    SelectionPolicy, WheelPolicy,
};
pub use strobe_button::{
    ButtonArt, ButtonDraw, ButtonImage, ButtonInput, ButtonOutcome, ButtonOutput, ButtonRepeat,
    ButtonState, ButtonVisualState, Checkbox, StrobeButton, StrobeButtonResponse,
};
pub use tab_control::{
    TabControl, TabControlOutput, TabControlState, TabDirection, TabPresentation, TabSpec,
    TabTraversal,
};
