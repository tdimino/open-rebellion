//! Scripted pointer and key input for the developer commands.
//!
//! On macOS the game builds real `NSEvent`s and hands each to its own view's
//! handler (`mouseDown:`, `keyDown:` …) through
//! `performSelector:withObject:afterDelay:0`. The run loop runs that call
//! between frames, where miniquad dispatches hardware events, so the event
//! enters miniquad's input path into macroquad and egui exactly as a hand's
//! would, and a scripted click tests that path rather than bypassing it.
//! Posting through `-[NSApplication postEvent:]` instead lets AppKit spend a
//! button press on activating the inactive window (2026-10-09: moves
//! arrived, presses never did). Nothing moves the hardware pointer or
//! changes which app is in front. One step posts per frame, so a press and its release land in
//! different frames, as a hand's would. Other platforms refuse the commands.
//! The FFI uses the `objc` macros miniquad re-exports: no new dependency.

use std::collections::VecDeque;

/// The original game's logical screen, which every coordinate names.
#[cfg(target_os = "macos")]
const LOGICAL: (f32, f32) = (640.0, 480.0);
/// Frames a drag spends moving between its press and its release.
const DRAG_FRAMES: u16 = 8;

/// One posted event.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Step {
    Move(f32, f32),
    Down {
        x: f32,
        y: f32,
        right: bool,
    },
    Drag(f32, f32),
    Up {
        x: f32,
        y: f32,
        right: bool,
    },
    Key {
        code: u16,
        down: bool,
    },
    /// Control pressed or released, as AppKit reports a modifier key.
    Ctrl(bool),
    /// One wheel line: positive away from the user (up), negative down.
    Scroll(i32),
}

/// Input waiting to be posted, one step a frame.
#[derive(Debug, Default)]
pub struct DevInput {
    steps: VecDeque<Step>,
}

impl DevInput {
    /// Whether steps are still waiting; the command queue holds until not.
    #[must_use]
    pub fn busy(&self) -> bool {
        !self.steps.is_empty()
    }

    /// Queue a click at a logical point, with Control held around it when
    /// `ctrl`.
    pub fn click(&mut self, x: f32, y: f32, right: bool, ctrl: bool) {
        if ctrl {
            self.steps.push_back(Step::Ctrl(true));
        }
        self.steps.extend([
            Step::Move(x, y),
            Step::Down { x, y, right },
            Step::Up { x, y, right },
        ]);
        if ctrl {
            self.steps.push_back(Step::Ctrl(false));
        }
    }

    /// Queue a press at `from`, a move in even steps, and a release at `to`,
    /// with Control held around them when `ctrl`.
    pub fn drag(&mut self, from: (f32, f32), to: (f32, f32), ctrl: bool) {
        if ctrl {
            self.steps.push_back(Step::Ctrl(true));
        }
        self.hold_drag(from, to);
        self.release(to.0, to.1);
        if ctrl {
            self.steps.push_back(Step::Ctrl(false));
        }
    }

    /// Queue a press at `from` and a move in even steps to `to`, leaving the
    /// left button held until [`Self::release`]. A focus change can then
    /// land mid-gesture, which a `drag` finishes too quickly to meet.
    pub fn hold_drag(&mut self, from: (f32, f32), to: (f32, f32)) {
        self.steps.push_back(Step::Move(from.0, from.1));
        self.steps.push_back(Step::Down {
            x: from.0,
            y: from.1,
            right: false,
        });
        for i in 1..=DRAG_FRAMES {
            let t = f32::from(i) / f32::from(DRAG_FRAMES);
            self.steps.push_back(Step::Drag(
                from.0 + (to.0 - from.0) * t,
                from.1 + (to.1 - from.1) * t,
            ));
        }
    }

    /// Queue the left button's release at a logical point.
    pub fn release(&mut self, x: f32, y: f32) {
        self.steps.push_back(Step::Up { x, y, right: false });
    }

    /// Queue a move to a logical point and `notches` wheel lines there, one
    /// a frame: positive scrolls down a list, negative up.
    pub fn scroll(&mut self, x: f32, y: f32, notches: i32) {
        self.steps.push_back(Step::Move(x, y));
        let line = if notches > 0 { -1 } else { 1 };
        self.steps
            .extend((0..notches.unsigned_abs()).map(|_| Step::Scroll(line)));
    }

    /// Queue a key's press and release, or say why the name is unknown.
    pub fn press(&mut self, key: &str) -> Result<(), String> {
        let code = key_code(key).ok_or_else(|| format!("no key named {key:?}"))?;
        self.steps.extend([
            Step::Key { code, down: true },
            Step::Key { code, down: false },
        ]);
        Ok(())
    }

    /// Post this frame's step. An error drops the rest of the input but its
    /// releases, so a failed drag or key press never leaves a button held.
    pub fn post_next(&mut self) -> Result<(), String> {
        let Some(step) = self.steps.pop_front() else {
            return Ok(());
        };
        let result = platform::post(step);
        if result.is_err() {
            self.steps.retain(|step| {
                matches!(
                    step,
                    Step::Up { .. } | Step::Key { down: false, .. } | Step::Ctrl(false)
                )
            });
        }
        result
    }
}

/// A logical point as a point in a content view of `width`×`height` points,
/// with AppKit's origin at the bottom left. The game letterboxes its 640×480
/// canvas, uniformly scaled and centred, as `TacticalCanvas` does.
#[cfg(target_os = "macos")]
fn view_point(x: f32, y: f32, width: f32, height: f32) -> (f64, f64) {
    let scale = (width / LOGICAL.0).min(height / LOGICAL.1);
    let left = (width - LOGICAL.0 * scale) * 0.5;
    let top = (height - LOGICAL.1 * scale) * 0.5;
    let from_top = top + (y + 0.5) * scale;
    (
        f64::from(left + (x + 0.5) * scale),
        f64::from(height - from_top),
    )
}

/// macOS virtual key codes (`kVK_*`, HIToolbox `Events.h`) by key name.
fn key_code(name: &str) -> Option<u16> {
    const LETTERS: [u16; 26] = [
        0, 11, 8, 2, 14, 3, 5, 4, 34, 38, 40, 37, 46, 45, 31, 35, 12, 15, 1, 17, 32, 9, 13, 7, 16,
        6,
    ];
    const DIGITS: [u16; 10] = [29, 18, 19, 20, 21, 23, 22, 26, 28, 25];
    const F_KEYS: [u16; 12] = [122, 120, 99, 118, 96, 97, 98, 100, 101, 109, 103, 111];
    let name = name.to_ascii_lowercase();
    let mut chars = name.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) {
        if c.is_ascii_lowercase() {
            return Some(LETTERS[usize::from(c as u8 - b'a')]);
        }
        if c.is_ascii_digit() {
            return Some(DIGITS[usize::from(c as u8 - b'0')]);
        }
    }
    if let Some(n) = name.strip_prefix('f').and_then(|n| n.parse::<usize>().ok()) {
        return F_KEYS.get(n.checked_sub(1)?).copied();
    }
    Some(match name.as_str() {
        "return" | "enter" => 36,
        "tab" => 48,
        "space" => 49,
        "delete" | "backspace" => 51,
        "escape" | "esc" => 53,
        "left" => 123,
        "right" => 124,
        "down" => 125,
        "up" => 126,
        "home" => 115,
        "end" => 119,
        "pageup" => 116,
        "pagedown" => 121,
        _ => return None,
    })
}

#[cfg(target_os = "macos")]
mod platform {
    use super::{view_point, Step};
    use macroquad::miniquad::native::apple::frameworks::{
        class, msg_send, nil, sel, sel_impl, NSPoint, NSRect, ObjcId, Sel, NO,
    };

    // NSEventType values.
    const LEFT_DOWN: u64 = 1;
    const LEFT_UP: u64 = 2;
    const RIGHT_DOWN: u64 = 3;
    const RIGHT_UP: u64 = 4;
    const MOVED: u64 = 5;
    const LEFT_DRAGGED: u64 = 6;
    const KEY_DOWN: u64 = 10;
    const KEY_UP: u64 = 11;
    const FLAGS_CHANGED: u64 = 12;
    /// `NSEventModifierFlagControl` with the left-Control device bit, the
    /// mask miniquad's `flagsChanged:` reads as left Control.
    const LEFT_CONTROL: u64 = 0x04_0001;
    /// `kVK_Control`.
    const CONTROL_KEY: u16 = 59;
    /// `kCGScrollEventUnitLine`: miniquad scales a line by 10, as a mouse
    /// wheel's.
    const SCROLL_LINE: u32 = 1;

    // AppKit has no scroll-wheel constructor; a CoreGraphics event wrapped
    // by `+[NSEvent eventWithCGEvent:]` is one. Both frameworks come with
    // AppKit.
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGEventCreateScrollWheelEvent(
            source: *const std::ffi::c_void,
            units: u32,
            wheel_count: u32,
            wheel1: i32,
            ...
        ) -> *mut std::ffi::c_void;
    }
    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFRelease(object: *const std::ffi::c_void);
    }

    /// The game window and its content view (miniquad's view): the
    /// application's largest window (it also owns a small menu-bar strip).
    unsafe fn game_window(app: ObjcId) -> Option<(ObjcId, ObjcId, NSRect)> {
        let windows: ObjcId = msg_send![app, windows];
        let count: usize = msg_send![windows, count];
        (0..count)
            .map(|i| {
                let window: ObjcId = msg_send![windows, objectAtIndex: i];
                let view: ObjcId = msg_send![window, contentView];
                let frame: NSRect = msg_send![view, frame];
                (window, view, frame)
            })
            .max_by(|a, b| {
                let area = |r: &NSRect| r.size.width * r.size.height;
                area(&a.2).total_cmp(&area(&b.2))
            })
            .filter(|(_, view, frame)| !view.is_null() && frame.size.width > 100.0)
    }

    pub(super) fn post(step: Step) -> Result<(), String> {
        // SAFETY: called on the main thread inside the game loop, which
        // AppKit runs; every receiver is an AppKit object checked non-nil.
        unsafe {
            let app: ObjcId = msg_send![class!(NSApplication), sharedApplication];
            let (window, view, frame) = game_window(app).ok_or("no game window")?;
            let number: isize = msg_send![window, windowNumber];
            let info: ObjcId = msg_send![class!(NSProcessInfo), processInfo];
            let time: f64 = msg_send![info, systemUptime];
            let (w, h) = (frame.size.width as f32, frame.size.height as f32);
            let at = |x: f32, y: f32| {
                let (x, y) = view_point(x, y, w, h);
                NSPoint { x, y }
            };
            let mouse = |kind: u64, point: NSPoint| -> ObjcId {
                msg_send![class!(NSEvent),
                    mouseEventWithType: kind
                    location: point
                    modifierFlags: 0u64
                    timestamp: time
                    windowNumber: number
                    context: nil
                    eventNumber: 0isize
                    clickCount: 1isize
                    pressure: 1.0f32]
            };
            let (event, handler): (ObjcId, Sel) = match step {
                Step::Move(x, y) => (mouse(MOVED, at(x, y)), sel!(mouseMoved:)),
                Step::Drag(x, y) => (mouse(LEFT_DRAGGED, at(x, y)), sel!(mouseDragged:)),
                Step::Down { x, y, right: false } => (mouse(LEFT_DOWN, at(x, y)), sel!(mouseDown:)),
                Step::Down { x, y, right: true } => {
                    (mouse(RIGHT_DOWN, at(x, y)), sel!(rightMouseDown:))
                }
                Step::Up { x, y, right: false } => (mouse(LEFT_UP, at(x, y)), sel!(mouseUp:)),
                Step::Up { x, y, right: true } => (mouse(RIGHT_UP, at(x, y)), sel!(rightMouseUp:)),
                Step::Key { code, down } => {
                    let empty: ObjcId = msg_send![class!(NSString), string];
                    let event: ObjcId = msg_send![class!(NSEvent),
                        keyEventWithType: if down { KEY_DOWN } else { KEY_UP }
                        location: NSPoint { x: 0.0, y: 0.0 }
                        modifierFlags: 0u64
                        timestamp: time
                        windowNumber: number
                        context: nil
                        characters: empty
                        charactersIgnoringModifiers: empty
                        isARepeat: NO
                        keyCode: code];
                    (event, if down { sel!(keyDown:) } else { sel!(keyUp:) })
                }
                Step::Ctrl(down) => {
                    let empty: ObjcId = msg_send![class!(NSString), string];
                    let event: ObjcId = msg_send![class!(NSEvent),
                        keyEventWithType: FLAGS_CHANGED
                        location: NSPoint { x: 0.0, y: 0.0 }
                        modifierFlags: if down { LEFT_CONTROL } else { 0u64 }
                        timestamp: time
                        windowNumber: number
                        context: nil
                        characters: empty
                        charactersIgnoringModifiers: empty
                        isARepeat: NO
                        keyCode: CONTROL_KEY];
                    (event, sel!(flagsChanged:))
                }
                Step::Scroll(lines) => {
                    let cg = CGEventCreateScrollWheelEvent(std::ptr::null(), SCROLL_LINE, 1, lines);
                    if cg.is_null() {
                        return Err("CoreGraphics made no scroll event".into());
                    }
                    // The NSEvent retains the CoreGraphics event.
                    let event: ObjcId = msg_send![class!(NSEvent), eventWithCGEvent: cg];
                    CFRelease(cg);
                    (event, sel!(scrollWheel:))
                }
            };
            if event.is_null() {
                return Err("AppKit made no event".into());
            }
            // The run loop retains the event until the call runs.
            let () = msg_send![view,
                performSelector: handler
                withObject: event
                afterDelay: 0.0f64];
        }
        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
mod platform {
    use super::Step;

    pub(super) fn post(_step: Step) -> Result<(), String> {
        Err("scripted input is built for macOS only".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_held_drag_keeps_the_button_down_until_its_release() {
        let mut input = DevInput::default();
        input.hold_drag((10.0, 20.0), (50.0, 20.0));
        assert_eq!(input.steps.len(), 2 + usize::from(DRAG_FRAMES));
        let released = input.steps.iter().any(|s| matches!(s, Step::Up { .. }));
        assert!(!released);
        assert_eq!(input.steps.back(), Some(&Step::Drag(50.0, 20.0)));

        input.release(50.0, 20.0);
        assert_eq!(
            input.steps.back(),
            Some(&Step::Up {
                x: 50.0,
                y: 20.0,
                right: false
            })
        );
    }

    #[test]
    fn a_whole_drag_is_a_held_drag_and_its_release() {
        let mut whole = DevInput::default();
        whole.drag((10.0, 20.0), (50.0, 20.0), false);
        let mut split = DevInput::default();
        split.hold_drag((10.0, 20.0), (50.0, 20.0));
        split.release(50.0, 20.0);
        assert_eq!(whole.steps, split.steps);
    }

    #[test]
    fn a_ctrl_drag_holds_control_around_the_whole_drag() {
        let mut ctrl = DevInput::default();
        ctrl.drag((10.0, 20.0), (50.0, 20.0), true);
        let mut plain = DevInput::default();
        plain.drag((10.0, 20.0), (50.0, 20.0), false);
        let mut expected = std::collections::VecDeque::from([Step::Ctrl(true)]);
        expected.extend(plain.steps);
        expected.push_back(Step::Ctrl(false));
        assert_eq!(ctrl.steps, expected);
    }
}
