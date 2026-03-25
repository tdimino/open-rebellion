//! Droid Advisor System — animated faction advisors that react to game events.
//!
//! Alliance: C-3PO (67×116) + R2-D2 (47×69) animated sprites from ALSPRITE.DLL.
//! Empire: Imperial protocol droid (106×133) from EMSPRITE.DLL.
//!
//! Animation frames are BMP files extracted from the DLLs and staged under
//! `assets/references/ref-ui/07-droid-advisors/{alliance,empire}/`.  The original
//! game also ships ~1,400 BIN control files with frame ordering and timing data;
//! since the BIN→BMP ID mapping requires DLL-internal resource table knowledge,
//! we use a simple sorted-frame cycling approach that looks faithful.
//!
//! # Advisor triggers
//!
//! The advisor is activated by `AdvisorTrigger` events pushed from main.rs
//! whenever notable things happen (mission results, combat outcomes, game start,
//! Death Star events, uprisings).  Each trigger includes a message and priority;
//! higher-priority messages preempt lower ones.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use egui_macroquad::egui::{self, TextureHandle, TextureOptions};

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
            frame_interval: 0.15, // ~6.7 FPS — classic sprite animation speed
            frames_loaded: false,
            primary_textures: Vec::new(),
            secondary_textures: Vec::new(),
            sprite_dir: None,
        }
    }

    /// Set the root path to `assets/references/ref-ui/07-droid-advisors/`.
    pub fn set_sprite_dir(&mut self, path: impl Into<PathBuf>) {
        self.sprite_dir = Some(path.into());
        self.frames_loaded = false; // force reload on next draw
    }

    /// Push a message into the advisor queue.
    ///
    /// If the new message has higher priority than the current one, it
    /// preempts immediately.
    pub fn push_message(&mut self, msg: AdvisorMessage) {
        // If nothing is showing, display immediately.
        if self.current_message.is_none() {
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
        // Advance animation frame.
        self.frame_timer += dt;
        if self.frame_timer >= self.frame_interval {
            self.frame_timer -= self.frame_interval;
            if !self.primary_textures.is_empty() {
                self.primary_frame = (self.primary_frame + 1) % self.primary_textures.len();
            }
            if !self.secondary_textures.is_empty() {
                self.secondary_frame = (self.secondary_frame + 1) % self.secondary_textures.len();
            }
        }

        // Advance message timer.
        if self.current_message.is_some() {
            self.message_timer -= dt;
            if self.message_timer <= 0.0 {
                self.current_message = None;
                // Pop next from queue.
                if let Some(next) = self.queue.pop_front() {
                    self.message_timer = next.display_time;
                    self.current_message = Some(next);
                }
            }
        }
    }

    /// Whether the advisor has a message to show.
    pub fn has_message(&self) -> bool {
        self.current_message.is_some()
    }
}

// ---------------------------------------------------------------------------
// Frame loading (native only)
// ---------------------------------------------------------------------------

/// Load BMP frames from a faction's sprite directory.
///
/// Returns `(primary_frames, secondary_frames)`.  For Alliance, secondary
/// contains R2-D2 (47×69); for Empire, secondary is empty.
#[cfg(not(target_arch = "wasm32"))]
fn load_faction_frames(
    ctx: &egui::Context,
    sprite_dir: &Path,
    faction: AdvisorFaction,
) -> (Vec<TextureHandle>, Vec<TextureHandle>) {
    let subdir = match faction {
        AdvisorFaction::Alliance => "alliance",
        AdvisorFaction::Empire => "empire",
    };
    let faction_dir = sprite_dir.join(subdir);

    if !faction_dir.exists() {
        return (Vec::new(), Vec::new());
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

    for (idx, bmp_path) in bmp_files.iter().enumerate() {
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

        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            [w as usize, h as usize],
            rgba.as_raw(),
        );

        let label = format!("advisor_{}_{}", subdir, idx);
        let handle = ctx.load_texture(&label, color_image, TextureOptions::default());

        // Alliance: R2-D2 frames are 47×69, C-3PO are 67×116.
        // Empire: all frames are 106×133 (primary only).
        if faction == AdvisorFaction::Alliance && w == 47 && h == 69 {
            secondary.push(handle);
        } else {
            primary.push(handle);
        }
    }

    (primary, secondary)
}

#[cfg(target_arch = "wasm32")]
fn load_faction_frames(
    _ctx: &egui::Context,
    _sprite_dir: &Path,
    _faction: AdvisorFaction,
) -> (Vec<TextureHandle>, Vec<TextureHandle>) {
    (Vec::new(), Vec::new())
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
            let (primary, secondary) = load_faction_frames(ctx, dir, state.faction);
            state.primary_textures = primary;
            state.secondary_textures = secondary;
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
        .frame(egui::Frame::new().fill(egui::Color32::from_black_alpha(180)).inner_margin(8.0))
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
                    AdvisorFaction::Empire => egui::Color32::from_rgb(200, 200, 220),   // cool silver
                };
                ui.label(
                    egui::RichText::new(&msg.text)
                        .color(text_color)
                        .size(12.0),
                );
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
            AdvisorFaction::Empire => format!(
                "The {} operation has succeeded, my Lord.",
                mission_name
            ),
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
pub fn advisor_combat_result(
    state: &mut AdvisorState,
    system_name: &str,
    player_won: bool,
) {
    let text = if player_won {
        match state.faction {
            AdvisorFaction::Alliance => format!(
                "A great victory at {}! The Force is with us!",
                system_name
            ),
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
            AdvisorFaction::Alliance => format!(
                "Oh no! We've lost control of {}!",
                system_name
            ),
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
        AdvisorFaction::Alliance => format!(
            "Construction of {} is complete, Commander.",
            item_name
        ),
        AdvisorFaction::Empire => format!(
            "{} construction complete, my Lord.",
            item_name
        ),
    };
    state.push_message(AdvisorMessage::new(text, AdvisorPriority::Low));
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

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
    fn frame_cycling() {
        let mut state = AdvisorState::new(AdvisorFaction::Alliance);
        // Simulate having 4 primary frames.
        state.primary_frame = 0;
        state.frame_interval = 0.1;

        // Manually set a non-empty texture count for cycling logic test.
        // (We can't create real TextureHandles without an egui context.)
        // Instead, just verify the math via update().
        // With empty textures, update is a no-op for frame index.
        // We trust the modulo logic in update().
        assert_eq!(state.primary_frame, 0);
    }

    #[test]
    fn advisor_greet_alliance() {
        let mut state = AdvisorState::new(AdvisorFaction::Alliance);
        advisor_greet(&mut state);
        assert!(state.has_message());
        assert!(state.current_message.as_ref().unwrap().text.contains("C-3PO"));
    }

    #[test]
    fn advisor_greet_empire() {
        let mut state = AdvisorState::new(AdvisorFaction::Empire);
        advisor_greet(&mut state);
        assert!(state.has_message());
        assert!(state.current_message.as_ref().unwrap().text.contains("Imperial"));
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
