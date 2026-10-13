use rebellion_windowing::{
    Cardinality, ControlId, ExistingOpenPolicy, FocusOnOpen, FocusRestoration, FocusTarget,
    InputPolicy, OpenPlacement, PresentationPolicy, StackBand, SubjectId, WindowCommand,
    WindowError, WindowId, WindowKey, WindowKind, WindowManager, WindowOwner, WindowPolicy,
    WindowSpec, WindowTransition,
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
