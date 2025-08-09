export enum Recharge {
  FourToSix = "4-6",
  FiveToSix = "5-6",
  Six = "6",
  ShortRest = "Short rest",
  LongRest = "Long rest",
  AnyRest = "Any rest",
  Hour = "1 hour",
  Day = "1 day",
}

export const Recharges: Recharge[] = Object.values(Recharge);
