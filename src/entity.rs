use crate::{enemy::Enemy, player::Player};

pub type EntityTable<'a> = Vec<Entity<'a>>;

pub enum Entity<'a> {
    Player(Player<'a>),
    Enemy(Enemy<'a>),
}
