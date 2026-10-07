//! Star Wars Rebellion egui theme.
//!
//! Dark space background with gold/amber accents, faction-colored elements,
//! and Liberation Sans font (metrically identical to the original game's Arial).
//!
//! Architecture: macroquad renders BMP backgrounds first, then egui panels
//! overlay on top. Colors here match the original game palette so that when
//! DLL-extracted cockpit frames and button sprites are loaded, they integrate
//! seamlessly with the themed UI.

use egui_macroquad::egui::{self, Color32, FontData, FontDefinitions, FontFamily, Style, Visuals};

// ── Color palette ────────────────────────────────────────────────────────────

/// Deep space background — nearly black with a hint of blue.
pub const BG_SPACE: Color32 = Color32::from_rgb(5, 5, 16);
/// Slightly lighter panel background.
pub const BG_PANEL: Color32 = Color32::from_rgb(12, 14, 24);
/// Panel/window frame color.
pub const BG_FRAME: Color32 = Color32::from_rgb(22, 24, 36);
/// Gold accent — button highlights, selection, headers.
pub const GOLD: Color32 = Color32::from_rgb(218, 165, 32);
/// Darker gold for hover states.
pub const GOLD_DIM: Color32 = Color32::from_rgb(160, 120, 24);
/// Bright gold for active/pressed states.
pub const GOLD_BRIGHT: Color32 = Color32::from_rgb(255, 200, 60);
/// Alliance blue.
pub const ALLIANCE_BLUE: Color32 = Color32::from_rgb(100, 160, 255);
/// Empire red.
pub const EMPIRE_RED: Color32 = Color32::from_rgb(220, 80, 80);
/// Neutral / unaligned gray.
pub const NEUTRAL_GRAY: Color32 = Color32::from_rgb(140, 140, 140);
/// Primary text color — off-white.
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(220, 215, 200);
/// Secondary text color — muted.
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(150, 145, 135);
/// Disabled text.
pub const TEXT_DISABLED: Color32 = Color32::from_rgb(80, 78, 72);
/// Success / positive indicator.
pub const SUCCESS_GREEN: Color32 = Color32::from_rgb(60, 180, 80);
/// Warning / caution.
pub const WARNING_AMBER: Color32 = Color32::from_rgb(220, 160, 40);
/// Error / danger.
pub const DANGER_RED: Color32 = Color32::from_rgb(200, 50, 50);

// ── Font loading ─────────────────────────────────────────────────────────────

fn production_font_definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "liberation-sans".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(
            "../../../assets/fonts/LiberationSans-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "liberation-sans-bold".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(
            "../../../assets/fonts/LiberationSans-Bold.ttf"
        ))),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "liberation-sans".to_owned());
    fonts.families.insert(
        FontFamily::Name("liberation-sans-bold".into()),
        vec!["liberation-sans-bold".to_owned()],
    );
    fonts
}

/// The metric-compatible bold face used by original game-font entry 5.
#[must_use]
pub fn original_bold_font(size: f32) -> egui::FontId {
    egui::FontId::new(size, FontFamily::Name("liberation-sans-bold".into()))
}

/// Load the metric-compatible Liberation Sans faces into egui.
///
/// The font bytes are embedded so native and production WASM builds use the
/// same face; WASM cannot satisfy the old runtime filesystem lookup.
pub fn load_fonts(ctx: &egui::Context) {
    ctx.set_fonts(production_font_definitions());
}

// ── Theme application ────────────────────────────────────────────────────────

/// Apply the Star Wars Rebellion theme to the egui context.
///
/// Sets dark space visuals with gold/amber accents. Call once at startup.
pub fn apply_theme(ctx: &egui::Context) {
    let mut style = Style::default();

    // ── Visuals ──────────────────────────────────────────────────────────────
    let mut visuals = Visuals::dark();

    // Window / panel backgrounds
    visuals.window_fill = BG_PANEL;
    visuals.panel_fill = BG_PANEL;
    visuals.extreme_bg_color = BG_SPACE;
    visuals.faint_bg_color = BG_FRAME;

    // Window frame / border
    visuals.window_stroke = egui::Stroke::new(1.0_f32, GOLD_DIM);

    // Selection color
    visuals.selection.bg_fill = Color32::from_rgba_premultiplied(218, 165, 32, 60);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, GOLD);

    // Hyperlink color
    visuals.hyperlink_color = GOLD;

    // Widget styles — inactive
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(30, 32, 48);
    visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(25, 27, 40);
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, Color32::from_rgb(50, 52, 68));
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0_f32, TEXT_PRIMARY);
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(3);

    // Widget styles — hovered
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(40, 42, 60);
    visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(35, 37, 52);
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, GOLD_DIM);
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0_f32, GOLD);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(3);

    // Widget styles — active (pressed)
    visuals.widgets.active.bg_fill = Color32::from_rgb(50, 48, 30);
    visuals.widgets.active.weak_bg_fill = Color32::from_rgb(45, 43, 28);
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.5_f32, GOLD_BRIGHT);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.5_f32, GOLD_BRIGHT);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(3);

    // Widget styles — open (expanded combo boxes, etc.)
    visuals.widgets.open.bg_fill = Color32::from_rgb(35, 37, 52);
    visuals.widgets.open.weak_bg_fill = Color32::from_rgb(30, 32, 46);
    visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0_f32, GOLD_DIM);
    visuals.widgets.open.fg_stroke = egui::Stroke::new(1.0_f32, GOLD);
    visuals.widgets.open.corner_radius = egui::CornerRadius::same(3);

    // Widget styles — non-interactive (labels, etc.)
    visuals.widgets.noninteractive.bg_fill = BG_PANEL;
    visuals.widgets.noninteractive.weak_bg_fill = BG_PANEL;
    visuals.widgets.noninteractive.bg_stroke =
        egui::Stroke::new(0.5_f32, Color32::from_rgb(40, 42, 56));
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0_f32, TEXT_PRIMARY);
    visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(2);

    // Window and menu rounding
    visuals.window_corner_radius = egui::CornerRadius::same(4);
    visuals.menu_corner_radius = egui::CornerRadius::same(4);

    style.visuals = visuals;

    // ── Spacing ──────────────────────────────────────────────────────────────
    style.spacing.item_spacing = egui::vec2(8.0, 4.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(12);

    ctx.set_style(style);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_font_definitions_embed_metric_compatible_faces() {
        let fonts = production_font_definitions();

        assert!(fonts.font_data.contains_key("liberation-sans"));
        assert!(fonts.font_data.contains_key("liberation-sans-bold"));
        assert_eq!(
            fonts.families[&FontFamily::Proportional].first(),
            Some(&"liberation-sans".to_owned())
        );
        assert_eq!(
            fonts.families[&FontFamily::Name("liberation-sans-bold".into())].first(),
            Some(&"liberation-sans-bold".to_owned())
        );
    }

    #[test]
    fn original_bold_font_selects_the_embedded_named_family() {
        assert_eq!(
            original_bold_font(15.0),
            egui::FontId::new(15.0, FontFamily::Name("liberation-sans-bold".into()),)
        );
    }

    #[test]
    fn load_fonts_installs_the_production_named_family() {
        let ctx = egui::Context::default();
        load_fonts(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |_| {});

        assert!(ctx.fonts(|fonts| {
            fonts
                .families()
                .contains(&FontFamily::Name("liberation-sans-bold".into()))
        }));
    }
}
