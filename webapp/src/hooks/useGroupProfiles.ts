import { useCallback, useEffect, useState } from 'react';
import type { GroupProfile, MatchMode, RoleAssignment } from '../types/profiles';

const STORAGE_KEY = 'open-rebellion.groupProfiles.v1';

/** Persistent group profile storage + CRUD. */
export function useGroupProfiles() {
  const [profiles, setProfiles] = useState<GroupProfile[]>(() => loadProfiles());

  useEffect(() => {
    saveProfiles(profiles);
  }, [profiles]);

  const createProfile = useCallback(
    (init: Omit<GroupProfile, 'id' | 'createdAt' | 'updatedAt'>) => {
      const now = Date.now();
      const profile: GroupProfile = {
        id: makeId(),
        createdAt: now,
        updatedAt: now,
        ...init,
      };
      setProfiles((prev) => [...prev, profile]);
      return profile;
    },
    [],
  );

  const updateProfile = useCallback((id: string, patch: Partial<GroupProfile>) => {
    setProfiles((prev) =>
      prev.map((p) => (p.id === id ? { ...p, ...patch, updatedAt: Date.now() } : p)),
    );
  }, []);

  const deleteProfile = useCallback((id: string) => {
    setProfiles((prev) => prev.filter((p) => p.id !== id));
  }, []);

  const toggleEnabled = useCallback((id: string) => {
    setProfiles((prev) =>
      prev.map((p) =>
        p.id === id ? { ...p, enabled: !p.enabled, updatedAt: Date.now() } : p,
      ),
    );
  }, []);

  const exportJson = useCallback(() => JSON.stringify(profiles, null, 2), [profiles]);

  const importJson = useCallback((json: string) => {
    try {
      const parsed = JSON.parse(json) as GroupProfile[];
      if (!Array.isArray(parsed)) throw new Error('Expected array');
      setProfiles((prev) => {
        // De-duplicate by id (incoming overrides)
        const byId = new Map(prev.map((p) => [p.id, p]));
        for (const p of parsed) byId.set(p.id, p);
        return [...byId.values()];
      });
    } catch (e) {
      console.error('Failed to import profiles', e);
      throw e;
    }
  }, []);

  const clearAll = useCallback(() => setProfiles([]), []);

  return {
    profiles,
    createProfile,
    updateProfile,
    deleteProfile,
    toggleEnabled,
    exportJson,
    importJson,
    clearAll,
  };
}

// Helpers

function loadProfiles(): GroupProfile[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return SEED_PROFILES;
    const parsed = JSON.parse(raw) as GroupProfile[];
    return Array.isArray(parsed) ? parsed : SEED_PROFILES;
  } catch {
    return SEED_PROFILES;
  }
}

function saveProfiles(profiles: GroupProfile[]) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(profiles));
  } catch (e) {
    console.warn('Failed to save profiles', e);
  }
}

function makeId() {
  return `gp-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

// Seed profiles: famous canonical pairings, ready out of the box.
// (Character IDs match the demo world built by `crates/rebellion-web/src/demo_world.rs`.)
const SEED_PROFILES: GroupProfile[] = [
  {
    id: 'seed-han-chewie',
    name: 'Han + Chewie Smuggling Team',
    description: 'The classic pair. Han runs sabotage, Chewie scouts.',
    requiredMembers: [3, 9], // Han Solo, Chewbacca
    roleAssignments: [
      { characterId: 3, characterName: 'Han Solo', preferredMission: 'Sabotage' },
      { characterId: 9, characterName: 'Chewbacca', preferredMission: 'Espionage' },
    ],
    matchMode: 'subset',
    enabled: true,
    createdAt: 0,
    updatedAt: 0,
  },
  {
    id: 'seed-leia-mothma',
    name: 'Diplomatic Corps (Leia + Mothma)',
    description: 'Both top-tier diplomats — split to cover two systems faster.',
    requiredMembers: [0, 1], // Mon Mothma, Leia
    roleAssignments: [
      { characterId: 0, characterName: 'Mon Mothma', preferredMission: 'Diplomacy' },
      { characterId: 1, characterName: 'Leia Organa', preferredMission: 'Diplomacy' },
    ],
    matchMode: 'subset',
    enabled: true,
    createdAt: 0,
    updatedAt: 0,
  },
  {
    id: 'seed-luke-strike',
    name: 'Luke Strike Force',
    description: 'When Luke pairs with Wedge or Lando — Luke handles combat ops.',
    requiredMembers: [2], // Luke Skywalker
    roleAssignments: [
      { characterId: 2, characterName: 'Luke Skywalker', preferredMission: 'Assassination' },
    ],
    matchMode: 'subset',
    enabled: false, // off by default — power user feature
    createdAt: 0,
    updatedAt: 0,
  },
];
