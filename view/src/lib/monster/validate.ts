import * as z from "zod";
import {
  Alignment,
  Attribute,
  Condition,
  DamageType,
  Language,
  MonsterType,
  Movement,
  Sight,
  Size,
  Skill,
  SpellLevel,
  SpellLevels,
  type Monster,
} from "$types";
import { spellSchema } from "$spell/validate";
import { CreateFieldErrors, type FieldErrors } from "$lib/shared/utils/error";

/*
 * Validate that a monster fits the expected data structure and values for the backend.
 */
export const Validate = async (
  monster: Monster,
): Promise<FieldErrors | null> => {
  const result = await schema.safeParseAsync(monster);

  if (!result.success) {
    return CreateFieldErrors(result.error);
  }

  return null;
};

const schema = z.object({
  id: z.uuid().optional(),
  name: z
    .string("A name must be specified")
    .min(1, "A name must be at least 1 character long"),
  challenge_rating: z.number("A challenge rating must be specified"),
  xp: z.number("An XP yield must be specified"),
  proficiency_bonus: z.number("A proficiency bonus must be specified"),
  size: z.enum(Size, "A size must be specified"),
  monster_type: z.enum(MonsterType, "A monster type must be specified"),
  species: z.string().optional(),
  alignment: z.enum(Alignment, "An alignment must be specified"),
  strength: z.number("A strength must be specified"),
  dexterity: z.number("A dexterity must be specified"),
  constitution: z.number("A constitution must be specified"),
  intelligence: z.number("An intelligence must be specified"),
  wisdom: z.number("A wisdom must be specified"),
  charisma: z.number("A charisma must be specified"),
  hit_points: z.number("Hit points must be specified"),
  rollable_hit_points: z
    .string("Rollable hit points must be specified")
    .regex(/^\d+d\d+([+-]\d+)?$/, "Invalid rollable hit points format"),
  armor_class: z.number("An armor class must be specified"),
  armor_type: z.string().optional(),
  saving_throws: z.array(
    z.object({
      attribute: z.enum(
        Attribute,
        "A saving throw must have an attribute specified",
      ),
      modifier: z.number("A saving throw must have a modifier specified"),
    }),
  ),
  damage_resistances: z.array(
    z.enum(DamageType, "Damage resistances must specify a damage type"),
  ),
  damage_immunities: z.array(
    z.enum(DamageType, "Damge immunities must specify a damage type"),
  ),
  condition_immunities: z.array(
    z.enum(Condition, "Condition immunities must specify a condition"),
  ),
  visions: z.array(
    z.object({
      sight: z.enum(Sight, "A sight type must be specified"),
      range: z.number("A range must be specified"),
    }),
  ),
  passive_perception: z.number("A passive perception must be specified"),
  speeds: z.array(
    z.object({
      movement: z.enum(Movement, "A movement type must be specified"),
      distance: z.number("A distance must be specified"),
    }),
  ),
  languages: z.array(z.enum(Language)),
  skills: z.array(
    z.object({
      skill: z.enum(Skill, "A skill must have a skill specified"),
      modifier: z.number("A skill must have a modifier specified"),
    }),
  ),
  traits: z.array(
    z.object({
      name: z.string("A name must be specified"),
      description: z.string("A description must be specified"),
    }),
  ),
  regular_actions: z.array(
    z.object({
      name: z.string("A name must be specified action"),
      description: z.string("A description must be specified action"),
    }),
  ),
  melee_attack_actions: z
    .array(
      z.object({
        name: z.string("A name must be specified attack action"),
        hit_bonus: z.number("A bonus to hit must be specified attack action"),
        reach: z.number("A reach must be specified attack action"),
        one_handed_attack: z
          .string()
          .regex(
            /^\d+d\d+([+-]\d+)?$/,
            "Invalid dice roll format attack action",
          )
          .optional(),
        two_handed_attack: z
          .string()
          .regex(
            /^\d+d\d+([+-]\d+)?$/,
            "Invalid dice roll format attack action",
          )
          .optional(),
        damage_type: z.enum(
          DamageType,
          "A damage type must be specified attack action",
        ),
      }),
    )
    .superRefine((data, ctx) => {
      for (const [i, entry] of data.entries()) {
        if (!entry.one_handed_attack && !entry.two_handed_attack) {
          ctx.addIssue({
            code: "custom",
            message: `Melee attack action ${i + 1} does not have either a one-handed or two-handed attack roll specified`,
            path: ["meleeAttackActions"],
          });
        }
      }
    }),
  ranged_attack_actions: z.array(
    z.object({
      name: z.string("A name must be specified attack action"),
      hit_bonus: z.number("A bonus to hit must be specified attack action"),
      normal_range: z.number("A normal range must be specified attack action"),
      long_range: z.number("A long range must be specified attack action"),
      attack: z
        .string("An attack roll must be specified attack action")
        .regex(/^\d+d\d+([+-]\d+)?$/, "Invalid dice roll format attack action"),
      damage_type: z.enum(
        DamageType,
        "A damage type must be specified attack action",
      ),
    }),
  ),
  recharge_actions: z.array(
    z.object({
      name: z.string("A name must be specified action"),
      recharge: z.string("A recharge must be specified for at recharge action"),
      description: z.string("A description must be specified action"),
    }),
  ),
  bonus_actions: z.array(
    z.object({
      name: z.string("A name must be specified action"),
      description: z.string("A description must be specified action"),
    }),
  ),
  reactions: z.array(
    z.object({
      name: z.string("A name must be specified"),
      description: z.string("A description must be specified"),
    }),
  ),
  available_legendary_actions_per_turn: z.number().optional(),
  legendary_actions: z.array(
    z.object({
      name: z.string("A name must be specified action"),
      cost: z.number("A cost must be specified action"),
      description: z.string("A description must be specified action"),
    }),
  ),
  lair_actions: z.array(
    z.object({
      name: z.string("A name must be specified action"),
      description: z.string("A description must be specified action"),
    }),
  ),
  spellcasting: z
    .object({
      level: z.number().optional(),
      attribute: z.enum(Attribute).optional(),
      dc: z.number().optional(),
      attack_bonus: z.number().optional(),
      spell_slots: z.array(
        z.object({
          level: z.enum(
            SpellLevel,
            "A spell slot must have a spell level specified",
          ),
          slots: z.number(
            "A spell slot must have the number of spell slots specified",
          ),
        }),
      ),
      spells: z.array(spellSchema),
    })
    .superRefine((spellcasting, ctx) => {
      const spellcastingProps = [
        spellcasting.level,
        spellcasting.attribute,
        spellcasting.dc,
        spellcasting.attack_bonus,
      ];
      const spellcastingPropsUndefined = spellcastingProps.filter(
        (prop) => prop === undefined,
      ).length;

      if (
        spellcastingPropsUndefined !== spellcastingProps.length &&
        spellcastingPropsUndefined !== 0
      ) {
        ctx.addIssue({
          code: "custom",
          message:
            "All spellcasting fields must be set together, or none at all.",
          path: ["level"],
        });
      }

      for (const spellLevel of SpellLevels) {
        // There are no spell slots for cantrips
        if (spellLevel === SpellLevel.Cantrip) {
          continue;
        }

        const currentLevelSpells = spellcasting.spells.filter(
          (spell) => spell.level === spellLevel,
        );
        const currentLevelSpellSlots = spellcasting.spell_slots.filter(
          (spell_slots) => spell_slots.level === spellLevel,
        );

        if (
          currentLevelSpells.length !== 0 &&
          currentLevelSpellSlots.length === 0
        ) {
          ctx.addIssue({
            code: "custom",
            message: `Spells of ${spellLevel} level are defined, but no spells slots are defined for this level of spells`,
            path: ["spellcastingSpellSlots"],
          });
        } else if (
          currentLevelSpells.length === 0 &&
          currentLevelSpellSlots.length !== 0
        ) {
          ctx.addIssue({
            code: "custom",
            message: `No spells of ${spellLevel} level are defined, but spells slots are defined for this level of spells`,
            path: ["spellcastingSpellSlots"],
          });
        }
      }
    }),
});
