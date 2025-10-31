export enum Trigger {
  Nothing = "nothing",
  StartOfTurn = "start_of_turn",
  EndOfTurn = "end_of_turn",
  StartOfRound = "start_of_round",
  EndOfRound = "end_of_round",
  Damage = "damage",
}

export const SaveTriggers: Trigger[] = Object.values(Trigger);

export const TurnTriggers: Trigger[] = [Trigger.StartOfTurn, Trigger.EndOfTurn];

export const RoundTriggers: Trigger[] = [
  Trigger.StartOfRound,
  Trigger.EndOfRound,
];

export const DisplayTrigger: Readonly<Record<Trigger, string>> = {
  [Trigger.Nothing]: "Nothing",
  [Trigger.StartOfTurn]: "Start of turn",
  [Trigger.EndOfTurn]: "End of turn",
  [Trigger.StartOfRound]: "Start of round",
  [Trigger.EndOfRound]: "End of round",
  [Trigger.Damage]: "On damage taken",
} as const;
