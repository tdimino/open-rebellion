# Tactical planet resource path

The saved `REBEXE.EXE` Ghidra project establishes the system-selected tactical
planet resource chain:

1. `FUN_0040b0e0` receives the current system object and calls
   `FUN_00509610` on its referenced strategic object.
2. `FUN_00509610` follows the object at offset `+0x2c` and returns the selector
   at offset `+0x44`. The separate strategic system rendering path in
   `FUN_0046a9c0` uses the same getter for star-system object families
   `0x90` through `0x97`.
3. `FUN_0040b0e0` passes that selector through `FUN_00595d60` and its tactical
   scene constructors to `FUN_0059a850`, which stores it at tactical manager
   offset `+0x8d0`.
4. `FUN_00596ad0` returns `5500 + selector`, the type-303 planet resource.
5. `FUN_005c2e60` adds 30, loads `5530 + selector`, copies 256 RGB triples,
   and realizes the paired palette with flags `0x44`.

Open Rebellion binds the current `SYSTEMSD.picture_id` to this selector because
the DAT field supplies the same system picture identity consumed by the
strategic getter. The production renderer therefore requests planet
`5500 + SYSTEMSD.picture_id` with palette
`5530 + SYSTEMSD.picture_id`, and rejects a mismatched pair.

This source chain proves resource identity and pairing. The current left-edge
planet transform is provisional A1 placement guided by available screenshots.
Exact native retained-frame transform, visibility conditions, and occlusion
remain open for lossless owned-original capture or further tracing.

The checked-in note records conclusions only. The saved Ghidra database,
generated decompilation directory, and proprietary resource bytes remain
untracked.
