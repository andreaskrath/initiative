use crate::condition::{ConditionTable, NEW_CONDITION_TABLE};

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    initiative: i8,
    conditions: ConditionTable,
}

impl Player {
    pub fn new(name: String, initiative: i8) -> Self {
        Self {
            name,
            initiative,
            conditions: NEW_CONDITION_TABLE,
        }
    }
}
