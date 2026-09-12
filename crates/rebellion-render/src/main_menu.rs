//! Original 640x480 shuttle-cockpit main menu.
//!
//! `COMMON.DLL` bitmap 20001 contains only the empty cockpit shell. The
//! original game assembles the controls below from separate bitmap resources;
//! their geometry and command mapping are recovered in
//! `agent_docs/main-menu-parity.md`.

use egui_macroquad::egui::{
    self, Color32, FontFamily, FontId, Pos2, Rect, Sense, Shape, Stroke, Vec2, WidgetInfo,
    WidgetType,
};
use rebellion_core::dat::GalaxySize;
use rebellion_core::missions::MissionFaction;

use crate::audio::SfxKind;
use crate::bmp_cache::{resources, BmpCache, DllSource};
use crate::panels::game_setup::Difficulty;

pub const LOGICAL_WIDTH: f32 = 640.0;
pub const LOGICAL_HEIGHT: f32 = 480.0;
pub const ORIGINAL_CONTROL_COUNT: usize = 14;
pub const MUSIC_TOGGLE_RECT: LogicalRect = LogicalRect::new(594.0, 10.0, 30.0, 22.0);
const ANIMATION_FPS: f64 = 15.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MainMenuControl {
    Easy,
    Intermediate,
    Expert,
    GalaxyLever,
    SmallGalaxy,
    MediumGalaxy,
    LargeGalaxy,
    GameType,
    Empire,
    Alliance,
    LoadOptions,
    Credits,
    Multiplayer,
    Quit,
    /// Open Rebellion convenience extension; not present in the 1998 cockpit.
    MusicToggle,
}

impl MainMenuControl {
    /// Stable DOM/keyboard order shared with the browser accessibility bridge.
    pub const fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(index: u32) -> Option<Self> {
        CONTROL_RECTS
            .get(index as usize)
            .map(|(control, _)| *control)
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Easy => "Easy difficulty, X-wing",
            Self::Intermediate => "Intermediate difficulty, Star Destroyer",
            Self::Expert => "Expert difficulty, Death Star",
            Self::GalaxyLever => "Cycle galaxy size",
            Self::SmallGalaxy => "Small galaxy",
            Self::MediumGalaxy => "Medium galaxy",
            Self::LargeGalaxy => "Large galaxy",
            Self::GameType => "Toggle Standard or Headquarters Only game",
            Self::Empire => "Start as the Galactic Empire",
            Self::Alliance => "Start as the Rebel Alliance",
            Self::LoadOptions => "Load game and options",
            Self::Credits => "Credits",
            Self::Multiplayer => "Multiplayer",
            Self::Quit => "Quit",
            Self::MusicToggle => "Menu music",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogicalRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl LogicalRect {
    const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn contains(self, point: Pos2) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

pub const CONTROL_RECTS: &[(MainMenuControl, LogicalRect)] = &[
    (
        MainMenuControl::Easy,
        LogicalRect::new(61.0, 41.0, 51.0, 36.0),
    ),
    (
        MainMenuControl::Intermediate,
        LogicalRect::new(124.0, 40.0, 49.0, 36.0),
    ),
    (
        MainMenuControl::Expert,
        LogicalRect::new(187.0, 41.0, 45.0, 36.0),
    ),
    (
        MainMenuControl::GalaxyLever,
        LogicalRect::new(242.0, 271.0, 44.0, 47.0),
    ),
    (
        MainMenuControl::SmallGalaxy,
        LogicalRect::new(290.0, 293.0, 24.0, 21.0),
    ),
    (
        MainMenuControl::MediumGalaxy,
        LogicalRect::new(326.0, 293.0, 24.0, 21.0),
    ),
    (
        MainMenuControl::LargeGalaxy,
        LogicalRect::new(362.0, 293.0, 24.0, 21.0),
    ),
    (
        MainMenuControl::GameType,
        LogicalRect::new(305.0, 333.0, 42.0, 30.0),
    ),
    (
        MainMenuControl::Empire,
        LogicalRect::new(153.0, 308.0, 62.0, 55.0),
    ),
    (
        MainMenuControl::Alliance,
        LogicalRect::new(437.0, 307.0, 62.0, 55.0),
    ),
    (
        MainMenuControl::LoadOptions,
        LogicalRect::new(411.0, 232.0, 40.0, 37.0),
    ),
    (
        MainMenuControl::Credits,
        LogicalRect::new(459.0, 242.0, 33.0, 28.0),
    ),
    (
        MainMenuControl::Multiplayer,
        LogicalRect::new(67.0, 381.0, 51.0, 61.0),
    ),
    (
        MainMenuControl::Quit,
        LogicalRect::new(536.0, 393.0, 63.0, 64.0),
    ),
    (MainMenuControl::MusicToggle, MUSIC_TOGGLE_RECT),
];

/// Persistent selections and animation state for the cockpit menu.
#[derive(Debug, Clone)]
pub struct MainMenuState {
    pub difficulty: Difficulty,
    pub galaxy_size: GalaxySize,
    pub headquarters_only: bool,
    hovered: Option<MainMenuControl>,
    hover_started_at: f64,
    keyboard_focus: Option<MainMenuControl>,
    semantic_focus: Option<MainMenuControl>,
    pending_sfx: Option<SfxKind>,
}

impl Default for MainMenuState {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Easy,
            galaxy_size: GalaxySize::Standard,
            headquarters_only: false,
            hovered: None,
            hover_started_at: 0.0,
            keyboard_focus: None,
            semantic_focus: None,
            pending_sfx: None,
        }
    }
}

impl MainMenuState {
    /// Take the sound assigned by the original COMMON.DLL control constructor.
    pub fn take_sfx(&mut self) -> Option<SfxKind> {
        self.pending_sfx.take()
    }

    /// Mirror DOM focus onto the authentic bitmap control without adding a
    /// visible replacement widget.
    pub fn set_semantic_focus(&mut self, control: Option<MainMenuControl>) {
        self.semantic_focus = control;
        if let Some(control) = control {
            self.keyboard_focus = Some(control);
        }
    }

    pub fn semantic_focus(&self) -> Option<MainMenuControl> {
        self.semantic_focus
    }

    /// Route assistive-technology activation through the exact pointer and
    /// canvas-keyboard behavior, including the recovered COMMON.DLL effect.
    pub fn activate_control(&mut self, control: MainMenuControl) -> Option<MainMenuAction> {
        self.pending_sfx = Some(sfx_for(control));
        activate(control, self)
    }
}

/// Actions produced by the original cockpit controls.
#[derive(Debug, Clone, PartialEq)]
pub enum MainMenuAction {
    StartGame {
        difficulty: Difficulty,
        faction: MissionFaction,
        galaxy_size: GalaxySize,
        headquarters_only: bool,
    },
    LoadGame,
    Credits,
    Multiplayer,
    Quit,
    ToggleMusic,
}

pub fn main_menu_canvas_rect(viewport: Rect) -> Rect {
    let scale = (viewport.width() / LOGICAL_WIDTH)
        .min(viewport.height() / LOGICAL_HEIGHT)
        .max(0.0);
    let size = Vec2::new(LOGICAL_WIDTH * scale, LOGICAL_HEIGHT * scale);
    Rect::from_center_size(viewport.center(), size)
}

pub fn control_rect(canvas: Rect, logical: LogicalRect) -> Rect {
    let scale = canvas.width() / LOGICAL_WIDTH;
    Rect::from_min_size(
        canvas.min + Vec2::new(logical.x * scale, logical.y * scale),
        Vec2::new(logical.width * scale, logical.height * scale),
    )
}

/// Device-pixel-aligned bounds shared by the extension's paint and hit paths.
pub fn music_toggle_rect(canvas: Rect) -> Rect {
    let rect = control_rect(canvas, MUSIC_TOGGLE_RECT);
    Rect::from_min_max(
        Pos2::new(rect.min.x.round(), rect.min.y.round()),
        Pos2::new(rect.max.x.round(), rect.max.y.round()),
    )
}

fn logical_pointer(canvas: Rect, pointer: Pos2) -> Option<Pos2> {
    if !canvas.contains(pointer) || canvas.width() <= 0.0 {
        return None;
    }
    let scale = canvas.width() / LOGICAL_WIDTH;
    Some(Pos2::new(
        (pointer.x - canvas.min.x) / scale,
        (pointer.y - canvas.min.y) / scale,
    ))
}

pub fn hit_test(canvas: Rect, pointer: Pos2) -> Option<MainMenuControl> {
    if music_toggle_rect(canvas).contains(pointer) {
        return Some(MainMenuControl::MusicToggle);
    }
    let logical = logical_pointer(canvas, pointer)?;
    CONTROL_RECTS[..ORIGINAL_CONTROL_COUNT]
        .iter()
        .find_map(|(control, rect)| rect.contains(logical).then_some(*control))
}

fn animation_resource(start: u32, count: u32, elapsed: f64) -> u32 {
    start + ((elapsed * ANIMATION_FPS) as u32 % count)
}

fn texture_for(
    control: MainMenuControl,
    state: &MainMenuState,
    hovered: bool,
    elapsed: f64,
) -> u32 {
    match control {
        MainMenuControl::Easy => {
            if state.difficulty == Difficulty::Easy {
                11273
            } else if hovered {
                animation_resource(11061, 30, elapsed)
            } else {
                11061
            }
        }
        MainMenuControl::Intermediate => {
            if state.difficulty == Difficulty::Medium {
                11275
            } else if hovered {
                animation_resource(11091, 30, elapsed)
            } else {
                11091
            }
        }
        MainMenuControl::Expert => {
            if state.difficulty == Difficulty::Hard {
                11274
            } else if hovered {
                animation_resource(11121, 30, elapsed)
            } else {
                11121
            }
        }
        MainMenuControl::GalaxyLever => match state.galaxy_size {
            GalaxySize::Standard => 10001,
            GalaxySize::Large => 10002,
            GalaxySize::Huge => 10003,
        },
        MainMenuControl::SmallGalaxy => 10019,
        MainMenuControl::MediumGalaxy => 10018,
        MainMenuControl::LargeGalaxy => 10017,
        MainMenuControl::GameType => {
            if state.headquarters_only {
                10159
            } else {
                10158
            }
        }
        MainMenuControl::Empire => {
            if hovered {
                animation_resource(11001, 15, elapsed)
            } else {
                10009
            }
        }
        MainMenuControl::Alliance => {
            if hovered {
                animation_resource(11031, 15, elapsed)
            } else {
                10007
            }
        }
        MainMenuControl::Multiplayer => {
            if hovered {
                animation_resource(11151, 30, elapsed)
            } else {
                10005
            }
        }
        MainMenuControl::LoadOptions => {
            if hovered {
                animation_resource(11241, 15, elapsed)
            } else {
                10013
            }
        }
        MainMenuControl::Credits => {
            if hovered && ((elapsed * 5.0) as u32 % 2 == 1) {
                11272
            } else {
                11271
            }
        }
        MainMenuControl::Quit => {
            if hovered {
                animation_resource(11181, 30, elapsed)
            } else {
                10011
            }
        }
        MainMenuControl::MusicToggle => {
            unreachable!("the Open Rebellion music toggle uses vector cockpit chrome")
        }
    }
}

fn draw_texture(
    painter: &egui::Painter,
    cache: &mut BmpCache,
    ctx: &egui::Context,
    resource_id: u32,
    rect: Rect,
) {
    if let Some(texture) = cache.get(ctx, DllSource::Common, resource_id) {
        painter.image(
            texture.id(),
            rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
    }
}

fn activate(control: MainMenuControl, state: &mut MainMenuState) -> Option<MainMenuAction> {
    match control {
        MainMenuControl::Easy => state.difficulty = Difficulty::Easy,
        MainMenuControl::Intermediate => state.difficulty = Difficulty::Medium,
        MainMenuControl::Expert => state.difficulty = Difficulty::Hard,
        MainMenuControl::GalaxyLever => {
            state.galaxy_size = match state.galaxy_size {
                GalaxySize::Standard => GalaxySize::Large,
                GalaxySize::Large => GalaxySize::Huge,
                GalaxySize::Huge => GalaxySize::Standard,
            }
        }
        MainMenuControl::SmallGalaxy => state.galaxy_size = GalaxySize::Standard,
        MainMenuControl::MediumGalaxy => state.galaxy_size = GalaxySize::Large,
        MainMenuControl::LargeGalaxy => state.galaxy_size = GalaxySize::Huge,
        MainMenuControl::GameType => state.headquarters_only = !state.headquarters_only,
        MainMenuControl::Empire | MainMenuControl::Alliance => {
            return Some(MainMenuAction::StartGame {
                difficulty: state.difficulty,
                faction: if control == MainMenuControl::Alliance {
                    MissionFaction::Alliance
                } else {
                    MissionFaction::Empire
                },
                galaxy_size: state.galaxy_size,
                headquarters_only: state.headquarters_only,
            });
        }
        MainMenuControl::LoadOptions => return Some(MainMenuAction::LoadGame),
        MainMenuControl::Credits => return Some(MainMenuAction::Credits),
        MainMenuControl::Multiplayer => return Some(MainMenuAction::Multiplayer),
        MainMenuControl::Quit => return Some(MainMenuAction::Quit),
        MainMenuControl::MusicToggle => return Some(MainMenuAction::ToggleMusic),
    }
    None
}

fn sfx_for(control: MainMenuControl) -> SfxKind {
    match control {
        MainMenuControl::GalaxyLever
        | MainMenuControl::SmallGalaxy
        | MainMenuControl::MediumGalaxy
        | MainMenuControl::LargeGalaxy => SfxKind::MenuGalaxySize,
        MainMenuControl::Multiplayer => SfxKind::MenuLoadOptions,
        MainMenuControl::Quit => SfxKind::MenuQuit,
        _ => SfxKind::MenuSelect,
    }
}

fn adjacent_control(current: Option<MainMenuControl>, backwards: bool) -> MainMenuControl {
    let len = CONTROL_RECTS.len();
    let index = current
        .and_then(|focused| {
            CONTROL_RECTS
                .iter()
                .position(|(control, _)| *control == focused)
        })
        .unwrap_or(if backwards { 0 } else { len - 1 });
    let next = if backwards {
        (index + len - 1) % len
    } else {
        (index + 1) % len
    };
    CONTROL_RECTS[next].0
}

fn draw_music_toggle(
    painter: &egui::Painter,
    rect: Rect,
    music_enabled: bool,
    hovered: bool,
    pressed: bool,
    hover_elapsed: f64,
) {
    // Egui centers line strokes on their paths. Clip the complete device to
    // the shared interaction rect so sub-native scales cannot paint a bevel
    // pixel outside the clickable housing.
    let painter = painter.with_clip_rect(rect);
    let scale = rect.width() / 30.0;
    let stroke = scale.max(1.0);
    let top_left = Color32::from_rgb(214, 226, 222);
    let face = Color32::from_rgb(166, 180, 176);
    let bottom_right = Color32::from_rgb(92, 104, 104);
    let lip = Color32::from_rgb(24, 24, 32);
    painter.rect_filled(rect, scale, face);
    painter.add(Shape::line(
        vec![rect.left_bottom(), rect.left_top(), rect.right_top()],
        Stroke::new(stroke, top_left),
    ));
    painter.add(Shape::line(
        vec![rect.left_bottom(), rect.right_bottom(), rect.right_top()],
        Stroke::new(stroke, bottom_right),
    ));
    let aperture = rect.shrink(3.0 * scale);
    painter.rect_filled(aperture, 0.5 * scale, Color32::from_rgb(8, 10, 14));
    painter.rect_stroke(
        aperture,
        0.5 * scale,
        Stroke::new(stroke, lip),
        egui::StrokeKind::Inside,
    );

    let pressed_offset = if pressed { scale } else { 0.0 };
    let center_y = rect.center().y + pressed_offset;
    let core_alpha = if pressed {
        90
    } else if !music_enabled {
        110
    } else if hovered && ((hover_elapsed * 5.0) as u32 % 2 == 1) {
        160
    } else {
        230
    };
    let icon = Color32::from_rgba_unmultiplied(176, 226, 240, core_alpha);
    let bloom = Color32::from_rgba_unmultiplied(72, 168, 216, 64);
    let origin_x = rect.left() + pressed_offset;
    let speaker_body = Rect::from_min_max(
        Pos2::new(origin_x + 5.0 * scale, center_y - 2.5 * scale),
        Pos2::new(origin_x + 9.0 * scale, center_y + 2.5 * scale),
    );
    let speaker_cone = vec![
        Pos2::new(origin_x + 9.0 * scale, center_y - 2.5 * scale),
        Pos2::new(origin_x + 14.0 * scale, center_y - 6.0 * scale),
        Pos2::new(origin_x + 14.0 * scale, center_y + 6.0 * scale),
        Pos2::new(origin_x + 9.0 * scale, center_y + 2.5 * scale),
    ];
    painter.rect_filled(speaker_body.expand(scale), scale, bloom);
    for dx in [-0.75, 0.75] {
        painter.add(Shape::convex_polygon(
            speaker_cone
                .iter()
                .map(|point| Pos2::new(point.x + dx * scale, point.y))
                .collect(),
            bloom,
            Stroke::NONE,
        ));
    }
    painter.rect_filled(speaker_body, 0.5 * scale, icon);
    painter.add(Shape::convex_polygon(speaker_cone, icon, Stroke::NONE));
    if music_enabled {
        for (x, height) in [(16.0, 3.5), (19.5, 5.5)] {
            let points = vec![
                Pos2::new(origin_x + x * scale, center_y - height * scale),
                Pos2::new(origin_x + (x + 2.0) * scale, center_y),
                Pos2::new(origin_x + x * scale, center_y + height * scale),
            ];
            painter.add(Shape::line(
                points.clone(),
                Stroke::new((3.0 * scale).max(1.0), bloom),
            ));
            painter.add(Shape::line(
                points,
                Stroke::new((1.5 * scale).max(1.0), icon),
            ));
        }
    } else {
        painter.line_segment(
            [
                Pos2::new(origin_x + 5.0 * scale, center_y + 6.0 * scale),
                Pos2::new(origin_x + 22.0 * scale, center_y - 6.0 * scale),
            ],
            Stroke::new((1.5 * scale).max(1.0), icon),
        );
    }

    let scan_phase = if hovered && ((hover_elapsed * 5.0) as u32 % 2 == 1) {
        1.0
    } else {
        0.0
    };
    for line in -2..=2 {
        let dy = (line as f32 * 2.0 + scan_phase) * scale;
        let abs_y = (line as f32 * 2.0 + scan_phase).abs();
        let left = if abs_y <= 2.5 {
            origin_x + 5.0 * scale
        } else {
            origin_x + (9.0 + (abs_y - 2.5) * (5.0 / 3.5)) * scale
        };
        painter.line_segment(
            [
                Pos2::new(left, center_y + dy),
                Pos2::new(origin_x + 14.0 * scale, center_y + dy),
            ],
            Stroke::new(stroke, Color32::from_black_alpha(96)),
        );
    }

    // Keep the state lamp wholly inside the dark aperture. Sitting it on the
    // lower bevel made the final device pixel look clipped at some scales.
    let led_surround = Rect::from_min_size(
        Pos2::new(rect.right() - 9.0 * scale, rect.top() + 3.0 * scale),
        Vec2::new(6.0 * scale, 5.0 * scale),
    );
    painter.rect_filled(led_surround, 0.0, Color32::from_rgb(24, 30, 32));
    let led = led_surround.shrink(scale);
    painter.rect_filled(
        led,
        0.0,
        if pressed {
            Color32::from_rgb(32, 38, 38)
        } else if music_enabled {
            Color32::from_rgb(72, 232, 96)
        } else {
            Color32::from_rgb(232, 92, 72)
        },
    );
}

/// Draw the assembled cockpit and return an action when a control activates.
pub fn draw_main_menu(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    state: &mut MainMenuState,
    music_enabled: bool,
) -> Option<MainMenuAction> {
    let mut action = None;
    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(Color32::BLACK))
        .show(ctx, |ui| {
            let canvas = main_menu_canvas_rect(ui.max_rect());
            let now = ctx.input(|input| input.time);
            // egui owns Tab/Shift+Tab traversal for these focusable controls.
            // Handling Tab here as well advances twice in the browser.
            let focus_direction = ui.input(|input| {
                if input.key_pressed(egui::Key::ArrowLeft) || input.key_pressed(egui::Key::ArrowUp)
                {
                    Some(true)
                } else if input.key_pressed(egui::Key::ArrowRight)
                    || input.key_pressed(egui::Key::ArrowDown)
                {
                    Some(false)
                } else {
                    None
                }
            });
            if let Some(backwards) = focus_direction {
                let focused = adjacent_control(state.keyboard_focus, backwards);
                state.keyboard_focus = Some(focused);
                ui.memory_mut(|memory| memory.request_focus(ui.id().with(focused as u8)));
            }
            let hovered = ctx
                .input(|input| input.pointer.hover_pos())
                .and_then(|pointer| hit_test(canvas, pointer));
            if hovered != state.hovered {
                state.hovered = hovered;
                state.hover_started_at = now;
            }
            let hover_elapsed = (now - state.hover_started_at).max(0.0);

            if let Some(background) =
                cache.get(ctx, DllSource::Common, resources::common::MAIN_MENU_BG)
            {
                ui.painter().image(
                    background.id(),
                    canvas,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );
            }

            let galaxy_indicator = match state.galaxy_size {
                GalaxySize::Standard => MainMenuControl::SmallGalaxy,
                GalaxySize::Large => MainMenuControl::MediumGalaxy,
                GalaxySize::Huge => MainMenuControl::LargeGalaxy,
            };

            for (control, logical) in CONTROL_RECTS {
                // Only the selected galaxy screen receives the original
                // highlight overlay; the three galaxy images are in 20001.
                let draw_control = !matches!(
                    control,
                    MainMenuControl::SmallGalaxy
                        | MainMenuControl::MediumGalaxy
                        | MainMenuControl::LargeGalaxy
                ) || *control == galaxy_indicator;
                let rect = if *control == MainMenuControl::MusicToggle {
                    music_toggle_rect(canvas)
                } else {
                    control_rect(canvas, *logical)
                };
                let response = ui.interact(rect, ui.id().with(*control as u8), Sense::click());
                response.widget_info(|| {
                    if *control == MainMenuControl::MusicToggle {
                        WidgetInfo::selected(
                            WidgetType::Button,
                            true,
                            music_enabled,
                            control.label(),
                        )
                    } else {
                        WidgetInfo::labeled(WidgetType::Button, true, control.label())
                    }
                });
                let keyboard_activation = response.has_focus()
                    && ui.input(|input| {
                        input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
                    });

                if response.has_focus() {
                    state.keyboard_focus = Some(*control);
                }

                if *control == MainMenuControl::MusicToggle {
                    draw_music_toggle(
                        ui.painter(),
                        rect,
                        music_enabled,
                        state.hovered == Some(*control),
                        response.is_pointer_button_down_on(),
                        hover_elapsed,
                    );
                } else if draw_control {
                    let resource_id = texture_for(
                        *control,
                        state,
                        state.hovered == Some(*control),
                        hover_elapsed,
                    );
                    draw_texture(ui.painter(), cache, ctx, resource_id, rect);
                }

                let focus_visible = state
                    .semantic_focus
                    .map_or_else(|| response.has_focus(), |focused| focused == *control);
                if focus_visible {
                    ui.painter().rect_stroke(
                        rect.expand(2.0),
                        1.0,
                        Stroke::new((canvas.width() / LOGICAL_WIDTH).max(1.0), Color32::GOLD),
                        egui::StrokeKind::Outside,
                    );
                }
                // egui expands clickable widgets by its interaction radius. The
                // original menu only activated inside its Win32 control rect.
                let exact_pointer_activation =
                    response.clicked() && state.hovered == Some(*control);
                if exact_pointer_activation || keyboard_activation {
                    action = state.activate_control(*control);
                }
            }

            let label_rect = control_rect(canvas, LogicalRect::new(271.0, 374.0, 110.0, 14.0));
            let scale = canvas.width() / LOGICAL_WIDTH;
            ui.painter().text(
                label_rect.center(),
                egui::Align2::CENTER_CENTER,
                if state.headquarters_only {
                    "Headquarters Only"
                } else {
                    "Standard Game"
                },
                FontId::new((10.0 * scale).max(8.0), FontFamily::Monospace),
                Color32::from_rgb(80, 255, 80),
            );
        });
    action
}

#[cfg(test)]
mod tests {
    use super::*;

    fn viewport(width: f32, height: f32) -> Rect {
        Rect::from_min_size(Pos2::ZERO, Vec2::new(width, height))
    }

    #[test]
    fn preserves_four_by_three_and_centers_letterbox() {
        let wide = main_menu_canvas_rect(viewport(1280.0, 800.0));
        assert!((wide.width() - 1066.6666).abs() < 0.001);
        assert_eq!(wide.height(), 800.0);
        assert!((wide.min.x - 106.6667).abs() < 0.001);
        assert_eq!(wide.min.y, 0.0);

        let tall = main_menu_canvas_rect(viewport(640.0, 600.0));
        assert_eq!(tall.size(), Vec2::new(640.0, 480.0));
        assert_eq!(tall.min, Pos2::new(0.0, 60.0));
    }

    #[test]
    fn transformed_hit_testing_matches_original_regions() {
        let canvas = main_menu_canvas_rect(viewport(1280.0, 960.0));
        assert_eq!(
            hit_test(canvas, Pos2::new(122.0, 82.0)),
            Some(MainMenuControl::Easy)
        );
        assert_eq!(
            hit_test(canvas, Pos2::new(936.0, 670.0)),
            Some(MainMenuControl::Alliance)
        );
        assert_eq!(hit_test(canvas, Pos2::new(5.0, 5.0)), None);
    }

    #[test]
    fn cockpit_destination_hotspots_match_their_artwork() {
        for scale in [1.0, 2.0] {
            let canvas = main_menu_canvas_rect(viewport(640.0 * scale, 480.0 * scale));
            for (point, control, action, bitmap) in [
                (
                    Pos2::new(92.0, 411.0),
                    MainMenuControl::Multiplayer,
                    MainMenuAction::Multiplayer,
                    10005,
                ),
                (
                    Pos2::new(431.0, 250.0),
                    MainMenuControl::LoadOptions,
                    MainMenuAction::LoadGame,
                    10013,
                ),
                (
                    Pos2::new(475.0, 256.0),
                    MainMenuControl::Credits,
                    MainMenuAction::Credits,
                    11271,
                ),
            ] {
                let hit = hit_test(canvas, Pos2::new(point.x * scale, point.y * scale));
                assert_eq!(hit, Some(control));
                let mut state = MainMenuState::default();
                assert_eq!(state.activate_control(hit.unwrap()), Some(action));
                assert_eq!(texture_for(control, &state, false, 0.0), bitmap);
                assert_eq!(
                    MainMenuControl::from_index(control.index() as u32),
                    Some(control)
                );
            }
        }
    }

    #[test]
    fn expert_hit_region_does_not_leak_into_adjacent_pixels() {
        let canvas = Rect::from_min_size(Pos2::ZERO, Vec2::new(640.0, 480.0));

        assert_eq!(hit_test(canvas, Pos2::new(186.9, 59.0)), None);
        assert_eq!(
            hit_test(canvas, Pos2::new(187.0, 59.0)),
            Some(MainMenuControl::Expert)
        );
        assert_eq!(
            hit_test(canvas, Pos2::new(232.0, 59.0)),
            Some(MainMenuControl::Expert)
        );
        assert_eq!(hit_test(canvas, Pos2::new(232.1, 59.0)), None);
    }

    #[test]
    fn original_defaults_and_control_transitions_are_stable() {
        let mut state = MainMenuState::default();
        assert_eq!(state.difficulty, Difficulty::Easy);
        assert_eq!(state.galaxy_size, GalaxySize::Standard);
        assert!(!state.headquarters_only);

        activate(MainMenuControl::GalaxyLever, &mut state);
        assert_eq!(state.galaxy_size, GalaxySize::Large);
        activate(MainMenuControl::Expert, &mut state);
        activate(MainMenuControl::GameType, &mut state);
        assert_eq!(state.difficulty, Difficulty::Hard);
        assert!(state.headquarters_only);

        assert_eq!(
            activate(MainMenuControl::Empire, &mut state),
            Some(MainMenuAction::StartGame {
                difficulty: Difficulty::Hard,
                faction: MissionFaction::Empire,
                galaxy_size: GalaxySize::Large,
                headquarters_only: true,
            })
        );
    }

    #[test]
    fn resource_families_match_binary_mapping() {
        let state = MainMenuState::default();
        assert_eq!(
            texture_for(MainMenuControl::Easy, &state, false, 0.0),
            11273
        );
        assert_eq!(
            texture_for(MainMenuControl::Empire, &state, false, 0.0),
            10009
        );
        assert_eq!(
            texture_for(MainMenuControl::Alliance, &state, true, 0.0),
            11031
        );
        assert_eq!(
            texture_for(MainMenuControl::SmallGalaxy, &state, false, 0.0),
            10019
        );
        assert_eq!(
            texture_for(MainMenuControl::MediumGalaxy, &state, false, 0.0),
            10018
        );
        assert_eq!(
            texture_for(MainMenuControl::LargeGalaxy, &state, false, 0.0),
            10017
        );
        assert_eq!(texture_for(MainMenuControl::Quit, &state, true, 1.0), 11196);
    }

    #[test]
    fn keyboard_focus_wraps_through_every_original_control() {
        assert_eq!(adjacent_control(None, false), MainMenuControl::Easy);
        assert_eq!(
            adjacent_control(Some(MainMenuControl::Easy), true),
            MainMenuControl::MusicToggle
        );
        assert_eq!(
            adjacent_control(Some(MainMenuControl::Quit), false),
            MainMenuControl::MusicToggle
        );
        assert_eq!(
            adjacent_control(Some(MainMenuControl::MusicToggle), false),
            MainMenuControl::Easy
        );
        assert_eq!(
            adjacent_control(Some(MainMenuControl::Credits), false),
            MainMenuControl::Multiplayer
        );
    }

    #[test]
    fn semantic_indices_cover_original_controls_and_music_extension() {
        assert_eq!(ORIGINAL_CONTROL_COUNT, 14);
        assert_eq!(CONTROL_RECTS.len(), ORIGINAL_CONTROL_COUNT + 1);
        for (index, (control, _)) in CONTROL_RECTS.iter().enumerate() {
            assert_eq!(control.index(), index);
            assert_eq!(MainMenuControl::from_index(index as u32), Some(*control));
            assert!(!control.label().is_empty());
        }
        assert_eq!(
            CONTROL_RECTS[ORIGINAL_CONTROL_COUNT].0,
            MainMenuControl::MusicToggle
        );
        assert_eq!(MainMenuControl::from_index(15), None);
    }

    #[test]
    fn semantic_activation_uses_original_state_and_sound_path() {
        let mut state = MainMenuState::default();
        state.set_semantic_focus(Some(MainMenuControl::Expert));
        assert_eq!(state.semantic_focus(), Some(MainMenuControl::Expert));
        assert_eq!(state.activate_control(MainMenuControl::Expert), None);
        assert_eq!(state.difficulty, Difficulty::Hard);
        assert_eq!(state.take_sfx(), Some(SfxKind::MenuSelect));

        assert_eq!(
            state.activate_control(MainMenuControl::LoadOptions),
            Some(MainMenuAction::LoadGame)
        );
        assert_eq!(state.take_sfx(), Some(SfxKind::MenuSelect));
        state.set_semantic_focus(None);
        assert_eq!(state.semantic_focus(), None);

        assert_eq!(
            state.activate_control(MainMenuControl::MusicToggle),
            Some(MainMenuAction::ToggleMusic)
        );
        assert_eq!(state.take_sfx(), Some(SfxKind::MenuSelect));
    }

    #[test]
    fn music_extension_occupies_only_its_top_right_region() {
        let canvas = Rect::from_min_size(Pos2::ZERO, Vec2::new(640.0, 480.0));
        assert_eq!(MUSIC_TOGGLE_RECT.width, 30.0);
        assert_eq!(MUSIC_TOGGLE_RECT.height, 22.0);
        assert_eq!(
            hit_test(canvas, Pos2::new(609.0, 25.0)),
            Some(MainMenuControl::MusicToggle)
        );
        assert_eq!(hit_test(canvas, Pos2::new(593.9, 25.0)), None);
        assert_eq!(hit_test(canvas, Pos2::new(624.1, 25.0)), None);
        assert_eq!(hit_test(canvas, Pos2::new(609.0, 9.9)), None);
        assert_eq!(hit_test(canvas, Pos2::new(609.0, 32.1)), None);

        for (_, original) in &CONTROL_RECTS[..ORIGINAL_CONTROL_COUNT] {
            let separated = MUSIC_TOGGLE_RECT.x + MUSIC_TOGGLE_RECT.width < original.x
                || original.x + original.width < MUSIC_TOGGLE_RECT.x
                || MUSIC_TOGGLE_RECT.y + MUSIC_TOGGLE_RECT.height < original.y
                || original.y + original.height < MUSIC_TOGGLE_RECT.y;
            assert!(separated, "music extension overlaps an original control");
        }

        let scaled = control_rect(
            Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 960.0)),
            MUSIC_TOGGLE_RECT,
        );
        assert_eq!(scaled.width(), 60.0);
        assert_eq!(scaled.height(), 44.0);

        let narrow = control_rect(
            Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0)),
            MUSIC_TOGGLE_RECT,
        );
        assert_eq!(narrow.width(), 15.0);
        assert_eq!(narrow.height(), 11.0);
    }

    #[test]
    fn sound_assignments_match_common_dll_constructor() {
        assert_eq!(
            sfx_for(MainMenuControl::GalaxyLever),
            SfxKind::MenuGalaxySize
        );
        assert_eq!(
            sfx_for(MainMenuControl::SmallGalaxy),
            SfxKind::MenuGalaxySize
        );
        assert_eq!(
            sfx_for(MainMenuControl::Multiplayer),
            SfxKind::MenuLoadOptions
        );
        assert_eq!(sfx_for(MainMenuControl::Quit), SfxKind::MenuQuit);
        assert_eq!(sfx_for(MainMenuControl::Credits), SfxKind::MenuSelect);
        assert_eq!(sfx_for(MainMenuControl::Alliance), SfxKind::MenuSelect);
    }
}
