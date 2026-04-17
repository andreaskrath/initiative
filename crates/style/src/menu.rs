use crate::NO_BORDER_ROUNDED;
use crate::NO_SHADOW;
use crate::hovered;
use crate::theme::Theme;

use iced::Background;
use iced::widget::overlay::menu::Catalog;
use iced::widget::overlay::menu::Style;

impl Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> <Self as Catalog>::Class<'a> {}

    fn style(&self, _class: &<Self as Catalog>::Class<'_>) -> Style {
        Style {
            background: Background::Color(self.interaction),
            border: NO_BORDER_ROUNDED,
            text_color: self.text,
            selected_text_color: self.text,
            selected_background: Background::Color(hovered(self.interaction)),
            shadow: NO_SHADOW,
        }
    }
}
