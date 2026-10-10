//! When the cockpit droids move, and what they play: the side's advice agent
//! and the two droid players (`ghidra/notes/droid-advisor-triggers.md`).
//!
//! The droids rest on one still frame. A game message's advice code, or idle
//! chatter once the game has been quiet, stamps a reaction slot; the agent's
//! pass queues the first live slot's records on the droids, and each droid's
//! player steps its queue every 67 ms, falling back to its rest frame.

use std::collections::{HashMap, VecDeque};

use crate::advisor::AdvisorFaction;

/// The droid players' tick: `FUN_0042adb0` passes `0x43` ms per frame to
/// `FUN_00441a60`, and `FUN_00441d70` steps on it.
pub const TICK_SECONDS: f32 = 0x43 as f32 / 1000.0;

/// Steps since the last reaction before idle chatter may arm
/// (`FUN_00439ef0`: `DAT_006b28d0 + 0x32 < now`).
const CHATTER_QUIET_STEPS: u32 = 0x32;
/// Steps a slot waits after it fires (`FUN_004c2b20`: `+0x16c` = now + 0x3c).
const SLOT_COOLDOWN_STEPS: u32 = 0x3c;
/// The window the agent gives a stamp that should outlast the game.
const FOREVER: u32 = 40000;
/// Ticks the default rest commands hold (`FUN_00472b20(.., 0xf, ..)`).
const REST_HOLD_TICKS: u32 = 0xf;
/// Ticks of the first rest command (`FUN_0042adb0`: command 5, `p2` 5).
const FIRST_HOLD_TICKS: u32 = 5;
/// The partner's sign-off after a turn (`FUN_00411b80`: `FUN_0041d690(0x2d37)`).
const PARTNER_SIGN_OFF: u16 = 0x2d37;

/// `FUN_005fefd0`'s module for the side's sprite DLL (ALSPRITE, EMSPRITE).
pub const SPRITE_MODULE: u16 = 9;
/// `FUN_005fefd0`'s module for the side's briefing DLL (ALBRIEF, EMBRIEF).
pub const BRIEF_MODULE: u16 = 13;

/// One record of an advice slot: an action to queue on the agent droid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionRecord {
    /// The action script's `RT_RCDATA` id.
    pub action: u16,
    /// Bit 0: spoken advice (`DAT_006b28b0` `0x1000` without `0x4000`).
    /// Bit 1: the action comes from the briefing DLL, module 13.
    pub flags: u32,
    pub p2: u32,
    pub p3: u32,
}

impl ActionRecord {
    /// Bit 1: `FUN_0042b1d0` loads the action from module 13 (ALBRIEF.DLL).
    #[must_use]
    pub const fn from_briefing(self) -> bool {
        self.flags & 2 != 0
    }
}

/// A side's advice table, `GData/C3POACT.SPT` or `GData/IMP22ACT.SPT`
/// (`FUN_0049c6f0`, `FUN_004c52d0`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActionTable {
    slots: HashMap<u32, Vec<ActionRecord>>,
}

impl ActionTable {
    /// Parse the table: a `u32` slot count, then per slot `key`, a `u32`,
    /// `count` and a cursor, then `count` records of `u16` action and three
    /// `u32`s. `None` when the bytes run out early.
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let mut reader = Reader { bytes, at: 0 };
        let slot_count = reader.u32()?;
        let mut slots = HashMap::new();
        for _ in 0..slot_count {
            let key = reader.u32()?;
            let _ = reader.u32()?;
            let count = reader.u32()?;
            let _ = reader.u32()?;
            let mut records = Vec::with_capacity(count.min(64) as usize);
            for _ in 0..count {
                records.push(ActionRecord {
                    action: reader.u16()?,
                    flags: reader.u32()?,
                    p2: reader.u32()?,
                    p3: reader.u32()?,
                });
            }
            slots.insert(key, records);
        }
        Some(Self { slots })
    }

    /// The records of `slot`, empty when it has none.
    #[must_use]
    pub fn records(&self, slot: u32) -> &[ActionRecord] {
        self.slots.get(&slot).map_or(&[], Vec::as_slice)
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn u16(&mut self) -> Option<u16> {
        let value = self.bytes.get(self.at..self.at + 2)?;
        self.at += 2;
        Some(u16::from_le_bytes([value[0], value[1]]))
    }

    fn u32(&mut self) -> Option<u32> {
        let value = self.bytes.get(self.at..self.at + 4)?;
        self.at += 4;
        Some(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
    }
}

/// The words of an `RT_RCDATA` script.
#[must_use]
pub fn script_words(bytes: &[u8]) -> Vec<u16> {
    bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .collect()
}

/// The advice messages' codes (`FUN_0048b2e0`) pick one of a run of slots
/// at random, `now + 10`: `(first slot, count, only when none is pending)`.
/// Code `0x2a`: `FUN_004c4430` (Alliance, `+0x114`) and `FUN_004c2270`
/// (Empire, `+0x108`) stamp one of four unless one is already stamped.
/// Code `0x2b`: `FUN_004c4480` (`+0x124`) and `FUN_004c22c0` (`+0x118`)
/// stamp one of two.
const fn advice_code_slots(faction: AdvisorFaction, code: u8) -> Option<(u32, u32, bool)> {
    match (faction, code) {
        (AdvisorFaction::Alliance, 0x2a) => Some((0x114 / 4, 4, true)),
        (AdvisorFaction::Empire, 0x2a) => Some((0x108 / 4, 4, true)),
        (AdvisorFaction::Alliance, 0x2b) => Some((0x124 / 4, 2, false)),
        (AdvisorFaction::Empire, 0x2b) => Some((0x118 / 4, 2, false)),
        _ => None,
    }
}

/// Where a game message's advice code lands: `FUN_004c44b0` (Alliance) and
/// `FUN_004c22f0` (Empire) stamp `(slot, window)`.
#[must_use]
pub const fn code_slot(faction: AdvisorFaction, code: u8) -> Option<(u32, u32)> {
    match faction {
        AdvisorFaction::Alliance => Some(match code {
            1 => (52, 10),
            2 => (53, 10),
            3 => (56, 5),
            4 => (58, 10),
            5 => (59, 10),
            6 => (61, 10),
            8 => (60, 10),
            9 => (62, 10),
            0xc => (57, 10),
            0xd => (65, 10),
            0xe => (67, 10),
            0x14 => (42, 10),
            0x15 => (66, 10),
            0x16 => (45, FOREVER),
            0x17 => (43, FOREVER),
            0x18 => (44, FOREVER),
            0x19 => (46, FOREVER),
            0x1a | 0x1b => (42, FOREVER),
            0x1c => (55, 10),
            0x1e => (28, FOREVER),
            0x1f => (27, FOREVER),
            0x20 => (29, FOREVER),
            0x21 => (30, FOREVER),
            0x22 => (32, FOREVER),
            0x23 => (31, FOREVER),
            0x24 => (51, FOREVER),
            0x25 => (48, FOREVER),
            0x26 => (47, FOREVER),
            0x27 => (50, FOREVER),
            0x28 => (49, FOREVER),
            0x29 => (68, 50),
            0x2c => (75, 10),
            0x2e => (25, 10),
            0x2f => (26, 10),
            _ => return None,
        }),
        AdvisorFaction::Empire => Some(match code {
            1 => (48, 10),
            2 => (49, 10),
            3 => (52, 10),
            4 => (54, 10),
            5 => (55, 10),
            6 => (57, 10),
            8 => (56, 10),
            9 => (58, 10),
            0xc => (53, 10),
            0xd => (61, 10),
            0xe => (62, 5),
            0x14 => (64, 10),
            0x15 => (63, 10),
            0x16..=0x19 => (64, FOREVER),
            0x1a => (34, FOREVER),
            0x1b => (33, FOREVER),
            0x1c => (51, FOREVER),
            0x1e => (42, FOREVER),
            0x1f => (41, FOREVER),
            0x20 => (43, FOREVER),
            0x21 => (44, FOREVER),
            0x22 => (46, FOREVER),
            0x23 => (45, FOREVER),
            0x24 => (47, FOREVER),
            0x25 => (38, FOREVER),
            0x26 => (37, FOREVER),
            0x27 => (40, FOREVER),
            0x28 => (39, FOREVER),
            0x29 => (65, 50),
            0x2d => (72, 10),
            0x2e => (35, 10),
            0x2f => (36, 10),
            _ => return None,
        }),
    }
}

/// The side's advice agent (`FUN_004c27f0`, `FUN_004c0710`): one stamp and
/// one cooldown per reaction slot, in scheduler steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdviceAgent {
    faction: AdvisorFaction,
    /// `+0x168`: the step each slot fires by, 0 when unset.
    fire_by: Vec<u32>,
    /// `+0x16c`: the step each slot may next fire after.
    may_fire: Vec<u32>,
    /// `DAT_006b28d0`: the step of the last reaction.
    last_reaction: u32,
    /// `DAT_006b28bc`: the pass scans the slots.
    scanning: bool,
    /// `DAT_006b28b0` bit `0x80000000`: idle chatter is due.
    chatter: bool,
    /// `DAT_006b28b0` bit `0x2000`: the player skipped the briefing tour
    /// (`FUN_0043a200`).
    skip: bool,
    /// `+0x174`: the last refused order's status.
    last_refusal: Option<RefusalStatus>,
    /// `+0x17c`: the agent has already answered that status once more.
    refusal_repeated: bool,
}

/// A refused order's two-word status, as the validator returns it to
/// `FUN_00487c90` (`ghidra/notes/mission-dialog.md`, "Refusal").
pub type RefusalStatus = (u32, u32);

/// The steps after a refusal or advice message its reaction fires by
/// (`DAT_006b28cc + 10`).
const REACTION_WINDOW: u32 = 10;

/// What one pass of the agent did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassOutcome {
    /// Nothing fired.
    Idle,
    /// A slot fired; queue its records.
    Fired(u32),
    /// The briefing was skipped: empty both droids' queues
    /// (`FUN_0041da30` → `FUN_0042d620`).
    FlushDroids,
}

impl AdviceAgent {
    /// A new agent. It stamps its opening slot for the whole game: slot 41
    /// for the Alliance (`FUN_004c27f0`), 74 for the Empire (`FUN_004c0710`).
    #[must_use]
    pub fn new(faction: AdvisorFaction) -> Self {
        let slots = match faction {
            AdvisorFaction::Alliance => 0x58,
            AdvisorFaction::Empire => 0x56,
        };
        let mut fire_by = vec![0; slots];
        fire_by[Self::opening_slot(faction) as usize] = FOREVER;
        Self {
            faction,
            fire_by,
            may_fire: vec![0; slots],
            last_reaction: 0,
            scanning: false,
            chatter: false,
            skip: false,
            last_refusal: None,
            refusal_repeated: false,
        }
    }

    /// The slot the agent stamps when it is made: the briefing tour.
    #[must_use]
    pub const fn opening_slot(faction: AdvisorFaction) -> u32 {
        match faction {
            AdvisorFaction::Alliance => 0xa4 / 4,
            AdvisorFaction::Empire => 0x128 / 4,
        }
    }

    /// Drop the opening stamp: a loaded game has had its briefing.
    pub fn clear_opening(&mut self) {
        let slot = Self::opening_slot(self.faction) as usize;
        if let Some(stamp) = self.fire_by.get_mut(slot) {
            *stamp = 0;
        }
    }

    /// The player skipped the briefing tour (`FUN_0043a200`).
    pub fn request_skip(&mut self) {
        self.skip = true;
    }

    /// Answer a refused order (agent slot `+0xc`: `FUN_004c2940` for the
    /// Alliance, `FUN_004c0870` for the Empire). The same status twice in a
    /// row stamps the first repeat slot, a third time the second and forgets
    /// it (`FUN_004c43d0`, `FUN_004c21d0`). Any other status is remembered;
    /// a `0x40` status takes the generic reaction, one of two slots by
    /// `FUN_0041cd80(2)` (`FUN_004c4390`, `FUN_004c2230`). `roll` draws that.
    ///
    /// port: the decoy reaction (`0x40`/`0x91` for a Mission with a decoy
    /// list, slot `+0x50` / `+0x54`) is not told apart; it takes the generic
    /// one.
    pub fn refuse(&mut self, status: RefusalStatus, now: u32, roll: impl FnOnce() -> u32) {
        let (first_repeat, generic) = match self.faction {
            AdvisorFaction::Alliance => (0x3c / 4, 0x44 / 4),
            AdvisorFaction::Empire => (0x40 / 4, 0x48 / 4),
        };
        let slot = if self.last_refusal == Some(status) {
            if self.refusal_repeated {
                self.refusal_repeated = false;
                self.last_refusal = None;
                first_repeat + 1
            } else {
                self.refusal_repeated = true;
                first_repeat
            }
        } else {
            self.last_refusal = Some(status);
            self.refusal_repeated = false;
            let pick = roll();
            if pick >= 2 {
                return;
            }
            generic + pick
        };
        if let Some(stamp) = self.fire_by.get_mut(slot as usize) {
            *stamp = now.saturating_add(REACTION_WINDOW);
        }
    }

    /// File a message's advice code at step `now` (agent `VT[5]`).
    /// `roll(n)` draws `FUN_0041cd80(n)` for the advice codes.
    pub fn post(&mut self, code: u8, now: u32, roll: impl FnOnce(u32) -> u32) {
        if let Some((first, count, unless_pending)) = advice_code_slots(self.faction, code) {
            let slots = first as usize..(first + count) as usize;
            if unless_pending
                && self
                    .fire_by
                    .get(slots.clone())
                    .is_some_and(|stamps| stamps.iter().any(|&stamp| stamp != 0))
            {
                return;
            }
            let pick = roll(count);
            if pick < count {
                if let Some(stamp) = self.fire_by.get_mut((first + pick) as usize) {
                    *stamp = now.saturating_add(REACTION_WINDOW);
                }
            }
            return;
        }
        if let Some((slot, window)) = code_slot(self.faction, code) {
            if let Some(stamp) = self.fire_by.get_mut(slot as usize) {
                *stamp = now.saturating_add(window);
            }
        }
    }

    /// The agent droid's rest command started (`FUN_0041cea0` →
    /// `FUN_00439ef0`): scan the slots on the next pass, and arm chatter
    /// once the game has been quiet long enough.
    pub fn arm(&mut self, now: u32) {
        self.scanning = true;
        if self.last_reaction.saturating_add(CHATTER_QUIET_STEPS) < now {
            self.chatter = true;
        }
    }

    /// One pass (`FUN_004c2b20`, `FUN_004c0a60`). Returns the slot that
    /// fires; at most one does. `roll` draws `FUN_0041cd80(12)`.
    pub fn pass(&mut self, now: u32, roll: impl FnOnce() -> u32) -> PassOutcome {
        if self.scanning {
            for slot in 1..self.fire_by.len() {
                let stamp = std::mem::take(&mut self.fire_by[slot]);
                if stamp == 0 || stamp < now {
                    continue;
                }
                if self.may_fire[slot] < now {
                    self.may_fire[slot] = now + SLOT_COOLDOWN_STEPS;
                    self.last_reaction = now;
                    return PassOutcome::Fired(slot as u32);
                }
            }
        }
        // FUN_004c2dd0 / FUN_004c0d00: nothing fired; stop scanning and
        // turn the pending chatter into a stamp. A roll below 6 picks one of
        // the six chatter slots.
        self.scanning = false;
        // Bit 0x2000 comes before chatter: the skip slot (Alliance 0xa0 / 4,
        // Empire 0x124 / 4) is stamped for the whole game.
        if std::mem::take(&mut self.skip) {
            let slot = match self.faction {
                AdvisorFaction::Alliance => 0xa0 / 4,
                AdvisorFaction::Empire => 0x124 / 4,
            };
            self.fire_by[slot] = now.saturating_add(FOREVER);
            return PassOutcome::FlushDroids;
        }
        if std::mem::take(&mut self.chatter) {
            self.scanning = true;
            let (first, window) = match self.faction {
                AdvisorFaction::Alliance => (0x140 / 4, FOREVER),
                AdvisorFaction::Empire => (0x138 / 4, 5),
            };
            let pick = roll();
            if pick < 6 {
                self.fire_by[first + pick as usize] = now.saturating_add(window);
            }
        }
        PassOutcome::Idle
    }
}

/// A frame of a droid: an anchor bitmap of a module, and how many type-302
/// deltas past it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DroidFrame {
    pub module: u16,
    pub anchor: u16,
    pub index: u16,
}

impl DroidFrame {
    const fn still(module: u16, anchor: u16) -> Self {
        Self {
            module,
            anchor,
            index: 0,
        }
    }
}

/// What a command's step asks of the rest of the advisor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DroidEvent {
    /// The agent droid's rest command started (`FUN_0041cea0`).
    ArmChatter,
    /// Queue an action on the partner droid (`FUN_0041d690`).
    PartnerAction(u16),
    /// The partner's sign-off finished (`FUN_00412080`: `DAT_006b12b0` = 1).
    PartnerSignedOff,
    /// Play a sound (`FUN_00403f70`): the wave id and its module.
    Sound { wave: u16, module: u16 },
    /// A command `0x11` finished: its `(p2, p3)` cockpit step for the
    /// agent's next pass (`FUN_00412400` → `FUN_00439eb0`).
    CockpitStep { kind: u32, step: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    /// Commands 4 and `0x13`: play the anchor's run once, a frame a tick
    /// (`FUN_00404440`, `FUN_00404360`).
    Play {
        module: u16,
        anchor: u16,
        frames: u16,
        shown: u16,
        sound: Option<(u16, u16)>,
    },
    /// Commands 5, `0x11`, `0x12` and `0x15`: hold a still frame for `ticks`
    /// more ticks.
    Hold {
        module: u16,
        bitmap: u16,
        ticks: u32,
        arms_chatter: bool,
        signs_off: bool,
        step: Option<(u32, u32)>,
    },
    /// Command `0x14`: hold a frame while the partner plays its action and
    /// signs off (`FUN_00411b80`, `FUN_00411b60`).
    Turn {
        module: u16,
        bitmap: u16,
        partner_action: u16,
    },
}

impl Command {
    /// Build command `number` from its script (`FUN_00403ed0`). `None` when
    /// the script is missing or the command is not one the droids use.
    fn build(action: &ResolvedAction) -> Option<Self> {
        let (module, words, p2) = (action.module, action.words.as_slice(), action.p2);
        Some(match action.command {
            4 | 0x13 => Self::Play {
                module,
                anchor: *words.get(4)?,
                frames: (*words.get(3)?).max(1),
                shown: 0,
                sound: match (words.get(1), words.get(2)) {
                    (Some(&wave), Some(&module)) if wave != 0 => Some((wave, module)),
                    _ => None,
                },
            },
            5 | 0x12 => Self::Hold {
                module,
                bitmap: *words.get(1)?,
                ticks: p2,
                arms_chatter: action.command == 0x12,
                signs_off: false,
                step: None,
            },
            // Command 0x11 holds its frame one tick (`FUN_00412420` sets
            // +0x50 to 1, `FUN_00412260` counts it down). Its p2 and p3 are a
            // cockpit step for the agent's next pass (`FUN_00412400` →
            // `FUN_00439eb0` → `FUN_004c3060`): p2 2 runs briefing step p3
            // (`FUN_004c30c0`), p2 3 selects an object (`FUN_0041d830`).
            0x11 => Self::Hold {
                module,
                bitmap: *words.first()?,
                ticks: 1,
                arms_chatter: false,
                signs_off: false,
                step: Some((p2, action.p3)),
            },
            // Command 0x15 holds one tick the same way (`FUN_004120a0`),
            // then signs off (`FUN_00412080`).
            0x15 => Self::Hold {
                module,
                bitmap: *words.first()?,
                ticks: 1,
                arms_chatter: false,
                signs_off: true,
                step: None,
            },
            0x14 => Self::Turn {
                module,
                bitmap: *words.get(1)?,
                partner_action: *words.get(2)?,
            },
            _ => return None,
        })
    }

    const fn first_frame(&self) -> DroidFrame {
        match *self {
            Self::Play { module, anchor, .. } => DroidFrame::still(module, anchor),
            Self::Hold { module, bitmap, .. } | Self::Turn { module, bitmap, .. } => {
                DroidFrame::still(module, bitmap)
            }
        }
    }
}

/// An action resolved against its scripts (`FUN_0042b1d0`): the module it
/// was loaded from, the command number, the script words, and its `p2` and
/// `p3` (the record's own, else the action's).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedAction {
    pub module: u16,
    pub command: u16,
    pub words: Vec<u16>,
    pub p2: u32,
    pub p3: u32,
}

/// One droid's player and command queue (`FUN_00441a60`, `FUN_004727e0`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DroidPlayer {
    queue: VecDeque<Command>,
    current: Option<Command>,
    frame: DroidFrame,
    rest_bitmap: u16,
    /// The agent's default is command `0x12` (arms chatter); the partner's
    /// is command 5.
    is_agent: bool,
    /// `DAT_006b12b0`: the partner signed off since the last turn began.
    partner_done: bool,
}

impl DroidPlayer {
    /// A player resting on `rest_bitmap`. Its first command holds the rest
    /// frame for five ticks (`FUN_0042adb0`).
    #[must_use]
    pub fn new(rest_bitmap: u16, is_agent: bool) -> Self {
        let first = Command::Hold {
            module: SPRITE_MODULE,
            bitmap: rest_bitmap,
            ticks: FIRST_HOLD_TICKS,
            arms_chatter: false,
            signs_off: false,
            step: None,
        };
        Self {
            queue: VecDeque::from([first]),
            current: None,
            frame: DroidFrame::still(SPRITE_MODULE, rest_bitmap),
            rest_bitmap,
            is_agent,
            partner_done: false,
        }
    }

    /// The frame on screen.
    #[must_use]
    pub const fn frame(&self) -> DroidFrame {
        self.frame
    }

    /// Queue `action` (`FUN_00472930`). An action whose command is not one
    /// of the droids' is dropped.
    pub fn enqueue(&mut self, action: &ResolvedAction) {
        if let Some(command) = Command::build(action) {
            self.queue.push_back(command);
        }
    }

    /// Drop the running command and the queue (`VT[0x3c]` from
    /// `FUN_0042d620`); the next tick starts the default.
    pub fn flush(&mut self) {
        self.queue.clear();
        self.current = None;
    }

    /// The partner's sign-off finished.
    pub fn partner_signed_off(&mut self) {
        self.partner_done = true;
    }

    /// One 67 ms tick (`FUN_004729c0`): step the running command, and start
    /// the next one, or the default, when it is done.
    pub fn tick(&mut self, events: &mut Vec<DroidEvent>) {
        let done = match self.current.as_mut() {
            None => true,
            Some(Command::Play {
                module,
                anchor,
                frames,
                shown,
                ..
            }) => {
                *shown += 1;
                if *shown < *frames {
                    self.frame = DroidFrame {
                        module: *module,
                        anchor: *anchor,
                        index: *shown,
                    };
                    false
                } else {
                    true
                }
            }
            Some(Command::Hold {
                ticks,
                signs_off,
                step,
                ..
            }) => {
                if *ticks > 0 {
                    *ticks -= 1;
                    false
                } else {
                    if *signs_off {
                        events.push(DroidEvent::PartnerSignedOff);
                    }
                    if let Some((kind, step)) = *step {
                        events.push(DroidEvent::CockpitStep { kind, step });
                    }
                    true
                }
            }
            Some(Command::Turn { .. }) => self.partner_done,
        };
        if !done {
            return;
        }
        let next = self.queue.pop_front().unwrap_or(Command::Hold {
            module: SPRITE_MODULE,
            bitmap: self.rest_bitmap,
            ticks: REST_HOLD_TICKS,
            arms_chatter: self.is_agent,
            signs_off: false,
            step: None,
        });
        self.frame = next.first_frame();
        match next {
            Command::Hold {
                arms_chatter: true, ..
            } => events.push(DroidEvent::ArmChatter),
            Command::Turn { partner_action, .. } => {
                self.partner_done = false;
                events.push(DroidEvent::PartnerAction(partner_action));
                events.push(DroidEvent::PartnerAction(PARTNER_SIGN_OFF));
            }
            Command::Play {
                sound: Some((wave, module)),
                ..
            } => events.push(DroidEvent::Sound { wave, module }),
            _ => {}
        }
        self.current = Some(next);
    }
}
