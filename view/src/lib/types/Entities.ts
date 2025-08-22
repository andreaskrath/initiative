import { type Monster } from "./Monster";

export interface Entity {
  id?: string;
  name: string;
  initiative: number;
  isActive: boolean;
}

interface CombatEntity extends Entity {
  concentration: boolean;
}

export interface PlayerEntity extends CombatEntity {
  type: "player";
}

export interface MonsterEntity extends CombatEntity {
  type: "monster";
  current_hp: number;
  max_hp: number;
  temporary_hp: number;
  monster: Monster;
}

export interface ReminderEntity extends Entity {
  type: "reminder";
  description: string;
}

export type EncounterEntity = PlayerEntity | MonsterEntity | ReminderEntity;
