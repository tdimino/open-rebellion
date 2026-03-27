# Illustrated Reference Books

6 reference books with 1,097 extracted illustrations, renamed by vision LLM (Gemini Flash Lite via OpenRouter).

**Source**: Anna's Archive (annas-archive.gl)
**Rename manifest**: `../renames.jsonl` (1,113 entries with old→new mapping + descriptions)

## Books + Extracted Images

| File | Title | Year | Extracted |
|------|-------|------|-----------|
| `essential-guide-characters-1994.pdf` | Essential Guide to Characters (Andy Mangels) | 1994 | 214 pages → `characters-pages/` |
| `essential-guide-planets-moons-1998.pdf` | Essential Guide to Planets and Moons (Daniel Wallace) | 1998 | 226 pages → `planets-pages/` |
| `essential-guide-weapons-technology-1997.epub` | Essential Guide to Weapons and Technology (Bill Smith) | 1997 | 199 images → `weapons-images/` |
| `encyclopedia-starfighters-vehicles.azw3` | Encyclopedia of Starfighters and Other Vehicles (DK) | 2018 | 208 images → `starfighters-images/` |
| `rotj-illustrated-edition.pdf` | Return of the Jedi Illustrated Edition (James Kahn) | 2021 | 217 pages → `rotj-pages/` |
| `incredible-cross-sections-1998.pdf` | In `../ref-cross-sections/` | 1998 | 33 pages |

## Extraction + Naming

- PDFs extracted via PyMuPDF at zoom=3 (288 DPI)
- EPUB/AZW3 images extracted via zipfile (after Calibre conversion for AZW3)
- All 1,097 images renamed by Gemini Flash Lite — descriptive filenames like `luke-skywalker-dagobah-training-yoda.png`
- 2 images failed (timeout) — left as `page_NNNN.png`

## Relevance to DLL Assets

| Directory | Best For |
|-----------|---------|
| `characters-pages/` | GOKRES portraits (2000-2999) — 100+ painted character portraits |
| `planets-pages/` | STRATEGY planet panels, system backgrounds |
| `weapons-images/` | TACTICAL weapon sprites, COMMON UI tech elements |
| `starfighters-images/` | GOKRES ship views, TACTICAL ship sprites — HD vehicle photography |
| `rotj-pages/` | STRATEGY event screens — OT scene paintings |
