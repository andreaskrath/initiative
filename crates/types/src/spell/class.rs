use std::ops::Index;
use std::ops::IndexMut;

use strum::AsRefStr;
use strum::Display;
use strum::EnumCount;
use strum::VariantArray;

/// The list of all spell casting classes.
pub const SPELLCASTING_CLASSES: [Class; 9] = [
    Class::Artificer,
    Class::Bard,
    Class::Cleric,
    Class::Druid,
    Class::Paladin,
    Class::Ranger,
    Class::Sorcerer,
    Class::Warlock,
    Class::Wizard,
];

#[derive(Debug, Display, VariantArray, Clone, Copy, EnumCount, PartialEq, AsRefStr)]
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
