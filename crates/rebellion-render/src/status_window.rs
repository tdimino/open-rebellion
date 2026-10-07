//! The original Status window (window type `0x1a`, `FUN_00442d70`; create
//! `FUN_00443130`; controls `FUN_00443020`, `FUN_004443a0`). The object
//! menu's Status (`0x103`) opens it (`FUN_0042a440`). Recovery notes:
//! `ghidra/notes/status-window.md`.
//!
//! A 379 by 272 window: a two-column list of labels and values on the left,
//! the object's portrait and name on the right, and the Encyclopedia and
//! Close buttons under them. The port fills it for characters
//! (`FUN_004486f0`, manual p. 101, Fig. 3.46).

use egui_macroquad::egui;
use rebellion_core::ids::CharacterKey;
use rebellion_core::missions::{MemberTransit, MissionMember};
use rebellion_core::world::{Character, GameWorld};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::fleet_window::paint_native;
use crate::mission_dialog::{button, galaxy_centered_rect, paint};
use crate::system_window::{character_mini_resource_id, fleet_label, logical_rect, rect_contains};

pub const STATUS_WINDOW_WIDTH: f32 = 379.0;
pub const STATUS_WINDOW_HEIGHT: f32 = 272.0;

/// The background (STRATEGY `0x2d22..`), by the object's side bits, for a
/// side-1 player; a player of another side takes the bitmap three on.
const BACKGROUND: [u32; 3] = [11_554, 11_555, 11_556];
/// `0x66`, Encyclopedia, and `0x65`, Close (`FUN_00443020`).
const ENCYCLOPEDIA: (u32, u32) = (11_552, 11_553);
const CLOSE: (u32, u32) = (10_370, 10_371);
const ENCYCLOPEDIA_RECT: (f32, f32, f32, f32) = (258.0, 218.0, 32.0, 31.0);
const CLOSE_RECT: (f32, f32, f32, f32) = (324.0, 218.0, 32.0, 31.0);

/// `+0x130`: (15, 18), 211 by 18, font 5, format `0x21`.
const TITLE: (f32, f32, f32, f32) = (15.0, 18.0, 211.0, 18.0);
/// `+0x198`: (242, 137), 130 by 44, font 5, format `0x11`.
const NAME: (f32, f32, f32, f32) = (242.0, 137.0, 130.0, 44.0);
/// `+0x118`: the list, (18, 47), 208 by 204.
const LIST: (f32, f32, f32, f32) = (18.0, 47.0, 208.0, 204.0);
/// `FUN_00449e00`: labels at x 0 and values at x 103, each 103 wide.
const COLUMN: f32 = 103.0;
/// The picture is centred on (307, 64) (`FUN_00443130`).
const PICTURE_CENTRE: (i32, i32) = (307, 64);

/// TEXTSTRA 34595.
const CHARACTER_STATUS: &str = "Character Status";

/// One list row: `FUN_0044b1d0` appends the label to one list and the value
/// to the other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusRow {
    pub label: String,
    pub value: String,
}

fn row(label: &str, value: impl Into<String>) -> StatusRow {
    StatusRow {
        label: label.to_owned(),
        value: value.into(),
    }
}

/// TEXTSTRA 34646 "Yes" or 34645 "No".
const fn yes_no(value: bool) -> &'static str {
    if value {
        "Yes"
    } else {
        "No"
    }
}

/// Force Ranking by the Force value `+0x8c` (`FUN_004486f0`).
#[must_use]
pub const fn force_ranking(value: u32) -> &'static str {
    match value {
        0..=9 => "None",
        10..=19 => "Novice",
        20..=79 => "Trainee",
        80..=99 => "Jedi Student",
        100..=119 => "Jedi Knight",
        _ => "Jedi Master",
    }
}

/// The Character Status rows (`FUN_004486f0`). `en_route` is the mission
/// transits (`MissionState::en_route`).
///
/// port: Commanding (`+0xa8`) is always "None" and "Injured" (`+0x94`)
/// never shows: the port keeps neither. A character is en route while its
/// mission transit lasts; Attached then names the transit's destination,
/// the container of an object in hyperspace (`FUN_00556390`).
#[must_use]
pub fn character_rows(
    world: &GameWorld,
    en_route: &[MemberTransit],
    key: CharacterKey,
) -> Option<Vec<StatusRow>> {
    let character = world.characters.get(key)?;
    let transit = en_route
        .iter()
        .find(|transit| transit.member == MissionMember::Character(key));
    let attached = transit
        .and_then(|transit| world.systems.get(transit.to))
        .map(|system| system.name.clone())
        .or_else(|| {
            character
                .current_fleet
                .and_then(|fleet| fleet_label(world, fleet))
        })
        .or_else(|| {
            character
                .current_system
                .and_then(|system| world.systems.get(system))
                .map(|system| system.name.clone())
        })
        .unwrap_or_default();
    // The first that holds: en route (+0x50 bit 4), captured (+0xac bit 0),
    // injured (+0x94), a mission that is not hidden (FUN_00520b70).
    let status = if transit.is_some() {
        "Enroute"
    } else if character.is_captive {
        "Captured"
    } else if character.on_mission && !character.on_hidden_mission {
        "On Mission"
    } else {
        "Awaiting Orders"
    };
    let mut rows = vec![
        row("Commanding:", "None"),
        // FUN_0044a2e0: the literal, for the shipped TEXTSTRA 1.00.00
        // (FUN_00406850).
        row("Attached: ", attached),
        row("Status:", status),
    ];
    // FUN_004fd2b0: "ETA Destination:" and TEXTSTRA 14356 "Day " plus the
    // arrival day, only while en route.
    if let Some(transit) = transit {
        rows.push(row("ETA Destination:", format!("Day {}", transit.arrival)));
    }
    rows.extend([
        row("Force Ranking:", force_ranking(character.jedi_level.base)),
        row("Diplomacy Rating:", character.diplomacy.base.to_string()),
        row("Espionage Rating:", character.espionage.base.to_string()),
        row("Combat Rating:", character.combat.base.to_string()),
        row("Leadership Rating:", character.leadership.base.to_string()),
        // TEXTSTRA 34608 "R&&D Capabilities", DrawText's escape for "&".
        row("R&D Capabilities", ""),
        row(" Ship Design", yes_no(character.ship_design.base != 0)),
        row(
            " Troop Training",
            yes_no(character.troop_training.base != 0),
        ),
        row(
            " Facility Design",
            yes_no(character.facility_design.base != 0),
        ),
        row("Possible Command Ranks", ""),
        row("Admiral:", yes_no(character.can_be_admiral)),
        row("General:", yes_no(character.can_be_general)),
        row("Commander:", yes_no(character.can_be_commander)),
    ]);
    Some(rows)
}

/// The object's side bits (`+0x24 >> 6 & 3`) as a background index.
const fn character_side(character: &Character) -> usize {
    if character.is_alliance {
        0
    } else if character.is_empire {
        1
    } else {
        2
    }
}

/// The background for an object of side index `side`, seen by `viewer`
/// (`FUN_00443130`); a side past 2 takes the third ("other") bitmap.
#[must_use]
pub fn background(side: usize, viewer: CockpitFaction) -> u32 {
    let base = BACKGROUND.get(side).copied().unwrap_or(BACKGROUND[2]);
    match viewer {
        CockpitFaction::Alliance => base,
        CockpitFaction::Empire => base + 3,
    }
}

/// A character's GOKRES portrait, `class & 0xfff`: its mini less `0x4000`
/// (`FUN_0042c3b0(.., 1, 1)`).
fn character_portrait(character: &Character) -> Option<u32> {
    character_mini_resource_id(character.dat_id, character.is_major).map(|mini| mini - 0x4000)
}

/// What the player chose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusWindowAction {
    /// `0x66`: close, then open the Encyclopedia (`FUN_0041d6b0`). port: on
    /// its index, as the object menu's Encyclopedia opens it.
    Encyclopedia,
    /// `0x65`, or Escape: close.
    Close,
}

#[derive(Debug, Clone, PartialEq)]
struct OpenStatusWindow {
    character: CharacterKey,
    /// The list's scroll offset, in window pixels.
    scroll: f32,
}

/// Whether a Status window is open. There is one at a time: the galaxy view
/// builds it from its `0x468` handler, as the move confirmation.
#[derive(Debug, Clone, Default)]
pub struct StatusWindowState {
    window: Option<OpenStatusWindow>,
}

impl StatusWindowState {
    /// `FUN_0042a440` → the `0x468` handler: open on a character the player
    /// knows (`FUN_004f2d10`).
    pub fn open_character(&mut self, character: CharacterKey) {
        self.window = Some(OpenStatusWindow {
            character,
            scroll: 0.0,
        });
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.window.is_some()
    }

    /// Whether `point` falls on the open window, so the galaxy map under it
    /// takes no input.
    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        self.window.is_some() && rect_contains(window_rect(layout), egui::pos2(point.0, point.1))
    }
}

/// The window's screen rectangle (`FUN_00606980` places it from the galaxy
/// view's rectangle). hyp: centred there, as the move confirmation is.
#[must_use]
pub fn window_rect(layout: CockpitLayout) -> egui::Rect {
    galaxy_centered_rect(layout, STATUS_WINDOW_WIDTH, STATUS_WINDOW_HEIGHT)
}

/// `FUN_00449e00`'s row placement from each row's (label, value) height: a
/// row is as tall as its taller cell, and a shorter value moves down to the
/// label's bottom. Returns each row's (label, value) top and the total.
fn row_tops(heights: &[(f32, f32)]) -> (Vec<(f32, f32)>, f32) {
    let mut tops = Vec::with_capacity(heights.len());
    let mut y = 0.0;
    for &(label, value) in heights {
        tops.push((y, y + (label - value).max(0.0)));
        y += label.max(value);
    }
    (tops, y)
}

/// The picture's top-left in the window: centred on (307, 64) with integer
/// halves (`FUN_00443130`).
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "Bitmap sizes are small integers."
)]
fn picture_origin(width: usize, height: usize) -> (f32, f32) {
    let (cx, cy) = PICTURE_CENTRE;
    (
        (cx - width as i32 / 2) as f32,
        (cy - height as i32 / 2) as f32,
    )
}

/// The list's scroll after a wheel `delta` (screen pixels), kept within the
/// rows' `height` (screen pixels) less the list's own.
fn scrolled(scroll: f32, delta: f32, scale: f32, height: f32) -> f32 {
    (scroll - delta / scale).clamp(0.0, (height / scale - LIST.3).max(0.0))
}

/// One laid-out row: its label and value galleys and their tops.
type LaidRow = (
    std::sync::Arc<egui::Galley>,
    std::sync::Arc<egui::Galley>,
    f32,
    f32,
);

/// Lay out the rows' galleys and place them with [`row_tops`].
fn layout_rows(
    painter: &egui::Painter,
    rows: &[StatusRow],
    font: &egui::FontId,
    column: f32,
) -> (Vec<LaidRow>, f32) {
    let galleys: Vec<_> = rows
        .iter()
        .map(|row| {
            let layout = |text: &str| {
                painter.layout(text.to_owned(), font.clone(), egui::Color32::WHITE, column)
            };
            (layout(&row.label), layout(&row.value))
        })
        .collect();
    let heights: Vec<_> = galleys
        .iter()
        .map(|(label, value)| (label.size().y, value.size().y))
        .collect();
    let (tops, height) = row_tops(&heights);
    let laid = galleys
        .into_iter()
        .zip(tops)
        .map(|((label, value), (label_y, value_y))| (label, value, label_y, value_y))
        .collect();
    (laid, height)
}

/// Draw the open window, if any, and report what the player chose.
pub fn draw_status_window(
    ctx: &egui::Context,
    world: &GameWorld,
    en_route: &[MemberTransit],
    state: &mut StatusWindowState,
    viewer: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Option<StatusWindowAction> {
    let open = state.window.as_mut()?;
    let Some(character) = world.characters.get(open.character) else {
        state.window = None;
        return None;
    };
    let rows = character_rows(world, en_route, open.character).unwrap_or_default();
    let scale = layout.scale;
    let rect = window_rect(layout);
    let mut action = None;

    // Above the modeless windows, as the move confirmation is.
    egui::Area::new(egui::Id::new("original-status-window"))
        .fixed_pos(rect.min)
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let (frame, _) = ui.allocate_exact_size(rect.size(), egui::Sense::hover());
            let painter = ui.painter().with_clip_rect(frame);
            let at = |(x, y, w, h): (f32, f32, f32, f32)| logical_rect(frame, scale, x, y, w, h);

            paint(
                &painter,
                ctx,
                cache,
                background(character_side(character), viewer),
                frame,
                scale,
            );

            // hyp: fonts 5 and 10 are not mapped; the Missions window's
            // sizes stand in.
            let title_font = egui::FontId::proportional((11.0 * scale).max(7.0));
            let list_font = egui::FontId::proportional((9.0 * scale).max(6.0));

            let title = at(TITLE);
            painter.text(
                title.center(),
                egui::Align2::CENTER_CENTER,
                CHARACTER_STATUS,
                title_font.clone(),
                egui::Color32::WHITE,
            );

            // The picture, keyed (FUN_005fd0f0), centred with integer
            // offsets on (307, 64).
            if let Some(portrait) = character_portrait(character) {
                if let Some([width, height]) =
                    cache.original_resource_size(DllSource::Gokres, portrait)
                {
                    let (x, y) = picture_origin(width, height);
                    paint_native(
                        &painter,
                        ctx,
                        cache,
                        DllSource::Gokres,
                        portrait,
                        frame,
                        scale,
                        x,
                        y,
                    );
                }
            }

            let name = at(NAME);
            let galley = painter.layout(
                character.name.clone(),
                title_font,
                egui::Color32::WHITE,
                name.width(),
            );
            let name_at = egui::pos2(
                name.center().x - galley.size().x / 2.0,
                name.center().y - galley.size().y / 2.0,
            );
            painter
                .with_clip_rect(name)
                .galley(name_at, galley, egui::Color32::WHITE);

            // The list. port: the wheel scrolls it; no scroll bar is drawn.
            let list = at(LIST);
            let (laid, height) = layout_rows(&painter, &rows, &list_font, COLUMN * scale);
            let list_response =
                ui.interact(list, ui.id().with("status-list"), egui::Sense::hover());
            let wheel = if list_response.hovered() {
                ui.ctx().input(|input| input.smooth_scroll_delta.y)
            } else {
                0.0
            };
            open.scroll = scrolled(open.scroll, wheel, scale, height);
            let list_painter = painter.with_clip_rect(list);
            let top = list.min.y - open.scroll * scale;
            for (label, value, label_y, value_y) in laid {
                list_painter.galley(
                    egui::pos2(list.min.x, top + label_y),
                    label,
                    egui::Color32::WHITE,
                );
                list_painter.galley(
                    egui::pos2(list.min.x + COLUMN * scale, top + value_y),
                    value,
                    egui::Color32::WHITE,
                );
            }

            if button(
                ui,
                cache,
                at(ENCYCLOPEDIA_RECT),
                "status-encyclopedia",
                ENCYCLOPEDIA,
                false,
                scale,
            ) {
                action = Some(StatusWindowAction::Encyclopedia);
            }
            if button(
                ui,
                cache,
                at(CLOSE_RECT),
                "status-close",
                CLOSE,
                false,
                scale,
            ) {
                action = Some(StatusWindowAction::Close);
            }
        });

    // port: Escape closes; no key slot is traced.
    if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
        action = Some(StatusWindowAction::Close);
    }
    if action.is_some() {
        state.window = None;
    }
    action
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
    use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
    use rebellion_core::ids::{DatId, SystemKey};
    use rebellion_core::world::{ControlKind, ForceTier, Sector, SkillPair, System};

    fn skill(base: u32) -> SkillPair {
        SkillPair { base, variance: 0 }
    }

    fn luke() -> Character {
        Character {
            // MJCHARSD index 578, TEXTSTRA 10306 (0x2842) "Luke Skywalker".
            dat_id: DatId::new(0x3000_0242),
            name: "Luke Skywalker".into(),
            is_alliance: true,
            is_empire: false,
            is_major: true,
            diplomacy: skill(75),
            espionage: skill(75),
            ship_design: skill(0),
            troop_training: skill(0),
            facility_design: skill(0),
            combat: skill(135),
            leadership: skill(70),
            loyalty: skill(100),
            jedi_probability: 0,
            jedi_level: skill(40),
            can_be_admiral: true,
            can_be_commander: true,
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
            heritage_known: false,
            is_killed: false,
            recruited: true,
        }
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
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        })
    }

    fn world_with_luke() -> (GameWorld, CharacterKey, SystemKey, SystemKey) {
        let mut world = GameWorld::default();
        let yavin = add_system(&mut world, "Yavin");
        let hoth = add_system(&mut world, "Hoth");
        let mut character = luke();
        character.current_system = Some(yavin);
        let key = world.characters.insert(character);
        (world, key, yavin, hoth)
    }

    fn pairs(rows: &[StatusRow]) -> Vec<(&str, &str)> {
        rows.iter()
            .map(|row| (row.label.as_str(), row.value.as_str()))
            .collect()
    }

    #[test]
    fn a_character_at_a_system_lists_the_rows_of_figure_3_46() {
        // FUN_004486f0, in order; manual p. 101, Fig. 3.46 (Luke at Yavin,
        // Awaiting Orders, Trainee, 75/75/135/70, no R&D, Admiral Yes).
        let (world, key, ..) = world_with_luke();
        let rows = character_rows(&world, &[], key).expect("rows");
        assert_eq!(
            pairs(&rows),
            [
                ("Commanding:", "None"),
                ("Attached: ", "Yavin"),
                ("Status:", "Awaiting Orders"),
                ("Force Ranking:", "Trainee"),
                ("Diplomacy Rating:", "75"),
                ("Espionage Rating:", "75"),
                ("Combat Rating:", "135"),
                ("Leadership Rating:", "70"),
                ("R&D Capabilities", ""),
                (" Ship Design", "No"),
                (" Troop Training", "No"),
                (" Facility Design", "No"),
                ("Possible Command Ranks", ""),
                ("Admiral:", "Yes"),
                ("General:", "No"),
                ("Commander:", "Yes"),
            ]
        );
    }

    #[test]
    fn a_skill_above_zero_is_an_r_and_d_capability() {
        // FUN_004486f0: +0x80, +0x82 and +0x84 show Yes when non-zero.
        let (mut world, key, ..) = world_with_luke();
        world.characters[key].troop_training = skill(1);
        let rows = character_rows(&world, &[], key).expect("rows");
        let value = |label: &str| {
            rows.iter()
                .find(|row| row.label == label)
                .map(|row| row.value.clone())
        };
        assert_eq!(value(" Troop Training").as_deref(), Some("Yes"));
        assert_eq!(value(" Ship Design").as_deref(), Some("No"));
    }

    #[test]
    fn a_character_on_its_way_is_enroute_with_its_destination_and_arrival_day() {
        // FUN_004486f0: +0x50 bit 4 first, then FUN_004fd2b0's "ETA
        // Destination:" and "Day " + the arrival; Attached names the
        // container, the destination of an object in hyperspace.
        let (mut world, key, _, hoth) = world_with_luke();
        world.characters[key].current_system = None;
        world.characters[key].on_mission = true;
        let transit = MemberTransit {
            member: MissionMember::Character(key),
            mission_id: 3,
            to: hoth,
            arrival: 42,
        };
        let rows = character_rows(&world, &[transit], key).expect("rows");
        assert_eq!(
            pairs(&rows[..4]),
            [
                ("Commanding:", "None"),
                ("Attached: ", "Hoth"),
                ("Status:", "Enroute"),
                ("ETA Destination:", "Day 42"),
            ]
        );
        assert_eq!(rows[4].label, "Force Ranking:");
    }

    #[test]
    fn the_status_word_takes_the_first_state_that_holds() {
        // FUN_004486f0: captured before a mission; a hidden mission
        // (FUN_00520b70) reads as Awaiting Orders.
        let (mut world, key, ..) = world_with_luke();
        let status = |world: &GameWorld| {
            character_rows(world, &[], key).expect("rows")[2]
                .value
                .clone()
        };
        world.characters[key].on_mission = true;
        assert_eq!(status(&world), "On Mission");
        world.characters[key].on_hidden_mission = true;
        assert_eq!(status(&world), "Awaiting Orders");
        world.characters[key].on_hidden_mission = false;
        world.characters[key].is_captive = true;
        assert_eq!(status(&world), "Captured");
    }

    #[test]
    fn force_ranking_follows_the_force_value_thresholds() {
        // FUN_004486f0: <10, <20, <80, <100, <120, else Jedi Master.
        for (value, word) in [
            (0, "None"),
            (9, "None"),
            (10, "Novice"),
            (19, "Novice"),
            (20, "Trainee"),
            (79, "Trainee"),
            (80, "Jedi Student"),
            (99, "Jedi Student"),
            (100, "Jedi Knight"),
            (119, "Jedi Knight"),
            (120, "Jedi Master"),
        ] {
            assert_eq!(force_ranking(value), word, "{value}");
        }
    }

    #[test]
    fn the_background_follows_the_objects_side_and_the_players() {
        // FUN_00443130: 0x2d22 + side, plus 3 for a player not of side 1.
        assert_eq!(background(0, CockpitFaction::Alliance), 11_554);
        assert_eq!(background(1, CockpitFaction::Alliance), 11_555);
        assert_eq!(background(2, CockpitFaction::Alliance), 11_556);
        assert_eq!(background(0, CockpitFaction::Empire), 11_557);
        assert_eq!(background(1, CockpitFaction::Empire), 11_558);
        assert_eq!(background(2, CockpitFaction::Empire), 11_559);
    }

    #[test]
    fn a_characters_picture_is_its_portrait_its_mini_less_0x4000() {
        // FUN_0042c3b0(.., 1, 1): class +0x30 & 0xfff. Luke's TEXTSTRA id
        // is 0x2842, so his mini is 18498 and his portrait 2114.
        assert_eq!(character_portrait(&luke()), Some(2_114));
    }

    fn layout() -> CockpitLayout {
        CockpitLayout {
            canvas: CockpitViewport {
                x: 10.0,
                y: 20.0,
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

    fn press(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    /// Run two idle frames, then one per entry of `frames`, with the pointer
    /// at window pixel `(x, y)`.
    fn drive(
        world: &GameWorld,
        state: &mut StatusWindowState,
        (x, y): (f32, f32),
        frames: Vec<Vec<egui::Event>>,
    ) -> Option<StatusWindowAction> {
        let layout = layout();
        let pos = window_rect(layout).min + egui::vec2(x, y);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let mut emitted = None;
        for extra in [vec![], vec![]].into_iter().chain(frames) {
            let mut events = vec![egui::Event::PointerMoved(pos)];
            events.extend(extra);
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 520.0),
                )),
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                if let Some(action) = draw_status_window(
                    ctx,
                    world,
                    &[],
                    state,
                    CockpitFaction::Alliance,
                    layout,
                    &mut cache,
                ) {
                    emitted = Some(action);
                }
            });
        }
        emitted
    }

    fn click(
        world: &GameWorld,
        state: &mut StatusWindowState,
        point: (f32, f32),
    ) -> Option<StatusWindowAction> {
        let pos = window_rect(layout()).min + egui::vec2(point.0, point.1);
        drive(
            world,
            state,
            point,
            vec![vec![press(pos, true)], vec![press(pos, false)]],
        )
    }

    #[test]
    fn the_buttons_open_the_encyclopedia_or_close_and_both_close_the_window() {
        // FUN_00443020: 0x66 at (258, 218), 0x65 at (324, 218);
        // FUN_004443a0 closes (+0x30) for both.
        let (world, key, ..) = world_with_luke();
        for (point, expected) in [
            (
                (258.0 + 16.0, 218.0 + 15.0),
                StatusWindowAction::Encyclopedia,
            ),
            ((324.0 + 16.0, 218.0 + 15.0), StatusWindowAction::Close),
        ] {
            let mut state = StatusWindowState::default();
            state.open_character(key);
            assert_eq!(click(&world, &mut state, point), Some(expected));
            assert!(!state.is_open());
        }
    }

    #[test]
    fn a_click_beside_the_buttons_keeps_the_window_open() {
        let (world, key, ..) = world_with_luke();
        for point in [(257.0, 230.0), (300.0, 230.0), (340.0, 249.5)] {
            let mut state = StatusWindowState::default();
            state.open_character(key);
            assert_eq!(click(&world, &mut state, point), None, "{point:?}");
            assert!(state.is_open());
        }
    }

    #[test]
    fn escape_closes_the_window() {
        let (world, key, ..) = world_with_luke();
        let mut state = StatusWindowState::default();
        state.open_character(key);
        let escape = egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        };
        assert_eq!(
            drive(&world, &mut state, (0.0, 0.0), vec![vec![escape]]),
            Some(StatusWindowAction::Close)
        );
        assert!(!state.is_open());
    }

    #[test]
    fn the_window_paints_its_background_and_takes_the_pointer_only_over_itself() {
        // FUN_00442d70: 379 by 272; the side-1 character's background for a
        // side-1 player is 11554.
        let (world, key, ..) = world_with_luke();
        let mut state = StatusWindowState::default();
        state.open_character(key);
        crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow_mut().clear());
        let _ = drive(&world, &mut state, (0.0, 0.0), vec![]);
        let rect = window_rect(layout());
        let painted = crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow().clone());
        assert!(painted.contains(&(11_554, rect.min)), "{painted:?}");
        assert!(painted.iter().any(|(id, _)| *id == ENCYCLOPEDIA.0));
        assert!(painted.iter().any(|(id, _)| *id == CLOSE.0));

        assert_eq!(
            rect.size(),
            egui::vec2(STATUS_WINDOW_WIDTH, STATUS_WINDOW_HEIGHT)
        );
        assert!(state.contains_screen_point(layout(), (rect.min.x, rect.min.y)));
        assert!(!state.contains_screen_point(layout(), (rect.min.x - 1.0, rect.min.y)));
        assert!(
            !StatusWindowState::default().contains_screen_point(layout(), (rect.min.x, rect.min.y))
        );
    }

    #[test]
    fn a_shorter_cell_sits_at_the_bottom_of_its_row_and_rows_stack() {
        // FUN_00449e00: the row advances by its taller cell; a value shorter
        // than its label moves down by the difference, a taller one does not
        // move the label.
        let (tops, height) = row_tops(&[(20.0, 10.0), (10.0, 30.0), (5.0, 5.0)]);
        assert_eq!(tops, [(0.0, 10.0), (20.0, 20.0), (50.0, 50.0)]);
        assert_eq!(height, 55.0);
    }

    #[test]
    fn the_picture_centres_on_307_64_with_integer_halves() {
        // FUN_00443130: (0x133 - w / 2, 0x40 - h / 2).
        assert_eq!(picture_origin(80, 80), (267.0, 24.0));
        assert_eq!(picture_origin(81, 79), (267.0, 25.0));
    }

    #[test]
    fn the_wheel_scrolls_the_list_only_as_far_as_its_rows_reach() {
        // port: the list's 204 rows of height scroll; at scale 2, 500 screen
        // pixels of rows overflow it by 46 window pixels.
        assert_eq!(scrolled(0.0, -20.0, 2.0, 500.0), 10.0);
        assert_eq!(scrolled(10.0, 8.0, 2.0, 500.0), 6.0);
        assert_eq!(scrolled(40.0, -200.0, 2.0, 500.0), 46.0);
        assert_eq!(scrolled(5.0, 100.0, 2.0, 500.0), 0.0);
        assert_eq!(scrolled(5.0, -100.0, 2.0, 300.0), 0.0);
    }

    #[test]
    fn a_side_past_the_third_takes_the_other_background() {
        assert_eq!(background(3, CockpitFaction::Alliance), 11_556);
        assert_eq!(background(3, CockpitFaction::Empire), 11_559);
    }

    fn scaled() -> CockpitLayout {
        CockpitLayout {
            canvas: CockpitViewport {
                x: 20.0,
                y: 40.0,
                width: 1280.0,
                height: 960.0,
            },
            galaxy: CockpitViewport {
                x: 130.0,
                y: 120.0,
                width: 970.0,
                height: 700.0,
            },
            scale: 2.0,
        }
    }

    /// A drawn text: its string, top-left, size and font size.
    type Text = (String, egui::Pos2, egui::Vec2, f32);

    /// Two frames at scale 2; the last frame's text shapes as (text,
    /// top-left, size, font size), and the bitmaps painted.
    fn render(
        world: &GameWorld,
        state: &mut StatusWindowState,
    ) -> (Vec<Text>, Vec<(u32, egui::Pos2)>) {
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        cache.set_base_path(staged_portrait());
        let mut output = None;
        crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow_mut().clear());
        for _ in 0..2 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1100.0),
                )),
                ..Default::default()
            };
            output = Some(ctx.run(input, |ctx| {
                let _ = draw_status_window(
                    ctx,
                    world,
                    &[],
                    state,
                    CockpitFaction::Alliance,
                    scaled(),
                    &mut cache,
                );
            }));
        }
        fn collect(shape: &egui::Shape, out: &mut Vec<Text>) {
            match shape {
                egui::Shape::Text(text) => out.push((
                    text.galley.text().to_owned(),
                    text.pos,
                    text.galley.size(),
                    text.galley
                        .job
                        .sections
                        .first()
                        .map_or(0.0, |section| section.format.font_id.size),
                )),
                egui::Shape::Vec(shapes) => {
                    for shape in shapes {
                        collect(shape, out);
                    }
                }
                _ => {}
            }
        }
        let mut texts = Vec::new();
        for clipped in &output.expect("a frame").shapes {
            collect(&clipped.shape, &mut texts);
        }
        let painted = crate::fleet_window::tests::PAINTED.with(|painted| painted.borrow().clone());
        (texts, painted)
    }

    /// A staged 80 by 80 GOKRES 2114, Luke's portrait's size, in a fresh
    /// directory.
    fn staged_portrait() -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("status-portrait-{}", std::process::id()));
        let dir = root.join("gokres-dll/BMP");
        std::fs::create_dir_all(&dir).unwrap();
        // An 8-bit indexed BMP, as the original resources are.
        let (side, offset) = (80usize, 54usize + 1024);
        let mut bmp = vec![1_u8; offset + side * side];
        bmp[..2].copy_from_slice(b"BM");
        bmp[10..14].copy_from_slice(&u32::try_from(offset).unwrap().to_le_bytes());
        bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
        bmp[18..22].copy_from_slice(&i32::try_from(side).unwrap().to_le_bytes());
        bmp[22..26].copy_from_slice(&i32::try_from(side).unwrap().to_le_bytes());
        bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
        bmp[28..30].copy_from_slice(&8u16.to_le_bytes());
        bmp[30..54].fill(0);
        std::fs::write(dir.join("2114.bmp"), bmp).unwrap();
        root
    }

    fn find<'a>(texts: &'a [Text], text: &str) -> &'a Text {
        texts
            .iter()
            .find(|(found, ..)| found == text)
            .unwrap_or_else(|| panic!("{text} not drawn in {texts:?}"))
    }

    fn near(a: egui::Pos2, b: egui::Pos2) -> bool {
        (a - b).length() < 0.01
    }

    #[test]
    fn the_title_name_and_rows_sit_where_the_window_lays_them_out() {
        // FUN_00443130 at scale 2: the title centred in (15, 18, 211, 18),
        // the name centred in (242, 137, 130, 44), labels at the list's
        // (18, 47) and values 103 to their right (FUN_00449e00).
        let (world, key, ..) = world_with_luke();
        let mut state = StatusWindowState::default();
        state.open_character(key);
        let (texts, _) = render(&world, &mut state);
        let frame = window_rect(scaled()).min;
        let at = |x: f32, y: f32| frame + egui::vec2(x, y) * 2.0;

        let (_, pos, size, font) = find(&texts, "Character Status");
        assert!(
            near(*pos + *size / 2.0, at(15.0 + 105.5, 18.0 + 9.0)),
            "{pos:?} {size:?}"
        );
        assert_eq!(*font, 22.0);
        let (_, pos, size, font) = find(&texts, "Luke Skywalker");
        assert!(
            near(*pos + *size / 2.0, at(242.0 + 65.0, 137.0 + 22.0)),
            "{pos:?} {size:?}"
        );
        assert_eq!(*font, 22.0);

        let (_, commanding, row, font) = find(&texts, "Commanding:");
        assert!(near(*commanding, at(18.0, 47.0)), "{commanding:?}");
        assert_eq!(*font, 18.0);
        let (_, none, ..) = find(&texts, "None");
        assert!(near(*none, at(18.0 + 103.0, 47.0)), "{none:?}");
        let (_, yavin, ..) = find(&texts, "Yavin");
        assert!(near(*yavin, *none + egui::vec2(0.0, row.y)), "{yavin:?}");
        let (_, attached, ..) = find(&texts, "Attached: ");
        assert!(
            near(*attached, *commanding + egui::vec2(0.0, row.y)),
            "{attached:?}"
        );
    }

    #[test]
    fn a_scrolled_list_draws_its_rows_higher() {
        // A name that wraps over many lines pushes the rows past the list.
        let (mut world, key, yavin, _) = world_with_luke();
        world.systems[yavin].name = "Yavin ".repeat(40);
        let mut state = StatusWindowState::default();
        state.open_character(key);
        if let Some(open) = state.window.as_mut() {
            open.scroll = 10.0;
        }
        let (texts, _) = render(&world, &mut state);
        let frame = window_rect(scaled()).min;
        let (_, commanding, ..) = find(&texts, "Commanding:");
        assert!(
            near(*commanding, frame + egui::vec2(18.0, 47.0 - 10.0) * 2.0),
            "{commanding:?}"
        );
    }

    #[test]
    fn the_portrait_and_the_sides_background_are_painted() {
        // FUN_00443130: Luke's 80 by 80 portrait at (267, 24); an Empire
        // character's background 11555 and a neutral one's 11556 for a
        // side-1 player. Reads the staged GOKRES and STRATEGY bitmaps.
        let (mut world, key, ..) = world_with_luke();
        let mut state = StatusWindowState::default();
        state.open_character(key);
        let frame = window_rect(scaled()).min;
        let (_, painted) = render(&world, &mut state);
        assert!(painted.contains(&(11_554, frame)), "{painted:?}");
        assert!(
            painted.contains(&(2_114, frame + egui::vec2(267.0, 24.0) * 2.0)),
            "{painted:?}"
        );

        world.characters[key].is_alliance = false;
        world.characters[key].is_empire = true;
        let (_, painted) = render(&world, &mut state);
        assert!(painted.contains(&(11_555, frame)), "{painted:?}");
        world.characters[key].is_empire = false;
        let (_, painted) = render(&world, &mut state);
        assert!(painted.contains(&(11_556, frame)), "{painted:?}");
    }

    #[test]
    fn a_window_for_a_character_that_is_gone_closes() {
        let (mut world, key, ..) = world_with_luke();
        let mut state = StatusWindowState::default();
        state.open_character(key);
        world.characters.remove(key);
        let _ = drive(&world, &mut state, (0.0, 0.0), vec![]);
        assert!(!state.is_open());
    }
}
