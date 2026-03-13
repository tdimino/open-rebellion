//! Manufacturing panel — per-system production queues.
//!
//! Shows all systems with active manufacturing facilities.  For each system the
//! player can see the current queue, add new items, cancel or prioritize
//! existing items.  Actions are returned as `PanelAction` variants so the
//! caller applies them to `ManufacturingState` without the panel needing a
//! mutable borrow.

use egui_macroquad::egui::{self, Color32, RichText, ScrollArea};
use rebellion_core::ids::SystemKey;
use rebellion_core::manufacturing::{BuildableKind, ManufacturingState};
use rebellion_core::missions::MissionFaction;
use rebellion_core::world::GameWorld;

use super::PanelAction;

// ---------------------------------------------------------------------------
// ManufacturingPanelState
// ---------------------------------------------------------------------------

/// Mutable UI state for the manufacturing panel.
#[derive(Debug, Clone, Default)]
pub struct ManufacturingPanelState {
    /// Which system's queue is currently expanded.
    pub expanded_system: Option<SystemKey>,
    /// Current selection in the "add to queue" combo for the expanded system.
    pub add_selection: AddSelection,
}

/// What the player has selected to add to the production queue.
#[derive(Debug, Clone, Default)]
pub enum AddSelection {
    #[default]
    None,
    CapitalShip(usize),   // index into a display list
    Fighter(usize),
}

// ---------------------------------------------------------------------------
// draw_manufacturing
// ---------------------------------------------------------------------------

/// Render the manufacturing panel as a left-side egui panel.
///
/// `mfg_state` is a read-only snapshot of the current queues — the panel never
/// mutates it directly.  All mutations are returned as `PanelAction`.
pub fn draw_manufacturing(
    ctx: &egui::Context,
    world: &GameWorld,
    mfg_state: &ManufacturingState,
    panel_state: &mut ManufacturingPanelState,
    player_faction: MissionFaction,
) -> Option<PanelAction> {
    let mut action = None;

    egui::SidePanel::left("manufacturing_panel")
        .min_width(300.0)
        .max_width(380.0)
        .show(ctx, |ui| {
            ui.heading("Manufacturing");
            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                for (sys_key, system) in &world.systems {
                    // Only show systems with manufacturing facilities.
                    if system.manufacturing_facilities.is_empty() {
                        continue;
                    }

                    // Popularity ownership check: show systems where the player
                    // faction is dominant (> 0.5).
                    let is_controlled = match player_faction {
                        MissionFaction::Alliance => {
                            system.popularity_alliance > system.popularity_empire
                        }
                        MissionFaction::Empire => {
                            system.popularity_empire > system.popularity_alliance
                        }
                    };
                    if !is_controlled {
                        continue;
                    }

                    let queue = mfg_state.queue(sys_key);
                    let queue_len = queue.map(|q| q.len()).unwrap_or(0);
                    let is_expanded = panel_state.expanded_system == Some(sys_key);

                    // ── System header row ─────────────────────────────────────
                    ui.horizontal(|ui| {
                        let toggle = if is_expanded { "▼" } else { "▶" };
                        if ui.small_button(toggle).clicked() {
                            panel_state.expanded_system = if is_expanded {
                                None
                            } else {
                                Some(sys_key)
                            };
                            panel_state.add_selection = AddSelection::None;
                        }

                        ui.label(RichText::new(&system.name).strong());

                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if queue_len > 0 {
                                    // Active item progress bar.
                                    if let Some(q) = queue {
                                        if let Some(active) = q.active() {
                                            let fraction = active.progress_fraction();
                                            draw_mini_progress(ui, fraction);
                                        }
                                    }
                                    ui.label(
                                        RichText::new(format!("{queue_len} queued"))
                                            .small()
                                            .color(Color32::from_gray(160)),
                                    );
                                } else {
                                    ui.label(
                                        RichText::new("idle")
                                            .small()
                                            .color(Color32::from_gray(100)),
                                    );
                                }
                            },
                        );
                    });

                    if is_expanded {
                        ui.indent("mfg_detail", |ui| {
                            // ── Current queue ─────────────────────────────────
                            if let Some(q) = queue {
                                let items: Vec<_> = q.items().iter().collect();
                                if items.is_empty() {
                                    ui.label(
                                        RichText::new("Queue empty.").small().color(Color32::from_gray(120)),
                                    );
                                } else {
                                    ui.label(
                                        RichText::new("Queue:")
                                            .small()
                                            .strong()
                                            .color(Color32::from_gray(180)),
                                    );

                                    for (idx, item) in items.iter().enumerate() {
                                        let kind_label = buildable_label(item.kind, world);
                                        let is_active = idx == 0;

                                        ui.horizontal(|ui| {
                                            if is_active {
                                                // Progress bar for the active item.
                                                let frac = item.progress_fraction();
                                                let bar_w = 60.0;
                                                let (rect, _) = ui.allocate_exact_size(
                                                    egui::vec2(bar_w, 10.0),
                                                    egui::Sense::hover(),
                                                );
                                                ui.painter().rect_filled(
                                                    rect, 2.0, Color32::from_gray(40),
                                                );
                                                ui.painter().rect_filled(
                                                    egui::Rect::from_min_size(
                                                        rect.min,
                                                        egui::vec2(bar_w * frac, rect.height()),
                                                    ),
                                                    2.0,
                                                    Color32::from_rgb(100, 200, 100),
                                                );
                                            } else {
                                                ui.label(
                                                    RichText::new(format!("  {idx}."))
                                                        .small()
                                                        .color(Color32::from_gray(120)),
                                                );
                                            }

                                            ui.label(
                                                RichText::new(format!(
                                                    "{kind_label} ({} days left)",
                                                    item.ticks_remaining
                                                ))
                                                .small(),
                                            );

                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    if ui
                                                        .small_button(
                                                            RichText::new("✕")
                                                                .color(Color32::from_rgb(200, 80, 80)),
                                                        )
                                                        .clicked()
                                                    {
                                                        action = Some(PanelAction::CancelQueueItem {
                                                            system: sys_key,
                                                            index: idx,
                                                        });
                                                    }
                                                    if idx > 0
                                                        && ui.small_button("↑").clicked()
                                                    {
                                                        action =
                                                            Some(PanelAction::PrioritizeQueueItem {
                                                                system: sys_key,
                                                                index: idx,
                                                            });
                                                    }
                                                },
                                            );
                                        });
                                    }
                                }
                            }

                            // ── Add to queue ──────────────────────────────────
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("Add to queue:")
                                    .small()
                                    .strong()
                                    .color(Color32::from_gray(180)),
                            );

                            // Collect buildable capital ships for this faction.
                            let ships: Vec<_> = world
                                .capital_ship_classes
                                .iter()
                                .filter(|(_, c)| match player_faction {
                                    MissionFaction::Alliance => c.is_alliance,
                                    MissionFaction::Empire => c.is_empire,
                                })
                                .collect();

                            let fighters: Vec<_> = world
                                .fighter_classes
                                .iter()
                                .filter(|(_, c)| match player_faction {
                                    MissionFaction::Alliance => c.is_alliance,
                                    MissionFaction::Empire => c.is_empire,
                                })
                                .collect();

                            // Ships combo.
                            if !ships.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Ship:").small());
                                    let selected_ship_name = match &panel_state.add_selection {
                                        AddSelection::CapitalShip(i) => {
                                            ships.get(*i).map(|(_, c)| c.name.as_str()).unwrap_or("—")
                                        }
                                        _ => "—",
                                    };
                                    egui::ComboBox::from_id_salt(format!("ship_combo_{:?}", sys_key))
                                        .selected_text(selected_ship_name)
                                        .show_ui(ui, |ui| {
                                            for (i, (_, class)) in ships.iter().enumerate() {
                                                let sel = matches!(
                                                    &panel_state.add_selection,
                                                    AddSelection::CapitalShip(j) if *j == i
                                                );
                                                if ui
                                                    .selectable_label(
                                                        sel,
                                                        format!(
                                                            "{} ({}mat, {}d)",
                                                            class.name,
                                                            class.refined_material_cost,
                                                            class.research_difficulty,
                                                        ),
                                                    )
                                                    .clicked()
                                                {
                                                    panel_state.add_selection =
                                                        AddSelection::CapitalShip(i);
                                                }
                                            }
                                        });

                                    if ui.small_button("Enqueue").clicked() {
                                        if let AddSelection::CapitalShip(i) =
                                            &panel_state.add_selection
                                        {
                                            if let Some((class_key, class)) = ships.get(*i) {
                                                action = Some(PanelAction::Enqueue {
                                                    system: sys_key,
                                                    kind: BuildableKind::CapitalShip(*class_key),
                                                    cost: class.refined_material_cost,
                                                    ticks: class.research_difficulty.max(1),
                                                });
                                            }
                                        }
                                    }
                                });
                            }

                            // Fighters combo.
                            if !fighters.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Fighter:").small());
                                    let selected_ftr_name = match &panel_state.add_selection {
                                        AddSelection::Fighter(i) => {
                                            fighters.get(*i).map(|(_, c)| c.name.as_str()).unwrap_or("—")
                                        }
                                        _ => "—",
                                    };
                                    egui::ComboBox::from_id_salt(format!("ftr_combo_{:?}", sys_key))
                                        .selected_text(selected_ftr_name)
                                        .show_ui(ui, |ui| {
                                            for (i, (_, class)) in fighters.iter().enumerate() {
                                                let sel = matches!(
                                                    &panel_state.add_selection,
                                                    AddSelection::Fighter(j) if *j == i
                                                );
                                                if ui
                                                    .selectable_label(
                                                        sel,
                                                        format!(
                                                            "{} ({}mat)",
                                                            class.name,
                                                            class.refined_material_cost,
                                                        ),
                                                    )
                                                    .clicked()
                                                {
                                                    panel_state.add_selection =
                                                        AddSelection::Fighter(i);
                                                }
                                            }
                                        });

                                    if ui.small_button("Enqueue").clicked() {
                                        if let AddSelection::Fighter(i) =
                                            &panel_state.add_selection
                                        {
                                            if let Some((class_key, class)) = fighters.get(*i) {
                                                action = Some(PanelAction::Enqueue {
                                                    system: sys_key,
                                                    kind: BuildableKind::Fighter(*class_key),
                                                    cost: class.refined_material_cost,
                                                    // Fighters have no research_difficulty; use
                                                    // material cost / 5 + 5 as a build-time proxy.
                                                    ticks: (class.refined_material_cost / 5 + 5).max(1),
                                                });
                                            }
                                        }
                                    }
                                });
                            }
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

/// Human-readable label for a `BuildableKind`.
fn buildable_label(kind: BuildableKind, world: &GameWorld) -> String {
    match kind {
        BuildableKind::CapitalShip(k) => world
            .capital_ship_classes
            .get(k)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "Capital Ship".into()),
        BuildableKind::Fighter(k) => world
            .fighter_classes
            .get(k)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "Fighter".into()),
        BuildableKind::Troop(_) => "Troop Regiment".into(),
        BuildableKind::DefenseFacility(_) => "Defense Facility".into(),
        BuildableKind::ManufacturingFacility(_) => "Manufacturing Facility".into(),
        BuildableKind::ProductionFacility(_) => "Production Facility".into(),
    }
}

/// Tiny inline progress bar drawn right-to-left inside a horizontal layout.
fn draw_mini_progress(ui: &mut egui::Ui, fraction: f32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(40.0, 8.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 2.0, Color32::from_gray(40));
    ui.painter().rect_filled(
        egui::Rect::from_min_size(
            rect.min,
            egui::vec2(rect.width() * fraction.clamp(0.0, 1.0), rect.height()),
        ),
        2.0,
        Color32::from_rgb(100, 200, 100),
    );
}
