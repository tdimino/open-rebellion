//! VS Code-style command palette for play-testing.
//!
//! Triggered by backtick (`` ` ``). Fuzzy search via nucleo-matcher.
//! Gated behind `#[cfg(debug_assertions)]` for release builds.

use egui_macroquad::egui::{self, Align, Color32, Key, Layout, RichText, ScrollArea};
use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher};

/// A registered command in the palette.
#[derive(Debug, Clone)]
pub struct CommandItem {
    pub label: String,
    pub description: String,
    pub category: String,
    pub action: super::PanelAction,
}

/// State for the command palette modal.
#[derive(Debug)]
pub struct CommandPaletteState {
    pub open: bool,
    pub query: String,
    pub selected_index: usize,
    pub just_opened: bool,
    commands: Vec<CommandItem>,
    filtered_indices: Vec<(usize, u32)>, // (command index, score)
}

impl CommandPaletteState {
    pub fn new() -> Self {
        let commands = vec![
            // ── Time Control ─────────────────────────────────────────
            CommandItem {
                label: "Advance 1 tick".into(),
                description: "Step simulation forward by 1 tick".into(),
                category: "Time".into(),
                action: super::PanelAction::AdvanceTicks(1),
            },
            CommandItem {
                label: "Advance 10 ticks".into(),
                description: "Step simulation forward by 10 ticks".into(),
                category: "Time".into(),
                action: super::PanelAction::AdvanceTicks(10),
            },
            CommandItem {
                label: "Advance 100 ticks".into(),
                description: "Step simulation forward by 100 ticks".into(),
                category: "Time".into(),
                action: super::PanelAction::AdvanceTicks(100),
            },
            CommandItem {
                label: "Advance 1000 ticks".into(),
                description: "Step simulation forward by 1000 ticks".into(),
                category: "Time".into(),
                action: super::PanelAction::AdvanceTicks(1000),
            },
            // ── Speed ────────────────────────────────────────────────
            CommandItem {
                label: "Set Speed: Paused".into(),
                description: "Pause the simulation".into(),
                category: "Speed".into(),
                action: super::PanelAction::SetGameSpeed(0),
            },
            CommandItem {
                label: "Set Speed: Normal".into(),
                description: "Set simulation to 1x speed".into(),
                category: "Speed".into(),
                action: super::PanelAction::SetGameSpeed(1),
            },
            CommandItem {
                label: "Set Speed: Fast".into(),
                description: "Set simulation to 2x speed".into(),
                category: "Speed".into(),
                action: super::PanelAction::SetGameSpeed(2),
            },
            CommandItem {
                label: "Set Speed: Faster".into(),
                description: "Set simulation to 4x speed".into(),
                category: "Speed".into(),
                action: super::PanelAction::SetGameSpeed(4),
            },
            // ── Inspection ───────────────────────────────────────────
            CommandItem {
                label: "Show Game Stats".into(),
                description: "Display current game statistics".into(),
                category: "Inspect".into(),
                action: super::PanelAction::ShowGameStats,
            },
            CommandItem {
                label: "List Active Missions".into(),
                description: "Show all in-progress missions".into(),
                category: "Inspect".into(),
                action: super::PanelAction::ListActiveMissions,
            },
            CommandItem {
                label: "List Active Fleets".into(),
                description: "Show all fleet positions and compositions".into(),
                category: "Inspect".into(),
                action: super::PanelAction::ListActiveFleets,
            },
            CommandItem {
                label: "Show Event Count".into(),
                description: "Display number of triggered events".into(),
                category: "Inspect".into(),
                action: super::PanelAction::ShowEventCount,
            },
            // ── Control ──────────────────────────────────────────────
            CommandItem {
                label: "Toggle Dual AI".into(),
                description: "Enable/disable AI control for both factions".into(),
                category: "Control".into(),
                action: super::PanelAction::ToggleDualAI,
            },
            CommandItem {
                label: "Force Victory Check".into(),
                description: "Immediately evaluate victory conditions".into(),
                category: "Control".into(),
                action: super::PanelAction::ForceVictoryCheck,
            },
            CommandItem {
                label: "Reveal All Systems".into(),
                description: "Remove fog of war from all systems".into(),
                category: "Control".into(),
                action: super::PanelAction::RevealAllFog,
            },
            CommandItem {
                label: "Export Game Log".into(),
                description: "Write message log to file".into(),
                category: "Control".into(),
                action: super::PanelAction::ExportGameLog,
            },
        ];

        let filtered_indices = (0..commands.len()).map(|i| (i, 0u32)).collect();

        Self {
            open: false,
            query: String::new(),
            selected_index: 0,
            just_opened: false,
            commands,
            filtered_indices,
        }
    }

    /// Toggle the palette open/closed.
    pub fn toggle(&mut self) {
        self.open = !self.open;
        if self.open {
            self.query.clear();
            self.selected_index = 0;
            self.just_opened = true;
            self.filtered_indices = (0..self.commands.len()).map(|i| (i, 0u32)).collect();
        }
    }

    /// Update filtered results based on current query.
    pub fn update_filter(&mut self) {
        if self.query.is_empty() {
            self.filtered_indices = (0..self.commands.len()).map(|i| (i, 0u32)).collect();
        } else {
            let mut matcher = Matcher::new(Config::DEFAULT);
            let pattern = Pattern::new(
                &self.query,
                CaseMatching::Ignore,
                Normalization::Smart,
                AtomKind::Fuzzy,
            );

            let mut results: Vec<(usize, u32)> = Vec::new();
            for (i, cmd) in self.commands.iter().enumerate() {
                let matches =
                    pattern.match_list(std::iter::once(cmd.label.as_str()), &mut matcher);
                if let Some(&(_, score)) = matches.first() {
                    results.push((i, score));
                }
            }
            results.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered_indices = results;
        }

        // Clamp selected index.
        if !self.filtered_indices.is_empty() {
            self.selected_index = self.selected_index.min(self.filtered_indices.len() - 1);
        } else {
            self.selected_index = 0;
        }
    }

    /// Get the command at the current selection, if any.
    fn selected_command(&self) -> Option<&CommandItem> {
        self.filtered_indices
            .get(self.selected_index)
            .and_then(|(idx, _)| self.commands.get(*idx))
    }
}

/// Draw the command palette overlay. Returns actions to execute.
pub fn draw_command_palette(
    ctx: &egui::Context,
    state: &mut CommandPaletteState,
) -> Vec<super::PanelAction> {
    if !state.open {
        return Vec::new();
    }

    let mut actions = Vec::new();
    let mut close = false;

    // Keyboard navigation — read before the window so Escape works even if
    // the text input swallowed focus.
    let up = ctx.input(|i: &egui::InputState| i.key_pressed(Key::ArrowUp));
    let down = ctx.input(|i: &egui::InputState| i.key_pressed(Key::ArrowDown));
    let enter = ctx.input(|i: &egui::InputState| i.key_pressed(Key::Enter));
    let escape = ctx.input(|i: &egui::InputState| i.key_pressed(Key::Escape));

    if escape {
        close = true;
    }

    if !state.filtered_indices.is_empty() {
        if up && state.selected_index > 0 {
            state.selected_index -= 1;
        }
        if down && state.selected_index + 1 < state.filtered_indices.len() {
            state.selected_index += 1;
        }
        if enter {
            if let Some(cmd) = state.selected_command() {
                actions.push(cmd.action.clone());
                close = true;
            }
        }
    }

    // Dark backdrop via an Area covering the full screen.
    egui::Area::new(egui::Id::new("cmd_palette_backdrop"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .order(egui::Order::Foreground)
        .interactable(true)
        .show(ctx, |ui: &mut egui::Ui| {
            let screen = ctx.screen_rect();
            let (_, response) =
                ui.allocate_exact_size(screen.size(), egui::Sense::click());
            ui.painter()
                .rect_filled(screen, 0.0, Color32::from_black_alpha(160));
            if response.clicked() {
                close = true;
            }
        });

    // The palette window itself.
    let screen = ctx.screen_rect();
    let panel_width = (screen.width() * 0.5).max(300.0);

    egui::Window::new("Command Palette")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .min_width(panel_width)
        .max_width(panel_width)
        .anchor(egui::Align2::CENTER_TOP, [0.0, screen.height() * 0.15])
        .order(egui::Order::Foreground)
        .frame(
            egui::Frame::window(ctx.style().as_ref())
                .fill(Color32::from_rgb(30, 30, 40))
                .corner_radius(8.0)
                .inner_margin(12.0),
        )
        .show(ctx, |ui: &mut egui::Ui| {
            // Search input.
            let input = egui::TextEdit::singleline(&mut state.query)
                .hint_text("Type a command...")
                .desired_width(panel_width - 24.0)
                .text_color(Color32::WHITE);

            let response = ui.add(input);

            // Auto-focus on open.
            if state.just_opened {
                response.request_focus();
                state.just_opened = false;
            }

            // Update filter when query changes.
            if response.changed() {
                state.update_filter();
            }

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // Results list.
            ScrollArea::vertical()
                .max_height(360.0)
                .show(ui, |ui: &mut egui::Ui| {
                    for (list_idx, &(cmd_idx, _score)) in
                        state.filtered_indices.iter().enumerate()
                    {
                        let cmd = &state.commands[cmd_idx];
                        let is_selected = list_idx == state.selected_index;

                        let bg = if is_selected {
                            Color32::from_rgb(50, 60, 90)
                        } else {
                            Color32::TRANSPARENT
                        };

                        let frame_response = egui::Frame::NONE
                            .fill(bg)
                            .corner_radius(4.0)
                            .inner_margin(egui::Margin::symmetric(8, 4))
                            .show(ui, |ui: &mut egui::Ui| {
                                ui.set_width(panel_width - 40.0);
                                ui.horizontal(|ui: &mut egui::Ui| {
                                    // Label (bold) + description (dimmed) on the left.
                                    ui.label(
                                        RichText::new(&cmd.label)
                                            .color(Color32::WHITE)
                                            .strong(),
                                    );
                                    ui.label(
                                        RichText::new(&cmd.description)
                                            .color(Color32::from_gray(140))
                                            .small(),
                                    );
                                    ui.with_layout(
                                        Layout::right_to_left(Align::Center),
                                        |ui: &mut egui::Ui| {
                                            ui.label(
                                                RichText::new(&cmd.category)
                                                    .color(Color32::from_rgb(
                                                        130, 160, 220,
                                                    ))
                                                    .small(),
                                            );
                                        },
                                    );
                                });
                            });

                        // Click to execute.
                        let row_response = ui.interact(
                            frame_response.response.rect,
                            egui::Id::new(("cmd_row", cmd_idx)),
                            egui::Sense::click(),
                        );
                        if row_response.clicked() {
                            actions.push(cmd.action.clone());
                            close = true;
                        }
                        if row_response.hovered() {
                            state.selected_index = list_idx;
                        }
                    }

                    if state.filtered_indices.is_empty() {
                        ui.label(
                            RichText::new("No matching commands")
                                .color(Color32::from_gray(100))
                                .italics(),
                        );
                    }
                });
        });

    if close {
        state.open = false;
        state.query.clear();
        state.selected_index = 0;
    }

    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_palette_state_defaults() {
        let state = CommandPaletteState::new();
        assert!(!state.open);
        assert!(state.query.is_empty());
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn fuzzy_filter_matches_advance() {
        let mut state = CommandPaletteState::new();
        state.query = "adv".to_string();
        state.update_filter();
        assert!(!state.filtered_indices.is_empty());
        let first_cmd = &state.commands[state.filtered_indices[0].0];
        assert!(
            first_cmd.label.contains("Advance"),
            "Expected 'Advance' in label, got: {}",
            first_cmd.label
        );
    }

    #[test]
    fn fuzzy_filter_empty_returns_all() {
        let mut state = CommandPaletteState::new();
        state.query.clear();
        state.update_filter();
        assert_eq!(state.filtered_indices.len(), state.commands.len());
    }

    #[test]
    fn all_commands_have_unique_labels() {
        let state = CommandPaletteState::new();
        let mut labels: Vec<&str> = state.commands.iter().map(|c| c.label.as_str()).collect();
        let total = labels.len();
        labels.sort();
        labels.dedup();
        assert_eq!(labels.len(), total, "Duplicate command labels found");
    }
}
