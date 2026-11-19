//! Source-shaped renderer controls shared by the original-game surfaces.

mod drag_list;
mod selection_box;
mod slider;
mod status_bar;
mod string_field;
mod strobe_button;
mod tab_control;

pub(crate) use drag_list::{
    paint_label, DragList, DragListEvent, DragListInput, DragListLayout, DragListSpec,
    DragListState, DragPolicy, ListKey, SelectionPolicy, WheelPolicy,
};
pub(crate) use selection_box::{
    ScreenBounds, SelectionBox, SelectionBoxEvent, SelectionBoxSpec, SelectionBoxState,
};
pub(crate) use slider::{Slider, SliderInput, SliderSpec, SliderState};
pub(crate) use status_bar::StatusBar;
pub(crate) use string_field::{
    StringField, StringFieldEvent, StringFieldLease, StringFieldSpec, StringFieldState,
    StringFieldStyle,
};
pub use strobe_button::{
    ButtonArt, ButtonDraw, ButtonImage, ButtonInput, ButtonOutcome, ButtonOutput, ButtonRepeat,
    ButtonState, ButtonVisualState, Checkbox, StrobeButton, StrobeButtonResponse,
};
pub use tab_control::{
    TabControl, TabControlOutput, TabControlState, TabDirection, TabPresentation, TabSpec,
    TabTraversal,
};

// These two recovered families already have canonical implementations. Keep
// their source modules intact and expose their contracts beside the other
// original controls rather than cloning or mechanically moving them.
pub(crate) use crate::scroll_bar::{
    egui_pointer as scroll_bar_pointer, paint_egui as paint_scroll_bar, ScrollBar, ScrollBarArt,
    ScrollBlit, ScrollDrag, ScrollPointer, ScrollRange, FLEET_BAR_29CB, LIST_BAR_29CC,
    LIST_BAR_29FC, TEXT_BAR_299D,
};
pub use crate::tooltip::{
    draw_cockpit_tooltips, draw_registered_tooltips, TooltipPresentation, TooltipState,
    TooltipTarget,
};
