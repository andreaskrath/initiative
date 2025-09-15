export enum ReminderType {
  Initiative = "initiative",
  Round = "round",
  Turn = "turn",
}

export const DisplayReminderType: Readonly<Record<ReminderType, string>> = {
  [ReminderType.Initiative]: "Initiative",
  [ReminderType.Round]: "Round",
  [ReminderType.Turn]: "Turn",
} as const;

export const ReminderTypes: ReminderType[] = Object.values(ReminderType);
