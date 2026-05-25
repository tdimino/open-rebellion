/**
 * TypeScript wrapper around the `rebellion-web` WASM module.
 *
 * When wasm-pack runs `npm run build-wasm`, it generates:
 *   webapp/src/wasm/pkg/rebellion_web.js  — JS glue
 *   webapp/src/wasm/pkg/rebellion_web.d.ts — TS types
 *   webapp/src/wasm/pkg/rebellion_web_bg.wasm — actual WASM
 *
 * This file imports those and provides a clean async API for React.
 */

import type {
  Character,
  StarSystem,
  ActiveMission,
  WorldState,
  MissionKind,
} from '../types/game';

// Lazy-loaded WASM module reference
type WasmModule = {
  init_demo_world: () => void;
  world_loaded: () => boolean;
  get_world_state: () => WorldState;
  get_characters: () => Character[];
  get_characters_on_system: (systemId: number) => Character[];
  get_systems: () => StarSystem[];
  get_active_missions: () => ActiveMission[];
  dispatch_mission: (charId: number, targetSystemId: number, kind: string) => bigint;
  advance_days: (n: number) => void;
};

let wasmModule: WasmModule | null = null;
let initPromise: Promise<void> | null = null;

async function ensureWasmLoaded(): Promise<WasmModule> {
  if (wasmModule) return wasmModule;
  if (!initPromise) {
    initPromise = (async () => {
      try {
        // wasm-pack generated module
        // @ts-expect-error - generated at build time
        const mod = await import('./pkg/rebellion_web.js');
        await mod.default();
        wasmModule = mod as unknown as WasmModule;
        wasmModule.init_demo_world();
      } catch (err) {
        console.warn('[wasm] real module not available, using mock', err);
        wasmModule = createMockEngine();
        wasmModule.init_demo_world();
      }
    })();
  }
  await initPromise;
  return wasmModule!;
}

/** Public API used by React components. */
export const Engine = {
  async ready(): Promise<void> {
    await ensureWasmLoaded();
  },

  async getWorldState(): Promise<WorldState> {
    const w = await ensureWasmLoaded();
    return w.get_world_state();
  },

  async getCharacters(): Promise<Character[]> {
    const w = await ensureWasmLoaded();
    return w.get_characters();
  },

  async getCharactersOnSystem(systemId: number): Promise<Character[]> {
    const w = await ensureWasmLoaded();
    return w.get_characters_on_system(systemId);
  },

  async getSystems(): Promise<StarSystem[]> {
    const w = await ensureWasmLoaded();
    return w.get_systems();
  },

  async getActiveMissions(): Promise<ActiveMission[]> {
    const w = await ensureWasmLoaded();
    return w.get_active_missions();
  },

  async dispatchMission(
    characterId: number,
    targetSystemId: number,
    kind: MissionKind,
  ): Promise<bigint> {
    const w = await ensureWasmLoaded();
    return w.dispatch_mission(characterId, targetSystemId, kind);
  },

  async advanceDays(n: number): Promise<void> {
    const w = await ensureWasmLoaded();
    w.advance_days(n);
  },
};

// ─────────────────────────────────────────────────────────────────────────
// Mock engine — used during local UI development before the WASM build
// is available. Mirrors the demo world the Rust crate builds.
// ─────────────────────────────────────────────────────────────────────────

function createMockEngine(): WasmModule {
  let currentDay = 192;
  let nextMissionId = 1n;

  const characters: Character[] = [
    // Alliance majors
    char(0, 'Mon Mothma', 'Alliance', true, 1),
    char(1, 'Leia Organa', 'Alliance', true, 1),
    char(2, 'Luke Skywalker', 'Alliance', true, 1),
    char(3, 'Han Solo', 'Alliance', true, 1),
    // Empire majors
    char(4, 'Emperor Palpatine', 'Empire', true, 0),
    char(5, 'Darth Vader', 'Empire', true, 0),
    // Alliance minors
    char(6, 'Admiral Ackbar', 'Alliance', false, 1),
    char(7, 'Wedge Antilles', 'Alliance', false, 1),
    char(8, 'Lando Calrissian', 'Alliance', false, 1),
    char(9, 'Chewbacca', 'Alliance', false, 1),
    char(10, 'Jan Dodonna', 'Alliance', false, 1),
    // Empire minors
    char(11, 'Admiral Ozzel', 'Empire', false, 0),
    char(12, 'Admiral Piett', 'Empire', false, 0),
    char(13, 'General Grammel', 'Empire', false, 0),
    char(14, 'Grand Admiral Thrawn', 'Empire', false, 0),
    char(15, 'Admiral Daala', 'Empire', false, 0),
  ];

  const systems: StarSystem[] = [
    { id: 0, name: 'Coruscant', sectorId: 0, control: 'Empire', popularityAlliance: 0.25, popularityEmpire: 0.75 },
    { id: 1, name: 'Yavin', sectorId: 0, control: 'Alliance', popularityAlliance: 0.75, popularityEmpire: 0.25 },
    { id: 2, name: 'Hoth', sectorId: 0, control: 'Alliance', popularityAlliance: 0.75, popularityEmpire: 0.25 },
    { id: 3, name: 'Tatooine', sectorId: 0, control: 'Empire', popularityAlliance: 0.25, popularityEmpire: 0.75 },
    { id: 4, name: 'Bortras', sectorId: 0, control: 'Empire', popularityAlliance: 0.25, popularityEmpire: 0.75 },
  ];

  const missions: ActiveMission[] = [];

  return {
    init_demo_world() {},
    world_loaded() { return true; },
    get_world_state(): WorldState {
      return {
        currentDay,
        characterCount: characters.length,
        systemCount: systems.length,
        activeMissionCount: missions.length,
      };
    },
    get_characters() { return [...characters]; },
    get_characters_on_system(sys: number) {
      return characters.filter((c) => c.currentSystemId === sys);
    },
    get_systems() { return [...systems]; },
    get_active_missions() { return [...missions]; },
    dispatch_mission(charId, targetSys, kind) {
      const c = characters.find((c) => c.id === charId);
      const s = systems.find((s) => s.id === targetSys);
      if (!c || !s) throw new Error('not found');
      const id = nextMissionId++;
      const dur = synthDuration(kind as MissionKind);
      missions.push({
        id: Number(id),
        characterId: charId,
        characterName: c.name,
        kind: kind as MissionKind,
        targetSystemId: targetSys,
        targetSystemName: s.name,
        ticksRemaining: dur,
        totalTicks: dur,
      });
      c.onMission = true;
      return id;
    },
    advance_days(n) {
      currentDay += n;
      for (const m of missions) {
        m.ticksRemaining = Math.max(0, m.ticksRemaining - n);
      }
      const completed = missions.filter((m) => m.ticksRemaining === 0).map((m) => m.characterId);
      for (let i = missions.length - 1; i >= 0; i--) {
        if (missions[i].ticksRemaining === 0) missions.splice(i, 1);
      }
      for (const cid of completed) {
        const c = characters.find((c) => c.id === cid);
        if (c) c.onMission = false;
      }
    },
  };
}

function char(
  id: number, name: string, faction: 'Alliance' | 'Empire', isMajor: boolean, locationId: number,
): Character {
  return {
    id, name, faction, isMajor,
    onMission: false, onHiddenMission: false, isCaptive: false,
    currentSystemId: locationId,
    diplomacy: { base: 50, variance: 10 },
    espionage: { base: 50, variance: 10 },
    combat: { base: 50, variance: 10 },
    leadership: { base: 50, variance: 10 },
    loyalty: { base: 80, variance: 10 },
  };
}

function synthDuration(k: MissionKind): number {
  return ({
    Diplomacy: 10, Espionage: 8, Sabotage: 12, Assassination: 15,
    Rescue: 14, Abduction: 16, InciteUprising: 20, Recruitment: 7,
  } as Record<MissionKind, number>)[k];
}
