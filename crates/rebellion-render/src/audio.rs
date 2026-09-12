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
/// crate (which maps them to file names and `quad-snd` handles).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SfxKind {
    MissionSuccess,
    MissionFail,
    BuildComplete,
    FleetDeparture,
    FleetArrival,
    /// Space battle begins at a system.
    CombatStart,
    /// Clicked a button or opened a panel.
    UiClick,
    /// Closed a panel or dismissed a dialog.
    UiClose,
    /// Original COMMON.DLL WAVE 8000, assigned to galaxy-size controls.
    MenuGalaxySize,
    /// Original COMMON.DLL WAVE 8001, assigned to Load/Options.
    MenuLoadOptions,
    /// Original COMMON.DLL WAVE 8002, assigned to Quit.
    MenuQuit,
    /// Original COMMON.DLL WAVE 8004, assigned to the remaining cockpit controls.
    MenuSelect,
}

// ---------------------------------------------------------------------------
// MusicTrack — named music track identifiers
// ---------------------------------------------------------------------------

/// Named music track identifiers.
///
/// Maps 1:1 to WAV files under `data/sounds/music/`.  Files are named
/// `{lowercase_variant}.wav` (e.g. `main_theme.wav`, `battle.wav`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicTrack {
    MainTheme,
    Battle,
    Victory,
    Defeat,
    /// Emperor Arrives / Death of Yoda / Obi-Wan Revelation (MDATA.306)
    Imperial,
    /// Battle of Hoth medley (MDATA.312)
    Hoth,
    /// Battle of Endor III medley (MDATA.315)
    Endor,
}

// ---------------------------------------------------------------------------
// MusicContext — high-level game context → track selection
// ---------------------------------------------------------------------------

/// High-level game context used by the app to pick a `MusicTrack`.
///
/// The audio engine maps context to track so callers do not hard-code tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicContext {
    /// Title screen and setup screens.
    MainMenu,
    /// Galaxy map during normal play.
    GalaxyMap,
    /// Active space battle (tactical or auto-resolve).
    Combat,
    /// Game won.
    Victory,
    /// Game lost.
    Defeat,
}

// ---------------------------------------------------------------------------
// VoiceLine — faction voice line identifiers
// ---------------------------------------------------------------------------

/// Voice line event identifiers.
///
/// The audio engine maps these to WAV files in
/// `data/sounds/voice/{alliance,empire}/{id}.wav`.
/// Resource IDs correspond to VOICEFXA.DLL (14001–15163) and
/// VOICEFXE.DLL (15001–15132) extraction outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoiceLine {
    /// Generic mission success acknowledgement.
    AllianceMissionSuccess,
    /// Generic mission failure.
    AllianceMissionFail,
    /// Fleet departure order confirmed.
    AllianceFleetDeparts,
    /// Construction complete at a system.
    AllianceBuildComplete,
    /// Empire mission success acknowledgement.
    EmpireMissionSuccess,
    /// Empire mission failure.
    EmpireMissionFail,
    /// Empire fleet departure.
    EmpireFleetDeparts,
    /// Empire construction complete.
    EmpireBuildComplete,
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
    /// If true, music is silenced while sound effects remain audible.
    pub music_muted: bool,
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
            music_muted: false,
            dirty: false,
            backend_available: true,
        }
    }
}

impl AudioVolumeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Effective music volume: 0.0 when globally or music-only muted.
    pub fn effective_music_volume(&self) -> f64 {
        if self.muted || self.music_muted {
            0.0
        } else {
            self.music_volume as f64
        }
    }

    /// Effective SFX volume: 0.0 when muted, otherwise `sfx_volume`.
    pub fn effective_sfx_volume(&self) -> f64 {
        if self.muted {
            0.0
        } else {
            self.sfx_volume as f64
        }
    }

    /// Toggle music independently of sound effects.
    pub fn toggle_music(&mut self) {
        self.music_muted = !self.music_muted;
        self.dirty = true;
    }

    /// Whether the dedicated music path is enabled.
    pub fn music_enabled(&self) -> bool {
        !self.music_muted
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
    ui.label(RichText::new("Mus").color(Color32::from_gray(160)).small());
    let music_changed = ui
        .add_sized(
            [50.0, 16.0],
            egui::Slider::new(&mut state.music_volume, 0.0_f32..=1.0_f32).show_value(false),
        )
        .changed();
    if music_changed {
        state.dirty = true;
    }

    // ── SFX volume ────────────────────────────────────────────────────────────
    ui.label(RichText::new("SFX").color(Color32::from_gray(160)).small());
    let sfx_changed = ui
        .add_sized(
            [50.0, 16.0],
            egui::Slider::new(&mut state.sfx_volume, 0.0_f32..=1.0_f32).show_value(false),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mute_and_gain_produce_exact_effective_levels() {
        let mut state = AudioVolumeState {
            music_volume: 0.35,
            sfx_volume: 0.6,
            ..Default::default()
        };
        assert_eq!(state.effective_music_volume(), 0.35_f32 as f64);
        assert_eq!(state.effective_sfx_volume(), 0.6_f32 as f64);

        state.muted = true;
        assert_eq!(state.effective_music_volume(), 0.0);
        assert_eq!(state.effective_sfx_volume(), 0.0);
    }

    #[test]
    fn music_toggle_preserves_sound_effects() {
        let mut state = AudioVolumeState::default();
        state.toggle_music();

        assert!(state.music_muted);
        assert!(state.dirty);
        assert_eq!(state.effective_music_volume(), 0.0);
        assert_eq!(state.effective_sfx_volume(), 1.0);

        state.toggle_music();
        assert!(state.music_enabled());
        assert_eq!(state.effective_music_volume(), 0.8_f32 as f64);
    }
}
