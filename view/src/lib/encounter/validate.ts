import { PrepareForValidation } from "$lib/shared/utils/validate";
import {
  Attribute,
  Condition,
  ExhaustionLevel,
  SaveTrigger,
  type PlayerEntity,
} from "$types";
import * as z from "zod";

export const ValidatePlayerEntity = async (
  playerEntity: PlayerEntity,
): Promise<z.ZodError | null> => {
  const preparedPlayerEntity = PrepareForValidation(playerEntity);
  const result = await playerEntitySchema.safeParseAsync(preparedPlayerEntity);

  if (!result.success) {
    return result.error;
  }

  return null;
};

export const ValidateMonsterEntity = {};

export const ValidateReminderEntity = {};

const combatConditionSchema = z.object({
  condition: z.enum(Condition, "A condition type must be specified"),
  saving_throw_dc: z.number(
    "A condition must have a saving throw DC specified",
  ),
  saving_throw_attribute: z.enum(
    Attribute,
    "A condition must have a saving throw attribute specified",
  ),
  source: z.string().optional(),
  cause: z.string().optional(),
  save_trigger: z.enum(
    SaveTrigger,
    "A condition must have a save trigger specified",
  ),
});

const playerEntitySchema = z.object({
  id: z.uuid().optional(),
  type: z.literal("player", "The type of a player must be 'player'"),
  name: z.string("A player must have a name specified"),
  initiative: z.number("A player must have an initiative specified"),
  concentration: z.boolean(
    "A player must have a concentration specified, even if false",
  ),
  exhaustion_level: z.enum(
    ExhaustionLevel,
    "A player must have an exhaustion level specified",
  ),
  conditions: z.array(
    combatConditionSchema,
    "Conditions must be specified for a player, even if empty",
  ),
});
