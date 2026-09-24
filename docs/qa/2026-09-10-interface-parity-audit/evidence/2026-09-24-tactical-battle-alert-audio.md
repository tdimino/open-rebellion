# P58-B15 tactical Battle Alert and audio routing

P58-B15 maps the final two deterministic A1 tactical cells. It restores the
original faction Battle Alert surface before command and routes the recovered
tactical score and first source-identified event cue through the production
audio engine. It does not claim strict original-game acceptance.

## Source contract

| Source | Recovered behavior |
|---|---|
| `FUN_0044f860` | Builds Battle Alert from faction frame `10710/10711`, scene `10712/10713`, right rail `10820/10821`, four tab pairs, and three 134×27 command families |
| Alliance controls | Retreat `10971–10973`, Simulate `10716–10718`, Take Command `10719–10721` |
| Imperial controls | Retreat `10974–10976`, Simulate `10722–10724`, Take Command `10725–10727` |
| Original manual | Opposing fleets enter Battle Alert before the Tactical Display; tabs show the battle, both forces, and system; Take Command enters paused combat |
| `MDATA.307` | Tactical battle score, staged in the runtime pack with SHA-256 `58f7166f9805e10f83ba6dea9cbd853c041000f2aca4c6111d44cf5cf1b99803` |
| `FUN_005bae60`, `FUN_005ba980`, `FUN_005bad50`, `FUN_005ba520` | Tactical audio-manager construction, four-variant event selection, and event `0x14` routing to TACTICAL WAVE `13054` |
| `TACTICAL.DLL` | Owned source for WAVE `13054`; DLL SHA-256 `db12cdcfb6c85cec572a425307d2ad617bbc850027657424c3cc1f61c6ec275a` |

Correction: this checkpoint proved extraction and routing for WAVE `13054`,
but its runtime trigger labeled event `0x14` as ship destruction. Subsequent
source recovery identifies `0x14` as `SHIP_TAKE_TORPEDO_HIT`; ship destruction
is a separate event. [P58-B16](2026-09-24-tactical-weapon-audio.md) removes the
inferred destruction trigger and restores the complete weapon-event bank.

Take Command closes Battle Alert and enters the same paused production battle
scene used by campaign combat. Simulate routes to production auto-resolution.
Retreat marks eligible player hyperdrive ships for withdrawal. Battle audio
does not begin until the alert closes.

## Result

| Measure | Result |
|---|---:|
| Canonical tactical cells | 106 |
| Cells with a deterministic A1 scenario | 106 |
| Journey-mapped cells | 87 |
| Snapshot-mapped cells | 19 |
| Cells needing a new scenario | 0 |
| A0 captures | 0 |
| Strictly accepted cells | 0 |

The focused browser journeys cover both factions at 640×480 and letterboxed
1280×800. Native-size probes prove the unobscured source bitmaps exactly; the
journey also exercises every tab, the held Take Command bitmap, paused tactical
entry, MDATA `307`, WAVE `13054` transport, four-request startup, muted audio, and clean
browser shutdown.

## Verification

| Gate | Result |
|---|---|
| Renderer tests | 254 passed, 1 ignored |
| App tests | 16 passed |
| Extractor tests | Passed |
| Harness unit tests | 26 passed |
| Tactical catalog and matrix checks | Passed; 106 mapped, 0 missing |
| Battle Alert browser journey | 4 of 4 passed |
| Tactical audio browser journey | 4 of 4 passed |
| Startup, mute, and cleanup | 8 four-request starts, 8 muted launches, 8 closed browsers |
| Independent browser review | Passed all 8 cases; one A0-reference-needed text-contrast note |
| A0 acceptance | 0 of 106; strict gate not run |

Colored alert text crosses bright authored scene art in places. The review
records this as reference-needed rather than adding an invented backing panel
without lossless A0 evidence.

Raw screenshots, network records, console logs, and per-case JSON remain in
the ignored `.artifacts/interface-parity/` tree. Durable hashes and summaries
are indexed in the
[artifact bundle](p58-b15-tactical-alert-audio/README.md).
