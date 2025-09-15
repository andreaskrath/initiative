import type { BaseReminder } from "./BaseReminder";
import type { Trigger } from "./Trigger";

export interface TurnReminder extends BaseReminder {
  reminder_type: "turn";
  trigger?: Trigger.StartOfTurn | Trigger.EndOfTurn;
  targets: string[];
}
