---
title: "F-016C Main-Menu Completion Evidence"
description: "Destinations, keyboard traversal, campaign reset, corrected music, original button effects, and browser acceptance"
category: qa
created: 2026-09-09
updated: 2026-09-11
tags: [qa, main-menu, wasm, audio, astra, bitmap]
---

# F-016C Main-Menu Completion Evidence

F-016C completes the remaining functional cockpit and game-setup rows. The
browser release artifact passes; P04 is complete. Native build/startup and
original-resource loading also pass. Native interactive visual acceptance and
per-control browser semantics remain explicit P03 release-hardening work.

## Implemented behavior

- Credits opens a populated scrolling destination and returns to the cockpit.
- Multiplayer opens a populated setup destination and reports authoritative
  networking as unavailable until M4 instead of silently failing.
- Load/Options exposes mute plus music/SFX gain controls.
- Escape returns from a running campaign to the same populated cockpit and
  preserves one menu-music loop.
- Starting a second campaign replaces the first campaign's tick, missions,
  movements, combat cooldowns, AI instances, and event definitions.
- Tab and Shift+Tab traverse the 14 original controls one at a time; arrows,
  Enter, and Space remain single-shot.
- Browser and native paths load the actual `MDATA.300` shuttle-menu cue and
  the original `COMMON.DLL` button effects recovered from `FUN_00405560`.
- The Small, Medium, and Large screens use COMMON resources `10019`, `10018`,
  and `10017` respectively, so each selected state retains its matching scale.

## Original audio identity

| Asset | Original identity | Duration | SHA-256 |
|---|---|---:|---|
| Menu music | `MDATA.300`, *Return of the Jedi* / Battle of Endor cue | 35.340771 s | `7d1212e1a91eb8d88ebaf0ce59000753561e4066425a24177baf1b04d07447f5` |
| Galaxy-size effect | `COMMON.DLL` WAVE `8000` | 0.334 s | `c54c5be5081943b35f41fd5be268b211e895cbd3c26fcad840823968df5d4905` |
| Load/Options effect | `COMMON.DLL` WAVE `8001` | 0.360 s | `14963ae2154a8caa3a6fc6492ad742c21ff4957fe5c28b6eaa61dd11b842a420` |
| Quit effect | `COMMON.DLL` WAVE `8002` | 1.036 s | `4482e8415f306480e2a5ddbf73cef943202ff2aa2b99c7096511559a55338ffa` |
| Selection effect | `COMMON.DLL` WAVE `8004` | 0.357 s | `791165a1ad0cc579357e2248e5d71e463fd77e75634db70deb3bbec09653063f` |

The packaging script extracts these owned resources locally; no licensed WAV
is tracked. Public soundtrack indexing independently identifies `MDATA.300` as
the shuttle/main-menu cue: [full soundtrack index](https://www.youtube.com/watch?v=F48PXDa3fZY).

## Verification

| Gate | Result |
|---|---|
| Workspace all targets | 500 passed, 0 failed, 2 ignored |
| Native build | Pass with existing warnings |
| Native startup | 200 systems loaded; 4 original cockpit SFX loaded from `COMMON.DLL`; process remained live |
| Packaged WASM | Pass; 4,676,410 bytes |
| Runtime pack | 52 game files + 2,231 bitmaps + 5 audio files; 29,096,058 bytes |
| Astra broad pass | Credits, Multiplayer/M4 status, audio gain/mute, two campaigns, save-v11 namespace, reset, Quit, and actual WebAudio pass |
| Astra final R4 | 14 passed, 0 failed; safe to commit |
| Browser errors | 0 application errors, page errors, failed requests, or missing texture/bridge diagnostics |

Astra measured one 35.340770833-second looping source, no old 110.608-second
payload, no duplicate music loop, and the four expected non-looping SFX
durations. Muting prevented new sound starts; adjusted gains reached the audio
graph. Quit played resource 8002, stopped music, and left the application
surface cleanly.

The broad pass also proved two consecutive configurations without a reload:
Alliance / Expert / Large / Headquarters Only, then Empire / Easy / Small /
Standard. Generation two began at tick 0 with zero missions, movements, and
cooldowns, one active AI, no second AI, and 34 freshly defined events.

## Artifact identity

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `web/open-rebellion.wasm` | 4,676,410 | `b26c242ed1df3848f01367517e53b56e15a54df2fb9aca64cd9103beb5ef97c7` |
| `web/data/runtime.orpk` | 29,096,058 | `6123deec14cfbfd070573c7001ef1123bd52067273498ad0c36cb7fdb108f509` |

## Retained evidence

- [Broad R2 ledger](main-menu-completion/astra-r2-ledger.json) and
  [raw trace](main-menu-completion/astra-r2-raw.json)
- [R3 defect ledger](main-menu-completion/astra-r3-ledger.json)
- [Final R4 ledger](main-menu-completion/astra-r4-ledger.json) and
  [raw trace](main-menu-completion/astra-r4-raw.json)
- [Resting cockpit](main-menu-completion/cockpit.png),
  [Credits](main-menu-completion/credits.png),
  [Multiplayer](main-menu-completion/multiplayer.png), and
  [audio controls](main-menu-completion/audio-controls.png)
- [Campaign return](main-menu-completion/campaign-return.png),
  [clean second campaign](main-menu-completion/second-campaign.png), and
  [Quit](main-menu-completion/quit.png)
- [Forward Tab traversal](main-menu-completion/tab-forward.png) and
  [reverse traversal](main-menu-completion/tab-reverse.png)
- [Corrected Small, Medium, and Large selected states](main-menu-r3/galaxy-sizes-corrected-2026-09-11.png)

## 2026-09-11 selected-state regression

A focused resource-mapping test and a muted packaged-browser pass cover all
three galaxy selections. Each screen now keeps the matching native-scale art,
the lever moves to the corresponding detent, and no browser error or missing
asset diagnostic appears.
