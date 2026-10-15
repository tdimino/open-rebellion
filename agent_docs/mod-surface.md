---
title: "Mod Surface"
description: "What the original REBEXE fixes in code rather than data, area by area, what that means for content mods and era overhauls, the community's record of what failed, and the rules for turning a fixed rule into port data"
category: "agent-docs"
created: 2026-10-14
updated: 2026-10-14
tags: [modding, mod-surface, ghidra, literals, era-overhaul, parity]
---

# Mod Surface

`modding.md` documents the port's mod loader. This page answers the question
before it: what can a mod change at all? The original reads some rules from
its DAT files and fixes others in REBEXE (literal object ids, family-range
checks, two-slot per-side structures, switch tables). A port that wants mods
must know which is which, keep the original's behaviour as the default, and
expose the fixed rules as data one at a time.

The answers come from the epic `or-ep-mod-surface-vdch` in the task graph
(`task-graph.md`). Each row below names its Ghidra note; read the note before
changing the matching port code.

## What the original fixes, by area

| Area | Data (DAT, GNPRTB, SDPRTB) | Fixed in REBEXE | Note |
|---|---|---|---|
| Force users | Sensitivity, starting level and trainer flag per character (MJCHARSD/MNCHARSD); discover and teach floors and the training gain (GNPRTB 2580, 2581, 6173) | Leia is found only by Luke; detection flags Luke and Vader first; the meeting rewards name Luke, Leia, Vader and Palpatine; the AI's "Force >= 100" bit is Luke and Vader; the skill bonus `base × Force / 100`; the computer player never runs Jedi Training; no Dark Side attribute exists | `ghidra/notes/force-users.md` |
| Story encounters | Timers `0x392`/`0x393` and the final battle's Force floor (GNPRTB) | Triggers are change events on six literal characters through one gate (`FUN_005438a0`); the five abodes `0x98000440`-`0x98000481`; the Luke/Leia versus Vader/Palpatine pair filter; no story chain plays a movie | `ghidra/notes/story-encounters.md`, `story-missions.md` |
| Movies | The MDATA files themselves | A closed id-to-file switch (`FUN_00421ba0`, ids `0x65`-`0x74`); triggers are app states, four message classes and the trench run | `ghidra/notes/movies.md` |
| Ship types | Strategic icon and encyclopedia art (`text_stra_dll_id` arithmetic); stats | Tactical mesh, HUD portrait and fighter art come from a 38-entry ordinal registry (`FUN_00597610`); an unknown type gets ordinal 0 (Calamari); 29 mesh bases are immediate stores | `ghidra/notes/ship-type-assets.md` |
| Troops and facilities | Build, row, overview and encyclopedia art from the TEXTSTRA id; build lists (side flags, cost, research order) | Producer families `0x28`/`0x29`/`0x2a`; the factory registry accepts exact family bytes only; facility sprites exist for ordinals 1-6 and 1-2; the Troop Finder has ten literal columns | `ghidra/notes/ground-unit-assets.md` |
| Galaxy | Which sectors exist at each galaxy size (SECTORSD `galaxy_size`); seed tables | The nearness table is the one literal 200; system ids are a 100-299 window; system and sector families are literals in about 300 functions; galaxy size is 1..3 | `ghidra/notes/galaxy-limits.md` |
| Sides | Which classes each side may build or recruit (class `+0x40`/`+0x44`); SDPRTB columns | The side is one integer tested `== 1`, everything else is the Empire; session slots, support (`v` and `100 - v`), the win word and the tactical manager hold two sides; owner 3 (Neutral) is never a player; interface DLLs, advisor, voices and movies switch by side | `ghidra/notes/side-switches.md`, `side-numbering.md` |
| Scenario | Seed tables (CMUN*TB, FACL*TB, SYFC*TB), the maintenance budget, minor-character side flags, research orders and costs | Seven characters, Coruscant (16 sites), Yavin (7), the HQ instance `0x20000005`, the named-character placement list, three research trees, two game types | `ghidra/notes/scenario-literals.md`, `victory-conditions.md` |
| Record limits | Any number of records per file (the header count); a second record in an existing family; build lists by side flag, cost and research order | Header ranges must be one of 17 registered pairs and families one of 58 with a class (two free); three producer families and research slots; one researched record per tier; encyclopedia keys by the text id's low 12 bits; 77 record ids named in code; AI special-force classes by literal id | `ghidra/notes/record-registry.md` |
| Character roster | New minors (MNCHARSD family `0x38`) join the recruit pool by their side flags; who may be a Commander, Admiral or General (`can_be_*`); start counts (SDPRTB `0x1400`-`0x1402`) | Only families `0x38`-`0x3b` can be recruited and only `0x30`-`0x37` can recruit; each major family has its own class and only `0x30`-`0x35`, `0x38` have factories; a major always leads a mission; ranks 1-3 and their name offsets are literals | `ghidra/notes/character-roster.md` |
| Superweapons | A CAPSHPSD record with family 24 (`0x18`) and either side's flag builds, moves and fires Destroy System like the Death Star; costs, support shifts, the character roll, sabotage odds and the shield facility are DAT | Every rule keys on the family range `[0x18,0x1c)`; one such ship per system (literal 1); only families `0x14` and `0x18` have object factories; Destroy System is the only effect; the tactical attacker of a Death Star must be side 1 | `ghidra/notes/death-star-strategic.md`, `tactical-death-star-path.md`, `bombardment.md` |
| Audio | WAVE resources in STRATEGY, COMMON, TACTICAL, VOICEFXA/E and the sprite and briefing DLLs; advisor script `(wave, module)` pairs; the 16 music files `MDATA.300`-`315` | Wave ids are literals at each call site; the module-to-DLL map is fixed registration code; the tactical table (317 events, 351 ids) is constructor code; which track plays when (advice agent state, Battle Alert, battle result) is code; a DLL swap replaces sounds only at the same ids | `ghidra/notes/audio-dispatch.md` |
| Text | TEXTSTRA, TEXTCOMM, TEXTTACT (strings and TEXTSTRA RCDATA); ENCYTEXT, REBDLOG, CDDERR; `data0.reb` for five start-up messages | String ids are literals in code and a missing id shows nothing; the TEXTSTRA `FileVersion` gate picks five English literals over ids; `Current Latency: `, three start-up boxes, separators and the `Arial` face are fixed | `ghidra/notes/text-sources.md` |

`ghidra/notes/rebexe-map.md` maps every address range of REBEXE. Use it to
find the code behind a rule before opening the decompiles.

## What modders hit in the original

Twenty years of community modding (swrebellion.net and its forums) ran into
the rows above. The record agrees with the notes:

- **Adding records fails; only overwriting works.** Appending a ship,
  character or sector to its DAT file and its art to the DLLs never worked,
  so total conversions reskin the shipped slots
  (<https://swrebellion.net/forums/topic/458-adding-units-instead-of-replacing-existing/>,
  <https://swrebellion.net/forums/topic/1344-galaxy-size/>). The loader reads
  only the header count, and every appended record also needs a registered
  family, a unique encyclopedia key and its art (`record-registry.md`).
- **Character abilities sit on fixed slots.** Luke's and Leia's bonuses, the
  Emperor's and Vader's roles and the starting officers belong to their ids,
  not to fields (<https://swrebellion.net/forums/topic/3386-is-it-possible-to-mod-dll-hardcodingeg-character-bonuses/>).
- **Two sides only.** Era conversions rename the two sides; none added a
  third (<https://swrebellion.net/rebed/>).

Tools and format work worth citing: RebED (<https://swrebellion.net/rebed/>),
StarWarsRebellionEditor.NET
(<https://github.com/MetasharpNet/StarWarsRebellionEditor.NET>), the
Deep-Dive wiki on GNPRTB, SDPRTB, seed tables and the save
(<https://github.com/TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals/wiki>),
and Hexdragon's mechanics posts
(<https://swrebellion.net/forums/topic/9639-mechanics-inside-rebellion-part-ii/>).

## What era overhauls change in other games

Era mods for Empire at War (Thrawn's Revenge, Republic at War, Fall of the
Republic, Yuuzhan Vong at War) and Stellaris replace the same systems every
time: factions and their rosters, unique heroes and their abilities, the
galaxy map and start date, victory conditions, superweapons, scripted
events, research, audio and interface skins. Their authors' complaints are
the hard-coding to avoid: Empire at War's faction cap stood for fifteen years
until its 2021 patch, and its hero behaviours could not be overridden.

Several of their systems never existed in Rebellion: minor factions,
diplomacy, influence and unrest, several start dates. Those are new engine
work, not parity, and wait until after v1.0.

## Rules for port work

1. **The original's behaviour is the default.** A rule moves from a literal
   to data only with the original's value as the shipped data, so an
   unmodded game behaves exactly as before. Replay goldens must not change.
2. **One extension point per port node.** Each node cites the note row it
   exposes, names the literals it replaces, and is verified end to end for
   both factions. Changing what the loader accepts (for example, letting
   `mods.rs` add records with new ids, which it rejects today) is an
   architecture change: explain it and ask first.
3. **Expose only what the original computes.** Where the original has no
   mechanism (a Dark Side attribute, a third side, diplomacy), the port adds
   none until Tom approves the feature.
4. **Do not claim a mod works** until a mod that uses the extension point
   has run in the native and browser builds.
