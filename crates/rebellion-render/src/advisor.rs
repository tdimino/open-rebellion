//! The cockpit droids: C-3PO and R2-D2 for the Alliance, IMP-22 and SD-7 for
//! the Empire, drawn into the apertures `FUN_0042adb0` gives them.
//!
//! Each droid rests on one still frame and moves only when its side's
//! advice agent fires a reaction, or idle chatter comes due
//! ([`crate::advisor_script`], `ghidra/notes/droid-advisor-triggers.md`).
//! A frame is an anchor bitmap of the side's sprite DLL, or a type-302
//! delta applied to the frame before it: `decode_type302_frame` reproduces
//! the original's 17-byte header, scanline offsets, skips and additive runs
//! (`FUN_0041c6c0`, `FUN_0041c7a0`, `FUN_0041c930`).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use egui_macroquad::egui::{self, TextureHandle, TextureOptions};

use crate::advisor_script::{
    script_words, ActionTable, AdviceAgent, DroidEvent, DroidFrame, DroidPlayer, PassOutcome,
    ResolvedAction, TICK_SECONDS,
};
use crate::cockpit::{CockpitFaction, CockpitState};

// ---------------------------------------------------------------------------
// Advisor faction
// ---------------------------------------------------------------------------

/// Which droid set to show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvisorFaction {
    /// C-3PO + R2-D2
    Alliance,
    /// Imperial protocol droid
    Empire,
}

impl From<CockpitFaction> for AdvisorFaction {
    fn from(f: CockpitFaction) -> Self {
        match f {
            CockpitFaction::Alliance => AdvisorFaction::Alliance,
            CockpitFaction::Empire => AdvisorFaction::Empire,
        }
    }
}

/// Exact decoded pixels from one original PE type-302 sparse frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedAdvisorFrame {
    pub width: usize,
    pub height: usize,
    pub indices: Vec<u8>,
    pub rgba: Vec<u8>,
}

/// Indexed base bitmap used by the original additive type-302 renderer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvisorFrameBase {
    pub width: usize,
    pub height: usize,
    pub indices: Vec<u8>,
    pub palette: Vec<[u8; 3]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdvisorFrameError {
    TruncatedHeader {
        actual_len: usize,
    },
    InvalidDimensions {
        width: usize,
        height: usize,
    },
    BaseDimensionsMismatch {
        frame_width: usize,
        frame_height: usize,
        base_width: usize,
        base_height: usize,
    },
    SizeMismatch {
        declared: usize,
        actual: usize,
    },
    RowOffsetOutOfBounds {
        row: usize,
        offset: usize,
    },
    TruncatedRun {
        row: usize,
        x: usize,
    },
    RowOverflow {
        row: usize,
        x: usize,
        count: usize,
    },
    PaletteTooSmall {
        index: usize,
        available: usize,
    },
    InvalidAnchorBitmap,
}

/// Decode the sparse scanline format loaded by original functions
/// `FUN_0041c6c0`, `FUN_0041c7a0`, and `FUN_0041c930`.
///
/// Each scanline begins at its authored little-endian offset and alternates an
/// unchanged-pixel skip count with a literal run of 8-bit values. The original
/// renderer adds each literal byte to the corresponding anchor pixel with
/// wrapping arithmetic before palette lookup. The resource is a delta, not a
/// standalone transparent image.
///
/// # Errors
/// Returns an error for invalid dimensions, inconsistent anchor data, truncated
/// payloads, or malformed row offsets and runs.
pub fn decode_type302_frame(
    bytes: &[u8],
    base: &AdvisorFrameBase,
) -> Result<DecodedAdvisorFrame, AdvisorFrameError> {
    if bytes.len() < TYPE302_HEADER_LEN {
        return Err(AdvisorFrameError::TruncatedHeader {
            actual_len: bytes.len(),
        });
    }

    let width = u16::from_le_bytes([bytes[0], bytes[1]]) as usize;
    let height = u16::from_le_bytes([bytes[2], bytes[3]]) as usize;
    let declared = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    if width == 0
        || height == 0
        || width > MAX_TYPE302_WIDTH
        || height > MAX_TYPE302_HEIGHT
        || width.saturating_mul(height) > MAX_TYPE302_PIXELS
    {
        return Err(AdvisorFrameError::InvalidDimensions { width, height });
    }
    if width != base.width || height != base.height {
        return Err(AdvisorFrameError::BaseDimensionsMismatch {
            frame_width: width,
            frame_height: height,
            base_width: base.width,
            base_height: base.height,
        });
    }

    let payload_start = TYPE302_HEADER_LEN
        .checked_add(
            height
                .checked_mul(4)
                .ok_or(AdvisorFrameError::InvalidDimensions { width, height })?,
        )
        .ok_or(AdvisorFrameError::InvalidDimensions { width, height })?;
    let payload = bytes
        .get(payload_start..)
        .ok_or(AdvisorFrameError::SizeMismatch {
            declared,
            actual: bytes.len().saturating_sub(payload_start),
        })?;
    if payload.len() != declared {
        return Err(AdvisorFrameError::SizeMismatch {
            declared,
            actual: payload.len(),
        });
    }

    let pixel_count = width
        .checked_mul(height)
        .ok_or(AdvisorFrameError::InvalidDimensions { width, height })?;
    if base.indices.len() != pixel_count {
        return Err(AdvisorFrameError::InvalidAnchorBitmap);
    }
    let mut indices = base.indices.clone();
    for row in 0..height {
        let offset_start = TYPE302_HEADER_LEN + row * 4;
        let offset = u32::from_le_bytes([
            bytes[offset_start],
            bytes[offset_start + 1],
            bytes[offset_start + 2],
            bytes[offset_start + 3],
        ]) as usize;
        if offset >= payload.len() {
            return Err(AdvisorFrameError::RowOffsetOutOfBounds { row, offset });
        }

        let mut cursor = offset;
        let mut x = 0usize;
        let mut literal = false;
        while x < width {
            let count = *payload
                .get(cursor)
                .ok_or(AdvisorFrameError::TruncatedRun { row, x })?
                as usize;
            cursor += 1;
            if x + count > width {
                return Err(AdvisorFrameError::RowOverflow { row, x, count });
            }
            if literal {
                let deltas = payload
                    .get(cursor..cursor + count)
                    .ok_or(AdvisorFrameError::TruncatedRun { row, x })?;
                for (run_x, &delta) in deltas.iter().enumerate() {
                    let destination = row * width + x + run_x;
                    indices[destination] = indices[destination].wrapping_add(delta);
                }
                cursor += count;
            }
            x += count;
            literal = !literal;
        }
    }

    indexed_frame(width, height, &indices, &base.palette)
}

fn indexed_frame(
    width: usize,
    height: usize,
    indices: &[u8],
    palette: &[[u8; 3]],
) -> Result<DecodedAdvisorFrame, AdvisorFrameError> {
    let mut rgba = Vec::with_capacity(indices.len() * 4);
    for &index in indices {
        let color = palette
            .get(index as usize)
            .ok_or(AdvisorFrameError::PaletteTooSmall {
                index: index as usize,
                available: palette.len(),
            })?;
        let alpha = if index == 0 { 0 } else { 255 };
        rgba.extend_from_slice(&[color[0], color[1], color[2], alpha]);
    }
    Ok(DecodedAdvisorFrame {
        width,
        height,
        indices: indices.to_vec(),
        rgba,
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_sign_loss,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
fn decode_anchor_bitmap(bytes: &[u8]) -> Result<AdvisorFrameBase, AdvisorFrameError> {
    if bytes.get(0..2) != Some(b"BM") || bytes.len() < 54 {
        return Err(AdvisorFrameError::InvalidAnchorBitmap);
    }
    let pixel_offset = u32::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13]]) as usize;
    let dib_start = 14usize;
    let header_size = u32::from_le_bytes([
        bytes[dib_start],
        bytes[dib_start + 1],
        bytes[dib_start + 2],
        bytes[dib_start + 3],
    ]) as usize;
    let palette_start = dib_start
        .checked_add(header_size)
        .ok_or(AdvisorFrameError::InvalidAnchorBitmap)?;
    if header_size < 40 || palette_start > bytes.len() {
        return Err(AdvisorFrameError::InvalidAnchorBitmap);
    }
    let width_signed = i32::from_le_bytes([bytes[18], bytes[19], bytes[20], bytes[21]]);
    let height_signed = i32::from_le_bytes([bytes[22], bytes[23], bytes[24], bytes[25]]);
    let planes = u16::from_le_bytes([bytes[26], bytes[27]]);
    let bit_count = u16::from_le_bytes([bytes[dib_start + 14], bytes[dib_start + 15]]);
    let compression = u32::from_le_bytes([bytes[30], bytes[31], bytes[32], bytes[33]]);
    if width_signed <= 0
        || height_signed == 0
        || height_signed == i32::MIN
        || planes != 1
        || bit_count != 8
        || compression != 0
    {
        return Err(AdvisorFrameError::InvalidAnchorBitmap);
    }
    let width = width_signed as usize;
    let height = height_signed.unsigned_abs() as usize;
    if width > MAX_TYPE302_WIDTH
        || height > MAX_TYPE302_HEIGHT
        || width.saturating_mul(height) > MAX_TYPE302_PIXELS
    {
        return Err(AdvisorFrameError::InvalidAnchorBitmap);
    }
    let colors_used = u32::from_le_bytes([
        bytes[dib_start + 32],
        bytes[dib_start + 33],
        bytes[dib_start + 34],
        bytes[dib_start + 35],
    ]) as usize;
    let color_count = if colors_used == 0 { 256 } else { colors_used };
    if !(1..=256).contains(&color_count) {
        return Err(AdvisorFrameError::InvalidAnchorBitmap);
    }
    let palette_bytes = color_count
        .checked_mul(4)
        .ok_or(AdvisorFrameError::InvalidAnchorBitmap)?;
    let palette_end = palette_start
        .checked_add(palette_bytes)
        .ok_or(AdvisorFrameError::InvalidAnchorBitmap)?;
    if pixel_offset < palette_end {
        return Err(AdvisorFrameError::InvalidAnchorBitmap);
    }
    let entries = bytes
        .get(palette_start..palette_end)
        .ok_or(AdvisorFrameError::InvalidAnchorBitmap)?;
    let palette: Vec<[u8; 3]> = entries
        .as_chunks::<4>()
        .0
        .iter()
        .map(|entry| [entry[2], entry[1], entry[0]])
        .collect();

    let row_stride = width
        .checked_add(3)
        .map(|value| value & !3)
        .ok_or(AdvisorFrameError::InvalidAnchorBitmap)?;
    let pixel_bytes = row_stride
        .checked_mul(height)
        .ok_or(AdvisorFrameError::InvalidAnchorBitmap)?;
    let source_end = pixel_offset
        .checked_add(pixel_bytes)
        .ok_or(AdvisorFrameError::InvalidAnchorBitmap)?;
    let source = bytes
        .get(pixel_offset..source_end)
        .ok_or(AdvisorFrameError::InvalidAnchorBitmap)?;
    let mut indices = vec![0; width * height];
    for output_row in 0..height {
        let source_row = if height_signed > 0 {
            height - 1 - output_row
        } else {
            output_row
        };
        let source_start = source_row * row_stride;
        indices[output_row * width..(output_row + 1) * width]
            .copy_from_slice(&source[source_start..source_start + width]);
    }
    if let Some(&index) = indices
        .iter()
        .find(|&&index| index as usize >= palette.len())
    {
        return Err(AdvisorFrameError::PaletteTooSmall {
            index: index as usize,
            available: palette.len(),
        });
    }

    Ok(AdvisorFrameBase {
        width,
        height,
        indices,
        palette,
    })
}

// ---------------------------------------------------------------------------
// Assets
// ---------------------------------------------------------------------------

const TYPE302_HEADER_LEN: usize = 17;
const MAX_TYPE302_WIDTH: usize = 640;
const MAX_TYPE302_HEIGHT: usize = 480;
const MAX_TYPE302_PIXELS: usize = MAX_TYPE302_WIDTH * MAX_TYPE302_HEIGHT;

/// The agent droid's rest script (`FUN_0042adb0`: `9:0x193`, both sides).
const AGENT_REST_SCRIPT: u16 = 0x193;

pub(crate) use crate::advisor_script::{BRIEF_MODULE, SPRITE_MODULE};

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
struct WasmAdvisorAssets {
    frames: HashMap<String, Vec<u8>>,
    bitmaps: HashMap<String, Vec<u8>>,
}

#[cfg(target_arch = "wasm32")]
static WASM_ADVISOR_ASSETS: std::sync::LazyLock<std::sync::Mutex<WasmAdvisorAssets>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(WasmAdvisorAssets::default()));

/// Install the advisor resources unpacked by the browser runtime pack:
/// type-302 frames as `dll/id`, action scripts as `dll/rcdata/id`, sounds as
/// `dll/wave/id` and the side's advice table as `dll/spt`, beside the anchor
/// bitmaps.
#[cfg(target_arch = "wasm32")]
pub fn set_advisor_asset_cache(
    frames: HashMap<String, Vec<u8>>,
    bitmaps: HashMap<String, Vec<u8>>,
) {
    *WASM_ADVISOR_ASSETS.lock().unwrap() = WasmAdvisorAssets { frames, bitmaps };
}

/// A staged advisor resource of one of the side's DLLs.
#[derive(Debug, Clone, Copy)]
enum AdvisorAsset {
    Anchor(u16),
    Delta(u16),
    Script(u16),
    Wave(u16),
    Table(&'static str),
}

impl AdvisorFaction {
    /// The staged directory of `FUN_005fefd0` module `module`: the side's
    /// briefing DLL for module 13, its sprite DLL otherwise.
    const fn dll_dir(self, module: u16) -> &'static str {
        match (self, module) {
            (Self::Alliance, BRIEF_MODULE) => "albrief-dll",
            (Self::Empire, BRIEF_MODULE) => "embrief-dll",
            (Self::Alliance, _) => "alsprite-dll",
            (Self::Empire, _) => "emsprite-dll",
        }
    }

    /// The side's advice table (`FUN_004c2c70`, `FUN_004c0ba0`).
    const fn table_name(self) -> &'static str {
        match self {
            Self::Alliance => "C3POACT.SPT",
            Self::Empire => "IMP22ACT.SPT",
        }
    }

    /// The partner droid's rest script (`FUN_0042adb0`: `9:0x192` for the
    /// Alliance, `9:0x191` for the Empire).
    const fn partner_rest_script(self) -> u16 {
        match self {
            Self::Alliance => 0x192,
            Self::Empire => 0x191,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_advisor_asset(root: &Path, dll: &str, asset: AdvisorAsset) -> Option<Vec<u8>> {
    let dir = root.join(dll);
    let path = match asset {
        AdvisorAsset::Anchor(id) => dir.join("BMP").join(format!("{id}.bmp")),
        AdvisorAsset::Delta(id) => dir.join("TYPE302").join(format!("{id}.bin")),
        AdvisorAsset::Script(id) => dir.join("RCDATA").join(format!("{id}.bin")),
        AdvisorAsset::Wave(id) => dir.join("WAVE").join(format!("{id}.wav")),
        AdvisorAsset::Table(name) => dir.join("SPT").join(name),
    };
    std::fs::read(path).ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn list_advisor_waves(root: &Path, dll: &str) -> Vec<u16> {
    std::fs::read_dir(root.join(dll).join("WAVE"))
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok()?.path().file_stem()?.to_str()?.parse().ok())
        .collect()
}

#[cfg(target_arch = "wasm32")]
fn read_advisor_asset(_root: &Path, dll: &str, asset: AdvisorAsset) -> Option<Vec<u8>> {
    let web = WASM_ADVISOR_ASSETS.lock().unwrap();
    match asset {
        AdvisorAsset::Anchor(id) => web.bitmaps.get(&format!("{dll}/{id}")),
        AdvisorAsset::Delta(id) => web.frames.get(&format!("{dll}/{id}")),
        AdvisorAsset::Script(id) => web.frames.get(&format!("{dll}/rcdata/{id}")),
        AdvisorAsset::Wave(id) => web.frames.get(&format!("{dll}/wave/{id}")),
        AdvisorAsset::Table(_table_name) => web.frames.get(&format!("{dll}/spt")),
    }
    .cloned()
}

#[cfg(target_arch = "wasm32")]
fn list_advisor_waves(_root: &Path, dll: &str) -> Vec<u16> {
    let prefix = format!("{dll}/wave/");
    let web = WASM_ADVISOR_ASSETS.lock().unwrap();
    web.frames
        .keys()
        .filter_map(|key| key.strip_prefix(&prefix)?.parse().ok())
        .collect()
}

/// A staged WAVE resource of any DLL: `root/<dll>/WAVE/<id>.wav` natively,
/// `<dll>/wave/<id>` from the browser runtime pack. The Message Index's
/// sounds are STRATEGY's (`strategy-dll`).
#[must_use]
pub fn staged_wave(root: &Path, dll: &str, id: u16) -> Option<Vec<u8>> {
    read_advisor_asset(root, dll, AdvisorAsset::Wave(id))
}

/// A droid sound for the app's audio engine: a cache key unique across both
/// sides, and the WAVE resource of its module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvisorVoice {
    pub key: String,
    pub module: u16,
    pub wave: u16,
}

/// A cockpit step the agent's pass runs (`FUN_004c3060`): kind 2 runs
/// briefing step `step` (`FUN_004c30c0`), kind 3 selects an object
/// (`FUN_0041d830`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CockpitStep {
    pub kind: u32,
    pub step: u32,
}

/// One anchor's run of frames, decoded as far as the droids have needed:
/// the anchor bitmap, then each type-302 delta applied to the frame before.
struct AnchorRun {
    base: Option<AdvisorFrameBase>,
    textures: Vec<TextureHandle>,
    broken: bool,
}

// ---------------------------------------------------------------------------
// AdvisorState
// ---------------------------------------------------------------------------

/// The cockpit droids: the side's advice agent and its two players
/// (`ghidra/notes/droid-advisor-triggers.md`).
pub struct AdvisorState {
    /// Which faction's droid set is active.
    pub faction: AdvisorFaction,
    /// Whether the droids are drawn.
    pub visible: bool,
    /// Staged UI root holding `alsprite-dll` and `emsprite-dll`.
    sprite_dir: PathBuf,
    loaded: bool,
    table: ActionTable,
    scripts: HashMap<(u16, u16), Option<Vec<u16>>>,
    agent: AdviceAgent,
    agent_droid: DroidPlayer,
    partner_droid: DroidPlayer,
    /// The scheduler's step count (`FUN_004fcee0`), fractional.
    steps: f64,
    /// Seconds toward the droids' next 67 ms tick.
    tick_clock: f32,
    runs: HashMap<(u16, u16), AnchorRun>,
    /// Sounds started since the app last took them, as (module, wave).
    voices: Vec<(u16, u16)>,
    /// `DAT_006b28c4`, `DAT_006b28c8`: the step command `0x11` left for the
    /// next pass.
    pending_step: Option<CockpitStep>,
    /// Steps the passes ran since the app last took them.
    steps_run: Vec<CockpitStep>,
    /// `DAT_006b14bc`: the briefing tour holds the cockpit's input, from
    /// step 12 (`FUN_0041d9d0`) to step 13 (`FUN_0041da80`).
    tour_active: bool,
    /// `DAT_006b14b4`: the tour was skipped, which it can be once.
    tour_skipped: bool,
    /// A new game's clock holds through the tour until its step 11
    /// (`FUN_0041dbe0` → `FUN_0041e320`).
    holds_clock: bool,
}

impl AdvisorState {
    /// Create the droids for `faction`, resting until their assets load.
    #[must_use]
    pub fn new(faction: AdvisorFaction) -> Self {
        Self {
            faction,
            visible: true,
            // WASM resolves assets through the runtime pack; native builds
            // replace this with the staged UI directory.
            sprite_dir: PathBuf::new(),
            loaded: false,
            table: ActionTable::default(),
            scripts: HashMap::new(),
            agent: AdviceAgent::new(faction),
            agent_droid: DroidPlayer::new(0, true),
            partner_droid: DroidPlayer::new(0, false),
            // hyp: the scheduler has counted a step by the time the agent
            // first passes; at 0 no slot could fire (`+0x16c` < now) while
            // the briefing holds the clock.
            steps: 1.0,
            tick_clock: 0.0,
            runs: HashMap::new(),
            voices: Vec::new(),
            pending_step: None,
            steps_run: Vec::new(),
            tour_active: false,
            tour_skipped: false,
            holds_clock: true,
        }
    }

    /// Set the staged UI root, and start the droids over.
    pub fn set_sprite_dir(&mut self, path: impl Into<PathBuf>) {
        let faction = self.faction;
        *self = Self::new(faction);
        self.sprite_dir = path.into();
    }

    /// Start `faction`'s droids for a loaded game: the briefing has run, so
    /// the opening slot is dropped and the clock is not held
    /// (`FUN_00439320` sets `0x8000` unless the session is a new game).
    pub fn resume_saved_game(&mut self, faction: AdvisorFaction) {
        let sprite_dir = std::mem::take(&mut self.sprite_dir);
        *self = Self::new(faction);
        self.sprite_dir = sprite_dir;
        self.agent.clear_opening();
        self.holds_clock = false;
    }

    /// Whether the briefing tour holds the game clock.
    #[must_use]
    pub const fn holds_clock(&self) -> bool {
        self.holds_clock
    }

    /// Switch to `faction`'s droids, starting them over.
    pub fn set_faction(&mut self, faction: AdvisorFaction) {
        if self.faction == faction {
            return;
        }
        let sprite_dir = std::mem::take(&mut self.sprite_dir);
        *self = Self::new(faction);
        self.sprite_dir = sprite_dir;
    }

    fn asset(&self, module: u16, asset: AdvisorAsset) -> Option<Vec<u8>> {
        read_advisor_asset(&self.sprite_dir, self.faction.dll_dir(module), asset)
    }

    fn script(&mut self, module: u16, id: u16) -> Option<Vec<u16>> {
        if !self.scripts.contains_key(&(module, id)) {
            let words = self
                .asset(module, AdvisorAsset::Script(id))
                .map(|bytes| script_words(&bytes));
            self.scripts.insert((module, id), words);
        }
        self.scripts.get(&(module, id)).cloned().flatten()
    }

    /// Load the advice table and the rest frames once.
    fn ensure_loaded(&mut self) {
        if self.loaded {
            return;
        }
        self.loaded = true;
        match self
            .asset(
                SPRITE_MODULE,
                AdvisorAsset::Table(self.faction.table_name()),
            )
            .and_then(|bytes| ActionTable::parse(&bytes))
        {
            Some(table) => {
                // Without the tour's records nothing would release the clock.
                if table
                    .records(AdviceAgent::opening_slot(self.faction))
                    .is_empty()
                {
                    self.holds_clock = false;
                }
                self.table = table;
            }
            None => {
                self.holds_clock = false;
                macroquad::logging::warn!(
                    "[advisor] missing advice table {}; restage UI assets",
                    self.faction.table_name()
                );
            }
        }
        let agent_rest = self
            .script(SPRITE_MODULE, AGENT_REST_SCRIPT)
            .and_then(|words| words.get(1).copied());
        let partner_rest = self
            .script(SPRITE_MODULE, self.faction.partner_rest_script())
            .and_then(|words| words.get(1).copied());
        match (agent_rest, partner_rest) {
            (Some(agent), Some(partner)) => {
                self.agent_droid = DroidPlayer::new(agent, true);
                self.partner_droid = DroidPlayer::new(partner, false);
            }
            _ => {
                macroquad::logging::warn!(
                    "[advisor] missing rest scripts in {}; restage UI assets",
                    self.faction.dll_dir(SPRITE_MODULE)
                );
            }
        }
    }

    /// The scheduler's step count (`DAT_006b28cc`).
    #[must_use]
    pub fn now(&self) -> u32 {
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "the step count is a non-negative whole number of steps"
        )]
        let now = self.steps as u32;
        now
    }

    fn voice(&self, module: u16, wave: u16) -> AdvisorVoice {
        AdvisorVoice {
            key: format!("{}/{wave}", self.faction.dll_dir(module)),
            module,
            wave,
        }
    }

    /// The sounds the droids started since the last call.
    pub fn take_voices(&mut self) -> Vec<AdvisorVoice> {
        let started = std::mem::take(&mut self.voices);
        started
            .into_iter()
            .map(|(module, wave)| self.voice(module, wave))
            .collect()
    }

    /// Every sound of the side's sprite and briefing DLLs, for loading ahead
    /// of play: the browser decodes audio asynchronously.
    #[must_use]
    pub fn voice_clips(&self) -> Vec<AdvisorVoice> {
        [SPRITE_MODULE, BRIEF_MODULE]
            .into_iter()
            .flat_map(|module| {
                let mut waves = list_advisor_waves(&self.sprite_dir, self.faction.dll_dir(module));
                waves.sort_unstable();
                waves.into_iter().map(move |wave| (module, wave))
            })
            .map(|(module, wave)| self.voice(module, wave))
            .collect()
    }

    /// The bytes of `voice`'s WAVE resource.
    #[must_use]
    pub fn voice_bytes(&self, voice: &AdvisorVoice) -> Option<Vec<u8>> {
        self.asset(voice.module, AdvisorAsset::Wave(voice.wave))
    }

    /// Whether the briefing tour holds the cockpit's input.
    #[must_use]
    pub const fn tour_active(&self) -> bool {
        self.tour_active
    }

    /// The player pressed Escape or a mouse button during the tour
    /// (`lpfn_0041d8f0`, `lpfn_0041d950` → `FUN_0043a200`). Only the first
    /// press counts (`FUN_0041da30`: `DAT_006b14b4`).
    pub fn skip_tour(&mut self) {
        if self.tour_active && !self.tour_skipped {
            self.tour_skipped = true;
            self.agent.request_skip();
        }
    }

    /// The cockpit steps the agent ran since the last call.
    pub fn take_cockpit_steps(&mut self) -> Vec<CockpitStep> {
        std::mem::take(&mut self.steps_run)
    }

    /// The player's order was refused: the agent schedules its reaction
    /// (`FUN_00487c90` → agent slot `+0xc`).
    pub fn refuse(&mut self, status: crate::advisor_script::RefusalStatus) {
        let now = self.now();
        macroquad::logging::info!(
            "[advisor] refusal {:#x}/{:#x} at step {}",
            status.0,
            status.1,
            now
        );
        self.agent
            .refuse(status, now, || macroquad::rand::gen_range(0_u32, 2));
    }

    /// File a game message's advice code (`FUN_0048a060` → agent `VT[5]`).
    pub fn post_code(&mut self, code: u8) {
        let now = self.now();
        macroquad::logging::info!("[advisor] code {:#x} posted at step {}", code, now);
        self.agent
            .post(code, now, |n| macroquad::rand::gen_range(0_u32, n));
    }

    /// Step the droids by `dt` seconds. The step count advances at the
    /// scheduler's rate, `steps_per_second`, while the game clock runs.
    pub fn update(&mut self, dt: f32, steps_per_second: Option<f32>) {
        self.ensure_loaded();
        if let Some(rate) = steps_per_second {
            self.steps += f64::from(dt * rate);
        }
        let now = self.now();
        // FUN_004c2b20 / FUN_004c0a60: a stored cockpit step takes the whole
        // pass (FUN_004c3060).
        if let Some(step) = self.pending_step.take() {
            macroquad::logging::info!(
                "[advisor] cockpit step {} {} at step {}",
                step.kind,
                step.step,
                now
            );
            if step.kind == 2 {
                match step.step {
                    11 => self.holds_clock = false,
                    12 => self.tour_active = true,
                    13 => self.tour_active = false,
                    _ => {}
                }
            }
            self.steps_run.push(step);
        } else {
            match self
                .agent
                .pass(now, || macroquad::rand::gen_range(0_u32, 12))
            {
                PassOutcome::Fired(slot) => self.queue_slot(slot),
                PassOutcome::FlushDroids => {
                    macroquad::logging::info!("[advisor] briefing skipped at step {}", now);
                    self.agent_droid.flush();
                    self.partner_droid.flush();
                }
                PassOutcome::Idle => {}
            }
        }
        self.tick_clock += dt;
        let mut events = Vec::new();
        while self.tick_clock >= TICK_SECONDS {
            self.tick_clock -= TICK_SECONDS;
            self.agent_droid.tick(&mut events);
            self.partner_droid.tick(&mut events);
            for event in events.drain(..) {
                self.route(event, now);
            }
        }
    }

    fn route(&mut self, event: DroidEvent, now: u32) {
        match event {
            DroidEvent::ArmChatter => self.agent.arm(now),
            // FUN_0042b290 loads the partner's actions from module 9.
            DroidEvent::PartnerAction(action) => {
                if let Some(action) = self.resolve(SPRITE_MODULE, action, 0, 0) {
                    self.partner_droid.enqueue(&action);
                }
            }
            DroidEvent::PartnerSignedOff => self.agent_droid.partner_signed_off(),
            DroidEvent::Sound { wave, module } => self.voices.push((module, wave)),
            DroidEvent::CockpitStep { kind, step } => {
                self.pending_step = Some(CockpitStep { kind, step });
            }
        }
    }

    /// An action script resolved to its command (`FUN_0042b1d0`): the words
    /// `(command, script, p2, p3)` of `module`, with the record's `p2` and
    /// `p3` when it has them.
    fn resolve(&mut self, module: u16, action: u16, p2: u32, p3: u32) -> Option<ResolvedAction> {
        let header = self.script(module, action)?;
        let command = *header.first()?;
        let script = *header.get(1)?;
        let words = self.script(module, script)?;
        let or_default = |given: u32, index: usize| {
            if given == 0 {
                u32::from(header.get(index).copied().unwrap_or(0))
            } else {
                given
            }
        };
        Some(ResolvedAction {
            module,
            command,
            p2: or_default(p2, 2),
            p3: or_default(p3, 3),
            words,
        })
    }

    /// Queue a fired slot's records on the agent droid (`FUN_004c2b20`).
    /// Spoken advice plays: the original sets `0x1000` when the agent is
    /// made and nothing in the port sets `0x4000`.
    fn queue_slot(&mut self, slot: u32) {
        let records = self.table.records(slot).to_vec();
        macroquad::logging::info!(
            "[advisor] slot {slot} fired at step {} ({} records)",
            self.now(),
            records.len()
        );
        for record in records {
            let module = if record.from_briefing() {
                BRIEF_MODULE
            } else {
                SPRITE_MODULE
            };
            if let Some(action) = self.resolve(module, record.action, record.p2, record.p3) {
                self.agent_droid.enqueue(&action);
            }
        }
    }

    /// The texture of `frame`, decoding its anchor's run up to it.
    fn texture(&mut self, ctx: &egui::Context, frame: DroidFrame) -> Option<TextureHandle> {
        let key = (frame.module, frame.anchor);
        if !self.runs.contains_key(&key) {
            let base = self
                .asset(frame.module, AdvisorAsset::Anchor(frame.anchor))
                .and_then(|bytes| decode_anchor_bitmap(&bytes).ok());
            let mut run = AnchorRun {
                base,
                textures: Vec::new(),
                broken: false,
            };
            match run
                .base
                .as_ref()
                .map(|base| indexed_frame(base.width, base.height, &base.indices, &base.palette))
            {
                Some(Ok(image)) => {
                    run.textures
                        .push(self.load_texture(ctx, frame.module, frame.anchor, &image))
                }
                _ => run.broken = true,
            }
            self.runs.insert(key, run);
        }
        while self.runs[&key].textures.len() <= usize::from(frame.index) && !self.runs[&key].broken
        {
            let next =
                frame.anchor + u16::try_from(self.runs[&key].textures.len()).unwrap_or(u16::MAX);
            let bytes = self.asset(frame.module, AdvisorAsset::Delta(next));
            let run = self.runs.get_mut(&key)?;
            let decoded = bytes
                .zip(run.base.as_ref())
                .map(|(bytes, base)| decode_type302_frame(&bytes, base));
            match decoded {
                Some(Ok(image)) => {
                    if let Some(base) = run.base.as_mut() {
                        base.indices.clone_from(&image.indices);
                    }
                    let texture = self.load_texture(ctx, frame.module, next, &image);
                    self.runs.get_mut(&key)?.textures.push(texture);
                }
                _ => {
                    macroquad::logging::warn!(
                        "[advisor] frame {next} of run {} did not decode",
                        frame.anchor
                    );
                    run.broken = true;
                }
            }
        }
        let run = &self.runs[&key];
        run.textures
            .get(usize::from(frame.index))
            .or_else(|| run.textures.last())
            .cloned()
    }

    fn load_texture(
        &self,
        ctx: &egui::Context,
        module: u16,
        id: u16,
        image: &DecodedAdvisorFrame,
    ) -> TextureHandle {
        ctx.load_texture(
            format!("advisor_{}_{id}", self.faction.dll_dir(module)),
            egui::ColorImage::from_rgba_unmultiplied([image.width, image.height], &image.rgba),
            TextureOptions::NEAREST,
        )
    }
}

// ---------------------------------------------------------------------------
// Draw
// ---------------------------------------------------------------------------

fn advisor_apertures(faction: AdvisorFaction) -> [(f32, f32, f32, f32); 2] {
    match faction {
        AdvisorFaction::Alliance => [(541.0, 337.0, 67.0, 116.0), (316.0, 411.0, 47.0, 69.0)],
        AdvisorFaction::Empire => [(0.0, 347.0, 107.0, 133.0), (302.0, 401.0, 101.0, 79.0)],
    }
}

/// The agent droid's aperture, the larger of the two (`FUN_0042adb0`), in
/// 640 by 480 canvas points.
#[must_use]
pub fn agent_aperture(faction: CockpitFaction) -> (f32, f32, f32, f32) {
    advisor_apertures(match faction {
        CockpitFaction::Alliance => AdvisorFaction::Alliance,
        CockpitFaction::Empire => AdvisorFaction::Empire,
    })[0]
}

fn scaled_advisor_apertures(
    faction: AdvisorFaction,
    screen_width: f32,
    screen_height: f32,
) -> [(f32, f32, f32, f32); 2] {
    let cockpit_faction = match faction {
        AdvisorFaction::Alliance => CockpitFaction::Alliance,
        AdvisorFaction::Empire => CockpitFaction::Empire,
    };
    let layout = CockpitState::new(cockpit_faction).layout_for(screen_width, screen_height);
    advisor_apertures(faction).map(|(x, y, width, height)| {
        (
            layout.canvas.x + x * layout.scale,
            layout.canvas.y + y * layout.scale,
            width * layout.scale,
            height * layout.scale,
        )
    })
}
/// Draw both droids into their apertures (`FUN_0042adb0`), each frame at
/// its own size from the aperture's corner and clipped to the aperture.
pub fn draw_advisor(ctx: &egui::Context, state: &mut AdvisorState) {
    state.ensure_loaded();
    if !state.visible {
        return;
    }
    let screen = ctx.screen_rect();
    let apertures = scaled_advisor_apertures(state.faction, screen.width(), screen.height());
    let native = advisor_apertures(state.faction);
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Middle,
        egui::Id::new("authentic_droid_advisors"),
    ));
    let frames = [state.agent_droid.frame(), state.partner_droid.frame()];
    for (index, frame) in frames.into_iter().enumerate() {
        let Some(texture) = state.texture(ctx, frame) else {
            continue;
        };
        let (x, y, width, height) = apertures[index];
        let scale = width / native[index].2;
        let corner = egui::pos2(screen.min.x + x, screen.min.y + y);
        #[expect(clippy::cast_precision_loss, reason = "frame sizes are under 640")]
        let size = texture.size().map(|side| side as f32 * scale);
        painter
            .with_clip_rect(egui::Rect::from_min_size(corner, egui::vec2(width, height)))
            .image(
                texture.id(),
                egui::Rect::from_min_size(corner, egui::vec2(size[0], size[1])),
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn advisor_apertures_follow_the_uniform_letterboxed_canvas() {
        let alliance = scaled_advisor_apertures(AdvisorFaction::Alliance, 640.0, 600.0);
        assert_eq!(alliance[0], (541.0, 397.0, 67.0, 116.0));
        assert_eq!(alliance[1], (316.0, 471.0, 47.0, 69.0));

        let empire = scaled_advisor_apertures(AdvisorFaction::Empire, 1280.0, 800.0);
        let scale = 800.0 / 480.0;
        let canvas_x = (1280.0 - 640.0 * scale) / 2.0;
        assert_close(empire[0].0, canvas_x);
        assert_close(empire[0].1, 347.0 * scale);
        assert_close(empire[0].2, 107.0 * scale);
        assert_close(empire[0].3, 133.0 * scale);
        assert_close(empire[1].0, canvas_x + 302.0 * scale);
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn type302_fixture() -> Vec<u8> {
        let payload = [
            1, 2, 4, 5, 1, // row 0: skip 1, draw 2, skip 1
            0, 4, 1, 2, 3, 4, // row 1: draw all 4 pixels
        ];
        let mut bytes = vec![0; TYPE302_HEADER_LEN + 2 * 4];
        bytes[0..2].copy_from_slice(&4_u16.to_le_bytes());
        bytes[2..4].copy_from_slice(&2_u16.to_le_bytes());
        bytes[4..8].copy_from_slice(&(payload.len() as u32).to_le_bytes());
        bytes[TYPE302_HEADER_LEN..TYPE302_HEADER_LEN + 4].copy_from_slice(&0_u32.to_le_bytes());
        bytes[TYPE302_HEADER_LEN + 4..TYPE302_HEADER_LEN + 8].copy_from_slice(&5_u32.to_le_bytes());
        bytes.extend_from_slice(&payload);
        bytes
    }

    fn frame_base() -> AdvisorFrameBase {
        let mut indices = vec![1; 8];
        indices[0] = 0;
        AdvisorFrameBase {
            width: 4,
            height: 2,
            indices,
            palette: (0..=255).map(|index| [index, 0, 255 - index]).collect(),
        }
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn indexed_bmp_fixture() -> Vec<u8> {
        let pixel_offset = 14 + 40 + 256 * 4;
        let mut bytes = vec![0; pixel_offset + 8];
        bytes[0..2].copy_from_slice(b"BM");
        let file_len = bytes.len() as u32;
        bytes[2..6].copy_from_slice(&file_len.to_le_bytes());
        bytes[10..14].copy_from_slice(&(pixel_offset as u32).to_le_bytes());
        bytes[14..18].copy_from_slice(&40_u32.to_le_bytes());
        bytes[18..22].copy_from_slice(&4_i32.to_le_bytes());
        bytes[22..26].copy_from_slice(&2_i32.to_le_bytes());
        bytes[26..28].copy_from_slice(&1_u16.to_le_bytes());
        bytes[28..30].copy_from_slice(&8_u16.to_le_bytes());
        for index in 0..256 {
            let start = 54 + index * 4;
            bytes[start..start + 4].copy_from_slice(&[index as u8, 0, 0, 0]);
        }
        // Positive-height BMP rows are stored bottom-up.
        bytes[pixel_offset..pixel_offset + 4].copy_from_slice(&[5, 6, 7, 8]);
        bytes[pixel_offset + 4..pixel_offset + 8].copy_from_slice(&[1, 2, 3, 4]);
        bytes
    }

    #[test]
    fn type302_decoder_applies_additive_runs_over_anchor_pixels() {
        let decoded = decode_type302_frame(&type302_fixture(), &frame_base()).unwrap();

        assert_eq!((decoded.width, decoded.height), (4, 2));
        assert_eq!(&decoded.rgba[0..4], &[0, 0, 255, 0]);
        assert_eq!(&decoded.rgba[4..8], &[5, 0, 250, 255]);
        assert_eq!(&decoded.rgba[8..12], &[6, 0, 249, 255]);
        assert_eq!(&decoded.rgba[12..16], &[1, 0, 254, 255]);
        assert_eq!(&decoded.rgba[16..20], &[2, 0, 253, 255]);
        assert_eq!(&decoded.rgba[28..32], &[5, 0, 250, 255]);
    }

    #[test]
    fn type302_sequence_applies_each_delta_to_the_previous_frame() {
        let mut base = frame_base();
        let first = decode_type302_frame(&type302_fixture(), &base).unwrap();
        base.indices.clone_from(&first.indices);
        let second = decode_type302_frame(&type302_fixture(), &base).unwrap();

        assert_eq!(first.indices[1], 5);
        assert_eq!(second.indices[1], 9);
        assert_eq!(second.indices[0], 0);
    }

    #[test]
    fn indexed_anchor_decoder_restores_top_down_rows_and_palette() {
        let base = decode_anchor_bitmap(&indexed_bmp_fixture()).unwrap();
        assert_eq!((base.width, base.height), (4, 2));
        assert_eq!(base.indices, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(base.palette[7], [0, 0, 7]);
    }

    #[test]
    fn indexed_anchor_decoder_rejects_pixels_outside_declared_palette() {
        let mut bytes = indexed_bmp_fixture();
        bytes[46..50].copy_from_slice(&1_u32.to_le_bytes());

        assert_eq!(
            decode_anchor_bitmap(&bytes),
            Err(AdvisorFrameError::PaletteTooSmall {
                index: 1,
                available: 1,
            })
        );
    }
    #[test]
    fn type302_decoder_rejects_corrupt_payloads() {
        let base = frame_base();
        let mut truncated = type302_fixture();
        truncated.pop();
        assert!(matches!(
            decode_type302_frame(&truncated, &base),
            Err(AdvisorFrameError::SizeMismatch { .. })
        ));

        let mut bad_offset = type302_fixture();
        bad_offset[TYPE302_HEADER_LEN..TYPE302_HEADER_LEN + 4]
            .copy_from_slice(&999_u32.to_le_bytes());
        assert_eq!(
            decode_type302_frame(&bad_offset, &base),
            Err(AdvisorFrameError::RowOffsetOutOfBounds {
                row: 0,
                offset: 999
            })
        );
    }

    #[test]
    fn type302_decoder_rejects_pathological_dimensions_before_allocation() {
        let height = u16::MAX as usize;
        let mut oversized = vec![0; TYPE302_HEADER_LEN + height * 4 + 1];
        oversized[0..2].copy_from_slice(&u16::MAX.to_le_bytes());
        oversized[2..4].copy_from_slice(&u16::MAX.to_le_bytes());
        oversized[4..8].copy_from_slice(&1_u32.to_le_bytes());

        assert_eq!(
            decode_type302_frame(&oversized, &frame_base()),
            Err(AdvisorFrameError::InvalidDimensions {
                width: u16::MAX as usize,
                height,
            })
        );
    }

    #[test]
    fn advisor_apertures_match_recovered_original_geometry() {
        // Source: FUN_0042adb0 advisor apertures (ghidra/notes/FUN_0042adb0.c).
        assert_eq!(
            advisor_apertures(AdvisorFaction::Alliance),
            [(541.0, 337.0, 67.0, 116.0), (316.0, 411.0, 47.0, 69.0)]
        );
        assert_eq!(
            advisor_apertures(AdvisorFaction::Empire),
            [(0.0, 347.0, 107.0, 133.0), (302.0, 401.0, 101.0, 79.0)]
        );
    }
    #[test]
    fn advisor_faction_converts_from_cockpit_faction() {
        assert_eq!(
            AdvisorFaction::from(CockpitFaction::Alliance),
            AdvisorFaction::Alliance,
        );
        assert_eq!(
            AdvisorFaction::from(CockpitFaction::Empire),
            AdvisorFaction::Empire,
        );
    }
}
