import type { CombatCondition } from "./CombatCondition";
import type { CreatureReminder } from "./CreatureReminder";
import { type Entity } from "./Entity";
import type { ExhaustionLevel } from "./ExhaustionLevel";

export interface CombatEntity extends Entity {
  initiative?: number;
  concentration: boolean;
  exhaustion_level: ExhaustionLevel;
  conditions: CombatCondition[];
  reminders: CreatureReminder[];
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

  RemoveCondition: (
    combatEntity: CombatEntity,
    combatConditionToRemove: CombatCondition,
  ): void => {
    combatEntity.conditions = combatEntity.conditions.filter(
      (combatCondition) => combatCondition !== combatConditionToRemove,
    );
  },
};
