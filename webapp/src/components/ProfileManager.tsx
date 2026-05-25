import { useState } from 'react';
import type { GroupProfile, MatchMode, RoleAssignment } from '../types/profiles';
import { ALL_MISSION_KINDS, type Character, type MissionKind } from '../types/game';
import { useGroupProfiles } from '../hooks/useGroupProfiles';

interface Props {
  characters: Character[];
}

/** Top-level profile management UI: list, create, edit, delete, import/export. */
export function ProfileManager({ characters }: Props) {
  const { profiles, createProfile, updateProfile, deleteProfile, toggleEnabled, exportJson, importJson } =
    useGroupProfiles();
  const [editing, setEditing] = useState<GroupProfile | null>(null);
  const [creating, setCreating] = useState(false);

  const charById = new Map(characters.map((c) => [c.id, c]));

  return (
    <section className="card">
      <header className="card-header">
        <h2>Character Group Profiles</h2>
        <div className="badge">{profiles.length} total · {profiles.filter((p) => p.enabled).length} enabled</div>
      </header>

      {profiles.length === 0 ? (
        <div className="empty">
          No profiles defined yet.
          <button className="btn-primary" onClick={() => setCreating(true)}>+ Create First Profile</button>
        </div>
      ) : (
        <ul className="profile-list">
          {profiles.map((p) => (
            <li key={p.id} className={`profile-card ${p.enabled ? 'enabled' : 'disabled'}`}>
              <div className="profile-card-head">
                <label className="profile-toggle">
                  <input type="checkbox" checked={p.enabled} onChange={() => toggleEnabled(p.id)} />
                  <span className="profile-name">{p.name}</span>
                </label>
                <span className="profile-mode-badge">{p.matchMode}</span>
                <div className="profile-actions">
                  <button className="btn-ghost" onClick={() => setEditing(p)}>Edit</button>
                  <button className="btn-danger" onClick={() => {
                    if (confirm(`Delete profile "${p.name}"?`)) deleteProfile(p.id);
                  }}>Delete</button>
                </div>
              </div>
              {p.description && <div className="profile-desc">{p.description}</div>}
              <div className="profile-members">
                <span className="dim">When selection contains:</span>
                {p.requiredMembers.map((id) => (
                  <span key={id} className="member-chip">{charById.get(id)?.name ?? `#${id}`}</span>
                ))}
              </div>
              <div className="profile-roles">
                <span className="dim">Auto-assign:</span>
                {p.roleAssignments.map((r) => (
                  <span key={r.characterId} className={`role-chip role-${r.preferredMission.toLowerCase()}`}>
                    {r.characterName} → {r.preferredMission}
                  </span>
                ))}
              </div>
            </li>
          ))}
        </ul>
      )}

      <footer className="card-footer">
        <button className="btn-primary" onClick={() => setCreating(true)}>+ New Profile</button>
        <button className="btn-ghost" onClick={() => {
          const text = exportJson();
          navigator.clipboard.writeText(text);
          alert(`Exported ${profiles.length} profiles to clipboard.`);
        }}>Export to Clipboard</button>
        <button className="btn-ghost" onClick={() => {
          const text = prompt('Paste profile JSON:');
          if (text) {
            try {
              importJson(text);
              alert('Profiles imported.');
            } catch (e) {
              alert(`Import failed: ${e instanceof Error ? e.message : e}`);
            }
          }
        }}>Import from Clipboard</button>
      </footer>

      {(creating || editing) && (
        <ProfileEditor
          characters={characters}
          initial={editing}
          onSave={(profile) => {
            if (editing) updateProfile(editing.id, profile);
            else createProfile(profile);
            setCreating(false);
            setEditing(null);
          }}
          onCancel={() => {
            setCreating(false);
            setEditing(null);
          }}
        />
      )}
    </section>
  );
}

// ─────────────────────────────────────────────────────────────────────────
// Editor modal
// ─────────────────────────────────────────────────────────────────────────

interface EditorProps {
  characters: Character[];
  initial: GroupProfile | null;
  onSave: (profile: Omit<GroupProfile, 'id' | 'createdAt' | 'updatedAt'>) => void;
  onCancel: () => void;
}

function ProfileEditor({ characters, initial, onSave, onCancel }: EditorProps) {
  const [name, setName] = useState(initial?.name ?? '');
  const [description, setDescription] = useState(initial?.description ?? '');
  const [matchMode, setMatchMode] = useState<MatchMode>(initial?.matchMode ?? 'subset');
  const [requiredMembers, setRequiredMembers] = useState<number[]>(initial?.requiredMembers ?? []);
  const [roles, setRoles] = useState<RoleAssignment[]>(initial?.roleAssignments ?? []);
  const [enabled, setEnabled] = useState(initial?.enabled ?? true);

  const charById = new Map(characters.map((c) => [c.id, c]));

  const toggleMember = (id: number) => {
    setRequiredMembers((prev) => {
      const next = prev.includes(id) ? prev.filter((x) => x !== id) : [...prev, id];
      // When removing, also remove from role assignments
      if (!next.includes(id)) {
        setRoles((rs) => rs.filter((r) => r.characterId !== id));
      } else if (!roles.some((r) => r.characterId === id)) {
        // Add default role assignment when newly added
        const c = charById.get(id);
        if (c) {
          setRoles((rs) => [...rs, {
            characterId: id,
            characterName: c.name,
            preferredMission: bestSkillToMission(c),
          }]);
        }
      }
      return next;
    });
  };

  const setRoleFor = (charId: number, mission: MissionKind) => {
    setRoles((prev) => prev.map((r) => r.characterId === charId ? { ...r, preferredMission: mission } : r));
  };

  const canSave = name.trim().length > 0 && requiredMembers.length > 0;

  return (
    <div className="modal-backdrop">
      <div className="modal">
        <header className="modal-header">
          <h3>{initial ? 'Edit Profile' : 'New Profile'}</h3>
          <button className="btn-ghost" onClick={onCancel}>✕</button>
        </header>

        <div className="modal-body">
          <label className="field">
            <span>Name</span>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder='e.g., "Han + Chewie Smuggling Team"'
            />
          </label>

          <label className="field">
            <span>Description (optional)</span>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="When and why does this profile apply?"
              rows={2}
            />
          </label>

          <label className="field">
            <span>Match Mode</span>
            <select value={matchMode} onChange={(e) => setMatchMode(e.target.value as MatchMode)}>
              <option value="subset">Subset — selection contains ALL listed members</option>
              <option value="exact">Exact — selection is EXACTLY these members</option>
              <option value="any">Any — selection contains ANY listed member</option>
            </select>
          </label>

          <fieldset className="field">
            <legend>Required Members</legend>
            <div className="member-grid">
              {characters
                .sort((a, b) => Number(b.isMajor) - Number(a.isMajor) || a.name.localeCompare(b.name))
                .map((c) => (
                  <label key={c.id} className={`member-pick ${requiredMembers.includes(c.id) ? 'picked' : ''}`}>
                    <input
                      type="checkbox"
                      checked={requiredMembers.includes(c.id)}
                      onChange={() => toggleMember(c.id)}
                    />
                    <span className={`pill faction-${c.faction.toLowerCase()}`}>{c.faction[0]}</span>
                    {c.isMajor && <span className="major-star">★</span>}
                    <span>{c.name}</span>
                  </label>
                ))}
            </div>
          </fieldset>

          {roles.length > 0 && (
            <fieldset className="field">
              <legend>Auto-Assignments</legend>
              <table className="role-table">
                <thead>
                  <tr><th>Character</th><th>Preferred Mission</th></tr>
                </thead>
                <tbody>
                  {roles.map((r) => (
                    <tr key={r.characterId}>
                      <td>{r.characterName}</td>
                      <td>
                        <select
                          value={r.preferredMission}
                          onChange={(e) => setRoleFor(r.characterId, e.target.value as MissionKind)}
                        >
                          {ALL_MISSION_KINDS.map((m) => (
                            <option key={m} value={m}>{m}</option>
                          ))}
                        </select>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </fieldset>
          )}

          <label className="field-inline">
            <input type="checkbox" checked={enabled} onChange={(e) => setEnabled(e.target.checked)} />
            <span>Enabled</span>
          </label>
        </div>

        <footer className="modal-footer">
          <button className="btn-ghost" onClick={onCancel}>Cancel</button>
          <button
            className="btn-primary"
            disabled={!canSave}
            onClick={() => onSave({
              name: name.trim(),
              description: description.trim() || undefined,
              matchMode,
              requiredMembers,
              roleAssignments: roles,
              enabled,
            })}
          >
            {initial ? 'Save Changes' : 'Create Profile'}
          </button>
        </footer>
      </div>
    </div>
  );
}

function bestSkillToMission(c: Character): MissionKind {
  const scores: [MissionKind, number][] = [
    ['Diplomacy', c.diplomacy.base],
    ['Espionage', c.espionage.base],
    ['Sabotage', c.espionage.base * 0.7 + c.combat.base * 0.3],
    ['Assassination', c.combat.base],
    ['Recruitment', c.diplomacy.base * 0.6 + c.leadership.base * 0.4],
  ];
  scores.sort((a, b) => b[1] - a[1]);
  return scores[0][0];
}
