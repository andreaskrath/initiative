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
  type CreatureReminder,
  type Spell,
} from "$types";

type NamedDescription = { name?: string; description?: string };

type Vision = { sight?: Sight; range?: number };

type Speed = { movement?: Movement; distance?: number };

type MeleeAttack = {
  name?: string;
  hit_bonus?: number;
  reach?: number;
  one_handed_attack?: string;
  two_handed_attack?: string;
  damage_type?: DamageType;
};

type RangedAttack = {
  name?: string;
  hit_bonus?: number;
  normal_range?: number;
  long_range?: number;
  attack?: string;
  damage_type?: DamageType;
};

type RechargeAction = {
  name?: string;
  recharge?: string;
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
  challenge_rating?: number;
  xp?: number;
  proficiency_bonus?: number;
  size?: Size;
  monster_type?: MonsterType;
  species?: string;
  alignment?: Alignment;
  strength?: number;
  dexterity?: number;
  constitution?: number;
  intelligence?: number;
  wisdom?: number;
  charisma?: number;
  hit_points?: number;
  rollable_hit_points?: string;
  armor_class?: number;
  armor_type?: string;
  saving_throws: {
    attribute: Attribute;
    modifier: number;
  }[];
  damage_resistances: DamageType[];
  damage_immunities: DamageType[];
  condition_immunities: Condition[];
  visions: Vision[];
  passive_perception?: number;
  speeds: Speed[];
  languages: Language[];
  skills: {
    skill: Skill;
    modifier: number;
  }[];
  traits: NamedDescription[];
  regular_actions: NamedDescription[];
  melee_attack_actions: MeleeAttack[];
  ranged_attack_actions: RangedAttack[];
  recharge_actions: RechargeAction[];
  bonus_actions: NamedDescription[];
  reactions: NamedDescription[];
  available_legendary_actions_per_turn?: number;
  legendary_actions: LegendaryAction[];
  lair_actions: NamedDescription[];
  // While this can be undefined, it will only ever be undefined when sending the object to the backend.
  spellcasting?: {
    level?: number;
    attribute?: Attribute;
    dc?: number;
    attack_bonus?: number;
    spell_slots: {
      level: SpellLevel;
      slots: number;
    }[];
    spells: Spell[];
  };
  reminders: CreatureReminder[];
};

export const MonsterActions = {
  EmptyMonster: (): Monster => ({
    id: undefined,
    name: undefined,
    challenge_rating: undefined,
    xp: undefined,
    proficiency_bonus: undefined,
    size: undefined,
    monster_type: undefined,
    species: undefined,
    alignment: undefined,
    strength: undefined,
    dexterity: undefined,
    constitution: undefined,
    intelligence: undefined,
    wisdom: undefined,
    charisma: undefined,
    hit_points: undefined,
    rollable_hit_points: undefined,
    armor_class: undefined,
    armor_type: undefined,
    saving_throws: [],
    damage_resistances: [],
    damage_immunities: [],
    condition_immunities: [],
    visions: [],
    passive_perception: undefined,
    speeds: [],
    languages: [],
    skills: [],
    traits: [],
    regular_actions: [],
    melee_attack_actions: [],
    ranged_attack_actions: [],
    recharge_actions: [],
    bonus_actions: [],
    reactions: [],
    available_legendary_actions_per_turn: undefined,
    legendary_actions: [],
    lair_actions: [],
    spellcasting: {
      level: undefined,
      attribute: undefined,
      dc: undefined,
      attack_bonus: undefined,
      spell_slots: [],
      spells: [],
    },
    reminders: [],
  }),

  AddVision: (monster: Monster): void => {
    monster.visions = [
      ...monster.visions,
      { sight: undefined, range: undefined },
    ];
  },

  RemoveVision: (monster: Monster, visionToRemove: Vision) => {
    monster.visions = monster.visions.filter(
      (vision) => vision !== visionToRemove,
    );
  },

  AddSpeed: (monster: Monster) => {
    monster.speeds = [
      ...monster.speeds,
      { movement: undefined, distance: undefined },
    ];
  },

  RemoveSpeed: (monster: Monster, speedToRemove: Speed) => {
    monster.speeds = monster.speeds.filter((speed) => speed !== speedToRemove);
  },

  AddTrait: (monster: Monster) => {
    monster.traits = [
      ...monster.traits,
      { name: undefined, description: undefined },
    ];
  },

  RemoveTrait: (monster: Monster, traitToRemove: NamedDescription) => {
    monster.traits = monster.traits.filter((trait) => trait !== traitToRemove);
  },

  AddReminder: (monster: Monster) => {
    monster.reminders = [
      ...monster.reminders,
      {
        name: undefined,
        type: "reminder",
        description: undefined,
        trigger: undefined,
      },
    ];
  },

  RemoveReminder: (monster: Monster, reminderToRemove: CreatureReminder) => {
    monster.reminders = monster.reminders.filter(
      (reminder) => reminder !== reminderToRemove,
    );
  },

  AddRegularAction: (monster: Monster) => {
    monster.regular_actions = [
      ...monster.regular_actions,
      { name: undefined, description: undefined },
    ];
  },

  RemoveRegularAction: (
    monster: Monster,
    regularActionToRemove: NamedDescription,
  ) => {
    monster.regular_actions = monster.regular_actions.filter(
      (regularAction) => regularAction !== regularActionToRemove,
    );
  },

  AddMeleeAttackAction: (monster: Monster) => {
    monster.melee_attack_actions = [
      ...monster.melee_attack_actions,
      {
        name: undefined,
        hit_bonus: undefined,
        reach: undefined,
        one_handed_attack: undefined,
        two_handed_attack: undefined,
        damage_type: undefined,
      },
    ];
  },

  RemoveMeleeAttackAction: (
    monster: Monster,
    meleeAttackActionToRemove: MeleeAttack,
  ) => {
    monster.melee_attack_actions = monster.melee_attack_actions.filter(
      (meleeAttackAction) => meleeAttackAction !== meleeAttackActionToRemove,
    );
  },

  AddRangedAttackAction: (monster: Monster) => {
    monster.ranged_attack_actions = [
      ...monster.ranged_attack_actions,
      {
        name: undefined,
        hit_bonus: undefined,
        normal_range: undefined,
        long_range: undefined,
        attack: undefined,
        damage_type: undefined,
      },
    ];
  },

  RemoveRangedAttackAction: (
    monster: Monster,
    rangedAttackActionToRemove: RangedAttack,
  ) => {
    monster.ranged_attack_actions = monster.ranged_attack_actions.filter(
      (rangedAttackAction) => rangedAttackAction !== rangedAttackActionToRemove,
    );
  },

  AddRechargeAction: (monster: Monster) => {
    monster.recharge_actions = [
      ...monster.recharge_actions,
      {
        name: undefined,
        recharge: undefined,
        description: undefined,
      },
    ];
  },

  RemoveRechargeAction: (
    monster: Monster,
    rechargeActionToRemove: RechargeAction,
  ) => {
    monster.recharge_actions = monster.recharge_actions.filter(
      (rechargeAction) => rechargeAction !== rechargeActionToRemove,
    );
  },

  AddBonusAction: (monster: Monster) => {
    monster.bonus_actions = [
      ...monster.bonus_actions,
      { name: undefined, description: undefined },
    ];
  },

  RemoveBonusAction: (
    monster: Monster,
    bonusActionToRemove: NamedDescription,
  ) => {
    monster.bonus_actions = monster.bonus_actions.filter(
      (bonusAction) => bonusAction !== bonusActionToRemove,
    );
  },

  AddReaction: (monster: Monster) => {
    monster.reactions = [
      ...monster.reactions,
      { name: undefined, description: undefined },
    ];
  },

  RemoveReaction: (monster: Monster, reactionToRemove: NamedDescription) => {
    monster.reactions = monster.reactions.filter(
      (reaction) => reaction !== reactionToRemove,
    );
  },

  AddLegendaryAction: (monster: Monster) => {
    monster.legendary_actions = [
      ...monster.legendary_actions,
      { name: undefined, cost: undefined, description: undefined },
    ];
  },

  RemoveLegendaryAction: (
    monster: Monster,
    legendaryActionToRemove: LegendaryAction,
  ) => {
    monster.legendary_actions = monster.legendary_actions.filter(
      (legendaryAction) => legendaryAction !== legendaryActionToRemove,
    );

    // Clear available legendary actions if all legendary actions are removed
    if (monster.legendary_actions.length === 0) {
      monster.available_legendary_actions_per_turn = undefined;
    }
  },

  AddLairAction: (monster: Monster) => {
    monster.lair_actions = [
      ...monster.lair_actions,
      { name: undefined, description: undefined },
    ];
  },

  RemoveLairAction: (
    monster: Monster,
    lairActionToRemove: NamedDescription,
  ) => {
    monster.lair_actions = monster.lair_actions.filter(
      (lairAction) => lairAction !== lairActionToRemove,
    );
  },

  RemoveSpell: (monster: Monster, spellToRemove: Spell) => {
    monster.spellcasting!.spells = monster.spellcasting!.spells.filter(
      (spell) => spell !== spellToRemove,
    );
  },
};
