/**
 * Types mirroring the DTOs exposed by the `rebellion-web` WASM crate.
 * Keep these in sync with `crates/rebellion-web/src/lib.rs`.
 */

export type Faction = 'Alliance' | 'Empire';

export type SystemControl =
  | 'Uncontrolled'
  | 'Alliance'
  | 'Empire'
  | 'Contested'
  | 'Uprising';

export type MissionKind =
  | 'Diplomacy'
  | 'Espionage'
  | 'Sabotage'
  | 'Assassination'
  | 'Rescue'
  | 'Abduction'
  | 'InciteUprising'
  | 'Recruitment';

export const ALL_MISSION_KINDS: MissionKind[] = [
  'Diplomacy',
  'Espionage',
  'Sabotage',
  'Assassination',
  'Rescue',
  'Abduction',
  'InciteUprising',
  'Recruitment',
];

export interface Skill {
  base: number;
  variance: number;
}

export interface Character {
  id: number;
  name: string;
  faction: Faction;
  isMajor: boolean;
  onMission: boolean;
  onHiddenMission: boolean;
  isCaptive: boolean;
  currentSystemId: number | null;
  diplomacy: Skill;
  espionage: Skill;
  combat: Skill;
  leadership: Skill;
  loyalty: Skill;
}

export interface StarSystem {
  id: number;
  name: string;
  sectorId: number;
  control: SystemControl;
  popularityAlliance: number;
  popularityEmpire: number;
}

export interface ActiveMission {
  id: number;
  characterId: number;
  characterName: string;
  kind: MissionKind;
  targetSystemId: number;
  targetSystemName: string;
  ticksRemaining: number;
  totalTicks: number;
}

export interface WorldState {
  currentDay: number;
  characterCount: number;
  systemCount: number;
  activeMissionCount: number;
}
