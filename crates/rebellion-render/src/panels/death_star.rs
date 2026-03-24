//! Death Star control panel — construction progress, fire order, target selection.
//!
//! Empire players see construction countdown, fleet location, and fire button.
//! Alliance players see detection status and interception info.
//! Returns `PanelAction::FireDeathStar` / `MoveDeathStar`.

use egui_macroquad::egui::{self, Color32, ProgressBar, RichText};
use rebellion_core::death_star::{DeathStarState, DEATH_STAR_CONSTRUCTION_TICKS};
use rebellion_core::ids::SystemKey;
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::GameWorld;

use crate::theme;
use super::PanelAction;

/// Draw the Death Star control panel as a left-side egui panel.
pub fn draw_death_star(
    ctx: &egui::Context,
    world: &GameWorld,
    ds_state: &DeathStarState,
    player_faction: MissionFaction,
) -> Option<PanelAction> {
    let mut action = None;
    let is_empire = player_faction == MissionFaction::Empire;

    egui::SidePanel::left("death_star_panel")
        .min_width(280.0)
        .max_width(340.0)
        .show(ctx, |ui| {
            let title = if is_empire { "Death Star Command" } else { "Death Star Threat" };
            ui.heading(RichText::new(title).color(theme::DANGER_RED));
            ui.separator();

            if is_empire {
                draw_empire_view(ui, world, ds_state, &mut action);
            } else {
                draw_alliance_view(ui, world, ds_state);
            }
        });

    action
}

/// Empire view: construction status, location, fire button.
fn draw_empire_view(
    ui: &mut egui::Ui,
    world: &GameWorld,
    ds_state: &DeathStarState,
    action: &mut Option<PanelAction>,
) {
    // ── Construction ─────────────────────────────────────────────
    if let Some(construction) = &ds_state.under_construction {
        ui.label(
            RichText::new("UNDER CONSTRUCTION")
                .color(theme::WARNING_AMBER)
                .size(12.0)
                .strong(),
        );

        let sys_name = world
            .systems
            .get(construction.system)
            .map(|s| s.name.as_str())
            .unwrap_or("Unknown");
        ui.label(
            RichText::new(format!("Location: {}", sys_name))
                .color(theme::TEXT_PRIMARY)
                .size(11.0),
        );

        let total = DEATH_STAR_CONSTRUCTION_TICKS as f32;
        let remaining = construction.ticks_remaining as f32;
        let progress = 1.0 - (remaining / total);

        ui.add(
            ProgressBar::new(progress.clamp(0.0, 1.0))
                .text(format!(
                    "{:.0}% — {} days remaining",
                    progress * 100.0,
                    construction.ticks_remaining
                ))
                .fill(Color32::from_rgb(120, 40, 40)),
        );

        ui.add_space(8.0);
        ui.label(
            RichText::new("The station will be operational when construction completes.")
                .color(theme::TEXT_SECONDARY)
                .size(10.0),
        );
        return;
    }

    // ── Operational Death Star ────────────────────────────────────
    if let Some(fleet_key) = ds_state.death_star_fleet {
        if let Some(fleet) = world.fleets.get(fleet_key) {
            ui.label(
                RichText::new("FULLY OPERATIONAL")
                    .color(theme::SUCCESS_GREEN)
                    .size(12.0)
                    .strong(),
            );

            let sys_name = world
                .systems
                .get(fleet.location)
                .map(|s| s.name.as_str())
                .unwrap_or("Unknown");
            ui.label(
                RichText::new(format!("Location: {}", sys_name))
                    .color(theme::TEXT_PRIMARY)
                    .size(11.0),
            );

            let ship_count: u32 = fleet.capital_ships.iter().map(|e| e.count).sum();
            ui.label(
                RichText::new(format!("Escort: {} capital ships", ship_count))
                    .color(theme::TEXT_SECONDARY)
                    .size(10.0),
            );

            ui.add_space(8.0);
            ui.separator();

            // ── Target selection ─────────────────────────────────
            ui.label(
                RichText::new("SUPERLASER TARGETING")
                    .color(theme::GOLD_DIM)
                    .size(10.0)
                    .strong(),
            );

            // Find enemy systems at fleet location
            let current_system = world.systems.get(fleet.location);
            if let Some(system) = current_system {
                let is_enemy = match system.control {
                    rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => true,
                    _ => false,
                };

                if system.is_destroyed {
                    ui.label(
                        RichText::new("This system has already been destroyed.")
                            .color(theme::TEXT_DISABLED)
                            .size(11.0),
                    );
                } else if is_enemy {
                    ui.label(
                        RichText::new(format!("Target: {}", system.name))
                            .color(theme::DANGER_RED)
                            .size(12.0)
                            .strong(),
                    );
                    ui.label(
                        RichText::new("Firing will destroy this planet permanently.")
                            .color(theme::WARNING_AMBER)
                            .size(10.0),
                    );

                    ui.add_space(8.0);
                    if ui
                        .button(
                            RichText::new("FIRE SUPERLASER")
                                .color(theme::DANGER_RED)
                                .strong()
                                .size(14.0),
                        )
                        .clicked()
                    {
                        *action = Some(PanelAction::FireDeathStar {
                            system: fleet.location,
                        });
                    }
                } else {
                    ui.label(
                        RichText::new("No enemy system at current location.")
                            .color(theme::TEXT_DISABLED)
                            .size(11.0),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("Move the Death Star fleet to an enemy system to fire.")
                            .color(theme::TEXT_SECONDARY)
                            .size(10.0),
                    );
                }
            }

            // Nearby systems list for movement
            ui.add_space(8.0);
            ui.separator();
            ui.label(
                RichText::new("NEARBY SYSTEMS")
                    .color(theme::GOLD_DIM)
                    .size(10.0)
                    .strong(),
            );

            let mut nearby: Vec<(SystemKey, &str, f32)> = Vec::new();
            if let Some(cur_sys) = world.systems.get(fleet.location) {
                for (sk, sys) in world.systems.iter() {
                    if sk == fleet.location || sys.is_destroyed {
                        continue;
                    }
                    let dx = (sys.x as f32) - (cur_sys.x as f32);
                    let dy = (sys.y as f32) - (cur_sys.y as f32);
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < 500.0 {
                        nearby.push((sk, &sys.name, dist));
                    }
                }
            }
            nearby.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
            nearby.truncate(8);

            if nearby.is_empty() {
                ui.label(
                    RichText::new("No nearby systems")
                        .color(theme::TEXT_DISABLED)
                        .size(10.0),
                );
            } else {
                for (sk, name, dist) in &nearby {
                    ui.horizontal(|ui| {
                        if ui
                            .small_button(
                                RichText::new(format!("{} ({:.0})", name, dist))
                                    .color(theme::TEXT_PRIMARY)
                                    .size(10.0),
                            )
                            .clicked()
                        {
                            *action = Some(PanelAction::MoveDeathStar { system: *sk });
                        }
                    });
                }
            }
        } else {
            ui.label(
                RichText::new("Death Star fleet not found.")
                    .color(theme::DANGER_RED)
                    .size(11.0),
            );
        }
    } else {
        // No Death Star at all
        ui.add_space(20.0);
        ui.label(
            RichText::new("No Death Star in service.")
                .color(theme::TEXT_DISABLED),
        );
        ui.add_space(4.0);
        ui.label(
            RichText::new("Construct one at a system with an Advanced Shipyard.")
                .color(theme::TEXT_SECONDARY)
                .size(10.0),
        );
    }
}

/// Alliance view: threat detection, warning status.
fn draw_alliance_view(
    ui: &mut egui::Ui,
    world: &GameWorld,
    ds_state: &DeathStarState,
) {
    if ds_state.under_construction.is_some() {
        ui.label(
            RichText::new("INTELLIGENCE REPORTS")
                .color(theme::GOLD_DIM)
                .size(10.0)
                .strong(),
        );
        ui.label(
            RichText::new("The Empire is constructing a Death Star.")
                .color(theme::WARNING_AMBER)
                .size(12.0),
        );
        ui.add_space(4.0);
        ui.label(
            RichText::new("Send spies to locate the construction site.")
                .color(theme::TEXT_SECONDARY)
                .size(10.0),
        );
    } else if let Some(fleet_key) = ds_state.death_star_fleet {
        ui.label(
            RichText::new("THREAT ACTIVE")
                .color(theme::DANGER_RED)
                .size(12.0)
                .strong(),
        );

        if let Some(fleet) = world.fleets.get(fleet_key) {
            let sys_name = world
                .systems
                .get(fleet.location)
                .map(|s| s.name.as_str())
                .unwrap_or("Unknown");
            ui.label(
                RichText::new(format!("Last known location: {}", sys_name))
                    .color(theme::TEXT_PRIMARY)
                    .size(11.0),
            );
        }

        ui.add_space(4.0);
        ui.label(
            RichText::new("Sabotage missions can delay or destroy the Death Star.")
                .color(theme::TEXT_SECONDARY)
                .size(10.0),
        );
    } else {
        ui.add_space(20.0);
        ui.label(
            RichText::new("No Death Star threat detected.")
                .color(theme::TEXT_DISABLED),
        );
    }
}
