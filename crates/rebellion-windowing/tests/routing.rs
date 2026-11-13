mod common;

use std::collections::BTreeSet;

use common::{control_focus, menu as menu_spec, open, system_and_fleet, window_focus};

use rebellion_windowing::{
    BubblePolicy, FocusOnOpen, InputPolicy, RouteTarget, SubjectId, WindowCommand, WindowError,
    WindowId, WindowKind, WindowManager, WindowOwner, WindowPolicy, WindowSpec,
};

fn policy(input: InputPolicy) -> WindowPolicy {
    WindowPolicy::restorable_history(input, FocusOnOpen::Window)
}

fn spec(kind: WindowKind, subject: u64, input: InputPolicy) -> WindowSpec {
    WindowSpec {
        key: rebellion_windowing::WindowKey::contextual(kind, SubjectId::new(2, subject)),
        owner: WindowOwner::StrategicRoot,
        policy: policy(input),
    }
}

fn open_routed(
    manager: &mut WindowManager,
    kind: WindowKind,
    subject: u64,
    input: InputPolicy,
) -> WindowId {
    open(manager, spec(kind, subject, input))
}

fn hits(
    ids: impl IntoIterator<Item = rebellion_windowing::WindowId>,
) -> BTreeSet<rebellion_windowing::WindowId> {
    ids.into_iter().collect()
}

fn open_pair(
    first: (WindowKind, u64, InputPolicy),
    second: (WindowKind, u64, InputPolicy),
) -> (WindowManager, WindowId, WindowId) {
    let mut manager = WindowManager::new();
    let first = open_routed(&mut manager, first.0, first.1, first.2);
    let second = open_routed(&mut manager, second.0, second.1, second.2);
    (manager, first, second)
}

fn owner_menu() -> (WindowManager, WindowId, WindowId) {
    let mut manager = WindowManager::new();
    let owner = open_routed(&mut manager, WindowKind::System, 1, InputPolicy::Modeless);
    let menu = open(&mut manager, menu_spec(owner));
    (manager, owner, menu)
}

fn strategic_modal() -> (WindowManager, WindowId) {
    let mut manager = WindowManager::new();
    let modal = open_routed(
        &mut manager,
        WindowKind::Status,
        1,
        InputPolicy::StrategicModal,
    );
    (manager, modal)
}

#[test]
fn topmost_eligible_hit_wins_independent_of_hit_registration_order() {
    let (manager, lower, upper) = system_and_fleet();
    let frame = manager.begin_frame();
    for registered in [hits([lower, upper]), hits([upper, lower])] {
        assert_eq!(
            frame.pointer_route(&registered).target,
            RouteTarget::Window(window_focus(upper))
        );
    }
}

#[test]
fn modeless_eligibility_does_not_grant_all_windows_the_same_event() {
    let (manager, lower, upper) = system_and_fleet();
    let frame = manager.begin_frame();
    assert!(frame.allows_input(lower));
    assert!(frame.allows_input(upper));
    assert_eq!(
        frame.pointer_route(&hits([lower, upper])).target,
        RouteTarget::Window(window_focus(upper))
    );
}

#[test]
fn modal_blocks_root_inside_and_outside_its_hit_regions() {
    let (manager, background, modal) = open_pair(
        (WindowKind::System, 1, InputPolicy::Modeless),
        (WindowKind::Status, 2, InputPolicy::StrategicModal),
    );
    let frame = manager.begin_frame();
    assert_eq!(
        frame.pointer_route(&BTreeSet::new()).target,
        RouteTarget::BlockedBy(modal)
    );
    assert_eq!(
        frame.pointer_route(&hits([background])).target,
        RouteTarget::BlockedBy(modal)
    );
    assert!(!frame.pointer_allows_root());
}

#[test]
fn modal_descendant_menu_is_eligible_but_unrelated_menu_is_not() {
    let mut manager = WindowManager::new();
    let background = open_routed(&mut manager, WindowKind::System, 1, InputPolicy::Modeless);
    let unrelated = open(&mut manager, menu_spec(background));
    manager.apply(WindowCommand::Close(unrelated)).unwrap();
    let modal = open_routed(
        &mut manager,
        WindowKind::Status,
        2,
        InputPolicy::StrategicModal,
    );
    let menu = open(&mut manager, menu_spec(modal));
    let frame = manager.begin_frame();
    assert!(frame.allows_input(menu));
    assert!(!frame.allows_input(background));
    assert_eq!(
        frame.pointer_route(&hits([background, menu])).target,
        RouteTarget::Window(window_focus(menu))
    );
}

#[test]
fn capture_routes_motion_outside_owner_bounds() {
    let mut manager = WindowManager::new();
    let owner = open_routed(&mut manager, WindowKind::System, 1, InputPolicy::Modeless);
    manager.apply(WindowCommand::CapturePointer(owner)).unwrap();
    assert_eq!(
        manager.begin_frame().pointer_route(&BTreeSet::new()).target,
        RouteTarget::Window(window_focus(owner))
    );
}

#[test]
fn escape_close_keeps_the_old_modal_keyboard_lease_until_the_next_snapshot() {
    let (mut manager, modal) = strategic_modal();
    let old = manager.begin_frame();
    manager.apply(WindowCommand::Close(modal)).unwrap();
    assert_eq!(
        old.keyboard_route().target,
        RouteTarget::Window(window_focus(modal))
    );
    assert_eq!(
        manager.begin_frame().keyboard_route().target,
        RouteTarget::StrategicRoot
    );
}

#[test]
fn outside_release_that_closes_a_modal_cannot_activate_the_exposed_cockpit() {
    let (mut manager, modal) = strategic_modal();
    let lease = manager.begin_frame();
    manager.apply(WindowCommand::Close(modal)).unwrap();
    assert_eq!(
        lease.pointer_route(&BTreeSet::new()).target,
        RouteTarget::BlockedBy(modal)
    );
}

#[test]
fn handled_text_stays_with_the_control_and_unhandled_keys_stop_at_a_strategic_modal() {
    let (mut manager, modal) = strategic_modal();
    let control = control_focus(modal, 7);
    manager.apply(WindowCommand::Focus(control)).unwrap();
    let route = manager.begin_frame().keyboard_route();
    assert_eq!(route.target, RouteTarget::Window(control));
    assert_eq!(route.bubble, BubblePolicy::StopAtWindow);
}

#[test]
fn dismissible_menu_blocks_outside_press_until_explicit_close() {
    let (mut manager, _owner, menu) = owner_menu();
    let old = manager.begin_frame();
    assert_eq!(
        old.pointer_route(&BTreeSet::new()).target,
        RouteTarget::BlockedBy(menu)
    );
    manager.apply(WindowCommand::Close(menu)).unwrap();
    assert_eq!(
        old.pointer_route(&BTreeSet::new()).target,
        RouteTarget::BlockedBy(menu)
    );
    assert!(manager.begin_frame().pointer_allows_root());
}

#[test]
fn dismissible_menu_blocks_press_on_its_underlying_owner() {
    let (manager, owner, menu) = owner_menu();
    assert_eq!(
        manager.begin_frame().pointer_route(&hits([owner])).target,
        RouteTarget::BlockedBy(menu)
    );
}

#[test]
fn scene_restriction_removes_inactive_modal_and_focus_from_route() {
    let (manager, background, modal) = open_pair(
        (WindowKind::System, 1, InputPolicy::Modeless),
        (WindowKind::Status, 2, InputPolicy::StrategicModal),
    );
    let restricted = manager.begin_frame().restrict_to(&hits([background]));
    assert_eq!(restricted.active_modal(), None);
    assert_ne!(
        restricted.focused().map(|target| target.window),
        Some(modal)
    );
    assert!(restricted.pointer_allows_root());
}

#[test]
fn scene_restriction_fails_closed_on_child_without_active_owner() {
    let (manager, _owner, menu) = owner_menu();
    let restricted = manager.begin_frame().restrict_to(&hits([menu]));
    assert!(!restricted.is_visible(menu));
    assert!(restricted.pointer_allows_root());
}

#[test]
fn repeated_scene_restriction_cannot_reintroduce_an_inactive_identity() {
    let mut manager = WindowManager::new();
    let first = open_routed(&mut manager, WindowKind::System, 1, InputPolicy::Modeless);
    let second = open_routed(&mut manager, WindowKind::Fleet, 2, InputPolicy::Modeless);
    let restricted = manager
        .begin_frame()
        .restrict_to(&hits([first]))
        .restrict_to(&hits([second]));
    assert_eq!(restricted.paint_order(), []);
    assert!(restricted.key(second).is_none());
}

#[test]
fn active_membership_changes_command_eligibility_and_snapshot_together() {
    let mut manager = WindowManager::new();
    let galaxy = open_routed(&mut manager, WindowKind::System, 1, InputPolicy::Modeless);
    let tactical = open_routed(
        &mut manager,
        WindowKind::BattleAlert,
        2,
        InputPolicy::StrategicModal,
    );
    manager.set_active_windows(&hits([galaxy])).unwrap();
    let frame = manager.begin_frame();
    assert!(frame.is_visible(galaxy));
    assert!(!frame.is_visible(tactical));
    assert_eq!(
        manager.apply(WindowCommand::Focus(window_focus(tactical))),
        Err(WindowError::IneligibleFocus(window_focus(tactical)))
    );
}

#[test]
fn inactive_modal_does_not_constrain_active_scene_open_or_root_route() {
    let mut manager = WindowManager::new();
    let modal = open_routed(
        &mut manager,
        WindowKind::BattleAlert,
        1,
        InputPolicy::StrategicModal,
    );
    manager.set_active_windows(&BTreeSet::new()).unwrap();
    let active = open_routed(&mut manager, WindowKind::System, 2, InputPolicy::Modeless);
    let frame = manager.begin_frame();
    assert!(!frame.is_visible(modal));
    assert!(frame.is_visible(active));
    assert!(frame.pointer_allows_root());
}

#[test]
fn reactivated_modal_repairs_order_and_cancels_background_capture() {
    let (mut manager, background, modal) = open_pair(
        (WindowKind::System, 1, InputPolicy::Modeless),
        (WindowKind::BattleAlert, 2, InputPolicy::StrategicModal),
    );
    manager.set_active_windows(&hits([background])).unwrap();
    manager
        .apply(WindowCommand::CapturePointer(background))
        .unwrap();

    // Opening while the older modal is inactive places this peer at the top.
    let later = open_routed(&mut manager, WindowKind::Fleet, 3, InputPolicy::Modeless);
    let events = manager
        .set_active_windows(&hits([background, modal, later]))
        .unwrap();
    assert!(events.iter().any(|event| matches!(
        event,
        rebellion_windowing::WindowEvent::CaptureCancelled {
            window,
            reason: rebellion_windowing::CaptureCancelReason::SceneChanged,
        } if *window == background
    )));
    assert_eq!(manager.capture_owner(), None);
    let frame = manager.begin_frame();
    assert_eq!(frame.paint_order().last(), Some(&modal));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn unknown_active_member_rejects_scene_update_without_mutation() {
    let mut manager = WindowManager::new();
    let known = open_routed(&mut manager, WindowKind::System, 1, InputPolicy::Modeless);
    let stale = open_routed(&mut manager, WindowKind::Fleet, 2, InputPolicy::Modeless);
    manager.apply(WindowCommand::Close(stale)).unwrap();
    let before = manager.begin_frame().paint_order().to_vec();
    assert_eq!(
        manager.set_active_windows(&hits([known, stale])),
        Err(WindowError::UnknownWindow(stale))
    );
    assert_eq!(manager.begin_frame().paint_order(), before);
}
