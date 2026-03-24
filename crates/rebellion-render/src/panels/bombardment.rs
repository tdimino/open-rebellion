//! Bombardment targeting panel — select fleet and target for orbital bombardment.
//!
//! Shows fleets with bombardment capability at the selected system,
//! damage forecast, and a "Fire" confirmation button.
//! Returns `PanelAction::OrderBombardment { fleet, system }`.

use egui_macroquad::egui::{self, RichText, ScrollArea};
use rebellion_core::ids::FleetKey;
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::GameWorld;

use crate::theme;
use super::PanelAction;

/// Mutable UI state for the bombardment panel.
#[derive(Debug, Clone, Default)]
pub struct BombardmentPanelState {
    /// Selected fleet for bombardment.
    pub selected_fleet: Option<FleetKey>,
}

/// Draw the bombardment targeting panel as a left-side egui panel.
pub fn draw_bombardment(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut BombardmentPanelState,
    player_faction: MissionFaction,
) -> Option<PanelAction> {
    let mut action = None;
    let is_alliance = player_faction == MissionFaction::Alliance;

    egui::SidePanel::left("bombardment_panel")
        .min_width(280.0)
        .max_width(340.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Orbital Bombardment").color(theme::DANGER_RED));
            ui.separator();

            // Find player fleets with capital ships (bombardment capability).
            let capable_fleets: Vec<(FleetKey, &rebellion_core::world::Fleet)> = world
                .fleets
                .iter()
                .filter(|(_, f)| {
                    let owns = if is_alliance { f.is_alliance } else { !f.is_alliance };
                    owns && !f.capital_ships.is_empty()
                })
                .collect();

            if capable_fleets.is_empty() {
                ui.add_space(20.0);
                ui.label(
                    RichText::new("No fleets with bombardment capability.")
                        .color(theme::TEXT_DISABLED),
                );
                return;
            }

            ui.label(
                RichText::new(format!("{} capable fleet(s)", capable_fleets.len()))
                    .color(theme::TEXT_SECONDARY)
                    .size(11.0),
            );
            ui.add_space(8.0);

            // ── Fleet selection ──────────────────────────────────────────
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (fleet_key, fleet) in &capable_fleets {
                        let sys_name = world
                            .systems
                            .get(fleet.location)
                            .map(|s| s.name.as_str())
                            .unwrap_or("Unknown");

                        let ship_count: u32 = fleet.capital_ships.iter().map(|e| e.count).sum();
                        let is_selected = state.selected_fleet == Some(*fleet_key);

                        ui.group(|ui| {
                            let label_color = if is_selected { theme::GOLD } else { theme::TEXT_PRIMARY };
                            ui.horizontal(|ui| {
                                if ui.selectable_label(is_selected,
                                    RichText::new(format!("Fleet @ {}", sys_name))
                                        .color(label_color)
                                        .strong(),
                                ).clicked() {
                                    state.selected_fleet = if is_selected { None } else { Some(*fleet_key) };
                                }
                                ui.label(
                                    RichText::new(format!("{} ships", ship_count))
                                        .color(theme::TEXT_SECONDARY)
                                        .size(10.0),
                                );
                            });

                            // Ship class breakdown when selected
                            if is_selected {
                                for entry in &fleet.capital_ships {
                                    if let Some(class) = world.capital_ship_classes.get(entry.class) {
                                        ui.label(
                                            RichText::new(format!("  {} ×{}", class.name, entry.count))
                                                .color(theme::TEXT_SECONDARY)
                                                .size(10.0),
                                        );
                                    }
                                }

                                // Find enemy systems at this location for targeting
                                let sys = world.systems.get(fleet.location);
                                if let Some(system) = sys {
                                    let is_enemy = match system.control {
                                        rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Alliance) => !is_alliance,
                                        rebellion_core::world::ControlKind::Controlled(rebellion_core::dat::Faction::Empire) => is_alliance,
                                        _ => false,
                                    };

                                    ui.add_space(4.0);
                                    if system.is_destroyed {
                                        ui.label(
                                            RichText::new("System already destroyed")
                                                .color(theme::TEXT_DISABLED)
                                                .size(10.0),
                                        );
                                    } else if is_enemy {
                                        // Damage forecast
                                        let def_fac = system.defense_facilities.len();
                                        ui.label(
                                            RichText::new(format!("Target: {} ({}def)", system.name, def_fac))
                                                .color(theme::WARNING_AMBER)
                                                .size(11.0),
                                        );
                                        ui.label(
                                            RichText::new("Bombardment will damage facilities and lower enemy support.")
                                                .color(theme::TEXT_SECONDARY)
                                                .size(10.0),
                                        );

                                        ui.add_space(4.0);
                                        if ui.button(
                                            RichText::new("FIRE")
                                                .color(theme::DANGER_RED)
                                                .strong()
                                                .size(14.0),
                                        ).clicked() {
                                            action = Some(PanelAction::OrderBombardment {
                                                fleet: *fleet_key,
                                                system: fleet.location,
                                            });
                                        }
                                    } else {
                                        ui.label(
                                            RichText::new("No enemy system at fleet location")
                                                .color(theme::TEXT_DISABLED)
                                                .size(10.0),
                                        );
                                    }
                                }
                            }
                        });
                        ui.add_space(4.0);
                    }
                });
        });

    action
}
