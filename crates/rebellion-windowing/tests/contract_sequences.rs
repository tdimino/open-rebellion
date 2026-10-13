use std::collections::BTreeSet;

use rebellion_windowing::{
    BubblePolicy, CaptureCancelReason, CloseReason, ContentReport, ContentStatus, ControlId,
    FocusTarget, RouteTarget, WindowCommand, WindowError, WindowEvent, WindowId, WindowKind,
    WindowManager, WindowOwner, WindowTransition,
};

fn detail(kind: WindowKind, subject: u64) -> rebellion_windowing::WindowSpec {
    use rebellion_windowing::{
        Cardinality, ExistingOpenPolicy, FocusOnOpen, FocusRestoration, InputPolicy, OpenPlacement,
        PresentationPolicy, StackBand, SubjectId, WindowKey, WindowPolicy, WindowSpec,
    };
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

fn modal() -> rebellion_windowing::WindowSpec {
    use rebellion_windowing::{
        Cardinality, ExistingOpenPolicy, InputPolicy, PresentationPolicy, WindowKey,
    };
    let mut spec = detail(WindowKind::Encyclopedia, 0);
    spec.key = WindowKey::singleton(WindowKind::Encyclopedia);
    spec.policy.cardinality = Cardinality::Singleton;
    spec.policy.existing = ExistingOpenPolicy::NoOp;
    spec.policy.input = InputPolicy::StrategicModal;
    spec.policy.presentation = PresentationPolicy::None;
    spec
}

fn menu(owner: WindowId) -> rebellion_windowing::WindowSpec {
    use rebellion_windowing::{FocusRestoration, InputPolicy, OpenPlacement, StackBand, WindowKey};
    let mut spec = modal();
    spec.key = WindowKey::singleton(WindowKind::Menu);
    spec.owner = WindowOwner::Window(owner);
    spec.policy.input = InputPolicy::DismissOnOutsidePress;
    spec.policy.band = StackBand::Transient;
    spec.policy.placement = OpenPlacement::AboveOwner;
    spec.policy.restoration = FocusRestoration::SavedMenuTarget;
    spec
}

fn checked(
    manager: &mut WindowManager,
    command: WindowCommand,
) -> rebellion_windowing::WindowCommit {
    let commit = manager
        .apply(command)
        .unwrap_or_else(|error| panic!("{command:?}: {error:?}"));
    assert_eq!(manager.validate(), Ok(()), "after {command:?}");
    commit
}

fn open(manager: &mut WindowManager, spec: rebellion_windowing::WindowSpec) -> WindowId {
    match checked(manager, WindowCommand::Open(spec)).transition {
        WindowTransition::Opened { id } => id,
        other => panic!("expected fresh allocation: {other:?}"),
    }
}

fn target(window: WindowId) -> RouteTarget {
    RouteTarget::Window(FocusTarget {
        window,
        control: None,
    })
}

fn reports(
    frame: &rebellion_windowing::RoutingSnapshot,
    ids: &[WindowId],
    absent: Option<WindowId>,
) -> Vec<ContentReport> {
    ids.iter()
        .copied()
        .filter(|id| frame.key(*id).is_some())
        .map(|window| ContentReport {
            window,
            status: if absent == Some(window) {
                ContentStatus::Absent
            } else {
                ContentStatus::Suspended
            },
        })
        .collect()
}

#[test]
fn contract_sequence_preserves_all_authorities() {
    // Port: deterministic fixture permutations compose the bounded SW-02R/03/04 contracts.
    // Sources: FUN_00428b40 history, FUN_00601080 capture, FUN_00606960 deferred close.
    for seed in 0..32 {
        let mut manager = WindowManager::new();
        let root_spec = detail(WindowKind::System, seed * 100);
        let peer_spec = detail(WindowKind::Defenses, seed * 100 + 1);
        let (root, peer) = if seed % 2 == 0 {
            let root = open(&mut manager, root_spec);
            (root, open(&mut manager, peer_spec))
        } else {
            let peer = open(&mut manager, peer_spec);
            (open(&mut manager, root_spec), peer)
        };
        let mut child_spec = detail(WindowKind::Fleet, seed * 100 + 2);
        child_spec.owner = WindowOwner::Window(root);
        let child = open(&mut manager, child_spec);
        let control = FocusTarget {
            window: root,
            control: Some(ControlId(seed + 1)),
        };
        checked(&mut manager, WindowCommand::Focus(control));
        checked(&mut manager, WindowCommand::CapturePointer(child));
        let captured = manager.begin_frame();
        let minimized = checked(&mut manager, WindowCommand::Minimize(child));
        assert_eq!(
            minimized.events[0],
            WindowEvent::CaptureCancelled {
                window: child,
                reason: CaptureCancelReason::Minimized,
            }
        );
        assert_eq!(manager.presentation_history(), &[child]);
        assert_eq!(
            captured.pointer_route(&BTreeSet::new()).target,
            target(child)
        );
        assert_eq!(captured.owner(child), Some(WindowOwner::Window(root)));
        checked(&mut manager, WindowCommand::Restore(child));
        assert!(manager.presentation_history().is_empty());
        checked(&mut manager, WindowCommand::Focus(control));
        let menu_id = open(&mut manager, menu(root));
        let menu_lease = manager.begin_frame();
        checked(&mut manager, WindowCommand::Close(menu_id));
        assert_eq!(manager.focused(), Some(control));
        assert_eq!(
            menu_lease.pointer_route(&BTreeSet::new()).target,
            target(menu_id)
        );
        assert_eq!(
            menu_lease.keyboard_route().bubble,
            BubblePolicy::StopAtWindow
        );

        checked(&mut manager, WindowCommand::CapturePointer(child));
        let modal_id = open(&mut manager, modal());
        assert_eq!(manager.capture_owner(), None);
        let modal_menu = open(&mut manager, menu(modal_id));
        checked(&mut manager, WindowCommand::CapturePointer(modal_menu));
        let modal_lease = manager.begin_frame();
        let scene_events = manager.set_active_windows(&BTreeSet::from([peer])).unwrap();
        assert_eq!(
            scene_events[0],
            WindowEvent::CaptureCancelled {
                window: modal_menu,
                reason: CaptureCancelReason::SceneChanged,
            }
        );
        assert_eq!(manager.validate(), Ok(()));
        assert_eq!(manager.begin_frame().paint_order(), &[peer]);
        assert_eq!(manager.begin_frame().active_modal(), None);
        checked(&mut manager, WindowCommand::FocusRoot);
        let fresh_spec = detail(WindowKind::System, seed * 100 + 3);
        let fresh = open(&mut manager, fresh_spec);
        let ids = [root, peer, child, modal_id, modal_menu, fresh];
        let frame = manager.begin_frame();
        let retained = manager.reconcile(&frame, &reports(&frame, &ids, None));
        assert!(retained.removed.is_empty());
        assert!(retained.invalid_reports.is_empty());
        assert!(retained.events.is_empty());
        assert!(retained.blocking_unpainted.is_empty());
        assert_eq!(manager.validate(), Ok(()));

        checked(
            &mut manager,
            WindowCommand::Focus(FocusTarget {
                window: peer,
                control: None,
            }),
        );
        checked(&mut manager, WindowCommand::CapturePointer(peer));
        let frame = manager.begin_frame();
        let absent = manager.reconcile(&frame, &reports(&frame, &ids, Some(peer)));
        assert_eq!(absent.removed, vec![peer]);
        assert!(absent.invalid_reports.is_empty());
        assert_eq!(
            absent.events[0],
            WindowEvent::CaptureCancelled {
                window: peer,
                reason: CaptureCancelReason::OwnerClosed,
            }
        );
        assert_eq!(
            absent.events[1],
            WindowEvent::Closed {
                window: peer,
                key: peer_spec.key,
                reason: CloseReason::ContentRemoved,
            }
        );
        assert_eq!(manager.validate(), Ok(()));
        assert_eq!(frame.pointer_route(&BTreeSet::new()).target, target(peer));

        let before_replace = manager.begin_frame();
        let replacement_spec = detail(WindowKind::System, seed * 100 + 4);
        let replaced = checked(
            &mut manager,
            WindowCommand::Replace {
                old: root,
                new: replacement_spec,
            },
        );
        let replacement = match replaced.transition {
            WindowTransition::Replaced { old, new } => {
                assert_eq!(old, root);
                new
            }
            other => panic!("expected replacement: {other:?}"),
        };
        let closed: Vec<_> = replaced
            .events
            .iter()
            .filter_map(|event| match event {
                WindowEvent::Closed { window, reason, .. } => {
                    // FUN_005ffb60 owner cascade; SW-02R's root alone has Replaced.
                    assert_eq!(
                        *reason,
                        if *window == root {
                            CloseReason::Replaced
                        } else {
                            CloseReason::OwnerClosed
                        }
                    );
                    Some(*window)
                }
                _ => None,
            })
            .collect();
        assert_eq!(closed, vec![child, root]);
        assert!(replacement.get() > fresh.get());
        assert_eq!(manager.id_for(root_spec.key), None);
        assert_eq!(manager.id_for(child_spec.key), None);
        let before = format!("{manager:?}");
        let mut late_reports = reports(&before_replace, &ids, Some(root));
        late_reports.push(ContentReport {
            window: replacement,
            status: ContentStatus::Absent,
        });
        let late = manager.reconcile(&before_replace, &late_reports);
        assert!(late.removed.is_empty());
        assert!(late.events.is_empty());
        assert_eq!(
            late.invalid_reports,
            vec![root.min(child), root.max(child), replacement]
        );
        assert_eq!(format!("{manager:?}"), before);
        assert_eq!(before_replace.key(root), Some(root_spec.key));
        assert_eq!(before_replace.owner(child), Some(WindowOwner::Window(root)));
        checked(&mut manager, WindowCommand::Close(modal_id));
        assert_eq!(modal_lease.capture_owner(), Some(modal_menu));
        assert_eq!(
            modal_lease.pointer_route(&BTreeSet::new()).target,
            target(modal_menu)
        );

        checked(&mut manager, WindowCommand::CapturePointer(replacement));
        let reset_lease = manager.begin_frame();
        let reset_events = manager.reset_session();
        assert_eq!(
            reset_events[0],
            WindowEvent::CaptureCancelled {
                window: replacement,
                reason: CaptureCancelReason::SessionReset,
            }
        );
        assert_eq!(manager.validate(), Ok(()));
        assert!(manager.begin_frame().paint_order().is_empty());
        let reopened = open(&mut manager, replacement_spec);
        assert!(reopened.get() > replacement.get());
        let before = format!("{manager:?}");
        for stale in [root, child, replacement, modal_id, modal_menu, peer] {
            assert_eq!(
                manager.apply(WindowCommand::Close(stale)),
                Err(WindowError::UnknownWindow(stale))
            );
            assert_eq!(format!("{manager:?}"), before);
        }
        let late = manager.reconcile(
            &reset_lease,
            &[
                ContentReport {
                    window: replacement,
                    status: ContentStatus::Absent,
                },
                ContentReport {
                    window: reopened,
                    status: ContentStatus::Absent,
                },
            ],
        );
        assert!(late.removed.is_empty());
        assert!(late.invalid_reports.contains(&reopened));
        assert_eq!(manager.id_for(replacement_spec.key), Some(reopened));
        assert_eq!(format!("{manager:?}"), before);
        assert_eq!(
            reset_lease.pointer_route(&BTreeSet::new()).target,
            target(replacement)
        );
        assert_eq!(manager.validate(), Ok(()));

        // FUN_00428b40: history evicts only the oldest minimized identity at the rail bound.
        let mut history = Vec::new();
        for offset in 0..14 {
            let id = open(
                &mut manager,
                detail(WindowKind::System, seed * 100 + 10 + offset),
            );
            checked(&mut manager, WindowCommand::Minimize(id));
            history.push(id);
            let start = history
                .len()
                .saturating_sub(WindowManager::PRESENTATION_HISTORY_CAPACITY);
            assert_eq!(manager.presentation_history(), &history[start..]);
        }
        assert_eq!(manager.begin_frame().key(history[0]), None);
        assert_eq!(manager.begin_frame().key(history[1]), None);
        assert_eq!(manager.id_for(replacement_spec.key), Some(reopened));
    }
}

#[test]
fn contract_rejected_commands_preserve_complete_state() {
    // Port: semantic failures must not publish partial teardown or consume allocation.
    let mut manager = WindowManager::new();
    let stale = open(&mut manager, detail(WindowKind::Status, 0));
    checked(&mut manager, WindowCommand::Close(stale));
    let root_spec = detail(WindowKind::System, 1);
    let root = open(&mut manager, root_spec);
    let peer = open(&mut manager, detail(WindowKind::Defenses, 2));
    let mut child_spec = detail(WindowKind::Fleet, 3);
    child_spec.owner = WindowOwner::Window(root);
    let child = open(&mut manager, child_spec);
    let hidden = open(&mut manager, detail(WindowKind::Missions, 4));
    checked(&mut manager, WindowCommand::Minimize(hidden));
    checked(
        &mut manager,
        WindowCommand::Focus(FocusTarget {
            window: root,
            control: Some(ControlId(99)),
        }),
    );
    checked(&mut manager, WindowCommand::CapturePointer(root));
    let mut removed_owner = detail(WindowKind::System, 10);
    removed_owner.owner = WindowOwner::Window(child);
    let before = format!("{manager:?}");
    for (command, error) in [
        (
            WindowCommand::Replace {
                old: root,
                new: removed_owner,
            },
            WindowError::InvalidReplacement(root),
        ),
        (
            WindowCommand::Close(stale),
            WindowError::UnknownWindow(stale),
        ),
        (
            WindowCommand::Raise(stale),
            WindowError::UnknownWindow(stale),
        ),
        (
            WindowCommand::Activate(stale),
            WindowError::UnknownWindow(stale),
        ),
        (
            WindowCommand::Minimize(stale),
            WindowError::UnknownWindow(stale),
        ),
        (
            WindowCommand::Restore(stale),
            WindowError::UnknownWindow(stale),
        ),
        (
            WindowCommand::Focus(FocusTarget {
                window: hidden,
                control: None,
            }),
            WindowError::IneligibleFocus(FocusTarget {
                window: hidden,
                control: None,
            }),
        ),
        (
            WindowCommand::CapturePointer(hidden),
            WindowError::IneligibleCapture(hidden),
        ),
        (
            WindowCommand::CapturePointer(peer),
            WindowError::CaptureOwnedBy(root),
        ),
        (
            WindowCommand::ReleasePointer(peer),
            WindowError::CaptureOwnedBy(root),
        ),
    ] {
        assert_eq!(manager.apply(command), Err(error), "{command:?}");
        assert_eq!(format!("{manager:?}"), before, "{command:?}");
        assert_eq!(manager.validate(), Ok(()));
    }
    assert_eq!(
        manager.set_active_windows(&BTreeSet::from([peer, stale])),
        Err(WindowError::UnknownWindow(stale))
    );
    assert_eq!(format!("{manager:?}"), before);
    let next = open(&mut manager, detail(WindowKind::System, 11));
    assert_eq!(next.get(), hidden.get() + 1);
    let modal_id = open(&mut manager, modal());
    let before = format!("{manager:?}");
    assert_eq!(
        manager.apply(WindowCommand::FocusRoot),
        Err(WindowError::RootBlockedBy(modal_id))
    );
    assert_eq!(format!("{manager:?}"), before);
    manager.set_active_windows(&BTreeSet::from([peer])).unwrap();
    let before = format!("{manager:?}");
    for command in [
        WindowCommand::Activate(root),
        WindowCommand::Raise(root),
        WindowCommand::Focus(FocusTarget {
            window: root,
            control: None,
        }),
    ] {
        assert_eq!(
            manager.apply(command),
            Err(WindowError::IneligibleFocus(FocusTarget {
                window: root,
                control: None
            }))
        );
        assert_eq!(format!("{manager:?}"), before);
    }
    assert_eq!(
        manager.apply(WindowCommand::CapturePointer(root)),
        Err(WindowError::IneligibleCapture(root))
    );
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn contract_dormant_normal_restore_preserves_scene_membership_and_focus() {
    // Port: retained ownership/order and active scene membership are independent.
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(WindowKind::System, 1));
    let mut spec = detail(WindowKind::Fleet, 2);
    spec.owner = WindowOwner::Window(owner);
    let child = open(&mut manager, spec);
    checked(&mut manager, WindowCommand::Minimize(child));
    let peer = open(&mut manager, detail(WindowKind::Defenses, 3));
    manager.set_active_windows(&BTreeSet::from([peer])).unwrap();
    let focus_before = manager.focused();
    let lease = manager.begin_frame();
    let commit = checked(&mut manager, WindowCommand::Restore(child));
    assert_eq!(commit.transition, WindowTransition::Restored { id: child });
    assert!(commit.events.is_empty());
    assert!(manager.presentation_history().is_empty());
    assert_eq!(manager.begin_frame().paint_order(), &[peer]);
    assert_eq!(manager.focused(), focus_before);
    assert_eq!(manager.id_for(spec.key), Some(child));
    assert_eq!(lease.paint_order(), &[peer]);
    manager
        .set_active_windows(&BTreeSet::from([owner, child, peer]))
        .unwrap();
    assert_eq!(manager.validate(), Ok(()));
    assert!(manager.begin_frame().paint_order().contains(&child));
}

#[test]
fn contract_transient_restore_rejects_a_dormant_owner_atomically() {
    // Port: modeless history may be transient; that placement needs an eligible owner.
    let mut manager = WindowManager::new();
    let owner = open(&mut manager, detail(WindowKind::System, 1));
    let mut spec = detail(WindowKind::Fleet, 2);
    spec.owner = WindowOwner::Window(owner);
    spec.policy.band = rebellion_windowing::StackBand::Transient;
    spec.policy.placement = rebellion_windowing::OpenPlacement::AboveOwner;
    let child = open(&mut manager, spec);
    checked(&mut manager, WindowCommand::Minimize(child));
    let peer = open(&mut manager, detail(WindowKind::Defenses, 3));
    manager.set_active_windows(&BTreeSet::from([peer])).unwrap();
    let before = format!("{manager:?}");
    assert_eq!(
        manager.apply(WindowCommand::Restore(child)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.validate(), Ok(()));
}
