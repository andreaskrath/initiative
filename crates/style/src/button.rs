use crate::NO_BORDER_ROUNDED;
use crate::NO_SHADOW;
use crate::disabled;
use crate::hovered;
use crate::pressed;
use crate::theme::Theme;

use iced::Background;
use iced::Border;
use iced::Color;
use iced::widget::button::Catalog;
use iced::widget::button::Status;
use iced::widget::button::Style;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ButtonClass {
    Primary,
    Secondary,
    Tertiary,
    Interaction,
    Success,
    Warning,
    Danger,
    Ghost,
    Outlined,
    Overlay,
}

impl Catalog for Theme {
    type Class<'a> = ButtonClass;

    fn default<'a>() -> Self::Class<'a> {
        ButtonClass::Primary
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        if *class == ButtonClass::Ghost {
            let text = match status {
                Status::Active => self.text,
                Status::Hovered => hovered(self.text),
                Status::Pressed => pressed(self.text),
                Status::Disabled => disabled(self.text),
            };

            return Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: text,
                border: NO_BORDER_ROUNDED,
                shadow: NO_SHADOW,
                snap: true,
            };
        }

        if *class == ButtonClass::Overlay {
            let background = match status {
                Status::Active => self.interaction,
                Status::Hovered => hovered(self.interaction),
                Status::Pressed => pressed(self.interaction),
                Status::Disabled => disabled(self.interaction),
            };

            return Style {
                background: Some(Background::Color(background)),
                text_color: self.text,
                border: NO_BORDER_ROUNDED,
                shadow: NO_SHADOW,
                snap: true,
            };
        }

        if *class == ButtonClass::Outlined {
            let border_color = match status {
                Status::Active => self.primary,
                Status::Hovered => hovered(self.primary),
                Status::Pressed => pressed(self.primary),
                Status::Disabled => disabled(self.primary),
            };

            return Style {
                background: None,
                text_color: self.text,
                border: Border {
                    color: border_color,
                    ..NO_BORDER_ROUNDED
                },
                shadow: NO_SHADOW,
                snap: true,
            };
        }

        let mut text = self.text;
        let mut background = match class {
            ButtonClass::Primary => self.primary,
            ButtonClass::Secondary => self.secondary,
            ButtonClass::Tertiary => self.tertiary,
            ButtonClass::Success => self.success,
            ButtonClass::Warning => self.warning,
            ButtonClass::Danger => self.danger,
            ButtonClass::Interaction => self.interaction,
            ButtonClass::Ghost | ButtonClass::Outlined | ButtonClass::Overlay => {
                unreachable!(
                    "ghost, outlined, and overlay are handled in guard cases before this match"
                )
            }
        };

        match status {
            Status::Active => {}
            Status::Hovered => background = hovered(background),
            Status::Pressed => background = pressed(background),
            Status::Disabled => {
                background = disabled(background);
                text = disabled(self.text);
            }
        }

        Style {
            background: Some(Background::Color(background)),
            text_color: text,
            border: NO_BORDER_ROUNDED,
            shadow: NO_SHADOW,
            snap: true,
        }
    }
}
