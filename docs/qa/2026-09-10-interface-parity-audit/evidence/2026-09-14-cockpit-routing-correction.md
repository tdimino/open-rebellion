---
title: "PR 11 Cockpit Routing Correction Evidence"
description: "Recovered command identities, authentic control states, input boundaries, and browser evidence for cockpit commands 0x131 through 0x133"
category: qa
created: 2026-09-14
updated: 2026-09-14
tags: [qa, interface, parity, cockpit, gid, bitmap]
---

# PR 11 Cockpit Routing Correction Evidence

PR #11 corrects three strategic command identities for both factions. It also
strengthens the permanent browser harness around those controls. No substitute
Game Options or Encyclopedia screen is accepted by this checkpoint.

## Recovered contract

| Command | ID | Alliance rectangle and normal/held resources | Imperial rectangle and normal/held resources |
|---|---:|---|---|
| Encyclopedia | `0x131` | `(394,405,27,16)`, `10010/10009` | `(465,434,35,24)`, `10024/10023` |
| Galactic Information Display | `0x132` | `(446,406,27,16)`, `10012/10011` | `(519,434,37,25)`, `10026/10025` |
| Game Options side globe | `0x133` | `(3,355,27,41)`, `10013/10014` | `(79,192,35,57)`, `10027/10028` |

`REBEXE.EXE` functions `FUN_00427270`, `FUN_00422ce0`, `FUN_00602d30`,
`FUN_006028c0`, and `FUN_005fca00` establish the constructor records, command
dispatch, held-state paint, capture lifecycle, and source-shaped hit rule. F1
dispatches Game Options and F7 dispatches Encyclopedia.

## Implemented boundary

- The real `0x132` bottom control opens and closes the original GID menu. The
  same control performs both transitions.
- All three controls use their recovered faction rectangles, resource pairs,
  strict outside-edge behavior, capture, cancel, and release semantics.
- Physical `0x131` and `0x133` presses show distinct original held resources.
  Release restores the current strategic surface.
- F1 and F7 share the corrected command routing.
- Game Options and Encyclopedia log `pending_original_window` and remain
  closed. The old `E` shortcut is also fail-closed. Their original
  bitmap-driven windows are still open work.
- Event overlays suppress galaxy-map and cockpit pointer input for their entire
  dismissal frame. Egui keyboard ownership also suppresses cockpit
  accelerators.
- The former route-specific replacement screen and dead cockpit Save/Load
  control are removed.

## Verification

| Gate | Result |
|---|---|
| Workspace tests | 677 passed, 0 failed, 21 ignored |
| Asset-dependent tests | 4 passed, 0 failed |
| Cockpit-focused renderer tests | 27 passed, 0 failed |
| Browser-harness unit tests | 11 passed, 0 failed |
| GID and cockpit browser matrix | 152/152 passed; both factions and both viewports; 152 four-request startups; 152 muted launches; 0 errors; 152 browser closures |
| Tactical regression matrix | 40/40 passed; 40 four-request startups; 40 muted launches; 0 errors; 40 browser closures |
| Astra medium browser-evidence review | A1 pass; 16 retained screenshots and all 192 browser results inspected; 0 blockers |
| Production WASM | 5,267,498 bytes; SHA-256 `1c3b575f070a45a7139de909f910546d55216918b73c85775d8d62efe059a6cd` |
| Test-fixture WASM | 5,336,900 bytes; SHA-256 `8269c830e1b15cd1e6f8fc1b709a4160f0e23887a8790c7b32715ccc1abe3998` |
| Runtime pack | 52 game files, 2,303 bitmaps, 3,988 advisor frames, 87 tactical meshes, and 397 tactical textures; SHA-256 `430c90f76628ea2b5db74d2e1a582da83df311d92031ab288cc92ee4beeb58a8` |

The full strategic run is `2026-09-14T20-47-43-278Z-69098`. The tactical
regression run is `2026-09-14T20-52-21-860Z-78166`. Curated captures, hashes,
and probe results are preserved in the
[`pr11-cockpit-routing`](pr11-cockpit-routing/README.md) bundle.

## Acceptance boundary

This is an A1 implementation checkpoint. All 152 strategic captures and all 40
tactical captures are unbaselined against lossless owned-original screenshots.
No strict interface cell closes here. Original Game Options and Encyclopedia
windows, complete command-center state coverage, and the full cross-browser
matrix remain open.
