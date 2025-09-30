import type { EncounterEntity } from "./EncounterEntity";
import type { PlayerEntity } from "./PlayerEntity";

export type Encounter = {
  id?: string;
  name?: string;
  entities: EncounterEntity[];
  active: number;
};

export const EncounterActions = {
  EmptyEncounter: (): Encounter => ({
    id: undefined,
    name: undefined,
    entities: [],
    active: 0,
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
