# Open Rebellion Interface Parity

This folder is the source of truth for restoring every visible surface from
*Star Wars: Rebellion* and *Star Wars: Supremacy*. The target is the original
bitmap-driven interface. Replacement panels, invented controls, approximate
geometry, and unverified asset mappings do not count as parity.

## Start here

- Read the [audit overview](index.md) for the current result and definition of
  100% interface parity.
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

The first recovered implementation tranche now packages and renders the four
original faction-advisor idle runs. See the
[P44 evidence](../2026-09-08-full-functionality-audit/evidence/2026-09-10-authentic-droid-advisors.md).
Full `CMD-07` action, voice, chrome, and shell integration remains open.

P46A now verifies the exact centered 640x480 strategic canvas, faction shell
crop, recovered galaxy apertures, and shared browser transform. See the
[P46A evidence](evidence/2026-09-11-strategic-shell-canvas.md). `CMD-01` and
`UIP-T01` remain open because the authentic controls, GID and map art,
window-reference rail, original system windows, and replacement surfaces are
not complete.
