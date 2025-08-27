import type { CombatEntity } from "./CombatEntity";

export interface PlayerEntity extends CombatEntity {
  type: "player";
}
