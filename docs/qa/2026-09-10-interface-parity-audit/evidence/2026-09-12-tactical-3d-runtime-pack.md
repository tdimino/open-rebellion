---
title: "P55 Tactical 3D Runtime Pack Evidence"
description: "Deterministic decoding and independent verification of the original tactical mesh, indexed-image, and palette corpus"
category: qa
created: 2026-09-12
updated: 2026-09-12
tags: [interface, parity, tactical, assets, directx, verification]
---

# P55 tactical 3D runtime pack

P55 converts the complete raw corpus staged by
[P54](2026-09-12-tactical-3d-staging.md) into a bounded,
content-addressed runtime store. No proprietary source or generated object is
tracked. This is a converter checkpoint, not visual acceptance. All 106
`TAC-01` through `TAC-07` cells remain open.

## Implemented contract

- Parse the original `xof 0302bin 0032` and `xof 0303bin 0032` token streams
  without a production dependency.
- Validate finite positions, normals, UVs, bounds, material indices, face and
  normal topology, strings, counts, and bounded aggregate input.
- Emit versioned `.mesh` objects with material data, deterministic triangle
  chunks, `u16` indices, bounds, provenance, and normal-source mode.
- Decode type-303 RLE to original indexed pixels. A zero run byte represents
  256 pixels.
- Preserve all 27 RGB palettes. Planet images `5501` through `5527` bind
  palettes `5531` through `5557`; the other 343 indexed resources retain the
  active battle-palette rule instead of baking the editor-preview palette.
- Resolve all 59 binary-X texture names uniquely and in the same language to
  named type-303 resources using the original Windows case-insensitive rule.
- Restrict derived normals to original resource `2082` and four-byte RLE tails
  to original resources `4200` through `4204`, each by DLL, resource, language,
  and source hash.
- Preflight aggregate decoded output before RLE expansion or mesh
  triangulation, then enforce per-object and exact aggregate bounds again.
- Verify every generated object hash, internal header, count, index, finite
  float, identity, complete texture/palette relationship graph, source
  exception, and aggregate-size bound without rereading raw source objects.
- Derive planet-palette requirements from numeric resource identity so a
  consistently rewritten manifest and texture object cannot downgrade a
  planet to the active battle-palette rule.

The parser follows the official
[Microsoft binary X encoding](https://learn.microsoft.com/en-us/windows/win32/direct3d9/binary-encoding).
[Assimp's X parser](https://github.com/assimp/assimp/blob/master/code/AssetLib/X/XFileParser.cpp)
is an independent implementation oracle, not a runtime dependency.

## Owned-source result

The source is the same 7,843,840-byte owned `TACTICAL.DLL` used by P54, with
SHA-256 `db12cdcfb6c85cec572a425307d2ad617bbc850027657424c3cc1f61c6ec275a`.

| Check | Result |
| --- | --- |
| Meshes | 87 converted; 3,090 source faces and 3,090 runtime triangles |
| Source normals | 86 meshes |
| Source-bound derived normals | resource `2082` only |
| Textured meshes | 59; all embedded names resolve uniquely |
| Type-303 resources | 370 indexed images plus 27 RGB palettes |
| Palette routing | 27 `planet_pair`; 343 `battle_active` |
| Source-bound trailing data | resources `4200` through `4204` only |
| Referenced runtime objects | 484; 4,173,028 bytes |
| Raw manifest | 168,624 bytes; SHA-256 `703c1b5040f6daa525ea129de5a22940fd2e62e837b7fd184540b7fc210b8665` |
| Runtime manifest | 266,669 bytes; SHA-256 `dc9d64e0e75bbe775dbd1aa6e726742ca75695d669d07a40cc42b66badc3dea1` |
| Repeat conversion | 0 written, 484 unchanged |

Resource `2560` retains 48 indexed source positions, 62 triangles, one
material, source normals, original bounds, and the `sdesti52.bmp` binding to
named resource `SDESTI52.BMP`.

## Independent oracle

The reproducible `--tactical-3d-assimp-oracle` gate ran local Assimp 6.0.5 raw
dumps against all 87 original files:

- all triangle connectivity matched after Assimp's winding inversion;
- every Assimp comparison was bound byte-for-byte to the retained,
  content-addressed runtime mesh generated from that same source object;
- every expanded position matched after Assimp's Z handedness inversion;
- every source normal matched after the same Z inversion; resource `2082`,
  which has no source normals, retained the separately tested derived mode;
- every UV matched after Assimp's `v = 1 - v` transform;
- diffuse, specular, emissive, shininess, material assignment, and all 59
  texture filenames matched;
- all numeric comparisons used a `0.00001` tolerance for ASSXML's six-decimal
  representation.

Assimp's default post-processing split degenerate triangles in resources
`2140`, `2530`, `2531`, and `2532` into separate line and triangle meshes.
Repeating the oracle with `assimp info -r` proved that this was a
post-processing transform, not a source-parser discrepancy.

## Verification

```text
env GOCACHE=/private/tmp/open-rebellion-go-cache go test -count=1 ./tools/stage-ui-assets
  PASS: 78 tests, 0 failures

env GOCACHE=/private/tmp/open-rebellion-go-cache go test -race -count=1 ./tools/stage-ui-assets
  PASS

env GOCACHE=/private/tmp/open-rebellion-go-cache go vet ./tools/stage-ui-assets
  PASS

env GOCACHE=/private/tmp/open-rebellion-go-cache GOMODCACHE=/private/tmp/open-rebellion-go-mod-cache \
  go build -o /private/tmp/open-rebellion-stage-ui-assets ./tools/stage-ui-assets
  PASS

/private/tmp/open-rebellion-stage-ui-assets --tactical-3d-convert --verify \
  --output /private/tmp/open-rebellion-p54.C60elI
  Verified tactical runtime pack with 87 meshes and 397 type-303 resources

/private/tmp/open-rebellion-stage-ui-assets \
  --tactical-3d-assimp-oracle /opt/homebrew/Cellar/assimp/6.0.5/bin/assimp \
  --output /private/tmp/open-rebellion-p54.C60elI
  Verified 87 tactical meshes against Assimp raw dumps
```

Synthetic tests cover deterministic reruns, tamper rejection, malformed and
truncated token streams, non-finite data, RLE overflow and underflow, zero-run
decoding, palette selection, source-bound exceptions, CLI exclusivity, and
standalone verification. Adversarial manifest tests cover negative and
oversized tail counts, missing or ambiguous texture targets, absent bindings,
missing same-language palettes, and decoded-output budget exhaustion before
RLE expansion. They also reject a consistently rehashed planet-palette
downgrade, altered runtime connectivity, and a consistently rehashed retained
mesh that differs from the source-derived object.

## Boundary and next proof

P55 does not package or render these objects, identify resource `2560` as a
specific DAT class, select the live battle palette, or prove handedness,
winding, UV orientation, camera, lighting, and depth in native or WebGL.
P56 must render `2560` with `SDESTI52.BMP` inside the original tactical
aperture, start Chromium muted, capture both supported viewports, and obtain an
Astra medium browser review. An original-runtime A/B capture remains required
before calling that view parity-correct.
