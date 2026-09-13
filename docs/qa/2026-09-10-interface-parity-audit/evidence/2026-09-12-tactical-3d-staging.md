# Tactical 3D raw-resource staging (P54)

Status: verified tooling checkpoint. This does not accept an interface-parity
cell. All 106 `TAC-01` through `TAC-07` cells remain pending.

## Contract

P54 extends the dependency-free Go asset extractor with an opt-in path for the
original tactical renderer's type-301 DirectX meshes and type-303 texture and
palette resources. It preserves raw bytes in a content-addressed store and
records each PE identifier, language, code page, reserved value, size, object
hash, and source DLL hash in a deterministic manifest.

Named PE resources are decoded as validated UTF-16. Resource names never become
filesystem paths. The staging preflight rejects malformed names, duplicate or
case-ambiguous identities, unsupported X headers, unexpected counts, and size
bounds before writing. Repeat staging repairs missing objects. A different
manifest requires `--force`.

The extractor reads `TACTICAL.DLL` once into a bounded 256 MiB snapshot, then
hashes and parses that same byte slice. PE traversal enforces the expected
resource count, a 64 MiB per-resource bound, and a 128 MiB aggregate bound
shared across meshes and textures before copying payloads. Repeat staging and
manifest verification require regular files and use bounded reads before
comparing or hashing manifests and objects.

The default bitmap, advisor, audio, string, and cutscene extraction path is
unchanged. Raw tactical output remains ignored and is generated only from a
contributor-owned installation.

## Owned-source inventory

The audited `TACTICAL.DLL` has SHA-256
`db12cdcfb6c85cec572a425307d2ad617bbc850027657424c3cc1f61c6ec275a`.
The extractor found and verified:

- 87 type-301 meshes totaling 268,936 bytes. Of these, 76 use
  `xof 0303bin 0032` and 11 use `xof 0302bin 0032`;
- 397 type-303 resources totaling 1,674,840 bytes. Of these, 59 have named PE
  identifiers and 338 have numeric identifiers;
- 484 unique content-addressed objects in a 3.4 MiB raw staging directory.

Type-301 resource `2560` is 5,417 bytes with SHA-256
`c9b6441a5b186b079b508c6f54bf0fd19a2ee7ffe872fd8bcb8979b94584cbc1`.
Its binary X token stream names `sdesti52.bmp`. Windows case-insensitive
resolution joins it to named type-303 resource `SDESTI52.BMP`, which is 5,414
bytes with SHA-256
`c128cd6b7304de44063b835c6f0f68cbbd4cc4a3cf7ccd288b8fb481c9b481e2`.
This proves one mesh-to-texture resource edge. It does not yet prove the mesh's
DAT identity or its original camera and LOD rules.

## Verification

- Go package: 53 test cases passed, 0 failed.
- Go race detector: passed.
- Go vet and build: passed.
- Rust workspace: 653 tests passed, 20 ignored, 0 failed.
- Fresh owned-source staging: 484 written and 0 unchanged.
- Identical repeat: 0 written and 484 unchanged.
- Verify-only pass: 87 type-301 and 397 type-303 resources rehashed and
  validated without reading the source DLL.
- Proprietary raw resources, generated derivatives, and the temporary manifest
  were not added to the repository.

There is no browser or Astra browser-acceptance run for P54 because this
checkpoint does not alter production rendering or the browser package. An
Astra low code-and-documentation review found no remaining P0 through P2
issues. The first visible 3D proof is P56. Its browser harness review uses Astra
medium effort and starts muted.

## Remaining work

P55 must decode the binary X templates and type-303 texture and palette data,
then emit a deterministic runtime mesh pack. P56 will render resource `2560`
and its exact named texture inside the existing original tactical aperture.
All semantic identities, original rendering rules, procedural-content removal,
and complete tactical acceptance remain open.

See the [tactical 3D asset plan](../../../plans/2026-09-12-feat-tactical-3d-asset-pipeline.md)
and [space-battle asset inventory](../../../reference/asset-library/space-battle.md).
