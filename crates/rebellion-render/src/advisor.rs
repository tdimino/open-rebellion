//! Droid Advisor System — animated faction advisors that react to game events.
//!
//! Alliance: C-3PO (67×116) + R2-D2 (47×69) animated sprites from ALSPRITE.DLL.
//! Empire: Imperial protocol droid (106×133) from EMSPRITE.DLL.
//!
//! Animation frames are BMP files extracted from the DLLs and staged under
//! `assets/references/ref-ui/07-droid-advisors/{alliance,empire}/`. The original
//! game also ships ~1,400 BIN control files with frame ordering data; we now
//! parse those BIN sequences and let them drive frame order/sequence length.
//!
//! # BIN format variants (cascading decoder)
//!
//! Hex analysis of the ~1,500 BIN files reveals four distinct binary formats.
//! The cascading decoder (`parse_advisor_bin_cascade`) tries each in priority
//! order and returns the first successful parse:
//!
//! | Format | Layout (LE u16s)                       | Size  | Pct  | Description |
//! |--------|----------------------------------------|-------|------|-------------|
//! | v1     | `count \| id[0] \| … \| id[count-1]`  | 2+2N  | ~12% | Explicit frame ID list (original simple format) |
//! | v2     | `count \| base \| 0 \| 0`              | 8     | ~45% | Sequential range: `[base..base+count)` |
//! | v3     | `0 \| ref_id \| 9 \| count \| base`    | 10    | ~38% | Referenced sequential range with BMP resource base |
//! | v4     | `0 \| ref_id \| bmp_id`                | 6     | ~4%  | Single-frame BMP reference |
//!
//! v1/v2 frame IDs target an internal DLL resource index (1301-range) and still
//! use modulo mapping over the sorted BMP pool. v3/v4 `base`/`bmp_id` values
//! are literal BMP resource IDs (2001+ range) and can be looked up directly via
//! the `bmp_resource_ids` map when available.
//!
//! Remaining ~1% of files (2-byte stubs, 4-byte zero-prefix stubs) are too
//! small to carry useful animation data and are silently skipped.
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

use egui_macroquad::egui::{self, TextureHandle};

#[cfg(not(target_arch = "wasm32"))]
use egui_macroquad::egui::TextureOptions;

use crate::cockpit::CockpitFaction;

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
    for chunk in bytes[2..].chunks_exact(2) {
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
        for chunk in bytes[2..].chunks_exact(2) {
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

    /// Path to the advisor sprite directory.
    sprite_dir: Option<PathBuf>,
}

impl AdvisorState {
    /// Create a new advisor state.
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
            sprite_dir: None,
        }
    }

    /// Set the root path to `assets/references/ref-ui/07-droid-advisors/`.
    pub fn set_sprite_dir(&mut self, path: impl Into<PathBuf>) {
        self.sprite_dir = Some(path.into());
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
                let demoted = self.current_message.take().unwrap();
                self.queue.push_front(demoted);
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
// Frame loading (native only)
// ---------------------------------------------------------------------------

/// Result of loading a faction's sprite directory.
struct FactionFrames {
    primary: Vec<TextureHandle>,
    secondary: Vec<TextureHandle>,
    bin_sequences: Vec<BinSequence>,
    /// Maps BMP resource ID (from filename) → index in `primary`.
    bmp_resource_id_map: HashMap<u16, usize>,
}

/// Load BMP frames from a faction's sprite directory.
///
/// Returns primary frames, secondary frames (R2-D2 for Alliance), parsed BIN
/// sequences from the cascading decoder, and a BMP resource ID lookup map.
#[cfg(not(target_arch = "wasm32"))]
fn load_faction_frames(
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
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("bmp"))
        })
        .collect();
    bmp_files.sort();

    let mut primary = Vec::new();
    let mut secondary = Vec::new();
    let mut bmp_resource_id_map: HashMap<u16, usize> = HashMap::new();
    let mut bin_files: Vec<PathBuf> = std::fs::read_dir(&faction_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("bin"))
        })
        .collect();
    bin_files.sort();

    for bmp_path in bmp_files.iter() {
        let bytes = match std::fs::read(bmp_path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let img = match image::load_from_memory(&bytes) {
            Ok(i) => i,
            Err(_) => continue,
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
        let label = format!("advisor_{}_{}", subdir, idx);
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
        let bytes = match std::fs::read(&bin_path) {
            Ok(b) => b,
            Err(_) => {
                io_failures += 1;
                continue;
            }
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
    if total_bins > 0 {
        let valid_total = valid_v1 + valid_v2 + valid_v3 + valid_v4;
        let pct = 100 * valid_total / total_bins;
        if parse_failures > 3 {
            eprintln!(
                "[advisor] ... and {} more parse failures suppressed",
                parse_failures - 3
            );
        }
        eprintln!(
            "[advisor] {} BIN files: {}/{} valid ({}%) \
             [v1={}, v2={}, v3={}, v4={}], \
             {} bmp-mapped, {} parse-failed, {} empty, {} io-failed",
            subdir,
            valid_total,
            total_bins,
            pct,
            valid_v1,
            valid_v2,
            valid_v3,
            valid_v4,
            bmp_mapped_count,
            parse_failures,
            empty,
            io_failures,
        );
    }

    FactionFrames {
        primary,
        secondary,
        bin_sequences,
        bmp_resource_id_map,
    }
}

#[cfg(target_arch = "wasm32")]
fn load_faction_frames(
    _ctx: &egui::Context,
    _sprite_dir: &Path,
    _faction: AdvisorFaction,
) -> FactionFrames {
    FactionFrames {
        primary: Vec::new(),
        secondary: Vec::new(),
        bin_sequences: Vec::new(),
        bmp_resource_id_map: HashMap::new(),
    }
}

// ---------------------------------------------------------------------------
// Draw
// ---------------------------------------------------------------------------

/// Draw the droid advisor floating window.
///
/// Call this inside an `egui_macroquad::ui(|ctx| { ... })` block during the
/// Galaxy mode egui pass.  The advisor window anchors to the bottom-right of
/// the screen, above the status bar.
pub fn draw_advisor(ctx: &egui::Context, state: &mut AdvisorState) {
    // Lazy-load frames on first draw (needs egui context for texture registration).
    if !state.frames_loaded {
        if let Some(ref dir) = state.sprite_dir {
            let frames = load_faction_frames(ctx, dir, state.faction);
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
        }
        state.frames_loaded = true;
    }

    if !state.visible {
        return;
    }

    // If no frames loaded and no message, nothing to draw.
    if state.primary_textures.is_empty() && state.current_message.is_none() {
        return;
    }

    // Window sizing — based on faction frame dimensions + message area.
    let (primary_w, primary_h) = if !state.primary_textures.is_empty() {
        let tex = &state.primary_textures[0];
        (tex.size()[0] as f32, tex.size()[1] as f32)
    } else {
        match state.faction {
            AdvisorFaction::Alliance => (67.0, 116.0),
            AdvisorFaction::Empire => (106.0, 133.0),
        }
    };

    let has_r2 = !state.secondary_textures.is_empty();
    let r2_w = if has_r2 { 47.0 } else { 0.0 };

    // Total sprite area width: primary + gap + R2 (if alliance).
    let sprite_area_w = primary_w + if has_r2 { 8.0 + r2_w } else { 0.0 };
    let window_w = sprite_area_w.max(200.0) + 16.0; // padding

    // Anchor bottom-right, above the bottom bar (~90px).
    let screen = ctx.screen_rect();
    let anchor_x = screen.right() - window_w - 12.0;
    let anchor_y = screen.bottom() - primary_h - 100.0;

    egui::Window::new("Advisor")
        .id(egui::Id::new("droid_advisor"))
        .fixed_pos(egui::pos2(anchor_x, anchor_y))
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_black_alpha(180))
                .inner_margin(8.0),
        )
        .show(ctx, |ui| {
            // Sprite row: primary droid (+ R2-D2 if Alliance).
            ui.horizontal(|ui| {
                // Primary droid frame.
                if !state.primary_textures.is_empty() {
                    let idx = state.primary_frame % state.primary_textures.len();
                    let tex = &state.primary_textures[idx];
                    ui.image(egui::load::SizedTexture::new(
                        tex.id(),
                        egui::vec2(primary_w, primary_h),
                    ));
                }

                // R2-D2 (Alliance only) — positioned at bottom of primary area.
                if has_r2 {
                    let r2_h = 69.0;
                    let offset_y = primary_h - r2_h; // align bottoms
                    ui.allocate_space(egui::vec2(0.0, offset_y));
                    let idx = state.secondary_frame % state.secondary_textures.len();
                    let tex = &state.secondary_textures[idx];
                    ui.image(egui::load::SizedTexture::new(
                        tex.id(),
                        egui::vec2(r2_w, r2_h),
                    ));
                }
            });

            // Message text below the sprite.
            if let Some(ref msg) = state.current_message {
                ui.add_space(4.0);
                let text_color = match state.faction {
                    AdvisorFaction::Alliance => egui::Color32::from_rgb(255, 232, 140), // gold
                    AdvisorFaction::Empire => egui::Color32::from_rgb(200, 200, 220), // cool silver
                };
                ui.label(egui::RichText::new(&msg.text).color(text_color).size(12.0));
            }
        });
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
            AdvisorFaction::Alliance => format!(
                "Wonderful news! The {} mission was a success, sir!",
                mission_name
            ),
            AdvisorFaction::Empire => {
                format!("The {} operation has succeeded, my Lord.", mission_name)
            }
        }
    } else {
        match state.faction {
            AdvisorFaction::Alliance => format!(
                "Oh dear! I'm afraid the {} mission has failed.",
                mission_name
            ),
            AdvisorFaction::Empire => format!(
                "The {} operation has failed. Most unfortunate, my Lord.",
                mission_name
            ),
        }
    };
    state.push_message(AdvisorMessage::new(text, AdvisorPriority::Normal));
}

/// Notify the advisor of a combat outcome.
pub fn advisor_combat_result(state: &mut AdvisorState, system_name: &str, player_won: bool) {
    let text = if player_won {
        match state.faction {
            AdvisorFaction::Alliance => {
                format!("A great victory at {}! The Force is with us!", system_name)
            }
            AdvisorFaction::Empire => format!(
                "Victory at {}, my Lord. The enemy has been crushed.",
                system_name
            ),
        }
    } else {
        match state.faction {
            AdvisorFaction::Alliance => format!(
                "We've suffered a defeat at {}. We must regroup.",
                system_name
            ),
            AdvisorFaction::Empire => format!(
                "Our forces at {} have been repelled. Reinforcements are advised.",
                system_name
            ),
        }
    };
    state.push_message(AdvisorMessage::new(text, AdvisorPriority::High));
}

/// Notify the advisor of an uprising.
pub fn advisor_uprising(state: &mut AdvisorState, system_name: &str, gained: bool) {
    let text = if gained {
        match state.faction {
            AdvisorFaction::Alliance => format!(
                "Excellent! The people of {} have risen up to join us!",
                system_name
            ),
            AdvisorFaction::Empire => format!(
                "The population of {} has been brought to heel, my Lord.",
                system_name
            ),
        }
    } else {
        match state.faction {
            AdvisorFaction::Alliance => format!("Oh no! We've lost control of {}!", system_name),
            AdvisorFaction::Empire => format!(
                "Unacceptable. {} has slipped from Imperial control.",
                system_name
            ),
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
            format!("Construction of {} is complete, Commander.", item_name)
        }
        AdvisorFaction::Empire => format!("{} construction complete, my Lord.", item_name),
    };
    state.push_message(AdvisorMessage::new(text, AdvisorPriority::Low));
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

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
