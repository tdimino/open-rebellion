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
// MdataTrack and MusicSelector — the original's 16 tracks and default pick
// ---------------------------------------------------------------------------

/// A music track: the file `MDATA.(300 + index)` for index 0..15, the table
/// at `0x006a8478` that `FUN_00417520` plays from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MdataTrack(u8);

impl MdataTrack {
    /// The track a session-less pick returns (`FUN_0041d3b0`): the menu's.
    pub const MENU: Self = Self(0);

    /// `FUN_00417520` plays only `track < 0x10`.
    #[must_use]
    pub const fn new(index: u8) -> Option<Self> {
        if index < 16 {
            Some(Self(index))
        } else {
            None
        }
    }

    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// The file name under `MDATA\`.
    #[must_use]
    pub fn file_name(self) -> String {
        format!("MDATA.{}", 300 + u16::from(self.0))
    }

    /// The Battle Alert's track (`FUN_0044f860`): 10 when the window's side
    /// (`+0x9c`) is the Alliance, 11 otherwise.
    #[must_use]
    pub const fn battle_alert(player_alliance: bool) -> Self {
        Self(if player_alliance { 10 } else { 11 })
    }

    /// The battle result window's track (`FUN_0046f360`): 12 or 14 when the
    /// report's winner (`+0x50`) is the player's side, else 13 or 15, the
    /// Alliance's pair first.
    #[must_use]
    pub const fn battle_result(player_alliance: bool, player_won: bool) -> Self {
        Self(match (player_alliance, player_won) {
            (true, true) => 12,
            (true, false) => 13,
            (false, true) => 14,
            (false, false) => 15,
        })
    }
}

/// The default track pick (`FUN_0041d3b0` -> `FUN_0043a250` on the side's
/// advice agent).
#[derive(Debug, Clone, Default)]
pub struct MusicSelector {
    /// The agent's counter at `+0x190`: 0 when the agent is built
    /// (`FUN_00439320`). port: the save does not carry it
    /// (`FUN_004397a0` writes it), so a loaded game starts at 0.
    counter: u8,
}

impl MusicSelector {
    /// The next default track. Without a game session it is track 0. In a
    /// session, while the counter is above 0 it counts down and returns
    /// `rand(6) / 2 + 1`, a calm track 1..3; at 0 it resets to 3 and returns
    /// the agent's situational track (vtable slot `+0x1c`), so the
    /// situational track plays one pick in four. `roll(n)` draws from
    /// `0..n` (`FUN_0041cd80`).
    ///
    /// port: the situational slot switches on the agent's state byte
    /// `+0x170` (`FUN_004c48e0` Alliance 4..9, `FUN_004c2700` Empire 9..4),
    /// which the port does not track yet (or-q-advice-music-state-rkf2); it
    /// takes the slot's own default branch, `rand(3) + 1`.
    pub fn default_pick(&mut self, in_session: bool, mut roll: impl FnMut(u32) -> u32) -> MdataTrack {
        if !in_session {
            return MdataTrack::MENU;
        }
        if self.counter > 0 {
            self.counter -= 1;
            return calm_track(roll(6) / 2 + 1);
        }
        self.counter = 3;
        calm_track(roll(3) + 1)
    }

    /// A new game's agent starts the counter at 0.
    pub fn reset(&mut self) {
        self.counter = 0;
    }
}

fn calm_track(index: u32) -> MdataTrack {
    MdataTrack(u8::try_from(index.clamp(1, 3)).unwrap_or(1))
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
#[expect(
    clippy::struct_excessive_bools,
    reason = "These independent flags preserve the existing state and serialization model."
)]
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Effective music volume: 0.0 when globally or music-only muted.
    #[must_use]
    pub fn effective_music_volume(&self) -> f64 {
        if self.muted || self.music_muted {
            0.0
        } else {
            f64::from(self.music_volume)
        }
    }

    /// Effective SFX volume: 0.0 when muted, otherwise `sfx_volume`.
    #[must_use]
    pub fn effective_sfx_volume(&self) -> f64 {
        if self.muted {
            0.0
        } else {
            f64::from(self.sfx_volume)
        }
    }

    /// Cutscene audio volume: 0.0 when muted, otherwise full volume.
    #[must_use]
    pub const fn cutscene_volume(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            1.0
        }
    }

    /// Toggle music independently of sound effects.
    pub fn toggle_music(&mut self) {
        self.music_muted = !self.music_muted;
        self.dirty = true;
    }

    /// Whether the dedicated music path is enabled.
    #[must_use]
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
    fn muting_silences_cutscenes_too() {
        let mut state = AudioVolumeState::default();
        assert!((state.cutscene_volume() - 1.0).abs() < f32::EPSILON);
        state.muted = true;
        assert!(state.cutscene_volume().abs() < f32::EPSILON);
    }

    #[test]
    #[expect(
        clippy::float_cmp,
        reason = "These regression checks require exact copied values, endpoints, and pixel coordinates."
    )]
    fn mute_and_gain_produce_exact_effective_levels() {
        let mut state = AudioVolumeState {
            music_volume: 0.35,
            sfx_volume: 0.6,
            ..Default::default()
        };
        assert_eq!(state.effective_music_volume(), f64::from(0.35_f32));
        assert_eq!(state.effective_sfx_volume(), f64::from(0.6_f32));

        state.muted = true;
        assert_eq!(state.effective_music_volume(), 0.0);
        assert_eq!(state.effective_sfx_volume(), 0.0);
    }

    #[test]
    #[expect(
        clippy::float_cmp,
        reason = "These regression checks require exact copied values, endpoints, and pixel coordinates."
    )]
    fn music_toggle_preserves_sound_effects() {
        let mut state = AudioVolumeState::default();
        state.toggle_music();

        assert!(state.music_muted);
        assert!(state.dirty);
        assert_eq!(state.effective_music_volume(), 0.0);
        assert_eq!(state.effective_sfx_volume(), 1.0);

        state.toggle_music();
        assert!(state.music_enabled());
        assert_eq!(state.effective_music_volume(), f64::from(0.8_f32));
    }
}
