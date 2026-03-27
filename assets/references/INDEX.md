# Reference Image Collections

518 reference files across 21 collections for HD upscaling, concept art, and visual parity. Plus ~1,097 extracted page images in subdirectories (characters-pages, planets-pages, weapons-images, starfighters-images, rotj-pages).

## Upscale References (new — 2026-03-26)

For Gemini multi-image upscaling pipeline. Feed alongside source BMP to guide detail reconstruction.

| Collection | Files | Size | Content | Use For |
|-----------|-------|------|---------|---------|
| [ref-wookieepedia-portraits](ref-wookieepedia-portraits/) | 60 | 10MB | All 60 game characters from Wookieepedia | GOKRES portrait upscaling |
| [ref-ccg-card-art](ref-ccg-card-art/) | 56 | 14MB | Decipher/FFG Star Wars CCG painted card art | Portrait style reference |
| [ref-cross-sections](ref-cross-sections/) | 1 PDF | 13MB | Incredible Cross-Sections (1998) — OT vehicle cutaways | Ship diagram upscaling |
| [ref-illustrated-books](ref-illustrated-books/) | 1 PDF | 11MB | ROTJ Illustrated Edition (2021) | Scene art reference |
| [ref-gb-concept-art](ref-gb-concept-art/) | 25 | 1.9MB | Galactic Battlegrounds concept art (MixnMojo) | Same-era LucasArts style |
| [ref-galactic-battlegrounds](ref-galactic-battlegrounds/) | 5 | 124KB | GB sprite sheets (Spriters Resource) | Tactical sprite reference |

## Game Asset References (existing)

Original game EData extracts at 400x200 — the source art before DLL compression.

| Collection | Files | Size | Content |
|-----------|-------|------|---------|
| [ref-characters](ref-characters/) | 7 | 668KB | Major character panels (Luke, Han, Vader, etc.) |
| [ref-capital-ships](ref-capital-ships/) | 5 | 232KB | Capital ship panels (ISD, MC80, Nebulon-B, etc.) |
| [ref-facilities](ref-facilities/) | 9 | 840KB | Facility panels (mines, shipyards, shields, etc.) |
| [ref-fighters](ref-fighters/) | 4 | 212KB | Fighter panels (X-Wing, A-Wing, B-Wing, Y-Wing) |
| [ref-troops](ref-troops/) | 8 | 820KB | Troop unit panels |
| [ref-special-forces](ref-special-forces/) | 6 | 604KB | Special forces unit panels |
| [ref-damage-diagrams](ref-damage-diagrams/) | 14 | 60KB | GOKRES ship damage overlay sprites |
| [ref-missions](ref-missions/) | 8 | 544KB | Mission event panels |
| [ref-planets](ref-planets/) | 9 | 528KB | Planet panels |
| [ref-battle-backgrounds](ref-battle-backgrounds/) | 7 | 504KB | Tactical combat backgrounds |
| [ref-squadron-sprites](ref-squadron-sprites/) | 14 | 244KB | Fighter squadron sprites |

## NJO Total Conversion References

| Collection | Files | Size | Content |
|-----------|-------|------|---------|
| [ref-vong-wookieepedia](ref-vong-wookieepedia/) | 139 | 18MB | Yuuzhan Vong Wookieepedia images |
| [ref-vong-ships](ref-vong-ships/) | 73 | 28MB | Vong ship renders and concept art |
| [ref-vong-tcg](ref-vong-tcg/) | 18 | 3.3MB | Vong TCG card art |
| [ref-njo-japanese](ref-njo-japanese/) | 10 | 5.8MB | Japanese NJO cover art |

## Empty Directories (pending collection)

ref-ui/, ref-ui-full/ (contain subdirectories with DLL dumps, not individual reference images), ref-videos/ (video files), ref-vong-biotech/, ref-vong-characters/, ref-vong-facilities/, ref-vong-warriors/

## Mapping: Reference → DLL Asset Category

| DLL Assets | Best Reference Collections |
|-----------|---------------------------|
| GOKRES portraits (2000-2999) | ref-wookieepedia-portraits + ref-ccg-card-art |
| GOKRES ship views (1-1999) | ref-capital-ships + ref-cross-sections |
| GOKRES damage diagrams (21000-26999) | ref-damage-diagrams + ref-cross-sections |
| STRATEGY event screens (6000-6999) | ref-illustrated-books + ref-missions |
| STRATEGY character panels (1000-1999) | ref-characters + ref-ccg-card-art |
| TACTICAL ship sprites (2000-2999) | ref-squadron-sprites + ref-galactic-battlegrounds |
| TACTICAL HUD (1000-1999) | ref-gb-concept-art |
| COMMON buttons/chrome (10000-15999) | ref-gb-concept-art |
