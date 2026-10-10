//! The live developer channel: drive a running native game from files.
//!
//! `OPEN_REBELLION_INBOX` names an input file (the launcher makes
//! `<evidence>/commands.in`). The game polls it every 250 ms and queues each
//! new complete line behind the command script, so a line runs exactly as a
//! script line does. Each command's outcome goes to `commands.out` beside it
//! as one JSON line after a `{"protocol":1}` header. Nothing listens on a
//! port, and a line is only ever parsed into a command or refused. The
//! channel opens only behind the dev gate and never in a browser build.
//! `agent_docs/dev-commands.md` documents the protocol.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// The protocol version `commands.out` declares first.
const PROTOCOL: u32 = 1;
/// How often the inbox is checked, in seconds.
const POLL_SECONDS: f64 = 0.25;
/// The longest line accepted; longer ones are refused unread.
const MAX_LINE: usize = 512;

/// The inbox, its results file and the evidence folder captures go to.
pub struct DevChannel {
    inbox: Option<PathBuf>,
    out: Option<File>,
    evidence: PathBuf,
    /// Bytes of the inbox already consumed.
    offset: u64,
    /// Complete lines read so far: the next line's sequence number.
    next_seq: u64,
    /// A partial line held until its newline arrives.
    partial: Vec<u8>,
    /// An over-long line already refused: its bytes are dropped through
    /// the next newline, so it keeps the one sequence number it was given.
    discarding: bool,
    last_poll: f64,
    /// Unnamed captures so far.
    captures: u32,
    /// The command running this frame: its sequence number and line, and the
    /// outcome to report at the frame's end.
    pending: Option<Pending>,
}

/// A command whose result is written at the end of its frame, or later for a
/// wait.
struct Pending {
    seq: u64,
    line: String,
    status: &'static str,
    detail: String,
    path: Option<String>,
}

impl DevChannel {
    /// The channel the environment names, when the dev gate is on. Without
    /// an inbox, captures still go to `OPEN_REBELLION_EVIDENCE` or the
    /// working directory.
    #[must_use]
    pub fn from_env() -> Self {
        let enabled = crate::dev_commands::palette_enabled();
        let inbox = std::env::var_os("OPEN_REBELLION_INBOX")
            .filter(|_| enabled)
            .map(PathBuf::from);
        let evidence = std::env::var_os("OPEN_REBELLION_EVIDENCE")
            .map(PathBuf::from)
            .or_else(|| inbox.as_ref().and_then(|p| p.parent().map(Path::to_path_buf)))
            .unwrap_or_else(|| PathBuf::from("."));
        let out = inbox.as_ref().and_then(|inbox| {
            let path = inbox.with_extension("out");
            match File::create(&path) {
                Ok(mut file) => {
                    let _ = writeln!(file, "{{\"protocol\":{PROTOCOL}}}");
                    eprintln!(
                        "[dev-channel] reading {} and answering in {}",
                        inbox.display(),
                        path.display()
                    );
                    Some(file)
                }
                Err(error) => {
                    eprintln!("[dev-channel] cannot write {}: {error}", path.display());
                    None
                }
            }
        });
        Self {
            inbox,
            out,
            evidence,
            offset: 0,
            next_seq: 1,
            partial: Vec::new(),
            discarding: false,
            last_poll: f64::NEG_INFINITY,
            captures: 0,
            pending: None,
        }
    }

    /// Whether an inbox is open, so the queue never finishes.
    #[must_use]
    pub const fn is_live(&self) -> bool {
        self.inbox.is_some()
    }

    /// The folder captures go to.
    #[must_use]
    pub fn evidence(&self) -> &Path {
        &self.evidence
    }

    /// The number for the next unnamed capture, counting from 1.
    pub fn next_capture_number(&mut self) -> u32 {
        self.captures += 1;
        self.captures
    }

    /// New complete lines since the last poll, each with its sequence
    /// number. Over-long or non-UTF-8 lines are answered `refused` here.
    pub fn poll(&mut self, now: f64) -> Vec<(u64, String)> {
        if now - self.last_poll < POLL_SECONDS {
            return Vec::new();
        }
        self.last_poll = now;
        let Some(path) = self.inbox.clone() else {
            return Vec::new();
        };
        let Ok(mut file) = File::open(&path) else {
            return Vec::new();
        };
        let len = file.metadata().map_or(0, |meta| meta.len());
        if len < self.offset {
            eprintln!("[dev-channel] {} shrank; reading from its start", path.display());
            self.offset = 0;
            self.partial.clear();
            self.discarding = false;
        }
        if len == self.offset || file.seek(SeekFrom::Start(self.offset)).is_err() {
            return Vec::new();
        }
        let mut bytes = Vec::new();
        if file.take(64 * 1024).read_to_end(&mut bytes).is_err() {
            return Vec::new();
        }
        self.offset += bytes.len() as u64;
        let mut bytes = bytes.as_slice();
        if self.discarding {
            match bytes.iter().position(|&b| b == b'\n') {
                Some(end) => {
                    self.discarding = false;
                    bytes = &bytes[end + 1..];
                }
                None => return Vec::new(),
            }
        }
        self.partial.extend_from_slice(bytes);
        let mut lines = Vec::new();
        while let Some(end) = self.partial.iter().position(|&b| b == b'\n') {
            let raw: Vec<u8> = self.partial.drain(..=end).collect();
            let raw = &raw[..raw.len() - 1];
            let seq = self.next_seq;
            self.next_seq += 1;
            match std::str::from_utf8(raw) {
                _ if raw.len() > MAX_LINE => {
                    self.answer(seq, "", "refused", &format!("line over {MAX_LINE} bytes"), None);
                }
                Err(_) => self.answer(seq, "", "refused", "line is not UTF-8", None),
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        self.answer(seq, line, "skipped", "", None);
                    } else {
                        lines.push((seq, line.to_owned()));
                    }
                }
            }
        }
        if self.partial.len() > MAX_LINE {
            let seq = self.next_seq;
            self.next_seq += 1;
            self.partial.clear();
            self.discarding = true;
            self.answer(seq, "", "refused", &format!("line over {MAX_LINE} bytes"), None);
        }
        lines
    }

    /// Start reporting on an inbox command; its result defaults to `done`.
    pub fn begin(&mut self, seq: Option<u64>, line: &str) {
        self.finish();
        self.pending = seq.map(|seq| Pending {
            seq,
            line: line.to_owned(),
            status: "done",
            detail: String::new(),
            path: None,
        });
    }

    /// Mark the running command refused, and log why.
    pub fn refuse(&mut self, why: &str) {
        eprintln!("[dev-command] refused: {why}");
        if let Some(pending) = self.pending.as_mut() {
            pending.status = "refused";
            pending.detail = why.to_owned();
        }
    }

    /// Note a detail on the running command's result.
    pub fn note(&mut self, detail: &str) {
        if let Some(pending) = self.pending.as_mut() {
            if !pending.detail.is_empty() {
                pending.detail.push_str("; ");
            }
            pending.detail.push_str(detail);
        }
    }

    /// Record a capture's path on the running command's result.
    pub fn set_path(&mut self, path: &Path) {
        if let Some(pending) = self.pending.as_mut() {
            pending.path = Some(path.display().to_string());
        }
    }

    /// Write the running command's result, if any.
    pub fn finish(&mut self) {
        if let Some(p) = self.pending.take() {
            self.answer(p.seq, &p.line, p.status, &p.detail, p.path.as_deref());
        }
    }

    fn answer(&mut self, seq: u64, line: &str, status: &str, detail: &str, path: Option<&str>) {
        let Some(out) = self.out.as_mut() else {
            return;
        };
        let mut json = format!(
            "{{\"seq\":{seq},\"line\":{},\"status\":\"{status}\",\"detail\":{}",
            json_string(line),
            json_string(detail)
        );
        if let Some(path) = path {
            json.push_str(&format!(",\"path\":{}", json_string(path)));
        }
        json.push('}');
        if writeln!(out, "{json}").and_then(|()| out.flush()).is_err() {
            eprintln!("[dev-channel] cannot write a result; closing the results file");
            self.out = None;
        }
    }
}

/// `text` as a JSON string literal.
fn json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if u32::from(c) < 0x20 => out.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Save the frame just drawn as `<dir>/<name>.png`. Call it after the
/// frame's drawing and before `next_frame`; macroquad's screen data already
/// runs top row first. Alpha is forced opaque, as the window shows it.
pub fn capture_frame(dir: &Path, name: &str) -> Result<(PathBuf, u16, u16), String> {
    let mut image = macroquad::texture::get_screen_data();
    for pixel in image.bytes.chunks_exact_mut(4) {
        pixel[3] = 255;
    }
    std::fs::create_dir_all(dir).map_err(|e| format!("cannot make {}: {e}", dir.display()))?;
    let path = dir.join(format!("{name}.png"));
    // export_png unwraps its write; a failed capture must not end the game.
    let target = path.to_string_lossy().into_owned();
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| image.export_png(&target)))
        .map_err(|_| format!("cannot write {target}"))?;
    Ok((path, image.width, image.height))
}
