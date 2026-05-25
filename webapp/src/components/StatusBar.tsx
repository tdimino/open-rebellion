import type { WorldState, ActiveMission } from '../types/game';

interface Props {
  world: WorldState | null;
  missions: ActiveMission[];
  onAdvanceDay: () => void;
  onAdvanceWeek: () => void;
}

export function StatusBar({ world, missions, onAdvanceDay, onAdvanceWeek }: Props) {
  if (!world) return null;

  const inflight = missions.length;
  const nextArrival = missions
    .slice()
    .sort((a, b) => a.ticksRemaining - b.ticksRemaining)[0];

  return (
    <div className="status-bar">
      <div className="status-item">
        <span className="status-label">Day</span>
        <span className="status-value">{world.currentDay}</span>
      </div>
      <div className="status-item">
        <span className="status-label">Characters</span>
        <span className="status-value">{world.characterCount}</span>
      </div>
      <div className="status-item">
        <span className="status-label">Systems</span>
        <span className="status-value">{world.systemCount}</span>
      </div>
      <div className="status-item">
        <span className="status-label">Active Missions</span>
        <span className="status-value">{inflight}</span>
      </div>
      {nextArrival && (
        <div className="status-item">
          <span className="status-label">Next Arrival</span>
          <span className="status-value">
            {nextArrival.characterName} → {nextArrival.targetSystemName} in {nextArrival.ticksRemaining}d
          </span>
        </div>
      )}
      <div className="status-spacer" />
      <button className="btn-ghost" onClick={onAdvanceDay}>+1 day</button>
      <button className="btn-ghost" onClick={onAdvanceWeek}>+7 days</button>
    </div>
  );
}
