import { useMemo, useState } from 'react';
import type { Character, StarSystem, MissionKind } from '../types/game';
import { ALL_MISSION_KINDS } from '../types/game';
import type { GroupProfile } from '../types/profiles';
import { matchProfiles, computeEffectiveRoles } from '../types/profiles';

interface Props {
  selectedCharacterIds: number[];
  characters: Character[];
  systems: StarSystem[];
  profiles: GroupProfile[];
  onDispatch: (assignments: { characterId: number; mission: MissionKind; targetSystemId: number }[]) => Promise<void>;
  onClose: () => void;
}

/**
 * Mission Planner modal — shows auto-applied profiles, allows overrides,
 * sends a batch of dispatch commands when player confirms.
 */
export function MissionPlanner({
  selectedCharacterIds,
  characters,
  systems,
  profiles,
  onDispatch,
  onClose,
}: Props) {
  const selectedChars = useMemo(
    () => characters.filter((c) => selectedCharacterIds.includes(c.id)),
    [selectedCharacterIds, characters],
  );

  const [targetSystemId, setTargetSystemId] = useState<number | null>(systems[0]?.id ?? null);
  const [overrides, setOverrides] = useState<Map<number, MissionKind>>(new Map());
  const [dispatching, setDispatching] = useState(false);

  // Match profiles against the current selection
  const matches = useMemo(
    () => matchProfiles(profiles, selectedCharacterIds),
    [profiles, selectedCharacterIds],
  );

  // Compute effective role for each character
  const effectiveRoles = useMemo(
    () => computeEffectiveRoles(selectedCharacterIds, matches, overrides),
    [selectedCharacterIds, matches, overrides],
  );

  const setOverride = (charId: number, mission: MissionKind) => {
    setOverrides((prev) => new Map(prev).set(charId, mission));
  };

  const clearOverride = (charId: number) => {
    setOverrides((prev) => {
      const next = new Map(prev);
      next.delete(charId);
      return next;
    });
  };

  const allCharactersHaveMission = selectedChars.every((c) => effectiveRoles.get(c.id)?.mission != null);
  const canDispatch = targetSystemId != null && allCharactersHaveMission && !dispatching;

  const handleDispatch = async () => {
    if (targetSystemId == null) return;
    const assignments = selectedChars
      .map((c) => ({
        characterId: c.id,
        mission: effectiveRoles.get(c.id)!.mission!,
        targetSystemId,
      }))
      .filter((a) => a.mission != null);
    setDispatching(true);
    try {
      await onDispatch(assignments);
      onClose();
    } finally {
      setDispatching(false);
    }
  };

  return (
    <div className="modal-backdrop">
      <div className="modal modal-wide">
        <header className="modal-header">
          <h3>Mission Planner</h3>
          <button className="btn-ghost" onClick={onClose}>✕</button>
        </header>

        <div className="modal-body">
          <label className="field">
            <span>Target System</span>
            <select
              value={targetSystemId ?? ''}
              onChange={(e) => setTargetSystemId(Number(e.target.value))}
            >
              {systems.map((s) => (
                <option key={s.id} value={s.id}>
                  {s.name} ({s.control})
                </option>
              ))}
            </select>
          </label>

          {matches.length > 0 && (
            <div className="profile-applied-banner">
              ✨ <strong>{matches.length} profile{matches.length === 1 ? '' : 's'} matched:</strong>{' '}
              {matches.map((m) => m.profile.name).join(', ')}
            </div>
          )}

          <table className="planner-table">
            <thead>
              <tr>
                <th>Character</th>
                <th>Best Skills</th>
                <th>Mission</th>
                <th>Source</th>
                <th />
              </tr>
            </thead>
            <tbody>
              {selectedChars.map((c) => {
                const eff = effectiveRoles.get(c.id);
                return (
                  <tr key={c.id} className={c.onMission ? 'on-mission-warn' : ''}>
                    <td>
                      <span className={`pill faction-${c.faction.toLowerCase()}`}>{c.faction[0]}</span>
                      {c.isMajor && <span className="major-star">★</span>}
                      {c.name}
                      {c.onMission && <span className="status-tag mission inline">already busy!</span>}
                    </td>
                    <td className="skills-col">{topSkills(c).join(' · ')}</td>
                    <td>
                      <select
                        value={eff?.mission ?? ''}
                        onChange={(e) => setOverride(c.id, e.target.value as MissionKind)}
                      >
                        <option value="">— pick —</option>
                        {ALL_MISSION_KINDS.map((m) => (
                          <option key={m} value={m}>{m}</option>
                        ))}
                      </select>
                    </td>
                    <td className="source-col">
                      {eff?.source === 'profile' && (
                        <span className="source-tag profile" title={`Profile: ${eff.profileName}`}>
                          ⭐ {eff.profileName}
                        </span>
                      )}
                      {eff?.source === 'override' && (
                        <span className="source-tag override">manual</span>
                      )}
                      {eff?.source === 'unset' && (
                        <span className="source-tag unset">choose mission</span>
                      )}
                    </td>
                    <td>
                      {overrides.has(c.id) && (
                        <button className="btn-link" onClick={() => clearOverride(c.id)}>
                          revert to profile
                        </button>
                      )}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>

          {!allCharactersHaveMission && (
            <div className="warning">
              ⚠ Pick a mission for every selected character before dispatching.
            </div>
          )}
        </div>

        <footer className="modal-footer">
          <button className="btn-ghost" onClick={onClose}>Cancel</button>
          <button className="btn-primary" disabled={!canDispatch} onClick={handleDispatch}>
            {dispatching ? 'Dispatching…' : `Dispatch ${selectedChars.length} Mission${selectedChars.length === 1 ? '' : 's'}`}
          </button>
        </footer>
      </div>
    </div>
  );
}

function topSkills(c: Character): string[] {
  const skills = [
    ['Dipl', c.diplomacy.base],
    ['Espn', c.espionage.base],
    ['Combat', c.combat.base],
    ['Lead', c.leadership.base],
  ] as const;
  return [...skills]
    .sort((a, b) => b[1] - a[1])
    .slice(0, 2)
    .map(([n, v]) => `${n} ${v}`);
}
