//! Mission dispatch panel — pick commander, mission type, target, show probability.
//!
//! Two sub-views:
//! 1. **Active missions list** — shows in-flight missions with progress bars and
//!    a cancel button.
//! 2. **Dispatch form** — pick a character, mission kind, and target system,
//!    preview estimated success probability, then confirm.
//!
//! Actions:
//! - `PanelAction::DispatchMission` when the player confirms a new mission.
//! - `PanelAction::CancelMission(id)` when the player cancels an active one.

use egui_macroquad::egui::{self, Color32, RichText, ScrollArea};
use rebellion_core::ids::{CharacterKey, SystemKey};
use rebellion_core::missions::{
    clamp_prob, quadratic_prob, ActiveMission, MissionFaction, MissionKind, MissionState,
};
use rebellion_core::world::GameWorld;

use super::PanelAction;

// ---------------------------------------------------------------------------
// MissionsPanelState
// ---------------------------------------------------------------------------

/// Mutable UI state for the missions panel.
#[derive(Debug, Clone, Default)]
pub struct MissionsPanelState {
    /// Which tab is shown: active missions vs. dispatch form.
    pub tab: MissionsTab,

    // ── Dispatch form state ────────────────────────────────────────────────
    pub selected_commander: Option<CharacterKey>,
    pub selected_kind: Option<MissionKind>,
    pub selected_target: Option<SystemKey>,

    /// Pre-supplied [0,1) roll used if the player dispatches.
    /// Refreshed each frame so consecutive dispatches get different durations.
    pub pending_duration_roll: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MissionsTab {
    #[default]
    Active,
    Dispatch,
}

// ---------------------------------------------------------------------------
// draw_missions
// ---------------------------------------------------------------------------

/// Render the missions panel as a left-side egui panel.
///
/// `mission_state` is read-only — mutations come back as `PanelAction`.
/// `duration_roll` should be a fresh random value each frame from the caller.
pub fn draw_missions(
    ctx: &egui::Context,
    world: &GameWorld,
    mission_state: &MissionState,
    panel_state: &mut MissionsPanelState,
    player_faction: MissionFaction,
    duration_roll: f64,
) -> Option<PanelAction> {
    panel_state.pending_duration_roll = duration_roll;
    let mut action = None;

    egui::SidePanel::left("missions_panel")
        .min_width(300.0)
        .max_width(400.0)
        .show(ctx, |ui| {
            ui.heading("Missions");
            ui.separator();

            // ── Tab bar ───────────────────────────────────────────────────────
            ui.horizontal(|ui| {
                let active_count = mission_state
                    .missions()
                    .iter()
                    .filter(|m| m.faction == player_faction)
                    .count();

                let active_label = format!("Active ({})", active_count);
                if ui
                    .selectable_label(panel_state.tab == MissionsTab::Active, active_label)
                    .clicked()
                {
                    panel_state.tab = MissionsTab::Active;
                }
                if ui
                    .selectable_label(panel_state.tab == MissionsTab::Dispatch, "Dispatch")
                    .clicked()
                {
                    panel_state.tab = MissionsTab::Dispatch;
                }
            });

            ui.separator();

            match panel_state.tab {
                MissionsTab::Active => {
                    draw_active_tab(ui, world, mission_state, player_faction, &mut action);
                }
                MissionsTab::Dispatch => {
                    draw_dispatch_tab(ui, world, panel_state, player_faction, &mut action);
                }
            }
        });

    action
}

// ---------------------------------------------------------------------------
// Active missions tab
// ---------------------------------------------------------------------------

fn draw_active_tab(
    ui: &mut egui::Ui,
    world: &GameWorld,
    mission_state: &MissionState,
    player_faction: MissionFaction,
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

                // Commander name.
                let commander_name = world
                    .characters
                    .get(mission.character)
                    .map(|c| c.name.as_str())
                    .unwrap_or("Unknown");
                ui.label(RichText::new(commander_name).small());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .small_button(
                            RichText::new("Cancel").color(Color32::from_rgb(200, 80, 80)),
                        )
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
                .map(|s| s.name.as_str())
                .unwrap_or("Unknown");

            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.label(
                    RichText::new(format!("→ {target_name}"))
                        .small()
                        .color(Color32::from_gray(160)),
                );

                let frac = mission.progress_fraction();
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(80.0, 8.0), egui::Sense::hover());
                ui.painter().rect_filled(rect, 2.0, Color32::from_gray(40));
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        rect.min,
                        egui::vec2(rect.width() * frac, rect.height()),
                    ),
                    2.0,
                    Color32::from_rgb(100, 160, 255),
                );

                ui.label(
                    RichText::new(format!("{} days", mission.ticks_remaining))
                        .small()
                        .color(Color32::from_gray(140)),
                );
            });

            ui.separator();
        }
    });
}

// ---------------------------------------------------------------------------
// Dispatch form tab
// ---------------------------------------------------------------------------

fn draw_dispatch_tab(
    ui: &mut egui::Ui,
    world: &GameWorld,
    panel_state: &mut MissionsPanelState,
    player_faction: MissionFaction,
    action: &mut Option<PanelAction>,
) {
    // ── Commander selection ───────────────────────────────────────────────────
    ui.label(RichText::new("Commander:").color(Color32::from_gray(180)).small().strong());

    let available_commanders: Vec<_> = world
        .characters
        .iter()
        .filter(|(_, c)| {
            c.can_be_commander
                && match player_faction {
                    MissionFaction::Alliance => c.is_alliance,
                    MissionFaction::Empire => c.is_empire,
                }
        })
        .collect();

    let commander_name = panel_state
        .selected_commander
        .and_then(|k| world.characters.get(k))
        .map(|c| c.name.as_str())
        .unwrap_or("— Select —");

    egui::ComboBox::from_id_salt("dispatch_commander")
        .selected_text(commander_name)
        .show_ui(ui, |ui| {
            for (key, character) in &available_commanders {
                let selected = panel_state.selected_commander == Some(*key);
                let label = format!(
                    "{} (dip:{} esp:{})",
                    character.name, character.diplomacy.base, character.espionage.base
                );
                if ui.selectable_label(selected, label).clicked() {
                    panel_state.selected_commander = Some(*key);
                }
            }
        });

    ui.add_space(6.0);

    // ── Mission type selection ────────────────────────────────────────────────
    ui.label(RichText::new("Mission Type:").color(Color32::from_gray(180)).small().strong());
    ui.horizontal(|ui| {
        let is_diplo = panel_state.selected_kind == Some(MissionKind::Diplomacy);
        let is_recrt = panel_state.selected_kind == Some(MissionKind::Recruitment);

        if ui.selectable_label(is_diplo, "Diplomacy").clicked() {
            panel_state.selected_kind = Some(MissionKind::Diplomacy);
        }
        if ui.selectable_label(is_recrt, "Recruitment").clicked() {
            panel_state.selected_kind = Some(MissionKind::Recruitment);
        }
    });

    ui.add_space(6.0);

    // ── Target system selection ───────────────────────────────────────────────
    ui.label(RichText::new("Target System:").color(Color32::from_gray(180)).small().strong());

    let target_name = panel_state
        .selected_target
        .and_then(|k| world.systems.get(k))
        .map(|s| s.name.as_str())
        .unwrap_or("— Select —");

    egui::ComboBox::from_id_salt("dispatch_target")
        .selected_text(target_name)
        .show_ui(ui, |ui| {
            let mut sorted_systems: Vec<_> = world.systems.iter().collect();
            sorted_systems.sort_by_key(|(_, s)| s.name.as_str());
            for (key, system) in sorted_systems {
                let selected = panel_state.selected_target == Some(key);
                if ui.selectable_label(selected, &system.name).clicked() {
                    panel_state.selected_target = Some(key);
                }
            }
        });

    ui.add_space(8.0);
    ui.separator();

    // ── Success probability preview ───────────────────────────────────────────
    if let (Some(char_key), Some(kind)) =
        (panel_state.selected_commander, panel_state.selected_kind)
    {
        if let Some(character) = world.characters.get(char_key) {
            let skill = match kind {
                MissionKind::Diplomacy => character.diplomacy,
                MissionKind::Recruitment => character.leadership,
            };
            let score = skill.base as f64 + skill.variance as f64 * 0.5;
            let (a, b, c) = kind.coefficients();
            let raw = quadratic_prob(score, a, b, c);
            let prob = clamp_prob(raw, kind.min_success_prob(), kind.max_success_prob());

            let prob_color = if prob >= 70.0 {
                Color32::from_rgb(100, 220, 100)
            } else if prob >= 40.0 {
                Color32::from_rgb(220, 200, 60)
            } else {
                Color32::from_rgb(220, 100, 60)
            };

            ui.horizontal(|ui| {
                ui.label(RichText::new("Est. success:").small().color(Color32::from_gray(160)));
                ui.label(
                    RichText::new(format!("{:.0}%", prob))
                        .strong()
                        .color(prob_color),
                );
            });

            let (min_t, max_t) = kind.tick_range();
            ui.label(
                RichText::new(format!("Duration: {min_t}–{max_t} days"))
                    .small()
                    .color(Color32::from_gray(140)),
            );

            ui.add_space(6.0);
        }
    }

    // ── Dispatch button ───────────────────────────────────────────────────────
    let can_dispatch = panel_state.selected_commander.is_some()
        && panel_state.selected_kind.is_some()
        && panel_state.selected_target.is_some();

    ui.add_enabled_ui(can_dispatch, |ui| {
        if ui
            .button(RichText::new("Dispatch Mission").strong())
            .clicked()
        {
            if let (Some(character), Some(kind), Some(target)) = (
                panel_state.selected_commander,
                panel_state.selected_kind,
                panel_state.selected_target,
            ) {
                *action = Some(PanelAction::DispatchMission {
                    kind,
                    faction: player_faction,
                    character,
                    target,
                    duration_roll: panel_state.pending_duration_roll,
                });
                // Reset form after dispatch.
                panel_state.selected_commander = None;
                panel_state.selected_kind = None;
                panel_state.selected_target = None;
            }
        }
    });

    if !can_dispatch {
        ui.label(
            RichText::new("Select commander, type, and target to dispatch.")
                .small()
                .color(Color32::from_gray(100)),
        );
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn mission_kind_display(kind: MissionKind) -> (&'static str, Color32) {
    match kind {
        MissionKind::Diplomacy => ("[DIPLO]", Color32::from_rgb(255, 220, 80)),
        MissionKind::Recruitment => ("[RECRT]", Color32::from_rgb(100, 220, 180)),
    }
}
