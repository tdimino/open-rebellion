//! One focus across the galaxy view's child windows. The original's sector,
//! System, Fleet, Defenses and Missions windows are children of one galaxy
//! window (its `+0x6c` registry), so their z-order is one Win32 order
//! whatever their kind. Each window remembers when it last came to the
//! front; the highest number is on top.

use std::sync::atomic::{AtomicU64, Ordering};

use rebellion_core::ids::{SectorKey, SystemKey};
use rebellion_windowing::{RoutingSnapshot, WindowCommand, WindowId};

use crate::cockpit::GidUiState;
use crate::defenses_window::DefensesWindowState;
use crate::fleet_window::FleetWindowState;
use crate::missions_window::MissionsWindowState;
use crate::sector_window::SectorWindowState;
use crate::system_window::SystemWindowState;

static NEXT: AtomicU64 = AtomicU64::new(1);

/// A number larger than every one handed out before.
pub(crate) fn next_serial() -> u64 {
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// One child window of the galaxy view, by kind and subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GalaxyChild {
    Sector(SectorKey),
    System(SystemKey),
    Defenses(SystemKey),
    Missions(SystemKey),
    Fleet(WindowId),
}

/// The topmost of `children` by z serial: where the galaxy view's
/// activation (`FUN_00422ce0` 0x467 with no window) lands, its walk from
/// `GetWindow(view, GW_CHILD)` taking the first registered window that is
/// not minimized. Minimized windows are not among `children`.
#[must_use]
pub fn topmost(children: impl IntoIterator<Item = (u64, GalaxyChild)>) -> Option<GalaxyChild> {
    children
        .into_iter()
        .max_by_key(|(serial, _)| *serial)
        .map(|(_, child)| child)
}

/// Where the keyboard focus is among the galaxy view's windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GalaxyFocus {
    /// The GID menu: built on the view's first GID press (`FUN_00422ce0`
    /// 0x132 → `FUN_00451130`), its `WM_CREATE` step `FUN_004511e0` ends with
    /// `SetFocus` on it; later presses only show it.
    Menu,
    Child(GalaxyChild),
    /// The galaxy view itself, whose `WM_KEYDOWN` has no Escape.
    View,
}

/// What the galaxy view does to a child after a focus change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusStep {
    /// The view's activation (`FUN_00422ce0` 0x467): `ShowWindow` 5,
    /// `BringWindowToTop` and `SetFocus`.
    Raise(GalaxyChild),
    /// A window just opened: created visible (`FUN_0045aac0` slot `+0x2c`,
    /// `FUN_00429ce0`), without taking the focus.
    Show(GalaxyChild),
    /// The keyed close (slot `+0x30`, `FUN_00606960` posting 0x405).
    Close(GalaxyChild),
}

/// The galaxy view's keyboard focus and its last activated child
/// (`param_1[0x2e]`), tracked across the open windows of every kind.
#[derive(Debug, Clone)]
pub struct GalaxyFocusState {
    pub focus: GalaxyFocus,
    /// The view's current child (`param_1[0x2e]`).
    pub current: Option<GalaxyChild>,
    children: Vec<(u64, GalaxyChild)>,
}

impl Default for GalaxyFocusState {
    /// A new game, or a load that restores no window.
    fn default() -> Self {
        Self::after_load(Vec::new())
    }
}

impl GalaxyFocusState {
    /// A loaded game with the windows its save restored: the focus and the
    /// view's current child are the top restored window that is shown and
    /// not a sector window (the loader sends sector windows to
    /// `HWND_BOTTOM`), or the view with none. Under Wine,
    /// empire-fleet-to-yaga-minor loaded at day 38 put the focus on
    /// Coruscant, over Sesswenna: its first Escape closed it (runs r, t3,
    /// w1); the first GID press then moved the focus to the menu
    /// (`FUN_00451130` → `FUN_004511e0`; runs t1, u).
    #[must_use]
    pub fn after_load(restored: Vec<(u64, GalaxyChild)>) -> Self {
        let top = topmost(
            restored
                .iter()
                .copied()
                .filter(|(_, child)| !matches!(child, GalaxyChild::Sector(_))),
        );
        Self {
            focus: top.map_or(GalaxyFocus::View, GalaxyFocus::Child),
            current: top,
            children: restored,
        }
    }

    /// A GID press: `FUN_00422ce0` 0x132 hides every child (slot `+8`),
    /// and Windows moves the focus off a focused window `ShowWindow` hides,
    /// to the view, which has no Escape (under Wine on
    /// empire-fleet-to-yaga-minor at day 38: after a click in Coruscant and a
    /// second GID press, Escape closed nothing). The press that builds the
    /// menu then gives it the focus (`FUN_004511e0`).
    pub fn gid_opened(&mut self, built_now: bool) {
        if matches!(self.focus, GalaxyFocus::Child(_)) {
            self.focus = GalaxyFocus::View;
        }
        if built_now {
            self.focus = GalaxyFocus::Menu;
        }
    }

    /// Make `child` the focus and the view's current child (0x467 for it).
    pub fn activate(&mut self, child: GalaxyChild) {
        self.focus = GalaxyFocus::Child(child);
        self.current = Some(child);
    }

    /// 0x467 with no window: with no child ever activated the focus goes to
    /// the view; otherwise the walk from the top of the z-order takes the
    /// first window not minimized, and with none the view takes the focus.
    fn activate_top(&mut self) -> Option<FocusStep> {
        if self.current.is_none() {
            self.focus = GalaxyFocus::View;
            return None;
        }
        let Some(child) = topmost(self.children.iter().copied()) else {
            self.current = None;
            self.focus = GalaxyFocus::View;
            return None;
        };
        self.activate(child);
        Some(FocusStep::Raise(child))
    }

    /// Follow the frame's open windows. A new window leaves the focus where
    /// it was (`FUN_0045aac0` and `FUN_00429ce0` neither focus nor activate
    /// it) and is shown; a closed one makes the view activate the next
    /// (0x405 → 0x467), whatever else opened in the same frame, as does a
    /// focus left on a window that is gone; a window a click raised takes the
    /// focus (`FUN_004534f0`: 0x467 for itself on the press, `SetFocus` on
    /// the release).
    pub fn sync(&mut self, now: Vec<(u64, GalaxyChild)>) -> Vec<FocusStep> {
        let before = std::mem::replace(&mut self.children, now);
        let had = |child: GalaxyChild| before.iter().any(|(_, held)| *held == child);
        let mut opened: Vec<(u64, GalaxyChild)> = self
            .children
            .iter()
            .copied()
            .filter(|(_, child)| !had(*child))
            .collect();
        opened.sort_by_key(|(serial, _)| *serial);
        let mut steps: Vec<FocusStep> = opened
            .into_iter()
            .map(|(_, child)| FocusStep::Show(child))
            .collect();
        let present = |child: &GalaxyChild| self.children.iter().any(|(_, held)| held == child);
        let closed = before.iter().any(|(_, child)| !present(child));
        let focus_gone = matches!(self.focus, GalaxyFocus::Child(child) if !present(&child));
        if closed || focus_gone {
            steps.extend(self.activate_top());
            return steps;
        }
        if !steps.is_empty() {
            return steps;
        }
        let raised = self
            .children
            .iter()
            .filter(|(serial, child)| {
                before
                    .iter()
                    .any(|(old, held)| held == child && old < serial)
            })
            .max_by_key(|(serial, _)| *serial)
            .map(|(_, child)| *child);
        if let Some(child) = raised {
            self.activate(child);
        }
        steps
    }

    /// Escape where the focus is. The menu (`FUN_004aab50`) gives the focus
    /// to the view and posts 0x405 for its own key, which closes nothing, so
    /// the view's 0x467 shows the top window. A detail window closes
    /// (`FUN_004ac3a0`, `FUN_00458980` for the System window); a hidden one
    /// never has the focus (`gid_opened`). A sector window passes
    /// the key to the view (`FUN_004593e0`), which has no Escape. A Fleet
    /// window closes as its close button does (slot `+0x30`, `FUN_004a7390`
    /// -> `FUN_004ac3a0`), through the window manager.
    pub fn escape(&mut self) -> Option<FocusStep> {
        match self.focus {
            GalaxyFocus::Menu => {
                self.focus = GalaxyFocus::View;
                self.activate_top()
            }
            GalaxyFocus::Child(
                child @ (GalaxyChild::System(_)
                | GalaxyChild::Defenses(_)
                | GalaxyChild::Missions(_)
                | GalaxyChild::Fleet(_)),
            ) => Some(FocusStep::Close(child)),
            GalaxyFocus::Child(GalaxyChild::Sector(_)) | GalaxyFocus::View => None,
        }
    }
}

/// Every open galaxy child window with its z serial.
#[must_use]
pub fn galaxy_children(
    sectors: &SectorWindowState,
    systems: &SystemWindowState,
    defenses: &DefensesWindowState,
    missions: &MissionsWindowState,
    fleets: &FleetWindowState,
    snapshot: &RoutingSnapshot,
) -> Vec<(u64, GalaxyChild)> {
    sectors
        .children()
        .chain(systems.children())
        .chain(defenses.children())
        .chain(missions.children())
        .chain(fleets.children(snapshot))
        .collect()
}

/// Carry out a focus step on the window it names. A Fleet window is the
/// window manager's, so its activation and its close go to it through
/// `fleet` as manager commands.
pub fn apply_focus_step(
    step: FocusStep,
    gid: &mut GidUiState,
    sectors: &mut SectorWindowState,
    systems: &mut SystemWindowState,
    defenses: &mut DefensesWindowState,
    missions: &mut MissionsWindowState,
    fleet: &mut dyn FnMut(WindowCommand),
) {
    match step {
        FocusStep::Show(child) => gid.show_child(child),
        FocusStep::Raise(child) => {
            gid.show_child(child);
            match child {
                GalaxyChild::Sector(sector) => {
                    sectors.raise(sector);
                }
                GalaxyChild::System(system) => {
                    systems.raise(system);
                }
                GalaxyChild::Defenses(system) => {
                    defenses.raise(system);
                }
                GalaxyChild::Missions(system) => {
                    missions.raise(system);
                }
                GalaxyChild::Fleet(window) => fleet(WindowCommand::Activate(window)),
            }
        }
        FocusStep::Close(child) => match child {
            GalaxyChild::System(system) => {
                systems.close_window(system);
            }
            GalaxyChild::Defenses(system) => {
                defenses.close_window(system);
            }
            GalaxyChild::Missions(system) => {
                missions.close_window(system);
            }
            GalaxyChild::Fleet(window) => fleet(WindowCommand::Close(window)),
            GalaxyChild::Sector(_) => {}
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::{CockpitFaction, CockpitState};
    use rebellion_core::world::GameWorld;

    /// A sector window and two System windows' identities.
    fn children() -> (GalaxyChild, GalaxyChild, GalaxyChild) {
        let mut world = GameWorld::default();
        let sector = world
            .sectors
            .insert(rebellion_core::world::Sector::default());
        let first = world
            .systems
            .insert(rebellion_core::world::System::default());
        let second = world
            .systems
            .insert(rebellion_core::world::System::default());
        (
            GalaxyChild::Sector(sector),
            GalaxyChild::System(first),
            GalaxyChild::System(second),
        )
    }

    /// empire-fleet-to-yaga-minor loaded at day 38: the save restores the
    /// Sesswenna sector window and the Coruscant System window over it.
    fn loaded_yaga() -> (GalaxyFocusState, GalaxyChild, GalaxyChild) {
        let (sesswenna, coruscant, _) = children();
        let focus = GalaxyFocusState::after_load(vec![(0, sesswenna), (5, coruscant)]);
        (focus, sesswenna, coruscant)
    }

    #[test]
    fn a_load_focuses_the_top_restored_window_that_is_not_a_sector() {
        // Runs r, t3 and w1 under Wine (empire-fleet-to-yaga-minor, day 38):
        // the focus is on Coruscant, over Sesswenna. A sector window alone,
        // or no window (alliance-day54-advice, day 61), leaves it on the view.
        let (focus, _, coruscant) = loaded_yaga();
        assert_eq!(focus.focus, GalaxyFocus::Child(coruscant));
        assert_eq!(focus.current, Some(coruscant));
        let (sector, _, _) = children();
        let alone = GalaxyFocusState::after_load(vec![(9, sector)]);
        assert_eq!((alone.focus, alone.current), (GalaxyFocus::View, None));
        let none = GalaxyFocusState::default();
        assert_eq!((none.focus, none.current), (GalaxyFocus::View, None));
    }

    /// A GID press, as the GID control dispatches it.
    fn press_gid(focus: &mut GalaxyFocusState, gid: &mut GidUiState) {
        let built_now = gid.open_menu();
        focus.gid_opened(built_now);
    }

    #[test]
    fn with_the_menu_never_opened_escape_closes_the_restored_top_window_then_reaches_the_sector() {
        // Runs r and t3 under Wine (empire-fleet-to-yaga-minor, day 38):
        // Escape #1 closed Coruscant at once (FUN_00458980 → slot +0x30 →
        // 0x405) and the view's 0x467 focused Sesswenna, whose Escape (#2)
        // did nothing (FUN_004593e0 passes it to the view).
        let (mut focus, sesswenna, coruscant) = loaded_yaga();
        assert_eq!(focus.escape(), Some(FocusStep::Close(coruscant)));
        assert_eq!(
            focus.sync(vec![(0, sesswenna)]),
            vec![FocusStep::Raise(sesswenna)]
        );
        assert_eq!(focus.escape(), None);
    }

    #[test]
    fn a_focused_fleet_windows_escape_closes_it_through_the_manager() {
        // fleet-window.md, the window procedure's controls: the close button
        // 0x65 and Escape both run slot +0x30 (FUN_004a7390 -> FUN_004ac3a0),
        // the close a System window's Escape runs too.
        use rebellion_windowing::{
            SubjectId, WindowKind, WindowManager, WindowOwner, WindowTransition,
        };
        let mut manager = WindowManager::new();
        let commit = manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::contextual_spec(
                    WindowKind::Fleet,
                    SubjectId::new(2, 1),
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
        let WindowTransition::Opened { id } = commit.transition else {
            panic!("expected Fleet open");
        };
        let fleet = GalaxyChild::Fleet(id);
        let (sesswenna, _, _) = children();
        let mut focus = GalaxyFocusState::after_load(vec![(0, sesswenna), (5, fleet)]);
        let step = focus.escape();
        assert_eq!(step, Some(FocusStep::Close(fleet)));

        let mut commands = Vec::new();
        apply_focus_step(
            step.unwrap(),
            &mut GidUiState::default(),
            &mut SectorWindowState::default(),
            &mut SystemWindowState::default(),
            &mut DefensesWindowState::default(),
            &mut MissionsWindowState::default(),
            &mut |command| commands.push(command),
        );
        assert_eq!(commands, [WindowCommand::Close(id)]);
    }

    #[test]
    fn a_click_in_a_system_window_keeps_its_escape_closing_it() {
        // Run w1 under Wine (empire-fleet-to-yaga-minor, day 38): a click in
        // Coruscant's body (FUN_004534f0: 0x467 for itself on the press,
        // SetFocus on the release), then Escape closed it.
        let (mut focus, sesswenna, coruscant) = loaded_yaga();
        assert_eq!(
            focus.sync(vec![(0, sesswenna), (6, coruscant)]),
            Vec::<FocusStep>::new()
        );
        assert_eq!(focus.escape(), Some(FocusStep::Close(coruscant)));
    }

    #[test]
    fn the_first_gid_press_gives_the_menu_the_focus_whose_escape_shows_the_top_window() {
        // Runs t1 and n (no click) and p (a click in Coruscant first) under
        // Wine, empire-fleet-to-yaga-minor, days 38 to 45. The first press
        // builds the menu, whose FUN_004511e0 takes the focus; its Escape
        // (FUN_004aab50) posts 0x405 for the menu's own key and the view's
        // 0x467 shows Coruscant over the open menu (#1); Coruscant's Escape
        // closes it and the next 0x467 shows Sesswenna (#2); the sector
        // window's Escape does nothing (#3).
        for click in [false, true] {
            let (mut focus, sesswenna, coruscant) = loaded_yaga();
            if click {
                assert_eq!(
                    focus.sync(vec![(0, sesswenna), (6, coruscant)]),
                    Vec::<FocusStep>::new()
                );
            }
            let mut gid = GidUiState::default();
            press_gid(&mut focus, &mut gid);
            assert_eq!(focus.focus, GalaxyFocus::Menu, "click {click}");

            assert_eq!(focus.escape(), Some(FocusStep::Raise(coruscant)));
            gid.show_child(coruscant);
            assert!(gid.menu_open && !gid.hides(coruscant) && gid.hides(sesswenna));
            assert_eq!(focus.escape(), Some(FocusStep::Close(coruscant)));
            let left = vec![(0, sesswenna)];
            assert_eq!(focus.sync(left), vec![FocusStep::Raise(sesswenna)]);
            gid.show_child(sesswenna);
            assert_eq!(focus.escape(), None, "click {click}");
            assert!(gid.menu_open);
        }
    }

    #[test]
    fn a_later_gid_press_takes_the_focus_from_the_window_it_hides() {
        // Run u under Wine (empire-fleet-to-yaga-minor, day 38): first GID
        // press, a galaxy press, a click in Coruscant, a second GID press,
        // Escape: nothing, and the next galaxy press brought back Coruscant
        // and Sesswenna. Slot +8 (FUN_00422ce0 0x132) hides the focused
        // window and Windows gives the focus to the view; the reused menu
        // takes none.
        let (mut focus, sesswenna, coruscant) = loaded_yaga();
        let mut gid = GidUiState::default();
        press_gid(&mut focus, &mut gid);
        gid.close_menu();
        assert_eq!(
            focus.sync(vec![(0, sesswenna), (6, coruscant)]),
            Vec::<FocusStep>::new()
        );
        assert_eq!(focus.focus, GalaxyFocus::Child(coruscant));
        press_gid(&mut focus, &mut gid);
        assert_eq!(focus.focus, GalaxyFocus::View);
        assert_eq!(focus.escape(), None);
        gid.close_menu();
        assert!(!gid.hides(coruscant) && !gid.hides(sesswenna));
    }

    #[test]
    fn a_new_window_leaves_the_focus_where_it_was() {
        // Run q under Wine (empire-fleet-to-yaga-minor, days 42-45): with
        // the focus on Sesswenna, a double-click on Ghorman opened its System
        // window (FUN_004593e0 0x203 → FUN_0045aac0, which shows and raises
        // it but neither focuses nor activates it); a later GID press, then
        // Escapes did nothing and Ghorman survived.
        let (sesswenna, _, ghorman) = children();
        let mut focus = GalaxyFocusState {
            focus: GalaxyFocus::Child(sesswenna),
            current: Some(sesswenna),
            children: vec![(0, sesswenna)],
        };
        assert_eq!(
            focus.sync(vec![(0, sesswenna), (6, ghorman)]),
            vec![FocusStep::Show(ghorman)]
        );
        assert_eq!(focus.focus, GalaxyFocus::Child(sesswenna));
        let mut gid = GidUiState {
            built: true,
            ..GidUiState::default()
        };
        press_gid(&mut focus, &mut gid);
        assert_eq!(focus.escape(), None);
        assert_eq!(focus.escape(), None);
    }

    #[test]
    fn the_first_menus_escape_with_no_window_activated_only_focuses_the_view() {
        // Run m under Wine (alliance-day54-advice, day 61): no window is
        // restored; a galaxy double-click opened the Corellian sector window,
        // which FUN_00429ce0 does not activate, and the first GID press gave
        // the menu the focus. Its Escape's 0x467 found param_1[0x2e] empty
        // and only focused the view.
        let (sector, _, _) = children();
        let mut focus = GalaxyFocusState::default();
        assert_eq!(focus.sync(vec![(0, sector)]), vec![FocusStep::Show(sector)]);
        let mut gid = GidUiState::default();
        press_gid(&mut focus, &mut gid);
        assert_eq!(focus.focus, GalaxyFocus::Menu);
        assert_eq!(focus.escape(), None);
        assert_eq!(focus.focus, GalaxyFocus::View);
        assert!(gid.hides(sector));
    }

    #[test]
    fn a_focus_step_raises_shows_or_closes_the_window_it_names() {
        // FUN_00422ce0 0x467 shows (ShowWindow 5) and raises
        // (BringWindowToTop) the window it activates; the keyed close
        // (slot +0x30) destroys one.
        let mut world = GameWorld::default();
        let sector = world
            .sectors
            .insert(rebellion_core::world::Sector::default());
        let system = rebellion_core::world::System {
            sector,
            ..rebellion_core::world::System::default()
        };
        let first = world.systems.insert(system.clone());
        let second = world.systems.insert(system);
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let layout = CockpitState::new(faction).layout_for(640.0, 480.0);
            let mut sectors = SectorWindowState::default();
            let mut systems = SystemWindowState::default();
            let mut defenses = DefensesWindowState::default();
            let mut missions = MissionsWindowState::default();
            let fleets = FleetWindowState::default();
            let manager = rebellion_windowing::WindowManager::new();
            assert!(sectors.open_for_system(&world, first, faction));
            assert!(systems.open(&world, first, (100, 50), faction, layout));
            assert!(systems.open(&world, second, (100, 50), faction, layout));
            let snapshot = manager.begin_frame();
            let all = galaxy_children(&sectors, &systems, &defenses, &missions, &fleets, &snapshot);
            assert_eq!(all.len(), 3, "{faction:?}");
            let mut gid = GidUiState::default();
            gid.open_menu();
            let mut step = |step,
                            gid: &mut GidUiState,
                            sectors: &mut SectorWindowState,
                            systems: &mut SystemWindowState| {
                apply_focus_step(
                    step,
                    gid,
                    sectors,
                    systems,
                    &mut defenses,
                    &mut missions,
                    &mut |_| {},
                );
            };
            step(
                FocusStep::Raise(GalaxyChild::System(first)),
                &mut gid,
                &mut sectors,
                &mut systems,
            );
            assert_eq!(
                topmost(systems.children()),
                Some(GalaxyChild::System(first))
            );
            assert!(
                !gid.hides(GalaxyChild::System(first)) && gid.hides(GalaxyChild::System(second))
            );
            step(
                FocusStep::Raise(GalaxyChild::Sector(sector)),
                &mut gid,
                &mut sectors,
                &mut systems,
            );
            assert!(sectors.children().all(|(z, _)| z > 0), "{faction:?}");
            step(
                FocusStep::Show(GalaxyChild::System(second)),
                &mut gid,
                &mut sectors,
                &mut systems,
            );
            assert!(!gid.hides(GalaxyChild::System(second)));
            step(
                FocusStep::Close(GalaxyChild::System(first)),
                &mut gid,
                &mut sectors,
                &mut systems,
            );
            assert_eq!(systems.children().count(), 1, "{faction:?}");
        }
    }

    #[test]
    fn a_click_on_a_window_gives_it_the_focus() {
        // FUN_004534f0 sends 0x467 for itself on a press, which activates it
        // unless it is the view's current child, and calls SetFocus on itself
        // on the release.
        let (_, first, second) = children();
        let mut focus = GalaxyFocusState {
            focus: GalaxyFocus::View,
            current: Some(second),
            children: vec![(1, first), (2, second)],
        };
        assert_eq!(
            focus.sync(vec![(1, first), (3, second)]),
            Vec::<FocusStep>::new()
        );
        assert_eq!(focus.focus, GalaxyFocus::Child(second));
        assert_eq!(
            focus.sync(vec![(4, first), (3, second)]),
            Vec::<FocusStep>::new()
        );
        assert_eq!(focus.focus, GalaxyFocus::Child(first));
        assert_eq!(focus.current, Some(first));
        assert_eq!(
            focus.sync(vec![(4, first), (3, second)]),
            Vec::<FocusStep>::new()
        );
        assert_eq!(focus.focus, GalaxyFocus::Child(first));
    }

    #[test]
    fn a_frame_with_no_window_raised_leaves_the_focus_alone() {
        let (_, first, second) = children();
        let mut focus = GalaxyFocusState {
            focus: GalaxyFocus::View,
            current: Some(second),
            children: vec![(1, first), (2, second)],
        };
        assert_eq!(
            focus.sync(vec![(1, first), (2, second)]),
            Vec::<FocusStep>::new()
        );
        assert_eq!(focus.focus, GalaxyFocus::View);
    }

    #[test]
    fn a_closed_window_hands_the_focus_to_the_top_one_left() {
        // FUN_00422ce0 0x405 destroys the window and posts 0x467(0,0), which
        // activates the top remaining child.
        let (sector, first, second) = children();
        let mut focus = GalaxyFocusState {
            focus: GalaxyFocus::Child(second),
            current: Some(second),
            children: vec![(0, sector), (1, first), (2, second)],
        };
        assert_eq!(
            focus.sync(vec![(0, sector), (1, first)]),
            vec![FocusStep::Raise(first)]
        );
        assert_eq!(focus.focus, GalaxyFocus::Child(first));
    }

    #[test]
    fn a_closure_hands_the_focus_on_even_in_a_frame_where_a_window_opens() {
        // FUN_00422ce0 0x405 destroys the closed window and posts 0x467(0,0)
        // whatever else opened in that frame; the walk activates the top
        // child left, here the new System window, which FUN_0045aac0 raised
        // to HWND_TOP. A focus left on a window that is gone takes the same
        // walk.
        let (sector, first, second) = children();
        let mut focus = GalaxyFocusState {
            focus: GalaxyFocus::Child(first),
            current: Some(first),
            children: vec![(0, sector), (1, first)],
        };
        assert_eq!(
            focus.sync(vec![(0, sector), (2, second)]),
            vec![FocusStep::Show(second), FocusStep::Raise(second)]
        );
        assert_eq!(focus.focus, GalaxyFocus::Child(second));
        assert_eq!(focus.escape(), Some(FocusStep::Close(second)));

        let mut stale = GalaxyFocusState {
            focus: GalaxyFocus::Child(first),
            current: Some(first),
            children: vec![(0, sector)],
        };
        assert_eq!(
            stale.sync(vec![(0, sector)]),
            vec![FocusStep::Raise(sector)]
        );
        assert_eq!(stale.focus, GalaxyFocus::Child(sector));
    }
}
