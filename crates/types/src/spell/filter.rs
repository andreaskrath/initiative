use crate::{MagicSchool, SpellLevel};

#[derive(Debug, Clone, PartialEq)]
pub enum SpellFilter {
    Level(SpellLevel),
    School(MagicSchool),
}
