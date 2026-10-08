//! The galaxy view's sector name under the pointer.
//!
//! `FUN_00422ce0`'s `WM_MOUSEMOVE` walks the view list at `+0x24c` and
//! marks the first item whose rectangle (`+0x40..+0x4c`) holds the cursor;
//! `WM_PAINT` then draws that item's name (`+0x14`) with the label at
//! `+0x98`, anchored at the item's position (`+0x28`, `+0x2c`). The label
//! shows and hides with the pointer, with no delay, and nothing names a
//! single system.

use egui_macroquad::egui;
use rebellion_core::ids::SectorKey;
use rebellion_core::world::GameWorld;

use crate::cockpit::{self, CockpitFaction, CockpitLayout};
use crate::theme::game_font_on;

/// hyp: font 2 (14 pixels, bold). The original's label font is set outside
/// the traced code. In a capture of the original, "Sesswenna" and "Sector"
/// measure 69.6 and 39.6 pixels, a 12.8-pixel em in Arial's bold metrics,
/// which font 2's 12.5-pixel em fits within GDI's whole-pixel rounding.
const FONT_ENTRY: u8 = 2;
/// hyp: the capture's line pitch, 16 pixels from one line's cap top to the
/// next.
const LINE_PITCH: f32 = 16.0;
/// hyp: VGA yellow, measured (255, 255, 84) in a capture of the original.
const COLOR: egui::Color32 = egui::Color32::from_rgb(255, 255, 85);
/// hyp: the label wraps at the 60 pixels `WM_MOUSEMOVE` adds to the item's
/// repaint rectangle (`+0x3c`); "Sesswenna Sector" wraps after its first
/// word in the capture.
const WRAP_WIDTH: f32 = 60.0;
/// hyp: the margin around a sector's systems that its rectangle covers. The
/// items' rectangles (`FUN_005ae460`) are not traced.
const MARGIN: f32 = 6.0;

/// A galaxy point in the 640x480 canvas, as `FUN_00425d00` maps it.
#[must_use]
pub fn galaxy_canvas_point(faction: CockpitFaction, x: u16, y: u16) -> (f32, f32) {
    let (offset_x, offset_y) = cockpit::galaxy_backdrop_offset(faction);
    let (scale_x, scale_y) = crate::GALAXY_UNITS_TO_PIXELS;
    (
        (f32::from(x) * scale_x).trunc() + offset_x,
        (f32::from(y) * scale_y).trunc() + offset_y,
    )
}

/// The sector whose systems' area holds a canvas point.
#[must_use]
pub fn sector_at(
    world: &GameWorld,
    faction: CockpitFaction,
    point: (f32, f32),
) -> Option<SectorKey> {
    world.sectors.iter().find_map(|(key, sector)| {
        let mut points = sector.systems.iter().filter_map(|system| {
            world
                .systems
                .get(*system)
                .map(|system| galaxy_canvas_point(faction, system.x, system.y))
        });
        let first = points.next()?;
        let (min, max) = points.fold((first, first), |(min, max), (x, y)| {
            ((min.0.min(x), min.1.min(y)), (max.0.max(x), max.1.max(y)))
        });
        (point.0 >= min.0 - MARGIN
            && point.0 <= max.0 + MARGIN
            && point.1 >= min.1 - MARGIN
            && point.1 <= max.1 + MARGIN)
            .then_some(key)
    })
}

/// Break `name` into lines as `DrawTextA`'s `DT_WORDBREAK` does: only at
/// spaces, starting a new line when the next word would pass `max_width`.
/// A word wider than `max_width` keeps its line whole and runs past it.
fn word_break(ctx: &egui::Context, name: &str, font_id: &egui::FontId, max_width: f32) -> String {
    let width = |text: &str| {
        ctx.fonts(|fonts| {
            fonts
                .layout_no_wrap(text.to_owned(), font_id.clone(), COLOR)
                .size()
                .x
        })
    };
    let mut lines: Vec<String> = Vec::new();
    for word in name.split(' ').filter(|word| !word.is_empty()) {
        match lines.last_mut() {
            Some(line) if width(&format!("{line} {word}")) <= max_width => {
                line.push(' ');
                line.push_str(word);
            }
            _ => lines.push(word.to_owned()),
        }
    }
    lines.join("\n")
}

/// The label's text, broken at `WRAP_WIDTH` with `LINE_PITCH` lines.
fn label_job(
    ctx: &egui::Context,
    name: &str,
    font_id: egui::FontId,
    scale: f32,
) -> egui::text::LayoutJob {
    let text = word_break(ctx, name, &font_id, WRAP_WIDTH * scale);
    egui::text::LayoutJob::single_section(
        text,
        egui::TextFormat {
            font_id,
            color: COLOR,
            line_height: Some(LINE_PITCH * scale),
            ..Default::default()
        },
    )
}

/// Name the sector under the pointer at the sector's own map position.
/// Nothing shows while the pointer is over a window or outside the galaxy
/// aperture.
pub fn draw_sector_hover_label(
    ctx: &egui::Context,
    world: &GameWorld,
    layout: CockpitLayout,
    faction: CockpitFaction,
) {
    if layout.scale <= 0.0 || ctx.is_pointer_over_area() {
        return;
    }
    let Some(pointer) = ctx.input(|input| input.pointer.hover_pos()) else {
        return;
    };
    let aperture = layout.galaxy;
    if pointer.x < aperture.x
        || pointer.x >= aperture.x + aperture.width
        || pointer.y < aperture.y
        || pointer.y >= aperture.y + aperture.height
    {
        return;
    }
    let point = (
        (pointer.x - layout.canvas.x) / layout.scale,
        (pointer.y - layout.canvas.y) / layout.scale,
    );
    let Some(sector) = sector_at(world, faction, point).and_then(|key| world.sectors.get(key))
    else {
        return;
    };
    let (x, y) = galaxy_canvas_point(faction, sector.x, sector.y);
    let painter = ctx.layer_painter(egui::LayerId::background());
    let galley = painter.layout_job(label_job(
        ctx,
        &sector.name,
        game_font_on(ctx, FONT_ENTRY, layout.scale),
        layout.scale,
    ));
    painter.galley(
        egui::pos2(
            layout.canvas.x + x * layout.scale,
            layout.canvas.y + y * layout.scale,
        ),
        galley,
        COLOR,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{ControlKind, Sector, System};

    #[test]
    fn a_sector_sits_where_the_galaxy_transform_puts_it() {
        // FUN_00425d00 with the Empire's (0x54, 0x1b): Sesswenna's SECTORSD
        // (317, 248) lands on (271, 132). A capture of the original's
        // "Sesswenna Sector" label starts at x 271.8, its glyphs at y 135.
        assert_eq!(
            galaxy_canvas_point(CockpitFaction::Empire, 317, 248),
            (271.0, 132.0)
        );
        assert_eq!(
            galaxy_canvas_point(CockpitFaction::Alliance, 0, 0),
            (21.0, 25.0)
        );
    }

    #[test]
    fn the_label_breaks_only_between_words_on_a_sixteen_pixel_pitch() {
        // hyp (measured): the original's "Sesswenna Sector" breaks after its
        // first word, whole, with lines 16 pixels apart; DT_WORDBREAK never
        // splits a word wider than the line.
        let ctx = egui::Context::default();
        let mut galley = None;
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            let job = label_job(
                ctx,
                "Sesswenna Sector",
                crate::theme::game_font(FONT_ENTRY, 2.0),
                2.0,
            );
            galley = Some(ctx.fonts(|fonts| fonts.layout_job(job)));
        });
        let galley = galley.unwrap();
        let rows: Vec<String> = galley
            .rows
            .iter()
            .map(|row| row.glyphs.iter().map(|glyph| glyph.chr).collect())
            .collect();
        assert_eq!(rows, ["Sesswenna", "Sector"]);
        assert!(galley.rows[0].rect.width() > 60.0 * 2.0);
        assert!((galley.rows[1].min_y() - galley.rows[0].min_y() - 32.0).abs() < 0.01);
    }

    #[test]
    fn the_pointer_finds_the_sector_around_its_systems() {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(36),
            name: "Sesswenna Sector".into(),
            group: SectorGroup::Core,
            x: 317,
            y: 248,
            systems: Vec::new(),
        });
        let mut add = |x: u16, y: u16| {
            let system = world.systems.insert(System {
                dat_id: DatId::new(u32::from(x)),
                name: String::new(),
                sector,
                x,
                y,
                exploration_status: ExplorationStatus::Explored,
                popularity_alliance: 0.0,
                popularity_empire: 0.0,
                is_populated: true,
                total_energy: 0,
                raw_materials: 0,
                espionage_rating: 0.0,
                fleets: Vec::new(),
                ground_units: Vec::new(),
                special_forces: Vec::new(),
                defense_facilities: Vec::new(),
                manufacturing_facilities: Vec::new(),
                production_facilities: Vec::new(),
                is_headquarters: false,
                is_destroyed: false,
                control: ControlKind::Uncontrolled,
            });
            world.sectors[sector].systems.push(system);
        };
        add(330, 260);
        add(400, 330);
        let (left, top) = galaxy_canvas_point(CockpitFaction::Empire, 330, 260);
        let (right, bottom) = galaxy_canvas_point(CockpitFaction::Empire, 400, 330);
        let middle = ((left + right) / 2.0, (top + bottom) / 2.0);
        assert_eq!(
            sector_at(&world, CockpitFaction::Empire, middle),
            Some(sector)
        );
        assert_eq!(
            sector_at(&world, CockpitFaction::Empire, (left - 6.0, top - 6.0)),
            Some(sector)
        );
        assert_eq!(
            sector_at(&world, CockpitFaction::Empire, (left - 7.0, top)),
            None
        );
        assert_eq!(
            sector_at(&world, CockpitFaction::Empire, (right + 7.0, bottom)),
            None
        );
    }
}
