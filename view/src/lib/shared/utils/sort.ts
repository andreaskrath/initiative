import { Trigger, type EncounterEntity } from "$types";

/*
 * Compare function for sorting encounter entities.
 * */
export const CompareEncounterEntities = (
  a: EncounterEntity,
  b: EncounterEntity,
): number => {
  // Both are RoundReminders
  if (
    a.type === "reminder" &&
    a.reminder_type === "round" &&
    b.type === "reminder" &&
    b.reminder_type === "round"
  ) {
    const aIsStartOfRound = a.trigger === Trigger.StartOfRound;
    const bIsStartOfRound = b.trigger === Trigger.StartOfRound;

    // Both start, meaning they are equal
    if (aIsStartOfRound && bIsStartOfRound) {
      return 0;
    }
    // 'a' is first
    else if (aIsStartOfRound) {
      return -1;
    }
    // 'b' is first
    else if (bIsStartOfRound) {
      return 1;
    }
    // Both end, meaning they are equal
    else {
      return 0;
    }
  }
  // Only 'a' is a RoundReminder
  else if (a.type === "reminder" && a.reminder_type === "round") {
    // If 'a' is start, then it goes first
    return a.trigger === Trigger.StartOfRound ? -1 : 1;
  }
  // Only 'b' is a RoundReminder
  else if (b.type === "reminder" && b.reminder_type === "round") {
    // If 'b' is start, then it goes first
    return b.trigger === Trigger.StartOfRound ? 1 : -1;
  }
  // Neither is a RoundReminder
  else {
    const aInit = a.initiative ?? 0;
    const bInit = b.initiative ?? 0;

    return bInit - aInit;
  }
};
