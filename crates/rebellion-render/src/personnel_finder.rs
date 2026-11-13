//! The Personnel Finder (window type `0x17`, `FUN_0042a4d0`;
//! `ghidra/notes/personnel-finder.md`).
//!
//! A 470 by 330 window the cockpit's Personnel Finder control or F4
//! opens. Two side tabs (Alliance, Imperial) choose the faction; two view
//! tabs choose between a Characters list and a Special Forces list.
//!
//! Characters view: rows are "Name - Location (state) (rank)"
//! (`FUN_00465bb0`). Special Forces view: rows per system with four
//! count cells by class (`FUN_00465540`). Typing a name picks the best
//! match (`FUN_00609650`); Display, Enter or a double click opens the
//! appropriate window (`FUN_00429440`).

use rebellion_core::dat::Faction;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{CharacterKey, FleetKey, SpecialForceKey, SystemKey};

use crate::finder_common::{self, prelude::*};
use crate::system_window::fleet_label;

finder_common::define_finder_interface!(
    PERSONNEL_FINDER_WIDTH,
    PERSONNEL_FINDER_HEIGHT,
    PERSONNEL_FINDER_NAME_CONTROL,
    PERSONNEL_FINDER_LIST_CONTROL
);

// STRATEGY bitmaps by side (FUN_004637a0).
const RAIL: [u32; 2] = [10586, 10590];

/// Panel bitmaps indexed by `[view][side]` (`FUN_004637a0` / `FUN_00464e10`).
///
/// Characters + Alliance = 10538, Characters + Imperial = 10539,
/// `SpecForces` + Alliance = 10536, `SpecForces` + Imperial = 10537.
const PANELS: [[u32; 2]; 2] = [
    [10538, 10539], // Characters view: Alliance, Imperial
    [10536, 10537], // SpecForces view: Alliance, Imperial
];

finder_common::define_side_tab! {
    /// The side tab (`FUN_004637a0`'s control 100, TEXTSTRA 0x1893/0x1894).
    pub enum PersonnelTab {
        alliance: "Alliance Personnel",
        imperial: "Imperial Personnel",
    }
}

/// The view tab (`FUN_004637a0`'s control 0x6e, values 2 and 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PersonnelView {
    Characters,
    #[default]
    SpecForces,
}

impl PersonnelView {
    const ALL: [Self; 2] = [Self::SpecForces, Self::Characters];

    const fn index(self) -> usize {
        match self {
            Self::Characters => 0,
            Self::SpecForces => 1,
        }
    }

    /// The tab label y: Characters → 119, `SpecForces` → 115
    /// (`FUN_00464e10` lines 294–300).
    const fn label_y(self) -> f32 {
        match self {
            Self::Characters => 119.0,
            Self::SpecForces => 115.0,
        }
    }
}

/// The side's Close and Display controls (`FUN_004637a0`).
struct SideControls {
    close: Control,
    display: Control,
    /// View tab group origin and individual button size.
    view_origin: (f32, f32),
    view_button_size: (f32, f32),
}

const CONTROLS: [SideControls; 2] = [
    // Alliance (side 1): FUN_004637a0.
    SideControls {
        close: Control::new((423.0, 25.0, 32.0, 31.0), (10514, 10515)),
        display: Control::new((423.0, 93.0, 32.0, 31.0), (10518, 10519)),
        view_origin: (423.0, 147.0),
        view_button_size: (32.0, 31.0),
    },
    // Empire (side 2).
    SideControls {
        close: Control::new((426.0, 21.0, 44.0, 41.0), (10516, 10517)),
        display: Control::new((426.0, 89.0, 42.0, 41.0), (10520, 10521)),
        view_origin: (426.0, 143.0),
        view_button_size: (42.0, 41.0),
    },
];

/// View tab art: `[side][view]` (`FUN_004637a0`).
/// `SpecForces`: Alliance 10542/10543, Empire 10544/10545.
/// Characters: Alliance 10546/10547, Empire 10548/10549.
const VIEW_TAB_ART: [[(u32, u32); 2]; 2] = [
    // Alliance: [SpecForces, Characters]
    [(10542, 10543), (10546, 10547)],
    // Empire: [SpecForces, Characters]
    [(10544, 10545), (10548, 10549)],
];

const LIST: (f32, f32, f32, f32) = (37.0, 143.0, 349.0, 161.0);
const ROW_HEIGHT: f32 = 20.0;

/// Four Special Forces `DatIds` per side (`FUN_00465540`).
const ALLIANCE_SF_CLASSES: [u32; 4] = [0x3c00_0004, 0x3c00_0002, 0x3c00_0001, 0x3c00_0003];
const EMPIRE_SF_CLASSES: [u32; 4] = [0x3c00_0007, 0x3c00_0006, 0x3c00_0008, 0x3c00_0005];

fn sf_classes(tab: PersonnelTab) -> &'static [u32; 4] {
    match tab {
        PersonnelTab::Alliance => &ALLIANCE_SF_CLASSES,
        PersonnelTab::Imperial => &EMPIRE_SF_CLASSES,
    }
}

/// One count cell in a `SpecForces` row.
pub use finder_common::FinderCount as SfCount;

/// One list row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersonnelFinderRow {
    /// A character: "Name - Location (state)" (`FUN_00465bb0`).
    Character {
        key: CharacterKey,
        name: String,
        location: String,
        state: String,
        system: Option<SystemKey>,
    },
    /// A system (or fleet) with `SpecForces` counts (`FUN_00465540`).
    SpecForces {
        name: String,
        system: SystemKey,
        fleet: Option<FleetKey>,
        counts: [SfCount; 4],
    },
}

impl PersonnelFinderRow {
    fn display_name(&self) -> &str {
        match self {
            Self::Character { name, .. } | Self::SpecForces { name, .. } => name,
        }
    }
}

impl FinderRowName for PersonnelFinderRow {
    fn finder_name(&self) -> &str {
        self.display_name()
    }
}

finder_common::define_prefix_choice!(PersonnelFinderRow);

finder_common::define_world_class_counter!(
    count_sf,
    SpecialForceKey,
    PersonnelTab,
    SfCount,
    4,
    sf_classes,
    special_forces
);

/// A character's state word (`FUN_00465bb0`): Captured (`0x8746`) when held,
/// else On Mission (`0x8745`), else Injured (`0x8747`), else Enroute
/// (`0x8744`, `+0x50` bit 5); the first that applies. port: the port keeps no
/// injury.
fn character_state(character: &rebellion_core::world::Character) -> Option<&'static str> {
    if character.is_captive {
        Some("Captured")
    } else if character.on_mission {
        Some("On Mission")
    } else if character.en_route {
        Some("Enroute")
    } else {
        None
    }
}

/// The window a character's Display opens (`FUN_00429440`, family `0x30`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterTarget {
    /// Aboard: its fleet's Fleet window (kind 4).
    Fleet(FleetKey),
    /// At a system on a mission that is not hidden (`FUN_00520b70` is 0):
    /// the Missions window (kind 11).
    Missions,
    /// At a system otherwise: the Defenses window (kind 10).
    Defenses,
}

/// Which window `character`'s Display opens (`FUN_00429440`): its mission
/// key `+0x68` names a mission (`FUN_004f3000`); a hidden one opens the
/// Defenses window, where its member still lists.
#[must_use]
pub fn character_target(character: &rebellion_core::world::Character) -> CharacterTarget {
    match character.current_fleet {
        Some(fleet) => CharacterTarget::Fleet(fleet),
        None if character.on_mission && !character.on_hidden_mission => CharacterTarget::Missions,
        None => CharacterTarget::Defenses,
    }
}

/// The name of the container holding `character` off a fleet
/// (`FUN_004f62d0`): its jail abode (`FUN_00549580`), else its system.
fn container_name(
    world: &GameWorld,
    character: &rebellion_core::world::Character,
) -> Option<String> {
    match character.held_abode() {
        Some(abode) => Some(abode.name().to_owned()),
        None => character
            .current_system
            .and_then(|system| world.systems.get(system))
            .map(|system| system.name.clone()),
    }
}

/// A Characters row's text (`FUN_00465bb0`): the name, " - " (`0x1897`),
/// then "Killed" for a destroyed character, else its container's name
/// ("Location Unknown", `0x1895`, with none) and the state word in " ( " and
/// " ) " (`DAT_006a8790`, `DAT_006a878c`). hyp: "Killed" is TEXTSTRA
/// `0x8798`; the destroyed branch's string id is not resolved. port: the
/// command rank (`+0x96`: 1 Commander, 2 Admiral, 3 General) is not kept,
/// and a character aboard names its fleet, not its ship.
fn character_text(world: &GameWorld, character: &rebellion_core::world::Character) -> String {
    if character.is_killed {
        return format!("{} - Killed", character.display_name());
    }
    // The family-0x98 branch (0x00465fed) names the abode and skips the
    // state and rank words (jail.md, "The Personnel Finder row").
    if let Some(abode) = character.held_abode() {
        return format!("{} - {}", character.display_name(), abode.name());
    }
    let location = character
        .current_fleet
        .and_then(|fleet| fleet_label(world, fleet))
        .or_else(|| container_name(world, character))
        .unwrap_or_else(|| "Location Unknown".to_owned());
    match character_state(character) {
        Some(state) => format!("{} - {location} ( {state} ) ", character.display_name()),
        None => format!("{} - {location}", character.display_name()),
    }
}

/// Build the Characters view rows for `tab` (`FUN_00464e10` view==3,
/// `FUN_00465bb0`).
#[must_use]
pub fn character_rows(
    world: &GameWorld,
    _fog: &FogState,
    _player: Faction,
    tab: PersonnelTab,
) -> Vec<PersonnelFinderRow> {
    let mut rows = Vec::new();
    for (key, character) in &world.characters {
        if !character.recruited {
            continue;
        }
        if !tab.lists(character.is_alliance) {
            continue;
        }
        // hyp: fog gating for opposing side's characters.
        let location = container_name(world, character).unwrap_or_default();
        let state = character_state(character).unwrap_or_default().to_owned();
        let display = character_text(world, character);
        rows.push(PersonnelFinderRow::Character {
            key,
            name: display,
            location,
            state,
            system: character.current_system,
        });
    }
    finder_common::sort_rows_by_name(&mut rows);
    rows
}

/// Build the `SpecForces` view rows for `tab` (`FUN_00464e10` view==2,
/// `FUN_00465540`).
#[must_use]
pub fn specforces_rows(
    world: &GameWorld,
    _fog: &FogState,
    _player: Faction,
    tab: PersonnelTab,
) -> Vec<PersonnelFinderRow> {
    let mut rows = Vec::new();

    for (system_key, system) in &world.systems {
        // Direct SF units at this system.
        let ground = finder_common::side_keys(&system.special_forces, tab.index(), |key| {
            world.special_forces.get(key).map(|sf| sf.is_alliance)
        });
        if !ground.is_empty() {
            let counts = count_sf(world, &ground, tab);
            rows.push(PersonnelFinderRow::SpecForces {
                name: system.name.clone(),
                system: system_key,
                fleet: None,
                counts,
            });
        }

        // hyp: fleets at this system carrying SF units. The decompile
        // (FUN_00464e10) walks each fleet's ships and checks for SF
        // cargo; the port approximates by walking fleet characters and
        // system SF associations.
        // port: fleet-carried SF is not yet modelled in the port; only
        // surface SF units appear.
    }

    finder_common::sort_rows_by_name(&mut rows);
    rows
}

/// Build rows for the current view.
#[must_use]
pub fn rows(
    world: &GameWorld,
    fog: &FogState,
    player: Faction,
    tab: PersonnelTab,
    view: PersonnelView,
) -> Vec<PersonnelFinderRow> {
    match view {
        PersonnelView::Characters => character_rows(world, fog, player, tab),
        PersonnelView::SpecForces => specforces_rows(world, fog, player, tab),
    }
}

/// What the Personnel Finder asks of the galaxy view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonnelFinderAction {
    /// Character row: open the character window (`FUN_00429440` family 0x30).
    OpenCharacter {
        character: CharacterKey,
        system: Option<SystemKey>,
    },
    /// `SpecForces` system row: open the Sector and System windows
    /// (`FUN_00429440` family 0x90).
    OpenSystem { system: SystemKey },
    /// `SpecForces` fleet row: open the Fleet window.
    OpenFleet { system: SystemKey, fleet: FleetKey },
}

/// Manager-owned Personnel Finder intent. Dependent opens retain the exact
/// Finder identity whose accepted close receipt gates their publication.
pub type ManagedPersonnelFinderAction = finder_common::ManagedFinderAction<PersonnelFinderAction>;

#[derive(Debug, Clone)]
struct OpenFinder {
    ui: FinderUiState,
    tab: PersonnelTab,
    view: PersonnelView,
    chosen: Option<usize>,
}

finder_common::define_finder_state! {
    content: OpenFinder;
    /// The Personnel Finder, one at most (`FUN_0042a4d0`).
    state: PersonnelFinderState;
    /// Candidate Personnel Finder content. It remains inert until the app
    /// coordinator accepts the singleton identity and returns its open receipt.
    prepared: PreparedPersonnelFinder;
    kind: WindowKind::PersonnelFinder;
    layer: "original-personnel-finder";
    open: |faction, window_id| {
        OpenFinder {
            ui: FinderUiState::new(window_id, faction),
            tab: finder_common::side_value(faction, PersonnelTab::ALL),
            view: PersonnelView::SpecForces,
            chosen: None,
        }
    }
}

/// Rows visible at once: 161 / 20 = 8.
const VISIBLE_ROWS: usize = crate::list_control::visible_row_capacity(LIST.3, ROW_HEIGHT);

const FINDER_CONFIG: finder_common::SideFinderConfig<PersonnelTab> = finder_common::side_finder_config! {
    PersonnelTab, &PersonnelTab::ALL;
    area: "original-personnel-finder",
    name: "original-personnel-finder-name",
    list_id: "original-personnel-finder-list",
    row: "personnel",
    title: "Personnel Finder",
    name_label: "Character Name";
    list: LIST,
    row_height: ROW_HEIGHT,
    visible_rows: VISIBLE_ROWS;
    controls: (PERSONNEL_FINDER_NAME_CONTROL, PERSONNEL_FINDER_LIST_CONTROL),
};

impl PersonnelFinderState {
    finder_common::impl_finder_category_setter!(set_tab(PersonnelTab) => tab);
    finder_common::impl_finder_category_setter!(set_view(PersonnelView) => view);
}

type PersonnelFinderDrawOutcome = finder_common::FinderDrawOutcome<PersonnelFinderAction>;

/// Draw a fixture-opened Personnel Finder, if any, and report an action.
pub fn draw_personnel_finder(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    state: &mut PersonnelFinderState,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<PersonnelFinderAction> {
    let outcome = draw_personnel_finder_outcome(ctx, world, fog, state, None, layout, cache);
    finder_common::consume_direct_outcome(outcome, || state.close())
}

/// Draw the open manager-owned Personnel Finder using the frozen semantic focus.
pub fn draw_personnel_finder_focused(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    state: &mut PersonnelFinderState,
    focus: Option<FocusTarget>,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<ManagedPersonnelFinderAction> {
    let finder = state.window_id()?;
    let outcome = draw_personnel_finder_outcome(ctx, world, fog, state, focus, layout, cache);
    finder_common::manage_outcome(finder, outcome)
}

#[allow(
    clippy::too_many_lines,
    reason = "The window carries its existing render context plus semantic focus; splitting it hides the original control order."
)]
fn draw_personnel_finder_outcome(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    state: &mut PersonnelFinderState,
    focus: Option<FocusTarget>,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<PersonnelFinderDrawOutcome> {
    let (window, side_idx, controls) = finder_common::side_finder_snapshot!(state, CONTROLS);
    let list = rows(
        world,
        fog,
        finder_common::player_side(window.ui.faction),
        window.tab,
        window.view,
    );
    let mut new_view_tab = None;
    let outcome = finder_common::draw_coloured_side_finder_window!(
        ctx, cache, layout, focus, window, list, &FINDER_CONFIG,
        finder_common::side_finder_presentation!(
            window.tab.label(), (40.0, window.view.label_y()), window.tab.index();
            PANELS[window.view.index()][window.tab.index()], side_idx, RAIL,
            (controls.close, controls.display)
        );
        extra_controls: |ui, canvas, cache| {
            // View tabs (FUN_004637a0 group 0x6e).
            for candidate in PersonnelView::ALL {
                let y_offset = match candidate {
                    PersonnelView::SpecForces => 0.0,
                    PersonnelView::Characters => 54.0,
                };
                let view_rect = (
                    controls.view_origin.0,
                    controls.view_origin.1 + y_offset,
                    controls.view_button_size.0,
                    controls.view_button_size.1,
                );
                let art = VIEW_TAB_ART[side_idx][candidate.index()];
                let down = candidate == window.view;
                if canvas.button(
                    ui,
                    cache,
                    finder_common::BitmapControl::new(view_rect, art),
                    ("personnel-view-tab", candidate.index()),
                    down,
                ) {
                    new_view_tab = Some(candidate);
                }
            }
        }, paint_details: |canvas: &finder_common::FinderCanvas<'_>,
                           painter: &egui::Painter,
                           row: &PersonnelFinderRow,
                           line: f32,
                           cell_font: &egui::FontId,
                           colours: (egui::Color32, egui::Color32)| {
            if let PersonnelFinderRow::SpecForces { counts, .. } = row {
                // Four columns at x = 222 + 28*i (FUN_00465540).
                finder_common::paint_count_cells(
                    painter,
                    canvas.frame,
                    canvas.scale,
                    (LIST.0, LIST.1),
                    ROW_HEIGHT,
                    line,
                    222.0,
                    counts.iter().map(|cell| cell.count),
                    cell_font,
                    colours,
                );
            }
        };
        row_action: action_for_row,
    );
    let open_window = state.window.as_mut()?;
    let committed = finder_common::commit_indexed_output(
        &mut open_window.ui,
        &mut open_window.tab,
        &mut open_window.chosen,
        &list,
        outcome,
        |index, _| index,
    );
    if let Some(view) = new_view_tab {
        state.set_view(view);
    }
    committed
}

fn action_for_row(row: &PersonnelFinderRow) -> PersonnelFinderAction {
    match row {
        PersonnelFinderRow::Character { key, system, .. } => PersonnelFinderAction::OpenCharacter {
            character: *key,
            system: *system,
        },
        PersonnelFinderRow::SpecForces { system, fleet, .. } => finder_common::location_action(
            *system,
            *fleet,
            |system, fleet| PersonnelFinderAction::OpenFleet { system, fleet },
            |system| PersonnelFinderAction::OpenSystem { system },
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        alliance_system as add_system, control_focus as focus, finder_escape_input as escape_input,
        finder_layout as layout, key_state, tap_key as tap,
    };
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{Character, SkillPair};
    use rebellion_windowing::{ContentStatus, FocusTarget, WindowId, WindowOwner};

    crate::test_support::define_managed_finder_fixture!(
        managed_finder,
        PersonnelFinderState,
        "Personnel Finder"
    );

    fn drive_focused_in_context(
        ctx: &egui::Context,
        world: &GameWorld,
        fog: &FogState,
        state: &mut PersonnelFinderState,
        focus: FocusTarget,
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<ManagedPersonnelFinderAction> {
        let layout = layout();
        let mut cache = BmpCache::new();
        crate::test_support::run_action_frames(ctx, frames, |ctx| {
            draw_personnel_finder_focused(ctx, world, fog, state, Some(focus), layout, &mut cache)
        })
    }

    fn drive_event(
        ctx: &egui::Context,
        world: &GameWorld,
        fog: &FogState,
        state: &mut PersonnelFinderState,
        focus: FocusTarget,
        event: egui::Event,
    ) -> Option<ManagedPersonnelFinderAction> {
        drive_focused_in_context(ctx, world, fog, state, focus, vec![vec![event]])
    }

    fn drive_key(
        ctx: &egui::Context,
        world: &GameWorld,
        fog: &FogState,
        state: &mut PersonnelFinderState,
        focus: FocusTarget,
        key: egui::Key,
    ) -> Option<ManagedPersonnelFinderAction> {
        drive_event(ctx, world, fog, state, focus, key_state(key, true))
    }

    fn drive_key_without_action(
        ctx: &egui::Context,
        world: &GameWorld,
        fog: &FogState,
        state: &mut PersonnelFinderState,
        focus: FocusTarget,
        key: egui::Key,
    ) {
        assert_eq!(drive_key(ctx, world, fog, state, focus, key), None);
    }

    fn drive_tab(
        ctx: &egui::Context,
        world: &GameWorld,
        fog: &FogState,
        state: &mut PersonnelFinderState,
        focus: FocusTarget,
        key: egui::Key,
        expected: PersonnelTab,
    ) {
        drive_key_without_action(ctx, world, fog, state, focus, key);
        assert_eq!(state.window.as_ref().unwrap().tab, expected);
    }

    fn drive_text_without_action(
        ctx: &egui::Context,
        world: &GameWorld,
        fog: &FogState,
        state: &mut PersonnelFinderState,
        focus: FocusTarget,
        text: &str,
    ) {
        assert_eq!(
            drive_event(
                ctx,
                world,
                fog,
                state,
                focus,
                egui::Event::Text(text.into()),
            ),
            None
        );
    }

    fn managed_navigation() -> (
        PersonnelFinderState,
        WindowId,
        FocusTarget,
        FocusTarget,
        egui::Context,
    ) {
        let (_manager, state, id, name_focus, list_focus, ctx) =
            crate::test_support::finder_navigation_fixture(
                managed_finder(),
                PERSONNEL_FINDER_NAME_CONTROL,
                PERSONNEL_FINDER_LIST_CONTROL,
            );
        (state, id, name_focus, list_focus, ctx)
    }

    fn draw_fixture_frame(
        ctx: &egui::Context,
        world: &GameWorld,
        fog: &FogState,
        state: &mut PersonnelFinderState,
        layout: CockpitLayout,
        cache: &mut BmpCache,
        screen_rect: egui::Rect,
    ) {
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(screen_rect),
                ..Default::default()
            },
            |ctx| {
                let _ = draw_personnel_finder(ctx, world, fog, state, layout, cache);
            },
        );
    }

    #[test]
    fn personnel_finder_duplicate_preserves_content() {
        let (mut manager, mut state, id) = managed_finder();

        let content = state.window.as_mut().unwrap();
        content.tab = PersonnelTab::Imperial;
        content.view = PersonnelView::Characters;
        content.chosen = Some(3);
        let focused = focus(id, PERSONNEL_FINDER_LIST_CONTROL);
        crate::test_support::assert_focused_duplicate_finder_ui(
            &mut manager,
            &mut state,
            id,
            focused,
            "Leia",
            2,
            false,
        );
        let content = state.window.as_ref().unwrap();
        assert_eq!(content.tab, PersonnelTab::Imperial);
        assert_eq!(content.view, PersonnelView::Characters);
        assert_eq!(content.chosen, Some(3));
    }

    #[test]
    fn personnel_finder_rejects_stale_publication_receipts_without_retargeting() {
        let (mut manager, mut state, id) = managed_finder();
        let content = state.window.as_mut().unwrap();
        content.tab = PersonnelTab::Imperial;
        content.ui.name = "Leia".into();

        crate::test_support::assert_stale_finder_publications_rejected(
            &mut manager,
            &mut state,
            id,
            WindowKind::TroopFinder,
        );
        let content = state.window.as_ref().unwrap();
        assert_eq!(content.tab, PersonnelTab::Imperial);
        assert_eq!(content.ui.name, "Leia");
    }

    #[test]
    fn personnel_finder_modal_blocks_underlay() {
        let (mut manager, state, id) = managed_finder();
        let (frame, inside) =
            crate::test_support::assert_finder_modal_routing(&manager, &state, id, layout());
        assert_eq!(state.keyboard_owner(&frame), Some(id));
        assert_eq!(
            state.hit_window(
                &frame,
                layout(),
                (inside.x + PERSONNEL_FINDER_WIDTH, inside.y),
            ),
            None
        );
        crate::test_support::open_singleton_window(
            &mut manager,
            WindowKind::TroopFinder,
            WindowOwner::StrategicRoot,
            "modal cover",
        );
        let blocked_frame = manager.begin_frame();
        assert!(!blocked_frame.allows_input(id));
        assert_eq!(state.keyboard_owner(&blocked_frame), None);
        assert_eq!(
            state.hit_window(&blocked_frame, layout(), (inside.x, inside.y)),
            None
        );
        assert_eq!(
            state.content_reports(&blocked_frame),
            [rebellion_windowing::ContentReport {
                window: id,
                status: ContentStatus::Ready,
            }]
        );
        assert_eq!(
            PersonnelFinderState::default().content_reports(&blocked_frame),
            [rebellion_windowing::ContentReport {
                window: id,
                status: ContentStatus::Unpaintable,
            }]
        );
    }

    #[test]
    fn personnel_finder_close_consumes_trigger() {
        let (mut manager, mut state, id) = managed_finder();
        let world = GameWorld::default();
        let fog = FogState::new(Faction::Alliance);
        crate::test_support::assert_managed_finder_escape_close(
            &mut manager,
            &mut state,
            id,
            "Personnel Finder",
            ManagedPersonnelFinderAction::Close(id),
            |ctx, state, focus, cache| {
                draw_personnel_finder_focused(ctx, &world, &fog, state, focus, layout(), cache)
            },
            PersonnelFinderState::is_open,
            PersonnelFinderState::remove_content,
        );
    }

    #[test]
    fn personnel_finder_focused_control_consumes_text_navigation_enter_and_escape() {
        let mut world = GameWorld::default();
        let alpha = add_system(&mut world, "Alpha");
        let yavin = add_system(&mut world, "Yavin");
        let zulu = add_system(&mut world, "Zulu");
        add_sf(&mut world, alpha, 0x3c00_0004, true);
        add_sf(&mut world, yavin, 0x3c00_0004, true);
        add_sf(&mut world, zulu, 0x3c00_0004, true);
        let fog = FogState::new(Faction::Alliance);
        let (mut state, id, name_focus, list_focus, ctx) = managed_navigation();
        let _ = ctx.run(egui::RawInput::default(), |_| {});

        drive_text_without_action(&ctx, &world, &fog, &mut state, name_focus, "Y");
        assert_eq!(state.window.as_ref().unwrap().ui.name, "Y");
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(1));
        drive_key_without_action(
            &ctx,
            &world,
            &fog,
            &mut state,
            name_focus,
            egui::Key::ArrowDown,
        );
        assert_eq!(state.take_focus_request(), Some(list_focus));
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(1));

        drive_key_without_action(
            &ctx,
            &world,
            &fog,
            &mut state,
            list_focus,
            egui::Key::ArrowDown,
        );
        crate::test_support::assert_indexed_finder_selection!(&state, 2, "Zulu");
        drive_key_without_action(
            &ctx,
            &world,
            &fog,
            &mut state,
            list_focus,
            egui::Key::ArrowDown,
        );
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(2));

        drive_key_without_action(
            &ctx,
            &world,
            &fog,
            &mut state,
            list_focus,
            egui::Key::ArrowUp,
        );
        crate::test_support::assert_indexed_finder_selection!(&state, 1, "Yavin");
        drive_key_without_action(
            &ctx,
            &world,
            &fog,
            &mut state,
            list_focus,
            egui::Key::ArrowUp,
        );
        crate::test_support::assert_indexed_finder_selection!(&state, 0, "Alpha");
        drive_key_without_action(
            &ctx,
            &world,
            &fog,
            &mut state,
            list_focus,
            egui::Key::ArrowUp,
        );
        assert_eq!(state.take_focus_request(), Some(name_focus));
        state.window.as_mut().unwrap().ui.name.clear();
        drive_text_without_action(&ctx, &world, &fog, &mut state, name_focus, "Y");
        assert_eq!(state.window.as_ref().unwrap().ui.name, "Y");

        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                tap(egui::Key::Enter),
            ),
            Some(ManagedPersonnelFinderAction::Open {
                finder: id,
                destination: PersonnelFinderAction::OpenSystem { system: yavin },
            })
        );
        assert!(state.is_open(), "managed content waits for close receipt");

        assert_eq!(state.window.as_ref().unwrap().tab, PersonnelTab::Alliance);
        for (key, expected) in [
            (egui::Key::ArrowRight, PersonnelTab::Imperial),
            (egui::Key::ArrowRight, PersonnelTab::Imperial),
            (egui::Key::ArrowLeft, PersonnelTab::Alliance),
        ] {
            drive_tab(&ctx, &world, &fog, &mut state, list_focus, key, expected);
        }

        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                tap(egui::Key::Escape),
            ),
            Some(ManagedPersonnelFinderAction::Close(id))
        );
        assert!(
            state.is_open(),
            "Escape is an intent until its receipt commits"
        );
    }

    #[test]
    fn personnel_finder_first_draw_filters_newlines_and_keeps_paste() {
        let world = GameWorld::default();
        let fog = FogState::new(Faction::Alliance);
        let (_manager, mut state, id) = managed_finder();
        let focus = focus(id, PERSONNEL_FINDER_NAME_CONTROL);
        let ctx = egui::Context::default();

        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                focus,
                vec![vec![
                    egui::Event::Text("\n".into()),
                    egui::Event::Text("\r".into()),
                    egui::Event::Paste("Y".into()),
                ]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().ui.name, "Y");
    }

    #[test]
    fn personnel_finder_list_focus_does_not_capture_deferred_name_input() {
        let world = GameWorld::default();
        let fog = FogState::new(Faction::Alliance);
        let (_manager, mut state, id) = managed_finder();
        state.window.as_mut().unwrap().ui.focus_name = false;
        let focus = focus(id, PERSONNEL_FINDER_LIST_CONTROL);
        let ctx = egui::Context::default();

        assert_eq!(
            drive_event(
                &ctx,
                &world,
                &fog,
                &mut state,
                focus,
                egui::Event::Text("Y".into()),
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().ui.name, "");
        assert!(ctx.memory(|memory| {
            memory.has_focus(egui::Id::new("original-personnel-finder-list"))
        }));
    }

    #[test]
    fn personnel_finder_navigation_normalizes_a_stale_selection() {
        let mut world = GameWorld::default();
        for (name, class) in [
            ("Alpha", 0x3c00_0004),
            ("Yavin", 0x3c00_0003),
            ("Zulu", 0x3c00_0002),
        ] {
            let system = add_system(&mut world, name);
            add_sf(&mut world, system, class, true);
        }
        let fog = FogState::new(Faction::Alliance);
        let (mut state, _id, name_focus, list_focus, ctx) = managed_navigation();
        state.window.as_mut().unwrap().chosen = Some(usize::MAX);

        drive_key_without_action(
            &ctx,
            &world,
            &fog,
            &mut state,
            name_focus,
            egui::Key::ArrowDown,
        );
        crate::test_support::assert_indexed_finder_selection!(&state, 2, "Zulu");

        state.window.as_mut().unwrap().chosen = Some(usize::MAX);
        drive_key_without_action(
            &ctx,
            &world,
            &fog,
            &mut state,
            list_focus,
            egui::Key::ArrowUp,
        );
        crate::test_support::assert_indexed_finder_selection!(&state, 1, "Yavin");
    }

    #[test]
    fn fixture_personnel_finder_never_emits_manager_focus_requests() {
        let world = GameWorld::default();
        let fog = FogState::new(Faction::Alliance);
        let mut state = PersonnelFinderState::default();
        assert!(state.open(CockpitFaction::Alliance));
        let ctx = egui::Context::default();
        let layout = layout();
        let mut cache = BmpCache::new();
        let screen_rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(700.0, 520.0));
        draw_fixture_frame(
            &ctx,
            &world,
            &fog,
            &mut state,
            layout,
            &mut cache,
            screen_rect,
        );
        state.window.as_mut().unwrap().ui.focus_name = true;
        ctx.memory_mut(|memory| {
            memory.surrender_focus(egui::Id::new("original-personnel-finder-name"));
        });
        draw_fixture_frame(
            &ctx,
            &world,
            &fog,
            &mut state,
            layout,
            &mut cache,
            screen_rect,
        );

        assert_eq!(state.window.as_ref().unwrap().ui.pending_focus, None);
    }

    fn default_character() -> Character {
        let skill = SkillPair {
            base: 50,
            variance: 0,
        };
        Character {
            dat_id: DatId::new(0x3000_0001),
            is_alliance: true,
            is_major: true,
            diplomacy: skill,
            espionage: skill,
            ship_design: skill,
            troop_training: skill,
            facility_design: skill,
            combat: skill,
            leadership: skill,
            loyalty: skill,
            recruited: true,
            ..Default::default()
        }
    }

    fn listed_luke(configure: impl FnOnce(&mut Character)) -> String {
        let mut world = GameWorld::default();
        let yavin = add_system(&mut world, "Yavin");
        let mut luke = default_character();
        luke.name = "Luke Skywalker".into();
        luke.current_system = Some(yavin);
        configure(&mut luke);
        world.characters.insert(luke);
        character_rows(
            &world,
            &FogState::new(Faction::Alliance),
            Faction::Alliance,
            PersonnelTab::Alliance,
        )[0]
        .display_name()
        .to_owned()
    }

    fn add_sf(
        world: &mut GameWorld,
        system: SystemKey,
        class_raw: u32,
        is_alliance: bool,
    ) -> SpecialForceKey {
        crate::test_support::add_special_force(world, system, class_raw, is_alliance)
    }

    #[test]
    fn the_specforces_view_lists_systems_holding_the_tabs_sf_units() {
        // FUN_00464e10: walk systems, check for SF of the tab's side.
        let mut world = GameWorld::default();
        let yavin = add_system(&mut world, "Yavin");
        add_sf(&mut world, yavin, 0x3c00_0004, true);
        add_sf(&mut world, yavin, 0x3c00_0004, true);
        add_sf(&mut world, yavin, 0x3c00_0002, true);
        let fog = FogState::new(Faction::Alliance);

        let listed = specforces_rows(&world, &fog, Faction::Alliance, PersonnelTab::Alliance);

        assert_eq!(listed.len(), 1);
        if let PersonnelFinderRow::SpecForces { counts, .. } = &listed[0] {
            assert_eq!(counts[0].count, 2, "first SF class column");
            assert_eq!(counts[1].count, 1, "second SF class column");
            assert_eq!(counts[2].count, 0);
            assert_eq!(counts[3].count, 0);
        } else {
            panic!("expected SpecForces row");
        }
    }

    #[test]
    fn the_characters_view_lists_recruited_living_characters_of_the_tabs_side() {
        // FUN_00464e10 view==3: walk characters of the side, call
        // FUN_00465bb0 for each.
        let mut world = GameWorld::default();
        let yavin = add_system(&mut world, "Yavin");

        let mut luke = default_character();
        luke.name = "Luke Skywalker".into();
        luke.is_alliance = true;
        luke.current_system = Some(yavin);
        luke.recruited = true;
        world.characters.insert(luke);

        let mut vader = default_character();
        vader.name = "Darth Vader".into();
        vader.is_alliance = false;
        vader.is_empire = true;
        vader.current_system = Some(yavin);
        vader.recruited = true;
        world.characters.insert(vader);

        // Not recruited: hidden.
        let mut hidden = default_character();
        hidden.name = "Hidden".into();
        hidden.is_alliance = true;
        hidden.recruited = false;
        world.characters.insert(hidden);

        let fog = FogState::new(Faction::Alliance);

        let alliance = character_rows(&world, &fog, Faction::Alliance, PersonnelTab::Alliance);
        assert_eq!(alliance.len(), 1);
        assert!(alliance[0].display_name().starts_with("Luke Skywalker"));

        let imperial = character_rows(&world, &fog, Faction::Alliance, PersonnelTab::Imperial);
        assert_eq!(imperial.len(), 1);
        assert!(imperial[0].display_name().starts_with("Darth Vader"));
    }

    #[test]
    fn display_opens_the_fleet_the_missions_or_the_defenses_window() {
        // FUN_00429440 family 0x30: a fleet parent opens kind 4; at a
        // system, a mission that is not hidden (FUN_00520b70 == 0) opens
        // kind 11, anything else kind 10. Manual p. 100: "system defenses,
        // fleet or mission".
        let mut character = default_character();
        assert_eq!(character_target(&character), CharacterTarget::Defenses);
        character.on_mission = true;
        assert_eq!(character_target(&character), CharacterTarget::Missions);
        character.on_hidden_mission = true;
        assert_eq!(character_target(&character), CharacterTarget::Defenses);
        let fleet = FleetKey::default();
        character.current_fleet = Some(fleet);
        character.on_hidden_mission = false;
        assert_eq!(character_target(&character), CharacterTarget::Fleet(fleet));
    }

    #[test]
    fn a_character_on_mission_shows_its_state_in_the_name() {
        // FUN_00465bb0: "Name - Location ( On Mission ) ".
        assert!(listed_luke(|luke| luke.on_mission = true).contains("On Mission"));
    }

    #[test]
    fn a_killed_character_is_listed_as_killed() {
        // FUN_00465bb0's destroyed branch; the reference capture lists
        // "Ackbar - Killed" (steam-guide guide-023.png).
        assert_eq!(
            listed_luke(|luke| luke.is_killed = true),
            "Luke Skywalker - Killed"
        );
    }

    #[test]
    fn a_rows_state_word_sits_in_spaced_brackets_and_captured_comes_first() {
        // FUN_00465bb0: " ( " + the first of Captured, On Mission, Injured,
        // Enroute + " ) " (DAT_006a8790, DAT_006a878c).
        let mut world = GameWorld::default();
        let yavin = add_system(&mut world, "Yavin");
        let mut leia = default_character();
        leia.name = "Leia".into();
        leia.current_system = Some(yavin);
        leia.on_mission = true;
        leia.is_captive = true;
        world.characters.insert(leia);
        // A traveller stands at its destination (move-order.md).
        let mut han = default_character();
        han.name = "Han".into();
        han.current_system = Some(yavin);
        han.en_route = true;
        world.characters.insert(han);
        // Aboard a fleet is not en route, wherever the fleet is.
        let mut wedge = default_character();
        wedge.name = "Wedge".into();
        wedge.current_fleet = Some(rebellion_core::ids::FleetKey::default());
        world.characters.insert(wedge);

        let fog = FogState::new(Faction::Alliance);
        let listed = character_rows(&world, &fog, Faction::Alliance, PersonnelTab::Alliance);
        assert_eq!(
            rows(
                &world,
                &fog,
                Faction::Alliance,
                PersonnelTab::Alliance,
                PersonnelView::Characters
            ),
            listed
        );
        let names: Vec<&str> = listed
            .iter()
            .map(PersonnelFinderRow::display_name)
            .collect();
        assert_eq!(
            names,
            [
                "Han - Yavin ( Enroute ) ",
                "Leia - Yavin ( Captured ) ",
                "Wedge - Location Unknown"
            ]
        );
    }

    #[test]
    fn a_jailed_character_reads_its_name_and_the_other_sides_jail_alone() {
        // FUN_00465bb0's family-0x98 branch (0x00465fed) names the abode
        // (FUN_004f62d0; ABODESD names TEXTSTRA 11600, 11616) and skips the
        // state and rank words. The jail is the captor's (FUN_00549580), and
        // the Characters view lists a character on its native side's tab
        // (FUN_004637a0 stores FUN_004edc80).
        for (is_alliance, tab, abode, text) in [
            (
                true,
                PersonnelTab::Alliance,
                rebellion_core::world::Abode::EmpireJail,
                "Wedge - Empire Jail",
            ),
            (
                false,
                PersonnelTab::Imperial,
                rebellion_core::world::Abode::AllianceJail,
                "Wedge - Alliance Jail",
            ),
        ] {
            let mut world = GameWorld::default();
            let mut wedge = default_character();
            wedge.name = "Wedge".into();
            wedge.is_alliance = is_alliance;
            wedge.is_empire = !is_alliance;
            wedge.is_captive = true;
            wedge.abode = Some(abode);
            world.characters.insert(wedge);

            let fog = FogState::new(Faction::Alliance);
            let listed = character_rows(&world, &fog, Faction::Alliance, tab);

            assert_eq!(listed[0].display_name(), text);
            let PersonnelFinderRow::Character {
                location, system, ..
            } = &listed[0]
            else {
                panic!("a character row");
            };
            assert_eq!((location.as_str(), *system), (abode.name(), None));
        }
    }

    #[test]
    fn a_character_out_of_its_jail_reads_its_new_container() {
        // FUN_00465bb0 names the character's container (FUN_004f62d0). An
        // abode holds a prisoner with no captor in no system or fleet
        // (FUN_00549580); a capture that names a captor, or a placement at
        // a system, takes it out.
        for (is_alliance, tab) in [
            (true, PersonnelTab::Alliance),
            (false, PersonnelTab::Imperial),
        ] {
            let mut world = GameWorld::default();
            let yavin = add_system(&mut world, "Yavin");
            let jailed = |name: &str| {
                let mut character = default_character();
                character.name = name.into();
                character.is_alliance = is_alliance;
                character.is_empire = !is_alliance;
                character.is_captive = true;
                character.abode = Some(rebellion_core::world::Abode::jail_of(!is_alliance));
                character
            };
            let mut held = jailed("Held");
            held.captured_by = Some(if is_alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            });
            world.characters.insert(held);
            let mut placed = jailed("Placed");
            placed.current_system = Some(yavin);
            world.characters.insert(placed);

            let fog = FogState::new(Faction::Alliance);
            let listed = character_rows(&world, &fog, Faction::Alliance, tab);
            let names: Vec<&str> = listed
                .iter()
                .map(PersonnelFinderRow::display_name)
                .collect();

            assert_eq!(
                names,
                [
                    "Held - Location Unknown ( Captured ) ",
                    "Placed - Yavin ( Captured ) "
                ],
                "{is_alliance}"
            );
        }
    }

    #[test]
    fn rows_are_sorted_by_name_ignoring_case() {
        // FUN_005f59f0: rows inserted in name order.
        let mut world = GameWorld::default();
        let beta = add_system(&mut world, "beta");
        let alpha = add_system(&mut world, "Alpha");
        add_sf(&mut world, beta, 0x3c00_0004, true);
        add_sf(&mut world, alpha, 0x3c00_0004, true);
        let fog = FogState::new(Faction::Alliance);

        let listed = specforces_rows(&world, &fog, Faction::Alliance, PersonnelTab::Alliance);
        let names: Vec<&str> = listed
            .iter()
            .map(PersonnelFinderRow::display_name)
            .collect();
        assert_eq!(names, ["Alpha", "beta"]);
    }

    #[test]
    fn typed_names_use_the_longest_prefix_and_a_tie_keeps_the_first_row() {
        // FUN_00609650 keeps a row only for a strictly longer match.
        crate::test_support::assert_finder_prefix_choices(
            |name| PersonnelFinderRow::SpecForces {
                name: name.into(),
                system: SystemKey::default(),
                fleet: None,
                counts: [SfCount::default(); 4],
            },
            prefix_choice,
        );
    }

    #[test]
    fn the_action_for_a_character_row_opens_character_and_sf_row_opens_system() {
        // FUN_00429440: character → family 0x30, system → family 0x90.
        let system = SystemKey::default();
        let character = CharacterKey::default();

        let char_row = PersonnelFinderRow::Character {
            key: character,
            name: "Luke - Yavin".into(),
            location: "Yavin".into(),
            state: String::new(),
            system: Some(system),
        };
        assert_eq!(
            action_for_row(&char_row),
            PersonnelFinderAction::OpenCharacter {
                character,
                system: Some(system),
            }
        );

        let sf_row = PersonnelFinderRow::SpecForces {
            name: "Yavin".into(),
            system,
            fleet: None,
            counts: [SfCount::default(); 4],
        };
        assert_eq!(
            action_for_row(&sf_row),
            PersonnelFinderAction::OpenSystem { system }
        );
    }

    #[test]
    fn the_visible_rows_count_is_eight() {
        // FUN_00607ea0: list 161 tall, rows 20 tall → 8 visible.
        assert_eq!(VISIBLE_ROWS, 8);
    }

    #[test]
    fn each_side_tab_names_its_side_and_paints_its_own_cells() {
        // TEXTSTRA 0x1893/0x1894; FUN_00465540's brushes as the Troop
        // Finder's: red with white text, green with black.
        assert_eq!(PersonnelTab::Alliance.label(), "Alliance Personnel");
        assert_eq!(PersonnelTab::Imperial.label(), "Imperial Personnel");
        crate::test_support::assert_finder_count_colours(
            PersonnelTab::Alliance.index(),
            PersonnelTab::Imperial.index(),
        );
    }

    #[test]
    fn opening_an_open_finder_keeps_its_state() {
        // FUN_0042a4d0 builds the window only when no type 0x17 is open.
        crate::test_support::assert_finder_reopen_preserves::<PersonnelFinderState>(
            |state| state.set_tab(PersonnelTab::Imperial),
            |state| assert_eq!(state.window.as_ref().unwrap().tab, PersonnelTab::Imperial),
        );
    }

    #[test]
    fn a_new_side_tab_clears_the_choice() {
        // FUN_00464e10: tab switch rebuilds the list.
        crate::test_support::assert_finder_category_change_clears!(
            PersonnelFinderState,
            Some(0),
            |state: &mut PersonnelFinderState| state.set_tab(PersonnelTab::Imperial),
        );
    }

    #[test]
    fn a_new_view_tab_clears_the_choice() {
        // FUN_00464e10: view switch rebuilds the list.
        crate::test_support::assert_finder_category_change_clears!(
            PersonnelFinderState,
            Some(0),
            |state: &mut PersonnelFinderState| state.set_view(PersonnelView::Characters),
        );
    }

    #[test]
    fn the_finder_opens_with_specforces_view_and_players_side_tab() {
        // FUN_004637a0: FUN_0060d7e0(+0x148, 3, 1) = SpecForces view,
        // FUN_0060d7e0(+0x144, side, 1) = player's side.
        let mut state = PersonnelFinderState::default();
        state.open(CockpitFaction::Alliance);
        let window = state.window.as_ref().unwrap();
        assert_eq!(window.view, PersonnelView::SpecForces);
        assert_eq!(window.tab, PersonnelTab::Alliance);

        let mut state = PersonnelFinderState::default();
        state.open(CockpitFaction::Empire);
        let window = state.window.as_ref().unwrap();
        assert_eq!(window.view, PersonnelView::SpecForces);
        assert_eq!(window.tab, PersonnelTab::Imperial);
    }

    #[test]
    fn escape_closes_the_finder() {
        let mut state = PersonnelFinderState::default();
        state.open(CockpitFaction::Alliance);
        let world = GameWorld::default();
        let fog = FogState::new(Faction::Alliance);
        let mut cache = BmpCache::new();
        let layout = layout();
        let ctx = egui::Context::default();

        let input = escape_input();
        let _ = ctx.run(input, |ctx| {
            draw_personnel_finder(ctx, &world, &fog, &mut state, layout, &mut cache);
        });

        assert!(!state.is_open());
    }

    #[test]
    fn the_finder_blocks_only_its_own_rectangle() {
        let mut state = PersonnelFinderState::default();
        crate::test_support::assert_finder_bounds(&mut state, layout());
    }

    #[test]
    fn count_sf_tallies_by_class_for_the_tabs_side() {
        // FUN_00465540: four columns, each the count of its DatId.
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Yavin");
        let k1 = add_sf(&mut world, system, 0x3c00_0004, true); // col 0
        let k2 = add_sf(&mut world, system, 0x3c00_0004, true); // col 0
        let k3 = add_sf(&mut world, system, 0x3c00_0003, true); // col 3
        let _k4 = add_sf(&mut world, system, 0x3c00_0007, false); // empire, ignored

        let keys = vec![k1, k2, k3];
        let counts = count_sf(&world, &keys, PersonnelTab::Alliance);
        assert_eq!(counts[0].count, 2);
        assert_eq!(counts[1].count, 0);
        assert_eq!(counts[3].count, 1);
        assert!(counts[0].all_usable);
    }

    #[test]
    fn the_tab_label_y_differs_by_view() {
        // FUN_00464e10: Characters → y=0x77=119, SpecForces → y=0x73=115.
        assert!((PersonnelView::Characters.label_y() - 119.0).abs() < f32::EPSILON);
        assert!((PersonnelView::SpecForces.label_y() - 115.0).abs() < f32::EPSILON);
    }
}
