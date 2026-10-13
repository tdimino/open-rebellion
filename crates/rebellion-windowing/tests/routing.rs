use std::collections::BTreeSet;

use rebellion_windowing::{
    BubblePolicy, Cardinality, ContentReport, ContentStatus, ExistingOpenPolicy, FocusOnOpen,
    FocusRestoration, FocusTarget, InputPolicy, OpenPlacement, PresentationPolicy, RouteTarget,
    StackBand, SubjectId, WindowCommand, WindowError, WindowKind, WindowManager, WindowOwner,
    WindowPolicy, WindowSpec, WindowTransition,
};

fn policy(input: InputPolicy) -> WindowPolicy {
    WindowPolicy {
        cardinality: Cardinality::Subject { max_open: None },
        band: StackBand::Normal,
        placement: OpenPlacement::TopOfBand,
        existing: ExistingOpenPolicy::RaiseRestore,
        input,
        presentation: PresentationPolicy::History,
        focus_on_open: FocusOnOpen::Window,
        restoration: FocusRestoration::PreviousEligible,
    }
}

fn spec(kind: WindowKind, subject: u64, input: InputPolicy) -> WindowSpec {
    WindowSpec {
        key: rebellion_windowing::WindowKey::contextual(kind, SubjectId::new(2, subject)),
        owner: WindowOwner::StrategicRoot,
        policy: policy(input),
    }
}

fn menu_spec(owner: rebellion_windowing::WindowId) -> WindowSpec {
    WindowSpec {
        key: rebellion_windowing::WindowKey::singleton(WindowKind::Menu),
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

fn open(manager: &mut WindowManager, spec: WindowSpec) -> rebellion_windowing::WindowId {
    match manager.apply(WindowCommand::Open(spec)).unwrap().transition {
        WindowTransition::Opened { id } => id,
        other => panic!("expected a fresh window, got {other:?}"),
    }
}

fn hits(
    ids: impl IntoIterator<Item = rebellion_windowing::WindowId>,
) -> BTreeSet<rebellion_windowing::WindowId> {
    ids.into_iter().collect()
}

#[test]
fn topmost_eligible_hit_wins_independent_of_hit_registration_order() {
    let mut manager = WindowManager::new();
    let lower = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let upper = open(
        &mut manager,
        spec(WindowKind::Fleet, 2, InputPolicy::Modeless),
    );
    let frame = manager.begin_frame();
    for registered in [hits([lower, upper]), hits([upper, lower])] {
        assert_eq!(
            frame.pointer_route(&registered).target,
            RouteTarget::Window(FocusTarget {
                window: upper,
                control: None
            })
        );
    }
}

#[test]
fn modeless_eligibility_does_not_grant_all_windows_the_same_event() {
    let mut manager = WindowManager::new();
    let lower = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let upper = open(
        &mut manager,
        spec(WindowKind::Fleet, 2, InputPolicy::Modeless),
    );
    let frame = manager.begin_frame();
    assert!(frame.allows_input(lower));
    assert!(frame.allows_input(upper));
    assert_eq!(
        frame.pointer_route(&hits([lower, upper])).target,
        RouteTarget::Window(FocusTarget {
            window: upper,
            control: None
        })
    );
}

#[test]
fn modal_blocks_root_inside_and_outside_its_hit_regions() {
    let mut manager = WindowManager::new();
    let background = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let modal = open(
        &mut manager,
        spec(WindowKind::Status, 2, InputPolicy::StrategicModal),
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
    let background = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let unrelated = open(&mut manager, menu_spec(background));
    manager.apply(WindowCommand::Close(unrelated)).unwrap();
    let modal = open(
        &mut manager,
        spec(WindowKind::Status, 2, InputPolicy::StrategicModal),
    );
    let menu = open(&mut manager, menu_spec(modal));
    let frame = manager.begin_frame();
    assert!(frame.allows_input(menu));
    assert!(!frame.allows_input(background));
    assert_eq!(
        frame.pointer_route(&hits([background, menu])).target,
        RouteTarget::Window(FocusTarget {
            window: menu,
            control: None
        })
    );
}

#[test]
fn capture_routes_motion_outside_owner_bounds() {
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    manager.apply(WindowCommand::CapturePointer(owner)).unwrap();
    assert_eq!(
        manager.begin_frame().pointer_route(&BTreeSet::new()).target,
        RouteTarget::Window(FocusTarget {
            window: owner,
            control: None
        })
    );
}

#[test]
fn escape_close_keeps_the_old_modal_keyboard_lease_until_the_next_snapshot() {
    let mut manager = WindowManager::new();
    let modal = open(
        &mut manager,
        spec(WindowKind::Status, 1, InputPolicy::StrategicModal),
    );
    let old = manager.begin_frame();
    manager.apply(WindowCommand::Close(modal)).unwrap();
    assert_eq!(
        old.keyboard_route().target,
        RouteTarget::Window(FocusTarget {
            window: modal,
            control: None
        })
    );
    assert_eq!(
        manager.begin_frame().keyboard_route().target,
        RouteTarget::StrategicRoot
    );
}

#[test]
fn outside_release_that_closes_a_modal_cannot_activate_the_exposed_cockpit() {
    let mut manager = WindowManager::new();
    let modal = open(
        &mut manager,
        spec(WindowKind::Status, 1, InputPolicy::StrategicModal),
    );
    let lease = manager.begin_frame();
    manager.apply(WindowCommand::Close(modal)).unwrap();
    assert_eq!(
        lease.pointer_route(&BTreeSet::new()).target,
        RouteTarget::BlockedBy(modal)
    );
}

#[test]
fn handled_text_stays_with_the_control_and_unhandled_keys_stop_at_a_strategic_modal() {
    let mut manager = WindowManager::new();
    let modal = open(
        &mut manager,
        spec(WindowKind::Status, 1, InputPolicy::StrategicModal),
    );
    let control = FocusTarget {
        window: modal,
        control: Some(rebellion_windowing::ControlId(7)),
    };
    manager.apply(WindowCommand::Focus(control)).unwrap();
    let route = manager.begin_frame().keyboard_route();
    assert_eq!(route.target, RouteTarget::Window(control));
    assert_eq!(route.bubble, BubblePolicy::StopAtWindow);
}

#[test]
fn dismissible_menu_blocks_outside_press_until_explicit_close() {
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let menu = open(&mut manager, menu_spec(owner));
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
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let menu = open(&mut manager, menu_spec(owner));
    assert_eq!(
        manager.begin_frame().pointer_route(&hits([owner])).target,
        RouteTarget::BlockedBy(menu)
    );
}

#[test]
fn an_unpainted_strategic_modal_is_retained_as_a_fail_closed_blocker() {
    let mut manager = WindowManager::new();
    let modal = open(
        &mut manager,
        spec(WindowKind::Status, 1, InputPolicy::StrategicModal),
    );
    let frame = manager.begin_frame();
    let report = manager.reconcile(
        &frame,
        &[ContentReport {
            window: modal,
            status: ContentStatus::Unpaintable,
        }],
    );
    assert_eq!(report.blocking_unpainted, vec![modal]);
    assert_eq!(manager.id_for(frame.key(modal).unwrap()), Some(modal));
}

#[test]
fn unpaintable_modeless_content_is_retained_without_implicit_close() {
    let mut manager = WindowManager::new();
    let window = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let frame = manager.begin_frame();
    let report = manager.reconcile(
        &frame,
        &[ContentReport {
            window,
            status: ContentStatus::Unpaintable,
        }],
    );
    assert!(report.removed.is_empty());
    assert_eq!(manager.id_for(frame.key(window).unwrap()), Some(window));
}

#[test]
fn explicitly_absent_modeless_content_closes_with_events() {
    let mut manager = WindowManager::new();
    let window = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let frame = manager.begin_frame();
    let report = manager.reconcile(
        &frame,
        &[ContentReport {
            window,
            status: ContentStatus::Absent,
        }],
    );
    assert_eq!(report.removed, vec![window]);
    assert!(!report.events.is_empty());
    assert_eq!(manager.id_for(frame.key(window).unwrap()), None);
}

#[test]
fn explicitly_absent_minimized_content_removes_its_history_entry() {
    let mut manager = WindowManager::new();
    let window = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    manager.apply(WindowCommand::Minimize(window)).unwrap();
    let frame = manager.begin_frame();
    assert_eq!(frame.presentation_history(), &[window]);
    let report = manager.reconcile(
        &frame,
        &[ContentReport {
            window,
            status: ContentStatus::Absent,
        }],
    );
    assert_eq!(report.removed, vec![window]);
    assert!(manager.begin_frame().presentation_history().is_empty());
}

#[test]
fn absent_modal_reports_blocker_until_explicit_close() {
    let mut manager = WindowManager::new();
    let modal = open(
        &mut manager,
        spec(WindowKind::Status, 1, InputPolicy::StrategicModal),
    );
    let frame = manager.begin_frame();
    let report = manager.reconcile(
        &frame,
        &[ContentReport {
            window: modal,
            status: ContentStatus::Absent,
        }],
    );
    assert_eq!(report.blocking_unpainted, vec![modal]);
    assert_eq!(manager.id_for(frame.key(modal).unwrap()), Some(modal));
}

#[test]
fn absent_modeless_owner_cannot_implicitly_close_ready_modal_child() {
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let mut child_spec = spec(WindowKind::Status, 2, InputPolicy::StrategicModal);
    child_spec.owner = WindowOwner::Window(owner);
    child_spec.policy.presentation = PresentationPolicy::None;
    let modal = open(&mut manager, child_spec);
    let frame = manager.begin_frame();
    let report = manager.reconcile(
        &frame,
        &[
            ContentReport {
                window: owner,
                status: ContentStatus::Absent,
            },
            ContentReport {
                window: modal,
                status: ContentStatus::Ready,
            },
        ],
    );
    assert_eq!(report.invalid_reports, vec![owner]);
    assert!(report.removed.is_empty());
    assert!(manager.begin_frame().is_visible(owner));
    assert!(manager.begin_frame().is_visible(modal));
}

#[test]
fn suspended_content_is_retained_without_claiming_it_painted() {
    let mut manager = WindowManager::new();
    let window = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let frame = manager.begin_frame();
    let report = manager.reconcile(
        &frame,
        &[ContentReport {
            window,
            status: ContentStatus::Suspended,
        }],
    );
    assert!(report.removed.is_empty());
    assert!(report.blocking_unpainted.is_empty());
}

#[test]
fn reconcile_never_removes_windows_created_after_its_snapshot() {
    let mut manager = WindowManager::new();
    let old = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let frame = manager.begin_frame();
    let new = open(
        &mut manager,
        spec(WindowKind::Fleet, 2, InputPolicy::Modeless),
    );
    let report = manager.reconcile(
        &frame,
        &[ContentReport {
            window: old,
            status: ContentStatus::Ready,
        }],
    );
    assert!(report.removed.is_empty());
    assert!(manager.begin_frame().is_visible(new));
}

#[test]
fn contradictory_or_unknown_content_reports_are_diagnosed() {
    let mut manager = WindowManager::new();
    let window = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let stale = {
        let mut other = WindowManager::new();
        open(
            &mut other,
            spec(WindowKind::Fleet, 9, InputPolicy::Modeless),
        )
    };
    // Advance the local allocator so the foreign ID cannot alias the live one.
    let second = open(
        &mut manager,
        spec(WindowKind::Fleet, 2, InputPolicy::Modeless),
    );
    manager.apply(WindowCommand::Close(second)).unwrap();
    let frame = manager.begin_frame();
    let report = manager.reconcile(
        &frame,
        &[
            ContentReport {
                window,
                status: ContentStatus::Ready,
            },
            ContentReport {
                window,
                status: ContentStatus::Absent,
            },
            ContentReport {
                window: stale,
                status: ContentStatus::Ready,
            },
        ],
    );
    assert!(report.invalid_reports.contains(&window));
    // A foreign allocator can produce the same numeric ID, so use the closed local ID
    // as the unambiguous unknown diagnostic as well.
    let report = manager.reconcile(
        &frame,
        &[
            ContentReport {
                window,
                status: ContentStatus::Ready,
            },
            ContentReport {
                window: second,
                status: ContentStatus::Ready,
            },
        ],
    );
    assert!(report.invalid_reports.contains(&second));
}

#[test]
fn foreign_snapshot_cannot_reconcile_an_aliasing_window_id() {
    let mut first = WindowManager::new();
    let foreign = open(
        &mut first,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let frame = first.begin_frame();

    let mut second = WindowManager::new();
    let local = open(
        &mut second,
        spec(WindowKind::Fleet, 2, InputPolicy::Modeless),
    );
    assert_eq!(foreign.get(), local.get());
    let report = second.reconcile(
        &frame,
        &[ContentReport {
            window: foreign,
            status: ContentStatus::Absent,
        }],
    );
    assert_eq!(report.invalid_reports, vec![foreign]);
    assert!(report.removed.is_empty());
    assert!(second.begin_frame().is_visible(local));
}

#[test]
fn scene_restriction_removes_inactive_modal_and_focus_from_route() {
    let mut manager = WindowManager::new();
    let background = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let modal = open(
        &mut manager,
        spec(WindowKind::Status, 2, InputPolicy::StrategicModal),
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
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let menu = open(&mut manager, menu_spec(owner));
    let restricted = manager.begin_frame().restrict_to(&hits([menu]));
    assert!(!restricted.is_visible(menu));
    assert!(restricted.pointer_allows_root());
}

#[test]
fn repeated_scene_restriction_cannot_reintroduce_an_inactive_identity() {
    let mut manager = WindowManager::new();
    let first = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let second = open(
        &mut manager,
        spec(WindowKind::Fleet, 2, InputPolicy::Modeless),
    );
    let restricted = manager
        .begin_frame()
        .restrict_to(&hits([first]))
        .restrict_to(&hits([second]));
    assert!(restricted.paint_order().is_empty());
    assert!(restricted.key(second).is_none());
}

#[test]
fn active_membership_changes_command_eligibility_and_snapshot_together() {
    let mut manager = WindowManager::new();
    let galaxy = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let tactical = open(
        &mut manager,
        spec(WindowKind::BattleAlert, 2, InputPolicy::StrategicModal),
    );
    manager.set_active_windows(&hits([galaxy])).unwrap();
    let frame = manager.begin_frame();
    assert!(frame.is_visible(galaxy));
    assert!(!frame.is_visible(tactical));
    assert_eq!(
        manager.apply(WindowCommand::Focus(FocusTarget {
            window: tactical,
            control: None
        })),
        Err(WindowError::IneligibleFocus(FocusTarget {
            window: tactical,
            control: None
        }))
    );
}

#[test]
fn inactive_modal_does_not_constrain_active_scene_open_or_root_route() {
    let mut manager = WindowManager::new();
    let modal = open(
        &mut manager,
        spec(WindowKind::BattleAlert, 1, InputPolicy::StrategicModal),
    );
    manager.set_active_windows(&BTreeSet::new()).unwrap();
    let active = open(
        &mut manager,
        spec(WindowKind::System, 2, InputPolicy::Modeless),
    );
    let frame = manager.begin_frame();
    assert!(!frame.is_visible(modal));
    assert!(frame.is_visible(active));
    assert!(frame.pointer_allows_root());
}

#[test]
fn reactivated_modal_repairs_order_and_cancels_background_capture() {
    let mut manager = WindowManager::new();
    let background = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let modal = open(
        &mut manager,
        spec(WindowKind::BattleAlert, 2, InputPolicy::StrategicModal),
    );
    manager.set_active_windows(&hits([background])).unwrap();
    manager
        .apply(WindowCommand::CapturePointer(background))
        .unwrap();

    // Opening while the older modal is inactive places this peer at the top.
    let later = open(
        &mut manager,
        spec(WindowKind::Fleet, 3, InputPolicy::Modeless),
    );
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
    let known = open(
        &mut manager,
        spec(WindowKind::System, 1, InputPolicy::Modeless),
    );
    let stale = open(
        &mut manager,
        spec(WindowKind::Fleet, 2, InputPolicy::Modeless),
    );
    manager.apply(WindowCommand::Close(stale)).unwrap();
    let before = manager.begin_frame().paint_order().to_vec();
    assert_eq!(
        manager.set_active_windows(&hits([known, stale])),
        Err(WindowError::UnknownWindow(stale))
    );
    assert_eq!(manager.begin_frame().paint_order(), before);
}
