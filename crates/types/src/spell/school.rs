use strum::Display;
use strum::VariantArray;

#[derive(Debug, Display, VariantArray, Clone, Copy, PartialEq)]
pub enum School {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}
