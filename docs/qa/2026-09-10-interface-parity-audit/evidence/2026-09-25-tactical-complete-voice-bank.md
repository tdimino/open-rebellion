# P58-B18 complete tactical voice bank

P58-B18 transports every faction tactical voice resource addressed by the
original executable: 153 Alliance recordings and 132 Imperial recordings,
285 total. Production tactical state now emits the source event for withdrawal
start and completion, battle outcomes, Death Star firing and recharge,
broken-off and destroyed Death Star attacks, and the four Alliance trench-run
groups and results. Browser verification remained muted.

This checkpoint proves complete bank identity and transport plus the listed
production transitions. It does not claim that every original completion,
destruction, recovery, warning, or ordered trench-run line has a production
caller. Exact shared-RNG sequencing, mixing, interruption, audible native
comparison, lossless original captures, and strict acceptance remain open.

## Source contract

`FUN_005bae60` builds one continuous tactical event table. `FUN_005ba980`
queues an event index and `FUN_005bad50` resolves it to the faction DLL and
WAVE resource.

| Event interval | Resource interval | Faction | Count |
|---|---|---|---:|
| `0x20–0x99` | `VOICEFXA 14001–14122` | Alliance | 122 |
| `0x9a–0x11d` | `VOICEFXE 15001–15132` | Imperial | 132 |
| `0x11e–0x13c` | `VOICEFXA 15133–15163` | Alliance trench run | 31 |

The exact per-family spans, event IDs, resource IDs, transcription confidence,
implemented routes, and remaining routes are in the
[tactical voice map](../../../reference/asset-library/tactical-voice-map.json).
The compact map is validated against the executable formulas and covers every
event from decimal 32 through 316 exactly once.

## Implementation result

- `dat-dumper --extract-tactical-voice` extracts all 285 resources from owned
  `VOICEFXA.DLL` and `VOICEFXE.DLL` files without committing proprietary audio.
- Every extracted file byte-matches the existing dual-engine transcription
  manifest used to identify the voice families.
- Native and WASM loaders share the same event-to-resource mapping. The
  deterministic browser pack carries all 285 recordings.
- The fixture emits the resources in executable table order and the browser
  harness rejects missing, duplicate, or out-of-range voice events.
- Production tactical transitions now route the selected withdrawal, battle
  result, Death Star, and Alliance trench-run events in addition to P58-B17's
  battle-ready and command acknowledgements.

## Verification

| Gate | Result |
|---|---|
| Complete Rust workspace | 804 passed, 0 failed, 21 ignored |
| Focused renderer tests | 285 passed, 0 failed, 1 ignored |
| Fixture-feature renderer tests | 286 passed, 0 failed, 2 ignored |
| Focused app tests | 17 passed, 0 failed |
| Scoped mutation gate | Default: 71 total, 50 caught, 16 unviable, 5 explained; fixture feature: 68 total, 51 caught, 17 unviable, 0 missed |
| Owned-DLL extraction | 153 Alliance plus 132 Imperial WAVs; 285 total |
| Mapping validation | 285 unique events and resources across the three exact source ranges |
| Browser runtime pack | 313 audio entries total; 285 tactical voices plus 22 tactical weapon effects |
| Focused muted browser journey | 4 of 4 passed; both factions and both viewports |
| Browser audio route | 285 voices and 22 weapon effects loaded and routed per case |
| Browser startup and cleanup | 4 four-request starts, 4 muted launches, 4 closed browsers |
| Independent browser review | PASS; populated faction shells, stable viewports, no in-scope blocker |
| Strict 106-cell gate | Not run; A0 coverage and acceptance remain 0 of 106 |

The five survivors in the default mutation configuration are bounded rather
than silent: two fixture-only mutations are compiled out there and are caught
by the zero-miss feature-enabled run; `dat-dumper` process completion and the
fixture's ready/failed status emitters are observed by the extraction and
browser integration gates rather than a unit-test return value. All mutated
production tactical branches were either caught or failed to compile.

The combined-source focused run is `2026-09-25T14-33-15-631Z-45212`. Its four
screenshots are byte-identical to the independently reviewed run. Raw
screenshots, console logs, request ledgers, and per-case results remain under
the ignored `.artifacts/interface-parity/` tree. Durable hashes and the exact
scope are in the [artifact bundle](p58-b18-tactical-voice-bank/README.md).
