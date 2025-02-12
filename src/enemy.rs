use crate::{
    condition::{ConditionTable, NEW_CONDITION_TABLE},
    damage::{DamageTable, NEW_DAMAGE_TABLE},
};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: String,
    pub initiative: i8,
    pub conditions: ConditionTable,
    pub vulnerabilies: DamageTable,
    pub immunities: DamageTable,
    pub max_hp: u16,
    pub current_hp: u16,
    pub temp_hp: u16,
}

impl Enemy {
    pub fn new(name: String, initiative: i8, max_hp: u16) -> Self {
        Self {
            name,
            initiative,
            conditions: NEW_CONDITION_TABLE,
            vulnerabilies: NEW_DAMAGE_TABLE,
            immunities: NEW_DAMAGE_TABLE,
            max_hp,
            current_hp: max_hp,
            temp_hp: 0,
        }
    }
}
