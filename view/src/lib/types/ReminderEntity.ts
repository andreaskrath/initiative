import type { Entity } from "./Entity";

export interface ReminderEntity extends Entity {
  type: "reminder";
  description: string;
  win_initiative_tie: boolean;
}
