use super::{
    Alignment, Attribute, Condition, DamageType, Language, MonsterType, Movement, Sight, Size,
    Skill, Spell, SpellLevel,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Monster {
    id: Option<Uuid>,
    name: String,
    challenge_rating: f32,
    xp: i16,
    proficiency_bonus: i16,
    size: Size,
    monster_type: MonsterType,
    species: Option<String>,
    alignment: Alignment,
    strength: i16,
    dexterity: i16,
    constitution: i16,
    intelligence: i16,
    wisdom: i16,
    charisma: i16,
    hit_points: i16,
    rollable_hit_points: String,
    armor_class: i16,
    armor_type: Option<String>,
    #[sqlx(json)]
    saving_throws: Vec<SavingThrow>,
    damage_resistances: Vec<DamageType>,
    damage_immunities: Vec<DamageType>,
    condition_immunities: Vec<Condition>,
    #[sqlx(json)]
    visions: Vec<Vision>,
    passive_perception: i16,
    #[sqlx(json)]
    speeds: Vec<Speed>,
    languages: Vec<Language>,
    #[sqlx(json)]
    skills: Vec<SkillModifier>,
    #[sqlx(json)]
    traits: Vec<Trait>,
    #[sqlx(json)]
    regular_actions: Vec<RegularAction>,
    #[sqlx(json)]
    melee_attack_actions: Vec<MeleeAttackAction>,
    #[sqlx(json)]
    ranged_attack_actions: Vec<RangedAttackAction>,
    #[sqlx(json)]
    recharge_actions: Vec<RechargeAction>,
    #[sqlx(json)]
    bonus_actions: Vec<BonusAction>,
    #[sqlx(json)]
    reactions: Vec<Reaction>,
    available_legendary_actions: Option<i16>,
    #[sqlx(json)]
    legendary_actions: Vec<LegendaryAction>,
    #[sqlx(json)]
    lair_actions: Vec<LairAction>,
    #[sqlx(json)]
    spellcasting: Option<Spellcasting>,
    #[sqlx(json)]
    spell_slots: Vec<SpellSlot>,
    #[sqlx(json)]
    spells: Vec<Spell>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SavingThrow {
    attribute: Attribute,
    modifier: i16,
}

#[derive(Debug, Serialize, Deserialize)]
struct Vision {
    sight: Sight,
    range: i16,
}

#[derive(Debug, Serialize, Deserialize)]
struct Speed {
    movement: Movement,
    distance: i16,
}

#[derive(Debug, Serialize, Deserialize)]
struct SkillModifier {
    skill: Skill,
    modifier: i16,
}

#[derive(Debug, Serialize, Deserialize)]
struct Trait {
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegularAction {
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MeleeAttackAction {
    name: String,
    hit_bonus: i16,
    reach: i16,
    one_handed_attack: Option<String>,
    two_handed_attack: Option<String>,
    damage_type: DamageType,
}

#[derive(Debug, Serialize, Deserialize)]
struct RangedAttackAction {
    name: String,
    hit_bonus: i16,
    normal_range: i16,
    long_range: i16,
    attack: String,
    damage_type: DamageType,
}

#[derive(Debug, Serialize, Deserialize)]
struct RechargeAction {
    name: String,
    recharge: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BonusAction {
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reaction {
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LegendaryAction {
    name: String,
    cost: i16,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LairAction {
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Spellcasting {
    level: SpellLevel,
    attribute: Attribute,
    dc: i16,
    attack_bonus: i16,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpellSlot {
    level: SpellLevel,
    slots: i16,
}
