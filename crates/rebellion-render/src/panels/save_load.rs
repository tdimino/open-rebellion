//! Save / load panel — egui modal with up to 10 named save slots.
//!
//! # Usage
//!
//! ```ignore
//! egui_macroquad::ui(|ctx| {
//!     if let Some(action) = panels::draw_save_load(ctx, &meta_list, &mut state) {
//!         match action {
//!             PanelAction::SaveGame { slot, name } => { /* call rebellion_data::save::save_slot */ }
//!             PanelAction::LoadGame { slot }       => { /* call rebellion_data::save::load_slot */ }
//!             PanelAction::DeleteSave { slot }     => { /* call rebellion_data::save::delete_slot */ }
//!             PanelAction::CloseSaveLoadPanel      => { /* hide the panel */ }
//!             _ => {}
//!         }
//!     }
//! });
//! ```
//!
//! The panel is rendered as a centered egui Window that overlays the galaxy
//! map. It is shown only when `SaveLoadPanelState::open` is true.

use egui_macroquad::egui::{self, Color32, RichText, ScrollArea};

use super::PanelAction;

// ---------------------------------------------------------------------------
// SaveSlotInfo — lightweight metadata for rendering
// ---------------------------------------------------------------------------

/// Metadata about one save slot, passed from the caller (who holds
/// `rebellion_data::save::SaveMeta`) down to this panel.
///
/// The panel does not import `rebellion-data` directly — the caller bridges
/// the two crates. This keeps `rebellion-render` free of the data layer dep.
#[derive(Debug, Clone)]
pub struct SaveSlotInfo {
    /// Slot index (`0..MAX_SAVE_SLOTS`).
    pub slot: usize,
    /// Human-readable save name.
    pub name: String,
    /// Formatted timestamp string (e.g. "2026-03-15 14:32").
    pub timestamp: String,
    /// Game tick at save time (displayed as "Day N").
    pub game_tick: u64,
}

// ---------------------------------------------------------------------------
// SaveLoadPanelState
// ---------------------------------------------------------------------------

/// Mutable UI state for the save/load panel.
#[derive(Debug, Clone, Default)]
pub struct SaveLoadPanelState {
    /// Whether the panel is currently visible.
    pub open: bool,
    /// Whether the panel is in save mode (`true`) or load mode (`false`).
    pub save_mode: bool,
    /// Text currently entered in the save-name input field.
    pub name_input: String,
    /// Slot the player has selected (highlighted row).
    pub selected_slot: Option<usize>,
    /// Non-empty when an error occurred during the last save/load op.
    pub error_message: Option<String>,
}

impl SaveLoadPanelState {
    /// Open the panel in save mode.
    pub fn open_save(&mut self) {
        self.open = true;
        self.save_mode = true;
        self.selected_slot = None;
        self.name_input.clear();
        self.error_message = None;
    }

    /// Open the panel in load mode.
    pub fn open_load(&mut self) {
        self.open = true;
        self.save_mode = false;
        self.selected_slot = None;
        self.name_input.clear();
        self.error_message = None;
    }

    /// Close the panel.
    pub fn close(&mut self) {
        self.open = false;
        self.error_message = None;
    }
}

fn slot_is_selectable(save_mode: bool, occupied: bool) -> bool {
    save_mode || occupied
}

fn confirm_is_enabled(saves: &[SaveSlotInfo], state: &SaveLoadPanelState) -> bool {
    let Some(slot) = state.selected_slot else {
        return false;
    };

    if state.save_mode {
        !state.name_input.trim().is_empty()
    } else {
        saves.iter().any(|save| save.slot == slot)
    }
}

// ---------------------------------------------------------------------------
// draw_save_load
// ---------------------------------------------------------------------------

/// Render the save/load modal panel.
///
/// `saves` is the current list of occupied slots, obtained by calling
/// `rebellion_data::save::list_saves()` and converting to `SaveSlotInfo`.
/// It may be empty (all slots are free).
///
/// Returns a `PanelAction` whenever the player confirms a save/load/delete
/// or closes the window; returns `None` while the player is still browsing.
///
/// The maximum number of save slots is `MAX_DISPLAY_SLOTS`.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
///
/// # Panics
/// Panics if an enabled confirmation action has no selected slot.
pub fn draw_save_load(
    ctx: &egui::Context,
    saves: &[SaveSlotInfo],
    state: &mut SaveLoadPanelState,
) -> Option<PanelAction> {
    if !state.open {
        return None;
    }

    let title = if state.save_mode {
        "Save Game"
    } else {
        "Load Game"
    };
    let mut action: Option<PanelAction> = None;
    let mut close_requested = false;

    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .min_width(380.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            // ── Slot list ────────────────────────────────────────────────
            ui.label(
                RichText::new("Select a save slot:")
                    .color(Color32::from_rgb(200, 200, 200))
                    .size(13.0),
            );
            ui.add_space(4.0);

            ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                for slot in 0..10usize {
                    let existing = saves.iter().find(|s| s.slot == slot);
                    let is_selected = state.selected_slot == Some(slot);

                    let slot_text = match existing {
                        Some(info) => format!(
                            "[{}]  {}  —  Day {}  ({})",
                            slot + 1,
                            info.name,
                            info.game_tick,
                            info.timestamp
                        ),
                        None => format!("[{}]  — empty —", slot + 1),
                    };

                    let label_color = if is_selected {
                        Color32::from_rgb(255, 220, 100)
                    } else if existing.is_some() {
                        Color32::from_rgb(220, 220, 220)
                    } else {
                        Color32::from_rgb(140, 140, 140)
                    };

                    let response = ui.add_enabled(
                        slot_is_selectable(state.save_mode, existing.is_some()),
                        egui::SelectableLabel::new(
                            is_selected,
                            RichText::new(&slot_text).color(label_color),
                        ),
                    );
                    if response.clicked() {
                        state.selected_slot = Some(slot);
                        // Pre-fill name with existing save name if loading
                        if let Some(info) = existing {
                            if state.save_mode {
                                state.name_input.clone_from(&info.name);
                            }
                        }
                    }

                    // Delete button (save mode only, existing slots)
                    if state.save_mode {
                        if let Some(info) = existing {
                            ui.horizontal(|ui| {
                                ui.add_space(20.0);
                                if ui
                                    .small_button(
                                        RichText::new("✕ Delete")
                                            .color(Color32::from_rgb(200, 80, 80)),
                                    )
                                    .clicked()
                                {
                                    action = Some(PanelAction::DeleteSave { slot: info.slot });
                                }
                            });
                        }
                    }
                }
            });

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(4.0);

            // ── Save name input (save mode only) ─────────────────────────
            if state.save_mode {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Save name:").color(Color32::from_rgb(180, 180, 180)));
                    ui.text_edit_singleline(&mut state.name_input);
                });
                ui.add_space(4.0);
            }

            // ── Error banner ─────────────────────────────────────────────
            if let Some(ref err) = state.error_message {
                ui.colored_label(Color32::from_rgb(220, 60, 60), err);
                ui.add_space(4.0);
            }

            // ── Action buttons ────────────────────────────────────────────
            ui.horizontal(|ui| {
                let confirm_label = if state.save_mode { "Save" } else { "Load" };
                let confirm_enabled = confirm_is_enabled(saves, state);

                if ui
                    .add_enabled(
                        confirm_enabled,
                        egui::Button::new(
                            RichText::new(confirm_label).color(Color32::from_rgb(100, 220, 100)),
                        ),
                    )
                    .clicked()
                {
                    let slot = state.selected_slot.unwrap();
                    if state.save_mode {
                        action = Some(PanelAction::SaveGame {
                            slot,
                            name: state.name_input.trim().to_string(),
                        });
                    } else {
                        action = Some(PanelAction::LoadGame { slot });
                    }
                }

                ui.add_space(8.0);

                if ui
                    .button(RichText::new("Cancel").color(Color32::from_rgb(180, 180, 180)))
                    .clicked()
                {
                    close_requested = true;
                }
            });
        });

    if close_requested {
        state.close();
        return Some(PanelAction::CloseSaveLoadPanel);
    }

    action
}

#[cfg(test)]
mod tests {
    use super::*;

    fn occupied_slot(slot: usize) -> SaveSlotInfo {
        SaveSlotInfo {
            slot,
            name: "Campaign".to_string(),
            timestamp: "Browser save".to_string(),
            game_tick: 42,
        }
    }

    #[test]
    fn load_mode_rejects_empty_slots() {
        let state = SaveLoadPanelState {
            open: true,
            save_mode: false,
            selected_slot: Some(0),
            ..Default::default()
        };

        assert!(!slot_is_selectable(false, false));
        assert!(!confirm_is_enabled(&[], &state));
    }

    #[test]
    fn load_mode_accepts_only_an_occupied_selection() {
        let saves = vec![occupied_slot(1)];
        let mut state = SaveLoadPanelState {
            open: true,
            save_mode: false,
            selected_slot: Some(0),
            ..Default::default()
        };

        assert!(slot_is_selectable(false, true));
        assert!(!confirm_is_enabled(&saves, &state));
        state.selected_slot = Some(1);
        assert!(confirm_is_enabled(&saves, &state));
    }

    #[test]
    fn save_mode_allows_empty_slot_after_a_name_is_entered() {
        let mut state = SaveLoadPanelState {
            open: true,
            save_mode: true,
            selected_slot: Some(0),
            ..Default::default()
        };

        assert!(slot_is_selectable(true, false));
        assert!(!confirm_is_enabled(&[], &state));
        state.name_input = "New Campaign".to_string();
        assert!(confirm_is_enabled(&[], &state));
    }
}
