import type { EncounterEntity, MonsterEntity } from "./Entities";
import type { Monster } from "./Monster";

export type Encounter = {
  id?: string;
  entities: EncounterEntity[];
};

export const EncounterActions = {
  EmptyEncounter: (): Encounter => ({
    id: undefined,
    entities: [],
  }),

  AddMonster: (
    encounter: Encounter,
    monster: Monster,
    event: MouseEvent,
  ): void => {
    event.preventDefault();

    const monsterEntity: MonsterEntity = {
      id: undefined,
      name: "",
      initiative: 0,
      isActive: false,
      type: "monster",
      current_hp: 0,
      max_hp: 0,
      concentration: false,
      temporary_hp: 0,
      monster: monster,
    };

    encounter.entities.push(monsterEntity);
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
