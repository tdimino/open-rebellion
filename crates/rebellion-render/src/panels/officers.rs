//! Officers panel — character roster with skills, role flags, and Jedi status.
//!
//! Lists all characters accessible to the player faction. Clicking a row
//! focuses that character's detail view in the same panel. The panel is
//! rendered as an egui side panel on the left side of the screen.

use egui_macroquad::egui::{self, Color32, ProgressBar, RichText, ScrollArea, Vec2};
use rebellion_core::ids::CharacterKey;
use rebellion_core::jedi::{DETECT_PROB_AWARE, DETECT_PROB_TRAINING, DETECT_PROB_EXPERIENCED,
    XP_TO_TRAINING, XP_TO_EXPERIENCED};
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::{Character, ForceTier, GameWorld, SkillPair};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::theme;
use super::PanelAction;

// ---------------------------------------------------------------------------
// Portrait resource ID helpers
// ---------------------------------------------------------------------------

/// Return the GOKRES.DLL resource ID for a character's 80×80 portrait.
///
/// Formula derived from the DAT binary layout:
///   `portrait_id = dat_id + family_id * 32`
///
/// Major characters use `family_id = 48` (0x30), giving offset 1536.
/// Minor characters use `family_id = 56` (0x38), giving offset 1792.
fn portrait_resource_id(character: &Character) -> u32 {
    let offset = if character.is_major { 1536u32 } else { 1792u32 };
    character.dat_id.raw() + offset
}

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
/// `bmp_cache` provides GOKRES.DLL portrait textures for the detail view.
/// Returns `Some(PanelAction::FocusCharacter(_))` when the player clicks a row.
pub fn draw_officers(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut OfficersState,
    player_faction: MissionFaction,
    bmp_cache: &mut BmpCache,
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
                    draw_character_detail(ui, ctx, key, character, world, bmp_cache);
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
fn draw_character_detail(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    key: CharacterKey,
    character: &Character,
    world: &GameWorld,
    bmp_cache: &mut BmpCache,
) {
    // ── Portrait (GOKRES.DLL 80×80 headshot) ─────────────────────────────────
    let portrait_id = portrait_resource_id(character);
    if let Some(texture) = bmp_cache.get(ctx, DllSource::Gokres, portrait_id) {
        ui.add(egui::Image::new(texture).fit_to_exact_size(Vec2::new(80.0, 80.0)));
    }

    // ── 40a: Themed header + status badges ───────────────────────────────────
    let name_color = if character.is_alliance { theme::ALLIANCE_BLUE }
        else if character.is_empire { theme::EMPIRE_RED }
        else { theme::TEXT_PRIMARY };
    ui.heading(RichText::new(&character.name).color(name_color));

    // Status badges
    ui.horizontal(|ui| {
        let type_label = if character.is_major { "MAJOR" } else { "MINOR" };
        ui.label(RichText::new(type_label).color(theme::TEXT_SECONDARY).size(10.0));

        if character.is_captive {
            ui.label(RichText::new("CAPTIVE").color(theme::DANGER_RED).size(10.0).strong());
        }
        if character.on_mission {
            ui.label(RichText::new("ON MISSION").color(theme::WARNING_AMBER).size(10.0).strong());
        }
        if character.on_mandatory_mission {
            ui.label(RichText::new("MANDATORY").color(theme::WARNING_AMBER).size(10.0).strong());
        }

        // Force tier badge
        match character.force_tier {
            ForceTier::Aware => { ui.label(RichText::new("FORCE AWARE").color(Color32::from_rgb(100, 160, 255)).size(10.0)); }
            ForceTier::Training => { ui.label(RichText::new("TRAINING").color(Color32::from_rgb(180, 120, 255)).size(10.0)); }
            ForceTier::Experienced => { ui.label(RichText::new("JEDI").color(Color32::from_rgb(60, 200, 100)).size(10.0).strong()); }
            ForceTier::None => {}
        }
    });

    // Role flags
    let mut roles = Vec::new();
    if character.can_be_admiral { roles.push("Admiral"); }
    if character.can_be_commander { roles.push("Commander"); }
    if character.can_be_general { roles.push("General"); }
    if !roles.is_empty() {
        ui.label(RichText::new(roles.join(" / ")).color(theme::GOLD_DIM).size(11.0));
    }

    ui.add_space(4.0);
    ui.separator();

    // ── 40c: Location + assignment section ───────────────────────────────────
    ui.label(RichText::new("STATUS").color(theme::GOLD_DIM).size(10.0).strong());

    // Find current location via fleet assignment
    let mut location_name: Option<String> = None;
    let mut fleet_info: Option<String> = None;
    for (_, fleet) in world.fleets.iter() {
        if fleet.characters.contains(&key) {
            if let Some(sys) = world.systems.get(fleet.location) {
                location_name = Some(sys.name.clone());
            }
            let ship_count: u32 = fleet.capital_ships.iter().map(|e| e.count).sum();
            let tag = if fleet.is_alliance { "Alliance" } else { "Empire" };
            fleet_info = Some(format!("{} fleet ({} ships)", tag, ship_count));
            break;
        }
    }

    if let Some(loc) = &location_name {
        ui.label(RichText::new(format!("Location: {}", loc)).color(theme::TEXT_PRIMARY).size(11.0));
    }
    if let Some(fleet) = &fleet_info {
        ui.label(RichText::new(format!("Fleet: {}", fleet)).color(theme::TEXT_SECONDARY).size(11.0));
    }
    if character.on_mission {
        ui.label(RichText::new("Currently on mission").color(theme::WARNING_AMBER).size(11.0));
    }
    if location_name.is_none() && !character.on_mission {
        ui.label(RichText::new("Unassigned").color(theme::TEXT_DISABLED).size(11.0));
    }

    ui.add_space(4.0);
    ui.separator();

    // ── 40d: Force progression detail ────────────────────────────────────────
    if character.jedi_probability > 0 || character.force_tier != ForceTier::None {
        ui.label(RichText::new("FORCE").color(theme::GOLD_DIM).size(10.0).strong());

        let (tier_label, tier_color) = match character.force_tier {
            ForceTier::None => ("Latent", Color32::from_gray(120)),
            ForceTier::Aware => ("Force Aware", Color32::from_rgb(100, 160, 255)),
            ForceTier::Training => ("In Training", Color32::from_rgb(180, 120, 255)),
            ForceTier::Experienced => ("Jedi Knight", Color32::from_rgb(60, 200, 100)),
        };
        ui.label(RichText::new(tier_label).color(tier_color).size(12.0).strong());

        // XP progress bar
        match character.force_tier {
            ForceTier::Aware => {
                let xp = character.force_experience;
                let progress = xp as f32 / XP_TO_TRAINING as f32;
                ui.add(ProgressBar::new(progress.min(1.0))
                    .text(format!("{}/{} XP", xp, XP_TO_TRAINING))
                    .fill(Color32::from_rgb(60, 100, 180)));
            }
            ForceTier::Training => {
                let xp = character.force_experience;
                let progress = xp as f32 / XP_TO_EXPERIENCED as f32;
                ui.add(ProgressBar::new(progress.min(1.0))
                    .text(format!("{}/{} XP", xp, XP_TO_EXPERIENCED))
                    .fill(Color32::from_rgb(100, 60, 180)));
            }
            _ => {}
        }

        // Detection risk
        if character.force_tier != ForceTier::None && character.force_tier != ForceTier::Experienced {
            let risk = match character.force_tier {
                ForceTier::Aware => DETECT_PROB_AWARE,
                ForceTier::Training => DETECT_PROB_TRAINING,
                _ => DETECT_PROB_EXPERIENCED,
            };
            let risk_color = if risk > 0.1 { theme::WARNING_AMBER } else { theme::TEXT_SECONDARY };
            ui.label(RichText::new(format!("Detection risk: {:.0}%", risk * 100.0)).color(risk_color).size(10.0));
        }

        if character.force_tier == ForceTier::None && character.jedi_probability > 0 {
            ui.label(RichText::new(format!("Sensitivity: {}%", character.jedi_probability)).color(theme::TEXT_SECONDARY).size(10.0));
        }

        ui.add_space(4.0);
        ui.separator();
    }

    // ── 40b: Themed skill progress bars ──────────────────────────────────────
    ui.label(RichText::new("SKILLS").color(theme::GOLD_DIM).size(10.0).strong());
    ui.add_space(2.0);

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

/// Render one skill row inside a grid: label | progress bar + value.
fn skill_row(ui: &mut egui::Ui, label: &str, skill: SkillPair) {
    ui.label(RichText::new(label).color(theme::TEXT_SECONDARY).size(11.0));
    ui.horizontal(|ui| {
        let fraction = (skill.base as f32 / 100.0).clamp(0.0, 1.0);
        let bar_color = skill_bar_color(skill.base);

        let text = if skill.variance > 0 {
            format!("{} (+{})", skill.base, skill.variance)
        } else {
            format!("{}", skill.base)
        };

        ui.add_sized(
            [100.0, 14.0],
            ProgressBar::new(fraction).text(text).fill(bar_color),
        );
    });
    ui.end_row();
}

/// Skill bar fill color by value tier — themed.
fn skill_bar_color(value: u32) -> Color32 {
    if value >= 75 {
        theme::SUCCESS_GREEN
    } else if value >= 45 {
        theme::GOLD_DIM
    } else if value >= 20 {
        Color32::from_rgb(180, 100, 60)
    } else {
        Color32::from_rgb(100, 60, 60)
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
