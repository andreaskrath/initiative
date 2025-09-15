import type { Entity } from "./Entity";

export interface BaseReminder extends Entity {
  type: "reminder";
  description?: string;
}
