//! One frozen route and one claim per normalized physical input dispatch.
//!
//! Port: SW-05I reproduces child handling and asynchronous-close boundaries from
//! FUN_00413560/FUN_00606940/FUN_00606960 without a native message queue. Semantic
//! capture, focus and stacking remain entirely in the kernel snapshot.

use crate::windowing::{EguiWindowSurfaces, SurfaceError, SurfaceInput, SurfacePass, SurfacePhase};
use egui_macroquad::egui;
use rebellion_windowing::{
    BubblePolicy, FocusTarget, RouteTarget, RoutingSnapshot, WindowEvent, WindowId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputEventId {
    pub frame: u64,
    pub ordinal: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDisposition {
    Handled,
    Unhandled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputRecipient {
    Control(FocusTarget),
    Window(WindowId),
    Root,
    Blocked,
}

impl InputRecipient {
    fn window(self) -> Option<WindowId> {
        match self {
            Self::Control(target) => Some(target.window),
            Self::Window(window) => Some(window),
            Self::Root | Self::Blocked => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedEvent {
    pub id: InputEventId,
    pub event: egui::Event,
}

/// Canonical egui order plus unmatched macroquad events. Backend copies map to
/// canonical tokens by matching occurrence, preserving equal independent events.
/// Macroquad projections use the existing locked egui event vocabulary; callers
/// must not synthesize text from keys or supply a second independently ordered
/// authoritative event stream. Polling pointer coordinates never replace egui's.
#[derive(Debug, Clone)]
pub struct NormalizedBatch {
    pub events: Vec<NormalizedEvent>,
    pub egui: Vec<InputEventId>,
    pub macroquad: Vec<InputEventId>,
    /// Clear these feature payloads before dispatch, including root gestures.
    /// Includes scene/modal/hold changes and focus loss in this batch.
    pub cancelled_recipients: Vec<InputRecipient>,
    /// Window projection of `cancelled_recipients`, for window payload cleanup.
    pub cancelled_windows: Vec<WindowId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputError {
    StaleFrame,
    TooManyEvents,
    InvalidPosition,
    UnknownEvent,
    WrongRecipient,
    NotKeyboard,
    AlreadyDelivered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeError {
    Input(InputError),
    Surface(SurfaceError),
}

#[derive(Debug, Clone)]
struct Entry {
    event: egui::Event,
    delivery: SurfaceInput,
    recipient: InputRecipient,
    initial: InputRecipient,
    bubble: BubblePolicy,
    claimed: bool,
    egui_delivered: bool,
}

#[derive(Debug, Clone, Copy)]
struct PressLease {
    // None retains quarantine until physical button-up, without a gesture payload.
    owner: Option<InputRecipient>,
}

#[derive(Default, Debug)]
pub struct FrameInputRouter {
    frame: Option<u64>,
    snapshot: Option<RoutingSnapshot>,
    entries: Vec<Entry>,
    presses: [Option<PressLease>; egui::NUM_POINTER_BUTTONS],
    cursor: Option<egui::Pos2>,
    held: bool,
}

impl FrameInputRouter {
    /// Normalize once before dispatch, using current geometry and one immutable
    /// snapshot. Frame numbers must increase even across campaign resets.
    /// A backend mirror is related to one occurrence, never every equal event.
    #[allow(clippy::too_many_arguments)] // One collection boundary with both backend projections.
    pub fn begin_frame(
        &mut self,
        frame: u64,
        snapshot: &RoutingSnapshot,
        surfaces: &EguiWindowSurfaces,
        egui_events: &[egui::Event],
        macroquad_events: &[egui::Event],
        held: bool,
    ) -> Result<NormalizedBatch, InputError> {
        if self.frame.is_some_and(|old| frame <= old) {
            return Err(InputError::StaleFrame);
        }
        let mut events = egui_events.to_vec();
        let mut matched = vec![false; egui_events.len()];
        let mut mirrors = Vec::new();
        for event in macroquad_events {
            if let Some(index) = egui_events
                .iter()
                .enumerate()
                .position(|(i, original)| !matched[i] && copies_match(original, event))
            {
                matched[index] = true;
                mirrors.push(index);
            } else {
                mirrors.push(events.len());
                events.push(event.clone());
            }
        }
        validate_event_count(events.len())?;
        if events
            .iter()
            .any(|event| event_position(event).is_some_and(|point| !point.is_finite()))
        {
            return Err(InputError::InvalidPosition);
        }
        let token = |ordinal: usize| InputEventId {
            frame,
            ordinal: ordinal as u32,
        };
        let mut cancelled_recipients = Vec::new();
        for lease in self.presses.iter_mut().flatten() {
            if let Some(owner) = lease.owner {
                let eligible = match owner {
                    InputRecipient::Window(window) => snapshot.allows_input(window),
                    InputRecipient::Root => snapshot.pointer_allows_root(),
                    _ => false,
                };
                if held || !eligible {
                    if !cancelled_recipients.contains(&owner) {
                        cancelled_recipients.push(owner);
                    }
                    lease.owner = None;
                }
            }
        }
        self.frame = Some(frame);
        self.snapshot = Some(snapshot.clone());
        self.entries.clear();
        self.held = held;
        for event in &events {
            if let Some(point) = event_position(event) {
                self.cursor = Some(point);
            }
            if matches!(event, egui::Event::PointerGone) {
                self.cursor = None;
            }
            let delivery = event_delivery(event, self.cursor);
            let mut recipient = match delivery {
                SurfaceInput::Keyboard => match snapshot.keyboard_route().target {
                    RouteTarget::Window(target) if target.control.is_some() => {
                        InputRecipient::Control(target)
                    }
                    other => recipient_for(other),
                },
                SurfaceInput::Pointer(None) if snapshot.capture_owner().is_none() => {
                    InputRecipient::Blocked
                }
                SurfaceInput::Pointer(point) => recipient_for(
                    snapshot
                        .pointer_route(
                            &point.map_or_else(Default::default, |p| surfaces.hit_windows(p)),
                        )
                        .target,
                ),
                SurfaceInput::Paint => InputRecipient::Blocked,
            };
            if self.held {
                recipient = InputRecipient::Blocked;
            }
            match event {
                egui::Event::PointerButton {
                    button,
                    pressed: true,
                    ..
                } => {
                    let lease = &mut self.presses[*button as usize];
                    if lease.is_some() {
                        recipient = InputRecipient::Blocked;
                    } else {
                        *lease = Some(PressLease {
                            owner: (recipient != InputRecipient::Blocked).then_some(recipient),
                        });
                    }
                }
                egui::Event::PointerButton {
                    button,
                    pressed: false,
                    ..
                } => {
                    let owner = self.presses[*button as usize]
                        .take()
                        .and_then(|lease| lease.owner);
                    recipient = owner
                        .filter(|owner| {
                            !self.held
                                && match owner {
                                    InputRecipient::Window(window) => {
                                        snapshot.allows_input(*window)
                                            && snapshot
                                                .capture_owner()
                                                .is_none_or(|capture| capture == *window)
                                    }
                                    InputRecipient::Root => {
                                        recipient == InputRecipient::Root
                                            && snapshot.pointer_allows_root()
                                    }
                                    _ => false,
                                }
                        })
                        .unwrap_or(InputRecipient::Blocked);
                }
                egui::Event::PointerMoved(_) | egui::Event::MouseMoved(_) => {
                    if self
                        .presses
                        .iter()
                        .flatten()
                        .any(|lease| lease.owner.is_none())
                    {
                        recipient = InputRecipient::Blocked;
                    }
                }
                egui::Event::WindowFocused(false) => {
                    self.cursor = None;
                    for owner in self.restrict_hold(true) {
                        if !cancelled_recipients.contains(&owner) {
                            cancelled_recipients.push(owner);
                        }
                    }
                    recipient = InputRecipient::Blocked;
                }
                _ => {}
            }
            self.entries.push(Entry {
                event: event.clone(),
                delivery,
                recipient,
                initial: recipient,
                bubble: snapshot.keyboard_route().bubble,
                claimed: false,
                egui_delivered: false,
            });
        }
        Ok(NormalizedBatch {
            events: events
                .into_iter()
                .enumerate()
                .map(|(ordinal, event)| NormalizedEvent {
                    id: token(ordinal),
                    event,
                })
                .collect(),
            egui: (0..egui_events.len()).map(token).collect(),
            macroquad: mirrors.into_iter().map(token).collect(),
            cancelled_windows: cancelled_recipients
                .iter()
                .filter_map(|owner| owner.window())
                .collect(),
            cancelled_recipients,
        })
    }

    pub fn recipient(&self, event: InputEventId) -> InputRecipient {
        self.entry(event)
            .filter(|entry| !entry.claimed && !self.held)
            .map_or(InputRecipient::Blocked, |entry| entry.recipient)
    }

    pub fn root_may_handle(&self, event: InputEventId) -> bool {
        self.recipient(event) == InputRecipient::Root
    }

    /// Position frozen for this pointer dispatch, including the wheel's position
    /// before later motion. Root consumers never need the backend's final cursor.
    pub fn pointer_position(&self, event: InputEventId) -> Option<egui::Pos2> {
        match self.entry(event)?.delivery {
            SurfaceInput::Pointer(position) => position,
            SurfaceInput::Keyboard | SurfaceInput::Paint => None,
        }
    }

    /// Claim a permitted effect once, irrespective of which backend copy asked.
    pub fn claim(&mut self, event: InputEventId, recipient: InputRecipient) -> bool {
        if recipient == InputRecipient::Blocked || self.recipient(event) != recipient {
            return false;
        }
        self.entry_mut(event).expect("validated token").claimed = true;
        true
    }

    /// Repeats/releases may reach controls, but cannot retrigger one-shot commands.
    pub fn claim_trigger(&mut self, event: InputEventId, recipient: InputRecipient) -> bool {
        let trigger = self.entry(event).is_some_and(|entry| {
            matches!(
                entry.event,
                egui::Event::Key {
                    pressed: true,
                    repeat: false,
                    ..
                } | egui::Event::PointerButton { pressed: true, .. }
            )
        });
        trigger && self.claim(event, recipient)
    }

    /// Actual child -> top-level -> root handling. Wrong, repeated or stale reports
    /// cannot advance a chain. A modal/menu boundary stops every keyboard class.
    pub fn report_key(
        &mut self,
        event: InputEventId,
        owner: InputRecipient,
        disposition: InputDisposition,
    ) -> Result<(), InputError> {
        if self.entry(event).is_none() {
            return Err(InputError::UnknownEvent);
        }
        if self
            .entry(event)
            .is_none_or(|entry| entry.delivery != SurfaceInput::Keyboard)
        {
            return Err(InputError::NotKeyboard);
        }
        if owner == InputRecipient::Blocked || self.recipient(event) != owner {
            return Err(InputError::WrongRecipient);
        }
        let entry = self.entry_mut(event).expect("validated token");
        match (owner, disposition) {
            (_, InputDisposition::Handled) => entry.claimed = true,
            (InputRecipient::Control(target), InputDisposition::Unhandled) => {
                entry.recipient = InputRecipient::Window(target.window)
            }
            (InputRecipient::Window(_), InputDisposition::Unhandled)
                if entry.bubble == BubblePolicy::ToRoot =>
            {
                entry.recipient = InputRecipient::Root
            }
            _ => entry.claimed = true,
        }
        Ok(())
    }

    /// Locked-egui hook: consume_key/consume_shortcut removes the actual event.
    /// Text/IME and response-owned edits require the caller's actual disposition;
    /// key class and wants_keyboard_input never grant root permission by themselves.
    pub fn report_egui_key(
        &mut self,
        event: InputEventId,
        owner: InputRecipient,
        ctx: &egui::Context,
        actual: InputDisposition,
    ) -> Result<(), InputError> {
        let original = &self.entry(event).ok_or(InputError::UnknownEvent)?.event;
        let consumed = matches!(original, egui::Event::Key { .. })
            && ctx.input(|input| !input.events.contains(original));
        self.report_key(
            event,
            owner,
            if consumed {
                InputDisposition::Handled
            } else {
                actual
            },
        )
    }

    /// Deliver exactly one canonical event through the accepted stable-host
    /// bridge. Geometry callbacks use temporary backing values; interactive
    /// callbacks return actual handling, including TextEdit response changes.
    /// RawInput events are replaced by this token's event. A cancelled/claimed
    /// release still updates egui's physical state with every widget disabled.
    /// Pointer recipients are intersected with SW05D's permit, so an old press
    /// cannot enable a newly exposed control. Drags outside bounds require the
    /// kernel's semantic capture; the router never fabricates capture to reroute.
    #[allow(clippy::too_many_arguments)] // One event and its proven backend realization boundary.
    pub fn deliver_egui(
        &mut self,
        event: InputEventId,
        ctx: &egui::Context,
        surfaces: &mut EguiWindowSurfaces,
        canvas: egui::Rect,
        mut raw: egui::RawInput,
        mut draw: impl FnMut(
            &mut egui::Ui,
            WindowId,
            SurfacePass,
            &mut EguiWindowSurfaces,
        ) -> InputDisposition,
    ) -> Result<egui::FullOutput, BridgeError> {
        let entry = self
            .entry(event)
            .ok_or(BridgeError::Input(InputError::UnknownEvent))?;
        if entry.egui_delivered {
            return Err(BridgeError::Input(InputError::AlreadyDelivered));
        }
        let recipient = self.recipient(event);
        if entry.delivery == SurfaceInput::Keyboard
            && recipient != InputRecipient::Blocked
            && recipient != entry.initial
        {
            return Err(BridgeError::Input(InputError::WrongRecipient));
        }
        let delivery = entry.delivery;
        raw.events = vec![entry.event.clone()];
        let snapshot = self
            .snapshot
            .as_ref()
            .ok_or(BridgeError::Input(InputError::UnknownEvent))?;
        let mut handled = false;
        let output = surfaces
            .run(
                ctx,
                snapshot,
                canvas,
                delivery,
                raw,
                |ui, window, mut pass, surfaces| {
                    let owns = recipient.window() == Some(window);
                    pass.permit.pointer &= owns;
                    pass.permit.keyboard &= owns;
                    if !pass.permit.pointer && !pass.permit.keyboard {
                        ui.disable();
                    }
                    let result = draw(ui, window, pass, surfaces);
                    if pass.phase == SurfacePhase::Interactive
                        && pass.permit.keyboard
                        && result == InputDisposition::Handled
                    {
                        handled = true;
                    }
                },
            )
            .map_err(BridgeError::Surface)?;
        self.entry_mut(event)
            .expect("validated token")
            .egui_delivered = true;
        if recipient != InputRecipient::Blocked {
            if delivery == SurfaceInput::Keyboard && recipient != InputRecipient::Root {
                self.report_egui_key(
                    event,
                    recipient,
                    ctx,
                    if handled {
                        InputDisposition::Handled
                    } else {
                        InputDisposition::Unhandled
                    },
                )
                .map_err(BridgeError::Input)?;
            } else if recipient.window().is_some() {
                self.claim(event, recipient);
            }
        }
        Ok(output)
    }

    pub fn gesture_owner(&self, button: egui::PointerButton) -> Option<InputRecipient> {
        self.presses[button as usize].and_then(|lease| lease.owner)
    }

    /// Clear a cancelled window's press payload, retaining quarantine until up.
    /// Also suppress pending old-lease events; never reroute them to exposed UI.
    pub fn cancel_window(&mut self, window: WindowId) -> bool {
        let mut cancelled = false;
        for lease in self.presses.iter_mut().flatten() {
            if lease.owner.and_then(InputRecipient::window) == Some(window) {
                lease.owner = None;
                cancelled = true;
            }
        }
        for entry in &mut self.entries {
            if entry.initial.window() == Some(window) {
                entry.claimed = true;
            }
        }
        cancelled
    }

    /// Return exact cancelled payload owners for feature cleanup, once per batch.
    pub fn apply_receipts(&mut self, events: &[WindowEvent]) -> Vec<WindowId> {
        let mut cancelled = Vec::new();
        for event in events {
            if let WindowEvent::CaptureCancelled { window, .. }
            | WindowEvent::Closed { window, .. } = event
            {
                self.cancel_window(*window);
                if !cancelled.contains(window) {
                    cancelled.push(*window);
                }
            }
        }
        cancelled
    }

    /// Restriction is monotonic for this frame; release waits for the next frame.
    /// Clear returned payload owners before any further dispatch. Pending button
    /// events count too: normalization may already have observed their release
    /// while an earlier handler still owns a feature payload.
    pub fn restrict_hold(&mut self, held: bool) -> Vec<InputRecipient> {
        let mut cancelled = Vec::new();
        if !held || self.held {
            return cancelled;
        }
        self.held = true;
        for lease in self.presses.iter_mut().flatten() {
            if let Some(owner) = lease.owner.take() {
                if !cancelled.contains(&owner) {
                    cancelled.push(owner);
                }
            }
        }
        for entry in &mut self.entries {
            if matches!(entry.event, egui::Event::PointerButton { .. })
                && entry.initial != InputRecipient::Blocked
                && !cancelled.contains(&entry.initial)
            {
                cancelled.push(entry.initial);
            }
            entry.claimed = true;
        }
        cancelled
    }

    /// Clear returned feature payloads before entering the new session.
    pub fn reset_session(&mut self) -> Vec<InputRecipient> {
        let cancelled = self.restrict_hold(true);
        self.entries.clear();
        self.snapshot = None;
        self.cursor = None;
        // Keep frame progression and quarantined physical buttons across resets.
        cancelled
    }

    fn entry(&self, event: InputEventId) -> Option<&Entry> {
        (self.frame == Some(event.frame))
            .then(|| self.entries.get(event.ordinal as usize))
            .flatten()
    }
    fn entry_mut(&mut self, event: InputEventId) -> Option<&mut Entry> {
        (self.frame == Some(event.frame))
            .then(|| self.entries.get_mut(event.ordinal as usize))
            .flatten()
    }
}

// Separate the allocation-free ordinal boundary from collecting backend events.
fn validate_event_count(count: usize) -> Result<(), InputError> {
    u32::try_from(count)
        .map(|_| ())
        .map_err(|_| InputError::TooManyEvents)
}

fn recipient_for(target: RouteTarget) -> InputRecipient {
    match target {
        RouteTarget::Window(target) => InputRecipient::Window(target.window),
        RouteTarget::StrategicRoot => InputRecipient::Root,
        RouteTarget::BlockedBy(_) => InputRecipient::Blocked,
    }
}

fn event_position(event: &egui::Event) -> Option<egui::Pos2> {
    match event {
        egui::Event::PointerMoved(pos)
        | egui::Event::PointerButton { pos, .. }
        | egui::Event::Touch { pos, .. } => Some(*pos),
        _ => None,
    }
}

fn event_delivery(event: &egui::Event, cursor: Option<egui::Pos2>) -> SurfaceInput {
    match event {
        egui::Event::Copy
        | egui::Event::Cut
        | egui::Event::Paste(_)
        | egui::Event::Text(_)
        | egui::Event::Key { .. }
        | egui::Event::Ime(_) => SurfaceInput::Keyboard,
        egui::Event::WindowFocused(_) | egui::Event::Screenshot { .. } => SurfaceInput::Paint,
        _ => SurfaceInput::Pointer(cursor),
    }
}

fn copies_match(original: &egui::Event, mirror: &egui::Event) -> bool {
    match (original, mirror) {
        (
            egui::Event::PointerButton {
                button: a,
                pressed: ap,
                ..
            },
            egui::Event::PointerButton {
                button: b,
                pressed: bp,
                ..
            },
        ) => a == b && ap == bp,
        (
            egui::Event::Key {
                key: a,
                pressed: ap,
                ..
            },
            egui::Event::Key {
                key: b,
                pressed: bp,
                ..
            },
        ) => a == b && ap == bp,
        _ => original == mirror,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::windowing_policy::{contextual_spec, singleton_spec};
    use rebellion_windowing::{ControlId, WindowCommand, WindowKind, WindowManager, WindowOwner};

    fn setup(modal: bool, child: bool) -> (WindowManager, WindowId, EguiWindowSurfaces) {
        let mut manager = WindowManager::new();
        let spec = if modal {
            singleton_spec(WindowKind::Encyclopedia, WindowOwner::StrategicRoot)
        } else {
            contextual_spec(
                WindowKind::Fleet,
                rebellion_windowing::SubjectId::new(2, 1),
                WindowOwner::StrategicRoot,
            )
        };
        manager.apply(WindowCommand::Open(spec)).unwrap();
        let id = manager.id_for(spec.key).unwrap();
        if child {
            manager
                .apply(WindowCommand::Focus(FocusTarget {
                    window: id,
                    control: Some(ControlId(1)),
                }))
                .unwrap();
        }
        let mut surfaces = EguiWindowSurfaces::default();
        surfaces.register_geometry(id, rect(0., 100.), rect(0., 100.));
        surfaces.register_hit_rect(id, rect(0., 100.));
        (manager, id, surfaces)
    }
    fn rect(left: f32, right: f32) -> egui::Rect {
        egui::Rect::from_min_max(egui::pos2(left, 0.), egui::pos2(right, 100.))
    }
    fn key(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }
    fn button(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }
    fn frame(
        router: &mut FrameInputRouter,
        number: u64,
        manager: &WindowManager,
        surfaces: &EguiWindowSurfaces,
        events: &[egui::Event],
    ) -> NormalizedBatch {
        router
            .begin_frame(number, &manager.begin_frame(), surfaces, events, &[], false)
            .unwrap()
    }

    #[test]
    fn windowing_input_handled_child_key_never_reaches_root() {
        let (manager, id, surfaces) = setup(false, true);
        let mut router = FrameInputRouter::default();
        let batch = frame(&mut router, 1, &manager, &surfaces, &[key(egui::Key::F7)]);
        let event = batch.egui[0];
        let child = InputRecipient::Control(FocusTarget {
            window: id,
            control: Some(ControlId(1)),
        });
        assert_eq!(router.recipient(event), child);
        assert!(!router.claim(event, InputRecipient::Root));
        router
            .report_key(event, child, InputDisposition::Handled)
            .unwrap();
        assert!(!router.root_may_handle(event));
        assert!(!router.claim(event, InputRecipient::Root));
        assert!(router
            .report_key(
                event,
                InputRecipient::Window(id),
                InputDisposition::Unhandled
            )
            .is_err());
    }

    #[test]
    fn windowing_input_unhandled_modeless_key_can_reach_root_once() {
        let (manager, id, surfaces) = setup(false, true);
        let mut router = FrameInputRouter::default();
        let batch = frame(&mut router, 1, &manager, &surfaces, &[key(egui::Key::F3)]);
        let event = batch.egui[0];
        let child = router.recipient(event);
        router
            .report_key(event, child, InputDisposition::Unhandled)
            .unwrap();
        assert_eq!(router.recipient(event), InputRecipient::Window(id));
        assert!(!router.root_may_handle(event));
        router
            .report_key(
                event,
                InputRecipient::Window(id),
                InputDisposition::Unhandled,
            )
            .unwrap();
        assert!(router.root_may_handle(event));
        assert!(router.claim(event, InputRecipient::Root));
        assert!(!router.claim(event, InputRecipient::Root));
    }

    #[test]
    fn windowing_input_modal_unhandled_key_stops_at_window() {
        let (manager, id, surfaces) = setup(true, false);
        let mut router = FrameInputRouter::default();
        let mut alt = key(egui::Key::P);
        if let egui::Event::Key { modifiers, .. } = &mut alt {
            modifiers.alt = true;
        }
        let events = [
            key(egui::Key::F2),
            key(egui::Key::Escape),
            alt,
            egui::Event::Text("x".into()),
        ];
        let batch = frame(&mut router, 1, &manager, &surfaces, &events);
        for event in batch.egui {
            router
                .report_key(
                    event,
                    InputRecipient::Window(id),
                    InputDisposition::Unhandled,
                )
                .unwrap();
            assert!(!router.root_may_handle(event));
        }
    }

    #[test]
    fn windowing_input_macroquad_and_egui_copies_share_one_claim() {
        let (manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let canonical = button(egui::pos2(10., 10.), true);
        let mirror = button(egui::pos2(210., 10.), true);
        let batch = router
            .begin_frame(
                1,
                &manager.begin_frame(),
                &surfaces,
                &[canonical],
                &[mirror],
                false,
            )
            .unwrap();
        assert_eq!(batch.egui, batch.macroquad);
        assert_eq!(batch.events.len(), 1);
        assert!(router.claim(batch.macroquad[0], InputRecipient::Window(id)));
        assert!(!router.claim(batch.egui[0], InputRecipient::Window(id)));
    }

    #[test]
    fn windowing_input_event_coordinates_not_final_cursor_choose_target() {
        let (mut manager, first, mut surfaces) = setup(false, false);
        let spec = contextual_spec(
            WindowKind::Fleet,
            rebellion_windowing::SubjectId::new(2, 2),
            WindowOwner::StrategicRoot,
        );
        manager.apply(WindowCommand::Open(spec)).unwrap();
        let second = manager.id_for(spec.key).unwrap();
        surfaces.register_geometry(second, rect(200., 300.), rect(200., 300.));
        surfaces.register_hit_rect(second, rect(200., 300.));
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[
                button(egui::pos2(10., 10.), true),
                egui::Event::PointerMoved(egui::pos2(210., 10.)),
            ],
        );
        assert_eq!(
            router.recipient(batch.egui[0]),
            InputRecipient::Window(first)
        );
        assert_eq!(
            router.recipient(batch.egui[1]),
            InputRecipient::Window(second)
        );
    }

    #[test]
    fn windowing_input_wheel_has_one_recipient() {
        let (mut manager, old, mut surfaces) = setup(false, false);
        let spec = contextual_spec(
            WindowKind::Fleet,
            rebellion_windowing::SubjectId::new(2, 2),
            WindowOwner::StrategicRoot,
        );
        manager.apply(WindowCommand::Open(spec)).unwrap();
        let top = manager.id_for(spec.key).unwrap();
        surfaces.register_geometry(top, rect(0., 100.), rect(0., 100.));
        surfaces.register_hit_rect(top, rect(0., 100.));
        let wheel = egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0., 10.),
            modifiers: egui::Modifiers::NONE,
        };
        let mut router = FrameInputRouter::default();
        let batch = router
            .begin_frame(
                1,
                &manager.begin_frame(),
                &surfaces,
                &[
                    egui::Event::PointerMoved(egui::pos2(10., 10.)),
                    wheel.clone(),
                ],
                &[wheel],
                false,
            )
            .unwrap();
        let event = batch.egui[1];
        assert_eq!(batch.macroquad[0], event);
        assert!(!router.claim(event, InputRecipient::Window(old)));
        assert!(router.claim(event, InputRecipient::Window(top)));
        assert!(!router.claim(event, InputRecipient::Window(top)));
    }

    #[test]
    fn windowing_input_cancelled_drag_quarantines_later_release() {
        let (mut manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true)],
        );
        assert_eq!(
            router.gesture_owner(egui::PointerButton::Primary),
            Some(InputRecipient::Window(id))
        );
        manager.apply(WindowCommand::CapturePointer(id)).unwrap();
        let cancellation = manager
            .apply(WindowCommand::CancelCapture(
                rebellion_windowing::CaptureCancelReason::HoldStarted,
            ))
            .unwrap();
        assert_eq!(router.apply_receipts(&cancellation.events), [id]);
        assert_eq!(router.gesture_owner(egui::PointerButton::Primary), None);
        let batch = frame(
            &mut router,
            2,
            &manager,
            &surfaces,
            &[button(egui::pos2(210., 10.), false)],
        );
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
        assert!(!router.claim(batch.egui[0], InputRecipient::Root));
        let batch = frame(
            &mut router,
            3,
            &manager,
            &surfaces,
            &[button(egui::pos2(210., 10.), true)],
        );
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Root);
    }

    #[test]
    fn windowing_input_hold_release_does_not_unlock_existing_frame() {
        let (manager, _, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(210., 10.), true)],
        );
        router.restrict_hold(true);
        router.restrict_hold(false);
        assert!(!router.claim(batch.egui[0], InputRecipient::Root));
        let release = frame(
            &mut router,
            2,
            &manager,
            &surfaces,
            &[button(egui::pos2(210., 10.), false)],
        );
        assert_eq!(router.recipient(release.egui[0]), InputRecipient::Blocked);
        let fresh = frame(
            &mut router,
            3,
            &manager,
            &surfaces,
            &[button(egui::pos2(210., 10.), true)],
        );
        assert!(router.claim(fresh.egui[0], InputRecipient::Root));
    }

    #[test]
    fn windowing_input_identical_events_in_one_backend_remain_distinct_and_stale_tokens_fail_closed(
    ) {
        let (manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let batch = router
            .begin_frame(
                1,
                &manager.begin_frame(),
                &surfaces,
                &[key(egui::Key::F3), key(egui::Key::F3)],
                &[key(egui::Key::F3), key(egui::Key::F3)],
                false,
            )
            .unwrap();
        assert_eq!(batch.events.len(), 2);
        assert_ne!(batch.egui[0], batch.egui[1]);
        assert_eq!(batch.egui, batch.macroquad);
        let old = batch.egui[0];
        frame(&mut router, 2, &manager, &surfaces, &[]);
        assert!(!router.claim(old, InputRecipient::Window(id)));
        assert!(router
            .begin_frame(2, &manager.begin_frame(), &surfaces, &[], &[], false)
            .is_err());
        router.reset_session();
        assert!(router
            .begin_frame(1, &manager.begin_frame(), &surfaces, &[], &[], false)
            .is_err());
    }

    #[test]
    fn windowing_input_cancellation_reports_payload_cleanup_even_after_a_batched_release() {
        let (manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[
                button(egui::pos2(10., 10.), true),
                button(egui::pos2(10., 10.), false),
            ],
        );
        // Normalization has observed button-up, but the press's feature handler
        // can still start a payload before an intervening modal cancels it.
        let event = WindowEvent::CaptureCancelled {
            window: id,
            reason: rebellion_windowing::CaptureCancelReason::ModalOpened,
        };
        assert_eq!(router.apply_receipts(&[event, event]), [id]);
        assert_eq!(router.recipient(batch.egui[1]), InputRecipient::Blocked);
    }

    #[test]
    fn windowing_input_release_without_press_cannot_click_an_exposed_control() {
        let (manager, _, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), false)],
        );
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
    }

    #[test]
    fn windowing_input_hold_cleans_retained_window_and_root_payloads_after_an_empty_frame() {
        let (manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let mut secondary = button(egui::pos2(210., 10.), true);
        if let egui::Event::PointerButton { button, .. } = &mut secondary {
            *button = egui::PointerButton::Secondary;
        }
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true), secondary],
        );
        let mut payloads = vec![InputRecipient::Window(id), InputRecipient::Root];
        assert!(router.claim(batch.egui[0], payloads[0]));
        assert!(router.claim(batch.egui[1], payloads[1]));
        frame(&mut router, 2, &manager, &surfaces, &[]);
        let cancelled = router.restrict_hold(true);
        assert_eq!(cancelled, payloads);
        for owner in cancelled {
            payloads.retain(|payload| *payload != owner);
        }
        assert!(payloads.is_empty());
        assert!(router.restrict_hold(true).is_empty());
        assert_eq!(router.gesture_owner(egui::PointerButton::Primary), None);
        assert_eq!(router.gesture_owner(egui::PointerButton::Secondary), None);
    }

    #[test]
    fn windowing_input_metadata_never_claims_a_remembered_window_or_root_position() {
        let (manager, id, surfaces) = setup(false, false);
        for position in [egui::pos2(10., 10.), egui::pos2(210., 10.)] {
            let mut router = FrameInputRouter::default();
            let batch = frame(
                &mut router,
                1,
                &manager,
                &surfaces,
                &[
                    egui::Event::PointerMoved(position),
                    egui::Event::WindowFocused(true),
                    egui::Event::Screenshot {
                        viewport_id: egui::ViewportId::ROOT,
                        user_data: Default::default(),
                        image: std::sync::Arc::new(egui::ColorImage::new(
                            [1, 1],
                            egui::Color32::BLACK,
                        )),
                    },
                ],
            );
            for event in &batch.egui[1..] {
                assert_eq!(router.recipient(*event), InputRecipient::Blocked);
                assert!(!router.claim(*event, InputRecipient::Window(id)));
                assert!(!router.claim(*event, InputRecipient::Root));
            }
        }
    }

    #[test]
    fn windowing_input_event_count_rejects_ordinal_overflow_without_allocating_billions_of_events()
    {
        assert_eq!(validate_event_count(0), Ok(()));
        assert_eq!(validate_event_count(u32::MAX as usize), Ok(()));
        if let Some(overflow) = (u32::MAX as usize).checked_add(1) {
            assert_eq!(
                validate_event_count(overflow),
                Err(InputError::TooManyEvents)
            );
            assert_eq!(
                validate_event_count(usize::MAX),
                Err(InputError::TooManyEvents)
            );
        }
    }

    #[test]
    fn windowing_input_root_press_survives_a_frame_and_releases_only_over_root() {
        for release_point in [egui::pos2(210., 10.), egui::pos2(10., 10.)] {
            let (manager, _, surfaces) = setup(false, false);
            let mut router = FrameInputRouter::default();
            frame(
                &mut router,
                1,
                &manager,
                &surfaces,
                &[button(egui::pos2(210., 10.), true)],
            );
            let idle = frame(&mut router, 2, &manager, &surfaces, &[]);
            assert!(idle.cancelled_recipients.is_empty());
            assert_eq!(
                router.gesture_owner(egui::PointerButton::Primary),
                Some(InputRecipient::Root)
            );
            let batch = frame(
                &mut router,
                3,
                &manager,
                &surfaces,
                &[button(release_point, false)],
            );
            let expected = if release_point.x > 100. {
                InputRecipient::Root
            } else {
                InputRecipient::Blocked
            };
            assert_eq!(router.recipient(batch.egui[0]), expected);
        }
    }

    #[test]
    fn windowing_input_new_capture_cannot_steal_an_old_press_release() {
        let (mut manager, old, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true)],
        );
        let spec = contextual_spec(
            WindowKind::Fleet,
            rebellion_windowing::SubjectId::new(2, 2),
            WindowOwner::StrategicRoot,
        );
        manager.apply(WindowCommand::Open(spec)).unwrap();
        let capture = manager.id_for(spec.key).unwrap();
        assert_ne!(capture, old);
        manager
            .apply(WindowCommand::CapturePointer(capture))
            .unwrap();
        let batch = frame(
            &mut router,
            2,
            &manager,
            &surfaces,
            &[button(egui::pos2(210., 10.), false)],
        );
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
        assert!(!router.claim(batch.egui[0], InputRecipient::Window(capture)));
    }

    #[test]
    fn windowing_input_quarantined_motion_is_blocked_but_capture_can_route_without_a_position() {
        let (mut manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true)],
        );
        router.cancel_window(id);
        let batch = frame(
            &mut router,
            2,
            &manager,
            &surfaces,
            &[
                egui::Event::PointerMoved(egui::pos2(210., 10.)),
                egui::Event::MouseMoved(egui::vec2(2., 0.)),
            ],
        );
        for event in batch.egui {
            assert_eq!(router.recipient(event), InputRecipient::Blocked);
        }
        let mut router = FrameInputRouter::default();
        manager.apply(WindowCommand::CapturePointer(id)).unwrap();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0., 1.),
                modifiers: egui::Modifiers::NONE,
            }],
        );
        assert_eq!(router.pointer_position(batch.egui[0]), None);
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Window(id));
    }

    #[test]
    fn windowing_input_text_and_ime_need_actual_handling_instead_of_key_consumption() {
        let (manager, id, surfaces) = setup(false, true);
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[
                egui::Event::Text("x".into()),
                egui::Event::Ime(egui::ImeEvent::Commit("y".into())),
            ],
        );
        // These events are absent from egui's key list; that does not mean a
        // Text/IME control handled them. The callback reports its actual result.
        let ctx = egui::Context::default();
        for event in batch.egui {
            router
                .report_egui_key(
                    event,
                    router.recipient(event),
                    &ctx,
                    InputDisposition::Unhandled,
                )
                .unwrap();
            assert_eq!(router.recipient(event), InputRecipient::Window(id));
            router
                .report_key(
                    event,
                    InputRecipient::Window(id),
                    InputDisposition::Unhandled,
                )
                .unwrap();
            assert!(router.claim(event, InputRecipient::Root));
        }
    }

    #[test]
    fn windowing_input_egui_cannot_repeat_an_advanced_child_or_root_key_dispatch() {
        for advance_to_root in [false, true] {
            let (manager, id, mut surfaces) = setup(false, true);
            let mut router = FrameInputRouter::default();
            let batch = frame(&mut router, 1, &manager, &surfaces, &[key(egui::Key::F7)]);
            router
                .report_key(
                    batch.egui[0],
                    router.recipient(batch.egui[0]),
                    InputDisposition::Unhandled,
                )
                .unwrap();
            if advance_to_root {
                router
                    .report_key(
                        batch.egui[0],
                        InputRecipient::Window(id),
                        InputDisposition::Unhandled,
                    )
                    .unwrap();
            }
            let mut calls = 0;
            assert!(matches!(
                router.deliver_egui(
                    batch.egui[0],
                    &egui::Context::default(),
                    &mut surfaces,
                    rect(0., 300.),
                    Default::default(),
                    |_, _, _, _| {
                        calls += 1;
                        InputDisposition::Unhandled
                    }
                ),
                Err(BridgeError::Input(InputError::WrongRecipient))
            ));
            assert_eq!(calls, 0);
        }
    }

    #[test]
    fn windowing_input_egui_preview_handling_cannot_stop_an_unhandled_modeless_key() {
        let (manager, id, mut surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let batch = frame(&mut router, 1, &manager, &surfaces, &[key(egui::Key::F7)]);
        let mut interactive = 0;
        router
            .deliver_egui(
                batch.egui[0],
                &egui::Context::default(),
                &mut surfaces,
                rect(0., 300.),
                Default::default(),
                |_, window, pass, _| {
                    assert_eq!(window, id);
                    if pass.phase == SurfacePhase::Geometry {
                        InputDisposition::Handled
                    } else {
                        assert!(pass.permit.keyboard);
                        interactive += 1;
                        InputDisposition::Unhandled
                    }
                },
            )
            .unwrap();
        assert_eq!(interactive, 1);
        assert!(router.claim(batch.egui[0], InputRecipient::Root));
    }

    #[test]
    fn windowing_input_egui_disables_claimed_keyboard_and_quarantined_pointer_widgets() {
        for keyboard in [false, true] {
            let (manager, id, mut surfaces) = setup(false, false);
            let mut router = FrameInputRouter::default();
            let event = if keyboard {
                key(egui::Key::F7)
            } else {
                button(egui::pos2(10., 10.), false)
            };
            let batch = frame(&mut router, 1, &manager, &surfaces, &[event]);
            if keyboard {
                assert!(router.claim(batch.egui[0], InputRecipient::Window(id)));
            }
            let mut interactive = 0;
            router
                .deliver_egui(
                    batch.egui[0],
                    &egui::Context::default(),
                    &mut surfaces,
                    rect(0., 300.),
                    Default::default(),
                    |ui, _, pass, _| {
                        assert!(!pass.permit.keyboard);
                        assert!(!pass.permit.pointer);
                        assert!(!ui.is_enabled());
                        if pass.phase == SurfacePhase::Interactive {
                            interactive += 1;
                        }
                        InputDisposition::Handled
                    },
                )
                .unwrap();
            assert_eq!(interactive, 1);
            assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
        }
    }

    #[test]
    fn windowing_input_egui_delivery_consumes_window_pointer_but_preserves_root_input() {
        for event in [
            button(egui::pos2(10., 10.), true),
            button(egui::pos2(210., 10.), true),
        ] {
            let (manager, id, mut surfaces) = setup(false, false);
            let mut router = FrameInputRouter::default();
            let batch = frame(&mut router, 1, &manager, &surfaces, &[event]);
            let owner = router.recipient(batch.egui[0]);
            router
                .deliver_egui(
                    batch.egui[0],
                    &egui::Context::default(),
                    &mut surfaces,
                    rect(0., 300.),
                    Default::default(),
                    |_, _, _, _| InputDisposition::Unhandled,
                )
                .unwrap();
            if owner == InputRecipient::Root {
                assert!(router.claim(batch.egui[0], owner));
            } else {
                assert_eq!(owner, InputRecipient::Window(id));
                assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
            }
        }
        let manager = WindowManager::new();
        let mut surfaces = EguiWindowSurfaces::default();
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[key(egui::Key::F7), egui::Event::WindowFocused(true)],
        );
        router
            .deliver_egui(
                batch.egui[0],
                &egui::Context::default(),
                &mut surfaces,
                rect(0., 300.),
                Default::default(),
                |_, _, _, _| InputDisposition::Unhandled,
            )
            .unwrap();
        assert!(router.claim(batch.egui[0], InputRecipient::Root));
        assert_eq!(router.recipient(batch.egui[1]), InputRecipient::Blocked);
    }

    #[test]
    fn windowing_input_egui_consumed_function_key_stops_despite_unhandled_classification() {
        let (manager, id, surfaces) = setup(false, true);
        let mut router = FrameInputRouter::default();
        let event = key(egui::Key::F7);
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            std::slice::from_ref(&event),
        );
        let ctx = egui::Context::default();
        let _ = ctx.run(
            egui::RawInput {
                events: vec![event],
                ..Default::default()
            },
            |ctx| {
                assert!(ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F7)));
            },
        );
        router
            .report_egui_key(
                batch.egui[0],
                router.recipient(batch.egui[0]),
                &ctx,
                InputDisposition::Unhandled,
            )
            .unwrap();
        assert!(!router.root_may_handle(batch.egui[0]));
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
        assert_eq!(manager.focused().unwrap().window, id);
    }
    #[test]
    fn windowing_input_wheel_without_an_event_position_never_guesses_the_root() {
        let (manager, _, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let wheel = egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0., 1.),
            modifiers: egui::Modifiers::NONE,
        };
        let batch = frame(&mut router, 1, &manager, &surfaces, &[wheel]);
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
    }

    #[test]
    fn windowing_input_focus_loss_reports_payload_cleanup_before_dispatch() {
        let (manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[
                button(egui::pos2(10., 10.), true),
                egui::Event::WindowFocused(false),
            ],
        );
        assert_eq!(batch.cancelled_windows, [id]);
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
        assert_eq!(router.gesture_owner(egui::PointerButton::Primary), None);
    }

    #[test]
    fn windowing_input_hold_returns_window_and_root_payloads_once_even_with_a_batched_release() {
        let (manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let mut root_press = button(egui::pos2(210., 10.), true);
        if let egui::Event::PointerButton { button, .. } = &mut root_press {
            *button = egui::PointerButton::Secondary;
        }
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[
                button(egui::pos2(10., 10.), true),
                button(egui::pos2(10., 10.), false),
                root_press,
            ],
        );
        // Dispatch can start a payload even though normalization has already
        // observed up. A hold between handlers must clear that payload too.
        let mut payloads = vec![InputRecipient::Window(id), InputRecipient::Root];
        assert!(router.claim(batch.egui[0], InputRecipient::Window(id)));
        assert!(router.claim(batch.egui[2], InputRecipient::Root));
        let cancelled = router.restrict_hold(true);
        for owner in &cancelled {
            payloads.retain(|payload| payload != owner);
        }
        assert!(payloads.is_empty());
        assert_eq!(cancelled.len(), 2);
        assert!(router.restrict_hold(true).is_empty());
        assert!(router.restrict_hold(false).is_empty());
        assert_eq!(router.recipient(batch.egui[1]), InputRecipient::Blocked);
    }

    #[test]
    fn windowing_input_reset_returns_payload_cleanup_and_invalidates_pending_tokens_immediately() {
        let (manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true)],
        );
        let mut payload = Some(InputRecipient::Window(id));
        assert!(router.claim(batch.egui[0], payload.unwrap()));
        let cancelled = router.reset_session();
        if cancelled.contains(&payload.unwrap()) {
            payload = None;
        }
        assert_eq!(payload, None);
        assert_eq!(cancelled, [InputRecipient::Window(id)]);
        assert_eq!(router.gesture_owner(egui::PointerButton::Primary), None);
        assert_eq!(router.pointer_position(batch.egui[0]), None);
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
        assert!(router.reset_session().is_empty());
    }

    #[test]
    fn windowing_input_modal_cancels_and_reports_a_root_drag_payload() {
        let (mut manager, _, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(210., 10.), true)],
        );
        assert!(router.claim(batch.egui[0], InputRecipient::Root));
        manager
            .apply(WindowCommand::Open(singleton_spec(
                WindowKind::Encyclopedia,
                WindowOwner::StrategicRoot,
            )))
            .unwrap();
        let batch = frame(
            &mut router,
            2,
            &manager,
            &surfaces,
            &[button(egui::pos2(210., 10.), false)],
        );
        let mut root_drag = true;
        if batch.cancelled_recipients.contains(&InputRecipient::Root) {
            root_drag = false;
        }
        assert!(!root_drag);
        assert_eq!(batch.cancelled_recipients, [InputRecipient::Root]);
        assert!(batch.cancelled_windows.is_empty());
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
    }

    #[test]
    fn windowing_input_wheel_after_focus_loss_requires_a_fresh_position() {
        let (manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[
                egui::Event::PointerMoved(egui::pos2(10., 10.)),
                egui::Event::WindowFocused(false),
            ],
        );
        let wheel = egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0., 1.),
            modifiers: egui::Modifiers::NONE,
        };
        let batch = frame(
            &mut router,
            2,
            &manager,
            &surfaces,
            &[
                egui::Event::WindowFocused(true),
                wheel.clone(),
                egui::Event::PointerMoved(egui::pos2(10., 10.)),
                wheel,
            ],
        );
        assert_eq!(router.recipient(batch.egui[1]), InputRecipient::Blocked);
        assert_eq!(router.pointer_position(batch.egui[1]), None);
        assert_eq!(router.recipient(batch.egui[3]), InputRecipient::Window(id));
    }

    #[test]
    fn windowing_input_egui_bridge_mutates_focused_text_once_and_blocks_a_backend_copy() {
        let (manager, id, mut surfaces) = setup(false, true);
        let mut router = FrameInputRouter::default();
        let event = egui::Event::Text("+".into());
        let batch = router
            .begin_frame(
                1,
                &manager.begin_frame(),
                &surfaces,
                std::slice::from_ref(&event),
                std::slice::from_ref(&event),
                false,
            )
            .unwrap();
        let ctx = egui::Context::default();
        let mut text = "a".to_owned();
        let target = FocusTarget {
            window: id,
            control: Some(ControlId(1)),
        };
        let mut draw = |ui: &mut egui::Ui,
                        window: WindowId,
                        pass: crate::windowing::SurfacePass,
                        surfaces: &mut EguiWindowSurfaces| {
            assert_eq!(window, id);
            let mut preview = text.clone();
            let backing = if pass.phase == crate::windowing::SurfacePhase::Geometry {
                &mut preview
            } else {
                &mut text
            };
            let response = ui.add(
                egui::TextEdit::singleline(backing)
                    .id(EguiWindowSurfaces::widget_id(target))
                    .desired_width(80.),
            );
            surfaces.register_control(target, response.id);
            if response.changed() {
                InputDisposition::Handled
            } else {
                InputDisposition::Unhandled
            }
        };
        router
            .deliver_egui(
                batch.egui[0],
                &ctx,
                &mut surfaces,
                rect(0., 300.),
                egui::RawInput::default(),
                &mut draw,
            )
            .unwrap();
        assert!(!router.root_may_handle(batch.egui[0]));
        assert!(matches!(
            router.deliver_egui(
                batch.macroquad[0],
                &ctx,
                &mut surfaces,
                rect(0., 300.),
                egui::RawInput::default(),
                &mut draw
            ),
            Err(BridgeError::Input(InputError::AlreadyDelivered))
        ));
        assert_eq!(text.matches('+').count(), 1);
        assert_eq!(text.len(), 2);
    }

    #[test]
    fn windowing_input_egui_bridge_quarantines_raw_release_before_custom_input_mutates() {
        let (manager, id, mut surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true)],
        );
        router.cancel_window(id);
        let batch = frame(
            &mut router,
            2,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), false)],
        );
        let ctx = egui::Context::default();
        let mut drops = 0;
        router
            .deliver_egui(
                batch.egui[0],
                &ctx,
                &mut surfaces,
                rect(0., 300.),
                egui::RawInput::default(),
                |ui, _, pass, _| {
                    if pass.phase == crate::windowing::SurfacePhase::Interactive
                        && pass.permit.pointer
                        && ui.input(|input| input.pointer.any_released())
                    {
                        drops += 1;
                    }
                    InputDisposition::Unhandled
                },
            )
            .unwrap();
        assert_eq!(drops, 0);
        assert!(!router.root_may_handle(batch.egui[0]));
    }
    #[test]
    fn windowing_input_captured_motion_and_release_stay_with_the_press_owner_outside_geometry() {
        let (mut manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true)],
        );
        manager.apply(WindowCommand::CapturePointer(id)).unwrap();
        let batch = frame(
            &mut router,
            2,
            &manager,
            &surfaces,
            &[
                egui::Event::PointerMoved(egui::pos2(210., 10.)),
                button(egui::pos2(210., 10.), false),
            ],
        );
        for event in batch.egui {
            assert_eq!(router.recipient(event), InputRecipient::Window(id));
            assert!(router.claim(event, InputRecipient::Window(id)));
        }
        assert_eq!(router.gesture_owner(egui::PointerButton::Primary), None);
    }

    #[test]
    fn windowing_input_invalid_frame_or_position_preserves_the_previous_routes_and_press() {
        let (manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true)],
        );
        let bad = button(egui::pos2(f32::NAN, 0.), false);
        assert!(matches!(
            router.begin_frame(2, &manager.begin_frame(), &surfaces, &[bad], &[], false),
            Err(InputError::InvalidPosition)
        ));
        assert_eq!(
            router.gesture_owner(egui::PointerButton::Primary),
            Some(InputRecipient::Window(id))
        );
        assert!(router.claim(batch.egui[0], InputRecipient::Window(id)));
        assert!(matches!(
            router.begin_frame(1, &manager.begin_frame(), &surfaces, &[], &[], false),
            Err(InputError::StaleFrame)
        ));
        let fresh = frame(&mut router, 2, &manager, &surfaces, &[key(egui::Key::F3)]);
        assert!(!router.claim(batch.egui[0], InputRecipient::Window(id)));
        assert_eq!(router.recipient(fresh.egui[0]), InputRecipient::Window(id));
    }

    #[test]
    fn windowing_input_scene_change_quarantines_the_old_physical_press() {
        let (mut manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true)],
        );
        manager.set_active_windows(&Default::default()).unwrap();
        let batch = frame(
            &mut router,
            2,
            &manager,
            &surfaces,
            &[button(egui::pos2(210., 10.), false)],
        );
        assert_eq!(batch.cancelled_windows, [id]);
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
        assert_eq!(router.gesture_owner(egui::PointerButton::Primary), None);
    }

    #[test]
    fn windowing_input_reset_quarantines_a_release_in_the_new_campaign() {
        let (mut manager, id, mut surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let old = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true)],
        )
        .egui[0];
        router.reset_session();
        manager.reset_session();
        surfaces.begin_frame();
        let spec = contextual_spec(
            WindowKind::Fleet,
            rebellion_windowing::SubjectId::new(2, 1),
            WindowOwner::StrategicRoot,
        );
        manager.apply(WindowCommand::Open(spec)).unwrap();
        let fresh = manager.id_for(spec.key).unwrap();
        assert_ne!(fresh, id);
        surfaces.register_geometry(fresh, rect(0., 100.), rect(0., 100.));
        surfaces.register_hit_rect(fresh, rect(0., 100.));
        let batch = frame(
            &mut router,
            2,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), false)],
        );
        assert!(!router.claim(old, InputRecipient::Window(fresh)));
        assert_eq!(router.recipient(batch.egui[0]), InputRecipient::Blocked);
        let batch = frame(
            &mut router,
            3,
            &manager,
            &surfaces,
            &[button(egui::pos2(10., 10.), true)],
        );
        assert!(router.claim(batch.egui[0], InputRecipient::Window(fresh)));
    }

    #[test]
    fn windowing_input_one_shot_claim_rejects_key_repeat_and_release() {
        let (manager, id, surfaces) = setup(false, false);
        let mut repeat = key(egui::Key::Enter);
        if let egui::Event::Key { repeat, .. } = &mut repeat {
            *repeat = true;
        }
        let mut release = key(egui::Key::Enter);
        if let egui::Event::Key { pressed, .. } = &mut release {
            *pressed = false;
        }
        let mut router = FrameInputRouter::default();
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[repeat, release, key(egui::Key::Enter)],
        );
        assert!(!router.claim_trigger(batch.egui[0], InputRecipient::Window(id)));
        assert!(!router.claim_trigger(batch.egui[1], InputRecipient::Window(id)));
        assert!(router.claim_trigger(batch.egui[2], InputRecipient::Window(id)));
        assert!(!router.claim_trigger(batch.egui[2], InputRecipient::Window(id)));
    }

    #[test]
    fn windowing_input_button_leases_are_independent_and_a_second_down_cannot_transfer_them() {
        let (manager, id, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let mut secondary = button(egui::pos2(210., 10.), true);
        if let egui::Event::PointerButton { button, .. } = &mut secondary {
            *button = egui::PointerButton::Secondary;
        }
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[
                button(egui::pos2(10., 10.), true),
                secondary,
                button(egui::pos2(210., 10.), true),
            ],
        );
        assert_eq!(router.recipient(batch.egui[2]), InputRecipient::Blocked);
        assert_eq!(
            router.gesture_owner(egui::PointerButton::Primary),
            Some(InputRecipient::Window(id))
        );
        assert_eq!(
            router.gesture_owner(egui::PointerButton::Secondary),
            Some(InputRecipient::Root)
        );
        router.cancel_window(id);
        assert_eq!(
            router.gesture_owner(egui::PointerButton::Secondary),
            Some(InputRecipient::Root)
        );
    }

    #[test]
    fn windowing_input_egui_bridge_rejects_invalid_canvas_and_stale_event_before_callbacks() {
        let (manager, _, mut surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let batch = frame(&mut router, 1, &manager, &surfaces, &[key(egui::Key::F3)]);
        let ctx = egui::Context::default();
        let mut calls = 0;
        assert!(matches!(
            router.deliver_egui(
                batch.egui[0],
                &ctx,
                &mut surfaces,
                egui::Rect::NOTHING,
                Default::default(),
                |_, _, _, _| {
                    calls += 1;
                    InputDisposition::Unhandled
                }
            ),
            Err(BridgeError::Surface(SurfaceError::InvalidCanvas))
        ));
        assert_eq!(calls, 0);
        assert_eq!(
            router.recipient(batch.egui[0]),
            InputRecipient::Window(manager.focused().unwrap().window)
        );
        frame(&mut router, 2, &manager, &surfaces, &[key(egui::Key::F3)]);
        assert!(matches!(
            router.deliver_egui(
                batch.egui[0],
                &ctx,
                &mut surfaces,
                rect(0., 300.),
                Default::default(),
                |_, _, _, _| {
                    calls += 1;
                    InputDisposition::Unhandled
                }
            ),
            Err(BridgeError::Input(InputError::UnknownEvent))
        ));
        assert_eq!(calls, 0);
    }

    #[test]
    fn windowing_input_wheel_position_is_retained_before_later_cursor_motion() {
        let (manager, _, surfaces) = setup(false, false);
        let mut router = FrameInputRouter::default();
        let wheel = egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0., 1.),
            modifiers: egui::Modifiers::NONE,
        };
        let batch = frame(
            &mut router,
            1,
            &manager,
            &surfaces,
            &[
                egui::Event::PointerMoved(egui::pos2(10., 10.)),
                wheel,
                egui::Event::PointerMoved(egui::pos2(210., 10.)),
            ],
        );
        assert_eq!(
            router.pointer_position(batch.egui[1]),
            Some(egui::pos2(10., 10.))
        );
    }
    #[test]
    fn windowing_input_polling_key_mirror_keeps_the_canonical_repeat_and_modifier_metadata() {
        let (manager, id, surfaces) = setup(false, false);
        let mut canonical = key(egui::Key::F7);
        if let egui::Event::Key {
            repeat,
            physical_key,
            modifiers,
            ..
        } = &mut canonical
        {
            *repeat = true;
            *physical_key = Some(egui::Key::F7);
            modifiers.alt = true;
        }
        // A polling projection knows a pressed key, not the original repeat,
        // chord or scan metadata. It is a copy of the canonical occurrence.
        let mirror = key(egui::Key::F7);
        let mut router = FrameInputRouter::default();
        let batch = router
            .begin_frame(
                1,
                &manager.begin_frame(),
                &surfaces,
                std::slice::from_ref(&canonical),
                &[mirror],
                false,
            )
            .unwrap();
        assert_eq!(batch.events.len(), 1);
        assert_eq!(batch.egui, batch.macroquad);
        assert_eq!(batch.events[0].event, canonical);
        router
            .report_key(
                batch.egui[0],
                InputRecipient::Window(id),
                InputDisposition::Unhandled,
            )
            .unwrap();
        assert!(!router.claim_trigger(batch.macroquad[0], InputRecipient::Root));
    }

    #[test]
    fn windowing_input_polling_pointer_mirror_keeps_the_canonical_chord_and_coordinates() {
        let (manager, id, surfaces) = setup(false, false);
        let mut canonical = button(egui::pos2(10., 10.), true);
        if let egui::Event::PointerButton { modifiers, .. } = &mut canonical {
            modifiers.alt = true;
        }
        let mirror = button(egui::pos2(210., 10.), true);
        let mut router = FrameInputRouter::default();
        let batch = router
            .begin_frame(
                1,
                &manager.begin_frame(),
                &surfaces,
                std::slice::from_ref(&canonical),
                &[mirror],
                false,
            )
            .unwrap();
        assert_eq!(batch.events.len(), 1);
        assert_eq!(batch.egui, batch.macroquad);
        assert_eq!(batch.events[0].event, canonical);
        assert!(router.claim(batch.macroquad[0], InputRecipient::Window(id)));
        assert!(!router.claim(batch.egui[0], InputRecipient::Window(id)));
    }

    #[test]
    fn windowing_input_mirror_correlation_distinguishes_key_button_and_press_phase() {
        let (manager, _, surfaces) = setup(false, false);
        let press_a = key(egui::Key::F3);
        let press_b = key(egui::Key::F7);
        let mut release_a = key(egui::Key::F3);
        if let egui::Event::Key { pressed, .. } = &mut release_a {
            *pressed = false;
        }
        let pointer_down = button(egui::pos2(10., 10.), true);
        let pointer_up = button(egui::pos2(10., 10.), false);
        let mut secondary = pointer_down.clone();
        if let egui::Event::PointerButton { button, .. } = &mut secondary {
            *button = egui::PointerButton::Secondary;
        }
        let originals = [
            press_a.clone(),
            press_b.clone(),
            release_a.clone(),
            pointer_down.clone(),
            pointer_up.clone(),
            secondary.clone(),
        ];
        let mirrors = [
            press_b,
            release_a,
            press_a,
            pointer_up,
            secondary,
            pointer_down,
        ];
        let mut router = FrameInputRouter::default();
        let batch = router
            .begin_frame(
                1,
                &manager.begin_frame(),
                &surfaces,
                &originals,
                &mirrors,
                false,
            )
            .unwrap();
        assert_eq!(batch.events.len(), 6);
        assert_eq!(
            batch.macroquad,
            [
                batch.egui[1],
                batch.egui[2],
                batch.egui[0],
                batch.egui[4],
                batch.egui[5],
                batch.egui[3]
            ]
        );
    }
}
