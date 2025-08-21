import { type Monster } from "./Monster";

export interface Entity {
  id?: string;
  name: string;
  initiative: number;
  isActive: boolean;
}

interface Player {}

export interface PlayerEntity extends Entity {
  type: "player";
  player: Player;
}

export interface MonsterEntity extends Entity {
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

export type CombatEntity = PlayerEntity | MonsterEntity | ReminderEntity;
