import { CreateFieldErrors, type FieldErrors } from "$utils/error";
import { Class, MagicSchool, SpellLevel, type Spell } from "$types";
import * as z from "zod";

export const ValidateSpell = async (
  spell: Spell,
): Promise<FieldErrors | null> => {
  const result = await SpellSchema.safeParseAsync(spell);

  if (!result.success) {
    return CreateFieldErrors(result.error);
  }

  return null;
};

export const SpellSchema = z
  .object({
    id: z.uuid().optional(),
    name: z
      .string("A name must be specified")
      .min(1, "A name must be at least a single character long"),
    school: z.enum(MagicSchool, "A school of magic must be specified"),
    level: z.enum(SpellLevel, "A spell level must be specified"),
    casting_time: z
      .string("A casting time must be specified")
      .min(1, "A casting time must be at least a single character long"),
    verbal: z.boolean(),
    somatic: z.boolean(),
    material: z.string().optional(),
    material_consumed: z.boolean(),
    ritual: z.boolean(),
    concentration: z.boolean(),
    duration: z
      .string("A duration must be specified")
      .min(1, "A duration must be at least a single character long"),
    range: z
      .string("A range must be specified")
      .min(1, "A range must be at least a single character long"),
    area: z
      .string("A area must be specified")
      .min(1, "An area must be at least a single character long"),
    shape: z.string().optional(),
    classes: z
      .array(z.enum(Class))
      .min(1, "At least a single class must be specified"),
    description: z
      .string("A description must be specified")
      .min(1, "A description must be at least a single character long"),
    at_higher_levels: z.string().optional(),
  })
  .refine(
    (data) => {
      // If materialConsumed is true, material must have a value
      if (data.material_consumed === true) {
        return data.material !== undefined && data.material.length > 0;
      }
      return true;
    },
    {
      message: "Material components must be specified when they are consumed",
      path: ["material"],
    },
  );
