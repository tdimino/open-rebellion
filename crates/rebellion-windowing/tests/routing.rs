use std::collections::BTreeSet;

use rebellion_windowing::{
    BubblePolicy, Cardinality, ControlId, ExistingOpenPolicy, FocusOnOpen, FocusRestoration,
    FocusTarget, InputPolicy, KeyboardRoute, OpenPlacement, PresentationPolicy, RouteTarget,
    StackBand, SubjectId, WindowCommand, WindowId, WindowKey, WindowKind, WindowManager,
    WindowOwner, WindowPolicy, WindowSpec, WindowTransition,
};

fn detail(kind: WindowKind, subject: u64) -> WindowSpec {
    WindowSpec {
        key: WindowKey::contextual(kind, SubjectId::new(2, subject)),
        owner: WindowOwner::StrategicRoot,
        policy: WindowPolicy {
            cardinality: Cardinality::Subject { max_open: None },
            band: StackBand::Normal,
            placement: OpenPlacement::TopOfBand,
            existing: ExistingOpenPolicy::RaiseRestore,
            input: InputPolicy::Modeless,
            presentation: PresentationPolicy::History,
            focus_on_open: FocusOnOpen::Window,
            restoration: FocusRestoration::PreviousEligible,
        },
    }
}

fn modal() -> WindowSpec {
    let mut spec = detail(WindowKind::Encyclopedia, 0);
    spec.key = WindowKey::singleton(WindowKind::Encyclopedia);
    spec.policy.cardinality = Cardinality::Singleton;
    spec.policy.existing = ExistingOpenPolicy::NoOp;
    spec.policy.input = InputPolicy::StrategicModal;
    spec.policy.presentation = PresentationPolicy::None;
    spec
}

fn menu(owner: WindowId) -> WindowSpec {
    let mut spec = modal();
    spec.key = WindowKey::singleton(WindowKind::Menu);
    spec.owner = WindowOwner::Window(owner);
    spec.policy.band = StackBand::Transient;
    spec.policy.placement = OpenPlacement::AboveOwner;
    spec.policy.input = InputPolicy::DismissOnOutsidePress;
    spec.policy.restoration = FocusRestoration::SavedMenuTarget;
    spec
}

fn open(manager: &mut WindowManager, spec: WindowSpec) -> WindowId {
    match manager.apply(WindowCommand::Open(spec)).unwrap().transition {
        WindowTransition::Opened { id } => id,
        other => panic!("expected a new window: {other:?}"),
    }
}

fn target(window: WindowId) -> RouteTarget {
    RouteTarget::Window(FocusTarget {
        window,
        control: None,
    })
}

#[test]
fn empty_snapshot_routes_to_the_strategic_root() {
    let frame = WindowManager::new().begin_frame();
    assert_eq!(
        frame.pointer_route(&BTreeSet::new()).target,
        RouteTarget::StrategicRoot
    );
    assert_eq!(
        frame.keyboard_route(),
        KeyboardRoute {
            target: RouteTarget::StrategicRoot,
            bubble: BubblePolicy::ToRoot
        }
    );
    assert!(frame.paint_order().is_empty());
    assert!(frame.presentation_history().is_empty());
    assert!(frame.pointer_allows_root());
    assert_eq!(frame.active_modal(), None);
}

#[test]
fn routing_snapshot_is_immutable_after_close() {
    // Port lease from FUN_00606960/FUN_00422ce0 asynchronous close/restoration.
    let mut manager = WindowManager::new();
    let hidden_spec = detail(WindowKind::System, 1);
    let hidden = open(&mut manager, hidden_spec);
    manager.apply(WindowCommand::Minimize(hidden)).unwrap();
    let visible_spec = detail(WindowKind::Fleet, 2);
    let visible = open(&mut manager, visible_spec);
    let focus = FocusTarget {
        window: visible,
        control: Some(ControlId(7)),
    };
    manager.apply(WindowCommand::Focus(focus)).unwrap();
    manager
        .apply(WindowCommand::CapturePointer(visible))
        .unwrap();
    let old = manager.begin_frame();
    let cloned = old.clone();
    manager.apply(WindowCommand::Close(visible)).unwrap();
    manager.reset_session();
    let fresh = open(&mut manager, visible_spec);
    assert_ne!(fresh, visible);
    assert_eq!(manager.begin_frame().paint_order(), &[fresh]);
    drop(manager);

    for frame in [old, cloned] {
        assert_eq!(frame.paint_order(), &[visible]);
        assert_eq!(frame.presentation_history(), &[hidden]);
        assert_eq!(frame.key(visible), Some(visible_spec.key));
        assert_eq!(frame.key(hidden), Some(hidden_spec.key));
        assert_eq!(frame.owner(visible), Some(WindowOwner::StrategicRoot));
        assert_eq!(frame.focused(), Some(focus));
        assert_eq!(frame.capture_owner(), Some(visible));
        assert!(frame.is_visible(visible));
        assert!(!frame.is_visible(hidden));
        assert!(!frame.allows_input(hidden));
        assert_eq!(frame.key(fresh), None);
        assert_eq!(frame.owner(fresh), None);
        assert!(!frame.allows_input(fresh));
        assert_eq!(
            frame.pointer_route(&BTreeSet::new()).target,
            target(visible)
        );
        assert_eq!(frame.keyboard_route().target, RouteTarget::Window(focus));
    }
}

#[test]
fn topmost_eligible_hit_wins_independent_of_hit_registration_order() {
    // Port: paint order, not registry/key/ID ordering, arbitrates overlapping hits.
    let mut manager = WindowManager::new();
    let first = open(&mut manager, detail(WindowKind::System, 1));
    let second = open(&mut manager, detail(WindowKind::Fleet, 2));
    manager.apply(WindowCommand::Raise(first)).unwrap();
    let frame = manager.begin_frame();
    let mut forward = BTreeSet::new();
    forward.insert(first);
    forward.insert(second);
    let mut reverse = BTreeSet::new();
    reverse.insert(second);
    reverse.insert(first);
    assert_eq!(frame.paint_order(), &[second, first]);
    assert_eq!(frame.pointer_route(&forward).target, target(first));
    assert_eq!(frame.pointer_route(&reverse).target, target(first));
    assert_eq!(
        frame.pointer_route(&BTreeSet::from([second])).target,
        target(second)
    );
}

#[test]
fn modeless_eligibility_does_not_grant_all_windows_the_same_event() {
    let mut manager = WindowManager::new();
    let first = open(&mut manager, detail(WindowKind::System, 1));
    let second = open(&mut manager, detail(WindowKind::Fleet, 2));
    let frame = manager.begin_frame();
    assert!(frame.allows_input(first));
    assert!(frame.allows_input(second));
    assert_eq!(
        frame.pointer_route(&BTreeSet::from([first, second])).target,
        target(second)
    );
    assert_eq!(
        frame.pointer_route(&BTreeSet::from([first])).target,
        target(first)
    );
    assert_eq!(
        frame.pointer_route(&BTreeSet::new()).target,
        RouteTarget::StrategicRoot
    );
    assert_eq!(
        frame.keyboard_route(),
        KeyboardRoute {
            target: target(second),
            bubble: BubblePolicy::ToRoot
        }
    );
}

#[test]
fn modal_blocks_root_inside_and_outside_its_hit_regions() {
    // Source: FUN_00422ce0 modal keyboard bit and retained Encyclopedia Wine pointer evidence.
    let mut manager = WindowManager::new();
    let background = open(&mut manager, detail(WindowKind::System, 1));
    let modal = open(&mut manager, modal());
    let frame = manager.begin_frame();
    assert_eq!(frame.active_modal(), Some(modal));
    assert!(!frame.allows_input(background));
    assert!(frame.allows_input(modal));
    assert!(!frame.pointer_allows_root());
    assert_eq!(
        frame.pointer_route(&BTreeSet::new()).target,
        RouteTarget::BlockedBy(modal)
    );
    assert_eq!(
        frame.pointer_route(&BTreeSet::from([background])).target,
        RouteTarget::BlockedBy(modal)
    );
    assert_eq!(
        frame
            .pointer_route(&BTreeSet::from([background, modal]))
            .target,
        target(modal)
    );
    assert_eq!(
        frame.keyboard_route(),
        KeyboardRoute {
            target: target(modal),
            bubble: BubblePolicy::StopAtWindow
        }
    );
}

#[test]
fn modal_descendant_menu_is_eligible_but_unrelated_menu_is_not() {
    // Port modal ownership boundary: unrelated/stale transients cannot pierce it.
    let mut manager = WindowManager::new();
    let background = open(&mut manager, detail(WindowKind::System, 1));
    let unrelated = open(&mut manager, menu(background));
    manager.apply(WindowCommand::Close(unrelated)).unwrap();
    let modal = open(&mut manager, modal());
    let mut child_spec = detail(WindowKind::Fleet, 2);
    child_spec.owner = WindowOwner::Window(modal);
    let child = open(&mut manager, child_spec);
    let child_menu = open(&mut manager, menu(child));
    let frame = manager.begin_frame();
    assert_eq!(frame.owner(child_menu), Some(WindowOwner::Window(child)));
    assert!(frame.allows_input(child_menu));
    assert!(frame.allows_input(child));
    assert!(!frame.allows_input(unrelated));
    assert!(!frame.allows_input(background));
    assert_eq!(
        frame
            .pointer_route(&BTreeSet::from([unrelated, child_menu, background]))
            .target,
        target(child_menu)
    );
    assert_eq!(
        frame
            .pointer_route(&BTreeSet::from([unrelated, background]))
            .target,
        target(child_menu)
    );
    assert_eq!(frame.keyboard_route().bubble, BubblePolicy::StopAtWindow);
}

#[test]
fn capture_routes_motion_outside_owner_bounds() {
    // Source: FUN_00601080/FUN_00601090 title drag capture; off-rectangle hit is irrelevant.
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(WindowKind::System, 1));
    let other = open(&mut manager, detail(WindowKind::Fleet, 2));
    manager.apply(WindowCommand::CapturePointer(owner)).unwrap();
    let old = manager.begin_frame();
    assert_eq!(old.pointer_route(&BTreeSet::new()).target, target(owner));
    assert_eq!(
        old.pointer_route(&BTreeSet::from([other])).target,
        target(owner)
    );
    assert!(!old.pointer_allows_root());
    manager.apply(WindowCommand::ReleasePointer(owner)).unwrap();
    assert_eq!(old.pointer_route(&BTreeSet::new()).target, target(owner));
    assert_eq!(
        manager
            .begin_frame()
            .pointer_route(&BTreeSet::from([other]))
            .target,
        target(other)
    );
}

#[test]
fn escape_close_keeps_the_old_modal_keyboard_lease_until_the_next_snapshot() {
    // Source: FUN_00606960 posts close; FUN_00422ce0 restores on a later dispatch.
    let mut manager = WindowManager::new();
    let detail = open(&mut manager, detail(WindowKind::System, 1));
    let modal = open(&mut manager, modal());
    let old = manager.begin_frame();
    manager.apply(WindowCommand::Close(modal)).unwrap();
    assert_eq!(
        old.keyboard_route(),
        KeyboardRoute {
            target: target(modal),
            bubble: BubblePolicy::StopAtWindow
        }
    );
    assert_eq!(
        manager.begin_frame().keyboard_route(),
        KeyboardRoute {
            target: target(detail),
            bubble: BubblePolicy::ToRoot
        }
    );
}

#[test]
fn outside_release_that_closes_a_modal_cannot_activate_the_exposed_cockpit() {
    // Port: the old dispatch lease remains a blocker after its modal closes.
    // Physical press/release admission belongs to SW-05B, not this pure route contract.
    let mut manager = WindowManager::new();
    let modal = open(&mut manager, modal());
    let old = manager.begin_frame();
    manager.apply(WindowCommand::Close(modal)).unwrap();
    assert_eq!(
        old.pointer_route(&BTreeSet::new()).target,
        RouteTarget::BlockedBy(modal)
    );
    assert!(!old.pointer_allows_root());
    assert_eq!(
        manager.begin_frame().pointer_route(&BTreeSet::new()).target,
        RouteTarget::StrategicRoot
    );
}

#[test]
fn handled_text_stays_with_the_control_and_unhandled_keys_stop_at_a_strategic_modal() {
    // Source: FUN_00461590/FUN_00463360 and FUN_00606940 child-to-owner forwarding.
    // This kernel expresses ownership and propagation; SW-05B applies Handled/Unhandled.
    let mut manager = WindowManager::new();
    let modal = open(&mut manager, modal());
    let control = FocusTarget {
        window: modal,
        control: Some(ControlId(17)),
    };
    manager.apply(WindowCommand::Focus(control)).unwrap();
    let route = manager.begin_frame().keyboard_route();
    assert_eq!(
        route,
        KeyboardRoute {
            target: RouteTarget::Window(control),
            bubble: BubblePolicy::StopAtWindow
        }
    );
    manager.apply(WindowCommand::Close(modal)).unwrap();
    assert_eq!(route.target, RouteTarget::Window(control));
    assert_eq!(route.bubble, BubblePolicy::StopAtWindow);
}

#[test]
fn dismissible_menu_blocks_outside_press_until_explicit_close() {
    // Source: FUN_00442380/FUN_00442430 menu family owns dismissal/focus restoration.
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(WindowKind::System, 1));
    let menu = open(&mut manager, menu(owner));
    let old = manager.begin_frame();
    assert_eq!(old.pointer_route(&BTreeSet::new()).target, target(menu));
    assert_eq!(
        old.pointer_route(&BTreeSet::from([owner])).target,
        target(menu)
    );
    assert_eq!(
        old.pointer_route(&BTreeSet::from([menu])).target,
        target(menu)
    );
    assert!(!old.pointer_allows_root());
    assert_eq!(
        old.keyboard_route(),
        KeyboardRoute {
            target: target(menu),
            bubble: BubblePolicy::StopAtWindow
        }
    );
    manager.apply(WindowCommand::Close(menu)).unwrap();
    assert_eq!(
        old.pointer_route(&BTreeSet::from([owner])).target,
        target(menu)
    );
    assert_eq!(
        manager
            .begin_frame()
            .pointer_route(&BTreeSet::from([owner]))
            .target,
        target(owner)
    );
    assert!(manager.begin_frame().pointer_allows_root());
}

#[test]
fn live_unrelated_menu_cannot_pierce_a_modal_in_the_transient_band() {
    // Port: modality follows actual ownership even when two surfaces share a paint band.
    // Break caught: granting an unrelated dismissible menu an outside route under a modal.
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(WindowKind::System, 1));
    let unrelated_menu = open(&mut manager, menu(owner));
    let mut modal_spec = modal();
    modal_spec.owner = WindowOwner::Window(owner);
    modal_spec.policy.band = StackBand::Transient;
    modal_spec.policy.placement = OpenPlacement::AboveOwner;
    let modal = open(&mut manager, modal_spec);
    assert_eq!(manager.validate(), Ok(()));
    let frame = manager.begin_frame();
    assert_eq!(frame.paint_order(), &[owner, unrelated_menu, modal]);
    assert!(frame.is_visible(unrelated_menu));
    assert!(!frame.allows_input(unrelated_menu));
    assert_eq!(
        frame
            .pointer_route(&BTreeSet::from([unrelated_menu]))
            .target,
        RouteTarget::BlockedBy(modal)
    );
    assert_eq!(
        frame.pointer_route(&BTreeSet::new()).target,
        RouteTarget::BlockedBy(modal)
    );
    assert_eq!(
        frame
            .pointer_route(&BTreeSet::from([modal, unrelated_menu]))
            .target,
        target(modal)
    );
    assert!(!frame.pointer_allows_root());
}

#[test]
fn modeless_control_bubbles_to_root_until_it_has_a_menu_ancestor() {
    // FUN_00606940 child forwarding plus the port's transient propagation boundary.
    // Break caught: dropping the owner walk when the focused child itself is modeless.
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(WindowKind::System, 1));
    let root_control = FocusTarget {
        window: owner,
        control: Some(ControlId(41)),
    };
    manager.apply(WindowCommand::Focus(root_control)).unwrap();
    let root_frame = manager.begin_frame();
    assert_eq!(
        root_frame.keyboard_route(),
        KeyboardRoute {
            target: RouteTarget::Window(root_control),
            bubble: BubblePolicy::ToRoot
        }
    );
    let menu = open(&mut manager, menu(owner));
    let mut child_spec = detail(WindowKind::Fleet, 2);
    child_spec.owner = WindowOwner::Window(menu);
    child_spec.policy.band = StackBand::Transient;
    child_spec.policy.placement = OpenPlacement::AboveOwner;
    let child = open(&mut manager, child_spec);
    let control = FocusTarget {
        window: child,
        control: Some(ControlId(42)),
    };
    manager.apply(WindowCommand::Focus(control)).unwrap();
    assert_eq!(manager.validate(), Ok(()));
    assert_eq!(
        manager.begin_frame().keyboard_route(),
        KeyboardRoute {
            target: RouteTarget::Window(control),
            bubble: BubblePolicy::StopAtWindow
        }
    );
    assert_eq!(root_frame.keyboard_route().bubble, BubblePolicy::ToRoot);
}
