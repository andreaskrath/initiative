import type { EncounterEntity } from "./EncounterEntity";
import type { PlayerEntity } from "./PlayerEntity";
import type { Monster } from "./Monster";

export type Wall = {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
};

export type EntityPosition = {
  entityId: string;
  x: number;
  y: number;
};

export type BattleMap = {
  gridSize: number; // Size of each grid square in pixels
  cellSize: number; // Size of each cell in feet (typically 5 for D&D)
  width: number; // Width in grid cells
  height: number; // Height in grid cells
  walls: Wall[];
  positions: EntityPosition[];
};

export type Encounter = {
  id?: string;
  name?: string;
  entities: EncounterEntity[];
  active: number;
  monsters: Record<string, Monster>;
  battleMap?: BattleMap;
};

export const EncounterActions = {
  EmptyEncounter: (): Encounter => ({
    id: undefined,
    name: undefined,
    entities: [],
    active: 0,
    monsters: {},
  }),

  AddPlayer: (encounter: Encounter, player: PlayerEntity): void => {
    encounter.entities.push(player);
  },

  RemoveEntity: (
    encounter: Encounter,
    entityToRemove: EncounterEntity,
    event: Event,
  ): void => {
    event.preventDefault();

    encounter.entities = encounter.entities.filter(
      (entity) => entity !== entityToRemove,
    );
  },
};
