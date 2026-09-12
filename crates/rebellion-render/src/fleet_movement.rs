//! Fleet movement visualization for the galaxy map.
//!
//! Renders three layers on top of the star map:
//! 1. **Stationary fleet icons** — small diamond at each system that has fleets
//! 2. **Transit markers** — moving dot interpolated along the route line
//! 3. **Route lines** — dashed line from origin to destination for in-transit fleets
//!
//! # Integration
//!
//! Call `draw_fleet_overlays` after the system dots are drawn in `draw_galaxy_map`,
//! passing the same camera so coordinates and aperture offsets align:
//!
//! ```ignore
//! // Inside draw_galaxy_map, after draw_circle calls:
//! fleet_movement::draw_fleet_overlays(world, movement_state, &camera);
//! ```

use macroquad::prelude::*;
use rebellion_core::movement::MovementState;
use rebellion_core::world::GameWorld;

use crate::CameraView;

// ---------------------------------------------------------------------------
// Visual constants
// ---------------------------------------------------------------------------

/// Radius of the fleet icon diamond at a stationary system.
const FLEET_ICON_RADIUS: f32 = 4.0;

/// Radius of the in-transit fleet dot.
const TRANSIT_DOT_RADIUS: f32 = 3.5;

/// Length of each dash segment in screen pixels.
const DASH_LENGTH: f32 = 6.0;

/// Gap between dash segments in screen pixels.
const DASH_GAP: f32 = 4.0;

/// Color of the route line (dim cyan).
const ROUTE_COLOR: Color = Color {
    r: 0.3,
    g: 0.8,
    b: 0.9,
    a: 0.5,
};

/// Color of Alliance fleet icons and transit dots.
const ALLIANCE_FLEET_COLOR: Color = Color {
    r: 0.3,
    g: 0.6,
    b: 1.0,
    a: 0.9,
};

/// Color of Empire fleet icons and transit dots.
const EMPIRE_FLEET_COLOR: Color = Color {
    r: 1.0,
    g: 0.3,
    b: 0.3,
    a: 0.9,
};

/// Color for fleets whose faction can't be determined (fallback).
const NEUTRAL_FLEET_COLOR: Color = Color {
    r: 0.7,
    g: 0.7,
    b: 0.7,
    a: 0.9,
};

// ---------------------------------------------------------------------------
// Drawing primitives
// ---------------------------------------------------------------------------

/// Draw a diamond-shaped fleet icon centered at (cx, cy).
fn draw_fleet_diamond(cx: f32, cy: f32, r: f32, line_width: f32, color: Color) {
    // Diamond = 4 lines connecting N/E/S/W points
    draw_line(cx, cy - r, cx + r, cy, line_width, color); // top-right
    draw_line(cx + r, cy, cx, cy + r, line_width, color); // bottom-right
    draw_line(cx, cy + r, cx - r, cy, line_width, color); // bottom-left
    draw_line(cx - r, cy, cx, cy - r, line_width, color); // top-left
}

/// Draw a dashed line from (x1, y1) to (x2, y2).
fn draw_dashed_line(x1: f32, y1: f32, x2: f32, y2: f32, display_scale: f32, color: Color) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1.0 {
        return;
    }
    let ux = dx / len;
    let uy = dy / len;

    let dash_length = DASH_LENGTH * display_scale;
    let segment = (DASH_LENGTH + DASH_GAP) * display_scale;
    let mut t = 0.0_f32;

    while t < len {
        let dash_end = (t + dash_length).min(len);
        draw_line(
            x1 + ux * t,
            y1 + uy * t,
            x1 + ux * dash_end,
            y1 + uy * dash_end,
            1.2 * display_scale,
            color,
        );
        t += segment;
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Draw all fleet overlays on the galaxy map.
///
/// Must be called after the system dots are drawn so fleet icons render on top.
/// Pass the same camera that `draw_galaxy_map` returns.
///
/// # Parameters
/// - `world` — game world (fleet and system data)
/// - `movement_state` — active transit orders
/// - `camera` — the exact transform and faction aperture from `draw_galaxy_map`
pub fn draw_fleet_overlays(world: &GameWorld, movement_state: &MovementState, camera: &CameraView) {
    draw_stationary_fleets(world, movement_state, camera);
    draw_transit_routes(world, movement_state, camera);
}

/// Find the fleet (if any) under the mouse cursor at (mx, my) screen position.
///
/// Checks stationary fleet diamonds and in-transit fleet dots.
/// Returns `Some(FleetKey)` for the nearest fleet within hit radius.
#[must_use]
pub fn hovered_fleet(
    world: &GameWorld,
    movement_state: &MovementState,
    camera: &CameraView,
    mx: f32,
    my: f32,
) -> Option<rebellion_core::ids::FleetKey> {
    if !camera.contains(mx, my) {
        return None;
    }

    let hit_radius =
        (FLEET_ICON_RADIUS * camera.logical_zoom + 4.0).max(8.0) * camera.display_scale;
    let mut best: Option<(rebellion_core::ids::FleetKey, f32)> = None;

    // Check stationary fleets
    for (fleet_key, fleet) in &world.fleets {
        if movement_state.get(fleet_key).is_some() {
            continue;
        }
        let Some(system) = world.systems.get(fleet.location) else {
            continue;
        };
        let (sx, sy) = camera.to_screen(f32::from(system.x), f32::from(system.y));
        if !camera.contains_with_margin(sx, sy, camera.scale_pixels(20.0)) {
            continue;
        }
        let r = (FLEET_ICON_RADIUS * camera.logical_zoom).max(2.5) * camera.display_scale;
        // Diamond is offset above system dot
        let dy = sy - r - 2.0 * camera.zoom;
        let dist = ((mx - sx).powi(2) + (my - dy).powi(2)).sqrt();
        if dist < hit_radius && best.is_none_or(|(_, bd)| dist < bd) {
            best = Some((fleet_key, dist));
        }
    }

    // Check in-transit fleets
    for order in movement_state.orders().values() {
        let Some(origin_sys) = world.systems.get(order.origin) else {
            continue;
        };
        let Some(dest_sys) = world.systems.get(order.destination) else {
            continue;
        };
        let (ox, oy) = camera.to_screen(f32::from(origin_sys.x), f32::from(origin_sys.y));
        let (dx, dy) = camera.to_screen(f32::from(dest_sys.x), f32::from(dest_sys.y));
        let t = order.progress();
        let fx = ox + (dx - ox) * t;
        let fy = oy + (dy - oy) * t;
        let dist = ((mx - fx).powi(2) + (my - fy).powi(2)).sqrt();
        if dist < hit_radius && best.is_none_or(|(_, bd)| dist < bd) {
            best = Some((order.fleet, dist));
        }
    }

    best.map(|(k, _)| k)
}

/// Draw diamond icons for fleets that are NOT currently in transit.
fn draw_stationary_fleets(world: &GameWorld, movement_state: &MovementState, camera: &CameraView) {
    for (fleet_key, fleet) in &world.fleets {
        // Skip fleets that are currently in transit.
        if movement_state.get(fleet_key).is_some() {
            continue;
        }

        let Some(system) = world.systems.get(fleet.location) else {
            continue;
        };

        let (sx, sy) = camera.to_screen(f32::from(system.x), f32::from(system.y));

        if !camera.contains_with_margin(sx, sy, camera.scale_pixels(20.0)) {
            continue;
        }

        let color = if fleet.is_alliance {
            ALLIANCE_FLEET_COLOR
        } else {
            EMPIRE_FLEET_COLOR
        };

        let r = (FLEET_ICON_RADIUS * camera.logical_zoom).max(2.5) * camera.display_scale;
        // Offset slightly above the system dot so they don't overlap.
        draw_fleet_diamond(
            sx,
            sy - r - 2.0 * camera.zoom,
            r,
            camera.scale_pixels(1.5),
            color,
        );
    }
}

/// Draw route lines and transit dots for in-transit fleets.
fn draw_transit_routes(world: &GameWorld, movement_state: &MovementState, camera: &CameraView) {
    for order in movement_state.orders().values() {
        let Some(origin_sys) = world.systems.get(order.origin) else {
            continue;
        };
        let Some(dest_sys) = world.systems.get(order.destination) else {
            continue;
        };

        let (ox, oy) = camera.to_screen(f32::from(origin_sys.x), f32::from(origin_sys.y));
        let (dx, dy) = camera.to_screen(f32::from(dest_sys.x), f32::from(dest_sys.y));

        // Draw dashed route from origin to destination.
        draw_dashed_line(ox, oy, dx, dy, camera.display_scale, ROUTE_COLOR);

        // Interpolate fleet position along the route.
        let t = order.progress();
        let fx = ox + (dx - ox) * t;
        let fy = oy + (dy - oy) * t;

        if camera.contains_with_margin(fx, fy, camera.scale_pixels(20.0)) {
            // Determine fleet color — look up the fleet to find its faction.
            let color = world
                .fleets
                .get(order.fleet)
                .map_or(NEUTRAL_FLEET_COLOR, |f| {
                    if f.is_alliance {
                        ALLIANCE_FLEET_COLOR
                    } else {
                        EMPIRE_FLEET_COLOR
                    }
                });

            let r = (TRANSIT_DOT_RADIUS * camera.logical_zoom).max(2.0) * camera.display_scale;

            // Glow ring behind the dot.
            draw_circle(fx, fy, r + 2.0 * camera.zoom, Color { a: 0.25, ..color });
            draw_circle(fx, fy, r, color);

            // ETA label near the dot when zoomed in enough.
            if camera.logical_zoom > 1.5 {
                let remaining = order.ticks_remaining();
                let label = format!("{remaining}d");
                draw_text(
                    &label,
                    fx + r + 3.0,
                    fy - r,
                    (12.0 * camera.logical_zoom).min(18.0) * camera.display_scale,
                    color,
                );
            }
        }
    }
}
