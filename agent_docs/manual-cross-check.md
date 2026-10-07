---
title: "Manual Cross-Check"
description: "Read the 1998 manual before and after each interface feature, and settle every claim it makes against the REBEXE.EXE trace"
category: "agent-docs"
created: 2026-10-06
updated: 2026-10-06
tags: [manual, interface, audit, ghidra, faction-wars]
---

# Manual Cross-Check

A Ghidra trace says what the executable does. The manual says what the
player is promised: the windows, controls, gestures, menus and names they
meet. A port can match every traced function and still miss an interaction
that no trace was asked about. Sector-window icons showed this on
2026-10-06: the port answered only a double click, while the manual
(p. 109, Fig. 3.50) gives the mission icon a right-click menu, and the trace
then confirmed menus for all four icons (`ghidra/notes/sector-icon-menus.md`).

## When

1. **Before a feature.** Find every manual passage on it and list each claim
   the player could observe: named controls and fields, the gesture that
   opens or acts (click, double click, right click, drag, Ctrl-drag, keys),
   the menu items, the confirmation, the names and icons shown.
2. **After a feature.** Reread the same passages and settle every claim.

## Settling a claim

| Status | Meaning |
|---|---|
| Confirmed | A trace agrees; cite the `FUN_` and the note. |
| Contradicted | The trace disagrees. The executable decides the behavior; record both readings in the note. |
| Unresolved | No trace yet. Do not implement it as fact: tag it `hyp:` or leave it open. |

Then rate the port against each claim: **Present** (element and interaction
both exist), **Partial** (exists but differs), **Missing**, or **Disabled**
(visible but unavailable, with a `port:` tag saying why).

Say how far you read: a claim about code is `read-in-full`, `spot-checked`
or `inferred`, and "the original never does X" needs `read-in-full` of the
handler that would do it.

## Reading the manual

- `docs/reference/campaign-history/archive/star-wars-rebellion-manual.md`
  is the grep-able Markdown transcription (Mistral OCR 4.1, 2026-10-06).
  Each page carries a `<!-- pdf-page: N | printed: M -->` anchor and a
  `## p. M` heading; figures are blockquote annotations. Use this for text
  search and citation lookup.
- `docs/reference/campaign-history/archive/star-wars-rebellion-manual.pdf`
  is the scanned original and remains the authority for figures, layout,
  and visual details. Read pages directly (`Read` with `pages`) when you
  need to verify a screenshot or callout leader line.
- Printed page numbers run ahead of PDF pages; the offset varies by chapter
  (see the YAML `page_offset_table` in the Markdown front matter). In
  chapter 3, printed p. 123 is PDF page 121 (offset +2); check the footer.
- Cite the printed page and figure: "manual p. 123", "Fig. 3.70".
- A figure's callouts are claims too (Fig. 3.70: "Open Fleet window and
  Sector window for selected fleet").

## Where results go

- The behavior and its trace: the `ghidra/notes/` note for the feature.
- Each claim's status: the audit's manual cross-check cells for the feature.
- Intentional departures: the audit's known-deviations log.

## Peer project

Faction Wars by TeeJS (<https://github.com/TeeJS/faction-wars>), a Godot
re-implementation, reads the manual page by page (`GAMEPLAY.md`) and keeps
per-window manual checklists (`docs/window-checklists.md`). This workflow,
its Present/Partial/Missing ratings and its read-depth labels follow their
practice. Faction Wars has no licence: cite it by file, section and commit
permalink, paraphrase its findings, and never copy its text, tables or code.
Treat its readings as leads to confirm against the trace, not as evidence;
it also departs from the original on purpose (it lets sector windows move,
for one).
