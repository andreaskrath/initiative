import {
  Alignment,
  Language,
  Languages,
  MonsterType,
  Movement,
  Recharge,
  Sight,
  Size,
  Skill,
  Skills,
} from "$monster/types";
import {
  Attribute,
  Attributes,
  Condition,
  Conditions,
  DamageType,
  DamageTypes,
} from "$shared/types";
import { RecordFactory } from "$shared/utils/factories";
import { type Spell } from "$spell/types";

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

export type Monster = {
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
  spells: Spell[];
};

export const MonsterActions = {
  EmptyMonster: (): Monster => ({
    name: undefined,
    challengeRating: undefined,
    xp: undefined,
    proficiencyBonus: undefined,
    size: undefined,
    monsterType: undefined,
    species: undefined,
    alignment: undefined,
    attributes: RecordFactory(Attributes, undefined),
    hitPoints: undefined,
    rollableHitPoints: undefined,
    armorClass: undefined,
    armorType: undefined,
    savingThrows: RecordFactory(Attributes, undefined),
    damageResistances: RecordFactory(DamageTypes, false),
    damageImmunities: RecordFactory(DamageTypes, false),
    conditionImmunities: RecordFactory(Conditions, false),
    visions: [],
    passivePerception: undefined,
    speeds: [],
    languages: RecordFactory(Languages, false),
    skills: RecordFactory(Skills, undefined),
    traits: [],
    regularActions: [],
    meleeAttackActions: [],
    rangedAttackActions: [],
    rechargeActions: [],
    bonusActions: [],
    reactions: [],
    availableLegendaryActionsPerTurn: undefined,
    legendaryActions: [],
    lairActions: [],
    spellcastingLevel: undefined,
    spellcastingAttribute: undefined,
    spellcastingDC: undefined,
    spellcastingAttackBonus: undefined,
    spells: [],
  }),

  AddVision: (monster: Monster, event: MouseEvent): void => {
    monster.visions = [
      ...monster.visions,
      { type: undefined, range: undefined },
    ];

    event.preventDefault();
  },

  RemoveVision: (monster: Monster, visionToRemove: Vision) => {
    monster.visions = monster.visions.filter(
      (vision) => vision !== visionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  AddSpeed: (monster: Monster, event: MouseEvent) => {
    monster.speeds = [
      ...monster.speeds,
      { type: undefined, distance: undefined },
    ];

    event.preventDefault();
  },

  RemoveSpeed: (monster: Monster, speedToRemove: Speed) => {
    monster.speeds = monster.speeds.filter((speed) => speed !== speedToRemove);

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  AddTrait: (monster: Monster, event: MouseEvent) => {
    monster.traits = [
      ...monster.traits,
      { name: undefined, description: undefined },
    ];

    event.preventDefault();
  },

  RemoveTrait: (monster: Monster, traitToRemove: NamedDescription) => {
    monster.traits = monster.traits.filter((trait) => trait !== traitToRemove);
    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  AddRegularAction: (monster: Monster, event: MouseEvent) => {
    monster.regularActions = [
      ...monster.regularActions,
      { name: undefined, description: undefined },
    ];

    event.preventDefault();
  },

  RemoveRegularAction: (
    monster: Monster,
    regularActionToRemove: NamedDescription,
  ) => {
    monster.regularActions = monster.regularActions.filter(
      (regularAction) => regularAction !== regularActionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  AddMeleeAttackAction: (monster: Monster, event: MouseEvent) => {
    monster.meleeAttackActions = [
      ...monster.meleeAttackActions,
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
  },

  RemoveMeleeAttackAction: (
    monster: Monster,
    meleeAttackActionToRemove: MeleeAttack,
  ) => {
    monster.meleeAttackActions = monster.meleeAttackActions.filter(
      (meleeAttackAction) => meleeAttackAction !== meleeAttackActionToRemove,
    );
    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  AddRangedAttackAction: (monster: Monster, event: MouseEvent) => {
    monster.rangedAttackActions = [
      ...monster.rangedAttackActions,
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
  },

  RemoveRangedAttackAction: (
    monster: Monster,
    rangedAttackActionToRemove: RangedAttack,
  ) => {
    monster.rangedAttackActions = monster.rangedAttackActions.filter(
      (rangedAttackAction) => rangedAttackAction !== rangedAttackActionToRemove,
    );
    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  AddRechargeAction: (monster: Monster, event: MouseEvent) => {
    monster.rechargeActions = [
      ...monster.rechargeActions,
      {
        name: undefined,
        rechargeDice: undefined,
        description: undefined,
      },
    ];

    event.preventDefault();
  },

  RemoveRechargeAction: (
    monster: Monster,
    rechargeActionToRemove: RechargeAction,
  ) => {
    monster.rechargeActions = monster.rechargeActions.filter(
      (rechargeAction) => rechargeAction !== rechargeActionToRemove,
    );
    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  AddBonusAction: (monster: Monster, event: MouseEvent) => {
    monster.bonusActions = [
      ...monster.bonusActions,
      { name: undefined, description: undefined },
    ];

    event.preventDefault();
  },

  RemoveBonusAction: (
    monster: Monster,
    bonusActionToRemove: NamedDescription,
  ) => {
    monster.bonusActions = monster.bonusActions.filter(
      (bonusAction) => bonusAction !== bonusActionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  AddReaction: (monster: Monster, event: MouseEvent) => {
    monster.reactions = [
      ...monster.reactions,
      { name: undefined, description: undefined },
    ];

    event.preventDefault();
  },

  RemoveReaction: (monster: Monster, reactionToRemove: NamedDescription) => {
    monster.reactions = monster.reactions.filter(
      (reaction) => reaction !== reactionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  AddLegendaryAction: (monster: Monster, event: MouseEvent) => {
    monster.legendaryActions = [
      ...monster.legendaryActions,
      { name: undefined, cost: undefined, description: undefined },
    ];

    event.preventDefault();
  },

  RemoveLegendaryAction: (
    monster: Monster,
    legendaryActionToRemove: LegendaryAction,
  ) => {
    monster.legendaryActions = monster.legendaryActions.filter(
      (legendaryAction) => legendaryAction !== legendaryActionToRemove,
    );

    // Clear available legendary actions if all legendary actions are removed
    if (monster.legendaryActions.length === 0) {
      monster.availableLegendaryActionsPerTurn = undefined;
    }

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  AddLairAction: (monster: Monster, event: MouseEvent) => {
    monster.lairActions = [
      ...monster.lairActions,
      { name: undefined, description: undefined },
    ];

    event.preventDefault();
  },

  RemoveLairAction: (
    monster: Monster,
    lairActionToRemove: NamedDescription,
  ) => {
    monster.lairActions = monster.lairActions.filter(
      (lairAction) => lairAction !== lairActionToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },

  RemoveSpell: (monster: Monster, spellToRemove: Spell) => {
    monster.spells = monster.spells.filter((spell) => spell !== spellToRemove);

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },
};
