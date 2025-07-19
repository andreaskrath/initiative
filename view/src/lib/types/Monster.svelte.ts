import { Alignment } from "$types/Alignment";
import { Attribute, Attributes } from "$types/Attribute";
import { Condition, Conditions } from "$types/Condition";
import { DamageType, DamageTypes } from "$types/DamageType";
import { Language, Languages } from "$types/Language";
import { MonsterType } from "$types/MonsterType";
import { Recharge } from "$types/Recharge";
import { Sight } from "$types/Sight";
import { Size } from "$types/Size";
import { Skill, Skills } from "$types/Skill";
import { RecordFactory } from "$utils/factories";
import { Movement } from "$types/Movement";

type NamedDescription = { name?: string; description?: string };

type Vision = { type?: Sight; range?: number };

type Speed = { type?: Movement; distance?: number };

type MeleeAttack = {
  name?: string;
  hitBonus?: number;
  reach?: number;
  oneHandedAttack?: string;
  twoHandedAttack?: string;
  damageType?: DamageType;
};

type RangedAttack = {
  name?: string;
  hitBonus?: number;
  normalRange?: number;
  longRange?: number;
  attack?: string;
  damageType?: DamageType;
};

type RechargeAction = {
  name?: string;
  rechargeDice?: Recharge;
  description?: string;
};

type LegendaryAction = {
  name?: string;
  cost?: number;
  description?: string;
};

export class Monster {
  name?: string;
  challengeRating?: number;
  xp?: number;
  proficiencyBonus?: number;
  size?: Size;
  monsterType?: MonsterType;
  species?: string;
  alignment?: Alignment;
  attributes: Record<Attribute, number | undefined>;
  hitPoints?: number;
  rollableHitPoints?: string;
  armorClass?: number;
  armorType?: string;
  savingThrows: Record<Attribute, number | undefined>;
  damageResistances: Record<DamageType, boolean>;
  damageImmunities: Record<DamageType, boolean>;
  conditionImmunities: Record<Condition, boolean>;
  visions: Vision[];
  passivePerception?: number;
  speeds: Speed[];
  languages: Record<Language, boolean>;
  skills: Record<Skill, number | undefined>;
  traits: NamedDescription[];
  regularActions: NamedDescription[];
  meleeAttackActions: MeleeAttack[];
  rangedAttackActions: RangedAttack[];
  rechargeActions: RechargeAction[];
  bonusActions: NamedDescription[];
  reactions: NamedDescription[];
  availableLegendaryActionsPerTurn?: number;
  legendaryActions: LegendaryAction[];
  lairActions: NamedDescription[];
  spellcastingLevel?: number;
  spellcastingAttribute?: Attribute;
  spellcastingDC?: number;
  spellcastingAttackBonus?: number;

  constructor() {
    this.name = $state(undefined);
    this.challengeRating = $state(undefined);
    this.xp = $state(undefined);
    this.proficiencyBonus = $state(undefined);
    this.size = $state(undefined);
    this.monsterType = $state(undefined);
    this.species = $state(undefined);
    this.alignment = $state(undefined);
    this.attributes = $state(RecordFactory(Attributes, undefined));
    this.hitPoints = $state(undefined);
    this.rollableHitPoints = $state(undefined);
    this.armorClass = $state(undefined);
    this.armorType = $state(undefined);
    this.savingThrows = $state(RecordFactory(Attributes, undefined));
    this.damageResistances = $state(RecordFactory(DamageTypes, false));
    this.damageImmunities = $state(RecordFactory(DamageTypes, false));
    this.conditionImmunities = $state(RecordFactory(Conditions, false));
    this.visions = $state([]);
    this.passivePerception = $state(undefined);
    this.speeds = $state([]);
    this.languages = $state(RecordFactory(Languages, false));
    this.skills = $state(RecordFactory(Skills, undefined));
    this.traits = $state([]);
    this.regularActions = $state([]);
    this.meleeAttackActions = $state([]);
    this.rangedAttackActions = $state([]);
    this.rechargeActions = $state([]);
    this.bonusActions = $state([]);
    this.reactions = $state([]);
    this.availableLegendaryActionsPerTurn = $state(undefined);
    this.legendaryActions = $state([]);
    this.lairActions = $state([]);
    this.spellcastingLevel = $state(undefined);
    this.spellcastingAttribute = $state(undefined);
    this.spellcastingDC = $state(undefined);
    this.spellcastingAttackBonus = $state(undefined);
  }

  public AddVision(event: MouseEvent) {
    this.visions = [...this.visions, { type: undefined, range: undefined }];

    event.preventDefault();
  }

  public RemoveVision(visionToRemove: Vision) {
    this.visions = this.visions.filter((vision) => vision !== visionToRemove);

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddSpeed(event: MouseEvent) {
    this.speeds = [...this.speeds, { type: undefined, distance: undefined }];

    event.preventDefault();
  }

  public RemoveSpeed(speedToRemove: Speed) {
    this.speeds = this.speeds.filter((speed) => speed !== speedToRemove);

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddTrait(event: MouseEvent) {
    this.traits = [...this.traits, { name: undefined, description: undefined }];

    event.preventDefault();
  }

  public RemoveTrait(traitToRemove: NamedDescription) {
    this.traits = this.traits.filter((trait) => trait !== traitToRemove);
    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddRegularAction(event: MouseEvent) {
    this.regularActions = [
      ...this.regularActions,
      { name: undefined, description: undefined },
    ];

    event.preventDefault();
  }

  public RemoveRegularAction(regularActionToRemove: NamedDescription) {
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
        name: undefined,
        hitBonus: undefined,
        reach: undefined,
        oneHandedAttack: undefined,
        twoHandedAttack: undefined,
        damageType: undefined,
      },
    ];

    event.preventDefault();
  }

  public RemoveMeleeAttackAction(meleeAttackActionToRemove: MeleeAttack) {
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
        name: undefined,
        hitBonus: undefined,
        normalRange: undefined,
        longRange: undefined,
        attack: undefined,
        damageType: undefined,
      },
    ];

    event.preventDefault();
  }

  public RemoveRangedAttackAction(rangedAttackActionToRemove: RangedAttack) {
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
        name: undefined,
        rechargeDice: undefined,
        description: undefined,
      },
    ];

    event.preventDefault();
  }

  public RemoveRechargeAction(rechargeActionToRemove: RechargeAction) {
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
      { name: undefined, description: undefined },
    ];

    event.preventDefault();
  }

  public RemoveBonusAction(bonusActionToRemove: NamedDescription) {
    this.bonusActions = this.bonusActions.filter(
      (bonusAction) => bonusAction !== bonusActionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddReaction(event: MouseEvent) {
    this.reactions = [
      ...this.reactions,
      { name: undefined, description: undefined },
    ];

    event.preventDefault();
  }

  public RemoveReaction(reactionToRemove: NamedDescription) {
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
      { name: undefined, cost: undefined, description: undefined },
    ];

    event.preventDefault();
  }

  public RemoveLegendaryAction(legendaryActionToRemove: LegendaryAction) {
    this.legendaryActions = this.legendaryActions.filter(
      (legendaryAction) => legendaryAction !== legendaryActionToRemove,
    );

    // Clear available legendary actions if all legendary actions are removed
    if (this.legendaryActions.length === 0) {
      this.availableLegendaryActionsPerTurn = undefined;
    }

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  public AddLairAction(event: MouseEvent) {
    this.lairActions = [
      ...this.lairActions,
      { name: undefined, description: undefined },
    ];

    event.preventDefault();
  }

  public RemoveLairAction(lairActionToRemove: NamedDescription) {
    this.lairActions = this.lairActions.filter(
      (lairAction) => lairAction !== lairActionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }
}
