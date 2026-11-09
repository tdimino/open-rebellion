//! Frame-local input admission for strategic windows and root UI.

use std::collections::{BTreeMap, BTreeSet};

use rebellion_windowing::{BubblePolicy, RouteTarget, RoutingSnapshot, WindowId};

/// Stable identity assigned while normalizing one backend input batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputEventId {
    /// Frame number in the local renderer session.
    pub frame: u64,
    /// Physical event position in that frame's normalized batch.
    pub ordinal: u32,
}

/// Whether a recipient performed an action for a keyboard event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDisposition {
    /// The event changed state or activated a command.
    Handled,
    /// The recipient deliberately declined the event.
    Unhandled,
}

/// Semantic owner of one normalized physical event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputRecipient {
    /// Persistent strategic root.
    Root,
    /// One managed top-level window.
    Window(WindowId),
    /// A modal, hold, cancelled gesture, or invalid route consumed the event.
    Blocked,
}

/// Stable identity for a physical pointer button across backend event copies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PointerButtonId {
    /// Primary or left mouse button.
    Primary,
    /// Secondary or right mouse button.
    Secondary,
    /// Middle mouse button.
    Middle,
    /// Backend-specific additional button.
    Extra(u8),
}

/// Pointer event role used to preserve physical press ownership.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerPhase {
    /// Physical button-down.
    Press(PointerButtonId),
    /// Drag motion attributed to one still-held physical button.
    Motion(PointerButtonId),
    /// Physical button-up.
    Release(PointerButtonId),
    /// Motion with no admitted button gesture.
    Hover,
    /// Wheel or other position-bearing pointer input.
    Auxiliary,
}

/// Escape dispatch order for focused controls and their owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeRoute {
    /// Focused child control gets first refusal.
    Control(rebellion_windowing::FocusTarget),
    /// Top-level window receives Escape.
    Window(WindowId),
    /// Strategic root receives Escape.
    Root,
    /// A hold prevents delivery.
    Blocked,
}

#[derive(Debug, Clone, Copy)]
struct EventState {
    recipient: InputRecipient,
    bubble: BubblePolicy,
    disposition: Option<InputDisposition>,
    recipient_claimed: bool,
    root_claimed: bool,
}

/// Frame pointer routing plus event-local keyboard routing, with bounded
/// cross-frame gesture ownership.
#[derive(Debug, Default)]
pub struct FrameInputRouter {
    frame_number: Option<u64>,
    snapshot: Option<RoutingSnapshot>,
    events: BTreeMap<InputEventId, EventState>,
    press_owners: BTreeMap<PointerButtonId, InputRecipient>,
    quarantined_releases: BTreeSet<PointerButtonId>,
    frame_locked: bool,
    hold_after_ordinal: Option<u32>,
}

impl FrameInputRouter {
    /// Starts a frame without discarding a valid cross-frame pointer gesture.
    pub fn begin_frame(&mut self, frame_number: u64, snapshot: &RoutingSnapshot) {
        let blocked_buttons: Vec<_> = self
            .press_owners
            .iter()
            .filter_map(|(button, owner)| {
                let blocked = match owner {
                    InputRecipient::Root => !snapshot.pointer_allows_root(),
                    InputRecipient::Window(window) => !snapshot.allows_input(*window),
                    InputRecipient::Blocked => false,
                };
                blocked.then_some(*button)
            })
            .collect();
        for button in blocked_buttons {
            self.press_owners.remove(&button);
            self.quarantined_releases.insert(button);
        }
        self.frame_number = Some(frame_number);
        self.snapshot = Some(snapshot.clone());
        self.events.clear();
        self.frame_locked = false;
        self.hold_after_ordinal = None;
    }

    /// Applies an application hold monotonically for the current frame.
    /// Releasing the hold takes effect only after the next `begin_frame`.
    pub fn set_application_hold(&mut self, blocked: bool) {
        self.frame_locked |= blocked;
        if blocked {
            let pressed_buttons: Vec<_> = self.press_owners.keys().copied().collect();
            self.press_owners.clear();
            self.quarantined_releases.extend(pressed_buttons);
        }
    }

    /// Fences only physical events later than the input that introduced a
    /// same-frame application restriction. Earlier pointer/key events retain
    /// their already-earned route; later ones cannot pass the new boundary.
    pub fn set_application_hold_after(&mut self, event: InputEventId) {
        if !self.valid_event(event) {
            return;
        }
        self.hold_after_ordinal = Some(
            self.hold_after_ordinal
                .map_or(event.ordinal, |ordinal| ordinal.min(event.ordinal)),
        );
    }

    /// Routes one pointer event using its own point-derived hit set.
    pub fn collect_pointer(
        &mut self,
        event: InputEventId,
        hit_windows: &BTreeSet<WindowId>,
        phase: PointerPhase,
    ) -> InputRecipient {
        let target = self
            .snapshot
            .as_ref()
            .expect("begin_frame installs a snapshot")
            .pointer_route(hit_windows)
            .target;
        self.collect_pointer_route(event, target, phase)
    }

    /// Routes one pointer event against its event-local manager projection.
    /// Gesture ownership remains in this router; only the hit-test snapshot
    /// advances after an earlier physical lifecycle event.
    pub fn collect_pointer_from_snapshot(
        &mut self,
        event: InputEventId,
        snapshot: &RoutingSnapshot,
        hit_windows: &BTreeSet<WindowId>,
        phase: PointerPhase,
    ) -> InputRecipient {
        self.collect_pointer_route(event, snapshot.pointer_route(hit_windows).target, phase)
    }

    fn collect_pointer_route(
        &mut self,
        event: InputEventId,
        target: RouteTarget,
        phase: PointerPhase,
    ) -> InputRecipient {
        if !self.valid_event(event) || self.events.contains_key(&event) {
            return InputRecipient::Blocked;
        }
        let routed = route_recipient(target);
        if let PointerPhase::Press(button) = phase {
            // A new press proves this same button returned up even if its
            // earlier release was not observed. Other buttons remain isolated.
            self.quarantined_releases.remove(&button);
        }
        let quarantined_button = match phase {
            PointerPhase::Release(button) => self.quarantined_releases.remove(&button),
            PointerPhase::Press(_)
            | PointerPhase::Motion(_)
            | PointerPhase::Hover
            | PointerPhase::Auxiliary => false,
        };
        let recipient = if quarantined_button {
            if let PointerPhase::Release(button) = phase {
                self.press_owners.remove(&button);
            }
            InputRecipient::Blocked
        } else if self.event_blocked(event) {
            InputRecipient::Blocked
        } else {
            match phase {
                PointerPhase::Press(button) => {
                    self.press_owners.insert(button, routed);
                    routed
                }
                PointerPhase::Motion(button) => self
                    .press_owners
                    .get(&button)
                    .copied()
                    .unwrap_or(InputRecipient::Blocked),
                PointerPhase::Release(button) => self
                    .press_owners
                    .remove(&button)
                    .unwrap_or(InputRecipient::Blocked),
                PointerPhase::Hover | PointerPhase::Auxiliary => routed,
            }
        };
        self.events.insert(
            event,
            EventState {
                recipient,
                bubble: BubblePolicy::StopAtWindow,
                disposition: None,
                recipient_claimed: false,
                root_claimed: false,
            },
        );
        recipient
    }

    /// Routes one keyboard event against the frame's immutable focus lease.
    pub fn collect_keyboard(&mut self, event: InputEventId) -> InputRecipient {
        let route = self
            .snapshot
            .as_ref()
            .expect("begin_frame installs a snapshot")
            .keyboard_route();
        self.collect_keyboard_route(event, route)
    }

    /// Routes one physical keyboard event against its event-local manager
    /// projection. Pointer gestures retain the frame snapshot and their
    /// press owner; this override is only for ordered keyboard dispatch after
    /// an earlier key projected a lifecycle/focus command.
    pub fn collect_keyboard_from_snapshot(
        &mut self,
        event: InputEventId,
        snapshot: &RoutingSnapshot,
    ) -> InputRecipient {
        self.collect_keyboard_route(event, snapshot.keyboard_route())
    }

    fn collect_keyboard_route(
        &mut self,
        event: InputEventId,
        route: rebellion_windowing::KeyboardRoute,
    ) -> InputRecipient {
        if !self.valid_event(event) || self.events.contains_key(&event) {
            return InputRecipient::Blocked;
        }
        let recipient = if self.event_blocked(event) {
            InputRecipient::Blocked
        } else {
            route_recipient(route.target)
        };
        self.events.insert(
            event,
            EventState {
                recipient,
                bubble: route.bubble,
                disposition: None,
                recipient_claimed: false,
                root_claimed: false,
            },
        );
        recipient
    }

    /// Returns the previously normalized pointer recipient.
    #[must_use]
    pub fn pointer_recipient(&self, event: InputEventId) -> InputRecipient {
        self.recipient(event)
    }

    /// Returns the previously normalized keyboard recipient.
    #[must_use]
    pub fn keyboard_recipient(&self, event: InputEventId) -> InputRecipient {
        self.recipient(event)
    }

    /// Whether an application hold, rather than the manager route itself,
    /// fences this physical event.
    #[must_use]
    pub fn application_fences(&self, event: InputEventId) -> bool {
        self.event_blocked(event)
    }

    /// Claims one physical event once across macroquad and egui copies.
    pub fn claim(&mut self, event: InputEventId, owner: InputRecipient) -> bool {
        if owner == InputRecipient::Blocked || self.event_blocked(event) {
            return false;
        }
        let Some(state) = self.events.get_mut(&event) else {
            return false;
        };
        if state.recipient != owner {
            return false;
        }
        if owner == InputRecipient::Root {
            if state.root_claimed {
                return false;
            }
            state.root_claimed = true;
            true
        } else {
            if state.recipient_claimed {
                return false;
            }
            state.recipient_claimed = true;
            true
        }
    }

    /// Claims the root leg of a direct or explicitly bubbled key once.
    pub fn claim_root(&mut self, event: InputEventId) -> bool {
        if !self.root_may_handle(event) {
            return false;
        }
        let state = self
            .events
            .get_mut(&event)
            .expect("root eligibility requires a collected event");
        if state.root_claimed {
            return false;
        }
        state.root_claimed = true;
        true
    }

    /// Records actual child/top-level keyboard disposition.
    pub fn report_key(
        &mut self,
        event: InputEventId,
        owner: InputRecipient,
        result: InputDisposition,
    ) {
        if self.event_blocked(event) {
            return;
        }
        if let Some(state) = self.events.get_mut(&event) {
            if state.recipient == owner && state.disposition.is_none() {
                state.disposition = Some(result);
            }
        }
    }

    /// Whether the root may execute this key exactly after owner disposition.
    #[must_use]
    pub fn root_may_handle(&self, event: InputEventId) -> bool {
        if self.event_blocked(event) {
            return false;
        }
        let Some(state) = self.events.get(&event) else {
            return false;
        };
        if state.root_claimed {
            return false;
        }
        match state.recipient {
            InputRecipient::Root => state.disposition != Some(InputDisposition::Handled),
            InputRecipient::Window(_) => {
                state.bubble == BubblePolicy::ToRoot
                    && state.disposition == Some(InputDisposition::Unhandled)
            }
            InputRecipient::Blocked => false,
        }
    }

    /// Earliest physical event that a recipient has acted on this frame.
    /// Used to place a same-frame application fence without erasing earlier
    /// pointer/key order.
    #[must_use]
    pub fn first_handled_event(&self) -> Option<InputEventId> {
        self.events.iter().find_map(|(event, state)| {
            (state.recipient_claimed
                || state.root_claimed
                || state.disposition == Some(InputDisposition::Handled))
            .then_some(*event)
        })
    }

    /// Cancels a matching drag and quarantines its eventual physical release.
    pub fn cancel_window(&mut self, id: WindowId) {
        let cancelled_buttons: Vec<_> = self
            .press_owners
            .iter()
            .filter_map(|(button, owner)| (*owner == InputRecipient::Window(id)).then_some(*button))
            .collect();
        for button in cancelled_buttons {
            self.press_owners.remove(&button);
            self.quarantined_releases.insert(button);
        }
    }

    /// Drops all frame and gesture state at a campaign/session boundary.
    pub fn reset_session(&mut self) {
        self.frame_number = None;
        self.snapshot = None;
        self.events.clear();
        self.press_owners.clear();
        self.quarantined_releases.clear();
        self.frame_locked = false;
        self.hold_after_ordinal = None;
    }

    /// Returns the frozen Escape propagation entry point.
    #[must_use]
    pub fn escape_route(&self) -> EscapeRoute {
        if self.frame_locked {
            return EscapeRoute::Blocked;
        }
        match self
            .snapshot
            .as_ref()
            .map(RoutingSnapshot::keyboard_route)
            .map(|route| route.target)
        {
            Some(RouteTarget::Window(target)) if target.control.is_some() => {
                EscapeRoute::Control(target)
            }
            Some(RouteTarget::Window(target)) => EscapeRoute::Window(target.window),
            Some(RouteTarget::StrategicRoot) => EscapeRoute::Root,
            Some(RouteTarget::BlockedBy(_)) | None => EscapeRoute::Blocked,
        }
    }

    fn valid_event(&self, event: InputEventId) -> bool {
        self.frame_number == Some(event.frame)
    }

    fn event_blocked(&self, event: InputEventId) -> bool {
        self.frame_locked
            || (self.valid_event(event)
                && self
                    .hold_after_ordinal
                    .is_some_and(|ordinal| event.ordinal > ordinal))
    }

    fn recipient(&self, event: InputEventId) -> InputRecipient {
        if self.event_blocked(event) {
            return InputRecipient::Blocked;
        }
        self.events
            .get(&event)
            .map_or(InputRecipient::Blocked, |state| state.recipient)
    }
}

fn route_recipient(target: RouteTarget) -> InputRecipient {
    match target {
        RouteTarget::StrategicRoot => InputRecipient::Root,
        RouteTarget::Window(target) => InputRecipient::Window(target.window),
        RouteTarget::BlockedBy(_) => InputRecipient::Blocked,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_windowing::{
        Cardinality, ExistingOpenPolicy, FocusOnOpen, FocusRestoration, InputPolicy, OpenPlacement,
        PresentationPolicy, StackBand, SubjectId, WindowCommand, WindowKey, WindowKind,
        WindowManager, WindowOwner, WindowPolicy, WindowSpec, WindowTransition,
    };

    fn open(manager: &mut WindowManager, subject: u64, input: InputPolicy) -> WindowId {
        let spec = WindowSpec {
            key: WindowKey::contextual(WindowKind::System, SubjectId::new(2, subject)),
            owner: WindowOwner::StrategicRoot,
            policy: WindowPolicy {
                cardinality: Cardinality::Subject { max_open: None },
                band: StackBand::Normal,
                placement: OpenPlacement::TopOfBand,
                existing: ExistingOpenPolicy::RaiseRestore,
                input,
                presentation: PresentationPolicy::History,
                focus_on_open: FocusOnOpen::Window,
                restoration: FocusRestoration::PreviousEligible,
            },
        };
        match manager.apply(WindowCommand::Open(spec)).unwrap().transition {
            WindowTransition::Opened { id } => id,
            other => panic!("expected open, got {other:?}"),
        }
    }

    fn event(frame: u64, ordinal: u32) -> InputEventId {
        InputEventId { frame, ordinal }
    }

    #[test]
    fn windowing_input_handled_child_key_never_reaches_root() {
        let mut manager = WindowManager::new();
        let window = open(&mut manager, 1, InputPolicy::Modeless);
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        let id = event(1, 0);
        assert_eq!(router.collect_keyboard(id), InputRecipient::Window(window));
        router.report_key(
            id,
            InputRecipient::Window(window),
            InputDisposition::Handled,
        );
        assert!(!router.root_may_handle(id));
    }

    #[test]
    fn windowing_input_same_frame_hold_preserves_pointer_key_physical_order() {
        let manager = WindowManager::new();
        let mut router = FrameInputRouter::default();
        router.begin_frame(19, &manager.begin_frame());
        let key_before = event(19, 0);
        let pointer_press = event(19, 1);
        let restricting_pointer = event(19, 2);
        let key_after = event(19, 3);
        assert_eq!(router.collect_keyboard(key_before), InputRecipient::Root);
        assert_eq!(
            router.collect_pointer(
                pointer_press,
                &BTreeSet::new(),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Root
        );
        assert_eq!(
            router.collect_pointer(
                restricting_pointer,
                &BTreeSet::new(),
                PointerPhase::Release(PointerButtonId::Primary),
            ),
            InputRecipient::Root
        );
        assert_eq!(router.collect_keyboard(key_after), InputRecipient::Root);

        router.set_application_hold_after(restricting_pointer);

        assert!(router.claim_root(key_before));
        assert_eq!(
            router.keyboard_recipient(key_after),
            InputRecipient::Blocked
        );
        assert!(!router.claim_root(key_after));
    }

    #[test]
    fn windowing_input_unhandled_modeless_key_can_reach_root_once() {
        let mut manager = WindowManager::new();
        let window = open(&mut manager, 1, InputPolicy::Modeless);
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        let id = event(1, 0);
        router.collect_keyboard(id);
        assert!(router.claim(id, InputRecipient::Window(window)));
        router.report_key(
            id,
            InputRecipient::Window(window),
            InputDisposition::Unhandled,
        );
        assert!(router.root_may_handle(id));
        assert!(router.claim_root(id));
        assert!(!router.claim_root(id));
    }

    #[test]
    fn windowing_input_held_release_consumes_quarantine_before_next_gesture() {
        let mut manager = WindowManager::new();
        let window = open(&mut manager, 1, InputPolicy::Modeless);
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        router.collect_pointer(
            event(1, 0),
            &BTreeSet::from([window]),
            PointerPhase::Press(PointerButtonId::Primary),
        );
        router.cancel_window(window);

        router.begin_frame(2, &manager.begin_frame());
        router.set_application_hold(true);
        assert_eq!(
            router.collect_pointer(
                event(2, 0),
                &BTreeSet::new(),
                PointerPhase::Release(PointerButtonId::Primary),
            ),
            InputRecipient::Blocked
        );

        router.begin_frame(3, &manager.begin_frame());
        assert_eq!(
            router.collect_pointer(
                event(3, 0),
                &BTreeSet::from([window]),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Window(window)
        );
        assert_eq!(
            router.collect_pointer(
                event(3, 1),
                &BTreeSet::new(),
                PointerPhase::Release(PointerButtonId::Primary),
            ),
            InputRecipient::Window(window)
        );
    }

    #[test]
    fn windowing_input_new_press_clears_an_unobserved_cancelled_release() {
        let mut manager = WindowManager::new();
        let window = open(&mut manager, 1, InputPolicy::Modeless);
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        router.collect_pointer(
            event(1, 0),
            &BTreeSet::from([window]),
            PointerPhase::Press(PointerButtonId::Primary),
        );
        router.cancel_window(window);

        // The physical release occurred while no input batch was collected.
        router.begin_frame(2, &manager.begin_frame());
        assert_eq!(
            router.collect_pointer(
                event(2, 0),
                &BTreeSet::from([window]),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Window(window)
        );
        assert_eq!(
            router.collect_pointer(
                event(2, 1),
                &BTreeSet::new(),
                PointerPhase::Release(PointerButtonId::Primary),
            ),
            InputRecipient::Window(window)
        );
    }

    #[test]
    fn windowing_input_overlapping_buttons_keep_cancelled_release_isolated() {
        let mut manager = WindowManager::new();
        let window = open(&mut manager, 1, InputPolicy::Modeless);
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        router.collect_pointer(
            event(1, 0),
            &BTreeSet::from([window]),
            PointerPhase::Press(PointerButtonId::Primary),
        );
        router.cancel_window(window);

        router.begin_frame(2, &manager.begin_frame());
        assert_eq!(
            router.collect_pointer(
                event(2, 0),
                &BTreeSet::from([window]),
                PointerPhase::Press(PointerButtonId::Secondary),
            ),
            InputRecipient::Window(window)
        );
        assert_eq!(
            router.collect_pointer(
                event(2, 1),
                &BTreeSet::new(),
                PointerPhase::Release(PointerButtonId::Primary),
            ),
            InputRecipient::Blocked
        );
        assert_eq!(
            router.collect_pointer(
                event(2, 2),
                &BTreeSet::new(),
                PointerPhase::Release(PointerButtonId::Secondary),
            ),
            InputRecipient::Window(window)
        );
    }

    #[test]
    fn windowing_input_overlapping_buttons_route_motion_to_named_owner() {
        let mut manager = WindowManager::new();
        let primary_owner = open(&mut manager, 1, InputPolicy::Modeless);
        let secondary_owner = open(&mut manager, 2, InputPolicy::Modeless);
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        assert_eq!(
            router.collect_pointer(
                event(1, 0),
                &BTreeSet::from([primary_owner]),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Window(primary_owner)
        );
        assert_eq!(
            router.collect_pointer(
                event(1, 1),
                &BTreeSet::from([secondary_owner]),
                PointerPhase::Press(PointerButtonId::Secondary),
            ),
            InputRecipient::Window(secondary_owner)
        );
        assert_eq!(
            router.collect_pointer(
                event(1, 2),
                &BTreeSet::new(),
                PointerPhase::Motion(PointerButtonId::Secondary),
            ),
            InputRecipient::Window(secondary_owner)
        );
        assert_eq!(
            router.collect_pointer(
                event(1, 3),
                &BTreeSet::new(),
                PointerPhase::Motion(PointerButtonId::Primary),
            ),
            InputRecipient::Window(primary_owner)
        );
    }

    #[test]
    fn windowing_input_modal_unhandled_key_stops_at_window() {
        let mut manager = WindowManager::new();
        let modal = open(&mut manager, 1, InputPolicy::StrategicModal);
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        let id = event(1, 0);
        router.collect_keyboard(id);
        router.report_key(
            id,
            InputRecipient::Window(modal),
            InputDisposition::Unhandled,
        );
        assert!(!router.root_may_handle(id));
    }

    #[test]
    fn windowing_input_macroquad_and_egui_copies_share_one_claim() {
        let mut manager = WindowManager::new();
        let window = open(&mut manager, 1, InputPolicy::Modeless);
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        let id = event(1, 0);
        router.collect_pointer(
            id,
            &BTreeSet::from([window]),
            PointerPhase::Press(PointerButtonId::Primary),
        );
        assert!(router.claim(id, InputRecipient::Window(window)));
        assert!(!router.claim(id, InputRecipient::Window(window)));
    }

    #[test]
    fn windowing_input_event_coordinates_not_final_cursor_choose_target() {
        let mut manager = WindowManager::new();
        let first = open(&mut manager, 1, InputPolicy::Modeless);
        let second = open(&mut manager, 2, InputPolicy::Modeless);
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        let id = event(1, 0);
        assert_eq!(
            router.collect_pointer(
                id,
                &BTreeSet::from([first]),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Window(first)
        );
        assert_ne!(router.pointer_recipient(id), InputRecipient::Window(second));
    }

    #[test]
    fn windowing_input_cancelled_drag_quarantines_later_release() {
        let mut manager = WindowManager::new();
        let window = open(&mut manager, 1, InputPolicy::Modeless);
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        router.collect_pointer(
            event(1, 0),
            &BTreeSet::from([window]),
            PointerPhase::Press(PointerButtonId::Primary),
        );
        router.cancel_window(window);
        router.begin_frame(2, &manager.begin_frame());
        assert_eq!(
            router.collect_pointer(
                event(2, 0),
                &BTreeSet::new(),
                PointerPhase::Release(PointerButtonId::Primary),
            ),
            InputRecipient::Blocked
        );
    }

    #[test]
    fn windowing_input_new_modal_quarantines_an_existing_root_gesture() {
        let mut manager = WindowManager::new();
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        assert_eq!(
            router.collect_pointer(
                event(1, 0),
                &BTreeSet::new(),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Root
        );
        open(&mut manager, 1, InputPolicy::StrategicModal);
        router.begin_frame(2, &manager.begin_frame());
        assert_eq!(
            router.collect_pointer(
                event(2, 0),
                &BTreeSet::new(),
                PointerPhase::Release(PointerButtonId::Primary),
            ),
            InputRecipient::Blocked
        );
    }

    #[test]
    fn windowing_input_hold_release_does_not_unlock_existing_frame() {
        let manager = WindowManager::new();
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        router.set_application_hold(true);
        router.set_application_hold(false);
        assert_eq!(
            router.collect_keyboard(event(1, 0)),
            InputRecipient::Blocked
        );
        router.begin_frame(2, &manager.begin_frame());
        assert_eq!(router.collect_keyboard(event(2, 0)), InputRecipient::Root);
    }

    #[test]
    fn windowing_input_blocked_event_is_never_claimable() {
        let manager = WindowManager::new();
        let mut router = FrameInputRouter::default();
        router.begin_frame(1, &manager.begin_frame());
        router.set_application_hold(true);
        let blocked = event(1, 0);
        assert_eq!(
            router.collect_pointer(
                blocked,
                &BTreeSet::new(),
                PointerPhase::Press(PointerButtonId::Primary),
            ),
            InputRecipient::Blocked
        );
        assert!(!router.claim(blocked, InputRecipient::Blocked));
    }
}
