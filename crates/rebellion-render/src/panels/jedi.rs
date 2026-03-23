//! Jedi training panel — Force progression, training management, detection risk.
//!
//! Shows Force-sensitive characters, their tier (None→Aware→Training→Experienced),
//! training progress via XP bars, detection risk, and start/stop controls.
//! Returns `PanelAction::StartJediTraining` / `StopJediTraining`.

use egui_macroquad::egui::{self, Color32, ProgressBar, RichText, ScrollArea};
use rebellion_core::ids::CharacterKey;
use rebellion_core::jedi::{JediState, XP_TO_EXPERIENCED, XP_TO_TRAINING,
    DETECT_PROB_AWARE, DETECT_PROB_TRAINING, DETECT_PROB_EXPERIENCED};
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::{Character, ForceTier, GameWorld};

use crate::theme;
use super::PanelAction;

/// Mutable UI state for the Jedi training panel.
#[derive(Debug, Clone, Default)]
pub struct JediPanelState {
    /// If set, show detail view for this character.
    pub focused: Option<CharacterKey>,
}

/// Draw the Jedi training panel as a left-side egui panel.
pub fn draw_jedi(
    ctx: &egui::Context,
    world: &GameWorld,
    jedi_state: &JediState,
    state: &mut JediPanelState,
    player_faction: MissionFaction,
) -> Option<PanelAction> {
    let mut action = None;
    let is_alliance = player_faction == MissionFaction::Alliance;

    egui::SidePanel::left("jedi_panel")
        .min_width(280.0)
        .max_width(340.0)
        .show(ctx, |ui| {
            let title = if is_alliance { "Jedi Training" } else { "Sith Training" };
            ui.heading(RichText::new(title).color(theme::GOLD));
            ui.separator();

            // Collect Force-sensitive characters for this faction
            let mut sensitives: Vec<(CharacterKey, &Character, Option<u32>)> = Vec::new();
            for (key, character) in world.characters.iter() {
                let owns = if is_alliance { character.is_alliance } else { !character.is_alliance };
                if !owns { continue; }
                if character.jedi_probability == 0 && character.force_tier == ForceTier::None {
                    continue;
                }

                // Get accumulated XP from training record if exists
                let xp = jedi_state.training.iter()
                    .find(|r| r.character == key)
                    .map(|r| r.accumulated_xp);

                sensitives.push((key, character, xp));
            }

            // Sort: Training first, then Experienced, then Aware, then None
            sensitives.sort_by_key(|(_, c, _)| match c.force_tier {
                ForceTier::Training => 0,
                ForceTier::Experienced => 1,
                ForceTier::Aware => 2,
                ForceTier::None => 3,
            });

            if sensitives.is_empty() {
                ui.add_space(20.0);
                ui.label(
                    RichText::new("No Force-sensitive characters detected.")
                        .color(theme::TEXT_DISABLED),
                );
                return;
            }

            // Summary counts
            let training_count = sensitives.iter().filter(|(_, c, _)| c.force_tier == ForceTier::Training).count();
            let experienced_count = sensitives.iter().filter(|(_, c, _)| c.force_tier == ForceTier::Experienced).count();
            let aware_count = sensitives.iter().filter(|(_, c, _)| c.force_tier == ForceTier::Aware).count();

            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Training: {}", training_count)).color(theme::WARNING_AMBER).size(11.0));
                ui.label(RichText::new(format!("Ready: {}", experienced_count)).color(theme::SUCCESS_GREEN).size(11.0));
                ui.label(RichText::new(format!("Aware: {}", aware_count)).color(theme::TEXT_SECONDARY).size(11.0));
            });
            ui.add_space(8.0);

            // ── Character list ────────────────────────────────────────────────
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (key, character, xp) in &sensitives {
                        let is_training = jedi_state.is_training(*key);
                        let tier = character.force_tier;

                        ui.group(|ui| {
                            // Name + tier badge
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(&character.name)
                                        .color(theme::TEXT_PRIMARY)
                                        .strong(),
                                );
                                let (tier_label, tier_color) = tier_display(tier);
                                ui.label(
                                    RichText::new(tier_label)
                                        .color(tier_color)
                                        .size(10.0),
                                );
                            });

                            // XP progress bar (only for Aware and Training)
                            match tier {
                                ForceTier::Aware => {
                                    let current_xp = xp.unwrap_or(0);
                                    let progress = current_xp as f32 / XP_TO_TRAINING as f32;
                                    ui.add(
                                        ProgressBar::new(progress.min(1.0))
                                            .text(format!("{}/{} XP to Training", current_xp, XP_TO_TRAINING))
                                            .fill(Color32::from_rgb(60, 100, 180)),
                                    );
                                }
                                ForceTier::Training => {
                                    let current_xp = xp.unwrap_or(0);
                                    let progress = current_xp as f32 / XP_TO_EXPERIENCED as f32;
                                    ui.add(
                                        ProgressBar::new(progress.min(1.0))
                                            .text(format!("{}/{} XP to Experienced", current_xp, XP_TO_EXPERIENCED))
                                            .fill(Color32::from_rgb(100, 60, 180)),
                                    );
                                }
                                ForceTier::Experienced => {
                                    ui.label(
                                        RichText::new("FULLY TRAINED")
                                            .color(theme::SUCCESS_GREEN)
                                            .size(10.0),
                                    );
                                }
                                ForceTier::None => {
                                    ui.label(
                                        RichText::new(format!("Force potential: {}%", character.jedi_probability))
                                            .color(theme::TEXT_DISABLED)
                                            .size(10.0),
                                    );
                                }
                            }

                            // Detection risk
                            if tier != ForceTier::None && tier != ForceTier::Experienced {
                                let risk = match tier {
                                    ForceTier::Aware => DETECT_PROB_AWARE,
                                    ForceTier::Training => DETECT_PROB_TRAINING,
                                    _ => DETECT_PROB_EXPERIENCED,
                                };
                                ui.label(
                                    RichText::new(format!("Detection risk: {:.0}%", risk * 100.0))
                                        .color(if risk > 0.1 { theme::WARNING_AMBER } else { theme::TEXT_SECONDARY })
                                        .size(10.0),
                                );
                            }

                            // Training controls
                            match tier {
                                ForceTier::Aware | ForceTier::Training => {
                                    if is_training {
                                        if ui.button(
                                            RichText::new("Stop Training")
                                                .color(theme::DANGER_RED)
                                                .size(11.0),
                                        ).clicked() {
                                            action = Some(PanelAction::StopJediTraining {
                                                character: *key,
                                            });
                                        }
                                    } else if !character.is_captive && !character.on_mission {
                                        if ui.button(
                                            RichText::new("Begin Training")
                                                .color(theme::GOLD)
                                                .size(11.0),
                                        ).clicked() {
                                            action = Some(PanelAction::StartJediTraining {
                                                character: *key,
                                                faction: player_faction,
                                            });
                                        }
                                    } else {
                                        ui.label(
                                            RichText::new("Unavailable (on mission or captured)")
                                                .color(theme::TEXT_DISABLED)
                                                .size(10.0),
                                        );
                                    }
                                }
                                _ => {}
                            }
                        });
                        ui.add_space(4.0);
                    }
                });
        });

    action
}

/// Display label and color for a Force tier.
fn tier_display(tier: ForceTier) -> (&'static str, Color32) {
    match tier {
        ForceTier::None => ("LATENT", Color32::from_rgb(100, 100, 100)),
        ForceTier::Aware => ("AWARE", Color32::from_rgb(100, 160, 255)),
        ForceTier::Training => ("TRAINING", Color32::from_rgb(180, 120, 255)),
        ForceTier::Experienced => ("EXPERIENCED", Color32::from_rgb(60, 200, 100)),
    }
}
