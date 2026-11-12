//! Shared constructors for window-manager integration tests.

#![allow(dead_code)]

use rebellion_windowing::{
    Cardinality, ExistingOpenPolicy, FocusOnOpen, FocusRestoration, InputPolicy, OpenPlacement,
    PresentationPolicy, StackBand, SubjectId, WindowCommand, WindowId, WindowKey, WindowKind,
    WindowManager, WindowOwner, WindowPolicy, WindowSpec, WindowTransition,
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

/// Canonical pair of modeless strategic details, opened in stack order.
pub fn system_and_fleet() -> (WindowManager, WindowId, WindowId) {
    let mut manager = WindowManager::new();
    let system = open(&mut manager, root_detail(WindowKind::System, 1));
    let fleet = open(&mut manager, root_detail(WindowKind::Fleet, 2));
    (manager, system, fleet)
}

pub fn modal(kind: WindowKind) -> WindowSpec {
    WindowSpec {
        key: WindowKey::singleton(kind),
        owner: WindowOwner::StrategicRoot,
        policy: WindowPolicy {
            cardinality: Cardinality::Singleton,
            band: StackBand::Normal,
            placement: OpenPlacement::TopOfBand,
            existing: ExistingOpenPolicy::NoOp,
            input: InputPolicy::StrategicModal,
            presentation: PresentationPolicy::None,
            focus_on_open: FocusOnOpen::Window,
            restoration: FocusRestoration::PreviousEligible,
        },
    }
}

pub fn menu(owner: WindowId) -> WindowSpec {
    WindowSpec {
        key: WindowKey::singleton(WindowKind::Menu),
        owner: WindowOwner::Window(owner),
        policy: WindowPolicy {
            cardinality: Cardinality::Singleton,
            band: StackBand::Transient,
            placement: OpenPlacement::AboveOwner,
            existing: ExistingOpenPolicy::NoOp,
            input: InputPolicy::DismissOnOutsidePress,
            presentation: PresentationPolicy::None,
            focus_on_open: FocusOnOpen::Window,
            restoration: FocusRestoration::SavedMenuTarget,
        },
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
