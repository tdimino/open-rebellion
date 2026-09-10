---
title: "F-016B Campaign Setup and Victory-Rule Evidence"
description: "Save v11 persistence, Standard and Headquarters Only rules, and Astra browser acceptance"
category: qa
created: 2026-09-09
updated: 2026-09-09
tags: [qa, astra, main-menu, save-load, victory, bitmap]
---

# F-016B Campaign Setup and Victory-Rule Evidence

## Outcome

F-016B passes. The original cockpit's faction, difficulty, galaxy size, and
game type now become explicit campaign configuration, survive save/reload/load,
and reach victory evaluation. P04 remains open only for a clean second-campaign
reset; this tranche does not claim the remaining P03 native, navigation,
accessibility, or audio-lifecycle gates.

## Behavior implemented

- `CampaignConfig` records `player_faction`, `difficulty`, `galaxy_size`, and
  `victory_conditions` in the live simulation and save v11.
- Standard mode propagates the game-type selection. A later source review
  reopened the resolution logic: the correct contract requires destruction of
  the mobile Alliance HQ plus Luke Skywalker and Mon Mothma for Empire, or
  capture of Coruscant plus Emperor Palpatine and Darth Vader for the Alliance.
  Current occupation and Death Star shortcuts remain parity defects.
- Headquarters Only removes the leader requirements while retaining the
  faction-specific HQ objective.
- The v10 positional body remains explicit. Its fingerprint is checked before
  migration; recoverable faction/difficulty values are inferred, unavailable
  setup fields receive documented Standard defaults, and the migrated v11
  fingerprint is honestly marked unverified.
- Browser writes use only `rebellion_save_v11_*` and
  `rebellion_meta_v11_*`; load/list/delete retain v10 and v9 fallbacks.

## Local verification

| Gate | Result |
|---|---:|
| Standard/Headquarters Only focused tests | 12 passed, 0 failed |
| Save/migration focused tests | 22 passed, 0 failed |
| Workspace all targets | 495 passed, 0 failed, 2 ignored |
| WASM release packaging | Passed with existing warnings |
| WASM | 4,628,391 bytes; `72416bf12663f174a94595ec87fc244c5bb7fa132b00786f7b3d02115fdaabad` |
| Runtime pack | 30,724,745 bytes; `472c3c8ce7968e34e250bbd97ae1e1cc5830a3d2e08b21ee5dd1d5ce2d4c5a18` |

## Astra medium browser acceptance

The first browser review passed 13 gates and failed 3 because native
`eprintln!` configuration records did not reach the browser console. Replacing
those records with the existing macroquad browser logger made the state
observable without changing gameplay.

The cleared R2 acceptance run passed 18/18:

- exact Alliance configuration before and after load:
  `Expert, Large Galaxy, Headquarters Only`;
- untouched Empire defaults: `Easy, Small Galaxy, Standard Game`;
- v11-only slot keys and a matching verified fingerprint
  `v1:7ee5e0093d3910e8` after reload/load;
- exactly four successful requests per cold load and reload;
- populated original cockpit and Alliance/Empire galaxy views at 640x480;
- zero application, bridge, page, request, or missing-bitmap errors.

The complete machine-readable result is
[`game-setup-r2/ledger.json`](game-setup-r2/ledger.json). Representative visual
evidence:

- [selected Alliance setup](game-setup-r2/05-alliance-selected.png)
- [Alliance galaxy](game-setup-r2/06-alliance-galaxy.png)
- [saved slot](game-setup-r2/08-slot1-saved.png)
- [cockpit load result](game-setup-r2/11-main-cockpit-load.png)
- [loaded Alliance galaxy](game-setup-r2/12-alliance-loaded-galaxy.png)
- [default Empire cockpit](game-setup-r2/13-empire-default-cockpit.png)
- [default Empire galaxy](game-setup-r2/14-empire-default-galaxy.png)

## Remaining boundary

Starting another campaign in the same process must still prove that every
campaign subsystem is rebuilt without carrying state from the previous game.
That is the remaining P04 gate and is deliberately not marked complete here.
