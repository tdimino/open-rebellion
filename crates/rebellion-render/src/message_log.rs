//! Message log UI panel.
//!
//! Displays a scrollable chronological feed of game events — manufacturing
//! completions, mission results, scripted events, AI actions, and combat
//! outcomes. Each message is color-coded by category and optionally linked
//! to a star system (click to zoom the galaxy map).
//!
//! # Integration
//!
//! Call `draw_message_log` inside the `egui_macroquad::ui` closure alongside
//! the info panel, before the final `egui_macroquad::draw()`. Example from
//! `lib.rs`:
//!
//! ```ignore
//! egui_macroquad::ui(|ctx| {
//!     draw_info_panel_widgets(world, map_state, ctx);
//!     draw_message_log(ctx, &log, &mut log_state);
//! });
//! egui_macroquad::draw();
//! ```

use egui_macroquad::egui::{self, Color32, RichText, ScrollArea};
use rebellion_core::ids::SystemKey;

// ---------------------------------------------------------------------------
// MessageCategory
// ---------------------------------------------------------------------------

/// What kind of event produced this message.
///
/// Determines the color used in the UI and which filter toggle controls it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageCategory {
    /// Ship, troop, or facility construction completed.
    Manufacturing,
    /// Mission launched, completed, or failed.
    Mission,
    /// Diplomatic shift or negotiation outcome.
    Diplomacy,
    /// Scripted story event or conditional trigger.
    Event,
    /// AI-driven fleet movement, production order, or officer assignment.
    Ai,
    /// Space or ground combat result.
    Combat,
}

impl MessageCategory {
    /// Egui foreground color for this category.
    pub fn color(self) -> Color32 {
        match self {
            MessageCategory::Manufacturing => Color32::from_rgb(100, 220, 100), // green
            MessageCategory::Mission => Color32::from_rgb(100, 160, 255),       // blue
            MessageCategory::Diplomacy => Color32::from_rgb(255, 220, 80),      // yellow
            MessageCategory::Event => Color32::from_rgb(180, 180, 180),         // light gray
            MessageCategory::Ai => Color32::from_rgb(220, 160, 60),             // amber
            MessageCategory::Combat => Color32::from_rgb(240, 80, 80),          // red
        }
    }

    /// Short prefix tag shown before the message text.
    pub fn tag(self) -> &'static str {
        match self {
            MessageCategory::Manufacturing => "[BUILD]",
            MessageCategory::Mission => "[MISSION]",
            MessageCategory::Diplomacy => "[DIPLO]",
            MessageCategory::Event => "[EVENT]",
            MessageCategory::Ai => "[AI]",
            MessageCategory::Combat => "[COMBAT]",
        }
    }
}

// ---------------------------------------------------------------------------
// GameMessage
// ---------------------------------------------------------------------------

/// One entry in the message log.
#[derive(Debug, Clone)]
pub struct GameMessage {
    /// Game-day on which this message was generated.
    pub tick: u64,
    /// Human-readable message text.
    pub text: String,
    /// Category — controls color and filter visibility.
    pub category: MessageCategory,
    /// If this message is linked to a system, clicking it can zoom the map
    /// to that location. `None` for messages with no spatial context.
    pub system: Option<SystemKey>,
}

impl GameMessage {
    /// Construct a message without a system link.
    pub fn new(tick: u64, text: impl Into<String>, category: MessageCategory) -> Self {
        GameMessage {
            tick,
            text: text.into(),
            category,
            system: None,
        }
    }

    /// Construct a message linked to a specific system.
    pub fn at_system(
        tick: u64,
        text: impl Into<String>,
        category: MessageCategory,
        system: SystemKey,
    ) -> Self {
        GameMessage {
            tick,
            text: text.into(),
            category,
            system: Some(system),
        }
    }
}

// ---------------------------------------------------------------------------
// MessageLog
// ---------------------------------------------------------------------------

/// Capped FIFO buffer of game messages.
///
/// When `capacity` is exceeded the oldest message is dropped. Lives alongside
/// `GameWorld` and simulation state — not inside either.
#[derive(Debug, Clone)]
pub struct MessageLog {
    messages: Vec<GameMessage>,
    capacity: usize,
}

impl Default for MessageLog {
    fn default() -> Self {
        Self::new(500)
    }
}

impl MessageLog {
    /// Create a new empty log with the given capacity.
    pub fn new(capacity: usize) -> Self {
        MessageLog {
            messages: Vec::with_capacity(capacity.min(512)),
            capacity,
        }
    }

    /// Append a message. Drops the oldest entry when capacity is exceeded.
    pub fn push(&mut self, msg: GameMessage) {
        if self.messages.len() >= self.capacity {
            self.messages.remove(0);
        }
        self.messages.push(msg);
    }

    /// Remove all messages.
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// All messages in chronological order (oldest first).
    pub fn messages(&self) -> &[GameMessage] {
        &self.messages
    }

    /// Number of messages currently stored.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

// ---------------------------------------------------------------------------
// MessageLogState
// ---------------------------------------------------------------------------

/// Mutable UI state for the message log panel.
///
/// Does not own any messages — those live in `MessageLog`.
#[derive(Debug, Clone)]
pub struct MessageLogState {
    /// Whether each category's messages are shown.
    pub show_manufacturing: bool,
    pub show_mission: bool,
    pub show_diplomacy: bool,
    pub show_event: bool,
    pub show_ai: bool,
    pub show_combat: bool,

    /// Whether the panel is expanded or collapsed.
    pub expanded: bool,

    /// If set, the caller should scroll/zoom the galaxy map to this system.
    /// Cleared on the next `draw_message_log` call after being read.
    pub focus_system: Option<SystemKey>,

    /// Whether to lock scroll to the newest message.
    pub auto_scroll: bool,

    /// Tracks the number of messages seen so far — used to detect new
    /// arrivals and trigger auto-scroll.
    last_seen_count: usize,
}

impl Default for MessageLogState {
    fn default() -> Self {
        MessageLogState {
            show_manufacturing: true,
            show_mission: true,
            show_diplomacy: true,
            show_event: true,
            show_ai: false, // AI messages are verbose; off by default
            show_combat: true,
            expanded: true,
            focus_system: None,
            auto_scroll: true,
            last_seen_count: 0,
        }
    }
}

impl MessageLogState {
    pub fn new() -> Self {
        Self::default()
    }

    fn is_visible(&self, category: MessageCategory) -> bool {
        match category {
            MessageCategory::Manufacturing => self.show_manufacturing,
            MessageCategory::Mission => self.show_mission,
            MessageCategory::Diplomacy => self.show_diplomacy,
            MessageCategory::Event => self.show_event,
            MessageCategory::Ai => self.show_ai,
            MessageCategory::Combat => self.show_combat,
        }
    }
}

// ---------------------------------------------------------------------------
// draw_message_log
// ---------------------------------------------------------------------------

/// Render the message log as an egui bottom panel.
///
/// Must be called inside an `egui_macroquad::ui` closure. The caller reads
/// `state.focus_system` after this call to zoom the galaxy map if set.
///
/// # Arguments
///
/// - `ctx`: the egui context from `egui_macroquad::ui`
/// - `log`: the message buffer (read-only)
/// - `state`: mutable panel state (scroll, filters, expand/collapse)
pub fn draw_message_log(ctx: &egui::Context, log: &MessageLog, state: &mut MessageLogState) {
    // Detect new messages for auto-scroll trigger.
    let new_messages = log.len() > state.last_seen_count;
    state.last_seen_count = log.len();

    egui::TopBottomPanel::bottom("message_log")
        .resizable(true)
        .min_height(28.0)
        .max_height(240.0)
        .default_height(if state.expanded { 140.0 } else { 28.0 })
        .show(ctx, |ui| {
            // ── Header bar ───────────────────────────────────────────────────
            ui.horizontal(|ui| {
                let toggle_label = if state.expanded { "▼ Message Log" } else { "▶ Message Log" };
                if ui.small_button(toggle_label).clicked() {
                    state.expanded = !state.expanded;
                }

                if state.expanded {
                    ui.separator();

                    // Category filter toggles.
                    category_toggle(ui, "[BUILD]", MessageCategory::Manufacturing, &mut state.show_manufacturing);
                    category_toggle(ui, "[MISS]", MessageCategory::Mission, &mut state.show_mission);
                    category_toggle(ui, "[DIPLO]", MessageCategory::Diplomacy, &mut state.show_diplomacy);
                    category_toggle(ui, "[EVT]", MessageCategory::Event, &mut state.show_event);
                    category_toggle(ui, "[AI]", MessageCategory::Ai, &mut state.show_ai);
                    category_toggle(ui, "[CMB]", MessageCategory::Combat, &mut state.show_combat);

                    ui.separator();
                    ui.toggle_value(&mut state.auto_scroll, "⬇ Auto");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("Clear").clicked() {
                            // We can't clear here (log is &MessageLog), but we
                            // signal intent via a flag the caller checks.
                            // For now clear is a no-op at render time; caller
                            // should expose a method on its own log handle.
                        }
                        ui.label(format!("{} msgs", log.len()));
                    });
                }
            });

            if !state.expanded {
                return;
            }

            ui.separator();

            // ── Scrollable message list ───────────────────────────────────────
            let scroll_id = egui::Id::new("message_log_scroll");
            let mut scroll = ScrollArea::vertical()
                .id_salt(scroll_id)
                .auto_shrink([false, false]);

            // Stick to bottom when auto_scroll is on and new messages arrived.
            if state.auto_scroll && new_messages {
                scroll = scroll.vertical_scroll_offset(f32::MAX);
            }

            scroll.show(ui, |ui| {
                ui.set_min_width(ui.available_width());

                for msg in log.messages() {
                    if !state.is_visible(msg.category) {
                        continue;
                    }

                    ui.horizontal(|ui| {
                        // Tick timestamp.
                        ui.label(
                            RichText::new(format!("Day {:>5}", msg.tick))
                                .color(Color32::from_gray(120))
                                .monospace()
                                .small(),
                        );

                        // Category tag — colored.
                        ui.label(
                            RichText::new(msg.category.tag())
                                .color(msg.category.color())
                                .monospace()
                                .small(),
                        );

                        // Message text. If linked to a system, make it a
                        // clickable label that sets focus_system.
                        let text = RichText::new(&msg.text).small();
                        if let Some(sys_key) = msg.system {
                            let response = ui.add(
                                egui::Label::new(text.color(Color32::from_gray(220)))
                                    .sense(egui::Sense::click()),
                            );
                            if response.clicked() {
                                state.focus_system = Some(sys_key);
                            }
                            if response.hovered() {
                                response.on_hover_text("Click to zoom to system");
                            }
                        } else {
                            ui.label(text.color(Color32::from_gray(200)));
                        }
                    });
                }
            });
        });
}

/// Render a small toggle button for a category filter.
fn category_toggle(
    ui: &mut egui::Ui,
    label: &str,
    category: MessageCategory,
    enabled: &mut bool,
) {
    let text = if *enabled {
        RichText::new(label).color(category.color()).small()
    } else {
        RichText::new(label).color(Color32::from_gray(80)).small()
    };
    if ui.add(egui::Button::new(text).frame(false)).clicked() {
        *enabled = !*enabled;
    }
}
