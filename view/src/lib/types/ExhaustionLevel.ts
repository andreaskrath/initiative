export enum ExhaustionLevel {
  None = "none",
  One = "one",
  Two = "two",
  Three = "three",
  Four = "four",
  Five = "five",
  Six = "six",
}

export const ExhaustionLevels: ExhaustionLevel[] =
  Object.values(ExhaustionLevel);

export const DisplayExhaustionLevel: Readonly<Record<ExhaustionLevel, string>> =
  {
    [ExhaustionLevel.None]: "None",
    [ExhaustionLevel.One]: "1 - Disadvantage on ability checks",
    [ExhaustionLevel.Two]: "2 - Speed halved",
    [ExhaustionLevel.Three]:
      "3 - Disadvantage on attack rolls and saving throws",
    [ExhaustionLevel.Four]: "4 - Hit point maximum halved",
    [ExhaustionLevel.Five]: "5 - Speed reduced to 0",
    [ExhaustionLevel.Six]: "6 - Death",
  } as const;
