use crate::NO_BORDER_ROUNDED;
use crate::hovered;
use crate::pressed;
use crate::theme::Theme;

use iced::Background;
use iced::Color;
use iced::widget::slider::Catalog;
use iced::widget::slider::Handle;
use iced::widget::slider::HandleShape;
use iced::widget::slider::Rail;
use iced::widget::slider::Status;
use iced::widget::slider::Style;

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
