use strum::{Display, VariantArray};

#[derive(Debug, Display, VariantArray, Clone, Copy, PartialEq)]
pub enum SpellShapeKind {
    #[strum(to_string = "No shape")]
    NoShape,
    Cone,
    Cube,
    Cylinder,
    Line,
    Sphere,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpellShape {
    NoShape,
    Cone { length: u16 },
    Cube { length: u16 },
    Cylinder { radius: u16, height: u16 },
    Line { width: u16, length: u16 },
    Sphere { radius: u16 },
}
