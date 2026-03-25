//! Full-screen event screen overlay for story events and mission results.
//!
//! The original game shows a 400×200 BMP over the screen when scripted story
//! events fire (Luke training on Dagobah, the Final Battle, etc.).  Assets
//! live in STRATEGY.DLL at resource IDs 6000–6268 — 61 character dialogue
//! scenes at 400×200.
//!
//! This module provides:
//! - [`EventScreenState`] — mutable overlay state held by the app.
//! - [`show_event_screen`] — trigger an overlay by story-event ID.
//! - [`draw_event_screen`] — egui draw call; returns `true` when dismissed.
//!
//! # Mapping
//!
//! Story event IDs are the constants in `rebellion_core::story_events`
//! (`0x100`–`0x1ff`).  We map them to STRATEGY.DLL resource IDs using a
//! small lookup table derived from the order the original game uses them.
//!
//! # Auto-dismiss
//!
//! The overlay auto-dismisses after [`AUTO_DISMISS_SECS`] seconds, or
//! immediately when the player clicks anywhere on it.
//!
//! # WASM
//!
//! `BmpCache` returns `None` on WASM — the fallback is a dark overlay with a
//! story-event title card rendered in egui text.

use egui_macroquad::egui::{self, Color32, RichText};
use macroquad::prelude::screen_width;

use crate::bmp_cache::{BmpCache, DllSource};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Seconds before the overlay auto-dismisses.
pub const AUTO_DISMISS_SECS: f32 = 5.0;

/// STRATEGY.DLL base ID for the character event dialogue scenes.
const STRATEGY_EVENT_BASE: u32 = 6208;

// ---------------------------------------------------------------------------
// Story-event → resource ID mapping
// ---------------------------------------------------------------------------

/// Map a story-event ID (from `rebellion_core::events` constants) to
/// the STRATEGY.DLL resource ID for the best-matching event screen BMP.
///
/// The STRATEGY.DLL 6208–6268 block contains 61 character dialogue scenes at
/// 400×200.  The mapping assigns each story chain to the character portrait
/// most central to that story beat.
///
/// # Offset layout (0-indexed within 6208–6268)
///
/// | Offset | Character |
/// |--------|-----------|
/// |  0–7   | Luke Skywalker |
/// |  8–15  | Princess Leia |
/// | 16–23  | Han Solo |
/// | 24–31  | Darth Vader |
/// | 32–39  | Emperor Palpatine |
/// | 40–47  | Mon Mothma |
/// | 48–55  | Jabba the Hutt |
/// | 56–60  | Boba Fett / Bounty Hunters |
pub fn event_id_to_resource(story_event_id: u32) -> Option<u32> {
    // Constants from rebellion_core::events
    const EVT_CHARACTER_FORCE:  u32 = 0x1e1; // Luke's Force potential noticed
    const EVT_FORCE_TRAINING:   u32 = 0x1e5; // Luke begins Force training
    const EVT_LUKE_DAGOBAH:     u32 = 0x221; // Luke departs for Dagobah
    const EVT_DAGOBAH_COMPLETED:u32 = 0x210; // Luke completes Dagobah training
    const EVT_FINAL_BATTLE:     u32 = 0x220; // Final battle triggered
    const EVT_BOUNTY_ATTACK:    u32 = 0x212; // Bounty hunters capture Han

    // Additional chain IDs from story_events.rs
    // 0x390-0x39A range: extended story chains
    const EVT_LEIA_PLAN:        u32 = 0x381; // Leia plans rescue
    const EVT_LEIA_RESCUE:      u32 = 0x382; // Leia rescue mission begins
    const EVT_JABBA_DEMAND:     u32 = 0x380; // Jabba's initial demand
    const EVT_JABBA_END:        u32 = 0x383; // Jabba arc resolution

    let offset: u32 = match story_event_id {
        EVT_CHARACTER_FORCE   => 0,  // Luke — Force awakening
        EVT_FORCE_TRAINING    => 2,  // Luke — training begins
        EVT_LUKE_DAGOBAH      => 4,  // Luke — departs for Dagobah
        EVT_DAGOBAH_COMPLETED => 6,  // Luke — training complete
        EVT_FINAL_BATTLE      => 32, // Emperor Palpatine — final battle
        EVT_BOUNTY_ATTACK     => 16, // Han Solo — captured
        EVT_LEIA_PLAN         => 8,  // Princess Leia — rescue plan
        EVT_LEIA_RESCUE       => 10, // Princess Leia — rescue mission
        EVT_JABBA_DEMAND      => 48, // Jabba the Hutt — demands Solo
        EVT_JABBA_END         => 50, // Jabba arc end
        // Vader / Emperor related events in 0x390-0x39A range
        0x390 => 24, // Vader — Empire strikes
        0x391 => 26, // Vader — confrontation
        0x392 => 28, // Vader — ultimatum
        0x393 => 33, // Emperor — watches
        0x394 => 35, // Emperor — intervenes
        0x396 => 37, // Emperor — final warning
        0x397 => 56, // Bounty hunters dispatched
        0x398 => 58, // Bounty hunters closing in
        0x399 => 40, // Mon Mothma — Alliance mobilizes
        0x39A => 42, // Mon Mothma — final stand
        _ => return None,
    };

    Some(STRATEGY_EVENT_BASE + offset.min(60))
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Mutable state for the event screen overlay.
///
/// Hold this in the app loop and pass it to [`draw_event_screen`] each frame.
/// Trigger an overlay with [`show_event_screen`].
#[derive(Debug, Default)]
pub struct EventScreenState {
    /// Active overlay; `None` = no overlay this frame.
    active: Option<ActiveOverlay>,
}

#[derive(Debug)]
struct ActiveOverlay {
    /// STRATEGY.DLL resource ID to display (or `None` = text-only fallback).
    resource_id: Option<u32>,
    /// Human-readable title for the text fallback.
    title: String,
    /// Brief description (shown in text fallback and as tooltip).
    description: String,
    /// Time remaining before auto-dismiss (seconds).
    timer: f32,
}

impl EventScreenState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if an overlay is currently active.
    pub fn is_active(&self) -> bool {
        self.active.is_some()
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Trigger an event screen overlay.
///
/// `story_event_id` is a `rebellion_core::story_events` constant.
/// If the ID has no sprite mapping, a text-only fallback card is shown.
///
/// Calling this when an overlay is already active replaces it.
pub fn show_event_screen(
    state: &mut EventScreenState,
    story_event_id: u32,
    title: impl Into<String>,
    description: impl Into<String>,
) {
    let resource_id = event_id_to_resource(story_event_id);
    state.active = Some(ActiveOverlay {
        resource_id,
        title: title.into(),
        description: description.into(),
        timer: AUTO_DISMISS_SECS,
    });
}

/// Trigger an event screen overlay with an explicit STRATEGY.DLL resource ID.
///
/// Use this when you know the exact ID (e.g. for encyclopedia or custom events)
/// rather than deriving it from a story-event constant.
pub fn show_event_screen_raw(
    state: &mut EventScreenState,
    resource_id: u32,
    title: impl Into<String>,
    description: impl Into<String>,
) {
    state.active = Some(ActiveOverlay {
        resource_id: Some(resource_id),
        title: title.into(),
        description: description.into(),
        timer: AUTO_DISMISS_SECS,
    });
}

/// Advance the overlay timer by `dt` seconds.
///
/// Call once per frame **before** `draw_event_screen`.
pub fn update_event_screen(state: &mut EventScreenState, dt: f32) {
    if let Some(overlay) = &mut state.active {
        overlay.timer -= dt;
        if overlay.timer <= 0.0 {
            state.active = None;
        }
    }
}

/// Draw the event screen overlay.
///
/// Call inside `egui_macroquad::ui(|ctx| { ... })`.
///
/// Returns `true` if the overlay was dismissed this frame (click or timeout).
/// The caller should also check [`EventScreenState::is_active`] to detect
/// timer-based dismissal handled by [`update_event_screen`].
pub fn draw_event_screen(
    ctx: &egui::Context,
    state: &mut EventScreenState,
    cache: &mut BmpCache,
) -> bool {
    let overlay = match &state.active {
        Some(o) => o,
        None => return false,
    };

    let mut dismissed = false;

    // Full-screen translucent backdrop — blocks clicks on the galaxy map.
    let screen_rect = ctx.screen_rect();

    egui::Area::new(egui::Id::new("event_screen_backdrop"))
        .fixed_pos(egui::pos2(0.0, 0.0))
        .order(egui::Order::Background)
        .interactable(false)
        .show(ctx, |ui| {
            ui.allocate_exact_size(
                screen_rect.size(),
                egui::Sense::hover(),
            );
            let painter = ui.painter();
            painter.rect_filled(
                screen_rect,
                0.0,
                Color32::from_rgba_unmultiplied(0, 0, 0, 180),
            );
        });

    // Centered event screen panel.
    egui::Area::new(egui::Id::new("event_screen_overlay"))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let resource_id = overlay.resource_id;
            let title = overlay.title.clone();
            let description = overlay.description.clone();
            let timer = overlay.timer;

            // Try to show the BMP sprite.
            let sprite_shown = if let Some(res_id) = resource_id {
                if let Some(tex) = cache.get(ctx, DllSource::Strategy, res_id) {
                    // Original size 400×200; scale to ≤70% of screen width.
                    let max_w = (screen_width() * 0.7).min(600.0);
                    let aspect = 400.0 / 200.0;
                    let img_w = max_w;
                    let img_h = img_w / aspect;
                    let tex_id = tex.id();
                    let img = egui::Image::new((tex_id, egui::vec2(img_w, img_h)));
                    let response = ui.add(img.sense(egui::Sense::click()));
                    if response.clicked() {
                        dismissed = true;
                    }
                    true
                } else {
                    false
                }
            } else {
                false
            };

            // Text fallback or caption below sprite.
            if !sprite_shown {
                egui::Frame::new()
                    .fill(Color32::from_rgba_unmultiplied(10, 15, 30, 230))
                    .inner_margin(egui::Margin::same(24))
                    .show(ui, |ui| {
                        ui.set_width(420.0);
                        ui.vertical_centered(|ui| {
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new(&title)
                                    .size(18.0)
                                    .color(Color32::from_rgb(230, 200, 100)),
                            );
                            ui.add_space(12.0);
                            ui.label(
                                RichText::new(&description)
                                    .size(13.0)
                                    .color(Color32::from_rgb(200, 195, 185)),
                            );
                            ui.add_space(16.0);
                            ui.label(
                                RichText::new("[ Click to continue ]")
                                    .size(11.0)
                                    .color(Color32::from_rgb(140, 135, 120)),
                            );
                            ui.add_space(8.0);
                        });
                        if ui.rect_contains_pointer(ui.min_rect())
                            && ui.input(|i| i.pointer.any_click())
                        {
                            dismissed = true;
                        }
                    });
            } else {
                // Progress bar below sprite showing time remaining.
                let ratio = (timer / AUTO_DISMISS_SECS).clamp(0.0, 1.0);
                let bar_w = (screen_width() * 0.7).min(600.0);
                let (bar_rect, _) = ui.allocate_exact_size(
                    egui::vec2(bar_w, 4.0),
                    egui::Sense::hover(),
                );
                let painter = ui.painter();
                painter.rect_filled(bar_rect, 0.0, Color32::from_rgb(40, 40, 55));
                let mut filled = bar_rect;
                filled.set_width(bar_w * ratio);
                painter.rect_filled(filled, 0.0, Color32::from_rgb(100, 160, 255));

                // Click-to-dismiss hint below bar.
                ui.vertical_centered(|ui| {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("[ Click to continue ]")
                            .size(10.0)
                            .color(Color32::from_rgb(120, 115, 105)),
                    );
                });
            }
        });

    if dismissed {
        state.active = None;
    }

    dismissed
}
