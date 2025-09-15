import type { InitiativeReminder } from "./InitiativeReminder";
import type { RoundReminder } from "./RoundReminder";
import { DisplayTrigger } from "./Trigger";
import type { TurnReminder } from "./TurnReminder";

export type ReminderEntity = InitiativeReminder | TurnReminder | RoundReminder;

export const ReminderActions = {
  EmptyInitiativeReminder: (): InitiativeReminder => ({
    name: undefined,
    type: "reminder",
    reminder_type: "initiative",
    initiative: undefined,
    description: undefined,
  }),

  EmptyTurnReminder: (): TurnReminder => ({
    name: undefined,
    type: "reminder",
    reminder_type: "turn",
    trigger: undefined,
    description: undefined,
    targets: [],
  }),

  EmptyRoundReminder: (): RoundReminder => ({
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
      case "turn":
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
      case "turn":
        details = "Targets";
        break;
      case "round":
        break;
      default:
        details = "Unknown";
    }

    return details;
  },

  RemoveTarget: (turnReminder: TurnReminder, targetToRemove: string) => {
    turnReminder.targets = turnReminder.targets.filter(
      (target) => target !== targetToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },
};
