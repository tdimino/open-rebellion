---
title: "P46D Detailed-System Tab Item Evidence"
description: "Source-mapped GOKRES items, bounded scrolling, selection, focus, and browser evidence for the six core system tabs"
category: qa
created: 2026-09-11
updated: 2026-09-11
tags: [qa, interface, parity, strategy, system, bitmap, scroll]
---

# P46D Detailed-System Tab Item Evidence

P46D replaces the text summaries inside the recovered detailed-system shell
with the first original-resource item composition for all six core tabs. This
is a partial `CMD-03`, `CMD-04`, and `OBJ-02` checkpoint. It does not close any
strict acceptance cell or claim complete object-window parity.

## Original contract

The item mapping is derived from original DAT identities and staged GOKRES
resources. Unknown classes fail closed instead of displaying unrelated art.

| Family | Implemented source mapping |
|---|---|
| Personnel | Major and minor character DAT identities to GOKRES miniature portraits |
| Fleets | Capital-ship and fighter DAT identities to the existing exact GOKRES miniature tables |
| Defenses | KDY-150, LNR I/II, GenCore I/II, and Death Star Shield resources |
| Manufacturing | Basic and advanced shipyard, training, construction, and Alliance Headquarters resources |
| Troops | Five Alliance and five Imperial regiment resources |
| Special forces | Four Alliance and five Imperial special-force resources |
| Production | Mine `16385` and refinery `16386` |

The retained original
[populated Troops capture](../reference-captures/lp-archive-discovery/friendly-populated-troops.png)
shows a three-column item grid with labels and bounded right-side navigation.
STRATEGY resources `10367`/`10368` provide the up states, `10365`/`10366`
provide the down states, and `10369` provides the track. The runtime displays
three rows at a time and scrolls by complete three-item rows.

## Implemented behavior

- Every displayed object uses its original GOKRES miniature and natural aspect
  ratio. The source blue matte is transparent for the GOKRES miniature range.
- Each visible miniature has a displayed entity label. Character names come
  from campaign data; fleet numbering and known facility and unit names are
  currently assigned by the renderer. Exact TEXTSTRA label sourcing remains
  open. Selection adds the relationship-colored outline used by the first
  recovered composition.
- Tab changes clear item selection and reset the row offset. Minimize and
  restore preserve the active tab, selected item, scroll row, and window
  position.
- Dense tabs remain inside the 231x304 detailed window. Later rows are reached
  through the recovered scrollbar buttons, and arrow input clamps at both
  ends.
- Detailed-system windows use foreground ownership over sector windows, so a
  restored or clicked detail remains visible and interactive.
- Item and window hit rectangles keep exclusive right and bottom edges after
  viewport scaling. Hidden rows do not paint or accept input.
- Explored hostile systems expose opposing objects only while the active
  faction's `FogState` currently reveals the system. Player-owned objects stay
  visible, unexplored systems expose no items, and tabs with only unsupported
  mappings remain disabled instead of opening blank. Exact original
  intelligence age, espionage, and detection semantics remain open.

Representative accepted captures:

- [Alliance Personnel selection](p46d-detailed-system-tabs/alliance-personnel-selected.png)
- [Alliance Production selection](p46d-detailed-system-tabs/alliance-production-selected.png)
- [Empire Troops at the first row](p46d-detailed-system-tabs/empire-troops-top.png)
- [Empire later item selection](p46d-detailed-system-tabs/empire-troops-later-selected.png)
- [Alliance detail foreground focus](p46d-detailed-system-tabs/alliance-detail-foreground.png)
- [Alliance six-tab sheet at 1000x700](p46d-detailed-system-tabs/alliance-1000-tabs-sheet.png)
- [Empire six-tab sheet at 1000x700](p46d-detailed-system-tabs/empire-1000-tabs-sheet.png)
- [Alliance overflow sheet at 640x480](p46d-detailed-system-tabs/alliance-overflow-640-sheet.png)
- [Alliance overflow sheet at 1000x700](p46d-detailed-system-tabs/alliance-overflow-1000-sheet.png)
- [Alliance focus sheet at 640x480](p46d-detailed-system-tabs/alliance-640-focus-sheet.png)
- [Alliance focus sheet at 1000x700](p46d-detailed-system-tabs/alliance-1000-focus-sheet.png)
- [GOKRES cache identity sheet](p46d-detailed-system-tabs/gokres-cache-sheet.png)

## Browser acceptance

The first Astra review found two P2 defects: normal seeded Troops could paint
and receive clicks outside the detail window, and a restored detail could
remain beneath its sector window. The implementation then added bounded row
navigation and explicit cross-family foreground ownership.

The broad rerun passed those corrections and the two-faction tab matrix, but
found that the first packaged scrollbar assigned the up and down glyphs in the
opposite directions. The source mapping was corrected and the WASM was rebuilt.
The final focused run validates the corrected artifact separately so the
earlier finding is not hidden by a source-only change.

The final acceptance covered both factions at 640x480 and 1000x700, all six tab
families, empty and disabled tabs, 80 integer inside/outside edge probes,
selection, tab clearing, minimize/restore, foreground focus, and GOKRES
transparency. A temporary day-zero Alliance save fixture supplied a dense owned
Troops tab. It was loaded through the normal Load interface without modifying
the served artifact. The follow-up verified the Alliance scrollbar, both clamp
limits, state restoration, exposed-title window raising, and blocked clicks
through an overlapping foreground window at both viewports.

All three final browser sessions loaded exactly four resources with HTTP 200
responses, matched the accepted WASM and runtime-pack hashes, started with
in-game music disabled, and closed their muted Chromium contexts. Astra
reported no P0 through P3 findings and `ready_to_commit: true`.

Machine-readable evidence:

- [final Astra acceptance](p46d-detailed-system-tabs/astra-acceptance.json)
- [main browser report](p46d-detailed-system-tabs/astra-browser-report.json)
- [GOKRES matches](p46d-detailed-system-tabs/gokres-matches.json),
  [resource matches](p46d-detailed-system-tabs/resource-matches.json), and
  [state comparisons](p46d-detailed-system-tabs/state-comparisons.json)
- [integer edge sweep](p46d-detailed-system-tabs/integer-edge-sweep.json)
- [Alliance overflow report](p46d-detailed-system-tabs/alliance-overflow-report.json),
  [resource matches](p46d-detailed-system-tabs/alliance-overflow-resource-matches.json),
  and [state comparisons](p46d-detailed-system-tabs/alliance-overflow-state-comparisons.json)
- [Alliance focus report](p46d-detailed-system-tabs/alliance-focus-report.json),
  [event log](p46d-detailed-system-tabs/alliance-focus-browser.jsonl), and
  [pixel comparisons](p46d-detailed-system-tabs/alliance-focus-comparisons.json)

## Deliberately open boundary

The following work remains and prevents complete `CMD-03`, `CMD-04`, and
`OBJ-02` acceptance:

- nested fleet ships, fighters, carried units, object status, and assignment
  compositions;
- original item activation, context, drag, drop, transfer, and command paths;
- exact intelligence age, espionage, detection, and hostile-visibility
  semantics in a full campaign matrix;
- complete HQ, blockade, uprising, construction, transit, damage, destroyed,
  unexplored, and uninhabited state compositions;
- exact rail thumbnails and every active, inactive, capacity, and multiwindow
  permutation;
- the complete lossless A0, native, browser, viewport, DPR, faction, and state
  matrix.

## Verification

| Gate | Result |
|---|---|
| `rebellion-render` tests | 137 passed, 0 failed |
| Workspace suite | 629 passed, 0 failed, 20 ignored |
| Packaged WASM build | Passed with pre-existing warnings |
| Final WASM SHA-256 | `7dcdd00685c9e73c6bf7d1cf41020c5a3603f71497fe839d791224303091976f` |
| Runtime pack | 52 game files, 2,303 bitmaps, 3,988 advisor frames, five audio files |
| Runtime pack SHA-256 | `1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862` |
| Browser startup | Four requests, all HTTP 200; zero observed console, page, request, WASM, panic, WebGL, decode, or missing-asset errors |
| Browser audio | Chromium `--mute-audio`; in-game music false before each campaign |
| Astra medium | 137 renderer tests and 17 adversarial tests passed; three muted browser sessions covered both factions and viewports with no P0-P3 findings and `ready_to_commit: true` |
| Scoped Clippy | Exit 0 with pre-existing warnings |
| Strict interface cells accepted | 0 of 564; the formal matrix remains open |

The navigation-shell checkpoint and its earlier evidence remain in the
[UIP-B01 strategic navigation record](2026-09-11-strategic-window-navigation.md).
