//! Loyalty & Uprising dashboard — per-system loyalty view with uprising risk
//! and character betrayal risk, sorted by danger level.
//!
//! Read-only dashboard (no PanelActions) — surfaces information from the
//! uprising and betrayal simulation systems to help the player prioritize
//! diplomatic missions and troop deployments.

use egui_macroquad::egui::{self, Color32, ProgressBar, RichText, ScrollArea};
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::{ControlKind, GameWorld};

use crate::theme;
use super::PanelAction;

/// Draw the loyalty dashboard as a left-side egui panel.
pub fn draw_loyalty(
    ctx: &egui::Context,
    world: &GameWorld,
    player_faction: MissionFaction,
) -> Option<PanelAction> {
    let is_alliance = player_faction == MissionFaction::Alliance;

    egui::SidePanel::left("loyalty_panel")
        .min_width(280.0)
        .max_width(340.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Loyalty & Uprisings").color(theme::GOLD));
            ui.separator();

            // ── Collect systems with their loyalty scores ─────────────
            let mut systems: Vec<(&str, f32, f32, bool, bool)> = Vec::new();

            for (_, system) in world.systems.iter() {
                if system.is_destroyed { continue; }
                let our_pop = if is_alliance {
                    system.popularity_alliance
                } else {
                    system.popularity_empire
                };
                let enemy_pop = if is_alliance {
                    system.popularity_empire
                } else {
                    system.popularity_alliance
                };

                let is_ours = match system.control {
                    ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => is_alliance,
                    ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => !is_alliance,
                    _ => false,
                };
                let is_uprising = matches!(system.control, ControlKind::Uprising(_));

                systems.push((&system.name, our_pop, enemy_pop, is_ours, is_uprising));
            }

            // Sort by danger: uprisings first, then lowest loyalty
            systems.sort_by(|a, b| {
                let a_danger = if a.4 { -1.0 } else { a.1 };
                let b_danger = if b.4 { -1.0 } else { b.1 };
                a_danger.partial_cmp(&b_danger).unwrap_or(std::cmp::Ordering::Equal)
            });

            // ── Summary counts ───────────────────────────────────────
            let uprising_count = systems.iter().filter(|s| s.4).count();
            let at_risk = systems.iter().filter(|s| s.3 && s.1 < 0.4 && !s.4).count();
            let stable = systems.iter().filter(|s| s.3 && s.1 >= 0.6).count();

            ui.horizontal(|ui| {
                if uprising_count > 0 {
                    ui.label(
                        RichText::new(format!("{} uprising", uprising_count))
                            .color(theme::DANGER_RED)
                            .size(11.0)
                            .strong(),
                    );
                }
                if at_risk > 0 {
                    ui.label(
                        RichText::new(format!("{} at risk", at_risk))
                            .color(theme::WARNING_AMBER)
                            .size(11.0),
                    );
                }
                ui.label(
                    RichText::new(format!("{} stable", stable))
                        .color(theme::SUCCESS_GREEN)
                        .size(11.0),
                );
            });
            ui.add_space(8.0);

            // ── System list ──────────────────────────────────────────
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (name, our_pop, enemy_pop, is_ours, is_uprising) in &systems {
                        let (status_label, status_color) = if *is_uprising {
                            ("UPRISING", theme::DANGER_RED)
                        } else if *is_ours && *our_pop < 0.3 {
                            ("CRITICAL", theme::DANGER_RED)
                        } else if *is_ours && *our_pop < 0.4 {
                            ("AT RISK", theme::WARNING_AMBER)
                        } else if *is_ours {
                            ("STABLE", theme::SUCCESS_GREEN)
                        } else {
                            ("ENEMY", Color32::from_gray(100))
                        };

                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(status_label)
                                    .color(status_color)
                                    .size(9.0)
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(*name)
                                    .color(theme::TEXT_PRIMARY)
                                    .size(11.0),
                            );
                        });

                        // Loyalty bars
                        let our_color = if is_alliance { theme::ALLIANCE_BLUE } else { theme::EMPIRE_RED };
                        let enemy_color = if is_alliance { theme::EMPIRE_RED } else { theme::ALLIANCE_BLUE };

                        ui.add_sized(
                            [200.0, 12.0],
                            ProgressBar::new(*our_pop)
                                .text(format!("{:.0}%", our_pop * 100.0))
                                .fill(our_color),
                        );
                        ui.add_sized(
                            [200.0, 12.0],
                            ProgressBar::new(*enemy_pop)
                                .text(format!("{:.0}%", enemy_pop * 100.0))
                                .fill(enemy_color),
                        );

                        ui.add_space(4.0);
                    }
                });

            // ── Character betrayal risk ──────────────────────────────
            ui.add_space(8.0);
            ui.separator();
            ui.label(
                RichText::new("BETRAYAL RISK")
                    .color(theme::GOLD_DIM)
                    .size(10.0)
                    .strong(),
            );

            let mut at_risk_chars: Vec<(&str, u32)> = Vec::new();
            for (_, c) in world.characters.iter() {
                let owns = if is_alliance { c.is_alliance } else { c.is_empire };
                if !owns || c.is_captive {
                    continue;
                }
                if c.is_unable_to_betray {
                    continue;
                }
                if c.loyalty.base < 40 {
                    at_risk_chars.push((&c.name, c.loyalty.base));
                }
            }
            at_risk_chars.sort_by_key(|(_, l)| *l);

            if at_risk_chars.is_empty() {
                ui.label(
                    RichText::new("No characters at risk of betrayal.")
                        .color(theme::TEXT_DISABLED)
                        .size(10.0),
                );
            } else {
                for (name, loyalty) in &at_risk_chars {
                    let risk_color = if *loyalty < 20 {
                        theme::DANGER_RED
                    } else {
                        theme::WARNING_AMBER
                    };
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(*name)
                                .color(theme::TEXT_PRIMARY)
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new(format!("Loyalty: {}", loyalty))
                                .color(risk_color)
                                .size(10.0),
                        );
                    });
                }
            }
        });

    None
}
