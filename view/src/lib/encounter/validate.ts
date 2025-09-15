import { PrepareForValidation } from "$utils/validate";
import {
  Attribute,
  Condition,
  ExhaustionLevel,
  Trigger,
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
  saving_throw_dc: z.number("A saving throw DC must be specified"),
  saving_throw_attribute: z.enum(
    Attribute,
    "A saving throw attribute must be specified",
  ),
  source: z.string().optional(),
  cause: z.string().optional(),
  save_trigger: z.enum(Trigger, "A save trigger must be specified"),
});

const playerEntitySchema = z.object({
  id: z.uuid().optional(),
  type: z.literal("player", "The type of a player must be 'player'"),
  name: z.string("A name must be specified"),
  initiative: z.number("An initiative must be specified"),
  concentration: z.boolean(
    "A player must have a concentration specified, even if false",
  ),
  exhaustion_level: z.enum(
    ExhaustionLevel,
    "An exhaustion level must be specified",
  ),
  conditions: z.array(
    combatConditionSchema,
    "Conditions must be specified for a player, even if empty",
  ),
});
