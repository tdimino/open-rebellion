//! Missions panel: the player's in-flight missions with progress bars and a
//! cancel button.
//!
//! A mission starts from the object pop-up menu (`crate::object_menu`) and the
//! galaxy view's targeting mode (`crate::targeting`), which open the original
//! mission dialog (`crate::mission_dialog`).
//!
//! Actions:
//! - `PanelAction::CancelMission(id)` when the player cancels an active one.

use egui_macroquad::egui::{self, Color32, RichText, ScrollArea};
use rebellion_core::missions::{ActiveMission, MissionFaction, MissionKind, MissionState};
use rebellion_core::world::GameWorld;

use super::PanelAction;

// ---------------------------------------------------------------------------
// draw_missions
// ---------------------------------------------------------------------------

/// Render the missions panel as a left-side egui panel.
///
/// `mission_state` is read-only — mutations come back as `PanelAction`.
/// `today` is the current game day.
pub fn draw_missions(
    ctx: &egui::Context,
    world: &GameWorld,
    mission_state: &MissionState,
    player_faction: MissionFaction,
    today: u64,
) -> Option<PanelAction> {
    let mut action = None;

    egui::SidePanel::left("missions_panel")
        .min_width(300.0)
        .max_width(400.0)
        .show(ctx, |ui| {
            ui.heading("Missions");
            ui.separator();

            draw_active_missions(ui, world, mission_state, player_faction, today, &mut action);
        });

    action
}

// ---------------------------------------------------------------------------
// Active missions
// ---------------------------------------------------------------------------

fn draw_active_missions(
    ui: &mut egui::Ui,
    world: &GameWorld,
    mission_state: &MissionState,
    player_faction: MissionFaction,
    today: u64,
    action: &mut Option<PanelAction>,
) {
    let faction_missions: Vec<&ActiveMission> = mission_state
        .missions()
        .iter()
        .filter(|m| m.faction == player_faction)
        .collect();

    if faction_missions.is_empty() {
        ui.label(
            RichText::new("No active missions.")
                .color(Color32::from_gray(120))
                .small(),
        );
        return;
    }

    ScrollArea::vertical().show(ui, |ui| {
        for mission in faction_missions {
            ui.horizontal(|ui| {
                // Kind badge.
                let (kind_label, kind_color) = mission_kind_display(mission.kind);
                ui.colored_label(kind_color, kind_label);

                // Team lead, plus how many more members and decoys go along.
                let commander_name = mission
                    .lead_character(world)
                    .and_then(|key| world.characters.get(key))
                    .map_or("Unknown", |c| c.name.as_str());
                let others = mission.team.len().saturating_sub(1);
                let label = match (others, mission.decoys.len()) {
                    (0, 0) => commander_name.to_string(),
                    (0, d) => format!("{commander_name} (+{d} decoys)"),
                    (t, 0) => format!("{commander_name} +{t}"),
                    (t, d) => format!("{commander_name} +{t} (+{d} decoys)"),
                };
                ui.label(RichText::new(label).small());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .small_button(RichText::new("Cancel").color(Color32::from_rgb(200, 80, 80)))
                        .clicked()
                    {
                        *action = Some(PanelAction::CancelMission(mission.id));
                    }
                });
            });

            // Target system + progress bar.
            let target_name = world
                .systems
                .get(mission.target_system)
                .map_or("Unknown", |s| s.name.as_str());

            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.label(
                    RichText::new(format!("→ {target_name}"))
                        .small()
                        .color(Color32::from_gray(160)),
                );

                let last_arrival = mission_state.last_arrival(mission.id);
                let frac = mission.wait_fraction(today, last_arrival);
                let (rect, _) = ui.allocate_exact_size(egui::vec2(80.0, 8.0), egui::Sense::hover());
                ui.painter().rect_filled(rect, 2.0, Color32::from_gray(40));
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        rect.min,
                        egui::vec2(rect.width() * frac, rect.height()),
                    ),
                    2.0,
                    Color32::from_rgb(100, 160, 255),
                );

                let wait = mission
                    .days_left(today, last_arrival)
                    .map_or_else(String::new, |days| format!("{days} days"));
                ui.label(RichText::new(wait).small().color(Color32::from_gray(140)));
            });

            ui.separator();
        }
    });
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn mission_kind_display(kind: MissionKind) -> (&'static str, Color32) {
    match kind {
        MissionKind::Diplomacy => ("[DIPLO]", Color32::from_rgb(255, 220, 80)),
        MissionKind::Recruitment => ("[RECRT]", Color32::from_rgb(100, 220, 180)),
        MissionKind::Sabotage => ("[SBTGE]", Color32::from_rgb(220, 100, 60)),
        MissionKind::Assassination => ("[ASSN]", Color32::from_rgb(200, 50, 50)),
        MissionKind::Espionage => ("[ESPI]", Color32::from_rgb(160, 120, 220)),
        MissionKind::Rescue => ("[RESC]", Color32::from_rgb(80, 180, 255)),
        MissionKind::Abduction => ("[ABDC]", Color32::from_rgb(220, 160, 60)),
        MissionKind::InciteUprising => ("[INCT]", Color32::from_rgb(255, 140, 40)),
        MissionKind::SubdueUprising => ("[SUBD]", Color32::from_rgb(100, 200, 100)),
        MissionKind::DeathStarSabotage => ("[DSSB]", Color32::from_rgb(255, 60, 60)),
        MissionKind::Autoscrap => ("[AUTO]", Color32::from_rgb(120, 120, 120)),
        MissionKind::Move => ("[MOVE]", Color32::from_rgb(120, 120, 120)),
    }
}
