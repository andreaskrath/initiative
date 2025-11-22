use strum::{Display, VariantArray};

#[derive(Debug, Display, VariantArray, Clone, Copy)]
pub enum MagicSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}
