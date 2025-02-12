use crate::create_new_enemy::CreateNewEnemy;

pub enum Screen {
    Encounter,
    NoEncounter,
    CreateNewEnemy(CreateNewEnemy),
    CreateNewPlayer,
}

impl Default for Screen {
    fn default() -> Self {
        Self::NoEncounter
    }
}
