//! Officers panel — character roster with skills, role flags, and Jedi status.
//!
//! Lists all characters accessible to the player faction. Clicking a row
//! focuses that character's detail view in the same panel. The panel is
//! rendered as an egui side panel on the left side of the screen.

use egui_macroquad::egui::{self, Color32, RichText, ScrollArea};
use rebellion_core::ids::CharacterKey;
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::{Character, GameWorld, SkillPair};

use super::PanelAction;

// ---------------------------------------------------------------------------
// OfficersState
// ---------------------------------------------------------------------------

/// Mutable state for the officers panel.
#[derive(Debug, Clone, Default)]
pub struct OfficersState {
    /// The character currently shown in the detail pane.  `None` = list view.
    pub focused: Option<CharacterKey>,
    /// Current name filter string.
    pub filter: String,
    /// If true, show only characters belonging to the player faction.
    pub filter_faction: bool,
}

// ---------------------------------------------------------------------------
// draw_officers
// ---------------------------------------------------------------------------

/// Render the character roster as a left-side egui panel.
///
/// `player_faction` determines which characters are highlighted as "yours".
/// Returns `Some(PanelAction::FocusCharacter(_))` when the player clicks a row.
pub fn draw_officers(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut OfficersState,
    player_faction: MissionFaction,
) -> Option<PanelAction> {
    let mut action = None;

    egui::SidePanel::left("officers_panel")
        .min_width(260.0)
        .max_width(320.0)
        .show(ctx, |ui| {
            ui.heading("Officers");
            ui.separator();

            // ── Filters ───────────────────────────────────────────────────────
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut state.filter);
            });
            ui.checkbox(&mut state.filter_faction, "Your faction only");
            ui.add_space(4.0);

            if let Some(key) = state.focused {
                // ── Detail view ───────────────────────────────────────────────
                if ui.small_button("← Back").clicked() {
                    state.focused = None;
                }
                ui.separator();
                if let Some(character) = world.characters.get(key) {
                    draw_character_detail(ui, character);
                } else {
                    ui.label("Character not found.");
                    state.focused = None;
                }
            } else {
                // ── Roster list ───────────────────────────────────────────────
                ScrollArea::vertical().show(ui, |ui| {
                    let filter_lower = state.filter.to_lowercase();

                    for (key, character) in &world.characters {
                        // Faction filter.
                        if state.filter_faction {
                            let belongs = match player_faction {
                                MissionFaction::Alliance => character.is_alliance,
                                MissionFaction::Empire => character.is_empire,
                            };
                            if !belongs {
                                continue;
                            }
                        }

                        // Name filter.
                        if !filter_lower.is_empty()
                            && !character.name.to_lowercase().contains(&filter_lower)
                        {
                            continue;
                        }

                        let is_player_faction = match player_faction {
                            MissionFaction::Alliance => character.is_alliance,
                            MissionFaction::Empire => character.is_empire,
                        };

                        let name_color = if is_player_faction {
                            Color32::from_rgb(200, 220, 255)
                        } else {
                            Color32::from_gray(160)
                        };

                        ui.horizontal(|ui| {
                            // Faction dot indicator.
                            let dot_color = faction_dot_color(character);
                            ui.colored_label(dot_color, "●");

                            let response = ui.add(
                                egui::Label::new(
                                    RichText::new(&character.name).color(name_color),
                                )
                                .sense(egui::Sense::click()),
                            );

                            if response.clicked() {
                                state.focused = Some(key);
                                action = Some(PanelAction::FocusCharacter(key));
                            }

                            // Role badges on the right.
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    draw_role_badges(ui, character);
                                },
                            );
                        });
                    }
                });
            }
        });

    action
}

// ---------------------------------------------------------------------------
// Detail view helpers
// ---------------------------------------------------------------------------

/// Render the full character detail inside an already-open panel UI.
fn draw_character_detail(ui: &mut egui::Ui, character: &Character) {
    ui.heading(&character.name);
    ui.add_space(2.0);

    // ── Identity ──────────────────────────────────────────────────────────────
    let faction_str = match (character.is_alliance, character.is_empire) {
        (true, false) => "Rebel Alliance",
        (false, true) => "Galactic Empire",
        (true, true) => "Both factions",
        _ => "Independent",
    };
    ui.label(format!("Faction: {faction_str}"));
    ui.label(format!("Type: {}", if character.is_major { "Major" } else { "Minor" }));

    // Role flags.
    let mut roles = Vec::new();
    if character.can_be_admiral { roles.push("Admiral"); }
    if character.can_be_commander { roles.push("Commander"); }
    if character.can_be_general { roles.push("General"); }
    if !roles.is_empty() {
        ui.label(format!("Roles: {}", roles.join(", ")));
    }

    // Jedi info.
    if character.jedi_probability > 0 {
        ui.colored_label(
            Color32::from_rgb(200, 200, 100),
            format!(
                "Force sensitivity: {}%  (level {}/{})",
                character.jedi_probability,
                character.jedi_level.base,
                character.jedi_level.base + character.jedi_level.variance,
            ),
        );
    }

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);

    // ── Skill table ───────────────────────────────────────────────────────────
    egui::Grid::new("char_skills")
        .num_columns(2)
        .striped(true)
        .spacing([8.0, 2.0])
        .show(ui, |ui| {
            skill_row(ui, "Diplomacy",   character.diplomacy);
            skill_row(ui, "Espionage",   character.espionage);
            skill_row(ui, "Ship Design", character.ship_design);
            skill_row(ui, "Troop Train", character.troop_training);
            skill_row(ui, "Fac. Design", character.facility_design);
            skill_row(ui, "Combat",      character.combat);
            skill_row(ui, "Leadership",  character.leadership);
            skill_row(ui, "Loyalty",     character.loyalty);
        });
}

/// Render one skill row inside a grid: label | bar + value range.
fn skill_row(ui: &mut egui::Ui, label: &str, skill: SkillPair) {
    ui.label(RichText::new(label).color(Color32::from_gray(180)).small());
    ui.horizontal(|ui| {
        // Progress bar up to base value (max 100).
        let fraction = (skill.base as f32 / 100.0).clamp(0.0, 1.0);
        let bar_color = skill_bar_color(skill.base);
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(80.0, 10.0),
            egui::Sense::hover(),
        );
        ui.painter().rect_filled(
            rect,
            2.0,
            Color32::from_gray(40),
        );
        let fill_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(rect.width() * fraction, rect.height()),
        );
        ui.painter().rect_filled(fill_rect, 2.0, bar_color);

        if skill.variance > 0 {
            ui.label(
                RichText::new(format!("{} (+{})", skill.base, skill.variance))
                    .small()
                    .color(Color32::from_gray(200)),
            );
        } else {
            ui.label(
                RichText::new(format!("{}", skill.base))
                    .small()
                    .color(Color32::from_gray(200)),
            );
        }
    });
    ui.end_row();
}

/// Skill bar fill color by value tier.
fn skill_bar_color(value: u32) -> Color32 {
    if value >= 75 {
        Color32::from_rgb(100, 220, 100)
    } else if value >= 45 {
        Color32::from_rgb(220, 200, 60)
    } else {
        Color32::from_rgb(200, 100, 60)
    }
}

/// Colored dot indicating faction ownership.
fn faction_dot_color(character: &Character) -> Color32 {
    match (character.is_alliance, character.is_empire) {
        (true, false) => Color32::from_rgb(80, 140, 220),
        (false, true) => Color32::from_rgb(200, 60, 60),
        _ => Color32::from_gray(120),
    }
}

/// Compact role badge labels rendered right-to-left.
fn draw_role_badges(ui: &mut egui::Ui, character: &Character) {
    if character.can_be_general {
        ui.label(RichText::new("GEN").color(Color32::from_rgb(180, 140, 60)).small());
    }
    if character.can_be_admiral {
        ui.label(RichText::new("ADM").color(Color32::from_rgb(100, 180, 220)).small());
    }
    if character.can_be_commander {
        ui.label(RichText::new("CMD").color(Color32::from_rgb(120, 200, 120)).small());
    }
    if character.jedi_probability > 0 {
        ui.label(RichText::new("JEDI").color(Color32::from_rgb(220, 210, 120)).small());
    }
}
