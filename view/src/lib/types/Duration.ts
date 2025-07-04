export enum Duration {
  Instantaneous = "Instantaneous",
  OneRound = "1 round",
  OneMinute = "1 minute",
  TenMinutes = "10 minutes",
  OneHour = "1 hour",
  EightHours = "8 hours",
  OneDay = "24 hours",
  SevenDays = "7 days",
  Permanent = "Permanent",
  UntilDispelled = "Until dispelled",
}

export const Durations: Duration[] = Object.values(Duration);
