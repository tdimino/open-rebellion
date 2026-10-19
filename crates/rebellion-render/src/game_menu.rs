//! The original Game Menu Window: the pop-up the speed control and the
//! object right-click menus open (`FUN_00442860`).
//!
//! `FUN_00442a80` lays out the rows, `FUN_004aab50` paints them, and
//! `FUN_004aba60` draws each item in one of three colors that
//! `FUN_004abe10` stores: `+0x30` for a normal item, `+0x2c` for the
//! highlighted one, and `+0x34` when the item is disabled. Every builder
//! passes the faction color as the highlight, white as the normal color, and
//! gray `0x2808080` as the disabled one (`FUN_0042d190`, `FUN_004ac5c0`).
//! Recovery notes: `ghidra/notes/object-popup-menu.md` and
//! `docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-24-game-speed-recovery.md`.

use egui_macroquad::egui;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{
    gid_popup_frame, paint_gid_frame_border, CockpitFaction, CockpitLayout, CockpitViewport,
    STRATEGIC_LOGICAL_HEIGHT, STRATEGIC_LOGICAL_WIDTH,
};

/// Menu text height. The Game Menu Window uses the application default font
/// (`FUN_00603850`), whose table entry is not yet recovered.
const MENU_FONT_HEIGHT: f32 = 14.0;

/// Icon column a submenu parent without an icon reserves (`FUN_004abf60`).
const SUBMENU_ICON_WIDTH: f32 = 20.0;

/// Normal item color `0x2ffffff`.
const NORMAL_COLOR: egui::Color32 = egui::Color32::WHITE;

/// Disabled item color `0x2808080`.
const DISABLED_COLOR: egui::Color32 = egui::Color32::from_rgb(0x80, 0x80, 0x80);

/// Faction text color: Alliance `0x20000ff` (red), Empire `0x200ff00` (green).
/// The menus highlight an item in it, and the day readout is drawn in it.
#[must_use]
pub const fn faction_text_color(faction: CockpitFaction) -> egui::Color32 {
    match faction {
        CockpitFaction::Alliance => egui::Color32::from_rgb(255, 0, 0),
        CockpitFaction::Empire => egui::Color32::from_rgb(0, 255, 0),
    }
}

/// One row of a Game Menu Window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameMenuEntry<'a> {
    pub label: &'a str,
    /// STRATEGY icon named by the record's words 7 and 8.
    pub icon: Option<u32>,
    /// Cleared when the record's flags carry bit 0 (`FUN_00442590`).
    pub enabled: bool,
    /// A submenu parent (`FUN_00442590` flags 8), which widens the icon column.
    pub submenu: bool,
    /// The submenu's rows, built with the menu (`FUN_00442590` adds each
    /// child record to its parent row's list, `FUN_00442860` builds the
    /// window).
    pub children: &'a [GameMenuEntry<'a>],
}

/// The color `FUN_004aba60` draws an entry's text in.
#[must_use]
pub fn entry_text_color(
    entry: &GameMenuEntry<'_>,
    highlighted: bool,
    faction: CockpitFaction,
) -> egui::Color32 {
    if !entry.enabled {
        DISABLED_COLOR
    } else if highlighted {
        faction_text_color(faction)
    } else {
        NORMAL_COLOR
    }
}

/// The width `FUN_004abf60` measures for an entry's icon column.
#[must_use]
pub fn entry_icon_width(entry: &GameMenuEntry<'_>, icon_width: Option<f32>) -> f32 {
    match icon_width {
        Some(width) => width,
        None if entry.submenu => SUBMENU_ICON_WIDTH,
        None => 0.0,
    }
}

/// Row geometry of a Game Menu Window, in logical pixels.
#[derive(Debug, Clone, PartialEq)]
pub struct GameMenuGeometry {
    /// Widest item icon (`FUN_004abf60`), shared by every row.
    pub icon_column: f32,
    pub width: f32,
    pub height: f32,
    /// Top edge and height of each row.
    pub rows: Vec<(f32, f32)>,
}

/// Lay out rows as `FUN_00442a80` and `FUN_004abb80` / `FUN_004abbf0` /
/// `FUN_004abc70` do.
///
/// Each row is `icon_column + 12 + text` wide and `max(icon, text) + 4` tall.
/// Rows start two pixels down, and the window adds two pixels below the sum.
#[must_use]
pub fn game_menu_geometry(
    icon_sizes: &[(f32, f32)],
    text_sizes: &[(f32, f32)],
) -> GameMenuGeometry {
    let icon_column = icon_sizes.iter().map(|size| size.0).fold(0.0, f32::max);
    let mut width = 0.0_f32;
    let mut rows = Vec::with_capacity(text_sizes.len());
    let mut top = 2.0;
    for (index, text) in text_sizes.iter().enumerate() {
        let icon_height = icon_sizes.get(index).map_or(0.0, |size| size.1);
        width = width.max(icon_column + 12.0 + text.0);
        let height = (icon_height + 4.0).max(text.1 + 4.0);
        rows.push((top, height));
        top += height;
    }
    let height = rows.iter().map(|row| row.1).sum::<f32>() + 2.0;
    GameMenuGeometry {
        icon_column,
        width,
        height,
        rows,
    }
}

/// Place a menu at the cursor, flipping as `FUN_00442860` does when it would
/// cross the owner window's right or bottom edge.
#[must_use]
pub fn game_menu_origin(cursor: (f32, f32), size: (f32, f32), owner: (f32, f32)) -> (f32, f32) {
    let (mut x, mut y) = cursor;
    if y + size.1 > owner.1 {
        y -= size.1;
    }
    if x < 0.0 {
        x += size.0;
    } else if x + size.0 > owner.0 {
        x -= size.0;
    }
    (x, y)
}

/// Where a menu opens: the cursor point and the window that owns the menu,
/// both in 640 by 480 canvas coordinates. `FUN_00442860` flips the menu at
/// the owner's right and bottom edges.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameMenuPlacement {
    pub owner: CockpitViewport,
    pub point: (f32, f32),
}

impl GameMenuPlacement {
    /// A menu owned by the main frame, as the speed menu is (`FUN_0042d190`
    /// passes the command center window).
    #[must_use]
    pub const fn in_frame(point: (f32, f32)) -> Self {
        Self {
            owner: CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: STRATEGIC_LOGICAL_WIDTH,
                height: STRATEGIC_LOGICAL_HEIGHT,
            },
            point,
        }
    }

    /// The menu's top-left corner for a menu of `size`.
    #[must_use]
    pub fn origin(self, size: (f32, f32)) -> (f32, f32) {
        let (x, y) = game_menu_origin(
            (self.point.0 - self.owner.x, self.point.1 - self.owner.y),
            size,
            (self.owner.width, self.owner.height),
        );
        (self.owner.x + x, self.owner.y + y)
    }
}

/// What the player did with an open menu this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMenuResponse {
    Open,
    /// The index of the enabled entry chosen; the menu closes.
    Chosen(usize),
    /// An enabled entry of the submenu under entry `row`; the whole menu
    /// closes (`FUN_004424c0` posts `0x409` to the view).
    ChosenChild { row: usize, child: usize },
    /// A press outside the menu or Escape closed it.
    Dismissed,
}

/// Submenu arrows, highlighted and normal: 10117 and 10118 for side 1, else
/// 10128 and 10129 (`FUN_00442860`, loaded by `FUN_004ab3a0`).
const fn submenu_arrows(faction: CockpitFaction) -> (u32, u32) {
    match faction {
        CockpitFaction::Alliance => (10117, 10118),
        CockpitFaction::Empire => (10128, 10129),
    }
}

/// The period of the timer `FUN_004aab50` sets when a menu window shows
/// (`SetTimer(hwnd, 100, 500)`); each tick shows the highlighted row's
/// submenu.
const SUBMENU_TIMER_SECONDS: f64 = 0.5;

/// Where `FUN_004ab3a0` puts a submenu of `child` size beside row
/// `row_span` (top, height) of the parent window `parent` (x, y, width,
/// height): left of the parent, overlapping it by three pixels, or right of
/// it when that would cross the left edge; level with the row's top less
/// three when it fits above the parent's bottom, else with its bottom on
/// the row's bottom plus three.
/// hyp: the window fields `+0x28`, `+0x2c`, `+0x30`, `+0x3c` it reads are
/// the parent's left, top, width and height.
#[must_use]
pub fn submenu_origin(
    parent: (f32, f32, f32, f32),
    row_span: (f32, f32),
    child: (f32, f32),
) -> (f32, f32) {
    let (px, py, pw, ph) = parent;
    let (top, height) = row_span;
    let mut x = px - child.0 + 3.0;
    if x < 0.0 {
        x = px + pw - 3.0;
    }
    let y = if top + child.1 < ph {
        py + top - 3.0
    } else {
        py + top + height - child.1 + 3.0
    };
    (x, y)
}

/// The rows `FUN_004aab50` keeps between messages: the highlighted row
/// (`+0x46`, moved only onto enabled rows), the row whose submenu shows,
/// the submenu's highlighted row, and the timer's ticks so far.
#[derive(Debug, Clone, Copy, Default)]
struct MenuMemory {
    highlighted: Option<usize>,
    shown: Option<usize>,
    child_highlighted: Option<usize>,
    opened_at: f64,
    ticks: u64,
    last_pass: u64,
}

/// Text and icon sizes of `entries` and their layout.
fn measure(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    entries: &[GameMenuEntry<'_>],
    font: &egui::FontId,
    scale: f32,
) -> (Vec<(f32, f32)>, GameMenuGeometry) {
    let text_sizes: Vec<(f32, f32)> = entries
        .iter()
        .map(|entry| {
            let size = ctx
                .fonts(|fonts| {
                    fonts.layout_no_wrap(entry.label.to_owned(), font.clone(), NORMAL_COLOR)
                })
                .size();
            (size.x / scale, size.y / scale)
        })
        .collect();
    let icon_sizes: Vec<(f32, f32)> = entries
        .iter()
        .map(|entry| {
            let icon = entry.icon.and_then(|id| {
                cache
                    .get(ctx, DllSource::Strategy, id)
                    .map(|texture| texture.size_vec2())
            });
            (
                entry_icon_width(entry, icon.map(|size| size.x)),
                icon.map_or(0.0, |size| size.y),
            )
        })
        .collect();
    let geometry = game_menu_geometry(&icon_sizes, &text_sizes);
    (text_sizes, geometry)
}

/// One Game Menu Window drawn in its own area: the enabled row clicked, and
/// the area's screen rectangle.
struct PaintedMenu {
    clicked: Option<usize>,
    rect: egui::Rect,
}

/// The enabled row of a menu window at canvas `origin` under the screen
/// point `pointer` (`FUN_004aab50`'s `WM_MOUSEMOVE` takes only rows with
/// `+0x58` set), and whether the point is inside the window at all.
fn row_under(
    layout: CockpitLayout,
    origin: (f32, f32),
    entries: &[GameMenuEntry<'_>],
    geometry: &GameMenuGeometry,
    pointer: egui::Pos2,
) -> (bool, Option<usize>) {
    let scale = layout.scale.max(0.5);
    let min = egui::pos2(
        layout.canvas.x + origin.0 * layout.scale,
        layout.canvas.y + origin.1 * layout.scale,
    );
    let rect = egui::Rect::from_min_size(
        min,
        egui::vec2(geometry.width * scale, geometry.height * scale),
    );
    if !rect.contains(pointer) {
        return (false, None);
    }
    let row = geometry
        .rows
        .iter()
        .position(|&(top, height)| {
            let y = pointer.y - rect.min.y;
            y >= top * scale && y < (top + height) * scale
        })
        .filter(|&index| entries.get(index).is_some_and(|entry| entry.enabled));
    (true, row)
}

/// Paint one menu window at canvas `origin` (`FUN_004aab50`'s `WM_PAINT`):
/// the rows, the icon or check mark at (6, row top) (`FUN_004aba60`), the
/// submenu arrow at the same place (highlighted or normal), and the frame.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep the window's area, faction, layout and rows explicit at this drawing boundary."
)]
fn paint_menu(
    ctx: &egui::Context,
    id: egui::Id,
    cache: &mut BmpCache,
    layout: CockpitLayout,
    faction: CockpitFaction,
    origin: (f32, f32),
    entries: &[GameMenuEntry<'_>],
    measured: &(Vec<(f32, f32)>, GameMenuGeometry),
    highlighted: Option<usize>,
    input_enabled: bool,
    window: &'static str,
) -> PaintedMenu {
    let scale = layout.scale.max(0.5);
    let font = egui::FontId::proportional(MENU_FONT_HEIGHT * scale);
    let (text_sizes, geometry) = measured;
    let screen = egui::pos2(
        layout.canvas.x + origin.0 * layout.scale,
        layout.canvas.y + origin.1 * layout.scale,
    );
    let menu_size = egui::vec2(geometry.width * scale, geometry.height * scale);
    let (arrow_highlighted, arrow_normal) = submenu_arrows(faction);

    // FUN_00442860 opens a pop-up over every child window. The modeless
    // windows share Foreground and a focused one raises itself each frame
    // (`system_window.rs`), so the menu takes the order above them, which
    // it shares with the mission dialog, and raises itself there.
    ctx.move_to_top(egui::LayerId::new(egui::Order::Tooltip, id));
    let shown = egui::Area::new(id)
        .order(egui::Order::Tooltip)
        .fade_in(false)
        .fixed_pos(screen)
        .show(ctx, |ui| {
            if !input_enabled {
                ui.disable();
            }
            let (rect, _) = ui.allocate_exact_size(menu_size, egui::Sense::hover());
            ui.painter().rect_filled(rect, 0.0, gid_popup_frame().fill);
            let mut clicked = None;
            for (index, ((entry, (top, height)), text)) in entries
                .iter()
                .zip(geometry.rows.iter().copied())
                .zip(text_sizes.iter().copied())
                .enumerate()
            {
                let row = egui::Rect::from_min_size(
                    rect.min + egui::vec2(0.0, top * scale),
                    egui::vec2(rect.width(), height * scale),
                );
                let sense = if entry.enabled {
                    egui::Sense::click()
                } else {
                    egui::Sense::hover()
                };
                crate::control_registry::register_screen(window, entry.label, row);
                let response = ui.interact(row, id.with(("entry", index)), sense);
                let is_highlighted = entry.enabled && highlighted == Some(index);
                let icon = if entry.submenu {
                    Some(if is_highlighted {
                        arrow_highlighted
                    } else {
                        arrow_normal
                    })
                } else {
                    entry.icon
                };
                if let Some(texture) = icon.and_then(|icon| cache.get(ctx, DllSource::Strategy, icon))
                {
                    let size = texture.size_vec2() * scale;
                    ui.painter().image(
                        texture.id(),
                        egui::Rect::from_min_size(row.min + egui::vec2(6.0 * scale, 0.0), size),
                        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
                ui.painter().text(
                    egui::pos2(
                        row.min.x + (geometry.icon_column + 6.0) * scale,
                        row.min.y + ((height - text.1) / 2.0) * scale,
                    ),
                    egui::Align2::LEFT_TOP,
                    entry.label,
                    font.clone(),
                    entry_text_color(entry, is_highlighted, faction),
                );
                if entry.enabled && response.clicked() {
                    clicked = Some(index);
                }
            }
            paint_gid_frame_border(ui, cache, rect, scale);
            clicked
        });
    PaintedMenu {
        clicked: shown.inner,
        rect: shown.response.rect,
    }
}

/// Draw a Game Menu Window at `placement`, with the submenu of the entry
/// whose `children` show.
///
/// A row highlights when the pointer moves onto it and stays highlighted
/// until the pointer moves onto another enabled row (`FUN_004aab50`,
/// `WM_MOUSEMOVE`). The window's 500 ms timer shows the highlighted row's
/// submenu; highlighting another row hides it. A left click chooses an
/// enabled row that is not a submenu parent (`FUN_004424c0`,
/// `FUN_004ab550`), and a press outside the menu and its submenu closes both
/// (`FUN_00442b90`). Escape hides a shown submenu, else closes the menu
/// (`FUN_00442cf0`).
/// port: the timer fires whatever input is pending; the original skips a
/// tick while a message waits for the window.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep the menu's owner, faction, placement and entries explicit at this drawing boundary."
)]
pub fn draw_game_menu(
    ctx: &egui::Context,
    id: egui::Id,
    cache: &mut BmpCache,
    layout: CockpitLayout,
    faction: CockpitFaction,
    placement: GameMenuPlacement,
    entries: &[GameMenuEntry<'_>],
    input_enabled: bool,
) -> GameMenuResponse {
    let scale = layout.scale.max(0.5);
    let font = egui::FontId::proportional(MENU_FONT_HEIGHT * scale);
    let now = ctx.input(|input| input.time);
    let pass = ctx.cumulative_pass_nr();
    let memory_id = id.with("memory");
    // A menu not drawn last pass was closed by its owner; it opens afresh.
    let mut memory = ctx
        .data(|data| data.get_temp::<MenuMemory>(memory_id))
        .filter(|memory| memory.last_pass + 1 >= pass)
        .unwrap_or(MenuMemory {
            opened_at: now,
            ..MenuMemory::default()
        });
    memory.last_pass = pass;

    let window = crate::control_registry::take_menu_window();
    let measured = measure(ctx, cache, entries, &font, scale);
    let geometry = &measured.1;
    let (x, y) = placement.origin((geometry.width, geometry.height));
    let child_layout = |memory: &MenuMemory, cache: &mut BmpCache| {
        let row = memory.shown?;
        let span = *geometry.rows.get(row)?;
        let children = entries.get(row)?.children;
        let child_measured = measure(ctx, cache, children, &font, scale);
        let origin = submenu_origin(
            (x, y, geometry.width, geometry.height),
            span,
            (child_measured.1.width, child_measured.1.height),
        );
        Some((row, children, child_measured, origin))
    };

    // WM_MOUSEMOVE moves the highlight before the window repaints.
    if input_enabled {
        if let Some(pointer) = ctx.input(|input| input.pointer.hover_pos()) {
            let in_child = child_layout(&memory, cache).is_some_and(|(_, children, m, origin)| {
                let (inside, row) = row_under(layout, origin, children, &m.1, pointer);
                if let Some(row) = row {
                    memory.child_highlighted = Some(row);
                }
                inside
            });
            if !in_child {
                if let (_, Some(index)) = row_under(layout, (x, y), entries, geometry, pointer) {
                    if memory.highlighted != Some(index) {
                        memory.highlighted = Some(index);
                        memory.shown = None;
                        memory.child_highlighted = None;
                    }
                }
            }
        }
    }
    // The window's timer ticks whether or not input reaches it.
    let ticks = ((now - memory.opened_at).max(0.0) / SUBMENU_TIMER_SECONDS) as u64;
    if ticks > memory.ticks {
        memory.ticks = ticks;
        if let Some(row) = memory.highlighted.filter(|&row| {
            entries
                .get(row)
                .is_some_and(|entry| entry.submenu && !entry.children.is_empty())
        }) {
            memory.shown = Some(row);
        }
    }
    ctx.request_repaint_after(std::time::Duration::from_millis(100));

    let root = paint_menu(
        ctx,
        id,
        cache,
        layout,
        faction,
        (x, y),
        entries,
        &measured,
        memory.highlighted,
        input_enabled,
        window,
    );

    let mut child_painted = None;
    if let Some((row, children, child_measured, origin)) = child_layout(&memory, cache) {
        child_painted = Some(paint_menu(
            ctx,
            id.with(("submenu", row)),
            cache,
            layout,
            faction,
            origin,
            children,
            &child_measured,
            memory.child_highlighted,
            input_enabled,
            window,
        ));
    }

    let mut response = GameMenuResponse::Open;
    if input_enabled {
        let child_click = child_painted
            .as_ref()
            .and_then(|child| child.clicked)
            .zip(memory.shown);
        let root_click = root
            .clicked
            .filter(|&index| entries.get(index).is_some_and(|entry| !entry.submenu));
        let inside = |pointer: egui::Pos2| {
            root.rect.contains(pointer)
                || child_painted
                    .as_ref()
                    .is_some_and(|child| child.rect.contains(pointer))
        };
        if let Some((child, row)) = child_click {
            response = GameMenuResponse::ChosenChild { row, child };
        } else if let Some(index) = root_click {
            response = GameMenuResponse::Chosen(index);
        } else if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
            if memory.shown.is_some() {
                memory.shown = None;
                memory.child_highlighted = None;
            } else {
                response = GameMenuResponse::Dismissed;
            }
        } else if ctx.input(|input| {
            input.pointer.any_pressed()
                && input
                    .pointer
                    .interact_pos()
                    .is_some_and(|pointer| !inside(pointer))
        }) {
            response = GameMenuResponse::Dismissed;
        }
    }

    if response == GameMenuResponse::Open {
        ctx.data_mut(|data| data.insert_temp(memory_id, memory));
    } else {
        ctx.data_mut(|data| data.remove::<MenuMemory>(memory_id));
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(enabled: bool, submenu: bool) -> GameMenuEntry<'static> {
        GameMenuEntry {
            label: "Mission",
            icon: None,
            enabled,
            submenu,
            children: &[],
        }
    }

    const MENU_ID: &str = "game_menu_test";

    #[test]
    fn the_menu_opens_above_a_dialog_shown_after_it() {
        // FUN_00442860's pop-up is the topmost window; the mission dialog
        // shares the order above the modeless windows (mission_dialog.rs).
        let layout = layout(1.0);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let dialog = egui::Id::new("dialog_above_windows");
        let entries = [labelled("Mission", true)];
        for _ in 0..3 {
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                let _ = draw_game_menu(
                    ctx,
                    egui::Id::new(MENU_ID),
                    &mut cache,
                    layout,
                    CockpitFaction::Alliance,
                    GameMenuPlacement::in_frame((100.0, 100.0)),
                    &entries,
                    true,
                );
                egui::Area::new(dialog)
                    .order(egui::Order::Tooltip)
                    .fixed_pos(egui::Pos2::ZERO)
                    .show(ctx, |ui| {
                        ui.allocate_response(egui::vec2(400.0, 400.0), egui::Sense::click());
                    });
            });
        }
        let menu = ctx
            .memory(|memory| memory.area_rect(egui::Id::new(MENU_ID)))
            .expect("the menu is laid out");

        assert_eq!(
            ctx.layer_id_at(menu.center()).map(|layer| layer.id),
            Some(egui::Id::new(MENU_ID))
        );
    }

    #[test]
    fn the_menu_stays_above_a_window_that_raises_itself_every_frame() {
        // FUN_00442860 makes the Game Menu Window a pop-up over every child
        // window; a focused system window raises itself each frame.
        let layout = layout(1.0);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let window = egui::Id::new("raising_window");
        let entries = [labelled("Mission", true)];
        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 480.0),
                )),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                ctx.move_to_top(egui::LayerId::new(egui::Order::Foreground, window));
                egui::Area::new(window)
                    .order(egui::Order::Foreground)
                    .fixed_pos(egui::Pos2::ZERO)
                    .show(ctx, |ui| {
                        ui.allocate_response(egui::vec2(400.0, 400.0), egui::Sense::click());
                    });
                let _ = draw_game_menu(
                    ctx,
                    egui::Id::new(MENU_ID),
                    &mut cache,
                    layout,
                    CockpitFaction::Alliance,
                    GameMenuPlacement::in_frame((100.0, 100.0)),
                    &entries,
                    true,
                );
            });
        }
        let menu = ctx
            .memory(|memory| memory.area_rect(egui::Id::new(MENU_ID)))
            .expect("the menu is laid out");

        assert_eq!(
            ctx.layer_id_at(menu.center()).map(|layer| layer.id),
            Some(egui::Id::new(MENU_ID))
        );
    }

    fn labelled(label: &'static str, enabled: bool) -> GameMenuEntry<'static> {
        GameMenuEntry {
            label,
            icon: None,
            enabled,
            submenu: false,
            children: &[],
        }
    }

    fn pointer_button(pos: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    /// What one test run saw: the last frame's response, the menu rectangle,
    /// and the text it painted, as (label, top-left corner, size, color).
    struct Run {
        response: GameMenuResponse,
        rect: egui::Rect,
        texts: Vec<(String, egui::Pos2, egui::Vec2, egui::Color32)>,
    }

    fn layout(scale: f32) -> CockpitLayout {
        crate::cockpit::CockpitState::new(CockpitFaction::Alliance)
            .layout_for(640.0 * scale, 480.0 * scale)
    }

    /// Open a menu of `entries` at (100, 100) on a 640 by 480 owner drawn at
    /// `scale`, run two idle frames so the area lays out, then one frame per
    /// entry of `frames` with the pointer at the point `at` returns for the
    /// laid-out menu rectangle.
    fn run(
        scale: f32,
        entries: &[GameMenuEntry<'_>],
        at: impl Fn(egui::Rect) -> egui::Pos2,
        frames: Vec<Vec<egui::Event>>,
    ) -> Run {
        let layout = layout(scale);
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let id = egui::Id::new(MENU_ID);
        let mut response = GameMenuResponse::Open;
        let mut rect = egui::Rect::NOTHING;
        let mut texts = Vec::new();
        for (frame, extra) in [vec![], vec![]].into_iter().chain(frames).enumerate() {
            let pointer = if frame >= 2 {
                rect = ctx
                    .memory(|memory| memory.area_rect(id))
                    .expect("the menu area is laid out");
                at(rect)
            } else {
                egui::Pos2::ZERO
            };
            let mut events = vec![egui::Event::PointerMoved(pointer)];
            events.extend(extra.into_iter().map(|event| match event {
                egui::Event::PointerButton { pressed, .. } => pointer_button(pointer, pressed),
                other => other,
            }));
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0 * scale, 520.0 * scale),
                )),
                events,
                ..Default::default()
            };
            let output = ctx.run(input, |ctx| {
                response = draw_game_menu(
                    ctx,
                    id,
                    &mut cache,
                    layout,
                    CockpitFaction::Alliance,
                    GameMenuPlacement::in_frame((100.0, 100.0)),
                    entries,
                    true,
                );
            });
            texts = output
                .shapes
                .into_iter()
                .filter_map(|clipped| match clipped.shape {
                    egui::Shape::Text(text) => Some((
                        text.galley.text().to_owned(),
                        text.pos,
                        text.galley.size(),
                        text.fallback_color,
                    )),
                    _ => None,
                })
                .collect();
        }
        Run {
            response,
            rect,
            texts,
        }
    }

    fn drive(
        entries: &[GameMenuEntry<'_>],
        at: impl Fn(egui::Rect) -> egui::Pos2,
        frames: Vec<Vec<egui::Event>>,
    ) -> GameMenuResponse {
        run(1.0, entries, at, frames).response
    }

    /// The top and height of row `index` of `count` equal rows drawn at
    /// `scale`: two pixels of top padding, then the rows, as
    /// `game_menu_geometry` lays them out.
    fn row_span(rect: egui::Rect, scale: f32, index: usize, count: usize) -> (f32, f32) {
        let row = (rect.height() - 2.0 * scale) / count as f32;
        (rect.min.y + 2.0 * scale + row * index as f32, row)
    }

    fn row_center(rect: egui::Rect, scale: f32, index: usize, count: usize) -> egui::Pos2 {
        let (top, row) = row_span(rect, scale, index, count);
        egui::pos2(rect.center().x, top + row / 2.0)
    }

    fn click() -> Vec<Vec<egui::Event>> {
        let at = egui::Pos2::ZERO;
        vec![
            vec![pointer_button(at, true)],
            vec![pointer_button(at, false)],
        ]
    }

    #[test]
    fn a_click_on_an_enabled_row_chooses_that_row() {
        // FUN_004424c0: a left release on an item reports its kind to the owner.
        let entries = [
            labelled("Move", true),
            labelled("Mission", true),
            labelled("Encyclopedia", true),
        ];
        for index in 0..entries.len() {
            let response = drive(&entries, |rect| row_center(rect, 1.0, index, 3), click());
            assert_eq!(response, GameMenuResponse::Chosen(index));
        }
    }

    #[test]
    fn a_click_on_a_disabled_row_chooses_nothing_and_keeps_the_menu_open() {
        // FUN_004aba60: a disabled item (flags bit 0) neither highlights nor reports.
        let entries = [labelled("Move", false), labelled("Mission", true)];
        let response = drive(&entries, |rect| row_center(rect, 1.0, 0, 2), click());
        assert_eq!(response, GameMenuResponse::Open);
    }

    #[test]
    fn a_press_outside_the_menu_dismisses_it() {
        // FUN_00442860's window closes on a press outside it.
        let entries = [labelled("Mission", true)];
        let response = drive(
            &entries,
            |rect| rect.max + egui::vec2(20.0, 20.0),
            vec![vec![pointer_button(egui::Pos2::ZERO, true)]],
        );
        assert_eq!(response, GameMenuResponse::Dismissed);
    }

    #[test]
    fn hovering_the_menu_without_a_press_keeps_it_open() {
        let entries = [labelled("Mission", true)];
        let response = drive(&entries, |rect| row_center(rect, 1.0, 0, 1), vec![vec![]]);
        assert_eq!(response, GameMenuResponse::Open);
    }

    #[test]
    fn escape_dismisses_the_menu() {
        // port: Escape closes the menu, as the speed menu's port already did.
        let entries = [labelled("Mission", true)];
        let escape = egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        };
        let response = drive(
            &entries,
            |rect| row_center(rect, 1.0, 0, 1),
            vec![vec![escape]],
        );
        assert_eq!(response, GameMenuResponse::Dismissed);
    }

    #[test]
    fn the_menu_opens_at_the_anchor_and_scales_with_the_owner() {
        // FUN_00442860 places the menu's top-left corner at the click point.
        let entries = [labelled("Move", true), labelled("Mission", true)];
        let small = run(1.0, &entries, |rect| rect.center(), vec![vec![]]);
        let large = run(2.0, &entries, |rect| rect.center(), vec![vec![]]);
        for (scale, seen) in [(1.0, &small), (2.0, &large)] {
            let layout = layout(scale);
            assert_eq!(
                seen.rect.min,
                egui::pos2(
                    layout.canvas.x + 100.0 * layout.scale,
                    layout.canvas.y + 100.0 * layout.scale,
                ),
                "scale {scale}"
            );
        }
        // Heights scale exactly; egui measures text widths at the scaled font
        // size, which rounds differently from twice the 14-pixel width.
        assert_eq!(large.rect.height(), small.rect.height() * 2.0);
        assert!(
            (large.rect.width() - small.rect.width() * 2.0).abs() <= 2.0,
            "{} vs {}",
            large.rect.width(),
            small.rect.width()
        );
    }

    #[test]
    fn rows_are_chosen_by_their_place_at_any_scale() {
        let entries = [
            labelled("Move", true),
            labelled("Mission", true),
            labelled("Encyclopedia", true),
        ];
        // Near each row's top, middle and bottom.
        for index in 0..entries.len() {
            for depth in [0.1, 0.5, 0.9] {
                let seen = run(
                    2.0,
                    &entries,
                    |rect| {
                        let (top, row) = row_span(rect, 2.0, index, 3);
                        egui::pos2(rect.center().x, top + row * depth)
                    },
                    click(),
                );
                assert_eq!(
                    seen.response,
                    GameMenuResponse::Chosen(index),
                    "row {index} at depth {depth}"
                );
            }
        }
    }

    #[test]
    fn text_sits_six_pixels_in_and_centred_in_its_row() {
        // FUN_004aba60: text starts after the icon column plus 6 and is
        // vertically centred in the row.
        for scale in [1.0, 2.0] {
            let entries = [labelled("Move", true), labelled("Mission", true)];
            let seen = run(
                scale,
                &entries,
                |rect| rect.max + egui::vec2(9.0, 9.0),
                vec![vec![]],
            );
            for (index, label) in ["Move", "Mission"].into_iter().enumerate() {
                let (_, pos, size, _) = seen
                    .texts
                    .iter()
                    .find(|(text, ..)| text == label)
                    .unwrap_or_else(|| panic!("{label} is painted"));
                let (top, row) = row_span(seen.rect, scale, index, 2);
                assert_eq!(pos.x, seen.rect.min.x + 6.0 * scale, "{label} at {scale}");
                assert!(
                    (pos.y - (top + (row - size.y) / 2.0)).abs() < 0.01,
                    "{label} at {scale}: top {} in row {top}+{row}, height {}",
                    pos.y,
                    size.y
                );
            }
            // The widest row sets the width: icon column, 12 pixels, text.
            let widest = seen
                .texts
                .iter()
                .map(|(.., size, _)| size.x)
                .fold(0.0, f32::max);
            assert!(
                (seen.rect.width() - (widest + 12.0 * scale)).abs() < 0.01,
                "width {} for text {widest} at {scale}",
                seen.rect.width()
            );
        }
    }

    #[test]
    fn the_hovered_enabled_row_is_painted_in_the_faction_color() {
        // FUN_004aba60: the highlighted item uses +0x2c, the faction color.
        let entries = [
            labelled("Move", false),
            labelled("Mission", true),
            labelled("Status", true),
        ];
        let color = |seen: &Run, label: &str| {
            seen.texts
                .iter()
                .find(|(text, ..)| text == label)
                .map(|(.., color)| *color)
        };
        let hover_mission = run(
            1.0,
            &entries,
            |rect| row_center(rect, 1.0, 1, 3),
            vec![vec![]],
        );
        assert_eq!(
            color(&hover_mission, "Mission"),
            Some(egui::Color32::from_rgb(255, 0, 0))
        );
        assert_eq!(color(&hover_mission, "Status"), Some(egui::Color32::WHITE));
        let hover_move = run(
            1.0,
            &entries,
            |rect| row_center(rect, 1.0, 0, 3),
            vec![vec![]],
        );
        assert_eq!(color(&hover_move, "Move"), Some(DISABLED_COLOR));
        assert_eq!(color(&hover_move, "Mission"), Some(egui::Color32::WHITE));
    }

    #[test]
    fn hovering_outside_without_a_press_keeps_the_menu_open() {
        let entries = [labelled("Mission", true)];
        let response = drive(
            &entries,
            |rect| rect.max + egui::vec2(20.0, 20.0),
            vec![vec![]],
        );
        assert_eq!(response, GameMenuResponse::Open);
    }

    #[test]
    fn items_are_white_highlighted_in_the_faction_color_and_gray_when_disabled() {
        // Source: FUN_004aba60 draws +0x30 normally, +0x2c highlighted and +0x34 disabled;
        // FUN_004abe10 stores (faction, 0x2ffffff, 0x2808080) from FUN_00442590.
        let enabled = entry(true, false);
        assert_eq!(
            entry_text_color(&enabled, false, CockpitFaction::Alliance),
            egui::Color32::WHITE
        );
        assert_eq!(
            entry_text_color(&enabled, true, CockpitFaction::Alliance),
            egui::Color32::from_rgb(255, 0, 0)
        );
        assert_eq!(
            entry_text_color(&enabled, true, CockpitFaction::Empire),
            egui::Color32::from_rgb(0, 255, 0)
        );
        let disabled = entry(false, false);
        for highlighted in [false, true] {
            assert_eq!(
                entry_text_color(&disabled, highlighted, CockpitFaction::Empire),
                egui::Color32::from_rgb(0x80, 0x80, 0x80)
            );
        }
    }

    #[test]
    fn a_submenu_parent_without_an_icon_reserves_twenty_pixels() {
        // Source: FUN_004abf60 returns 0x14 for an item whose flags carry 8.
        assert_eq!(entry_icon_width(&entry(true, true), None), 20.0);
        assert_eq!(entry_icon_width(&entry(true, false), None), 0.0);
        assert_eq!(entry_icon_width(&entry(true, true), Some(16.0)), 16.0);
    }

    #[test]
    fn menu_rows_follow_native_width_and_height_rules() {
        let icons = [(16.0, 10.0); 5];
        let texts = [
            (30.0, 16.0),
            (55.0, 16.0),
            (24.0, 16.0),
            (44.0, 16.0),
            (26.0, 16.0),
        ];
        let geometry = game_menu_geometry(&icons, &texts);
        assert_eq!(geometry.icon_column, 16.0);
        // Widest row: 16 + 12 + 55.
        assert_eq!(geometry.width, 83.0);
        assert_eq!(
            geometry.rows,
            vec![
                (2.0, 20.0),
                (22.0, 20.0),
                (42.0, 20.0),
                (62.0, 20.0),
                (82.0, 20.0)
            ]
        );
        assert_eq!(geometry.height, 102.0);
    }

    #[test]
    fn short_text_rows_keep_icon_height_plus_padding() {
        let geometry = game_menu_geometry(&[(16.0, 10.0)], &[(10.0, 8.0)]);
        assert_eq!(geometry.rows, vec![(2.0, 14.0)]);
    }

    #[test]
    fn menu_touching_an_owner_edge_does_not_flip() {
        // FUN_00442860 flips only when the menu strictly crosses the edge.
        let owner = (640.0, 480.0);
        assert_eq!(
            game_menu_origin((557.0, 378.0), (83.0, 102.0), owner),
            (557.0, 378.0)
        );
        assert_eq!(
            game_menu_origin((558.0, 379.0), (83.0, 102.0), owner),
            (475.0, 277.0)
        );
    }

    #[test]
    fn menu_left_of_the_owner_moves_right_by_its_width() {
        // FUN_00442860: a negative x gains the menu width; zero stays put.
        let owner = (640.0, 480.0);
        assert_eq!(
            game_menu_origin((-10.0, 26.0), (83.0, 102.0), owner),
            (73.0, 26.0)
        );
        assert_eq!(
            game_menu_origin((0.0, 26.0), (83.0, 102.0), owner),
            (0.0, 26.0)
        );
    }

    #[test]
    fn a_menu_flips_at_its_owner_window_not_the_canvas() {
        // FUN_004ac5c0 maps the point into the galaxy view, which owns the
        // menu (FUN_00442380); FUN_00442860 flips at the owner's edges.
        let galaxy = GameMenuPlacement {
            owner: CockpitViewport {
                x: 55.0,
                y: 40.0,
                width: 485.0,
                height: 350.0,
            },
            point: (500.0, 360.0),
        };
        assert_eq!(galaxy.origin((83.0, 102.0)), (417.0, 258.0));
        let inside = GameMenuPlacement {
            point: (120.0, 60.0),
            ..galaxy
        };
        assert_eq!(inside.origin((83.0, 102.0)), (120.0, 60.0));
        // The same point in the frame does not cross its edges.
        assert_eq!(
            GameMenuPlacement::in_frame((500.0, 360.0)).origin((83.0, 102.0)),
            (500.0, 360.0)
        );
    }

    #[test]
    fn menu_flips_away_from_right_and_bottom_owner_edges() {
        let owner = (640.0, 480.0);
        assert_eq!(
            game_menu_origin((120.0, 26.0), (83.0, 102.0), owner),
            (120.0, 26.0)
        );
        assert_eq!(
            game_menu_origin((600.0, 26.0), (83.0, 102.0), owner),
            (517.0, 26.0)
        );
        assert_eq!(
            game_menu_origin((120.0, 420.0), (83.0, 102.0), owner),
            (120.0, 318.0)
        );
    }
}
