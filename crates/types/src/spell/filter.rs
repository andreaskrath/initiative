use crate::{MagicSchool, SpellLevel};

#[derive(Debug, Clone)]
pub enum SpellFilter {
    Level(SpellLevel),
    School(MagicSchool),
}
