//! Audio engine — `quad-snd`-backed music and sound effects.
//!
//! # Why not kira?
//!
//! `kira` uses the `cpal` crate for its audio backend.  `cpal` declares
//! `links = "alsa"` in its Cargo metadata.  `miniquad` (the backend for
//! `egui-macroquad`) also declares `links = "alsa"`.  Cargo forbids two
//! packages in the same dependency closure from declaring the same `links`
//! value — even with `optional = true`, the resolver still pulls both in.
//!
//! `quad-snd` uses the same `miniquad` audio subsystem as macroquad, so
//! there is no link conflict.  It supports native (CoreAudio on macOS,
//! ALSA/PulseAudio on Linux) and WASM (WebAudio) through the same unified
//! API.  Feature parity for this project's needs: looped background music
//! and one-shot SFX — both fully covered.
//!
//! # File layout convention
//!
//! ```text
//! data/sounds/
//!   music/
//!     main_theme.ogg   (or .wav)
//!     battle.ogg
//!   sfx/
//!     mission_success.wav
//!     mission_fail.wav
//!     build_complete.wav
//!     fleet_departure.wav
//!     fleet_arrival.wav
//! ```
//!
//! Missing files are silently skipped — the engine remains silent rather
//! than crashing.
//!
//! # Usage
//!
//! ```rust,ignore
//! let mut audio = AudioEngine::new();
//! audio.load_all(Path::new("data/sounds"));
//! audio.play_music(MusicTrack::MainTheme);
//!
//! // Each frame after egui controls:
//! if vol_state.dirty {
//!     audio.apply_volume(&vol_state);
//!     vol_state.dirty = false;
//! }
//!
//! // On game events:
//! audio.play_sfx(SfxKind::MissionSuccess, &vol_state);
//! ```

use std::collections::HashMap;
use std::path::Path;

use quad_snd::{AudioContext, Sound};

pub use rebellion_render::audio::{AudioVolumeState, MusicTrack, SfxKind};

// ---------------------------------------------------------------------------
// File name conventions
// ---------------------------------------------------------------------------

fn sfx_file(kind: SfxKind) -> &'static str {
    match kind {
        SfxKind::MissionSuccess => "mission_success.wav",
        SfxKind::MissionFail    => "mission_fail.wav",
        SfxKind::BuildComplete  => "build_complete.wav",
        SfxKind::FleetDeparture => "fleet_departure.wav",
        SfxKind::FleetArrival   => "fleet_arrival.wav",
    }
}

fn music_file(track: MusicTrack) -> &'static str {
    match track {
        MusicTrack::MainTheme => "main_theme.ogg",
        MusicTrack::Battle    => "battle.ogg",
        MusicTrack::Victory   => "victory.ogg",
        MusicTrack::Defeat    => "defeat.ogg",
    }
}

// ---------------------------------------------------------------------------
// AudioEngine
// ---------------------------------------------------------------------------

/// `quad-snd`-backed audio engine.
///
/// Handles background music (looped) and one-shot sound effects.
/// Gracefully no-ops if audio files are absent or the audio context fails.
pub struct AudioEngine {
    ctx: AudioContext,

    /// Pre-loaded one-shot SFX.
    sfx: HashMap<SfxKind, Sound>,

    /// Currently loaded music track data + which track it is.
    music: Option<(Sound, MusicTrack)>,

    /// Whether music is currently playing.
    music_playing: bool,
}

impl AudioEngine {
    /// Create a new engine and initialise the `quad-snd` audio context.
    pub fn new() -> Self {
        AudioEngine {
            ctx: AudioContext::new(),
            sfx: HashMap::new(),
            music: None,
            music_playing: false,
        }
    }

    /// Pre-load SFX from `sounds_dir/sfx/`.  Missing files are silently skipped.
    pub fn load_sfx(&mut self, sounds_dir: &Path) {
        let sfx_dir = sounds_dir.join("sfx");
        for &kind in &[
            SfxKind::MissionSuccess,
            SfxKind::MissionFail,
            SfxKind::BuildComplete,
            SfxKind::FleetDeparture,
            SfxKind::FleetArrival,
        ] {
            let path = sfx_dir.join(sfx_file(kind));
            if !path.exists() {
                continue;
            }
            match std::fs::read(&path) {
                Ok(bytes) => {
                    let sound = Sound::load(&self.ctx, &bytes);
                    self.sfx.insert(kind, sound);
                }
                Err(e) => eprintln!("[audio] Failed to read SFX {}: {e}", path.display()),
            }
        }
    }

    /// Load all audio resources from `sounds_dir`.
    ///
    /// SFX are pre-loaded.  Music is loaded on demand in `play_music`.
    pub fn load_all(&mut self, sounds_dir: &Path) {
        self.load_sfx(sounds_dir);
    }

    /// Play a one-shot SFX at the current SFX volume.
    ///
    /// No-op if the SFX was not loaded or the effective volume is zero.
    pub fn play_sfx(&mut self, kind: SfxKind, vol_state: &AudioVolumeState) {
        let vol = vol_state.effective_sfx_volume() as f32;
        if vol <= 0.0 {
            return;
        }
        if let Some(sound) = self.sfx.get(&kind) {
            sound.play(&self.ctx, quad_snd::PlaySoundParams {
                looped: false,
                volume: vol,
            });
        }
    }

    /// Start a looping music track, loading it from `sounds_dir/music/` if
    /// not already loaded.
    ///
    /// If the same track is already playing, this is a no-op.
    pub fn play_music(
        &mut self,
        track: MusicTrack,
        sounds_dir: &Path,
        vol_state: &AudioVolumeState,
    ) {
        let vol = vol_state.effective_music_volume() as f32;

        // If this track is already loaded and playing, do nothing.
        if let Some((_, current)) = &self.music {
            if *current == track && self.music_playing {
                return;
            }
        }

        // Stop the current track if switching.
        self.stop_music();

        let path = sounds_dir.join("music").join(music_file(track));
        if !path.exists() {
            return;
        }

        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("[audio] Failed to read music {}: {e}", path.display());
                return;
            }
        };

        {
            let sound = Sound::load(&self.ctx, &bytes);
            sound.play(&self.ctx, quad_snd::PlaySoundParams {
                looped: true,
                volume: vol,
            });
            self.music = Some((sound, track));
            self.music_playing = true;
        }
    }

    /// Stop the current music track immediately.
    pub fn stop_music(&mut self) {
        if let Some((sound, _)) = self.music.take() {
            sound.stop(&self.ctx);
            self.music_playing = false;
        }
    }

    /// Apply volume changes to the currently playing music.
    ///
    /// Call when `AudioVolumeState::dirty` is true, then clear `dirty`.
    pub fn apply_volume(&mut self, vol_state: &AudioVolumeState) {
        let vol = vol_state.effective_music_volume() as f32;
        if let Some((sound, _)) = &self.music {
            sound.set_volume(&self.ctx, vol);
        }
    }

    /// Always returns `true` — `quad-snd` initialises unconditionally.
    pub fn is_available(&self) -> bool {
        true
    }
}
