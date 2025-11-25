use strum::{Display, VariantArray};

#[derive(Debug, Display, VariantArray, Clone, Copy, PartialEq)]
pub enum SpellLevel {
    Cantrip,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
}
