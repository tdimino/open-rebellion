//! Shared constructors for window-manager integration tests.

#![allow(dead_code)]

use rebellion_windowing::{
    Cardinality, CloseReason, ControlId, ExistingOpenPolicy, FocusOnOpen, FocusRestoration,
    FocusTarget, InputPolicy, OpenPlacement, PresentationPolicy, StackBand, SubjectId,
    WindowCommand, WindowEvent, WindowId, WindowKey, WindowKind, WindowManager, WindowOwner,
    WindowPolicy, WindowSpec, WindowTransition,
};

pub fn detail(kind: WindowKind, subject: u64, owner: WindowOwner) -> WindowSpec {
    WindowSpec {
        key: WindowKey::contextual(kind, SubjectId::new(2, subject)),
        owner,
        policy: WindowPolicy::restorable_modeless_history(FocusOnOpen::Window),
    }
}

pub fn root_detail(kind: WindowKind, subject: u64) -> WindowSpec {
    detail(kind, subject, WindowOwner::StrategicRoot)
}

/// Open one modeless strategic-root detail with the canonical test policy.
pub fn open_root_detail(manager: &mut WindowManager, kind: WindowKind, subject: u64) -> WindowId {
    open(manager, root_detail(kind, subject))
}

/// Canonical pair of modeless strategic details, opened in stack order.
pub fn system_and_fleet() -> (WindowManager, WindowId, WindowId) {
    let mut manager = WindowManager::new();
    let system = open_root_detail(&mut manager, WindowKind::System, 1);
    let fleet = open_root_detail(&mut manager, WindowKind::Fleet, 2);
    (manager, system, fleet)
}

/// Window-level focus target used by lifecycle and routing assertions.
pub const fn window_focus(window: WindowId) -> FocusTarget {
    focus_target(window, None)
}

/// Child-control focus target used by lifecycle and routing assertions.
pub const fn control_focus(window: WindowId, control: u64) -> FocusTarget {
    focus_target(window, Some(ControlId(control)))
}

const fn focus_target(window: WindowId, control: Option<ControlId>) -> FocusTarget {
    FocusTarget { window, control }
}

/// Canonical close receipt used by lifecycle and reconciliation assertions.
pub const fn closed_event(window: WindowId, key: WindowKey, reason: CloseReason) -> WindowEvent {
    WindowEvent::Closed {
        window,
        key,
        reason,
    }
}

/// Canonical focus-change receipt used by lifecycle and reconciliation assertions.
pub const fn focus_changed_event(
    previous: Option<FocusTarget>,
    current: Option<FocusTarget>,
) -> WindowEvent {
    WindowEvent::FocusChanged { previous, current }
}

/// Canonical close transition used by lifecycle assertions.
pub fn closed_transition(
    root: WindowId,
    closed: Vec<WindowId>,
    restored_focus: Option<FocusTarget>,
) -> WindowTransition {
    WindowTransition::Closed {
        root,
        closed,
        restored_focus,
    }
}

pub fn modal(kind: WindowKind) -> WindowSpec {
    WindowSpec {
        key: WindowKey::singleton(kind),
        owner: WindowOwner::StrategicRoot,
        policy: singleton_policy(
            StackBand::Normal,
            OpenPlacement::TopOfBand,
            InputPolicy::StrategicModal,
            FocusRestoration::PreviousEligible,
        ),
    }
}

pub fn menu(owner: WindowId) -> WindowSpec {
    WindowSpec {
        key: WindowKey::singleton(WindowKind::Menu),
        owner: WindowOwner::Window(owner),
        policy: singleton_policy(
            StackBand::Transient,
            OpenPlacement::AboveOwner,
            InputPolicy::DismissOnOutsidePress,
            FocusRestoration::SavedMenuTarget,
        ),
    }
}

const fn singleton_policy(
    band: StackBand,
    placement: OpenPlacement,
    input: InputPolicy,
    restoration: FocusRestoration,
) -> WindowPolicy {
    WindowPolicy {
        cardinality: Cardinality::Singleton,
        band,
        placement,
        existing: ExistingOpenPolicy::NoOp,
        input,
        presentation: PresentationPolicy::None,
        focus_on_open: FocusOnOpen::Window,
        restoration,
    }
}

pub fn open(manager: &mut WindowManager, spec: WindowSpec) -> WindowId {
    manager
        .apply(WindowCommand::Open(spec))
        .expect("window opens")
        .transition
        .opened_id()
        .expect("expected a new window transition")
}

pub fn opened_id(transition: WindowTransition) -> WindowId {
    match transition {
        WindowTransition::Opened { id } => id,
        other => panic!("expected a new window, got {other:?}"),
    }
}
