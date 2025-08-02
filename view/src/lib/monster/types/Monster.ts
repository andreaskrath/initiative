import {
  Alignment,
  Language,
  MonsterType,
  Movement,
  Sight,
  Size,
  Skill,
} from "$monster/types";
import { Attribute, Condition, DamageType } from "$shared/types";
import { SpellLevel, type Spell } from "$spell/types";

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
  rechargeDice?: string;
  description?: string;
};

type LegendaryAction = {
  name?: string;
  cost?: number;
  description?: string;
};

export type Monster = {
  id?: string;
  name?: string;
  challengeRating?: number;
  xp?: number;
  proficiencyBonus?: number;
  size?: Size;
  monsterType?: MonsterType;
  species?: string;
  alignment?: Alignment;
  strength?: number;
  dexterity?: number;
  constitution?: number;
  intelligence?: number;
  wisdom?: number;
  charisma?: number;
  hitPoints?: number;
  rollableHitPoints?: string;
  armorClass?: number;
  armorType?: string;
  savingThrows: [Attribute, number][];
  damageResistances: DamageType[];
  damageImmunities: DamageType[];
  conditionImmunities: Condition[];
  visions: Vision[];
  passivePerception?: number;
  speeds: Speed[];
  languages: Language[];
  skills: [Skill, number][];
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
  spellcasting: {
    level?: number;
    attribute?: Attribute;
    dc?: number;
    attackBonus?: number;
    spellSlots: [SpellLevel, number][];
    spells: Spell[];
  };
};

export const MonsterActions = {
  EmptyMonster: (): Monster => ({
    id: undefined,
    name: undefined,
    challengeRating: undefined,
    xp: undefined,
    proficiencyBonus: undefined,
    size: undefined,
    monsterType: undefined,
    species: undefined,
    alignment: undefined,
    strength: undefined,
    dexterity: undefined,
    constitution: undefined,
    intelligence: undefined,
    wisdom: undefined,
    charisma: undefined,
    hitPoints: undefined,
    rollableHitPoints: undefined,
    armorClass: undefined,
    armorType: undefined,
    savingThrows: [],
    damageResistances: [],
    damageImmunities: [],
    conditionImmunities: [],
    visions: [],
    passivePerception: undefined,
    speeds: [],
    languages: [],
    skills: [],
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
    spellcasting: {
      level: undefined,
      attribute: undefined,
      dc: undefined,
      attackBonus: undefined,
      spellSlots: [],
      spells: [],
    },
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
    monster.spellcasting.spells = monster.spellcasting.spells.filter(
      (spell) => spell !== spellToRemove,
    );

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  },
};
