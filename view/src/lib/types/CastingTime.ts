export enum CastingTime {
  Action = "1 Action",
  BonusAction = "1 Bonus Action",
  Reaction = "1 Reaction",
  OneMinute = "1 Minute",
  OneHour = "1 Hour",
  EightHours = "8 Hours",
  Day = "24 Hours",
}

export const CastingTimes: CastingTime[] = Object.values(CastingTime);
