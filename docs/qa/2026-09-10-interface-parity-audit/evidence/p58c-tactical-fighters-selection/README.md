# P58C tactical fighters and selection evidence

This directory preserves the browser evidence for P58C. The bundle renders
source-mapped type-303 fighter groups and aligns capital-ship selection and
targeting with the projected source mesh bounds.

## Inventory

- `full-summary.json`: the 36-case browser-gate summary.
- `alliance-*` and `empire-*`: production captures and result records at
  640x480 and 1280x800 letterboxed.
- `alliance-control-*` and `empire-control-*`: paired negative controls with
  mapped capital and fighter rendering suppressed.
- `alliance-deselection-640x480.png`, `alliance-selection-640x480.png`, and
  `alliance-target-640x480.png`: the retained Alliance interaction journey.
- `empire-deselection-640x480.png`, `empire-selection-640x480.png`, and
  `empire-target-640x480.png`: the retained Imperial interaction journey.
- `astra-browser-acceptance.json`: the Astra medium artifact review.

The production records identify close, far, and indicator fighter resources,
per-object framebuffer probes, source projections, and distinct interaction
hashes. Every run is muted, stable across two paused frames, limited to four
HTTP 200 startup requests, free of recorded runtime errors, and marked closed.

These are A1 implementation artifacts. They do not replace lossless, owned A0
captures from the original game. All 106 strict tactical cells remain open.
