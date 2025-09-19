import type { CombatEntity } from "./CombatEntity";
import { ExhaustionLevel } from "./ExhaustionLevel";
import { v4 as uuid } from "uuid";

export interface PlayerEntity extends CombatEntity {
  type: "player";
}

export const PlayerEntityActions = {
  EmptyPlayerEntity: (): PlayerEntity => ({
    id: uuid(),
    type: "player",
    name: undefined,
    initiative: undefined,
    exhaustion_level: ExhaustionLevel.None,
    concentration: false,
    conditions: [],
  }),
};
