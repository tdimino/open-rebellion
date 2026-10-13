use rebellion_windowing::{
    CaptureCancelReason, Cardinality, CloseReason, ControlId, ExistingOpenPolicy, FocusOnOpen,
    FocusRestoration, FocusTarget, InputPolicy, OpenPlacement, PresentationPolicy, StackBand,
    SubjectId, TransitionSubject, UnchangedReason, WindowCommand, WindowError, WindowEvent,
    WindowId, WindowKey, WindowKind, WindowManager, WindowOwner, WindowPolicy, WindowSpec,
    WindowTransition,
};

fn modeless_spec(kind: WindowKind, subject: u64, owner: WindowOwner) -> WindowSpec {
    WindowSpec {
        key: WindowKey::contextual(kind, SubjectId::new(2, subject)),
        owner,
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

fn menu_spec(owner: WindowId) -> WindowSpec {
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

fn modal_spec(kind: WindowKind) -> WindowSpec {
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

#[test]
fn only_menu_may_request_saved_focus_restoration() {
    // Source: strategic-windowing-toolkit.md, "Native focus"; the generic saved-focus
    // field is dormant and only the menu family has an explicit saved target.
    // Break caught: granting ordinary windows a private saved-focus stack.
    let mut manager = WindowManager::new();
    let mut invalid = modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot);
    invalid.policy.restoration = FocusRestoration::SavedMenuTarget;

    assert_eq!(
        manager.apply(WindowCommand::Open(invalid)),
        Err(WindowError::KindPolicyMismatch(WindowKind::System))
    );
    let valid = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    assert_eq!(valid.get(), 1, "rejection must not consume an identity");
    assert_eq!(manager.validate(), Ok(()));
}

fn opened_id(transition: WindowTransition) -> WindowId {
    match transition {
        WindowTransition::Opened { id } => id,
        other => panic!("expected a new window, got {other:?}"),
    }
}

fn open(manager: &mut WindowManager, spec: WindowSpec) -> WindowId {
    opened_id(
        manager
            .apply(WindowCommand::Open(spec))
            .expect("window opens")
            .transition,
    )
}

#[test]
fn minimize_removes_visible_order_and_adds_history_once() {
    // Source: FUN_00428b40 presentation rail; port history contains hidden details only.
    let mut manager = WindowManager::new();
    let first = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let second = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    let commit = manager.apply(WindowCommand::Minimize(second)).unwrap();
    assert_eq!(
        commit.transition,
        WindowTransition::Minimized {
            id: second,
            evicted: vec![]
        }
    );
    assert_eq!(manager.visible_order(), &[first]);
    assert_eq!(manager.presentation_history(), &[second]);
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: first,
            control: None
        })
    );
    assert_eq!(
        manager.apply(WindowCommand::Minimize(second)).unwrap(),
        rebellion_windowing::WindowCommit {
            transition: WindowTransition::Unchanged {
                subject: TransitionSubject::Window(second),
                reason: UnchangedReason::AlreadyMinimized
            },
            events: vec![],
        }
    );
    assert_eq!(manager.presentation_history(), &[second]);
    assert_eq!(
        manager.apply(WindowCommand::CapturePointer(second)),
        Err(WindowError::IneligibleCapture(second))
    );
    assert_eq!(
        manager.apply(WindowCommand::Focus(FocusTarget {
            window: second,
            control: None
        })),
        Err(WindowError::IneligibleFocus(FocusTarget {
            window: second,
            control: None
        }))
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn thirteenth_minimized_detail_evicts_oldest_with_teardown_events() {
    // Source: FUN_00428b40 fixes capacity at 12; closing evicted content is a port decision.
    let mut manager = WindowManager::new();
    let mut ids = Vec::new();
    let kinds = [
        WindowKind::System,
        WindowKind::Fleet,
        WindowKind::Defenses,
        WindowKind::Missions,
    ];
    for subject in 0..12 {
        let id = open(
            &mut manager,
            modeless_spec(
                kinds[subject as usize % kinds.len()],
                subject,
                WindowOwner::StrategicRoot,
            ),
        );
        let commit = manager.apply(WindowCommand::Minimize(id)).unwrap();
        assert_eq!(
            commit.transition,
            WindowTransition::Minimized {
                id,
                evicted: vec![]
            }
        );
        ids.push(id);
        assert_eq!(manager.presentation_history(), ids);
        assert_eq!(manager.validate(), Ok(()));
    }
    let newest = open(
        &mut manager,
        modeless_spec(WindowKind::System, 12, WindowOwner::StrategicRoot),
    );
    let commit = manager.apply(WindowCommand::Minimize(newest)).unwrap();
    assert_eq!(
        commit.transition,
        WindowTransition::Minimized {
            id: newest,
            evicted: vec![ids[0]]
        }
    );
    assert_eq!(
        commit.events,
        vec![
            WindowEvent::Closed {
                window: ids[0],
                key: modeless_spec(WindowKind::System, 0, WindowOwner::StrategicRoot).key,
                reason: CloseReason::Evicted
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: newest,
                    control: None
                }),
                current: None
            },
        ]
    );
    let oldest = ids.remove(0);
    ids.push(newest);
    assert_eq!(manager.presentation_history(), ids);
    assert!(manager.visible_order().is_empty());
    assert_eq!(
        manager.id_for(modeless_spec(WindowKind::System, 0, WindowOwner::StrategicRoot).key),
        None
    );
    assert_eq!(
        manager.apply(WindowCommand::Restore(oldest)),
        Err(WindowError::UnknownWindow(oldest))
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn restore_removes_history_and_raises_without_reallocating_identity() {
    // Source: FUN_00429020 removes the selected rail entry and restores its presentation.
    let mut manager = WindowManager::new();
    let spec = modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot);
    let id = open(&mut manager, spec);
    manager.apply(WindowCommand::Minimize(id)).unwrap();
    let other = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    assert_eq!(
        manager
            .apply(WindowCommand::Restore(id))
            .unwrap()
            .transition,
        WindowTransition::Restored { id }
    );
    assert_eq!(manager.id_for(spec.key), Some(id));
    assert_eq!(manager.visible_order(), &[other, id]);
    assert!(manager.presentation_history().is_empty());
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: id,
            control: None
        })
    );
    manager.apply(WindowCommand::FocusRoot).unwrap();
    assert_eq!(
        manager
            .apply(WindowCommand::Restore(id))
            .unwrap()
            .transition,
        WindowTransition::Unchanged {
            subject: TransitionSubject::Window(id),
            reason: UnchangedReason::AlreadyVisible,
        }
    );
    assert_eq!(manager.focused(), None);
    let next = open(
        &mut manager,
        modeless_spec(WindowKind::Missions, 3, WindowOwner::StrategicRoot),
    );
    assert_eq!(next.get(), other.get() + 1);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn reopening_minimized_detail_restores_and_raises() {
    // Source: FUN_0045aac0 restores and raises an existing subject/kind detail.
    let mut manager = WindowManager::new();
    let spec = modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot);
    let id = open(&mut manager, spec);
    manager.apply(WindowCommand::Minimize(id)).unwrap();
    let modal = open(&mut manager, modal_spec(WindowKind::Encyclopedia));
    let commit = manager.apply(WindowCommand::Open(spec)).unwrap();
    assert_eq!(
        commit.transition,
        WindowTransition::Existing {
            id,
            effect: rebellion_windowing::ExistingEffect::RestoredAndRaised
        }
    );
    assert!(commit.events.is_empty());
    assert_eq!(manager.visible_order(), &[id, modal]);
    assert!(manager.presentation_history().is_empty());
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: modal,
            control: None
        })
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn non_history_kind_cannot_minimize() {
    // Port: only the four history-capable modeless detail families can minimize.
    for kind in WindowKind::ALL {
        let mut manager = WindowManager::new();
        let spec = modeless_spec(kind, 1, WindowOwner::StrategicRoot);
        if kind == WindowKind::Menu {
            continue;
        }
        let id = open(&mut manager, spec);
        if !matches!(
            kind,
            WindowKind::System | WindowKind::Fleet | WindowKind::Defenses | WindowKind::Missions
        ) {
            assert_eq!(
                manager.apply(WindowCommand::Minimize(id)),
                Err(WindowError::NotMinimizable(id))
            );
            assert_eq!(manager.visible_order(), &[id]);
            assert!(manager.presentation_history().is_empty());
        }
        assert_eq!(manager.validate(), Ok(()));
    }
    let mut manager = WindowManager::new();
    let mut spec = modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot);
    spec.policy.presentation = PresentationPolicy::None;
    let id = open(&mut manager, spec);
    assert_eq!(
        manager.apply(WindowCommand::Minimize(id)),
        Err(WindowError::NotMinimizable(id))
    );
}

#[test]
fn minimizing_owner_with_modal_child_is_rejected_atomically() {
    // Port: an owned modal must never become an invisible input blocker.
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let mut spec = modal_spec(WindowKind::Status);
    spec.owner = WindowOwner::Window(owner);
    let child = open(&mut manager, spec);
    manager.apply(WindowCommand::CapturePointer(child)).unwrap();
    let order = manager.visible_order().to_vec();
    let focus = manager.focused();
    assert_eq!(
        manager.apply(WindowCommand::Minimize(owner)),
        Err(WindowError::OwnerHasModalChild(owner))
    );
    assert_eq!(manager.visible_order(), order);
    assert!(manager.presentation_history().is_empty());
    assert_eq!(manager.focused(), focus);
    assert_eq!(manager.capture_owner(), Some(child));
    assert_eq!(manager.id_for(spec.key), Some(child));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn minimize_close_and_reset_cancel_capture_once() {
    // Port: cancellation precedes hiding/teardown; reset includes minimized content.
    let mut manager = WindowManager::new();
    let spec = modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot);
    let id = open(&mut manager, spec);
    manager.apply(WindowCommand::CapturePointer(id)).unwrap();
    assert_eq!(
        manager.apply(WindowCommand::Minimize(id)).unwrap().events,
        vec![
            WindowEvent::CaptureCancelled {
                window: id,
                reason: CaptureCancelReason::Minimized
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: id,
                    control: None
                }),
                current: None
            },
        ]
    );
    let closed = manager.apply(WindowCommand::Close(id)).unwrap();
    assert_eq!(
        closed.events,
        vec![WindowEvent::Closed {
            window: id,
            key: spec.key,
            reason: CloseReason::Requested
        }]
    );
    assert_eq!(
        closed.transition,
        WindowTransition::Closed {
            root: id,
            closed: vec![id],
            restored_focus: None
        }
    );
    assert!(manager.presentation_history().is_empty());
    let hidden = open(&mut manager, spec);
    manager.apply(WindowCommand::Minimize(hidden)).unwrap();
    let visible_spec = modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot);
    let visible = open(&mut manager, visible_spec);
    manager
        .apply(WindowCommand::CapturePointer(visible))
        .unwrap();
    assert_eq!(
        manager.reset_session(),
        vec![
            WindowEvent::CaptureCancelled {
                window: visible,
                reason: CaptureCancelReason::SessionReset
            },
            WindowEvent::Closed {
                window: visible,
                key: visible_spec.key,
                reason: CloseReason::SessionReset
            },
            WindowEvent::Closed {
                window: hidden,
                key: spec.key,
                reason: CloseReason::SessionReset
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: visible,
                    control: None
                }),
                current: None
            },
        ]
    );
    assert!(manager.reset_session().is_empty());
    assert!(manager.presentation_history().is_empty());
    assert_eq!(manager.id_for(spec.key), None);
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.validate(), Ok(()));
    let next = open(&mut manager, spec);
    assert_eq!(next.get(), visible.get() + 1);
}

#[test]
fn history_eviction_delivers_capture_close_and_focus_receipts() {
    // FUN_00428b40 bound + port cleanup: cancellation, child teardown, eviction, final focus.
    let mut manager = WindowManager::new();
    let first_spec = modeless_spec(WindowKind::System, 0, WindowOwner::StrategicRoot);
    let oldest = open(&mut manager, first_spec);
    manager.apply(WindowCommand::Minimize(oldest)).unwrap();
    for subject in 1..12 {
        let id = open(
            &mut manager,
            modeless_spec(WindowKind::System, subject, WindowOwner::StrategicRoot),
        );
        manager.apply(WindowCommand::Minimize(id)).unwrap();
    }
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 12, WindowOwner::StrategicRoot),
    );
    let transient = menu_spec(owner);
    let child = open(&mut manager, transient);
    manager.apply(WindowCommand::CapturePointer(child)).unwrap();
    let commit = manager.apply(WindowCommand::Minimize(owner)).unwrap();
    assert_eq!(
        commit.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: child,
                reason: CaptureCancelReason::Minimized
            },
            WindowEvent::Closed {
                window: child,
                key: transient.key,
                reason: CloseReason::OwnerClosed
            },
            WindowEvent::Closed {
                window: oldest,
                key: first_spec.key,
                reason: CloseReason::Evicted
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: child,
                    control: None
                }),
                current: None
            },
        ]
    );
    assert_eq!(
        commit.transition,
        WindowTransition::Minimized {
            id: owner,
            evicted: vec![oldest]
        }
    );
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.id_for(transient.key), None);
    assert_eq!(manager.presentation_history().len(), 12);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn hidden_owned_details_close_child_first_and_cannot_restore_under_a_hidden_owner() {
    // Port lifecycle ownership applies to retained history, not just visible order.
    let mut manager = WindowManager::new();
    let owner_spec = modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot);
    let owner = open(&mut manager, owner_spec);
    let child_spec = modeless_spec(WindowKind::Fleet, 2, WindowOwner::Window(owner));
    let child = open(&mut manager, child_spec);
    manager.apply(WindowCommand::Minimize(child)).unwrap();
    manager.apply(WindowCommand::Minimize(owner)).unwrap();
    assert_eq!(manager.presentation_history(), &[child, owner]);
    assert_eq!(manager.validate(), Ok(()));
    assert_eq!(
        manager.apply(WindowCommand::Restore(child)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(
        manager.apply(WindowCommand::Open(child_spec)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.presentation_history(), &[child, owner]);
    assert!(manager.visible_order().is_empty());
    let events = manager.reset_session();
    assert_eq!(
        events,
        vec![
            WindowEvent::Closed {
                window: child,
                key: child_spec.key,
                reason: CloseReason::SessionReset
            },
            WindowEvent::Closed {
                window: owner,
                key: owner_spec.key,
                reason: CloseReason::SessionReset
            },
        ]
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn minimizing_a_visible_nontransient_child_owner_is_rejected_without_losing_content() {
    // Port: minimizing closes transient children, not persistent child content.
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let child = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::Window(owner)),
    );
    manager.apply(WindowCommand::CapturePointer(child)).unwrap();
    assert_eq!(
        manager.apply(WindowCommand::Minimize(owner)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.visible_order(), &[owner, child]);
    assert!(manager.presentation_history().is_empty());
    assert_eq!(manager.capture_owner(), Some(child));
    assert_eq!(manager.focused().map(|target| target.window), Some(child));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn minimized_noop_duplicate_preserves_history_and_rejected_duplicates_are_atomic() {
    // Port: NoOp preserves all state, including hidden content and rail position.
    let mut manager = WindowManager::new();
    let mut spec = modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot);
    spec.policy.existing = ExistingOpenPolicy::NoOp;
    let id = open(&mut manager, spec);
    manager.apply(WindowCommand::Minimize(id)).unwrap();
    let other = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    manager.apply(WindowCommand::CapturePointer(other)).unwrap();
    assert_eq!(
        manager.apply(WindowCommand::Open(spec)).unwrap().transition,
        WindowTransition::Existing {
            id,
            effect: rebellion_windowing::ExistingEffect::NoOp
        }
    );
    let mut wrong_owner = spec;
    wrong_owner.owner = WindowOwner::Window(other);
    assert_eq!(
        manager.apply(WindowCommand::Open(wrong_owner)),
        Err(WindowError::DuplicateKeyWithDifferentOwner(spec.key))
    );
    let mut wrong_policy = spec;
    wrong_policy.policy.existing = ExistingOpenPolicy::RaiseRestore;
    assert_eq!(
        manager.apply(WindowCommand::Open(wrong_policy)),
        Err(WindowError::DuplicateKeyWithDifferentPolicy(spec.key))
    );
    assert_eq!(manager.presentation_history(), &[id]);
    assert_eq!(manager.visible_order(), &[other]);
    assert_eq!(manager.capture_owner(), Some(other));
    assert_eq!(manager.focused().map(|target| target.window), Some(other));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn restoring_and_reminimizing_a_middle_entry_updates_only_its_age() {
    // FUN_00429020 removes a chosen entry; FUN_00428b40 appends a fresh minimization.
    let mut manager = WindowManager::new();
    let mut ids = Vec::new();
    for subject in 0..3 {
        let id = open(
            &mut manager,
            modeless_spec(WindowKind::System, subject, WindowOwner::StrategicRoot),
        );
        manager.apply(WindowCommand::Minimize(id)).unwrap();
        ids.push(id);
    }
    let commit = manager.apply(WindowCommand::Restore(ids[1])).unwrap();
    assert_eq!(
        commit.events,
        vec![WindowEvent::FocusChanged {
            previous: None,
            current: Some(FocusTarget {
                window: ids[1],
                control: None
            })
        }]
    );
    assert_eq!(manager.presentation_history(), &[ids[0], ids[2]]);
    manager.apply(WindowCommand::Minimize(ids[1])).unwrap();
    assert_eq!(manager.presentation_history(), &[ids[0], ids[2], ids[1]]);
    manager.apply(WindowCommand::Close(ids[2])).unwrap();
    assert_eq!(manager.presentation_history(), &[ids[0], ids[1]]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn minimize_preserves_unrelated_focus_capture_and_rejects_stale_ids() {
    // Port: background minimization does not interrupt an unrelated gesture/control.
    let mut manager = WindowManager::new();
    let id = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let other = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    let target = FocusTarget {
        window: other,
        control: Some(ControlId(17)),
    };
    manager.apply(WindowCommand::Focus(target)).unwrap();
    manager.apply(WindowCommand::CapturePointer(other)).unwrap();
    assert!(manager
        .apply(WindowCommand::Minimize(id))
        .unwrap()
        .events
        .is_empty());
    assert_eq!(manager.focused(), Some(target));
    assert_eq!(manager.capture_owner(), Some(other));
    assert_eq!(manager.validate(), Ok(()));
    manager.apply(WindowCommand::Close(id)).unwrap();
    assert_eq!(
        manager.apply(WindowCommand::Minimize(id)),
        Err(WindowError::UnknownWindow(id))
    );
    assert_eq!(
        manager.apply(WindowCommand::Restore(id)),
        Err(WindowError::UnknownWindow(id))
    );
    assert_eq!(manager.capture_owner(), Some(other));
}

fn restore_rejects_now_ineligible_transient_placement(reopen: bool) {
    // Port: restoring must revalidate placement against the current modal boundary.
    // Independent Sol reproducer: a hidden transient can no longer be above its
    // blocked owner without violating the stack bands after another modal opens.
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let mut spec = modeless_spec(WindowKind::Fleet, 2, WindowOwner::Window(owner));
    spec.policy.band = StackBand::Transient;
    spec.policy.placement = OpenPlacement::AboveOwner;
    let child = open(&mut manager, spec);
    manager.apply(WindowCommand::Minimize(child)).unwrap();
    let modal = open(&mut manager, modal_spec(WindowKind::Status));
    manager.apply(WindowCommand::CapturePointer(modal)).unwrap();
    assert_eq!(manager.validate(), Ok(()));
    let before = format!("{manager:?}");
    let command = if reopen {
        WindowCommand::Open(spec)
    } else {
        WindowCommand::Restore(child)
    };
    assert_eq!(manager.apply(command), Err(WindowError::InvalidPlacement));
    assert_eq!(
        format!("{manager:?}"),
        before,
        "rejection preserves allocator, registry, order, history, focus and capture"
    );
    assert_eq!(manager.validate(), Ok(()));
    manager.apply(WindowCommand::Close(modal)).unwrap();
    manager.apply(command).unwrap();
    assert_eq!(manager.visible_order(), &[owner, child]);
    assert!(manager.presentation_history().is_empty());
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn direct_restore_rejects_now_ineligible_transient_placement_atomically() {
    restore_rejects_now_ineligible_transient_placement(false);
}

#[test]
fn raise_restore_reopen_rejects_now_ineligible_transient_placement_atomically() {
    restore_rejects_now_ineligible_transient_placement(true);
}

#[test]
fn background_minimize_preserves_the_previous_focus_candidate_ahead_of_topmost_fallback() {
    // Port shared focus contract: hiding an unrelated detail preserves the eligible
    // previous window, so a later close restores it before the topmost fallback.
    // Break caught: clearing an eligible previous candidate during minimization.
    let mut manager = WindowManager::new();
    let previous = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let current_spec = modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot);
    let current = open(&mut manager, current_spec);
    let mut top_spec = modeless_spec(WindowKind::Defenses, 3, WindowOwner::StrategicRoot);
    top_spec.policy.focus_on_open = FocusOnOpen::Preserve;
    let top = open(&mut manager, top_spec);
    let mut hidden_spec = modeless_spec(WindowKind::Missions, 4, WindowOwner::StrategicRoot);
    hidden_spec.policy.focus_on_open = FocusOnOpen::Preserve;
    let hidden = open(&mut manager, hidden_spec);
    manager
        .apply(WindowCommand::CapturePointer(current))
        .unwrap();
    assert!(manager
        .apply(WindowCommand::Minimize(hidden))
        .unwrap()
        .events
        .is_empty());
    let target = Some(FocusTarget {
        window: previous,
        control: None,
    });
    assert_eq!(
        manager.apply(WindowCommand::Close(current)).unwrap().events,
        vec![
            WindowEvent::CaptureCancelled {
                window: current,
                reason: CaptureCancelReason::OwnerClosed
            },
            WindowEvent::Closed {
                window: current,
                key: current_spec.key,
                reason: CloseReason::Requested
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: current,
                    control: None
                }),
                current: target
            },
        ]
    );
    assert_eq!(manager.focused(), target);
    assert_eq!(manager.visible_order(), &[previous, top]);
    assert_eq!(manager.presentation_history(), &[hidden]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn lifecycle_close_is_child_first_and_preserves_unrelated_focus() {
    // Source: strategic-windowing-toolkit.md, "Close sequence"; port contract says
    // actual lifecycle descendants close before focus is selected outside the subtree.
    // Break caught: orphaning a descendant or clearing an unrelated focused control.
    let mut manager = WindowManager::new();
    let unrelated = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    let child = open(
        &mut manager,
        modeless_spec(WindowKind::Status, 3, WindowOwner::Window(owner)),
    );
    let grandchild = open(&mut manager, menu_spec(child));
    let unrelated_control = FocusTarget {
        window: unrelated,
        control: Some(ControlId(41)),
    };
    manager
        .apply(WindowCommand::Focus(unrelated_control))
        .expect("unrelated visible control can focus");

    let commit = manager
        .apply(WindowCommand::Close(owner))
        .expect("owner close succeeds atomically");

    assert_eq!(
        commit.transition,
        WindowTransition::Closed {
            root: owner,
            closed: vec![grandchild, child, owner],
            restored_focus: Some(unrelated_control),
        }
    );
    assert_eq!(manager.focused(), Some(unrelated_control));
    assert_eq!(manager.visible_order(), &[unrelated]);
    assert_eq!(
        manager.id_for(modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot).key),
        None
    );
    assert_eq!(
        manager.id_for(modeless_spec(WindowKind::Status, 3, WindowOwner::Window(owner)).key),
        None
    );
    assert_eq!(manager.id_for(menu_spec(child).key), None);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn lifecycle_menu_saved_focus_revalidates_target() {
    // Source: strategic-windowing-toolkit.md, "Native focus" menu rule.
    // Break caught: restoring a stale control target after its window disappeared.
    let mut manager = WindowManager::new();
    let stale = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let menu_owner = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    let stale_control = FocusTarget {
        window: stale,
        control: Some(ControlId(7)),
    };
    manager
        .apply(WindowCommand::Focus(stale_control))
        .expect("saved control can focus before the menu opens");
    let menu = open(&mut manager, menu_spec(menu_owner));
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: menu,
            control: None,
        })
    );

    manager
        .apply(WindowCommand::Close(stale))
        .expect("unrelated saved window can close while the menu remains");
    let commit = manager
        .apply(WindowCommand::Close(menu))
        .expect("menu close revalidates its saved target");

    let fallback = FocusTarget {
        window: menu_owner,
        control: None,
    };
    assert_eq!(
        commit.transition,
        WindowTransition::Closed {
            root: menu,
            closed: vec![menu],
            restored_focus: Some(fallback),
        }
    );
    assert_eq!(manager.focused(), Some(fallback));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn menu_close_restores_the_saved_root_focus() {
    // Source: strategic-windowing-toolkit.md, the root is a real focus target even
    // though the public representation is `None`.
    // Break caught: flattening `Some(None)` and falling back to the menu owner.
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    manager
        .apply(WindowCommand::FocusRoot)
        .expect("root can focus before opening a menu");
    let menu = open(&mut manager, menu_spec(owner));

    let commit = manager
        .apply(WindowCommand::Close(menu))
        .expect("menu closes");

    assert_eq!(
        commit.transition,
        WindowTransition::Closed {
            root: menu,
            closed: vec![menu],
            restored_focus: None,
        }
    );
    assert_eq!(manager.focused(), None);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn menu_close_restores_saved_control_only_if_still_eligible() {
    // Source: strategic-windowing-toolkit.md, `FUN_00442380` / `FUN_00442430`.
    // Break caught: degrading a live menu invoker control to window-level focus.
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let saved = FocusTarget {
        window: owner,
        control: Some(ControlId(99)),
    };
    manager
        .apply(WindowCommand::Focus(saved))
        .expect("menu invoker control can focus");
    let menu = open(&mut manager, menu_spec(owner));
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: menu,
            control: None,
        })
    );

    let commit = manager
        .apply(WindowCommand::Close(menu))
        .expect("menu closes");

    assert_eq!(
        commit.transition,
        WindowTransition::Closed {
            root: menu,
            closed: vec![menu],
            restored_focus: Some(saved),
        }
    );
    assert_eq!(manager.focused(), Some(saved));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn activate_changes_order_and_focus_but_focus_alone_does_not() {
    // Source: implementation plan, lifecycle transaction matrix `Activate` / `Focus child`.
    // Break caught: treating keyboard focus as an implicit z-order mutation.
    let mut manager = WindowManager::new();
    let first = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let second = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    let first_control = FocusTarget {
        window: first,
        control: Some(ControlId(5)),
    };

    manager
        .apply(WindowCommand::Focus(first_control))
        .expect("focus alone succeeds");
    assert_eq!(manager.visible_order(), &[first, second]);
    assert_eq!(manager.focused(), Some(first_control));

    assert_eq!(
        manager
            .apply(WindowCommand::Activate(first))
            .expect("activation raises and focuses")
            .transition,
        WindowTransition::Raised { id: first }
    );
    assert_eq!(manager.visible_order(), &[second, first]);
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: first,
            control: None,
        })
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn raise_already_top_still_repairs_different_focus() {
    // Source: implementation plan, public kernel interface contract for `Raise`.
    // Break caught: returning early on already-top order before repairing focus.
    let mut manager = WindowManager::new();
    let first = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let top = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    manager
        .apply(WindowCommand::Focus(FocusTarget {
            window: first,
            control: Some(ControlId(8)),
        }))
        .expect("covered window can own keyboard focus");
    let order_before = manager.visible_order().to_vec();

    assert_eq!(
        manager
            .apply(WindowCommand::Raise(top))
            .expect("already-top raise repairs focus")
            .transition,
        WindowTransition::Raised { id: top }
    );
    assert_eq!(manager.visible_order(), order_before);
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: top,
            control: None,
        })
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn duplicate_raise_restore_repairs_focus_without_reallocating() {
    // Source: strategic-windowing-toolkit.md, "Existing-window policy".
    // Break caught: moving an existing detail to the top while leaving keyboard focus elsewhere.
    let mut manager = WindowManager::new();
    let detail_spec = modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot);
    let detail = open(&mut manager, detail_spec);
    let other = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: other,
            control: None,
        })
    );

    let commit = manager
        .apply(WindowCommand::Open(detail_spec))
        .expect("compatible detail reopen succeeds");

    assert_eq!(
        commit.transition,
        WindowTransition::Existing {
            id: detail,
            effect: rebellion_windowing::ExistingEffect::Raised,
        }
    );
    assert_eq!(
        commit.events,
        vec![WindowEvent::FocusChanged {
            previous: Some(FocusTarget {
                window: other,
                control: None,
            }),
            current: Some(FocusTarget {
                window: detail,
                control: None,
            }),
        }]
    );
    assert_eq!(manager.visible_order(), &[other, detail]);
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: detail,
            control: None,
        })
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn close_chooses_previous_eligible_survivor_outside_removed_subtree() {
    // Source: implementation plan, "Lifecycle, focus, and history".
    // Break caught: restoring an owned descendant or ignoring the one global previous candidate.
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 1, WindowOwner::StrategicRoot),
    );
    let child = open(
        &mut manager,
        modeless_spec(WindowKind::Status, 2, WindowOwner::Window(owner)),
    );
    let previous = open(
        &mut manager,
        modeless_spec(WindowKind::System, 3, WindowOwner::StrategicRoot),
    );
    let topmost = open(
        &mut manager,
        modeless_spec(WindowKind::Defenses, 4, WindowOwner::StrategicRoot),
    );
    manager
        .apply(WindowCommand::Focus(FocusTarget {
            window: previous,
            control: Some(ControlId(12)),
        }))
        .expect("previous survivor can focus");
    manager
        .apply(WindowCommand::Focus(FocusTarget {
            window: child,
            control: Some(ControlId(13)),
        }))
        .expect("owned child can focus");

    let commit = manager
        .apply(WindowCommand::Close(owner))
        .expect("owner subtree closes");

    let restored = FocusTarget {
        window: previous,
        control: None,
    };
    assert_eq!(
        commit.transition,
        WindowTransition::Closed {
            root: owner,
            closed: vec![child, owner],
            restored_focus: Some(restored),
        }
    );
    assert_eq!(manager.focused(), Some(restored));
    assert_eq!(manager.visible_order(), &[previous, topmost]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn close_preserves_surviving_root_focus() {
    // Source: implementation plan, close restoration first preserves unrelated
    // surviving focus; the persistent strategic root is such a target.
    // Break caught: treating root focus as absence and selecting a surface.
    let mut manager = WindowManager::new();
    let survivor = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let closing = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    manager
        .apply(WindowCommand::FocusRoot)
        .expect("root can focus without a modal");

    let commit = manager
        .apply(WindowCommand::Close(closing))
        .expect("unrelated surface closes");

    assert_eq!(
        commit.transition,
        WindowTransition::Closed {
            root: closing,
            closed: vec![closing],
            restored_focus: None,
        }
    );
    assert_eq!(manager.focused(), None);
    assert_eq!(manager.visible_order(), &[survivor]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn close_retains_a_previous_candidate_while_a_modal_temporarily_blocks_it() {
    // Source: implementation plan, one global previous-eligible survivor is
    // revalidated when restoration is attempted, not discarded while blocked.
    // Break caught: an unrelated close under a modal erasing the later restore target.
    let mut manager = WindowManager::new();
    let previous = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let fallback = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    manager
        .apply(WindowCommand::Focus(FocusTarget {
            window: previous,
            control: Some(ControlId(23)),
        }))
        .expect("future restore target can focus");
    let modal = open(&mut manager, modal_spec(WindowKind::SystemFinder));
    let unrelated = open(
        &mut manager,
        modeless_spec(WindowKind::Defenses, 3, WindowOwner::StrategicRoot),
    );

    manager
        .apply(WindowCommand::Close(unrelated))
        .expect("unrelated blocked surface closes while modal remains active");
    let commit = manager
        .apply(WindowCommand::Close(modal))
        .expect("modal closes");

    let restored = FocusTarget {
        window: previous,
        control: None,
    };
    assert_eq!(
        commit.transition,
        WindowTransition::Closed {
            root: modal,
            closed: vec![modal],
            restored_focus: Some(restored),
        }
    );
    assert_eq!(manager.focused(), Some(restored));
    assert_eq!(manager.visible_order(), &[previous, fallback]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn ordinary_window_close_uses_global_eligible_order_not_a_private_focus_stack() {
    // Source: strategic-windowing-toolkit.md, "Native focus"; the dormant native
    // saved-focus slot is not a basis for a per-window stack in the port.
    // Break caught: restoring the target saved when this ordinary window opened.
    let mut manager = WindowManager::new();
    let old_focus = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let closing = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    let global_previous = open(
        &mut manager,
        modeless_spec(WindowKind::Defenses, 3, WindowOwner::StrategicRoot),
    );
    manager
        .apply(WindowCommand::Focus(FocusTarget {
            window: closing,
            control: Some(ControlId(22)),
        }))
        .expect("ordinary window can regain focus");

    let commit = manager
        .apply(WindowCommand::Close(closing))
        .expect("ordinary window closes");

    let restored = FocusTarget {
        window: global_previous,
        control: None,
    };
    assert_eq!(
        commit.transition,
        WindowTransition::Closed {
            root: closing,
            closed: vec![closing],
            restored_focus: Some(restored),
        }
    );
    assert_eq!(manager.focused(), Some(restored));
    assert_ne!(
        manager.focused().map(|target| target.window),
        Some(old_focus)
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn focus_rejects_hidden_blocked_and_unknown_targets() {
    // Source: strategic-windowing-toolkit.md, "Modality and input routing".
    // Break caught: changing focus to a below-modal or already-closed identity.
    let mut manager = WindowManager::new();
    let background = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let modal = open(&mut manager, modal_spec(WindowKind::SystemFinder));
    let order_before = manager.visible_order().to_vec();
    let focus_before = manager.focused();
    let modal_control = FocusTarget {
        window: modal,
        control: Some(ControlId(30)),
    };
    manager
        .apply(WindowCommand::Focus(modal_control))
        .expect("the active modal and its controls remain eligible");
    let blocked = FocusTarget {
        window: background,
        control: Some(ControlId(31)),
    };

    assert_eq!(
        manager.apply(WindowCommand::Focus(blocked)),
        Err(WindowError::BlockedByModal {
            window: background,
            modal,
        })
    );
    assert_eq!(manager.visible_order(), order_before);
    assert_ne!(manager.focused(), focus_before);
    assert_eq!(manager.focused(), Some(modal_control));

    manager
        .apply(WindowCommand::Close(background))
        .expect("blocked background may close explicitly");
    let order_after_close = manager.visible_order().to_vec();
    let focus_after_close = manager.focused();
    assert_eq!(
        manager.apply(WindowCommand::Focus(blocked)),
        Err(WindowError::UnknownWindow(background))
    );
    assert_eq!(manager.visible_order(), order_after_close);
    assert_eq!(manager.focused(), focus_after_close);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn preserve_focus_modal_repairs_root_focus_to_the_modal() {
    // Source: strategic-windowing-toolkit.md, strategic modality blocks the root.
    // Break caught: leaving semantic root focus active when a preserve-focus modal opens.
    let mut manager = WindowManager::new();
    let mut modal = modal_spec(WindowKind::Status);
    modal.policy.focus_on_open = FocusOnOpen::Preserve;

    let modal_id = open(&mut manager, modal);

    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: modal_id,
            control: None,
        })
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn preserve_focus_modal_repairs_blocked_underlay_focus_to_the_modal() {
    // Source: strategic-windowing-toolkit.md, strategic modality is a whole-view
    // input boundary regardless of the modal's focus-on-open preference.
    // Break caught: leaving an existing background surface focused below a modal.
    let mut manager = WindowManager::new();
    let underlay = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: underlay,
            control: None,
        })
    );
    let mut modal = modal_spec(WindowKind::Status);
    modal.policy.focus_on_open = FocusOnOpen::Preserve;

    let modal_id = open(&mut manager, modal);

    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: modal_id,
            control: None,
        })
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn focus_root_is_rejected_while_an_active_modal_blocks_it() {
    // Source: strategic-windowing-toolkit.md, modality is a whole-view input boundary.
    // Break caught: clearing modal focus to route keyboard input to the strategic root.
    let mut manager = WindowManager::new();
    let background = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    assert_eq!(
        manager
            .apply(WindowCommand::FocusRoot)
            .expect("root can focus without a modal")
            .transition,
        WindowTransition::Focused {
            previous: Some(FocusTarget {
                window: background,
                control: None,
            }),
            current: None,
        }
    );

    let modal = open(&mut manager, modal_spec(WindowKind::Encyclopedia));
    let modal_focus = FocusTarget {
        window: modal,
        control: None,
    };
    assert_eq!(manager.focused(), Some(modal_focus));
    let order_before = manager.visible_order().to_vec();

    assert_eq!(
        manager.apply(WindowCommand::FocusRoot),
        Err(WindowError::RootBlockedBy(modal))
    );
    assert_eq!(manager.visible_order(), order_before);
    assert_eq!(manager.focused(), Some(modal_focus));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn capture_is_exclusive_and_release_by_another_window_fails() {
    // Source: strategic-windowing-toolkit.md, `FUN_00606ee0` requires no
    // foreign capture before a temporary title drag can claim the pointer.
    // Break caught: a second window stealing or releasing the active gesture.
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let other = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );

    let captured = manager
        .apply(WindowCommand::CapturePointer(owner))
        .expect("eligible owner captures");
    assert_eq!(
        captured.transition,
        WindowTransition::CaptureChanged {
            previous: None,
            current: Some(owner),
        }
    );
    assert!(captured.events.is_empty());
    assert_eq!(manager.capture_owner(), Some(owner));

    assert_eq!(
        manager.apply(WindowCommand::CapturePointer(other)),
        Err(WindowError::CaptureOwnedBy(owner))
    );
    assert_eq!(
        manager.apply(WindowCommand::ReleasePointer(other)),
        Err(WindowError::CaptureOwnedBy(owner))
    );
    assert_eq!(manager.capture_owner(), Some(owner));

    let released = manager
        .apply(WindowCommand::ReleasePointer(owner))
        .expect("capture owner releases on physical completion");
    assert_eq!(
        released.transition,
        WindowTransition::CaptureChanged {
            previous: Some(owner),
            current: None,
        }
    );
    assert!(released.events.is_empty());
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn explicit_capture_cancellation_returns_reasoned_receipts() {
    // Source: approved port boundary in the implementation plan; hold and
    // application-focus loss share the lifecycle cancellation primitive.
    // Break caught: clearing capture without telling the feature why its
    // in-progress gesture must discard its payload.
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 1, WindowOwner::StrategicRoot),
    );

    for reason in [
        CaptureCancelReason::HoldStarted,
        CaptureCancelReason::FocusLost,
    ] {
        manager
            .apply(WindowCommand::CapturePointer(owner))
            .expect("eligible gesture acquires capture");
        let cancelled = manager
            .apply(WindowCommand::CancelCapture(reason))
            .expect("coordinator cancellation succeeds");

        assert_eq!(
            cancelled.transition,
            WindowTransition::CaptureChanged {
                previous: Some(owner),
                current: None,
            }
        );
        assert_eq!(
            cancelled.events,
            vec![WindowEvent::CaptureCancelled {
                window: owner,
                reason,
            }]
        );
        assert_eq!(manager.capture_owner(), None);
    }
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn capture_close_cancels_before_child_content_teardown() {
    // Source: strategic-windowing-toolkit.md, "Close sequence"; native child
    // destruction unregisters capture before parent teardown and restoration.
    // Break caught: dropping feature content before its drag payload receives
    // cancellation, or emitting the owner before its child.
    let mut manager = WindowManager::new();
    let owner_spec = modeless_spec(WindowKind::Fleet, 1, WindowOwner::StrategicRoot);
    let owner = open(&mut manager, owner_spec);
    let child_spec = menu_spec(owner);
    let child = open(&mut manager, child_spec);
    manager
        .apply(WindowCommand::CapturePointer(child))
        .expect("eligible child owns the gesture");

    let commit = manager
        .apply(WindowCommand::Close(owner))
        .expect("owner subtree closes");

    assert_eq!(
        commit.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: child,
                reason: CaptureCancelReason::OwnerClosed,
            },
            WindowEvent::Closed {
                window: child,
                key: child_spec.key,
                reason: CloseReason::OwnerClosed,
            },
            WindowEvent::Closed {
                window: owner,
                key: owner_spec.key,
                reason: CloseReason::Requested,
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: child,
                    control: None,
                }),
                current: None,
            },
        ]
    );
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.focused(), None);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn strategic_modal_open_cancels_incompatible_capture_before_focus_changes() {
    // Source: strategic-windowing-toolkit.md, `FUN_00601340` drains temporary
    // capture before the strategic dialog-opening path completes.
    // Break caught: leaving a blocked underlay gesture live beneath a modal,
    // or asking the renderer to focus the modal before clearing that payload.
    let mut manager = WindowManager::new();
    let background = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 1, WindowOwner::StrategicRoot),
    );
    manager
        .apply(WindowCommand::CapturePointer(background))
        .expect("background gesture captures before modal open");

    let commit = manager
        .apply(WindowCommand::Open(modal_spec(WindowKind::Encyclopedia)))
        .expect("modal opens atomically");
    let modal = opened_id(commit.transition.clone());
    let modal_focus = FocusTarget {
        window: modal,
        control: None,
    };

    assert_eq!(
        commit.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: background,
                reason: CaptureCancelReason::ModalOpened,
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: background,
                    control: None,
                }),
                current: Some(modal_focus),
            },
        ]
    );
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.focused(), Some(modal_focus));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn strategic_modal_reopen_cancels_newly_blocked_capture_before_focus_changes() {
    let mut manager = WindowManager::new();
    let mut lower_spec = modal_spec(WindowKind::SystemFinder);
    lower_spec.policy.existing = ExistingOpenPolicy::RaiseRestore;
    let lower = open(&mut manager, lower_spec);
    let mut upper_spec = modal_spec(WindowKind::FleetFinder);
    upper_spec.policy.existing = ExistingOpenPolicy::RaiseRestore;
    let upper = open(&mut manager, upper_spec);
    manager
        .apply(WindowCommand::CapturePointer(upper))
        .expect("active modal captures");

    let reopened = manager
        .apply(WindowCommand::Open(lower_spec))
        .expect("compatible lower modal reopens");

    assert_eq!(
        reopened.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: upper,
                reason: CaptureCancelReason::ModalOpened,
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: upper,
                    control: None,
                }),
                current: Some(FocusTarget {
                    window: lower,
                    control: None,
                }),
            },
        ]
    );
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.focused().map(|target| target.window), Some(lower));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn capture_reset_preserves_monotonic_allocator() {
    // Source: approved port boundary in the implementation plan; a discarded
    // campaign clears every gesture/content reference without recycling IDs.
    // Break caught: orphaning a drag payload during reset or letting a stale
    // callback address a fresh campaign window.
    let mut manager = WindowManager::new();
    let owner_spec = modeless_spec(WindowKind::Fleet, 1, WindowOwner::StrategicRoot);
    let owner = open(&mut manager, owner_spec);
    let child_spec = menu_spec(owner);
    let child = open(&mut manager, child_spec);
    manager
        .apply(WindowCommand::CapturePointer(child))
        .expect("child gesture captures before reset");

    let events = manager.reset_session();

    assert_eq!(
        events,
        vec![
            WindowEvent::CaptureCancelled {
                window: child,
                reason: CaptureCancelReason::SessionReset,
            },
            WindowEvent::Closed {
                window: child,
                key: child_spec.key,
                reason: CloseReason::SessionReset,
            },
            WindowEvent::Closed {
                window: owner,
                key: owner_spec.key,
                reason: CloseReason::SessionReset,
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: child,
                    control: None,
                }),
                current: None,
            },
        ]
    );
    assert_eq!(manager.visible_order(), &[]);
    assert_eq!(manager.focused(), None);
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.validate(), Ok(()));

    let fresh = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 1, WindowOwner::StrategicRoot),
    );
    assert!(fresh.get() > child.get());
}

#[test]
fn closing_an_owner_removes_descendants_and_clears_focus_and_capture_before_restoration() {
    // Source: strategic-windowing-toolkit.md, "Close sequence" port contract.
    // Break caught: retaining a descendant gesture/focus target or restoring
    // focus before child-first feature content has been removed.
    let mut manager = WindowManager::new();
    let survivor = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let owner_spec = modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot);
    let owner = open(&mut manager, owner_spec);
    let child_spec = menu_spec(owner);
    let child = open(&mut manager, child_spec);
    manager
        .apply(WindowCommand::CapturePointer(child))
        .expect("child captures before its owner closes");

    let commit = manager
        .apply(WindowCommand::Close(owner))
        .expect("owner subtree closes");
    let survivor_focus = FocusTarget {
        window: survivor,
        control: None,
    };

    assert_eq!(
        commit.transition,
        WindowTransition::Closed {
            root: owner,
            closed: vec![child, owner],
            restored_focus: Some(survivor_focus),
        }
    );
    assert_eq!(
        commit.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: child,
                reason: CaptureCancelReason::OwnerClosed,
            },
            WindowEvent::Closed {
                window: child,
                key: child_spec.key,
                reason: CloseReason::OwnerClosed,
            },
            WindowEvent::Closed {
                window: owner,
                key: owner_spec.key,
                reason: CloseReason::Requested,
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: child,
                    control: None,
                }),
                current: Some(survivor_focus),
            },
        ]
    );
    assert_eq!(manager.visible_order(), &[survivor]);
    assert_eq!(manager.focused(), Some(survivor_focus));
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn reset_clears_records_but_never_reuses_old_ids() {
    // Source: implementation plan, session-reset identity boundary.
    // Break caught: retaining a discarded semantic key or recycling an ID
    // that a deferred callback from the old campaign can still name.
    let mut manager = WindowManager::new();
    let first_spec = modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot);
    let first = open(&mut manager, first_spec);
    let second_spec = modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot);
    let second = open(&mut manager, second_spec);

    let events = manager.reset_session();

    assert_eq!(
        events,
        vec![
            WindowEvent::Closed {
                window: second,
                key: second_spec.key,
                reason: CloseReason::SessionReset,
            },
            WindowEvent::Closed {
                window: first,
                key: first_spec.key,
                reason: CloseReason::SessionReset,
            },
            WindowEvent::FocusChanged {
                previous: Some(FocusTarget {
                    window: second,
                    control: None,
                }),
                current: None,
            },
        ]
    );
    assert_eq!(manager.id_for(first_spec.key), None);
    assert_eq!(manager.id_for(second_spec.key), None);
    assert_eq!(
        manager.apply(WindowCommand::Close(first)),
        Err(WindowError::UnknownWindow(first))
    );

    let fresh = open(&mut manager, first_spec);
    assert!(fresh.get() > second.get());
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn capture_cancellation_is_exactly_once_when_nothing_is_captured() {
    // Source: implementation plan, cancellation clears payload and suppresses
    // orphan release; only the transition that owned a gesture emits a receipt.
    // Break caught: duplicate payload teardown after a later release/cancel.
    let mut manager = WindowManager::new();
    let owner = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 1, WindowOwner::StrategicRoot),
    );

    let release = manager
        .apply(WindowCommand::ReleasePointer(owner))
        .expect("release without capture is an explicit no-op");
    assert_eq!(
        release.transition,
        WindowTransition::Unchanged {
            subject: TransitionSubject::Window(owner),
            reason: UnchangedReason::NotCaptured,
        }
    );
    assert!(release.events.is_empty());

    let cancel = manager
        .apply(WindowCommand::CancelCapture(CaptureCancelReason::FocusLost))
        .expect("repeated coordinator cancellation is an explicit no-op");
    assert_eq!(
        cancel.transition,
        WindowTransition::Unchanged {
            subject: TransitionSubject::StrategicRoot,
            reason: UnchangedReason::NotCaptured,
        }
    );
    assert!(cancel.events.is_empty());
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn focus_commands_return_ordered_focus_receipts() {
    let mut manager = WindowManager::new();
    let first = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let second = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    let first_control = FocusTarget {
        window: first,
        control: Some(ControlId(7)),
    };

    let focused = manager
        .apply(WindowCommand::Focus(first_control))
        .expect("eligible control receives focus");
    assert_eq!(
        focused.events,
        vec![WindowEvent::FocusChanged {
            previous: Some(FocusTarget {
                window: second,
                control: None,
            }),
            current: Some(first_control),
        }]
    );

    let activated = manager
        .apply(WindowCommand::Activate(second))
        .expect("eligible window activates");
    assert_eq!(
        activated.events,
        vec![WindowEvent::FocusChanged {
            previous: Some(first_control),
            current: Some(FocusTarget {
                window: second,
                control: None,
            }),
        }]
    );

    let rooted = manager
        .apply(WindowCommand::FocusRoot)
        .expect("strategic root receives focus");
    assert_eq!(
        rooted.events,
        vec![WindowEvent::FocusChanged {
            previous: Some(FocusTarget {
                window: second,
                control: None,
            }),
            current: None,
        }]
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn capture_rejects_ineligible_targets_and_failed_modal_open_is_atomic() {
    let mut manager = WindowManager::new();
    let background = open(
        &mut manager,
        modeless_spec(WindowKind::System, 1, WindowOwner::StrategicRoot),
    );
    let stale = open(
        &mut manager,
        modeless_spec(WindowKind::Fleet, 2, WindowOwner::StrategicRoot),
    );
    manager
        .apply(WindowCommand::Close(stale))
        .expect("fixture identity becomes stale");
    manager
        .apply(WindowCommand::CapturePointer(background))
        .expect("eligible background window captures");
    let order_before = manager.visible_order().to_vec();
    let focus_before = manager.focused();
    let mut invalid_modal = modal_spec(WindowKind::SystemFinder);
    invalid_modal.policy.restoration = FocusRestoration::SavedMenuTarget;

    assert_eq!(
        manager.apply(WindowCommand::Open(invalid_modal)),
        Err(WindowError::KindPolicyMismatch(WindowKind::SystemFinder))
    );
    assert_eq!(manager.capture_owner(), Some(background));
    assert_eq!(manager.visible_order(), order_before);
    assert_eq!(manager.focused(), focus_before);

    let modal = open(&mut manager, modal_spec(WindowKind::SystemFinder));
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(
        manager.apply(WindowCommand::CapturePointer(background)),
        Err(WindowError::IneligibleCapture(background))
    );
    assert_eq!(
        manager.apply(WindowCommand::CapturePointer(stale)),
        Err(WindowError::UnknownWindow(stale))
    );
    assert_eq!(manager.focused().map(|target| target.window), Some(modal));
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.validate(), Ok(()));
}
