import { Alignment } from "./Alignment";
import { Attribute, Attributes } from "./Attribute";
import { Condition, Conditions } from "./Condition";
import { DamageType, DamageTypes } from "./DamageType";
import { Language, Languages } from "./Language";
import { MonsterType } from "./MonsterType";
import { Recharge } from "./Recharge";
import { Sight } from "./Sight";
import { Size } from "./Size";
import { Skill, Skills } from "./Skill";

type NamedDescription = { name: string | null; description: string | null };

type Vision = { type: Sight; range: number | null };

type MeleeAttack = {
  name: string | null;
  hitBonus: number | null;
  reach: number | null;
  oneHandedAttack: string | null;
  twoHandedAttack: string | null;
  damageType: DamageType | null;
};

type RangedAttack = {
  name: string | null;
  hitBonus: number | null;
  normalRange: number | null;
  longRange: number | null;
  attack: string | null;
  damageType: DamageType | null;
};

type RechargeAction = {
  name: string | null;
  rechargeDice: Recharge | null;
  description: string | null;
};

type LegendaryAction = {
  name: string | null;
  cost: number | null;
  description: string | null;
};

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
  visions: Vision[];
  passivePerception: number | null;
  languages: Record<Language, boolean>;
  skills: Record<Skill, number | null>;
  traits: NamedDescription[];
  regularActions: NamedDescription[];
  meleeAttackActions: MeleeAttack[];
  rangedAttackActions: RangedAttack[];
  rechargeActions: RechargeAction[];
  bonusActions: NamedDescription[];
  reactions: NamedDescription[];
  availableLegendaryActionsPerTurn: number | null;
  legendaryActions: LegendaryAction[];
  lairActions: NamedDescription[];

  constructor() {
    this.name = $state(null);
    this.challengeRating = $state(null);
    this.xp = $state(null);
    this.proficiencyBonus = $state(null);
    this.size = $state(null);
    this.monsterType = $state(null);
    this.species = $state(null);
    this.alignment = $state(null);
    this.attributes = $state(Monster.RecordFactory(Attributes, null));
    this.hitPoints = $state(null);
    this.rollableHitPoints = $state(null);
    this.armorClass = $state(null);
    this.armorType = $state(null);
    this.savingThrows = $state(Monster.RecordFactory(Attributes, null));
    this.damageResistances = $state(Monster.RecordFactory(DamageTypes, false));
    this.damageImmunities = $state(Monster.RecordFactory(DamageTypes, false));
    this.conditionImmunities = $state(Monster.RecordFactory(Conditions, false));
    this.visions = $state([]);
    this.passivePerception = $state(null);
    this.languages = $state(Monster.RecordFactory(Languages, false));
    this.skills = $state(Monster.RecordFactory(Skills, null));
    this.traits = $state([]);
    this.regularActions = $state([]);
    this.meleeAttackActions = $state([]);
    this.rangedAttackActions = $state([]);
    this.rechargeActions = $state([]);
    this.bonusActions = $state([]);
    this.reactions = $state([]);
    this.availableLegendaryActionsPerTurn = $state(null);
    this.legendaryActions = $state([]);
    this.lairActions = $state([]);
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

  public AddRegularAction(event: MouseEvent) {
    this.regularActions = [
      ...this.regularActions,
      { name: null, description: null },
    ];

    event.preventDefault();
  }

  public RemoveRegularAction(regularActionToRemove: {
    name: string | null;
    description: string | null;
  }) {
    this.regularActions = this.regularActions.filter(
      (regularAction) => regularAction !== regularActionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddMeleeAttackAction(event: MouseEvent) {
    this.meleeAttackActions = [
      ...this.meleeAttackActions,
      {
        name: null,
        hitBonus: null,
        reach: null,
        oneHandedAttack: null,
        twoHandedAttack: null,
        damageType: null,
      },
    ];

    event.preventDefault();
  }

  public RemoveMeleeAttackAction(meleeAttackActionToRemove: {
    name: string | null;
    hitBonus: number | null;
    reach: number | null;
    oneHandedAttack: string | null;
    twoHandedAttack: string | null;
    damageType: DamageType | null;
  }) {
    this.meleeAttackActions = this.meleeAttackActions.filter(
      (meleeAttackAction) => meleeAttackAction !== meleeAttackActionToRemove,
    );
    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddRangedAttackAction(event: MouseEvent) {
    this.rangedAttackActions = [
      ...this.rangedAttackActions,
      {
        name: null,
        hitBonus: null,
        normalRange: null,
        longRange: null,
        attack: null,
        damageType: null,
      },
    ];

    event.preventDefault();
  }

  public RemoveRangedAttackAction(rangedAttackActionToRemove: {
    name: string | null;
    hitBonus: number | null;
    normalRange: number | null;
    longRange: number | null;
    attack: string | null;
    damageType: DamageType | null;
  }) {
    this.rangedAttackActions = this.rangedAttackActions.filter(
      (rangedAttackAction) => rangedAttackAction !== rangedAttackActionToRemove,
    );
    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddRechargeAction(event: MouseEvent) {
    this.rechargeActions = [
      ...this.rechargeActions,
      {
        name: null,
        rechargeDice: null,
        description: null,
      },
    ];

    event.preventDefault();
  }

  public RemoveRechargeAction(rechargeActionToRemove: {
    name: string | null;
    rechargeDice: Recharge | null;
    description: string | null;
  }) {
    this.rechargeActions = this.rechargeActions.filter(
      (rechargeAction) => rechargeAction !== rechargeActionToRemove,
    );
    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddBonusAction(event: MouseEvent) {
    this.bonusActions = [
      ...this.bonusActions,
      { name: null, description: null },
    ];

    event.preventDefault();
  }

  public RemoveBonusAction(bonusActionToRemove: {
    name: string | null;
    description: string | null;
  }) {
    this.bonusActions = this.bonusActions.filter(
      (bonusAction) => bonusAction !== bonusActionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddReaction(event: MouseEvent) {
    this.reactions = [...this.reactions, { name: null, description: null }];

    event.preventDefault();
  }

  public RemoveReaction(reactionToRemove: {
    name: string | null;
    description: string | null;
  }) {
    this.reactions = this.reactions.filter(
      (reaction) => reaction !== reactionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddLegendaryAction(event: MouseEvent) {
    this.legendaryActions = [
      ...this.legendaryActions,
      { name: null, cost: null, description: null },
    ];

    event.preventDefault();
  }

  public RemoveLegendaryAction(legendaryActionToRemove: {
    name: string | null;
    cost: number | null;
    description: string | null;
  }) {
    this.legendaryActions = this.legendaryActions.filter(
      (legendaryAction) => legendaryAction !== legendaryActionToRemove,
    );

    // Clear available legendary actions if all legendary actions are removed
    if (this.legendaryActions.length === 0) {
      this.availableLegendaryActionsPerTurn = null;
    }

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddLairAction(event: MouseEvent) {
    this.lairActions = [...this.lairActions, { name: null, description: null }];

    event.preventDefault();
  }

  public RemoveLairAction(lairActionToRemove: {
    name: string | null;
    description: string | null;
  }) {
    this.lairActions = this.lairActions.filter(
      (lairAction) => lairAction !== lairActionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  /**
   * Generic factory helper for creating Records.
   */
  private static RecordFactory<T extends string | number | symbol, V>(
    items: readonly T[],
    defaultValue: V,
  ): Record<T, V> {
    return items.reduce(
      (record, item) => {
        record[item] = defaultValue;
        return record;
      },
      {} as Record<T, V>,
    );
  }
}
