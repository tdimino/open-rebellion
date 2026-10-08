# P65 Encyclopedia index-catalog checkpoint

P65 populates the authentic Galactic Encyclopedia index shell with an immutable
catalog reconstructed from the owned game data. The production renderer and
catalog loader compile on native and browser targets, while the browser route
remains test-only until topic composition and the complete modal lifecycle are
implemented.

> Corrected 2026-10-08 by P68: the original shared-object builder excludes ten
> gameplay-only or hidden mission records. The visible catalog is 346 entries,
> not 356. Historical browser artifact hashes below remain the P65 record.

## Source contract

`FUN_0045f100` maps the seven index controls to original object-family ranges.
The corresponding TEXTSTRA resources supply the displayed English labels:

| Command | Category | Object families | TEXTSTRA | Entries |
|---|---|---|---:|---:|
| `0x6f` | All Databases | all supported families | `0x1850` | 346 |
| `0x70` | System Database | `[0x90,0x98)` | `0x1855` | 200 |
| `0x71` | Ship Database | `[0x14,0x20)` | `0x1854` | 38 |
| `0x72` | Facilities Database | `[0x20,0x30)` | `0x1852` | 14 |
| `0x73` | Missions Database | visible `[0x50,0x80)` records with hidden flag clear | `0x1851` | 15 |
| `0x74` | Troop Database | `[0x10,0x14)` | `0x1856` | 10 |
| `0x75` | Personnel Database | `[0x30,0x40)` | `0x1853` | 69 |

The total is independently reproduced from the source DAT families: 200
systems, 30 capital ships, eight fighters, six defenses, six manufacturing
facilities, two production facilities, 15 visible mission types, ten troop types, six
major characters, 54 minor characters, and nine special-forces types. Entries
are sorted case-insensitively by their source name, with compound object ID as
the stable tie breaker.

## Implementation

- `crates/rebellion-data/src/encyclopedia_catalog.rs` loads the immutable
  catalog without adding it to `GameWorld` or changing the save format. Native
  builds read TEXTSTRA from the owned DLL; browser builds use the runtime
  pack's preloaded string table.
- `crates/rebellion-render/src/encyclopedia.rs` renders the source labels,
  current object, nine visible catalog rows, row selection, wheel scrolling,
  and the original list-scroll arrow region inside the P64 shell. The ninth
  row's paint and hit rectangle clip to the native 160-pixel list boundary.
- Selection is keyed by compound source-object identity. It survives a
  category change when the selected object belongs to the new filter and
  otherwise advances to the first matching source entry.
- Fixture scenario 42 and the populated-catalog route are compiled only with
  `interface-test-fixtures`. Production command `0x131` remains fail-closed.
- `tools/interface-parity/encyclopedia-index-shell.mjs --catalog` exercises all
  seven categories, scroll, row selection, modal click-through rejection, the
  exact catalog-count log, and the four-request startup contract in one fresh
  muted browser per faction.

## Verification

The durable machine-readable result is in
[`p65-encyclopedia-index-catalog/summary.json`](p65-encyclopedia-index-catalog/summary.json).
Raw owned bitmaps and browser captures remain in ignored local storage.

| Gate | Result |
|---|---|
| Source catalog tests | 3 passed |
| Renderer tests | 15 Encyclopedia-related tests passed |
| Fixture decoding tests | 2 passed |
| Legacy WASM manifest test | passed; all 11 catalog DATs are present in the per-file fallback |
| Changed-code mutation tests | data: 15/30 caught, 4 unviable; app/renderer: 70/117 caught, 5 unviable; no timeouts |
| Workspace tests | pass |
| Fresh muted browser processes | 2 of 2 passed and closed |
| Startup requests | 8 of 8 returned HTTP 200 |
| Catalog browser states | 22 of 22 passed |
| Static shell pixels compared | 1,567,280; zero different |
| Dynamic regions | title, Topic/current object, category label, and list separately populated in every state; exact A0 comparison deferred |
| Browser diagnostics | zero errors |
| P64 regression matrix | 32 of 32 exact; 4,963,200 pixels; zero different |
| Independent browser-evidence visual review | pass; no P0-P3 findings in shell, scroll, or row-selection states |
| Independent source/code review | pass after three P2/P3 corrections; no P0/P1 findings |

The catalog run verifies the exact P64 bitmap shell outside the dynamic text
and list rectangles. Those rectangles are excluded from source-pixel equality
because the source composite intentionally contains no rendered catalog text.
The harness separately requires visible content in each of the three dynamic
regions and checks catalog counts, category transitions, scrolling, selection,
and modal input behavior. Across every state, the minimum observed differences
from the empty source composite are 784 title pixels, 368 Topic/current-object
pixels, 499 category-label pixels, and 3,550 list pixels. An independent visual reviewer then
inspected both faction shells plus ordered before/after scroll and row-selection
screenshots. It found no P0-P3 issue and confirmed that scrolling preserves the
current object and geometry while row selection updates both the highlight and
current-object field.

Independent source/code review originally reproduced all 356 compound
identities from the owned DAT headers but did not apply the earlier
`FUN_00422620` collection predicate. P68 corrects that omission to 346 visible
identities. The review also
found three pre-commit gaps: two catalog DATs were absent from the legacy WASM
fallback, the ninth row extended two pixels beyond the native list, and the
dynamic comparison mask covered the lower edge of the category controls. P65
adds the missing manifest entries and regression test, clips paint and hit
geometry, tightens the mask into separate label/list rectangles, and reruns
both browser matrices after those corrections.

Mutation testing is diagnostic rather than acceptance. The data survivors are
one equivalent disjoint-bitfield OR/XOR mutation and ten native/WASM string
loader substitutions outside the ordinary host-unit boundary; the live browser
catalog-count assertion covers the exercised WASM loader. The app/renderer
survivors are concentrated in the async `main` entry, fixture routing, and egui
paint/input substitutions that host unit tests cannot observe. The retained
two-faction browser matrices exercise those visible paths. P65 does not convert
those classifications into any `OBJ-01` acceptance.

## Acceptance boundary

P65 proves source-derived catalog completeness and ordering, localized English
labels, source-family filtering, stable list selection, scrolling, the native
selected-category no-op, both faction
shells, and the test-only browser route. It does not prove other localizations,
exact original font rasterization, topic composition, ENCYTEXT or EDATA
association, previous and next navigation, contextual entry, missing-entry
fallback, close and production routing, window dragging, the viewport matrix,
or an original-runtime A0 comparison. No `OBJ-01` cell is accepted by P65
alone.
