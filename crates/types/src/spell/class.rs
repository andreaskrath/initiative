use std::ops::{Index, IndexMut};

use strum::{Display, EnumCount, VariantArray};

#[derive(Display, VariantArray, Clone, Copy, EnumCount)]
pub enum Class {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl Index<Class> for [bool; Class::COUNT] {
    type Output = bool;

    fn index(&self, index: Class) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Class> for [bool; Class::COUNT] {
    fn index_mut(&mut self, index: Class) -> &mut Self::Output {
        &mut self[index as usize]
    }
}
