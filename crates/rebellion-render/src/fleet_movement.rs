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
//! passing the same camera parameters so coordinates align:
//!
//! ```ignore
//! // Inside draw_galaxy_map, after draw_circle calls:
//! fleet_movement::draw_fleet_overlays(world, movement_state, cam_x, cam_y, zoom, map_width, sh);
//! ```

use macroquad::prelude::*;
use rebellion_core::movement::MovementState;
use rebellion_core::world::GameWorld;

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
// Coordinate helper (mirrors the closure in lib.rs)
// ---------------------------------------------------------------------------

/// Convert game-coordinate space to screen space using current camera settings.
#[inline]
fn to_screen(dat_x: f32, dat_y: f32, cam_x: f32, cam_y: f32, zoom: f32, map_width: f32, sh: f32) -> (f32, f32) {
    let sx = (dat_x - cam_x) * zoom + map_width / 2.0;
    let sy = (dat_y - cam_y) * zoom + sh / 2.0;
    (sx, sy)
}

/// True if a screen-space point is within the visible map area.
#[inline]
fn in_viewport(sx: f32, sy: f32, map_width: f32, sh: f32) -> bool {
    sx > -20.0 && sx < map_width + 20.0 && sy > -20.0 && sy < sh + 20.0
}

// ---------------------------------------------------------------------------
// Drawing primitives
// ---------------------------------------------------------------------------

/// Draw a diamond-shaped fleet icon centered at (cx, cy).
fn draw_fleet_diamond(cx: f32, cy: f32, r: f32, color: Color) {
    // Diamond = 4 lines connecting N/E/S/W points
    draw_line(cx, cy - r, cx + r, cy, 1.5, color); // top-right
    draw_line(cx + r, cy, cx, cy + r, 1.5, color); // bottom-right
    draw_line(cx, cy + r, cx - r, cy, 1.5, color); // bottom-left
    draw_line(cx - r, cy, cx, cy - r, 1.5, color); // top-left
}

/// Draw a dashed line from (x1, y1) to (x2, y2).
fn draw_dashed_line(x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1.0 {
        return;
    }
    let ux = dx / len;
    let uy = dy / len;

    let segment = DASH_LENGTH + DASH_GAP;
    let mut t = 0.0_f32;

    while t < len {
        let dash_end = (t + DASH_LENGTH).min(len);
        draw_line(
            x1 + ux * t,
            y1 + uy * t,
            x1 + ux * dash_end,
            y1 + uy * dash_end,
            1.2,
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
/// Pass the same camera parameters that `draw_galaxy_map` uses.
///
/// # Parameters
/// - `world` — game world (fleet and system data)
/// - `movement_state` — active transit orders
/// - `cam_x / cam_y` — camera world-space position
/// - `zoom` — current zoom factor
/// - `map_width` — screen pixels available for the map (total width minus panel)
/// - `sh` — screen height in pixels
pub fn draw_fleet_overlays(
    world: &GameWorld,
    movement_state: &MovementState,
    cam_x: f32,
    cam_y: f32,
    zoom: f32,
    map_width: f32,
    sh: f32,
) {
    draw_stationary_fleets(world, movement_state, cam_x, cam_y, zoom, map_width, sh);
    draw_transit_routes(world, movement_state, cam_x, cam_y, zoom, map_width, sh);
}

/// Draw diamond icons for fleets that are NOT currently in transit.
fn draw_stationary_fleets(
    world: &GameWorld,
    movement_state: &MovementState,
    cam_x: f32,
    cam_y: f32,
    zoom: f32,
    map_width: f32,
    sh: f32,
) {
    for (fleet_key, fleet) in world.fleets.iter() {
        // Skip fleets that are currently in transit.
        if movement_state.get(fleet_key).is_some() {
            continue;
        }

        let system = match world.systems.get(fleet.location) {
            Some(s) => s,
            None => continue,
        };

        let (sx, sy) = to_screen(
            system.x as f32,
            system.y as f32,
            cam_x,
            cam_y,
            zoom,
            map_width,
            sh,
        );

        if !in_viewport(sx, sy, map_width, sh) {
            continue;
        }

        let color = if fleet.is_alliance {
            ALLIANCE_FLEET_COLOR
        } else {
            EMPIRE_FLEET_COLOR
        };

        let r = (FLEET_ICON_RADIUS * zoom).max(2.5);
        // Offset slightly above the system dot so they don't overlap.
        draw_fleet_diamond(sx, sy - r - 2.0 * zoom, r, color);
    }
}

/// Draw route lines and transit dots for in-transit fleets.
fn draw_transit_routes(
    world: &GameWorld,
    movement_state: &MovementState,
    cam_x: f32,
    cam_y: f32,
    zoom: f32,
    map_width: f32,
    sh: f32,
) {
    for (_fleet_key, order) in movement_state.orders().iter() {
        let origin_sys = match world.systems.get(order.origin) {
            Some(s) => s,
            None => continue,
        };
        let dest_sys = match world.systems.get(order.destination) {
            Some(s) => s,
            None => continue,
        };

        let (ox, oy) = to_screen(
            origin_sys.x as f32,
            origin_sys.y as f32,
            cam_x,
            cam_y,
            zoom,
            map_width,
            sh,
        );
        let (dx, dy) = to_screen(
            dest_sys.x as f32,
            dest_sys.y as f32,
            cam_x,
            cam_y,
            zoom,
            map_width,
            sh,
        );

        // Draw dashed route from origin to destination.
        draw_dashed_line(ox, oy, dx, dy, ROUTE_COLOR);

        // Interpolate fleet position along the route.
        let t = order.progress();
        let fx = ox + (dx - ox) * t;
        let fy = oy + (dy - oy) * t;

        if in_viewport(fx, fy, map_width, sh) {
            // Determine fleet color — look up the fleet to find its faction.
            let color = world
                .fleets
                .get(order.fleet)
                .map(|f| {
                    if f.is_alliance {
                        ALLIANCE_FLEET_COLOR
                    } else {
                        EMPIRE_FLEET_COLOR
                    }
                })
                .unwrap_or(NEUTRAL_FLEET_COLOR);

            let r = (TRANSIT_DOT_RADIUS * zoom).max(2.0);

            // Glow ring behind the dot.
            draw_circle(fx, fy, r + 2.0 * zoom, Color { a: 0.25, ..color });
            draw_circle(fx, fy, r, color);

            // ETA label near the dot when zoomed in enough.
            if zoom > 1.5 {
                let remaining = order.ticks_remaining();
                let label = format!("{remaining}d");
                draw_text(&label, fx + r + 3.0, fy - r, (12.0 * zoom).min(18.0), color);
            }
        }
    }
}
