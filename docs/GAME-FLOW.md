# Star Wars: Rebellion (1998) --- Complete Game Flow Reference

> Comprehensive screen-by-screen documentation of every interaction from executable launch to game over.
> Compiled from: Prima Official Strategy Guide, game manual, GameFAQs (Rogue), Steam guide (Crownbreaker), CheatCC (xiriod), PCGamingWiki, Wookieepedia, and community sources.

---

## Table of Contents

1. [Pre-Game Flow](#1-pre-game-flow)
2. [New Game Setup](#2-new-game-setup)
3. [Opening Briefing](#3-opening-briefing)
4. [Main Game Interface (The Cockpit)](#4-main-game-interface-the-cockpit)
5. [Galaxy Map](#5-galaxy-map)
6. [System View](#6-system-view)
7. [Droid Advisors](#7-droid-advisors)
8. [Control Buttons / Command Bar](#8-control-buttons--command-bar)
9. [Character Management](#9-character-management)
10. [Fleet Management](#10-fleet-management)
11. [Production / Manufacturing](#11-production--manufacturing)
12. [Research](#12-research)
13. [Missions Panel](#13-missions-panel)
14. [Encyclopedia / Databank](#14-encyclopedia--databank)
15. [Space Combat (Tactical Mode)](#15-space-combat-tactical-mode)
16. [Ground Combat](#16-ground-combat)
17. [Bombardment](#17-bombardment)
18. [Event Popups and Notifications](#18-event-popups-and-notifications)
19. [Save / Load Interface](#19-save--load-interface)
20. [Options Menu](#20-options-menu)
21. [Victory / Defeat](#21-victory--defeat)
22. [Audio Architecture](#22-audio-architecture)
23. [Screen Transition Map](#23-screen-transition-map)

---

## 1. Pre-Game Flow

### 1.1 Executable Launch

- **Executable**: `REBELLION.EXE` (Win32, DirectX 5.0)
- **Resolution**: 640x480, 256 color (8-bit palette), DirectDraw
- **DRM**: SafeDisc on retail CD; DRM-free on GOG; Steam wrapper on Steam version
- **Config files**: Stored in game installation directory
- **Save location**: `<install-path>\SaveGame\`

### 1.2 Splash Screens (Sequential, Non-Interactive)

1. **LucasArts Logo** --- Gold LucasArts Entertainment Company LLC logo on black background. Accompanied by the LucasArts fanfare audio sting. ~3 seconds.
2. **Coolhand Interactive Logo** --- Developer credit screen. ~2 seconds.
3. **LucasFilm THX Logo** (optional depending on version) --- THX audio certification.

These splash screens cannot be skipped on some versions (Vista+). On XP and earlier, pressing Escape or clicking will advance past them.

### 1.3 Main Menu

After splash screens, the game presents the **Main Menu** overlaid on a space starfield background with the Star Wars: Rebellion logo.

**Menu Options (top to bottom):**

| Button | Action |
|--------|--------|
| **New Game** | Opens the New Game Setup screen |
| **Load Game** | Opens the Save/Load file browser |
| **Multiplayer** | Opens multiplayer setup (LAN/Modem/Internet via Zone.com) |
| **Options** | Opens audio/video/game speed settings |
| **Credits** | Scrolling credits sequence |
| **Quit** | Exits to Windows |

**Audio**: The main menu plays an arrangement of the Imperial March / Star Wars main theme. Ambient starfield with subtle particle effects.

**Controls**: Mouse-driven. Click on menu items. No keyboard shortcuts on the main menu itself.

---

## 2. New Game Setup

### 2.1 Setup Screen Layout

The New Game Setup screen presents configuration options before starting a campaign. All settings are configured via dropdown menus or radio buttons.

**Configuration Options:**

| Setting | Options | Default |
|---------|---------|---------|
| **Faction** | Galactic Empire / Rebel Alliance | Empire |
| **Galaxy Size** | Small (4 core, 6 rim) / Medium (4 core, 11 rim) / Large (4 core, 15 rim) | Medium |
| **Difficulty** | Easy / Intermediate / Expert | Intermediate |
| **Game Type** | Standard / Headquarters Only | Standard |
| **Starting Bonuses** | On / Off | On (for Easy) |

**Faction Selection:**
- Each faction is represented by its insignia (Imperial crest / Rebel phoenix)
- Selecting a faction changes the cockpit frame, droid advisor, and starting position

**Galaxy Size:**
- Controls the number of Outer Rim sectors
- All sizes have 4 Core sectors: Sesswenna, Sluis, Farfin, Fakir (always present)
- Small: 6 Outer Rim sectors
- Medium: 11 Outer Rim sectors (the "standard" experience)
- Large: 15 Outer Rim sectors

**Difficulty:**
- Easy: More starting units, weaker AI, starting bonuses available
- Intermediate: Standard balance
- Expert: Stronger AI, same starting conditions as Intermediate but less margin for error

**Game Type:**
- Standard: Capture both enemy leaders + enemy HQ planet
  - Empire must capture: Luke Skywalker, Mon Mothma, and the Rebel HQ
  - Rebels must capture: Emperor Palpatine, Darth Vader, and Coruscant
- Headquarters Only: Capture only the enemy HQ

**Buttons:**
- **Start Game** --- Begins the game with selected settings
- **Cancel** --- Returns to Main Menu

### 2.2 Galaxy Generation

Upon clicking Start Game:
- Galaxy is procedurally generated based on size setting
- Planet loyalties are randomized within constraints
- Starting character pool is partially randomized (each side gets fixed "major" characters plus 2 random "minor" characters)
- Starting unit placement is randomized within faction-controlled systems
- Brief loading screen with progress indicator

---

## 3. Opening Briefing

### 3.1 The Droid Briefing (Unskippable on Vista+)

Immediately after galaxy generation, the player is presented with a **mandatory droid briefing sequence** lasting approximately 3 minutes.

**Alliance Version:**
- C-3PO and R2-D2 appear in the cockpit advisory window
- They deliver a briefing on the current galactic situation
- Opening crawl text describes the post-Death Star destruction setting
- C-3PO explains the interface, basic controls, and immediate objectives
- R2-D2 provides beeps and whistles as commentary

**Empire Version:**
- The IT-O Interrogator Droid delivers the briefing
- Same structural information but with Imperial flavor
- Explains the Empire's strategic position and objectives

**The opening crawl text reads:**

> REBELLION
>
> It is a dark time for the Rebellion. Although the Death Star has been destroyed, Imperial forces have located the hidden Rebel base on Yavin and are poised to strike back.
>
> On the planet Coruscant, the heart of the Empire, Darth Vader and the Emperor make plans to crush the rebel Alliance once and for all. The full weight of the Empire is about to come to bear against the rebels.
>
> In their secret headquarters, Alliance leaders, resolute after their recent victory, gather the warships of the rebel fleet. Although they have won a significant battle, the war between the Alliance and the Empire has only just begun...

**Briefing DLL Files:**
- `ALBRIEF.DLL` --- Alliance briefing audio/video
- `EMBRIEF.DLL` --- Empire briefing audio/video
- Deleting/renaming these files skips the spoken dialog (briefing runs in ~2 seconds with no audio)

**Technical Note:** On Windows XP and earlier, pressing Escape could skip this briefing. On Vista and later, the skip functionality is broken---the briefing must play through to completion. This is one of the first things most players learn to work around (by creating a "master save" immediately after the briefing).

---

## 4. Main Game Interface (The Cockpit)

### 4.1 Overview

The main game screen is the **cockpit view**---a faction-specific decorative frame surrounding the galaxy map. This is where the player spends ~95% of their time. Everything is controlled from this single screen, with panels overlaying it.

### 4.2 Cockpit Frame

The cockpit frame is a pre-rendered border around the edges of the 640x480 screen:

**Alliance Cockpit:**
- Lighter color scheme (grays, whites, blues)
- Organic/rounded aesthetic reflecting Rebel ship design
- Control panels integrated into the frame borders
- Droid advisory window in the lower-left showing C-3PO and R2-D2

**Empire Cockpit:**
- Darker color scheme (blacks, dark grays, reds)
- Angular/geometric aesthetic reflecting Imperial design language
- Control panels integrated into the frame borders
- Droid advisory window in the lower-left showing the IT-O Interrogator Droid

### 4.3 Screen Regions

The cockpit is divided into these functional regions:

```
+--------------------------------------------------+
|  [Resource Bar - Raw/Refined/Maintenance]        |
+--------+-------------------------------+---------+
|        |                               |         |
| LEFT   |      GALAXY MAP              | RIGHT   |
| PANEL  |      (Central Area)           | PANEL   |
| (Cmd   |                               | (Info   |
|  Btns) |                               |  Area)  |
|        |                               |         |
+--------+-------------------------------+---------+
| [Droid  |   [Time Controls]    | [Game  |         |
|  Window]|   Speed + Day Counter| Ctrl]  |         |
+---------+-------------------------------+---------+
```

**Top Bar (Resource Display):**
- Three resource indicators with icons and numbers:
  1. **Raw Materials** (mine icon) --- Current stockpile of unrefined resources
  2. **Refined Materials** (refinery icon) --- Current stockpile of finished goods
  3. **Maintenance Capacity** (wrench icon) --- Current surplus/deficit of maintenance points
- Each mine/refinery pair produces 50 Maintenance Capacity
- Numbers update in real-time as the game clock advances

**Bottom Bar (Time Controls):**
- **Day Counter** --- Shows current game day (starts at Day 1)
- **Speed Control** --- 6 speed settings:
  1. Paused (full stop, no actions possible except menu access)
  2. Very Slow
  3. Slow
  4. Medium
  5. Fast
  6. Very Fast
- Speed buttons are small clickable arrows or a slider

**Bottom-Left (Droid Advisory Window):**
- Animated droid advisor sprite
- Text display area for droid commentary/notifications
- Right-click on the droid to access additional options:
  - **Galaxy Overview** --- Summary statistics (total mines, refineries, systems controlled, etc.)
  - Notification preferences

---

## 5. Galaxy Map

### 5.1 Overview

The galaxy map occupies the central area of the cockpit and is the primary navigation interface. It displays the Star Wars galaxy from a top-down perspective.

### 5.2 Zoom Levels

The galaxy map has **two primary zoom levels**:

**Sector View (Zoomed Out):**
- Shows the entire galaxy divided into sectors
- Core sectors (4) are in the center; Outer Rim sectors surround them
- Each sector is labeled with its name
- Sectors show aggregate faction control via color coding
- Fleet movement lines are visible as arrows between sectors
- Clicking a sector zooms into System View

**System View (Zoomed In):**
- Shows individual planets/systems within a single sector
- Core sectors contain ~10 planets each; Rim sectors contain ~10 planets each
- Each planet is shown as a sphere with its name
- Planets show faction color (green = Alliance, red = Empire, gray/white = neutral, star = unexplored rim)
- Loyalty bar displayed beneath each planet (green-to-red gradient)
- Clicking a planet opens the Planet Detail panel
- Fleet icons shown orbiting planets or in transit between them

### 5.3 Planet Display

Each planet in System View shows:

- **Planet name** (e.g., "Coruscant", "Yavin IV")
- **Faction control indicator** (colored border or glow)
- **Loyalty bar** --- Horizontal bar beneath the planet:
  - Full green = 100% Alliance loyal
  - Full red = 100% Empire loyal
  - Mixed = divided loyalties
  - The bar has no numerical percentage---it must be eyeballed
  - At ~60% toward one side, the system is "sympathetic" to that faction
- **Facility slots** --- Rows of small squares beneath the loyalty bar:
  - **Blue squares** (top row) = Total building capacity for the planet
  - **White squares** = Slots currently occupied by buildings
  - **Red squares** (bottom row) = Subset of total slots available for mines specifically
  - **Yellow-filled red squares** = Mine slots currently in use
- **Uprising indicator** --- Flashing or special icon if the planet is in uprising state
- **Shield indicator** --- Icon if GenCore shields are present
- **Fleet icons** --- Ship silhouettes orbiting the planet if fleets are present

### 5.4 Map Interactions

| Action | Result |
|--------|--------|
| Left-click planet | Select planet, show brief info tooltip |
| Double-click planet | Open Planet Detail panel |
| Right-click planet | Context menu (see below) |
| Left-click fleet icon | Select fleet |
| Right-click fleet icon | Fleet context menu |
| Click-drag on empty space | Pan the map |
| Scroll wheel / zoom buttons | Toggle between Sector and System view |
| Click sector name (in sector view) | Zoom into that sector |

**Planet Right-Click Context Menu:**
- View Defenses
- View Manufacturing
- Build (opens construction queue)
- View Characters (on planet)
- View Fleets (in orbit)
- Send Mission To...

**Fleet Right-Click Context Menu:**
- View Fleet Contents
- Split Fleet
- Merge Fleet
- Assign Commander (Admiral/Commander/General)
- Move Fleet To... (then click destination)
- Bombard (if in orbit of enemy planet)
- Land Troops (if in orbit)

---

## 6. System View --- Planet Detail Panel

### 6.1 Planet Detail Overlay

When a planet is selected (double-clicked or via context menu), a detail panel overlays part of the screen showing:

**Planet Information:**
- Planet name and sector
- Controlling faction
- Loyalty bar (larger, more detailed than map view)
- Garrison requirement (number of troops needed to prevent uprising)
- Current garrison count
- Population indicator

**Defenses Tab:**
- List of defensive structures (GenCores, LNR Lasers, KDY-150 Ion Guns)
- Stationed troop regiments with type and count
- Stationed starfighter squadrons
- Detection rating summary (total detection capability)
- Any characters present (shown with portraits and roles)
- Officers assigned (General/Commander/Admiral) with leadership bonus

**Manufacturing Tab:**
- Construction Yards (with queue)
- Shipyards (with queue)
- Training Facilities (with queue)
- Current build progress bars
- Queue management (add, remove, reorder items)

**Structures Tab:**
- Full list of all structures on the planet
- Mines count
- Refineries count
- Defensive structures
- Production facilities
- Slots remaining (blue boxes available)

### 6.2 Facility Slot System

Each planet has a fixed capacity determined at galaxy generation:

- **Energy Consumption slots** (blue squares) --- Hard cap on total structures
- **Mine slots** (red squares) --- Subset cap on mines specifically
- Building a mine uses both a red and a blue slot
- Building anything else uses only a blue slot
- Structures can be deleted to free slots (instantaneous)
- Deleting mines/refineries reduces Maintenance Capacity

---

## 7. Droid Advisors

### 7.1 Alliance Droids: C-3PO and R2-D2

- Appear in the lower-left advisory window
- C-3PO delivers verbal notifications and warnings in his characteristic voice
- R2-D2 provides complementary beeps and whistles
- They announce:
  - Mission results (success, failure, foiled)
  - Fleet arrivals and departures
  - Construction completions
  - Research breakthroughs
  - Enemy fleet sightings
  - Uprising warnings
  - Character events (recruitment, injury, capture, escape)
  - Force events (Luke/Vader encounters, Dagobah training)
  - Bounty Hunter attacks on Han
  - Random galactic events
- Right-clicking C-3PO opens the **Galaxy Overview** panel

### 7.2 Empire Droid: IT-O Interrogator Droid

- Same functional role as C-3PO/R2-D2 but with Imperial aesthetic
- Delivers notifications in a mechanical, menacing voice
- Floating spherical droid with various probes and sensors
- Same right-click menu for Galaxy Overview

### 7.3 Galaxy Overview Panel

Accessed via right-click on the droid advisor:

- Total systems controlled by each faction
- Total mines and refineries (with imbalance warning)
- Total Maintenance Capacity and current usage
- Total military units by category
- Total characters (active, captured, injured)
- Research progress summary
- Overall strategic summary

---

## 8. Control Buttons / Command Bar

### 8.1 Left Panel Command Buttons

The left side of the cockpit frame contains the primary command buttons. These are faction-styled (Imperial angular / Rebel rounded) but functionally identical:

| Button Icon | Name | Function |
|------------|------|----------|
| Character silhouette | **Characters** | Opens the Character Management panel |
| Ship silhouette | **Fleets** | Opens Fleet Management panel |
| Factory/gear | **Manufacturing** | Opens Production overview for selected system |
| Beaker/flask | **Research** | Opens Research progress panel |
| Scroll/mission | **Missions** | Opens Mission assignment panel |
| Book/database | **Encyclopedia** | Opens the in-game Databank |
| Floppy disk | **Save/Load** | Opens Save/Load interface |
| Gear/wrench | **Options** | Opens game options |

### 8.2 Additional Interface Buttons

- **End Turn / Advance Time** --- Not applicable (game is real-time, not turn-based)
- **Pause Button** --- Stops the game clock (no actions can be taken while fully paused)
- **Speed Buttons** --- Adjust game speed from Very Slow to Very Fast

---

## 9. Character Management

### 9.1 Character List Panel

Accessed via the Characters button or right-clicking a character on the map:

**Panel Contents:**
- Scrollable list of all characters on your side
- Each entry shows:
  - Character portrait (small)
  - Character name
  - Current location (planet/fleet/in transit)
  - Current assignment (mission type, officer role, or idle)
  - Health status (healthy, injured, captured)
  - Force status (if applicable: Novice, Trainee, Student, Knight, Master)

**Character Detail View** (clicking a character):
- Full character portrait
- **Stats Display:**
  - Diplomacy (0-200+)
  - Espionage (0-200+)
  - Combat (0-200+)
  - Leadership (0-200+)
- **Research Capabilities:** (if any)
  - Ship Design
  - Troop Training
  - Facility Design
- **Officer Capabilities:**
  - Can be Admiral (Y/N)
  - Can be Commander (Y/N)
  - Can be General (Y/N)
- **Force Sensitivity:**
  - Current Jedi rank (if known)
  - Force icon/indicator
- **Current Assignment:** Description of what they're doing
- **Special Abilities:** (e.g., Han's Millennium Falcon effect: +50 hyperspace when traveling solo with characters only)

### 9.2 Character Actions

From the character panel, you can:

| Action | Description |
|--------|-------------|
| **Assign to Mission** | Opens mission selection (see Missions Panel) |
| **Assign as Officer** | Promote to Admiral/Commander/General at current location |
| **Move To** | Transport character to another planet/fleet |
| **Retire** | Permanently remove character (cannot retire major characters) |
| **Group With** | Assign to a fleet or group with other characters |

### 9.3 Starting Characters

**Empire (Always Start With):**
- Emperor Palpatine (Jedi Master, Recruiter) --- Coruscant
- Darth Vader (Jedi Master, Recruiter, all-rounder) --- Coruscant
- Jerjerrod (Diplomat/Officer)
- Piett (Diplomat/Officer)
- Veers (Troop Researcher)
- +2 random minor characters

**Alliance (Always Start With):**
- Mon Mothma (Best Recruiter) --- Rebel HQ (outer rim)
- Luke Skywalker (Jedi Novice, versatile) --- Yavin IV
- Leia Organa (Best Diplomat, latent Force) --- Yavin IV
- Han Solo (Spec Ops, Millennium Falcon effect) --- Yavin IV
- Chewbacca (Spec Ops) --- Yavin IV
- Jan Dodonna (Diplomat) --- Yavin IV
- Wedge Antilles (Ship Researcher) --- Yavin IV
- +2 random minor characters

**Total Recruitable Characters:** 28 per side (including starting characters)

---

## 10. Fleet Management

### 10.1 Fleet List Panel

Accessed via the Fleets button:

**Panel Contents:**
- List of all fleets and individual ships
- Each fleet entry shows:
  - Fleet name (auto-generated or player-named)
  - Location (planet orbit / in transit / destination + ETA)
  - Ship count summary (capital ships, fighters, transports)
  - Assigned officer (Admiral/Commander if any)
  - Troop count aboard

### 10.2 Fleet Detail View

Clicking a fleet shows:

- **Ship Roster:** Every ship in the fleet with:
  - Ship type and name
  - Hull integrity (damage bar)
  - Shield status
  - Fighter complement (for carriers/capital ships)
  - Troop complement (for transports/Star Destroyers)
  - System damage indicators (weapons, shields, engines, hyperdrive)
- **Fleet Commander:** Assigned Admiral with leadership bonus
- **Fighter Commander:** Assigned Commander with starfighter bonus
- **Characters Aboard:** Listed with portraits
- **Spec Ops Aboard:** Count of each type (takes no ship space)

### 10.3 Fleet Actions

| Action | Description |
|--------|-------------|
| **Create Fleet** | Group selected ships into a new fleet |
| **Split Fleet** | Divide fleet into two or more groups |
| **Merge Fleet** | Combine two fleets at same location |
| **Move Fleet** | Click destination on galaxy map; fleet begins hyperspace travel |
| **Assign Officer** | Assign Admiral, Commander, or General to fleet |
| **Load/Unload Troops** | Transfer troops between ships and planet surface |
| **Load/Unload Fighters** | Transfer fighters into carrier/capital ship bays |
| **Bombard** | Initiate planetary bombardment (when in orbit of enemy planet) |
| **Land Troops** | Initiate ground invasion (when shields are down) |
| **Blockade** | Position fleet over enemy planet to interdict supply lines |

### 10.4 Fleet Movement

- Ships travel via hyperspace at speeds determined by the slowest ship in the fleet
- Han Solo's Millennium Falcon effect: +50 hyperspace modifier when traveling with ONLY characters (no ships, troops, or spec ops)
- Travel time between adjacent core sectors: ~20-40 days
- Travel time core to outer rim: ~40-100+ days
- Fleet movement shown on galaxy map as animated arrow/line
- Interdictor Cruisers (Empire) / CC-7700 Frigates (Rebels) prevent enemy fleet retreat during combat

---

## 11. Production / Manufacturing

### 11.1 Manufacturing Panel

Accessed via the Manufacturing button or from a planet's detail panel:

**Panel Layout:**
- Left side: Sector/planet selector
- Center: Available items to build (categorized by facility type)
- Right side: Build queue for selected planet
- Bottom: Cost information and build time estimate

### 11.2 Production Facility Types

| Facility | Builds | Upgrade |
|----------|--------|---------|
| **Construction Yard** | Facilities, structures, defenses | Advanced Construction Yard (2x speed) |
| **Shipyard** | Capital ships, fighters, transports | Advanced Shipyard (2x speed) |
| **Training Facility** | Troops, special forces | Advanced Training Facility (2x speed) |

- Multiple facilities of the same type on the same planet stack multiplicatively on build speed
- 6 Advanced Shipyards on one planet = fastest possible build time for capital ships
- Build speed also influenced by Refined Materials availability

### 11.3 Build Queue

Each planet's production facilities maintain separate queues:

- **Construction Queue** (from Construction Yards)
- **Ship Queue** (from Shipyards)
- **Training Queue** (from Training Facilities)

Queue interface allows:
- Adding items to the end of the queue
- Removing items (refunds Refined Materials)
- Reordering queue items
- Viewing progress bar for current item
- Viewing estimated completion date

### 11.4 Buildable Items

**Structures (Construction Yards):**
- Construction Yard / Advanced Construction Yard
- Shipyard / Advanced Shipyard
- Training Facility / Advanced Training Facility
- GenCore Level I Shield / GenCore Level II Shield
- LNR Series I Planetary Gun
- KDY-150 Ion Gun
- Mine
- Refinery
- Death Star Shield (Empire only)

**Ships (Shipyards):** See unit lists in Section 15.

**Troops & Spec Ops (Training Facilities):** See unit lists in Sections 16 and 13.

### 11.5 Cost System

Each item has two costs:
1. **Refined Materials** (up-front, consumed when construction begins)
2. **Maintenance Capacity** (ongoing, consumed for the item's lifetime)

Examples:
- Imperial Probe Droid: 1 Refined Materials, 1 Maintenance
- Death Star: 584 Refined Materials, 600 Maintenance
- X-Wing squadron: low cost, low maintenance
- ImpStar II: high cost, high maintenance

If insufficient Refined Materials exist, the item is queued and waits until enough accumulate.

Scrapping a unit refunds its Refined Materials and frees its Maintenance Capacity.

---

## 12. Research

### 12.1 Research Panel

Accessed via the Research button:

**Panel Layout:**
- Three research branches displayed as tech trees:
  1. **Ship Design** --- Unlocks new ship types
  2. **Troop Training** --- Unlocks new troop/spec ops types
  3. **Facility Design** --- Unlocks advanced facilities
- Each branch shows:
  - Already unlocked items (highlighted)
  - Next item to be unlocked (with progress bar)
  - Remaining items (grayed out)
  - Assigned researcher character (portrait + name)
  - Research speed indicator

### 12.2 Research Mechanics

- Research progresses via a hidden point pool per branch
- Each "making progress" report from a researcher adds points to the pool
- Having facilities of the appropriate type provides a slow passive trickle
- Breakthroughs always occur in the same fixed order per branch
- Once a researcher exhausts a branch, they become available for other duties

**Researcher Assignment:**
- Only specific characters can research (marked in their stats)
- Each side has ~2 characters per branch plus 1 "wildcard" who can do any branch
- Researchers must be on a planet with the appropriate facility:
  - Ship Design: requires Shipyard on planet
  - Troop Training: requires Training Facility on planet
  - Facility Design: requires Construction Yard on planet

### 12.3 Research Trees (Fixed Order)

**Empire Ship Research:**
TIE Bomber > Lancer Frigate > Star Galleon > Assault Transport > Interdictor Cruiser > VSD II > Strike Cruiser > ISD II > Super Star Destroyer

**Empire Troop Research:**
War Droid Regiment > Dark Trooper Regiment

**Empire Facility Research:**
Advanced Training Facility > Advanced Shipyard > Advanced Construction Yard > Death Star Shield

**Rebel Ship Research:**
Nebulon-B Frigate > Mon Calamari Cruiser > Corellian Gunship > CC-7700 Frigate > Assault Frigate > Liberator Cruiser > CC-9600 Frigate > Dauntless Cruiser > Bulwark Battlecruiser

**Rebel Troop Research:**
Sullustan Regiment > Mon Calamari Regiment > Wookiee Regiment

**Rebel Facility Research:**
Advanced Training Facility > Advanced Shipyard > Advanced Construction Yard

---

## 13. Missions Panel

### 13.1 Mission Assignment Interface

Accessed via the Missions button or from character/planet context menus:

**Panel Layout:**
- Left: Mission type selector (list of available mission types)
- Center: Team assembly area
  - Primary team members (characters + spec ops who will perform the mission)
  - Decoy slots (characters + spec ops who will provide cover)
- Right: Target selector (planet, fleet, character, or structure depending on mission type)
- Bottom: Confirmation button, estimated travel time

### 13.2 Team Assembly

- Drag characters/spec ops from a roster into Primary or Decoy slots
- Primary team members' stats contribute to mission success
- Decoys' espionage ratings help avoid detection but do NOT contribute to mission success
- Decoys do NOT gain stat increases from successful missions
- Characters and spec ops take up no space on ships (unlimited capacity)
- Characters can be captured or killed if mission is foiled
- Spec ops can only be killed (never captured)

### 13.3 Mission Types

| Mission | Used By | Key Stats | Target | Notes |
|---------|---------|-----------|--------|-------|
| **Espionage** | Bothan Spies / Espionage Droids + Characters | Espionage | Enemy/Neutral planet | Reveals all info about target + bonus random enemy planet |
| **Sabotage** | Infiltrators / Commandos + Characters | Espionage + Combat (averaged) | Specific structure, ship, or unit | Can destroy any single target. Click specific target, not just planet |
| **Diplomacy** | Characters only | Diplomacy | Neutral/Friendly planet | Progressive mission; repeats until stopped. At ~60% loyalty, planet joins your side |
| **Recruitment** | Specific characters only (Luke/Leia/Han/Mon Mothma for Rebels; Palpatine/Vader for Empire) | Leadership | Friendly planet | Success influenced by planet loyalty. Recruits new characters |
| **Abduction** | Infiltrators / Noghri Death Commandos + Characters | Combat (team total vs. target combat) | Enemy character | Captures target character as prisoner |
| **Assassination** | Noghri Death Commandos + Characters (Empire only) | Combat | Enemy character | Kills target (cannot kill major characters: Luke, Leia, Han, Mon Mothma, Vader, Palpatine) |
| **Rescue** | Infiltrators / Noghri Death Commandos + Characters | Combat | Captured friendly character | Frees prisoner and returns them |
| **Incite Uprising** | Guerrillas / Commandos + Characters | Leadership | Enemy planet (with sympathetic population) | Progressive mission; gradually shifts loyalty. Can trigger full planet flip |
| **Subdue Uprising** | Guerrillas / Commandos + Characters | Leadership | Friendly planet in uprising | Progressive mission; calms unrest. Must also meet garrison troop requirement |
| **Research** | Specific researcher characters only | N/A (passive) | Friendly planet with appropriate facility | Long-running progressive mission; unlocks tech tree items |
| **Reconnaissance** | Probe Droids / Y-Wing Longprobes only | N/A | Any planet | Scouts a single planet; returns same info as flying a ship past it |
| **Jedi Training** | Luke (Rebel) / Vader (Empire) only | N/A | Friendly planet | ~90 day mission; increases Jedi Score of trainees by 25-50%. Luke must be Jedi Knight+ |
| **Death Star Sabotage** | Infiltrators + Characters (Rebel only) | Espionage + Combat | Death Star | Attempt to destroy the Death Star via trench run-style sabotage |

### 13.4 Mission Resolution

Every mission has two phases:

**Phase 1 --- Detection:**
1. If enemy detectors exist at target (troops, ships, officers, fighters), each gets a "detection roll"
2. Detection ratings of enemy units determine their chance to detect the mission
3. Officers (Generals especially) dramatically increase detection capability
4. If a detection is made, a second roll determines if a primary member or decoy was found
5. Decoys with higher espionage ratings are more likely to absorb detection
6. If a primary member is detected: mission is "foiled" --- risk of capture/injury/death
7. If only decoys are detected: mission proceeds

**Phase 2 --- Execution:**
1. If detection passed, the mission itself is resolved
2. Each mission type uses different stats (see table above)
3. There is always a base chance of failure (~10%) that cannot be eliminated
4. Success = mission objectives achieved + stat increases for primary team members
5. Failure = "mission unsuccessful" but team returns safely

**Detection Layers:**
- Orbiting fleet provides first detection layer
- Planet surface troops/officers provide second detection layer
- Departing through orbiting fleet provides third detection layer (can be captured leaving even after success)

---

## 14. Encyclopedia / Databank

### 14.1 Encyclopedia Panel

Accessed via the Encyclopedia button:

**Panel Layout:**
- Left: Category tabs
- Center: Scrollable list of entries
- Right: Detailed information view with stats, description, and image

**Categories:**
- **Ships** --- All capital ships and fighters for both factions with full stats
- **Troops** --- All troop types with attack/defense/detection ratings
- **Characters** --- Bios for all characters (including those not yet recruited)
- **Structures** --- All buildable facilities with descriptions
- **Planets** --- Information on known planets
- **Special Forces** --- Spec ops unit descriptions and capabilities

**Ship Stats Display:**
- Hull points
- Shield strength
- Turbolaser batteries (fore/aft/port/starboard)
- Laser cannons
- Ion cannons
- Bombardment rating
- Fighter capacity
- Troop capacity
- Hyperdrive speed
- Maneuverability rating
- Detection rating
- Maintenance cost
- Refined Material cost
- Build time (at 1 shipyard)

**Character Bio Display:**
- Portrait (large)
- Biographical text (drawn from Star Wars Expanded Universe / Legends)
- Base stat ranges
- Special abilities notation
- Force sensitivity indicator

---

## 15. Space Combat (Tactical Mode)

### 15.1 Combat Initiation

Space combat occurs when two opposing fleets occupy the same location. The game presents a choice:

**Combat Options Dialog:**
- **Resolve Automatically** --- The computer calculates the battle outcome instantly using unit stats. Results screen shows losses for both sides.
- **Fight Tactical Battle** --- Enters the 3D tactical battle mode for manual control.
- **Retreat** --- Attempt to withdraw (may not be possible if enemy has Interdictor/CC-7700)

### 15.2 Tactical Battle Mode (3D View)

If the player chooses tactical battle:

**Screen Layout:**
- **Main View:** 3D rendered space scene showing ships from a top-down/isometric perspective
- **Minimap:** Small radar/overview in corner showing all ship positions
- **Ship Selection Panel:** Bottom area showing currently selected ship(s) with health bars
- **Command Buttons:** Along the bottom or side:
  - Move
  - Attack
  - Target Selection
  - Formations
  - Tactics (preset maneuvers)
  - Retreat
  - Speed control (pause/play/fast-forward)

**Ship Placement Phase:**
- At battle start, ships are arranged in a default formation
- Player can reposition ships before combat begins by dragging them
- Ships arranged based on type: capital ships in center, fighters screening, transports in rear

**Combat Mechanics:**
- Real-time 3D rendered battle
- Ships fire automatically at assigned targets
- Player controls ship movement and target priority
- Turbolasers effective against capital ships
- Laser cannons effective against fighters
- Ion cannons disable ship systems
- Proton torpedoes launched by fighters at capital ships (when shields are down)
- Damage modeled per ship section (fore/aft/port/starboard)
- Individual ship systems can be damaged/destroyed:
  - Weapons systems
  - Shield generators
  - Engines/maneuverability
  - Hyperdrive

**Formations (preset via Tactics button):**
- **Line Abreast** --- Ships spread horizontally
- **Column** --- Ships in a line
- **Wedge/Arrow** --- V-formation
- **Anvil** --- Pincer movement (uses tractor beams to slow retreating ships)
- **Screen** --- Fighters screen capital ships

**Maneuvers:**
- Ships can be given waypoints for movement
- Navigating around obstacles (asteroids, debris)
- Retreat command attempts to move ships to hyperspace jump point
- Interdictor/CC-7700 presence prevents retreat

**Fighter Operations:**
- Fighters launch from carriers and capital ships
- Fighter squadrons can be given individual targets
- Bombers (TIE Bombers, B-Wings, Y-Wings) are more effective against capital ships in tactical mode
- Fighters dock at carriers to rearm/repair between engagements

### 15.3 Combat Results Screen

After battle resolution (auto or tactical):

- **Battle Summary:** Win/Loss/Draw indicator
- **Ship Losses:** List of destroyed ships for both sides
- **Ship Damage:** List of damaged/disabled ships
- **Fighter Losses:** Squadron counts
- **Troop Losses:** If boarding/ramming occurred
- **Salvage:** Any captured ships
- **Sector Loyalty Impact:** Destroying enemy capital ships shifts sector loyalty in your favor
- **OK Button:** Returns to main cockpit view

---

## 16. Ground Combat

### 16.1 Ground Combat Initiation

Ground combat occurs when:
1. A fleet in orbit initiates a troop landing (planet shields must be down)
2. Troops are transferred from ships to planet surface where enemy troops exist

### 16.2 Ground Combat Resolution

Ground combat is **always auto-resolved** --- there is no tactical ground combat mode.

**Resolution Process:**
1. Attacking troops land on the planet
2. Each troop regiment has Attack and Defense ratings
3. Combat is resolved in rounds until one side is eliminated
4. Bombardment Defense rating determines troop survival against orbital bombardment
5. Officers (Generals) on the planet boost defensive troops
6. Planetary guns (LNR) fire on orbiting ships during invasion

**After Ground Combat:**
- If attacker wins: Planet changes control
  - If planet's loyalty was already sympathetic to attacker: smooth transition
  - If planet was hostile: may immediately go into uprising, requiring garrison
  - Garrison requirement: 1-6 troops depending on loyalty level
  - Need 6+ troops to suppress most hostile planets
- If defender wins: Attacking troops destroyed, planet remains under original control

### 16.3 Troop Types

**Empire:**
| Troop | Attack | Defense | Bombardment Def | Detection | Notes |
|-------|--------|---------|-----------------|-----------|-------|
| Army Regiment | Low | Low | Low | 15 | Starting basic |
| Fleet Regiment | Low | Med | Low | 20 | Starting, slightly better |
| Stormtrooper | Med | Med | Med | 25 | Starting, standard garrison |
| War Droid | High | Med | Med | 5 | Researched, poor detection |
| Dark Trooper | High | High | High | 30 | Researched, best Imperial troop |

**Alliance:**
| Troop | Attack | Defense | Bombardment Def | Detection | Notes |
|-------|--------|---------|-----------------|-----------|-------|
| Army Regiment | Low | Low | Low | 10 | Starting basic |
| Fleet Regiment | Low | Med | Low | 15 | Starting, slightly better |
| Sullustan | Med | Med | Med | 35 | Researched, BEST detection in game |
| Mon Calamari | Med | High | Med | 20 | Researched, good defense |
| Wookiee | High | Med | Med | 20 | Researched, best attack |

---

## 17. Bombardment

### 17.1 Bombardment Interface

Initiated from a fleet in orbit of an enemy planet:

**Bombardment Panel:**
- Shows planet surface with all structures, troops, and units
- Player selects specific targets for bombardment
- Each ship's bombardment rating determines damage output
- GenCore shields block bombardment entirely:
  - GenCore I: Blocks limited bombardment
  - GenCore II: Equivalent to 2x GenCore I
  - 2 GenCore I shields = effectively immune to bombardment (even multiple VSD salvos)
  - Must sabotage GenCores before bombardment is effective
- LNR Planetary Guns fire back at bombarding ships

**Bombardment Targeting:**
- Can target specific structures (factories, shields, guns)
- Can target troops
- Can target all military targets (selective)
- **CRITICAL**: Destroying non-military structures (mines, refineries, civilian buildings) causes massive negative loyalty shift across the entire sector
- Destroying only enemy troops has a positive sector-wide loyalty effect
- Best practice: Target only military targets (troops, shields, guns)

### 17.2 Death Star Bombardment

- The Death Star can destroy an entire planet
- This is the ONLY action that shifts loyalty galaxy-wide (not just sector-wide)
- Massive negative loyalty shift for the Empire everywhere
- Only advisable as a final act (destroying the Rebel HQ to win the game)

---

## 18. Event Popups and Notifications

### 18.1 Notification System

The game uses a popup dialog system for all events:

**Notification Types:**
- **Mission Results** --- "Mission successful at [Planet]" / "Mission foiled at [Planet]" / "Mission unsuccessful at [Planet]"
- **Construction Complete** --- "[Unit] completed at [Planet]"
- **Research Breakthrough** --- "[Technology] now available"
- **Fleet Arrival** --- "Fleet has arrived at [Planet]"
- **Enemy Sighting** --- "Enemy fleet detected near [Planet]"
- **Uprising Alert** --- "[Planet] has gone into uprising!"
- **Character Events** --- Capture, escape, injury, death, recruitment
- **Force Events** --- Dagobah training, Vader/Luke encounters, heritage revelations
- **Bounty Hunter Events** --- Han Solo bounty hunter attacks
- **Random Events** --- Planetary disasters (destroy facilities/slots), planetary bonuses (add slots)
- **Traitor Detection** --- Jedi character detects a traitor

**Popup Structure:**
- Faction-styled dialog box
- Droid advisor portrait
- Text description of event
- OK button (some events have multiple pages)
- Some events offer choices (e.g., combat: Auto-resolve / Tactical / Retreat)

### 18.2 Major Story Events (Scripted)

**Luke's Dagobah Training:**
- Triggers automatically between Days 200-500 (usually ~Day 300-350)
- Luke announces departure, disappears for ~100 days
- Returns with significant Jedi Score boost (often jumps to Jedi Student)
- Cannot be prevented or accelerated

**Luke/Vader Encounter ("I Am Your Father"):**
- Triggers when Luke and Vader are in the same system
- Neither can be a decoy; they must be on active missions or present in the system
- Outcomes: Luke may be injured, captured, or escape
- Luke escaping grants 25% Jedi Score bonus
- First encounter reveals Luke's "heritage"
- Event is repeatable (for the Jedi Score bonus)

**Luke Tells Leia ("The Force is Strong in My Family"):**
- After Luke learns his heritage, being near Leia (mission together or same system) triggers
- Unlocks Leia's latent Force sensitivity
- Leia gains immediate stat boosts

**Final Battle ("So Be It, Jedi"):**
- If Luke is captured and his Jedi Score >60:
  - Vader automatically travels to Luke (uncontrollable)
  - Vader takes Luke to the Emperor
  - If Luke's Jedi Score <100: Luke remains captured and injured
  - If Luke's Jedi Score >100: Luke captures BOTH Vader AND the Emperor
- This event only fires once

**Han Solo Bounty Hunters:**
- Every 1-100 days: 30% chance of bounty hunter encounter
- Usually Han escapes; very low capture rate (~3-5%)
- If captured: Taken to Jabba's Palace
  - Luke, Leia, and Chewie automatically attempt rescue
  - Each gets individual rescue attempt
  - If any succeed: everyone freed
  - If all fail: All four sent to Vader as prisoners

**Emperor's Seat of Power:**
- While Emperor Palpatine is in the Coruscant system (on planet or ship):
  - ALL Imperial Officer characters get doubled Leadership scores
  - This is a massive passive bonus for the Empire

---

## 19. Save / Load Interface

### 19.1 Save Game Screen

Accessed via Save button or main menu:

- File browser showing existing saves
- Save slot selection (named saves)
- Ability to name/rename saves
- Overwrite confirmation dialog
- Save files stored in `<install-path>\SaveGame\`
- No auto-save feature
- No quicksave/quickload (must use the menu)

### 19.2 Load Game Screen

Same layout as Save:
- Browse existing save files
- Preview information (date, faction, turn number)
- Load button
- Cancel button

---

## 20. Options Menu

### 20.1 Options Panel

Accessible from Main Menu or in-game Options button:

**Audio Settings:**
- Music Volume (slider)
- Sound Effects Volume (slider)
- Voice Volume (slider, controls droid advisor speech)

**Video Settings:**
- Limited options (game is fixed 640x480, 8-bit color)
- Brightness/gamma adjustment (on some versions)

**Game Settings:**
- Game Speed default
- Notification preferences (which events pause the game)
- Auto-resolve vs. prompt for tactical combat

**Multiplayer Settings (when applicable):**
- Network type: LAN / Modem / Internet (Zone.com)
- Player name
- Connection settings

---

## 21. Victory / Defeat

### 21.1 Victory Conditions

**Standard Game:**
- **Empire Wins:** Capture Luke Skywalker + Mon Mothma + Capture Rebel HQ planet
- **Rebels Win:** Capture Emperor Palpatine + Darth Vader + Capture Coruscant

**Headquarters Only:**
- **Empire Wins:** Capture Rebel HQ planet
- **Rebels Win:** Capture Coruscant

"Capture" for characters means they are prisoners in your custody.
"Capture" for a planet means you control it militarily.

### 21.2 Victory Screen

When victory conditions are met:

- Full-screen cinematic / victory cutscene
- Faction-specific victory celebration
  - Rebels: Celebration scene reminiscent of Endor victory
  - Empire: Scene of galactic domination / fleet parade
- Statistics summary:
  - Total days elapsed
  - Systems controlled
  - Ships destroyed
  - Characters captured
  - Major events triggered
- **Return to Main Menu** button

### 21.3 Defeat Screen

If the opposing side meets their victory conditions:

- Defeat notification
- Summary of what went wrong
- Option to load a save or return to Main Menu

### 21.4 Credits

Accessible from Main Menu:

- Scrolling credits listing development team at Coolhand Interactive
- LucasArts production credits
- Music credits (John Williams compositions arranged for the game)
- Star Wars Rebellion: Rebellion logo
- Copyright notices

---

## 22. Audio Architecture

### 22.1 Music

**Main Menu:**
- Arrangement of the Star Wars Main Theme / Imperial March
- Orchestral MIDI or pre-recorded tracks from the soundtrack

**In-Game Ambient Music:**
- Continuous ambient music during gameplay
- Faction-influenced selections:
  - Empire: Darker, more ominous arrangements (Imperial March variations, Emperor's Theme)
  - Rebels: Heroic, hopeful arrangements (Rebel Fanfare, Luke's Theme, Force Theme)
- Music shifts dynamically with game events:
  - Combat initiation triggers combat music
  - Victory events trigger celebratory themes
  - Tense situations (uprising, enemy nearby) trigger suspense cues
- Music drawn from John Williams' Original Trilogy score, re-arranged

**Tactical Combat Music:**
- High-energy battle music during space combat
- Separate from main game ambient music

### 22.2 Sound Effects

| Category | Examples |
|----------|---------|
| **Interface** | Button clicks, panel open/close, selection confirmation, queue assignment |
| **Fleet Movement** | Hyperspace jump initiation sound, fleet arrival chime |
| **Combat** | Turbolaser fire, laser cannon fire, explosions, shield impacts, torpedo launches |
| **Notifications** | Alert chimes for events, different tones for good/bad news |
| **Construction** | Completion fanfare when a unit finishes building |
| **Mission Sounds** | Mission launch, mission success fanfare, mission failure tone |
| **Droid Sounds** | R2-D2 beeps and whistles, IT-O mechanical sounds |

### 22.3 Voice

- **C-3PO** (Alliance droid advisor) --- Voiced with characteristic speech patterns
- **R2-D2** (Alliance droid) --- Characteristic beeps, whistles, warbles
- **IT-O Interrogator Droid** (Empire advisor) --- Mechanical voice
- Droid advisors deliver all notifications verbally
- No other character voice acting (characters are text/portrait only)

---

## 23. Screen Transition Map

```
EXECUTABLE LAUNCH
       |
  [Splash Screens]
  LucasArts Logo -> Coolhand Logo -> (THX)
       |
  [MAIN MENU]
   |   |   |   |   |   |
   |   |   |   |   |   +-> Quit (exits)
   |   |   |   |   +-> Credits (scrolling, returns to Main Menu)
   |   |   |   +-> Options (overlay panel, returns to Main Menu)
   |   |   +-> Multiplayer Setup -> Game
   |   +-> Load Game -> [COCKPIT]
   +-> New Game Setup
        |
   [Galaxy Generation + Loading]
        |
   [Droid Briefing - ~3 min]
        |
   [COCKPIT / MAIN GAME SCREEN] <-- Central Hub
        |
        +-- Galaxy Map (always visible in center)
        |     +-- Sector View <-> System View (zoom toggle)
        |     +-- Planet Detail Panel (overlay)
        |           +-- Defenses Tab
        |           +-- Manufacturing Tab
        |           +-- Structures Tab
        |
        +-- Characters Panel (overlay)
        |     +-- Character Detail
        |     +-- Assign Mission -> Missions Panel
        |     +-- Assign Officer
        |     +-- Move Character
        |
        +-- Fleet Panel (overlay)
        |     +-- Fleet Detail
        |     +-- Split/Merge Fleet
        |     +-- Move Fleet
        |     +-- Bombard -> Bombardment Interface
        |     +-- Land Troops -> Ground Combat (auto-resolve)
        |
        +-- Manufacturing Panel (overlay)
        |     +-- Build Queue per planet
        |     +-- Item cost/time info
        |
        +-- Research Panel (overlay)
        |     +-- Tech Tree display
        |     +-- Researcher assignment
        |
        +-- Missions Panel (overlay)
        |     +-- Mission Type selection
        |     +-- Team Assembly (Primary + Decoys)
        |     +-- Target Selection
        |     +-- Launch Mission
        |
        +-- Encyclopedia Panel (overlay)
        |     +-- Ships / Troops / Characters / Structures / Planets / Spec Ops
        |
        +-- Save/Load (overlay)
        |
        +-- Options (overlay)
        |
        +-- Event Popups (modal dialogs)
        |     +-- Mission results
        |     +-- Combat initiated -> Auto-resolve / Tactical / Retreat
        |     +-- Story events
        |     +-- Notifications
        |
        +-- Space Combat (full screen transition)
        |     +-- Ship Placement Phase
        |     +-- Real-Time 3D Battle
        |     +-- Combat Results Screen
        |     +-- Return to Cockpit
        |
        +-- Victory/Defeat Screen -> Main Menu
```

---

## Appendix A: Keyboard Shortcuts

| Key | Action |
|-----|--------|
| **Escape** | Close current panel / Return to previous screen |
| **Space** | Pause/Unpause game |
| **+/-** | Increase/Decrease game speed |
| **F1-F8** | Quick access to various panels (varies by version) |
| **Tab** | Cycle through fleets/characters |
| **Delete** | Scrap selected unit |

(Note: The game is primarily mouse-driven. Keyboard shortcuts are limited.)

## Appendix B: Galaxy Structure

**Core Sectors (Always 4):**
1. **Sesswenna Sector** --- Contains Coruscant (Imperial HQ)
2. **Sluis Sector**
3. **Farfin Sector**
4. **Fakir Sector**

Each core sector contains ~10 planets.

**Outer Rim Sectors (Variable: 6/11/15 based on galaxy size):**
- Named sectors containing ~10 planets each
- Planets start as unexplored (shown as stars on map)
- Some inhabited (have population and loyalty), some uninhabited
- Uninhabited planets: No loyalty bar, must be colonized by dropping a troop + building a structure
- Rebel HQ is hidden in one Outer Rim sector
- Yavin IV is always in the Sumitra sector

## Appendix C: Key Numerical Constants

| Constant | Value |
|----------|-------|
| Starting characters per side | 7 fixed + 2 random = 9 |
| Total recruitable per side | 28 |
| Maintenance per Mine/Refinery pair | 50 |
| Jedi Novice threshold | 10 Force Points |
| Jedi Trainee threshold | 20 Force Points |
| Jedi Student threshold | 80 Force Points |
| Jedi Knight threshold | 100 Force Points |
| Jedi Master threshold | 120 Force Points |
| Dagobah training trigger window | Days 200-500 |
| Dagobah training duration | ~100 days |
| Bounty Hunter check frequency | Every 1-100 days, 30% chance |
| Vader/Luke encounter Jedi Score bonus | 25% |
| Emperor/Luke encounter Jedi Score bonus | 20% |
| Final Battle Jedi Score threshold | 100 (for Luke to win) |
| Loyalty threshold for planet to join your side | ~60% |
| Max garrison requirement | 6 troops (standard), 12+ (during uprising) |
| Han Solo Millennium Falcon bonus | +50 hyperspace modifier |
| Emperor Leadership doubling | All Imperial Officer characters while Emperor is in Coruscant system |

## Appendix D: Sources

1. **Prima Official Strategy Guide** --- `https://files.swrebellion.net/Star_Wars_Rebellion_Guide_PRIMA.pdf`
2. **Game Manual** --- `https://files.swrebellion.net/Star_Wars_Rebellion-Manual.pdf` and `https://archive.org/details/star-wars-rebellion-manual`
3. **GameFAQs Strategy Guide by Rogue** --- `https://gamefaqs.gamespot.com/pc/198776-star-wars-rebellion/faqs/19754`
4. **Steam Complete Guide by Crownbreaker** --- `https://steamcommunity.com/sharedfiles/filedetails/?id=2796495125`
5. **CheatCC Unofficial Strategy Guide by xiriod** --- `https://www.cheatcc.com/articles/star-wars-rebellion-strategy-guide-pc-sg/`
6. **PCGamingWiki** --- `https://www.pcgamingwiki.com/wiki/Star_Wars:_Rebellion`
7. **Wookieepedia** --- `https://starwars.fandom.com/wiki/Star_Wars:_Rebellion_(video_game)`
8. **SWRebellion.net Community** --- `https://swrebellion.net/`
9. **LucasArts Official Rebellion Website (archived)** --- `https://web.archive.org/web/19990218124734/http://www.lucasarts.com/static/rebellion/rebellion.htm`
10. **Ross Walker Manual Mirror** --- `http://www.rosswalker.co.uk/rebellion.htm`
