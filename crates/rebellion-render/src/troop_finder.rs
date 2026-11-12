//! The Troop Finder (window type `0x16`, `FUN_0042a4d0`;
//! `ghidra/notes/troop-finder.md`).
//!
//! A 470 by 330 window the cockpit's Troop Finder control (`0x130`) or F5
//! opens. It lists systems and fleets that hold regiments of the selected
//! side under two tabs (Alliance, Imperial), sorted by name
//! (`FUN_0046ea10`). Each row shows five count cells by troop class.
//! Typing a name picks the row it best begins (`FUN_00609650`); Display,
//! Enter or a double click opens the Sector and System Defenses windows
//! for the choice (`FUN_00429440`).

use rebellion_core::ids::{FleetKey, SystemKey, TroopKey};
use rebellion_core::troop_transport::TroopTransportState;

use crate::finder_common::{self, prelude::*};
use crate::system_window::fleet_label;

finder_common::define_finder_interface!(
    TROOP_FINDER_WIDTH,
    TROOP_FINDER_HEIGHT,
    TROOP_FINDER_NAME_CONTROL,
    TROOP_FINDER_LIST_CONTROL
);

// STRATEGY bitmaps by side, Alliance then Empire (FUN_0046ce40).
// Panel bitmap chosen by SELECTED TAB (FUN_0046ce40: 0x292c, 0x292d).
const PANEL: [u32; 2] = [10540, 10541];
const RAIL: [u32; 2] = [10584, 10588];

/// The side's Close (`200`) and Display (`0xc9`) controls (`FUN_0046ce40`).
struct SideControls {
    close: Control,
    display: Control,
}

const CONTROLS: [SideControls; 2] = [
    // Alliance (side 1): (0x1ab, 0x19) = (427, 25) and (0x1ab, 0x5d) = (427, 93).
    SideControls {
        close: Control::new((427.0, 25.0, 32.0, 31.0), (10514, 10515)),
        display: Control::new((427.0, 93.0, 32.0, 31.0), (10518, 10519)),
    },
    // Empire (side 2): (0x1aa, 0x15) = (426, 21) and (0x1aa, 0x59) = (426, 89).
    SideControls {
        close: Control::new((426.0, 21.0, 44.0, 41.0), (10516, 10517)),
        display: Control::new((426.0, 89.0, 44.0, 41.0), (10520, 10521)),
    },
];

// Window rectangles (x, y, width, height), FUN_0046ce40.
const TAB_LABEL_AT: (f32, f32) = (40.0, 119.0); // 0x28, 0x77
const LIST: (f32, f32, f32, f32) = (37.0, 144.0, 349.0, 159.0); // 0x25, 0x90
/// `FUN_00607ea0(.., 0xca, 330, 25)`: each row is 330 by 25.
const ROW_HEIGHT: f32 = 25.0;

finder_common::define_side_tab! {
    /// The side tab (`FUN_0046ea10`'s `param_1` 1 or 2).
    pub enum TroopTab {
        alliance: "Alliance Troops",
        imperial: "Imperial Troops",
    }
}

/// Five troop `DatIds` per side (`FUN_0046d8d0`).
const ALLIANCE_TROOP_CLASSES: [u32; 5] = [
    0x1000_0002, // Army Regiment
    0x1000_0001, // Fleet Regiment
    0x1000_0005,
    0x1000_0003,
    0x1000_0004,
];
const EMPIRE_TROOP_CLASSES: [u32; 5] = [
    0x1000_0008, // Stormtrooper Regiment
    0x1000_000a, // ScoutTrooper Regiment
    0x1000_0007,
    0x1000_0006,
    0x1000_0009,
];

fn troop_classes(tab: TroopTab) -> &'static [u32; 5] {
    match tab {
        TroopTab::Alliance => &ALLIANCE_TROOP_CLASSES,
        TroopTab::Imperial => &EMPIRE_TROOP_CLASSES,
    }
}

/// One count cell in a row: the number of regiments and whether any are
/// not usable. port: the port has no usable bit for regiments; all are
/// treated as usable.
pub use finder_common::FinderCount as TroopCount;

/// One list row: its name and the five troop-class counts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TroopFinderRow {
    /// A system name or fleet name.
    pub name: String,
    /// The system this row represents (system row → the system; fleet
    /// row → the fleet's system).
    pub system: SystemKey,
    /// If the row represents a fleet at that system, the fleet key.
    pub fleet: Option<FleetKey>,
    /// Five columns of troop counts per class.
    pub counts: [TroopCount; 5],
}

impl FinderRowName for TroopFinderRow {
    fn finder_name(&self) -> &str {
        &self.name
    }
}

/// A typed name chooses the first row it best begins (`FUN_00609650`).
#[must_use]
pub fn prefix_choice(rows: &[TroopFinderRow], text: &str) -> Option<usize> {
    finder_common::prefix_choice(rows, text)
}

finder_common::define_world_class_counter!(
    count_troops,
    TroopKey,
    TroopTab,
    TroopCount,
    5,
    troop_classes,
    troops
);

/// The rows `FUN_0046ea10` lists for `tab`, in name order ignoring case.
///
/// For each system, if it holds regiments of the tab's side (direct
/// children via `System.ground_units`), the system is listed. Then, for
/// each fleet at that system of the tab's side, if any of its ships
/// carries regiments (via `TroopTransportState`), a row named by the
/// fleet is listed. The walks (`FUN_00504cc0`, `FUN_004ffef0`) run in
/// mode 3, existing, and test no visibility, so the other side's tab lists
/// every regiment it holds.
#[must_use]
pub fn rows(
    world: &GameWorld,
    tab: TroopTab,
    transport: &TroopTransportState,
) -> Vec<TroopFinderRow> {
    let mut rows = Vec::new();

    for (system_key, system) in &world.systems {
        // Check direct ground troops at this system. FUN_00504cc0 walks
        // them in mode 3, existing (+0x50 bit 6), with no visibility test.
        let ground = finder_common::side_keys(&system.ground_units, tab.index(), |key| {
            world.troops.get(key).map(|troop| troop.is_alliance)
        });
        if !ground.is_empty() {
            let counts = count_troops(world, &ground, tab);
            rows.push(TroopFinderRow {
                name: system.name.clone(),
                system: system_key,
                fleet: None,
                counts,
            });
        }

        // Check fleets at this system.
        for &fleet_key in &system.fleets {
            let Some(fleet) = world.fleets.get(fleet_key) else {
                continue;
            };
            if !tab.lists(fleet.is_alliance) {
                continue;
            }
            let cargo = transport.cargo(fleet_key);
            if cargo.is_empty() {
                continue;
            }
            // FUN_00504c40: any regiment aboard lists the fleet, whatever
            // its class; the five columns count those they name.
            let counts = count_troops(world, cargo, tab);
            let Some(label) = fleet_label(world, fleet_key) else {
                continue;
            };
            rows.push(TroopFinderRow {
                name: label,
                system: system_key,
                fleet: Some(fleet_key),
                counts,
            });
        }
    }

    rows.sort_by_cached_key(|row| row.name.to_ascii_lowercase());
    rows
}

/// What the Troop Finder asks of the galaxy view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TroopFinderAction {
    /// System row: open the Sector window and the System Defenses window
    /// (`FUN_00429440` kind 10) selecting the first regiment.
    OpenDefenses { system: SystemKey },
    /// Fleet row: open the Sector window and the Fleet window (kind 4)
    /// for the fleet's system.
    OpenFleet { system: SystemKey, fleet: FleetKey },
}

/// Manager-owned Troop Finder intent. Dependent opens retain the exact Finder
/// identity whose accepted close receipt gates their publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedTroopFinderAction {
    /// Close or Escape requests teardown of this exact manager identity.
    Close(WindowId),
    /// A selected destination paired with the Finder identity that must close.
    Open {
        finder: WindowId,
        destination: TroopFinderAction,
    },
}

#[derive(Debug, Clone)]
struct OpenFinder {
    ui: FinderUiState,
    tab: TroopTab,
    /// The chosen row index into the current list.
    chosen: Option<usize>,
}

finder_common::impl_finder_content!(OpenFinder);

/// The Troop Finder, one at most (`FUN_0042a4d0`).
#[derive(Debug, Clone, Default)]
pub struct TroopFinderState {
    window: Option<OpenFinder>,
}

finder_common::impl_finder_state_shell!(
    TroopFinderState,
    WindowKind::TroopFinder,
    "original-troop-finder"
);

/// Candidate Troop Finder content. It remains inert until the app coordinator
/// accepts the singleton identity and returns its open receipt.
#[derive(Debug, Clone, Copy)]
pub struct PreparedTroopFinder {
    faction: CockpitFaction,
}

finder_common::impl_finder_lifecycle!(
    TroopFinderState,
    PreparedTroopFinder,
    WindowKind::TroopFinder,
    |faction, window_id| {
        OpenFinder {
            ui: FinderUiState::new(window_id, faction),
            tab: finder_common::side_value(faction, TroopTab::ALL),
            chosen: None,
        }
    }
);

impl TroopFinderState {
    /// Show `tab` (group `100`): the list rebuilds and the choice clears
    /// (`FUN_0046ea10`).
    pub fn set_tab(&mut self, tab: TroopTab) {
        if let Some(window) = &mut self.window {
            finder_common::change_category(
                &mut window.tab,
                &mut window.chosen,
                &mut window.ui,
                tab,
            );
        }
    }
}

/// Rows the list shows at once: 159 / 25 = 6.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Six whole rows of 25 in a list 159 tall."
)]
const VISIBLE_ROWS: usize = (LIST.3 / ROW_HEIGHT) as usize;

const FINDER_CONFIG: finder_common::SideFinderConfig<TroopTab> = finder_common::side_finder_config! {
    TroopTab, &TroopTab::ALL;
    area: "original-troop-finder",
    name: "original-troop-finder-name",
    list_id: "original-troop-finder-list",
    row: "troop",
    title: "Troop Finder",
    name_label: "Troop Location";
    list: LIST,
    row_height: ROW_HEIGHT,
    visible_rows: VISIBLE_ROWS;
    controls: (TROOP_FINDER_NAME_CONTROL, TROOP_FINDER_LIST_CONTROL),
};

/// The five column cell colours (`FUN_0046d8d0`).
///
/// Brush index 0 = red (0x20000ff → RGB 255,0,0): tab Alliance, all usable.
/// Brush index 1 = green (0x200ff00 → RGB 0,255,0): tab Imperial, all usable.
/// Brush index 3 = dark red (0x80 → RGB 128,0,0): tab Alliance, some not usable.
/// Brush index 4 = dark green (0x8000 → RGB 0,128,0): tab Imperial, some not usable.
///
/// port: all regiments are treated as usable, so only bright colours appear.
type TroopFinderDrawOutcome = finder_common::FinderDrawOutcome<TroopFinderAction>;

/// Draw a fixture-opened Troop Finder, if any, and report an action.
pub fn draw_troop_finder(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut TroopFinderState,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    transport: &TroopTransportState,
) -> Option<TroopFinderAction> {
    match draw_troop_finder_outcome(ctx, world, state, None, layout, cache, transport) {
        Some(TroopFinderDrawOutcome::Open(action)) => {
            state.close();
            Some(action)
        }
        Some(TroopFinderDrawOutcome::Close) => {
            state.close();
            None
        }
        None => None,
    }
}

/// Draw the open manager-owned Troop Finder using the frozen semantic focus.
pub fn draw_troop_finder_focused(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut TroopFinderState,
    focus: Option<FocusTarget>,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    transport: &TroopTransportState,
) -> Option<ManagedTroopFinderAction> {
    let finder = state.window_id()?;
    match draw_troop_finder_outcome(ctx, world, state, focus, layout, cache, transport) {
        Some(TroopFinderDrawOutcome::Open(destination)) => Some(ManagedTroopFinderAction::Open {
            finder,
            destination,
        }),
        Some(TroopFinderDrawOutcome::Close) => Some(ManagedTroopFinderAction::Close(finder)),
        None => None,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "The window carries its existing render context plus semantic focus; splitting it hides the original control order."
)]
fn draw_troop_finder_outcome(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut TroopFinderState,
    focus: Option<FocusTarget>,
    layout: CockpitLayout,
    cache: &mut BmpCache,
    transport: &TroopTransportState,
) -> Option<TroopFinderDrawOutcome> {
    let (window, side_idx, controls) = finder_common::side_finder_snapshot!(state, CONTROLS);
    let list = rows(world, window.tab, transport);
    let outcome = finder_common::draw_coloured_side_finder_window!(
        ctx, cache, layout, focus, window, list, &FINDER_CONFIG,
        finder_common::side_finder_presentation!(
            window.tab.label(), TAB_LABEL_AT, window.tab.index();
            PANEL[window.tab.index()], side_idx, RAIL, (controls.close, controls.display)
        );
        extra_controls: |_, _, _| {}, paint_details: |canvas: &finder_common::FinderCanvas<'_>,
                                                       painter: &egui::Painter,
                                                       row: &TroopFinderRow,
                                                       line: f32,
                                                       cell_font: &egui::FontId,
                                                       colours: (egui::Color32, egui::Color32)| {
            // Five count columns at x = 194 + 28*i, y = 2 within the row
            // (FUN_0046d8d0).
            finder_common::paint_count_cells(
                painter,
                canvas.frame,
                canvas.scale,
                (LIST.0, LIST.1),
                ROW_HEIGHT,
                line,
                194.0,
                row.counts.iter().map(|cell| cell.count),
                cell_font,
                colours,
            );
        };
        row_action: action_for_row,
    );
    let open_window = state.window.as_mut()?;
    finder_common::commit_indexed_output(
        &mut open_window.ui,
        &mut open_window.tab,
        &mut open_window.chosen,
        &list,
        outcome,
        |index, _| index,
    )
}

/// The action for opening a row (`FUN_00429440`).
fn action_for_row(row: &TroopFinderRow) -> TroopFinderAction {
    finder_common::location_action(
        row.system,
        row.fleet,
        |system, fleet| TroopFinderAction::OpenFleet { system, fleet },
        |system| TroopFinderAction::OpenDefenses { system },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        add_system, control_focus as focus, finder_escape_input as escape_input,
        finder_layout as layout, key_press as key, tap_key as tap,
    };
    use rebellion_core::carriage::ProductRoom;
    use rebellion_core::dat::Faction;
    use rebellion_core::ids::{DatId, FleetKey, SystemKey};
    use rebellion_core::world::{ControlKind, TroopUnit};
    use rebellion_windowing::{FocusTarget, WindowCommand, WindowId, WindowManager, WindowOwner};

    crate::test_support::define_managed_finder_fixture!(
        managed_finder,
        TroopFinderState,
        "Troop Finder"
    );

    fn add_troop(
        world: &mut GameWorld,
        system: SystemKey,
        class_raw: u32,
        is_alliance: bool,
    ) -> TroopKey {
        let key = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(class_raw),
            is_alliance,
        });
        world.systems[system].ground_units.push(key);
        key
    }

    fn add_fleet(world: &mut GameWorld, system: SystemKey, is_alliance: bool) -> FleetKey {
        crate::test_support::add_transport_fleet(world, system, is_alliance, 4)
    }

    fn names(rows: &[TroopFinderRow]) -> Vec<&str> {
        rows.iter().map(|row| row.name.as_str()).collect()
    }

    fn managed_navigation() -> (
        WindowManager,
        TroopFinderState,
        WindowId,
        FocusTarget,
        FocusTarget,
        egui::Context,
    ) {
        let (manager, state, id) = managed_finder();
        (
            manager,
            state,
            id,
            focus(id, TROOP_FINDER_NAME_CONTROL),
            focus(id, TROOP_FINDER_LIST_CONTROL),
            egui::Context::default(),
        )
    }

    fn drive_focused_in_context(
        ctx: &egui::Context,
        galaxy: &Galaxy,
        state: &mut TroopFinderState,
        focus: FocusTarget,
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<ManagedTroopFinderAction> {
        let layout = layout();
        let mut cache = BmpCache::new();
        let transport = TroopTransportState::default();
        crate::test_support::run_action_frames(ctx, frames, |ctx| {
            draw_troop_finder_focused(
                ctx,
                &galaxy.world,
                state,
                Some(focus),
                layout,
                &mut cache,
                &transport,
            )
        })
    }

    fn drive_tap_without_action(
        ctx: &egui::Context,
        galaxy: &Galaxy,
        state: &mut TroopFinderState,
        focus: FocusTarget,
        key: egui::Key,
    ) {
        assert_eq!(
            drive_focused_in_context(ctx, galaxy, state, focus, tap(key)),
            None
        );
    }

    fn drive_tab(
        ctx: &egui::Context,
        galaxy: &Galaxy,
        state: &mut TroopFinderState,
        focus: FocusTarget,
        key: egui::Key,
        expected: TroopTab,
    ) {
        drive_tap_without_action(ctx, galaxy, state, focus, key);
        assert_eq!(state.window.as_ref().unwrap().tab, expected);
    }

    fn drive_tab_round_trip(
        ctx: &egui::Context,
        galaxy: &Galaxy,
        state: &mut TroopFinderState,
        focus: FocusTarget,
    ) {
        drive_tab(
            ctx,
            galaxy,
            state,
            focus,
            egui::Key::ArrowRight,
            TroopTab::Imperial,
        );
        assert_eq!(state.window.as_ref().unwrap().chosen, None);
        drive_tab(
            ctx,
            galaxy,
            state,
            focus,
            egui::Key::ArrowLeft,
            TroopTab::Alliance,
        );
    }

    struct Galaxy {
        world: GameWorld,
    }

    fn galaxy() -> Galaxy {
        let mut world = GameWorld::default();
        let yavin = add_system(
            &mut world,
            "Yavin",
            ControlKind::Controlled(Faction::Alliance),
        );
        // Two Alliance army regiments on Yavin.
        add_troop(&mut world, yavin, 0x1000_0002, true);
        add_troop(&mut world, yavin, 0x1000_0002, true);
        // One Alliance fleet regiment.
        add_troop(&mut world, yavin, 0x1000_0001, true);
        let coruscant = add_system(
            &mut world,
            "Coruscant",
            ControlKind::Controlled(Faction::Empire),
        );
        // One Empire stormtrooper regiment on Coruscant.
        add_troop(&mut world, coruscant, 0x1000_0008, false);
        Galaxy { world }
    }

    fn add_alliance_alderaan(galaxy: &mut Galaxy) -> SystemKey {
        let alderaan = add_system(
            &mut galaxy.world,
            "Alderaan",
            ControlKind::Controlled(Faction::Alliance),
        );
        add_troop(&mut galaxy.world, alderaan, 0x1000_0002, true);
        alderaan
    }

    #[test]
    fn the_troop_finder_lists_systems_holding_the_tabs_regiments() {
        // FUN_0046ea10: walk systems, check FUN_00504cc0 for regiments
        // of the tab's side.
        let galaxy = galaxy();
        let transport = TroopTransportState::default();

        let alliance = rows(&galaxy.world, TroopTab::Alliance, &transport);
        assert_eq!(names(&alliance), ["Yavin"]);
        assert_eq!(alliance[0].counts[0].count, 2, "Army Regiment column");
        assert_eq!(alliance[0].counts[1].count, 1, "Fleet Regiment column");
        assert_eq!(alliance[0].counts[2].count, 0);

        let imperial = rows(&galaxy.world, TroopTab::Imperial, &transport);
        // FUN_0046ea10 walks all systems; ground troops are shown
        // regardless of fog.
        assert_eq!(names(&imperial), ["Coruscant"]);
        assert_eq!(imperial[0].counts[0].count, 1, "Stormtrooper column");
    }

    #[test]
    fn a_fleet_carrying_regiments_is_listed_by_its_fleet_name() {
        // FUN_0046ea10: for each fleet at the system of the tab's side,
        // check FUN_00504c40 for regiments.
        let mut galaxy = galaxy();
        let yavin = galaxy.world.systems.iter().next().unwrap().0;
        let fleet = add_fleet(&mut galaxy.world, yavin, true);
        let troop = galaxy.world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0002),
            is_alliance: true,
        });
        galaxy.world.systems[yavin].ground_units.push(troop);
        let mut transport = TroopTransportState::default();
        transport
            .load(&mut galaxy.world, &ProductRoom::default(), fleet, &[troop])
            .unwrap();

        let listed = rows(&galaxy.world, TroopTab::Alliance, &transport);

        // Yavin (ground) and the fleet.
        assert_eq!(listed.len(), 2);
        assert!(listed.iter().any(|r| r.fleet == Some(fleet)));
        let fleet_row = listed.iter().find(|r| r.fleet.is_some()).unwrap();
        assert_eq!(fleet_row.counts[0].count, 1);
    }

    #[test]
    fn rows_are_sorted_by_name_ignoring_case() {
        // FUN_005f59f0: rows inserted in name order.
        let mut world = GameWorld::default();
        let beta = add_system(
            &mut world,
            "beta",
            ControlKind::Controlled(Faction::Alliance),
        );
        let alpha = add_system(
            &mut world,
            "Alpha",
            ControlKind::Controlled(Faction::Alliance),
        );
        add_troop(&mut world, beta, 0x1000_0002, true);
        add_troop(&mut world, alpha, 0x1000_0002, true);
        let transport = TroopTransportState::default();

        let listed = rows(&world, TroopTab::Alliance, &transport);
        assert_eq!(names(&listed), ["Alpha", "beta"]);
    }

    #[test]
    fn a_typed_name_chooses_the_first_row_it_best_begins() {
        // FUN_00609650: the same prefix-match the Fleet Finder uses.
        let rows = vec![
            TroopFinderRow {
                name: "Coruscant".into(),
                system: SystemKey::default(),
                fleet: None,
                counts: [TroopCount::default(); 5],
            },
            TroopFinderRow {
                name: "Yavin".into(),
                system: SystemKey::default(),
                fleet: None,
                counts: [TroopCount::default(); 5],
            },
        ];

        crate::test_support::assert_standard_prefix_choices(|text| prefix_choice(&rows, text));
    }

    #[test]
    fn the_action_for_a_system_row_opens_defenses_and_a_fleet_row_opens_fleet() {
        // FUN_00429440: system → kind 10 (defenses), fleet → kind 4.
        let system = SystemKey::default();
        let fleet = FleetKey::default();

        let system_row = TroopFinderRow {
            name: "Yavin".into(),
            system,
            fleet: None,
            counts: [TroopCount::default(); 5],
        };
        assert_eq!(
            action_for_row(&system_row),
            TroopFinderAction::OpenDefenses { system }
        );

        let fleet_row = TroopFinderRow {
            name: "Fleet 1".into(),
            system,
            fleet: Some(fleet),
            counts: [TroopCount::default(); 5],
        };
        assert_eq!(
            action_for_row(&fleet_row),
            TroopFinderAction::OpenFleet { system, fleet }
        );
    }

    #[test]
    fn the_visible_rows_count_is_six() {
        // FUN_00607ea0: list 159 tall, rows 25 tall → 6 visible.
        assert_eq!(VISIBLE_ROWS, 6);
    }

    #[test]
    fn the_list_scrolls_as_little_as_it_must_to_show_a_row() {
        assert_eq!(finder_common::scrolled_to(0, 5, VISIBLE_ROWS), 0);
        assert_eq!(finder_common::scrolled_to(0, 6, VISIBLE_ROWS), 1);
        assert_eq!(finder_common::scrolled_to(3, 1, VISIBLE_ROWS), 1);
        assert_eq!(finder_common::scrolled_to(3, 8, VISIBLE_ROWS), 3);
        assert_eq!(finder_common::scrolled_to(2, 8, VISIBLE_ROWS), 3);
    }

    #[test]
    fn opening_an_open_finder_keeps_its_state() {
        // FUN_0042a4d0 builds the window only when no type 0x16 is open.
        crate::test_support::assert_finder_reopen_preserves::<TroopFinderState>(
            |state| state.set_tab(TroopTab::Imperial),
            |state| assert_eq!(state.window.as_ref().unwrap().tab, TroopTab::Imperial),
        );
    }

    #[test]
    fn troop_finder_duplicate_preserves_content() {
        let (mut manager, mut state, id) = managed_finder();

        let content = state.window.as_mut().unwrap();
        content.tab = TroopTab::Imperial;
        content.ui.name = "Coruscant".into();
        content.chosen = Some(0);
        content.ui.first_row = 2;
        content.ui.focus_name = false;
        let focused = focus(id, TROOP_FINDER_LIST_CONTROL);
        crate::test_support::assert_focused_duplicate_finder_ui(
            &mut manager,
            &mut state,
            id,
            focused,
            "Coruscant",
            2,
            false,
        );
        let content = state.window.as_ref().unwrap();
        assert_eq!(content.tab, TroopTab::Imperial);
        assert_eq!(content.chosen, Some(0));
    }

    #[test]
    fn troop_finder_rejects_stale_publication_receipts_without_retargeting() {
        let (mut manager, mut state, id) = managed_finder();
        state.window.as_mut().unwrap().ui.name = "Yavin".into();

        crate::test_support::assert_stale_finder_publications_rejected(
            &mut manager,
            &mut state,
            id,
            WindowKind::PersonnelFinder,
        );
        let content = state.window.as_ref().unwrap();
        assert_eq!(content.ui.name, "Yavin");
    }

    #[test]
    fn troop_finder_modal_blocks_underlay() {
        let (manager, state, id) = managed_finder();
        crate::test_support::assert_finder_modal_routing(&manager, &state, id, layout());
    }

    #[test]
    fn troop_finder_hit_requires_its_geometry_and_current_input_lease() {
        let (mut manager, state, id) = managed_finder();
        let rect = state.screen_rect(layout()).unwrap();
        let frame = manager.begin_frame();
        assert_eq!(
            state.hit_window(&frame, layout(), (rect.max.x + 1.0, rect.center().y)),
            None
        );

        crate::test_support::open_singleton_window(
            &mut manager,
            WindowKind::PersonnelFinder,
            WindowOwner::StrategicRoot,
            "modal cover",
        );
        let blocked_frame = manager.begin_frame();
        assert!(!blocked_frame.allows_input(id));
        assert_eq!(
            state.hit_window(&blocked_frame, layout(), (rect.center().x, rect.center().y),),
            None
        );
    }

    #[test]
    fn the_longest_common_beginning_wins_and_a_tie_keeps_the_first() {
        // FUN_00609650 keeps a row only for a strictly longer match.
        let rows: Vec<TroopFinderRow> = ["Corulag", "Coruscant", "Yavin"]
            .into_iter()
            .map(|name| TroopFinderRow {
                name: name.into(),
                system: SystemKey::default(),
                fleet: None,
                counts: [TroopCount::default(); 5],
            })
            .collect();
        assert_eq!(prefix_choice(&rows, "corus"), Some(1));
        assert_eq!(prefix_choice(&rows, "cor"), Some(0));
    }

    #[test]
    fn each_tab_names_its_side_and_paints_its_own_cells() {
        // TEXTSTRA 0x1950/0x1951; panels 10540/10541 (FUN_006075e0);
        // FUN_0046d8d0's brushes: red with white text for the Alliance,
        // green with black for the Empire.
        assert_eq!(TroopTab::Alliance.label(), "Alliance Troops");
        assert_eq!(TroopTab::Imperial.label(), "Imperial Troops");
        assert_eq!(PANEL[TroopTab::Alliance.index()], 10540);
        assert_eq!(PANEL[TroopTab::Imperial.index()], 10541);
        crate::test_support::assert_finder_count_colours(
            TroopTab::Alliance.index(),
            TroopTab::Imperial.index(),
        );
    }

    #[test]
    fn a_new_tab_clears_the_choice() {
        // FUN_0046ea10: rebuilding clears the chosen row.
        crate::test_support::assert_finder_category_change_clears!(
            TroopFinderState,
            Some(0),
            |state: &mut TroopFinderState| state.set_tab(TroopTab::Imperial),
        );
    }

    #[test]
    fn escape_closes_the_finder() {
        let mut state = TroopFinderState::default();
        state.open(CockpitFaction::Alliance);
        let galaxy = galaxy();
        let transport = TroopTransportState::default();
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let layout = layout();

        // Run a frame with escape.
        let input = escape_input();
        let _ = ctx.run(input, |ctx| {
            draw_troop_finder(
                ctx,
                &galaxy.world,
                &mut state,
                layout,
                &mut cache,
                &transport,
            );
        });

        assert!(!state.is_open());
    }

    #[test]
    fn troop_finder_fixture_open_focuses_name_without_manager_focus() {
        let mut state = TroopFinderState::default();
        assert!(state.open(CockpitFaction::Alliance));
        let galaxy = galaxy();
        let transport = TroopTransportState::default();
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let input = crate::test_support::raw_input(Vec::new(), egui::vec2(700.0, 520.0));

        let _ = ctx.run(input, |ctx| {
            assert_eq!(
                draw_troop_finder(
                    ctx,
                    &galaxy.world,
                    &mut state,
                    layout(),
                    &mut cache,
                    &transport,
                ),
                None
            );
        });

        assert_eq!(
            ctx.memory(egui::Memory::focused),
            Some(egui::Id::new("original-troop-finder-name"))
        );
    }

    #[test]
    fn troop_finder_close_consumes_trigger() {
        let (mut manager, mut state, id) = managed_finder();
        let galaxy = galaxy();
        let transport = TroopTransportState::default();
        crate::test_support::assert_managed_finder_escape_close(
            &mut manager,
            &mut state,
            id,
            "Troop Finder",
            ManagedTroopFinderAction::Close(id),
            |ctx, state, focus, cache| {
                draw_troop_finder_focused(
                    ctx,
                    &galaxy.world,
                    state,
                    focus,
                    layout(),
                    cache,
                    &transport,
                )
            },
            TroopFinderState::is_open,
            TroopFinderState::remove_content,
        );
    }

    #[test]
    fn troop_finder_selection_waits_for_close_receipt() {
        let (manager, mut state, id) = managed_finder();
        state.window.as_mut().unwrap().chosen = Some(0);
        let galaxy = galaxy();
        let expected_system = rows(
            &galaxy.world,
            TroopTab::Alliance,
            &TroopTransportState::default(),
        )[0]
        .system;
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let layout = layout();
        let control = CONTROLS[0].display.rect;
        let pos = finder_common::window_rect(layout).min
            + egui::vec2(control.0 + control.2 / 2.0, control.1 + control.3 / 2.0);
        let mut action = None;
        for events in [
            Vec::new(),
            Vec::new(),
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            }],
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            }],
        ] {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                events: std::iter::once(egui::Event::PointerMoved(pos))
                    .chain(events)
                    .collect(),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                action = action.or_else(|| {
                    draw_troop_finder_focused(
                        ctx,
                        &galaxy.world,
                        &mut state,
                        manager.begin_frame().focused(),
                        layout,
                        &mut cache,
                        &TroopTransportState::default(),
                    )
                });
            });
        }

        assert_eq!(
            action,
            Some(ManagedTroopFinderAction::Open {
                finder: id,
                destination: TroopFinderAction::OpenDefenses {
                    system: expected_system,
                },
            })
        );
        assert!(
            state.is_open(),
            "dependent open waits for the exact Finder close receipt"
        );
        assert_eq!(state.window_id(), Some(id));
    }

    #[test]
    fn troop_finder_focused_control_consumes_text_navigation_enter_and_escape() {
        let galaxy = galaxy();
        let (mut manager, mut state, id, name_focus, list_focus, ctx) = managed_navigation();
        manager.apply(WindowCommand::Focus(name_focus)).unwrap();
        assert_eq!(state.keyboard_owner(&manager.begin_frame()), Some(id));

        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &galaxy,
                &mut state,
                name_focus,
                vec![vec![], vec![], vec![egui::Event::Text("Y".into())]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().ui.name, "Y");
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &galaxy,
                &mut state,
                name_focus,
                vec![vec![key(egui::Key::ArrowDown)]],
            ),
            None
        );
        assert_eq!(state.take_focus_request(), Some(list_focus));

        crate::test_support::assert_finder_list_focus_ignores_text(
            &ctx,
            &mut state,
            "original-troop-finder-list",
            |state, events| drive_focused_in_context(&ctx, &galaxy, state, list_focus, events),
        );
        assert_eq!(
            state.take_focus_request(),
            None,
            "stable list focus must not re-request the name control"
        );

        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &galaxy,
                &mut state,
                list_focus,
                vec![vec![key(egui::Key::Enter)]],
            ),
            Some(ManagedTroopFinderAction::Open {
                finder: id,
                destination: TroopFinderAction::OpenDefenses {
                    system: rows(
                        &galaxy.world,
                        TroopTab::Alliance,
                        &TroopTransportState::default(),
                    )[0]
                    .system,
                },
            })
        );
        assert!(state.is_open(), "managed content waits for close receipt");
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &galaxy,
                &mut state,
                list_focus,
                vec![vec![key(egui::Key::Escape)]],
            ),
            Some(ManagedTroopFinderAction::Close(id))
        );
    }

    #[test]
    fn troop_finder_child_navigation_respects_focus_bounds_and_tabs() {
        let mut galaxy = galaxy();
        add_alliance_alderaan(&mut galaxy);
        assert_eq!(
            names(&rows(
                &galaxy.world,
                TroopTab::Alliance,
                &TroopTransportState::default(),
            )),
            ["Alderaan", "Yavin"]
        );

        let (_manager, mut state, _id, name_focus, list_focus, ctx) = managed_navigation();

        drive_tap_without_action(&ctx, &galaxy, &mut state, name_focus, egui::Key::ArrowDown);
        crate::test_support::assert_indexed_finder_selection!(&state, 0, "Alderaan");
        assert_eq!(state.take_focus_request(), Some(list_focus));

        assert_eq!(
            drive_focused_in_context(&ctx, &galaxy, &mut state, list_focus, vec![vec![]]),
            None
        );
        assert_eq!(state.take_focus_request(), None);

        for (key, chosen, name) in [
            (egui::Key::ArrowDown, 1, "Yavin"),
            (egui::Key::ArrowDown, 1, "Yavin"),
            (egui::Key::ArrowUp, 0, "Alderaan"),
        ] {
            drive_tap_without_action(&ctx, &galaxy, &mut state, list_focus, key);
            crate::test_support::assert_indexed_finder_selection!(&state, chosen, name);
        }

        drive_tap_without_action(&ctx, &galaxy, &mut state, list_focus, egui::Key::ArrowUp);
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(0));
        assert_eq!(state.take_focus_request(), Some(name_focus));

        drive_tab(
            &ctx,
            &galaxy,
            &mut state,
            list_focus,
            egui::Key::ArrowRight,
            TroopTab::Imperial,
        );
        assert_eq!(state.window.as_ref().unwrap().chosen, None);
        drive_tab_round_trip(&ctx, &galaxy, &mut state, list_focus);
    }

    #[test]
    fn troop_finder_empty_tab_allows_keyboard_tab_recovery() {
        let mut galaxy = galaxy();
        for (_, system) in &mut galaxy.world.systems {
            if system.control == ControlKind::Controlled(Faction::Empire) {
                system.ground_units.clear();
            }
        }
        assert_empty!(rows(
            &galaxy.world,
            TroopTab::Imperial,
            &TroopTransportState::default(),
        ));

        let (_manager, mut state, _id, _name_focus, list_focus, ctx) = managed_navigation();

        drive_tab_round_trip(&ctx, &galaxy, &mut state, list_focus);
    }

    #[test]
    fn troop_finder_empty_results_allow_list_focus_to_return_to_name() {
        let mut galaxy = galaxy();
        let (_manager, mut state, _id, name_focus, list_focus, ctx) = managed_navigation();
        let open_window = state.window.as_mut().unwrap();
        open_window.chosen = Some(0);
        open_window.ui.focus_name = false;

        assert_eq!(
            drive_focused_in_context(&ctx, &galaxy, &mut state, list_focus, vec![vec![]]),
            None
        );
        assert_eq!(
            ctx.memory(egui::Memory::focused),
            Some(egui::Id::new("original-troop-finder-list"))
        );
        assert_eq!(state.take_focus_request(), None);

        for (_, system) in &mut galaxy.world.systems {
            system.ground_units.clear();
        }
        assert_empty!(rows(
            &galaxy.world,
            TroopTab::Alliance,
            &TroopTransportState::default(),
        ));

        drive_tap_without_action(&ctx, &galaxy, &mut state, list_focus, egui::Key::ArrowUp);
        assert_eq!(state.take_focus_request(), Some(name_focus));
    }

    #[test]
    fn troop_finder_navigation_clamps_a_selection_from_disappeared_rows() {
        let mut galaxy = galaxy();
        add_alliance_alderaan(&mut galaxy);
        let (_manager, mut state, _id, name_focus, list_focus, ctx) = managed_navigation();

        state.window.as_mut().unwrap().chosen = Some(usize::MAX);
        drive_tap_without_action(&ctx, &galaxy, &mut state, name_focus, egui::Key::ArrowDown);
        crate::test_support::assert_indexed_finder_selection!(&state, 1, "Yavin");

        state.window.as_mut().unwrap().chosen = Some(usize::MAX);
        drive_tap_without_action(&ctx, &galaxy, &mut state, list_focus, egui::Key::ArrowUp);
        crate::test_support::assert_indexed_finder_selection!(&state, 0, "Alderaan");
    }

    #[test]
    fn the_finder_blocks_only_its_own_rectangle() {
        let mut state = TroopFinderState::default();
        crate::test_support::assert_finder_bounds(&mut state, layout());
    }

    #[test]
    fn count_troops_tallies_by_class_for_the_tabs_side() {
        // FUN_0046d8d0: five columns, each the count of its DatId.
        let mut world = GameWorld::default();
        let system = add_system(
            &mut world,
            "Yavin",
            ControlKind::Controlled(Faction::Alliance),
        );
        let k1 = add_troop(&mut world, system, 0x1000_0002, true); // col 0
        let k2 = add_troop(&mut world, system, 0x1000_0002, true); // col 0
        let k3 = add_troop(&mut world, system, 0x1000_0004, true); // col 4
        let k4 = add_troop(&mut world, system, 0x1000_0008, false); // empire, ignored

        let keys = vec![k1, k2, k3, k4];
        let counts = count_troops(&world, &keys, TroopTab::Alliance);
        assert_eq!(counts[0].count, 2);
        assert_eq!(counts[1].count, 0);
        assert_eq!(counts[4].count, 1);
        // port: all usable.
        assert!(counts[0].all_usable);
    }

    #[test]
    fn the_troop_finder_opens_on_the_players_side_tab() {
        // FUN_0046ce40: FUN_0060d7e0(+0x144, side, 1) opens the player's tab.
        let mut state = TroopFinderState::default();
        state.open(CockpitFaction::Alliance);
        assert_eq!(state.window.as_ref().unwrap().tab, TroopTab::Alliance);

        let mut state = TroopFinderState::default();
        state.open(CockpitFaction::Empire);
        assert_eq!(state.window.as_ref().unwrap().tab, TroopTab::Imperial);
    }
}
