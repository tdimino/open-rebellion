# P58-B16 tactical weapon audio bank

P58-B16 replaces the provisional single-cue tactical route with the complete
weapon fire and impact bank recovered from the executable. Production capital
and fighter attacks now emit typed audio events, choose a deterministic source
variant, and route the matching `TACTICAL.DLL` WAVE through native and browser
audio backends. Browser verification remained muted.

This checkpoint does not claim exact original RNG sequencing, audible native
comparison, or strict original-game acceptance.

## Source contract

| Event | Registered source name | WAVE resources |
|---:|---|---:|
| `0x0d` | `SHIP_FIRELASERCANNON` | `13033–13035` |
| `0x0e` | `SHIP_FIRETURBOLASER` | `13036–13038` |
| `0x0f` | `SHIP_FIREIONCANNON` | `13039–13041` |
| `0x10` | `SHIP_FIRETORPEDO` | `13042–13044` |
| `0x11` | `SHIP_TAKE_LASER_HIT` | `13045–13047` |
| `0x12` | `SHIP_TAKE_ION_HIT` | `13048–13050` |
| `0x13` | `SHIP_TAKE_TURBO_HIT` | `13051–13053` |
| `0x14` | `SHIP_TAKE_TORPEDO_HIT` | `13054` |

[`FUN_005a7500_weapons_fire.c`](../../../../ghidra/notes/FUN_005a7500_weapons_fire.c)
establishes the event names and order. [`FUN_005bae60.c`](../../../../ghidra/notes/FUN_005bae60.c)
defines the variant counts and resource table, while
[`FUN_005bad50.c`](../../../../ghidra/notes/FUN_005bad50.c) selects one variant
from the event family. `FUN_005b3f10` preserves ion, laser, then turbolaser
dispatch order for capital batteries.

P58-B15 correctly established that WAVE `13054` could be extracted, packaged,
and routed, but labeled its trigger as ship destruction. The registered event
is `SHIP_TAKE_TORPEDO_HIT`; ship destruction is a separate source event. This
checkpoint removes the inferred destroyed-ship trigger and emits the recovered
events directly from weapon fire and impact resolution.

## Implementation result

- `dat-dumper --extract-tactical-sfx` extracts all 22 WAVE resources by exact
  ID without committing proprietary audio.
- The WASM build stages all 22 files and the runtime pack contains 28 audio
  entries in total.
- Capital-to-capital, capital-to-fighter, fighter-to-capital,
  fighter-to-fighter, fighter-to-Death-Star, and fighter-torpedo paths queue
  typed fire and impact cues.
- Each cue fixes its resource identity when the combat event is emitted, so
  frame scheduling cannot change variant selection.
- The deterministic selection seed is implementation evidence only. The
  executable's shared RNG sequence remains an open parity item.

## Verification

| Gate | Result |
|---|---|
| Workspace tests | Passed; renderer 255 passed and 1 ignored, app 16 passed |
| Focused audio mapping tests | Passed; all events and WAVE `13033–13054` covered exactly once |
| Runtime-pack Python tests | 4 passed |
| Harness unit tests | 26 passed |
| Catalog, matrix, A0-exclusion, production-exclusion checks | Passed; A1 106/106, strict 0/106 |
| Production WASM/package build | Passed; 22 tactical WAVs and 28 total audio entries |
| Focused muted browser journey | 4 of 4 passed; both factions, both viewports, and 22 of 22 audio-cache hits |
| Browser startup and cleanup | 4 four-request starts, 4 muted launches, 4 closed browsers |
| Strict 106-cell gate | Not run |

The focused run is
`2026-09-24T19-52-42-277Z-71200`. Raw screenshots, console logs, request
ledgers, and per-case results remain under the ignored
`.artifacts/interface-parity/` tree. Durable hashes and the exact source
contract are indexed in the
[artifact bundle](p58-b16-tactical-weapon-audio/README.md).
