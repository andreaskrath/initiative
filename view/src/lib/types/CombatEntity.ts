import type { CombatCondition } from "./CombatCondition";
import { type Entity } from "./Entity";

export interface CombatEntity extends Entity {
  concentration: boolean;
  exhaustion_level?: number;
  conditions: CombatCondition[];
}

export const CombatEntityActions = {
  AddCondition: (combatEntity: CombatEntity): void => {
    combatEntity.conditions.push({
      condition: undefined,
      saving_throw_dc: undefined,
      saving_throw_attribute: undefined,
      source: undefined,
      cause: undefined,
      save_trigger: undefined,
    });
  },
};
