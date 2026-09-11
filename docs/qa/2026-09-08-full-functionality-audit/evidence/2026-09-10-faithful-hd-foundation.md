---
title: "Faithful-HD Profile and Manifest Foundation"
description: "P45A render-profile, provenance, fallback, and browser-regression evidence"
category: qa
created: 2026-09-10
updated: 2026-09-10
tags: [qa, hd, bitmap, manifest, wasm, astra]
---

# Faithful-HD Profile and Manifest Foundation

P45A separates exact interface parity from optional higher-resolution artwork.
`original-parity` is the default and always decodes original resources with
nearest-neighbor sampling. Native `faithful-hd` is an explicit opt-in. Browser
builds remain fail-closed on original pixels until optional HD packs are
implemented.

The deterministic generator preserves indexed palettes and records the source
resource, source and palette digests, route and configuration, generator,
output digest, and image metadata. Approval requires a named reviewer, evidence,
and a passing human-review gate. Verification rejects changed source or output
bytes, incomplete review records, altered route or image metadata, mismatched
asset identity, and attempts to reuse an output from a different source tree.

The native loader verifies both source and output digests, then decodes the
exact output byte buffer it verified. A changed source or output falls back to
the current original bitmap. The fallback does not retain stale source bytes.

## Verification

| Gate | Result |
|---|---|
| Workspace tests | 592 passed, 0 failed, 20 ignored including 3 doc tests |
| Faithful-HD Python tests | 10 passed, 0 failed |
| Render tests | 100 passed, 0 failed |
| Native approval, digest, and fallback probes | 20 passed, 0 failed |
| Adversarial manifest probes | 21 passed, 0 failed |
| Source-switch regression | passed; stale output rejected unless explicitly regenerated |
| Native workspace check | pass |
| WASM app check | pass |
| JSON, diff, and scoped `bmp_cache` formatting | pass |
| Astra medium review | `ready_to_commit true`; no P0–P3 findings |

Astra medium ran four muted Chromium boots covering both factions at 640×480
DPR 1 and 960×720 DPR 2. It passed 32 of 32 menu selections, observed exactly
four requests per boot, and found no HD request, missing bitmap, console error,
network error, or WebGL error. These captures verify that P45A does not alter
the original-parity browser presentation.

Runtime artifacts:

- WASM SHA-256: `bc2f4d59cfbf256d3e72aef8914385e0058fa9b66d3bc4a3dbf9f5b2cefd1f4c`
- ORPK SHA-256: `1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862`
- ORPK contents: 52 game-data entries, 2,303 original bitmaps, 3,988 advisor
  frames, and 5 audio entries. It contains no HD keys.

P45A establishes the fail-closed foundation only. Complete-family learned-model
comparison, protected masks and temporal gates, optional browser packs, and
native/cross-browser faithful-HD visual acceptance remain open under P45B–P45E.
