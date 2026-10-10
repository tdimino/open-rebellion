//! TEXTSTRA.DLL's `RT_RCDATA` entries: the message templates the original
//! formats with `FUN_0060b9d0`, and the advice topic tables
//! (`FUN_0048b460`). Native builds read the DLL; the browser reads
//! `textstra-rcdata.json` from the runtime pack, one Latin-1 char per byte.

use std::collections::HashMap;

use anyhow::{Context, Result};

/// TEXTSTRA's RCDATA entries by resource id.
#[derive(Debug, Clone, Default)]
pub struct TextTemplates {
    entries: HashMap<u16, Vec<u8>>,
}

impl TextTemplates {
    /// Read the entries from `TEXTSTRA.DLL` in `gdata_path`.
    ///
    /// # Errors
    /// Returns an error if the DLL is missing or malformed.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load(gdata_path: &std::path::Path) -> Result<Self> {
        let path = gdata_path.join("TEXTSTRA.DLL");
        let entries = dat_dumper::types::textstra::load_rcdata(&path)
            .with_context(|| format!("loading TEXTSTRA RCDATA from {}", path.display()))?;
        Ok(Self { entries })
    }

    /// Read the entries from the runtime pack's `textstra-rcdata.json`.
    ///
    /// # Errors
    /// Returns an error if the JSON is malformed or a char is not Latin-1.
    pub fn from_json(bytes: &[u8]) -> Result<Self> {
        let text: HashMap<u16, String> =
            serde_json::from_slice(bytes).context("parsing textstra-rcdata.json")?;
        let entries = text
            .into_iter()
            .map(|(id, value)| {
                let bytes = value
                    .chars()
                    .map(|c| u8::try_from(u32::from(c)))
                    .collect::<Result<Vec<u8>, _>>()
                    .with_context(|| format!("RCDATA {id:#06x} is not Latin-1"))?;
                Ok((id, bytes))
            })
            .collect::<Result<_>>()?;
        Ok(Self { entries })
    }

    /// The raw bytes of RCDATA `id`.
    #[must_use]
    pub fn get(&self, id: u16) -> Option<&[u8]> {
        self.entries.get(&id).map(Vec::as_slice)
    }

    /// Format template `id` with up to four arguments (`FUN_0060b9d0` →
    /// `FUN_0060b840`). A placeholder is `|` (RCDATA `0xbb8`), the argument
    /// index byte and a little-endian u32 kind; the template ends at `0x01`
    /// (`0xbb9`). A missing argument renders nothing.
    #[must_use]
    pub fn format(&self, id: u16, args: &[Option<&TemplateArg>]) -> Option<String> {
        Some(format_template(self.get(id)?, args))
    }

    /// Whether no entries were loaded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// The placeholder prefix (TEXTSTRA RCDATA `0xbb8`).
const PLACEHOLDER: u8 = b'|';
/// The template end mark (TEXTSTRA RCDATA `0xbb9`).
const END: u8 = 0x01;

/// How a game object renders into a template: its render method switches on
/// the placeholder's kind (vtable slot 0 of the object's `+0x30`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TemplateArg {
    /// Kind 1: the proper name ("Drall", "Empire").
    pub name: String,
    /// Kind 3: the class name ("Star Destroyer").
    pub class: Option<String>,
    /// Kind 4: the side adjective ("Imperial").
    pub adjective: Option<String>,
}

impl TemplateArg {
    /// An object that renders only its name.
    #[must_use]
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// A system, named under kind 1 and kind 4: Battle Alert template `0x7022`
    /// passes the system as kind 4. hyp: a system's kind-4 render is untraced.
    #[must_use]
    pub fn place(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            adjective: Some(name.clone()),
            name,
            class: None,
        }
    }

    /// A side: kind 1 renders TEXTSTRA `0x4d50`/`0x4d51` ("join the Empire")
    /// and kind 4 renders `0x4d70`/`0x4d71`. hyp: the faction's render
    /// method is untraced; the strings sit in that table.
    #[must_use]
    pub fn side(alliance: bool) -> Self {
        let (name, adjective) = if alliance {
            ("Alliance", "Alliance")
        } else {
            ("Empire", "Imperial")
        };
        Self {
            name: name.into(),
            class: None,
            adjective: Some(adjective.into()),
        }
    }

    /// The text for `kind`; kinds the port does not carry render nothing.
    fn render(&self, kind: u32) -> &str {
        match kind {
            1 => &self.name,
            3 => self.class.as_deref().unwrap_or_default(),
            4 => self.adjective.as_deref().unwrap_or_default(),
            _ => "",
        }
    }
}

/// `FUN_0060b840`: copy literal bytes as Latin-1 and replace each 6-byte
/// placeholder with its argument's text.
fn format_template(bytes: &[u8], args: &[Option<&TemplateArg>]) -> String {
    let mut out = String::new();
    let mut at = 0;
    while let Some(&byte) = bytes.get(at) {
        match byte {
            END => break,
            PLACEHOLDER if at + 6 <= bytes.len() => {
                let index = usize::from(bytes[at + 1]);
                let kind = u32::from_le_bytes([
                    bytes[at + 2],
                    bytes[at + 3],
                    bytes[at + 4],
                    bytes[at + 5],
                ]);
                if let Some(Some(arg)) = index.checked_sub(1).and_then(|slot| args.get(slot)) {
                    out.push_str(arg.render(kind));
                }
                at += 6;
            }
            _ => {
                out.push(char::from(byte));
                at += 1;
            }
        }
    }
    out
}
