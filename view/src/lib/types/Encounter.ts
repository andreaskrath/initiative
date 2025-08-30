import type { EncounterEntity } from "./EncounterEntity";
import type { PlayerEntity } from "./PlayerEntity";

export type Encounter = {
  id?: string;
  entities: EncounterEntity[];
};

export const EncounterActions = {
  EmptyEncounter: (): Encounter => ({
    id: undefined,
    entities: [],
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
