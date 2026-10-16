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
//! there is no link conflict.  It supports native (`CoreAudio` on macOS,
//! ALSA/PulseAudio on Linux) and WASM (`WebAudio`) through the same unified
//! API.  Feature parity for this project's needs: looped background music,
//! one-shot SFX, and one-shot tactical voice lines — all fully covered.
//!
//! # File layout convention
//!
//! ```text
//! data/sounds/
//!   sfx/
//!     mission_success.wav
//!     mission_fail.wav
//!     build_complete.wav
//!     fleet_departure.wav
//!     fleet_arrival.wav
//!     combat_start.wav
//!     ui_click.wav
//!     ui_close.wav
//!     menu_galaxy_size.wav (COMMON.DLL WAVE 8000)
//!     menu_load_options.wav (COMMON.DLL WAVE 8001)
//!     menu_quit.wav        (COMMON.DLL WAVE 8002)
//!     menu_select.wav      (COMMON.DLL WAVE 8004)
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
//! # Tactical voice resource IDs
//!
//! The original VOICEFXA.DLL contains resources 14001–15163 (Alliance).
//! VOICEFXE.DLL contains resources 15001–15132 (Empire).  Extracted files
//! are in `assets/references/ref-ui-full/voice-alliance/` and
//! `voice-empire/` under names like `14001-voicefxa.wav`.  The staged
//! copies for runtime use live under `data/sounds/voice/`.
//!
//! # Music
//!
//! Music streams from the original's loose files `MDATA\\MDATA.300`..`.315`
//! (RIFF WAVE), a track index 0..15 (`MdataTrack`). A track plays once; when
//! it ends a forced track (Battle Alert, battle result) plays again and any
//! other asks the caller for the next default pick (`FUN_00417780`,
//! `ghidra/notes/audio-dispatch.md`, "Music").
//!
//! # WASM
//!
//! `quad-snd` provides a `WebAudio` backend for WASM builds with the same API.
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
//! audio.start_track(MdataTrack::MENU, &mdata_dir, &vol_state, get_time());
//!
//! // On game events:
//! audio.play_sfx(SfxKind::BuildComplete, &vol_state);
//!
//! // Each frame after egui controls:
//! if vol_state.dirty {
//!     audio.apply_volume(&vol_state);
//!     vol_state.dirty = false;
//! }
//! ```

use std::collections::HashMap;
#[cfg(any(test, not(target_arch = "wasm32")))]
use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

use quad_snd::{AudioContext, Sound};

pub use rebellion_render::audio::{AudioVolumeState, MdataTrack, SfxKind};
use rebellion_render::tactical_view::TacticalVoiceFaction;

#[cfg(not(target_arch = "wasm32"))]
const AUDIO_PREFIX: &str = "data";

#[cfg(not(target_arch = "wasm32"))]
fn audio_base_path(path: &Path) -> PathBuf {
    let prefix = Path::new(AUDIO_PREFIX);

    if path.starts_with(prefix) {
        return path.to_path_buf();
    }

    if let Ok(relative) = path.strip_prefix("data") {
        return prefix.join(relative);
    }

    prefix.join(path)
}

// ---------------------------------------------------------------------------
// File name conventions
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
fn sfx_file(kind: SfxKind) -> &'static str {
    match kind {
        SfxKind::MissionSuccess => "mission_success.wav",
        SfxKind::MissionFail => "mission_fail.wav",
        SfxKind::BuildComplete => "build_complete.wav",
        SfxKind::FleetDeparture => "fleet_departure.wav",
        SfxKind::FleetArrival => "fleet_arrival.wav",
        SfxKind::CombatStart => "combat_start.wav",
        SfxKind::UiClick => "ui_click.wav",
        SfxKind::UiClose => "ui_close.wav",
        SfxKind::MenuGalaxySize => "menu_galaxy_size.wav",
        SfxKind::MenuLoadOptions => "menu_load_options.wav",
        SfxKind::MenuQuit => "menu_quit.wav",
        SfxKind::MenuSelect => "menu_select.wav",
    }
}

/// Browser-pack filenames and COMMON.DLL resource identities for cockpit SFX.
pub const MENU_SFX_ASSETS: &[(SfxKind, &str, u32)] = &[
    (SfxKind::MenuGalaxySize, "sfx/menu_galaxy_size.wav", 8000),
    (SfxKind::MenuLoadOptions, "sfx/menu_load_options.wav", 8001),
    (SfxKind::MenuQuit, "sfx/menu_quit.wav", 8002),
    (SfxKind::MenuSelect, "sfx/menu_select.wav", 8004),
];

/// Exact weapon-event variant table built by `FUN_005bae60`. Each row is
/// `(event_id, variant, runtime_path, TACTICAL.DLL WAVE resource)`.
pub const TACTICAL_SFX_ASSETS: &[(u8, u8, &str, u32)] = &[
    (0x0d, 0, "sfx/tactical_event_0d_0.wav", 13_033),
    (0x0d, 1, "sfx/tactical_event_0d_1.wav", 13_034),
    (0x0d, 2, "sfx/tactical_event_0d_2.wav", 13_035),
    (0x0e, 0, "sfx/tactical_event_0e_0.wav", 13_036),
    (0x0e, 1, "sfx/tactical_event_0e_1.wav", 13_037),
    (0x0e, 2, "sfx/tactical_event_0e_2.wav", 13_038),
    (0x0f, 0, "sfx/tactical_event_0f_0.wav", 13_039),
    (0x0f, 1, "sfx/tactical_event_0f_1.wav", 13_040),
    (0x0f, 2, "sfx/tactical_event_0f_2.wav", 13_041),
    (0x10, 0, "sfx/tactical_event_10_0.wav", 13_042),
    (0x10, 1, "sfx/tactical_event_10_1.wav", 13_043),
    (0x10, 2, "sfx/tactical_event_10_2.wav", 13_044),
    (0x11, 0, "sfx/tactical_event_11_0.wav", 13_045),
    (0x11, 1, "sfx/tactical_event_11_1.wav", 13_046),
    (0x11, 2, "sfx/tactical_event_11_2.wav", 13_047),
    (0x12, 0, "sfx/tactical_event_12_0.wav", 13_048),
    (0x12, 1, "sfx/tactical_event_12_1.wav", 13_049),
    (0x12, 2, "sfx/tactical_event_12_2.wav", 13_050),
    (0x13, 0, "sfx/tactical_event_13_0.wav", 13_051),
    (0x13, 1, "sfx/tactical_event_13_1.wav", 13_052),
    (0x13, 2, "sfx/tactical_event_13_2.wav", 13_053),
    (0x14, 0, "sfx/tactical_event_14_0.wav", 13_054),
];

/// Build the filename for a voice line as found in the extracted DLL output.
///
/// Alliance: `{id}-voicefxa.wav`, Empire: `{id}-voicefxe.wav`.
fn voice_filename(faction: TacticalVoiceFaction, id: u32) -> String {
    let suffix = match faction {
        TacticalVoiceFaction::Alliance => "voicefxa",
        TacticalVoiceFaction::Empire => "voicefxe",
    };
    format!("{id}-{suffix}.wav")
}

fn voice_faction_dir(faction: TacticalVoiceFaction) -> &'static str {
    match faction {
        TacticalVoiceFaction::Alliance => "alliance",
        TacticalVoiceFaction::Empire => "empire",
    }
}

/// Complete source-proven tactical voice banks.
///
/// `VOICEFXA.DLL` stores the Alliance command/result bank at 14001-14122 and
/// the Alliance trench-run bank at 15133-15163. `VOICEFXE.DLL` stores the
/// Imperial command/result/Death Star bank at 15001-15132.
pub const TACTICAL_VOICE_RANGES: &[(TacticalVoiceFaction, u32, u32)] = &[
    (TacticalVoiceFaction::Alliance, 14_001, 14_122),
    (TacticalVoiceFaction::Alliance, 15_133, 15_163),
    (TacticalVoiceFaction::Empire, 15_001, 15_132),
];

/// Runtime-pack path and resource identity for every restored tactical voice.
pub fn tactical_voice_assets() -> Vec<(TacticalVoiceFaction, String, u32)> {
    TACTICAL_VOICE_RANGES
        .iter()
        .flat_map(|&(faction, first, last)| {
            (first..=last).map(move |resource_id| {
                (
                    faction,
                    format!(
                        "voice/{}/{}",
                        voice_faction_dir(faction),
                        voice_filename(faction, resource_id)
                    ),
                    resource_id,
                )
            })
        })
        .collect()
}

/// The STRATEGY WAVEs messages play when shown (`message-display.md`):
/// loyalty joins and neutrality (`0x450`, `0x451`), uprising begins and
/// ends (`0x454`, `0x455`), maintenance (`0x456`), fleet arrival (`0x45a`),
/// blockade (`0x45e`) and Advice (`0x461` Alliance, `0x462` Empire).
pub const MESSAGE_SOUNDS: [u32; 9] = [
    0x450, 0x451, 0x454, 0x455, 0x456, 0x45a, 0x45e, 0x461, 0x462,
];

/// A RIFF WAVE's play time: its `data` chunk over the `fmt ` byte rate.
#[cfg(not(target_arch = "wasm32"))]
fn riff_seconds(bytes: &[u8]) -> Option<f64> {
    if bytes.get(0..4)? != b"RIFF" || bytes.get(8..12)? != b"WAVE" {
        return None;
    }
    let word = |at: usize| -> Option<u32> {
        Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
    };
    let (mut at, mut byte_rate, mut data) = (12usize, None, None);
    while at + 8 <= bytes.len() {
        let size = usize::try_from(word(at + 4)?).ok()?;
        match bytes.get(at..at + 4)? {
            b"fmt " => byte_rate = word(at + 16),
            b"data" => data = Some(size.min(bytes.len() - at - 8)),
            _ => {}
        }
        at += 8 + size + (size & 1);
    }
    let byte_rate = f64::from(byte_rate.filter(|rate| *rate > 0)?);
    Some(f64::from(u32::try_from(data?).ok()?) / byte_rate)
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

    /// Original tactical weapon-event variants keyed by WAVE resource ID.
    tactical_sfx: HashMap<u32, Sound>,

    /// Source-mapped tactical voices keyed by faction and WAVE ID.
    tactical_voice: HashMap<(TacticalVoiceFaction, u32), Sound>,

    /// The cockpit droids' sounds, keyed by sprite DLL and WAVE ID.
    advisor_voice: HashMap<String, Sound>,

    /// The Message Index's message sounds, STRATEGY WAVEs by ID.
    message_sounds: HashMap<u32, Sound>,

    /// The message sound last started, which the next one stops.
    message_playing: Option<u32>,

    /// Currently loaded music track + which track it is.
    music: Option<(Sound, MdataTrack)>,

    /// Whether music is currently playing.
    music_playing: bool,

    /// When the playing track ends, in `get_time` seconds.
    #[cfg(not(target_arch = "wasm32"))]
    music_ends_at: Option<f64>,

    /// The forced flag (`DAT_006b1490` bit 2): set by `force_track`
    /// (`FUN_00417610`), cleared by a stop (`FUN_00417640`).
    #[cfg(not(target_arch = "wasm32"))]
    music_forced: bool,

    /// Missing tracks already reported, keeping diagnostics bounded.
    #[cfg(not(target_arch = "wasm32"))]
    missing_music_logged: HashSet<MdataTrack>,
}

impl AudioEngine {
    /// Create a new engine and initialise the `quad-snd` audio context.
    pub fn new() -> Self {
        AudioEngine {
            ctx: AudioContext::new(),
            sfx: HashMap::new(),
            tactical_sfx: HashMap::new(),
            tactical_voice: HashMap::new(),
            advisor_voice: HashMap::new(),
            message_sounds: HashMap::new(),
            message_playing: None,
            music: None,
            music_playing: false,
            #[cfg(not(target_arch = "wasm32"))]
            music_ends_at: None,
            #[cfg(not(target_arch = "wasm32"))]
            music_forced: false,
            #[cfg(not(target_arch = "wasm32"))]
            missing_music_logged: HashSet::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Loading
    // -----------------------------------------------------------------------

    /// Pre-load all SFX from `sounds_dir/sfx/`.  Missing files are silently skipped.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_sfx(&mut self, sounds_dir: &Path) {
        let sfx_dir = audio_base_path(sounds_dir).join("sfx");
        let kinds = [
            SfxKind::MissionSuccess,
            SfxKind::MissionFail,
            SfxKind::BuildComplete,
            SfxKind::FleetDeparture,
            SfxKind::FleetArrival,
            SfxKind::CombatStart,
            SfxKind::UiClick,
            SfxKind::UiClose,
            SfxKind::MenuGalaxySize,
            SfxKind::MenuLoadOptions,
            SfxKind::MenuQuit,
            SfxKind::MenuSelect,
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
        let audio_root = audio_base_path(sounds_dir);
        for &(_, _, relative_path, resource_id) in TACTICAL_SFX_ASSETS {
            let path = audio_root.join(relative_path);
            if !path.exists() {
                continue;
            }
            match std::fs::read(&path) {
                Ok(bytes) => self.load_tactical_sfx_bytes(resource_id, &bytes),
                Err(error) => {
                    eprintln!(
                        "[audio] Failed to read tactical SFX {}: {error}",
                        path.display()
                    );
                }
            }
        }
    }

    /// Pre-load the complete source-proven tactical voice bank. Missing files
    /// remain silent so an installation without the original DLLs still runs.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_tactical_voice(&mut self, sounds_dir: &Path) {
        let audio_root = audio_base_path(sounds_dir);
        for (faction, relative_path, resource_id) in tactical_voice_assets() {
            let path = audio_root.join(relative_path);
            if !path.exists() {
                continue;
            }
            match std::fs::read(&path) {
                Ok(bytes) => self.load_tactical_voice_bytes(faction, resource_id, &bytes),
                Err(error) => eprintln!(
                    "[audio] Failed to read tactical voice {}: {error}",
                    path.display()
                ),
            }
        }
    }

    /// Load all audio resources from `sounds_dir`.
    ///
    /// SFX and voice lines are pre-loaded.  Music is read when it starts
    /// (`start_track`).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_all(&mut self, sounds_dir: &Path) {
        self.load_sfx(sounds_dir);
        self.load_tactical_voice(sounds_dir);
    }

    /// Load the binary-mapped cockpit sounds directly from an owned COMMON.DLL.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_original_menu_sfx(&mut self, common_dll: &Path) {
        let resource_ids: Vec<u32> = MENU_SFX_ASSETS
            .iter()
            .map(|(_, _, resource_id)| *resource_id)
            .collect();
        match rebellion_data::load_wave_resources(common_dll, &resource_ids) {
            Ok(waves) => {
                for &(kind, _, resource_id) in MENU_SFX_ASSETS {
                    if let Some(bytes) = waves.get(&resource_id) {
                        self.load_sfx_bytes(kind, bytes);
                    }
                }
                eprintln!(
                    "[audio] loaded {} original cockpit SFX from {}",
                    waves.len(),
                    common_dll.display()
                );
            }
            Err(error) => eprintln!(
                "[audio] original cockpit SFX unavailable path={} error={error}",
                common_dll.display()
            ),
        }
    }

    /// Load recovered tactical event cues from an owned TACTICAL.DLL.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_original_tactical_sfx(&mut self, tactical_dll: &Path) {
        let resource_ids: Vec<u32> = TACTICAL_SFX_ASSETS
            .iter()
            .map(|(_, _, _, resource_id)| *resource_id)
            .collect();
        match rebellion_data::load_wave_resources(tactical_dll, &resource_ids) {
            Ok(waves) => {
                for &(_, _, _, resource_id) in TACTICAL_SFX_ASSETS {
                    if let Some(bytes) = waves.get(&resource_id) {
                        self.load_tactical_sfx_bytes(resource_id, bytes);
                    }
                }
                eprintln!(
                    "[audio] loaded {} original tactical SFX from {}",
                    waves.len(),
                    tactical_dll.display()
                );
            }
            Err(error) => eprintln!(
                "[audio] original tactical SFX unavailable path={} error={error}",
                tactical_dll.display()
            ),
        }
    }

    /// Load the message sounds from an owned STRATEGY.DLL.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_original_message_sounds(&mut self, strategy_dll: &Path) {
        match rebellion_data::load_wave_resources(strategy_dll, &MESSAGE_SOUNDS) {
            Ok(waves) => {
                for (&resource_id, bytes) in &waves {
                    self.load_message_sound_bytes(resource_id, bytes);
                }
                eprintln!(
                    "[audio] loaded {} message sounds from {}",
                    waves.len(),
                    strategy_dll.display()
                );
            }
            Err(error) => eprintln!(
                "[audio] message sounds unavailable path={} error={error}",
                strategy_dll.display()
            ),
        }
    }

    /// Load the complete source-proven tactical voice banks from owned DLLs.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_original_tactical_voice(&mut self, alliance_dll: &Path, empire_dll: &Path) {
        for (faction, dll_path) in [
            (TacticalVoiceFaction::Alliance, alliance_dll),
            (TacticalVoiceFaction::Empire, empire_dll),
        ] {
            if !dll_path.exists() {
                continue;
            }
            let resource_ids = tactical_voice_assets()
                .into_iter()
                .filter_map(|(candidate, _, resource_id)| {
                    (candidate == faction).then_some(resource_id)
                })
                .collect::<Vec<_>>();
            match rebellion_data::load_wave_resources(dll_path, &resource_ids) {
                Ok(waves) => {
                    for (resource_id, bytes) in waves {
                        self.load_tactical_voice_bytes(faction, resource_id, &bytes);
                    }
                    eprintln!(
                        "[audio] loaded {} original tactical voices from {}",
                        resource_ids.len(),
                        dll_path.display()
                    );
                }
                Err(error) => eprintln!(
                    "[audio] original tactical voice unavailable path={} error={error}",
                    dll_path.display()
                ),
            }
        }
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

    /// Load one source-mapped TACTICAL.DLL WAVE resource.
    pub fn load_tactical_sfx_bytes(&mut self, resource_id: u32, bytes: &[u8]) {
        let sound = Sound::load(&self.ctx, bytes);
        self.tactical_sfx.insert(resource_id, sound);
    }

    /// Load one source-mapped tactical voice from raw bytes.
    pub fn load_tactical_voice_bytes(
        &mut self,
        faction: TacticalVoiceFaction,
        resource_id: u32,
        bytes: &[u8],
    ) {
        let sound = Sound::load(&self.ctx, bytes);
        self.tactical_voice.insert((faction, resource_id), sound);
    }

    /// Load one message sound, a STRATEGY WAVE, from raw bytes.
    pub fn load_message_sound_bytes(&mut self, resource_id: u32, bytes: &[u8]) {
        let sound = Sound::load(&self.ctx, bytes);
        self.message_sounds.insert(resource_id, sound);
    }

    /// Whether the droid sound `key` is loaded.
    pub fn has_advisor_voice(&self, key: &str) -> bool {
        self.advisor_voice.contains_key(key)
    }

    /// Load a droid sound from its sprite DLL's WAVE bytes.
    pub fn load_advisor_voice_bytes(&mut self, key: &str, bytes: &[u8]) {
        let sound = Sound::load(&self.ctx, bytes);
        self.advisor_voice.insert(key.to_owned(), sound);
    }

    /// Begin decoding a music track supplied by the browser runtime pack.
    ///
    /// WebAudio decoding is asynchronous. Call `try_play_loaded_music` on
    /// subsequent frames after a user gesture rather than attempting playback
    /// before the decoded buffer exists.
    #[cfg(target_arch = "wasm32")]
    pub fn load_music_bytes(&mut self, track: MdataTrack, bytes: &[u8]) {
        self.stop_music();
        let sound = Sound::load(&self.ctx, bytes);
        self.music = Some((sound, track));
        self.music_playing = false;
    }

    /// Start decoded browser music once, returning whether playback is active.
    #[cfg(target_arch = "wasm32")]
    pub fn try_play_loaded_music(&mut self, vol_state: &AudioVolumeState) -> bool {
        if self.music_playing {
            return true;
        }

        let Some((sound, _)) = &self.music else {
            return false;
        };
        if !sound.is_loaded() {
            return false;
        }

        sound.play(
            &self.ctx,
            quad_snd::PlaySoundParams {
                looped: true,
                volume: vol_state.effective_music_volume() as f32,
            },
        );
        self.music_playing = true;
        true
    }

    // -----------------------------------------------------------------------
    // Playback
    // -----------------------------------------------------------------------

    /// Play a one-shot SFX at the current SFX volume.
    ///
    /// No-op when the SFX was not loaded or the effective volume is zero.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Audio gains are bounded values; the playback API takes f32."
    )]
    pub fn play_sfx(&mut self, kind: SfxKind, vol_state: &AudioVolumeState) {
        let vol = vol_state.effective_sfx_volume() as f32;
        if vol <= 0.0 {
            return;
        }
        if let Some(sound) = self.sfx.get(&kind) {
            sound.play(
                &self.ctx,
                quad_snd::PlaySoundParams {
                    looped: false,
                    volume: vol,
                },
            );
        }
    }

    /// Play one concrete original tactical event variant.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Audio gains are bounded values; the playback API takes f32."
    )]
    pub fn play_tactical_sfx(&mut self, resource_id: u32, vol_state: &AudioVolumeState) -> bool {
        let vol = vol_state.effective_sfx_volume() as f32;
        if let Some(sound) = self.tactical_sfx.get(&resource_id) {
            if vol > 0.0 {
                sound.play(
                    &self.ctx,
                    quad_snd::PlaySoundParams {
                        looped: false,
                        volume: vol,
                    },
                );
            }
            true
        } else {
            false
        }
    }

    /// Play the sound of the message the Message Index now shows, stopping
    /// the last one first (`FUN_0046a6e0`, then `FUN_00610c30`). False when
    /// the sound is not loaded.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Audio gains are bounded values; the playback API takes f32."
    )]
    pub fn play_message_sound(&mut self, resource_id: u32, vol_state: &AudioVolumeState) -> bool {
        if let Some(previous) = self.message_playing.take() {
            if let Some(sound) = self.message_sounds.get(&previous) {
                sound.stop(&self.ctx);
            }
        }
        let vol = vol_state.effective_sfx_volume() as f32;
        let Some(sound) = self.message_sounds.get(&resource_id) else {
            return false;
        };
        if vol > 0.0 {
            sound.play(
                &self.ctx,
                quad_snd::PlaySoundParams {
                    looped: false,
                    volume: vol,
                },
            );
            self.message_playing = Some(resource_id);
        }
        true
    }

    /// Play a loaded droid sound at the current SFX volume.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Audio gains are bounded values; the playback API takes f32."
    )]
    pub fn play_advisor_voice(&mut self, key: &str, vol_state: &AudioVolumeState) -> bool {
        let vol = vol_state.effective_sfx_volume() as f32;
        if let Some(sound) = self.advisor_voice.get(key) {
            if vol > 0.0 {
                sound.play(
                    &self.ctx,
                    quad_snd::PlaySoundParams {
                        looped: false,
                        volume: vol,
                    },
                );
            }
            true
        } else {
            false
        }
    }

    /// Play one exact tactical voice at the current SFX volume.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Audio gains are bounded values; the playback API takes f32."
    )]
    pub fn play_tactical_voice(
        &mut self,
        faction: TacticalVoiceFaction,
        resource_id: u32,
        vol_state: &AudioVolumeState,
    ) -> bool {
        let vol = vol_state.effective_sfx_volume() as f32;
        if let Some(sound) = self.tactical_voice.get(&(faction, resource_id)) {
            if vol > 0.0 {
                sound.play(
                    &self.ctx,
                    quad_snd::PlaySoundParams {
                        looped: false,
                        volume: vol,
                    },
                );
            }
            true
        } else {
            false
        }
    }

    /// Start a looping music track, loading it from `sounds_dir/music/` if
    /// not already loaded.
    ///
    /// If the same track is already playing, this is a no-op.
    /// Play `track` once from `mdata_dir` (`FUN_00417520`): a playing track
    /// stops first, which clears the forced flag.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Audio gains are bounded values; the playback API takes f32."
    )]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn start_track(
        &mut self,
        track: MdataTrack,
        mdata_dir: &Path,
        vol_state: &AudioVolumeState,
        now: f64,
    ) {
        if self.music_playing {
            self.stop_music();
        }
        self.music = None;
        self.music_ends_at = None;
        let path = mdata_dir.join(track.file_name());
        let bytes = match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) => {
                if self.missing_music_logged.insert(track) {
                    eprintln!(
                        "[audio] music unavailable track={} path={}: {error}",
                        track.index(),
                        path.display()
                    );
                }
                return;
            }
        };
        let Some(seconds) = riff_seconds(&bytes) else {
            eprintln!(
                "[audio] music track={} is not a RIFF WAVE: {}",
                track.index(),
                path.display()
            );
            return;
        };
        let sound = Sound::load(&self.ctx, &bytes);
        sound.play(
            &self.ctx,
            quad_snd::PlaySoundParams {
                looped: false,
                volume: vol_state.effective_music_volume() as f32,
            },
        );
        eprintln!(
            "[audio] music track={} file={} forced={} seconds={seconds:.1} muted={}",
            track.index(),
            track.file_name(),
            self.music_forced,
            vol_state.muted || vol_state.music_muted
        );
        self.music = Some((sound, track));
        self.music_playing = true;
        self.music_ends_at = Some(now + seconds);
    }

    /// Stop, play `track` and mark it forced (`FUN_00417610`): it plays again
    /// each time it ends until a stop.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn force_track(
        &mut self,
        track: MdataTrack,
        mdata_dir: &Path,
        vol_state: &AudioVolumeState,
        now: f64,
    ) {
        self.stop_music();
        self.music_forced = true;
        self.start_track(track, mdata_dir, vol_state, now);
    }

    /// The end-of-track event (`FUN_00417780`, message `0x4c9`): a forced
    /// track plays again here; `true` means a free track ended and the caller
    /// starts the next default pick.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn poll_track_end(
        &mut self,
        mdata_dir: &Path,
        vol_state: &AudioVolumeState,
        now: f64,
    ) -> bool {
        if !self.music_playing || self.music_ends_at.is_none_or(|end| now < end) {
            return false;
        }
        self.music_playing = false;
        self.music_ends_at = None;
        let Some((_, track)) = self.music.take() else {
            return false;
        };
        if self.music_forced {
            self.start_track(track, mdata_dir, vol_state, now);
            return false;
        }
        true
    }

    /// Stop the current music track immediately (`FUN_00417640`), which
    /// clears the forced flag.
    pub fn stop_music(&mut self) {
        if let Some((sound, track)) = self.music.take() {
            sound.stop(&self.ctx);
            if self.music_playing {
                eprintln!("[audio] music stopped track={}", track.index());
            }
            self.music_playing = false;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.music_ends_at = None;
            self.music_forced = false;
        }
    }

    /// Apply volume changes to the currently playing music.
    ///
    /// Call when `AudioVolumeState::dirty` is true, then clear `dirty`.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Audio gains are bounded values; the playback API takes f32."
    )]
    pub fn apply_volume(&mut self, vol_state: &AudioVolumeState) {
        let vol = vol_state.effective_music_volume() as f32;
        if let Some((sound, _)) = &self.music {
            sound.set_volume(&self.ctx, vol);
        }
    }

    /// Returns the currently playing track, if any.
    #[allow(dead_code)]
    pub fn current_track(&self) -> Option<MdataTrack> {
        self.music.as_ref().map(|(_, t)| *t)
    }

    /// Always returns `true` — `quad-snd` initialises unconditionally.
    #[expect(
        clippy::unused_self,
        reason = "Keep the same instance API as the audio backend on other platforms."
    )]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn is_available(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tactical_voice_assets_cover_complete_source_backed_banks() {
        // FUN_005bae60 registers the three continuous faction event ranges.
        let assets = tactical_voice_assets();
        assert_eq!(assets.len(), 285);
        let identities = assets
            .iter()
            .map(|(faction, _, resource_id)| (*faction, *resource_id))
            .collect::<HashSet<_>>();
        assert_eq!(identities.len(), 285);
        assert!(assets.contains(&(
            TacticalVoiceFaction::Alliance,
            "voice/alliance/14001-voicefxa.wav".to_string(),
            14_001,
        )));
        assert!(assets.contains(&(
            TacticalVoiceFaction::Empire,
            "voice/empire/15132-voicefxe.wav".to_string(),
            15_132,
        )));
        assert!(assets.contains(&(
            TacticalVoiceFaction::Alliance,
            "voice/alliance/15163-voicefxa.wav".to_string(),
            15_163,
        )));
        assert!(!identities.contains(&(TacticalVoiceFaction::Alliance, 15_001)));
        assert!(!identities.contains(&(TacticalVoiceFaction::Alliance, 15_132)));
        assert!(!identities.contains(&(TacticalVoiceFaction::Empire, 15_133)));
    }
}
