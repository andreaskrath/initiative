use crate::{NO_BORDER_ROUNDED, hovered, pressed, theme::Theme};

use iced::{
    Background, Color,
    widget::slider::{Catalog, Handle, HandleShape, Rail, Status, Style},
};

impl Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>, status: Status) -> Style {
        let handle = match status {
            Status::Active => self.primary,
            Status::Hovered => hovered(self.primary),
            Status::Dragged => pressed(self.primary),
        };

        let rail = Rail {
            // I think this is (filled, unfilled).
            backgrounds: (
                Background::Color(self.primary),
                Background::Color(self.interaction),
            ),
            width: 5.0,
            border: NO_BORDER_ROUNDED,
        };

        let handle = Handle {
            shape: HandleShape::Circle { radius: 5.0 },
            background: Background::Color(handle),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        };

        Style { rail, handle }
    }
}
