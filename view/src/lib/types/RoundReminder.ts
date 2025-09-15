import type { BaseReminder } from "./BaseReminder";
import type { Trigger } from "./Trigger";

export interface RoundReminder extends BaseReminder {
  reminder_type: "round";
  trigger?: Trigger.StartOfRound | Trigger.EndOfRound;
}
