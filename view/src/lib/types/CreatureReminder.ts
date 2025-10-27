import type { BaseReminder } from "./BaseReminder";
import type { Trigger } from "./Trigger";

export interface CreatureReminder extends BaseReminder {
  trigger?: Trigger.StartOfTurn | Trigger.EndOfTurn;
}
