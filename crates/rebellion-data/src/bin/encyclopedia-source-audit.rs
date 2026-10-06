use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{ensure, Context, Result};
use rebellion_data::encyclopedia_catalog::load_encyclopedia_catalog;
use rebellion_data::encyclopedia_topics::{
    load_encyclopedia_topics, parse_encyclopedia_source_with_manifest, EncyclopediaAudience,
    EncyclopediaMissingPart, EncyclopediaTopicCatalog, ENCYCLOPEDIA_SOURCE_EMPTY_OBJECT_IDS,
};
use serde::Serialize;

#[derive(Clone, Serialize)]
struct AudienceSummary {
    complete: usize,
    missing_text: usize,
    missing_artwork_mapping: usize,
    missing_system_picture: usize,
    distinct_bound_artwork: usize,
    missing_entries: Vec<MissingEntry>,
}

#[derive(Clone, Serialize)]
struct MissingEntry {
    object_id: String,
    title: String,
    missing: Vec<&'static str>,
}

#[derive(Serialize)]
struct AuditSummary {
    schema_version: u32,
    catalog_sha256: String,
    encytext_sha256: String,
    encybmap_sha256: String,
    index_entries: usize,
    source_texts: usize,
    source_artwork_mappings: usize,
    unbound_text_resource_ids: Vec<u16>,
    unbound_artwork_resource_ids: Vec<u16>,
    mapped_artwork_files: usize,
    owned_artwork_files: usize,
    mapped_artwork_missing: Vec<String>,
    owned_artwork_unmapped: Vec<String>,
    audiences: BTreeMap<&'static str, AudienceSummary>,
}

fn main() -> Result<()> {
    let mut args = env::args_os().skip(1);
    let gdata_path = PathBuf::from(args.next().context("missing GData path")?);
    let source_path = PathBuf::from(
        args.next()
            .context("missing Encyclopedia source JSON path")?,
    );
    let edata_path = PathBuf::from(args.next().context("missing owned EData directory path")?);
    ensure!(args.next().is_none(), "unexpected extra argument");

    let index = load_encyclopedia_catalog(&gdata_path)?;
    let source_bytes =
        fs::read(&source_path).with_context(|| format!("reading {}", source_path.display()))?;
    let mut manifest_name = source_path.as_os_str().to_owned();
    manifest_name.push(".manifest.json");
    let manifest_path = PathBuf::from(manifest_name);
    let manifest_bytes =
        fs::read(&manifest_path).with_context(|| format!("reading {}", manifest_path.display()))?;
    let (source, manifest) =
        parse_encyclopedia_source_with_manifest(&source_bytes, &manifest_bytes)?;
    let mapped_artwork = source.artwork.values().cloned().collect::<BTreeSet<_>>();
    let owned_artwork = read_owned_artwork(&edata_path)?;
    let mapped_artwork_missing = mapped_artwork
        .difference(&owned_artwork)
        .cloned()
        .collect::<Vec<_>>();
    let owned_artwork_unmapped = owned_artwork
        .difference(&mapped_artwork)
        .cloned()
        .collect::<Vec<_>>();
    let alliance =
        load_encyclopedia_topics(&gdata_path, &index, &source, EncyclopediaAudience::Alliance)?;
    let empire =
        load_encyclopedia_topics(&gdata_path, &index, &source, EncyclopediaAudience::Empire)?;

    let bound_texts = alliance
        .entries
        .iter()
        .map(|entry| entry.topic_text_resource_id)
        .collect::<HashSet<_>>();
    let bound_artwork = alliance
        .entries
        .iter()
        .chain(&empire.entries)
        .filter_map(|entry| entry.artwork_resource_id)
        .collect::<HashSet<_>>();
    let mut unbound_text_resource_ids = source
        .texts
        .keys()
        .filter(|resource_id| !bound_texts.contains(resource_id))
        .copied()
        .collect::<Vec<_>>();
    let mut unbound_artwork_resource_ids = source
        .artwork
        .keys()
        .filter(|resource_id| !bound_artwork.contains(resource_id))
        .copied()
        .collect::<Vec<_>>();
    unbound_text_resource_ids.sort_unstable();
    unbound_artwork_resource_ids.sort_unstable();

    let summary = AuditSummary {
        schema_version: 1,
        catalog_sha256: manifest.catalog_sha256,
        encytext_sha256: manifest.source_files.encytext_sha256,
        encybmap_sha256: manifest.source_files.encybmap_sha256,
        index_entries: index.entries.len(),
        source_texts: source.texts.len(),
        source_artwork_mappings: source.artwork.len(),
        unbound_text_resource_ids,
        unbound_artwork_resource_ids,
        mapped_artwork_files: mapped_artwork.len(),
        owned_artwork_files: owned_artwork.len(),
        mapped_artwork_missing,
        owned_artwork_unmapped,
        audiences: BTreeMap::from([
            ("alliance", summarize(&alliance)),
            ("empire", summarize(&empire)),
        ]),
    };
    validate_owned_english_profile(&summary)?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

fn validate_owned_english_profile(summary: &AuditSummary) -> Result<()> {
    ensure!(
        summary.catalog_sha256
            == "354643f3a5cb58c7bfa92e094d687ba3188c037eac14a961ea843f6b50566994",
        "unexpected owned Encyclopedia catalog identity"
    );
    ensure!(
        summary.encytext_sha256
            == "49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c",
        "unexpected owned ENCYTEXT identity"
    );
    ensure!(
        summary.encybmap_sha256
            == "fb545d19ae24b0277753494dbfaabf2dbdde660beab821287a32016c290e4560",
        "unexpected owned ENCYBMAP identity"
    );
    let expected_missing_objects =
        ENCYCLOPEDIA_SOURCE_EMPTY_OBJECT_IDS.map(|object_id| format!("{object_id:#010x}"));
    ensure!(summary.index_entries == 356, "expected 356 index entries");
    ensure!(summary.source_texts == 348, "expected 348 source texts");
    ensure!(
        summary.source_artwork_mappings == 191,
        "expected 191 source artwork mappings"
    );
    ensure!(
        summary.unbound_text_resource_ids == [7176, 7427],
        "unexpected unbound text identities"
    );
    ensure!(
        summary.unbound_artwork_resource_ids == [7188, 7427, 11284, 11523],
        "unexpected unbound artwork identities"
    );
    ensure!(
        summary.mapped_artwork_files == 186,
        "expected 186 distinct mapped artwork files"
    );
    ensure!(
        summary.owned_artwork_files == 187,
        "expected 187 owned artwork files"
    );
    ensure!(
        summary.mapped_artwork_missing.is_empty(),
        "mapped artwork is absent from the owned EData directory"
    );
    ensure!(
        summary.owned_artwork_unmapped == ["EDATA.192"],
        "unexpected unbound owned artwork"
    );
    for audience in ["alliance", "empire"] {
        let result = summary
            .audiences
            .get(audience)
            .with_context(|| format!("missing {audience} audit summary"))?;
        ensure!(
            result.complete == 346,
            "{audience}: expected 346 complete topics"
        );
        ensure!(
            result.missing_text == 10 && result.missing_artwork_mapping == 10,
            "{audience}: unexpected missing source counts"
        );
        ensure!(
            result.missing_system_picture == 0,
            "{audience}: unexpected missing system picture"
        );
        ensure!(
            result.distinct_bound_artwork == 172,
            "{audience}: expected 172 distinct bound artwork files"
        );
        let mut missing_ids = result
            .missing_entries
            .iter()
            .map(|entry| entry.object_id.clone())
            .collect::<Vec<_>>();
        missing_ids.sort_unstable();
        ensure!(
            missing_ids == expected_missing_objects,
            "{audience}: unexpected missing object identities"
        );
        ensure!(
            result
                .missing_entries
                .iter()
                .all(|entry| entry.missing == ["text", "artwork_mapping"]),
            "{audience}: missing objects must lack exactly text and artwork mapping"
        );
    }
    Ok(())
}

fn read_owned_artwork(edata_path: &std::path::Path) -> Result<BTreeSet<String>> {
    let mut filenames = BTreeSet::new();
    for entry in fs::read_dir(edata_path)
        .with_context(|| format!("reading owned artwork directory {}", edata_path.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let filename = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("EData contains a non-UTF-8 filename"))?;
        if filename.len() == 9
            && filename.starts_with("EDATA.")
            && filename.as_bytes()[6..].iter().all(u8::is_ascii_digit)
        {
            filenames.insert(filename);
        }
    }
    Ok(filenames)
}

fn summarize(catalog: &EncyclopediaTopicCatalog) -> AudienceSummary {
    let mut missing_text = 0;
    let mut missing_artwork_mapping = 0;
    let mut missing_system_picture = 0;
    let mut artwork = HashSet::new();
    let mut missing_entries = Vec::new();
    for entry in &catalog.entries {
        for missing in &entry.missing {
            match missing {
                EncyclopediaMissingPart::Text => missing_text += 1,
                EncyclopediaMissingPart::ArtworkMapping => missing_artwork_mapping += 1,
                EncyclopediaMissingPart::SystemPicture => missing_system_picture += 1,
            }
        }
        if let Some(filename) = &entry.artwork_filename {
            artwork.insert(filename);
        }
        if !entry.missing.is_empty() {
            missing_entries.push(MissingEntry {
                object_id: format!("0x{:08x}", entry.object_id),
                title: entry.title.clone(),
                missing: entry
                    .missing
                    .iter()
                    .map(|part| match part {
                        EncyclopediaMissingPart::Text => "text",
                        EncyclopediaMissingPart::ArtworkMapping => "artwork_mapping",
                        EncyclopediaMissingPart::SystemPicture => "system_picture",
                    })
                    .collect(),
            });
        }
    }
    AudienceSummary {
        complete: catalog.complete_count(),
        missing_text,
        missing_artwork_mapping,
        missing_system_picture,
        distinct_bound_artwork: artwork.len(),
        missing_entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected_summary() -> AuditSummary {
        let missing_entries = [
            ("0x44000004", "Adrift"),
            ("0x43000003", "Autorouting"),
            ("0x65000083", "Bounty"),
            ("0x71000043", "Dagobah"),
            ("0x41000001", "Move"),
            ("0x64000044", "Palace"),
            ("0x73000082", "Pickup"),
            ("0x42000002", "Return"),
            ("0x72000046", "Sabbatical"),
            ("0x72000045", "Vacation"),
        ]
        .into_iter()
        .map(|(object_id, title)| MissingEntry {
            object_id: object_id.into(),
            title: title.into(),
            missing: vec!["text", "artwork_mapping"],
        })
        .collect();
        let audience = AudienceSummary {
            complete: 346,
            missing_text: 10,
            missing_artwork_mapping: 10,
            missing_system_picture: 0,
            distinct_bound_artwork: 172,
            missing_entries,
        };
        AuditSummary {
            schema_version: 1,
            catalog_sha256: "354643f3a5cb58c7bfa92e094d687ba3188c037eac14a961ea843f6b50566994"
                .into(),
            encytext_sha256: "49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c"
                .into(),
            encybmap_sha256: "fb545d19ae24b0277753494dbfaabf2dbdde660beab821287a32016c290e4560"
                .into(),
            index_entries: 356,
            source_texts: 348,
            source_artwork_mappings: 191,
            unbound_text_resource_ids: vec![7176, 7427],
            unbound_artwork_resource_ids: vec![7188, 7427, 11284, 11523],
            mapped_artwork_files: 186,
            owned_artwork_files: 187,
            mapped_artwork_missing: Vec::new(),
            owned_artwork_unmapped: vec!["EDATA.192".into()],
            audiences: BTreeMap::from([("alliance", audience.clone()), ("empire", audience)]),
        }
    }

    #[test]
    fn owned_english_gate_accepts_only_the_expected_inventory() {
        validate_owned_english_profile(&expected_summary()).unwrap();

        let mut wrong_count = expected_summary();
        wrong_count.source_texts -= 1;
        assert!(validate_owned_english_profile(&wrong_count).is_err());

        let mut wrong_identity = expected_summary();
        wrong_identity
            .audiences
            .get_mut("empire")
            .unwrap()
            .missing_entries[0]
            .object_id = "0x44000005".into();
        assert!(validate_owned_english_profile(&wrong_identity).is_err());
    }
}
