import { useMemo } from 'react';
import type { Character, Faction, StarSystem } from '../types/game';
import { useSelection } from '../hooks/useSelection';
import { useHotkey } from '../hooks/useHotkeys';

interface Props {
  characters: Character[];
  systems: StarSystem[];
  factionFilter: Faction | 'All';
  systemFilter: number | 'All';
  onSelectionChange?: (selectedIds: number[]) => void;
  onLaunchMission?: (selectedIds: number[]) => void;
}

/**
 * Sortable, filterable, multi-selectable list of characters.
 *
 * Hotkeys:
 *   Ctrl+A   select all visible
 *   Escape   clear selection
 *   Enter    launch mission with selection
 *   Delete   clear selection (alias)
 */
export function CharacterList({
  characters,
  systems,
  factionFilter,
  systemFilter,
  onSelectionChange,
  onLaunchMission,
}: Props) {
  const visible = useMemo(() => {
    return characters
      .filter((c) => factionFilter === 'All' || c.faction === factionFilter)
      .filter((c) => systemFilter === 'All' || c.currentSystemId === systemFilter)
      .sort((a, b) => {
        // Majors first, then by name
        if (a.isMajor !== b.isMajor) return a.isMajor ? -1 : 1;
        return a.name.localeCompare(b.name);
      });
  }, [characters, factionFilter, systemFilter]);

  const visibleIds = useMemo(() => visible.map((c) => c.id), [visible]);
  const sel = useSelection<number>();

  useHotkey('ctrl+a', () => {
    sel.selectAll(visibleIds);
    onSelectionChange?.(visibleIds);
  }, [visibleIds, onSelectionChange]);

  useHotkey('escape', () => {
    sel.clear();
    onSelectionChange?.([]);
  });

  useHotkey('delete', () => {
    sel.clear();
    onSelectionChange?.([]);
  });

  useHotkey('enter', () => {
    if (sel.size > 0) onLaunchMission?.(sel.toArray());
  }, [sel.size]);

  const systemNameById = useMemo(() => {
    const m = new Map<number, string>();
    for (const s of systems) m.set(s.id, s.name);
    return m;
  }, [systems]);

  const onRowClick = (id: number, e: React.MouseEvent) => {
    sel.handleClick(id, e, visibleIds);
    // Defer to next tick so the selection state is updated first
    queueMicrotask(() => onSelectionChange?.([...sel.selected].concat(id).filter((v, i, a) => a.indexOf(v) === i)));
  };

  return (
    <section className="card">
      <header className="card-header">
        <h2>Characters</h2>
        <div className="badge">
          {visible.length} shown · {sel.size} selected
        </div>
        <div className="hotkey-hint">
          <kbd>Ctrl</kbd>+<kbd>A</kbd> select all · <kbd>Shift</kbd>+click range · <kbd>Esc</kbd> clear
        </div>
      </header>

      {visible.length === 0 ? (
        <div className="empty">No characters match these filters.</div>
      ) : (
        <table className="char-table">
          <thead>
            <tr>
              <th />
              <th>Name</th>
              <th>Faction</th>
              <th>Location</th>
              <th>Status</th>
              <th>Top Skills</th>
            </tr>
          </thead>
          <tbody>
            {visible.map((c) => {
              const selected = sel.isSelected(c.id);
              return (
                <tr
                  key={c.id}
                  className={`char-row ${selected ? 'selected' : ''} ${c.onMission ? 'on-mission' : ''}`}
                  onClick={(e) => onRowClick(c.id, e)}
                >
                  <td className="checkmark">{selected ? '✓' : ''}</td>
                  <td className="name">
                    {c.isMajor ? <span className="major-star" title="Major Character">★</span> : null}
                    {c.name}
                  </td>
                  <td>
                    <span className={`pill faction-${c.faction.toLowerCase()}`}>{c.faction}</span>
                  </td>
                  <td>
                    {c.currentSystemId != null
                      ? (systemNameById.get(c.currentSystemId) ?? '—')
                      : '—'}
                  </td>
                  <td className="status-col">
                    {c.isCaptive && <span className="status-tag captive">Captive</span>}
                    {c.onMission && <span className="status-tag mission">On Mission</span>}
                    {c.onHiddenMission && <span className="status-tag hidden">Hidden</span>}
                    {!c.isCaptive && !c.onMission && <span className="status-tag idle">Idle</span>}
                  </td>
                  <td className="skills-col">{topSkills(c).join(' · ')}</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      )}

      <footer className="card-footer">
        <button
          className="btn-primary"
          disabled={sel.size === 0}
          onClick={() => onLaunchMission?.(sel.toArray())}
        >
          Plan Mission ({sel.size} selected)
        </button>
        <button className="btn-ghost" onClick={() => sel.clear()} disabled={sel.size === 0}>
          Clear
        </button>
      </footer>
    </section>
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
