use crate::{create_new_enemy, enemy::Enemy};

#[derive(Debug, Clone)]
pub enum Message {
    NewEncounter,
    CreateNewEnemy,
    UpdateNewEnemy(create_new_enemy::Action),
    SubmitNewEnemy(Enemy),
    CreateNewPlayer,
}
