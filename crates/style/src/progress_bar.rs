use crate::NO_BORDER_ROUNDED;
use crate::theme::Theme;

use iced::Background;
use iced::widget::progress_bar::Catalog;
use iced::widget::progress_bar::Style;

impl Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>) -> Style {
        Style {
            background: Background::Color(self.secondary),
            bar: Background::Color(self.primary),
            border: NO_BORDER_ROUNDED,
        }
    }
}
