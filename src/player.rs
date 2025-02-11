use crate::condition::{ConditionTable, NEW_CONDITION_TABLE};

pub struct Player<'a> {
    name: &'a str,
    initiative: Option<i8>,
    conditions: ConditionTable,
}

impl<'a> Player<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            initiative: None,
            conditions: NEW_CONDITION_TABLE,
        }
    }
}
