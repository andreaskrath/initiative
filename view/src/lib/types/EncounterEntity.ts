import type { MonsterEntity } from "./MonsterEntity";
import type { PlayerEntity } from "./PlayerEntity";
import type { ReminderEntity } from "./ReminderEntity";

export type EncounterEntity = PlayerEntity | MonsterEntity | ReminderEntity;
