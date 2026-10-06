---
title: "Encyclopedia canonical publication and reader parity"
description: "W5A evidence for recoverable ORPK publication and native, packed-browser, and development-loose reader convergence"
category: "qa"
created: 2026-10-06
updated: 2026-10-06
tags: [encyclopedia, orpk, wasm, publication, parity]
---

# Encyclopedia canonical publication and reader parity

## Result

W5A passes the reader/publication half of the approved W5 work. The existing
ORPK v3 authority now publishes the canonical P66A catalog, its integrity
sidecar, and exactly the referenced original artwork under one stable
`encyclopedia/` namespace. Native, packed-browser, and development-loose
readers all hand the same bytes to the W2 session builder.

This checkpoint does not enable the Encyclopedia command. Command `0x131`
remains fail-closed until E30 proves packaged production journeys and E32
activates the production route. No strict `OBJ-01` cell or A0 comparison is
claimed here.

## Canonical publication contract

The only accepted keys are:

- `encyclopedia/catalog.json`;
- `encyclopedia/manifest.json`;
- `encyclopedia/assets/EDATA.NNN` for the unique artwork names declared by the
  validated catalog.

The pack builder rejects partial publication, duplicate or aliased identities,
unknown fields, unsafe or non-canonical resource keys, invalid UTF-8/JSON,
unsupported source profiles, count or digest mismatch, missing/unsafe artwork,
non-400-by-200 indexed BMPs, and source files that change after validation.
Entries retain deterministic `(kind, key)` order. The complete temporary ORPK
is read back byte-for-byte before atomic replacement, so a failed candidate
does not disturb the previous pack.

The development-only loose form publishes immutable content-hash generation
directories and atomically replaces a bounded `current.json` pointer only
after full read-back verification. Unsafe mirror/generation symlinks, partial
generations, and malformed pointers fail closed. Production always chooses
ORPK when it is present and never consults the loose pointer after a corrupt
pack; therefore the development mirror cannot change production resolution
order. ORPK and loose publication are independently atomic because they are
independent production and development resolution paths.

All readers apply 32 MiB per-artwork and 128 MiB aggregate transfer limits
before W2 installation. A wholly absent namespace remains compatible with old
packs; any present but partial namespace is an error. The installed session is
kept alive for the application lifetime, while the old basename artwork cache
is populated only in `interface-test-fixtures` builds.

## Cross-target identity

Ignored owned inputs were used only to generate local, ignored artifacts. No
catalog prose, original bitmap, ORPK, loose generation, WASM binary, or browser
capture is tracked.

| Item | Result |
|---|---|
| P66A catalog SHA-256 | `354643f3a5cb58c7bfa92e094d687ba3188c037eac14a961ea843f6b50566994` |
| P66A manifest SHA-256 | `238b8879565ab2603594a8705f3538c652da3541b147457be09ec1f3ef25e8b1` |
| Referenced unique artwork | 186 exact files |
| Alliance topics | 356 total, 346 complete, 10 explicit source-empty |
| Empire topics | 356 total, 346 complete, 10 explicit source-empty |
| Native/packed/loose logical fingerprint | `5c4b64bfd739508e63a87118fd7cac8503ea2d34999144838074a52736b00fe3` |
| Final ORPK SHA-256 | `e4c0157a8701aa1631f642db04c71b5e01d7280cf3e0cb691b428c502a1afa4e` |
| Final ORPK size | 50,846,574 bytes |
| Final production WASM SHA-256 | `a12e9a9dae22340717fb9889f7b1e2230b94b485e1ea64207516f7d059d866de` |

Two independent final pack builds and `web/data/runtime.orpk` were byte-equal
and shared the ORPK hash above. The pack contained 52 ordinary game files, two
Encyclopedia metadata records, 186 Encyclopedia artwork records, 2,326 UI
bitmaps, and 3,988 advisor frames.

## Browser and package evidence

The production browser run used Chrome `153.0.8010.12` and the release-mode
WASM above. It loaded exactly four successful requests:

1. `/`
2. `/gl.js`
3. `/open-rebellion.wasm`
4. `/data/runtime.orpk`

The harness exact-compared the packed catalog, manifest, and every declared
artwork byte with the ignored owned inputs. The application logged the expected
fingerprint and `356/356` faction counts. There were zero page, request, or
console errors, and the interface-fixture bridge was absent.

Local ignored evidence:

- `.artifacts/interface-parity/encyclopedia-publication-2026-10-06T22-05-30-662Z-2589997/summary.json`
- summary SHA-256 `fb9a3b9f5abbe080d6fd60d6361c4f38e5288b88e6de13c802d6f3e883e0cb9e`
- `dist/open-rebellion-web-dev.zip`, SHA-256
  `6b5f0b52b8ee36b79c15de5d60885065ef186a506614969f001d57b32eebb792`

The archive contains only `index.html`, `gl.js`, `open-rebellion.wasm`,
`data/runtime.orpk`, and `SHA256SUMS`. `python3 -m zipfile -t` and every shipped
SHA-256 entry passed. Production-fixture exclusion and the 38-scenario CMD-02
catalog validator also passed. The package script uses the system `zip` when
available and the already-required Python standard library otherwise.

The W4 regression suite was rebuilt with fixture compilation and passed all ten
two-faction surface cases at 640-by-480 and 800-by-600 with zero runtime errors.
That run remains synthetic A1 evidence rather than production or A0 acceptance.

## Verification

```text
python3 -m unittest scripts.test_build_runtime_pack
9 passed

cargo test -p rebellion-app encyclopedia_content --bin open-rebellion
8 passed; 1 owned-data test ignored

cargo test -p rebellion-app \
  encyclopedia_content::tests::owned_native_packed_and_loose_publications_match_exactly \
  --bin open-rebellion -- --ignored --nocapture
1 passed; exact native/packed/loose bytes, counts, and fingerprint matched

cargo test --workspace --quiet
1,428 passed; 0 failed; 37 intentionally ignored

cargo clippy -p rebellion-data --lib --no-deps -- -D warnings
PASS

cargo clippy -p rebellion-app --bin open-rebellion --no-deps \
  --features interface-test-fixtures --target wasm32-unknown-unknown -- \
  -D warnings -A dead-code -A unused-imports -A unused-variables \
  -A clippy::too-many-arguments -A clippy::type-complexity \
  -A clippy::needless-borrow
PASS; allowances name pre-existing target-specific findings

cargo mutants -p rebellion-app \
  --file crates/rebellion-app/src/encyclopedia_content.rs \
  --output /data/tmp/or-w5a-content-mutants-final -- \
  --bin open-rebellion encyclopedia_content
50 mutants: 45 caught, 5 unviable, 0 missed

REBELLION_EDATA_DIR=<ignored-owned-EData> \
  bash scripts/build-interface-test-wasm.sh
PASS; production build, ORPK read-back, 38-scenario catalog, fixture exclusion

node tools/interface-parity/encyclopedia-surface.mjs
PASS; 10 cases, both factions, zero runtime errors

REBELLION_EDATA_DIR=<ignored-owned-EData> bash scripts/package-web.sh
PASS; release archive created and verified

REBELLION_EDATA_DIR=<ignored-owned-EData> \
  node tools/interface-parity/encyclopedia-publication.mjs
PASS; exact namespace bytes, fingerprint, counts, four requests, clean runtime

python3 -m py_compile scripts/build-runtime-pack.py scripts/test_build_runtime_pack.py
bash -n scripts/build-wasm.sh scripts/package-web.sh
rustfmt --edition 2021 --check \
  crates/rebellion-app/src/encyclopedia_content.rs \
  crates/rebellion-data/src/encyclopedia_topics.rs
git diff --check
PASS
```

Native tests and mutation runs used the ignored local ALSA linker-name shim.
The workspace warnings are pre-existing missing lint inheritance and tactical
macro-semicolon warnings; no new scoped Clippy finding remains.

## Remaining work

E30 must inspect the real packaged Encyclopedia journeys for both factions,
including cockpit and contextual entry, navigation, close, and exact return
state. A0 visual comparison and strict `OBJ-01` acceptance remain open. E32 may
activate command `0x131` only after those gates pass.
