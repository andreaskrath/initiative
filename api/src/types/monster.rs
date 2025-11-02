use super::{
    Alignment, Attribute, Condition, DamageType, Language, MonsterType, Movement, Sight, Size,
    Skill, Spell, SpellLevel,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Monster {
    pub id: Option<Uuid>,
    pub name: String,
    pub challenge_rating: f32,
    pub xp: i16,
    pub proficiency_bonus: i16,
    pub size: Size,
    pub monster_type: MonsterType,
    pub species: Option<String>,
    pub alignment: Alignment,
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
    pub hit_points: i16,
    pub rollable_hit_points: String,
    pub armor_class: i16,
    pub armor_type: Option<String>,
    #[sqlx(json)]
    pub saving_throws: Vec<SavingThrow>,
    pub damage_resistances: Vec<DamageType>,
    pub damage_immunities: Vec<DamageType>,
    pub condition_immunities: Vec<Condition>,
    #[sqlx(json)]
    pub visions: Vec<Vision>,
    pub passive_perception: i16,
    #[sqlx(json)]
    pub speeds: Vec<Speed>,
    pub languages: Vec<Language>,
    #[sqlx(json)]
    pub skills: Vec<SkillModifier>,
    #[sqlx(json)]
    pub traits: Vec<Trait>,
    #[sqlx(json)]
    pub regular_actions: Vec<RegularAction>,
    #[sqlx(json)]
    pub melee_attack_actions: Vec<MeleeAttackAction>,
    #[sqlx(json)]
    pub ranged_attack_actions: Vec<RangedAttackAction>,
    #[sqlx(json)]
    pub recharge_actions: Vec<RechargeAction>,
    #[sqlx(json)]
    pub bonus_actions: Vec<BonusAction>,
    #[sqlx(json)]
    pub reactions: Vec<Reaction>,
    pub available_legendary_actions: Option<i16>,
    #[sqlx(json)]
    pub legendary_actions: Vec<LegendaryAction>,
    #[sqlx(json)]
    pub lair_actions: Vec<LairAction>,
    #[sqlx(json)]
    pub spellcasting: Option<Spellcasting>,
    #[sqlx(json)]
    pub reminders: Vec<TurnReminder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavingThrow {
    pub attribute: Attribute,
    pub modifier: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vision {
    pub sight: Sight,
    pub range: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Speed {
    pub movement: Movement,
    pub distance: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillModifier {
    pub skill: Skill,
    pub modifier: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trait {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegularAction {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeleeAttackAction {
    pub name: String,
    pub hit_bonus: i16,
    pub reach: i16,
    pub one_handed_attack: Option<String>,
    pub two_handed_attack: Option<String>,
    pub damage_type: DamageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangedAttackAction {
    pub name: String,
    pub hit_bonus: i16,
    pub normal_range: i16,
    pub long_range: i16,
    pub attack: String,
    pub damage_type: DamageType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RechargeAction {
    pub name: String,
    pub recharge: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BonusAction {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegendaryAction {
    pub name: String,
    pub cost: i16,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LairAction {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spellcasting {
    pub level: i16,
    pub attribute: Attribute,
    pub dc: i16,
    pub attack_bonus: i16,
    pub spell_slots: Vec<SpellSlot>,
    pub spells: Vec<Spell>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpellSlot {
    pub level: SpellLevel,
    pub slots: i16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    StartOfTurn,
    EndOfTurn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnReminder {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub trigger: Trigger,
}
