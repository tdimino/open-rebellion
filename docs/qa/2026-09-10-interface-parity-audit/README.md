# Open Rebellion Interface Parity

This folder is the source of truth for restoring every visible surface from
*Star Wars: Rebellion* and *Star Wars: Supremacy*. The target is the original
bitmap-driven interface. Replacement panels, invented controls, approximate
geometry, and unverified asset mappings do not count as parity.

## Start here

- Read the [audit overview](index.md) for the current result and definition of
  100% interface parity.
- Follow the current
  [batched execution plan](../../plans/2026-09-11-feat-batched-interface-parity-plan.md)
  for bundle boundaries, verification cadence, Astra review, and commit gates.
- Use its
  [acceleration sidecar](../../plans/2026-09-11-tooling-interface-parity-acceleration-sidecar.md)
  for the permanent muted harness, deterministic fixtures, visual comparisons,
  generated summaries, and complete-family GID pilot.
- Choose a bounded surface and state from the
  [machine-readable surface ledger](surface-ledger.json).
- Use the [reverse-engineering ledger](reverse-engineering-ledger.md) to find
  its original resources, executable paths, and named `RE-*` work package.
- Check the [screenshot ledger](screenshot-ledger.md) for available reference
  captures and known evidence gaps.
- Read the [audit report](audit-report.md) for findings, thresholds, and the
  six-tranche implementation order.
- Review the [P46A strategic shell evidence](evidence/2026-09-11-strategic-shell-canvas.md)
  for the verified canvas, aperture, and browser-transform checkpoint.
- Review the [P46B strategic-control evidence](evidence/2026-09-11-strategic-command-controls.md)
  for the recovered six-control geometry, bitmap, input, and command contract.
- Review the [UIP-B01 strategic-navigation evidence](evidence/2026-09-11-strategic-window-navigation.md)
  for the sector, system-shell, tab, and rail checkpoint and its remaining
  item-level boundaries.
- Review the [P46D detailed-system evidence](evidence/2026-09-11-detailed-system-tab-items.md)
  for source-mapped item miniatures, bounded scrolling, selection, coarse
  current-intelligence gating, and the remaining object-window boundaries.
- Review the [P47A galaxy-backdrop evidence](evidence/2026-09-11-authored-galaxy-backdrop.md)
  for STRATEGY 902 identity, canvas-origin placement, native/WASM transport,
  browser captures, and the remaining GID boundaries.
- Review the [P47B Popular Support evidence](evidence/2026-09-11-popular-support-gid.md)
  for the active 903 predicate, exact faction GID control, compact legend,
  original marker families, and recovered support thresholds.
- Review the [P48 GID browser-harness evidence](evidence/2026-09-12-gid-browser-harness.md)
  for the code-built menu, isolated fixtures, browser matrix, and remaining
  original-reference and filter-predicate gaps.
- Review the [P49 Message Index rail evidence](evidence/2026-09-12-message-index-rail.md)
  for exact faction resting bitmaps and the still-open index behavior.
- Review the [P50 GID menu-frame evidence](evidence/2026-09-12-gid-menu-frame.md)
  for original frame tiles, source-pixel checks, and the remaining menu gaps.
- Review the [P51 GID hover and occlusion evidence](evidence/2026-09-12-gid-hover-and-occlusion.md)
  for removal of the unsupported wash and visible-border checks under windows.

## Suggested contribution lanes

| Lane | Best starting record |
|------|----------------------|
| Original resource extraction or Ghidra analysis | An open `RE-*` package in [reverse-engineering-ledger.md](reverse-engineering-ledger.md) |
| Interface implementation | A required family or cell in [surface-ledger.json](surface-ledger.json) |
| Screenshot and provenance research | A missing state in [screenshot-ledger.md](screenshot-ledger.md) |
| Automated visual verification | The gates and thresholds in [audit-report.md](audit-report.md) |
| Space-battle interface | `TAC-01` through `TAC-07`, covering 106 baseline cells |

## Acceptance rule

A required cell passes only when it has authoritative original evidence, exact
resource identity, matching composition and interaction geometry, native and
packaged-browser captures, clean diagnostics, and correct audio where
applicable. Update the relevant Markdown and JSON records in the same atomic
commit as the verified implementation.

Do not commit copyrighted game assets. Extraction and runtime-pack tooling must
stage them from a contributor-owned installation.

## Progress snapshot

<!-- interface-parity-status:start -->
Required interface families: 43. Complete: 0. Partial: 9. Failing: 34. Blocked: 0. Strictly accepted cells: 0/564.
The current CMD-02 GID catalog has 38 scenarios (29 baseline cells and nine additional native filter variants) and 152 faction/viewport executions. Its strict original-evidence and cross-browser gate remains open.
<!-- interface-parity-status:end -->

The practical implementation estimate is approximately 20 to 23% tackled and
77 to 80% remaining. Formally, the ledger has 43 required families: 0 complete,
9 partial, and 34 failing. All 564 required cells remain pending until their
complete evidence and execution matrices pass. The practical estimate guides
bundle planning; it does not replace strict acceptance.

The first recovered implementation tranche now packages and renders the four
original faction-advisor idle runs. See the
[P44 evidence](../2026-09-08-full-functionality-audit/evidence/2026-09-10-authentic-droid-advisors.md).
Full `CMD-07` action, voice, chrome, and shell integration remains open.

P46A now verifies the exact centered 640x480 strategic canvas, faction shell
crop, recovered galaxy apertures, and shared browser transform. See the
[P46A evidence](evidence/2026-09-11-strategic-shell-canvas.md). P46B restores
the six primary faction controls and removes their replacement text strip. See
the [P46B evidence](evidence/2026-09-11-strategic-command-controls.md).
UIP-B01 now replaces the invented sidebar with first-pass original sector and
system shells and a working 12-slot rail lifecycle for both factions. See the
[navigation evidence](evidence/2026-09-11-strategic-window-navigation.md).
P46D populates all six system tabs with source-mapped GOKRES miniatures,
displayed entity labels, recovered scrollbar art, bounded interaction,
preserved selection, and coarse current-intelligence gating.
See the [detailed-system evidence](evidence/2026-09-11-detailed-system-tab-items.md).
P47A replaces the strategic map's flat fill with the source-aligned bright
STRATEGY 902 galaxy in native and packaged WASM. See the
[galaxy-backdrop evidence](evidence/2026-09-11-authored-galaxy-backdrop.md).
P47B restores the default active Popular Support baseline with STRATEGY 903,
the exact faction GID control, compact legend, and native marker families. It
also withholds the prior synthetic map overlays from this parity view. See the
[Popular Support evidence](evidence/2026-09-11-popular-support-gid.md).
P48 adds the original-command GID menu tree and a test-only muted browser
matrix. The [P48 evidence](evidence/2026-09-12-gid-browser-harness.md) keeps
non-support filter rules, expanded legend, special state overlays, and
original-executable capture acceptance open.
P49 restores the nine original Message Index rail icons per faction. Their
resting pixels match the source BMPs, but the index window and unread states
remain open. See the [rail evidence](evidence/2026-09-12-message-index-rail.md).
P50 restores the eight original GID frame tiles to the root and submenu and
removes the invented fade. Its native-size root border matches the source BMPs
where unobscured. See the [frame evidence](evidence/2026-09-12-gid-menu-frame.md).
P51 removes a provisional hover wash and extends those checks to the visible
frame border beneath a foreground system window. Its
[evidence](evidence/2026-09-12-gid-hover-and-occlusion.md) keeps original hover
and focus states open.
`CMD-01`, `CMD-03`, `CMD-04`, `CMD-10`, and `UIP-T01` remain open because the
full control matrix, remaining GID modes and map art, nested object
compositions and commands, exact rail thumbnails, original destination
windows, and replacement surfaces are not complete.
