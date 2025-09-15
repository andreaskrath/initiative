import type { CombatEntity } from "./CombatEntity";
import { ExhaustionLevel } from "./ExhaustionLevel";
import type { Monster } from "./Monster";

export interface MonsterEntity extends CombatEntity {
  type: "monster";
  current_hp?: number;
  max_hp?: number;
  temporary_hp?: number;
  monster?: Monster;
}

export const MonsterEntityActions = {
  EmptyMonsterEntity: (): MonsterEntity => ({
    name: undefined,
    initiative: undefined,
    is_active: undefined,
    concentration: false,
    exhaustion_level: ExhaustionLevel.None,
    conditions: [],
    type: "monster",
    current_hp: undefined,
    max_hp: undefined,
    temporary_hp: undefined,
    monster: undefined,
  }),
};
