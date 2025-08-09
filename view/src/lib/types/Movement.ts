export enum Movement {
  Normal = "normal",
  Burrow = "burrow",
  Climb = "climb",
  Fly = "fly",
  FlyHover = "fly_hover",
  Swim = "swim",
}

export const DisplayMovement: Readonly<Record<Movement, string>> = {
  [Movement.Normal]: "Normal",
  [Movement.Burrow]: "Burrow",
  [Movement.Climb]: "Climb",
  [Movement.Fly]: "Fly",
  [Movement.FlyHover]: "Fly (hover)",
  [Movement.Swim]: "Swim",
} as const;

export const Movements: Movement[] = Object.values(Movement);
