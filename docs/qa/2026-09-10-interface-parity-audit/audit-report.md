---
title: "Original Interface Parity Audit Report"
description: "Evidence-backed diagnosis and execution plan for complete bitmap-driven UI parity"
category: qa
created: 2026-09-10
updated: 2026-09-10
tags: [qa, interface, parity, bitmap, strategy, tactical, multiplayer]
---

# Original Interface Parity Audit Report

## Conclusion

The user-visible campaign interface currently fails parity with the 1998
Coolhand Interactive and LucasArts release of *Star Wars: Rebellion*, titled
*Supremacy* in the UK. The shuttle main menu passes its scoped implementation
gate, but remains partial here until every original interaction-state cell has
A0 or A1 evidence. Entering a campaign reaches a mostly custom macroquad/egui
reconstruction.

This explains the observed symptoms:

- The top and bottom text-button rows are temporary replacements. The source
  says the original command-to-sequence mapping is unresolved.
- The galaxy is drawn from a dark fill, vector circles, text labels, sector
  outlines, and 5×5 facility squares instead of the original starfield,
  Galactic Information Display rules, and bitmap marker families.
- Selecting a system opens a custom right `egui::SidePanel`. The original opens
  modeless sector/system windows with illustrated systems, tabbed contents,
  support/resources, and bitmap actions.
- The four original advisor idle runs now render in native and WASM, but
  replacement controls overlap their lower apertures. Original action, voice,
  and chrome behavior remains unimplemented.
- Several cockpit apertures remain blank because the original window-reference,
  advisor, message, and command compositions have not been implemented.

The project can retain its verified simulation behavior while replacing every
visible reconstruction with the original interface contract.

## Source evidence

The local source inspection establishes the implementation causes:

| Finding | Repository evidence | Original evidence |
|---------|---------------------|-------------------|
| Synthetic strategy controls | `crates/rebellion-render/src/cockpit.rs:281` | Manual pp. 60–66 and official Steam command-center captures |
| Synthetic galaxy and glyphs | `crates/rebellion-render/src/lib.rs:145`, `:253`, `:388` | Manual pp. 66–73 and official Steam faction captures |
| Invented system sidebar | `crates/rebellion-render/src/lib.rs:636` | Manual pp. 67–68, 97–100, and 122–124 |
| Partial browser droids | `crates/rebellion-render/src/advisor.rs` | Manual pp. 20–21 and 73–79; official Steam faction captures; [P44 evidence](../2026-09-08-full-functionality-audit/evidence/2026-09-10-authentic-droid-advisors.md) |
| Blank browser encyclopedia art | `crates/rebellion-render/src/encyclopedia.rs:486` | Manual pp. 71–72 and 192 original EDATA entries |
| Incomplete browser asset pack | `scripts/build-runtime-pack.py:29` | Original ALSPRITE, EMSPRITE, ALBRIEF, EMBRIEF, REBDLOG, and EDATA families |
| Invented live ground-combat screen | `crates/rebellion-render/src/ground_combat.rs:202` | Manual pp. 119–121 assault summaries and reports |
| Replacement tactical composition | `crates/rebellion-render/src/tactical_view.rs:773` | Manual pp. 139–150 and original TACTICAL resources |

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
These discoveries narrow implementation; they do not replace original-runtime
visual acceptance.

## Immediate findings

| ID | Severity | Finding | Status |
|----|----------|---------|--------|
| UIP-F-001 | P0 | Campaign cockpit shell is stretched and overlaid with replacement text controls and blank apertures. | fail |
| UIP-F-002 | P0 | Galaxy, stars, system markers, sector hulls, and facility indicators use synthetic primitives and incomplete rules. | fail |
| UIP-F-003 | P0 | System selection routes to an invented right sidebar instead of the original modeless sector/system surface. | fail |
| UIP-F-004 | P0 | Authentic advisor idle runs render for both factions, but the replacement shell overlaps them and complete action/voice/chrome behavior is absent. | partial |
| UIP-F-005 | P0 | Runtime pack v2 includes ALSPRITE and EMSPRITE BMP/type-302 content; ALBRIEF, EMBRIEF, REBDLOG, EDATA, action controls, and voice remain omitted. | partial |
| UIP-F-006 | P1 | WASM encyclopedia image loading always returns no texture. | fail |
| UIP-F-007 | P1 | Finders, production, missions, messages, options, and object status use replacement layouts or are absent. | fail |
| UIP-F-008 | P1 | Battle Alert, assault/bombardment reports, and battle-result routing are absent or bypassed. | fail |
| UIP-F-009 | P1 | Tactical HUD uses partial original art inside a replacement composition. | fail |
| UIP-F-010 | P1 | The custom live ground-combat screen has no original counterpart. | fail |
| UIP-F-011 | P1 | Original modeless-window minimize, restore, close, focus, and reference-rail behavior is absent. | fail |
| UIP-F-012 | P1 | Authoritative original captures are still missing for rare and transient states. | partial |

Existing browser checks that proved a bitmap decoded, a panel opened, or no HTTP
error occurred remain useful integration evidence. They do not prove that the
correct original bitmap was selected, placed, layered, populated, or interacted
with correctly. This audit supersedes those checks only for visual-parity claims.

## Surface coverage

The [surface ledger](surface-ledger.json) covers 43 original or explicitly
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

It now assigns 549 stable baseline cell IDs: 544 required cells and five
excluded extension cells. All 42 required families link to at least one of 27
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
derived execution ID. The 43-family inventory is not itself the final
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

- every shuttle and cockpit control hover, pressed, selected, and disabled state;
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
