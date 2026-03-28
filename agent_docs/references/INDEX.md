# Reference Image Catalogs

5 catalog files listing vision-LLM-named reference images by upscaling category, plus 4 new collections acquired 2026-03-27.

**Audited 2026-03-27**: Non-OT-era portraits removed, text-only book pages quarantined, fan art purged. 1,320 usable images across 21 collections.

## New Collections (2026-03-27)

| Collection | Files | Content |
|---------|-------|---------|
| ref-swccg-cards | 76 | Decipher SWCCG card scans — all 60 Rebellion characters, MIT license |
| ref-swg-tcg-art | 67 | SWG Galaxies TCG painted art (fan art purged) |
| ref-mcquarrie-rotj | 79 | McQuarrie ROTJ portfolio (21) + ESB concept paintings (58) |
| ref-wookieepedia-portraits-ot | 10 | OT /Legends re-scrape (audited, non-OT removed) |

## Existing Catalogs

| Catalog | Files (usable) | DLL Assets | Load When |
|---------|---------------|-----------|-----------|
| [catalog-portraits.md](catalog-portraits.md) | ~230 | GOKRES portraits (2000-2999), mini-icons (19000-19999) | Upscaling character images |
| [catalog-ships.md](catalog-ships.md) | 271 | GOKRES ship views (1-1999), TACTICAL sprites (2000-2999) | Upscaling ship/vehicle images |
| [catalog-weapons.md](catalog-weapons.md) | 198 | TACTICAL weapon sprites, COMMON UI tech | Upscaling weapons/tech diagrams |
| [catalog-planets.md](catalog-planets.md) | ~211 | STRATEGY planet panels, system backgrounds | Upscaling planet/location images |
| [catalog-scenes.md](catalog-scenes.md) | ~91 | STRATEGY event screens (6000-6999), briefings | Upscaling narrative scene art |

## Usage

Reference selection is automated via `scripts/select-references.py`:

1. Routes each BMP to a pack via `scripts/reference-packs.json` (12 packs, 24 routing rules)
2. Builds contact sheets of candidates, pre-filtered by entity slug when available
3. Vision LLM picks best 3-5 references (rejects text pages, non-OT content)
4. Output: `data/reference-selections.json` (consumed by `scripts/gemini-upscale.py`)

Entity mapping: `data/resource-entity-map.json` — 162 GOKRES resource IDs → character/ship names.
