---
title: "Addon: Create Your Own Character (Demiurgos)"
type: feat
status: planned
date: 2026-03-21
project: open-rebellion
designer: Nomos (soul architect subdaimon)
tags: [addon, character-creation, homeworld, portraits]
---

# Create Your Own Character — First Addon

## Design Philosophy

The original Rebellion gave you ~60 fixed characters. You never chose who joined your cause. **Create Your Own Character** breaks this contract deliberately. The LucasArts golden age principle: *give the player a meaningful choice that has consequences they can't fully predict*.

### Core Tensions
1. **Power fantasy vs. balance**: Custom character must matter but not outclass Luke/Vader
2. **Star Wars flavor vs. creative freedom**: Should feel like they belong in the galaxy
3. **Simplicity vs. depth**: Point-buy engaging for min-maxers, tedious for casuals

### Resolution: The Academy System
The character graduates from an **Academy** — Imperial Academy (Carida) or Rebel Training Facility (Yavin IV). The academy determines baseline capabilities. Player customizes *within* those constraints. The framing device simultaneously explains balance, delivers flavor, and structures the UI.

## Homeworld System (Universal — All Characters)

Every character gets a `homeworld: Option<SystemKey>`. This is a structural addition, not addon-only.

### Effects
- **+10%** to all skill checks when operating at homeworld
- **-5%** to all skill checks when enemy controls homeworld
- **-15 loyalty** one-time hit when homeworld falls to enemy
- **+10 loyalty** recovery when homeworld is liberated
- Special dialogue/events when defending or liberating homeworld

### Canon Assignments
`data/homeworlds.json` maps existing characters to EU homeworlds:
- Luke Skywalker → Tatooine
- Darth Vader → Tatooine
- Leia Organa → Alderaan
- Han Solo → Corellia
- Mon Mothma → Chandrila
- Thrawn → Csilla
- Admiral Ackbar → Mon Calamari

## Academy Archetypes (6)

| Archetype | Primary Skills | Admiral | General | Loyalty | Bonus Pts |
|-----------|---------------|---------|---------|---------|-----------|
| Diplomat | Diplomacy, Leadership | No | No | 55-80 | 40 |
| Operative | Espionage, Combat | No | No | 45-75 | 40 |
| Fleet Officer | Leadership, Combat | Yes | No | 55-85 | 35 |
| Ground Commander | Combat, Troop Training | No | Yes | 55-85 | 35 |
| Engineer | Ship Design, Facility Design | No | No | 55-80 | 35 |
| Fringer | Balanced (jack of all trades) | No | No | 35-65 | 50 |

**Skill ceiling: 85** (Luke's combat is 90, Thrawn's leadership 95 — custom < canon best).
**Total skill points: ~200-220** (canon majors average ~280).
**Loyalty: NOT player-chosen** — random within archetype range.

## Creation Flow (7 Steps)

1. **Choose Faction** — Alliance starbird or Imperial cog
2. **Choose Archetype** — 6 cards with skill radar charts
3. **Allocate Skills** — Point-buy within archetype floors/ceilings (5-point increments)
4. **Pick Homeworld** — Galaxy map, faction-controlled systems only
5. **Force Roll** — 8-16% chance based on species/archetype (NOT player-controlled)
6. **Name & Portrait** — Entry field + portrait browser (128 BMPs)
7. **Review & Commission** — Final confirmation, character inserted at HQ

## Force Sensitivity Balance

| Factor | Probability |
|--------|------------|
| Base rate | 8% |
| Species: Zabrak | +5% |
| Species: Twilek | +2% |
| Archetype: Fringer | +3% |
| **Maximum possible** | **16%** |

On success: `jedi_probability` = 15-30 (Luke = 100). Must still train in-game. Custom Force user needs 100+ training ticks to become relevant. Never rivals Luke.

## Hard Balance Caps

| Constraint | Value | Rationale |
|-----------|-------|-----------|
| Max custom characters per game | 3 | No army-of-clones |
| Skill ceiling (any single) | 85 | Below canon best |
| Total skill points | ~200-220 | Below canon majors (~280) |
| jedi_probability max | 30 | Luke = 100 |
| Loyalty | NOT player-chosen | Random, betrayal possible |
| hyperdrive_modifier | 0 (Fringer: +1) | Han Solo's bonus unique |
| is_unable_to_betray | Always false | Only Luke/Vader |
| is_jedi_trainer | Always false | Only Yoda |

## Data Structures

```rust
// Character additions:
pub homeworld: Option<SystemKey>,
pub origin: CharacterOrigin, // Dat | Custom

// New enums:
pub enum CharacterOrigin { Dat, Custom }
pub enum AcademyArchetype { Diplomat, Operative, FleetOfficer, GroundCommander, Engineer, Fringer }
pub enum Species { Human, MonCalamari, Bothan, Sullustan, Twilek, Wookiee, Rodian, Zabrak, Duros }
```

## Implementation Phases

1. **Homeworld System** — add field to Character, load from JSON, integrate into missions + betrayal, bump save v5
2. **Character Creation Core** — archetypes, 7-step UI, commission logic
3. **Force Sensitivity** — probability table, wire to Jedi system
4. **Portrait Generation (Galactic Daguerreotype)** — see below
5. **Flavor & Encyclopedia** — backstory templates, encyclopedia integration
6. **Mod Integration** — extend mod loader for `"action": "add"` entity creation

## Phase 4: Portrait Generation Pipeline (Galactic Daguerreotype)

Generate 20 custom character portraits (10F/10M, 9 species) matching the 1998 LucasArts painterly aesthetic via Nano Banana Pro. Designed by Kotharat subdaimon.

### Reference Collection (14 images — API maximum)

Expand the existing 7-portrait `ref-characters` collection to 14 by extracting from game DLLs:

| # | Source | Character | Species | Status |
|---|--------|-----------|---------|--------|
| 1-7 | Existing | Mon Mothma, Luke, Ackbar, Thrawn, Han, Vader, Luke (Jedi) | Human/Mon Cal/Chiss | Have |
| 8 | EDATA.080 | Leia Organa | Human F | Extract |
| 9 | EDATA.088 | Chewbacca | Wookiee | Extract |
| 10 | EDATA.083 | Wedge Antilles | Human M | Extract |
| 11 | EDATA.098 | Bib Fortuna | Twilek M | Extract |
| 12 | EDATA.110 | Talon Karrde | Human M | Extract |
| 13 | STRATEGY ~1015 | Borsk Fey'lya | Bothan M | Extract |
| 14 | STRATEGY ~1020 | Nien Nunb | Sullustan M | Extract |

All 14 upscaled to 4x via waifu2x before use as references.

### Species Distribution (20 Portraits)

**Female (10):**

| ID | Species | Archetype | Costume |
|----|---------|-----------|---------|
| `custom_f_human_01` | Human | Fleet Officer | Naval uniform, close-cropped hair |
| `custom_f_human_02` | Human | Diplomat | Senatorial robes, braided updo |
| `custom_f_human_03` | Human | Operative | Dark leather jacket, scar |
| `custom_f_twilek_01` | Twilek | Fringer | Green skin, pilot headset |
| `custom_f_twilek_02` | Twilek | Diplomat | Blue skin, ornate headpiece |
| `custom_f_zabrak_01` | Zabrak | Operative | Yellow-red skin, facial tattoos |
| `custom_f_bothan_01` | Bothan | Diplomat | Auburn fur, elegant neckpiece |
| `custom_f_moncal_01` | Mon Calamari | Engineer | Salmon skin, coveralls |
| `custom_f_rodian_01` | Rodian | Fringer | Green skin, leather jacket |
| `custom_f_duros_01` | Duros | Fleet Officer | Blue-grey skin, naval uniform |

**Male (10):**

| ID | Species | Archetype | Costume |
|----|---------|-----------|---------|
| `custom_m_human_01` | Human | Ground Commander | Field armor, close-cut beard |
| `custom_m_human_02` | Human | Engineer | Goggles, utility vest |
| `custom_m_human_03` | Human | Fleet Officer | Crisp naval uniform |
| `custom_m_twilek_01` | Twilek | Operative | Red skin, dark cloak, cybernetic eye |
| `custom_m_wookiee_01` | Wookiee | Ground Commander | Brown-gold fur, bandolier |
| `custom_m_sullustan_01` | Sullustan | Fleet Officer | Round jowls, flight uniform |
| `custom_m_zabrak_01` | Zabrak | Fringer | Orange skin, black tattoos |
| `custom_m_bothan_01` | Bothan | Operative | Tawny fur, hooded cloak |
| `custom_m_rodian_01` | Rodian | Engineer | Green skin, tool belt |
| `custom_m_duros_01` | Duros | Fringer | Blue skin, pilot vest |

All 9 species represented. Costumes are faction-neutral (no visible faction insignia).

### Prompt Template

```
Sci-fi space character portrait. {description}. Bust portrait from chest up,
3/4 angle facing slightly left. Dark background with subtle star field.
Dramatic warm key light from upper left, cool fill from lower right.
Painted style with visible brushwork, slightly idealized proportions,
oil-over-digital rendering. 1998 pre-rendered CGI aesthetic, same artistic
style as reference images. No text, no labels, no UI elements.
```

Species descriptions use anatomically precise language (e.g. Twilek: "two long fleshy head-tails (lekku) extending from the back of the skull, no hair, {color} skin").

### Execution Pipeline

```bash
# 1. Extract + upscale new references
python3 scripts/extract-portraits.py  # PIL convert from EDATA BMPs
waifu2x -i ref-characters/ -o ref-characters/ -n 1 -s 4

# 2. Generate batch (Nano Banana Pro, ~2-3 min, ~$2.60)
uv run scripts/generate-rebellion-assets.py \
  --category characters \
  --manifest data/manifests/custom-portraits.json

# 3. Post-process to game formats
mkdir -p assets/portraits/custom/{panels,thumbs,hd,bmp}
for f in data/generated/characters/custom_*.png; do
    base=$(basename "$f" .png)
    magick "$f" -resize 400x200^ -gravity center -extent 400x200 \
        "assets/portraits/custom/panels/${base}.png"
    magick "$f" -resize 80x80^ -gravity center -extent 80x80 \
        "assets/portraits/custom/thumbs/${base}.png"
    magick "$f" -resize 256x256^ -gravity center -extent 256x256 \
        "assets/portraits/custom/hd/${base}.png"
done

# 4. Validate via SmolVLM (automated style check, threshold >= 7/10)
# 5. Fallback: gemini-claude-resonance for failed portraits
```

### Output Formats

| Format | Size | Use |
|--------|------|-----|
| `panels/` | 400x200 | Officers panel, encyclopedia |
| `thumbs/` | 80x80 | Portrait picker grid (Step 6 of creation) |
| `hd/` | 256x256 | HD encyclopedia viewer |
| `bmp/` | Indexed palette | Game engine integration |

### Cost

~$4 for 20 portraits. ~$28 for full 128 (8 batches with varied costumes).

### Scaling to 128

Run 6 additional batches with varied costumes per species. The manifest structure supports this directly. Each batch uses the same 14 references and prompt template.

## Files to Modify

- `crates/rebellion-core/src/world/mod.rs` — homeworld, origin, enums
- `crates/rebellion-core/src/homeworld.rs` — NEW: HomeworldModifier
- `crates/rebellion-core/src/missions.rs` — homeworld probability modifier
- `crates/rebellion-core/src/betrayal.rs` — homeworld loyalty penalty
- `crates/rebellion-core/src/events.rs` — HomeworldCaptured/Liberated conditions
- `crates/rebellion-render/src/panels/character_creation.rs` — NEW: 7-step UI
- `crates/rebellion-data/src/mods.rs` — extend for entity addition
- `data/manifests/custom-portraits.json` — NEW: 20-item batch manifest
- `assets/portraits/custom/` — NEW: generated portrait output directory
- `data/homeworlds.json` — NEW: canon homeworld mappings

## The Three LucasArts Principles

1. **Every choice has consequences the player can't fully predict** — loyalty random, homeworld can fall, Force rolls rare
2. **The flavor IS the mechanic** — academy archetypes aren't just stat templates, they're identity
3. **The best characters are the ones the galaxy gives you** — custom characters supplement the roster, never replace it
