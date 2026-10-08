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
    fonts.families.insert(
        FontFamily::Name(GAME_BOLD_FAMILY.into()),
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

/// The pixel height `FUN_0060eed0` gives the game's font entry `entry`
/// (`lfHeight`): 1, 5 and the default 16; 2 and 10 14; 3 8; 6 and 7 18;
/// 8, 9 and 13 12; 11 24; 12 30. Any other entry takes the default.
#[must_use]
pub const fn game_font_height(entry: u8) -> f32 {
    match entry {
        2 | 10 => 14.0,
        3 => 8.0,
        6 | 7 => 18.0,
        8 | 9 | 13 => 12.0,
        11 => 24.0,
        12 => 30.0,
        _ => 16.0,
    }
}

/// Whether `FUN_0060eed0` makes entry `entry` bold: weight 700 for 1, 2, 5
/// and 7, and 900 for 13; every other entry is 400.
#[must_use]
pub const fn game_font_is_bold(entry: u8) -> bool {
    matches!(entry, 1 | 2 | 5 | 7 | 13)
}

/// Em size per pixel of cell height for Arial and Liberation Sans: 2048
/// units per em over a 1854 + 434 Windows ascent and descent. A positive
/// `lfHeight` asks GDI for that cell height, while egui sizes a font by its
/// em, so the game's heights shrink by this before reaching egui.
const EM_PER_CELL: f32 = 2048.0 / 2288.0;

/// The egui size of the game's font entry `entry`: its cell height as an em.
#[must_use]
pub const fn game_font_size(entry: u8) -> f32 {
    game_font_height(entry) * EM_PER_CELL
}

/// The game's font entry `entry` at `scale`, in Liberation Sans, which has
/// Arial's metrics. port: drawn at the regular weight; a bold entry is not
/// yet bold.
#[must_use]
pub fn game_font(entry: u8, scale: f32) -> egui::FontId {
    egui::FontId::proportional(game_font_size(entry) * scale)
}

/// The egui family `load_fonts` registers for the game's bold entries.
const GAME_BOLD_FAMILY: &str = "game-bold";

/// `game_font`, in bold for a bold entry when `ctx` has the bold face
/// loaded; without it, as in tests, the regular face stands in.
#[must_use]
pub fn game_font_on(ctx: &egui::Context, entry: u8, scale: f32) -> egui::FontId {
    let bold = FontFamily::Name(GAME_BOLD_FAMILY.into());
    if game_font_is_bold(entry) && ctx.fonts(|fonts| fonts.families().contains(&bold)) {
        egui::FontId::new(game_font_size(entry) * scale, bold)
    } else {
        game_font(entry, scale)
    }
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
        assert_eq!(
            fonts.families[&FontFamily::Name(GAME_BOLD_FAMILY.into())].first(),
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

    #[test]
    fn game_fonts_take_the_heights_and_weights_of_the_original_table() {
        // FUN_0060eed0: the lfHeight and lfWeight switch over the entry.
        let heights: Vec<f32> = (0..=14).map(game_font_height).collect();
        assert_eq!(
            heights,
            [
                16.0, 16.0, 14.0, 8.0, 16.0, 16.0, 18.0, 18.0, 12.0, 12.0, 14.0, 24.0, 30.0, 12.0,
                16.0
            ]
        );
        let bold: Vec<u8> = (0..=14).filter(|entry| game_font_is_bold(*entry)).collect();
        assert_eq!(bold, [1, 2, 5, 7, 13]);
        // A 14-pixel cell is a 12.53-pixel em (Liberation Sans OS/2 and
        // head tables: 2048 units per em, ascent 1854, descent 434).
        assert!((game_font_size(10) - 12.531_469).abs() < 1e-4);
        assert_eq!(
            game_font(10, 2.0),
            egui::FontId::proportional(game_font_size(10) * 2.0)
        );
    }
}
