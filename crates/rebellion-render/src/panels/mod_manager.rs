//! Mod Manager panel: discover, enable/disable, inspect mods.
//!
//! # Usage
//!
//! ```ignore
//! egui_macroquad::ui(|ctx| {
//!     let actions = panels::draw_mod_manager(ctx, &mod_infos, &mut mod_state);
//!     for action in actions {
//!         match action {
//!             ModManagerAction::ToggleMod(name) => { mod_runtime.toggle_mod(&name); }
//!             ModManagerAction::ReloadMods      => { mod_runtime.refresh(); }
//!         }
//!     }
//! });
//! ```
//!
//! The panel renders as a resizable egui Window. It is shown only when
//! `ModManagerState::open` is true.

use egui_macroquad::egui::{self, Color32, RichText};

// ---------------------------------------------------------------------------
// ModManagerState
// ---------------------------------------------------------------------------

/// Mutable UI state for the Mod Manager panel.
#[derive(Debug, Default)]
pub struct ModManagerState {
    /// Whether the panel is currently visible.
    pub open: bool,
    /// Selected mod index for detail view.
    pub selected: Option<usize>,
}

// ---------------------------------------------------------------------------
// ModInfo — display-only struct (no rebellion-data dependency)
// ---------------------------------------------------------------------------

/// Info about a discovered mod, passed from the data layer.
///
/// `rebellion-render` cannot depend on `rebellion-data`, so we use this
/// display struct. The caller converts `ModManifest` → `ModInfo`.
#[derive(Debug, Clone)]
pub struct ModInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub enabled: bool,
    pub dependencies: Vec<String>,
    pub has_error: bool,
    pub error_message: Option<String>,
}

// ---------------------------------------------------------------------------
// ModManagerAction
// ---------------------------------------------------------------------------

/// Actions returned by the Mod Manager panel for the app layer to handle.
#[derive(Debug, Clone)]
pub enum ModManagerAction {
    /// Toggle a mod's enabled/disabled state by name.
    ToggleMod(String),
    /// Reload all mods from disk.
    ReloadMods,
}

// ---------------------------------------------------------------------------
// draw_mod_manager
// ---------------------------------------------------------------------------

/// Render the Mod Manager panel.
///
/// `mods` is the current list of discovered mods, obtained by converting
/// `ModRuntime::discovered` into `ModInfo` slices. Returns a list of actions
/// for the caller to apply to `ModRuntime`.
pub fn draw_mod_manager(
    ctx: &egui::Context,
    mods: &[ModInfo],
    state: &mut ModManagerState,
) -> Vec<ModManagerAction> {
    let mut actions = Vec::new();

    if !state.open {
        return actions;
    }

    egui::Window::new("Mod Manager")
        .resizable(true)
        .default_width(400.0)
        .default_height(500.0)
        .show(ctx, |ui| {
            ui.heading("Installed Mods");
            ui.separator();

            if mods.is_empty() {
                ui.label("No mods found. Place mod folders in the mods/ directory.");
                // Don't return early — footer buttons (Reload, Close) must always render.
            }

            // ── Mod list ────────────────────────────────────────────────────
            for (i, mod_info) in mods.iter().enumerate() {
                ui.horizontal(|ui| {
                    // Enable/disable checkbox.
                    let mut enabled = mod_info.enabled;
                    if ui.checkbox(&mut enabled, "").changed() {
                        actions.push(ModManagerAction::ToggleMod(mod_info.name.clone()));
                    }

                    // Name — colored by status.
                    let name_text = if mod_info.has_error {
                        RichText::new(&mod_info.name).color(Color32::RED)
                    } else if mod_info.enabled {
                        RichText::new(&mod_info.name).color(Color32::LIGHT_GREEN)
                    } else {
                        RichText::new(&mod_info.name).color(Color32::GRAY)
                    };

                    if ui
                        .selectable_label(state.selected == Some(i), name_text)
                        .clicked()
                    {
                        state.selected = Some(i);
                    }

                    ui.label(RichText::new(&mod_info.version).weak());
                });
            }

            ui.separator();

            // ── Detail view for selected mod ────────────────────────────────
            if let Some(idx) = state.selected {
                if let Some(mod_info) = mods.get(idx) {
                    ui.heading(&mod_info.name);
                    ui.label(format!("Version: {}", mod_info.version));
                    if !mod_info.author.is_empty() {
                        ui.label(format!("Author: {}", mod_info.author));
                    }
                    if !mod_info.description.is_empty() {
                        ui.label(&mod_info.description);
                    }
                    if !mod_info.dependencies.is_empty() {
                        ui.label(format!(
                            "Dependencies: {}",
                            mod_info.dependencies.join(", ")
                        ));
                    }
                    if let Some(err) = &mod_info.error_message {
                        ui.colored_label(Color32::RED, format!("Error: {}", err));
                    }
                }
            }

            ui.separator();

            // ── Action buttons ──────────────────────────────────────────────
            ui.horizontal(|ui| {
                if ui.button("Reload Mods").clicked() {
                    actions.push(ModManagerAction::ReloadMods);
                }
                if ui.button("Close").clicked() {
                    state.open = false;
                }
            });
        });

    actions
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_manager_state_default() {
        let state = ModManagerState::default();
        assert!(!state.open);
        assert!(state.selected.is_none());
    }

    #[test]
    fn mod_info_display() {
        let info = ModInfo {
            name: "Test Mod".into(),
            version: "1.0.0".into(),
            author: "Author".into(),
            description: "A test mod".into(),
            enabled: true,
            dependencies: vec!["base".into()],
            has_error: false,
            error_message: None,
        };
        assert_eq!(info.name, "Test Mod");
        assert!(info.enabled);
        assert!(!info.has_error);
        assert_eq!(info.dependencies.len(), 1);
    }

    #[test]
    fn action_variants() {
        let toggle = ModManagerAction::ToggleMod("my-mod".into());
        assert!(matches!(toggle, ModManagerAction::ToggleMod(ref n) if n == "my-mod"));

        let reload = ModManagerAction::ReloadMods;
        assert!(matches!(reload, ModManagerAction::ReloadMods));
    }
}
