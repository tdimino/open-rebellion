# Encyclopedia immutable content-session checkpoint

Date: 2026-10-06
Plan: `docs/plans/2026-10-02-feat-will-forster-encyclopedia-handoff-adaptation.md`, W2
Bead: `orlocal-818.59`
Pre-W2 branch checkpoint: `0d4a52734fe88de3e9d453e947afa6bf065f0777`

## Scope

This checkpoint adds one platform-neutral, read-only Encyclopedia session over
the current P66A catalog and topic-binding authority. A caller supplies the
source-derived index, source JSON and manifest bytes, system-picture identities,
and exact `EDATA.NNN` bytes. The store parses, validates, binds both factions,
and hashes the entire candidate before replacing its active `Arc`.

The session owns the validated index and source catalog, both typed faction
topic catalogs, exact artwork bytes and SHA-256/length/dimension/bit-depth
metadata, one combined logical fingerprint, and one Encyclopedia-only
texture-generation identity. Catalog categories, entry identity/order, and
bindings are checked before publication. Only the ten P66A source-empty mission
objects may lack exactly prose and artwork; a missing system picture always
fails. Artwork must match the established original 400-by-200, 8-bit indexed,
uncompressed BMP class, fit 32 MiB per resource and 128 MiB in aggregate, and
contain its complete declared pixel range. It performs no filesystem, network,
renderer, GPU, `GameWorld`, save, replay, RNG, multiplayer, or simulation
mutation.

This is the approved W2 inactive rollback point. It does not add a second
schema or transport reader, enable command `0x131`, render a topic, or accept an
`OBJ-01` cell. W5 remains responsible for canonical native/runtime-pack/loose
readers and production publication.

## Atomicity and lifecycle proof

The focused tests establish these independent failures and outcomes:

- a valid candidate publishes one complete session with both faction bindings,
  four exact synthetic artwork resources, a 64-character logical fingerprint,
  and texture generation 1;
- a catalog/manifest digest mismatch returns before publication and preserves
  the exact prior `Arc`, bytes, and generation;
- repeating identical content reuses the active `Arc` and generation;
- replacing artwork bytes publishes generation 2 while a retained old reader
  continues to observe its original immutable bytes;
- teardown drops only the store's active session, increments the Encyclopedia
  generation once, and is idempotent while empty;
- missing, unexpected, empty, non-BMP, truncated, non-indexed, compressed,
  individually oversized, and aggregate-oversized artwork fail preparation;
- duplicate or malformed catalog entries, unapproved incomplete joins, and
  missing system pictures fail without disturbing the prior session;
- generation exhaustion preserves the prior session on both replacement and
  teardown;
- two independent platform-neutral installations of identical bytes yield the
  fixed logical fingerprint
  `dc504009a224e8a985b13710b2c0afb34e01bc50cd25db7278543019e7a4bb64`,
  matching topic counts, and a byte-identical serialized default `GameWorld`.

The tests use only the committed synthetic P66A conformance fixture and
generated synthetic 400-by-200 indexed BMPs. No original prose, image, pack, or
generated proprietary content is tracked.

## Verification

TDD RED:

```text
cargo test -p rebellion-data encyclopedia_session --lib
error[E0425]/error[E0599]: bounded image metadata and accessors did not exist

cargo test -p rebellion-data \
  encyclopedia_session::tests::nonindexed_and_compressed_bitmaps_fail_closed \
  --lib -- --exact
FAILED: a non-indexed candidate was installed
```

GREEN and regression gates:

```text
cargo test -p rebellion-data encyclopedia_session --lib
11 passed; 0 failed

cargo test -p rebellion-data --lib
127 passed; 0 failed; 13 ignored (owned original-data tests)

LIBRARY_PATH="$PWD/.artifacts/native-link" cargo test --workspace --quiet
1,401 passed; 0 failed; 39 ignored

cargo clippy -p rebellion-data --lib --no-deps -- -D warnings
PASS

cargo check -p rebellion-data --bin encyclopedia-source-audit
PASS

cargo mutants -p rebellion-data \
  --file crates/rebellion-data/src/encyclopedia_session.rs \
  -- --lib encyclopedia_session
54 mutants tested: 41 caught, 13 unviable, 0 missed

cargo check -p rebellion-app --target wasm32-unknown-unknown --release
PASS (existing target-only unused-code warnings retained)

rustfmt --edition 2021 --check \
  crates/rebellion-data/src/encyclopedia_session.rs \
  crates/rebellion-data/src/encyclopedia_topics.rs \
  crates/rebellion-data/src/bin/encyclopedia-source-audit.rs
PASS

git diff --check
PASS
```

The workspace test initially reached native linking and stopped because this
host has only the versioned `libasound.so.2`, not the unversioned development
link name required by `-lasound`. The successful rerun used the repository's
established ignored `.artifacts/native-link/libasound.so` symlink to that host
library; no source or tracked dependency changed. Existing manifest-lint and
future-incompatible macro warnings remain outside this checkpoint.

The repository-wide `cargo fmt --all -- --check` and dependency-inclusive
Clippy gate retain unrelated pre-existing failures outside this change. A
standalone `rebellion-data` WASM check also reaches the existing target feature
unification boundary in `getrandom`; the canonical `rebellion-app` WASM check
above supplies the workspace feature union and passes. That compile proves the
platform-neutral session remains WASM-compatible; it is not a native/browser
reader-equivalence run. W5 owns identical-pack execution through the canonical
native, runtime-pack, and development-loose readers.

## Provenance and remaining work

Historical PR #16 commits `49732c2c`, `fe5d0eac`, and `18f2dedf` supplied the
immutable-candidate and last-known-good design pattern only. This checkpoint is
implemented against the current P66A types and fixture rather than copying the
superseded catalog schema or bundle implementation.

W3 now adds the [pure presenter and source-style navigation](2026-10-06-encyclopedia-presenter.md)
over this session. W4 must own decoded textures and the authentic topic
surface. W5 must complete canonical publication/readers and the late W1/W2
reader-parity lines. W6/W7 still own original-first HD selection and native
overlays. Live native/browser and A0 acceptance remain open.
