use strum::{Display, VariantArray};

#[derive(Display, VariantArray, Clone, Copy)]
pub enum SpellShape {
    #[strum(to_string = "No shape")]
    NoShape,
    Cone,
    Cube,
    Cylinder,
    Line,
    Sphere,
}
