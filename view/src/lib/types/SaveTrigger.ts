export enum SaveTrigger {
  Nothing = "nothing",
  StartOfTurn = "start_of_turn",
  EndOfTurn = "end_of_turn",
  StartOfRound = "start_of_round",
  EndOfRound = "end_of_round",
  Damage = "damage",
}

export const SaveTriggers: SaveTrigger[] = Object.values(SaveTrigger);

export const DisplaySaveTrigger: Readonly<Record<SaveTrigger, string>> = {
  [SaveTrigger.Nothing]: "Nothing",
  [SaveTrigger.StartOfTurn]: "Start of the affected creatures turn",
  [SaveTrigger.EndOfTurn]: "End of the affected creatures turn",
  [SaveTrigger.StartOfRound]: "Start of a new round",
  [SaveTrigger.EndOfRound]: "End of the current round",
  [SaveTrigger.Damage]: "When the affected creature takes damage",
} as const;
