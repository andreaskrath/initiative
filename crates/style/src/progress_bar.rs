use crate::{NO_BORDER_ROUNDED, theme::Theme};

use iced::{
    Background,
    widget::progress_bar::{Catalog, Style},
};

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
