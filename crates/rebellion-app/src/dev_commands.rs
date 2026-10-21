//! Developer commands for play-testing: the command palette's gate and the
//! native command script.
//!
//! The palette (backtick) is on in debug builds, and in release builds when
//! `OPEN_REBELLION_DEV` is set. While it is on, a native run with
//! `OPEN_REBELLION_COMMANDS` naming a file runs that file's lines as palette
//! commands, one per frame, so an acceptance run reaches its state without
//! the clicks that lead to it, and `OPEN_REBELLION_SEED` fixes every new
//! campaign's seed so the script meets the same galaxy each run. None is
//! compiled into a release browser build.

#[cfg(not(target_arch = "wasm32"))]
use rebellion_core::dat::Faction;
#[cfg(not(target_arch = "wasm32"))]
use rebellion_core::missions::MissionKind;
#[cfg(not(target_arch = "wasm32"))]
use rebellion_core::research::TechType;
#[cfg(not(target_arch = "wasm32"))]
use std::collections::VecDeque;

/// True when the palette answers the backtick key.
#[must_use]
pub fn palette_enabled() -> bool {
    palette_switch(
        std::env::var("OPEN_REBELLION_DEV").ok().as_deref(),
        cfg!(debug_assertions),
    )
}

/// Whether the palette is on for an `OPEN_REBELLION_DEV` value: always in
/// a debug build, and in a native release build when the value is on.
fn palette_switch(value: Option<&str>, debug_build: bool) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    if value.is_some_and(crate::env_flag_on) {
        return true;
    }
    #[cfg(target_arch = "wasm32")]
    let _ = value;
    debug_build
}

/// The campaign seed `OPEN_REBELLION_SEED` fixes, if it is set and the
/// palette is on.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn campaign_seed() -> Option<u64> {
    seed_value(std::env::var("OPEN_REBELLION_SEED").ok().as_deref()).filter(|_| palette_enabled())
}

/// An `OPEN_REBELLION_SEED` value as a seed: a whole number, spaces around
/// it ignored.
#[cfg(not(target_arch = "wasm32"))]
fn seed_value(value: Option<&str>) -> Option<u64> {
    value?.trim().parse().ok()
}

/// The lines of a command script still to run.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Default)]
pub struct CommandScript {
    lines: VecDeque<(Option<u64>, String)>,
    /// The live-inbox sequence number of the line `step` last ran, if it
    /// came from the inbox.
    last_seq: Option<u64>,
}

/// What a command script does in a frame.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, PartialEq, Eq)]
pub enum ScriptStep {
    /// The game is not ready for a line.
    Wait,
    /// Run this line.
    Run(String),
    /// Every line has run.
    Done,
}

#[cfg(not(target_arch = "wasm32"))]
impl CommandScript {
    /// Each non-blank line of `text` that is not a `#` comment, trimmed: a
    /// palette command's label.
    #[must_use]
    pub fn parse(text: &str) -> Self {
        Self {
            lines: text
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty() && !line.starts_with('#'))
                .map(|line| (None, line.to_string()))
                .collect(),
            last_seq: None,
        }
    }

    /// Queue a palette line, which reports to no inbox.
    pub fn push_line(&mut self, line: String) {
        self.lines.push_back((None, line));
    }

    /// Queue a live-inbox line after the lines already waiting.
    pub fn push_live(&mut self, seq: u64, line: String) {
        self.lines.push_back((Some(seq), line));
    }

    /// The line that runs next.
    #[must_use]
    pub fn peek(&self) -> Option<&str> {
        self.lines.front().map(|(_, line)| line.as_str())
    }

    /// The inbox sequence number of the line `step` last returned.
    #[must_use]
    pub const fn last_seq(&self) -> Option<u64> {
        self.last_seq
    }

    /// The script `OPEN_REBELLION_COMMANDS` names, if it is set and reads
    /// and the palette is on.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn from_env() -> Option<Self> {
        let path = std::env::var_os("OPEN_REBELLION_COMMANDS").filter(|_| palette_enabled())?;
        Self::from_file(std::path::Path::new(&path))
    }

    /// The script in `path`, or `None`, logged, when it does not read.
    #[cfg(not(target_arch = "wasm32"))]
    fn from_file(path: &std::path::Path) -> Option<Self> {
        match std::fs::read_to_string(path) {
            Ok(text) => Some(Self::parse(&text)),
            Err(error) => {
                eprintln!("[dev-command] cannot read {}: {error}", path.display());
                None
            }
        }
    }

    /// This frame's step. `ready` is true when the game shows the main
    /// menu or the galaxy and the last line's commands have run. A line
    /// runs only then, and the script is done only when ready after its
    /// last line, so `Done` follows the last command.
    pub fn step(&mut self, ready: bool) -> ScriptStep {
        if !ready {
            return ScriptStep::Wait;
        }
        match self.lines.pop_front() {
            Some((seq, line)) => {
                self.last_seq = seq;
                ScriptStep::Run(line)
            }
            None => {
                self.last_seq = None;
                ScriptStep::Done
            }
        }
    }
}

/// A command that takes arguments or runs outside the galaxy: the lines a
/// palette label cannot name. A script line, a live-inbox line and a palette
/// entry all parse here first; a line that names no `DevCommand` falls back
/// to the palette's labels. `agent_docs/dev-commands.md` lists the grammar.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq)]
pub enum DevCommand {
    /// Save the next drawn frame as `<evidence>/<name>.png`.
    Capture(Option<String>),
    /// Hold the queue for this many milliseconds.
    WaitMs(u32),
    /// Hold the queue for this many frames.
    WaitFrames(u32),
    /// Hold the queue until a condition holds, or refuse after
    /// `WAIT_UNTIL_FRAMES`.
    WaitUntil(WaitCondition),
    /// `WaitUntil` refused after this many frames instead.
    WaitUntilWithin(WaitCondition, u32),
    /// Click at a point of the 640×480 logical screen, Ctrl held when `ctrl`.
    Click {
        x: f32,
        y: f32,
        right: bool,
        ctrl: bool,
    },
    /// Click the center of a named control of the last drawn frame
    /// (`<window>/<control>`, `rebellion_render::control_registry`).
    ClickControl { name: String, right: bool },
    /// Report a named control's logical rect and center.
    LocateControl(String),
    /// Report the control names of the last drawn frame, or one window's.
    ListControls(Option<String>),
    /// Press at one point, move, release at another, with Control held
    /// around them when `ctrl`.
    Drag {
        from: (f32, f32),
        to: (f32, f32),
        ctrl: bool,
    },
    /// Press at one point and move to another, leaving the button held.
    HoldDrag { from: (f32, f32), to: (f32, f32) },
    /// Release the held left button at a point.
    Release { x: f32, y: f32 },
    /// Wheel lines at a point: positive scrolls down a list, negative up.
    Scroll { x: f32, y: f32, notches: i32 },
    /// Press and release a named key.
    Press(String),
    /// Open the Message Index on a category command (`0x79..=0x82`), and
    /// with a row, show that message in mode 2.
    OpenMessageIndex { category: u16, row: Option<Row> },
    /// File a message class at a system, as the event that sends it would.
    PostMessage { class: MessageClass, system: String },
    /// Bring an idle fleet of each side missing from a system there; the
    /// next tick's combat check starts the battle.
    BattleAt(String),
    /// Bring an idle fleet of the side not holding a system there; the
    /// simulation then forms the blockade.
    Blockade(String),
    /// In a battle, put the player's nth ship's hyperdrive out of action
    /// (1-based, in roster order), as a subsystem hit does.
    KnockOutHyperdrive(usize),
    /// Mark the named Imperial fleet as carrying the Death Star.
    GiveDeathStar(String),
    /// Halve the hull of the first capital ship in the player's named fleet.
    DamageShip(String),
    /// Raise a side's maintenance capacity by at least `amount` (staging).
    MaintenanceCapacity { alliance: bool, amount: u32 },
    /// Destroy every defender of the system's holder there (staging).
    StripDefenders(String),
    /// Load `count` of the player's regiments at the named fleet's system
    /// onto it, each by the regiment's Move onto the fleet (staging).
    Embark { fleet: String, count: usize },
    /// Queue one unit of a class at a system's yard for a side, through
    /// Build Selection's checks and enqueue (staging).
    QueueBuild {
        alliance: bool,
        class: String,
        system: String,
    },
    /// Issue a Planetary Bombardment order (`0x220`-`0x223`) for a named
    /// fleet: the player's (no side, or the player's) as the fleet's menu
    /// row does, the other side's as the AI orders one.
    Bombard {
        kind: rebellion_core::bombardment::BombardKind,
        side: Option<Faction>,
        fleet: String,
    },
    /// Issue a Planetary Assault order (`0x234`) for the player's named
    /// fleet, as the fleet's menu row does.
    Assault(String),
    /// The opening briefing's own skip.
    SkipBriefing,
    /// Post an advice code to the droids, as a game event does.
    AdvisorPostCode(u16),
    /// Release an Advice topic and file its message.
    ReleaseAdviceTopic(u16),
    /// Report every system's holder and the sides with fleets there.
    ListSystems,
    /// Report every fleet, or one side's, and where it is.
    ListFleets(Option<Faction>),
    /// Report the Advice topics still held.
    ListAdviceTopics,
    /// Report the game's state: mode, day, speed, open dialogs, side.
    Status,
    /// Staging only: end the program at once, as Game Options' Exit (0x8f)
    /// does after its confirmation, from any screen.
    Quit,
    /// Report every character, or one side's.
    ListCharacters(Option<Faction>),
    /// Report the active missions, or one side's.
    ListMissions(Option<Faction>),
    /// Report each side's research levels, points and next cost.
    ShowResearch,
    /// Report the production queues, or those at one side's systems.
    ListProduction(Option<Faction>),
    /// Report one side's top strip and counts.
    ShowSide(Faction),
    /// Report one system's holder, support, stocks, units and fleets.
    ShowSystem(String),
    /// Save the campaign as `<evidence>/saves/<name>.reb`, through the game's
    /// own save path.
    Save(String),
    /// Load `<evidence>/saves/<name>.reb`, through the game's own load path.
    Load(String),
    /// Dispatch a mission as the Create Mission dialog's Begin Mission does.
    Mission {
        team: Vec<String>,
        kind: MissionKind,
        system: String,
        /// The capital ship the order names by its tag, else none.
        ship: Option<u32>,
        decoys: Vec<String>,
    },
    /// Apply the Recruitment mission's success effect to one character.
    Recruit {
        character: String,
        side: Faction,
        system: Option<String>,
    },
    /// Hold one character in the other side's jail, as a stranded mission
    /// member is held.
    Jail(String),
    /// Turn fast forward on or off: one game day each frame.
    FastForward(bool),
    /// What a wait does when a Battle Alert opens.
    BattlePolicy(crate::dev_policy::BattlePolicy),
    /// What a wait does when the Advice index opens.
    AdvicePolicy(crate::dev_policy::AdvicePolicy),
}

/// What `Wait until` waits for.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitCondition {
    /// The galaxy clock has reached this day.
    Day(u64),
    /// The Battle Alert is open.
    BattleAlert,
    /// A tactical battle is on screen.
    Battle,
    /// The galaxy view shows.
    Galaxy,
    /// The player's side resolves a mission after the wait began: the
    /// mission with this id, or the next one.
    MissionResolved(Option<u64>),
    /// A message is filed after the wait began in this Message Index
    /// category (a `MESSAGE_CATEGORIES` command; All takes any).
    Message(u16),
    /// The pause alert ("Resume Game Play?") is up.
    PauseAlert,
    /// A side's research in a tree has reached this level.
    Tech {
        alliance: bool,
        tech: rebellion_core::research::TechType,
        level: u32,
    },
    /// `Skip briefing`'s own wait: the briefing has let go of the clock
    /// (`dev_policy::skip_briefing`). No line names it.
    BriefingSkipped,
}

#[cfg(not(target_arch = "wasm32"))]
impl WaitCondition {
    /// Whether the wait is met only as game days pass, so fast forward may
    /// run the clock for it.
    #[must_use]
    pub const fn needs_clock(self) -> bool {
        !matches!(self, Self::Galaxy | Self::BriefingSkipped)
    }
}

/// The longest a `Wait until` holds before it is refused: the
/// `Wait <n> frames` cap, ten minutes at 60 frames a second.
#[cfg(not(target_arch = "wasm32"))]
pub const WAIT_UNTIL_FRAMES: u32 = 36_000;

/// A Message Index row: 1-based in list order (oldest first), or the last.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Row {
    Number(usize),
    Last,
}

/// The message classes `Post message` files.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageClass {
    UprisingBegan,
    UprisingEnded,
    Blockade,
    FleetArrival,
    LoyaltyJoins,
    LoyaltyNeutral,
}

/// Where a command may run.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Readiness {
    /// In any game mode, the main menu and battles included.
    Anywhere,
    /// Only once the galaxy shows and earlier commands have run; never on
    /// the main menu, whose background world is no campaign.
    Galaxy,
}

#[cfg(not(target_arch = "wasm32"))]
impl Readiness {
    /// Whether a command of this readiness may run now: `in_galaxy` when the
    /// galaxy view shows with no interface work pending.
    #[must_use]
    pub const fn admits(self, in_galaxy: bool) -> bool {
        match self {
            Self::Anywhere => true,
            Self::Galaxy => in_galaxy,
        }
    }
}

/// The Message Index categories by name, as the window's controls read.
#[cfg(not(target_arch = "wasm32"))]
pub const MESSAGE_CATEGORIES: [(&str, u16); 10] = [
    ("All", 0x79),
    ("Popular Support", 0x7a),
    ("Fleet", 0x7b),
    ("Mission", 0x7c),
    ("Resource", 0x7d),
    ("Manufacturing", 0x7e),
    ("Defense", 0x7f),
    ("Conflict", 0x80),
    ("Chat", 0x81),
    ("Advice", 0x82),
];

#[cfg(not(target_arch = "wasm32"))]
impl DevCommand {
    /// Where this command may run.
    #[must_use]
    pub const fn readiness(&self) -> Readiness {
        match self {
            Self::Capture(_)
            | Self::WaitMs(_)
            | Self::WaitFrames(_)
            | Self::WaitUntil(_)
            | Self::WaitUntilWithin(..)
            | Self::Click { .. }
            | Self::ClickControl { .. }
            | Self::LocateControl(_)
            | Self::ListControls(_)
            | Self::Drag { .. }
            | Self::HoldDrag { .. }
            | Self::Release { .. }
            | Self::Scroll { .. }
            | Self::Press(_)
            | Self::KnockOutHyperdrive(_)
            | Self::FastForward(_)
            | Self::BattlePolicy(_)
            | Self::AdvicePolicy(_)
            | Self::ListCharacters(_)
            | Self::ListFleets(_)
            | Self::ListMissions(_)
            | Self::ShowResearch
            | Self::ListProduction(_)
            | Self::ShowSide(_)
            | Self::ShowSystem(_)
            | Self::Load(_)
            | Self::Quit
            | Self::Status => Readiness::Anywhere,
            _ => Readiness::Galaxy,
        }
    }
}

/// `Embark: <fleet> <n>`: the fleet's name, then a positive count as the
/// last word.
#[cfg(not(target_arch = "wasm32"))]
fn parse_embark(tail: &str) -> Result<DevCommand, String> {
    let usage = "embark takes a fleet name and a count of regiments";
    let (fleet, count) = tail.rsplit_once(char::is_whitespace).ok_or(usage)?;
    let count: usize = count.parse().map_err(|_| usage)?;
    if fleet.trim().is_empty() || count == 0 {
        return Err(usage.into());
    }
    Ok(DevCommand::Embark {
        fleet: fleet.trim().to_owned(),
        count,
    })
}

/// The `DevCommand` a line names, `Ok(None)` when it names none (try the
/// palette), or `Err` when it names one with bad arguments.
#[cfg(not(target_arch = "wasm32"))]
pub fn parse_line(line: &str) -> Result<Option<DevCommand>, String> {
    let line = line.trim();
    let lower = line.to_ascii_lowercase();
    let rest = |prefix: &str| line[prefix.len()..].trim();
    if lower == "capture" {
        return Ok(Some(DevCommand::Capture(None)));
    }
    if lower.starts_with("capture:") {
        let name = rest("capture:");
        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
            || name.starts_with('.')
        {
            return Err(format!("capture name {name:?}: use letters, digits, - _ ."));
        }
        return Ok(Some(DevCommand::Capture(Some(name.to_owned()))));
    }
    if lower.starts_with("capture ") {
        return Err("capture takes `Capture` or `Capture: <name>`".into());
    }
    if let Some(what) = lower.strip_prefix("wait until ") {
        let (what, budget) = split_budget(what.trim())?;
        let parsed = match what.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["day", n] => n
                .parse()
                .map(|day| Some(DevCommand::WaitUntil(WaitCondition::Day(day))))
                .map_err(|_| format!("day {n:?} is not a whole number")),
            ["battle", "alert"] => Ok(Some(DevCommand::WaitUntil(WaitCondition::BattleAlert))),
            ["battle"] => Ok(Some(DevCommand::WaitUntil(WaitCondition::Battle))),
            ["galaxy"] => Ok(Some(DevCommand::WaitUntil(WaitCondition::Galaxy))),
            ["pause", "alert"] => Ok(Some(DevCommand::WaitUntil(WaitCondition::PauseAlert))),
            ["mission", "resolved"] => Ok(Some(DevCommand::WaitUntil(
                WaitCondition::MissionResolved(None),
            ))),
            ["mission", id, "resolved"] => id
                .parse()
                .map(|id| {
                    Some(DevCommand::WaitUntil(WaitCondition::MissionResolved(Some(
                        id,
                    ))))
                })
                .map_err(|_| format!("mission id {id:?} is not a whole number")),
            ["message", name @ ..] if !name.is_empty() => match message_index(&name.join(" "))? {
                DevCommand::OpenMessageIndex {
                    category,
                    row: None,
                } => Ok(Some(DevCommand::WaitUntil(WaitCondition::Message(
                    category,
                )))),
                _ => Err("wait until message takes a category name".into()),
            },
            ["tech", side, tree, level] => wait_tech(side, tree, level).map(Some),
            _ => Err(
                "wait until takes `day <n>`, `battle alert`, `battle`, `galaxy`, \
                      `pause alert`, `mission [<id>] resolved`, `message <category>` or \
                      `tech <alliance|empire> <ship|troop|facility> <n>`"
                    .into(),
            ),
        };
        return parsed.map(|command| {
            command.map(|command| match (command, budget) {
                (DevCommand::WaitUntil(condition), Some(frames)) => {
                    DevCommand::WaitUntilWithin(condition, frames)
                }
                (command, _) => command,
            })
        });
    }
    if lower.starts_with("wait ") {
        let words: Vec<&str> = rest("wait ").split_whitespace().collect();
        let count = words.first().and_then(|n| n.parse::<u32>().ok());
        return match (count, words.get(1).copied(), words.len()) {
            (Some(n), Some("ms"), 2) => Ok(Some(DevCommand::WaitMs(n.min(600_000)))),
            (Some(n), Some("frames" | "frame"), 2) => {
                Ok(Some(DevCommand::WaitFrames(n.min(36_000))))
            }
            _ => Err("wait takes `<n> ms` or `<n> frames`".into()),
        };
    }
    for (prefix, right) in [("right click control:", true), ("click control:", false)] {
        if lower.starts_with(prefix) {
            let name = rest(prefix);
            if !name.contains('/') || name.starts_with('/') || name.ends_with('/') {
                return Err(format!("{prefix} takes `<window>/<control>`"));
            }
            return Ok(Some(DevCommand::ClickControl {
                name: name.to_owned(),
                right,
            }));
        }
    }
    if lower.starts_with("locate ") {
        let name = rest("locate ");
        if !name.contains('/') || name.starts_with('/') || name.ends_with('/') {
            return Err("locate takes `<window>/<control>`".into());
        }
        return Ok(Some(DevCommand::LocateControl(name.to_owned())));
    }
    if lower == "list controls" {
        return Ok(Some(DevCommand::ListControls(None)));
    }
    if lower.starts_with("list controls ") {
        return Ok(Some(DevCommand::ListControls(Some(
            rest("list controls ").to_owned(),
        ))));
    }
    if lower.starts_with("right click ") {
        let (x, y) = point(rest("right click "))?;
        return Ok(Some(DevCommand::Click {
            x,
            y,
            right: true,
            ctrl: false,
        }));
    }
    if lower.starts_with("ctrl click ") {
        let (x, y) = point(rest("ctrl click "))?;
        return Ok(Some(DevCommand::Click {
            x,
            y,
            right: false,
            ctrl: true,
        }));
    }
    if lower.starts_with("click ") {
        let (x, y) = point(rest("click "))?;
        return Ok(Some(DevCommand::Click {
            x,
            y,
            right: false,
            ctrl: false,
        }));
    }
    if lower.starts_with("drag ") {
        let (from, to) = span(rest("drag "), "drag")?;
        return Ok(Some(DevCommand::Drag {
            from,
            to,
            ctrl: false,
        }));
    }
    if lower.starts_with("ctrl drag ") {
        let (from, to) = span(rest("ctrl drag "), "ctrl drag")?;
        return Ok(Some(DevCommand::Drag {
            from,
            to,
            ctrl: true,
        }));
    }
    if lower.starts_with("hold drag ") {
        let (from, to) = span(rest("hold drag "), "hold drag")?;
        return Ok(Some(DevCommand::HoldDrag { from, to }));
    }
    if lower.starts_with("release at ") {
        let (x, y) = point(rest("release at "))?;
        return Ok(Some(DevCommand::Release { x, y }));
    }
    if lower.starts_with("scroll ") {
        let args = rest("scroll ");
        let parsed = args.rsplit_once(' ').and_then(|(at, n)| {
            let notches = n
                .parse::<i32>()
                .ok()
                .filter(|n| *n != 0 && n.abs() <= 100)?;
            Some((at, notches))
        });
        let Some((at, notches)) = parsed else {
            return Err("scroll takes `<x>,<y> <notches>`, 1 to 100 down or -1 to -100 up".into());
        };
        let (x, y) = point(at)?;
        return Ok(Some(DevCommand::Scroll { x, y, notches }));
    }
    if lower.starts_with("press ") {
        let key = rest("press ");
        if key.is_empty() || key.contains(char::is_whitespace) {
            return Err("press takes one key name".into());
        }
        return Ok(Some(DevCommand::Press(key.to_ascii_lowercase())));
    }
    if lower.starts_with("open message index:") {
        return message_index(rest("open message index:")).map(Some);
    }
    if lower.starts_with("post message:") {
        return post_message(rest("post message:")).map(Some);
    }
    if lower.starts_with("battle at ") {
        return Ok(Some(DevCommand::BattleAt(rest("battle at ").to_owned())));
    }
    if lower.starts_with("knock out hyperdrive:") {
        return match rest("knock out hyperdrive:").parse::<usize>() {
            Ok(n) if n > 0 => Ok(Some(DevCommand::KnockOutHyperdrive(n))),
            _ => Err("knock out hyperdrive takes a ship number from 1".into()),
        };
    }
    if lower.starts_with("give death star:") {
        let fleet = rest("give death star:");
        if fleet.is_empty() {
            return Err("give death star takes a fleet name".into());
        }
        return Ok(Some(DevCommand::GiveDeathStar(fleet.to_owned())));
    }
    for (prefix, kind) in [
        (
            "bombard military:",
            rebellion_core::bombardment::BombardKind::Military,
        ),
        (
            "bombard civilian:",
            rebellion_core::bombardment::BombardKind::Civilian,
        ),
        (
            "bombard general:",
            rebellion_core::bombardment::BombardKind::General,
        ),
        (
            "destroy system:",
            rebellion_core::bombardment::BombardKind::DestroySystem,
        ),
    ] {
        if lower.starts_with(prefix) {
            let body = rest(prefix);
            let (side, fleet) = match body.split_once(' ') {
                Some((word, fleet)) => match side_filter(word) {
                    Ok(side @ Some(_)) => (side, fleet.trim()),
                    _ => (None, body),
                },
                None => (None, body),
            };
            if fleet.is_empty() || side_filter(fleet).is_ok_and(|side| side.is_some()) {
                return Err(format!(
                    "{} takes [alliance|empire] and a fleet name",
                    prefix.trim_end_matches(':')
                ));
            }
            return Ok(Some(DevCommand::Bombard {
                kind,
                side,
                fleet: fleet.to_owned(),
            }));
        }
    }
    if lower.starts_with("assault:") {
        let fleet = rest("assault:");
        if fleet.is_empty() {
            return Err("assault takes a fleet name".into());
        }
        return Ok(Some(DevCommand::Assault(fleet.to_owned())));
    }
    if lower.starts_with("damage ship:") {
        let fleet = rest("damage ship:");
        if fleet.is_empty() {
            return Err("damage ship takes a fleet name".into());
        }
        return Ok(Some(DevCommand::DamageShip(fleet.to_owned())));
    }
    if lower.starts_with("queue build:") {
        let usage = "queue build takes `alliance` or `empire`, a class and `at <system>`";
        let body = rest("queue build:");
        let (side, tail) = body.split_once(' ').ok_or(usage)?;
        let alliance = match side.to_ascii_lowercase().as_str() {
            "alliance" => true,
            "empire" => false,
            _ => return Err(usage.into()),
        };
        let lower_tail = tail.to_ascii_lowercase();
        let at = lower_tail.rfind(" at ").ok_or(usage)?;
        let (class, system) = (tail[..at].trim(), tail[at + 4..].trim());
        if class.is_empty() || system.is_empty() {
            return Err(usage.into());
        }
        return Ok(Some(DevCommand::QueueBuild {
            alliance,
            class: class.to_owned(),
            system: system.to_owned(),
        }));
    }
    if lower.starts_with("embark:") {
        return parse_embark(rest("embark:")).map(Some);
    }
    if lower.starts_with("strip defenders at ") {
        let system = rest("strip defenders at ");
        if system.is_empty() {
            return Err("strip defenders at takes a system name".into());
        }
        return Ok(Some(DevCommand::StripDefenders(system.to_owned())));
    }
    if lower.starts_with("maintenance capacity:") {
        let usage = "maintenance capacity takes `alliance` or `empire` and `+<n>`";
        let mut words = rest("maintenance capacity:").split_whitespace();
        let alliance = match words.next().map(str::to_ascii_lowercase).as_deref() {
            Some("alliance") => true,
            Some("empire") => false,
            _ => return Err(usage.into()),
        };
        let amount = words
            .next()
            .and_then(|word| word.strip_prefix('+'))
            .and_then(|n| n.parse::<u32>().ok())
            .filter(|&n| n > 0);
        return match (amount, words.next()) {
            (Some(amount), None) => Ok(Some(DevCommand::MaintenanceCapacity { alliance, amount })),
            _ => Err(usage.into()),
        };
    }
    if lower.starts_with("blockade ") {
        return Ok(Some(DevCommand::Blockade(rest("blockade ").to_owned())));
    }
    if lower == "status" {
        return Ok(Some(DevCommand::Status));
    }
    if lower == "quit" {
        return Ok(Some(DevCommand::Quit));
    }
    if lower.starts_with("fast forward:") {
        return match rest("fast forward:").to_ascii_lowercase().as_str() {
            "on" => Ok(Some(DevCommand::FastForward(true))),
            "off" => Ok(Some(DevCommand::FastForward(false))),
            _ => Err("fast forward takes `on` or `off`".into()),
        };
    }
    if lower.starts_with("battle policy:") {
        use crate::dev_policy::BattlePolicy;
        return match rest("battle policy:").to_ascii_lowercase().as_str() {
            "stop" => Ok(Some(DevCommand::BattlePolicy(BattlePolicy::Stop))),
            "simulate" => Ok(Some(DevCommand::BattlePolicy(BattlePolicy::Simulate))),
            "retreat" => Ok(Some(DevCommand::BattlePolicy(BattlePolicy::Retreat))),
            _ => Err("battle policy takes `simulate`, `retreat` or `stop`".into()),
        };
    }
    if lower.starts_with("advice policy:") {
        use crate::dev_policy::AdvicePolicy;
        return match rest("advice policy:").to_ascii_lowercase().as_str() {
            "stop" => Ok(Some(DevCommand::AdvicePolicy(AdvicePolicy::Stop))),
            "close" => Ok(Some(DevCommand::AdvicePolicy(AdvicePolicy::Close))),
            _ => Err("advice policy takes `close` or `stop`".into()),
        };
    }
    if let Some(side) = lower.strip_prefix("list characters") {
        return side_filter(side).map(|side| Some(DevCommand::ListCharacters(side)));
    }
    if let Some(side) = lower.strip_prefix("list fleets") {
        return side_filter(side).map(|side| Some(DevCommand::ListFleets(side)));
    }
    if let Some(side) = lower.strip_prefix("list missions") {
        return side_filter(side).map(|side| Some(DevCommand::ListMissions(side)));
    }
    if let Some(side) = lower.strip_prefix("list production") {
        return side_filter(side).map(|side| Some(DevCommand::ListProduction(side)));
    }
    if lower == "show research" {
        return Ok(Some(DevCommand::ShowResearch));
    }
    if lower.starts_with("show system") {
        let name = rest("show system");
        return if name.is_empty() {
            Err("show system takes a system name".into())
        } else {
            Ok(Some(DevCommand::ShowSystem(name.to_owned())))
        };
    }
    if let Some(side) = lower.strip_prefix("show side") {
        return match side_filter(side)? {
            Some(side) => Ok(Some(DevCommand::ShowSide(side))),
            None => Err("show side takes alliance or empire".into()),
        };
    }
    if lower.starts_with("save:") || lower.starts_with("load:") {
        let name = rest("save:");
        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
            || name.starts_with('.')
        {
            return Err(format!("save name {name:?}: use letters, digits, - _ ."));
        }
        let name = name.to_owned();
        return Ok(Some(if lower.starts_with("save:") {
            DevCommand::Save(name)
        } else {
            DevCommand::Load(name)
        }));
    }
    if lower.starts_with("mission:") {
        return mission(rest("mission:")).map(Some);
    }
    if lower.starts_with("recruit:") {
        return recruit(rest("recruit:")).map(Some);
    }
    if lower.starts_with("jail:") {
        let character = rest("jail:");
        if character.is_empty() {
            return Err("jail takes a character name".into());
        }
        return Ok(Some(DevCommand::Jail(character.to_owned())));
    }
    if lower == "list systems" {
        return Ok(Some(DevCommand::ListSystems));
    }
    if lower == "list advice topics" {
        return Ok(Some(DevCommand::ListAdviceTopics));
    }
    if lower == "skip briefing" {
        return Ok(Some(DevCommand::SkipBriefing));
    }
    if lower.starts_with("advisor: post code ") {
        return number(rest("advisor: post code "))
            .map(|code| Some(DevCommand::AdvisorPostCode(code)));
    }
    if lower.starts_with("release advice topic ") {
        return number(rest("release advice topic "))
            .map(|topic| Some(DevCommand::ReleaseAdviceTopic(topic)));
    }
    Ok(None)
}

/// A trailing `within <n> frames` on a `wait until` line: the line without
/// it, and the frame budget.
#[cfg(not(target_arch = "wasm32"))]
fn split_budget(what: &str) -> Result<(&str, Option<u32>), String> {
    let words: Vec<&str> = what.split_whitespace().collect();
    let [.., "within", n, "frames" | "frame"] = words.as_slice() else {
        return Ok((what, None));
    };
    match n.parse::<u32>() {
        Ok(n) if n > 0 => {
            let cut = what.rfind("within").unwrap_or(what.len());
            Ok((what[..cut].trim(), Some(n.min(WAIT_UNTIL_FRAMES))))
        }
        _ => Err("within takes a frame count from 1".into()),
    }
}

/// `<alliance|empire> <ship|troop|facility> <n>` after `wait until tech`.
#[cfg(not(target_arch = "wasm32"))]
fn wait_tech(side: &str, tree: &str, level: &str) -> Result<DevCommand, String> {
    use rebellion_core::research::TechType;
    let alliance = match side {
        "alliance" => true,
        "empire" => false,
        _ => return Err(format!("tech side {side:?}: use alliance or empire")),
    };
    let tech = match tree {
        "ship" => TechType::Ship,
        "troop" => TechType::Troop,
        "facility" => TechType::Facility,
        _ => return Err(format!("tech tree {tree:?}: use ship, troop or facility")),
    };
    let level = level
        .parse()
        .map_err(|_| format!("tech level {level:?} is not a whole number"))?;
    Ok(DevCommand::WaitUntil(WaitCondition::Tech {
        alliance,
        tech,
        level,
    }))
}

/// The rect a `<window>/<control>` name has in the last drawn frame, or the
/// names registered under its window to choose from.
#[cfg(not(target_arch = "wasm32"))]
pub fn control_rect(name: &str) -> Result<rebellion_render::control_registry::ControlRect, String> {
    use rebellion_render::control_registry::{find, names};
    find(name).ok_or_else(|| {
        let window = name.split_once('/').map_or(name, |(window, _)| window);
        let under = names(Some(window));
        if under.is_empty() {
            let mut windows: Vec<String> = names(None)
                .iter()
                .filter_map(|known| known.split_once('/').map(|(head, _)| head.to_owned()))
                .collect();
            windows.dedup();
            format!(
                "no control {name:?}; no controls under {window:?}, windows with controls: [{}]",
                windows.join(", ")
            )
        } else {
            format!(
                "no control {name:?}; under {window}: [{}]",
                under.join(", ")
            )
        }
    })
}

/// `<x>,<y>` in the 640×480 logical screen.
#[cfg(not(target_arch = "wasm32"))]
fn point(text: &str) -> Result<(f32, f32), String> {
    let parse = |part: Option<&str>| part.and_then(|p| p.trim().parse::<f32>().ok());
    let mut parts = text.split(',');
    match (parse(parts.next()), parse(parts.next()), parts.next()) {
        (Some(x), Some(y), None) if (0.0..640.0).contains(&x) && (0.0..480.0).contains(&y) => {
            Ok((x, y))
        }
        _ => Err(format!("point {text:?}: use <x>,<y> inside 640x480")),
    }
}

/// `<x>,<y> to <x>,<y>`, the two ends of a drag.
#[cfg(not(target_arch = "wasm32"))]
fn span(args: &str, command: &str) -> Result<((f32, f32), (f32, f32)), String> {
    let Some(split) = args.to_ascii_lowercase().find(" to ") else {
        return Err(format!("{command} takes `<x>,<y> to <x>,<y>`"));
    };
    Ok((point(&args[..split])?, point(&args[split + 4..])?))
}

/// A decimal or `0x` hexadecimal number.
#[cfg(not(target_arch = "wasm32"))]
fn number(text: &str) -> Result<u16, String> {
    let text = text.trim();
    let parsed = match text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        Some(hex) => u16::from_str_radix(hex, 16),
        None => text.parse(),
    };
    parsed.map_err(|_| format!("{text:?} is not a number"))
}

#[cfg(not(target_arch = "wasm32"))]
fn message_index(args: &str) -> Result<DevCommand, String> {
    let (name, row) = match args.rsplit_once(' ') {
        Some((head, tail)) if tail.eq_ignore_ascii_case("last") => (head.trim(), Some(Row::Last)),
        Some((head, tail)) => match tail.parse::<usize>() {
            Ok(0) => return Err("rows count from 1".into()),
            Ok(n) => (head.trim(), Some(Row::Number(n))),
            Err(_) => (args, None),
        },
        None => (args, None),
    };
    let wanted = name.trim_end_matches('s');
    let category = MESSAGE_CATEGORIES
        .iter()
        .find(|(label, _)| label.eq_ignore_ascii_case(wanted) || label.eq_ignore_ascii_case(name))
        .map(|&(_, command)| command)
        .ok_or_else(|| format!("no message category {name:?}"))?;
    Ok(DevCommand::OpenMessageIndex { category, row })
}

#[cfg(not(target_arch = "wasm32"))]
fn post_message(args: &str) -> Result<DevCommand, String> {
    const CLASSES: [(&str, MessageClass); 6] = [
        ("uprising began", MessageClass::UprisingBegan),
        ("uprising ended", MessageClass::UprisingEnded),
        ("blockade", MessageClass::Blockade),
        ("fleet arrival", MessageClass::FleetArrival),
        ("loyalty joins", MessageClass::LoyaltyJoins),
        ("loyalty neutral", MessageClass::LoyaltyNeutral),
    ];
    let lower = args.to_ascii_lowercase();
    let Some(split) = lower.find(" at ") else {
        return Err("post message takes `<class> at <system>`".into());
    };
    let class = CLASSES
        .iter()
        .find(|(name, _)| *name == lower[..split].trim())
        .map(|&(_, class)| class)
        .ok_or_else(|| format!("no message class {:?}", args[..split].trim()))?;
    let system = args[split + 4..].trim();
    if system.is_empty() {
        return Err("post message takes a system name".into());
    }
    Ok(DevCommand::PostMessage {
        class,
        system: system.to_owned(),
    })
}

/// The `[alliance|empire]` argument of a `List` command, or none.
#[cfg(not(target_arch = "wasm32"))]
fn side_filter(text: &str) -> Result<Option<Faction>, String> {
    rebellion_data::readouts::parse_side(text)
}

/// A comma-separated list of character names.
#[cfg(not(target_arch = "wasm32"))]
fn names(text: &str) -> Vec<String> {
    text.split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect()
}

/// The mission kinds `Mission:` names, by the dialog's own names (and
/// `Research(Ship)` for each research tree), longest first. Sabotage and
/// Death Star Sabotage take the first object at the system the dialog offers
/// them for (`dev_actions::mission_order`). Assassination, Abduction and
/// Rescue are absent: each needs a target character the line cannot name.
#[cfg(not(target_arch = "wasm32"))]
fn mission_kind_names() -> Vec<(String, MissionKind)> {
    let mut named: Vec<(String, MissionKind)> = [
        MissionKind::Diplomacy,
        MissionKind::Recruitment,
        MissionKind::Espionage,
        MissionKind::InciteUprising,
        MissionKind::SubdueUprising,
        MissionKind::Move,
        MissionKind::Sabotage,
        MissionKind::DeathStarSabotage,
    ]
    .into_iter()
    .chain(TechType::ALL.map(MissionKind::Research))
    .map(|kind| {
        let name = kind.name();
        (name.to_ascii_lowercase(), kind)
    })
    .collect();
    for (tree, tech) in [
        ("ship", TechType::Ship),
        ("troop", TechType::Troop),
        ("facility", TechType::Facility),
    ] {
        named.push((format!("research({tree})"), MissionKind::Research(tech)));
    }
    named.sort_by_key(|(name, _)| std::cmp::Reverse(name.len()));
    named
}

/// `<character>[, <character>...] <kind> at <system> [ship <tag>] [decoys
/// <character>, ...]`.
#[cfg(not(target_arch = "wasm32"))]
fn mission(args: &str) -> Result<DevCommand, String> {
    const USAGE: &str = "mission takes `<character>[, ...] <kind> at <system> [ship <tag>] \
                         [decoys <character>, ...]`";
    let lower = args.to_ascii_lowercase();
    let (body, decoys) = match lower.find(" decoys ") {
        Some(split) => (&args[..split], names(&args[split + " decoys ".len()..])),
        None => (args, Vec::new()),
    };
    let Some(at) = body.to_ascii_lowercase().rfind(" at ") else {
        return Err(USAGE.into());
    };
    let system = body[at + " at ".len()..].trim();
    let (system, ship) = match system.to_ascii_lowercase().rfind(" ship ") {
        Some(split) => {
            let tag = system[split + " ship ".len()..].trim();
            let tag = tag
                .parse::<u32>()
                .map_err(|_| format!("{USAGE}; {tag:?} is not a ship tag"))?;
            (system[..split].trim(), Some(tag))
        }
        None => (system, None),
    };
    let head = body[..at].trim();
    let head_lower = head.to_ascii_lowercase();
    let (kind, team) = mission_kind_names()
        .into_iter()
        .find_map(|(name, kind)| {
            let team = head_lower
                .strip_suffix(&name)
                .filter(|team| team.ends_with(char::is_whitespace))?;
            Some((kind, names(&head[..team.len()])))
        })
        .ok_or_else(|| format!("{USAGE}; no mission kind ends {head:?}"))?;
    if team.is_empty() || system.is_empty() {
        return Err(USAGE.into());
    }
    Ok(DevCommand::Mission {
        team,
        kind,
        system: system.to_owned(),
        ship,
        decoys,
    })
}

/// `<character> for <alliance|empire> [at <system>]`.
#[cfg(not(target_arch = "wasm32"))]
fn recruit(args: &str) -> Result<DevCommand, String> {
    const USAGE: &str = "recruit takes `<character> for <alliance|empire> [at <system>]`";
    let Some(split) = args.to_ascii_lowercase().rfind(" for ") else {
        return Err(USAGE.into());
    };
    let character = args[..split].trim();
    let tail = args[split + " for ".len()..].trim();
    let (side, system) = match tail.to_ascii_lowercase().find(" at ") {
        Some(at) => (&tail[..at], Some(tail[at + " at ".len()..].trim())),
        None => (tail, None),
    };
    let Some(side) = side_filter(side)? else {
        return Err(USAGE.into());
    };
    if character.is_empty() || system.is_some_and(str::is_empty) {
        return Err(USAGE.into());
    }
    Ok(DevCommand::Recruit {
        character: character.to_owned(),
        side,
        system: system.map(str::to_owned),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_palette_is_on_in_debug_builds_or_when_the_dev_switch_is_on() {
        for debug_build in [false, true] {
            for on in ["1", "true", "Yes", " on "] {
                assert!(palette_switch(Some(on), debug_build), "{on:?}");
            }
            for off in [None, Some(""), Some("0"), Some("off")] {
                assert_eq!(palette_switch(off, debug_build), debug_build, "{off:?}");
            }
        }
    }

    #[test]
    fn a_held_drag_and_its_release_parse_apart_from_a_whole_drag() {
        assert_eq!(
            parse_line("Hold drag 100,200 to 140,210"),
            Ok(Some(DevCommand::HoldDrag {
                from: (100.0, 200.0),
                to: (140.0, 210.0)
            }))
        );
        assert_eq!(
            parse_line("release at 140,210"),
            Ok(Some(DevCommand::Release { x: 140.0, y: 210.0 }))
        );
        assert_eq!(
            parse_line("Drag 100,200 to 140,210"),
            Ok(Some(DevCommand::Drag {
                from: (100.0, 200.0),
                to: (140.0, 210.0),
                ctrl: false
            }))
        );
        assert_eq!(
            parse_line("Ctrl drag 100,200 to 140,210"),
            Ok(Some(DevCommand::Drag {
                from: (100.0, 200.0),
                to: (140.0, 210.0),
                ctrl: true
            }))
        );
        assert!(parse_line("Hold drag 100,200").is_err());
        assert_eq!(
            parse_line("Release advice topic 3"),
            Ok(Some(DevCommand::ReleaseAdviceTopic(3)))
        );
    }

    #[test]
    fn the_four_bombardment_orders_parse_with_a_fleet_name() {
        use rebellion_core::bombardment::BombardKind;
        for (line, kind) in [
            ("Bombard military: Fleet 1", BombardKind::Military),
            ("Bombard civilian: Fleet 1", BombardKind::Civilian),
            ("bombard general: Fleet 1", BombardKind::General),
            ("Destroy system: Fleet 1", BombardKind::DestroySystem),
        ] {
            assert_eq!(
                parse_line(line),
                Ok(Some(DevCommand::Bombard {
                    kind,
                    side: None,
                    fleet: "Fleet 1".into()
                }))
            );
        }
        assert!(parse_line("Destroy system:").is_err());
    }

    #[test]
    fn a_strike_order_may_name_the_side_whose_fleet_strikes() {
        use rebellion_core::bombardment::BombardKind;
        for (line, side) in [
            ("Destroy system: empire Fleet 1", Some(Faction::Empire)),
            ("Destroy system: Alliance Fleet 1", Some(Faction::Alliance)),
            ("Destroy system: Fleet 1", None),
        ] {
            assert_eq!(
                parse_line(line),
                Ok(Some(DevCommand::Bombard {
                    kind: BombardKind::DestroySystem,
                    side,
                    fleet: "Fleet 1".into()
                })),
                "{line}"
            );
        }
        assert!(parse_line("Destroy system: empire").is_err());
        assert!(parse_line("Bombard general: empire  ").is_err());
        assert_eq!(
            parse_line("List fleets empire"),
            Ok(Some(DevCommand::ListFleets(Some(Faction::Empire))))
        );
        assert_eq!(
            parse_line("list fleets"),
            Ok(Some(DevCommand::ListFleets(None)))
        );
        assert_eq!(
            parse_line("List fleets").unwrap().unwrap().readiness(),
            Readiness::Anywhere
        );
        assert!(parse_line("List fleets rebels").is_err());
    }

    #[test]
    fn the_assault_order_parses_with_a_fleet_name() {
        assert_eq!(
            parse_line("Assault: Fleet 1"),
            Ok(Some(DevCommand::Assault("Fleet 1".into())))
        );
        assert_eq!(
            parse_line("Assault: Fleet 1").unwrap().unwrap().readiness(),
            Readiness::Galaxy
        );
        assert!(parse_line("assault:").is_err());
    }

    #[test]
    fn a_mission_line_names_sabotage_and_death_star_sabotage_by_the_dialogs_names() {
        assert_eq!(
            parse_line("Mission: Han Solo, Chewbacca Death Star Sabotage at Coruscant decoys Leia"),
            Ok(Some(DevCommand::Mission {
                team: vec!["Han Solo".into(), "Chewbacca".into()],
                kind: MissionKind::DeathStarSabotage,
                system: "Coruscant".into(),
                ship: None,
                decoys: vec!["Leia".into()],
            }))
        );
        assert_eq!(
            parse_line("mission: Han Solo sabotage at Kuat"),
            Ok(Some(DevCommand::Mission {
                team: vec!["Han Solo".into()],
                kind: MissionKind::Sabotage,
                system: "Kuat".into(),
                ship: None,
                decoys: Vec::new(),
            }))
        );
        assert!(parse_line("Mission: Han Solo Assassination at Kuat").is_err());
    }

    #[test]
    fn a_mission_line_names_its_target_ship_by_tag_before_its_decoys() {
        assert_eq!(
            parse_line("Mission: Han Solo Sabotage at Corellia ship 812 decoys Leia"),
            Ok(Some(DevCommand::Mission {
                team: vec!["Han Solo".into()],
                kind: MissionKind::Sabotage,
                system: "Corellia".into(),
                ship: Some(812),
                decoys: vec!["Leia".into()],
            }))
        );
        assert_eq!(
            parse_line("mission: Wedge Antilles death star sabotage at Coruscant SHIP 7"),
            Ok(Some(DevCommand::Mission {
                team: vec!["Wedge Antilles".into()],
                kind: MissionKind::DeathStarSabotage,
                system: "Coruscant".into(),
                ship: Some(7),
                decoys: Vec::new(),
            }))
        );
        assert!(parse_line("Mission: Han Solo Sabotage at Corellia ship Devastator").is_err());
        assert!(parse_line("Mission: Han Solo Sabotage at  decoys Leia").is_err());
    }

    #[test]
    fn a_galaxy_command_waits_off_the_galaxy_and_an_anywhere_command_does_not() {
        let staging = parse_line("Maintenance capacity: empire +600")
            .unwrap()
            .unwrap()
            .readiness();
        assert!(!staging.admits(false), "the main menu is not the galaxy");
        assert!(staging.admits(true));
        let readout = parse_line("Show side empire").unwrap().unwrap().readiness();
        assert!(readout.admits(false));
    }

    #[test]
    fn queue_build_takes_a_side_a_class_and_a_system() {
        assert_eq!(
            parse_line("Queue build: empire Death Star at Coruscant"),
            Ok(Some(DevCommand::QueueBuild {
                alliance: false,
                class: "Death Star".into(),
                system: "Coruscant".into(),
            }))
        );
        assert_eq!(
            parse_line("queue build: Alliance Bulk Transport at Sluis Van")
                .unwrap()
                .unwrap(),
            DevCommand::QueueBuild {
                alliance: true,
                class: "Bulk Transport".into(),
                system: "Sluis Van".into(),
            }
        );
        assert_eq!(
            parse_line("Queue build: empire Death Star at Coruscant")
                .unwrap()
                .unwrap()
                .readiness(),
            Readiness::Galaxy
        );
        for bad in [
            "Queue build:",
            "Queue build: empire Death Star",
            "Queue build: rebels Death Star at Coruscant",
            "Queue build: empire at Coruscant",
            "Queue build: empire Death Star at",
        ] {
            assert!(parse_line(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn strip_defenders_takes_a_system_and_waits_for_the_galaxy() {
        let command = parse_line("Strip defenders at Coruscant");
        assert_eq!(
            command,
            Ok(Some(DevCommand::StripDefenders("Coruscant".into())))
        );
        assert_eq!(command.unwrap().unwrap().readiness(), Readiness::Galaxy);
    }

    #[test]
    fn embark_takes_a_fleet_and_a_positive_count_and_waits_for_the_galaxy() {
        let command = parse_line("Embark: Fleet 1 3");
        assert_eq!(
            command,
            Ok(Some(DevCommand::Embark {
                fleet: "Fleet 1".into(),
                count: 3
            }))
        );
        assert_eq!(command.unwrap().unwrap().readiness(), Readiness::Galaxy);
        for bad in [
            "Embark:",
            "Embark: Fleet",
            "Embark: Fleet 1 0",
            "Embark: 3",
            "Embark: Fleet 1 x",
        ] {
            assert!(parse_line(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn maintenance_capacity_takes_a_side_and_a_positive_amount() {
        assert_eq!(
            parse_line("Maintenance capacity: empire +600"),
            Ok(Some(DevCommand::MaintenanceCapacity {
                alliance: false,
                amount: 600
            }))
        );
        assert_eq!(
            parse_line("maintenance capacity: Alliance +50")
                .unwrap()
                .unwrap()
                .readiness(),
            Readiness::Galaxy
        );
        for bad in [
            "Maintenance capacity:",
            "Maintenance capacity: empire",
            "Maintenance capacity: empire 600",
            "Maintenance capacity: empire +0",
            "Maintenance capacity: empire +600 more",
            "Maintenance capacity: rebels +600",
        ] {
            assert!(parse_line(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn the_jail_command_parses_with_a_character_name() {
        assert_eq!(
            parse_line("Jail: Han Solo"),
            Ok(Some(DevCommand::Jail("Han Solo".into())))
        );
        assert_eq!(
            parse_line("jail: Han Solo").unwrap().unwrap().readiness(),
            Readiness::Galaxy
        );
        assert!(parse_line("Jail:").is_err());
    }

    #[test]
    fn quit_parses_in_any_case_and_runs_anywhere() {
        // Staging only: the galaxy has no Escape route to the menu
        // (FUN_00422ce0), so a run ends with this or F1 and Exit.
        for line in ["Quit", "quit", "  QUIT  "] {
            let command = parse_line(line);
            assert_eq!(command, Ok(Some(DevCommand::Quit)), "{line}");
            assert_eq!(command.unwrap().unwrap().readiness(), Readiness::Anywhere);
        }
    }

    #[test]
    fn show_system_takes_a_system_name_and_runs_anywhere() {
        let command = parse_line("Show system Yavin");
        assert_eq!(command, Ok(Some(DevCommand::ShowSystem("Yavin".into()))));
        assert_eq!(command.unwrap().unwrap().readiness(), Readiness::Anywhere);
        assert!(parse_line("show system").is_err());
    }

    #[test]
    fn the_seed_switch_takes_a_whole_number() {
        assert_eq!(seed_value(Some(" 42\n")), Some(42));
        assert_eq!(seed_value(Some("seed")), None);
        assert_eq!(seed_value(Some("-1")), None);
        assert_eq!(seed_value(None), None);
    }

    #[test]
    fn a_script_file_reads_into_its_lines_and_a_missing_one_into_none() {
        let path = std::env::temp_dir().join(format!(
            "open-rebellion-dev-commands-{}.txt",
            std::process::id()
        ));
        std::fs::write(&path, "# setup\nStart game: Empire\n").unwrap();
        let mut script = CommandScript::from_file(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert_eq!(
            script.step(true),
            ScriptStep::Run("Start game: Empire".into())
        );
        assert_eq!(script.step(true), ScriptStep::Done);
        assert!(CommandScript::from_file(&path).is_none());
    }

    #[test]
    fn a_script_runs_its_command_lines_in_order_without_comments_or_blanks() {
        let mut script = CommandScript::parse(
            "# reach Sullust's fleet\n\n  Start game: Alliance  \nOpen Fleet window: Sullust\n",
        );
        assert_eq!(
            script.step(true),
            ScriptStep::Run("Start game: Alliance".into())
        );
        assert_eq!(
            script.step(true),
            ScriptStep::Run("Open Fleet window: Sullust".into())
        );
        assert_eq!(script.step(true), ScriptStep::Done);
    }

    #[test]
    fn a_script_waits_while_the_game_is_not_ready_and_is_done_only_when_ready_after() {
        let mut script = CommandScript::parse("Start game: Empire\nOpen sector window: Fakir\n");
        assert_eq!(script.step(false), ScriptStep::Wait);
        assert_eq!(
            script.step(true),
            ScriptStep::Run("Start game: Empire".into())
        );
        // The start is setting up.
        assert_eq!(script.step(false), ScriptStep::Wait);
        assert_eq!(
            script.step(true),
            ScriptStep::Run("Open sector window: Fakir".into())
        );
        // The last command has not run yet.
        assert_eq!(script.step(false), ScriptStep::Wait);
        assert_eq!(script.step(true), ScriptStep::Done);
    }
}
