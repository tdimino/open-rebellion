# P66A Encyclopedia topic-source checkpoint

P66A establishes the fail-closed source-data contract needed by the authentic
Galactic Encyclopedia topic view. It does not enable the cockpit route or
claim a visible `OBJ-01` state.

## Recovered identity rules

The implementation follows the original executable rather than matching names
or list positions:

- `FUN_0045d400` resolves a selected object's topic identity as the low 12 bits
  of its source name resource plus `0x1000`.
- `FUN_0045fa60` uses that identity for `ENCYTEXT.DLL` type-10 prose. Mission
  artwork uses the same low bits with faction offset `0x1000` or `0x2000`.
- Systems obtain artwork through `FUN_00509610` and the exact
  `FUN_0045f660` picture switch: pictures 1 through 23 map to `0x2b5c` through
  `0x2b72`, while 24, 25, and 26 map to `0x2b75`, `0x2b73`, and `0x2b74`.
- `FUN_0045fd20` disables previous or next when the selected source object has
  no corresponding neighbor. The exact native neighbor order still requires a
  separate trace and is not inferred from the P65 alphabetical index.

Later checkpoint note (2026-10-06): W3 uses a subsequent trace of
`FUN_0060a790`, `FUN_0060a890`, `FUN_00626ad0`, `FUN_005f59f0`,
`FUN_00442130`, `FUN_004ad730`, `FUN_004ad750`, and `FUN_0045da70` to establish
case-folded list ordering, skip-disabled neighbors, and null endpoint behavior.
The [W3 evidence](2026-10-06-encyclopedia-presenter.md) records that result and
the bounded equal-fold tie distinction; it does not retroactively expand this
P66A extraction checkpoint.

## Extraction and validation

`stage-ui-assets --encyclopedia-only` is a dependency-free local extractor for
the owned English installation. It:

- reads numeric type-10 `ENCYTEXT` and type-6 `ENCYBMAP` PE resources;
- requires language 1033 and the observed PE code-page field 0;
- decodes prose as strict Windows-1252, including the 29 records containing
  byte `0x92`, and rejects undefined bytes or embedded NULs;
- accepts only `EDATA.NNN` mapping values;
- emits ignored JSON plus catalog and source-DLL SHA-256 metadata; and
- verifies staged output without reopening the original DLLs.

The owned source profile produces:

| Source | Result |
|---|---:|
| `ENCYTEXT.DLL` SHA-256 | `49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c` |
| `ENCYBMAP.DLL` SHA-256 | `fb545d19ae24b0277753494dbfaabf2dbdde660beab821287a32016c290e4560` |
| Extracted topic texts | 348 |
| Logical artwork mappings | 191 |
| Distinct mapped EDATA filenames | 186 |
| Mapped filenames absent from owned EData | 0 |
| Owned files absent from ENCYBMAP | 1: `EDATA.192` |

No original prose or EDATA bytes are committed. The local catalog remains
under ignored `data/base/` or another operator-selected output path.

## Object coverage

`rebellion-data::encyclopedia_topics` joins the staged source to P65's 356
compound object identities. The join is deterministic for both factions:

| Result | Alliance | Empire |
|---|---:|---:|
| Complete text and artwork mapping | 346 | 346 |
| Missing text | 10 | 10 |
| Missing artwork mapping | 10 | 10 |
| Invalid or missing system-picture mapping | 0 | 0 |
| Distinct bound artwork files | 172 | 172 |

The same ten internal/special mission records lack both source resources:
Adrift, Autorouting, Bounty, Dagobah, Move, Palace, Pickup, Return, Sabbatical,
and Vacation. P66A exposes those missing parts explicitly. It does not borrow
another topic, generate lore, or guess an image. Two source prose records
(`7176`, obsolete-mission diagnostic; `7427`, generic fleet text) and four
artwork lookup records (`7188`, `7427`, `11284`, `11523`) are not bound to the
supported P65 objects and remain reported rather than silently reassigned.
The owned-profile audit exits nonzero if any expected count, missing identity,
unbound identity, system-picture result, or EData filename relation changes.

## Verification

The machine-readable inventory is
[`p66a-encyclopedia-topic-bindings/summary.json`](p66a-encyclopedia-topic-bindings/summary.json).

| Gate | Result |
|---|---|
| Go extractor tests | pass |
| Go vet | pass |
| Rust topic-binding tests | 5 passed |
| Rust owned-profile audit-gate test | 1 passed |
| Scoped Clippy (`rebellion-data`, all targets, no dependencies) | pass with warnings denied |
| Workspace tests | pass |
| Production WASM build | pass; existing missing-local-media warnings only |
| Owned extraction and offline verify | pass; 348 texts and 191 mappings |
| Two-faction source audit | pass; 346 complete and 10 explicit missing per faction |
| EDATA filename cross-check | pass; 186 of 186 mapped filenames present |
| Independent source/code review | pass after all findings were corrected; final review reported no P0/P1 and its P2 documentation plus P3 test-coverage cleanup were also corrected |

The exact reproducible command set, run from the repository root unless noted,
is:

```bash
cd tools/stage-ui-assets
env GOCACHE=/tmp/open-rebellion-go-cache go test ./...
env GOCACHE=/tmp/open-rebellion-go-cache go vet ./...
env GOCACHE=/tmp/open-rebellion-go-cache go run . --encyclopedia-only --source "$ORIGINAL" --encyclopedia-output /tmp/open-rebellion-encyclopedia-source/source.json --force
env GOCACHE=/tmp/open-rebellion-go-cache go run . --encyclopedia-only --verify --encyclopedia-output /tmp/open-rebellion-encyclopedia-source/source.json
cd ../..
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin cargo test -p rebellion-data encyclopedia_topics -- --nocapture
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin cargo test -p rebellion-data --bin encyclopedia-source-audit -- --nocapture
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin cargo run -q -p rebellion-data --bin encyclopedia-source-audit -- data/base /tmp/open-rebellion-encyclopedia-source/source.json "$ORIGINAL/EData"
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin cargo clippy -p rebellion-data --all-targets --no-deps -- -D warnings
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin cargo test --workspace
./scripts/build-wasm.sh
node scripts/validate-interface-parity-ledgers.mjs --check
git diff --check
```

Here `$ORIGINAL` is the operator's owned English installation root. The two
stager invocations each report 348 texts and 191 mappings. The audit reports
356 index entries; 346 complete topics, ten missing texts, ten missing artwork
mappings, zero missing system pictures, and 172 distinct bound artwork files
for each faction; exactly 186 mapped EData names are present in the 187-file
owned directory; and only `EDATA.192` is unbound. Focused Rust tests report five
topic-binding tests and one fail-closed audit-gate test passing. Scoped Clippy,
the workspace suite, the production WASM build, ledger validation, and the diff
check pass.

## Acceptance boundary

P66A proves local extraction, decoding, resource identity, system-picture
mapping, factional mission-art selection, and explicit missing-source behavior.
It does not yet transport this catalog in
the browser pack, compose the topic window, render text or EDATA, enable
previous/next controls, connect contextual or cockpit routes, prove other
languages, or compare a topic against an original A0 capture. All thirteen
`OBJ-01` cells therefore remain pending.
