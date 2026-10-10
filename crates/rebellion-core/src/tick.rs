//! Frame-independent game clock and tick system.
//!
//! The tick is the fundamental unit of simulation time in Open Rebellion.
//! One tick equals one game-day at every speed. All simulation systems
//! (manufacturing, missions, AI, events) advance in whole ticks — never
//! fractional ones — so game state is fully deterministic given the same
//! sequence of tick counts.
//!
//! # Usage
//!
//! ```
//! use rebellion_core::tick::{GameClock, GameSpeed};
//!
//! let mut clock = GameClock::new();
//! clock.set_speed(GameSpeed::Medium);
//!
//! // Each frame, pass the real elapsed seconds.
//! let events = clock.advance(1.0 / 60.0);
//! // events.len() == number of ticks that elapsed this frame.
//! ```

/// The original Game Speed menu's choices, in menu order.
///
/// `FUN_0042d190` builds Pause, Very Slow, Slow, Medium, and Fast; the
/// original has no faster setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameSpeed {
    /// Simulation frozen. No ticks advance.
    Paused,
    VerySlow,
    Slow,
    Medium,
    Fast,
}

impl GameSpeed {
    /// Every speed in original menu order.
    pub const ALL: [Self; 5] = [
        Self::Paused,
        Self::VerySlow,
        Self::Slow,
        Self::Medium,
        Self::Fast,
    ];

    /// Rate value `FUN_00487eb0` sends in game message `0x280`.
    ///
    /// Paused has no rate; the original routes it through `FUN_0041d2f0`.
    #[must_use]
    pub const fn original_rate(self) -> Option<u16> {
        match self {
            Self::Paused => None,
            Self::VerySlow => Some(600),
            Self::Slow => Some(60),
            Self::Medium => Some(12),
            Self::Fast => Some(4),
        }
    }

    /// Real seconds per game-day, or `None` while paused.
    #[must_use]
    pub fn seconds_per_day(self) -> Option<f32> {
        self.original_rate()
            .map(|rate| f32::from(rate) * ORIGINAL_RATE_UNIT_SECONDS + DAY_OVERHEAD_SECONDS)
    }

    /// The scheduler steps a second at this speed: `FUN_004fcee0`'s count
    /// advances by the rate each day. `None` while paused.
    #[must_use]
    pub fn steps_per_second(self) -> Option<f32> {
        self.original_rate()
            .zip(self.seconds_per_day())
            .map(|(rate, seconds)| f32::from(rate) / seconds)
    }

    /// Next faster running speed; Fast and Paused are unchanged.
    #[must_use]
    pub const fn faster(self) -> Self {
        match self {
            Self::VerySlow => Self::Slow,
            Self::Slow => Self::Medium,
            Self::Medium | Self::Fast => Self::Fast,
            Self::Paused => Self::Paused,
        }
    }

    /// Next slower running speed; Very Slow and Paused are unchanged.
    #[must_use]
    pub const fn slower(self) -> Self {
        match self {
            Self::Fast => Self::Medium,
            Self::Medium => Self::Slow,
            Self::Slow | Self::VerySlow => Self::VerySlow,
            Self::Paused => Self::Paused,
        }
    }
}

/// Real seconds represented by one unit of [`GameSpeed::original_rate`].
///
/// src: measured in the original under Wine (2026-10-08, A0 capture VM): a day
/// took 1.19-1.21 s at Fast, 3.24 s at Medium, 15.1 s at Slow and 150.6 s at
/// Very Slow, which is 0.25 s per rate unit plus [`DAY_OVERHEAD_SECONDS`].
/// The reader that turns the rate into days is still untraced.
pub const ORIGINAL_RATE_UNIT_SECONDS: f32 = 0.25;

/// The measured remainder of each day beyond its rate units (see
/// [`ORIGINAL_RATE_UNIT_SECONDS`]); hyp: one 200 ms poll of the strategic
/// timer before the day boundary is seen.
pub const DAY_OVERHEAD_SECONDS: f32 = 0.2;

/// A single completed game-day tick.
///
/// Currently a marker — future phases will carry a payload (manufacturing
/// completion, mission resolution, event triggers) so downstream systems
/// can process each day's outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickEvent {
    /// The tick counter value at the moment this day completed.
    pub tick: u64,
}

/// Frame-independent game clock that drives all simulation timing.
///
/// Accumulates the fraction of a game-day that elapsed real time covers at
/// the current speed. Each whole day emits a `TickEvent`; several can fire in
/// one `advance` call when `dt` is large. A speed change keeps the partial day.
///
/// # Determinism
///
/// Given the same sequence of `(speed, dt)` pairs, `advance` always
/// produces the same sequence of tick counts. The accumulator is purely
/// additive — no floating-point branching outside of the subtraction loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameClock {
    /// Total game-days elapsed since the scenario started.
    pub tick: u64,
    /// Current simulation speed.
    pub speed: GameSpeed,
    /// Fraction of the current game-day already elapsed, in `[0, 1)`.
    accumulator: f32,
    /// Day the clock holds at after a pause, as `DAT_006b1218+0x10` does.
    /// The speed is kept, so resuming continues at it.
    stop_day: Option<u64>,
}

impl GameClock {
    /// Create a new clock, paused at day zero.
    #[must_use]
    pub fn new() -> Self {
        GameClock {
            tick: 0,
            speed: GameSpeed::Paused,
            accumulator: 0.0,
            stop_day: None,
        }
    }

    /// Change the simulation speed. Takes effect on the next `advance` call.
    pub fn set_speed(&mut self, speed: GameSpeed) {
        self.speed = speed;
    }

    /// Pause as `FUN_0041d2f0` does: the current day finishes, then the clock
    /// holds until [`resume`](Self::resume).
    ///
    /// The stop only ever moves later (`FUN_0041e290`), so pausing twice in
    /// one day changes nothing. Returns whether a new stop was set. A clock
    /// already at [`GameSpeed::Paused`] has no day to finish.
    pub fn pause(&mut self) -> bool {
        let stop = self.tick + 1;
        if self.speed == GameSpeed::Paused || self.stop_day.is_some_and(|day| day >= stop) {
            return false;
        }
        self.stop_day = Some(stop);
        true
    }

    /// Lift a pause (`FUN_0041e2d0`); play continues at the kept speed.
    pub fn resume(&mut self) {
        self.stop_day = None;
    }

    /// Whether a pause is pending or holding.
    #[must_use]
    pub const fn pause_requested(&self) -> bool {
        self.stop_day.is_some()
    }

    /// Whether the clock is running: a speed is set and no pause holds it.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.speed != GameSpeed::Paused && self.stop_day.is_none_or(|day| self.tick < day)
    }

    /// Advance the clock by `dt` real seconds.
    ///
    /// Returns a `Vec<TickEvent>` — one entry per game-day that completed
    /// during this frame. The vec is empty when paused or when `dt` is
    /// too small to complete a tick. Each event carries the tick number
    /// at the moment that day resolved.
    ///
    /// # Notes
    ///
    /// - `dt` should be the real elapsed frame time (e.g. `macroquad::time::get_frame_time()`).
    /// - Negative or zero `dt` is a no-op.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    pub fn advance(&mut self, dt: f32) -> Vec<TickEvent> {
        let Some(seconds_per_day) = self.speed.seconds_per_day() else {
            return Vec::new();
        };
        if dt <= 0.0 || self.stop_day.is_some_and(|day| self.tick >= day) {
            return Vec::new();
        }

        self.accumulator += dt / seconds_per_day;

        // How many full days completed this frame?
        let mut ticks_elapsed = self.accumulator.floor() as u64;

        if ticks_elapsed == 0 {
            return Vec::new();
        }

        // Consume whole days from the accumulator, holding at the stop day.
        match self.stop_day {
            Some(day) if self.tick + ticks_elapsed >= day => {
                ticks_elapsed = day - self.tick;
                self.accumulator = 0.0;
            }
            _ => self.accumulator -= ticks_elapsed as f32,
        }

        let events = (0..ticks_elapsed)
            .map(|i| {
                let tick_number = self.tick + i + 1;
                TickEvent { tick: tick_number }
            })
            .collect();

        self.tick += ticks_elapsed;

        events
    }
}

impl Default for GameClock {
    fn default() -> Self {
        Self::new()
    }
}

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() < 1e-4
    }

    #[test]
    fn pause_finishes_the_current_day_then_holds() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Medium);
        assert!(clock.advance(1.6).is_empty());
        assert!(clock.pause());
        // The half-finished day completes; the next never starts.
        assert_eq!(clock.advance(1.6), vec![TickEvent { tick: 1 }]);
        assert!(clock.advance(10.0).is_empty());
        assert_eq!(clock.tick, 1);
        assert_eq!(clock.speed, GameSpeed::Medium);

        clock.resume();
        assert!(!clock.pause_requested());
        assert!(clock.advance(3.1).is_empty());
        assert_eq!(clock.advance(0.2), vec![TickEvent { tick: 2 }]);
    }

    #[test]
    fn a_large_frame_stops_at_the_pause_day() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Fast);
        clock.pause();
        assert_eq!(clock.advance(12.0), vec![TickEvent { tick: 1 }]);
        assert_eq!(clock.tick, 1);
    }

    #[test]
    fn pausing_again_never_moves_the_stop() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Slow);
        assert!(clock.pause());
        assert!(!clock.pause());
        // A speed change keeps the stop, as `FUN_00487eb0` does.
        clock.set_speed(GameSpeed::Fast);
        assert!(clock.pause_requested());
        clock.advance(30.0);
        assert_eq!(clock.tick, 1);
    }

    #[test]
    fn a_pause_later_in_the_game_stops_one_day_ahead() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Fast);
        clock.advance(3.9);
        assert_eq!(clock.tick, 3);
        clock.pause();
        assert_eq!(clock.advance(12.0), vec![TickEvent { tick: 4 }]);
        assert_eq!(clock.tick, 4);
    }

    #[test]
    fn negative_time_neither_ticks_nor_delays_the_next_day() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Medium);
        assert!(clock.advance(-5.0).is_empty());
        assert!(clock.advance(0.0).is_empty());
        assert_eq!(clock.advance(3.3), vec![TickEvent { tick: 1 }]);
    }

    #[test]
    fn a_clock_at_paused_speed_has_nothing_to_stop() {
        let mut clock = GameClock::new();
        assert!(!clock.pause());
        assert!(!clock.pause_requested());
    }

    #[test]
    fn paused_emits_no_ticks() {
        let mut clock = GameClock::new();
        // clock starts Paused
        let events = clock.advance(10.0);
        assert!(events.is_empty());
        assert_eq!(clock.tick, 0);
    }

    #[test]
    fn speeds_follow_recovered_menu_order_and_rates() {
        assert_eq!(
            GameSpeed::ALL.map(GameSpeed::original_rate),
            [None, Some(600), Some(60), Some(12), Some(4)]
        );
        let seconds = GameSpeed::ALL.map(GameSpeed::seconds_per_day);
        assert!(seconds[0].is_none());
        // Measured under Wine on 2026-10-08: 150.6, 15.1, 3.24 and 1.2 s.
        for (actual, expected) in seconds[1..].iter().zip([150.2, 15.2, 3.2, 1.2]) {
            assert!(close(actual.unwrap(), expected));
        }
    }

    #[test]
    fn medium_speed_one_tick_per_day_length() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Medium);

        // Just under a full day — no event yet.
        let events = clock.advance(3.19);
        assert!(events.is_empty());
        assert_eq!(clock.tick, 0);

        // Push over the threshold.
        let events = clock.advance(0.02);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].tick, 1);
        assert_eq!(clock.tick, 1);
    }

    #[test]
    fn fast_speed_completes_five_days_in_six_seconds() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Fast);

        let events = clock.advance(6.01);
        assert_eq!(events.len(), 5);
        assert_eq!(clock.tick, 5);
    }

    #[test]
    fn very_slow_speed_needs_two_and_a_half_minutes_per_day() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::VerySlow);

        assert!(clock.advance(150.1).is_empty());
        assert_eq!(clock.advance(0.2).len(), 1);
    }

    #[test]
    fn tick_events_carry_correct_numbers() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Fast);

        // 7.21 s at 1.2 s/day = 6 days.
        let events = clock.advance(7.21);
        assert_eq!(events.len(), 6);
        for (i, event) in events.iter().enumerate() {
            assert_eq!(event.tick, (i + 1) as u64);
        }
    }

    #[test]
    fn accumulator_carries_remainder() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Slow);

        // 9 s of a 15.2 s day — no tick yet.
        assert!(clock.advance(9.0).is_empty());

        // 18 s total — one tick fires, 2.8 s remains.
        assert_eq!(clock.advance(9.0).len(), 1);
        assert_eq!(clock.tick, 1);

        // 11.8 s of the next day — no tick yet.
        assert!(clock.advance(9.0).is_empty());
        assert_eq!(clock.tick, 1);
    }

    #[test]
    fn speed_change_keeps_partial_day() {
        let mut clock = GameClock::new();
        clock.set_speed(GameSpeed::Slow);
        clock.advance(7.6); // half of a 15.2 s day

        clock.set_speed(GameSpeed::Fast);
        // 0.61 s at 1.2 s/day adds just over half a day.
        let events = clock.advance(0.61);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn stepping_clamps_at_running_extremes_and_leaves_pause() {
        assert_eq!(GameSpeed::VerySlow.faster(), GameSpeed::Slow);
        assert_eq!(GameSpeed::Fast.faster(), GameSpeed::Fast);
        assert_eq!(GameSpeed::Fast.slower(), GameSpeed::Medium);
        assert_eq!(GameSpeed::VerySlow.slower(), GameSpeed::VerySlow);
        assert_eq!(GameSpeed::Paused.faster(), GameSpeed::Paused);
        assert_eq!(GameSpeed::Paused.slower(), GameSpeed::Paused);
    }

}
