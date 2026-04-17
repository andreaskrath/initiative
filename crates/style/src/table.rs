use crate::theme::Theme;

use iced::Background;
use iced::widget::table::Catalog;
use iced::widget::table::Style;

impl Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>) -> Style {
        let color = Background::Color(self.primary);

        Style {
            separator_x: color,
            separator_y: color,
        }
    }
}
