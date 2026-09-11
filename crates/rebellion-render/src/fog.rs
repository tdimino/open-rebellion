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
//! fog::draw_fog_overlay(world, fog_state, &camera);
//! ```
//!
//! Or call it *after* as a darkening pass — either is acceptable since
//! fully-visible systems skip the overlay entirely.

use macroquad::prelude::*;
use rebellion_core::dat::ExplorationStatus;
use rebellion_core::fog::FogState;
use rebellion_core::world::GameWorld;

use crate::CameraView;

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
/// - `camera` — the exact transform and faction aperture from `draw_galaxy_map`
pub fn draw_fog_overlay(world: &GameWorld, fog_state: &FogState, camera: &CameraView) {
    for (system_key, system) in &world.systems {
        // Fully visible systems are rendered at normal brightness elsewhere.
        if fog_state.is_visible(system_key) {
            continue;
        }

        let (sx, sy) = camera.to_screen(f32::from(system.x), f32::from(system.y));
        if !camera.contains_with_margin(sx, sy, camera.scale_pixels(20.0)) {
            continue;
        }

        let r = FOG_DOT_RADIUS * camera.zoom;

        // Choose dim color tier based on whether this system was ever explored
        // in the scenario data (ExplorationStatus comes from SYSTEMSD family_id).
        let color = match system.exploration_status {
            ExplorationStatus::Explored => FOG_EXPLORED_COLOR,
            ExplorationStatus::Unexplored => FOG_UNSEEN_COLOR,
        };

        draw_circle(sx, sy, r, color);

        // Show name for explored-but-not-visible systems at sufficient zoom.
        if system.exploration_status == ExplorationStatus::Explored && camera.logical_zoom > 0.8 {
            let font_size = (14.0 * camera.logical_zoom).clamp(9.0, 20.0) * camera.display_scale;
            draw_text(
                &system.name,
                sx + r + camera.scale_pixels(3.0),
                sy + camera.scale_pixels(4.0),
                font_size,
                Color::new(0.4, 0.4, 0.5, 0.5),
            );
        }
    }
}
