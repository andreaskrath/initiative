import type { BaseReminder } from "./BaseReminder";

export interface InitiativeReminder extends BaseReminder {
  reminder_type: "initiative";
  initiative?: number;
}
