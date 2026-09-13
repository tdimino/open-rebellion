---
title: "Tactical 3D Asset Pipeline"
description: "Recover the original DirectX battle meshes and textures for deterministic native and browser rendering"
category: plan
created: 2026-09-12
updated: 2026-09-12
tags: [interface, parity, tactical, 3d, wasm, assets]
status: in_progress
---

# Tactical 3D Asset Pipeline

This is the asset-rendering companion to the
[standalone battle launcher](2026-09-12-tooling-standalone-space-battle-launcher.md)
and `UIP-B06` in the
[batched interface plan](2026-09-11-feat-batched-interface-parity-plan.md).
It replaces the current procedural tactical ships, fighters, and planets with
the original game's real 3D resources. It does not create a second battle
renderer or treat modern replacement models as parity evidence.

## Confirmed source inventory

A September 2026 Codex Sol high-effort forensic review inspected the owned
editor export and existing Ghidra records. The new P54 extractor then audited
the owned original DLL directly:

- `TACTICAL.DLL` type 301 contains 87 DirectX `.x` capital-ship meshes. They
  form 29 close, medium, and far LOD families.
- The original DLL's meshes are compact uncompressed binary X resources:
  76 use `xof 0303bin 0032` and 11 use `xof 0302bin 0032`. Their combined raw
  size is 268,936 bytes. The editor export's 86 MSZIP-compressed files and one
  binary file are expanded derivatives, not the preferred runtime source.
- The original DLL contains 397 type-303 resources totaling 1,674,840 bytes:
  59 named and 338 numeric. The editor's 369 decoded images are a useful visual
  derivative but do not represent the complete raw inventory.
- Resource `2560` is 5,417 bytes with SHA-256
  `c9b6441a5b186b079b508c6f54bf0fd19a2ee7ffe872fd8bcb8979b94584cbc1`.
  Its binary token stream embeds `sdesti52.bmp`; the case-insensitive Windows
  resource match is named `SDESTI52.BMP`, size 5,414 bytes, SHA-256
  `c128cd6b7304de44063b835c6f0f68cbbd4cc4a3cf7ccd288b8fb481c9b481e2`.
- The native ordinal lookup proves 29 capital-ship and eight fighter resource
  families. The join from those ordinals to exact DAT identities and the
  original camera, lighting, culling, and LOD thresholds remains open.

The original executable used Direct3D Retained Mode 3D rendering. The previous
claim that pre-rendered sprite sheets represented the authentic 1998 battle
path was incorrect.

## Render profiles

| Profile | Allowed assets | Parity status |
| --- | --- | --- |
| `original-parity` | Original geometry, UVs, textures, LODs, shell, and source-traced camera/render rules | Only profile eligible for strict acceptance |
| `faithful-hd` | Same geometry, UVs, gameplay, camera, and LOD policy with separately approved texture upscales and higher internal resolution | Optional enhancement; never silently replaces parity assets |
| `experimental-remaster` | Replacement GLBs, altered geometry, PBR materials, normal maps, or generated models | Extension only; never counts toward parity |

Multiplayer simulation and network state must be independent of render profile,
visual LOD, or local texture choice.

Every P56 through P58 browser harness gate uses `codex-orchestrator` with Astra
at medium effort. Each browser scenario starts muted in a fresh Chromium
process and closes the browser and local server when complete.

## Pipeline

```text
owned TACTICAL.DLL
  -> raw type-301/type-303 resources plus source hashes
  -> deterministic binary X and type-303 decode
  -> validation GLB for inspection only
  -> versioned native mesh pack with positions, UVs, normals, u16 partitions,
     materials, bounds, LOD identity, and provenance
  -> lazy native/WASM upload for battle-visible resources
  -> Camera3D battle scene clipped inside the existing 640x480 aperture
  -> original bitmap HUD on the same canvas
```

GLB is an inspection and interchange artifact, not a required browser runtime
dependency. A small project-owned mesh-pack decoder avoids shipping Assimp or a
general glTF stack in WASM. Any converter must pin its version and record every
handedness, winding, UV-origin, triangulation, and vertex-splitting transform.

## Delivery passes

### P54. Reproducible raw staging

Status: complete. See the
[P54 evidence](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-3d-staging.md).

Add an opt-in, dependency-free extractor path for `TACTICAL.DLL` type 301 and
303 resources. Preserve numeric IDs or exact named identifiers, language,
code page, reserved value, bytes, and hashes in a content-addressed store.
Validate counts and X headers. Keep raw outputs ignored and generated only from
an owned installation.

Gate: synthetic PE tests, owned-source count and hash audit when the DLL is
available, unchanged rerun, force replacement, path safety, and no change to
the default UI extraction contract.

### P55. Deterministic converter and mesh pack

Status: complete. See the
[P55 evidence](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-3d-runtime-pack.md).

Parse the original binary X templates, decode type-303 textures and palettes,
emit deterministic geometry, and partition meshes that exceed macroquad's
`u16` index range. MSZIP support is optional compatibility for editor-derived
X files, not part of the original-DLL path. Use
a pinned Assimp 6.0.5 raw import as an independent oracle, not as a browser
runtime dependency. Resource `2082` is the one original mesh without source
normals; its identity and hash exclusively authorize derived face normals.

The converter emits 87 mesh objects and 397 texture objects. It preserves 370
indexed images rather than baking one guessed palette: 27 planet resources bind
their proven paired palettes, while 343 ship, fighter, and effect resources
retain the original active-battle-palette requirement for P56. All 59 embedded
texture filenames resolve uniquely to same-language named type-303 resources.
The optional pinned-Assimp gate compares triangle connectivity, transformed
positions and normals, transformed UVs, material values, and texture names for
every original mesh.

Gate: decoded hashes, finite coordinates, valid topology and indices, UV and
material references, normals, bounds, repeatable bytes, and parser differential
tests against the pinned oracle.

### P56. Single-resource browser proof

Status: next.

Render raw resource `2560` with embedded `sdesti52.bmp`, resolved and recorded
against original named resource `SDESTI52.BMP`, inside the
original tactical aperture. Do not claim its candidate DAT name yet. Use a
minimal normals-aware WebGL 1 material, fixed camera, depth test, and existing
HUD composition.

Gate: native and WASM selection, texture orientation, winding, depth, cold-load
memory, frame timing, context diagnostics, four-request packaging, both
viewports, and Astra medium browser review. An original-runtime A/B
capture is required before calling the view parity-correct.

### P57. One three-LOD family

Add `2561` and `2562`, then trace and implement the original LOD selection,
camera, filtering, culling, and lighting rules. Capture the same fixed views in
the original executable and Open Rebellion.

Gate: deterministic LOD transitions, no resource churn, accepted A0/A1 views,
and no simulation fingerprint change.

### P58. Fleet integration

Finish the DAT-to-tactical-ordinal join, load only resources present in the
battle, and integrate picking, selection, damage attachments, effects, and
fallback diagnostics. Repeat for fighters, planets, the Death Star, and the
remaining texture families.

Gate: every mapped entity has provenance, no procedural replacement remains in
accepted states, native/WASM and multiplayer simulation agree, and the relevant
`TAC-*` cells pass their complete evidence matrices.

## Performance boundaries

- Do not eagerly upload the full texture corpus. Fully expanded capital and
  system RGBA alone would consume roughly 234 MiB.
- Keep the current four-request browser startup contract by packaging generated
  runtime resources into the existing asset pack.
- Measure cold load, frame p95, GPU and heap memory, asset-cache bounds, and
  context-loss recovery. Rendering should remain presentation-only.
- Keep WebGL 1 as the first compatibility target. WebGL 2 can be enabled after
  evidence shows a concrete need. WebGPU is an optional later backend, not a
  parity prerequisite.
- KTX2/Basis is an optional post-profile optimization. Lossy texture encodings
  cannot replace original-parity pixels without an explicit acceptance gate.

## Evidence and legal boundary

Raw DLL resources, converted meshes, original textures, and generated runtime
packs remain ignored. The repository stores only extraction/conversion code,
schemas, fixtures built from synthetic data, hashes, and evidence records.
Password-protected deployment does not by itself authorize redistribution.

Primary format and runtime references:

- [Microsoft DirectX binary X encoding](https://learn.microsoft.com/en-us/windows/win32/direct3d9/binary-encoding)
- [Microsoft legacy X file format](https://learn.microsoft.com/en-us/windows/win32/direct3d9/x-files--legacy-)
- [Assimp X parser reference implementation](https://github.com/assimp/assimp/blob/master/code/AssetLib/X/XFileParser.cpp)
- [glTF 2.0 specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html)
- [KTX 2.0 specification](https://registry.khronos.org/KTX/specs/2.0/ktxspec.v2.html)
- [WebGPU specification](https://www.w3.org/TR/webgpu/)

The [space-battle inventory](../reference/asset-library/space-battle.md) and
[native tactical lookup](../reference/asset-library/tactical-lookup.json) remain
the canonical resource mapping records.
