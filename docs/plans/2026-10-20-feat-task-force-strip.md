---
title: "Tactical Group Strips: Finish the Task-Force and Fighter-Group Strips"
type: feat
status: active
progress: "15 of 15 beads — the epic waits on or-acc-taskforce-0oy and or-acc-fighter-strip-239, whose sixteen-TIE cells need or-f-sixteen-tie-c43"
epic: or-ep-task-force-strip-5kdd
created: 2026-10-20
updated: 2026-10-22
tags: [tactical, strip, task-force, fighters, beads-pilot]
---

# Tactical group strips: finish the task-force and fighter-group strips

The first plan beaded out under `agent_docs/task-graph.md`, "Plans and
epics". It finishes `or-p-task-force-strip-07t` and the two acceptance
regions waiting on it, `or-acc-taskforce-0oy` and `or-acc-fighter-strip-239`.

## Goal

The tactical window's two top strips look and act as the original's, on
both sides: the task-force strip (header and slots 1–8) and the
fighter-group strip (header and the Red, Blue, Green and Gold slots). A
slot click, a number key and the slot's look match the original, and the
C6 compare passes both regions in every listed cell.

## What the original does

`ghidra/notes/tactical-group-strips.md` is the reference. In short:

- Both strips are child buttons of the tactical window (ctor
  `FUN_0059a8d0`); their geometry is the same for both sides, only the
  header bitmap differs (`FUN_00595c40`).
- Task-force header `0xad01` at (9, 1), 49×19, bitmap 1001 (Alliance) or
  1004 (Empire); slots `0xad02`–`0xad09` at x = 60 + 29i, y 2, 25×17,
  `FUN_005c52e0` toggle buttons with bitmaps 1005 / 1006 (pressed) and the
  unassigned pair 1007 / 1007 (`FUN_0059be80.c:55-56`, `FUN_005c52e0.c:17-39`).
  Slots hit-test their rectangle and the headers their bitmap
  (`FUN_00602150.c:115-116`, `FUN_006035f0.c:24-38`).
- Fighter header `0xad0a` at (296, 1), bitmap 1008 or 1010; slots
  `0xad0b`–`0xad0e` at x = 347 + 29i, y 2, 25×15 `FUN_00602150` buttons,
  bitmaps 1012+i / 1016+i / 1020, the 25×17 bitmaps clipped to 15 rows
  (`FUN_0059c030`).
- A click goes through `FUN_0059b130`: task-force slots to `FUN_0059b450`
  (an assigned slot posts command 2+i and releases the others; an
  unassigned one acts only in modes `0xe`/`0x10`, `FUN_0059f6e0`, setting
  its hover text `FUN_00600c40`), fighter slots to `FUN_0059b620` (an
  existing group posts command `0xb`+i and releases the others). A second
  click on the selected slot returns to the base mode and clears the
  selection (`FUN_0059fd40.c:24-88`).
- Keys: `FUN_0059b730` maps 1–8, on key-down, to release every slot of both
  strips and press task-force slot i (`FUN_0059ca90`); the key-up branch has
  no case for them. F1–F8 and F9–F12 post camera mode `0xb` with the group
  index when the group exists (`FUN_005c2990`), and post no selection.
- Fighter-slot presence follows the controller's `+0x154` table, cleared by
  `FUN_005a0320` (orders 2 and 6, or a group with no squadron left) and
  given back by `FUN_0059f680` after a trench-run success.

## Where the port stands

`crates/rebellion-render/src/tactical_view.rs` (`TASK_FORCE_BUTTONS`,
`FIGHTER_GROUP_BUTTONS`, the header points and the strip draw) has the
traced geometry; a native click at (360, 10) selects Red Group on both
sides. C6 run 2026-10-14T18-52-43-758Z: displacement 0 in both regions;
fighter-group strip SSIM 0.997 with 17 edge or corner pixels differing;
task-force strip SSIM 0.942 with 3.9% of pixels changed.

## What this plan must settle and port

1. **Task-force number labels.** A0 draws them much larger than the port's
   13 px label. Trace where the caption's font, size and colour come from
   (`FUN_00600c40` and the button's paint) and port them, both sides.
2. **Slot hit test.** The port tests a bitmap's opaque pixels; the
   original's buttons take the whole rectangle (`FUN_00602150`,
   `FUN_005c52e0`). Port the rectangle test for both strips.
3. **Selection behaviour.** Slot clicks and keys 1–8 (and F1–F8 if the port
   has the camera mode) select and release as `FUN_0059b450`,
   `FUN_0059b620` and `FUN_0059b730` do, including the unassigned-slot rule
   and what modes `0xe` and `0x10` are (open in the note).
4. **Fighter-strip edge pixels.** Explain the 17 differing pixels (the
   button bitmaps' transparent edge over the strip background is the
   working guess) and fix or justify them.
5. **Acceptance.** `or-acc-taskforce-0oy` (8 cells) and
   `or-acc-fighter-strip-239` (3 cells) pass
   `tools/interface-parity/tactical-a0-compare.mjs --negative-controls`.

## Constraints

- AGENTS.md and CLAUDE.md apply: both factions, cited original behaviour,
  red-first tests in a Mini clone, `cargo mutants` on touched functions,
  Mini-only cargo, silent runs, the browser only through the
  interface-parity gates, no commit before Tom's word.
- Unit tests are expected; the parity-QA "no new unit tests" note on
  `07t` does not apply to this epic.
- No invented behaviour: anything the trace does not settle becomes a
  question bead.

## Decisions

- **Slot-label glyphs** (`or-ep-task-force-strip-5kdd.5`, Tom, 2026-10-21):
  the task-force numbers stay in the port's smoothed Liberation Sans, at
  font 4's size and the original's centred place; Tom takes Liberation Sans
  over GDI Arial's glyphs. C6 masks each caption cell under
  `or-dec-c6-text-masks-rue` (`task-force-caption-1`..`8`), and the label
  unit tests check the digit, its place and its size. The browser gate's
  `labelMask` gives way to the caption rectangle that `FUN_006026e0`
  computes. Longer text regions, such as the fighter status
  panel, follow `or-dec-c6-text-masks-rue`, which Tom approved the same day:
  measured text-box masks, each citing that decision, plus a separate check
  that the strings, positions and em size match A0.

## Out of scope

The tactical camera and holocube (done in the orbital-strike plan), the
order buttons below the strips, and the 3D scene.

## Completion

Beaded out into `or-ep-task-force-strip-5kdd`; progress is the epic's
closed beads on `tasks/INDEX.md`.
