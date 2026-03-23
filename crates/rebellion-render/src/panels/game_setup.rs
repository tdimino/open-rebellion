//! New Game Setup panel — galaxy size, difficulty, faction selection.
//!
//! Replaces the old faction_select.rs. Presented as a full-screen layout
//! before entering the galaxy view. Returns `GameSetupAction::StartGame`
//! with the chosen configuration when the player confirms.

use egui_macroquad::egui::{self, Align, Color32, Layout, RichText};
use rebellion_core::dat::GalaxySize;
use rebellion_core::missions::MissionFaction;

use crate::theme;

// ── Difficulty ───────────────────────────────────────────────────────────────

/// Game difficulty level. Affects AI aggressiveness and combat modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn label(self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Difficulty::Easy => "Relaxed pace. AI is less aggressive, combat favors the player.",
            Difficulty::Medium => "Balanced challenge. Standard AI behavior.",
            Difficulty::Hard => "Relentless opposition. AI is aggressive with production bonuses.",
        }
    }
}

// ── Setup state ──────────────────────────────────────────────────────────────

/// Mutable state for the game setup screen.
#[derive(Debug, Clone)]
pub struct GameSetupState {
    pub galaxy_size: GalaxySize,
    pub difficulty: Difficulty,
    pub faction: Option<MissionFaction>,
}

impl Default for GameSetupState {
    fn default() -> Self {
        Self {
            galaxy_size: GalaxySize::Standard,
            difficulty: Difficulty::Medium,
            faction: None,
        }
    }
}

// ── Actions ──────────────────────────────────────────────────────────────────

/// Actions the game setup screen can produce.
#[derive(Debug, Clone)]
pub enum GameSetupAction {
    /// Player confirmed all settings — start the game.
    StartGame {
        galaxy_size: GalaxySize,
        difficulty: Difficulty,
        faction: MissionFaction,
    },
    /// Player wants to go back to the main menu.
    Back,
}

// ── Rendering ────────────────────────────────────────────────────────────────

/// Render the game setup screen. Returns an action when the player confirms or goes back.
pub fn draw_game_setup(
    ctx: &egui::Context,
    state: &mut GameSetupState,
) -> Option<GameSetupAction> {
    let mut action = None;

    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(theme::BG_SPACE))
        .show(ctx, |ui| {
            let available = ui.available_size();

            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(available.y * 0.08);

                // ── Header ───────────────────────────────────────────────
                ui.label(
                    RichText::new("NEW GAME")
                        .color(theme::GOLD)
                        .size(28.0)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new("Configure your campaign")
                        .color(theme::TEXT_SECONDARY)
                        .size(13.0),
                );

                ui.add_space(available.y * 0.04);

                // Center the settings panel
                let panel_width = 500.0_f32.min(available.x - 40.0);
                ui.allocate_ui(egui::vec2(panel_width, available.y * 0.7), |ui| {
                    // ── Galaxy Size ──────────────────────────────────────
                    section_header(ui, "GALAXY SIZE");
                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        galaxy_size_button(ui, "Standard", "200 systems, 15 sectors", GalaxySize::Standard, &mut state.galaxy_size);
                        ui.add_space(8.0);
                        galaxy_size_button(ui, "Large", "300 systems, 19 sectors", GalaxySize::Large, &mut state.galaxy_size);
                        ui.add_space(8.0);
                        galaxy_size_button(ui, "Huge", "400 systems, 24 sectors", GalaxySize::Huge, &mut state.galaxy_size);
                    });

                    ui.add_space(20.0);

                    // ── Difficulty ────────────────────────────────────────
                    section_header(ui, "DIFFICULTY");
                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        difficulty_button(ui, Difficulty::Easy, &mut state.difficulty);
                        ui.add_space(8.0);
                        difficulty_button(ui, Difficulty::Medium, &mut state.difficulty);
                        ui.add_space(8.0);
                        difficulty_button(ui, Difficulty::Hard, &mut state.difficulty);
                    });

                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(state.difficulty.description())
                            .color(theme::TEXT_SECONDARY)
                            .size(11.0),
                    );

                    ui.add_space(20.0);

                    // ── Faction Selection ─────────────────────────────────
                    section_header(ui, "CHOOSE YOUR SIDE");
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        faction_button(
                            ui,
                            "REBEL ALLIANCE",
                            "Fight for freedom across the galaxy.\nLead the Rebellion to victory.",
                            theme::ALLIANCE_BLUE,
                            MissionFaction::Alliance,
                            &mut state.faction,
                        );
                        ui.add_space(16.0);
                        faction_button(
                            ui,
                            "GALACTIC EMPIRE",
                            "Crush the Rebellion and restore order.\nThe galaxy will kneel.",
                            theme::EMPIRE_RED,
                            MissionFaction::Empire,
                            &mut state.faction,
                        );
                    });

                    ui.add_space(32.0);

                    // ── Action buttons ────────────────────────────────────
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        let can_start = state.faction.is_some();

                        // Start Game button
                        let start_text = if can_start {
                            RichText::new("START CAMPAIGN")
                                .color(theme::BG_SPACE)
                                .size(16.0)
                                .strong()
                        } else {
                            RichText::new("SELECT A FACTION")
                                .color(theme::TEXT_DISABLED)
                                .size(16.0)
                        };

                        let start_btn = egui::Button::new(start_text)
                            .fill(if can_start { theme::GOLD } else { Color32::from_rgb(40, 42, 56) })
                            .stroke(egui::Stroke::new(
                                1.0,
                                if can_start { theme::GOLD_BRIGHT } else { Color32::from_rgb(60, 62, 76) },
                            ));

                        let resp = ui.add_sized([220.0, 44.0], start_btn);
                        if can_start && resp.clicked() {
                            action = Some(GameSetupAction::StartGame {
                                galaxy_size: state.galaxy_size,
                                difficulty: state.difficulty,
                                faction: state.faction.unwrap(),
                            });
                        }

                        ui.add_space(12.0);

                        // Back button
                        if ui.add_sized(
                            [120.0, 32.0],
                            egui::Button::new(
                                RichText::new("BACK")
                                    .color(theme::TEXT_SECONDARY)
                                    .size(12.0),
                            )
                            .fill(Color32::TRANSPARENT)
                            .stroke(egui::Stroke::new(0.5, theme::TEXT_DISABLED)),
                        ).clicked() {
                            action = Some(GameSetupAction::Back);
                        }
                    });
                });
            });
        });

    action
}

// ── Helper widgets ───────────────────────────────────────────────────────────

fn section_header(ui: &mut egui::Ui, label: &str) {
    ui.label(
        RichText::new(label)
            .color(theme::GOLD_DIM)
            .size(11.0)
            .strong(),
    );
    ui.add(egui::Separator::default().spacing(4.0));
}

fn galaxy_size_button(
    ui: &mut egui::Ui,
    label: &str,
    description: &str,
    value: GalaxySize,
    current: &mut GalaxySize,
) {
    let selected = *current == value;
    let fill = if selected {
        Color32::from_rgb(40, 38, 20)
    } else {
        Color32::from_rgb(20, 22, 36)
    };
    let stroke_color = if selected { theme::GOLD } else { theme::GOLD_DIM };

    let btn = egui::Button::new(
        RichText::new(format!("{}\n{}", label, description))
            .color(if selected { theme::GOLD_BRIGHT } else { theme::TEXT_PRIMARY })
            .size(12.0),
    )
    .fill(fill)
    .stroke(egui::Stroke::new(if selected { 1.5 } else { 0.5 }, stroke_color));

    if ui.add_sized([140.0, 56.0], btn).clicked() {
        *current = value;
    }
}

fn difficulty_button(
    ui: &mut egui::Ui,
    value: Difficulty,
    current: &mut Difficulty,
) {
    let selected = *current == value;
    let fill = if selected {
        Color32::from_rgb(40, 38, 20)
    } else {
        Color32::from_rgb(20, 22, 36)
    };
    let stroke_color = if selected { theme::GOLD } else { theme::GOLD_DIM };

    let btn = egui::Button::new(
        RichText::new(value.label())
            .color(if selected { theme::GOLD_BRIGHT } else { theme::TEXT_PRIMARY })
            .size(14.0)
            .strong(),
    )
    .fill(fill)
    .stroke(egui::Stroke::new(if selected { 1.5 } else { 0.5 }, stroke_color));

    if ui.add_sized([120.0, 40.0], btn).clicked() {
        *current = value;
    }
}

fn faction_button(
    ui: &mut egui::Ui,
    name: &str,
    description: &str,
    accent: Color32,
    value: MissionFaction,
    current: &mut Option<MissionFaction>,
) {
    let selected = *current == Some(value);
    let fill = if selected {
        Color32::from_rgb(25, 28, 45)
    } else {
        Color32::from_rgb(15, 17, 28)
    };
    let stroke_color = if selected { accent } else { Color32::from_rgb(50, 52, 68) };

    let resp = ui.allocate_ui(egui::vec2(200.0, 100.0), |ui| {
        let frame = egui::Frame::default()
            .fill(fill)
            .stroke(egui::Stroke::new(if selected { 2.0 } else { 1.0 }, stroke_color))
            .corner_radius(4.0)
            .inner_margin(12.0);

        frame.show(ui, |ui| {
            ui.set_min_size(egui::vec2(176.0, 72.0));
            ui.vertical(|ui| {
                ui.label(
                    RichText::new(name)
                        .color(if selected { accent } else { theme::TEXT_PRIMARY })
                        .size(15.0)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(description)
                        .color(theme::TEXT_SECONDARY)
                        .size(11.0),
                );
            });
        });
    });

    if resp.response.interact(egui::Sense::click()).clicked() {
        *current = Some(value);
    }
}
