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
`CMD-01`, `CMD-03`, `CMD-04`, `CMD-10`, and `UIP-T01` remain open because the
full control matrix, remaining GID and map art, nested object compositions and commands,
exact rail thumbnails, original destination windows, and replacement surfaces
are not complete.
