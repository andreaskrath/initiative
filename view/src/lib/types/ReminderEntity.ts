import type { InitiativeReminder } from "./InitiativeReminder";
import type { RoundReminder } from "./RoundReminder";
import { DisplayTrigger } from "./Trigger";
import { v4 as uuid } from "uuid";

export type ReminderEntity = InitiativeReminder | RoundReminder;

export const ReminderActions = {
  EmptyInitiativeReminder: (): InitiativeReminder => ({
    id: uuid(),
    name: undefined,
    type: "reminder",
    reminder_type: "initiative",
    initiative: undefined,
    description: undefined,
  }),

  EmptyRoundReminder: (): RoundReminder => ({
    id: uuid(),
    name: undefined,
    type: "reminder",
    reminder_type: "round",
    trigger: undefined,
    description: undefined,
  }),

  GetTrigger: (reminder: ReminderEntity): string => {
    let reminderTrigger = "";
    switch (reminder.reminder_type) {
      case "initiative":
        reminderTrigger = "Initiative";
        break;
      case "round":
        reminderTrigger = DisplayTrigger[reminder.trigger!];
        break;
      default:
        reminderTrigger = "Unknown";
    }

    return reminderTrigger;
  },

  GetDetails: (reminder: ReminderEntity): string => {
    let details = "";

    switch (reminder.reminder_type) {
      case "initiative":
        details = `${reminder.initiative!}`;
        break;
      case "round":
        break;
      default:
        details = "Unknown";
    }

    return details;
  },
};
