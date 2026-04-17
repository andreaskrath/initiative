use crate::NO_SHADOW;
use crate::theme::Theme;

use iced::border::Radius;
use iced::widget::float::Catalog;
use iced::widget::float::Style;

impl Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>) -> Style {
        Style {
            shadow: NO_SHADOW,
            shadow_border_radius: Radius::new(0.0),
        }
    }
}
