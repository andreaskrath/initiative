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
  type Monster,
} from "$types";
import { PrepareForValidation } from "$utils/validate";

export const Validate = async (monster: Monster): Promise<string[]> => {
  monster = PrepareForValidation(monster);
  const result = await schema.safeParseAsync(monster);

  if (!result.success) {
    return result.error.issues.map((issue) => issue.message);
  }

  return [];
};

const schema = z.object({
  id: z.uuid().optional(),
  name: z
    .string("A name must be specified")
    .min(1, "A name must be at least 1 character long"),
  challengeRating: z.number("A challenge rating must be specified"),
  xp: z.number("An XP yield must be specified"),
  proficiencyBonus: z.number("A proficiency bonus must be specified"),
  size: z.enum(Size, "A size must be specified"),
  monsterType: z.enum(MonsterType, "A monster type must be specified"),
  species: z.string().optional(),
  alignment: z.enum(Alignment, "An alignment must be specified"),
  strength: z.number("A strength attribute must be specified"),
  dexterity: z.number("A dexterity attribute must be specified"),
  constitution: z.number("A constitution attribute must be specified"),
  intelligence: z.number("An intelligence attribute must be specified"),
  wisdom: z.number("A wisdom attribute must be specified"),
  charisma: z.number("A charisma attribute must be specified"),
  hitPoints: z.number("Hit points must be specified"),
  rollableHitPoints: z
    .string("Rollable hit points must be specified")
    .regex(/^\d+d\d+([+-]\d+)?$/, "Invalid rollable hit points format"),
  armorClass: z.number("An armor class must be specified"),
  armorType: z.string().optional(),
  savingThrows: z.array(
    z.object({
      attribute: z.enum(
        Attribute,
        "A saving throw must have an attribute specified",
      ),
      modifier: z.number("A saving throw must have a modifier specified"),
    }),
  ),
  damageResistances: z.array(
    z.enum(DamageType, "Damage resistances must specify a damage type"),
  ),
  damageImmunities: z.array(
    z.enum(DamageType, "Damge immunities must specify a damage type"),
  ),
  conditionImmunities: z.array(
    z.enum(Condition, "Condition immunities must specify a condition"),
  ),
  visions: z.array(
    z.object({
      type: z.enum(Sight, "A sight type must be specified for a vision"),
      range: z.number("A range must be specified for a vision"),
    }),
  ),
  passivePerception: z.number("A passive perception must be specified"),
  speeds: z.array(
    z.object({
      type: z.enum(Movement, "A movement type must be specified for a speed"),
      distance: z.number("A distance must be specified for a speed"),
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
      name: z.string("A name must be specified for a trait"),
      description: z.string("A description must be specified for a trait"),
    }),
  ),
  regularActions: z.array(
    z.object({
      name: z.string("A name must be specified for a regular action"),
      description: z.string(
        "A description must be specified for a regular action",
      ),
    }),
  ),
  meleeAttackActions: z
    .array(
      z.object({
        name: z.string("A name must be specified for a melee attack action"),
        hitBonus: z.number(
          "A bonus to hit must be specified for a melee attack action",
        ),
        reach: z.number("A reach must be specified for a melee attack action"),
        oneHandedAttack: z
          .string()
          .regex(
            /^\d+d\d+$/,
            "Invalid dice roll format for a melee attack action",
          )
          .optional(),
        twoHandedAttack: z
          .string()
          .regex(
            /^\d+d\d+$/,
            "Invalid dice roll format for a melee attack action",
          )
          .optional(),
        damageType: z.enum(
          DamageType,
          "A damage type must be specified for a melee attack action",
        ),
      }),
    )
    .superRefine((data, ctx) => {
      for (const [i, entry] of data.entries()) {
        if (!entry.oneHandedAttack && !entry.twoHandedAttack) {
          ctx.addIssue({
            code: "custom",
            message: `Melee attack action ${i + 1} does not have either a one-handed or two-handed attack roll specified`,
            path: ["meleeAttackActions"],
          });
        }
      }
    }),
  rangedAttackActions: z.array(
    z.object({
      name: z.string("A name must be specified for a ranged attack action"),
      hitBonus: z.number(
        "A bonus to hit must be specified for a ranged attack action",
      ),
      normalRange: z.number(
        "A normal range must be specified for a ranged attack action",
      ),
      longRange: z.number(
        "A long range must be specified for a ranged attack action",
      ),
      attack: z
        .string("An attack roll must be specified for a ranged attack action")
        .regex(
          /^\d+d\d+$/,
          "Invalid dice roll format for a ranged attack action",
        ),
      damageType: z.enum(
        DamageType,
        "A damage type must be specified for a melee attack action",
      ),
    }),
  ),
  rechargeActions: z.array(
    z.object({
      name: z.string("A name must be specified for a recharge action"),
      rechargeDice: z.string(
        "A recharge must be specified for at recharge action",
      ),
      description: z.string(
        "A description must be specified for a recharge action",
      ),
    }),
  ),
  bonusActions: z.array(
    z.object({
      name: z.string("A name must be specified for a bonus action"),
      description: z.string(
        "A description must be specified for a bonus action",
      ),
    }),
  ),
  reactions: z.array(
    z.object({
      name: z.string("A name must be specified for a reaction"),
      description: z.string("A description must be specified for a reaction"),
    }),
  ),
  availableLegendaryActionsPerTurn: z.number().optional(),
  legendaryActions: z.array(
    z.object({
      name: z.string("A name must be specified for a legendary action"),
      cost: z.number("A cost must be specified for a legendary action"),
      description: z.string(
        "A description must be specified for a legendary action",
      ),
    }),
  ),
  lairActions: z.array(
    z.object({
      name: z.string("A name must be specified for a lair action"),
      description: z.string(
        "A description must be specified for a lair action",
      ),
    }),
  ),
  spellcasting: z
    .object({
      level: z.number().optional(),
      attribute: z.enum(Attribute).optional(),
      dc: z.number().optional(),
      attackBonus: z.number().optional(),
      spellSlots: z.array(
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
      spells: z.array(z.string()),
    })
    .superRefine((spellcasting, ctx) => {
      const spellcastingProps = [
        spellcasting.level,
        spellcasting.attribute,
        spellcasting.dc,
        spellcasting.attackBonus,
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
          path: ["spellcastingLevel"],
        });
      }
    }),
});
