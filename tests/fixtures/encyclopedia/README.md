# Encyclopedia v1 conformance fixtures

This directory is the executable, contributor-authored file corpus for the
coordinator-approved E09 v1 wire contract. It contains no bytes, prose, names,
paths, screenshots, or images from an original installation.

Run the complete fixture gate from the repository root:

```bash
node tools/interface-parity/validate-encyclopedia-fixtures.mjs
```

The gate generates `fixtures/` twice in temporary directories, compares both
file inventories and every byte, then compares them with the checked-in tree.
To intentionally regenerate the checked-in synthetic files after a reviewed
contract change:

```bash
node tests/fixtures/encyclopedia/generators/generate.mjs \
  tests/fixtures/encyclopedia/fixtures
```

`cases.json` is the stable inventory. Every executable case names its validation
layer, expected acceptance and diagnostic, and downstream consuming validator.
The `delegated_inventory` section names related cases that deliberately have no
file-conformance assertion here.

## What this runner proves

- Ajv 2020-12 accepts or rejects the strict catalog, manifest, and overlay file
  shapes as recorded. Ajv does not claim relationship, digest, image, or runtime
  behavior.
- A raw-byte scanner accepts nested objects/arrays and repeated key names in
  distinct object scopes, but rejects invalid UTF-8, literal duplicate keys,
  and escaped-equivalent duplicate keys before ordinary whole-document
  `JSON.parse` can overwrite them. Files under `fixtures/raw/` are intentionally
  kept raw; only individual JSON string tokens use `JSON.parse` while scanning.
- Small bundle probes exercise potential membership, the aggregate-only topic,
  category command order, family-qualified binding identity, source-reference
  closure, immutable manifest hashes, and actual synthetic file bytes.
- The generated 24-bit BMP is decoded row-by-row. The 70-byte PNG is generated
  independently with Node's `zlib`, has SHA-256
  `5b8ce344a9d7fe4bdf8780725fc2fc36dce3f297688e62f18c8848fce6fecb8b`,
  and is decoded with locked `pngjs` 7.0.0 after chunk termination and CRC
  validation. CRC-valid IHDR dimensions are required to be nonzero and are
  checked with `BigInt` against the pixel cap before `pngjs` is entered. A tiny
  oversized-IHDR fixture asserts that pre-decode boundary directly. Plain text
  named `.png`, zero-sized/oversized IHDRs, truncated streams, and
  extension/magic mismatch reject.
- Boundary payloads are synthesized in memory. The repository does not store
  megabytes of padding or compressed allocation bombs.

The base bundle intentionally rejects a fully valid PNG because v1 original
base art is BMP. The author overlay accepts the equivalent PNG path shape, and
the separate image probe fully decodes its bytes. This distinguishes file
schema, immutable-base policy, and effective mod-image inspection.

## What remains downstream

E10 and E11 consume the raw/structural/relationship/integrity cases in their Go
and Rust validators. E22 owns overlay capability and effective-image behavior.
The complete faction pair for viewer-faction topic `original:60004` is a
positive delegated E22 capability case, including its new `1041` record. A
separate structurally valid pair on static topic `original:60001` is the
delegated `image_override_capability` rejection; structural acceptance alone
does not claim semantic acceptance.
E31 owns live world/viewer admission; the two observed 247-row caches are never
serialized here. E44 owns whole-record language fallback, label presentation,
and title sorting. Generated mod IDs, retained buffers, atomic candidate
publication, and rollback remain Rust adapter/session behavior rather than base
file schema.

`topic_ids` therefore represents potential membership and registry tie order,
not captured live admission or frozen display order. There is no v1 `aliases`
field, no effective DTO field, and no deferred alternate-art binding.
