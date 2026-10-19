//! Standing policies for the developer commands' waits: what a wait does
//! when a modal screen stops the clock.
//!
//! A `Wait until` that the clock cannot satisfy refuses as soon as it is
//! clear the clock is stopped (`DevPolicy::blocker`), naming the cause. With
//! `Battle policy: simulate|retreat` or `Advice policy: close`, a wait
//! answers the modal itself instead, by posting a scripted click or key on
//! the control's own rect through `DevInput`, so the input path is the one a
//! hand would use. Every answered modal logs a `[dev-policy]` line, which
//! `scripts/dev-report.py` shows under the command that was waiting.

use crate::dev_commands::{control_rect, WaitCondition};
use crate::dev_input::DevInput;

/// Frames a stopped clock must stay stopped before a wait refuses, so a
/// modal that closes on its own does not fail the wait.
const BLOCKED_FRAMES: u32 = 30;
/// Frames the game may spend outside the galaxy, with nothing handling it,
/// before a wait refuses.
const OFF_GALAXY_FRAMES: u32 = 120;
/// Frames to let a posted click land (press and release, then the game's
/// reply) before the next one.
const CLICK_FRAMES: u32 = 8;
/// Frames the click that closes the Advice index is given.
const CLOSE_FRAMES: u32 = 30;
/// Clicks on one Battle Alert's button before the policy gives up: a
/// Retreat the alert shows disabled never closes it.
const BUTTON_ATTEMPTS: u8 = 3;
/// Frames a retreat's battle may run: Retreat opens the battle screen with
/// every ship that has a working hyperdrive flagged retreating, and the
/// Results come when they are gone.
const RETREAT_FRAMES: u32 = 3600;
/// Frames after the Advice index closes in which its pause alert is
/// answered.
const ADVICE_ALERT_FRAMES: u32 = 180;

/// What to do when a Battle Alert opens during a wait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BattlePolicy {
    /// Leave it up: the wait refuses as blocked.
    #[default]
    Stop,
    /// Click Simulate, then close the Results.
    Simulate,
    /// Click Retreat.
    Retreat,
}

/// What to do when the Advice index opens during a wait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AdvicePolicy {
    /// Leave it open: the wait refuses as blocked.
    #[default]
    Stop,
    /// Click `MessageIndex/Close`; the pause alert that follows is answered.
    Close,
}

/// What the game shows this frame, as the policies read it.
#[derive(Debug, Clone)]
pub struct Observed {
    pub day: u64,
    pub mode: String,
    pub galaxy: bool,
    pub main_menu: bool,
    /// The Battle Alert is up, and the system it names.
    pub battle_alert: Option<String>,
    /// The battle's Results window is up.
    pub results: bool,
    pub pause_alert: bool,
    /// The clock is on Paused: no speed, so no day passes.
    pub paused: bool,
    /// The advisor's opening briefing holds the clock.
    pub briefing: bool,
    /// The advisor's ending script has stopped the clock.
    pub ending: bool,
    /// The Message Index shows Advice.
    pub advice_index: bool,
}

/// The standing policies and the counters that act on them.
#[derive(Debug, Default)]
pub struct DevPolicy {
    pub battle: BattlePolicy,
    pub advice: AdvicePolicy,
    /// Dev-only: the clock advances exactly one day each frame.
    pub fast_forward: bool,
    blocked_frames: u32,
    off_galaxy_frames: u32,
    cooldown: u32,
    button_attempts: u8,
    gave_up: bool,
    /// Clicks on this Results window's or Advice index's close control.
    results_attempts: u8,
    advice_attempts: u8,
    advice_alert_frames: u32,
    /// Frames left for the battle a Retreat click opened.
    retreat_frames: u32,
    /// A control a policy needed that the control registry did not file this
    /// frame, and for how many frames running.
    missing: Option<(String, u32)>,
}

impl DevPolicy {
    /// The cause that keeps the clock from advancing for the wait's
    /// condition, once it has held long enough to be sure; `None` while the
    /// wait may still be met. Waits for a battle need no running clock's
    /// help here and never block.
    pub fn blocker(&mut self, observed: &Observed, condition: WaitCondition) -> Option<String> {
        if matches!(
            condition,
            WaitCondition::Battle | WaitCondition::BattleAlert
        ) {
            return None;
        }
        let o = observed;
        let answers_alert = self.battle != BattlePolicy::Stop && !self.gave_up;
        let cause = if let Some(system) = o.battle_alert.as_ref().filter(|_| !answers_alert) {
            Some(format!("battle alert at {system} day={}", o.day))
        } else if o.results && self.results_attempts >= BUTTON_ATTEMPTS {
            Some(format!(
                "Results/Close did not close the Results day={}",
                o.day
            ))
        } else if o.advice_index
            && self.advice == AdvicePolicy::Close
            && self.advice_attempts >= BUTTON_ATTEMPTS
        {
            Some(format!(
                "MessageIndex/Close did not close the Advice index day={}",
                o.day
            ))
        } else if o.pause_alert
            && condition != WaitCondition::PauseAlert
            // The advice policy answers the alert its close brings.
            && !(self.advice == AdvicePolicy::Close && self.advice_alert_frames > 0)
        {
            Some(format!("pause alert (Resume Game Play?) day={}", o.day))
        } else if o.briefing && condition != WaitCondition::BriefingSkipped {
            Some(format!(
                "the advisor's briefing holds the clock day={}",
                o.day
            ))
        } else if o.ending {
            Some(format!("the advisor's ending stops the clock day={}", o.day))
        } else if o.main_menu {
            Some(format!("main menu day={}", o.day))
        } else if o.paused
            && !matches!(
                condition,
                WaitCondition::PauseAlert | WaitCondition::BriefingSkipped
            )
        {
            Some(format!("the clock is paused day={}", o.day))
        } else {
            None
        };
        if let Some((name, frames)) = self.missing.as_ref() {
            if *frames >= BLOCKED_FRAMES {
                return Some(format!("blocked: {name} not registered day={}", o.day));
            }
        }
        self.blocked_frames = if cause.is_some() {
            self.blocked_frames + 1
        } else {
            0
        };
        if self.blocked_frames >= BLOCKED_FRAMES {
            return cause.map(|cause| format!("blocked: {cause}"));
        }
        // A modal a policy is answering keeps the mode off the galaxy for a
        // while; anything else that does is stuck.
        let answering = (o.battle_alert.is_some() && answers_alert)
            || (o.results
                && self.battle != BattlePolicy::Stop
                && self.results_attempts < BUTTON_ATTEMPTS)
            || (self.retreat_frames > 0 && !o.galaxy);
        let off_galaxy = !o.galaxy && condition != WaitCondition::Galaxy && !answering;
        self.off_galaxy_frames = if off_galaxy {
            self.off_galaxy_frames + 1
        } else {
            0
        };
        (self.off_galaxy_frames >= OFF_GALAXY_FRAMES).then(|| {
            format!(
                "mode {} for {OFF_GALAXY_FRAMES} frames day={}",
                o.mode, o.day
            )
        })
    }

    /// Click the control the registry filed under `name` this frame; when it
    /// is not filed, note it for `blocker` and click nothing.
    fn click_control(&mut self, name: &str, input: &mut DevInput) -> bool {
        match control_rect(name) {
            Ok(rect) => {
                let (x, y) = rect.center();
                input.click(x, y, false, false);
                self.missing = None;
                true
            }
            Err(_) => {
                let frames = self
                    .missing
                    .take()
                    .filter(|(known, _)| known == name)
                    .map_or(0, |(_, frames)| frames);
                self.missing = Some((name.to_owned(), frames + 1));
                false
            }
        }
    }

    /// Answer the modal on screen, if a policy covers it. Posts through
    /// `input` and logs `[dev-policy] …`; call once a frame while a wait runs.
    pub fn answer_modals(&mut self, observed: &Observed, input: &mut DevInput) {
        self.advice_alert_frames = self.advice_alert_frames.saturating_sub(1);
        self.retreat_frames = self.retreat_frames.saturating_sub(1);
        if observed.galaxy {
            self.retreat_frames = 0;
        }
        if self.cooldown > 0 {
            self.cooldown -= 1;
            return;
        }
        if input.busy() {
            return;
        }
        let o = observed;
        if let Some(system) = o.battle_alert.as_ref() {
            let (control, name) = match self.battle {
                BattlePolicy::Stop => return,
                BattlePolicy::Simulate => ("BattleAlert/Simulate", "simulate"),
                BattlePolicy::Retreat => ("BattleAlert/Retreat", "retreat"),
            };
            if self.button_attempts >= BUTTON_ATTEMPTS {
                if !self.gave_up {
                    self.gave_up = true;
                    eprintln!(
                        "[dev-policy] battle at {system} day {} -> {name} did not close the alert",
                        o.day
                    );
                }
                return;
            }
            if !self.click_control(control, input) {
                return;
            }
            if self.button_attempts == 0 {
                eprintln!("[dev-policy] battle at {system} day {} -> {name}", o.day);
            }
            self.button_attempts += 1;
            if self.battle == BattlePolicy::Retreat {
                self.retreat_frames = RETREAT_FRAMES;
            }
            self.cooldown = CLICK_FRAMES;
            return;
        }
        self.button_attempts = 0;
        self.gave_up = false;
        if !o.results {
            self.results_attempts = 0;
        }
        if !o.advice_index {
            self.advice_attempts = 0;
        }
        if o.results && self.battle != BattlePolicy::Stop {
            if self.results_attempts >= BUTTON_ATTEMPTS {
                return;
            }
            if self.click_control("Results/Close", input) {
                self.results_attempts += 1;
                if self.battle == BattlePolicy::Retreat {
                    eprintln!("[dev-policy] retreat battle ended day {}", o.day);
                }
                eprintln!("[dev-policy] battle results closed day {}", o.day);
                self.retreat_frames = 0;
                self.cooldown = CLICK_FRAMES;
            }
            return;
        }
        if self.advice != AdvicePolicy::Close {
            self.missing = None;
            return;
        }
        if o.advice_index {
            if self.advice_attempts >= BUTTON_ATTEMPTS {
                return;
            }
            if self.click_control("MessageIndex/Close", input) {
                self.advice_attempts += 1;
                eprintln!("[dev-policy] advice closed day {}", o.day);
                self.advice_alert_frames = ADVICE_ALERT_FRAMES;
                self.cooldown = CLOSE_FRAMES;
            }
        } else if o.pause_alert && self.advice_alert_frames > 0 {
            if self.click_control("PauseAlert/Resume", input) {
                eprintln!("[dev-policy] advice pause alert answered day {}", o.day);
                self.advice_alert_frames = 0;
                self.cooldown = CLICK_FRAMES;
            }
        } else {
            self.missing = None;
        }
    }
}

/// Whether the playing movie gives way to the commands: a script line waiting
/// for the main menu or the galaxy, or a `Wait until galaxy`, ends it as
/// Escape does (`FUN_005f7240` sends `0x404`; `FUN_00421790` closes the
/// movie and `FUN_004138a0` moves to the next state, whose own movie gives
/// way the same frame it opens). A command that runs anywhere (`Capture`,
/// `Wait <n> ms`, `Press`) leaves the movie playing.
#[must_use]
pub fn skips_movie(line_waiting: bool, wait: Option<WaitCondition>) -> bool {
    line_waiting || wait == Some(WaitCondition::Galaxy)
}

/// What `Skip briefing` reads of the advisor's opening briefing.
pub trait Briefing {
    /// The briefing holds the game clock, until its step 11.
    fn holds_clock(&self) -> bool;
    /// The tour's input hooks are in, from step 12 (`FUN_0041d9d0`) to
    /// step 13.
    fn tour_active(&self) -> bool;
    /// Escape on release while the hooks are in (`lpfn_0041d8f0` →
    /// `FUN_0043a200`); it counts once.
    fn skip_tour(&mut self);
}

impl Briefing for rebellion_render::AdvisorState {
    fn holds_clock(&self) -> bool {
        Self::holds_clock(self)
    }

    fn tour_active(&self) -> bool {
        Self::tour_active(self)
    }

    fn skip_tour(&mut self) {
        Self::skip_tour(self);
    }
}

/// One frame of `Skip briefing`'s wait; returns whether it is met. The
/// opening slot fires before its step 12 puts the tour's hooks in, and until
/// then Escape skips nothing (`ghidra/notes/droid-advisor-triggers.md`,
/// "Skipping the tour"), so the skip waits for them, as a hand pressing
/// Escape once the tour starts. The line is met once the briefing lets go
/// of the clock: the skip slot runs step 11 after its reply (ALBRIEF 10165,
/// EMBRIEF 11157).
pub fn skip_briefing(briefing: &mut impl Briefing) -> bool {
    if briefing.tour_active() {
        briefing.skip_tour();
    }
    !briefing.holds_clock()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_waiting_for_the_menu_or_the_galaxy_skips_the_movie_as_escape_does() {
        // FUN_005f7240 0x404 (Escape) ends any movie; only a line that
        // cannot run while it plays, or a wait for the galaxy, sends it.
        assert!(skips_movie(true, None));
        assert!(skips_movie(false, Some(WaitCondition::Galaxy)));
        assert!(!skips_movie(false, None));
        assert!(!skips_movie(false, Some(WaitCondition::Day(3))));
        assert!(!skips_movie(false, Some(WaitCondition::BattleAlert)));
    }

    fn observed(ending: bool) -> Observed {
        Observed {
            day: 9,
            mode: "Galaxy".to_owned(),
            galaxy: true,
            main_menu: false,
            battle_alert: None,
            results: false,
            pause_alert: false,
            paused: false,
            briefing: false,
            ending,
            advice_index: false,
        }
    }

    /// The frame (from 1) on which `blocker` first refuses within
    /// `frames` calls, and its cause.
    fn refusal(
        policy: &mut DevPolicy,
        observed: &Observed,
        condition: WaitCondition,
        frames: u32,
    ) -> Option<(u32, String)> {
        (1..=frames).find_map(|frame| {
            policy
                .blocker(observed, condition)
                .map(|cause| (frame, cause))
        })
    }

    const DAY: WaitCondition = WaitCondition::Day(10);

    #[test]
    fn a_wait_for_a_day_refuses_once_the_ending_has_stopped_the_clock() {
        let mut policy = DevPolicy::default();
        for _ in 1..BLOCKED_FRAMES {
            assert_eq!(policy.blocker(&observed(true), DAY), None);
        }
        assert_eq!(
            policy.blocker(&observed(true), DAY),
            Some("blocked: the advisor's ending stops the clock day=9".to_owned())
        );
        let mut running = DevPolicy::default();
        for _ in 0..=BLOCKED_FRAMES {
            assert_eq!(running.blocker(&observed(false), DAY), None);
        }
    }

    #[test]
    fn a_battle_alert_blocks_a_wait_unless_a_battle_policy_answers_it() {
        let alert = Observed {
            battle_alert: Some("Yavin".to_owned()),
            ..observed(false)
        };
        let blocked = Some((30, "blocked: battle alert at Yavin day=9".to_owned()));
        assert_eq!(refusal(&mut DevPolicy::default(), &alert, DAY, 200), blocked);
        let mut simulate = DevPolicy {
            battle: BattlePolicy::Simulate,
            ..DevPolicy::default()
        };
        assert_eq!(refusal(&mut simulate, &alert, DAY, 200), None);
        // Answering it keeps the mode off the galaxy without refusing.
        let off = Observed {
            galaxy: false,
            mode: "TacticalCombat".to_owned(),
            ..alert.clone()
        };
        assert_eq!(refusal(&mut simulate, &off, DAY, 200), None);
        let mut gave_up = DevPolicy {
            battle: BattlePolicy::Simulate,
            gave_up: true,
            ..DevPolicy::default()
        };
        assert_eq!(refusal(&mut gave_up, &alert, DAY, 200), blocked);
        // Waits for a battle never block.
        assert_eq!(
            refusal(&mut DevPolicy::default(), &alert, WaitCondition::Battle, 200),
            None
        );
    }

    #[test]
    fn results_and_an_advice_index_that_will_not_close_block_after_three_clicks() {
        let results = Observed {
            results: true,
            ..observed(false)
        };
        let stuck = DevPolicy {
            results_attempts: BUTTON_ATTEMPTS,
            ..DevPolicy::default()
        };
        assert_eq!(
            refusal(&mut { stuck }, &results, DAY, 200),
            Some((
                30,
                "blocked: Results/Close did not close the Results day=9".to_owned()
            ))
        );
        let trying = DevPolicy {
            results_attempts: BUTTON_ATTEMPTS - 1,
            ..DevPolicy::default()
        };
        assert_eq!(refusal(&mut { trying }, &results, DAY, 200), None);
        let no_results = DevPolicy {
            results_attempts: BUTTON_ATTEMPTS,
            ..DevPolicy::default()
        };
        assert_eq!(refusal(&mut { no_results }, &observed(false), DAY, 200), None);

        let index = Observed {
            advice_index: true,
            ..observed(false)
        };
        let close = |attempts| DevPolicy {
            advice: AdvicePolicy::Close,
            advice_attempts: attempts,
            ..DevPolicy::default()
        };
        assert_eq!(
            refusal(&mut close(BUTTON_ATTEMPTS), &index, DAY, 200),
            Some((
                30,
                "blocked: MessageIndex/Close did not close the Advice index day=9".to_owned()
            ))
        );
        assert_eq!(refusal(&mut close(BUTTON_ATTEMPTS - 1), &index, DAY, 200), None);
        let left_open = DevPolicy {
            advice_attempts: BUTTON_ATTEMPTS,
            ..DevPolicy::default()
        };
        assert_eq!(refusal(&mut { left_open }, &index, DAY, 200), None);
    }

    #[test]
    fn a_pause_alert_blocks_unless_waited_for_or_answered_by_the_advice_policy() {
        let alert = Observed {
            pause_alert: true,
            ..observed(false)
        };
        let blocked = Some((30, "blocked: pause alert (Resume Game Play?) day=9".to_owned()));
        assert_eq!(refusal(&mut DevPolicy::default(), &alert, DAY, 200), blocked);
        assert_eq!(
            refusal(&mut DevPolicy::default(), &alert, WaitCondition::PauseAlert, 200),
            None
        );
        let answering = |advice, frames| DevPolicy {
            advice,
            advice_alert_frames: frames,
            ..DevPolicy::default()
        };
        assert_eq!(refusal(&mut answering(AdvicePolicy::Close, 5), &alert, DAY, 200), None);
        assert_eq!(
            refusal(&mut answering(AdvicePolicy::Close, 0), &alert, DAY, 200),
            blocked
        );
        assert_eq!(
            refusal(&mut answering(AdvicePolicy::Stop, 5), &alert, DAY, 200),
            blocked
        );
    }

    #[test]
    fn a_paused_clock_blocks_every_wait_but_one_for_the_pause_alert() {
        let paused = Observed {
            paused: true,
            ..observed(false)
        };
        assert_eq!(
            refusal(&mut DevPolicy::default(), &paused, DAY, 200),
            Some((30, "blocked: the clock is paused day=9".to_owned()))
        );
        assert_eq!(
            refusal(&mut DevPolicy::default(), &paused, WaitCondition::PauseAlert, 200),
            None
        );
    }

    #[test]
    fn an_unregistered_control_blocks_once_it_has_been_missing_thirty_frames() {
        let missing = |frames| DevPolicy {
            missing: Some(("Results/Close".to_owned(), frames)),
            ..DevPolicy::default()
        };
        assert_eq!(
            missing(BLOCKED_FRAMES).blocker(&observed(false), DAY),
            Some("blocked: Results/Close not registered day=9".to_owned())
        );
        assert_eq!(missing(BLOCKED_FRAMES - 1).blocker(&observed(false), DAY), None);
    }

    #[test]
    fn a_wait_off_the_galaxy_refuses_after_120_frames_unless_a_policy_is_answering() {
        let off = Observed {
            galaxy: false,
            mode: "MultiplayerSetup".to_owned(),
            ..observed(false)
        };
        let stuck = Some((120, "mode MultiplayerSetup for 120 frames day=9".to_owned()));
        assert_eq!(refusal(&mut DevPolicy::default(), &off, DAY, 200), stuck);
        assert_eq!(refusal(&mut DevPolicy::default(), &observed(false), DAY, 200), None);
        assert_eq!(
            refusal(&mut DevPolicy::default(), &off, WaitCondition::Galaxy, 200),
            None
        );
        // A battle policy alone answers nothing.
        let simulate = || DevPolicy {
            battle: BattlePolicy::Simulate,
            ..DevPolicy::default()
        };
        assert_eq!(refusal(&mut simulate(), &off, DAY, 200), stuck);
        // The Results it is closing.
        let results = Observed {
            results: true,
            ..off.clone()
        };
        assert_eq!(refusal(&mut simulate(), &results, DAY, 200), None);
        let mut trying = DevPolicy {
            results_attempts: BUTTON_ATTEMPTS - 1,
            ..simulate()
        };
        assert_eq!(refusal(&mut trying, &results, DAY, 200), None);
        assert_eq!(
            refusal(&mut DevPolicy::default(), &results, DAY, 200),
            Some((120, "mode MultiplayerSetup for 120 frames day=9".to_owned()))
        );
        // A retreat's battle.
        let retreat = |frames| DevPolicy {
            retreat_frames: frames,
            ..DevPolicy::default()
        };
        assert_eq!(refusal(&mut retreat(5), &off, DAY, 200), None);
        assert_eq!(refusal(&mut retreat(0), &off, DAY, 200), stuck);
    }

    /// The opening briefing frame by frame: the hooks go in at `hooks_at`
    /// (step 12, after the tour's first clip); a skip while they are in
    /// counts once and lets the clock go `reply` frames later (the skip
    /// slot's clip, then step 11); unskipped, the tour lets it go at
    /// `tour`.
    struct TourBriefing {
        frame: u32,
        hooks_at: u32,
        reply: u32,
        tour: u32,
        skipped_at: Option<u32>,
    }

    impl TourBriefing {
        fn released_at(&self) -> u32 {
            self.skipped_at.map_or(self.tour, |at| at + self.reply)
        }
    }

    impl Briefing for TourBriefing {
        fn holds_clock(&self) -> bool {
            self.frame < self.released_at()
        }

        fn tour_active(&self) -> bool {
            self.frame >= self.hooks_at && self.frame < self.released_at()
        }

        fn skip_tour(&mut self) {
            if self.tour_active() && self.skipped_at.is_none() {
                self.skipped_at = Some(self.frame);
            }
        }
    }

    #[test]
    fn a_skip_sent_before_the_tours_hooks_answers_once_the_briefing_lets_go_of_the_clock() {
        // droid-advisor-triggers.md, "Skipping the tour": Escape skips only
        // while step 12's hooks are in; the skip slot's reply (ALBRIEF
        // 10165, 4.3 s; EMBRIEF 11157, 1.3 s) plays before its step 11.
        for (faction, reply) in [
            (rebellion_render::AdvisorFaction::Alliance, 258),
            (rebellion_render::AdvisorFaction::Empire, 78),
        ] {
            // A new game's advisor holds the clock with the hooks not yet in.
            let mut advisor = rebellion_render::AdvisorState::new(faction);
            assert!(!skip_briefing(&mut advisor), "{faction:?}");
            assert!(advisor.holds_clock(), "{faction:?}");
            // A loaded game's advisor holds nothing (FUN_00439320).
            advisor.resume_saved_game(faction);
            assert!(skip_briefing(&mut advisor), "{faction:?}");

            let mut tour = TourBriefing {
                frame: 0,
                hooks_at: 90,
                reply,
                tour: 10_800,
                skipped_at: None,
            };
            let met = (0..tour.tour).find(|_| {
                let met = skip_briefing(&mut tour);
                tour.frame += 1;
                met
            });
            assert_eq!(tour.skipped_at, Some(90), "{faction:?}");
            assert_eq!(met, Some(90 + reply), "{faction:?}");
        }
    }

    #[test]
    fn skip_briefing_s_wait_is_not_blocked_by_the_briefing_or_the_paused_clock_it_waits_out() {
        let briefing = Observed {
            briefing: true,
            paused: true,
            ..observed(false)
        };
        let skip = WaitCondition::BriefingSkipped;
        assert_eq!(
            refusal(&mut DevPolicy::default(), &briefing, skip, 200),
            None
        );
        assert_eq!(
            refusal(&mut DevPolicy::default(), &briefing, DAY, 200),
            Some((
                30,
                "blocked: the advisor's briefing holds the clock day=9".to_owned()
            ))
        );
        let paused = Observed {
            paused: true,
            ..observed(false)
        };
        assert_eq!(refusal(&mut DevPolicy::default(), &paused, skip, 200), None);
        // The game leaving the galaxy still stops it.
        let off = Observed {
            galaxy: false,
            mode: "MainMenu".to_owned(),
            main_menu: true,
            ..briefing
        };
        assert_eq!(
            refusal(&mut DevPolicy::default(), &off, skip, 200),
            Some((30, "blocked: main menu day=9".to_owned()))
        );
        // Fast forward runs no day for it.
        assert!(!skip.needs_clock());
        assert!(DAY.needs_clock());
    }
}
