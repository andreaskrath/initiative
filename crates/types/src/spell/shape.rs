use gpui::SharedString;
use gpui_component::select::SelectItem;
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

impl SelectItem for SpellShape {
    type Value = Self;

    fn title(&self) -> SharedString {
        self.to_string().into()
    }

    fn value(&self) -> &Self::Value {
        self
    }
}
