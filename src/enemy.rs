use crate::{
    condition::{ConditionTable, NEW_CONDITION_TABLE},
    damage::{DamageTable, NEW_DAMAGE_TABLE},
};

pub struct Enemy<'a> {
    pub name: &'a str,
    pub initiative: Option<i8>,
    pub conditions: ConditionTable,
    pub vulnerabilies: DamageTable,
    pub immunities: DamageTable,
}

impl<'a> Enemy<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            initiative: None,
            conditions: NEW_CONDITION_TABLE,
            vulnerabilies: NEW_DAMAGE_TABLE,
            immunities: NEW_DAMAGE_TABLE,
        }
    }
}
