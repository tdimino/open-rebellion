# Total Conversions & Era Expansions

Open Rebellion's mod system supports total conversions—complete replacements of factions, units, characters, and star systems. The Ghidra RE work mapped every moddable parameter in the original engine, and our mod loader uses RFC 7396 JSON Merge Patch for clean, composable overrides.

These are the eras and expansions we're designing toward.

---

## New Jedi Order (25–30 ABY)

<div style="text-align:center; margin: 1em 0;">
  <img src="/open-rebellion/assets/new-jedi-order.png" alt="Jedi Order" width="60">
</div>

The Yuuzhan Vong invasion—the defining crisis of the post-Endor Expanded Universe. 19 novels, from *Vector Prime* (1999) to *The Unifying Force* (2003).

**Concept**: A three-faction war. The New Republic and the Imperial Remnant face an extragalactic invasion by the Yuuzhan Vong—sometimes allied, sometimes at each other's throats, always outgunned. This is the first total conversion that requires third-faction support in the engine.

**What changes**:

- **Three factions**: New Republic, Imperial Remnant, Yuuzhan Vong—all on one galaxy map. The Remnant and Republic can cooperate against the Vong or fight each other for territory, mirroring the fractured politics of the NJO era
- **Units**: Organic biotech fleet (coralskippers, yorik-et, Kor Chokk grand cruisers) vs. New Republic ships (E-wings, Viscount-class Star Defenders) vs. Imperial Remnant (Imperial II Star Destroyers, Pellaeon-class)
- **Characters**: Jacen Solo, Jaina Solo, Anakin Solo, Luke Skywalker (Grand Master), Tsavong Lah, Nom Anor, Vergere, Warmaster Nas Choka, Gilad Pellaeon (Remnant)
- **Systems**: Helska IV, Myrkr, Zonama Sekot, Coruscant (occupied → Yuuzhan'tar), Bastion (Remnant capital)
- **Mechanics**: Vong immune to Force detection, dovin basals instead of shields, villip communication instead of HoloNet, shifting alliances between Republic and Remnant

**Reference material**: See [Vong Total Conversion scope](https://github.com/tdimino/open-rebellion/blob/main/docs/vong-total-conversion.md) and the [NJO asset references](https://github.com/tdimino/open-rebellion/tree/main/assets/references/njo-total-conversion).

---

## Knights of the Old Republic (3,956 BBY)

<div style="text-align:center; margin: 1em 0;">
  <img src="/open-rebellion/assets/kotor-era.png" alt="Old Republic" width="60">
</div>

Four thousand years before the Battle of Yavin. The Jedi Civil War between the Sith Empire under Darth Malak and the Galactic Republic. Based on BioWare's KOTOR (2003) and Obsidian's KOTOR II (2004).

**Concept**: Replace both factions with Old Republic era equivalents. The galaxy map shifts to focus on Sith-controlled space (Korriban, Lehon, Malachor V) vs. Republic strongholds (Coruscant, Dantooine, Telos).

**What changes**:

- **Factions**: Galactic Republic vs. Sith Empire
- **Units**: Hammerhead cruisers, Foray-class blockade runners vs. Interdictor-class cruisers, Sith fighters
- **Characters**: Revan, Bastila Shan, Carth Onasi, Darth Malak, Canderous Ordo, Jolee Bindo
- **Systems**: Dantooine, Kashyyyk, Manaan, Korriban, Lehon (Rakata Prime)
- **Mechanics**: Star Forge as Death Star equivalent (infinite production while active), Force bonds, Mandalorian mercenary fleets

---

## Expanded Universe Elements

<div style="text-align:center; margin: 1em 0;">
  <img src="/open-rebellion/assets/expanded-universe.png" alt="Heir to the Empire" width="50">
</div>

Not full conversions—these are expansions that enrich the base Galactic Civil War era with content from the broader Expanded Universe.

### Thrawn's Revenge

Grand Admiral Thrawn's campaign from Timothy Zahn's Heir to the Empire trilogy (1991–1993). Adds the post-Endor timeline: Thrawn's return, the clone facilities on Wayland, Mara Jade's journey, the Noghri, and the Katana fleet.

- New characters: Mara Jade, Talon Karrde, Gilad Pellaeon, Joruus C'baoth
- New units: Dreadnaught-class (Katana fleet), Missile Boats, Lancer-class frigates
- New systems: Wayland, Myrkr, Nirauan
- New mechanics: Ysalamiri (Force-nullification zones), cloaking devices

### Shadows of the Empire

The multimedia project bridging Empire Strikes Back and Return of the Jedi. Prince Xizor's Black Sun criminal empire as a third faction.

- New faction: Black Sun (neutral/hostile to both)
- New characters: Prince Xizor, Guri, Dash Rendar
- New units: Virago (StarViper assault platform), Outrider
- New systems: Ord Mantell (criminal hub)

### Dark Empire

Luke Skywalker's fall to the dark side and the Emperor reborn. The Galaxy Gun and World Devastators as superweapons.

- New superweapons: Galaxy Gun, World Devastators
- New units: E-wings, V-wings, Shadow Droids
- Cloned Emperor mechanics: Emperor returns after death (limited uses)

---

## Modding Architecture

Total conversions use the same mod system as balance patches—TOML manifests with JSON Merge Patch overrides. The key difference is scale: a TC replaces most entity definitions rather than tweaking a few values.

```toml
# mod.toml for a total conversion
name = "New Jedi Order"
version = "1.0.0"
description = "Yuuzhan Vong invasion—replaces Empire with extragalactic invaders"
author = "Open Rebellion Community"

[dependencies]
# No dependencies for a standalone TC
```

Patch files (JSON Merge Patch) go alongside the manifest and replace entity definitions wholesale.

See the [modding guide](https://github.com/tdimino/open-rebellion/blob/main/agent_docs/modding.md) (developer reference) for the full mod loader specification.

---

## Status

| Expansion | Status | Blocking On |
|-----------|--------|-------------|
| Homeworld Expansion | **Planned** (v0.15.0) | Character creation UI |
| NJO Total Conversion | Design phase | Entity addition in mods, asset pipeline |
| KOTOR Total Conversion | Concept | NJO TC completion (proves the pipeline) |
| Thrawn's Revenge | Concept | Expanded Galaxy addon |
| Shadows of the Empire | Concept | Third-faction support |
| Dark Empire | Concept | Superweapon system generalization |
