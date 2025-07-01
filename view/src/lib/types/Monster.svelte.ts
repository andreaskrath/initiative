import { Alignment } from "./Alignment";
import { Attribute, Attributes } from "./Attribute";
import { Condition, Conditions } from "./Condition";
import { DamageType, DamageTypes } from "./DamageType";
import { Language, Languages } from "./Language";
import { MonsterType } from "./MonsterType";
import { Sight } from "./Sight";
import { Size } from "./Size";
import { Skill, Skills } from "./Skill";

export class Monster {
  name: string | null;
  challengeRating: number | null;
  xp: number | null;
  proficiencyBonus: number | null;
  size: Size | null;
  monsterType: MonsterType | null;
  species: string | null;
  alignment: Alignment | null;
  attributes: Record<Attribute, number | null>;
  hitPoints: number | null;
  rollableHitPoints: string | null;
  armorClass: number | null;
  armorType: string | null;
  savingThrows: Record<Attribute, number | null>;
  damageResistances: Record<DamageType, boolean>;
  damageImmunities: Record<DamageType, boolean>;
  conditionImmunities: Record<Condition, boolean>;
  visions: { type: Sight; range: number | null }[];
  languages: Record<Language, boolean>;
  skills: Record<Skill, number | null>;
  traits: { name: string | null; description: string | null }[];

  constructor() {
    this.name = $state(null);
    this.challengeRating = $state(null);
    this.xp = $state(null);
    this.proficiencyBonus = $state(null);
    this.size = $state(null);
    this.monsterType = $state(null);
    this.species = $state(null);
    this.alignment = $state(null);
    this.attributes = $state(Monster.AttributesFactory());
    this.hitPoints = $state(null);
    this.rollableHitPoints = $state(null);
    this.armorClass = $state(null);
    this.armorType = $state(null);
    this.savingThrows = $state(Monster.AttributesFactory());
    this.damageResistances = $state(Monster.DamageTypesFactory());
    this.damageImmunities = $state(Monster.DamageTypesFactory());
    this.conditionImmunities = $state(Monster.ConditionsFactory());
    this.visions = $state([]);
    this.languages = $state(Monster.LanguagesFactory());
    this.skills = $state(Monster.SkillsFactory());
    this.traits = $state([]);
  }

  public AddVision(event: MouseEvent) {
    this.visions = [...this.visions, { type: Sight.Darkvision, range: null }];

    event.preventDefault();
  }

  public RemoveVision(visionToRemove: { type: Sight; range: number | null }) {
    this.visions = this.visions.filter((vision) => vision !== visionToRemove);

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddTrait(event: MouseEvent) {
    this.traits = [...this.traits, { name: null, description: null }];

    event.preventDefault();
  }

  public RemoveTrait(traitToRemove: {
    name: string | null;
    description: string | null;
  }) {
    this.traits = this.traits.filter((trait) => trait !== traitToRemove);
    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  private static AttributesFactory() {
    return Attributes.reduce(
      (record, attribute) => {
        record[attribute] = null;
        return record;
      },
      {} as Record<Attribute, number | null>,
    );
  }

  private static DamageTypesFactory() {
    return DamageTypes.reduce(
      (record, damageType) => {
        record[damageType] = false;
        return record;
      },
      {} as Record<DamageType, boolean>,
    );
  }

  private static ConditionsFactory() {
    return Conditions.reduce(
      (record, condition) => {
        record[condition] = false;
        return record;
      },
      {} as Record<Condition, boolean>,
    );
  }

  private static LanguagesFactory() {
    return Languages.reduce(
      (record, language) => {
        record[language] = false;
        return record;
      },
      {} as Record<Language, boolean>,
    );
  }

  private static SkillsFactory() {
    return Skills.reduce(
      (record, skill) => {
        record[skill] = null;
        return record;
      },
      {} as Record<Skill, number | null>,
    );
  }
}
