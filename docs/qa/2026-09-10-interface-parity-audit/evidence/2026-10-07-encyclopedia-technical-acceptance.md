# E34 Encyclopedia technical and isolation acceptance

Date: 2026-10-07

Bead: `orlocal-818.34`

Candidate branch: `test/encyclopedia-conformance`

Upstream integration baseline: merge `5a9f747c36601f3ecec7d365e28f14a75ddbb7ab`

## Result

The assembled W1-W7, E30, and E32 Encyclopedia implementation passes the
adapted plan's final technical, publication, replay, save, and isolation gates.
This checkpoint does not accept strict `OBJ-01`: lossless original A0
comparison and the strict faction/viewport matrix remain E35.

The final package installs the same 346-topic visible logical catalog on native,
packed-browser, and development-loose readers. Both production factions pass
cockpit entry, topic navigation, close/return, real object-popup contextual
entry, adjacent navigation, exact contextual return, four-request startup, and
old-pack fail-closed compatibility with no page, request, or console errors.

The current source-backed sector-placement work moved the Alliance fixture's
first right-half sector into the secondary column. The initial final-package
run correctly failed when the old object-menu click missed the shifted Fleet
window. `encyclopedia-publication.mjs` now follows the recovered placement and
its right-edge menu clamp. An Alliance-only red/green run and the required
two-faction packaged run both pass. No production route was weakened.

## Adapted acceptance boundary

E34's older decomposition names
`validate-encyclopedia-fixtures.mjs` and `encyclopedia-fetch.test.mjs` from the
divergent PR #16 lineage. Those files are not part of the approved handoff
adaptation, which explicitly forbids wholesale import of that chain. Their
current-plan coverage is supplied by:

- Go staging tests and both Python publication suites;
- the shared Rust catalog/session/content-reader corpus, including identical
  native, packed, and loose accept/reject decisions;
- `node --test tools/interface-parity/*.test.mjs` and the Ajv-backed interface
  catalog/matrix checks;
- `encyclopedia-publication.mjs` for exact namespace bytes, four-request
  startup, no navigation fetch, both consumers, and old-pack behavior.

The old E48 parallel-fetch harness is likewise not imported. W5A's accepted
development-loose immutable generation, read-back verifier, bounded reader,
and exact three-reader parity are the approved replacement. ORPK remains the
only packaged authority and a present ORPK cannot be overridden by loose data.

## Final artifact identity

- WASM SHA-256:
  `69a170811b911b0a15ba282d73a23f4470c4c3183d94a908e96bd7d3b0eef717`
- runtime ORPK SHA-256:
  `e4c0157a8701aa1631f642db04c71b5e01d7280cf3e0cb691b428c502a1afa4e`
- final local development ZIP SHA-256:
  `910668f06a1eef32e5df458bce6fe42557d6977e3d827d2f187fb285e44c30e1`
- canonical catalog SHA-256:
  `354643f3a5cb58c7bfa92e094d687ba3188c037eac14a961ea843f6b50566994`
- canonical manifest SHA-256:
  `238b8879565ab2603594a8705f3538c652da3541b147457be09ec1f3ef25e8b1`
- logical catalog fingerprint:
  `5c4b64bfd739508e63a87118fd7cac8503ea2d34999144838074a52736b00fe3`
- ORPK size and contents: 50,846,574 bytes; 52 game files, two Encyclopedia
  metadata entries, 186 exact referenced Encyclopedia assets, 2,326 UI
  bitmaps, and 3,988 advisor frames.

Two consecutive final builds produced identical WASM and ORPK hashes. The ZIP
contains only `index.html`, `gl.js`, `open-rebellion.wasm`,
`data/runtime.orpk`, and `SHA256SUMS`; its archive test and every shipped hash
pass.

Retained ignored local evidence:

- `.artifacts/interface-parity/encyclopedia-publication-2026-10-07T14-08-47-117Z-3676603/summary.json`
  (SHA-256 `e40f14c15bbc0601295e1c309a26695ccc658c545b038ee42aefae8796133737`);
- `.artifacts/interface-parity/e34-game-options-final/results.json`
  (SHA-256 `3a3b254b62c02af796697266a8970032cbc042d2e5d4bda695023a15d46d3cfe`);
- `dist/open-rebellion-web-dev.zip`.

## Verification

```text
go test ./tools/stage-ui-assets
PASS

python3 -m unittest discover -s scripts -p test_build_runtime_pack.py
10 passed

python3 -m unittest discover -s scripts -p test_encyclopedia_staging.py
2 passed

cargo test -p rebellion-data encyclopedia -- --nocapture
44 passed

cargo test -p rebellion-app encyclopedia -- --nocapture
29 passed; 3 intentional owned-source tests ignored

cargo test --workspace --quiet
1,629 passed; 0 failed; 39 intentionally ignored

cargo check --workspace
PASS; established lint-inheritance and target-specific warnings only

node --test tools/interface-parity/*.test.mjs
26 passed

npm --prefix tools/interface-parity run check
PASS; CMD-02 38-scenario Ajv catalog, tactical structural matrix,
proprietary-A0 exclusion, and production-fixture exclusion

node scripts/validate-interface-parity-ledgers.mjs --check
PASS; 44 surface families, 633 baseline cells, 27 RE packages

REBELLION_EDATA_DIR=<owned ignored EData> bash scripts/build-wasm.sh
REBELLION_EDATA_DIR=<owned ignored EData> bash scripts/package-web.sh dev
PASS; strict source requirement, atomic ORPK plus loose publication, read-back,
two-build determinism, five-file ZIP, and shipped checksums

REBELLION_EDATA_DIR=<owned ignored EData> \
  OPEN_REBELLION_ENCYCLOPEDIA_SITE=dist/open-rebellion-web-dev \
  node tools/interface-parity/encyclopedia-publication.mjs
PASS; both factions, cockpit/contextual journeys, exact returns, old pack,
four requests, zero page/request/console errors

REBELLION_ENCYCLOPEDIA_TEST_SOURCE=<absolute ignored P66A source> \
REBELLION_EDATA_DIR=<owned ignored EData> \
REBELLION_ENCYCLOPEDIA_TEST_PACK=<absolute final ORPK> \
REBELLION_ENCYCLOPEDIA_TEST_MIRROR=<absolute loose mirror> \
  cargo test -p rebellion-app \
  encyclopedia_content::tests::owned_native_packed_and_loose_publications_match_exactly \
  --bin open-rebellion -- --ignored --nocapture
1 passed; exact metadata, artwork bytes, counts, and fingerprint match

AGENT_BROWSER_ARGS=--no-sandbox \
AGENT_BROWSER_EXECUTABLE_PATH=<installed Chromium> \
  python3 scripts/check-replay-equivalence.py --skip-build --json
PASS; 9 checkpoints; v1:befc368b68d6b313 -> v1:61e86327d2c39ea6;
four replay requests, four normal-startup requests, zero browser errors,
invalid query and missing pack fail closed

CHROMIUM_PATH=<installed Chromium> \
OPTIONS_TEST_URL=<final local package> \
  node tools/interface-parity/game-options-smoke.mjs
PASS; both factions, matching save/load/restored fingerprints, slot 10,
delete/overwrite cancellation, corruption handling, restart, zero page errors

rustfmt --edition 2021 --check --config skip_children=true <all branch-changed Rust files>
git diff --check
PASS

cargo clippy -p rebellion-data --lib --no-deps -- -D warnings
cargo clippy -p rebellion-render --lib --no-deps -- -D warnings <named existing allowances>
cargo clippy -p rebellion-app --bin open-rebellion --no-deps -- -D warnings <named existing allowances>
PASS
```

The final package's same-run save/load/restored fingerprints are
`v1:1834f05ce2c7dff4` for Alliance and `v1:03c0d89649ce6ad0` for Empire. The
smoke deliberately creates fresh campaigns, so these are artifact evidence,
not new replay goldens; each saved fingerprint exactly matches both restored
loads in its own run.

`cargo fmt --all --check` remains red only in current-upstream files outside
the Encyclopedia contribution (`movement.rs`, `tick.rs`, `world/mod.rs`,
`integrator.rs`, `save.rs`, `fleet_finder.rs`, `fleet_registry.rs`, and
`main_menu.rs`). The branch-changed Rust set passes the scoped formatter; this
checkpoint does not rewrite unrelated upstream work.

Scoped mutation evidence was reused only where byte identity is exact, as E34
permits. The final `encyclopedia_surface.rs` blob is still
`eaabec0aff05c187df44118c824d060f76b62a07` (6 caught, 2 unviable, 0 missed),
`encyclopedia_overlay.rs` is still
`371769a7108f3c60e2de525786ad7c12b7d472f7` (41 caught, 2 unviable, 0
missed across its two scoped runs), and `catalog_object_id` retains SHA-256
`2bde209f78c0f42ce30d86d70634fec29363d2d10f575304df0e637ad7d9e112`
(12/12 resolver mutants caught). Content reader, session, presenter, HD, and
overlay source blobs are also unchanged from their zero-missed task evidence.

## Isolation review

- No `data/base` source, EData, generated ORPK/loose mirror, local artifact,
  screenshot, `.beads`, credential, or environment file appears in the branch
  diff. The owned source, generated packs, loose generations, `dist/`, and
  `.artifacts/` paths are ignored.
- The branch adds no Encyclopedia field to `GameWorld`, saves, replay fixtures,
  replay commands, RNG state, or deterministic simulation. Replay goldens are
  unchanged and pass on native and WASM.
- Production builds contain no interface-fixture token or bridge. There is no
  stale Encyclopedia Inspector route.
- Ordinary Go, Python, Rust, Node, check, and lint suites pass without an owned
  installation. Only the explicitly ignored exact-source test and production
  package/browser gates consume the identified P66A owned profile.
- Release packaging still uses ORPK only. Development-loose files are ignored,
  generation-addressed, independently verified, and never shipped.

## Remaining gate

E35 must acquire and compare lossless original A0 evidence for the complete
`OBJ-01` faction/viewport matrix. This technical checkpoint supplies the final
candidate, hashes, package, and deterministic implementation evidence but does
not substitute A1 or source evidence for that visual acceptance.
