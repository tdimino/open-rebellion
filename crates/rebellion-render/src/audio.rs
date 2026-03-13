//! Audio UI controls for the galaxy map status bar.
//!
//! This module owns only the **egui rendering** side of audio: volume sliders
//! and a mute toggle.  The actual audio engine (kira `AudioManager`) lives in
//! `rebellion-app/src/audio.rs` to avoid a `links = "alsa"` conflict between
//! `miniquad` (pulled in by `egui-macroquad`) and `cpal` (pulled in by kira).
//!
//! # Integration
//!
//! The app creates an `AudioVolumeState` and passes a `&mut AudioVolumeState`
//! to the audio engine whenever volumes change.  The engine reads
//! `AudioVolumeState` to set kira track volumes.
//!
//! ```ignore
//! egui_macroquad::ui(|ctx| {
//!     draw_audio_controls(ctx, &mut audio_volume_state);
//!     // ... other panels
//! });
//! egui_macroquad::draw();
//! // Apply any volume changes:
//! audio_engine.apply_volume(&audio_volume_state);
//! ```

use egui_macroquad::egui::{self, Color32, RichText};

// ---------------------------------------------------------------------------
// SfxKind — named sound effect identifiers
// ---------------------------------------------------------------------------

/// Named sound effect identifiers.
///
/// Shared between the render crate (for type-safe callbacks) and the app
/// crate (which maps them to file names and kira handles).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SfxKind {
    MissionSuccess,
    MissionFail,
    BuildComplete,
    FleetDeparture,
    FleetArrival,
}

// ---------------------------------------------------------------------------
// MusicTrack — named music track identifiers
// ---------------------------------------------------------------------------

/// Named music track identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicTrack {
    MainTheme,
    Battle,
    Victory,
    Defeat,
}

// ---------------------------------------------------------------------------
// AudioVolumeState — pure UI state, no kira types
// ---------------------------------------------------------------------------

/// Volume and mute state for the audio system.
///
/// Lives in the render layer (no kira dependency).  The app layer reads these
/// values and applies them to the kira audio manager.
///
/// `dirty` is set to `true` whenever a slider or mute toggle changes so the
/// app layer knows to call `apply_volume` without polling every frame.
#[derive(Debug, Clone)]
pub struct AudioVolumeState {
    /// Music volume in [0.0, 1.0].
    pub music_volume: f32,
    /// SFX volume in [0.0, 1.0].
    pub sfx_volume: f32,
    /// If true, all audio output is silenced.
    pub muted: bool,
    /// Set when the user changes a control.  The app layer clears this after
    /// applying the change to the audio engine.
    pub dirty: bool,
    /// Whether the underlying audio backend is available.  Written by the app
    /// layer so the panel can display a "no audio" indicator.
    pub backend_available: bool,
}

impl Default for AudioVolumeState {
    fn default() -> Self {
        AudioVolumeState {
            music_volume: 0.8,
            sfx_volume: 1.0,
            muted: false,
            dirty: false,
            backend_available: true,
        }
    }
}

impl AudioVolumeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Effective music volume: 0.0 when muted, otherwise `music_volume`.
    pub fn effective_music_volume(&self) -> f64 {
        if self.muted { 0.0 } else { self.music_volume as f64 }
    }

    /// Effective SFX volume: 0.0 when muted, otherwise `sfx_volume`.
    pub fn effective_sfx_volume(&self) -> f64 {
        if self.muted { 0.0 } else { self.sfx_volume as f64 }
    }
}

// ---------------------------------------------------------------------------
// draw_audio_controls
// ---------------------------------------------------------------------------

/// Render audio volume controls inside the bottom status bar.
///
/// Call this from within an existing `egui::Ui` horizontal layout (e.g. inside
/// the `status_bar` `TopBottomPanel`), not as a standalone panel.  This keeps
/// it beside the speed controls and day counter.
///
/// Sets `state.dirty = true` when any control changes.
pub fn draw_audio_controls(ui: &mut egui::Ui, state: &mut AudioVolumeState) {
    ui.separator();

    // ── Mute toggle ───────────────────────────────────────────────────────────
    let mute_label = if state.muted { "🔇" } else { "🔊" };
    if ui
        .selectable_label(state.muted, RichText::new(mute_label).small())
        .clicked()
    {
        state.muted = !state.muted;
        state.dirty = true;
    }

    // ── Music volume ──────────────────────────────────────────────────────────
    ui.label(
        RichText::new("Mus")
            .color(Color32::from_gray(160))
            .small(),
    );
    let music_changed = ui
        .add_sized(
            [50.0, 16.0],
            egui::Slider::new(&mut state.music_volume, 0.0_f32..=1.0_f32)
                .show_value(false),
        )
        .changed();
    if music_changed {
        state.dirty = true;
    }

    // ── SFX volume ────────────────────────────────────────────────────────────
    ui.label(
        RichText::new("SFX")
            .color(Color32::from_gray(160))
            .small(),
    );
    let sfx_changed = ui
        .add_sized(
            [50.0, 16.0],
            egui::Slider::new(&mut state.sfx_volume, 0.0_f32..=1.0_f32)
                .show_value(false),
        )
        .changed();
    if sfx_changed {
        state.dirty = true;
    }

    // ── No-audio indicator ────────────────────────────────────────────────────
    if !state.backend_available {
        ui.label(
            RichText::new("(no audio)")
                .small()
                .color(Color32::from_gray(80)),
        );
    }
}
