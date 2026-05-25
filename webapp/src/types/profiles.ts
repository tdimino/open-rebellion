/**
 * Character group profiles — a UI-layer feature that defines automatic
 * role assignments for groups of characters on missions.
 *
 * Example: "When Han Solo and Chewbacca are selected together,
 *           Han always goes as Sabotage, Chewie as Espionage."
 *
 * Profiles live entirely in the UI layer (localStorage). The Rust
 * engine knows nothing about them — it just receives normal mission
 * dispatch commands at the moment the player clicks "Send Mission".
 */

import type { MissionKind } from './game';

/** A single role assignment within a profile. */
export interface RoleAssignment {
  /** Character ID (matches Character.id from the engine). */
  characterId: number;
  /** Character name (cached for UI display when character is offline). */
  characterName: string;
  /** Which mission kind this character should take. */
  preferredMission: MissionKind;
}

/** Match strictness modes for a group profile. */
export type MatchMode =
  | 'exact'      // selection must contain EXACTLY the listed members
  | 'subset'     // selection must contain ALL listed members (may have more)
  | 'any';       // selection must contain ANY of the listed members

/** A persistent profile defining auto-assignments for a character group. */
export interface GroupProfile {
  /** Stable UUID. */
  id: string;
  /** Player-given name (e.g., "Han + Chewie Smuggling Team"). */
  name: string;
  /** Optional description / notes. */
  description?: string;
  /** Required character IDs. Selection match logic depends on `matchMode`. */
  requiredMembers: number[];
  /** Per-character role assignments. Members without an assignment use the
   *  player's default for the trip. */
  roleAssignments: RoleAssignment[];
  /** How the selection must match `requiredMembers`. */
  matchMode: MatchMode;
  /** If false, profile is ignored even when it would match. */
  enabled: boolean;
  /** Optional: only apply when targeting these systems. Empty = all. */
  restrictedToSystemIds?: number[];
  /** Optional: only apply for these mission kinds. Empty = all. */
  restrictedToMissionKinds?: MissionKind[];
  /** UNIX timestamp the profile was created. */
  createdAt: number;
  /** UNIX timestamp the profile was last edited. */
  updatedAt: number;
}

/** Result of matching a player's selection against a set of profiles. */
export interface ProfileMatch {
  profile: GroupProfile;
  /** Character IDs the profile auto-assigns roles for, in this selection. */
  matchedCharacterIds: number[];
}

/** Determine which profiles match a given selection. */
export function matchProfiles(
  profiles: GroupProfile[],
  selectedCharacterIds: number[],
): ProfileMatch[] {
  const selected = new Set(selectedCharacterIds);
  const matches: ProfileMatch[] = [];

  for (const profile of profiles) {
    if (!profile.enabled) continue;
    if (profile.requiredMembers.length === 0) continue;

    const required = new Set(profile.requiredMembers);
    let matched: number[] = [];

    switch (profile.matchMode) {
      case 'exact':
        if (
          required.size === selected.size &&
          [...required].every((id) => selected.has(id))
        ) {
          matched = profile.requiredMembers;
        }
        break;
      case 'subset':
        if ([...required].every((id) => selected.has(id))) {
          matched = profile.requiredMembers.filter((id) => selected.has(id));
        }
        break;
      case 'any':
        matched = profile.requiredMembers.filter((id) => selected.has(id));
        if (matched.length === 0) continue;
        break;
    }

    if (matched.length > 0) {
      matches.push({ profile, matchedCharacterIds: matched });
    }
  }

  return matches;
}

/** Compute the effective role for each selected character given matched profiles
 *  and any player overrides. Later profiles in the array take precedence. */
export function computeEffectiveRoles(
  selectedCharacterIds: number[],
  matches: ProfileMatch[],
  overrides: Map<number, MissionKind>,
): Map<number, { mission: MissionKind | null; source: 'profile' | 'override' | 'unset'; profileName?: string }> {
  const result = new Map<
    number,
    { mission: MissionKind | null; source: 'profile' | 'override' | 'unset'; profileName?: string }
  >();

  for (const charId of selectedCharacterIds) {
    result.set(charId, { mission: null, source: 'unset' });
  }

  // Apply matched profiles in order
  for (const match of matches) {
    for (const role of match.profile.roleAssignments) {
      if (!result.has(role.characterId)) continue;
      result.set(role.characterId, {
        mission: role.preferredMission,
        source: 'profile',
        profileName: match.profile.name,
      });
    }
  }

  // Apply user overrides (highest priority)
  for (const [charId, mission] of overrides) {
    if (!result.has(charId)) continue;
    result.set(charId, { mission, source: 'override' });
  }

  return result;
}
