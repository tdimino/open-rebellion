//! Fog of war rendering overlay for the galaxy map.
//!
//! Applies visual dimming to systems that are not yet visible to the
//! player's faction. Three tiers:
//!
//! 1. **Visible** — full color, normal rendering (handled by `draw_galaxy_map`)
//! 2. **Explored but not visible** — dim dot with name, no asset details
//! 3. **Never seen** — very dim dot, no name label
//!
//! # Integration
//!
//! Call `draw_fog_overlay` *before* `draw_galaxy_map` draws the system dots
//! so that dimmed systems are drawn first and selected/hovered systems
//! overlay them at full brightness:
//!
//! ```ignore
//! // In draw_galaxy_map, before the system draw pass:
//! fog::draw_fog_overlay(world, fog_state, cam_x, cam_y, zoom, map_width, sh);
//! ```
//!
//! Or call it *after* as a darkening pass — either is acceptable since
//! fully-visible systems skip the overlay entirely.

use macroquad::prelude::*;
use rebellion_core::dat::ExplorationStatus;
use rebellion_core::fog::FogState;
use rebellion_core::world::GameWorld;

// ---------------------------------------------------------------------------
// Visual constants
// ---------------------------------------------------------------------------

/// Overlay color for explored-but-not-visible systems (dim with partial alpha).
const FOG_EXPLORED_COLOR: Color = Color {
    r: 0.15,
    g: 0.15,
    b: 0.2,
    a: 0.75,
};

/// Overlay color for completely unseen systems (very dark).
const FOG_UNSEEN_COLOR: Color = Color {
    r: 0.08,
    g: 0.08,
    b: 0.1,
    a: 0.88,
};

/// Dot radius multiplier for dimmed systems (slightly smaller than the normal 3.0).
const FOG_DOT_RADIUS: f32 = 2.5;

// ---------------------------------------------------------------------------
// Coordinate helper (mirrors lib.rs)
// ---------------------------------------------------------------------------

#[inline]
fn to_screen(dat_x: f32, dat_y: f32, cam_x: f32, cam_y: f32, zoom: f32, map_width: f32, sh: f32) -> (f32, f32) {
    let sx = (dat_x - cam_x) * zoom + map_width / 2.0;
    let sy = (dat_y - cam_y) * zoom + sh / 2.0;
    (sx, sy)
}

#[inline]
fn in_viewport(sx: f32, sy: f32, map_width: f32, sh: f32) -> bool {
    sx > -20.0 && sx < map_width + 20.0 && sy > -20.0 && sy < sh + 20.0
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Draw fog-of-war dimming circles over unexplored/invisible systems.
///
/// Systems visible to `fog_state` are skipped entirely — they will be drawn
/// at full brightness by `draw_galaxy_map`. Only hidden systems receive the
/// dim overlay dot.
///
/// # Parameters
/// - `world` — game world (system positions and exploration status)
/// - `fog_state` — current visibility set for the player's faction
/// - `cam_x / cam_y` — camera world-space position (same as `draw_galaxy_map`)
/// - `zoom` — current zoom factor
/// - `map_width` — screen pixels for the map area
/// - `sh` — screen height in pixels
pub fn draw_fog_overlay(
    world: &GameWorld,
    fog_state: &FogState,
    cam_x: f32,
    cam_y: f32,
    zoom: f32,
    map_width: f32,
    sh: f32,
) {
    for (_key, system) in &world.systems {
        // Fully visible systems are rendered at normal brightness elsewhere.
        if fog_state.is_visible(_key) {
            continue;
        }

        let (sx, sy) = to_screen(system.x as f32, system.y as f32, cam_x, cam_y, zoom, map_width, sh);
        if !in_viewport(sx, sy, map_width, sh) {
            continue;
        }

        let r = FOG_DOT_RADIUS * zoom;

        // Choose dim color tier based on whether this system was ever explored
        // in the scenario data (ExplorationStatus comes from SYSTEMSD family_id).
        let color = match system.exploration_status {
            ExplorationStatus::Explored => FOG_EXPLORED_COLOR,
            ExplorationStatus::Unexplored => FOG_UNSEEN_COLOR,
        };

        draw_circle(sx, sy, r, color);

        // Show name for explored-but-not-visible systems at sufficient zoom.
        if system.exploration_status == ExplorationStatus::Explored && zoom > 0.8 {
            let font_size = (14.0 * zoom).clamp(9.0, 20.0);
            draw_text(
                &system.name,
                sx + r + 3.0,
                sy + 4.0,
                font_size,
                Color::new(0.4, 0.4, 0.5, 0.5),
            );
        }
    }
}
