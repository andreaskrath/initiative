use strum::{Display, VariantArray};

#[derive(Display, VariantArray, Clone, Copy)]
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
