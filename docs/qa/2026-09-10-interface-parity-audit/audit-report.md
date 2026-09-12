---
title: "Original Interface Parity Audit Report"
description: "Evidence-backed diagnosis and execution plan for complete bitmap-driven UI parity"
category: qa
created: 2026-09-10
updated: 2026-09-11
tags: [qa, interface, parity, bitmap, strategy, tactical, multiplayer]
---

# Original Interface Parity Audit Report

## Progress snapshot

| Measure | Current position |
|---|---:|
| Practical interface scope materially tackled | approximately 20 to 23% |
| Practical interface scope remaining | approximately 77 to 80% |
| Required surface families | 43 |
| Family status | 0 complete, 9 partial, 34 fail |
| Strictly accepted required cells | 0 of 564 |

The practical percentage measures scoped implementation work and guides the
[current batched plan](../../plans/2026-09-11-feat-batched-interface-parity-plan.md).
It is not a parity score. The strict cell denominator remains authoritative.

## Conclusion

The user-visible campaign interface currently fails parity with the 1998
Coolhand Interactive and LucasArts release of *Star Wars: Rebellion*, titled
*Supremacy* in the UK. The shuttle main menu passes its scoped implementation
gate, but remains partial here until every original interaction-state cell has
A0 or A1 evidence. Entering a campaign now reaches the recovered faction shell
on an exact, centered 640x480 canvas. Its six primary faction controls and GID
control now use recovered STRATEGY resources and the native input contract.
The default Popular Support view also uses its original active backdrop,
compact legend, marker families, and support thresholds. Most other content
within that shell remains incomplete.

This explains the observed symptoms:

- P46A fixes shell stretching, crops STRATEGY 900/901 from 640x481 sources to
  480 display rows, and applies the recovered faction apertures. P46B removes
  the replacement primary text-button strip and restores all six normal and
  captured-press bitmap pairs, exact hotspots, and command recognition. Their
  original destination windows remain closed instead of opening replacement
  panels. First-pass original sector and detailed system shells now replace
  the invented sidebar, including double-click routing, core tabs, and the
  rail lifecycle. All six tabs now render source-mapped GOKRES miniatures and
  displayed entity labels within the original three-column scrolled viewport.
  Coarse current-intelligence gating prevents stale hostile views from exposing
  opposing objects. Nested object compositions, exact intelligence semantics,
  and commands remain open. The replacement message and status
  bars are also withheld because
  they covered the original controls. Their authentic versions remain open.
- P49 restores the original nine Message Index side-rail resting icons for
  each faction. Their browser pixels match the source STRATEGY bitmaps at
  640x480. The unread state, input, and original index window remain open.
- P47A and P47B restore the bright Display Off and dim active galaxy resources,
  the default Popular Support caption, compact legend, and native marker
  families. Synthetic labels, circles, glows, facility squares, fleet glyphs,
  fog, and blockade primitives are withheld from this parity view. The
  P48 restores the original-command GID menu tree. P50 replaces its synthetic
  stroke with the eight original STRATEGY frame tiles and removes the fade.
  P51 removes an unsupported hover wash and checks visible frame pixels under
  the detailed system window.
  Menu interior, typography, detailed legend, remaining modes and overlays,
  and exact map interaction remain open.
- Selecting a system now opens a recovered modeless sector shell. Double-click
  opens the original detailed-system shell and core bitmap tabs. Illustrated
  first-pass item contents, bounded scrollbar navigation, and selection work.
  Nested contents, drag actions, complete state variants, and exact rail
  thumbnails remain incomplete.
- The four original advisor idle runs now follow the shared canvas transform in
  native and WASM. Original action, voice, and chrome behavior remains
  unimplemented.
- Several cockpit apertures remain blank because the original window-reference,
  advisor, message, and command compositions have not been implemented.

The project can retain its verified simulation behavior while replacing every
visible reconstruction with the original interface contract.

## Source evidence

The local source inspection establishes the implementation causes:

| Finding | Repository evidence | Original evidence |
|---------|---------------------|-------------------|
| Recovered strategic shell canvas | `crates/rebellion-render/src/cockpit.rs`, `crates/rebellion-render/src/lib.rs`, `crates/rebellion-app/src/main.rs` | `FUN_00421c70`, STRATEGY 900/901, and [P46A evidence](evidence/2026-09-11-strategic-shell-canvas.md) |
| Recovered primary strategy controls | `crates/rebellion-render/src/cockpit.rs`, `crates/rebellion-render/src/bmp_cache.rs`, `crates/rebellion-app/src/main.rs` | `FUN_00427270`, `FUN_006028c0`, `FUN_00602d30`, `FUN_005fca00`, manual Figure 3.8, and [P46B evidence](evidence/2026-09-11-strategic-command-controls.md) |
| Partial authored galaxy and GID | `crates/rebellion-render/src/lib.rs`, `crates/rebellion-render/src/bmp_cache.rs`, `crates/rebellion-render/src/cockpit.rs` | STRATEGY 902/903, 10013/10014, 10027/10028, 10100 through 10107, 10146 through 10158, 10168, manual pp. 66–73, [P47B evidence](evidence/2026-09-11-popular-support-gid.md), and [P50 evidence](evidence/2026-09-12-gid-menu-frame.md) |
| Partial original sector and system navigation | `crates/rebellion-render/src/sector_window.rs`, `crates/rebellion-render/src/system_window.rs` | Manual pp. 67–68, 97–100, and 122–124; recovered constructors, [navigation evidence](evidence/2026-09-11-strategic-window-navigation.md), and [tab-item evidence](evidence/2026-09-11-detailed-system-tab-items.md) |
| Partial browser droids | `crates/rebellion-render/src/advisor.rs` | Manual pp. 20–21 and 73–79; official Steam faction captures; [P44 evidence](../2026-09-08-full-functionality-audit/evidence/2026-09-10-authentic-droid-advisors.md) |
| Blank browser encyclopedia art | `crates/rebellion-render/src/encyclopedia.rs:486` | Manual pp. 71–72 and 192 original EDATA entries |
| Incomplete browser asset pack | `scripts/build-runtime-pack.py:29` | Original ALSPRITE, EMSPRITE, ALBRIEF, EMBRIEF, REBDLOG, and EDATA families |
| Invented live ground-combat screen | `crates/rebellion-render/src/ground_combat.rs:202` | Manual pp. 119–121 assault summaries and reports |
| First original tactical shell and controls; battle contents remain replacements | `crates/rebellion-render/src/tactical_view.rs`, `crates/rebellion-render/src/bmp_cache.rs` | Manual pp. 139–150, original TACTICAL resources, and [P52 evidence](evidence/2026-09-12-tactical-shell-controls.md) |

The original identity baseline comes from the locally preserved
[official manual](../../reference/campaign-history/archive/star-wars-rebellion-manual.pdf),
the [Steam product page](https://store.steampowered.com/app/441550/STAR_WARS_Rebellion/),
and the cited galleries in the [reference ledger](reference-ledger.json).

The [reverse-engineering ledger](reverse-engineering-ledger.md) owns the
executable-to-resource evidence queue. A read-only Ghidra pass recovered exact
strategic apertures, twelve reference-rail rectangles, control geometry,
commands, bitmap-state paint paths, managed object-window routing, GID display
construction, and faction advisor apertures. The extractor and runtime pack now
preserve all 3,988 ALSPRITE and EMSPRITE type-302 frames. Briefing, tactical,
dialog, encyclopedia, advisor-control, and voice families remain incomplete.
P46A corroborates the shell and aperture portion at runtime, including one
shared transform for overlays, hit tests, and advisors. P46B implements the six
primary faction controls from their exact resources, geometry, state, input,
and command paths. P46C and P46D add the first reference-rail lifecycle,
sector/system window journey, source-mapped tab items, coarse
current-intelligence gating, and recovered scrollbar art. Exact rail
thumbnails, destination-window compositions, nested object contents, exact
intelligence semantics, commands, other controls, and uncommon states remain open.
P47A restores the bright STRATEGY 902 galaxy at its source-aligned canvas
origin beneath both faction shells. P47B proves 902 is Display Off and restores
the active 903 Popular Support view, exact faction GID control, compact legend,
native marker families, and support thresholds. P48 adds the code-built GID
command tree. P50 restores its eight source frame tiles and instant display,
with exact source-pixel checks at native size. Menu interior, typography,
geometry, detailed legend, remaining modes and overlays, sector art, and exact
map input remain open. P51 removes a provisional hover wash and checks the
frame pixels visible under a foreground system window. Static discoveries do
not replace required original-runtime visual
acceptance.

## Immediate findings

| ID | Severity | Finding | Status |
|----|----------|---------|--------|
| UIP-F-001 | P0 | The shell, viewport scaling, six primary controls, and first rail lifecycle pass scoped checkpoints; the full control matrix, exact rail thumbnails, and required aperture content remain incomplete. | partial |
| UIP-F-002 | P0 | The original Display Off and active Popular Support backdrops, GID control, compact legend, marker families, and size thresholds render in native and WASM; the original menu, detailed legend, remaining modes and overlays, sector art, and exact map input remain incomplete. | partial |
| UIP-F-003 | P0 | Original sector and detailed-system shells replace the invented sidebar; all six tabs have first-pass source-mapped items, while nested contents, commands, indicators, and uncommon states remain open. | partial |
| UIP-F-004 | P0 | Authentic advisor idle runs render for both factions in the scaled apertures, but complete action, voice, and chrome behavior is absent. | partial |
| UIP-F-005 | P0 | Runtime pack v2 includes ALSPRITE and EMSPRITE BMP/type-302 content; ALBRIEF, EMBRIEF, REBDLOG, EDATA, action controls, and voice remain omitted. | partial |
| UIP-F-006 | P1 | WASM encyclopedia image loading always returns no texture. | fail |
| UIP-F-007 | P1 | Finders, production, missions, messages, options, and object status use replacement layouts or are absent. | fail |
| UIP-F-008 | P1 | Battle Alert, assault/bombardment reports, and battle-result routing are absent or bypassed. | fail |
| UIP-F-009 | P1 | The first original 640×480 tactical shell and pause, highlight, and zoom control checkpoint passes. Procedural battle contents and incomplete command, result, special-state, and audio paths remain. | fail |
| UIP-F-010 | P1 | The custom live ground-combat screen has no original counterpart. | fail |
| UIP-F-011 | P1 | Modeless focus, close, minimize, restore, and 12-slot rail routing work; exact active/inactive rail thumbnails and the complete multiwindow matrix remain open. | partial |
| UIP-F-012 | P1 | Authoritative original captures are still missing for rare and transient states. | partial |

Existing browser checks that proved a bitmap decoded, a panel opened, or no HTTP
error occurred remain useful integration evidence. They do not prove that the
correct original bitmap was selected, placed, layered, populated, or interacted
with correctly. This audit supersedes those checks only for visual-parity claims.

## Surface coverage

The [surface ledger](surface-ledger.json) covers 44 original or explicitly
non-original families across:

1. boot, intro, shuttle, options, credits, briefings, and multiplayer setup;
2. both command centers, galaxy/GID states, system and sector windows, droids,
   messages, speed controls, and window management;
3. encyclopedia and every object finder, status, production, mission, fleet,
   troop, facility, and headquarters surface;
4. events, battle entry, bombardment, assault, results, and end states;
5. the complete tactical shell, selection, orders, maneuvers, missions, damage,
   options, retreat, and results;
6. multiplayer host, join, ready, sync, chat, disconnect, and error states;
7. extensions, which are excluded from the parity denominator and must not
   replace or obscure original paths.

It now assigns 569 stable baseline cell IDs: 564 required cells and five
excluded extension cells. All required cells remain pending or open because
P46A through P46D did not execute the complete native and browser matrix. All 43 required
families link to at least one of 27
reverse-engineering, resource, runtime-capture, or replacement-removal packages.
The baseline cells are durable requirement identities. Compound requirements
must split into child cells before their surface enters implementation, after
which the applicable execution dimensions form the actual pass denominator.

Space battle is covered explicitly by 106 baseline cells across `TAC-01`
through `TAC-07`. They include Battle Alert entry, both faction HUDs, battle
initialization, capital ships, fighters, task-force assignments, selection,
targeting, damage, contents, navigation and camera controls, maneuvers, tactics,
missions, recovery, withdrawal, simulation and observation, Death Star charge
and attack states, trench-run routing, battle results, and strategic return.
`TAC-08` is separately reserved for the original ground-assault summary and
report flow, which had no live tactical minigame.

Asset-swap families may use exhaustive resource-identity checks only after the
shared layout and rendering rule pass. Every unique composite topology and every
state-changing visual variant still needs inspected screenshots.

## Acceptance contract

### Evidence authority

| Tier | Evidence | Can close parity? |
|------|----------|-------------------|
| A0 | Lossless capture from the unmodified original executable with fixture and input trace | Yes |
| A1 | Original executable/decompiled constructor, paint routine, command mapping, and geometry | Yes |
| A2 | Extracted resource identity plus two corroborating original visual sources | Artwork only |
| A3 | Authenticated original-game screenshot or video without executable mapping | Provisional only |
| A4 | Memory, inference, recreation, or modded media | Discovery only |

Interaction geometry and navigation require A0 or A1. A filename or guessed DLL
description is not sufficient authority.

### Required numerical gates

At the original logical canvas, prefer exact aligned region equality. Otherwise:

- lossless-reference SSIM ≥ 0.995;
- normalized MAE ≤ `1/255`;
- pixels whose channel delta exceeds 2 ≤ 0.5%;
- edge-mask IoU ≥ 0.98;
- element and text-baseline displacement ≤ 1 logical pixel;
- expected versus implemented hotspot IoU ≥ 0.98;
- required aperture occupancy = 100%;
- invented visible elements = 0;
- unknown visible elements = 0;
- unauthorized visible pixel ratio = 0;
- authorized navigation edges exercised = 100%.

Compressed A2/A3 material can provide provisional comparison at SSIM ≥ 0.97,
edge IoU ≥ 0.95, and ≤ 2 logical-pixel alignment. It cannot close geometry or
interaction parity by itself.

### Viewport transform

All original surfaces use one uniform transform from 640×480:

```text
scale = min(viewport_width / 640, viewport_height / 480)
```

Horizontal and vertical scale must match. Paint and hit testing must share the
same transform. Widescreen layouts are letterboxed, not stretched. Initial gates
cover 640×480, 800×600, 1280×960, 1280×800, 1440×900, and 1920×1080 at DPR 1 and
2. Release coverage includes native plus current Chrome, Firefox, and Safari.

### State and input probes

Each control needs every interaction and empty/error state that A0 or A1 evidence
shows the original supports. Enabled actionable hotspots are probed at the center,
corners, four one-pixel inside edges, and four one-pixel outside edges. Every
enabled inside probe fires exactly once; outside and disabled probes fire zero
times. Decorative, display-only, and drag-only regions use their original input
contract instead of an invented click action.

Every listed state has a stable baseline ID such as `TAC-05-C017`. Before
implementation acceptance begins for a surface, compound requirements become
explicit child cells and each applicable dimension cross-product receives a
derived execution ID. The 44-family inventory is not itself the final
measurable execution denominator.

Every navigation case records pre-state, visible response, model mutation, audio,
destination, return behavior, and focus restoration. Clicking a system must reach
the original system-management surface. Opening the current right sidebar fails.

### Resource and runtime trace

Every visible element must resolve to one provenance class:

```text
original_bitmap
original_dynamic_text
original_dynamic_marker
accessibility_overlay_invisible
approved_extension
unknown
invented
```

The runtime trace must record surface/state/element IDs, source binary and hash,
resource type and ID, extracted hash and dimensions, source/destination rectangles,
z-order, and cache result. Missing resources fail closed and log once.

## Screenshot program

The [screenshot ledger](screenshot-ledger.md) and hash manifest retain 370 images.
They establish broad discovery coverage, not final acceptance. The following
must still be captured from an owned English original installation:

- every shuttle control state, plus strategic command-control rest, press
  capture, release/cancel, disabled, mask, and edge probes;
- both faction variants of every GID overlay, legend, and object window;
- every mission dialog with legal, illegal, agent, decoy, result, and abort states;
- Destroy System enabled, confirmation, animation, and report;
- Death Star sabotage success and failure reports;
- tactical healthy, damaged, critical, destroyed, maneuver, tactic, mission,
  Death Star, and trench-run states;
- both victory and defeat endings and rare Dagobah, Jabba, disaster, and story events;
- real two-peer multiplayer host, ready, sync, play, chat, disconnect, and error;
- native-resolution, unscaled English reference pixels for every final comparison.

No missing reference cell may be marked passed.

## Execution order

Each tranche consumes the corresponding `RE-*` work packages in the
[reverse-engineering ledger](reverse-engineering-ledger.md). An empty exported
`FUN_*.c` file is not negative evidence; query the saved Ghidra project before
inventing behavior or geometry.

| Tranche | Scope | Exit |
|---------|-------|------|
| UIP-T01 | Strategic cockpit truth | Both factions have exact shell, galaxy/GID, controls, system navigation, embedded advisors, and markers on native/WASM with zero invented elements. |
| UIP-T02 | System management and command-center destinations | Original sector/system/object windows, reference rail, messages, options, finders, and return paths pass. |
| UIP-T03 | Characters, fleets, production, missions, and research | Every original object/status/action family and required state cell passes. |
| UIP-T04 | Dialogs, events, reports, encyclopedia, and end states | Original report/media routing and all EDATA surfaces pass. |
| UIP-T05 | Space-battle tactical interface | Full space-battle composition, both faction HUDs, ships, fighters, controls, state art, audio, results, strategic return, and Death Star paths pass. Ground assault retains its original report-only flow. |
| UIP-T06 | Multiplayer and release matrix | Complete the shuttle interaction-state matrix, two-peer original flow, cross-browser viewports, resource sweep, accessibility overlays, performance, and all cells pass. |

Documentation, screenshots, and machine-readable cells are updated in the same
atomic commit as each verified implementation tranche.

P46A completes the strategic canvas checkpoint within `UIP-T01`. P46B restores
the six primary controls. P46C and P46D restore the first sector-to-system
journey plus source-mapped core-tab items and bounded scrolling. P47A and P47B
restore the source-aligned galaxy backdrops and default Popular Support GID.
P48 and P50 add the GID command tree and original tiled frame. P51 removes the
unproven hover wash and tests partially covered frame edges. Its interior,
typography, detailed legend, remaining modes and map art, other controls,
exact rail thumbnails, nested object compositions,
commands, uncommon states, replacement message and status surfaces, and the A0
matrix remain open. `CMD-01`, `CMD-03`, `CMD-04`, `CMD-10`, `OBJ-02`,
`UIP-T01`, and project-wide interface parity do not pass.
