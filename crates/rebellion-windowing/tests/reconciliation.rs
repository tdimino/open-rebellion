mod common;

use std::collections::BTreeSet;

use common::{
    closed_event, focus_changed_event, open, root_detail as detail, window_focus as focus_target,
};

use rebellion_windowing::{
    CaptureCancelReason, CloseReason, ContentReport, ContentStatus, ReconcileReport, RouteTarget,
    WindowCommand, WindowEvent, WindowId, WindowKind, WindowManager, WindowOwner, WindowSpec,
    WindowTransition,
};

fn modal() -> WindowSpec {
    common::modal(WindowKind::Encyclopedia)
}

fn report(window: WindowId, status: ContentStatus) -> ContentReport {
    ContentReport { window, status }
}

fn assert_reconciliation_rejected(
    manager: &WindowManager,
    result: &ReconcileReport,
    before: &str,
    invalid: &[WindowId],
) {
    assert_inert_reconciliation(result, invalid, &[]);
    assert_eq!(format!("{manager:?}"), before);
}

fn assert_inert_reconciliation(
    result: &ReconcileReport,
    invalid: &[WindowId],
    blocking_unpainted: &[WindowId],
) {
    assert_eq!(result.invalid_reports, invalid);
    assert_eq!(result.blocking_unpainted, blocking_unpainted);
    assert_eq!(result.removed, Vec::new());
    assert_eq!(result.events, Vec::new());
}

#[test]
fn reconciliation_paint_absence_is_not_content_absence() {
    // port: source-plan "Content Reporting and Error Recovery".
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
    assert_inert_reconciliation(&result, &[], &[]);
    assert_eq!(format!("{manager:?}"), before);
    let missing = manager.reconcile(&frame, &[]);
    assert_inert_reconciliation(&missing, &[visible, hidden], &[]);
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
        vec![closed_event(old, old_spec.key, CloseReason::ContentRemoved,)]
    );
    assert_eq!(manager.id_for(old_spec.key), None);
    assert_eq!(manager.id_for(new_spec.key), Some(new));
    assert_eq!(manager.visible_order(), &[new]);
    assert_eq!(manager.focused(), Some(focus_target(new)));
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
    assert_inert_reconciliation(&missing, &[dialog], &[dialog]);
    let unpaintable = manager.reconcile(
        &frame,
        &[
            report(dialog, ContentStatus::Unpaintable),
            report(underlay, ContentStatus::Ready),
        ],
    );
    assert_inert_reconciliation(&unpaintable, &[], &[dialog]);
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
    assert_inert_reconciliation(&result, &[], &[]);
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.capture_owner(), Some(id));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn explicitly_absent_modeless_content_closes_with_events() {
    // Sources: FUN_005ffb60 child-first ownership teardown and FUN_00422ce0 focus restoration.
    // port: reuse close receipts for explicitly absent snapshot content, exactly once.
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
        .apply(WindowCommand::Focus(focus_target(child)))
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
    assert_eq!(result.removed, vec![root, hidden, child]);
    assert_eq!(result.invalid_reports, Vec::new());
    assert_eq!(result.blocking_unpainted, Vec::new());
    assert_eq!(
        result.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: child,
                reason: CaptureCancelReason::OwnerClosed
            },
            closed_event(child, child_spec.key, CloseReason::OwnerClosed),
            closed_event(hidden, hidden_spec.key, CloseReason::OwnerClosed),
            closed_event(root, root_spec.key, CloseReason::ContentRemoved),
            focus_changed_event(Some(focus_target(child)), Some(focus_target(survivor)),),
        ]
    );
    assert_eq!(manager.visible_order(), &[survivor]);
    assert_eq!(manager.presentation_history(), []);
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.focused(), Some(focus_target(survivor)));
    let again = manager.reconcile(&frame, &reports);
    assert_eq!(again.invalid_reports, vec![root, hidden, child]);
    assert_eq!(again.removed, Vec::new());
    assert_eq!(again.events, Vec::new());
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
    assert_inert_reconciliation(&result, &[], &[dialog]);
    assert_eq!(format!("{manager:?}"), before);
    manager.apply(WindowCommand::Close(dialog)).unwrap();
    let late = manager.reconcile(&frame, &[report(dialog, ContentStatus::Absent)]);
    assert_inert_reconciliation(&late, &[dialog], &[]);
    assert!(manager.begin_frame().pointer_allows_root());
}

#[test]
fn suspended_content_is_retained_without_claiming_it_painted() {
    // port: scene membership is separate; Suspended alone cannot change registry/input state.
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
    assert_inert_reconciliation(&result, &[], &[]);
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
    assert_reconciliation_rejected(&manager, &result, &before, &[owner]);
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
    assert_inert_reconciliation(&result, &[a, b, stale, late], &[]);
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
    assert_inert_reconciliation(&result, &[other], &[]);
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
        assert_reconciliation_rejected(&manager, &result, &before, &[owner]);
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
    assert_inert_reconciliation(&result, &[owner], &[dialog]);
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
    assert_inert_reconciliation(&result, &[old], &[]);
    assert_eq!(manager.id_for(spec.key), Some(fresh));
    assert_eq!(format!("{manager:?}"), before);
}

#[test]
fn multiple_absent_roots_deliver_cancellation_teardown_then_only_final_focus() {
    // port: reconciliation receipts have one final focus target and no duplicate teardown.
    // Break caught: coupling the stable removed summary to lifecycle event order, concatenating
    // per-close focus receipts, or cancelling after earlier teardown.
    let mut manager = WindowManager::new();
    let a_spec = detail(WindowKind::System, 1);
    let a = open(&mut manager, a_spec);
    let b_spec = detail(WindowKind::Fleet, 2);
    let b = open(&mut manager, b_spec);
    let survivor = open(&mut manager, detail(WindowKind::Defenses, 3));
    manager
        .apply(WindowCommand::Focus(focus_target(a)))
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
    assert_eq!(result.removed, vec![a, b]);
    assert_eq!(result.invalid_reports, Vec::new());
    assert_eq!(
        result.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: a,
                reason: CaptureCancelReason::OwnerClosed
            },
            closed_event(b, b_spec.key, CloseReason::ContentRemoved),
            closed_event(a, a_spec.key, CloseReason::ContentRemoved),
            focus_changed_event(Some(focus_target(a)), Some(focus_target(survivor))),
        ]
    );
    assert_eq!(manager.visible_order(), &[survivor]);
    assert_eq!(manager.focused(), Some(focus_target(survivor)));
    assert_eq!(manager.validate(), Ok(()));
}
