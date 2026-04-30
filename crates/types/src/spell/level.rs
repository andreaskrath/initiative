use strum::Display;
use strum::VariantArray;

#[derive(Debug, Display, VariantArray, Clone, Copy, PartialEq)]
pub enum Level {
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
