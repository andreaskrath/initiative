use crate::{
    create_new_enemy::{self, CreateNewEnemyError},
    enemy::Enemy,
};

#[derive(Debug, Clone)]
pub enum Message {
    NewEncounter,
    CreateNewEnemy,
    UpdateNewEnemy(create_new_enemy::Action),
    SubmitNewEnemy(Result<Enemy, CreateNewEnemyError>),
    CreateNewPlayer,
}
