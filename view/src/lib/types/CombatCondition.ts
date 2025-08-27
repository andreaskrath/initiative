import type { Attribute } from "./Attribute";
import type { Condition } from "./Condition";

export type CombatCondition = {
  /** The condition affecting the creature. */
  condition?: Condition;
  /** The saving throw DC of the condition. */
  saving_throw_dc?: number;
  /** The saving throw attribute of the condition. */
  saving_throw_attribute?: Attribute;
  /** The source of the condition, for example a player, monster, or the environment. */
  source?: string;
  /** The cause of the condition, for example a spell, trait, or magical item. */
  cause?: string;
  /** When to roll a new saving throw and attempt to remove the condition. */
  save_trigger?: string;
};
