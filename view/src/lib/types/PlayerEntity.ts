import type { CombatEntity } from "./CombatEntity";
import { ExhaustionLevel } from "./ExhaustionLevel";

export interface PlayerEntity extends CombatEntity {
  type: "player";
}

export const PlayerEntityActions = {
  EmptyPlayerEntity: (): PlayerEntity => ({
    type: "player",
    name: undefined,
    initiative: undefined,
    exhaustion_level: ExhaustionLevel.None,
    concentration: false,
    conditions: [],
  }),
};
