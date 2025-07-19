export enum Movement {
  Normal = "Normal",
  Burrow = "Burrow",
  Climb = "Climb",
  Fly = "Fly",
  FlyHover = "Fly (hover)",
  Swim = "Swim",
}

export const Movements: Movement[] = Object.values(Movement);
