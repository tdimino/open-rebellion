//! Original Message Index bitmap shell and row interaction.
//!
//! `FUN_0042a240` constructs the window through `FUN_00466350`, while
//! `FUN_004665f0` composes the faction shell and ten message-category controls.
//! `FUN_00468ab0` fills the list, `FUN_00468f20` filters by category, and
//! `FUN_00468fb0` transitions between modes 1 (index) and 2 (single message).

use std::collections::BTreeSet;

use egui_macroquad::egui::{self, Color32};

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::message_log::{GameMessage, MessageId, MessageLog, MessageRail};

// ---------------------------------------------------------------------------
// Shell constants (unchanged)
// ---------------------------------------------------------------------------

/// Native Message Index resource surface.
pub const MESSAGE_INDEX_WIDTH: f32 = 470.0;
pub const MESSAGE_INDEX_HEIGHT: f32 = 331.0;

const INDEX_CONTENT_X: f32 = 12.0;
const INDEX_CONTENT_Y: f32 = 13.0;
const INDEX_CONTENT_WIDTH: f32 = 400.0;
const INDEX_CONTENT_HEIGHT: f32 = 306.0;
const INDEX_CONTROLS_X: f32 = 22.0;
const INDEX_CONTROLS_Y: f32 = 46.0;

const INDEX_BASE_ALLIANCE: u32 = 10_335;
const INDEX_BASE_EMPIRE: u32 = 10_336;
const INDEX_RAIL_ALLIANCE: u32 = 10_820;
const INDEX_RAIL_EMPIRE: u32 = 10_821;
const INDEX_CONTENT: u32 = 10_822;

// ---------------------------------------------------------------------------
// List geometry (FUN_004665f0, FUN_006082c0 in FUN_00468ab0)
// ---------------------------------------------------------------------------

/// List position within the window, from `FUN_004665f0`: (25, 108).
const LIST_X: f32 = 25.0;
const LIST_Y: f32 = 108.0;
/// List size: 368 by 194 (`0x170 x 0xc2`).
const LIST_WIDTH: f32 = 368.0;
const LIST_HEIGHT: f32 = 194.0;
/// Row dimensions from `FUN_006082c0(_, 0x15a, 0x15)`.
const ROW_WIDTH: f32 = 346.0;
const ROW_HEIGHT: f32 = 21.0;
/// Text offset within a row (`FUN_00468ab0`: `+0x30 = 0x20`).
const TEXT_INSET: f32 = 32.0;
/// Text area for single-message display (`FUN_00469de0`).
const TEXT_AREA_X: f32 = 17.0;
const TEXT_AREA_Y: f32 = 234.0;
/// Default width for non-encyclopedia messages (`0x18b`).
const TEXT_AREA_WIDTH: f32 = 395.0;
const TEXT_AREA_HEIGHT: f32 = 80.0;

/// Visible rows the list shows at once: `194 / 21 = 9`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Nine whole rows of 21 in a list 194 tall."
)]
const VISIBLE_ROWS: usize = (LIST_HEIGHT / ROW_HEIGHT) as usize;

// ---------------------------------------------------------------------------
// Category control specs (unchanged)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MessageCategoryControlSpec {
    command_id: u16,
    x: u16,
    width: u16,
    normal_resource: u32,
    pressed_resource: u32,
}

const fn message_category_control_specs(
    faction: CockpitFaction,
) -> [MessageCategoryControlSpec; 10] {
    let (
        faction_normal,
        faction_pressed,
        fleet_normal,
        fleet_pressed,
        mission_normal,
        mission_pressed,
        advice_normal,
        advice_pressed,
    ) = match faction {
        CockpitFaction::Alliance => (
            10_832, 10_833, 10_840, 10_841, 10_842, 10_843, 10_846, 10_847,
        ),
        CockpitFaction::Empire => (
            10_834, 10_835, 10_838, 10_839, 10_844, 10_845, 10_848, 10_849,
        ),
    };
    [
        MessageCategoryControlSpec {
            command_id: 0x79,
            x: 0,
            width: 36,
            normal_resource: 10_830,
            pressed_resource: 10_831,
        },
        MessageCategoryControlSpec {
            command_id: 0x7a,
            x: 38,
            width: 36,
            normal_resource: faction_normal,
            pressed_resource: faction_pressed,
        },
        MessageCategoryControlSpec {
            command_id: 0x7b,
            x: 76,
            width: 36,
            normal_resource: fleet_normal,
            pressed_resource: fleet_pressed,
        },
        MessageCategoryControlSpec {
            command_id: 0x7c,
            x: 114,
            width: 35,
            normal_resource: mission_normal,
            pressed_resource: mission_pressed,
        },
        MessageCategoryControlSpec {
            command_id: 0x7d,
            x: 151,
            width: 36,
            normal_resource: 10_836,
            pressed_resource: 10_837,
        },
        MessageCategoryControlSpec {
            command_id: 0x7e,
            x: 189,
            width: 36,
            normal_resource: 10_852,
            pressed_resource: 10_853,
        },
        MessageCategoryControlSpec {
            command_id: 0x7f,
            x: 227,
            width: 34,
            normal_resource: 10_856,
            pressed_resource: 10_857,
        },
        MessageCategoryControlSpec {
            command_id: 0x80,
            x: 263,
            width: 36,
            normal_resource: 10_854,
            pressed_resource: 10_855,
        },
        MessageCategoryControlSpec {
            command_id: 0x81,
            x: 301,
            width: 35,
            normal_resource: 10_850,
            pressed_resource: 10_851,
        },
        MessageCategoryControlSpec {
            command_id: 0x82,
            x: 338,
            width: 37,
            normal_resource: advice_normal,
            pressed_resource: advice_pressed,
        },
    ]
}

/// The category a cockpit command opens the window on (`FUN_00422ce0`):
/// F6 (`0x75`) All, the rail `0x136..0x13c` Popular Support through
/// Conflict, `0x13d` Advice and `0x13e` Chat.
#[must_use]
pub fn opening_category(command: u16) -> Option<u16> {
    match command {
        0x75 => Some(0x79),
        0x136..=0x13c => Some(command - 0x136 + 0x7a),
        0x13d => Some(0x82),
        0x13e => Some(0x81),
        _ => None,
    }
}

/// The window's screen rectangle. hyp: `FUN_00466350` places it at the
/// parent's origin with flags 7; the port centers it in the galaxy view, as
/// the Finders.
#[must_use]
pub fn window_rect(layout: CockpitLayout) -> egui::Rect {
    crate::mission_dialog::galaxy_centered_rect(layout, MESSAGE_INDEX_WIDTH, MESSAGE_INDEX_HEIGHT)
}

// ---------------------------------------------------------------------------
// Window buttons (FUN_004665f0)
// ---------------------------------------------------------------------------

/// One of the window's own bitmap buttons, in window coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowButtonSpec {
    pub command_id: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub normal_resource: u32,
    pub pressed_resource: u32,
}

/// Close (`0x28`, `0x2882`/`0x2888`) and Display Message (`0x65`,
/// `0x2884`/`0x288a`) on the right rail, and Delete (`0x91`, `0x2a96`) under
/// the category row, as `FUN_004665f0` builds them. Delete shows in mode 1
/// only (`FUN_00468fb0`). port: Navigate (`0x90`) and the mode-2 scroll,
/// Encyclopedia and Detail buttons are not drawn.
#[must_use]
pub fn window_button_specs(faction: CockpitFaction, mode: IndexMode) -> Vec<WindowButtonSpec> {
    let (x, size, close, display) = match faction {
        CockpitFaction::Alliance => (0x1a7, (0x20, 0x1f), (0x19, 0x2882), (0x5d, 0x2884)),
        CockpitFaction::Empire => (0x1aa, (0x2c, 0x29), (0x15, 0x2888), (0x59, 0x288a)),
    };
    let rail = |command_id, (y, normal): (u16, u32)| WindowButtonSpec {
        command_id,
        x,
        y,
        width: size.0,
        height: size.1,
        normal_resource: normal,
        pressed_resource: normal + 1,
    };
    let mut buttons = vec![rail(0x28, close), rail(0x65, display)];
    if mode == IndexMode::List {
        buttons.push(WindowButtonSpec {
            command_id: 0x91,
            x: 0x154,
            y: 0x57,
            width: 0x38,
            height: 0x14,
            normal_resource: 0x2a96,
            pressed_resource: 0x2a97,
        });
    }
    buttons
}

/// Paint the window's buttons and return the one clicked.
fn draw_window_buttons(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
    mode: IndexMode,
) -> Option<u16> {
    let mut clicked = None;
    egui::Area::new(egui::Id::new("original-message-index-buttons"))
        .fixed_pos(origin)
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let window_rect = egui::Rect::from_min_size(
                origin,
                egui::vec2(MESSAGE_INDEX_WIDTH * scale, MESSAGE_INDEX_HEIGHT * scale),
            );
            let (pointer, primary_down) = ctx.input(|input| {
                (
                    input.pointer.interact_pos(),
                    input.pointer.button_down(egui::PointerButton::Primary),
                )
            });
            for button in window_button_specs(faction, mode) {
                let rect = message_index_rect(
                    window_rect,
                    scale,
                    f32::from(button.x),
                    f32::from(button.y),
                    f32::from(button.width),
                    f32::from(button.height),
                );
                let response = ui.interact(
                    rect,
                    ui.id().with(("message-index-button", button.command_id)),
                    egui::Sense::click(),
                );
                let pressed = primary_down
                    && pointer.is_some_and(|point| message_index_rect_contains(rect, point));
                paint_strategy_resource(
                    ui.painter(),
                    ctx,
                    cache,
                    if pressed {
                        button.pressed_resource
                    } else {
                        button.normal_resource
                    },
                    rect,
                );
                if response.clicked() {
                    clicked = Some(button.command_id);
                }
            }
        });
    clicked
}

// ---------------------------------------------------------------------------
// Shell drawing (preserved)
// ---------------------------------------------------------------------------

/// Paint the recovered Message Index shell and its ten original category
/// controls.
///
/// The command identities are All (`0x79`), Popular Support (`0x7a`), Fleet
/// (`0x7b`), Mission (`0x7c`), Resource (`0x7d`), Manufacturing (`0x7e`),
/// Defense (`0x7f`), Conflict (`0x80`), Chat (`0x81`), and Advice (`0x82`).
pub fn draw_message_index_shell(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
) -> Option<u16> {
    if scale <= 0.0 {
        return None;
    }
    let mut selected = None;
    egui::Area::new(egui::Id::new("original-message-index-shell"))
        .fixed_pos(origin)
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let size = egui::vec2(MESSAGE_INDEX_WIDTH * scale, MESSAGE_INDEX_HEIGHT * scale);
            let (window_rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
            let (base, rail) = match faction {
                CockpitFaction::Alliance => (INDEX_BASE_ALLIANCE, INDEX_RAIL_ALLIANCE),
                CockpitFaction::Empire => (INDEX_BASE_EMPIRE, INDEX_RAIL_EMPIRE),
            };
            paint_strategy_resource(ui.painter(), ctx, cache, base, window_rect);
            paint_strategy_resource(
                ui.painter(),
                ctx,
                cache,
                rail,
                message_index_rect(window_rect, scale, 412.0, 0.0, 58.0, 330.0),
            );
            paint_strategy_resource(
                ui.painter(),
                ctx,
                cache,
                INDEX_CONTENT,
                message_index_rect(
                    window_rect,
                    scale,
                    INDEX_CONTENT_X,
                    INDEX_CONTENT_Y,
                    INDEX_CONTENT_WIDTH,
                    INDEX_CONTENT_HEIGHT,
                ),
            );

            let (pointer, primary_down) = ctx.input(|input| {
                (
                    input.pointer.interact_pos(),
                    input.pointer.button_down(egui::PointerButton::Primary),
                )
            });
            for control in message_category_control_specs(faction) {
                let rect = message_index_rect(
                    window_rect,
                    scale,
                    INDEX_CONTROLS_X + f32::from(control.x),
                    INDEX_CONTROLS_Y,
                    f32::from(control.width),
                    41.0,
                );
                let response = ui.interact(
                    rect,
                    ui.id().with(("message-index-category", control.command_id)),
                    egui::Sense::click(),
                );
                let pressed = primary_down
                    && pointer.is_some_and(|point| message_index_rect_contains(rect, point));
                paint_strategy_resource(
                    ui.painter(),
                    ctx,
                    cache,
                    if pressed {
                        control.pressed_resource
                    } else {
                        control.normal_resource
                    },
                    rect,
                );
                if response.clicked()
                    && response
                        .interact_pointer_pos()
                        .is_some_and(|point| message_index_rect_contains(rect, point))
                {
                    selected = Some(control.command_id);
                }
            }
        });
    selected
}

// ---------------------------------------------------------------------------
// Geometry helpers
// ---------------------------------------------------------------------------

fn message_index_rect(
    parent: egui::Rect,
    scale: f32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(parent.min.x + x * scale, parent.min.y + y * scale),
        egui::vec2(width * scale, height * scale),
    )
}

fn message_index_rect_contains(rect: egui::Rect, point: egui::Pos2) -> bool {
    point.x >= rect.min.x && point.x < rect.max.x && point.y >= rect.min.y && point.y < rect.max.y
}

fn paint_strategy_resource(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    resource_id: u32,
    rect: egui::Rect,
) {
    let Some(texture) = cache.get(ctx, DllSource::Strategy, resource_id) else {
        return;
    };
    painter.image(
        texture.id(),
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
}

// ---------------------------------------------------------------------------
// IndexMode, Actions, State
// ---------------------------------------------------------------------------

/// Display mode of the Message Index window (`FUN_00468fb0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexMode {
    /// Mode 1: the scrollable message list.
    List,
    /// Mode 2: viewing a single message's text.
    SingleMessage,
}

/// What the Message Index window asks of the app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageIndexAction {
    /// Entering the Advice category: drop game speed to Very Slow.
    /// `FUN_004697b0` case `0x82` -> `FUN_0041d3f0` -> `FUN_00487ff0(1)`.
    AdviceShown,
    /// Leaving the Advice category or closing the window: restore speed.
    /// `FUN_004697b0` other cases -> `FUN_0041d410` -> `FUN_00487ff0(0)`.
    AdviceHidden,
    /// Closing the window: mark messages in this category read.
    /// `None` for All Messages (mark everything read); `Some(rail)` for one.
    /// `FUN_0048a530`.
    MarkCategoryRead(Option<MessageRail>),
    /// Delete the selected messages from the log.
    /// `FUN_005f54a0` per selected message.
    DeleteSelected(Vec<MessageId>),
    /// A single message was opened for display (mode 2).
    /// The app should mark it read (`FUN_00469de0`: `+0x38 = 10`).
    MessageDisplayed(MessageId),
    /// The Close button (`0x28`): the app applies
    /// [`MessageIndexState::close`] and drops the window.
    Close,
}

/// Mutable state for the Message Index window.
#[derive(Debug, Clone)]
pub struct MessageIndexState {
    /// Current display mode.
    mode: IndexMode,
    /// The active category command (`0x79..0x82`), default All (`0x79`).
    category: u16,
    /// Selected message ids.
    selected: BTreeSet<MessageId>,
    /// Anchor row index for shift-click range selection (manual pp. 78-80).
    anchor: Option<usize>,
    /// First visible row (scroll position).
    scroll_offset: usize,
    /// Message being viewed in mode 2.
    viewing: Option<MessageId>,
    /// Last clicked row and when, for double-click detection.
    /// port: egui counts double clicks across widgets; the list keeps its own
    /// (same approach as `fleet_finder.rs`).
    last_clicked: Option<(usize, f64)>,
}

impl Default for MessageIndexState {
    fn default() -> Self {
        Self {
            mode: IndexMode::List,
            category: 0x79,
            selected: BTreeSet::new(),
            anchor: None,
            scroll_offset: 0,
            viewing: None,
            last_clicked: None,
        }
    }
}

impl MessageIndexState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The active category command.
    #[must_use]
    pub fn category(&self) -> u16 {
        self.category
    }

    /// Current display mode.
    #[must_use]
    pub fn mode(&self) -> IndexMode {
        self.mode
    }

    /// The [`MessageRail`] for the active category, or `None` for All.
    #[must_use]
    pub fn active_rail(&self) -> Option<MessageRail> {
        category_rail(self.category)
    }

    /// The selected message ids.
    #[must_use]
    pub fn selected(&self) -> &BTreeSet<MessageId> {
        &self.selected
    }

    /// Switch the category, returning actions the app must handle.
    ///
    /// `FUN_004697b0`: switching to `0x82` (Advice) drops speed; switching
    /// away restores it.
    pub fn set_category(&mut self, command: u16) -> Vec<MessageIndexAction> {
        if command == self.category {
            return Vec::new();
        }
        let mut actions = Vec::new();
        let was_advice = self.category == 0x82;
        let is_advice = command == 0x82;

        if was_advice && !is_advice {
            actions.push(MessageIndexAction::AdviceHidden);
        }
        if is_advice && !was_advice {
            actions.push(MessageIndexAction::AdviceShown);
        }

        self.category = command;
        self.selected.clear();
        self.anchor = None;
        self.scroll_offset = 0;
        self.last_clicked = None;

        if self.mode != IndexMode::List {
            self.mode = IndexMode::List;
            self.viewing = None;
        }

        actions
    }

    /// Actions the app must handle when the window closes.
    ///
    /// `FUN_0046a6a0`: restore speed (`FUN_0041d410`) and mark the active
    /// category read (`FUN_0048a530`).
    pub fn close(&self) -> Vec<MessageIndexAction> {
        vec![
            MessageIndexAction::AdviceHidden,
            MessageIndexAction::MarkCategoryRead(self.active_rail()),
        ]
    }

    /// Delete the currently selected messages. Returns a
    /// [`MessageIndexAction::DeleteSelected`] the app must apply.
    pub fn delete_selected(&mut self) -> Option<MessageIndexAction> {
        if self.selected.is_empty() {
            return None;
        }
        let ids: Vec<MessageId> = self.selected.iter().copied().collect();
        self.selected.clear();
        self.anchor = None;
        self.last_clicked = None;
        Some(MessageIndexAction::DeleteSelected(ids))
    }

    /// The Display Message button (`0x65`, `FUN_00468fb0`): in mode 1 it
    /// shows the first selected message, and an empty selection stays put;
    /// in mode 2 it returns to the list.
    pub fn toggle_display(&mut self) -> Option<MessageIndexAction> {
        match self.mode {
            IndexMode::List => {
                let id = *self.selected.first()?;
                self.mode = IndexMode::SingleMessage;
                self.viewing = Some(id);
                self.last_clicked = None;
                Some(MessageIndexAction::MessageDisplayed(id))
            }
            IndexMode::SingleMessage => {
                self.return_to_list();
                None
            }
        }
    }

    /// Return to mode 1 (list) from mode 2 (single message).
    pub fn return_to_list(&mut self) {
        self.mode = IndexMode::List;
        self.viewing = None;
        self.last_clicked = None;
    }

    /// Build a report for the interface fixture, mirroring
    /// `FleetFinderReport`.
    #[must_use]
    pub fn report(&self, log: &MessageLog, player_is_alliance: bool) -> MessageIndexReport {
        let rows = filtered_messages(log, self.category, player_is_alliance);
        MessageIndexReport {
            mode: self.mode,
            category: self.category,
            rows: rows
                .iter()
                .map(|msg| MessageIndexRow {
                    id: msg.id,
                    text: msg.text.clone(),
                    rail: msg
                        .rail
                        .expect("filtered_messages only returns rail messages"),
                    unread: msg.unread,
                    tick: msg.tick,
                })
                .collect(),
            selected: self.selected.clone(),
            scroll_offset: self.scroll_offset,
            viewing: self.viewing,
        }
    }
}

// ---------------------------------------------------------------------------
// Report types
// ---------------------------------------------------------------------------

/// One row in a [`MessageIndexReport`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageIndexRow {
    pub id: MessageId,
    pub text: String,
    pub rail: MessageRail,
    pub unread: bool,
    pub tick: u64,
}

/// What the open Message Index shows, for the interface fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageIndexReport {
    pub mode: IndexMode,
    pub category: u16,
    pub rows: Vec<MessageIndexRow>,
    pub selected: BTreeSet<MessageId>,
    pub scroll_offset: usize,
    pub viewing: Option<MessageId>,
}

// ---------------------------------------------------------------------------
// Category helpers
// ---------------------------------------------------------------------------

/// The filter mask for a category command (`FUN_004697b0`).
///
/// All Messages (`0x79`) returns 0; the filter uses a special path for it.
#[must_use]
pub const fn category_filter_mask(command: u16) -> u16 {
    match command {
        0x79 => 0,     // All Messages
        0x7a => 0x001, // Popular Support
        0x7b => 0x080, // Fleet
        0x7c => 0x010, // Mission
        0x7d => 0x004, // Resource
        0x7e => 0x008, // Manufacturing
        0x7f => 0x040, // Defense
        0x80 => 0x100, // Conflict
        0x81 => 0x020, // Chat
        0x82 => 0x200, // Advice
        _ => 0,
    }
}

/// The [`MessageRail`] for a category command. All (`0x79`) has no single rail.
#[must_use]
pub const fn category_rail(command: u16) -> Option<MessageRail> {
    match command {
        0x7a => Some(MessageRail::PopularSupport),
        0x7b => Some(MessageRail::Fleet),
        0x7c => Some(MessageRail::Mission),
        0x7d => Some(MessageRail::Resource),
        0x7e => Some(MessageRail::Manufacturing),
        0x7f => Some(MessageRail::Defense),
        0x80 => Some(MessageRail::Conflict),
        0x81 => Some(MessageRail::Chat),
        0x82 => Some(MessageRail::Advice),
        _ => None,
    }
}

/// Messages visible in the current category for the player's side.
///
/// `FUN_00468f20`: an item is visible when `(filter & mask) != 0`, or when
/// the category is All (`0x79`) and the mask is not Advice (`0x200`).
#[must_use]
pub fn filtered_messages(
    log: &MessageLog,
    category: u16,
    player_is_alliance: bool,
) -> Vec<&GameMessage> {
    let filter_mask = category_filter_mask(category);
    log.messages()
        .iter()
        .filter(|msg| {
            let Some(rail) = msg.rail else {
                return false;
            };
            if !msg.audience.includes(player_is_alliance) {
                return false;
            }
            let msg_mask = rail.mask();
            // FUN_00468f20: visible = (filter & mask != 0) || (cat == 0x79 && mask != 0x200)
            (filter_mask & msg_mask) != 0
                || (category == 0x79 && msg_mask != MessageRail::Advice.mask())
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Drawing: full Message Index (shell + rows)
// ---------------------------------------------------------------------------

/// Draw the Message Index window: the shell, category buttons, and the
/// message list with selection, double-click display, and delete.
///
/// Returns actions the app must handle (speed changes, mark read, delete,
/// message displayed).
#[expect(
    clippy::too_many_arguments,
    reason = "The window needs context, cache, faction, origin, scale, log, state, and side."
)]
pub fn draw_message_index(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
    log: &MessageLog,
    state: &mut MessageIndexState,
    player_is_alliance: bool,
) -> Vec<MessageIndexAction> {
    let mut actions = Vec::new();

    // The window stays above the sector windows, which raise themselves
    // each frame, as the Finders do; its own layers keep their order.
    for id in [
        "original-message-index-shell",
        "message-index-list",
        "message-index-single-text",
        "original-message-index-buttons",
    ] {
        ctx.move_to_top(egui::LayerId::new(egui::Order::Tooltip, egui::Id::new(id)));
    }

    // Paint the shell and handle category clicks.
    if let Some(command) = draw_message_index_shell(ctx, cache, faction, origin, scale) {
        actions.extend(state.set_category(command));
    }

    if scale <= 0.0 {
        return actions;
    }

    let rows = filtered_messages(log, state.category, player_is_alliance);

    // FUN_00468f20: if no visible row is selected, select the first.
    let any_visible_selected = rows.iter().any(|msg| state.selected.contains(&msg.id));
    if !any_visible_selected && !rows.is_empty() && state.mode == IndexMode::List {
        state.selected.clear();
        state.selected.insert(rows[0].id);
        state.anchor = Some(0);
    }

    // Clamp scroll.
    let max_scroll = rows.len().saturating_sub(VISIBLE_ROWS);
    state.scroll_offset = state.scroll_offset.min(max_scroll);

    match state.mode {
        IndexMode::List => {
            let list_actions = draw_list_rows(ctx, cache, faction, origin, scale, &rows, state);
            actions.extend(list_actions);
        }
        IndexMode::SingleMessage => {
            draw_single_message(ctx, origin, scale, &rows, state);
        }
    }

    match draw_window_buttons(ctx, cache, faction, origin, scale, state.mode) {
        Some(0x28) => actions.push(MessageIndexAction::Close),
        Some(0x65) => actions.extend(state.toggle_display()),
        Some(0x91) => actions.extend(state.delete_selected()),
        _ => {}
    }

    actions
}

/// Draw the list rows in mode 1 and handle interaction.
fn draw_list_rows(
    ctx: &egui::Context,
    _cache: &mut BmpCache,
    _faction: CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
    rows: &[&GameMessage],
    state: &mut MessageIndexState,
) -> Vec<MessageIndexAction> {
    let mut actions = Vec::new();
    let mut clicked_row: Option<(usize, f64)> = None;
    let mut display_message: Option<MessageId> = None;
    let mut wheel = 0.0_f32;

    let list_id = egui::Id::new("message-index-list");
    egui::Area::new(list_id)
        .fixed_pos(egui::pos2(
            origin.x + LIST_X * scale,
            origin.y + LIST_Y * scale,
        ))
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let list_size = egui::vec2(LIST_WIDTH * scale, LIST_HEIGHT * scale);
            let (list_rect, _) = ui.allocate_exact_size(list_size, egui::Sense::hover());
            let painter = ui.painter().with_clip_rect(list_rect);

            // hyp: font 13 (unread, bolder) and font 10 (read) are not mapped
            // to specific typefaces; the port uses proportional at two sizes.
            let font_unread = egui::FontId::proportional((11.0 * scale).max(7.0));
            let font_read = egui::FontId::proportional((9.0 * scale).max(6.0));

            for line in 0..VISIBLE_ROWS {
                let row_index = state.scroll_offset + line;
                let Some(msg) = rows.get(row_index) else {
                    break;
                };

                let row_rect = egui::Rect::from_min_size(
                    egui::pos2(
                        list_rect.min.x,
                        list_rect.min.y + ROW_HEIGHT * scale * line as f32,
                    ),
                    egui::vec2(ROW_WIDTH * scale, ROW_HEIGHT * scale),
                );

                let is_selected = state.selected.contains(&msg.id);

                // hyp: selected rows use a highlight colour; the original
                // paints a selected-row background bitmap.
                if is_selected {
                    painter.rect_filled(
                        row_rect,
                        0.0,
                        Color32::from_rgba_premultiplied(60, 60, 120, 180),
                    );
                }

                // Row text.
                let font = if msg.unread {
                    font_unread.clone()
                } else {
                    font_read.clone()
                };
                let text_pos = egui::pos2(row_rect.min.x + TEXT_INSET * scale, row_rect.center().y);
                painter.text(
                    text_pos,
                    egui::Align2::LEFT_CENTER,
                    &msg.text,
                    font,
                    Color32::WHITE,
                );

                // Click interaction.
                let response = ui.interact(
                    row_rect,
                    ui.id().with(("msg-index-row", row_index)),
                    egui::Sense::click(),
                );
                if response.clicked() {
                    let now = ui.input(|input| input.time);
                    let delay = ctx.options(|options| options.input_options.max_double_click_delay);

                    if state
                        .last_clicked
                        .is_some_and(|(last, at)| last == row_index && now - at <= delay)
                    {
                        // Double click: display message (manual p. 43).
                        display_message = Some(msg.id);
                    } else {
                        clicked_row = Some((row_index, now));
                    }
                }
            }

            // Scroll wheel over the list.
            let pointer = ui.input(|input| input.pointer.hover_pos());
            if pointer.is_some_and(|p| message_index_rect_contains(list_rect, p)) {
                wheel = ui.input(|input| input.raw_scroll_delta.y);
            }
        });

    // Apply click.
    if let Some((row_index, now)) = clicked_row {
        state.last_clicked = Some((row_index, now));
        let modifiers = ctx.input(|i| i.modifiers);
        apply_click(state, row_index, rows, modifiers);
    }

    // Apply double-click: transition to mode 2.
    if let Some(id) = display_message {
        state.mode = IndexMode::SingleMessage;
        state.viewing = Some(id);
        state.last_clicked = None;
        actions.push(MessageIndexAction::MessageDisplayed(id));
    }

    // Apply scroll.
    if wheel != 0.0 {
        let max_scroll = rows.len().saturating_sub(VISIBLE_ROWS);
        state.scroll_offset = if wheel > 0.0 {
            state.scroll_offset.saturating_sub(1)
        } else {
            (state.scroll_offset + 1).min(max_scroll)
        };
    }

    actions
}

/// Apply a click with modifiers to the selection.
///
/// Manual pp. 78-80: click selects one, Ctrl toggles, Shift extends range.
fn apply_click(
    state: &mut MessageIndexState,
    row_index: usize,
    rows: &[&GameMessage],
    modifiers: egui::Modifiers,
) {
    let Some(msg) = rows.get(row_index) else {
        return;
    };

    if modifiers.ctrl || modifiers.command {
        // Ctrl-click: toggle one message (manual p. 43).
        if !state.selected.remove(&msg.id) {
            state.selected.insert(msg.id);
        }
        state.anchor = Some(row_index);
    } else if modifiers.shift {
        // Shift-click: contiguous range from anchor (manual p. 43).
        if let Some(anchor) = state.anchor {
            let start = anchor.min(row_index);
            let end = anchor.max(row_index);
            state.selected.clear();
            for i in start..=end {
                if let Some(m) = rows.get(i) {
                    state.selected.insert(m.id);
                }
            }
            // anchor stays
        } else {
            // No anchor: treat as normal click.
            state.selected.clear();
            state.selected.insert(msg.id);
            state.anchor = Some(row_index);
        }
    } else {
        // Normal click: select one, clear the rest.
        state.selected.clear();
        state.selected.insert(msg.id);
        state.anchor = Some(row_index);
    }
}

/// Draw the single-message text in mode 2.
fn draw_single_message(
    ctx: &egui::Context,
    origin: egui::Pos2,
    scale: f32,
    rows: &[&GameMessage],
    state: &MessageIndexState,
) {
    let Some(id) = state.viewing else {
        return;
    };
    let Some(msg) = rows.iter().find(|m| m.id == id) else {
        return;
    };

    let text_id = egui::Id::new("message-index-single-text");
    egui::Area::new(text_id)
        .fixed_pos(egui::pos2(
            origin.x + TEXT_AREA_X * scale,
            origin.y + TEXT_AREA_Y * scale,
        ))
        .order(egui::Order::Tooltip)
        .show(ctx, |ui| {
            let area_size = egui::vec2(TEXT_AREA_WIDTH * scale, TEXT_AREA_HEIGHT * scale);
            let (area_rect, _) = ui.allocate_exact_size(area_size, egui::Sense::hover());

            // hyp: font 10 (read) for the display; unmapped.
            let font = egui::FontId::proportional((9.0 * scale).max(6.0));
            ui.painter().with_clip_rect(area_rect).text(
                area_rect.left_top() + egui::vec2(2.0 * scale, 2.0 * scale),
                egui::Align2::LEFT_TOP,
                &msg.text,
                font,
                Color32::WHITE,
            );
        });
}

// ---------------------------------------------------------------------------
// Fixture adapter (preserved)
// ---------------------------------------------------------------------------

/// Fixed-position test adapter for deterministic browser inspection.
#[cfg(feature = "interface-test-fixtures")]
pub fn draw_message_index_fixture(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: CockpitFaction,
) {
    let _ = draw_message_index_shell(ctx, cache, faction, egui::pos2(85.0, 55.0), 1.0);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message_log::{MessageCategory, RailAudience};

    // ── Geometry tests (preserved) ──────────────────────────────────────

    #[test]
    fn category_geometry_matches_the_recovered_native_controls() {
        let controls = message_category_control_specs(CockpitFaction::Alliance);

        assert_eq!(
            controls.map(|control| (control.command_id, control.x, control.width)),
            [
                (0x79, 0, 36),
                (0x7a, 38, 36),
                (0x7b, 76, 36),
                (0x7c, 114, 35),
                (0x7d, 151, 36),
                (0x7e, 189, 36),
                (0x7f, 227, 34),
                (0x80, 263, 36),
                (0x81, 301, 35),
                (0x82, 338, 37),
            ]
        );
        assert_eq!(
            controls.last().unwrap().x + controls.last().unwrap().width,
            375
        );
    }

    #[test]
    fn category_resources_preserve_faction_variants() {
        let alliance = message_category_control_specs(CockpitFaction::Alliance);
        let empire = message_category_control_specs(CockpitFaction::Empire);

        assert_eq!(
            alliance.map(|control| (control.normal_resource, control.pressed_resource)),
            [
                (10_830, 10_831),
                (10_832, 10_833),
                (10_840, 10_841),
                (10_842, 10_843),
                (10_836, 10_837),
                (10_852, 10_853),
                (10_856, 10_857),
                (10_854, 10_855),
                (10_850, 10_851),
                (10_846, 10_847),
            ]
        );
        assert_eq!(
            empire.map(|control| (control.normal_resource, control.pressed_resource)),
            [
                (10_830, 10_831),
                (10_834, 10_835),
                (10_838, 10_839),
                (10_844, 10_845),
                (10_836, 10_837),
                (10_852, 10_853),
                (10_856, 10_857),
                (10_854, 10_855),
                (10_850, 10_851),
                (10_848, 10_849),
            ]
        );
    }

    #[test]
    fn hit_testing_excludes_right_and_bottom_edges() {
        let rect = egui::Rect::from_min_max(egui::pos2(22.0, 46.0), egui::pos2(58.0, 87.0));

        assert!(message_index_rect_contains(rect, egui::pos2(22.0, 46.0)));
        assert!(message_index_rect_contains(
            rect,
            egui::pos2(57.999, 86.999)
        ));
        assert!(!message_index_rect_contains(rect, egui::pos2(58.0, 46.0)));
        assert!(!message_index_rect_contains(rect, egui::pos2(22.0, 87.0)));
    }

    // ── List geometry ───────────────────────────────────────────────────

    #[test]
    fn list_geometry_matches_fun_004665f0_and_fun_006082c0() {
        // FUN_004665f0: list at (25, 108), 368x194.
        // FUN_006082c0 in FUN_00468ab0: row 0x15a (346) by 0x15 (21).
        assert_eq!((LIST_X, LIST_Y), (25.0, 108.0));
        assert_eq!((LIST_WIDTH, LIST_HEIGHT), (368.0, 194.0));
        assert_eq!((ROW_WIDTH, ROW_HEIGHT), (346.0, 21.0));
        assert_eq!(VISIBLE_ROWS, 9);
    }

    #[test]
    fn text_area_geometry_matches_fun_00469de0() {
        // FUN_00469de0: text area width 0x18b (395) for non-encyclopedia,
        // 0x140 (320) for type 5.
        assert_eq!((TEXT_AREA_X, TEXT_AREA_Y), (17.0, 234.0));
        assert_eq!(TEXT_AREA_WIDTH, 395.0);
        assert_eq!(TEXT_AREA_HEIGHT, 80.0);
    }

    // ── Category filter ─────────────────────────────────────────────────

    #[test]
    fn category_filter_masks_match_fun_004697b0() {
        // FUN_004697b0: the switch sets +0x158 for each command.
        assert_eq!(category_filter_mask(0x79), 0);
        assert_eq!(category_filter_mask(0x7a), 0x001);
        assert_eq!(category_filter_mask(0x7b), 0x080);
        assert_eq!(category_filter_mask(0x7c), 0x010);
        assert_eq!(category_filter_mask(0x7d), 0x004);
        assert_eq!(category_filter_mask(0x7e), 0x008);
        assert_eq!(category_filter_mask(0x7f), 0x040);
        assert_eq!(category_filter_mask(0x80), 0x100);
        assert_eq!(category_filter_mask(0x81), 0x020);
        assert_eq!(category_filter_mask(0x82), 0x200);
    }

    // ── Message filtering ───────────────────────────────────────────────

    fn test_log() -> MessageLog {
        let mut log = MessageLog::new(100);
        log.push(
            GameMessage::new(1, "Fleet arrived", MessageCategory::Mission)
                .on_rail(MessageRail::Fleet, RailAudience::Both),
        );
        log.push(
            GameMessage::new(2, "Built ship", MessageCategory::Manufacturing)
                .on_rail(MessageRail::Manufacturing, RailAudience::Both),
        );
        log.push(
            GameMessage::new(3, "Advisor says", MessageCategory::Event)
                .on_rail(MessageRail::Advice, RailAudience::Both),
        );
        log.push(
            GameMessage::new(4, "Mission done", MessageCategory::Mission)
                .on_rail(MessageRail::Mission, RailAudience::Both),
        );
        log.push(GameMessage::new(5, "No rail", MessageCategory::Event));
        log
    }

    #[test]
    fn all_messages_shows_everything_except_advice() {
        // FUN_00468f20: when category is 0x79 and mask is 0x200, the item
        // is hidden. All other masks are shown.
        let log = test_log();
        let rows = filtered_messages(&log, 0x79, true);
        let texts: Vec<&str> = rows.iter().map(|m| m.text.as_str()).collect();

        assert_eq!(texts, ["Fleet arrived", "Built ship", "Mission done"]);
        assert!(!rows.iter().any(|m| m.rail == Some(MessageRail::Advice)));
    }

    #[test]
    fn specific_category_shows_only_its_rail() {
        // FUN_00468f20: visible when (filter & mask) != 0.
        let log = test_log();
        let fleet = filtered_messages(&log, 0x7b, true);
        assert_eq!(fleet.len(), 1);
        assert_eq!(fleet[0].text, "Fleet arrived");

        let mfg = filtered_messages(&log, 0x7e, true);
        assert_eq!(mfg.len(), 1);
        assert_eq!(mfg[0].text, "Built ship");

        let advice = filtered_messages(&log, 0x82, true);
        assert_eq!(advice.len(), 1);
        assert_eq!(advice[0].text, "Advisor says");
    }

    #[test]
    fn no_rail_messages_never_appear_in_the_index() {
        let log = test_log();
        for cmd in [0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82] {
            let rows = filtered_messages(&log, cmd, true);
            assert!(
                !rows.iter().any(|m| m.text == "No rail"),
                "command {cmd:#x}",
            );
        }
    }

    #[test]
    fn messages_for_the_other_side_are_hidden() {
        let mut log = MessageLog::new(10);
        log.push(
            GameMessage::new(1, "For alliance", MessageCategory::Mission)
                .on_rail(MessageRail::Fleet, RailAudience::Alliance),
        );
        log.push(
            GameMessage::new(2, "For empire", MessageCategory::Mission)
                .on_rail(MessageRail::Fleet, RailAudience::Empire),
        );
        log.push(
            GameMessage::new(3, "For both", MessageCategory::Mission)
                .on_rail(MessageRail::Fleet, RailAudience::Both),
        );

        let alliance = filtered_messages(&log, 0x7b, true);
        assert_eq!(alliance.len(), 2);
        assert_eq!(alliance[0].text, "For alliance");
        assert_eq!(alliance[1].text, "For both");

        let empire = filtered_messages(&log, 0x7b, false);
        assert_eq!(empire.len(), 2);
        assert_eq!(empire[0].text, "For empire");
        assert_eq!(empire[1].text, "For both");
    }

    // ── Advice speed ────────────────────────────────────────────────────

    #[test]
    fn switching_to_advice_emits_advice_shown() {
        // FUN_004697b0 case 0x82 -> FUN_0041d3f0 -> FUN_00487ff0(1).
        let mut state = MessageIndexState::new();
        let actions = state.set_category(0x82);
        assert_eq!(actions, [MessageIndexAction::AdviceShown]);
    }

    #[test]
    fn switching_from_advice_emits_advice_hidden() {
        // FUN_004697b0 non-0x82 -> FUN_0041d410 -> FUN_00487ff0(0).
        let mut state = MessageIndexState::new();
        state.set_category(0x82);

        let actions = state.set_category(0x7b);

        assert_eq!(actions, [MessageIndexAction::AdviceHidden]);
    }

    #[test]
    fn switching_to_same_category_emits_nothing() {
        let mut state = MessageIndexState::new();
        let actions = state.set_category(0x79);
        assert!(actions.is_empty());
    }

    #[test]
    fn closing_the_window_emits_advice_hidden_and_mark_read() {
        // FUN_0046a6a0: close -> FUN_0041d410 (restore speed) + FUN_0048a530.
        let mut state = MessageIndexState::new();
        state.set_category(0x7b);

        let actions = state.close();

        assert!(actions.contains(&MessageIndexAction::AdviceHidden));
        assert!(actions.contains(&MessageIndexAction::MarkCategoryRead(Some(
            MessageRail::Fleet
        ))));
    }

    #[test]
    fn closing_all_messages_marks_all_read() {
        // FUN_0048a530 with param 0: mark all read.
        let state = MessageIndexState::new(); // default is All (0x79)
        let actions = state.close();
        assert!(actions.contains(&MessageIndexAction::MarkCategoryRead(None)));
    }

    // ── Selection model ─────────────────────────────────────────────────

    fn make_rows(log: &MessageLog) -> Vec<&GameMessage> {
        filtered_messages(log, 0x79, true)
    }

    #[test]
    fn click_selects_one_row_clearing_others() {
        // Manual p. 43: "a click selects one message."
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();

        apply_click(&mut state, 0, &rows, egui::Modifiers::NONE);
        assert_eq!(state.selected, BTreeSet::from([rows[0].id]));

        apply_click(&mut state, 2, &rows, egui::Modifiers::NONE);
        assert_eq!(state.selected, BTreeSet::from([rows[2].id]));
    }

    #[test]
    fn ctrl_click_toggles_an_individual_message() {
        // Manual p. 43: "Ctrl-click toggles an individual message."
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        let ctrl = egui::Modifiers {
            command: true,
            ..Default::default()
        };

        apply_click(&mut state, 0, &rows, egui::Modifiers::NONE);
        apply_click(&mut state, 1, &rows, ctrl);

        assert_eq!(state.selected, BTreeSet::from([rows[0].id, rows[1].id]));

        // Toggle off.
        apply_click(&mut state, 0, &rows, ctrl);
        assert_eq!(state.selected, BTreeSet::from([rows[1].id]));
    }

    #[test]
    fn shift_click_selects_a_contiguous_range_from_the_anchor() {
        // Manual p. 43: "Shift-click selects a contiguous range."
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        let shift = egui::Modifiers {
            shift: true,
            ..Default::default()
        };

        // Anchor at row 0.
        apply_click(&mut state, 0, &rows, egui::Modifiers::NONE);

        // Shift-click row 2: selects 0, 1, 2.
        apply_click(&mut state, 2, &rows, shift);

        assert_eq!(
            state.selected,
            BTreeSet::from([rows[0].id, rows[1].id, rows[2].id])
        );
        assert_eq!(state.anchor, Some(0)); // anchor stays
    }

    #[test]
    fn shift_click_range_works_backwards() {
        // Manual p. 43: range is contiguous, direction doesn't matter.
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        let shift = egui::Modifiers {
            shift: true,
            ..Default::default()
        };

        apply_click(&mut state, 2, &rows, egui::Modifiers::NONE);
        apply_click(&mut state, 0, &rows, shift);

        assert_eq!(
            state.selected,
            BTreeSet::from([rows[0].id, rows[1].id, rows[2].id])
        );
    }

    #[test]
    fn shift_click_without_anchor_acts_as_normal_click() {
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        let shift = egui::Modifiers {
            shift: true,
            ..Default::default()
        };

        apply_click(&mut state, 1, &rows, shift);

        assert_eq!(state.selected, BTreeSet::from([rows[1].id]));
        assert_eq!(state.anchor, Some(1));
    }

    // ── Delete ──────────────────────────────────────────────────────────

    #[test]
    fn delete_selected_returns_the_ids_and_clears_selection() {
        // FUN_005f54a0: delete a single message by key, iterated over
        // the selection.
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        let ctrl = egui::Modifiers {
            command: true,
            ..Default::default()
        };

        apply_click(&mut state, 0, &rows, egui::Modifiers::NONE);
        apply_click(&mut state, 2, &rows, ctrl);

        let action = state.delete_selected();

        assert!(matches!(action, Some(MessageIndexAction::DeleteSelected(ids)) if ids.len() == 2));
        assert!(state.selected.is_empty());
    }

    #[test]
    fn delete_with_nothing_selected_returns_none() {
        let mut state = MessageIndexState::new();
        assert!(state.delete_selected().is_none());
    }

    // ── Window buttons ──────────────────────────────────────────────────
    #[test]
    fn each_category_command_names_the_rail_its_mask_filters() {
        // FUN_004697b0: commands 0x7a..0x82 filter by the masks 0x001..0x200,
        // one rail's bit each; All (0x79) names none.
        for command in 0x7a..=0x82 {
            let rail = category_rail(command).expect("a one-rail category");
            assert_eq!(
                rail.mask(),
                category_filter_mask(command),
                "command 0x{command:x}"
            );
        }
        assert_eq!(category_rail(0x79), None);
    }

    #[test]
    fn only_entering_or_leaving_advice_moves_the_speed_and_a_switch_returns_to_the_list() {
        // FUN_004697b0: 0x82 calls FUN_0041d3f0, the others FUN_0041d410;
        // the hold guard (FUN_00487ff0) makes a switch between two others
        // change nothing. A switch rebuilds the list (FUN_00468ab0).
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        assert_eq!(state.set_category(0x7b), []);
        assert_eq!(state.set_category(0x7c), []);
        assert_eq!(state.category(), 0x7c);
        assert_eq!(state.set_category(0x82), [MessageIndexAction::AdviceShown]);
        assert_eq!(state.set_category(0x82), []);
        assert_eq!(state.set_category(0x79), [MessageIndexAction::AdviceHidden]);

        apply_click(&mut state, 0, &rows, egui::Modifiers::NONE);
        assert_eq!(state.selected(), &BTreeSet::from([rows[0].id]));
        state.toggle_display();
        assert_eq!(state.mode(), IndexMode::SingleMessage);
        state.set_category(0x7a);
        assert_eq!(state.mode(), IndexMode::List);
        assert_eq!(state.viewing, None);
        assert!(state.selected().is_empty());
    }

    #[test]
    fn each_cockpit_command_opens_its_own_category() {
        // FUN_00422ce0: 0x75 -> 0x79, 0x136..0x13c -> 0x7a..0x80,
        // 0x13d -> 0x82 (Advice), 0x13e -> 0x81 (Chat).
        assert_eq!(opening_category(0x75), Some(0x79));
        assert_eq!(opening_category(0x136), Some(0x7a));
        assert_eq!(opening_category(0x13c), Some(0x80));
        assert_eq!(opening_category(0x13d), Some(0x82));
        assert_eq!(opening_category(0x13e), Some(0x81));
        assert_eq!(opening_category(0x13f), None);
        assert_eq!(opening_category(0x135), None);
    }

    #[test]
    fn the_display_button_shows_the_selected_message_then_returns_to_the_list() {
        // FUN_00468fb0(0x65): mode 1 with a selection goes to mode 2; an
        // empty selection stays; mode 2 goes back to the list.
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        assert_eq!(state.toggle_display(), None);
        assert_eq!(state.mode, IndexMode::List);

        apply_click(&mut state, 1, &rows, egui::Modifiers::NONE);
        assert_eq!(
            state.toggle_display(),
            Some(MessageIndexAction::MessageDisplayed(rows[1].id))
        );
        assert_eq!(state.mode, IndexMode::SingleMessage);
        assert_eq!(state.viewing, Some(rows[1].id));

        assert_eq!(state.toggle_display(), None);
        assert_eq!(state.mode, IndexMode::List);
        assert_eq!(state.viewing, None);
    }

    #[test]
    fn close_and_display_sit_on_the_rail_and_delete_shows_in_the_list_only() {
        // FUN_004665f0: 0x28 and 0x65 at x 0x1a7 (Alliance) / 0x1aa (Empire);
        // 0x91 at (0x154, 0x57), shown in mode 1 (FUN_00468fb0).
        let list = window_button_specs(CockpitFaction::Alliance, IndexMode::List);
        assert_eq!(
            list.iter()
                .map(|button| (
                    button.command_id,
                    button.x,
                    button.y,
                    button.normal_resource
                ))
                .collect::<Vec<_>>(),
            [
                (0x28, 0x1a7, 0x19, 0x2882),
                (0x65, 0x1a7, 0x5d, 0x2884),
                (0x91, 0x154, 0x57, 0x2a96)
            ]
        );
        let empire = window_button_specs(CockpitFaction::Empire, IndexMode::SingleMessage);
        assert_eq!(
            empire
                .iter()
                .map(|button| (
                    button.command_id,
                    button.x,
                    button.width,
                    button.pressed_resource
                ))
                .collect::<Vec<_>>(),
            [(0x28, 0x1aa, 0x2c, 0x2889), (0x65, 0x1aa, 0x2c, 0x288b)]
        );
    }

    // ── Category switch state reset ─────────────────────────────────────

    #[test]
    fn category_switch_clears_selection_and_scroll() {
        // FUN_004697b0: rebuilds the list (FUN_00468ab0) on category change.
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();

        apply_click(&mut state, 1, &rows, egui::Modifiers::NONE);
        state.scroll_offset = 5;

        state.set_category(0x7b);

        assert!(state.selected.is_empty());
        assert_eq!(state.scroll_offset, 0);
        assert_eq!(state.anchor, None);
    }

    // ── Report ──────────────────────────────────────────────────────────

    #[test]
    fn report_reflects_the_filtered_rows_and_selection() {
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        apply_click(&mut state, 1, &rows, egui::Modifiers::NONE);

        let report = state.report(&log, true);

        assert_eq!(report.mode, IndexMode::List);
        assert_eq!(report.category, 0x79);
        assert_eq!(report.rows.len(), 3); // All except Advice and no-rail
        assert_eq!(report.selected, BTreeSet::from([rows[1].id]));
        assert_eq!(report.rows[0].text, "Fleet arrived");
        assert_eq!(report.rows[0].rail, MessageRail::Fleet);
        assert!(report.rows[0].unread);
    }

    // ── Headless egui interaction tests ──────────────────────────────────

    /// Build a `MessageLog` with enough fleet messages for scroll testing.
    #[allow(dead_code)]
    fn scroll_log(count: usize) -> MessageLog {
        let mut log = MessageLog::new(100);
        for i in 0..count {
            log.push(
                GameMessage::new(
                    i as u64,
                    format!("Fleet message {i}"),
                    MessageCategory::Mission,
                )
                .on_rail(MessageRail::Fleet, RailAudience::Both),
            );
        }
        log
    }

    fn press(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    fn press_with(pos: egui::Pos2, pressed: bool, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers,
        }
    }

    /// The screen position of the center of list row `line` (0-based
    /// visible line) at scale 1 with origin (0, 0).
    fn row_center(line: usize) -> egui::Pos2 {
        egui::pos2(
            LIST_X + TEXT_INSET + 40.0,
            LIST_Y + ROW_HEIGHT * line as f32 + ROW_HEIGHT / 2.0,
        )
    }

    struct DriveResult {
        actions: Vec<MessageIndexAction>,
    }

    /// Run two idle frames, then one frame per entry in `frames` with the
    /// given events, returning the collected actions.
    fn drive(
        log: &MessageLog,
        state: &mut MessageIndexState,
        frames: Vec<(egui::Pos2, Vec<egui::Event>)>,
    ) -> DriveResult {
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let origin = egui::Pos2::ZERO;
        let scale = 1.0;
        let mut all_actions = Vec::new();

        let first = frames.first().map_or(egui::Pos2::ZERO, |(pos, _)| *pos);
        let full_frames = [(first, vec![]), (first, vec![])].into_iter().chain(frames);

        for (index, (pos, events)) in full_frames.enumerate() {
            // Extract modifier state from PointerButton events so that
            // RawInput::modifiers (read by ctx.input(|i| i.modifiers))
            // reflects which keys are held during this frame.
            let frame_modifiers = events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::PointerButton { modifiers, .. } => Some(*modifiers),
                    _ => None,
                })
                .next_back()
                .unwrap_or_default();

            let mut raw_events = vec![egui::Event::PointerMoved(pos)];
            raw_events.extend(events.into_iter().map(|event| match event {
                egui::Event::PointerButton {
                    pressed, modifiers, ..
                } => egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers,
                },
                other => other,
            }));
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 480.0),
                )),
                time: Some(index as f64 * 0.05),
                modifiers: frame_modifiers,
                events: raw_events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                let actions = draw_message_index(
                    ctx,
                    &mut cache,
                    CockpitFaction::Alliance,
                    origin,
                    scale,
                    log,
                    state,
                    true,
                );
                all_actions.extend(actions);
            });
        }

        DriveResult {
            actions: all_actions,
        }
    }

    fn click_at(pos: egui::Pos2) -> Vec<(egui::Pos2, Vec<egui::Event>)> {
        vec![
            (pos, vec![press(pos, true)]),
            (pos, vec![press(pos, false)]),
        ]
    }

    fn ctrl_click_at(pos: egui::Pos2) -> Vec<(egui::Pos2, Vec<egui::Event>)> {
        let mods = egui::Modifiers {
            command: true,
            ..Default::default()
        };
        vec![
            (pos, vec![press_with(pos, true, mods)]),
            (pos, vec![press_with(pos, false, mods)]),
        ]
    }

    fn shift_click_at(pos: egui::Pos2) -> Vec<(egui::Pos2, Vec<egui::Event>)> {
        let mods = egui::Modifiers {
            shift: true,
            ..Default::default()
        };
        vec![
            (pos, vec![press_with(pos, true, mods)]),
            (pos, vec![press_with(pos, false, mods)]),
        ]
    }

    #[test]
    fn egui_click_selects_one_row() {
        // Manual p. 43: a click selects one message.
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        state.set_category(0x79);

        drive(&log, &mut state, click_at(row_center(1)));

        assert_eq!(state.selected, BTreeSet::from([rows[1].id]));
    }

    #[test]
    fn egui_ctrl_click_toggles_selection() {
        // Manual p. 43: Ctrl-click toggles an individual message.
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        state.set_category(0x79);

        let mut frames = click_at(row_center(0));
        frames.extend(ctrl_click_at(row_center(2)));
        drive(&log, &mut state, frames);

        assert_eq!(state.selected, BTreeSet::from([rows[0].id, rows[2].id]));
    }

    #[test]
    fn egui_shift_click_selects_contiguous_range() {
        // Manual p. 43: Shift-click selects a contiguous range.
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        state.set_category(0x79);

        let mut frames = click_at(row_center(0));
        frames.extend(shift_click_at(row_center(2)));
        drive(&log, &mut state, frames);

        assert_eq!(
            state.selected,
            BTreeSet::from([rows[0].id, rows[1].id, rows[2].id])
        );
    }

    #[test]
    fn egui_double_click_switches_to_mode_2_and_marks_read() {
        // Manual p. 43: double-click opens (reads) the message.
        // FUN_00469de0: +0x38 = 10 (read font).
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        state.set_category(0x79);

        let pos = row_center(1);
        let frames = vec![
            (pos, vec![press(pos, true)]),
            (pos, vec![press(pos, false)]),
            (pos, vec![press(pos, true)]),
            (pos, vec![press(pos, false)]),
        ];
        let result = drive(&log, &mut state, frames);

        assert_eq!(state.mode, IndexMode::SingleMessage);
        assert_eq!(state.viewing, Some(rows[1].id));
        assert!(result
            .actions
            .contains(&MessageIndexAction::MessageDisplayed(rows[1].id)));
    }

    #[test]
    fn egui_category_switch_resets_state_and_emits_actions() {
        // FUN_004697b0: switching category rebuilds the list.
        let log = test_log();
        let rows = make_rows(&log);
        let mut state = MessageIndexState::new();
        apply_click(&mut state, 1, &rows, egui::Modifiers::NONE);

        let actions = state.set_category(0x82);

        assert!(actions.contains(&MessageIndexAction::AdviceShown));
        assert!(state.selected.is_empty());
        assert_eq!(state.category, 0x82);
    }
}
