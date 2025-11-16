use gpui::SharedString;
use gpui_component::select::SelectItem;
use strum::{Display, VariantArray};

#[derive(Display, VariantArray, Clone, Copy)]
pub enum SpellArea {
    #[strum(to_string = "Single target")]
    SingleTarget,
    #[strum(to_string = "Multiple target")]
    MultipleTargets,
    #[strum(to_string = "All creatures in range")]
    AllCreaturesInRange,
    #[strum(to_string = "5 ft.")]
    Feet5,
    #[strum(to_string = "10 ft.")]
    Feet10,
    #[strum(to_string = "15 ft.")]
    Feet15,
    #[strum(to_string = "20 ft.")]
    Feet20,
    #[strum(to_string = "25 ft.")]
    Feet25,
    #[strum(to_string = "30 ft.")]
    Feet30,
    #[strum(to_string = "35 ft.")]
    Feet35,
    #[strum(to_string = "40 ft.")]
    Feet40,
    Unlimited,
}

impl SelectItem for SpellArea {
    type Value = Self;

    fn title(&self) -> SharedString {
        self.to_string().into()
    }

    fn value(&self) -> &Self::Value {
        self
    }
}
