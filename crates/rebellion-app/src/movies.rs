//! The movies the game plays, by the original's movie ids
//! (`ghidra/notes/movies.md`): the application states' movies and where each
//! one's end leads, the kind-6 game messages that carry one, and the ending
//! the advisor's script hands to the strategic window
//! (`ghidra/notes/game-end-path.md`).

use std::collections::VecDeque;

use egui_macroquad::egui;
use rebellion_core::dat::Faction;
use rebellion_core::victory::{AW, EW};
use rebellion_render::message_index::{MessageIndexAction, MessageIndexState};
use rebellion_render::status_window::StatusWindowState;

use crate::encyclopedia_surface::EncyclopediaSurfaceController;

/// "| Destroyed!", a system the Death Star destroyed (`FUN_004950d0` sets
/// `+0x5c`).
pub const SYSTEM_DESTROYED: u8 = 0x6f;
/// Death Star Sabotaged (`FUN_0048d470` sets `+0x5c`).
pub const DEATH_STAR_SABOTAGED: u8 = 0x72;
/// Alliance Headquarters Destroyed (`FUN_0048e5a0` sets `+0x5c`).
pub const HEADQUARTERS_DESTROYED: u8 = 0x73;
/// Coruscant Captured (`FUN_0048f790` sets `+0x5c`).
pub const CORUSCANT_CAPTURED: u8 = 0x74;

/// The MDATA file a movie id plays (`FUN_00421ba0`); none outside
/// `0x65`-`0x74`.
#[must_use]
pub const fn file_number(movie: u8) -> Option<u16> {
    Some(match movie {
        0x65 => 0,
        0x66 => 1,
        0x67 => 2,
        0x68 => 3,
        0x69 => 4,
        0x6a => 106,
        0x6b => 105,
        0x6c => 107,
        0x6d => 108,
        0x6e => 5,
        0x6f => 101,
        0x70 => 202,
        0x71 => 201,
        0x72 => 104,
        0x73 => 102,
        0x74 => 103,
        _ => return None,
    })
}

/// The staged copy of a movie id's MDATA file.
#[must_use]
pub fn path(movie: u8) -> Option<String> {
    file_number(movie).map(|number| format!("assets/references/ref-videos/{number:03}.webm"))
}

/// Whether a key other than Esc ends the movie: the movie window ignores
/// keys for the trench-run pair `0x70` and `0x71` only (`FUN_00421790`,
/// `WM_KEYDOWN`). Esc and the left button end any movie (`FUN_005f7240`).
#[must_use]
pub const fn keys_dismiss(movie: u8) -> bool {
    !matches!(movie, 0x70 | 0x71)
}

/// The ending code `FUN_00425c90` stores at strategic window `+0xa8` from
/// the win word, by the viewer (`+0x9c`): EW gives `0x12` to an Empire
/// viewer, else `0x11`; AW gives `0x10` to an Alliance viewer, else `0x13`;
/// no win bit gives 0.
#[must_use]
pub const fn ending_code(word: u32, viewer: Faction) -> u8 {
    if word & EW != 0 {
        if matches!(viewer, Faction::Empire) {
            0x12
        } else {
            0x11
        }
    } else if word & AW != 0 {
        if matches!(viewer, Faction::Alliance) {
            0x10
        } else {
            0x13
        }
    } else {
        0
    }
}

/// The movie an ending code plays after the teardown (`FUN_00414830`,
/// states `0x10`-`0x13`).
#[must_use]
pub const fn ending_movie(code: u8) -> Option<u8> {
    match code {
        0x10 => Some(0x6b),
        0x11 => Some(0x6a),
        0x12 => Some(0x6d),
        0x13 => Some(0x6c),
        _ => None,
    }
}

/// The intro, the first movie at the start of the program (`FUN_00414830`,
/// state 1).
pub const INTRO: u8 = 0x65;
/// The Credits movie the main menu's LucasArts logo plays (`FUN_00406000`,
/// control `0x73`).
pub const CREDITS: u8 = 0x6e;

/// The movie a new game opens with, by the player's side: `0x68` for the
/// Alliance, `0x69` for the Empire (`FUN_00414830`, state 7:
/// `(+0x108 != 1) + 0x68`).
#[must_use]
pub const fn side_movie(side: Faction) -> u8 {
    if matches!(side, Faction::Alliance) {
        0x68
    } else {
        0x69
    }
}

/// Where the game goes once a movie's window closes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scene {
    /// The next application state plays this movie.
    Movie(u8),
    /// State 3: the main menu (`FUN_00414a80`).
    MainMenu,
    /// State 8: the game's command center (`FUN_00414eb0`).
    Game,
    /// `0x402` to the window the movie played over, which resumes.
    Child,
}

/// The application window's `0x405` for a closed movie (`FUN_004138a0`,
/// the movie id in lParam), with the movie the next state plays
/// (`FUN_00414330` → `FUN_00414830`): `0x65` enters state 2, which plays
/// `0x66`; `0x66` and the endings enter state 3; `0x67` enters state 7, the
/// side's movie; `0x68` and `0x69` enter state 8. Any other id goes to the
/// child window: the strategic window for a message movie, the tactical
/// window for the trench run, the menu for the Credits movie (its callback
/// `FUN_00406520`).
#[must_use]
pub const fn after(movie: u8, side: Faction) -> Scene {
    match movie {
        0x65 => Scene::Movie(0x66),
        0x66 | 0x6a..=0x6d => Scene::MainMenu,
        0x67 => Scene::Movie(side_movie(side)),
        0x68 | 0x69 => Scene::Game,
        _ => Scene::Child,
    }
}

/// Where the game goes once `movie` ends, skipping each next movie that
/// `open` cannot open: a movie whose file is missing closes at once
/// (`FUN_005f6920` posts `0x403`, which `FUN_00421790` handles as the end).
pub fn after_movie(movie: u8, side: Faction, mut open: impl FnMut(u8) -> bool) -> Scene {
    let mut movie = movie;
    loop {
        match after(movie, side) {
            Scene::Movie(next) if !open(next) => movie = next,
            scene => return scene,
        }
    }
}

/// A kind-6 game message (`FUN_0048a060` files it on the manager's `+0x28`
/// list, never in the Message Index): its movie (`+0x5c`) and the body
/// (`+0x50`) the movie window shows as its caption (`FUN_00422ce0` →
/// `FUN_004019a0(entry + 0x5c, entry + 0x50, 0)`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageMovie {
    pub movie: u8,
    pub caption: String,
}

/// What the strategic window plays next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Showing {
    /// A message movie; the game resumes after it (`FUN_00422ce0`, `0x402`).
    Message(MessageMovie),
    /// The ending movie, after the game's teardown (`FUN_00414330`).
    Ending(u8),
}

/// The local player's message movies and the strategic window's ending
/// fields.
#[derive(Debug, Default)]
pub struct MovieQueue {
    /// The message manager's `+0x28` list.
    messages: VecDeque<MessageMovie>,
    /// Strategic window `+0xac` (`FUN_0041d810`): the window posts no more
    /// day advances `0x15e` (`FUN_0042db70`).
    days_stopped: bool,
    /// Strategic window `+0xa8`: the ending code, 0 for none.
    ending: u8,
}

impl MovieQueue {
    /// File kind-6 messages at the list's tail.
    pub fn file(&mut self, movies: impl IntoIterator<Item = MessageMovie>) {
        self.messages.extend(movies);
    }

    /// The advisor stamped its ending slot (`FUN_004c3010`, `FUN_004c0f10`
    /// → `FUN_0041d810`): the clock stops.
    pub fn stop_days(&mut self) {
        self.days_stopped = true;
    }

    /// Whether the clock has stopped for the ending.
    #[must_use]
    pub const fn days_stopped(&self) -> bool {
        self.days_stopped
    }

    /// The ending script finished (cockpit step 3, `FUN_004c3060` →
    /// `FUN_0041d830`): store the ending code of `word` for `viewer`.
    pub fn end(&mut self, word: u32, viewer: Faction) {
        self.ending = ending_code(word, viewer);
    }

    /// Whether a movie waits to play: a message's, or the stored ending.
    #[must_use]
    pub fn waiting(&self) -> bool {
        !self.messages.is_empty() || ending_movie(self.ending).is_some()
    }

    /// The movie to play now. Nothing plays until the window is `ready`
    /// (`FUN_00601340`: no capture and no modal). A message comes first:
    /// the session pump plays the list's head (`FUN_00488b90`) and runs the
    /// advisor's pass, which ends the script, only with no message pending
    /// (`FUN_00486520`). Then the ending (`FUN_0042db70` posts `0x482` with
    /// the code).
    pub fn next(&mut self, ready: bool) -> Option<Showing> {
        if !ready {
            return None;
        }
        if let Some(message) = self.messages.pop_front() {
            return Some(Showing::Message(message));
        }
        let movie = ending_movie(std::mem::take(&mut self.ending))?;
        Some(Showing::Ending(movie))
    }
}

/// Clear the screen for a waiting movie, as message `0x482` does before it
/// plays (`FUN_00422ce0.c:1192-1232`): `FUN_00601340` sends `0x40d` to the
/// capture window, else the modal stack's top, until there is none. The
/// Status window (`FUN_0044b250`), the Encyclopedia (`FUN_0045d8f0`) and the
/// Message Index (`FUN_00467e80`, vtable `0x65a1c0`) close on it (slot
/// `+0x30`); every other window ignores it, the Operation Report
/// (`FUN_00606650`) among them, so the movie waits until the player closes
/// that one. The capture is cancelled as the movie's scene starts. Returns
/// the closing Index's actions (`FUN_0046a6a0`).
pub fn clear_for_movie(
    queue: &MovieQueue,
    status: &mut StatusWindowState,
    encyclopedia: &mut EncyclopediaSurfaceController,
    index: &mut Option<MessageIndexState>,
) -> Vec<MessageIndexAction> {
    if !queue.waiting() {
        return Vec::new();
    }
    *status = StatusWindowState::default();
    encyclopedia.close();
    index.take().map(|index| index.close()).unwrap_or_default()
}

/// The movie window's caption field (`FUN_00421790`, `WM_CREATE`): a
/// TextScrollField at (4, 404), 580 by 70, in the 640 by 480 window, white
/// (`0x2ffffff`) font 4, made only for a caption.
const CAPTION: (f32, f32, f32, f32) = (4.0, 404.0, 580.0, 70.0);
/// The movie window's logical size.
const WINDOW: egui::Vec2 = egui::vec2(640.0, 480.0);

/// Where a message movie draws on `screen`: its `frame` scaled with the 640
/// by 480 window, and the caption field. hyp: the frame is centred in the
/// window; the 640 by 324 movies then end at y 402, just above the caption.
#[must_use]
pub fn message_layout(screen: egui::Rect, frame: egui::Vec2) -> (egui::Rect, egui::Rect, f32) {
    let canvas = rebellion_render::main_menu::main_menu_canvas_rect(screen);
    let scale = canvas.width() / WINDOW.x;
    let size = frame * scale;
    let picture = egui::Rect::from_center_size(canvas.center(), size);
    let (x, y, width, height) = CAPTION;
    let caption = egui::Rect::from_min_size(
        canvas.min + egui::vec2(x, y) * scale,
        egui::vec2(width, height) * scale,
    );
    (picture, caption, scale)
}

/// The staged message movies' frame: MDATA 101-104 are 640 by 324.
pub const MESSAGE_FRAME: egui::Vec2 = egui::vec2(640.0, 324.0);

/// The whole screen, as `video_bounds` gives it.
pub const FULL_SCREEN: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

/// Where the browser shell shows a movie's `<video>`, as fractions of
/// `screen` (left, top, width, height): a message movie's picture
/// (`message_layout`), which leaves the caption field to the canvas, or the
/// whole screen.
#[must_use]
pub fn video_bounds(message: bool, screen: egui::Rect) -> [f32; 4] {
    if !message {
        return FULL_SCREEN;
    }
    let (picture, _, _) = message_layout(screen, MESSAGE_FRAME);
    [
        (picture.min.x - screen.min.x) / screen.width(),
        (picture.min.y - screen.min.y) / screen.height(),
        picture.width() / screen.width(),
        picture.height() / screen.height(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_waiting_movie_closes_the_status_window_the_encyclopedia_and_the_index() {
        // FUN_00422ce0.c:1192-1232 loops FUN_00601340 (0x40d to the modal
        // top) before a movie; FUN_0044b250, FUN_0045d8f0 and FUN_00467e80
        // close on it, and nothing closes while no movie waits.
        let open = || {
            let mut status = StatusWindowState::default();
            status.open_character(rebellion_core::ids::CharacterKey::default());
            let mut encyclopedia = EncyclopediaSurfaceController::new();
            encyclopedia
                .open(
                    &crate::encyclopedia_surface::tests::session(),
                    rebellion_data::encyclopedia_presenter::EncyclopediaEntryIntent::Cockpit {
                        audience: rebellion_data::encyclopedia_topics::EncyclopediaAudience::Empire,
                    },
                )
                .unwrap();
            (status, encyclopedia, Some(MessageIndexState::new()))
        };

        let mut queue = MovieQueue::default();
        let (mut status, mut encyclopedia, mut index) = open();
        let actions = clear_for_movie(&queue, &mut status, &mut encyclopedia, &mut index);
        assert!(actions.is_empty());
        assert!(status.is_open() && encyclopedia.is_open() && index.is_some());

        queue.file([MessageMovie {
            movie: CORUSCANT_CAPTURED,
            caption: String::new(),
        }]);
        assert!(queue.waiting());
        let expected = MessageIndexState::new().close();
        let actions = clear_for_movie(&queue, &mut status, &mut encyclopedia, &mut index);
        assert_eq!(actions, expected);
        assert!(!status.is_open());
        assert!(!encyclopedia.is_open());
        assert!(index.is_none());
    }

    #[test]
    fn the_stored_ending_waits_like_a_message_movie() {
        // FUN_0042db70 posts 0x482 with the ending code, which runs the same
        // FUN_00601340 loop; a played movie leaves nothing waiting.
        let mut queue = MovieQueue::default();
        assert!(!queue.waiting());
        queue.end(EW, Faction::Empire);
        assert!(queue.waiting());
        assert!(matches!(queue.next(true), Some(Showing::Ending(_))));
        assert!(!queue.waiting());
    }

    #[test]
    fn a_message_movie_leaves_its_caption_field_to_the_canvas() {
        // FUN_00421790: the caption field at (4, 404) below the picture; the
        // browser's <video> covers the picture alone. A full-screen movie
        // (the intro, the endings, the trench run) takes the whole screen.
        for screen in [
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 960.0)),
            egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(1920.0, 1080.0)),
        ] {
            let (picture, caption, _) = message_layout(screen, MESSAGE_FRAME);
            let [left, top, width, height] = video_bounds(true, screen);
            let shown = egui::Rect::from_min_size(
                screen.min + egui::vec2(left * screen.width(), top * screen.height()),
                egui::vec2(width * screen.width(), height * screen.height()),
            );
            assert!((shown.min - picture.min).length() < 0.01, "{screen:?}");
            assert!((shown.max - picture.max).length() < 0.01, "{screen:?}");
            assert!(shown.max.y <= caption.min.y, "{screen:?}");
            assert_eq!(video_bounds(false, screen), [0.0, 0.0, 1.0, 1.0]);
        }
    }

    #[test]
    fn each_movie_id_opens_its_mdata_file() {
        // FUN_00421ba0: the closed id-to-file switch.
        let files: Vec<_> = (0x64..=0x75).map(file_number).collect();
        assert_eq!(
            files,
            [
                None,
                Some(0),
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(106),
                Some(105),
                Some(107),
                Some(108),
                Some(5),
                Some(101),
                Some(202),
                Some(201),
                Some(104),
                Some(102),
                Some(103),
                None,
            ]
        );
        assert_eq!(
            path(HEADQUARTERS_DESTROYED).as_deref(),
            Some("assets/references/ref-videos/102.webm")
        );
        assert_eq!(
            path(0x65).as_deref(),
            Some("assets/references/ref-videos/000.webm")
        );
        assert_eq!(path(0x75), None);
        // FUN_004950d0 and FUN_0048d470 store 0x6f and 0x72: MDATA 101, 104.
        assert_eq!(
            [SYSTEM_DESTROYED, DEATH_STAR_SABOTAGED].map(file_number),
            [Some(101), Some(104)]
        );
    }

    #[test]
    fn keys_end_every_movie_but_the_trench_run_pair() {
        // FUN_00421790: WM_KEYDOWN returns for ids 0x70 and 0x71.
        let keyed: Vec<_> = (0x65..=0x74).filter(|&id| !keys_dismiss(id)).collect();
        assert_eq!(keyed, [0x70, 0x71]);
    }

    #[test]
    fn the_ending_code_follows_the_winner_and_the_viewer() {
        // FUN_00425c90: EW -> (viewer == 2) + 0x11; AW -> ((viewer != 1) & 3)
        // + 0x10; neither -> 0.
        use rebellion_core::victory::{AD, GO};
        assert_eq!(ending_code(EW | GO | AD, Faction::Alliance), 0x11);
        assert_eq!(ending_code(EW | GO | AD, Faction::Empire), 0x12);
        assert_eq!(ending_code(AW | GO, Faction::Alliance), 0x10);
        assert_eq!(ending_code(AW | GO, Faction::Empire), 0x13);
        assert_eq!(ending_code(AD, Faction::Alliance), 0);
        assert_eq!(ending_code(AD, Faction::Empire), 0);
    }

    #[test]
    fn each_ending_code_plays_its_ending_movie() {
        // FUN_00414830 states 0x10-0x13, then FUN_00421ba0: MDATA 105, 106,
        // 108, 107.
        let movies: Vec<_> = (0x0f..=0x14).map(ending_movie).collect();
        assert_eq!(
            movies,
            [None, Some(0x6b), Some(0x6a), Some(0x6d), Some(0x6c), None]
        );
        let files: Vec<_> = (0x10..=0x13)
            .filter_map(ending_movie)
            .filter_map(file_number)
            .collect();
        assert_eq!(files, [105, 106, 108, 107]);
    }

    const SIDES: [Faction; 2] = [Faction::Alliance, Faction::Empire];

    #[test]
    fn the_intro_plays_its_second_movie_then_the_main_menu() {
        // FUN_004138a0 0x405: 0x65 enters state 2, which FUN_00414830 plays
        // as 0x66; 0x66 enters state 3, the menu. No side is read.
        for side in SIDES {
            assert_eq!(after(INTRO, side), Scene::Movie(0x66));
            assert_eq!(after(0x66, side), Scene::MainMenu);
        }
        assert_eq!(
            path(INTRO).as_deref(),
            Some("assets/references/ref-videos/000.webm")
        );
        assert_eq!(file_number(0x66), Some(1));
    }

    #[test]
    fn each_side_starts_its_game_with_its_own_movie_then_the_command_center() {
        // FUN_00414830 state 7: (+0x108 != 1) + 0x68; FUN_004138a0 0x405:
        // 0x68 and 0x69 enter state 8 (FUN_00414eb0), 0x67 enters state 7.
        assert_eq!(side_movie(Faction::Alliance), 0x68);
        assert_eq!(side_movie(Faction::Empire), 0x69);
        for side in SIDES {
            assert_eq!(after(0x68, side), Scene::Game);
            assert_eq!(after(0x69, side), Scene::Game);
            assert_eq!(after(0x67, side), Scene::Movie(side_movie(side)));
        }
        assert_eq!(
            [0x68, 0x69].map(file_number),
            [Some(3), Some(4)],
            "FUN_00421ba0: MDATA.003 and MDATA.004"
        );
    }

    #[test]
    fn the_endings_return_to_the_main_menu_and_the_rest_to_their_window() {
        // FUN_004138a0 0x405: 0x6a-0x6d enter state 3; any other id posts
        // 0x402 to the child window (the Credits movie's FUN_00406520, the
        // strategic window, the tactical window).
        for side in SIDES {
            for ending in 0x6a..=0x6d {
                assert_eq!(after(ending, side), Scene::MainMenu, "{ending:#x}");
            }
            for child in 0x6e..=0x74 {
                assert_eq!(after(child, side), Scene::Child, "{child:#x}");
            }
        }
        assert_eq!(file_number(CREDITS), Some(5));
    }

    #[test]
    fn a_next_movie_that_cannot_open_ends_at_once() {
        // FUN_005f6920 posts 0x403 for an unknown path; FUN_00421790 treats
        // it as the end and posts 0x405, so the next state follows at once.
        for side in SIDES {
            let mut tried = Vec::new();
            let scene = after_movie(INTRO, side, |next| {
                tried.push(next);
                false
            });
            assert_eq!((scene, tried), (Scene::MainMenu, vec![0x66]));
            assert_eq!(after_movie(INTRO, side, |_| true), Scene::Movie(0x66));
            assert_eq!(
                after_movie(0x67, side, |_| false),
                Scene::Game,
                "the side's movie missing, the game starts"
            );
            assert_eq!(after_movie(CREDITS, side, |_| true), Scene::Child);
        }
    }

    fn movie(id: u8) -> MessageMovie {
        MessageMovie {
            movie: id,
            caption: format!("{id:#x}"),
        }
    }

    #[test]
    fn message_movies_wait_for_a_free_window_then_play_in_filing_order() {
        // FUN_00422ce0 0x482: the modal pump waits on FUN_00601340;
        // FUN_00488b90 plays the head of list +0x28.
        let mut queue = MovieQueue::default();
        queue.file([movie(HEADQUARTERS_DESTROYED), movie(CORUSCANT_CAPTURED)]);
        assert_eq!(queue.next(false), None);
        assert_eq!(
            queue.next(true),
            Some(Showing::Message(movie(HEADQUARTERS_DESTROYED)))
        );
        assert_eq!(
            queue.next(true),
            Some(Showing::Message(movie(CORUSCANT_CAPTURED)))
        );
        assert_eq!(queue.next(true), None);
    }

    #[test]
    fn the_ending_plays_once_after_the_pending_messages_for_either_viewer() {
        // FUN_00486520 runs the advisor pass only with no message pending;
        // FUN_0042db70 posts 0x482 with the code at +0xa8.
        for (viewer, word, ending) in [
            (Faction::Alliance, AW, 0x6b),
            (Faction::Alliance, EW, 0x6a),
            (Faction::Empire, EW, 0x6d),
            (Faction::Empire, AW, 0x6c),
        ] {
            let mut queue = MovieQueue::default();
            queue.end(word, viewer);
            queue.file([movie(HEADQUARTERS_DESTROYED)]);
            assert_eq!(queue.next(false), None);
            assert_eq!(
                queue.next(true),
                Some(Showing::Message(movie(HEADQUARTERS_DESTROYED)))
            );
            assert_eq!(queue.next(true), Some(Showing::Ending(ending)));
            assert_eq!(queue.next(true), None);
        }
    }

    #[test]
    fn an_ending_step_without_a_win_bit_plays_nothing() {
        // FUN_00425c90 stores 0 without EW or AW; FUN_0042db70 then posts
        // the day advance instead of 0x482.
        let mut queue = MovieQueue::default();
        queue.end(rebellion_core::victory::AD, Faction::Empire);
        assert_eq!(queue.next(true), None);
    }

    #[test]
    fn the_clock_stops_only_once_the_ending_slot_is_stamped() {
        // FUN_0041d810 sets strategic +0xac; FUN_0042db70 then stops 0x15e.
        let mut queue = MovieQueue::default();
        assert!(!queue.days_stopped());
        queue.stop_days();
        assert!(queue.days_stopped());
    }

    #[test]
    fn the_caption_field_sits_under_the_centred_frame_in_the_window() {
        // FUN_00421790 WM_CREATE: the caption field at (4, 404), 580 by 70.
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 960.0));
        let (picture, caption, scale) = message_layout(screen, egui::vec2(640.0, 324.0));
        assert!((scale - 2.0).abs() < f32::EPSILON);
        assert_eq!(
            picture,
            egui::Rect::from_min_size(egui::pos2(0.0, 156.0), egui::vec2(1280.0, 648.0))
        );
        assert_eq!(
            caption,
            egui::Rect::from_min_size(egui::pos2(8.0, 808.0), egui::vec2(1160.0, 140.0))
        );
        let wide = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 480.0));
        let (picture, caption, scale) = message_layout(wide, egui::vec2(640.0, 324.0));
        assert!((scale - 1.0).abs() < f32::EPSILON);
        assert_eq!(picture.min, egui::pos2(320.0, 78.0));
        assert_eq!(caption.min, egui::pos2(324.0, 404.0));
    }
}
