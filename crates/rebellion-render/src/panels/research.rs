//! Research panel — tech tree progression with character assignment.
//!
//! Shows three tech trees (Ship, Troop, Facility) with current levels,
//! active research projects, assignable characters, and progress bars.
//! Returns `PanelAction::DispatchResearch` / `CancelResearch`.

use egui_macroquad::egui::{self, Color32, ProgressBar, RichText, ScrollArea};
use rebellion_core::ids::CharacterKey;
use rebellion_core::missions::MissionFaction;
use rebellion_core::research::{ResearchState, TechType, RESEARCH_MAX_LEVEL};
use rebellion_core::world::{Character, GameWorld};

use crate::theme;
use super::PanelAction;

/// Mutable UI state for the research panel.
#[derive(Debug, Clone, Default)]
pub struct ResearchPanelState {
    /// Currently selected tech tree tab.
    pub selected_tree: usize, // 0=Ship, 1=Troop, 2=Facility
}

/// Draw the research panel as a left-side egui panel.
///
/// Shows tech tree levels, active projects with progress, and assignable characters.
pub fn draw_research(
    ctx: &egui::Context,
    world: &GameWorld,
    research_state: &ResearchState,
    state: &mut ResearchPanelState,
    player_faction: MissionFaction,
) -> Option<PanelAction> {
    let mut action = None;
    let is_alliance = player_faction == MissionFaction::Alliance;

    egui::SidePanel::left("research_panel")
        .min_width(280.0)
        .max_width(340.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Research").color(theme::GOLD));
            ui.separator();

            // ── Tech tree tabs ───────────────────────────────────────────────
            ui.horizontal(|ui| {
                for (i, label) in ["Ship", "Troop", "Facility"].iter().enumerate() {
                    let selected = state.selected_tree == i;
                    let text = RichText::new(*label)
                        .color(if selected { theme::GOLD_BRIGHT } else { theme::TEXT_PRIMARY })
                        .strong();
                    if ui.selectable_label(selected, text).clicked() {
                        state.selected_tree = i;
                    }
                }
            });
            ui.add_space(8.0);

            let tech_type = match state.selected_tree {
                0 => TechType::Ship,
                1 => TechType::Troop,
                _ => TechType::Facility,
            };

            let current_level = research_state.level(is_alliance, tech_type);

            // ── Current level ────────────────────────────────────────────────
            ui.horizontal(|ui| {
                ui.label(RichText::new("Level:").color(theme::TEXT_SECONDARY));
                ui.label(
                    RichText::new(format!("{} / {}", current_level, RESEARCH_MAX_LEVEL))
                        .color(theme::GOLD)
                        .strong(),
                );
            });

            if current_level >= RESEARCH_MAX_LEVEL {
                ui.add_space(8.0);
                ui.label(
                    RichText::new("MAX LEVEL REACHED")
                        .color(theme::SUCCESS_GREEN)
                        .strong(),
                );
                return;
            }

            ui.add_space(8.0);

            // ── Active project ───────────────────────────────────────────────
            let active_project = research_state.projects.iter().find(|p| {
                p.faction_is_alliance == is_alliance && p.tech_type == tech_type
            });

            if let Some(project) = active_project {
                ui.group(|ui| {
                    ui.label(
                        RichText::new("ACTIVE RESEARCH")
                            .color(theme::GOLD_DIM)
                            .size(11.0)
                            .strong(),
                    );

                    // Find the character name
                    let char_name = world.characters.get(project.character)
                        .map(|c| c.name.as_str())
                        .unwrap_or("Unknown");
                    ui.label(
                        RichText::new(format!("Researcher: {}", char_name))
                            .color(theme::TEXT_PRIMARY),
                    );

                    // Progress bar
                    let progress = project.progress();
                    ui.add(
                        ProgressBar::new(progress)
                            .text(format!(
                                "Level {} → {} ({} ticks left)",
                                current_level,
                                current_level + 1,
                                project.ticks_remaining,
                            ))
                            .fill(theme::GOLD_DIM),
                    );

                    ui.add_space(4.0);

                    // Cancel button
                    if ui.button(
                        RichText::new("Cancel Research")
                            .color(theme::DANGER_RED)
                            .size(12.0),
                    ).clicked() {
                        action = Some(PanelAction::CancelResearch {
                            tech_type,
                            faction: player_faction,
                        });
                    }
                });
            } else {
                // ── No active project — show assignable characters ───────────
                ui.label(
                    RichText::new("NO ACTIVE PROJECT")
                        .color(theme::WARNING_AMBER)
                        .size(11.0)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new("Assign a character with the right skill to begin research.")
                        .color(theme::TEXT_SECONDARY)
                        .size(11.0),
                );
                ui.add_space(8.0);

                // Find eligible characters
                let eligible: Vec<(CharacterKey, &Character)> = world
                    .characters
                    .iter()
                    .filter(|(_, c)| {
                        let owns = if is_alliance { c.is_alliance } else { !c.is_alliance };
                        if !owns { return false; }
                        if c.is_captive || c.on_mission || c.on_mandatory_mission { return false; }
                        // Check relevant skill ≥ 1
                        let skill = match tech_type {
                            TechType::Ship => c.ship_design.base,
                            TechType::Troop => c.troop_training.base,
                            TechType::Facility => c.facility_design.base,
                        };
                        skill > 0
                    })
                    .collect();

                if eligible.is_empty() {
                    ui.label(
                        RichText::new("No eligible characters available.")
                            .color(theme::TEXT_DISABLED)
                            .size(11.0),
                    );
                } else {
                    ui.label(
                        RichText::new(format!("{} eligible:", eligible.len()))
                            .color(theme::TEXT_SECONDARY)
                            .size(11.0),
                    );
                    ui.add_space(4.0);

                    ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            for (key, character) in &eligible {
                                let skill_val = match tech_type {
                                    TechType::Ship => character.ship_design.base,
                                    TechType::Troop => character.troop_training.base,
                                    TechType::Facility => character.facility_design.base,
                                };
                                let skill_name = match tech_type {
                                    TechType::Ship => "Ship Design",
                                    TechType::Troop => "Troop Training",
                                    TechType::Facility => "Facility Design",
                                };

                                ui.horizontal(|ui| {
                                    let btn = ui.button(
                                        RichText::new(&character.name)
                                            .color(theme::TEXT_PRIMARY)
                                            .size(12.0),
                                    );

                                    ui.label(
                                        RichText::new(format!("{}: {}", skill_name, skill_val))
                                            .color(theme::TEXT_SECONDARY)
                                            .size(11.0),
                                    );

                                    if btn.clicked() {
                                        action = Some(PanelAction::DispatchResearch {
                                            character: *key,
                                            tech_type,
                                            faction: player_faction,
                                        });
                                    }
                                });
                            }
                        });
                }
            }
        });

    action
}
