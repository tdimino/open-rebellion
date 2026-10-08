//! Original bitmap-driven unified Game Options surface.

use macroquad::prelude::*;

use crate::audio::AudioVolumeState;
use crate::bmp_cache::{resources, BmpCache, DllSource};
use crate::panels::SaveSlotInfo;

const LOGICAL_WIDTH: f32 = 640.0;
const LOGICAL_HEIGHT: f32 = 480.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOptionsOrigin {
    ShuttleCockpit,
    CommandCenter,
    TacticalBattle,
}

impl GameOptionsOrigin {
    const fn has_live_campaign(self) -> bool {
        !matches!(self, Self::ShuttleCockpit)
    }

    const fn can_restart(self) -> bool {
        matches!(self, Self::CommandCenter)
    }

    const fn tactical_toggles_enabled(self) -> bool {
        !matches!(self, Self::TacticalBattle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameOptionsAction {
    None,
    Return,
    Restart,
    Exit,
    Save { slot: usize, name: String },
    Load { slot: usize },
    Delete { slot: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameOptionsControl {
    Save(usize),
    Load(usize),
    Music,
    MusicVolume,
    SoundVolume,
    Tactical(usize),
    Restart,
    Return,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct NativeRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl NativeRect {
    const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

// Control rectangles from the window's control constructor `FUN_00407180`.
const MUSIC_RECT: NativeRect = NativeRect::new(352.0, 76.0, 19.0, 35.0);
/// The volume sliders' origins (`FUN_00604570` at (392,130) and (392,190)).
/// hyp: their extent is the rail bitmap's, 212 by 54; the slider's own
/// width argument is untraced.
const MUSIC_SLIDER_RECT: NativeRect = NativeRect::new(392.0, 130.0, 212.0, 54.0);
const SOUND_SLIDER_RECT: NativeRect = NativeRect::new(392.0, 190.0, 212.0, 54.0);
/// The tactical display switches, 35 by 22 at x 357; their rows are not
/// evenly spaced.
const TACTICAL_TOGGLE_RECTS: [NativeRect; 5] = [
    NativeRect::new(357.0, 311.0, 35.0, 22.0),
    NativeRect::new(357.0, 337.0, 35.0, 22.0),
    NativeRect::new(357.0, 365.0, 35.0, 22.0),
    NativeRect::new(357.0, 392.0, 35.0, 22.0),
    NativeRect::new(357.0, 419.0, 35.0, 22.0),
];
const RESTART_RECT: NativeRect = NativeRect::new(76.0, 381.0, 42.0, 42.0);
const RETURN_RECT: NativeRect = NativeRect::new(162.0, 382.0, 42.0, 42.0);
const EXIT_RECT: NativeRect = NativeRect::new(248.0, 381.0, 42.0, 42.0);
/// The first saved-game row's y; each row is 42 lower (`FUN_00407180`'s
/// table: 0x51, 0x7b, 0xa5, 0xcf, 0xf9, 0x123).
const SAVE_ROW_Y: f32 = 81.0;
const SAVE_ROW_STEP: f32 = 42.0;
/// A row's name field (`FUN_00604cf0` at x 0x77, 160 by 20).
const NAME_FIELD_X: f32 = 119.0;
const NAME_FIELD_WIDTH: f32 = 160.0;
/// An occupied row's side emblem, two pixels below the row's top
/// (`FUN_005fd0f0` at x 0x55 with the 0x53.. table).
const SLOT_EMBLEM_X: f32 = 85.0;
const SLOT_EMBLEM_DY: f32 = 2.0;

/// How `DrawText` places a label in its box: the format flags passed to
/// `FUN_00601620` and `FUN_005ff6b0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TextAlign {
    /// `0x25`: `DT_CENTER | DT_VCENTER | DT_SINGLELINE`.
    Centered,
    /// `0x24`: `DT_VCENTER | DT_SINGLELINE`, left-aligned.
    LeftMiddle,
    /// `0x10`: `DT_WORDBREAK`, left- and top-aligned.
    LeftTop,
}

/// One label of the window, as `FUN_004084b0` builds it.
#[derive(Debug, Clone, Copy, PartialEq)]
struct OptionsText {
    rect: NativeRect,
    align: TextAlign,
    /// The game-font entry (`FUN_0060eed0`).
    font: u8,
}

/// The three panel titles (`FUN_005ff6b0`, arguments at `0x4084e1`):
/// TEXTCOMM 0x1028, 0x1026 and 0x1027, font 7, colour `0x200ff00`.
const TITLES: [(&str, NativeRect); 3] = [
    ("Saved Games", NativeRect::new(58.0, 37.0, 241.0, 21.0)),
    ("Sound Options", NativeRect::new(362.0, 37.0, 241.0, 21.0)),
    (
        "Tactical Display Options",
        NativeRect::new(363.0, 273.0, 239.0, 21.0),
    ),
];
/// TEXTCOMM 0x1025 "Play Music" and its On/Off at (581,79,31,33).
const MUSIC_LABEL: OptionsText = OptionsText {
    rect: NativeRect::new(376.0, 79.0, 183.0, 33.0),
    align: TextAlign::LeftMiddle,
    font: 7,
};
const MUSIC_STATE: OptionsText = OptionsText {
    rect: NativeRect::new(581.0, 79.0, 31.0, 33.0),
    align: TextAlign::LeftMiddle,
    font: 7,
};
/// TEXTCOMM 0x101f..0x1022 and 0x1024, in the order of their switches.
const TACTICAL_LABELS: [&str; 5] = [
    "Show Starfield",
    "Show Planet",
    "Show Pyrotechnics",
    "Use High Detail Models",
    "Display Holocube",
];
/// The tactical labels' tops: 0x139, 0x154, 0x16f, 0x18a, 0x1a5.
const TACTICAL_LABEL_Y: [f32; 5] = [313.0, 340.0, 367.0, 394.0, 421.0];
const fn tactical_label(row: usize) -> OptionsText {
    OptionsText {
        rect: NativeRect::new(394.0, TACTICAL_LABEL_Y[row], 183.0, 16.0),
        align: TextAlign::LeftTop,
        font: 7,
    }
}
const fn tactical_state(row: usize) -> OptionsText {
    OptionsText {
        rect: NativeRect::new(577.0, TACTICAL_LABEL_Y[row], 31.0, 16.0),
        align: TextAlign::LeftTop,
        font: 7,
    }
}
/// TEXTCOMM 0x1031 and 0x1032: `FUN_004084b0` passes `0x1032 - on`.
const fn on_off(on: bool) -> &'static str {
    if on {
        "On"
    } else {
        "Off"
    }
}
/// The version line: font 10, black (`0x2000000`), centred.
const VERSION_TEXT: OptionsText = OptionsText {
    rect: NativeRect::new(25.0, 437.0, 310.0, 18.0),
    align: TextAlign::Centered,
    font: 10,
};
/// The original composes TEXTCOMM 0x100c, its own "1.01.00", a space and
/// the locale's language name (`FUN_00406840`). port: Open Rebellion's own
/// version number stands in for "1.01.00"; the English data's language.
fn version_line() -> String {
    format!(
        "Version: {} English (United States)",
        env!("CARGO_PKG_VERSION")
    )
}
/// Title colour `0x200ff00`.
const TITLE_RGB: [u8; 3] = [0, 255, 0];
/// An option label's colour: `0x2008000`, plus `0x7f00` while the option
/// is on (`FUN_004084b0`).
const fn option_rgb(on: bool) -> [u8; 3] {
    if on {
        [0, 255, 0]
    } else {
        [0, 128, 0]
    }
}

#[derive(Debug, Clone, Copy)]
struct OptionsCanvas {
    scale: f32,
    offset_x: f32,
    offset_y: f32,
}

impl OptionsCanvas {
    fn new(width: f32, height: f32) -> Self {
        let scale = (width / LOGICAL_WIDTH).min(height / LOGICAL_HEIGHT);
        Self {
            scale,
            offset_x: (width - LOGICAL_WIDTH * scale) * 0.5,
            offset_y: (height - LOGICAL_HEIGHT * scale) * 0.5,
        }
    }

    fn point(self, x: f32, y: f32) -> (f32, f32) {
        (
            self.offset_x + x * self.scale,
            self.offset_y + y * self.scale,
        )
    }

    fn logical_pointer(self) -> (f32, f32) {
        let (x, y) = mouse_position();
        (
            (x - self.offset_x) / self.scale,
            (y - self.offset_y) / self.scale,
        )
    }
}

#[derive(Debug, Clone)]
pub struct GameOptionsState {
    pub pending: Option<GameOptionsAction>,
    pub occupied: [bool; 10],
    pub names: [String; 6],
    pub error: Option<String>,
    origin: GameOptionsOrigin,
    pressed: Option<GameOptionsControl>,
    /// Whether music plays, refreshed each frame for the overlay's label.
    music_on: bool,
    pub show_starfield: bool,
    pub show_planet: bool,
    pub show_pyrotechnics: bool,
    pub high_detail_models: bool,
    pub display_holocube: bool,
}

impl Default for GameOptionsState {
    fn default() -> Self {
        Self {
            pending: None,
            occupied: [false; 10],
            names: std::array::from_fn(|_| String::new()),
            error: None,
            origin: GameOptionsOrigin::ShuttleCockpit,
            pressed: None,
            music_on: true,
            show_starfield: true,
            show_planet: true,
            show_pyrotechnics: true,
            high_detail_models: true,
            display_holocube: true,
        }
    }
}

impl GameOptionsState {
    pub fn refresh_saves(&mut self, saves: &[SaveSlotInfo]) {
        // An empty slot's field is blank (`FUN_00407180` writes the empty
        // string at DAT_006b120c when a slot holds no save).
        self.occupied.fill(false);
        self.names = std::array::from_fn(|_| String::new());
        for save in saves {
            if let Some(occupied) = self.occupied.get_mut(save.slot) {
                *occupied = true;
            }
            if let Some(name) = self.names.get_mut(save.slot) {
                name.clone_from(&save.name);
            }
        }
    }

    fn enabled(&self, action: &GameOptionsAction) -> bool {
        match action {
            GameOptionsAction::Save { slot, name } => {
                self.origin == GameOptionsOrigin::CommandCenter
                    && *slot < self.occupied.len()
                    && !name.trim().is_empty()
            }
            GameOptionsAction::Load { slot } | GameOptionsAction::Delete { slot } => {
                self.origin != GameOptionsOrigin::TacticalBattle
                    && self.occupied.get(*slot).copied().unwrap_or(false)
            }
            GameOptionsAction::Restart => self.origin.can_restart(),
            GameOptionsAction::Return | GameOptionsAction::Exit => true,
            GameOptionsAction::None => false,
        }
    }

    pub fn request(&mut self, action: GameOptionsAction) -> GameOptionsAction {
        if self.pending.is_some() || !self.enabled(&action) {
            return GameOptionsAction::None;
        }
        let confirm = match &action {
            GameOptionsAction::Save { slot, .. } => self.occupied[*slot],
            GameOptionsAction::Load { .. } => self.origin.has_live_campaign(),
            GameOptionsAction::Restart
            | GameOptionsAction::Exit
            | GameOptionsAction::Delete { .. } => true,
            _ => false,
        };
        self.error = None;
        if confirm {
            self.pending = Some(action);
            GameOptionsAction::None
        } else {
            action
        }
    }

    pub fn confirm(&mut self, accepted: bool) -> GameOptionsAction {
        self.pending
            .take()
            .filter(|action| accepted && self.enabled(action))
            .unwrap_or(GameOptionsAction::None)
    }

    pub fn escape(&mut self) -> GameOptionsAction {
        if self.pending.take().is_some() {
            GameOptionsAction::None
        } else {
            GameOptionsAction::Return
        }
    }

    #[must_use]
    pub fn new(origin: GameOptionsOrigin) -> Self {
        Self {
            origin,
            ..Self::default()
        }
    }

    pub fn set_origin(&mut self, origin: GameOptionsOrigin) {
        self.origin = origin;
        self.pressed = None;
        self.pending = None;
        self.error = None;
    }

    #[must_use]
    pub const fn origin(&self) -> GameOptionsOrigin {
        self.origin
    }

    #[must_use]
    pub const fn tactical_flags(&self) -> [bool; 5] {
        [
            self.show_starfield,
            self.show_planet,
            self.show_pyrotechnics,
            self.high_detail_models,
            self.display_holocube,
        ]
    }
}

fn row_y(slot: usize) -> f32 {
    SAVE_ROW_Y + slot as f32 * SAVE_ROW_STEP
}

fn save_rect(slot: usize) -> NativeRect {
    NativeRect::new(34.0, row_y(slot), 42.0, 20.0)
}

fn load_rect(slot: usize) -> NativeRect {
    NativeRect::new(287.0, row_y(slot), 41.0, 20.0)
}

fn control_at(origin: GameOptionsOrigin, x: f32, y: f32) -> Option<GameOptionsControl> {
    for slot in 0..6 {
        if origin == GameOptionsOrigin::CommandCenter && save_rect(slot).contains(x, y) {
            return Some(GameOptionsControl::Save(slot));
        }
        if origin != GameOptionsOrigin::TacticalBattle && load_rect(slot).contains(x, y) {
            return Some(GameOptionsControl::Load(slot));
        }
    }
    if MUSIC_RECT.contains(x, y) {
        return Some(GameOptionsControl::Music);
    }
    if MUSIC_SLIDER_RECT.contains(x, y) {
        return Some(GameOptionsControl::MusicVolume);
    }
    if SOUND_SLIDER_RECT.contains(x, y) {
        return Some(GameOptionsControl::SoundVolume);
    }
    for (index, rect) in TACTICAL_TOGGLE_RECTS.into_iter().enumerate() {
        if origin.tactical_toggles_enabled() && rect.contains(x, y) {
            return Some(GameOptionsControl::Tactical(index));
        }
    }
    if origin.can_restart() && RESTART_RECT.contains(x, y) {
        return Some(GameOptionsControl::Restart);
    }
    if origin.has_live_campaign() && RETURN_RECT.contains(x, y) {
        return Some(GameOptionsControl::Return);
    }
    EXIT_RECT.contains(x, y).then_some(GameOptionsControl::Exit)
}

fn masked_control_at(
    cache: &mut BmpCache,
    origin: GameOptionsOrigin,
    x: f32,
    y: f32,
) -> Option<GameOptionsControl> {
    let control = control_at(origin, x, y)?;
    let (id, rect) = match control {
        GameOptionsControl::Save(slot) => (resources::common::OPTIONS_SAVE_NORMAL, save_rect(slot)),
        GameOptionsControl::Load(slot) => (resources::common::OPTIONS_LOAD_NORMAL, load_rect(slot)),
        GameOptionsControl::Music => (resources::common::OPTIONS_MUSIC_NORMAL, MUSIC_RECT),
        GameOptionsControl::MusicVolume => {
            (resources::common::OPTIONS_VOLUME_RAIL, MUSIC_SLIDER_RECT)
        }
        GameOptionsControl::SoundVolume => (
            resources::common::OPTIONS_VOLUME_RAIL_ALT,
            SOUND_SLIDER_RECT,
        ),
        GameOptionsControl::Tactical(index) => (
            resources::common::OPTIONS_TOGGLE_NORMAL,
            TACTICAL_TOGGLE_RECTS[index],
        ),
        GameOptionsControl::Restart => (resources::common::BTN_RESTART_GAME_NORMAL, RESTART_RECT),
        GameOptionsControl::Return => (
            resources::common::BTN_RETURN_COMMAND_CENTER_NORMAL,
            RETURN_RECT,
        ),
        GameOptionsControl::Exit => (resources::common::BTN_EXIT_GAME_NORMAL, EXIT_RECT),
    };
    cache
        .is_resource_hit(
            DllSource::Common,
            id,
            (x - rect.x) as usize,
            (y - rect.y) as usize,
        )
        .then_some(control)
}

fn draw_bitmap(cache: &mut BmpCache, canvas: OptionsCanvas, resource_id: u32, x: f32, y: f32) {
    let Some(texture) = cache.get_macroquad_original(DllSource::Common, resource_id) else {
        return;
    };
    let (screen_x, screen_y) = canvas.point(x, y);
    draw_texture_ex(
        texture,
        screen_x,
        screen_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                texture.width() * canvas.scale,
                texture.height() * canvas.scale,
            )),
            ..Default::default()
        },
    );
}

fn draw_volume(cache: &mut BmpCache, canvas: OptionsCanvas, y: f32, value: f32, alternate: bool) {
    draw_bitmap(
        cache,
        canvas,
        if alternate {
            resources::common::OPTIONS_VOLUME_RAIL_ALT
        } else {
            resources::common::OPTIONS_VOLUME_RAIL
        },
        MUSIC_SLIDER_RECT.x,
        y,
    );
    let handle_x = MUSIC_SLIDER_RECT.x + 1.0 + value.clamp(0.0, 1.0) * 199.0;
    draw_bitmap(
        cache,
        canvas,
        resources::common::OPTIONS_VOLUME_HANDLE,
        handle_x,
        y + 4.0,
    );
}

/// A tactical switch's bitmap: disabled 0x273d, pressed or checked
/// 0x273c, normal 0x273b (`FUN_00407180` states 2 and 4).
const fn toggle_resource(enabled: bool, held: bool, on: bool) -> u32 {
    if !enabled {
        resources::common::OPTIONS_TOGGLE_DISABLED
    } else if held || on {
        resources::common::OPTIONS_TOGGLE_PRESSED
    } else {
        resources::common::OPTIONS_TOGGLE_NORMAL
    }
}

/// An occupied slot's emblem: 0x2748 for side 1, the Alliance, and 0x2747
/// for side 2 (`FUN_00407180`). hyp: the 0x2749 case, chosen by a save
/// field not yet identified, is not drawn.
fn slot_emblem(save: &SaveSlotInfo) -> Option<u32> {
    save.player_is_alliance.map(|alliance| {
        if alliance {
            resources::common::OPTIONS_ALLIANCE_MARKER
        } else {
            resources::common::OPTIONS_EMPIRE_MARKER
        }
    })
}

fn set_slider(audio: &mut AudioVolumeState, control: GameOptionsControl, logical_x: f32) {
    let value = ((logical_x - (MUSIC_SLIDER_RECT.x + 5.0)) / 199.0).clamp(0.0, 1.0);
    match control {
        GameOptionsControl::MusicVolume => audio.music_volume = value,
        GameOptionsControl::SoundVolume => audio.sfx_volume = value,
        _ => {}
    }
    audio.dirty = true;
}

fn toggle_tactical(state: &mut GameOptionsState, index: usize) {
    match index {
        0 => state.show_starfield = !state.show_starfield,
        1 => state.show_planet = !state.show_planet,
        2 => state.show_pyrotechnics = !state.show_pyrotechnics,
        3 => state.high_detail_models = !state.high_detail_models,
        4 => state.display_holocube = !state.display_holocube,
        _ => {}
    }
}

fn activate(
    state: &mut GameOptionsState,
    control: GameOptionsControl,
    saves: &[SaveSlotInfo],
    audio: &mut AudioVolumeState,
) -> GameOptionsAction {
    match control {
        GameOptionsControl::Music => {
            audio.toggle_music();
            macroquad::logging::info!(
                "[game_options] control=music enabled={}",
                audio.music_enabled()
            );
        }
        GameOptionsControl::Tactical(index) => {
            toggle_tactical(state, index);
            macroquad::logging::info!(
                "[game_options] control=tactical index={} enabled={}",
                index,
                state.tactical_flags()[index]
            );
        }
        GameOptionsControl::Save(slot) => {
            let name = state.names[slot].clone();
            return state.request(GameOptionsAction::Save { slot, name });
        }
        GameOptionsControl::Load(slot) => {
            if saves.iter().any(|save| save.slot == slot) {
                return state.request(GameOptionsAction::Load { slot });
            }
        }
        GameOptionsControl::Restart => return state.request(GameOptionsAction::Restart),
        GameOptionsControl::Return => return state.request(GameOptionsAction::Return),
        GameOptionsControl::Exit => return state.request(GameOptionsAction::Exit),
        GameOptionsControl::MusicVolume | GameOptionsControl::SoundVolume => {}
    }
    GameOptionsAction::None
}

/// Draw the original 640×480 Game Options screen and dispatch its controls.
pub fn draw_game_options(
    state: &mut GameOptionsState,
    cache: &mut BmpCache,
    saves: &[SaveSlotInfo],
    audio: &mut AudioVolumeState,
) -> GameOptionsAction {
    let canvas = OptionsCanvas::new(screen_width(), screen_height());
    clear_background(BLACK);
    draw_bitmap(cache, canvas, resources::common::GAME_OPTIONS_BG, 0.0, 0.0);

    state.music_on = audio.music_enabled();

    let pointer = canvas.logical_pointer();
    let held = state
        .pressed
        .filter(|_| is_mouse_button_down(MouseButton::Left));
    draw_bitmap(
        cache,
        canvas,
        if !audio.backend_available {
            resources::common::OPTIONS_MUSIC_DISABLED
        } else if held == Some(GameOptionsControl::Music) || state.music_on {
            // A checked button shows its pressed bitmap (`FUN_00407180`
            // sets state 4 to 0x2739 while music is on).
            resources::common::OPTIONS_MUSIC_PRESSED
        } else {
            resources::common::OPTIONS_MUSIC_NORMAL
        },
        MUSIC_RECT.x,
        MUSIC_RECT.y,
    );
    draw_volume(
        cache,
        canvas,
        MUSIC_SLIDER_RECT.y,
        audio.music_volume,
        false,
    );
    draw_volume(cache, canvas, SOUND_SLIDER_RECT.y, audio.sfx_volume, true);

    for (index, rect) in TACTICAL_TOGGLE_RECTS.into_iter().enumerate() {
        let resource = toggle_resource(
            state.origin.tactical_toggles_enabled(),
            held == Some(GameOptionsControl::Tactical(index)),
            state.tactical_flags()[index],
        );
        draw_bitmap(cache, canvas, resource, rect.x, rect.y);
    }

    for slot in 0..6 {
        let occupied = saves.iter().find(|save| save.slot == slot);
        let save_resource = if state.origin != GameOptionsOrigin::CommandCenter {
            resources::common::OPTIONS_SAVE_DISABLED
        } else if held == Some(GameOptionsControl::Save(slot)) {
            resources::common::OPTIONS_SAVE_PRESSED
        } else {
            resources::common::OPTIONS_SAVE_NORMAL
        };
        let load_resource =
            if occupied.is_none() || state.origin == GameOptionsOrigin::TacticalBattle {
                resources::common::OPTIONS_LOAD_DISABLED
            } else if held == Some(GameOptionsControl::Load(slot)) {
                resources::common::OPTIONS_LOAD_PRESSED
            } else {
                resources::common::OPTIONS_LOAD_NORMAL
            };
        let save_bounds = save_rect(slot);
        let load_bounds = load_rect(slot);
        draw_bitmap(cache, canvas, save_resource, save_bounds.x, save_bounds.y);
        draw_bitmap(cache, canvas, load_resource, load_bounds.x, load_bounds.y);
        if let Some(emblem) = occupied.and_then(slot_emblem) {
            draw_bitmap(
                cache,
                canvas,
                emblem,
                SLOT_EMBLEM_X,
                row_y(slot) + SLOT_EMBLEM_DY,
            );
        }
    }

    draw_bitmap(
        cache,
        canvas,
        if !state.origin.can_restart() {
            resources::common::BTN_RESTART_GAME_DISABLED
        } else if held == Some(GameOptionsControl::Restart) {
            resources::common::BTN_RESTART_GAME_PRESSED
        } else {
            resources::common::BTN_RESTART_GAME_NORMAL
        },
        RESTART_RECT.x,
        RESTART_RECT.y,
    );
    draw_bitmap(
        cache,
        canvas,
        if !state.origin.has_live_campaign() {
            resources::common::BTN_RETURN_COMMAND_CENTER_DISABLED
        } else if held == Some(GameOptionsControl::Return) {
            resources::common::BTN_RETURN_COMMAND_CENTER_PRESSED
        } else {
            resources::common::BTN_RETURN_COMMAND_CENTER_NORMAL
        },
        RETURN_RECT.x,
        RETURN_RECT.y,
    );
    draw_bitmap(
        cache,
        canvas,
        if held == Some(GameOptionsControl::Exit) {
            resources::common::BTN_EXIT_GAME_PRESSED
        } else {
            resources::common::BTN_EXIT_GAME_NORMAL
        },
        EXIT_RECT.x,
        EXIT_RECT.y,
    );

    if state.pending.is_some() {
        state.pressed = None;
        return GameOptionsAction::None;
    }
    if let Some(control @ (GameOptionsControl::MusicVolume | GameOptionsControl::SoundVolume)) =
        state.pressed
    {
        if is_mouse_button_down(MouseButton::Left) {
            set_slider(audio, control, pointer.0);
        }
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        state.pressed = masked_control_at(cache, state.origin, pointer.0, pointer.1);
        if let Some(control @ (GameOptionsControl::MusicVolume | GameOptionsControl::SoundVolume)) =
            state.pressed
        {
            set_slider(audio, control, pointer.0);
        }
    }
    if is_mouse_button_released(MouseButton::Left) {
        let pressed = state.pressed.take();
        if pressed == masked_control_at(cache, state.origin, pointer.0, pointer.1) {
            if let Some(control) = pressed {
                return activate(state, control, saves, audio);
            }
        }
    }
    GameOptionsAction::None
}

/// Edit names and draw a confirmation above the macroquad options surface.
/// Missing original assets use visible standard buttons instead of an invisible modal.
pub fn draw_game_options_overlay(
    ctx: &egui_macroquad::egui::Context,
    cache: &mut BmpCache,
    state: &mut GameOptionsState,
) -> GameOptionsAction {
    use egui_macroquad::egui::{self, Align2, Color32, FontId, Pos2, Rect, Vec2};
    let screen = ctx.screen_rect();
    let canvas = OptionsCanvas::new(screen.width(), screen.height());
    let rect = |x, y, w, h| {
        let (x, y) = canvas.point(x, y);
        Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, h) * canvas.scale)
    };
    let mut answer = None;
    let label = |ui: &egui::Ui, text: &str, spec: OptionsText, rgb: [u8; 3]| {
        let font = crate::theme::game_font_on(ctx, spec.font, canvas.scale);
        let color = Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
        let galley = ui.painter().layout_no_wrap(text.to_owned(), font, color);
        let bounds = rect(spec.rect.x, spec.rect.y, spec.rect.width, spec.rect.height);
        let size = galley.size();
        let x = match spec.align {
            TextAlign::Centered => bounds.center().x - size.x / 2.0,
            TextAlign::LeftMiddle | TextAlign::LeftTop => bounds.left(),
        };
        let y = match spec.align {
            TextAlign::LeftTop => bounds.top(),
            TextAlign::Centered | TextAlign::LeftMiddle => bounds.center().y - size.y / 2.0,
        };
        ui.painter().galley(Pos2::new(x, y), galley, color);
    };
    egui::CentralPanel::default().frame(egui::Frame::NONE).show(ctx, |ui| {
        for (text, bounds) in TITLES {
            let spec = OptionsText { rect: bounds, align: TextAlign::Centered, font: 7 };
            label(ui, text, spec, TITLE_RGB);
        }
        label(ui, "Play Music", MUSIC_LABEL, option_rgb(state.music_on));
        label(ui, on_off(state.music_on), MUSIC_STATE, option_rgb(state.music_on));
        for (row, text) in TACTICAL_LABELS.into_iter().enumerate() {
            let on = state.tactical_flags()[row];
            label(ui, text, tactical_label(row), option_rgb(on));
            label(ui, on_off(on), tactical_state(row), option_rgb(on));
        }
        label(ui, &version_line(), VERSION_TEXT, [0, 0, 0]);
        // A row's name field: white text (`FUN_00407180` sets +0xe4 to
        // 0x2ffffff). hyp: the window's font, entry 10.
        let name_font = crate::theme::game_font_on(ctx, 10, canvas.scale);
        // The area itself must not claim the entire canvas; only actual controls do.
        for slot in 0..6 {
            let bounds = rect(NAME_FIELD_X, row_y(slot), NAME_FIELD_WIDTH, 20.0);
            if state.origin == GameOptionsOrigin::CommandCenter {
                ui.add_enabled_ui(state.pending.is_none(), |ui| {
                    ui.put(bounds,
                        egui::TextEdit::singleline(&mut state.names[slot])
                            .id_source(("options_save_name", slot)).frame(false)
                            .font(name_font.clone()).text_color(Color32::WHITE));
                });
            } else if state.occupied[slot] {
                ui.painter().text(bounds.left_center(), Align2::LEFT_CENTER,
                    &state.names[slot], name_font.clone(), Color32::WHITE);
            }
        }
        if let Some(action) = &state.pending {
            let assets = [10623, 10624, 10625, 10626, 10627];
            let complete = assets.iter().all(|id| cache.get(ctx, DllSource::Rebdlog, *id).is_some() && cache.original_resource_size(DllSource::Rebdlog, *id).is_some());
            let bounds = rect(114.0, 152.0, 412.0, 176.0);
            let uv = Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0));
            if complete {
                ui.painter().image(cache.get(ctx, DllSource::Rebdlog, 10623).unwrap().id(), bounds, uv, Color32::WHITE);
            } else {
                ui.painter().rect_filled(bounds, 4.0, Color32::from_rgb(30, 30, 35));
            }
            let message = match action {
                GameOptionsAction::Save { .. } => "Saving the selected game will destroy a previously saved game. Save anyway?",
                GameOptionsAction::Load { .. } => "Loading the selected game will destroy unsaved changes. Load without saving?",
                GameOptionsAction::Delete { .. } => "Delete the selected saved game?",
                GameOptionsAction::Restart => "Returning to the shuttle cockpit will cause unsaved changes to be lost. Return without saving?",
                GameOptionsAction::Exit => "Unsaved changes will be lost. Exit the game?",
                _ => "Return?",
            };
            let text = ui.painter().layout(message.into(), FontId::monospace(12.0 * canvas.scale), Color32::GREEN, 310.0 * canvas.scale);
            ui.painter().galley(rect(160.0, 177.0, 320.0, 75.0).center() - text.size() / 2.0, text, Color32::GREEN);
            for (x, normal, pressed, value, label) in [(252.0,10624,10625,true,"Yes"),(343.0,10626,10627,false,"No")] {
                let bounds = rect(x, 287.0, 57.0, 28.0);
                if complete {
                    let response = ui.interact(bounds, ui.id().with(normal), egui::Sense::click());
                    let id = if response.is_pointer_button_down_on() { pressed } else { normal };
                    ui.painter().image(cache.get(ctx, DllSource::Rebdlog, id).unwrap().id(), bounds, uv, Color32::WHITE);
                    let capture_id = ui.id().with((normal, "source_capture"));
                    if ctx.input(|i| i.pointer.primary_pressed()) {
                        let hit = ctx.pointer_latest_pos().is_some_and(|pos| bounds.contains(pos)
                            && cache.is_resource_hit(DllSource::Rebdlog, normal,
                                ((pos.x - bounds.left()) / canvas.scale) as usize,
                                ((pos.y - bounds.top()) / canvas.scale) as usize));
                        ui.data_mut(|data| data.insert_temp(capture_id, hit));
                    }
                    let captured = ui.data(|data| data.get_temp::<bool>(capture_id).unwrap_or(false));
                    if captured && response.clicked() && response.interact_pointer_pos().is_some_and(|pos| {
                        cache.is_resource_hit(DllSource::Rebdlog, normal,
                            ((pos.x - bounds.left()) / canvas.scale) as usize,
                            ((pos.y - bounds.top()) / canvas.scale) as usize)
                    }) { answer = Some(value); }
                } else {
                    let response = ui.put(bounds, egui::Button::new(label).min_size(bounds.size()));
                    if response.clicked() { answer = Some(value); }
                }
            }
        }
        if let Some(error) = &state.error {
            ui.painter().text(rect(25.0, 440.0, 590.0, 20.0).left_center(), Align2::LEFT_CENTER,
                error, FontId::monospace(11.0 * canvas.scale), Color32::LIGHT_RED);
        }
    });
    answer.map_or(GameOptionsAction::None, |accepted| state.confirm(accepted))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupied_legacy_slots_wait_for_confirmation_and_cancellation_preserves_them() {
        let mut state = GameOptionsState::new(GameOptionsOrigin::CommandCenter);
        state.refresh_saves(&[SaveSlotInfo {
            slot: 9,
            name: "Legacy".into(),
            timestamp: String::new(),
            game_tick: 0,
            player_is_alliance: None,
        }]);
        let action = GameOptionsAction::Load { slot: 9 };
        assert_eq!(state.request(action.clone()), GameOptionsAction::None);
        assert_eq!(state.pending, Some(action.clone()));
        assert_eq!(state.confirm(false), GameOptionsAction::None);
        assert!(state.occupied[9]);
        assert_eq!(state.request(action.clone()), GameOptionsAction::None);
        assert_eq!(state.confirm(true), action);
        assert_eq!(state.confirm(true), GameOptionsAction::None);
    }

    #[test]
    fn saves_require_valid_names_and_slots_and_overwrites_require_confirmation() {
        let mut state = GameOptionsState::new(GameOptionsOrigin::CommandCenter);
        for slot in [0, 9] {
            let save = GameOptionsAction::Save {
                slot,
                name: "Campaign".into(),
            };
            assert_eq!(state.request(save.clone()), save);
            state.occupied[slot] = true;
            assert_eq!(state.request(save.clone()), GameOptionsAction::None);
            assert_eq!(state.pending, Some(save.clone()));
            assert_eq!(state.confirm(false), GameOptionsAction::None);
            state.request(save.clone());
            assert_eq!(state.confirm(true), save);
        }
        for (slot, name) in [(10, "Name"), (usize::MAX, "Name"), (0, " ")] {
            assert_eq!(
                state.request(GameOptionsAction::Save {
                    slot,
                    name: name.into()
                }),
                GameOptionsAction::None
            );
            assert!(state.pending.is_none());
        }
        state.request(GameOptionsAction::Exit);
        assert_eq!(
            state.request(GameOptionsAction::Return),
            GameOptionsAction::None
        );
        assert_eq!(state.pending, Some(GameOptionsAction::Exit));
        assert_eq!(state.escape(), GameOptionsAction::None);
        assert_eq!(state.escape(), GameOptionsAction::Return);
    }

    #[test]
    fn tactical_options_reject_save_load_delete_and_restart() {
        // FUN_00407180: states 20–23 disable campaign save and load controls.
        let mut state = GameOptionsState::new(GameOptionsOrigin::TacticalBattle);
        state.occupied[0] = true;
        for action in [
            GameOptionsAction::Save {
                slot: 0,
                name: "Battle".into(),
            },
            GameOptionsAction::Load { slot: 0 },
            GameOptionsAction::Delete { slot: 0 },
            GameOptionsAction::Restart,
        ] {
            assert_eq!(state.request(action), GameOptionsAction::None);
            assert!(state.pending.is_none());
        }
        assert!(!GameOptionsOrigin::TacticalBattle.can_restart());
        assert!(!GameOptionsOrigin::ShuttleCockpit.can_restart());
        assert!(GameOptionsOrigin::CommandCenter.can_restart());
    }

    #[test]
    fn missing_dialog_assets_still_allow_confirmation_and_cancellation() {
        use egui_macroquad::egui::{self, Pos2, Rect, Vec2};
        for accepted in [false, true] {
            let ctx = egui::Context::default();
            let mut cache = BmpCache::new();
            let mut state = GameOptionsState::new(GameOptionsOrigin::CommandCenter);
            state.request(GameOptionsAction::Exit);
            let pos = Pos2::new(if accepted { 280.0 } else { 371.0 }, 301.0);
            let mut emitted = GameOptionsAction::None;
            for down in [None, Some(true), Some(false)] {
                let mut events = vec![egui::Event::PointerMoved(pos)];
                if let Some(pressed) = down {
                    events.push(egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: Default::default(),
                    });
                }
                let _ = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(640.0, 480.0))),
                        events,
                        ..Default::default()
                    },
                    |ctx| {
                        let action = draw_game_options_overlay(ctx, &mut cache, &mut state);
                        if action != GameOptionsAction::None {
                            emitted = action;
                        }
                    },
                );
            }
            assert!(state.pending.is_none(), "fallback must consume the answer");
            assert_eq!(
                emitted,
                if accepted {
                    GameOptionsAction::Exit
                } else {
                    GameOptionsAction::None
                }
            );
        }
    }

    #[test]
    fn bitmap_controls_reject_transparent_corners_and_exact_outer_edges() {
        // FUN_005fca00: palette-key hit mask and strict top/left/outer edges.
        let root = std::env::temp_dir().join(format!("options-hit-mask-{}", std::process::id()));
        let dir = root.join("common-dll/BMP");
        std::fs::create_dir_all(&dir).unwrap();
        let (width, height, stride, offset) = (19usize, 35usize, 20usize, 1078usize);
        let mut bmp = vec![0u8; offset + stride * height];
        bmp[..2].copy_from_slice(b"BM");
        bmp[10..14].copy_from_slice(&(offset as u32).to_le_bytes());
        bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
        bmp[18..22].copy_from_slice(&(width as i32).to_le_bytes());
        bmp[22..26].copy_from_slice(&(height as i32).to_le_bytes());
        bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
        bmp[28..30].copy_from_slice(&8u16.to_le_bytes());
        bmp[offset..].fill(3);
        bmp[offset] = 7;
        bmp[offset + (height - 2) * stride + 1] = 7;
        std::fs::write(dir.join("10040.bmp"), bmp).unwrap();
        let mut cache = BmpCache::new();
        cache.set_base_path(&root);
        let origin = GameOptionsOrigin::CommandCenter;
        assert_eq!(
            masked_control_at(&mut cache, origin, 356.0, 80.0),
            Some(GameOptionsControl::Music)
        );
        for (x, y) in [
            (352.0, 80.0),
            (356.0, 76.0),
            (371.0, 80.0),
            (356.0, 111.0),
            (353.0, 77.0),
        ] {
            assert_eq!(masked_control_at(&mut cache, origin, x, y), None, "{x},{y}");
        }
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn controls_sit_where_the_original_constructor_places_them() {
        // FUN_00407180: Play Music (0x160,0x4c), save x 0x22 and load x
        // 0x11f on rows 0x51 + 42n, switches at x 0x165 on 0x137, 0x151,
        // 0x16d, 0x188, 0x1a3, Restart (0x4c,0x17d), Return (0xa2,0x17e),
        // Exit (0xf8,0x17d). Each rectangle excludes its outer edge.
        let at = |x, y| control_at(GameOptionsOrigin::CommandCenter, x, y);
        assert_eq!(at(352.0, 76.0), Some(GameOptionsControl::Music));
        assert_eq!(at(351.0, 76.0), None);
        assert_eq!(at(371.0, 76.0), None);
        assert_eq!(at(34.0, 81.0), Some(GameOptionsControl::Save(0)));
        assert_eq!(at(33.0, 81.0), None);
        assert_eq!(at(287.0, 291.0), Some(GameOptionsControl::Load(5)));
        assert_eq!(at(286.0, 291.0), None);
        for (row, y) in [311.0, 337.0, 365.0, 392.0, 419.0].into_iter().enumerate() {
            assert_eq!(at(357.0, y), Some(GameOptionsControl::Tactical(row)));
            assert_ne!(at(357.0, y - 1.0), Some(GameOptionsControl::Tactical(row)));
        }
        assert_eq!(at(76.0, 381.0), Some(GameOptionsControl::Restart));
        assert_eq!(at(162.0, 382.0), Some(GameOptionsControl::Return));
        assert_eq!(at(162.0, 381.0), None);
        assert_eq!(at(248.0, 381.0), Some(GameOptionsControl::Exit));
        assert_eq!(at(247.0, 381.0), None);
    }

    #[test]
    fn a_checked_switch_shows_its_pressed_bitmap() {
        // FUN_00407180: normal 0x273b, pressed 0x273c, disabled 0x273d, and
        // state 4 (checked) also 0x273c.
        assert_eq!(toggle_resource(true, false, true), 10044);
        assert_eq!(toggle_resource(true, false, false), 10043);
        assert_eq!(toggle_resource(true, true, false), 10044);
        assert_eq!(toggle_resource(false, false, true), 10045);
    }

    #[test]
    fn empty_slots_are_blank_and_saved_slots_carry_their_name_and_side() {
        // FUN_00407180: an empty slot's field gets the empty string; an
        // occupied one its name and the emblem 0x2748 (side 1) or 0x2747.
        let mut state = GameOptionsState::new(GameOptionsOrigin::CommandCenter);
        let save = |slot, player_is_alliance| SaveSlotInfo {
            slot,
            name: format!("Campaign {slot}"),
            timestamp: String::new(),
            game_tick: 0,
            player_is_alliance,
        };
        state.refresh_saves(&[save(1, Some(true)), save(3, Some(false))]);
        assert_eq!(state.names[0], "");
        assert_eq!(state.names[1], "Campaign 1");
        assert_eq!(state.names[3], "Campaign 3");
        assert_eq!(slot_emblem(&save(1, Some(true))), Some(10056));
        assert_eq!(slot_emblem(&save(3, Some(false))), Some(10055));
        assert_eq!(slot_emblem(&save(4, None)), None);
    }

    #[test]
    fn labels_take_the_original_boxes_fonts_and_colours() {
        // FUN_004084b0 and the FUN_005ff6b0 calls at 0x4084e1: titles in
        // RECTs (58,37)-(299,58), (362,37)-(603,58), (363,273)-(602,294),
        // font 7; switch labels at x 0x18a on 0x139..0x1a5, 183 by 16;
        // colour 0x2008000, plus 0x7f00 when on.
        assert_eq!(
            TITLES.map(|(_, r)| (r.x, r.y, r.x + r.width, r.y + r.height)),
            [
                (58.0, 37.0, 299.0, 58.0),
                (362.0, 37.0, 603.0, 58.0),
                (363.0, 273.0, 602.0, 294.0)
            ]
        );
        assert_eq!(
            (0..5).map(|row| tactical_label(row).rect.y).collect::<Vec<_>>(),
            [313.0, 340.0, 367.0, 394.0, 421.0]
        );
        assert_eq!(tactical_label(2).rect, NativeRect::new(394.0, 367.0, 183.0, 16.0));
        assert_eq!(tactical_state(4).rect, NativeRect::new(577.0, 421.0, 31.0, 16.0));
        assert_eq!(tactical_label(0).font, 7);
        assert_eq!(VERSION_TEXT.font, 10);
        assert_eq!(option_rgb(false), [0, 128, 0]);
        assert_eq!(option_rgb(true), [0, 255, 0]);
        assert_eq!((on_off(true), on_off(false)), ("On", "Off"));
    }

    #[test]
    fn shuttle_and_tactical_contexts_disable_original_controls() {
        assert_eq!(
            control_at(GameOptionsOrigin::ShuttleCockpit, 80.0, 400.0),
            None
        );
        assert_eq!(
            control_at(GameOptionsOrigin::TacticalBattle, 370.0, 320.0),
            None
        );
        assert_eq!(
            control_at(GameOptionsOrigin::TacticalBattle, 175.0, 400.0),
            Some(GameOptionsControl::Return)
        );
    }

    #[test]
    fn tactical_flags_default_to_on() {
        // No recovered source: default tactical-option on state, kept as a regression pin.
        assert_eq!(GameOptionsState::default().tactical_flags(), [true; 5]);
    }
}
