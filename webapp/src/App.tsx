import { useState } from 'react';
import { useEngine } from './hooks/useEngine';
import { useGroupProfiles } from './hooks/useGroupProfiles';
import { Engine } from './wasm/engine';
import { CharacterList } from './components/CharacterList';
import { ProfileManager } from './components/ProfileManager';
import { MissionPlanner } from './components/MissionPlanner';
import { StatusBar } from './components/StatusBar';
import type { Faction, MissionKind } from './types/game';

type Tab = 'play' | 'profiles';

export function App() {
  const { ready, error, world, characters, systems, missions, refresh } = useEngine();
  const { profiles } = useGroupProfiles();

  const [tab, setTab] = useState<Tab>('play');
  const [factionFilter, setFactionFilter] = useState<Faction | 'All'>('All');
  const [systemFilter, setSystemFilter] = useState<number | 'All'>('All');
  const [plannerOpen, setPlannerOpen] = useState(false);
  const [plannerSelection, setPlannerSelection] = useState<number[]>([]);

  if (error) {
    return (
      <div className="app-error">
        <h1>Failed to load engine</h1>
        <pre>{error}</pre>
      </div>
    );
  }

  if (!ready || !world) {
    return (
      <div className="app-loading">
        <h1>Open Rebellion</h1>
        <p>Initializing simulation engine…</p>
      </div>
    );
  }

  const handleLaunchMission = (selectedIds: number[]) => {
    if (selectedIds.length === 0) return;
    setPlannerSelection(selectedIds);
    setPlannerOpen(true);
  };

  const handleDispatch = async (
    assignments: { characterId: number; mission: MissionKind; targetSystemId: number }[],
  ) => {
    for (const a of assignments) {
      try {
        await Engine.dispatchMission(a.characterId, a.targetSystemId, a.mission);
      } catch (e) {
        console.error('Failed to dispatch mission', a, e);
      }
    }
    await refresh();
  };

  const handleAdvance = async (days: number) => {
    await Engine.advanceDays(days);
    await refresh();
  };

  return (
    <div className="app">
      <header className="app-header">
        <div className="brand">
          <h1>Open Rebellion <span className="version">War Room</span></h1>
        </div>
        <nav className="tabs">
          <button className={tab === 'play' ? 'tab active' : 'tab'} onClick={() => setTab('play')}>
            🎮 Play
          </button>
          <button className={tab === 'profiles' ? 'tab active' : 'tab'} onClick={() => setTab('profiles')}>
            👥 Character Profiles ({profiles.filter((p) => p.enabled).length}/{profiles.length})
          </button>
        </nav>
      </header>

      <StatusBar
        world={world}
        missions={missions}
        onAdvanceDay={() => handleAdvance(1)}
        onAdvanceWeek={() => handleAdvance(7)}
      />

      <main className="app-main">
        {tab === 'play' && (
          <>
            <aside className="filters">
              <div className="filter-group">
                <label>Faction</label>
                <select
                  value={factionFilter}
                  onChange={(e) => setFactionFilter(e.target.value as Faction | 'All')}
                >
                  <option value="All">All Factions</option>
                  <option value="Alliance">Alliance</option>
                  <option value="Empire">Empire</option>
                </select>
              </div>
              <div className="filter-group">
                <label>Location</label>
                <select
                  value={systemFilter === 'All' ? 'All' : systemFilter.toString()}
                  onChange={(e) => setSystemFilter(e.target.value === 'All' ? 'All' : Number(e.target.value))}
                >
                  <option value="All">All Systems</option>
                  {systems.map((s) => (
                    <option key={s.id} value={s.id.toString()}>{s.name}</option>
                  ))}
                </select>
              </div>
              {missions.length > 0 && (
                <div className="mission-feed">
                  <h3>In Transit</h3>
                  <ul>
                    {missions.map((m) => (
                      <li key={m.id} className={`mission-item mission-${m.kind.toLowerCase()}`}>
                        <div className="mission-head">
                          <span className="mission-kind">{m.kind}</span>
                          <span className="mission-eta">arrives day {world.currentDay + m.ticksRemaining}</span>
                        </div>
                        <div className="mission-body">
                          <strong>{m.characterName}</strong> → {m.targetSystemName}
                        </div>
                        <progress max={m.totalTicks} value={m.totalTicks - m.ticksRemaining} />
                      </li>
                    ))}
                  </ul>
                </div>
              )}
            </aside>

            <section className="main-panel">
              <CharacterList
                characters={characters}
                systems={systems}
                factionFilter={factionFilter}
                systemFilter={systemFilter}
                onLaunchMission={handleLaunchMission}
              />
            </section>
          </>
        )}

        {tab === 'profiles' && (
          <section className="main-panel full-width">
            <ProfileManager characters={characters} />
          </section>
        )}
      </main>

      {plannerOpen && (
        <MissionPlanner
          selectedCharacterIds={plannerSelection}
          characters={characters}
          systems={systems}
          profiles={profiles}
          onDispatch={handleDispatch}
          onClose={() => setPlannerOpen(false)}
        />
      )}
    </div>
  );
}
