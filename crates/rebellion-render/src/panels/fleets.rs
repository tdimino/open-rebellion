//! Fleets panel — fleet roster with location, ship composition, and characters.
//!
//! Rendered as a left-side egui panel. Lists all fleets belonging to the player
//! faction. Clicking a fleet expands a detail row showing capital ships, fighter
//! squadrons, and assigned characters. A "Go to System" button emits
//! `PanelAction::FocusFleetSystem` so the galaxy map can pan to the fleet.

use egui_macroquad::egui::{self, Color32, RichText, ScrollArea};
use rebellion_core::ids::{CharacterKey, FleetKey};
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::GameWorld;

use super::PanelAction;

// ---------------------------------------------------------------------------
// FleetsState
// ---------------------------------------------------------------------------

/// Mutable state for the fleets panel.
#[derive(Debug, Clone, Default)]
pub struct FleetsState {
    /// The fleet currently expanded to show details.
    pub expanded_fleet: Option<FleetKey>,
    /// If set, the character being dragged to a fleet assignment.
    pub pending_assign: Option<CharacterKey>,
}

// ---------------------------------------------------------------------------
// draw_fleets
// ---------------------------------------------------------------------------

/// Render the fleet roster as a left-side egui panel.
///
/// `player_faction` filters which fleets are shown.
pub fn draw_fleets(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut FleetsState,
    player_faction: MissionFaction,
) -> Option<PanelAction> {
    let mut action = None;

    egui::SidePanel::left("fleets_panel")
        .min_width(280.0)
        .max_width(360.0)
        .show(ctx, |ui| {
            ui.heading("Fleets");
            ui.separator();

            let total: usize = world
                .fleets
                .values()
                .filter(|f| faction_matches(f.is_alliance, player_faction))
                .count();
            ui.label(
                RichText::new(format!("{total} fleet(s)"))
                    .color(Color32::from_gray(160))
                    .small(),
            );
            ui.add_space(4.0);

            ScrollArea::vertical().show(ui, |ui| {
                for (fleet_key, fleet) in &world.fleets {
                    if !faction_matches(fleet.is_alliance, player_faction) {
                        continue;
                    }

                    // Resolve system name for display.
                    let system_name = world
                        .systems
                        .get(fleet.location)
                        .map(|s| s.name.as_str())
                        .unwrap_or("Unknown");

                    let ship_count: u32 = fleet.capital_ships.iter().map(|e| e.count).sum();
                    let fighter_count: u32 = fleet.fighters.iter().map(|e| e.count).sum();

                    let is_expanded = state.expanded_fleet == Some(fleet_key);

                    // ── Fleet header row ──────────────────────────────────────
                    ui.horizontal(|ui| {
                        let toggle = if is_expanded { "▼" } else { "▶" };
                        if ui.small_button(toggle).clicked() {
                            state.expanded_fleet = if is_expanded {
                                None
                            } else {
                                Some(fleet_key)
                            };
                        }

                        ui.colored_label(
                            player_fleet_color(player_faction),
                            format!("Fleet @ {system_name}"),
                        );

                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "{ship_count} ship(s) · {fighter_count} sqd"
                                    ))
                                    .small()
                                    .color(Color32::from_gray(140)),
                                );
                            },
                        );
                    });

                    if is_expanded {
                        // ── Expanded detail ───────────────────────────────────
                        ui.indent("fleet_detail", |ui| {
                            // Capital ships.
                            if !fleet.capital_ships.is_empty() {
                                ui.label(
                                    RichText::new("Capital Ships")
                                        .color(Color32::from_gray(160))
                                        .small()
                                        .strong(),
                                );
                                for entry in &fleet.capital_ships {
                                    let class_name = world
                                        .capital_ship_classes
                                        .get(entry.class)
                                        .map(|c| c.name.as_str())
                                        .unwrap_or("Unknown");
                                    ui.label(
                                        RichText::new(format!(
                                            "  × {} — {}",
                                            entry.count, class_name
                                        ))
                                        .small(),
                                    );
                                }
                            }

                            // Fighter squadrons.
                            if !fleet.fighters.is_empty() {
                                ui.add_space(2.0);
                                ui.label(
                                    RichText::new("Fighter Squadrons")
                                        .color(Color32::from_gray(160))
                                        .small()
                                        .strong(),
                                );
                                for entry in &fleet.fighters {
                                    let class_name = world
                                        .fighter_classes
                                        .get(entry.class)
                                        .map(|c| c.name.as_str())
                                        .unwrap_or("Unknown");
                                    ui.label(
                                        RichText::new(format!(
                                            "  × {} — {}",
                                            entry.count, class_name
                                        ))
                                        .small(),
                                    );
                                }
                            }

                            // Assigned characters.
                            if !fleet.characters.is_empty() {
                                ui.add_space(2.0);
                                ui.label(
                                    RichText::new("Officers")
                                        .color(Color32::from_gray(160))
                                        .small()
                                        .strong(),
                                );
                                for &char_key in &fleet.characters {
                                    let char_name = world
                                        .characters
                                        .get(char_key)
                                        .map(|c| c.name.as_str())
                                        .unwrap_or("Unknown");
                                    ui.label(
                                        RichText::new(format!("  {char_name}")).small(),
                                    );
                                }
                            }

                            // Actions.
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                if ui.small_button("Go to System").clicked() {
                                    action = Some(PanelAction::FocusFleetSystem(fleet.location));
                                }
                            });
                        });

                        ui.add_space(4.0);
                    }

                    ui.separator();
                }
            });
        });

    action
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn faction_matches(is_alliance: bool, player: MissionFaction) -> bool {
    match player {
        MissionFaction::Alliance => is_alliance,
        MissionFaction::Empire => !is_alliance,
    }
}

fn player_fleet_color(faction: MissionFaction) -> Color32 {
    match faction {
        MissionFaction::Alliance => Color32::from_rgb(100, 160, 255),
        MissionFaction::Empire => Color32::from_rgb(220, 80, 80),
    }
}
