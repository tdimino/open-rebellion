//! Audio engine — `quad-snd`-backed music, voice lines, and sound effects.
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
//! API.  Feature parity for this project's needs: looped background music,
//! one-shot SFX, and one-shot voice lines — all fully covered.
//!
//! # File layout convention
//!
//! ```text
//! data/sounds/
//!   music/
//!     main_theme.wav      (MDATA.302 — Main Title + Tatooine)
//!     battle.wav          (MDATA.307 — Attacking a Star Destroyer)
//!     victory.wav         (MDATA.201 extraction)
//!     defeat.wav          (MDATA.202 extraction)
//!     imperial.wav        (MDATA.306 — Emperor Arrives / Death of Yoda)
//!     hoth.wav            (MDATA.312 — Battle of Hoth medley)
//!     endor.wav           (MDATA.300 — Battle of Endor medley)
//!   sfx/
//!     mission_success.wav
//!     mission_fail.wav
//!     build_complete.wav
//!     fleet_departure.wav
//!     fleet_arrival.wav
//!     combat_start.wav
//!     ui_click.wav
//!     ui_close.wav
//!   voice/
//!     alliance/
//!       14001-voicefxa.wav   (through 15163-voicefxa.wav — from VOICEFXA.DLL)
//!     empire/
//!       15001-voicefxe.wav   (through 15132-voicefxe.wav — from VOICEFXE.DLL)
//! ```
//!
//! All files are optional: missing files are silently skipped and the engine
//! remains silent rather than crashing.
//!
//! # Voice line resource IDs
//!
//! The original VOICEFXA.DLL contains resources 14001–15163 (Alliance).
//! VOICEFXE.DLL contains resources 15001–15132 (Empire).  Extracted files
//! are in `assets/references/ref-ui-full/voice-alliance/` and
//! `voice-empire/` under names like `14001-voicefxa.wav`.  The staged
//! copies for runtime use live under `data/sounds/voice/`.
//!
//! # Context-based music
//!
//! Call `play_music_for_context` instead of `play_music` when you only know
//! the game context (galaxy map, combat, etc.).  The engine maps contexts to
//! tracks:
//!
//! | Context | Track |
//! |---------|-------|
//! | MainMenu | MainTheme |
//! | GalaxyMap | MainTheme |
//! | Combat | Battle |
//! | Victory | Victory |
//! | Defeat | Defeat |
//!
//! # WASM
//!
//! `quad-snd` provides a WebAudio backend for WASM builds with the same API.
//! File loading differs: on WASM you must supply raw bytes (loaded via
//! `macroquad::file::load_file`).  The `load_*_bytes` methods accept raw
//! bytes for this purpose.  The standard `load_*` methods are native-only
//! (`#[cfg(not(target_arch = "wasm32"))]`).
//!
//! # Usage
//!
//! ```rust,ignore
//! let mut audio = AudioEngine::new();
//! audio.load_all(Path::new("data/sounds"));
//!
//! // Context-based: engine picks the right track.
//! audio.play_music_for_context(MusicContext::GalaxyMap, &vol_state);
//!
//! // On game events:
//! audio.play_sfx(SfxKind::BuildComplete, &vol_state);
//! audio.play_voice(VoiceLine::AllianceBuildComplete, &vol_state);
//!
//! // Each frame after egui controls:
//! if vol_state.dirty {
//!     audio.apply_volume(&vol_state);
//!     vol_state.dirty = false;
//! }
//! ```

use std::collections::HashMap;
use std::path::Path;

use quad_snd::{AudioContext, Sound};

pub use rebellion_render::audio::{AudioVolumeState, MusicContext, MusicTrack, SfxKind, VoiceLine};

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
        SfxKind::CombatStart    => "combat_start.wav",
        SfxKind::UiClick        => "ui_click.wav",
        SfxKind::UiClose        => "ui_close.wav",
    }
}

fn music_file(track: MusicTrack) -> &'static str {
    match track {
        MusicTrack::MainTheme => "main_theme.wav",
        MusicTrack::Battle    => "battle.wav",
        MusicTrack::Victory   => "victory.wav",
        MusicTrack::Defeat    => "defeat.wav",
        MusicTrack::Imperial  => "imperial.wav",
        MusicTrack::Hoth      => "hoth.wav",
        MusicTrack::Endor     => "endor.wav",
    }
}

/// Select a `MusicTrack` for a given `MusicContext`.
pub fn track_for_context(ctx: MusicContext) -> MusicTrack {
    match ctx {
        MusicContext::MainMenu  => MusicTrack::MainTheme,
        MusicContext::GalaxyMap => MusicTrack::MainTheme,
        MusicContext::Combat    => MusicTrack::Battle,
        MusicContext::Victory   => MusicTrack::Victory,
        MusicContext::Defeat    => MusicTrack::Defeat,
    }
}

// ---------------------------------------------------------------------------
// Voice line resource IDs
// ---------------------------------------------------------------------------

/// Return `(faction_dir, resource_id)` for a `VoiceLine`.
///
/// Resource IDs match the VOICEFXA/VOICEFXE extraction output filenames.
/// File names follow the pattern `{id}-voicefxa.wav` / `{id}-voicefxe.wav`.
fn voice_path(line: VoiceLine) -> (&'static str, u32) {
    match line {
        // Alliance voice lines (VOICEFXA.DLL, resource IDs 14001–15163)
        VoiceLine::AllianceMissionSuccess => ("alliance", 14001),
        VoiceLine::AllianceMissionFail    => ("alliance", 14002),
        VoiceLine::AllianceFleetDeparts   => ("alliance", 14003),
        VoiceLine::AllianceBuildComplete  => ("alliance", 14004),
        // Empire voice lines (VOICEFXE.DLL, resource IDs 15001–15132)
        VoiceLine::EmpireMissionSuccess   => ("empire",   15001),
        VoiceLine::EmpireMissionFail      => ("empire",   15002),
        VoiceLine::EmpireFleetDeparts     => ("empire",   15003),
        VoiceLine::EmpireBuildComplete    => ("empire",   15004),
    }
}

/// Build the filename for a voice line as found in the extracted DLL output.
///
/// Alliance: `{id}-voicefxa.wav`, Empire: `{id}-voicefxe.wav`.
fn voice_filename(faction: &str, id: u32) -> String {
    let suffix = if faction == "empire" { "voicefxe" } else { "voicefxa" };
    format!("{}-{}.wav", id, suffix)
}

// ---------------------------------------------------------------------------
// AudioEngine
// ---------------------------------------------------------------------------

/// `quad-snd`-backed audio engine.
///
/// Handles background music (looped), one-shot SFX, and one-shot voice lines.
/// Gracefully no-ops when audio files are absent or the context fails.
pub struct AudioEngine {
    ctx: AudioContext,

    /// Pre-loaded one-shot SFX.
    sfx: HashMap<SfxKind, Sound>,

    /// Pre-loaded voice lines.
    voice: HashMap<VoiceLine, Sound>,

    /// Currently loaded music track + which track it is.
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
            voice: HashMap::new(),
            music: None,
            music_playing: false,
        }
    }

    // -----------------------------------------------------------------------
    // Loading
    // -----------------------------------------------------------------------

    /// Pre-load all SFX from `sounds_dir/sfx/`.  Missing files are silently skipped.
    pub fn load_sfx(&mut self, sounds_dir: &Path) {
        let sfx_dir = sounds_dir.join("sfx");
        let kinds = [
            SfxKind::MissionSuccess,
            SfxKind::MissionFail,
            SfxKind::BuildComplete,
            SfxKind::FleetDeparture,
            SfxKind::FleetArrival,
            SfxKind::CombatStart,
            SfxKind::UiClick,
            SfxKind::UiClose,
        ];
        for kind in kinds {
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

    /// Pre-load all voice lines from `sounds_dir/voice/`.  Missing files are silently skipped.
    ///
    /// Voice files live at:
    /// - `sounds_dir/voice/alliance/{id}-voicefxa.wav`
    /// - `sounds_dir/voice/empire/{id}-voicefxe.wav`
    pub fn load_voice(&mut self, sounds_dir: &Path) {
        let voice_dir = sounds_dir.join("voice");
        let lines = [
            VoiceLine::AllianceMissionSuccess,
            VoiceLine::AllianceMissionFail,
            VoiceLine::AllianceFleetDeparts,
            VoiceLine::AllianceBuildComplete,
            VoiceLine::EmpireMissionSuccess,
            VoiceLine::EmpireMissionFail,
            VoiceLine::EmpireFleetDeparts,
            VoiceLine::EmpireBuildComplete,
        ];
        for line in lines {
            let (faction, id) = voice_path(line);
            let filename = voice_filename(faction, id);
            let path = voice_dir.join(faction).join(&filename);
            if !path.exists() {
                continue;
            }
            match std::fs::read(&path) {
                Ok(bytes) => {
                    let sound = Sound::load(&self.ctx, &bytes);
                    self.voice.insert(line, sound);
                }
                Err(e) => eprintln!("[audio] Failed to read voice line {}: {e}", path.display()),
            }
        }
    }

    /// Load all audio resources from `sounds_dir`.
    ///
    /// SFX and voice lines are pre-loaded.  Music is loaded on demand in
    /// `play_music`.
    pub fn load_all(&mut self, sounds_dir: &Path) {
        self.load_sfx(sounds_dir);
        self.load_voice(sounds_dir);
    }

    // -----------------------------------------------------------------------
    // WASM byte-level loaders (for macroquad::file::load_file results)
    // -----------------------------------------------------------------------

    /// Load a single SFX from raw bytes (e.g. from WASM HTTP fetch).
    #[allow(dead_code)]
    pub fn load_sfx_bytes(&mut self, kind: SfxKind, bytes: &[u8]) {
        let sound = Sound::load(&self.ctx, bytes);
        self.sfx.insert(kind, sound);
    }

    /// Load a single voice line from raw bytes.
    #[allow(dead_code)]
    pub fn load_voice_bytes(&mut self, line: VoiceLine, bytes: &[u8]) {
        let sound = Sound::load(&self.ctx, bytes);
        self.voice.insert(line, sound);
    }

    /// Load a music track from raw bytes and immediately start playing it.
    ///
    /// Used on WASM where files are fetched asynchronously.
    #[allow(dead_code)]
    pub fn load_and_play_music_bytes(
        &mut self,
        track: MusicTrack,
        bytes: &[u8],
        vol_state: &AudioVolumeState,
    ) {
        self.stop_music();
        let vol = vol_state.effective_music_volume() as f32;
        let sound = Sound::load(&self.ctx, bytes);
        sound.play(&self.ctx, quad_snd::PlaySoundParams {
            looped: true,
            volume: vol,
        });
        self.music = Some((sound, track));
        self.music_playing = true;
    }

    // -----------------------------------------------------------------------
    // Playback
    // -----------------------------------------------------------------------

    /// Play a one-shot SFX at the current SFX volume.
    ///
    /// No-op when the SFX was not loaded or the effective volume is zero.
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

    /// Play a one-shot voice line at the current SFX volume.
    ///
    /// No-op when the line was not loaded or the effective volume is zero.
    pub fn play_voice(&mut self, line: VoiceLine, vol_state: &AudioVolumeState) {
        let vol = vol_state.effective_sfx_volume() as f32;
        if vol <= 0.0 {
            return;
        }
        if let Some(sound) = self.voice.get(&line) {
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

        // Same track already playing — do nothing.
        if let Some((_, current)) = &self.music {
            if *current == track && self.music_playing {
                return;
            }
        }

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

        let sound = Sound::load(&self.ctx, &bytes);
        sound.play(&self.ctx, quad_snd::PlaySoundParams {
            looped: true,
            volume: vol,
        });
        self.music = Some((sound, track));
        self.music_playing = true;
    }

    /// Select and start the appropriate music track for a game context.
    ///
    /// Delegates to `play_music` after mapping context → track.  If the
    /// correct track is already playing, this is a no-op.
    pub fn play_music_for_context(
        &mut self,
        context: MusicContext,
        sounds_dir: &Path,
        vol_state: &AudioVolumeState,
    ) {
        let track = track_for_context(context);
        self.play_music(track, sounds_dir, vol_state);
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

    /// Returns the currently playing `MusicTrack`, if any.
    #[allow(dead_code)]
    pub fn current_track(&self) -> Option<MusicTrack> {
        self.music.as_ref().map(|(_, t)| *t)
    }

    /// Always returns `true` — `quad-snd` initialises unconditionally.
    pub fn is_available(&self) -> bool {
        true
    }
}
