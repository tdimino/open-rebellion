# Encyclopedia window construction

This note separates the original Galactic Encyclopedia from the Message Index.
The two surfaces were conflated in P63 because both load STRATEGY resources and
the Message Index constructor also opens `encybmap.dll`. The executable and
original screenshots establish distinct routes, layouts, and control sets.

## Entry and dimensions

`FUN_00429f30` is the command-center Encyclopedia route. It checks window ID
`0x19`, then constructs `FUN_0045d400` at 470 by 330. The constructor installs
vtable `PTR_FUN_00659fa8`, loads both `encytext.dll` and `encybmap.dll`, and
resolves an initial game object or falls back to index mode.

`FUN_0045ddc0` is the Encyclopedia layout routine. The client is 470 by 330,
while its faction base resources are 470 by 331 and therefore lose their
bottom row at the client boundary. Index mode composes STRATEGY 10338 at
`(12,13)`, faction base 10335 or 10336, faction rail 10585 or 10589 at
`(412,0)`, and TEXTSTRA `0x1843` at `(36,48)`. Topic mode substitutes STRATEGY
10337 at `(12,14)`. It is not `FUN_004665f0`; that function belongs to the
Message Index.

## Exact index controls

The category parent is `(36,78,361,41)`. Every child has a 49 by 41 control
rectangle. The table is ordered spatially; the constructor emits the controls
in a different order.

| Command | Absolute x | Filter | Label | Alliance normal/pressed | Empire normal/pressed |
| --- | ---: | --- | ---: | --- | --- |
| `0x6f` | 36 | all objects | `0x1850` | 10340 / 10339 | 10340 / 10339 |
| `0x70` | 88 | family `[0x90,0x98)` | `0x1855` | 10350 / 10349 | 10350 / 10349 |
| `0x71` | 140 | family `[0x14,0x20)` | `0x1854` | 10348 / 10347 | 10360 / 10359 |
| `0x72` | 192 | family `[0x20,0x30)` | `0x1852` | 10344 / 10343 | 10356 / 10355 |
| `0x73` | 244 | family `[0x40,0x80)` | `0x1851` | 11616 / 11615 | 11618 / 11617 |
| `0x74` | 296 | family `[0x10,0x14)` | `0x1856` | 10352 / 10351 | 10362 / 10361 |
| `0x75` | 348 | family `[0x30,0x40)` | `0x1853` | 10346 / 10345 | 10358 / 10357 |

The `0x75` resources are 49 or 50 by 57 pixels. The original paints from
source origin `(0,0)` into the fixed control, clipping the bottom 16 rows and,
for the 50-pixel-wide resources, the rightmost column. It does not scale or
center them.

The right rail uses close `0xfb`, topic `0x67`, and index `0x68`. Alliance
rectangles are `(423,25,32,31)`, `(423,93,32,31)`, and `(423,147,32,31)` with
resource pairs 10370/10371, 10374/10375, and 10372/10373. Empire rectangles are
`(426,21,44,41)`, `(426,89,44,41)`, and `(426,143,44,41)` with pairs
10376/10377, 10380/10381, and 10378/10379. Index mode makes `0x68` current and
topic mode makes `0x67` current.

## Text, list, and topic geometry

- main heading or topic title: `(36,14)`, width 350;
- current-object selector: `(143,45,245,18)` in index mode;
- selected-category label: `(40,119)`, width 283;
- object list: `(36,137,350,160)`, command `0x65`;
- EDATA image: native 400 by 200 at `(12,31)` in topic mode;
- topic body: `(17,231,395,80)`;
- previous `0x83`: `(28,14,21,17)`, resources 10385/10386/10387;
- next `0x84`: `(380,14,21,17)`, resources 10382/10383/10384.

Index Left and Right cycle visible category children and wrap. Topic Left and
Right invoke previous and next. Exact localized category names beyond the
source-backed labels must come from the original string resources, not icon
interpretation.

## Source catalog and labels

P65 binds the recovered family ranges to the immutable source DAT catalog.
The owned English TEXTSTRA table resolves `0x1842` to `Galactic Encyclopedia`,
`0x1843` to `Topic`, and the category resources to `All Databases`, `System
Database`, `Ship Database`, `Facilities Database`, `Missions Database`, `Troop
Database`, and `Personnel Database` in command order `0x6f..0x75`.

The shared collection built by `FUN_00422620` produces 346 visible entries: 200 systems; 30 capital
ships and eight fighters; six defenses, six manufacturing facilities, and two
production facilities; 15 visible mission types; ten troop types; and six major
characters, 54 minor characters, and nine special-forces types. A compound
object ID uses the DAT family as its high byte and the source record ID as its
low word. P65 sorts case-insensitively by source name with that identity as a
stable tie breaker, matching the alphabetical index behavior visible in the
classified original capture. Missions are restricted to family `[0x50,0x80)`
and then require the resolved mission record's `+0x5c` hidden flag to be zero.
Four `0x41..0x44` gameplay states and six hidden missions are therefore outside
the visible collection.

## Recovered workflow

- Index mode uses category commands `0x6f` through `0x75`.
- The index selector is an editable title field, not a read-only selected-name
  label. `FUN_0045d8f0` handles its change notification `0x408` for control
  `0x64`, calls `FUN_00609650` against the current list, and stores the
  resolved record at window `+0x148`. Its `0x407` Enter notification switches
  to topic mode through `FUN_0045f480(this, 2)` when a record is selected.
- `FUN_00609650` performs an ordered, case-insensitive longest-common-prefix
  search. The first equal-scoring row wins, and comparison does not require
  the whole query to match. Thus `tallon` retains `Talon Karrde` after the
  second `l` stops increasing the prefix score. An empty query clears the
  selection; a non-empty query with no shared first character retains the
  first ordered row.
- `FUN_00605160` gives the title field native edit focus and posts `0x408`
  after typing, backspace, or delete and `0x407` on Enter. The parent forwards
  Up, Down, Page Up, and Page Down to the list while the edit owns focus.
- `FUN_0045da70` copies a pointer/list-selected row title back into the edit
  field. `FUN_0045f100` reapplies the current edit query after rebuilding a
  category, so category changes retain the same lookup behavior.
- `FUN_0045f100` rebuilds the object list for a selected category from the
  authoritative game-object collection and updates the selected category label.
- `FUN_0045f480` switches between index and topic modes.
- `FUN_0045fa60` resolves the selected object's EDATA bitmap, loads its text
  resource from `encytext.dll`, updates the topic title, and configures previous
  and next availability through `FUN_0045fd20`.
- `FUN_0045fe60` routes keyboard navigation; in topic mode Left and Right invoke
  commands `0x83` and `0x84`.

## Evidence boundary

- Static source: `FUN_00429f30.c`, `FUN_0045d400.c`, `FUN_0045ddc0.c`,
  `FUN_0045f100.c`, `FUN_0045f480.c`, `FUN_0045fa60.c`, `FUN_0045fd20.c`, and
  `FUN_0045fe60.c` in this directory, plus disassembly of `FUN_0045d8f0`,
  `FUN_0045da70`, `FUN_00605160`, and decompiled `FUN_00609650.c`.
- Visual corroboration: the Encyclopedia index and topic captures classified in
  `docs/qa/2026-09-10-interface-parity-audit/screenshot-ledger.md`.
- P62 proves transport for all 187 owned EDATA images. It does not prove this
  window's category ordering, entity bindings, text, navigation, geometry, or
  A0 parity.
- P68 corrects P65's count and proves the 346-entry English visible catalog,
  including mission-family and hidden-record filtering,
  alphabetical ordering, stable row identity, and scrolling inside both
  faction shells. The route remains test-only.
- P66A proves strict local English ENCYTEXT/ENCYBMAP extraction plus ordinary,
  system-picture, and factional artwork binding for 346 topics per faction.
  Browser transport and visible topic composition remain open.
- No `OBJ-01` cell is accepted. Other localizations, exact font rendering,
  browser/runtime source ingestion, topic composition, navigation, contextual
  entry, visible missing entries, production routing, and original-runtime
  comparison remain open.
