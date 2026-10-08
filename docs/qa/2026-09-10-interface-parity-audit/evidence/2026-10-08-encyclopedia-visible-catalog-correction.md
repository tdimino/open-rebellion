# Encyclopedia visible-catalog correction

This checkpoint corrects the P65/P66A interpretation of `MISSNSD.DAT`.
Extraction was complete: the owned English source still contains 348
`ENCYTEXT` records, 191 `ENCYBMAP` mappings, and 187 owned EDATA images. The
error was catalog construction, which treated all 25 mission definitions as
visible Encyclopedia records.

`FUN_00422620` builds the shared Encyclopedia object collection before the
category view is populated. It admits mission families `0x50..0x80` only and,
for those families, resolves the mission record and requires its in-memory
`+0x5c` hidden flag to be zero. The ten rejected gameplay records are therefore
not blank Encyclopedia topics:

| Object | Source name | Exclusion |
|---:|---|---|
| `0x41000001` | Move | family below `0x50` |
| `0x42000002` | Return | family below `0x50` |
| `0x43000003` | Autorouting | family below `0x50` |
| `0x44000004` | Adrift | family below `0x50` |
| `0x64000044` | Palace | hidden flag set |
| `0x65000083` | Bounty | hidden flag set |
| `0x71000043` | Dagobah | hidden flag set |
| `0x72000045` | Vacation | hidden flag set |
| `0x72000046` | Sabbatical | hidden flag set |
| `0x73000082` | Pickup | hidden flag set |

The corrected production catalog contains 346 visible objects: 200 systems,
38 ships, 14 facilities, 15 missions, 10 troops, and 69 personnel. All 346
objects bind complete text and artwork for both factions. Session validation
now rejects every incomplete visible topic, including mod overlays that would
clear the text or image of an existing topic.

The owned audit reports logical fingerprint
`20c342868cee50e80ef3b94b9f81a898c48f4b593ddd0d83ab69b67d755ae9ea`,
zero missing visible bindings, 172 distinct bound artwork files per faction,
and the ten excluded gameplay identities in a separate provenance field. The
raw extraction inventory and its DLL hashes are unchanged.

## Focused verification

```text
cargo test -p rebellion-data encyclopedia_catalog::tests::owned_catalog_excludes_gameplay_only_and_hidden_mission_records -- --ignored --exact --nocapture
RED: catalog length was 356 rather than 346
GREEN: passed after applying the original collection predicate

cargo run -q -p rebellion-data --bin encyclopedia-source-audit -- \
  data/base data/base/encyclopedia/source.json <owned-install>/EData
PASS: 346 visible entries; 346 complete per faction; zero missing bindings

REBELLION_EDATA_DIR=<owned-install>/EData \
  bash scripts/build-interface-test-wasm.sh
PASS: production pack and isolated fixture build; 38 interface scenarios

REBELLION_EDATA_DIR=<owned-install>/EData \
  node tools/interface-parity/encyclopedia-canonical-surface.mjs
PASS: 12 packed-browser cases; both factions; 346 resolved topics; zero runtime
errors; exact owned artwork pixels; bounded navigation and long-text scrolling
```

The browser run used logical fingerprint
`20c342868cee50e80ef3b94b9f81a898c48f4b593ddd0d83ab69b67d755ae9ea`,
WASM SHA-256
`b7e5a27606ecbf7f70df2f2179c18ae0cbbba62f30284bd0e1e0d663dca468ac`,
runtime-pack SHA-256
`e3fd72ad1efc71e7d01a4068b67e826756c3145b26bc7132fd54ce955e0d99c1`,
and summary SHA-256
`973b0ea42a658de3340a522978687113c5ff2fdd27a692e72df68a0b89a91f09`.
The ignored local summary is
`.artifacts/interface-parity/encyclopedia-canonical-surface-2026-10-08T14-23-00-359Z-919033/summary.json`.

The historical P65 and P66A reports remain useful for extraction identity and
the original staged-resource inventory, but their 356-visible-object and
source-unavailable conclusions are superseded by this correction.
