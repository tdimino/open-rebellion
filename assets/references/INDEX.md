# Reference Image Collections

1,320 usable reference images across 21 collections for HD upscaling. All collections audited 2026-03-27: non-OT portraits removed, text-only book pages quarantined, fan art purged.

## Upscale References

For Gemini multi-image upscaling pipeline. Feed alongside source BMP to guide detail reconstruction.

| Collection | Files | Content | Use For |
|-----------|-------|---------|---------|
| [ref-wookieepedia-portraits](ref-wookieepedia-portraits/) | 11 | OT-era film stills (audited) | GOKRES portrait upscaling |
| [ref-wookieepedia-portraits-ot](ref-wookieepedia-portraits-ot/) | 10 | OT /Legends re-scrape (audited) | GOKRES portrait upscaling |
| [ref-ccg-card-art](ref-ccg-card-art/) | 28 | OT-era Decipher CCG painted card art (audited) | Portrait style reference |
| [ref-swccg-cards](ref-swccg-cards/) | 76 | Decipher SWCCG card scans — all 60 Rebellion characters (MIT) | Portrait + character reference |
| [ref-swg-tcg-art](ref-swg-tcg-art/) | 67 | SWG Galaxies TCG painted art (audited, fan art removed) | Painted character/scene style |
| [ref-mcquarrie-rotj](ref-mcquarrie-rotj/) | 79 | McQuarrie ROTJ portfolio (21) + ESB concept paintings (58) | Scene + atmosphere reference |
| [ref-cross-sections](ref-cross-sections/) | 33 | Incredible Cross-Sections (1998) cutaway diagrams | Ship diagram upscaling |
| [ref-illustrated-books](ref-illustrated-books/) | 895 | 5 book extracts: characters (189), ROTJ (91), planets (211), weapons (199), starfighters (204) | Scene + character reference |
| [ref-gb-concept-art](ref-gb-concept-art/) | 25 | Galactic Battlegrounds concept art (MixnMojo) | Same-era LucasArts style |
| [ref-galactic-battlegrounds](ref-galactic-battlegrounds/) | 5 | GB sprite sheets (Spriters Resource) | Tactical sprite reference |

## Game Asset References

Original game EData extracts at 400x200 — the source art before DLL compression.

| Collection | Files | Content |
|-----------|-------|---------|
| [ref-characters](ref-characters/) | 7 | Major character panels (Luke, Han, Vader, etc.) |
| [ref-capital-ships](ref-capital-ships/) | 5 | Capital ship panels (ISD, MC80, Nebulon-B, etc.) |
| [ref-facilities](ref-facilities/) | 9 | Facility panels (mines, shipyards, shields, etc.) |
| [ref-fighters](ref-fighters/) | 4 | Fighter panels (X-Wing, A-Wing, B-Wing, Y-Wing) |
| [ref-troops](ref-troops/) | 8 | Troop unit panels |
| [ref-special-forces](ref-special-forces/) | 6 | Special forces unit panels |
| [ref-damage-diagrams](ref-damage-diagrams/) | 14 | GOKRES ship damage overlay sprites |
| [ref-missions](ref-missions/) | 8 | Mission event panels |
| [ref-planets](ref-planets/) | 9 | Planet panels |
| [ref-battle-backgrounds](ref-battle-backgrounds/) | 7 | Tactical combat backgrounds |
| [ref-squadron-sprites](ref-squadron-sprites/) | 14 | Fighter squadron sprites |

## NJO Total Conversion References

| Collection | Files | Content |
|-----------|-------|---------|
| [ref-vong-wookieepedia](ref-vong-wookieepedia/) | 139 | Yuuzhan Vong Wookieepedia images |
| [ref-vong-ships](ref-vong-ships/) | 73 | Vong ship renders and concept art |
| [ref-vong-tcg](ref-vong-tcg/) | 18 | Vong TCG card art |
| [ref-njo-japanese](ref-njo-japanese/) | 10 | Japanese NJO cover art |

## Mapping: Reference → DLL Asset Category

| DLL Assets | Best Reference Collections |
|-----------|---------------------------|
| GOKRES portraits (2000-2999) | ref-swccg-cards + ref-ccg-card-art + ref-wookieepedia-portraits-ot + ref-illustrated-books/characters-pages |
| GOKRES mini-icons (19000-19999) | ref-swccg-cards + ref-ccg-card-art + ref-wookieepedia-portraits-ot |
| GOKRES ship views (1-1999) | ref-capital-ships + ref-cross-sections |
| GOKRES damage diagrams (21000-26999) | ref-damage-diagrams + ref-cross-sections |
| STRATEGY event screens (6000-6999) | ref-mcquarrie-rotj + ref-illustrated-books/rotj-pages + ref-missions |
| STRATEGY character panels (1000-1999) | ref-swccg-cards + ref-characters + ref-illustrated-books/characters-pages |
| TACTICAL ship sprites (2000-2999) | ref-squadron-sprites + ref-galactic-battlegrounds |
| TACTICAL HUD (1000-1999) | ref-gb-concept-art + ref-swg-tcg-art |
| COMMON buttons/chrome (10000-15999) | ref-gb-concept-art |
