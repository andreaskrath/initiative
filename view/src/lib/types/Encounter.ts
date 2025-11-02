import type { EncounterEntity } from "./EncounterEntity";
import type { PlayerEntity } from "./PlayerEntity";
import type { Monster } from "./Monster";

export type Encounter = {
  id?: string;
  name?: string;
  entities: EncounterEntity[];
  active: number;
  monsters: Record<string, Monster>;
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
