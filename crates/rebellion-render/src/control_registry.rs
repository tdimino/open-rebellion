//! Named hit rects for the developer commands.
//!
//! A native check that clicks pixel coordinates breaks whenever the galaxy
//! differs. In a dev run each draw function files its controls here by name,
//! `<window>/<control>`, every frame, in the 640×480 logical screen the dev
//! `Click` uses, and `Click control:` finds the rect by name. The app turns
//! the registry on only for a dev run (`set_enabled`); off, every call is one
//! relaxed load and returns. Nothing here reads or changes game state, draws,
//! or touches the simulation's RNG.
//!
//! A frame's registrations are complete only once the frame is drawn, so the
//! app calls [`begin_frame`] first: it makes the frame just drawn the
//! snapshot the dev commands read and starts a new one. A name filed twice in
//! a frame keeps its last rect (the focused window draws last).

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};

use egui_macroquad::egui;

static ENABLED: AtomicBool = AtomicBool::new(false);

/// A control's rect in the 640×480 logical screen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl ControlRect {
    /// The point a click on the control lands on.
    #[must_use]
    pub fn center(self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

#[derive(Default)]
struct Registry {
    /// Where the egui screen's logical canvas sits: left, top and scale.
    canvas: (f32, f32, f32),
    menu_window: Option<&'static str>,
    current: Vec<(String, ControlRect)>,
    shown: Vec<(String, ControlRect)>,
}

thread_local! {
    static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
}

/// Turn the registry on or off. The app turns it on for a dev run only.
pub fn set_enabled(on: bool) {
    ENABLED.store(on, Ordering::Relaxed);
}

/// Whether controls are being filed. Check it before building a name.
#[must_use]
pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// Start a frame: the frame just drawn becomes the snapshot.
pub fn begin_frame() {
    if !enabled() {
        return;
    }
    REGISTRY.with_borrow_mut(|registry| {
        let drawn = std::mem::take(&mut registry.current);
        registry.shown = drawn;
    });
}

/// Name the window of the next pop-up menu `draw_game_menu` draws, whose
/// rows are filed under it (`Menu` when unnamed).
pub fn name_next_menu(window: &'static str) {
    if enabled() {
        REGISTRY.with_borrow_mut(|registry| registry.menu_window = Some(window));
    }
}

/// The window the menu being drawn is filed under, and the name cleared.
#[must_use]
pub fn take_menu_window() -> &'static str {
    if !enabled() {
        return "Menu";
    }
    REGISTRY.with_borrow_mut(|registry| registry.menu_window.take().unwrap_or("Menu"))
}

/// Where the logical canvas sits on the egui screen this frame, so
/// [`register_screen`] can turn screen rects into logical ones.
pub fn set_canvas(left: f32, top: f32, scale: f32) {
    if enabled() {
        REGISTRY.with_borrow_mut(|registry| registry.canvas = (left, top, scale));
    }
}

fn file(window: &str, control: &str, rect: ControlRect) {
    let name = format!("{window}/{control}");
    REGISTRY.with_borrow_mut(|registry| {
        match registry
            .current
            .iter_mut()
            .find(|(known, _)| *known == name)
        {
            Some(entry) => entry.1 = rect,
            None => registry.current.push((name, rect)),
        }
    });
}

/// File a control by its logical rect.
pub fn register_logical(window: &str, control: &str, x: f32, y: f32, width: f32, height: f32) {
    if enabled() {
        file(
            window,
            control,
            ControlRect {
                x,
                y,
                width,
                height,
            },
        );
    }
}

/// File a control by its rect on the egui screen.
pub fn register_screen(window: &str, control: &str, rect: egui::Rect) {
    if !enabled() {
        return;
    }
    let (left, top, scale) = REGISTRY.with_borrow(|registry| registry.canvas);
    if scale > 0.0 {
        file(
            window,
            control,
            ControlRect {
                x: (rect.min.x - left) / scale,
                y: (rect.min.y - top) / scale,
                width: rect.width() / scale,
                height: rect.height() / scale,
            },
        );
    }
}

/// Names match without regard to case, and `-`, `_` and space are one.
fn same(a: &str, b: &str) -> bool {
    let fold = |c: char| {
        if matches!(c, '-' | '_') {
            ' '
        } else {
            c.to_ascii_lowercase()
        }
    };
    a.chars().map(fold).eq(b.chars().map(fold))
}

/// The rect filed under `<window>/<control>` in the last drawn frame.
#[must_use]
pub fn find(name: &str) -> Option<ControlRect> {
    REGISTRY.with_borrow(|registry| {
        registry
            .shown
            .iter()
            .find(|(known, _)| same(known, name))
            .map(|&(_, rect)| rect)
    })
}

/// The names filed in the last drawn frame, those under `window` only when
/// given, in drawing order.
#[must_use]
pub fn names(window: Option<&str>) -> Vec<String> {
    REGISTRY.with_borrow(|registry| {
        registry
            .shown
            .iter()
            .map(|(name, _)| name)
            .filter(|name| {
                window.is_none_or(|window| {
                    name.split_once('/')
                        .is_some_and(|(head, _)| same(head, window))
                })
            })
            .cloned()
            .collect()
    })
}
