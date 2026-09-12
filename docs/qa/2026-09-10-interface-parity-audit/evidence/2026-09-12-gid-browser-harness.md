# GID browser harness and provisional menu checkpoint

Status: implementation checkpoint, not CMD-02 acceptance. All 29 baseline
requirements and nine additional recovered filter variants have deterministic
fixture entries, yielding 152 faction/viewport executions. The muted Chrome
matrix passed 152/152 with four successful startup requests, no browser or
missing-asset diagnostics, two stable captured frames, and 152 closed browser
processes. Full console logs and 840 interaction-comparison slots were
retained. All 992 comparison slots remain unbaselined against an
accepted original-executable screenshot. Strict CMD-02 acceptance stays 0.

The [test-only harness](../../../../tools/interface-parity/README.md) builds a
separate fixture-enabled WASM artifact. The production WASM and HTML exclusion
check passes, and the fixture bridge does not ship in the packaged browser
build. Original-capture authority and implementation regression goldens are
separate, with an original-reference hash required before creating a golden.
Raw screenshots and complete browser console/cache logs remain in ignored
local artifacts; they are not copied into this document. Menu, hover, legend,
and pan/zoom captures have separate regression-comparison slots, but none can
pass a pixel-fidelity gate until its original source is accepted and its
implementation baseline is reviewed.

The faction shell, GID control, compact legend, display-on/off background,
and native marker bitmap families follow STRATEGY resources and the recovered
`FUN_004511e0` menu command tree. The root menu is bounded to the original
640×480 position and uses the extracted faction category, arrow, check, and
filter icons. The harness checks the interior and outside edge, press and
hover frames, root and leaf routing, compact-legend artwork, and pan/zoom at both
viewport sizes. It caught and fixed an Escape route that had incorrectly
returned from the GID popup to the shuttle menu.

The source-backed root-menu layout is only a first implementation. Independent
review found unsupported special-marker assignments and an invented enlarged
legend; neither ships in this checkpoint. Exact non-support filter metrics,
unusual marker identities, expanded-legend composition,
keyboard focus, native rendering, original-executable screenshots for each
filter/state, and cross-browser evidence still need independent adjudication.
Do not count a green harness result as exact bitmap/interface parity. The
[surface ledger](../surface-ledger.json), [screenshot ledger](../screenshot-ledger.md),
and [acceleration sidecar](../../../plans/2026-09-11-tooling-interface-parity-acceleration-sidecar.md)
retain these open gates.
