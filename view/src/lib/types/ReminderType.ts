export enum ReminderType {
  Initiative = "initiative",
  Round = "round",
}

export const DisplayReminderType: Readonly<Record<ReminderType, string>> = {
  [ReminderType.Initiative]: "Initiative",
  [ReminderType.Round]: "Round",
} as const;

export const ReminderTypes: ReminderType[] = Object.values(ReminderType);
