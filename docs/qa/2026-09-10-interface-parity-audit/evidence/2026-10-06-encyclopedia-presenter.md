# Encyclopedia pure-presenter checkpoint

Date: 2026-10-06
Plan: `docs/plans/2026-10-02-feat-will-forster-encyclopedia-handoff-adaptation.md`, W3
Bead: `orlocal-818.60`
Pre-W3 branch checkpoint: `2e67c31311ed550ded26152f1d63e1d1e1e61fa9`

## Scope

This checkpoint adds one graphics-free presenter over the W2 immutable
Encyclopedia session. It performs no filesystem or network I/O, owns no decoded
texture or renderer handle, and does not inspect or mutate `GameWorld`. Its
output contains the selected audience, category and rows; resolved prose and
validated artwork metadata or an explicit source-unavailable state; bounded
previous and next targets; and the immutable route to which close must return.

Cockpit, index, object and the five recovered contextual callers enter through
`EncyclopediaPresenter::present`. Follow-up index/topic transitions are created
from the prior presentation, retaining the original cockpit, index or contextual
return token instead of reconstructing an origin from the current row. A
contextual miss opens the full index but keeps its typed caller. An index may
retain a selected row for stable navigation without materializing topic prose.
Unknown internal categories or objects and objects outside a selected family
fail closed.

This remains an inactive W3 rollback point. It does not connect command `0x131`,
draw the authentic topic surface, decode a texture, publish a runtime pack, or
accept an `OBJ-01` cell.

## Ordering and navigation provenance

The presenter does not sort. It projects the sequence already validated and
owned by P65/W2, and category filtering retains that sequence. P65 currently
defines case-insensitive name ordering with compound object identity as its
deterministic equal-fold tie breaker.

A later original-executable trace, retained in historical documentation commit
`7e8e0f24`, resolves the earlier P66A navigation gap:

- `FUN_0060a790(..., 2)` through `FUN_0060a890` and `FUN_00626ad0`, with
  `FUN_005f59f0` insertion, establishes case-folded display-text ordering of the
  master and filtered linked lists;
- `FUN_00442130` through `FUN_004ad730` / `FUN_004ad750` traverses enabled
  previous/next links and skips disabled rows;
- `FUN_0045da70` uses those neighbors for commands `0x84` and `0x83`; a null
  neighbor retains the endpoint, so navigation does not wrap.

The trace reports source-insertion stability for comparator-equal native rows,
whereas P65 deliberately uses object identity for a deterministic equal-fold
tie. W3 neither hides nor reimplements that distinction: it preserves the
canonical catalog exactly. If a real supported catalog contains an equal-fold
tie whose native insertion order differs, that is a bounded P65 catalog
reconciliation, not permission for the presenter to invent a second sort.

## Coverage

The generated synthetic corpus exercises all 356 logical objects for Alliance
and Empire. For each audience it produces 346 resolved topics and exactly the
ten P66A source-empty mission identities as `SourceUnavailable`, with no prose
or artwork fallback. Independent cases cover:

- exact resolved title, prose, factional artwork identity and resource metadata;
- seven category counts and order-preserving category projection;
- first and last topic boundaries without wraparound;
- literal mixed-case and equal-fold catalog order, with an intentional reverse
  mutation caught by the test;
- cockpit, index, object and all five contextual entry tokens;
- contextual miss to index to topic to next and back to the exact caller;
- contextual topic previous/next while retaining the original request route;
- cockpit to index to topic while retaining the cockpit route;
- index selection without a topic payload; and
- fail-closed topic follow-up without a selected object, plus exact diagnostics
  for unknown category, unknown object and wrong category.

## Verification

TDD and review RED evidence:

```text
cargo test -p rebellion-data encyclopedia_presenter --lib
FAILED to compile: presenter DTOs and API did not exist

cargo test -p rebellion-data \
  encyclopedia_presenter::tests::entry_intents_preserve_the_exact_non_fixture_return_route \
  --lib
FAILED: index mode returned Some(EncyclopediaTopicView), expected None

cargo test -p rebellion-data \
  encyclopedia_presenter::tests::follow_up_navigation_keeps_the_origin_route_across_complete_journeys \
  --lib
FAILED to compile: EncyclopediaPresentation::follow_up did not exist

intentional reverse-projection mutation
FAILED: literal order was Beta, ALPHA, alpha instead of alpha, ALPHA, Beta

cargo test -p rebellion-data \
  encyclopedia_presenter::tests::invalid_internal_selections_fail_closed_instead_of_guessing \
  --lib
FAILED to compile: MissingTopicSelection did not exist
```

GREEN and regression gates:

```text
cargo test -p rebellion-data encyclopedia_presenter --lib
8 passed; 0 failed

cargo test -p rebellion-data --lib
135 passed; 0 failed; 13 ignored (owned original-data tests)

LIBRARY_PATH="$PWD/.artifacts/native-link" cargo test --workspace --quiet
1,409 passed; 0 failed; 39 ignored

cargo clippy -p rebellion-data --lib --no-deps -- -D warnings
PASS (existing workspace manifest-inheritance warnings remain)

cargo check -p rebellion-data --bin encyclopedia-source-audit
PASS

cargo check -p rebellion-app --target wasm32-unknown-unknown --release
PASS (existing target-only unused-code warnings remain)

cargo mutants -p rebellion-data \
  --file crates/rebellion-data/src/encyclopedia_presenter.rs \
  --output /tmp/open-rebellion-w3-mutants-final2-20261006 \
  -- --lib encyclopedia_presenter
15 mutants tested: 8 caught, 7 unviable, 0 missed, 0 timed out

rustfmt --edition 2021 --check \
  crates/rebellion-data/src/encyclopedia_presenter.rs
PASS

git diff --check
PASS
```

The workspace run used the existing ignored `.artifacts/native-link/libasound.so`
link to this host's versioned ALSA library. No tracked dependency or source
changed. Repository-wide formatting and dependency-inclusive Clippy retain
unrelated audited baseline failures; the touched Rust source and no-deps crate
gate above are clean.

## Remaining work

W4 must adapt these DTOs to the existing authentic Encyclopedia bitmap window,
own decoded texture lifetime, and prove the required visual and interaction
states. W5 must install identical content through the canonical native and
packaged browser readers, enable production routes only after full close/return
journeys pass, and close the late W1/W2 cross-reader gates. Original A0
comparison and every strict `OBJ-01` acceptance cell remain open.
