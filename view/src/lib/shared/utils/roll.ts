import { RollStrategy } from "$lib/types";
import { DiceRoll } from "@dice-roller/rpg-dice-roller";

export const Roll = (notation: string, strategy: RollStrategy) => {
  const roll = new DiceRoll(notation);
  let output: number;
  let average: number;

  switch (strategy) {
    case RollStrategy.Average:
      output = roll.averageTotal;
      break;
    case RollStrategy.Minimum:
      output = roll.minTotal;
      break;
    case RollStrategy.Maximum:
      output = roll.maxTotal;
      break;
    case RollStrategy.Low:
      average = roll.averageTotal;
      output = average;

      while (output >= average) {
        const newRoll = new DiceRoll(notation);
        output = newRoll.total;
      }

      break;
    case RollStrategy.High:
      average = roll.averageTotal;
      output = average;

      while (output <= average) {
        const newRoll = new DiceRoll(notation);
        output = newRoll.total;
      }

      break;
    default:
      output = roll.total;
  }

  return output;
};
