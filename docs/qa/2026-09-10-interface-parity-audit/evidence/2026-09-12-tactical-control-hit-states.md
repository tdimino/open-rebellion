# Tactical bitmap hit masks and zoom states (P53)

Status: verified scoped implementation checkpoint. This does not accept an
interface-parity cell. All 106 `TAC-01` through `TAC-07` cells remain pending.

## Contract

The original control routine recovered in `BmpCache::is_resource_hit` excludes
every source bitmap edge and then tests the bitmap's palette-key mask. Tactical
controls must therefore use their authored shape rather than the surrounding
rectangle. The extracted `TACTICAL.DLL` resources pair zoom-in `1044`/`1045`
and zoom-out `1046`/`1047` as normal and pressed states.

P53 applies that source-shaped input rule to the working pause, faction
highlight, zoom-in, and zoom-out controls. It also renders `1045` and `1047`
while the corresponding zoom control is held, returning to `1044` and `1046`
on release. Camera rotation, tilt, position memory, drag transitions, and
held-repeat behavior remain outside this checkpoint.

## Verification

- Renderer tests: 154 passed, including outer-edge rejection, opaque-pixel
  acceptance, and transparent internal matte rejection against the extracted
  control BMPs.
- Workspace tests: 653 passed, 20 ignored, 0 failed.
- Default scoped Clippy passed with the repository's known warnings. The strict
  pedantic configuration remains an audited repository-wide failure and is not
  reported as green.
- Browser run `2026-09-12T22-45-54-467Z-72972`: 4 of 4 fresh Chrome processes
  passed for both factions at 640×480 and 1280×800 letterboxed. Each launch was
  muted, made exactly four successful requests, reported no page, console, or
  missing-asset error, and closed its browser process.
- Each native faction run checked 1,848 zoom-control pixels across normal and
  pressed resources. It proved that source pixel `(1, 1)` is inert inside the
  pause, zoom-in, and zoom-out rectangles, captured held `1045` and `1047`,
  and restored the exact paused screenshot after zooming in and back out.
- The aperture-isolation check covered 112,284 pixels per native zoom
  transition and 483,060 per letterboxed transition.
- Production WASM SHA-256:
  `39d607fd68e061d96d566c1bc80fbe7d119cdf26a3a6110199c84e107c05d6f0`.
  Fixture WASM SHA-256:
  `ec596fc67a37ab4872353e681789136644b898391b11539eb4102d05e523eede`.
  Runtime pack SHA-256:
  `1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862`.

Astra medium inspected the code diff, all four result ledgers, and the held
pressed-state captures. It found no P0 through P2 issue and approved this as a
scoped checkpoint. Exact resource-pixel comparison is native-size only;
letterboxed pressed art was visually inspected. No original lossless A0 battle
capture exists for this state, so the screenshots remain unbaselined.

## Remaining work

The [P52 shell evidence](2026-09-12-tactical-shell-controls.md) lists the larger
tactical gaps. The next bundle must stage and decode the original type-301 and
type-303 battle graphics, then replace procedural ships, fighters, planets,
and effects without changing the authentic HUD coordinate system.
