use std::collections::BTreeSet;

use rebellion_windowing::{
    CaptureCancelReason, Cardinality, CloseReason, ContentReport, ContentStatus,
    ExistingOpenPolicy, FocusOnOpen, FocusRestoration, FocusTarget, InputPolicy, OpenPlacement,
    PresentationPolicy, RouteTarget, StackBand, SubjectId, WindowCommand, WindowEvent, WindowId,
    WindowKey, WindowKind, WindowManager, WindowOwner, WindowPolicy, WindowSpec, WindowTransition,
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

fn open(manager: &mut WindowManager, spec: WindowSpec) -> WindowId {
    match manager.apply(WindowCommand::Open(spec)).unwrap().transition {
        WindowTransition::Opened { id } => id,
        other => panic!("expected fresh identity, got {other:?}"),
    }
}

fn report(window: WindowId, status: ContentStatus) -> ContentReport {
    ContentReport { window, status }
}

fn focus(window: WindowId) -> Option<FocusTarget> {
    Some(FocusTarget {
        window,
        control: None,
    })
}

#[test]
fn reconciliation_paint_absence_is_not_content_absence() {
    // port: SW-04R and source-plan "Content Reporting and Error Recovery".
    // Break caught: deriving content deletion from missing paint or report receipts.
    let mut manager = WindowManager::new();
    let visible = open(&mut manager, detail(WindowKind::System, 1));
    let hidden = open(&mut manager, detail(WindowKind::Fleet, 2));
    manager.apply(WindowCommand::Minimize(hidden)).unwrap();
    let frame = manager.begin_frame();
    let before = format!("{manager:?}");
    let result = manager.reconcile(
        &frame,
        &[
            report(visible, ContentStatus::Unpaintable),
            report(hidden, ContentStatus::Suspended),
        ],
    );
    assert!(result.removed.is_empty());
    assert!(result.events.is_empty());
    assert!(result.invalid_reports.is_empty());
    assert!(result.blocking_unpainted.is_empty());
    assert_eq!(format!("{manager:?}"), before);
    let missing = manager.reconcile(&frame, &[]);
    assert_eq!(missing.invalid_reports, vec![visible, hidden]);
    assert!(missing.removed.is_empty());
    assert!(missing.events.is_empty());
    assert_eq!(manager.visible_order(), &[visible]);
    assert_eq!(manager.presentation_history(), &[hidden]);
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn reconciliation_new_open_survives_old_snapshot_reports() {
    // port: reports can only act on their frozen snapshot members, never a post-snapshot open.
    // Break caught: accepting an Absent report for a newly committed ID.
    let mut manager = WindowManager::new();
    let old_spec = detail(WindowKind::System, 1);
    let old = open(&mut manager, old_spec);
    let frame = manager.begin_frame();
    let new_spec = detail(WindowKind::Fleet, 2);
    let new = open(&mut manager, new_spec);
    let result = manager.reconcile(
        &frame,
        &[
            report(new, ContentStatus::Absent),
            report(old, ContentStatus::Absent),
        ],
    );
    assert_eq!(result.removed, vec![old]);
    assert_eq!(result.invalid_reports, vec![new]);
    assert_eq!(
        result.events,
        vec![WindowEvent::Closed {
            window: old,
            key: old_spec.key,
            reason: CloseReason::ContentRemoved,
        }]
    );
    assert_eq!(manager.id_for(old_spec.key), None);
    assert_eq!(manager.id_for(new_spec.key), Some(new));
    assert_eq!(manager.visible_order(), &[new]);
    assert_eq!(manager.focused(), focus(new));
    assert_eq!(open(&mut manager, detail(WindowKind::Missions, 3)).get(), 3);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn an_unpainted_strategic_modal_is_retained_as_a_fail_closed_blocker() {
    // Source: FUN_00422ce0 modal boundary; port: failed painting cannot dismiss it.
    // Break caught: deleting a missing/unpaintable modal or clearing its input boundary.
    let mut manager = WindowManager::new();
    let underlay = open(&mut manager, detail(WindowKind::System, 1));
    let dialog = open(&mut manager, modal());
    let frame = manager.begin_frame();
    let before = format!("{manager:?}");
    let missing = manager.reconcile(&frame, &[report(underlay, ContentStatus::Ready)]);
    assert_eq!(missing.blocking_unpainted, vec![dialog]);
    assert_eq!(missing.invalid_reports, vec![dialog]);
    assert!(missing.removed.is_empty());
    assert!(missing.events.is_empty());
    let unpaintable = manager.reconcile(
        &frame,
        &[
            report(dialog, ContentStatus::Unpaintable),
            report(underlay, ContentStatus::Ready),
        ],
    );
    assert_eq!(unpaintable.blocking_unpainted, vec![dialog]);
    assert!(unpaintable.invalid_reports.is_empty());
    assert!(unpaintable.removed.is_empty());
    assert!(unpaintable.events.is_empty());
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.begin_frame().active_modal(), Some(dialog));
    assert_eq!(
        manager
            .begin_frame()
            .pointer_route(&BTreeSet::from([underlay]))
            .target,
        RouteTarget::BlockedBy(dialog)
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn unpaintable_modeless_content_is_retained_without_implicit_close() {
    // port: unavailable art is retained content, not a lifecycle deletion request.
    // Break caught: using Unpaintable as Absent or cancelling an unrelated gesture.
    let mut manager = WindowManager::new();
    let id = open(&mut manager, detail(WindowKind::Fleet, 1));
    manager.apply(WindowCommand::CapturePointer(id)).unwrap();
    let frame = manager.begin_frame();
    let before = format!("{manager:?}");
    let result = manager.reconcile(&frame, &[report(id, ContentStatus::Unpaintable)]);
    assert!(result.removed.is_empty());
    assert!(result.blocking_unpainted.is_empty());
    assert!(result.invalid_reports.is_empty());
    assert!(result.events.is_empty());
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.capture_owner(), Some(id));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn explicitly_absent_modeless_content_closes_with_events() {
    // Sources: FUN_005ffb60 child-first ownership teardown and FUN_00422ce0 focus restoration.
    // port: SW-04R reuses close receipts for explicitly absent snapshot content, exactly once.
    // Break caught: direct registry subtraction, duplicate cleanup, or missing hidden/capture cleanup.
    let mut manager = WindowManager::new();
    let root_spec = detail(WindowKind::System, 1);
    let root = open(&mut manager, root_spec);
    let hidden_spec = WindowSpec {
        owner: WindowOwner::Window(root),
        ..detail(WindowKind::Fleet, 2)
    };
    let hidden = open(&mut manager, hidden_spec);
    manager.apply(WindowCommand::Minimize(hidden)).unwrap();
    let child_spec = WindowSpec {
        owner: WindowOwner::Window(root),
        ..detail(WindowKind::Defenses, 3)
    };
    let child = open(&mut manager, child_spec);
    let survivor = open(&mut manager, detail(WindowKind::Missions, 4));
    manager
        .apply(WindowCommand::Focus(focus(child).unwrap()))
        .unwrap();
    manager.apply(WindowCommand::CapturePointer(child)).unwrap();
    let frame = manager.begin_frame();
    let reports = [
        report(hidden, ContentStatus::Absent),
        report(root, ContentStatus::Absent),
        report(child, ContentStatus::Absent),
        report(survivor, ContentStatus::Ready),
    ];
    let result = manager.reconcile(&frame, &reports);
    assert_eq!(result.removed, vec![child, hidden, root]);
    assert!(result.invalid_reports.is_empty());
    assert!(result.blocking_unpainted.is_empty());
    assert_eq!(
        result.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: child,
                reason: CaptureCancelReason::OwnerClosed
            },
            WindowEvent::Closed {
                window: child,
                key: child_spec.key,
                reason: CloseReason::OwnerClosed
            },
            WindowEvent::Closed {
                window: hidden,
                key: hidden_spec.key,
                reason: CloseReason::OwnerClosed
            },
            WindowEvent::Closed {
                window: root,
                key: root_spec.key,
                reason: CloseReason::ContentRemoved
            },
            WindowEvent::FocusChanged {
                previous: focus(child),
                current: focus(survivor)
            },
        ]
    );
    assert_eq!(manager.visible_order(), &[survivor]);
    assert!(manager.presentation_history().is_empty());
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.focused(), focus(survivor));
    let again = manager.reconcile(&frame, &reports);
    assert_eq!(again.invalid_reports, vec![root, hidden, child]);
    assert!(again.removed.is_empty());
    assert!(again.events.is_empty());
    assert_eq!(frame.capture_owner(), Some(child));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn absent_modal_reports_blocker_until_explicit_close() {
    // port: an Absent content report cannot override a modal input boundary.
    // Break caught: interpreting modal absence as an implicit Close command.
    let mut manager = WindowManager::new();
    let dialog = open(&mut manager, modal());
    let frame = manager.begin_frame();
    let before = format!("{manager:?}");
    let result = manager.reconcile(&frame, &[report(dialog, ContentStatus::Absent)]);
    assert_eq!(result.blocking_unpainted, vec![dialog]);
    assert!(result.invalid_reports.is_empty());
    assert!(result.removed.is_empty());
    assert!(result.events.is_empty());
    assert_eq!(format!("{manager:?}"), before);
    manager.apply(WindowCommand::Close(dialog)).unwrap();
    let late = manager.reconcile(&frame, &[report(dialog, ContentStatus::Absent)]);
    assert_eq!(late.invalid_reports, vec![dialog]);
    assert!(late.blocking_unpainted.is_empty());
    assert!(late.events.is_empty());
    assert!(manager.begin_frame().pointer_allows_root());
}

#[test]
fn suspended_content_is_retained_without_claiming_it_painted() {
    // port: scene membership belongs to SW-04S; Suspended alone cannot change registry/input state.
    // Break caught: deleting or hiding content from a temporary scene report.
    let mut manager = WindowManager::new();
    let id = open(&mut manager, detail(WindowKind::System, 1));
    let dialog = open(&mut manager, modal());
    let frame = manager.begin_frame();
    let before = format!("{manager:?}");
    let result = manager.reconcile(
        &frame,
        &[
            report(id, ContentStatus::Suspended),
            report(dialog, ContentStatus::Suspended),
        ],
    );
    assert!(result.removed.is_empty());
    assert!(result.invalid_reports.is_empty());
    assert!(result.blocking_unpainted.is_empty());
    assert!(result.events.is_empty());
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.begin_frame().active_modal(), Some(dialog));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn reconcile_never_removes_windows_created_after_its_snapshot() {
    // port: a new owned child is as protected as an unrelated post-snapshot open.
    // Break caught: an old owner's valid absence cascading into a newly opened child.
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(WindowKind::System, 1));
    let frame = manager.begin_frame();
    let child_spec = WindowSpec {
        owner: WindowOwner::Window(owner),
        ..detail(WindowKind::Fleet, 2)
    };
    let child = open(&mut manager, child_spec);
    manager.apply(WindowCommand::CapturePointer(child)).unwrap();
    let before = format!("{manager:?}");
    let result = manager.reconcile(&frame, &[report(owner, ContentStatus::Absent)]);
    assert_eq!(result.invalid_reports, vec![owner]);
    assert!(result.removed.is_empty());
    assert!(result.events.is_empty());
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.id_for(child_spec.key), Some(child));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn contradictory_or_unknown_content_reports_are_diagnosed() {
    // port: duplicate (including identical), unknown and post-snapshot IDs cannot authorize removal.
    // Break caught: last-report-wins accounting or ignoring stale/late diagnostics.
    let mut manager = WindowManager::new();
    let a = open(&mut manager, detail(WindowKind::System, 1));
    let b = open(&mut manager, detail(WindowKind::Fleet, 2));
    let stale = open(&mut manager, detail(WindowKind::Defenses, 3));
    manager.apply(WindowCommand::Close(stale)).unwrap();
    let frame = manager.begin_frame();
    let late = open(&mut manager, detail(WindowKind::Missions, 4));
    manager.apply(WindowCommand::CapturePointer(a)).unwrap();
    let before = format!("{manager:?}");
    let result = manager.reconcile(
        &frame,
        &[
            report(a, ContentStatus::Absent),
            report(b, ContentStatus::Absent),
            report(stale, ContentStatus::Absent),
            report(a, ContentStatus::Ready),
            report(b, ContentStatus::Absent),
            report(late, ContentStatus::Absent),
        ],
    );
    assert_eq!(result.invalid_reports, vec![a, b, stale, late]);
    assert!(result.removed.is_empty());
    assert!(result.blocking_unpainted.is_empty());
    assert!(result.events.is_empty());
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(open(&mut manager, detail(WindowKind::System, 5)).get(), 5);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn foreign_snapshot_cannot_delete_an_equal_numeric_identity() {
    // port: source-plan foreign-session diagnostics must hold even when allocators share ordinals.
    // Break caught: accepting a different manager's snapshot solely by numeric ID/key equality.
    let mut manager = WindowManager::new();
    let spec = detail(WindowKind::System, 1);
    let local = open(&mut manager, spec);
    let mut foreign = WindowManager::new();
    let other = open(&mut foreign, spec);
    assert_eq!(local.get(), other.get());
    let frame = foreign.begin_frame();
    drop(foreign);
    let before = format!("{manager:?}");
    let result = manager.reconcile(&frame, &[report(other, ContentStatus::Absent)]);
    assert_eq!(result.invalid_reports, vec![other]);
    assert!(result.removed.is_empty());
    assert!(result.events.is_empty());
    assert_eq!(format!("{manager:?}"), before);
    let local_frame = manager.begin_frame().clone();
    assert_eq!(
        manager
            .reconcile(&local_frame, &[report(local, ContentStatus::Absent)])
            .removed,
        vec![local]
    );
}

#[test]
fn retained_children_prevent_an_absent_owner_from_discarding_live_content() {
    // port: Unpaintable/Suspended/Ready records retain content; ownership cannot bypass that guard.
    // Break caught: a teardown cascade removing retained or modal descendants from reconciliation.
    for status in [
        ContentStatus::Ready,
        ContentStatus::Unpaintable,
        ContentStatus::Suspended,
    ] {
        let mut manager = WindowManager::new();
        let owner = open(&mut manager, detail(WindowKind::System, 1));
        let child = open(
            &mut manager,
            WindowSpec {
                owner: WindowOwner::Window(owner),
                ..detail(WindowKind::Fleet, 2)
            },
        );
        let frame = manager.begin_frame();
        let before = format!("{manager:?}");
        let result = manager.reconcile(
            &frame,
            &[report(owner, ContentStatus::Absent), report(child, status)],
        );
        assert_eq!(result.invalid_reports, vec![owner]);
        assert!(result.removed.is_empty());
        assert!(result.events.is_empty());
        assert_eq!(format!("{manager:?}"), before);
        assert_eq!(manager.validate(), Ok(()));
    }
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(WindowKind::System, 1));
    let dialog = open(
        &mut manager,
        WindowSpec {
            owner: WindowOwner::Window(owner),
            ..modal()
        },
    );
    let frame = manager.begin_frame();
    let before = format!("{manager:?}");
    let result = manager.reconcile(
        &frame,
        &[
            report(owner, ContentStatus::Absent),
            report(dialog, ContentStatus::Absent),
        ],
    );
    assert_eq!(result.invalid_reports, vec![owner]);
    assert_eq!(result.blocking_unpainted, vec![dialog]);
    assert!(result.removed.is_empty());
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn same_key_replacement_survives_a_late_report_for_the_old_identity() {
    // port: source identity, not semantic-key rebinding, scopes the report after Replace.
    // Break caught: removing the freshly allocated replacement by looking up the old key.
    let mut manager = WindowManager::new();
    let spec = detail(WindowKind::System, 1);
    let old = open(&mut manager, spec);
    let frame = manager.begin_frame();
    let fresh = match manager
        .apply(WindowCommand::Replace { old, new: spec })
        .unwrap()
        .transition
    {
        WindowTransition::Replaced { new, .. } => new,
        other => panic!("expected replacement, got {other:?}"),
    };
    let before = format!("{manager:?}");
    let result = manager.reconcile(&frame, &[report(old, ContentStatus::Absent)]);
    assert_eq!(result.invalid_reports, vec![old]);
    assert!(result.removed.is_empty());
    assert!(result.events.is_empty());
    assert_eq!(manager.id_for(spec.key), Some(fresh));
    assert_eq!(format!("{manager:?}"), before);
}

#[test]
fn multiple_absent_roots_deliver_cancellation_teardown_then_only_final_focus() {
    // port: reconciliation receipts have one final focus target and no duplicate teardown.
    // Break caught: concatenating per-close focus receipts or cancelling after earlier teardown.
    let mut manager = WindowManager::new();
    let a_spec = detail(WindowKind::System, 1);
    let a = open(&mut manager, a_spec);
    let b_spec = detail(WindowKind::Fleet, 2);
    let b = open(&mut manager, b_spec);
    let survivor = open(&mut manager, detail(WindowKind::Defenses, 3));
    manager
        .apply(WindowCommand::Focus(focus(a).unwrap()))
        .unwrap();
    manager.apply(WindowCommand::CapturePointer(a)).unwrap();
    let frame = manager.begin_frame();
    let result = manager.reconcile(
        &frame,
        &[
            report(a, ContentStatus::Absent),
            report(b, ContentStatus::Absent),
            report(survivor, ContentStatus::Ready),
        ],
    );
    assert_eq!(result.removed, vec![b, a]);
    assert!(result.invalid_reports.is_empty());
    assert_eq!(
        result.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: a,
                reason: CaptureCancelReason::OwnerClosed
            },
            WindowEvent::Closed {
                window: b,
                key: b_spec.key,
                reason: CloseReason::ContentRemoved
            },
            WindowEvent::Closed {
                window: a,
                key: a_spec.key,
                reason: CloseReason::ContentRemoved
            },
            WindowEvent::FocusChanged {
                previous: focus(a),
                current: focus(survivor)
            },
        ]
    );
    assert_eq!(manager.visible_order(), &[survivor]);
    assert_eq!(manager.focused(), focus(survivor));
    assert_eq!(manager.validate(), Ok(()));
}
