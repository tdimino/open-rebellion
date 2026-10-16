//! Opt-in decision traces for the simulation's rules.
//!
//! A trace records the inputs a rule read and the choice it made: a
//! mission's odds and roll, the candidates the AI scored, a research point
//! accrual, a blockade's presence flags, a battle's pre-battle strength, a
//! control change's inputs. Each subsystem is one bit of a [`TraceMask`];
//! with its bit clear, a trace point costs one thread-local read and a branch
//! and builds nothing.
//!
//! port: tooling, not a rule of the original. A trace only reads: building
//! one never draws from or advances an RNG, reorders a collection, or
//! changes game state, so a run's fingerprints are identical with every
//! subsystem on and with all off. The records are data. The data and app
//! layers call [`drain`] after each tick and write them as telemetry.
//!
//! The mask and the buffer are per thread: the thread that runs the
//! simulation sets the mask, and drains its own records. Another thread,
//! such as a parallel test, sees its own mask, clear by default.

use std::cell::{Cell, RefCell};

/// The set of subsystems that record traces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TraceMask(pub u32);

impl TraceMask {
    pub const NONE: Self = Self(0);
    /// port: a tooling bit, not a rule of the original.
    pub const MISSIONS: Self = Self(1);
    /// port: a tooling bit.
    pub const AI: Self = Self(1 << 1);
    /// port: a tooling bit.
    pub const RESEARCH: Self = Self(1 << 2);
    /// port: a tooling bit.
    pub const BLOCKADE: Self = Self(1 << 3);
    /// port: a tooling bit.
    pub const COMBAT: Self = Self(1 << 4);
    /// port: a tooling bit.
    pub const CONTROL: Self = Self(1 << 5);
    /// port: every tooling bit.
    pub const ALL: Self = Self(0b11_1111);

    /// Every subsystem's toggle name, as `--trace` and
    /// `OPEN_REBELLION_TRACE` spell it.
    pub const NAMES: [(&'static str, Self); 6] = [
        ("missions", Self::MISSIONS),
        ("ai", Self::AI),
        ("research", Self::RESEARCH),
        ("blockade", Self::BLOCKADE),
        ("combat", Self::COMBAT),
        ("control", Self::CONTROL),
    ];

    /// Parse a comma-separated list of subsystem names; `all` selects every
    /// one and an empty list none.
    ///
    /// # Errors
    ///
    /// Returns the unknown name.
    pub fn parse(spec: &str) -> Result<Self, String> {
        let mut mask = Self::NONE;
        for name in spec.split(',').map(str::trim).filter(|n| !n.is_empty()) {
            if name == "all" {
                mask = mask.union(Self::ALL);
                continue;
            }
            let (_, bit) = Self::NAMES
                .iter()
                .find(|(known, _)| *known == name)
                .ok_or_else(|| format!("unknown trace subsystem {name:?}"))?;
            mask = mask.union(*bit);
        }
        Ok(mask)
    }

    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Whether every bit of `other` is set.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// The subsystem name of a single-bit mask.
    #[must_use]
    pub fn name(self) -> &'static str {
        Self::NAMES
            .iter()
            .find(|(_, bit)| *bit == self)
            .map_or("trace", |(name, _)| name)
    }
}

/// One decision record: the subsystem, what was decided, and its inputs.
#[derive(Debug, Clone, PartialEq)]
pub struct TraceRecord {
    pub subsystem: &'static str,
    pub kind: &'static str,
    pub details: serde_json::Value,
}

thread_local! {
    static MASK: Cell<u32> = const { Cell::new(0) };
    static RECORDS: RefCell<Vec<TraceRecord>> = const { RefCell::new(Vec::new()) };
}

/// Choose the subsystems that record.
pub fn set_mask(mask: TraceMask) {
    MASK.with(|bits| bits.set(mask.0));
}

#[must_use]
pub fn mask() -> TraceMask {
    TraceMask(MASK.with(Cell::get))
}

/// Whether `subsystem` records. Guard any work that only feeds a trace.
#[inline]
#[must_use]
pub fn enabled(subsystem: TraceMask) -> bool {
    MASK.with(Cell::get) & subsystem.0 != 0
}

/// Record a decision when `subsystem` is on. `build` runs only then.
#[inline]
pub fn record(subsystem: TraceMask, kind: &'static str, build: impl FnOnce() -> serde_json::Value) {
    if enabled(subsystem) {
        let record = TraceRecord {
            subsystem: subsystem.name(),
            kind,
            details: build(),
        };
        RECORDS.with(|records| records.borrow_mut().push(record));
    }
}

/// Take this thread's records, oldest first.
#[must_use]
pub fn drain() -> Vec<TraceRecord> {
    RECORDS.with(|records| std::mem::take(&mut *records.borrow_mut()))
}
