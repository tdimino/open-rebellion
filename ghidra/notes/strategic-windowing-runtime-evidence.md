---
title: "Strategic Windowing Runtime Evidence"
description: "Sanitized, incomplete ledger of original-runtime observations supporting the strategic windowing contract"
category: "ghidra"
status: incomplete
created: 2026-10-10
updated: 2026-10-10
---

# Strategic Windowing Runtime Evidence

This ledger records only the original-runtime observations that can currently
be reviewed from the repository. It is supporting evidence for
[`strategic-windowing-toolkit.md`](strategic-windowing-toolkit.md), not final
interface acceptance.

The executable observed under Wine 9.0 was the owned `REBEXE.EXE` with
SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
The executable, original assets, Wine prefix, and proprietary captures are not
committed. A former machine-local capture manifest is not available in this
checkout, so observations without retained action logs and artifact hashes
cannot close an audit cell.

## Retained observations

| Scenario | Side | Observation | Retained evidence | Acceptance effect |
|---|---|---|---|---|
| Encyclopedia above the strategic view | Alliance and Empire | Normal outside clicks did not dismiss the Encyclopedia or activate known-good underlying System Finder controls. F2 was suppressed. The focused title field accepted `tallon` and selected Talon Karrde without invoking cockpit shortcuts. Escape closed the Encyclopedia; the next F2 or cockpit click worked. | Written observation only; exact action log, screenshots, and hashes are unavailable | Corroborates the proposed blocking and no-fallthrough behavior. Does not close parity cells. |
| Two Sector windows with a dialog above them | Alliance | Two Sector windows and a dialog above them were observed together. | Written observation only; exact action sequence, screenshots, and hashes are unavailable | Corroborates that the strategic view can retain multiple modeless Sector windows below a dialog. Does not establish cross-family ordering or close parity cells. |

## Required capture queue

Each row remains open until the record includes faction, starting state,
exact actions, expected and observed results, screenshot paths, artifact
hashes, executable hash, and environment details.

| Required scenario | Status | What must be resolved |
|---|---|---|
| Cross-family modeless raise and restoration | Missing | Order among Sector, System/Manufacturing, Fleet, Defenses, and Missions detail windows |
| Status | Missing | Outside-pointer behavior, keyboard ownership, close route, and restoration |
| Battle Alert | Missing | Whole-strategic-view versus owner-only pointer scope |
| Build Selection | Missing | Duplicate-open behavior, pointer scope, close route, and restoration |
| Mission creation | Missing | Pointer scope, keyboard ownership, close route, and restoration |
| Move confirmation | Missing | Pointer scope, close route, press/release consumption, and restoration |
| Battle Results / Summary | Missing | Pointer and keyboard scope, dismissal, and restoration |
| Parent/child transitions | Missing | Owner closure, descendant teardown, and post-close focus |
| Both-faction repetition | Partial | Repeat every reachable policy-changing scenario for Alliance and Empire |

## Evidence rule

Static decompilation and the original manual may establish structure or a
minimum policy set. A live observation may corroborate visible behavior. No
runtime row closes an interface-audit cell unless its reproducible action
sequence and artifact hashes are retained with the applicable native and port
captures. Conservative `port:` behavior may protect users while a row remains
open, but it must not be described as recovered parity.
