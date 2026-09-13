//! Parser for the deterministic browser runtime asset pack.

use std::collections::HashMap;
use std::fmt;

const MAGIC: &[u8; 4] = b"ORPK";
const VERSION: u16 = 3;
const HEADER_LEN: usize = 12;
const ENTRY_HEADER_LEN: usize = 7;
const MAX_ENTRIES: usize = 10_000;
const MAX_KEY_LEN: usize = 512;

const KIND_GAME_DATA: u8 = 0;
const KIND_BITMAP: u8 = 1;
const KIND_AUDIO: u8 = 2;
const KIND_ADVISOR_FRAME: u8 = 3;
const KIND_TACTICAL_MESH: u8 = 4;
const KIND_TACTICAL_TEXTURE: u8 = 5;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RuntimePack {
    pub game_files: HashMap<String, Vec<u8>>,
    pub bitmaps: HashMap<String, Vec<u8>>,
    pub audio_files: HashMap<String, Vec<u8>>,
    pub advisor_frames: HashMap<String, Vec<u8>>,
    pub tactical_meshes: HashMap<String, Vec<u8>>,
    pub tactical_textures: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackError {
    Truncated(&'static str),
    InvalidMagic,
    UnsupportedVersion(u16),
    UnsupportedFlags(u16),
    TooManyEntries(u32),
    EmptyKey,
    KeyTooLong(u16),
    InvalidUtf8,
    UnknownKind(u8),
    DuplicateKey(String),
    TrailingBytes(usize),
}

impl fmt::Display for PackError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated(section) => write!(formatter, "truncated {section}"),
            Self::InvalidMagic => formatter.write_str("invalid runtime-pack magic"),
            Self::UnsupportedVersion(version) => {
                write!(formatter, "unsupported runtime-pack version {version}")
            }
            Self::UnsupportedFlags(flags) => {
                write!(formatter, "unsupported runtime-pack flags {flags:#06x}")
            }
            Self::TooManyEntries(count) => {
                write!(formatter, "runtime pack declares too many entries: {count}")
            }
            Self::EmptyKey => formatter.write_str("runtime-pack entry has an empty key"),
            Self::KeyTooLong(length) => {
                write!(formatter, "runtime-pack key is too long: {length} bytes")
            }
            Self::InvalidUtf8 => formatter.write_str("runtime-pack key is not valid UTF-8"),
            Self::UnknownKind(kind) => write!(formatter, "unknown runtime-pack entry kind {kind}"),
            Self::DuplicateKey(key) => write!(formatter, "duplicate runtime-pack key {key}"),
            Self::TrailingBytes(count) => {
                write!(formatter, "runtime pack contains {count} trailing bytes")
            }
        }
    }
}

pub fn parse_runtime_pack(bytes: &[u8]) -> Result<RuntimePack, PackError> {
    if bytes.len() < HEADER_LEN {
        return Err(PackError::Truncated("header"));
    }
    if &bytes[..4] != MAGIC {
        return Err(PackError::InvalidMagic);
    }

    let version = u16::from_le_bytes([bytes[4], bytes[5]]);
    if version != VERSION {
        return Err(PackError::UnsupportedVersion(version));
    }
    let flags = u16::from_le_bytes([bytes[6], bytes[7]]);
    if flags != 0 {
        return Err(PackError::UnsupportedFlags(flags));
    }
    let count = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    if count as usize > MAX_ENTRIES {
        return Err(PackError::TooManyEntries(count));
    }

    let mut cursor = HEADER_LEN;
    let mut pack = RuntimePack {
        game_files: HashMap::with_capacity(count as usize),
        bitmaps: HashMap::with_capacity(count as usize),
        audio_files: HashMap::new(),
        advisor_frames: HashMap::new(),
        tactical_meshes: HashMap::new(),
        tactical_textures: HashMap::new(),
    };

    for _ in 0..count {
        let entry_header = take(bytes, &mut cursor, ENTRY_HEADER_LEN, "entry header")?;
        let kind = entry_header[0];
        let key_len = u16::from_le_bytes([entry_header[1], entry_header[2]]);
        let data_len = u32::from_le_bytes([
            entry_header[3],
            entry_header[4],
            entry_header[5],
            entry_header[6],
        ]) as usize;

        if key_len == 0 {
            return Err(PackError::EmptyKey);
        }
        if key_len as usize > MAX_KEY_LEN {
            return Err(PackError::KeyTooLong(key_len));
        }
        let key_bytes = take(bytes, &mut cursor, key_len as usize, "entry key")?;
        let key = std::str::from_utf8(key_bytes)
            .map_err(|_| PackError::InvalidUtf8)?
            .to_owned();
        let data = take(bytes, &mut cursor, data_len, "entry data")?.to_vec();

        let destination = match kind {
            KIND_GAME_DATA => &mut pack.game_files,
            KIND_BITMAP => &mut pack.bitmaps,
            KIND_AUDIO => &mut pack.audio_files,
            KIND_ADVISOR_FRAME => &mut pack.advisor_frames,
            KIND_TACTICAL_MESH => &mut pack.tactical_meshes,
            KIND_TACTICAL_TEXTURE => &mut pack.tactical_textures,
            other => return Err(PackError::UnknownKind(other)),
        };
        if destination.insert(key.clone(), data).is_some() {
            return Err(PackError::DuplicateKey(key));
        }
    }

    if cursor != bytes.len() {
        return Err(PackError::TrailingBytes(bytes.len() - cursor));
    }
    Ok(pack)
}

fn take<'a>(
    bytes: &'a [u8],
    cursor: &mut usize,
    length: usize,
    section: &'static str,
) -> Result<&'a [u8], PackError> {
    let end = cursor
        .checked_add(length)
        .ok_or(PackError::Truncated(section))?;
    let slice = bytes
        .get(*cursor..end)
        .ok_or(PackError::Truncated(section))?;
    *cursor = end;
    Ok(slice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[expect(
        clippy::cast_possible_truncation,
        reason = "The test pack contains small literal entries that fit its fixed-width header."
    )]
    fn pack(entries: &[(u8, &str, &[u8])]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&VERSION.to_le_bytes());
        bytes.extend_from_slice(&0_u16.to_le_bytes());
        bytes.extend_from_slice(&(entries.len() as u32).to_le_bytes());
        for (kind, key, data) in entries {
            bytes.push(*kind);
            bytes.extend_from_slice(&(key.len() as u16).to_le_bytes());
            bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
            bytes.extend_from_slice(key.as_bytes());
            bytes.extend_from_slice(data);
        }
        bytes
    }

    #[test]
    fn parses_game_data_bitmap_audio_and_advisor_entries() {
        let bytes = pack(&[
            (KIND_GAME_DATA, "SYSTEMSD.DAT", b"systems"),
            (KIND_GAME_DATA, "textstra.json", b"{}"),
            (KIND_BITMAP, "strategy-dll/900", b"bitmap"),
            (KIND_AUDIO, "music/main_theme.wav", b"wave"),
            (KIND_ADVISOR_FRAME, "alsprite-dll/2002", b"sparse"),
            (KIND_TACTICAL_MESH, "2560/1033", b"mesh"),
            (KIND_TACTICAL_TEXTURE, "SDESTI52.BMP/1033", b"texture"),
        ]);

        let parsed = parse_runtime_pack(&bytes).unwrap();
        assert_eq!(parsed.game_files["SYSTEMSD.DAT"], b"systems");
        assert_eq!(parsed.game_files["textstra.json"], b"{}");
        assert_eq!(parsed.bitmaps["strategy-dll/900"], b"bitmap");
        assert_eq!(parsed.audio_files["music/main_theme.wav"], b"wave");
        assert_eq!(parsed.advisor_frames["alsprite-dll/2002"], b"sparse");
        assert_eq!(parsed.tactical_meshes["2560/1033"], b"mesh");
        assert_eq!(parsed.tactical_textures["SDESTI52.BMP/1033"], b"texture");
    }

    #[test]
    fn rejects_corrupt_or_ambiguous_packs() {
        assert_eq!(
            parse_runtime_pack(b"ORPK"),
            Err(PackError::Truncated("header"))
        );

        let mut bad_magic = pack(&[]);
        bad_magic[0] = b'X';
        assert_eq!(parse_runtime_pack(&bad_magic), Err(PackError::InvalidMagic));

        let duplicate = pack(&[
            (KIND_GAME_DATA, "SYSTEMSD.DAT", b"one"),
            (KIND_GAME_DATA, "SYSTEMSD.DAT", b"two"),
        ]);
        assert_eq!(
            parse_runtime_pack(&duplicate),
            Err(PackError::DuplicateKey("SYSTEMSD.DAT".to_string()))
        );

        let mut trailing = pack(&[]);
        trailing.push(0);
        assert_eq!(
            parse_runtime_pack(&trailing),
            Err(PackError::TrailingBytes(1))
        );
    }

    #[test]
    fn rejects_unknown_entry_kinds_and_truncation() {
        let unknown = pack(&[(9, "mystery", b"bytes")]);
        assert_eq!(parse_runtime_pack(&unknown), Err(PackError::UnknownKind(9)));

        let mut truncated = pack(&[(KIND_BITMAP, "common-dll/20001", b"bitmap")]);
        truncated.pop();
        assert_eq!(
            parse_runtime_pack(&truncated),
            Err(PackError::Truncated("entry data"))
        );
    }
}
