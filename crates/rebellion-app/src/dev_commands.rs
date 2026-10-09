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
    /// Click at a point of the 640×480 logical screen.
    Click { x: f32, y: f32, right: bool },
    /// Press at one point, move, release at another.
    Drag { from: (f32, f32), to: (f32, f32) },
    /// Press and release a named key.
    Press(String),
    /// Open the Message Index on a category command (`0x79..=0x82`), and
    /// with a row, show that message in mode 2.
    OpenMessageIndex { category: u16, row: Option<Row> },
    /// File a message class at a system, as the event that sends it would.
    PostMessage {
        class: MessageClass,
        system: String,
    },
    /// Bring an idle fleet of each side missing from a system there; the
    /// next tick's combat check starts the battle.
    BattleAt(String),
    /// Bring an idle fleet of the side not holding a system there; the
    /// simulation then forms the blockade.
    Blockade(String),
    /// The opening briefing's own skip.
    SkipBriefing,
    /// Post an advice code to the droids, as a game event does.
    AdvisorPostCode(u16),
    /// Release an Advice topic and file its message.
    ReleaseAdviceTopic(u16),
    /// Report every system's holder and the sides with fleets there.
    ListSystems,
    /// Report the Advice topics still held.
    ListAdviceTopics,
    /// Report the game's state: mode, day, speed, open dialogs, side.
    Status,
}

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
    /// Only once the galaxy shows and earlier commands have run.
    Galaxy,
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
            | Self::Click { .. }
            | Self::Drag { .. }
            | Self::Press(_)
            | Self::Status => Readiness::Anywhere,
            _ => Readiness::Galaxy,
        }
    }
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
    if lower.starts_with("right click ") {
        let (x, y) = point(rest("right click "))?;
        return Ok(Some(DevCommand::Click { x, y, right: true }));
    }
    if lower.starts_with("click ") {
        let (x, y) = point(rest("click "))?;
        return Ok(Some(DevCommand::Click { x, y, right: false }));
    }
    if lower.starts_with("drag ") {
        let args = rest("drag ");
        let Some(split) = args.to_ascii_lowercase().find(" to ") else {
            return Err("drag takes `<x>,<y> to <x>,<y>`".into());
        };
        let from = point(&args[..split])?;
        let to = point(&args[split + 4..])?;
        return Ok(Some(DevCommand::Drag { from, to }));
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
    if lower.starts_with("blockade ") {
        return Ok(Some(DevCommand::Blockade(rest("blockade ").to_owned())));
    }
    if lower == "status" {
        return Ok(Some(DevCommand::Status));
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
        return number(rest("advisor: post code ")).map(|code| Some(DevCommand::AdvisorPostCode(code)));
    }
    if lower.starts_with("release advice topic ") {
        return number(rest("release advice topic "))
            .map(|topic| Some(DevCommand::ReleaseAdviceTopic(topic)));
    }
    Ok(None)
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
