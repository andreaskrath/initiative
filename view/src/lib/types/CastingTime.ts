export enum CastingTime {
  Action = "1 action",
  BonusAction = "1 bonus action",
  Reaction = "1 reaction",
  OneMinute = "1 minute",
  OneHour = "1 hour",
  EightHours = "8 hours",
  Day = "24 hours",
}

export const CastingTimes: CastingTime[] = Object.values(CastingTime);
