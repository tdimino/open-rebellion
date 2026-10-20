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

use egui_macroquad::egui;
use rebellion_core::dat::Faction;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{CharacterKey, FleetKey, SpecialForceKey, SystemKey};
use rebellion_core::world::GameWorld;
use rebellion_windowing::{
    ContentReport, ContentStatus, ControlId, ExistingEffect, FocusTarget, RoutingSnapshot,
    WindowId, WindowKind, WindowOwner, WindowSpec, WindowTransition,
};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::paint_native;
use crate::mission_dialog::{button, galaxy_centered_rect};
use crate::system_window::{fleet_label, logical_rect, rect_contains};

pub const PERSONNEL_FINDER_WIDTH: f32 = 470.0;
pub const PERSONNEL_FINDER_HEIGHT: f32 = 330.0;
/// Stable child identity for the name edit (`0xcb`).
pub const PERSONNEL_FINDER_NAME_CONTROL: ControlId = ControlId(0xcb);
/// Stable child identity for the result list (`0xca`).
pub const PERSONNEL_FINDER_LIST_CONTROL: ControlId = ControlId(0xca);

// STRATEGY bitmaps by side (FUN_004637a0).
const BACKGROUND: [u32; 2] = [10335, 10336];
const RAIL: [u32; 2] = [10586, 10590];
const RAIL_AT: (f32, f32) = (412.0, 0.0);
const PANEL_AT: (f32, f32) = (12.0, 13.0);

/// Panel bitmaps indexed by `[view][side]` (FUN_004637a0 / FUN_00464e10).
///
/// Characters + Alliance = 10538, Characters + Imperial = 10539,
/// SpecForces + Alliance = 10536, SpecForces + Imperial = 10537.
const PANELS: [[u32; 2]; 2] = [
    [10538, 10539], // Characters view: Alliance, Imperial
    [10536, 10537], // SpecForces view: Alliance, Imperial
];

/// The side tab (FUN_004637a0's control 100, TEXTSTRA 0x1893/0x1894).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PersonnelTab {
    #[default]
    Alliance,
    Imperial,
}

impl PersonnelTab {
    const ALL: [Self; 2] = [Self::Alliance, Self::Imperial];

    fn lists(self, is_alliance: bool) -> bool {
        match self {
            Self::Alliance => is_alliance,
            Self::Imperial => !is_alliance,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Alliance => "Alliance Personnel",
            Self::Imperial => "Imperial Personnel",
        }
    }

    const fn index(self) -> usize {
        match self {
            Self::Alliance => 0,
            Self::Imperial => 1,
        }
    }
}

/// The view tab (FUN_004637a0's control 0x6e, values 2 and 3).
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

    /// The tab label y: Characters → 119, SpecForces → 115
    /// (FUN_00464e10 lines 294–300).
    const fn label_y(self) -> f32 {
        match self {
            Self::Characters => 119.0,
            Self::SpecForces => 115.0,
        }
    }
}

/// The side's Close and Display controls (FUN_004637a0).
struct SideControls {
    close: Control,
    display: Control,
    /// View tab group origin and individual button size.
    view_origin: (f32, f32),
    view_button_size: (f32, f32),
}

#[derive(Debug, Clone, Copy)]
struct Control {
    rect: (f32, f32, f32, f32),
    art: (u32, u32),
}

const CONTROLS: [SideControls; 2] = [
    // Alliance (side 1): FUN_004637a0.
    SideControls {
        close: Control {
            rect: (423.0, 25.0, 32.0, 31.0),
            art: (10514, 10515),
        },
        display: Control {
            rect: (423.0, 93.0, 32.0, 31.0),
            art: (10518, 10519),
        },
        view_origin: (423.0, 147.0),
        view_button_size: (32.0, 31.0),
    },
    // Empire (side 2).
    SideControls {
        close: Control {
            rect: (426.0, 21.0, 44.0, 41.0),
            art: (10516, 10517),
        },
        display: Control {
            rect: (426.0, 89.0, 42.0, 41.0),
            art: (10520, 10521),
        },
        view_origin: (426.0, 143.0),
        view_button_size: (42.0, 41.0),
    },
];

/// View tab art: `[side][view]` (FUN_004637a0).
/// SpecForces: Alliance 10542/10543, Empire 10544/10545.
/// Characters: Alliance 10546/10547, Empire 10548/10549.
const VIEW_TAB_ART: [[(u32, u32); 2]; 2] = [
    // Alliance: [SpecForces, Characters]
    [(10542, 10543), (10546, 10547)],
    // Empire: [SpecForces, Characters]
    [(10544, 10545), (10548, 10549)],
];

/// Side tabs (group 100) at (36, 72): same as troop/fleet finder.
const SIDE_TABS: [Control; 2] = [
    Control {
        rect: (36.0, 72.0, 49.0, 41.0),
        art: (10502, 10503),
    },
    Control {
        rect: (88.0, 72.0, 49.0, 41.0),
        art: (10505, 10506),
    },
];

const TITLE_AT: (f32, f32) = (36.0, 14.0);
const NAME_LABEL_AT: (f32, f32) = (36.0, 48.0);
const NAME_BOX: (f32, f32, f32, f32) = (143.0, 45.0, 250.0, 18.0);
const LIST: (f32, f32, f32, f32) = (37.0, 143.0, 349.0, 161.0);
const ROW_HEIGHT: f32 = 20.0;

/// Four Special Forces DatIds per side (FUN_00465540).
const ALLIANCE_SF_CLASSES: [u32; 4] = [0x3c00_0004, 0x3c00_0002, 0x3c00_0001, 0x3c00_0003];
const EMPIRE_SF_CLASSES: [u32; 4] = [0x3c00_0007, 0x3c00_0006, 0x3c00_0008, 0x3c00_0005];

fn sf_classes(tab: PersonnelTab) -> &'static [u32; 4] {
    match tab {
        PersonnelTab::Alliance => &ALLIANCE_SF_CLASSES,
        PersonnelTab::Imperial => &EMPIRE_SF_CLASSES,
    }
}

/// One count cell in a SpecForces row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SfCount {
    pub count: u32,
    /// port: always true (no usable bit in the port).
    pub all_usable: bool,
}

/// One list row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersonnelFinderRow {
    /// A character: "Name - Location (state)" (FUN_00465bb0).
    Character {
        key: CharacterKey,
        name: String,
        location: String,
        state: String,
        system: Option<SystemKey>,
    },
    /// A system (or fleet) with SpecForces counts (FUN_00465540).
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
            Self::Character { name, .. } => name,
            Self::SpecForces { name, .. } => name,
        }
    }
}

/// A typed name chooses the first row it best begins (FUN_00609650).
#[must_use]
pub fn prefix_choice(rows: &[PersonnelFinderRow], text: &str) -> Option<usize> {
    let typed: Vec<char> = text.chars().map(|c| c.to_ascii_lowercase()).collect();
    let mut best: Option<(usize, usize)> = None;
    for (index, row) in rows.iter().enumerate() {
        let common = row
            .display_name()
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .zip(&typed)
            .take_while(|(a, b)| a == *b)
            .count();
        if common > 0 && best.is_none_or(|(_, length)| common > length) {
            best = Some((index, common));
        }
    }
    best.map(|(index, _)| index)
}

/// Count special forces of each class among the given keys.
fn count_sf(world: &GameWorld, keys: &[SpecialForceKey], tab: PersonnelTab) -> [SfCount; 4] {
    let classes = sf_classes(tab);
    let mut counts = [SfCount {
        count: 0,
        all_usable: true,
    }; 4];
    for &key in keys {
        let Some(sf) = world.special_forces.get(key) else {
            continue;
        };
        if !tab.lists(sf.is_alliance) {
            continue;
        }
        let raw = sf.class_dat_id.raw();
        for (i, &class_raw) in classes.iter().enumerate() {
            if raw == class_raw {
                counts[i].count += 1;
                break;
            }
        }
    }
    counts
}

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
    let location = character
        .current_fleet
        .and_then(|fleet| fleet_label(world, fleet))
        .or_else(|| {
            character
                .current_system
                .and_then(|system| world.systems.get(system))
                .map(|system| system.name.clone())
        })
        .unwrap_or_else(|| "Location Unknown".to_owned());
    match character_state(character) {
        Some(state) => format!("{} - {location} ( {state} ) ", character.display_name()),
        None => format!("{} - {location}", character.display_name()),
    }
}

/// Build the Characters view rows for `tab` (FUN_00464e10 view==2,
/// FUN_00465bb0).
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
        let location = character
            .current_system
            .and_then(|sys| world.systems.get(sys))
            .map_or_else(String::new, |sys| sys.name.clone());
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
    rows.sort_by_cached_key(|row| row.display_name().to_ascii_lowercase());
    rows
}

/// Build the SpecForces view rows for `tab` (FUN_00464e10 view==3,
/// FUN_00465540).
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
        let ground: Vec<SpecialForceKey> = system
            .special_forces
            .iter()
            .copied()
            .filter(|&key| {
                world
                    .special_forces
                    .get(key)
                    .is_some_and(|sf| tab.lists(sf.is_alliance))
            })
            .collect();
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

    rows.sort_by_cached_key(|row| row.display_name().to_ascii_lowercase());
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
    /// Character row: open the character window (FUN_00429440 family 0x30).
    OpenCharacter {
        character: CharacterKey,
        system: Option<SystemKey>,
    },
    /// SpecForces system row: open the Sector and System windows
    /// (FUN_00429440 family 0x90).
    OpenSystem { system: SystemKey },
    /// SpecForces fleet row: open the Fleet window.
    OpenFleet { system: SystemKey, fleet: FleetKey },
}

/// Manager-owned Personnel Finder intent. Dependent opens retain the exact
/// Finder identity whose accepted close receipt gates their publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedPersonnelFinderAction {
    /// Close or Escape requests teardown of this exact manager identity.
    Close(WindowId),
    /// A selected destination paired with the Finder identity that must close.
    Open {
        finder: WindowId,
        destination: PersonnelFinderAction,
    },
}

#[derive(Debug, Clone)]
struct OpenFinder {
    /// Shared strategic-window identity accepted at the app commit boundary.
    id: Option<WindowId>,
    faction: CockpitFaction,
    tab: PersonnelTab,
    view: PersonnelView,
    name: String,
    chosen: Option<usize>,
    first_row: usize,
    focus_name: bool,
    /// Child focus requested by content input and committed by the app after
    /// this immutable frame finishes.
    pending_focus: Option<ControlId>,
    last_clicked: Option<(usize, f64)>,
}

/// The Personnel Finder, one at most (FUN_0042a4d0).
#[derive(Debug, Clone, Default)]
pub struct PersonnelFinderState {
    window: Option<OpenFinder>,
}

/// Candidate Personnel Finder content. It remains inert until the app
/// coordinator accepts the singleton identity and returns its open receipt.
#[derive(Debug, Clone, Copy)]
pub struct PreparedPersonnelFinder {
    faction: CockpitFaction,
}

impl PreparedPersonnelFinder {
    #[must_use]
    pub const fn spec(self) -> WindowSpec {
        crate::windowing_policy::singleton_spec(
            WindowKind::PersonnelFinder,
            WindowOwner::StrategicRoot,
        )
    }
}

/// Rows visible at once: 161 / 20 = 8.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Eight whole rows of 20 in a list 161 tall."
)]
const VISIBLE_ROWS: usize = (LIST.3 / ROW_HEIGHT) as usize;

fn scrolled_to(first_row: usize, row: usize) -> usize {
    if row < first_row {
        row
    } else if row >= first_row + VISIBLE_ROWS {
        row + 1 - VISIBLE_ROWS
    } else {
        first_row
    }
}

const fn side(faction: CockpitFaction) -> usize {
    match faction {
        CockpitFaction::Alliance => 0,
        CockpitFaction::Empire => 1,
    }
}

const fn player_side(faction: CockpitFaction) -> Faction {
    match faction {
        CockpitFaction::Alliance => Faction::Alliance,
        CockpitFaction::Empire => Faction::Empire,
    }
}

fn window_rect(layout: CockpitLayout) -> egui::Rect {
    galaxy_centered_rect(layout, PERSONNEL_FINDER_WIDTH, PERSONNEL_FINDER_HEIGHT)
}

fn at(frame: egui::Rect, scale: f32, (x, y, w, h): (f32, f32, f32, f32)) -> egui::Rect {
    logical_rect(frame, scale, x, y, w, h)
}

/// Cell colours for SpecForces counts (FUN_00465540): same as the troop
/// finder. Alliance tab → white on red; Imperial tab → black on green.
fn sf_cell_colours(tab: PersonnelTab) -> (egui::Color32, egui::Color32) {
    match tab {
        PersonnelTab::Alliance => (egui::Color32::from_rgb(255, 0, 0), egui::Color32::WHITE),
        PersonnelTab::Imperial => (egui::Color32::from_rgb(0, 255, 0), egui::Color32::BLACK),
    }
}

impl PersonnelFinderState {
    /// Prepares inert content for a manager-owned singleton open.
    #[must_use]
    pub const fn prepare_open(faction: CockpitFaction) -> PreparedPersonnelFinder {
        PreparedPersonnelFinder { faction }
    }

    /// Publishes prepared content only for a new manager identity. A duplicate
    /// NoOp acknowledges the existing identity without rebuilding its state.
    pub fn commit_open(
        &mut self,
        prepared: PreparedPersonnelFinder,
        transition: &WindowTransition,
    ) -> Option<WindowId> {
        match *transition {
            WindowTransition::Opened { id } if self.window.is_none() => {
                let tab = match prepared.faction {
                    CockpitFaction::Alliance => PersonnelTab::Alliance,
                    CockpitFaction::Empire => PersonnelTab::Imperial,
                };
                self.window = Some(OpenFinder {
                    id: Some(id),
                    faction: prepared.faction,
                    tab,
                    view: PersonnelView::SpecForces,
                    name: String::new(),
                    chosen: None,
                    first_row: 0,
                    focus_name: true,
                    pending_focus: None,
                    last_clicked: None,
                });
                Some(id)
            }
            WindowTransition::Existing {
                id,
                effect: ExistingEffect::NoOp,
            } if self.window_id() == Some(id) => Some(id),
            _ => None,
        }
    }

    /// Exact manager identity currently owning the published content.
    #[must_use]
    pub fn window_id(&self) -> Option<WindowId> {
        self.window.as_ref().and_then(|window| window.id)
    }

    /// Egui layer attached to the exact manager-issued Personnel Finder.
    #[must_use]
    pub fn managed_layer_id(&self, id: WindowId) -> Option<egui::LayerId> {
        (self.window_id() == Some(id)).then(|| {
            egui::LayerId::new(
                egui::Order::Tooltip,
                egui::Id::new("original-personnel-finder"),
            )
        })
    }

    /// Exact focused manager identity that receives this frame's keys.
    #[must_use]
    pub fn keyboard_owner(&self, frame: &RoutingSnapshot) -> Option<WindowId> {
        let id = self.window_id()?;
        matches!(
            frame.keyboard_route().target,
            rebellion_windowing::RouteTarget::Window(target) if target.window == id
        )
        .then_some(id)
    }

    /// Manager-owned Finder hit for event-local pointer routing.
    #[must_use]
    pub fn hit_window(
        &self,
        frame: &RoutingSnapshot,
        layout: CockpitLayout,
        point: (f32, f32),
    ) -> Option<WindowId> {
        let id = self.window_id()?;
        (frame
            .key(id)
            .is_some_and(|key| key.kind == WindowKind::PersonnelFinder)
            && frame.allows_input(id)
            && rect_contains(window_rect(layout), egui::pos2(point.0, point.1)))
        .then_some(id)
    }

    /// Explicit Personnel Finder content state for the immutable manager frame.
    #[must_use]
    pub fn content_reports(&self, frame: &RoutingSnapshot) -> Vec<ContentReport> {
        frame
            .paint_order()
            .iter()
            .copied()
            .filter(|id| {
                frame
                    .key(*id)
                    .is_some_and(|key| key.kind == WindowKind::PersonnelFinder)
            })
            .map(|id| ContentReport {
                window: id,
                status: if self.window_id() == Some(id) {
                    ContentStatus::Ready
                } else {
                    ContentStatus::Unpaintable
                },
            })
            .collect()
    }

    /// Releases only content named by manager close/reconciliation receipts.
    pub fn remove_content(&mut self, removed: &[WindowId]) {
        if self.window_id().is_some_and(|id| removed.contains(&id)) {
            self.window = None;
        }
    }

    /// Takes one child-focus intent scoped to the exact owning identity.
    pub fn take_focus_request(&mut self) -> Option<FocusTarget> {
        let window = self.window.as_mut()?;
        let window_id = window.id?;
        let control = window.pending_focus.take()?;
        Some(FocusTarget {
            window: window_id,
            control: Some(control),
        })
    }

    /// The window's screen rectangle while it is open.
    #[must_use]
    pub fn screen_rect(&self, layout: CockpitLayout) -> Option<egui::Rect> {
        self.window.as_ref().map(|_| window_rect(layout))
    }

    /// Open the Finder on the player's side tab, SpecForces view
    /// (FUN_004637a0). An open Finder stays as it is.
    pub fn open(&mut self, faction: CockpitFaction) -> bool {
        if self.window.is_some() {
            return false;
        }
        let tab = match faction {
            CockpitFaction::Alliance => PersonnelTab::Alliance,
            CockpitFaction::Empire => PersonnelTab::Imperial,
        };
        self.window = Some(OpenFinder {
            id: None,
            faction,
            tab,
            view: PersonnelView::SpecForces,
            name: String::new(),
            chosen: None,
            first_row: 0,
            focus_name: true,
            pending_focus: None,
            last_clicked: None,
        });
        true
    }

    pub fn close(&mut self) {
        self.window = None;
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.window.is_some()
    }

    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.window.is_some() && rect_contains(window_rect(layout), egui::pos2(point.0, point.1))
    }

    pub fn set_tab(&mut self, tab: PersonnelTab) {
        if let Some(window) = &mut self.window {
            if window.tab != tab {
                window.tab = tab;
                window.chosen = None;
                window.first_row = 0;
                window.last_clicked = None;
            }
        }
    }

    pub fn set_view(&mut self, view: PersonnelView) {
        if let Some(window) = &mut self.window {
            if window.view != view {
                window.view = view;
                window.chosen = None;
                window.first_row = 0;
                window.last_clicked = None;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PersonnelFinderDrawOutcome {
    Close,
    Open(PersonnelFinderAction),
}

/// Draw a fixture-opened Personnel Finder, if any, and report an action.
pub fn draw_personnel_finder(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    state: &mut PersonnelFinderState,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<PersonnelFinderAction> {
    match draw_personnel_finder_outcome(ctx, world, fog, state, None, layout, cache) {
        Some(PersonnelFinderDrawOutcome::Open(action)) => {
            state.close();
            Some(action)
        }
        Some(PersonnelFinderDrawOutcome::Close) => {
            state.close();
            None
        }
        None => None,
    }
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
    match draw_personnel_finder_outcome(ctx, world, fog, state, focus, layout, cache) {
        Some(PersonnelFinderDrawOutcome::Open(destination)) => {
            Some(ManagedPersonnelFinderAction::Open {
                finder,
                destination,
            })
        }
        Some(PersonnelFinderDrawOutcome::Close) => {
            Some(ManagedPersonnelFinderAction::Close(finder))
        }
        None => None,
    }
}

#[expect(
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
    let window = state.window.clone()?;
    let focused_control = window
        .id
        .and_then(|id| focus.filter(|target| target.window == id))
        .and_then(|target| target.control);
    let side_idx = side(window.faction);
    let controls = &CONTROLS[side_idx];
    let scale = layout.scale;
    let rect = window_rect(layout);
    let list = rows(
        world,
        fog,
        player_side(window.faction),
        window.tab,
        window.view,
    );

    let mut close = false;
    let mut open_action: Option<PersonnelFinderAction> = None;
    let mut new_side_tab = None;
    let mut new_view_tab = None;
    let mut clicked_row = None;
    let mut name = window.name.clone();
    let mut name_changed = false;
    let mut enter = false;
    let mut wheel = 0.0;
    let mut focus_request = None;

    let id = egui::Id::new("original-personnel-finder");
    let name_id = egui::Id::new("original-personnel-finder-name");
    let list_id = egui::Id::new("original-personnel-finder-list");
    let name_should_focus =
        window.focus_name || focused_control == Some(PERSONNEL_FINDER_NAME_CONTROL);
    let name_had_focus = ctx.memory(|memory| memory.has_focus(name_id));
    let deferred_name_input = if name_should_focus && !name_had_focus {
        ctx.input(|input| {
            input
                .events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::Text(text) if text != "\n" && text != "\r" => Some(text.as_str()),
                    egui::Event::Paste(text) => Some(text.as_str()),
                    _ => None,
                })
                .collect::<String>()
        })
    } else {
        String::new()
    };
    if focused_control == Some(PERSONNEL_FINDER_LIST_CONTROL) {
        ctx.memory_mut(|memory| memory.request_focus(list_id));
    } else if name_should_focus {
        // Establish the manager-owned child focus before the surface begins
        // its pass so the widget can consume this frame's first text event.
        ctx.memory_mut(|memory| memory.request_focus(name_id));
    }
    egui::Area::new(id)
        .fixed_pos(rect.min)
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let (frame, _) = ui.allocate_exact_size(rect.size(), egui::Sense::click());
            let painter = ui.painter().with_clip_rect(frame);
            let paint = |cache: &mut BmpCache, id: u32, (x, y): (f32, f32)| {
                paint_native(
                    &painter,
                    ctx,
                    cache,
                    DllSource::Strategy,
                    id,
                    frame,
                    scale,
                    x,
                    y,
                );
            };

            painter.rect_filled(frame, 0.0, egui::Color32::BLACK);
            paint(cache, BACKGROUND[side_idx], (0.0, 0.0));
            paint(
                cache,
                PANELS[window.view.index()][window.tab.index()],
                PANEL_AT,
            );
            paint(cache, RAIL[side_idx], RAIL_AT);

            // Title (0x1890) and label (0x1891).
            let title_font = egui::FontId::proportional((12.0 * scale).max(8.0));
            let font = egui::FontId::proportional((10.0 * scale).max(7.0));
            painter.text(
                at(frame, scale, (TITLE_AT.0, TITLE_AT.1, 0.0, 0.0)).min,
                egui::Align2::LEFT_TOP,
                "Personnel Finder",
                title_font.clone(),
                egui::Color32::WHITE,
            );
            painter.text(
                at(frame, scale, (NAME_LABEL_AT.0, NAME_LABEL_AT.1, 0.0, 0.0)).min,
                egui::Align2::LEFT_TOP,
                "Character Name",
                font.clone(),
                egui::Color32::WHITE,
            );
            let label_y = window.view.label_y();
            painter.text(
                at(frame, scale, (40.0, label_y, 0.0, 0.0)).min,
                egui::Align2::LEFT_TOP,
                window.tab.label(),
                title_font,
                egui::Color32::WHITE,
            );

            // Side tabs.
            for candidate in PersonnelTab::ALL {
                let control = SIDE_TABS[candidate.index()];
                let down = candidate == window.tab;
                if button(
                    ui,
                    cache,
                    at(frame, scale, control.rect),
                    ("personnel-side-tab", candidate.index()),
                    control.art,
                    down,
                    scale,
                ) {
                    new_side_tab = Some(candidate);
                }
            }

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
                if button(
                    ui,
                    cache,
                    at(frame, scale, view_rect),
                    ("personnel-view-tab", candidate.index()),
                    art,
                    down,
                    scale,
                ) {
                    new_view_tab = Some(candidate);
                }
            }

            // Close and Display.
            if button(
                ui,
                cache,
                at(frame, scale, controls.close.rect),
                "personnel-close",
                controls.close.art,
                false,
                scale,
            ) {
                close = true;
            }
            if button(
                ui,
                cache,
                at(frame, scale, controls.display.rect),
                "personnel-display",
                controls.display.art,
                false,
                scale,
            ) {
                if let Some(chosen) = window.chosen {
                    if let Some(row) = list.get(chosen) {
                        open_action = Some(action_for_row(row));
                    }
                }
            }

            // Name box (0xcb).
            let name_rect = at(frame, scale, NAME_BOX);
            let edit = ui.put(
                name_rect,
                egui::TextEdit::singleline(&mut name)
                    .id(name_id)
                    .frame(false)
                    .font(font.clone())
                    .text_color(egui::Color32::WHITE)
                    .desired_width(name_rect.width()),
            );
            if window.id.is_some() && edit.gained_focus() {
                focus_request = Some(PERSONNEL_FINDER_NAME_CONTROL);
            }
            name_changed = edit.changed();
            if !name_changed && !deferred_name_input.is_empty() {
                // `Area` disables widgets during its first sizing pass. Keep
                // the manager-owned edit's first text/paste event instead of
                // dropping it while egui establishes the widget state.
                name.push_str(&deferred_name_input);
                name_changed = true;
            }
            enter = edit.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));

            // The list (0xca).
            let list_rect = at(frame, scale, LIST);
            let list_painter = painter.with_clip_rect(list_rect);
            let cell_font = egui::FontId::proportional((10.0 * scale).max(7.0));

            for (offset, row) in list
                .iter()
                .enumerate()
                .skip(window.first_row)
                .take(VISIBLE_ROWS)
            {
                let line = (offset - window.first_row) as f32;
                let row_rect = at(
                    frame,
                    scale,
                    (LIST.0, LIST.1 + ROW_HEIGHT * line, LIST.2, ROW_HEIGHT),
                );
                let name_colour = if window.chosen == Some(offset) {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::WHITE
                };

                match row {
                    PersonnelFinderRow::Character { name, .. } => {
                        list_painter.text(
                            row_rect.left_center() + egui::vec2(4.0 * scale, 0.0),
                            egui::Align2::LEFT_CENTER,
                            name,
                            font.clone(),
                            name_colour,
                        );
                    }
                    PersonnelFinderRow::SpecForces { name, counts, .. } => {
                        list_painter.text(
                            row_rect.left_center() + egui::vec2(4.0 * scale, 0.0),
                            egui::Align2::LEFT_CENTER,
                            name,
                            font.clone(),
                            name_colour,
                        );
                        // Four columns at x = 222 + 28*i (FUN_00465540).
                        let (box_colour, text_on_box) = sf_cell_colours(window.tab);
                        for (i, cell) in counts.iter().enumerate() {
                            let cell_x = 222.0 + 28.0 * i as f32;
                            if cell.count == 0 {
                                let cell_pos = at(
                                    frame,
                                    scale,
                                    (LIST.0 + cell_x, LIST.1 + ROW_HEIGHT * line + 2.0, 28.0, 0.0),
                                )
                                .min;
                                list_painter.text(
                                    cell_pos + egui::vec2(10.0 * scale, 0.0),
                                    egui::Align2::CENTER_TOP,
                                    "-",
                                    cell_font.clone(),
                                    egui::Color32::WHITE,
                                );
                            } else {
                                let box_rect = at(
                                    frame,
                                    scale,
                                    (
                                        LIST.0 + cell_x,
                                        LIST.1 + ROW_HEIGHT * line + 2.0,
                                        21.0,
                                        16.0,
                                    ),
                                );
                                list_painter.rect_filled(box_rect, 0.0, box_colour);
                                list_painter.text(
                                    box_rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    cell.count.to_string(),
                                    cell_font.clone(),
                                    text_on_box,
                                );
                            }
                        }
                    }
                }

                let response = ui.interact(
                    row_rect,
                    ui.id().with(("personnel-row", offset)),
                    egui::Sense::click(),
                );
                if response.clicked() {
                    if window.id.is_some() {
                        focus_request = Some(PERSONNEL_FINDER_LIST_CONTROL);
                    }
                    let (now, delay) = (
                        ui.input(|input| input.time),
                        ctx.options(|options| options.input_options.max_double_click_delay),
                    );
                    if window
                        .last_clicked
                        .is_some_and(|(last, at_time)| last == offset && now - at_time <= delay)
                    {
                        open_action = Some(action_for_row(row));
                    } else {
                        clicked_row = Some((offset, now));
                    }
                }
            }
            let pointer = ui.input(|input| input.pointer.hover_pos());
            if pointer.is_some_and(|point| rect_contains(list_rect, point)) {
                wheel = ui.input(|input| input.raw_scroll_delta.y);
            }
        });

    let escape = ctx.input(|input| input.key_pressed(egui::Key::Escape));
    let arrow_up = ctx.input(|input| input.key_pressed(egui::Key::ArrowUp));
    let arrow_down = ctx.input(|input| input.key_pressed(egui::Key::ArrowDown));
    let arrow_left = ctx.input(|input| input.key_pressed(egui::Key::ArrowLeft));
    let arrow_right = ctx.input(|input| input.key_pressed(egui::Key::ArrowRight));
    let managed_enter = ctx.input(|input| input.key_pressed(egui::Key::Enter));
    let open_window = state.window.as_mut()?;
    open_window.focus_name = false;
    if name_changed {
        open_window.name.clone_from(&name);
        let choice = prefix_choice(&list, &name);
        open_window.chosen = choice;
        if let Some(index) = choice {
            open_window.first_row = scrolled_to(open_window.first_row, index);
        }
    }
    if let Some((row, at_time)) = clicked_row {
        open_window.last_clicked = Some((row, at_time));
        open_window.chosen = Some(row);
        // Copy the display name into the name box.
        open_window.name = list[row].display_name().to_owned();
    }
    if focused_control == Some(PERSONNEL_FINDER_NAME_CONTROL) && arrow_down && !list.is_empty() {
        let row = open_window.chosen.unwrap_or(0).min(list.len() - 1);
        open_window.chosen = Some(row);
        open_window.name = list[row].display_name().to_owned();
        open_window.first_row = scrolled_to(open_window.first_row, row);
        focus_request = Some(PERSONNEL_FINDER_LIST_CONTROL);
    }
    if focused_control == Some(PERSONNEL_FINDER_LIST_CONTROL) {
        if list.is_empty() {
            if arrow_up {
                focus_request = Some(PERSONNEL_FINDER_NAME_CONTROL);
            }
        } else {
            let current = open_window.chosen.unwrap_or(0).min(list.len() - 1);
            let next = if arrow_down {
                (current + 1).min(list.len() - 1)
            } else if arrow_up && current > 0 {
                current - 1
            } else {
                current
            };
            if arrow_up && current == 0 {
                focus_request = Some(PERSONNEL_FINDER_NAME_CONTROL);
            } else if arrow_up || arrow_down {
                open_window.chosen = Some(next);
                open_window.name = list[next].display_name().to_owned();
                open_window.first_row = scrolled_to(open_window.first_row, next);
            }
            if managed_enter {
                open_action = open_action.or_else(|| {
                    open_window
                        .chosen
                        .and_then(|chosen| list.get(chosen))
                        .map(action_for_row)
                });
            }
        }
        if arrow_left || arrow_right {
            let current = open_window.tab.index();
            let next = if arrow_left {
                current.saturating_sub(1)
            } else {
                (current + 1).min(PersonnelTab::ALL.len() - 1)
            };
            new_side_tab = Some(PersonnelTab::ALL[next]);
        }
    }
    if let Some(control) = focus_request {
        open_window.pending_focus = Some(control);
    }
    if enter {
        if let Some(chosen) = open_window.chosen {
            if let Some(row) = list.get(chosen) {
                open_action = Some(action_for_row(row));
            }
        }
    }
    if wheel != 0.0 {
        let last = list.len().saturating_sub(VISIBLE_ROWS);
        open_window.first_row = if wheel > 0.0 {
            open_window.first_row.saturating_sub(1)
        } else {
            (open_window.first_row + 1).min(last)
        };
    }
    if let Some(tab) = new_side_tab {
        state.set_tab(tab);
    }
    if let Some(view) = new_view_tab {
        state.set_view(view);
    }
    if let Some(action) = open_action {
        return Some(PersonnelFinderDrawOutcome::Open(action));
    }
    if close || escape {
        return Some(PersonnelFinderDrawOutcome::Close);
    }
    None
}

fn action_for_row(row: &PersonnelFinderRow) -> PersonnelFinderAction {
    match row {
        PersonnelFinderRow::Character { key, system, .. } => PersonnelFinderAction::OpenCharacter {
            character: *key,
            system: *system,
        },
        PersonnelFinderRow::SpecForces { system, fleet, .. } => {
            if let Some(fleet) = fleet {
                PersonnelFinderAction::OpenFleet {
                    system: *system,
                    fleet: *fleet,
                }
            } else {
                PersonnelFinderAction::OpenSystem { system: *system }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::cockpit::CockpitViewport;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{
        Character, ControlKind, ForceTier, Sector, SkillPair, SpecialForceUnit, System,
    };
    use rebellion_windowing::{
        ExistingEffect, FocusTarget, RouteTarget, WindowCommand, WindowManager, WindowTransition,
    };

    fn layout() -> CockpitLayout {
        CockpitLayout {
            canvas: CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: 640.0,
                height: 480.0,
            },
            galaxy: CockpitViewport {
                x: 65.0,
                y: 60.0,
                width: 485.0,
                height: 350.0,
            },
            scale: 1.0,
        }
    }

    fn key_state(key: egui::Key, pressed: bool) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }
    }

    fn tap(key: egui::Key) -> Vec<Vec<egui::Event>> {
        vec![vec![key_state(key, true)], vec![key_state(key, false)]]
    }

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
        let mut emitted = None;
        for events in frames {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                if let Some(action) = draw_personnel_finder_focused(
                    ctx,
                    world,
                    fog,
                    state,
                    Some(focus),
                    layout,
                    &mut cache,
                ) {
                    emitted = Some(action);
                }
            });
        }
        emitted
    }

    #[test]
    fn personnel_finder_duplicate_preserves_content() {
        let mut manager = WindowManager::new();
        let mut state = PersonnelFinderState::default();
        let prepared = PersonnelFinderState::prepare_open(CockpitFaction::Alliance);
        let opened = manager.apply(WindowCommand::Open(prepared.spec())).unwrap();
        let id = match opened.transition {
            WindowTransition::Opened { id } => id,
            ref other => panic!("expected Personnel Finder open, got {other:?}"),
        };
        assert_eq!(state.commit_open(prepared, &opened.transition), Some(id));

        let content = state.window.as_mut().unwrap();
        content.tab = PersonnelTab::Imperial;
        content.view = PersonnelView::Characters;
        content.name = "Leia".into();
        content.chosen = Some(3);
        content.first_row = 2;
        content.focus_name = false;
        let focused = FocusTarget {
            window: id,
            control: Some(PERSONNEL_FINDER_LIST_CONTROL),
        };
        manager.apply(WindowCommand::Focus(focused)).unwrap();

        let duplicate = PersonnelFinderState::prepare_open(CockpitFaction::Empire);
        let duplicate_commit = manager
            .apply(WindowCommand::Open(duplicate.spec()))
            .unwrap();
        assert!(matches!(
            duplicate_commit.transition,
            WindowTransition::Existing {
                id: existing,
                effect: ExistingEffect::NoOp,
            } if existing == id
        ));
        assert_eq!(
            state.commit_open(duplicate, &duplicate_commit.transition),
            Some(id)
        );

        let content = state.window.as_ref().unwrap();
        assert_eq!(state.window_id(), Some(id));
        assert_eq!(content.faction, CockpitFaction::Alliance);
        assert_eq!(content.tab, PersonnelTab::Imperial);
        assert_eq!(content.view, PersonnelView::Characters);
        assert_eq!(content.name, "Leia");
        assert_eq!(content.chosen, Some(3));
        assert_eq!(content.first_row, 2);
        assert!(!content.focus_name);
        assert_eq!(manager.begin_frame().focused(), Some(focused));
    }

    #[test]
    fn personnel_finder_rejects_stale_publication_receipts_without_retargeting() {
        let mut manager = WindowManager::new();
        let mut state = PersonnelFinderState::default();
        let prepared = PersonnelFinderState::prepare_open(CockpitFaction::Alliance);
        let opened = manager.apply(WindowCommand::Open(prepared.spec())).unwrap();
        let id = match opened.transition {
            WindowTransition::Opened { id } => id,
            ref other => panic!("expected Personnel Finder open, got {other:?}"),
        };
        assert_eq!(state.commit_open(prepared, &opened.transition), Some(id));
        let content = state.window.as_mut().unwrap();
        content.tab = PersonnelTab::Imperial;
        content.name = "Leia".into();

        let other = match manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::singleton_spec(
                    WindowKind::TroopFinder,
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap()
            .transition
        {
            WindowTransition::Opened { id } => id,
            ref transition => panic!("expected unrelated open, got {transition:?}"),
        };
        let replacement = PersonnelFinderState::prepare_open(CockpitFaction::Empire);

        assert_eq!(
            state.commit_open(replacement, &WindowTransition::Opened { id: other }),
            None
        );
        assert_eq!(
            state.commit_open(
                replacement,
                &WindowTransition::Existing {
                    id: other,
                    effect: ExistingEffect::NoOp,
                },
            ),
            None
        );
        assert_eq!(state.window_id(), Some(id));
        let content = state.window.as_ref().unwrap();
        assert_eq!(content.faction, CockpitFaction::Alliance);
        assert_eq!(content.tab, PersonnelTab::Imperial);
        assert_eq!(content.name, "Leia");
    }

    #[test]
    fn personnel_finder_modal_blocks_underlay() {
        let mut manager = WindowManager::new();
        let mut state = PersonnelFinderState::default();
        let prepared = PersonnelFinderState::prepare_open(CockpitFaction::Alliance);
        let opened = manager.apply(WindowCommand::Open(prepared.spec())).unwrap();
        let id = match opened.transition {
            WindowTransition::Opened { id } => id,
            ref other => panic!("expected Personnel Finder open, got {other:?}"),
        };
        assert_eq!(state.commit_open(prepared, &opened.transition), Some(id));
        let frame = manager.begin_frame();
        let inside = state.screen_rect(layout()).unwrap().center();
        let hits = state
            .hit_window(&frame, layout(), (inside.x, inside.y))
            .into_iter()
            .collect::<BTreeSet<_>>();

        assert_eq!(
            frame.pointer_route(&hits).target,
            RouteTarget::Window(FocusTarget {
                window: id,
                control: None,
            })
        );
        assert_eq!(
            frame.pointer_route(&BTreeSet::new()).target,
            RouteTarget::BlockedBy(id)
        );
        assert!(!frame.pointer_allows_root());
        assert_eq!(state.keyboard_owner(&frame), Some(id));
        assert_eq!(
            state.hit_window(
                &frame,
                layout(),
                (inside.x + PERSONNEL_FINDER_WIDTH, inside.y),
            ),
            None
        );
        assert_eq!(
            state.content_reports(&frame),
            [rebellion_windowing::ContentReport {
                window: id,
                status: ContentStatus::Ready,
            }]
        );

        manager
            .apply(WindowCommand::Open(
                crate::windowing_policy::singleton_spec(
                    WindowKind::TroopFinder,
                    WindowOwner::StrategicRoot,
                ),
            ))
            .unwrap();
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
        let mut manager = WindowManager::new();
        let mut state = PersonnelFinderState::default();
        let prepared = PersonnelFinderState::prepare_open(CockpitFaction::Alliance);
        let opened = manager.apply(WindowCommand::Open(prepared.spec())).unwrap();
        let id = match opened.transition {
            WindowTransition::Opened { id } => id,
            ref other => panic!("expected Personnel Finder open, got {other:?}"),
        };
        assert_eq!(state.commit_open(prepared, &opened.transition), Some(id));
        let world = GameWorld::default();
        let fog = FogState::new(Faction::Alliance);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(700.0, 520.0),
            )),
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::default(),
            }],
            ..Default::default()
        };
        let mut action = None;
        let _ = ctx.run(input, |ctx| {
            action = draw_personnel_finder_focused(
                ctx,
                &world,
                &fog,
                &mut state,
                manager.begin_frame().focused(),
                layout(),
                &mut cache,
            );
        });

        assert_eq!(action, Some(ManagedPersonnelFinderAction::Close(id)));
        assert!(
            state.is_open(),
            "managed content waits for the close receipt"
        );
        let closed = manager.apply(WindowCommand::Close(id)).unwrap();
        let removed = match closed.transition {
            WindowTransition::Closed { closed, .. } => closed,
            ref other => panic!("expected Personnel Finder close, got {other:?}"),
        };
        state.remove_content(&removed);
        assert!(!state.is_open());
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
        let mut manager = WindowManager::new();
        let mut state = PersonnelFinderState::default();
        let prepared = PersonnelFinderState::prepare_open(CockpitFaction::Alliance);
        let opened = manager.apply(WindowCommand::Open(prepared.spec())).unwrap();
        let id = match opened.transition {
            WindowTransition::Opened { id } => id,
            ref other => panic!("expected Personnel Finder open, got {other:?}"),
        };
        assert_eq!(state.commit_open(prepared, &opened.transition), Some(id));
        let name_focus = FocusTarget {
            window: id,
            control: Some(PERSONNEL_FINDER_NAME_CONTROL),
        };
        let list_focus = FocusTarget {
            window: id,
            control: Some(PERSONNEL_FINDER_LIST_CONTROL),
        };
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |_| {});

        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                name_focus,
                vec![vec![egui::Event::Text("Y".into())]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().name, "Y");
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(1));
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                name_focus,
                vec![vec![key_state(egui::Key::ArrowDown, true)]],
            ),
            None
        );
        assert_eq!(state.take_focus_request(), Some(list_focus));
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(1));

        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                vec![vec![key_state(egui::Key::ArrowDown, true)]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(2));
        assert_eq!(state.window.as_ref().unwrap().name, "Zulu");
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                vec![vec![key_state(egui::Key::ArrowDown, true)]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(2));

        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                vec![vec![key_state(egui::Key::ArrowUp, true)]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(1));
        assert_eq!(state.window.as_ref().unwrap().name, "Yavin");
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                vec![vec![key_state(egui::Key::ArrowUp, true)]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(0));
        assert_eq!(state.window.as_ref().unwrap().name, "Alpha");
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                vec![vec![key_state(egui::Key::ArrowUp, true)]],
            ),
            None
        );
        assert_eq!(state.take_focus_request(), Some(name_focus));
        state.window.as_mut().unwrap().name.clear();
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                name_focus,
                vec![vec![egui::Event::Text("Y".into())]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().name, "Y");

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
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                vec![vec![key_state(egui::Key::ArrowRight, true)]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().tab, PersonnelTab::Imperial);
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                vec![vec![key_state(egui::Key::ArrowRight, true)]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().tab, PersonnelTab::Imperial);
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                vec![vec![key_state(egui::Key::ArrowLeft, true)]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().tab, PersonnelTab::Alliance);

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
        let mut manager = WindowManager::new();
        let mut state = PersonnelFinderState::default();
        let prepared = PersonnelFinderState::prepare_open(CockpitFaction::Alliance);
        let opened = manager.apply(WindowCommand::Open(prepared.spec())).unwrap();
        let id = match opened.transition {
            WindowTransition::Opened { id } => id,
            ref other => panic!("expected Personnel Finder open, got {other:?}"),
        };
        assert_eq!(state.commit_open(prepared, &opened.transition), Some(id));
        let focus = FocusTarget {
            window: id,
            control: Some(PERSONNEL_FINDER_NAME_CONTROL),
        };
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
        assert_eq!(state.window.as_ref().unwrap().name, "Y");
    }

    #[test]
    fn personnel_finder_list_focus_does_not_capture_deferred_name_input() {
        let world = GameWorld::default();
        let fog = FogState::new(Faction::Alliance);
        let mut manager = WindowManager::new();
        let mut state = PersonnelFinderState::default();
        let prepared = PersonnelFinderState::prepare_open(CockpitFaction::Alliance);
        let opened = manager.apply(WindowCommand::Open(prepared.spec())).unwrap();
        let id = match opened.transition {
            WindowTransition::Opened { id } => id,
            ref other => panic!("expected Personnel Finder open, got {other:?}"),
        };
        assert_eq!(state.commit_open(prepared, &opened.transition), Some(id));
        state.window.as_mut().unwrap().focus_name = false;
        let focus = FocusTarget {
            window: id,
            control: Some(PERSONNEL_FINDER_LIST_CONTROL),
        };
        let ctx = egui::Context::default();

        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                focus,
                vec![vec![egui::Event::Text("Y".into())]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().name, "");
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
        let mut manager = WindowManager::new();
        let mut state = PersonnelFinderState::default();
        let prepared = PersonnelFinderState::prepare_open(CockpitFaction::Alliance);
        let opened = manager.apply(WindowCommand::Open(prepared.spec())).unwrap();
        let id = match opened.transition {
            WindowTransition::Opened { id } => id,
            ref other => panic!("expected Personnel Finder open, got {other:?}"),
        };
        assert_eq!(state.commit_open(prepared, &opened.transition), Some(id));
        let name_focus = FocusTarget {
            window: id,
            control: Some(PERSONNEL_FINDER_NAME_CONTROL),
        };
        let list_focus = FocusTarget {
            window: id,
            control: Some(PERSONNEL_FINDER_LIST_CONTROL),
        };
        let ctx = egui::Context::default();
        state.window.as_mut().unwrap().chosen = Some(usize::MAX);

        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                name_focus,
                vec![vec![key_state(egui::Key::ArrowDown, true)]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(2));
        assert_eq!(state.window.as_ref().unwrap().name, "Zulu");

        state.window.as_mut().unwrap().chosen = Some(usize::MAX);
        assert_eq!(
            drive_focused_in_context(
                &ctx,
                &world,
                &fog,
                &mut state,
                list_focus,
                vec![vec![key_state(egui::Key::ArrowUp, true)]],
            ),
            None
        );
        assert_eq!(state.window.as_ref().unwrap().chosen, Some(1));
        assert_eq!(state.window.as_ref().unwrap().name, "Yavin");
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
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(screen_rect),
                ..Default::default()
            },
            |ctx| {
                let _ = draw_personnel_finder(ctx, &world, &fog, &mut state, layout, &mut cache);
            },
        );
        state.window.as_mut().unwrap().focus_name = true;
        ctx.memory_mut(|memory| {
            memory.surrender_focus(egui::Id::new("original-personnel-finder-name"));
        });
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(screen_rect),
                ..Default::default()
            },
            |ctx| {
                let _ = draw_personnel_finder(ctx, &world, &fog, &mut state, layout, &mut cache);
            },
        );

        assert_eq!(state.window.as_ref().unwrap().pending_focus, None);
    }

    fn add_system(world: &mut GameWorld, name: &str) -> SystemKey {
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(36),
            name: "Sesswenna".into(),
            group: SectorGroup::Core,
            x: 317,
            y: 248,
            systems: Vec::new(),
        });
        world.systems.insert(System {
            dat_id: DatId::new(100),
            name: name.into(),
            sector,
            x: 320,
            y: 250,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            fighters: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        })
    }

    fn default_character() -> Character {
        Character {
            dat_id: DatId::new(0x3000_0001),
            name: String::new(),
            is_alliance: true,
            is_empire: false,
            is_major: true,
            diplomacy: SkillPair {
                base: 50,
                variance: 0,
            },
            espionage: SkillPair {
                base: 50,
                variance: 0,
            },
            ship_design: SkillPair {
                base: 50,
                variance: 0,
            },
            troop_training: SkillPair {
                base: 50,
                variance: 0,
            },
            facility_design: SkillPair {
                base: 50,
                variance: 0,
            },
            combat: SkillPair {
                base: 50,
                variance: 0,
            },
            leadership: SkillPair {
                base: 50,
                variance: 0,
            },
            loyalty: SkillPair {
                base: 50,
                variance: 0,
            },
            jedi_probability: 0,
            jedi_level: SkillPair {
                base: 0,
                variance: 0,
            },
            can_be_admiral: false,
            can_be_commander: false,
            can_be_general: false,
            force_tier: ForceTier::None,
            force_experience: 0,
            is_discovered_jedi: false,
            is_unable_to_betray: false,
            is_jedi_trainer: false,
            is_known_jedi: false,
            hyperdrive_modifier: 0,
            enhanced_loyalty: 0,
            on_mission: false,
            on_hidden_mission: false,
            on_mandatory_mission: false,
            captured_by: None,
            capture_tick: None,
            is_captive: false,
            current_system: None,
            current_fleet: None,
            en_route: false,
            heritage_known: false,
            is_killed: false,
            recruited: true,
            command_kind: rebellion_core::officer_rank::CommandKind::None,
            command_post: None,
            ranked_names: Default::default(),
        }
    }

    fn add_sf(
        world: &mut GameWorld,
        system: SystemKey,
        class_raw: u32,
        is_alliance: bool,
    ) -> SpecialForceKey {
        let key = world.special_forces.insert(SpecialForceUnit {
            class_dat_id: DatId::new(class_raw),
            is_alliance,
            skills: [0; 8],
            on_mission: false,
            en_route: false,
            on_hidden_mission: false,
        });
        world.systems[system].special_forces.push(key);
        key
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
        // FUN_00464e10 view==2: walk characters of the side, call
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
        let mut world = GameWorld::default();
        let yavin = add_system(&mut world, "Yavin");

        let mut luke = default_character();
        luke.name = "Luke Skywalker".into();
        luke.current_system = Some(yavin);
        luke.on_mission = true;
        world.characters.insert(luke);

        let fog = FogState::new(Faction::Alliance);
        let listed = character_rows(&world, &fog, Faction::Alliance, PersonnelTab::Alliance);
        assert!(listed[0].display_name().contains("On Mission"));
    }

    #[test]
    fn a_killed_character_is_listed_as_killed() {
        // FUN_00465bb0's destroyed branch; the reference capture lists
        // "Ackbar - Killed" (steam-guide guide-023.png).
        let mut world = GameWorld::default();
        let yavin = add_system(&mut world, "Yavin");

        let mut luke = default_character();
        luke.name = "Luke Skywalker".into();
        luke.current_system = Some(yavin);
        luke.is_killed = true;
        world.characters.insert(luke);

        let fog = FogState::new(Faction::Alliance);
        let listed = character_rows(&world, &fog, Faction::Alliance, PersonnelTab::Alliance);
        assert_eq!(listed[0].display_name(), "Luke Skywalker - Killed");
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
    fn rows_are_sorted_by_name_ignoring_case() {
        // FUN_005f59f0: rows inserted in name order.
        let mut world = GameWorld::default();
        let beta = add_system(&mut world, "beta");
        let alpha = add_system(&mut world, "Alpha");
        add_sf(&mut world, beta, 0x3c00_0004, true);
        add_sf(&mut world, alpha, 0x3c00_0004, true);
        let fog = FogState::new(Faction::Alliance);

        let listed = specforces_rows(&world, &fog, Faction::Alliance, PersonnelTab::Alliance);
        let names: Vec<&str> = listed.iter().map(|r| r.display_name()).collect();
        assert_eq!(names, ["Alpha", "beta"]);
    }

    #[test]
    fn a_typed_name_chooses_the_first_row_it_best_begins() {
        // FUN_00609650: prefix match across both row kinds.
        let rows = vec![
            PersonnelFinderRow::SpecForces {
                name: "Coruscant".into(),
                system: SystemKey::default(),
                fleet: None,
                counts: [SfCount::default(); 4],
            },
            PersonnelFinderRow::SpecForces {
                name: "Yavin".into(),
                system: SystemKey::default(),
                fleet: None,
                counts: [SfCount::default(); 4],
            },
        ];
        assert_eq!(prefix_choice(&rows, "ya"), Some(1));
        assert_eq!(prefix_choice(&rows, "cor"), Some(0));
        assert_eq!(prefix_choice(&rows, "z"), None);
        assert_eq!(prefix_choice(&rows, ""), None);
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
    fn the_list_scrolls_as_little_as_it_must_to_show_a_row() {
        assert_eq!(scrolled_to(0, 7), 0);
        assert_eq!(scrolled_to(0, 8), 1);
        assert_eq!(scrolled_to(3, 1), 1);
        assert_eq!(scrolled_to(3, 10), 3);
        assert_eq!(scrolled_to(2, 10), 3);
    }

    #[test]
    fn the_longest_common_beginning_wins_and_a_tie_keeps_the_first() {
        // FUN_00609650 keeps a row only for a strictly longer match.
        let rows: Vec<PersonnelFinderRow> = ["Corulag", "Coruscant", "Yavin"]
            .into_iter()
            .map(|name| PersonnelFinderRow::SpecForces {
                name: name.into(),
                system: SystemKey::default(),
                fleet: None,
                counts: [SfCount::default(); 4],
            })
            .collect();
        assert_eq!(prefix_choice(&rows, "corus"), Some(1));
        assert_eq!(prefix_choice(&rows, "cor"), Some(0));
    }

    #[test]
    fn each_side_tab_names_its_side_and_paints_its_own_cells() {
        // TEXTSTRA 0x1893/0x1894; FUN_00465540's brushes as the Troop
        // Finder's: red with white text, green with black.
        assert_eq!(PersonnelTab::Alliance.label(), "Alliance Personnel");
        assert_eq!(PersonnelTab::Imperial.label(), "Imperial Personnel");
        assert_eq!(
            sf_cell_colours(PersonnelTab::Alliance),
            (egui::Color32::from_rgb(255, 0, 0), egui::Color32::WHITE)
        );
        assert_eq!(
            sf_cell_colours(PersonnelTab::Imperial),
            (egui::Color32::from_rgb(0, 255, 0), egui::Color32::BLACK)
        );
    }

    #[test]
    fn opening_an_open_finder_keeps_its_state() {
        // FUN_0042a4d0 builds the window only when no type 0x17 is open.
        let mut state = PersonnelFinderState::default();
        assert!(state.open(CockpitFaction::Alliance));
        state.set_tab(PersonnelTab::Imperial);

        assert!(!state.open(CockpitFaction::Alliance));
        assert_eq!(state.window.as_ref().unwrap().tab, PersonnelTab::Imperial);
        assert!(state.is_open());
        state.close();
        assert!(!state.is_open());
    }

    #[test]
    fn a_new_side_tab_clears_the_choice() {
        // FUN_00464e10: tab switch rebuilds the list.
        let mut state = PersonnelFinderState::default();
        state.open(CockpitFaction::Alliance);
        state.window.as_mut().unwrap().chosen = Some(0);

        state.set_tab(PersonnelTab::Imperial);
        assert_eq!(state.window.as_ref().unwrap().chosen, None);
    }

    #[test]
    fn a_new_view_tab_clears_the_choice() {
        // FUN_00464e10: view switch rebuilds the list.
        let mut state = PersonnelFinderState::default();
        state.open(CockpitFaction::Alliance);
        state.window.as_mut().unwrap().chosen = Some(0);

        state.set_view(PersonnelView::Characters);
        assert_eq!(state.window.as_ref().unwrap().chosen, None);
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

        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(700.0, 520.0),
            )),
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::default(),
            }],
            ..Default::default()
        };
        let _ = ctx.run(input, |ctx| {
            draw_personnel_finder(ctx, &world, &fog, &mut state, layout, &mut cache);
        });

        assert!(!state.is_open());
    }

    #[test]
    fn the_finder_blocks_only_its_own_rectangle() {
        let mut state = PersonnelFinderState::default();
        let rect = window_rect(layout());
        assert!(!state.contains_screen_point(layout(), (rect.center().x, rect.center().y)));

        state.open(CockpitFaction::Alliance);
        assert!(state.contains_screen_point(layout(), (rect.center().x, rect.center().y)));
        assert!(!state.contains_screen_point(layout(), (rect.max.x + 1.0, rect.center().y)));
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
