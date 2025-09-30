export enum RollStrategy {
  /** A fully random roll, with no intervention otherwise. */
  Random = "random",

  /** No roll is performed, the average roll is always the outcome. */
  Average = "average",

  /** No roll is performed, the minimum possible value is the outcome. */
  Minimum = "minimum",

  /** No roll is performed, the maximum possible value is the outcome. */
  Maximum = "maximum",

  /** The roll will be re-rolled until it is below the average outcome. */
  Low = "low",

  /** The roll will be re-rolled until it is above the average outcome. */
  High = "high",
}

export const RollStrategies: RollStrategy[] = Object.values(RollStrategy);

export const DisplayRollStrategy: Readonly<Record<RollStrategy, string>> = {
  [RollStrategy.Random]: "Random",
  [RollStrategy.Average]: "Average",
  [RollStrategy.Minimum]: "Minimum",
  [RollStrategy.Maximum]: "Maximum",
  [RollStrategy.Low]: "Low",
  [RollStrategy.High]: "High",
} as const;
