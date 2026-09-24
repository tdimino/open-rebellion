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
//!   music/
//!     main_theme.wav      (MDATA.300 — Return of the Jedi/Battle of Endor cue)
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
//! # Context-based music
//!
//! Call `play_music_for_context` instead of `play_music` when you only know
//! the game context (galaxy map, combat, etc.).  The engine maps contexts to
//! tracks:
//!
//! | Context | Track |
//! |---------|-------|
//! | `MainMenu` | `MainTheme` |
//! | `GalaxyMap` | `MainTheme` |
//! | Combat | Battle |
//! | Victory | Victory |
//! | Defeat | Defeat |
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
//! // Context-based: engine picks the right track.
//! audio.play_music_for_context(MusicContext::GalaxyMap, &vol_state);
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

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use quad_snd::{AudioContext, Sound};

pub use rebellion_render::audio::{AudioVolumeState, MusicContext, MusicTrack, SfxKind};
use rebellion_render::tactical_view::TacticalVoiceFaction;

#[cfg(target_arch = "wasm32")]
const AUDIO_PREFIX: &str = "web/data";
#[cfg(not(target_arch = "wasm32"))]
const AUDIO_PREFIX: &str = "data";

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

fn music_file(track: MusicTrack) -> &'static str {
    match track {
        MusicTrack::MainTheme => "main_theme.wav",
        MusicTrack::Battle => "battle.wav",
        MusicTrack::Victory => "victory.wav",
        MusicTrack::Defeat => "defeat.wav",
        MusicTrack::Imperial => "imperial.wav",
        MusicTrack::Hoth => "hoth.wav",
        MusicTrack::Endor => "endor.wav",
    }
}

/// Select a `MusicTrack` for a given `MusicContext`.
pub fn track_for_context(ctx: MusicContext) -> MusicTrack {
    match ctx {
        MusicContext::MainMenu | MusicContext::GalaxyMap => MusicTrack::MainTheme,
        MusicContext::Combat => MusicTrack::Battle,
        MusicContext::Victory => MusicTrack::Victory,
        MusicContext::Defeat => MusicTrack::Defeat,
    }
}

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

/// Source-proven tactical command banks. These cover battle-ready and the
/// group-specific maneuver, attack, formation, and mission acknowledgements.
pub const TACTICAL_VOICE_RANGES: &[(TacticalVoiceFaction, u32, u32)] = &[
    (TacticalVoiceFaction::Alliance, 14_001, 14_001),
    (TacticalVoiceFaction::Alliance, 14_003, 14_014),
    (TacticalVoiceFaction::Alliance, 14_029, 14_040),
    (TacticalVoiceFaction::Alliance, 14_080, 14_087),
    (TacticalVoiceFaction::Alliance, 14_089, 14_100),
    (TacticalVoiceFaction::Empire, 15_001, 15_001),
    (TacticalVoiceFaction::Empire, 15_003, 15_014),
    (TacticalVoiceFaction::Empire, 15_029, 15_040),
    (TacticalVoiceFaction::Empire, 15_084, 15_091),
    (TacticalVoiceFaction::Empire, 15_093, 15_104),
];

/// Runtime-pack path and resource identity for every restored command voice.
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

    /// Source-mapped tactical command voices keyed by faction and WAVE ID.
    tactical_voice: HashMap<(TacticalVoiceFaction, u32), Sound>,

    /// Currently loaded music track + which track it is.
    music: Option<(Sound, MusicTrack)>,

    /// Whether music is currently playing.
    music_playing: bool,

    /// Missing tracks already reported, keeping diagnostics bounded.
    missing_music_logged: HashSet<MusicTrack>,
}

impl AudioEngine {
    /// Create a new engine and initialise the `quad-snd` audio context.
    pub fn new() -> Self {
        AudioEngine {
            ctx: AudioContext::new(),
            sfx: HashMap::new(),
            tactical_sfx: HashMap::new(),
            tactical_voice: HashMap::new(),
            music: None,
            music_playing: false,
            missing_music_logged: HashSet::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Loading
    // -----------------------------------------------------------------------

    /// Pre-load all SFX from `sounds_dir/sfx/`.  Missing files are silently skipped.
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

    /// Pre-load the source-proven tactical command voice bank. Missing files
    /// remain silent so an installation without the original DLLs still runs.
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
    /// SFX and voice lines are pre-loaded.  Music is loaded on demand in
    /// `play_music`.
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

    /// Load source-proven command acknowledgements from owned faction DLLs.
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

    /// Load one source-mapped tactical command voice from raw bytes.
    pub fn load_tactical_voice_bytes(
        &mut self,
        faction: TacticalVoiceFaction,
        resource_id: u32,
        bytes: &[u8],
    ) {
        let sound = Sound::load(&self.ctx, bytes);
        self.tactical_voice.insert((faction, resource_id), sound);
    }

    /// Begin decoding a music track supplied by the browser runtime pack.
    ///
    /// WebAudio decoding is asynchronous. Call `try_play_loaded_music` on
    /// subsequent frames after a user gesture rather than attempting playback
    /// before the decoded buffer exists.
    #[cfg(target_arch = "wasm32")]
    pub fn load_music_bytes(&mut self, track: MusicTrack, bytes: &[u8]) {
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

    /// Play one exact tactical command voice at the current SFX volume.
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
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Audio gains are bounded values; the playback API takes f32."
    )]
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

        let staged_path = audio_base_path(sounds_dir)
            .join("music")
            .join(music_file(track));
        let path = if staged_path.exists() {
            staged_path
        } else if track == MusicTrack::MainTheme {
            let configured = std::env::var_os("REBELLION_MDATA_DIR")
                .map(PathBuf::from)
                .map(|directory| directory.join("MDATA.300"));
            configured
                .filter(|candidate| candidate.exists())
                .unwrap_or_else(|| PathBuf::from("../star-wars-rebellion/MDATA/MDATA.300"))
        } else {
            staged_path
        };
        if !path.exists() {
            if self.missing_music_logged.insert(track) {
                eprintln!(
                    "[audio] music unavailable track={track:?} expected_path={}",
                    path.display()
                );
            }
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
        sound.play(
            &self.ctx,
            quad_snd::PlaySoundParams {
                looped: true,
                volume: vol,
            },
        );
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

    /// Returns the currently playing `MusicTrack`, if any.
    #[allow(dead_code)]
    pub fn current_track(&self) -> Option<MusicTrack> {
        self.music.as_ref().map(|(_, t)| *t)
    }

    /// Always returns `true` — `quad-snd` initialises unconditionally.
    #[expect(
        clippy::unused_self,
        reason = "Keep the same instance API as the audio backend on other platforms."
    )]
    pub fn is_available(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tactical_voice_assets_cover_exact_source_backed_command_bank() {
        let assets = tactical_voice_assets();
        assert_eq!(assets.len(), 90);
        let identities = assets
            .iter()
            .map(|(faction, _, resource_id)| (*faction, *resource_id))
            .collect::<HashSet<_>>();
        assert_eq!(identities.len(), 90);
        assert!(assets.contains(&(
            TacticalVoiceFaction::Alliance,
            "voice/alliance/14001-voicefxa.wav".to_string(),
            14_001,
        )));
        assert!(assets.contains(&(
            TacticalVoiceFaction::Empire,
            "voice/empire/15104-voicefxe.wav".to_string(),
            15_104,
        )));
        assert!(!identities.contains(&(TacticalVoiceFaction::Alliance, 14_002)));
        assert!(!identities.contains(&(TacticalVoiceFaction::Empire, 15_002)));
        assert!(!identities.contains(&(TacticalVoiceFaction::Alliance, 14_088)));
        assert!(!identities.contains(&(TacticalVoiceFaction::Empire, 15_092)));
    }
}
