use crate::disabled;
use crate::hovered;
use crate::theme::Theme;

use iced::Background;
use iced::Color;
use iced::border::Radius;
use iced::widget::toggler::Catalog;
use iced::widget::toggler::Status;
use iced::widget::toggler::Style;

impl Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>, status: Status) -> Style {
        let (background, foreground) = match status {
            Status::Active { is_toggled } => {
                if is_toggled {
                    (self.primary, self.interaction)
                } else {
                    (self.interaction, self.primary)
                }
            }
            Status::Hovered { is_toggled } => {
                if is_toggled {
                    (hovered(self.primary), hovered(self.interaction))
                } else {
                    (hovered(self.interaction), hovered(self.primary))
                }
            }
            Status::Disabled { is_toggled } => {
                if is_toggled {
                    (disabled(self.primary), disabled(self.interaction))
                } else {
                    (disabled(self.interaction), disabled(self.primary))
                }
            }
        };

        Style {
            background: Background::Color(background),
            background_border_width: 0.0,
            background_border_color: Color::TRANSPARENT,
            foreground: Background::Color(foreground),
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
            text_color: None,
            border_radius: Some(Radius::new(2.0)),
            padding_ratio: 2.0,
        }
    }
}
