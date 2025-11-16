use gpui::SharedString;
use gpui_component::select::SelectItem;
use strum::{Display, VariantArray};

#[derive(Display, VariantArray, Clone, Copy)]
pub enum SpellRange {
    #[strum(to_string = "Self")]
    Caster,
    Touch,
    Sight,
    #[strum(to_string = "30 ft.")]
    Feet30,
    #[strum(to_string = "60 ft.")]
    Feet60,
    #[strum(to_string = "90 ft.")]
    Feet90,
    #[strum(to_string = "120 ft.")]
    Feet120,
    #[strum(to_string = "150 ft.")]
    Feet150,
    #[strum(to_string = "300 ft.")]
    Feet300,
    #[strum(to_string = "500 ft.")]
    Feet500,
    #[strum(to_string = "1 mile")]
    OneMile,
    Unlimited,
}

impl SelectItem for SpellRange {
    type Value = Self;

    fn title(&self) -> SharedString {
        self.to_string().into()
    }

    fn value(&self) -> &Self::Value {
        self
    }
}
