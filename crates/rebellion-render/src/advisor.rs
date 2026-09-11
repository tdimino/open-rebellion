//! Droid advisor system for animated faction advisors that react to game events.
//!
//! Alliance: C-3PO (67×116) + R2-D2 (47×69) animated sprites from ALSPRITE.DLL.
//! Empire: IMP-22 (106×133) + SD-7 (101×79) animated sprites from EMSPRITE.DLL.
//!
//! The dependency-free asset tool preserves the DLLs' standard BMP anchors and
//! all custom PE type-302 frames. `decode_type302_frame` reproduces the original
//! 17-byte header, scanline offsets, unchanged skips, and additive pixel runs
//! recovered from `FUN_0041c6c0`, `FUN_0041c7a0`, and `FUN_0041c930`. Native and WASM use
//! the same decoded bytes and the exact apertures recovered from
//! `FUN_0042adb0`.
//!
//! The current visible idle runs are authoritative resources. Exact SPT/BIN/FDT
//! action selection, cadence, preemption, and sound mapping remain a separate
//! parity task. The legacy cascading BIN parser below is retained only as a
//! development fallback and must not be treated as authored action proof.
//!
//! # Advisor triggers
//!
//! The advisor is activated by `AdvisorTrigger` events pushed from main.rs
//! whenever notable things happen (mission results, combat outcomes, game start,
//! Death Star events, uprisings).  Each trigger includes a message and priority;
//! higher-priority messages preempt lower ones.

use std::collections::{HashMap, VecDeque};
use std::ops::Range;
use std::path::{Path, PathBuf};

use egui_macroquad::egui::{self, TextureHandle, TextureOptions};

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

// ---------------------------------------------------------------------------
// Advisor trigger / message
// ---------------------------------------------------------------------------

/// Priority tier — higher values preempt lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdvisorPriority {
    /// Ambient idle chatter (lowest).
    Low = 0,
    /// Standard game event (mission complete, manufacturing done).
    Normal = 1,
    /// Combat outcome, uprising, betrayal.
    High = 2,
    /// Death Star event, victory/defeat (highest).
    Critical = 3,
}

/// A message the advisor should deliver.
#[derive(Debug, Clone)]
pub struct AdvisorMessage {
    /// Text displayed in the advisor window.
    pub text: String,
    /// How important this message is (preemption).
    pub priority: AdvisorPriority,
    /// How long (seconds) the message stays visible.
    pub display_time: f32,
}

impl AdvisorMessage {
    pub fn new(text: impl Into<String>, priority: AdvisorPriority) -> Self {
        let display_time = match priority {
            AdvisorPriority::Low => 4.0,
            AdvisorPriority::Normal => 5.0,
            AdvisorPriority::High => 6.0,
            AdvisorPriority::Critical => 8.0,
        };
        Self {
            text: text.into(),
            priority,
            display_time,
        }
    }
}

// ---------------------------------------------------------------------------
// Animation frames
// ---------------------------------------------------------------------------

const DEFAULT_FRAME_INTERVAL: f32 = 0.15;
const TYPE302_HEADER_LEN: usize = 17;
const MAX_TYPE302_WIDTH: usize = 640;
const MAX_TYPE302_HEIGHT: usize = 480;
const MAX_TYPE302_PIXELS: usize = MAX_TYPE302_WIDTH * MAX_TYPE302_HEIGHT;

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
struct WasmAdvisorAssets {
    frames: HashMap<String, Vec<u8>>,
    bitmaps: HashMap<String, Vec<u8>>,
}

#[cfg(target_arch = "wasm32")]
static WASM_ADVISOR_ASSETS: std::sync::LazyLock<std::sync::Mutex<WasmAdvisorAssets>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(WasmAdvisorAssets::default()));

/// Install the custom advisor resources unpacked by the browser runtime pack.
#[cfg(target_arch = "wasm32")]
pub fn set_advisor_asset_cache(
    frames: HashMap<String, Vec<u8>>,
    bitmaps: HashMap<String, Vec<u8>>,
) {
    *WASM_ADVISOR_ASSETS.lock().unwrap() = WasmAdvisorAssets { frames, bitmaps };
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

/// Which BIN format variant was used to parse a sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinFormat {
    /// v1: `u16 count + count * u16 frame_ids` — explicit list.
    V1Explicit,
    /// v2: `u16 count + u16 base + u16 0 + u16 0` — sequential range (8 bytes).
    V2Range,
    /// v3: `u16 0 + u16 ref_id + u16 9 + u16 count + u16 base` — BMP-mapped range (10 bytes).
    V3BmpRange,
    /// v4: `u16 0 + u16 ref_id + u16 bmp_id` — single BMP frame (6 bytes).
    V4BmpSingle,
}

/// Parsed animation control data from an advisor BIN file.
#[derive(Debug, Clone, PartialEq)]
pub struct BinSequence {
    /// Best-effort ordered frame IDs from the authored BIN.
    pub frame_ids: Vec<u16>,
    /// Fallback interval used when replaying this sequence.
    pub default_interval: f32,
    /// Which format variant produced this sequence.
    pub format: BinFormat,
    /// Whether `frame_ids` are literal BMP resource IDs (v3/v4) rather than
    /// DLL-internal indices (v1/v2). When true, the frame IDs can be looked
    /// up directly in the `bmp_resource_id_map` instead of using modulo.
    pub bmp_mapped: bool,
}

/// Reasons an advisor BIN file could not be parsed as a frame sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinError {
    /// File ended before the `u16 frame_count` header.
    TruncatedHeader { actual_len: usize },
    /// File declared more frame IDs than were present.
    TruncatedFrames {
        declared_count: usize,
        actual_len: usize,
        expected_len: usize,
    },
    /// File length disagrees with `2 + count * 2`.
    LengthMismatch {
        declared_count: usize,
        actual_len: usize,
        expected_len: usize,
    },
    /// File is too small to carry useful animation data (e.g. 2-byte stubs).
    TooSmall { actual_len: usize },
    /// No format variant matched the file's structure.
    NoFormatMatch { actual_len: usize },
}

/// Parse an advisor BIN control file using the original v1 format only.
///
/// v1 format:
/// - `u16 frame_count` (little-endian)
/// - `u16 frame_id[frame_count]` (little-endian)
///
/// Kept for backwards compatibility and tests. Prefer `parse_advisor_bin_cascade`
/// for production use.
///
/// # Errors
/// Returns an error if the header or frame list is truncated or invalid.
pub fn parse_advisor_bin(bytes: &[u8]) -> Result<BinSequence, BinError> {
    if bytes.len() < 2 {
        return Err(BinError::TruncatedHeader {
            actual_len: bytes.len(),
        });
    }

    let declared_count = u16::from_le_bytes([bytes[0], bytes[1]]) as usize;
    let expected_len = 2 + declared_count * 2;

    if bytes.len() < expected_len {
        return Err(BinError::TruncatedFrames {
            declared_count,
            actual_len: bytes.len(),
            expected_len,
        });
    }
    if bytes.len() != expected_len {
        return Err(BinError::LengthMismatch {
            declared_count,
            actual_len: bytes.len(),
            expected_len,
        });
    }

    let mut frame_ids = Vec::with_capacity(declared_count);
    for chunk in bytes[2..].as_chunks::<2>().0 {
        frame_ids.push(u16::from_le_bytes([chunk[0], chunk[1]]));
    }

    Ok(BinSequence {
        frame_ids,
        default_interval: DEFAULT_FRAME_INTERVAL,
        format: BinFormat::V1Explicit,
        bmp_mapped: false,
    })
}

/// Cascading decoder that tries all four BIN format variants in order.
///
/// Returns `(BinSequence, BinFormat)` on success. The priority order is:
/// 1. **v3** (10 bytes, `0 | ref | 9 | count | base`) — BMP-mapped range
/// 2. **v4** (6 bytes, `0 | ref | bmp_id`) — BMP-mapped single frame
/// 3. **v2** (8 bytes, `count | base | 0 | 0`) — sequential range
/// 4. **v1** (variable, `count | ids…`) — explicit frame list
///
/// v3 and v4 are tried before v2/v1 because the zero-prefix discriminator
/// (`w0 == 0`) prevents ambiguity with v1/v2 (which require `w0 > 0`).
///
/// # Errors
/// Returns an error if the bytes do not encode a supported, valid BIN sequence.
#[expect(
    clippy::cast_possible_truncation,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
pub fn parse_advisor_bin_cascade(bytes: &[u8]) -> Result<BinSequence, BinError> {
    let len = bytes.len();

    // Reject stubs that cannot carry useful data.
    if len < 4 {
        return Err(BinError::TooSmall { actual_len: len });
    }

    let w0 = u16::from_le_bytes([bytes[0], bytes[1]]);

    // --- Zero-prefix formats (v3, v4) ---
    if w0 == 0 {
        // v3: 10 bytes = (0, ref_id, 9, count, base_bmp_id)
        if len == 10 {
            let w2 = u16::from_le_bytes([bytes[4], bytes[5]]);
            if w2 == 9 {
                let count = u16::from_le_bytes([bytes[6], bytes[7]]) as usize;
                let base = u16::from_le_bytes([bytes[8], bytes[9]]);
                let frame_ids: Vec<u16> = (0..count).map(|i| base + i as u16).collect();
                return Ok(BinSequence {
                    frame_ids,
                    default_interval: DEFAULT_FRAME_INTERVAL,
                    format: BinFormat::V3BmpRange,
                    bmp_mapped: true,
                });
            }
        }

        // v4: 6 bytes = (0, ref_id, bmp_id)
        if len == 6 {
            let bmp_id = u16::from_le_bytes([bytes[4], bytes[5]]);
            return Ok(BinSequence {
                frame_ids: vec![bmp_id],
                default_interval: DEFAULT_FRAME_INTERVAL,
                format: BinFormat::V4BmpSingle,
                bmp_mapped: true,
            });
        }

        // 4-byte zero-prefix stubs (0, some_id) — too small for useful animation.
        return Err(BinError::TooSmall { actual_len: len });
    }

    // --- Nonzero-prefix formats (v2, v1) ---
    let count = w0 as usize;

    // v2: exactly 8 bytes = (count, base, 0, 0) where count > 0.
    // Discriminator: the last two u16 words are both zero.
    if len == 8 && count > 0 {
        let w2 = u16::from_le_bytes([bytes[4], bytes[5]]);
        let w3 = u16::from_le_bytes([bytes[6], bytes[7]]);
        if w2 == 0 && w3 == 0 {
            let base = u16::from_le_bytes([bytes[2], bytes[3]]);
            let frame_ids: Vec<u16> = (0..count).map(|i| base + i as u16).collect();
            return Ok(BinSequence {
                frame_ids,
                default_interval: DEFAULT_FRAME_INTERVAL,
                format: BinFormat::V2Range,
                bmp_mapped: false,
            });
        }
    }

    // v1: variable length = 2 + count * 2, explicit frame ID list.
    let expected_len = 2 + count * 2;
    if len == expected_len && count > 0 {
        let mut frame_ids = Vec::with_capacity(count);
        for chunk in bytes[2..].as_chunks::<2>().0 {
            frame_ids.push(u16::from_le_bytes([chunk[0], chunk[1]]));
        }
        return Ok(BinSequence {
            frame_ids,
            default_interval: DEFAULT_FRAME_INTERVAL,
            format: BinFormat::V1Explicit,
            bmp_mapped: false,
        });
    }

    Err(BinError::NoFormatMatch { actual_len: len })
}

// ---------------------------------------------------------------------------
// AdvisorState
// ---------------------------------------------------------------------------

/// Mutable state for the droid advisor system.
pub struct AdvisorState {
    /// Which faction's droid set is active.
    pub faction: AdvisorFaction,
    /// Whether the advisor window is visible.
    pub visible: bool,
    /// Queued messages waiting to display.
    queue: VecDeque<AdvisorMessage>,
    /// Currently displaying message (if any).
    current_message: Option<AdvisorMessage>,
    /// Time remaining on current message (seconds).
    message_timer: f32,

    // Animation state
    /// Current primary frame index.
    primary_frame: usize,
    /// Current secondary frame index (R2-D2).
    secondary_frame: usize,
    /// Frame advance timer (seconds since last frame change).
    frame_timer: f32,
    /// Seconds per animation frame.
    frame_interval: f32,
    /// Parsed authored BIN sequences, sorted by filename.
    bin_sequences: Vec<BinSequence>,
    /// Current active BIN sequence.
    current_sequence: usize,
    /// Current cursor within the active BIN sequence.
    frame_cursor: usize,
    /// Number of frames in the primary BMP pool.
    primary_frame_pool_len: usize,
    /// Number of frames in the secondary BMP pool.
    secondary_frame_pool_len: usize,
    /// Maps BMP resource ID → index in `primary_textures` for direct lookup.
    /// Populated from BMP filenames (e.g. `02001-alsprite.bmp` → resource ID 2001).
    /// Used by v3/v4 BIN sequences whose `bmp_mapped` flag is true.
    bmp_resource_id_map: HashMap<u16, usize>,

    // Loaded textures (lazy-initialized)
    frames_loaded: bool,
    primary_textures: Vec<TextureHandle>,
    secondary_textures: Vec<TextureHandle>,

    /// Path to the staged UI root containing `alsprite-dll` and `emsprite-dll`.
    sprite_dir: PathBuf,
}

impl AdvisorState {
    /// Create a new advisor state.
    #[must_use]
    pub fn new(faction: AdvisorFaction) -> Self {
        Self {
            faction,
            visible: true,
            queue: VecDeque::new(),
            current_message: None,
            message_timer: 0.0,
            primary_frame: 0,
            secondary_frame: 0,
            frame_timer: 0.0,
            frame_interval: DEFAULT_FRAME_INTERVAL,
            bin_sequences: Vec::new(),
            current_sequence: 0,
            frame_cursor: 0,
            primary_frame_pool_len: 0,
            secondary_frame_pool_len: 0,
            bmp_resource_id_map: HashMap::new(),
            frames_loaded: false,
            primary_textures: Vec::new(),
            secondary_textures: Vec::new(),
            // WASM resolves this logical root through the installed runtime
            // cache. Native builds replace it with the staged UI directory.
            sprite_dir: PathBuf::new(),
        }
    }

    /// Set the staged UI root used for native advisor assets.
    pub fn set_sprite_dir(&mut self, path: impl Into<PathBuf>) {
        self.sprite_dir = path.into();
        self.primary_textures.clear();
        self.secondary_textures.clear();
        self.bin_sequences.clear();
        self.bmp_resource_id_map.clear();
        self.primary_frame_pool_len = 0;
        self.secondary_frame_pool_len = 0;
        self.current_sequence = 0;
        self.frame_cursor = 0;
        self.primary_frame = 0;
        self.secondary_frame = 0;
        self.frame_timer = 0.0;
        self.frames_loaded = false; // force reload on next draw
    }

    pub fn set_faction(&mut self, faction: AdvisorFaction) {
        if self.faction == faction {
            return;
        }
        self.faction = faction;
        self.primary_textures.clear();
        self.secondary_textures.clear();
        self.bin_sequences.clear();
        self.bmp_resource_id_map.clear();
        self.primary_frame_pool_len = 0;
        self.secondary_frame_pool_len = 0;
        self.primary_frame = 0;
        self.secondary_frame = 0;
        self.current_sequence = 0;
        self.frame_cursor = 0;
        self.frame_timer = 0.0;
        self.frames_loaded = false;
    }

    /// Push a message into the advisor queue.
    ///
    /// If the new message has higher priority than the current one, it
    /// preempts immediately.
    pub fn push_message(&mut self, msg: AdvisorMessage) {
        // If nothing is showing, display immediately.
        if self.current_message.is_none() {
            self.activate_sequence_for_priority(msg.priority);
            self.message_timer = msg.display_time;
            self.current_message = Some(msg);
            self.visible = true;
            return;
        }

        // Preempt if higher priority.
        if let Some(ref current) = self.current_message {
            if msg.priority > current.priority {
                // Demote current back to front of queue.
                if let Some(demoted) = self.current_message.take() {
                    self.queue.push_front(demoted);
                }
                self.activate_sequence_for_priority(msg.priority);
                self.message_timer = msg.display_time;
                self.current_message = Some(msg);
                return;
            }
        }

        // Otherwise queue it.
        self.queue.push_back(msg);
    }

    /// Advance animation and message timers by `dt` seconds.
    pub fn update(&mut self, dt: f32) {
        self.update_animation(dt);

        // Advance message timer.
        if self.current_message.is_some() {
            self.message_timer -= dt;
            if self.message_timer <= 0.0 {
                self.current_message = None;
                // Pop next from queue.
                if let Some(next) = self.queue.pop_front() {
                    self.activate_sequence_for_priority(next.priority);
                    self.message_timer = next.display_time;
                    self.current_message = Some(next);
                } else {
                    self.activate_idle_sequence();
                }
            }
        }
    }

    /// Whether the advisor has a message to show.
    #[must_use]
    pub fn has_message(&self) -> bool {
        self.current_message.is_some()
    }

    fn update_animation(&mut self, dt: f32) {
        self.frame_timer += dt;

        if self.bin_sequences.is_empty() {
            while self.frame_timer >= self.frame_interval {
                self.frame_timer -= self.frame_interval;
                self.advance_legacy_frames();
            }
            return;
        }

        loop {
            let interval = self.current_frame_interval();
            if self.frame_timer < interval {
                break;
            }

            self.frame_timer -= interval;
            self.advance_bin_frames();
        }
    }

    fn advance_legacy_frames(&mut self) {
        if self.primary_frame_pool_len > 0 {
            self.primary_frame = (self.primary_frame + 1) % self.primary_frame_pool_len;
        }
        if self.secondary_frame_pool_len > 0 {
            self.secondary_frame = (self.secondary_frame + 1) % self.secondary_frame_pool_len;
        }
    }

    fn advance_bin_frames(&mut self) {
        let priority = self.active_animation_priority();
        let sequence_len = self
            .bin_sequences
            .get(self.current_sequence)
            .map_or(0, |sequence| sequence.frame_ids.len());

        if sequence_len == 0 {
            if priority == AdvisorPriority::Low {
                self.frame_cursor = 0;
            } else {
                let next = self.next_sequence_for_priority(priority);
                self.set_sequence(next, false);
                return;
            }
        } else if self.frame_cursor + 1 < sequence_len {
            self.frame_cursor += 1;
        } else if priority == AdvisorPriority::Low {
            self.frame_cursor = 0;
        } else {
            let next = self.next_sequence_for_priority(priority);
            self.set_sequence(next, false);
            return;
        }

        self.sync_frames_from_sequence();
    }

    fn active_animation_priority(&self) -> AdvisorPriority {
        self.current_message
            .as_ref()
            .map_or(AdvisorPriority::Low, |message| message.priority)
    }

    fn current_frame_interval(&self) -> f32 {
        self.bin_sequences
            .get(self.current_sequence)
            .map_or(self.frame_interval, |sequence| sequence.default_interval)
    }

    fn activate_idle_sequence(&mut self) {
        if self.bin_sequences.is_empty() {
            return;
        }

        let idle_band = self.sequence_band_for_priority(AdvisorPriority::Low);
        if !idle_band.contains(&self.current_sequence) {
            self.set_sequence(idle_band.start, true);
        }
    }

    fn activate_sequence_for_priority(&mut self, priority: AdvisorPriority) {
        if self.bin_sequences.is_empty() {
            return;
        }

        let next = self.next_sequence_for_priority(priority);
        self.set_sequence(next, true);
    }

    fn next_sequence_for_priority(&self, priority: AdvisorPriority) -> usize {
        let band = self.sequence_band_for_priority(priority);
        if band.contains(&self.current_sequence) {
            let next = self.current_sequence + 1;
            if next < band.end {
                next
            } else {
                band.start
            }
        } else {
            band.start
        }
    }

    fn sequence_band_for_priority(&self, priority: AdvisorPriority) -> Range<usize> {
        let len = self.bin_sequences.len();
        if len == 0 {
            return 0..0;
        }

        // Without the original DLL metadata there is no semantic tag for each
        // BIN, so we bucket the valid sequences by sorted filename into
        // contiguous thirds: early = idle, middle = normal, late = high/critical.
        let idle_end = (len / 3).max(1);
        let normal_end = ((len * 2) / 3).max(idle_end + 1).min(len);

        let band = match priority {
            AdvisorPriority::Low => 0..idle_end,
            AdvisorPriority::Normal => idle_end..normal_end,
            AdvisorPriority::High | AdvisorPriority::Critical => normal_end..len,
        };

        if band.is_empty() {
            0..len
        } else {
            band
        }
    }

    fn set_sequence(&mut self, sequence_index: usize, reset_timer: bool) {
        if self.bin_sequences.is_empty() {
            return;
        }

        self.current_sequence = sequence_index.min(self.bin_sequences.len() - 1);
        self.frame_cursor = 0;
        if reset_timer {
            self.frame_timer = 0.0;
        }
        self.sync_frames_from_sequence();
    }

    fn sync_frames_from_sequence(&mut self) {
        let Some(sequence) = self.bin_sequences.get(self.current_sequence) else {
            return;
        };
        if sequence.frame_ids.is_empty() {
            return;
        }

        let frame_id = sequence.frame_ids[self.frame_cursor.min(sequence.frame_ids.len() - 1)];

        if sequence.bmp_mapped {
            // v3/v4: frame IDs are literal BMP resource IDs — use the direct
            // lookup map built during load. Falls back to modulo if the exact
            // resource ID is not found (which can happen when the BMP set is
            // incomplete or the BIN references frames beyond the extracted set).
            if let Some(&idx) = self.bmp_resource_id_map.get(&frame_id) {
                self.primary_frame = idx;
            } else if self.primary_frame_pool_len > 0 {
                self.primary_frame = frame_id as usize % self.primary_frame_pool_len;
            }
            // v3/v4 sequences do not drive R2-D2 independently; keep secondary
            // on its existing frame.
        } else {
            // v1/v2: frame IDs target an internal DLL resource index.
            // Best-effort modulo into the sorted BMP pools.
            if self.primary_frame_pool_len > 0 {
                self.primary_frame = frame_id as usize % self.primary_frame_pool_len;
            }
            if self.secondary_frame_pool_len > 0 {
                self.secondary_frame = frame_id as usize % self.secondary_frame_pool_len;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Frame loading
// ---------------------------------------------------------------------------

/// Result of loading a faction's sprite directory.
struct FactionFrames {
    primary: Vec<TextureHandle>,
    secondary: Vec<TextureHandle>,
    bin_sequences: Vec<BinSequence>,
    /// Maps BMP resource ID (from filename) → index in `primary`.
    bmp_resource_id_map: HashMap<u16, usize>,
}

#[derive(Debug, Clone, Copy)]
struct AuthoredFrameSpec {
    dll_dir: &'static str,
    primary_anchor: u32,
    primary_first: u32,
    primary_last: u32,
    secondary_anchor: u32,
    secondary_first: u32,
    secondary_last: u32,
}

impl AuthoredFrameSpec {
    fn for_faction(faction: AdvisorFaction) -> Self {
        match faction {
            AdvisorFaction::Alliance => Self {
                dll_dir: "alsprite-dll",
                primary_anchor: 2001,
                primary_first: 2002,
                primary_last: 2024,
                secondary_anchor: 3331,
                secondary_first: 3332,
                secondary_last: 3346,
            },
            AdvisorFaction::Empire => Self {
                dll_dir: "emsprite-dll",
                primary_anchor: 2001,
                primary_first: 2002,
                primary_last: 2016,
                secondary_anchor: 3001,
                secondary_first: 3002,
                secondary_last: 3016,
            },
        }
    }
}

#[derive(Default)]
struct FactionAssetBytes {
    bitmaps: HashMap<u32, Vec<u8>>,
    frames: HashMap<u32, Vec<u8>>,
}

#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
fn load_authored_faction_frames(
    ctx: &egui::Context,
    faction: AdvisorFaction,
    assets: &FactionAssetBytes,
) -> FactionFrames {
    let spec = AuthoredFrameSpec::for_faction(faction);
    let Some(palette_bitmap) = assets.bitmaps.get(&spec.primary_anchor) else {
        macroquad::logging::warn!(
            "[advisor] missing palette anchor source={} resource_id={}",
            spec.dll_dir,
            spec.primary_anchor
        );
        return FactionFrames {
            primary: Vec::new(),
            secondary: Vec::new(),
            bin_sequences: Vec::new(),
            bmp_resource_id_map: HashMap::new(),
        };
    };
    let mut primary_base = match decode_anchor_bitmap(palette_bitmap) {
        Ok(base) => base,
        Err(error) => {
            macroquad::logging::warn!(
                "[advisor] anchor decode failed source={} resource_id={} error={error:?}",
                spec.dll_dir,
                spec.primary_anchor
            );
            return FactionFrames {
                primary: Vec::new(),
                secondary: Vec::new(),
                bin_sequences: Vec::new(),
                bmp_resource_id_map: HashMap::new(),
            };
        }
    };

    let mut primary = Vec::new();
    let mut secondary = Vec::new();
    let mut bmp_resource_id_map = HashMap::new();

    let primary_anchor = match indexed_frame(
        primary_base.width,
        primary_base.height,
        &primary_base.indices,
        &primary_base.palette,
    ) {
        Ok(frame) => frame,
        Err(error) => {
            macroquad::logging::warn!(
                "[advisor] anchor conversion failed source={} resource_id={} error={error:?}",
                spec.dll_dir,
                spec.primary_anchor
            );
            return FactionFrames {
                primary: Vec::new(),
                secondary: Vec::new(),
                bin_sequences: Vec::new(),
                bmp_resource_id_map: HashMap::new(),
            };
        }
    };
    bmp_resource_id_map.insert(spec.primary_anchor as u16, primary.len());
    primary.push(ctx.load_texture(
        format!("advisor_{}_{}", spec.dll_dir, spec.primary_anchor),
        egui::ColorImage::from_rgba_unmultiplied(
            [primary_anchor.width, primary_anchor.height],
            &primary_anchor.rgba,
        ),
        TextureOptions::NEAREST,
    ));
    for resource_id in spec.primary_first..=spec.primary_last {
        let Some(bytes) = assets.frames.get(&resource_id) else {
            macroquad::logging::warn!(
                "[advisor] missing type-302 frame source={} resource_id={resource_id}",
                spec.dll_dir
            );
            break;
        };
        match decode_type302_frame(bytes, &primary_base) {
            Ok(frame) => {
                primary_base.indices.clone_from(&frame.indices);
                let image = egui::ColorImage::from_rgba_unmultiplied(
                    [frame.width, frame.height],
                    &frame.rgba,
                );
                bmp_resource_id_map.insert(resource_id as u16, primary.len());
                primary.push(ctx.load_texture(
                    format!("advisor_{}_{}", spec.dll_dir, resource_id),
                    image,
                    TextureOptions::NEAREST,
                ));
            }
            Err(error) => {
                macroquad::logging::warn!(
                    "[advisor] type-302 decode failed source={} resource_id={} error={error:?}",
                    spec.dll_dir,
                    resource_id
                );
                break;
            }
        }
    }

    let secondary_base = if let Some(bytes) = assets.bitmaps.get(&spec.secondary_anchor) {
        match decode_anchor_bitmap(bytes) {
            Ok(base) => Some(base),
            Err(error) => {
                macroquad::logging::warn!(
                "[advisor] secondary anchor decode failed source={} resource_id={} error={error:?}",
                spec.dll_dir, spec.secondary_anchor
            );
                None
            }
        }
    } else {
        macroquad::logging::warn!(
            "[advisor] missing secondary anchor source={} resource_id={}",
            spec.dll_dir,
            spec.secondary_anchor
        );
        None
    };
    if let Some(mut secondary_base) = secondary_base {
        let secondary_anchor = match indexed_frame(
            secondary_base.width,
            secondary_base.height,
            &secondary_base.indices,
            &secondary_base.palette,
        ) {
            Ok(frame) => frame,
            Err(error) => {
                macroquad::logging::warn!(
                    "[advisor] secondary anchor conversion failed source={} resource_id={} error={error:?}",
                    spec.dll_dir,
                    spec.secondary_anchor
                );
                return FactionFrames {
                    primary,
                    secondary,
                    bin_sequences: Vec::new(),
                    bmp_resource_id_map,
                };
            }
        };
        secondary.push(ctx.load_texture(
            format!("advisor_{}_{}", spec.dll_dir, spec.secondary_anchor),
            egui::ColorImage::from_rgba_unmultiplied(
                [secondary_anchor.width, secondary_anchor.height],
                &secondary_anchor.rgba,
            ),
            TextureOptions::NEAREST,
        ));
        for resource_id in spec.secondary_first..=spec.secondary_last {
            let Some(bytes) = assets.frames.get(&resource_id) else {
                macroquad::logging::warn!(
                    "[advisor] missing secondary type-302 frame source={} resource_id={resource_id}",
                    spec.dll_dir
                );
                break;
            };
            match decode_type302_frame(bytes, &secondary_base) {
                Ok(frame) => {
                    secondary_base.indices.clone_from(&frame.indices);
                    let image = egui::ColorImage::from_rgba_unmultiplied(
                        [frame.width, frame.height],
                        &frame.rgba,
                    );
                    secondary.push(ctx.load_texture(
                        format!("advisor_{}_{}", spec.dll_dir, resource_id),
                        image,
                        TextureOptions::NEAREST,
                    ));
                }
                Err(error) => {
                    macroquad::logging::warn!(
                        "[advisor] secondary type-302 decode failed source={} resource_id={} error={error:?}",
                        spec.dll_dir,
                        resource_id
                    );
                    break;
                }
            }
        }
    }

    let primary_last_loaded = spec.primary_anchor + primary.len().saturating_sub(1) as u32;
    if secondary.is_empty() {
        macroquad::logging::info!(
            "[advisor] loaded authentic source={} primary_resources={}..{} primary_frames={} secondary_anchor={} secondary_frames=0",
            spec.dll_dir,
            spec.primary_anchor,
            primary_last_loaded,
            primary.len(),
            spec.secondary_anchor
        );
    } else {
        let secondary_last_loaded =
            spec.secondary_anchor + secondary.len().saturating_sub(1) as u32;
        macroquad::logging::info!(
            "[advisor] loaded authentic source={} primary_resources={}..{} primary_frames={} secondary_resources={}..{} secondary_frames={}",
            spec.dll_dir,
            spec.primary_anchor,
            primary_last_loaded,
            primary.len(),
            spec.secondary_anchor,
            secondary_last_loaded,
            secondary.len()
        );
    }
    FactionFrames {
        primary,
        secondary,
        bin_sequences: Vec::new(),
        bmp_resource_id_map,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_authored_asset_bytes(asset_root: &Path, faction: AdvisorFaction) -> FactionAssetBytes {
    let spec = AuthoredFrameSpec::for_faction(faction);
    let source = asset_root.join(spec.dll_dir);
    let mut assets = FactionAssetBytes::default();
    for resource_id in [spec.primary_anchor, spec.secondary_anchor] {
        let path = source.join("BMP").join(format!("{resource_id}.bmp"));
        if let Ok(bytes) = std::fs::read(path) {
            assets.bitmaps.insert(resource_id, bytes);
        }
    }
    for resource_id in
        (spec.primary_first..=spec.primary_last).chain(spec.secondary_first..=spec.secondary_last)
    {
        let path = source.join("TYPE302").join(format!("{resource_id}.bin"));
        if let Ok(bytes) = std::fs::read(path) {
            assets.frames.insert(resource_id, bytes);
        }
    }
    assets
}

#[cfg(target_arch = "wasm32")]
fn load_authored_asset_bytes(_asset_root: &Path, faction: AdvisorFaction) -> FactionAssetBytes {
    let spec = AuthoredFrameSpec::for_faction(faction);
    let web = WASM_ADVISOR_ASSETS.lock().unwrap();
    let mut assets = FactionAssetBytes::default();
    for resource_id in [spec.primary_anchor, spec.secondary_anchor] {
        if let Some(bytes) = web
            .bitmaps
            .get(&format!("{}/{}", spec.dll_dir, resource_id))
        {
            assets.bitmaps.insert(resource_id, bytes.clone());
        }
    }
    for resource_id in
        (spec.primary_first..=spec.primary_last).chain(spec.secondary_first..=spec.secondary_last)
    {
        if let Some(bytes) = web.frames.get(&format!("{}/{}", spec.dll_dir, resource_id)) {
            assets.frames.insert(resource_id, bytes.clone());
        }
    }
    assets
}

/// Load BMP frames from a faction's sprite directory.
///
/// Returns primary frames, secondary frames (R2-D2 for Alliance), parsed BIN
/// sequences from the cascading decoder, and a BMP resource ID lookup map.
#[cfg(not(target_arch = "wasm32"))]
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
fn load_legacy_faction_frames(
    ctx: &egui::Context,
    sprite_dir: &Path,
    faction: AdvisorFaction,
) -> FactionFrames {
    let subdir = match faction {
        AdvisorFaction::Alliance => "alliance",
        AdvisorFaction::Empire => "empire",
    };
    let faction_dir = sprite_dir.join(subdir);

    if !faction_dir.exists() {
        return FactionFrames {
            primary: Vec::new(),
            secondary: Vec::new(),
            bin_sequences: Vec::new(),
            bmp_resource_id_map: HashMap::new(),
        };
    }

    // Collect all BMP files, sorted by name (ascending resource ID).
    let mut bmp_files: Vec<PathBuf> = std::fs::read_dir(&faction_dir)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("bmp"))
        })
        .collect();
    bmp_files.sort();

    let mut primary = Vec::new();
    let mut secondary = Vec::new();
    let mut bmp_resource_id_map: HashMap<u16, usize> = HashMap::new();
    let mut bin_files: Vec<PathBuf> = std::fs::read_dir(&faction_dir)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("bin"))
        })
        .collect();
    bin_files.sort();

    for bmp_path in &bmp_files {
        let Ok(bytes) = std::fs::read(bmp_path) else {
            continue;
        };
        let Ok(img) = image::load_from_memory(&bytes) else {
            continue;
        };
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();

        let color_image =
            egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgba.as_raw());

        // Extract resource ID from filename (e.g. "02001-alsprite.bmp" → 2001).
        let resource_id: Option<u16> = bmp_path
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split('-').next())
            .and_then(|s| s.parse::<u16>().ok());

        let idx = primary.len(); // index BEFORE pushing (secondary frames don't count)
        let label = format!("advisor_{subdir}_{idx}");
        let handle = ctx.load_texture(&label, color_image, TextureOptions::default());

        // Alliance: R2-D2 frames are 47×69, C-3PO are 67×116.
        // Empire: all frames are 106×133 (primary only).
        if faction == AdvisorFaction::Alliance && w == 47 && h == 69 {
            secondary.push(handle);
        } else {
            if let Some(rid) = resource_id {
                bmp_resource_id_map.insert(rid, primary.len());
            }
            primary.push(handle);
        }
    }

    // Walk every BIN file through the cascading decoder. Track per-format
    // counts for the load-time summary.
    let total_bins = bin_files.len();
    let mut bin_sequences: Vec<BinSequence> = Vec::new();
    let mut valid_v1 = 0usize;
    let mut valid_v2 = 0usize;
    let mut valid_v3 = 0usize;
    let mut valid_v4 = 0usize;
    let mut empty = 0usize;
    let mut io_failures = 0usize;
    let mut parse_failures = 0usize;
    let mut bmp_mapped_count = 0usize;

    for bin_path in bin_files {
        let Ok(bytes) = std::fs::read(&bin_path) else {
            io_failures += 1;
            continue;
        };
        match parse_advisor_bin_cascade(&bytes) {
            Ok(sequence) if !sequence.frame_ids.is_empty() => {
                match sequence.format {
                    BinFormat::V1Explicit => valid_v1 += 1,
                    BinFormat::V2Range => valid_v2 += 1,
                    BinFormat::V3BmpRange => valid_v3 += 1,
                    BinFormat::V4BmpSingle => valid_v4 += 1,
                }
                if sequence.bmp_mapped {
                    bmp_mapped_count += 1;
                }
                bin_sequences.push(sequence);
            }
            Ok(_) => empty += 1,
            Err(e) => {
                parse_failures += 1;
                // Log first few failures with error details for diagnostic purposes.
                // The ~1% that fail are typically 2-4 byte stubs with no decodable
                // frame data — they fall back to legacy sorted-frame cycling.
                if parse_failures <= 3 {
                    eprintln!(
                        "[advisor] {}: parse failed: {:?} (first {} bytes: {:02x?})",
                        bin_path.file_name().unwrap_or_default().to_string_lossy(),
                        e,
                        bytes.len().min(8),
                        &bytes[..bytes.len().min(8)],
                    );
                }
            }
        }
    }
    let valid_total = valid_v1 + valid_v2 + valid_v3 + valid_v4;
    if let Some(pct) = (100 * valid_total).checked_div(total_bins) {
        if parse_failures > 3 {
            eprintln!(
                "[advisor] ... and {} more parse failures suppressed",
                parse_failures - 3
            );
        }
        eprintln!(
            "[advisor] {subdir} BIN files: {valid_total}/{total_bins} valid ({pct}%) \
             [v1={valid_v1}, v2={valid_v2}, v3={valid_v3}, v4={valid_v4}], \
             {bmp_mapped_count} bmp-mapped, {parse_failures} parse-failed, {empty} empty, {io_failures} io-failed",
        );
    }

    FactionFrames {
        primary,
        secondary,
        bin_sequences,
        bmp_resource_id_map,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_faction_frames(
    ctx: &egui::Context,
    asset_root: &Path,
    faction: AdvisorFaction,
) -> FactionFrames {
    let authored = load_authored_faction_frames(
        ctx,
        faction,
        &load_authored_asset_bytes(asset_root, faction),
    );
    if !authored.primary.is_empty() {
        return authored;
    }

    // Retain the old curated reference path as a development fallback. It is
    // not used by packaged builds and does not satisfy the parity gate.
    load_legacy_faction_frames(ctx, asset_root, faction)
}

#[cfg(target_arch = "wasm32")]
fn load_faction_frames(
    ctx: &egui::Context,
    asset_root: &Path,
    faction: AdvisorFaction,
) -> FactionFrames {
    load_authored_faction_frames(
        ctx,
        faction,
        &load_authored_asset_bytes(asset_root, faction),
    )
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

/// Draw both faction droids directly into the original command-center
/// apertures recovered from `FUN_0042adb0`.
pub fn draw_advisor(ctx: &egui::Context, state: &mut AdvisorState) {
    // Lazy-load frames on first draw (needs egui context for texture registration).
    if !state.frames_loaded {
        let frames = load_faction_frames(ctx, &state.sprite_dir, state.faction);
        state.primary_frame_pool_len = frames.primary.len();
        state.secondary_frame_pool_len = frames.secondary.len();
        state.primary_textures = frames.primary;
        state.secondary_textures = frames.secondary;
        state.bin_sequences = frames.bin_sequences;
        state.bmp_resource_id_map = frames.bmp_resource_id_map;
        if state.bin_sequences.is_empty() {
            state.primary_frame = 0;
            state.secondary_frame = 0;
        } else {
            state.set_sequence(
                state.next_sequence_for_priority(state.active_animation_priority()),
                true,
            );
        }
        state.frames_loaded = true;
    }

    if !state.visible {
        return;
    }

    if state.primary_textures.is_empty() && state.secondary_textures.is_empty() {
        return;
    }

    let screen = ctx.screen_rect();
    let apertures = scaled_advisor_apertures(state.faction, screen.width(), screen.height());
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Middle,
        egui::Id::new("authentic_droid_advisors"),
    ));

    if !state.primary_textures.is_empty() {
        let texture = &state.primary_textures[state.primary_frame % state.primary_textures.len()];
        let (x, y, width, height) = apertures[0];
        painter.image(
            texture.id(),
            egui::Rect::from_min_size(
                egui::pos2(screen.min.x + x, screen.min.y + y),
                egui::vec2(width, height),
            ),
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }

    if !state.secondary_textures.is_empty() {
        let texture =
            &state.secondary_textures[state.secondary_frame % state.secondary_textures.len()];
        let (x, y, width, height) = apertures[1];
        painter.image(
            texture.id(),
            egui::Rect::from_min_size(
                egui::pos2(screen.min.x + x, screen.min.y + y),
                egui::vec2(width, height),
            ),
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}

// ---------------------------------------------------------------------------
// Convenience triggers
// ---------------------------------------------------------------------------

/// Push a game-start greeting.
pub fn advisor_greet(state: &mut AdvisorState) {
    let text = match state.faction {
        AdvisorFaction::Alliance => {
            "Greetings, Commander! I am C-3PO, human-cyborg relations. \
             R2-D2 and I are at your service."
        }
        AdvisorFaction::Empire => {
            "My Lord, your Imperial advisors stand ready. \
             The galaxy awaits your command."
        }
    };
    state.push_message(AdvisorMessage::new(text, AdvisorPriority::Normal));
}

/// Notify the advisor of a mission result.
pub fn advisor_mission_result(state: &mut AdvisorState, mission_name: &str, success: bool) {
    let text = if success {
        match state.faction {
            AdvisorFaction::Alliance => {
                format!("Wonderful news! The {mission_name} mission was a success, sir!")
            }
            AdvisorFaction::Empire => {
                format!("The {mission_name} operation has succeeded, my Lord.")
            }
        }
    } else {
        match state.faction {
            AdvisorFaction::Alliance => {
                format!("Oh dear! I'm afraid the {mission_name} mission has failed.")
            }
            AdvisorFaction::Empire => {
                format!("The {mission_name} operation has failed. Most unfortunate, my Lord.")
            }
        }
    };
    state.push_message(AdvisorMessage::new(text, AdvisorPriority::Normal));
}

/// Notify the advisor of a combat outcome.
pub fn advisor_combat_result(state: &mut AdvisorState, system_name: &str, player_won: bool) {
    let text = if player_won {
        match state.faction {
            AdvisorFaction::Alliance => {
                format!("A great victory at {system_name}! The Force is with us!")
            }
            AdvisorFaction::Empire => {
                format!("Victory at {system_name}, my Lord. The enemy has been crushed.")
            }
        }
    } else {
        match state.faction {
            AdvisorFaction::Alliance => {
                format!("We've suffered a defeat at {system_name}. We must regroup.")
            }
            AdvisorFaction::Empire => format!(
                "Our forces at {system_name} have been repelled. Reinforcements are advised."
            ),
        }
    };
    state.push_message(AdvisorMessage::new(text, AdvisorPriority::High));
}

/// Notify the advisor of an uprising.
pub fn advisor_uprising(state: &mut AdvisorState, system_name: &str, gained: bool) {
    let text = if gained {
        match state.faction {
            AdvisorFaction::Alliance => {
                format!("Excellent! The people of {system_name} have risen up to join us!")
            }
            AdvisorFaction::Empire => {
                format!("The population of {system_name} has been brought to heel, my Lord.")
            }
        }
    } else {
        match state.faction {
            AdvisorFaction::Alliance => format!("Oh no! We've lost control of {system_name}!"),
            AdvisorFaction::Empire => {
                format!("Unacceptable. {system_name} has slipped from Imperial control.")
            }
        }
    };
    state.push_message(AdvisorMessage::new(text, AdvisorPriority::High));
}

/// Notify the advisor of a Death Star event.
pub fn advisor_death_star(state: &mut AdvisorState, event_text: &str) {
    state.push_message(AdvisorMessage::new(event_text, AdvisorPriority::Critical));
}

/// Notify the advisor of manufacturing completion.
pub fn advisor_manufacturing_complete(state: &mut AdvisorState, item_name: &str) {
    let text = match state.faction {
        AdvisorFaction::Alliance => {
            format!("Construction of {item_name} is complete, Commander.")
        }
        AdvisorFaction::Empire => format!("{item_name} construction complete, my Lord."),
    };
    state.push_message(AdvisorMessage::new(text, AdvisorPriority::Low));
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
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
    )]
    fn authored_loader_stops_each_cumulative_run_at_first_gap_or_error() {
        let spec = AuthoredFrameSpec::for_faction(AdvisorFaction::Alliance);
        let mut assets = FactionAssetBytes::default();
        assets
            .bitmaps
            .insert(spec.primary_anchor, indexed_bmp_fixture());
        assets
            .bitmaps
            .insert(spec.secondary_anchor, indexed_bmp_fixture());

        assets.frames.insert(spec.primary_first, type302_fixture());
        assets
            .frames
            .insert(spec.primary_first + 2, type302_fixture());

        assets
            .frames
            .insert(spec.secondary_first, type302_fixture());
        assets.frames.insert(spec.secondary_first + 1, vec![0; 3]);
        assets
            .frames
            .insert(spec.secondary_first + 2, type302_fixture());

        let loaded = load_authored_faction_frames(
            &egui::Context::default(),
            AdvisorFaction::Alliance,
            &assets,
        );

        assert_eq!(loaded.primary.len(), 2, "later frame after gap was loaded");
        assert_eq!(
            loaded.secondary.len(),
            2,
            "later frame after corrupt delta was loaded"
        );
        assert!(!loaded
            .bmp_resource_id_map
            .contains_key(&((spec.primary_first + 2) as u16)));
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
        assert_eq!(
            advisor_apertures(AdvisorFaction::Alliance),
            [(541.0, 337.0, 67.0, 116.0), (316.0, 411.0, 47.0, 69.0)]
        );
        assert_eq!(
            advisor_apertures(AdvisorFaction::Empire),
            [(0.0, 347.0, 107.0, 133.0), (302.0, 401.0, 101.0, 79.0)]
        );
    }

    /// Helper to build a v1-style (non-BMP-mapped) sequence for state tests.
    fn sequence(frame_ids: &[u16], default_interval: f32) -> BinSequence {
        BinSequence {
            frame_ids: frame_ids.to_vec(),
            default_interval,
            format: BinFormat::V1Explicit,
            bmp_mapped: false,
        }
    }

    // -----------------------------------------------------------------------
    // Message queue tests
    // -----------------------------------------------------------------------

    #[test]
    fn advisor_message_queue_fifo() {
        let mut state = AdvisorState::new(AdvisorFaction::Alliance);

        state.push_message(AdvisorMessage::new("First", AdvisorPriority::Normal));
        state.push_message(AdvisorMessage::new("Second", AdvisorPriority::Normal));
        state.push_message(AdvisorMessage::new("Third", AdvisorPriority::Normal));

        assert!(state.has_message());
        assert_eq!(state.current_message.as_ref().unwrap().text, "First");

        // Expire current message.
        state.message_timer = 0.0;
        state.update(0.01);

        assert_eq!(state.current_message.as_ref().unwrap().text, "Second");
    }

    #[test]
    fn high_priority_preempts() {
        let mut state = AdvisorState::new(AdvisorFaction::Empire);

        state.push_message(AdvisorMessage::new("Low", AdvisorPriority::Low));
        assert_eq!(state.current_message.as_ref().unwrap().text, "Low");

        state.push_message(AdvisorMessage::new("Critical", AdvisorPriority::Critical));
        assert_eq!(state.current_message.as_ref().unwrap().text, "Critical");

        // After critical expires, demoted message returns.
        state.message_timer = 0.0;
        state.update(0.01);
        assert_eq!(state.current_message.as_ref().unwrap().text, "Low");
    }

    #[test]
    fn same_priority_does_not_preempt() {
        let mut state = AdvisorState::new(AdvisorFaction::Alliance);

        state.push_message(AdvisorMessage::new("First", AdvisorPriority::Normal));
        state.push_message(AdvisorMessage::new("Second", AdvisorPriority::Normal));

        assert_eq!(state.current_message.as_ref().unwrap().text, "First");
    }

    #[test]
    fn legacy_frame_cycling_fallback_without_bins() {
        let mut state = AdvisorState::new(AdvisorFaction::Alliance);
        state.primary_frame_pool_len = 3;
        state.secondary_frame_pool_len = 2;
        state.frame_interval = 0.1;

        state.update(0.1);
        assert_eq!(state.primary_frame, 1);
        assert_eq!(state.secondary_frame, 1);

        state.update(0.2);
        assert_eq!(state.primary_frame, 0);
        assert_eq!(state.secondary_frame, 1);
    }

    // -----------------------------------------------------------------------
    // v1 parser tests (original parse_advisor_bin)
    // -----------------------------------------------------------------------

    #[test]
    #[expect(
        clippy::float_cmp,
        reason = "These regression checks require exact copied values, endpoints, and pixel coordinates."
    )]
    fn parse_advisor_bin_happy_path() {
        let bytes = [0x03, 0x00, 0x15, 0x05, 0x16, 0x05, 0x17, 0x05];
        let seq = parse_advisor_bin(&bytes).unwrap();

        assert_eq!(seq.frame_ids, vec![1301, 1302, 1303]);
        assert_eq!(seq.default_interval, DEFAULT_FRAME_INTERVAL);
        assert_eq!(seq.format, BinFormat::V1Explicit);
        assert!(!seq.bmp_mapped);
    }

    #[test]
    fn parse_advisor_bin_rejects_truncated_frames() {
        let bytes = [0x02, 0x00, 0x1f, 0x05, 0x20];
        let err = parse_advisor_bin(&bytes).unwrap_err();

        assert_eq!(
            err,
            BinError::TruncatedFrames {
                declared_count: 2,
                actual_len: 5,
                expected_len: 6,
            }
        );
    }

    #[test]
    fn parse_advisor_bin_rejects_length_mismatch() {
        let bytes = [0x00, 0x00, 0x03, 0x0d];
        let err = parse_advisor_bin(&bytes).unwrap_err();

        assert_eq!(
            err,
            BinError::LengthMismatch {
                declared_count: 0,
                actual_len: 4,
                expected_len: 2,
            }
        );
    }

    // -----------------------------------------------------------------------
    // Cascading decoder tests
    // -----------------------------------------------------------------------

    #[test]
    fn cascade_v1_explicit_frame_list() {
        // (count=3, 1301, 1302, 1303) — classic v1 with non-zero trailing words.
        let bytes = [0x03, 0x00, 0x15, 0x05, 0x16, 0x05, 0x17, 0x05];
        let seq = parse_advisor_bin_cascade(&bytes).unwrap();

        assert_eq!(seq.format, BinFormat::V1Explicit);
        assert_eq!(seq.frame_ids, vec![1301, 1302, 1303]);
        assert!(!seq.bmp_mapped);
    }

    #[test]
    fn cascade_v2_sequential_range() {
        // (count=4, base=4001, 0, 0) — dominant 8-byte pattern.
        let bytes: [u8; 8] = [0x04, 0x00, 0xA1, 0x0F, 0x00, 0x00, 0x00, 0x00];
        let seq = parse_advisor_bin_cascade(&bytes).unwrap();

        assert_eq!(seq.format, BinFormat::V2Range);
        assert_eq!(seq.frame_ids, vec![4001, 4002, 4003, 4004]);
        assert!(!seq.bmp_mapped);
    }

    #[test]
    fn cascade_v2_distinguishes_from_v1_with_trailing_zeros() {
        // 8 bytes: (3, 951, 0, 0) — v1 would say [951, 0, 0] but v2 says [951, 952, 953].
        // The cascading decoder should pick v2 because w2==0 && w3==0.
        let bytes: [u8; 8] = [0x03, 0x00, 0xB7, 0x03, 0x00, 0x00, 0x00, 0x00];
        let seq = parse_advisor_bin_cascade(&bytes).unwrap();

        assert_eq!(seq.format, BinFormat::V2Range);
        assert_eq!(seq.frame_ids, vec![951, 952, 953]);
        assert!(!seq.bmp_mapped);
    }

    #[test]
    fn cascade_v3_bmp_range() {
        // (0, ref=1291, 9, count=16, base=19610) — 10-byte BMP-mapped range.
        let bytes: [u8; 10] = [
            0x00, 0x00, // w0 = 0
            0x0B, 0x05, // w1 = 1291 (ref_id)
            0x09, 0x00, // w2 = 9
            0x10, 0x00, // w3 = 16 (count)
            0x9A, 0x4C, // w4 = 19610 (base)
        ];
        let seq = parse_advisor_bin_cascade(&bytes).unwrap();

        assert_eq!(seq.format, BinFormat::V3BmpRange);
        assert!(seq.bmp_mapped);
        assert_eq!(seq.frame_ids.len(), 16);
        assert_eq!(seq.frame_ids[0], 19610);
        assert_eq!(seq.frame_ids[15], 19625);
    }

    #[test]
    fn cascade_v4_bmp_single() {
        // (0, ref=2001, bmp_id=10501) — 6-byte single-frame BMP reference.
        let bytes: [u8; 6] = [
            0x00, 0x00, // w0 = 0
            0xD1, 0x07, // w1 = 2001 (ref_id)
            0x05, 0x29, // w2 = 10501 (bmp_id)
        ];
        let seq = parse_advisor_bin_cascade(&bytes).unwrap();

        assert_eq!(seq.format, BinFormat::V4BmpSingle);
        assert!(seq.bmp_mapped);
        assert_eq!(seq.frame_ids, vec![10501]);
    }

    #[test]
    fn cascade_rejects_2_byte_stubs() {
        let bytes = [0x03, 0x0D];
        let err = parse_advisor_bin_cascade(&bytes).unwrap_err();
        assert_eq!(err, BinError::TooSmall { actual_len: 2 });
    }

    #[test]
    fn cascade_rejects_4_byte_zero_prefix_stubs() {
        // (0, 3331) — 4-byte zero-prefix stub, too small for v3/v4.
        let bytes: [u8; 4] = [0x00, 0x00, 0x03, 0x0D];
        let err = parse_advisor_bin_cascade(&bytes).unwrap_err();
        assert_eq!(err, BinError::TooSmall { actual_len: 4 });
    }

    #[test]
    fn cascade_v2_with_large_count() {
        // (count=20, base=4401, 0, 0) — the 30-file cluster.
        let bytes: [u8; 8] = [0x14, 0x00, 0x31, 0x11, 0x00, 0x00, 0x00, 0x00];
        let seq = parse_advisor_bin_cascade(&bytes).unwrap();

        assert_eq!(seq.format, BinFormat::V2Range);
        assert_eq!(seq.frame_ids.len(), 20);
        assert_eq!(seq.frame_ids[0], 4401);
        assert_eq!(seq.frame_ids[19], 4420);
    }

    #[test]
    fn cascade_v1_single_frame() {
        // (count=1, frame=1306) — 4-byte v1 with count=1.
        let bytes: [u8; 4] = [0x01, 0x00, 0x1A, 0x05];
        let seq = parse_advisor_bin_cascade(&bytes).unwrap();

        assert_eq!(seq.format, BinFormat::V1Explicit);
        assert_eq!(seq.frame_ids, vec![1306]);
        assert!(!seq.bmp_mapped);
    }

    // -----------------------------------------------------------------------
    // BMP-mapped frame sync tests
    // -----------------------------------------------------------------------

    #[test]
    fn bmp_mapped_sequence_uses_direct_lookup() {
        let mut state = AdvisorState::new(AdvisorFaction::Alliance);
        state.primary_frame_pool_len = 10;
        state.secondary_frame_pool_len = 0;

        // Simulate a BMP resource ID map: 2001->0, 2002->1, 2003->2.
        state.bmp_resource_id_map.insert(2001, 0);
        state.bmp_resource_id_map.insert(2002, 1);
        state.bmp_resource_id_map.insert(2003, 2);

        state.bin_sequences = vec![BinSequence {
            frame_ids: vec![2001, 2002, 2003],
            default_interval: 0.1,
            format: BinFormat::V3BmpRange,
            bmp_mapped: true,
        }];
        state.set_sequence(0, true);

        // Initial frame should resolve to index 0 (resource 2001).
        assert_eq!(state.primary_frame, 0);

        // Advance one frame -> resource 2002 -> index 1.
        state.update(0.1);
        assert_eq!(state.primary_frame, 1);

        // Advance again -> resource 2003 -> index 2.
        state.update(0.1);
        assert_eq!(state.primary_frame, 2);
    }

    #[test]
    fn bmp_mapped_falls_back_to_modulo_on_missing_id() {
        let mut state = AdvisorState::new(AdvisorFaction::Empire);
        state.primary_frame_pool_len = 5;
        state.secondary_frame_pool_len = 0;
        // Map is empty — no BMP resource IDs registered.

        state.bin_sequences = vec![BinSequence {
            frame_ids: vec![9999],
            default_interval: 0.1,
            format: BinFormat::V3BmpRange,
            bmp_mapped: true,
        }];
        state.set_sequence(0, true);

        // 9999 not in map -> falls back to 9999 % 5 = 4.
        assert_eq!(state.primary_frame, 4);
    }

    // -----------------------------------------------------------------------
    // BIN state animation tests
    // -----------------------------------------------------------------------

    #[test]
    fn bin_state_advances_and_wraps_for_normal_priority() {
        let mut state = AdvisorState::new(AdvisorFaction::Alliance);
        state.primary_frame_pool_len = 4;
        state.secondary_frame_pool_len = 2;
        state.bin_sequences = vec![
            sequence(&[1], 0.1),
            sequence(&[2], 0.1),
            sequence(&[10, 11], 0.1),
            sequence(&[20, 21], 0.1),
            sequence(&[30], 0.1),
            sequence(&[40], 0.1),
        ];
        state.current_message = Some(AdvisorMessage::new("Normal", AdvisorPriority::Normal));
        state.message_timer = 5.0;
        state.set_sequence(2, true);

        assert_eq!(state.primary_frame, 2);
        assert_eq!(state.secondary_frame, 0);

        state.update(0.1);
        assert_eq!(state.current_sequence, 2);
        assert_eq!(state.frame_cursor, 1);
        assert_eq!(state.primary_frame, 3);
        assert_eq!(state.secondary_frame, 1);

        state.update(0.1);
        assert_eq!(state.current_sequence, 3);
        assert_eq!(state.frame_cursor, 0);
        assert_eq!(state.primary_frame, 0);
        assert_eq!(state.secondary_frame, 0);

        state.update(0.2);
        assert_eq!(state.current_sequence, 2);
        assert_eq!(state.frame_cursor, 0);
        assert_eq!(state.primary_frame, 2);
        assert_eq!(state.secondary_frame, 0);
    }

    #[test]
    fn idle_sequence_loops_in_place() {
        let mut state = AdvisorState::new(AdvisorFaction::Empire);
        state.primary_frame_pool_len = 5;
        state.bin_sequences = vec![
            sequence(&[5, 6], 0.1),
            sequence(&[10, 11], 0.1),
            sequence(&[20, 21], 0.1),
        ];
        state.set_sequence(0, true);

        state.update(0.1);
        assert_eq!(state.current_sequence, 0);
        assert_eq!(state.frame_cursor, 1);
        assert_eq!(state.primary_frame, 1);

        state.update(0.1);
        assert_eq!(state.current_sequence, 0);
        assert_eq!(state.frame_cursor, 0);
        assert_eq!(state.primary_frame, 0);
    }

    // -----------------------------------------------------------------------
    // Convenience trigger tests
    // -----------------------------------------------------------------------

    #[test]
    fn advisor_greet_alliance() {
        let mut state = AdvisorState::new(AdvisorFaction::Alliance);
        advisor_greet(&mut state);
        assert!(state.has_message());
        assert!(state
            .current_message
            .as_ref()
            .unwrap()
            .text
            .contains("C-3PO"));
    }

    #[test]
    fn advisor_greet_empire() {
        let mut state = AdvisorState::new(AdvisorFaction::Empire);
        advisor_greet(&mut state);
        assert!(state.has_message());
        assert!(state
            .current_message
            .as_ref()
            .unwrap()
            .text
            .contains("Imperial"));
    }

    #[test]
    fn faction_from_cockpit() {
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
