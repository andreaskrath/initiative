use crate::{enemy::Enemy, player::Player};

pub type EntityTable = Vec<Entity>;

#[derive(Debug, Clone)]
pub enum Entity {
    Player(Player),
    Enemy(Enemy),
}
