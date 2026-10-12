use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hash;

use rebellion_windowing::{
    CaptureCancelReason, Cardinality, CloseReason, ControlId, ExistingEffect, ExistingOpenPolicy,
    FocusOnOpen, FocusRestoration, FocusTarget, InputPolicy, OpenPlacement, PresentationPolicy,
    StackBand, SubjectId, WindowCommand, WindowError, WindowEvent, WindowId, WindowKey, WindowKind,
    WindowManager, WindowOwner, WindowPolicy, WindowSpec, WindowTransition,
};

fn assert_opaque_id_traits<T: Copy + Debug + Eq + Hash + Ord>() {}

fn modeless_subject_spec(kind: WindowKind, subject: u64) -> WindowSpec {
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

fn sector_spec(subject: u64) -> WindowSpec {
    WindowSpec {
        key: WindowKey::contextual(WindowKind::Sector, SubjectId::new(1, subject)),
        owner: WindowOwner::StrategicRoot,
        policy: WindowPolicy {
            cardinality: Cardinality::Subject { max_open: Some(2) },
            band: StackBand::Low,
            placement: OpenPlacement::BottomOfBand,
            existing: ExistingOpenPolicy::NoOp,
            input: InputPolicy::Modeless,
            presentation: PresentationPolicy::None,
            focus_on_open: FocusOnOpen::Window,
            restoration: FocusRestoration::PreviousEligible,
        },
    }
}

fn singleton_spec(kind: WindowKind) -> WindowSpec {
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

fn transient_menu_spec(owner: WindowId) -> WindowSpec {
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

fn opened_id(transition: WindowTransition) -> WindowId {
    match transition {
        WindowTransition::Opened { id } => id,
        other => panic!("expected a new window, got {other:?}"),
    }
}

fn open_window(manager: &mut WindowManager, spec: WindowSpec) -> WindowId {
    opened_id(manager.apply(WindowCommand::Open(spec)).unwrap().transition)
}

fn replaced_id(transition: WindowTransition, expected_old: WindowId) -> WindowId {
    match transition {
        WindowTransition::Replaced { old, new } => {
            assert_eq!(old, expected_old);
            assert!(new.get() > old.get());
            new
        }
        other => panic!("expected atomic replacement, got {other:?}"),
    }
}

#[test]
fn replacement_cleans_history_capture_and_focus_atomically() {
    // port: SW-02R atomic publication; lifecycle sources FUN_00600f90,
    // FUN_005ffb60 (owner cascade), FUN_00428b40 (history), FUN_00422ce0 (focus).
    // Break caught: bypassing close cleanup or exposing intermediate focus receipts.
    let mut manager = WindowManager::new();
    let old_spec = modeless_subject_spec(WindowKind::System, 1);
    let old = open_window(&mut manager, old_spec);
    let survivor = open_window(&mut manager, modeless_subject_spec(WindowKind::Fleet, 2));
    let history_spec = modeless_subject_spec(WindowKind::Defenses, 3);
    let history_survivor = open_window(&mut manager, history_spec);
    manager
        .apply(WindowCommand::Minimize(history_survivor))
        .unwrap();
    let hidden_spec = WindowSpec {
        owner: WindowOwner::Window(old),
        ..modeless_subject_spec(WindowKind::Defenses, 4)
    };
    let hidden = open_window(&mut manager, hidden_spec);
    manager.apply(WindowCommand::Minimize(hidden)).unwrap();
    let child_spec = WindowSpec {
        owner: WindowOwner::Window(old),
        ..modeless_subject_spec(WindowKind::Missions, 5)
    };
    let child = open_window(&mut manager, child_spec);
    let menu_spec = transient_menu_spec(child);
    let menu = open_window(&mut manager, menu_spec);
    manager.apply(WindowCommand::CapturePointer(menu)).unwrap();
    let focus_before = manager.focused();
    assert_eq!(
        focus_before,
        Some(FocusTarget {
            window: menu,
            control: None
        })
    );

    let new_spec = modeless_subject_spec(WindowKind::System, 99);
    let commit = manager
        .apply(WindowCommand::Replace { old, new: new_spec })
        .unwrap();
    let new = replaced_id(commit.transition, old);
    assert_eq!(new.get(), 7);
    assert_eq!(
        commit.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: menu,
                reason: CaptureCancelReason::OwnerClosed
            },
            WindowEvent::Closed {
                window: menu,
                key: menu_spec.key,
                reason: CloseReason::OwnerClosed
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
                window: old,
                key: old_spec.key,
                reason: CloseReason::Replaced
            },
            WindowEvent::FocusChanged {
                previous: focus_before,
                current: Some(FocusTarget {
                    window: new,
                    control: None
                })
            },
        ]
    );
    for key in [old_spec.key, hidden_spec.key, child_spec.key, menu_spec.key] {
        assert_eq!(manager.id_for(key), None);
    }
    assert_eq!(manager.id_for(new_spec.key), Some(new));
    assert_eq!(manager.id_for(history_spec.key), Some(history_survivor));
    assert_eq!(manager.visible_order(), &[survivor, new]);
    assert_eq!(manager.presentation_history(), &[history_survivor]);
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(
        manager.focused(),
        Some(FocusTarget {
            window: new,
            control: None
        })
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn replacement_checks_capacity_after_removing_the_old_subtree() {
    // Source: FUN_00429ce0, max two Sectors; port: SW-02R post-removal validation.
    // Break caught: validating capacity against the original registry or keeping a third Sector.
    let mut manager = WindowManager::new();
    let old_spec = sector_spec(1);
    let old = open_window(&mut manager, old_spec);
    let survivor = open_window(&mut manager, sector_spec(2));
    let new_spec = sector_spec(3);
    let commit = manager
        .apply(WindowCommand::Replace { old, new: new_spec })
        .unwrap();
    let new = replaced_id(commit.transition, old);
    assert_eq!(new.get(), 3);
    assert_eq!(manager.id_for(old_spec.key), None);
    assert_eq!(manager.id_for(new_spec.key), Some(new));
    assert_eq!(manager.visible_order(), &[new, survivor]);
    let before = format!("{manager:?}");
    assert_eq!(
        manager.apply(WindowCommand::Open(sector_spec(4))),
        Err(WindowError::CardinalityExceeded(WindowKind::Sector))
    );
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.validate(), Ok(()));

    // Capacity released by descendants also counts, even when the root is another family.
    let mut manager = WindowManager::new();
    let root = open_window(&mut manager, modeless_subject_spec(WindowKind::System, 1));
    let bounded = WindowPolicy {
        cardinality: Cardinality::Subject { max_open: Some(2) },
        ..modeless_subject_spec(WindowKind::Fleet, 2).policy
    };
    for subject in [2, 3] {
        open_window(
            &mut manager,
            WindowSpec {
                owner: WindowOwner::Window(root),
                policy: bounded,
                ..modeless_subject_spec(WindowKind::Fleet, subject)
            },
        );
    }
    let new_spec = WindowSpec {
        policy: bounded,
        ..modeless_subject_spec(WindowKind::Fleet, 4)
    };
    let commit = manager
        .apply(WindowCommand::Replace {
            old: root,
            new: new_spec,
        })
        .unwrap();
    let new = replaced_id(commit.transition, root);
    assert_eq!(new.get(), 4);
    assert_eq!(manager.visible_order(), &[new]);
    assert_eq!(
        commit
            .events
            .iter()
            .filter(|event| matches!(event, WindowEvent::Closed { .. }))
            .count(),
        3
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn invalid_replacement_preserves_old_id_content_links_and_allocator() {
    // port: SW-02R complete rollback; no failed preparation can discard feature content.
    // Break caught: mutating cleanup/index/order/focus/capture or allocating before validation.
    let mut manager = WindowManager::new();
    let old_spec = modeless_subject_spec(WindowKind::System, 1);
    let old = open_window(&mut manager, old_spec);
    let child_spec = WindowSpec {
        owner: WindowOwner::Window(old),
        ..modeless_subject_spec(WindowKind::Fleet, 2)
    };
    let child = open_window(&mut manager, child_spec);
    manager.apply(WindowCommand::Minimize(child)).unwrap();
    let menu_spec = transient_menu_spec(old);
    let menu = open_window(&mut manager, menu_spec);
    manager.apply(WindowCommand::CapturePointer(menu)).unwrap();
    let stale = open_window(&mut manager, sector_spec(4));
    manager.apply(WindowCommand::Close(stale)).unwrap();
    let before = format!("{manager:?}");
    let invalid = WindowSpec {
        owner: WindowOwner::Window(stale),
        ..modeless_subject_spec(WindowKind::Missions, 5)
    };
    assert_eq!(
        manager.apply(WindowCommand::Replace { old, new: invalid }),
        Err(WindowError::UnknownOwner(stale))
    );
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.id_for(old_spec.key), Some(old));
    assert_eq!(manager.id_for(child_spec.key), Some(child));
    assert_eq!(manager.presentation_history(), &[child]);
    assert_eq!(manager.capture_owner(), Some(menu));
    assert_eq!(manager.validate(), Ok(()));
    let fresh = open_window(&mut manager, modeless_subject_spec(WindowKind::Defenses, 6));
    assert_eq!(fresh.get(), 5);
    let close = manager.apply(WindowCommand::Close(old)).unwrap();
    assert_eq!(
        close.transition,
        WindowTransition::Closed {
            root: old,
            closed: vec![menu, child, old],
            restored_focus: Some(FocusTarget {
                window: fresh,
                control: None
            }),
        }
    );
}

#[test]
fn replacement_rejects_an_owner_inside_the_replaced_subtree() {
    // port: SW-02R rejects new ownership into any removed record, including hidden descendants.
    // Break caught: leaving an orphan or accepting the root as the replacement's owner.
    let mut manager = WindowManager::new();
    let old = open_window(&mut manager, modeless_subject_spec(WindowKind::System, 1));
    let child = open_window(
        &mut manager,
        WindowSpec {
            owner: WindowOwner::Window(old),
            ..modeless_subject_spec(WindowKind::Fleet, 2)
        },
    );
    let hidden = open_window(
        &mut manager,
        WindowSpec {
            owner: WindowOwner::Window(child),
            ..modeless_subject_spec(WindowKind::Defenses, 3)
        },
    );
    manager.apply(WindowCommand::Minimize(hidden)).unwrap();
    manager.apply(WindowCommand::CapturePointer(child)).unwrap();
    let before = format!("{manager:?}");
    for owner in [old, child, hidden] {
        let new = WindowSpec {
            owner: WindowOwner::Window(owner),
            ..modeless_subject_spec(WindowKind::System, 4)
        };
        assert_eq!(
            manager.apply(WindowCommand::Replace { old, new }),
            Err(WindowError::InvalidReplacement(old))
        );
        assert_eq!(format!("{manager:?}"), before);
    }
    assert_eq!(manager.validate(), Ok(()));
    assert_eq!(
        open_window(&mut manager, modeless_subject_spec(WindowKind::Missions, 4)).get(),
        4
    );
}

#[test]
fn replacement_rejects_surviving_keys_and_invalid_specs_without_mutation() {
    // port: SW-02R rejects instead of turning replacement into an existing-key reopen.
    // Break caught: committing teardown before key, shape, capacity, policy or placement checks.
    let mut manager = WindowManager::new();
    let old = open_window(&mut manager, modeless_subject_spec(WindowKind::System, 1));
    let fleet_spec = modeless_subject_spec(WindowKind::Fleet, 2);
    let fleet = open_window(&mut manager, fleet_spec);
    open_window(&mut manager, sector_spec(1));
    open_window(&mut manager, sector_spec(2));
    manager.apply(WindowCommand::CapturePointer(fleet)).unwrap();
    let before = format!("{manager:?}");
    let invalid_shape = WindowSpec {
        key: WindowKey::singleton(WindowKind::Missions),
        ..modeless_subject_spec(WindowKind::Missions, 5)
    };
    let wrong_policy = WindowSpec {
        policy: WindowPolicy {
            focus_on_open: FocusOnOpen::Preserve,
            ..fleet_spec.policy
        },
        ..modeless_subject_spec(WindowKind::Fleet, 5)
    };
    let wrong_placement = WindowSpec {
        policy: WindowPolicy {
            placement: OpenPlacement::AboveOwner,
            ..fleet_spec.policy
        },
        ..modeless_subject_spec(WindowKind::Missions, 5)
    };
    for (new, error) in [
        (fleet_spec, WindowError::InvalidReplacement(old)),
        (
            invalid_shape,
            WindowError::SubjectMismatch(invalid_shape.key),
        ),
        (
            sector_spec(3),
            WindowError::CardinalityExceeded(WindowKind::Sector),
        ),
        (
            wrong_policy,
            WindowError::KindPolicyMismatch(WindowKind::Fleet),
        ),
        (wrong_placement, WindowError::InvalidPlacement),
    ] {
        assert_eq!(
            manager.apply(WindowCommand::Replace { old, new }),
            Err(error)
        );
        assert_eq!(format!("{manager:?}"), before);
    }
    assert_eq!(manager.validate(), Ok(()));
    assert_eq!(
        open_window(&mut manager, modeless_subject_spec(WindowKind::Missions, 5)).get(),
        5
    );
}

#[test]
fn replacement_cancels_newly_blocked_surviving_capture_before_teardown() {
    // Source: FUN_00601340 and strategic FUN_00422ce0 capture draining;
    // port: one SW-02R commit orders cancellation before content teardown and final focus.
    // Break caught: concatenating close/open receipts exposes late cancellation or intermediate focus.
    let mut manager = WindowManager::new();
    let old_spec = modeless_subject_spec(WindowKind::System, 1);
    let old = open_window(&mut manager, old_spec);
    let survivor = open_window(&mut manager, modeless_subject_spec(WindowKind::Fleet, 2));
    manager
        .apply(WindowCommand::CapturePointer(survivor))
        .unwrap();
    let focus_before = manager.focused();
    let commit = manager
        .apply(WindowCommand::Replace {
            old,
            new: singleton_spec(WindowKind::SystemFinder),
        })
        .unwrap();
    let new = replaced_id(commit.transition, old);
    assert_eq!(
        commit.events,
        vec![
            WindowEvent::CaptureCancelled {
                window: survivor,
                reason: CaptureCancelReason::ModalOpened
            },
            WindowEvent::Closed {
                window: old,
                key: old_spec.key,
                reason: CloseReason::Replaced
            },
            WindowEvent::FocusChanged {
                previous: focus_before,
                current: Some(FocusTarget {
                    window: new,
                    control: None
                })
            },
        ]
    );
    assert_eq!(manager.visible_order(), &[survivor, new]);
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn replacement_preserves_eligible_surviving_control_focus_and_capture() {
    // port: lifecycle removal must preserve unaffected input state.
    // Break caught: clearing global focus/control or capture while replacing an unrelated window.
    let mut manager = WindowManager::new();
    let old_spec = modeless_subject_spec(WindowKind::System, 1);
    let old = open_window(&mut manager, old_spec);
    let survivor = open_window(&mut manager, modeless_subject_spec(WindowKind::Fleet, 2));
    let focus = FocusTarget {
        window: survivor,
        control: Some(ControlId(17)),
    };
    manager.apply(WindowCommand::Focus(focus)).unwrap();
    manager
        .apply(WindowCommand::CapturePointer(survivor))
        .unwrap();
    let new_spec = WindowSpec {
        policy: WindowPolicy {
            focus_on_open: FocusOnOpen::Preserve,
            ..sector_spec(3).policy
        },
        ..sector_spec(3)
    };
    let commit = manager
        .apply(WindowCommand::Replace { old, new: new_spec })
        .unwrap();
    let new = replaced_id(commit.transition, old);
    assert_eq!(
        commit.events,
        vec![WindowEvent::Closed {
            window: old,
            key: old_spec.key,
            reason: CloseReason::Replaced
        }]
    );
    assert_eq!(manager.visible_order(), &[new, survivor]);
    assert_eq!(manager.focused(), Some(focus));
    assert_eq!(manager.capture_owner(), Some(survivor));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn menu_replacement_retains_saved_control_focus_and_fresh_identity() {
    // Source: FUN_00442380/FUN_00442430 saved menu focus; port: atomic same-key replacement.
    // Break caught: reopening the old ID, losing saved control focus or retaining removed-menu state.
    let mut manager = WindowManager::new();
    let owner = open_window(&mut manager, modeless_subject_spec(WindowKind::System, 1));
    let focus = FocusTarget {
        window: owner,
        control: Some(ControlId(7)),
    };
    manager.apply(WindowCommand::Focus(focus)).unwrap();
    let spec = transient_menu_spec(owner);
    let old = open_window(&mut manager, spec);
    manager.apply(WindowCommand::CapturePointer(old)).unwrap();
    let commit = manager
        .apply(WindowCommand::Replace { old, new: spec })
        .unwrap();
    let new = replaced_id(commit.transition, old);
    assert_eq!(new.get(), 3);
    assert_eq!(manager.id_for(spec.key), Some(new));
    assert_eq!(manager.visible_order(), &[owner, new]);
    assert_eq!(manager.capture_owner(), None);
    assert_eq!(
        manager.apply(WindowCommand::Close(old)),
        Err(WindowError::UnknownWindow(old))
    );
    let close = manager.apply(WindowCommand::Close(new)).unwrap();
    assert_eq!(
        close.transition,
        WindowTransition::Closed {
            root: new,
            closed: vec![new],
            restored_focus: Some(focus)
        }
    );
    assert_eq!(manager.focused(), Some(focus));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn replacement_validates_modal_placement_against_the_post_removal_state() {
    // port: SW-02R consumes ordinary modal placement validation after old descendants are gone.
    // Break caught: checking the pre-removal menu, or skipping placement checks on surviving menus.
    let mut manager = WindowManager::new();
    let owner = open_window(&mut manager, modeless_subject_spec(WindowKind::System, 1));
    let old_menu = open_window(&mut manager, transient_menu_spec(owner));
    let commit = manager
        .apply(WindowCommand::Replace {
            old: old_menu,
            new: singleton_spec(WindowKind::SystemFinder),
        })
        .unwrap();
    let modal = replaced_id(commit.transition, old_menu);
    assert_eq!(manager.visible_order(), &[owner, modal]);
    assert_eq!(manager.validate(), Ok(()));
    manager.apply(WindowCommand::Close(modal)).unwrap();
    let menu = open_window(&mut manager, transient_menu_spec(owner));
    let old = open_window(&mut manager, modeless_subject_spec(WindowKind::Fleet, 4));
    let before = format!("{manager:?}");
    assert_eq!(
        manager.apply(WindowCommand::Replace {
            old,
            new: singleton_spec(WindowKind::SystemFinder)
        }),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(format!("{manager:?}"), before);
    assert!(manager.visible_order().contains(&menu));
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn replacement_of_a_hidden_root_is_fresh_and_stale_roots_are_rejected() {
    // port: minimized identity is lifecycle state, not absence; replacement is fresh even at the same key.
    // Break caught: restricting Replace to visible IDs or silently accepting stale roots.
    let mut manager = WindowManager::new();
    let spec = modeless_subject_spec(WindowKind::System, 1);
    let old = open_window(&mut manager, spec);
    manager.apply(WindowCommand::Minimize(old)).unwrap();
    let commit = manager
        .apply(WindowCommand::Replace { old, new: spec })
        .unwrap();
    let new = replaced_id(commit.transition, old);
    assert_eq!(new.get(), 2);
    assert_eq!(manager.visible_order(), &[new]);
    assert!(manager.presentation_history().is_empty());
    assert_eq!(manager.id_for(spec.key), Some(new));
    let before = format!("{manager:?}");
    assert_eq!(
        manager.apply(WindowCommand::Replace {
            old,
            new: sector_spec(3)
        }),
        Err(WindowError::UnknownWindow(old))
    );
    assert_eq!(format!("{manager:?}"), before);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn fresh_opens_allocate_monotonic_nonzero_ids() {
    // Source: strategic-windowing-toolkit.md, "Conclusions that constrain the port" item 1.
    // Break caught: reusing or failing to advance the manager's session-local allocator.
    let mut manager = WindowManager::new();

    let first = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("first distinct open succeeds")
            .transition,
    );
    let second = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                42,
            )))
            .expect("second distinct open succeeds")
            .transition,
    );

    assert_eq!(first.get(), 1);
    assert_eq!(second.get(), 2);
    assert!(first.get() < second.get());
}

#[test]
fn lookup_order_is_independent_of_paint_order() {
    // Source: strategic-windowing-toolkit.md, "Four separate authorities".
    // Break caught: deriving visible order from registry or semantic-key iteration.
    let mut manager = WindowManager::new();
    let detail = modeless_subject_spec(WindowKind::System, 42);
    let sector = sector_spec(7);

    let detail_id = opened_id(
        manager
            .apply(WindowCommand::Open(detail))
            .expect("detail opens")
            .transition,
    );
    let sector_id = opened_id(
        manager
            .apply(WindowCommand::Open(sector))
            .expect("sector opens")
            .transition,
    );

    assert_eq!(manager.id_for(detail.key), Some(detail_id));
    assert_eq!(manager.id_for(sector.key), Some(sector_id));
    assert_eq!(manager.visible_order(), &[sector_id, detail_id]);
}

#[test]
fn low_band_insertions_do_not_raise_unrelated_normal_windows() {
    // Source: strategic-windowing-toolkit.md, research-gate answer 3.
    // Break caught: interpreting HWND_BOTTOM-style Sector insertion as global activation.
    let mut manager = WindowManager::new();
    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("System opens")
            .transition,
    );
    let fleet_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("Fleet opens")
            .transition,
    );
    let sector_one = opened_id(
        manager
            .apply(WindowCommand::Open(sector_spec(1)))
            .expect("first low-band Sector opens")
            .transition,
    );
    let sector_two = opened_id(
        manager
            .apply(WindowCommand::Open(sector_spec(2)))
            .expect("second low-band Sector opens")
            .transition,
    );

    assert_eq!(
        manager.visible_order(),
        &[sector_two, sector_one, system_id, fleet_id]
    );
}

#[test]
fn transient_open_stays_above_registered_owner_and_normal_band() {
    // Source: strategic-windowing-toolkit.md, research-gate answer 3.
    // Break caught: treating an owner-relative transient as ordinary Normal-band content.
    let mut manager = WindowManager::new();
    let sector_id = opened_id(
        manager
            .apply(WindowCommand::Open(sector_spec(7)))
            .expect("Sector opens")
            .transition,
    );
    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("System opens")
            .transition,
    );
    let fleet_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("unrelated Fleet opens")
            .transition,
    );
    let menu_id = opened_id(
        manager
            .apply(WindowCommand::Open(transient_menu_spec(system_id)))
            .expect("owned transient opens")
            .transition,
    );

    assert_eq!(
        manager.visible_order(),
        &[sector_id, system_id, fleet_id, menu_id]
    );
}

#[test]
fn background_detail_open_cannot_overtake_an_active_modal() {
    // Source: strategic-windowing-toolkit.md, research-gate answers 3 and 4.
    // Break caught: conflating paint bands with the active strategic input boundary.
    let mut manager = WindowManager::new();
    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("System opens")
            .transition,
    );
    let finder_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton_spec(
                WindowKind::SystemFinder,
            )))
            .expect("modal Finder opens")
            .transition,
    );
    assert_eq!(
        manager.apply(WindowCommand::Open(transient_menu_spec(system_id))),
        Err(WindowError::InvalidPlacement)
    );
    let menu_id = opened_id(
        manager
            .apply(WindowCommand::Open(transient_menu_spec(finder_id)))
            .expect("a transient owned by the active modal is eligible")
            .transition,
    );
    let fleet_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("background Fleet detail still registers")
            .transition,
    );

    assert_eq!(
        manager.visible_order(),
        &[system_id, fleet_id, finder_id, menu_id]
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn modal_open_rejects_an_ineligible_existing_transient_atomically() {
    // Source: strategic-windowing-toolkit.md, research-gate answers 3 and 4.
    // Break caught: placing a new modal below a transient owned by its underlay.
    let mut manager = WindowManager::new();
    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("System opens")
            .transition,
    );
    let menu_id = opened_id(
        manager
            .apply(WindowCommand::Open(transient_menu_spec(system_id)))
            .expect("System-owned menu opens")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    let finder = singleton_spec(WindowKind::SystemFinder);
    assert_eq!(
        manager.apply(WindowCommand::Open(finder)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.id_for(finder.key), None);
    assert_eq!(manager.visible_order(), before);

    let fleet_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("rejection does not consume an identity")
            .transition,
    );
    assert_eq!(fleet_id.get(), 3);
    assert_eq!(manager.visible_order(), &[system_id, fleet_id, menu_id]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn transient_owned_by_lower_modal_cannot_overtake_active_modal() {
    // Source: implementation plan, Task 2 active-modal order boundary.
    // Break caught: using the lowest modal instead of the topmost active modal for eligibility.
    let mut manager = WindowManager::new();
    let lower_modal_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton_spec(
                WindowKind::SystemFinder,
            )))
            .expect("first modal opens")
            .transition,
    );
    let active_modal_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton_spec(WindowKind::FleetFinder)))
            .expect("second modal becomes active")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    assert_eq!(
        manager.apply(WindowCommand::Open(transient_menu_spec(lower_modal_id))),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.visible_order(), before);

    let menu_id = opened_id(
        manager
            .apply(WindowCommand::Open(transient_menu_spec(active_modal_id)))
            .expect("the active modal may own a transient")
            .transition,
    );
    assert_eq!(
        manager.visible_order(),
        &[lower_modal_id, active_modal_id, menu_id]
    );
    assert_eq!(manager.validate(), Ok(()));
    assert_eq!(menu_id.get(), 3);
}

#[test]
fn lower_modal_owned_modeless_open_stays_below_active_modal() {
    // Source: implementation plan, Task 2 active-modal order boundary.
    // Break caught: treating any modal owner as eligible to overtake a newer modal.
    let mut manager = WindowManager::new();
    let lower_modal_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton_spec(
                WindowKind::SystemFinder,
            )))
            .expect("first modal opens")
            .transition,
    );
    let active_modal_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton_spec(WindowKind::FleetFinder)))
            .expect("second modal becomes active")
            .transition,
    );
    let child = WindowSpec {
        owner: WindowOwner::Window(lower_modal_id),
        ..modeless_subject_spec(WindowKind::System, 42)
    };
    let child_id = opened_id(
        manager
            .apply(WindowCommand::Open(child))
            .expect("lower-modal child opens below active modal")
            .transition,
    );

    assert_eq!(
        manager.visible_order(),
        &[lower_modal_id, child_id, active_modal_id]
    );
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn window_owned_open_cannot_commit_below_its_owner() {
    // Source: implementation plan, Task 2 owner/order validation.
    // Break caught: accepting an owner link whose band placement contradicts child-above-owner.
    let mut manager = WindowManager::new();
    let owner_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("owner opens")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    let mut below_owner = modeless_subject_spec(WindowKind::Fleet, 42);
    below_owner.owner = WindowOwner::Window(owner_id);
    below_owner.policy.band = StackBand::Low;
    below_owner.policy.placement = OpenPlacement::BottomOfBand;
    assert_eq!(
        manager.apply(WindowCommand::Open(below_owner)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.id_for(below_owner.key), None);
    assert_eq!(manager.visible_order(), before);

    let valid_child = WindowSpec {
        owner: WindowOwner::Window(owner_id),
        policy: WindowPolicy {
            band: StackBand::Normal,
            placement: OpenPlacement::TopOfBand,
            ..below_owner.policy
        },
        ..below_owner
    };
    let child_id = opened_id(
        manager
            .apply(WindowCommand::Open(valid_child))
            .expect("compatible owner-relative order opens")
            .transition,
    );
    assert_eq!(child_id.get(), 2);
    assert_eq!(manager.visible_order(), &[owner_id, child_id]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn raising_an_owner_keeps_its_visible_subtree_above_it() {
    // Source: implementation plan, Task 2 owner/order and RaiseRestore contracts.
    // Break caught: raising a parent alone above its child and invalidating owner order.
    let mut manager = WindowManager::new();
    let parent = modeless_subject_spec(WindowKind::System, 41);
    let parent_id = opened_id(
        manager
            .apply(WindowCommand::Open(parent))
            .expect("parent opens")
            .transition,
    );
    let child = WindowSpec {
        owner: WindowOwner::Window(parent_id),
        ..modeless_subject_spec(WindowKind::Fleet, 42)
    };
    let child_id = opened_id(
        manager
            .apply(WindowCommand::Open(child))
            .expect("child opens above parent")
            .transition,
    );
    let sibling_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Defenses,
                43,
            )))
            .expect("unrelated sibling opens")
            .transition,
    );
    assert_eq!(manager.visible_order(), &[parent_id, child_id, sibling_id]);

    assert_eq!(
        manager
            .apply(WindowCommand::Open(parent))
            .expect("reopening parent raises its subtree")
            .transition,
        WindowTransition::Existing {
            id: parent_id,
            effect: ExistingEffect::Raised,
        }
    );
    assert_eq!(manager.visible_order(), &[sibling_id, parent_id, child_id]);
    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn open_order_validation_accepts_registry_key_and_band_invariants() {
    // Source: implementation plan, Task 2 order-only validation contract.
    // Break caught: a successful open sequence leaving divergent registry/index/order state.
    let mut manager = WindowManager::new();
    manager
        .apply(WindowCommand::Open(sector_spec(7)))
        .expect("Sector opens");
    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                41,
            )))
            .expect("System opens")
            .transition,
    );
    manager
        .apply(WindowCommand::Open(transient_menu_spec(system_id)))
        .expect("transient menu opens");

    assert_eq!(manager.validate(), Ok(()));
}

#[test]
fn subject_cardinality_and_singleton_shape_are_enforced() {
    // Source: strategic-windowing-toolkit.md, "Existing-window policy" and family matrix.
    // Break caught: admitting a subject on a singleton, omitting a required subject, or
    // exceeding a subject family's declared capacity.
    let mut manager = WindowManager::new();
    let mut contextual_singleton = singleton_spec(WindowKind::FleetFinder);
    contextual_singleton.key =
        WindowKey::contextual(WindowKind::FleetFinder, SubjectId::new(2, 41));
    assert_eq!(
        manager.apply(WindowCommand::Open(contextual_singleton)),
        Err(WindowError::SubjectMismatch(contextual_singleton.key))
    );

    let singleton = singleton_spec(WindowKind::FleetFinder);
    let singleton_id = opened_id(
        manager
            .apply(WindowCommand::Open(singleton))
            .expect("a correctly shaped singleton opens")
            .transition,
    );
    assert_eq!(singleton_id.get(), 1);

    let mut subjectless_detail = modeless_subject_spec(WindowKind::System, 41);
    subjectless_detail.key = WindowKey::singleton(WindowKind::System);
    assert_eq!(
        manager.apply(WindowCommand::Open(subjectless_detail)),
        Err(WindowError::SubjectMismatch(subjectless_detail.key))
    );

    manager
        .apply(WindowCommand::Open(sector_spec(1)))
        .expect("first Sector opens");
    manager
        .apply(WindowCommand::Open(sector_spec(2)))
        .expect("second Sector opens");
    assert_eq!(
        manager.apply(WindowCommand::Open(sector_spec(3))),
        Err(WindowError::CardinalityExceeded(WindowKind::Sector))
    );
}

#[test]
fn open_order_atomic_rejection_preserves_allocator() {
    // Source: implementation plan, Task 2 atomic-open contract.
    // Break caught: allocating or partially indexing a spec before placement validation.
    let mut manager = WindowManager::new();
    let first = modeless_subject_spec(WindowKind::System, 41);
    let first_id = opened_id(
        manager
            .apply(WindowCommand::Open(first))
            .expect("valid detail opens")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    let mut invalid = modeless_subject_spec(WindowKind::Fleet, 42);
    invalid.policy.placement = OpenPlacement::AboveOwner;
    assert_eq!(
        manager.apply(WindowCommand::Open(invalid)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.id_for(invalid.key), None);
    assert_eq!(manager.id_for(first.key), Some(first_id));
    assert_eq!(manager.visible_order(), before);

    let mut transient_without_owner_placement = transient_menu_spec(first_id);
    transient_without_owner_placement.policy.placement = OpenPlacement::TopOfBand;
    assert_eq!(
        manager.apply(WindowCommand::Open(transient_without_owner_placement)),
        Err(WindowError::InvalidPlacement)
    );
    assert_eq!(manager.id_for(transient_without_owner_placement.key), None);
    assert_eq!(manager.visible_order(), before);

    let mut foreign = WindowManager::new();
    foreign
        .apply(WindowCommand::Open(modeless_subject_spec(
            WindowKind::System,
            91,
        )))
        .expect("foreign first ID opens");
    let foreign_second_id = opened_id(
        foreign
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                92,
            )))
            .expect("foreign second ID opens")
            .transition,
    );
    let unknown_owner = transient_menu_spec(foreign_second_id);
    assert_eq!(
        manager.apply(WindowCommand::Open(unknown_owner)),
        Err(WindowError::UnknownOwner(foreign_second_id))
    );
    assert_eq!(manager.id_for(unknown_owner.key), None);
    assert_eq!(manager.visible_order(), before);

    let second_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("rejection does not consume an identity")
            .transition,
    );
    assert_eq!(second_id.get(), 2);
}

#[test]
fn open_order_duplicate_noop_preserves_order() {
    // Source: strategic-windowing-toolkit.md, "Existing-window policy".
    // Break caught: duplicate NoOp allocating, reordering, or replacing identity.
    let mut manager = WindowManager::new();
    let sector = sector_spec(7);
    let sector_id = opened_id(
        manager
            .apply(WindowCommand::Open(sector))
            .expect("initial Sector opens")
            .transition,
    );
    let detail_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::System,
                42,
            )))
            .expect("detail opens above Sector")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    let duplicate = manager
        .apply(WindowCommand::Open(sector))
        .expect("compatible duplicate is a successful no-op");

    assert_eq!(
        duplicate.transition,
        WindowTransition::Existing {
            id: sector_id,
            effect: ExistingEffect::NoOp,
        }
    );
    assert_eq!(manager.id_for(sector.key), Some(sector_id));
    assert_eq!(manager.visible_order(), before);

    let next_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("a later fresh open still gets the next identity")
            .transition,
    );
    assert_eq!((sector_id.get(), detail_id.get(), next_id.get()), (1, 2, 3));
}

#[test]
fn duplicate_sector_and_singleton_opens_preserve_order_while_detail_reopen_raises() {
    // Source: strategic-windowing-toolkit.md, "Existing-window policy".
    // Break caught: applying one generic duplicate-open rule across distinct families.
    let mut manager = WindowManager::new();
    let system = modeless_subject_spec(WindowKind::System, 41);
    let fleet = modeless_subject_spec(WindowKind::Fleet, 42);
    let sector = sector_spec(7);
    let finder = singleton_spec(WindowKind::FleetFinder);

    let system_id = opened_id(
        manager
            .apply(WindowCommand::Open(system))
            .expect("System opens")
            .transition,
    );
    let fleet_id = opened_id(
        manager
            .apply(WindowCommand::Open(fleet))
            .expect("Fleet opens")
            .transition,
    );
    let sector_id = opened_id(
        manager
            .apply(WindowCommand::Open(sector))
            .expect("Sector opens")
            .transition,
    );
    let finder_id = opened_id(
        manager
            .apply(WindowCommand::Open(finder))
            .expect("Finder opens")
            .transition,
    );
    assert_eq!(
        manager.visible_order(),
        &[sector_id, system_id, fleet_id, finder_id]
    );

    let order_before_noops = manager.visible_order().to_vec();
    assert_eq!(
        manager
            .apply(WindowCommand::Open(sector))
            .expect("duplicate Sector is accepted")
            .transition,
        WindowTransition::Existing {
            id: sector_id,
            effect: ExistingEffect::NoOp,
        }
    );
    assert_eq!(
        manager
            .apply(WindowCommand::Open(finder))
            .expect("duplicate singleton is accepted")
            .transition,
        WindowTransition::Existing {
            id: finder_id,
            effect: ExistingEffect::NoOp,
        }
    );
    assert_eq!(manager.visible_order(), order_before_noops);

    assert_eq!(
        manager
            .apply(WindowCommand::Open(system))
            .expect("detail reopen raises the same identity")
            .transition,
        WindowTransition::Existing {
            id: system_id,
            effect: ExistingEffect::Raised,
        }
    );
    assert_eq!(
        manager.visible_order(),
        &[sector_id, fleet_id, system_id, finder_id]
    );
}

#[test]
fn existing_key_rejects_different_owner_or_policy_atomically() {
    // Source: implementation plan, Task 2 atomic-open contract.
    // Break caught: accepting a duplicate key that silently changes ownership or policy.
    let mut manager = WindowManager::new();
    let sector = sector_spec(7);
    let sector_id = opened_id(
        manager
            .apply(WindowCommand::Open(sector))
            .expect("Sector opens")
            .transition,
    );
    let detail = modeless_subject_spec(WindowKind::System, 42);
    let detail_id = opened_id(
        manager
            .apply(WindowCommand::Open(detail))
            .expect("detail opens")
            .transition,
    );
    let before = manager.visible_order().to_vec();

    let different_owner = WindowSpec {
        owner: WindowOwner::Window(sector_id),
        ..detail
    };
    assert_eq!(
        manager.apply(WindowCommand::Open(different_owner)),
        Err(WindowError::DuplicateKeyWithDifferentOwner(detail.key))
    );

    let different_policy = WindowSpec {
        policy: WindowPolicy {
            input: InputPolicy::StrategicModal,
            ..detail.policy
        },
        ..detail
    };
    assert_eq!(
        manager.apply(WindowCommand::Open(different_policy)),
        Err(WindowError::DuplicateKeyWithDifferentPolicy(detail.key))
    );

    assert_eq!(manager.id_for(detail.key), Some(detail_id));
    assert_eq!(manager.visible_order(), before);
    let next_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                42,
            )))
            .expect("rejections do not consume an identity")
            .transition,
    );
    assert_eq!(next_id.get(), 3);
}

#[test]
fn same_kind_cannot_bypass_capacity_by_changing_policy() {
    // Source: implementation plan, Task 2 family-policy validation.
    // Break caught: treating caller-supplied policy as authority after a family is registered.
    let mut manager = WindowManager::new();
    manager
        .apply(WindowCommand::Open(sector_spec(1)))
        .expect("first Sector opens");
    manager
        .apply(WindowCommand::Open(sector_spec(2)))
        .expect("second Sector opens");
    let before = manager.visible_order().to_vec();

    let mut bypass = sector_spec(3);
    bypass.policy.cardinality = Cardinality::Subject { max_open: None };
    assert_eq!(
        manager.apply(WindowCommand::Open(bypass)),
        Err(WindowError::KindPolicyMismatch(WindowKind::Sector))
    );
    assert_eq!(manager.id_for(bypass.key), None);
    assert_eq!(manager.visible_order(), before);

    let next_id = opened_id(
        manager
            .apply(WindowCommand::Open(modeless_subject_spec(
                WindowKind::Fleet,
                3,
            )))
            .expect("rejection does not consume an identity")
            .transition,
    );
    assert_eq!(next_id.get(), 3);
}

#[test]
fn subject_namespaces_do_not_alias_equal_numeric_values() {
    // Source: strategic-windowing-toolkit.md, "Conclusions that constrain the port" item 1.
    let sector = SubjectId::new(1, 42);
    let system = SubjectId::new(2, 42);

    assert_ne!(sector, system);
    assert_eq!(BTreeSet::from([sector, system]).len(), 2);
}

#[test]
fn window_keys_distinguish_kind_and_subject() {
    // Source: strategic-windowing-toolkit.md, "Window-family matrix".
    assert_opaque_id_traits::<WindowId>();

    let system_42 = WindowKey::contextual(WindowKind::System, SubjectId::new(2, 42));
    let fleet_42 = WindowKey::contextual(WindowKind::Fleet, SubjectId::new(2, 42));
    let system_43 = WindowKey::contextual(WindowKind::System, SubjectId::new(2, 43));
    let singleton = WindowKey::singleton(WindowKind::SystemFinder);

    assert_eq!(
        BTreeSet::from([system_42, fleet_42, system_43, singleton]).len(),
        4
    );
}

#[test]
fn all_window_kinds_are_unique_and_cover_the_public_inventory() {
    // Source: strategic-windowing-toolkit.md, "Window-family matrix" and its explicit exclusions.
    let expected = [
        WindowKind::Sector,
        WindowKind::System,
        WindowKind::Fleet,
        WindowKind::Defenses,
        WindowKind::Missions,
        WindowKind::SystemFinder,
        WindowKind::FleetFinder,
        WindowKind::TroopFinder,
        WindowKind::PersonnelFinder,
        WindowKind::MessageIndex,
        WindowKind::Encyclopedia,
        WindowKind::Status,
        WindowKind::BuildSelection,
        WindowKind::MissionDialog,
        WindowKind::MoveConfirmation,
        WindowKind::ScrapConfirmation,
        WindowKind::BattleAlert,
        WindowKind::BattleResults,
        WindowKind::Menu,
    ];

    assert_eq!(WindowKind::ALL, expected);
    assert_eq!(BTreeSet::from(expected).len(), 19);
}

#[test]
fn policy_axes_do_not_encode_modality_in_stack_band() {
    // Source: strategic-windowing-toolkit.md, "Modality and input routing".
    let modeless = WindowPolicy {
        cardinality: Cardinality::Subject { max_open: None },
        band: StackBand::Normal,
        placement: OpenPlacement::TopOfBand,
        existing: ExistingOpenPolicy::RaiseRestore,
        input: InputPolicy::Modeless,
        presentation: PresentationPolicy::History,
        focus_on_open: FocusOnOpen::Window,
        restoration: FocusRestoration::PreviousEligible,
    };
    let strategic_modal = WindowPolicy {
        input: InputPolicy::StrategicModal,
        ..modeless
    };

    assert_eq!(modeless.band, strategic_modal.band);
    assert_ne!(modeless.input, strategic_modal.input);
    assert_ne!(modeless, strategic_modal);
}
