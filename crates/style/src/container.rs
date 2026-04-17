use crate::DEFAULT_BORDER;
use crate::NO_BORDER;
use crate::NO_BORDER_ROUNDED;
use crate::NO_SHADOW;
use crate::theme::Theme;

use iced::Background;
use iced::Border;
use iced::Color;
use iced::widget::container::Catalog;
use iced::widget::container::Style;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ContainerClass {
    Background,

    /// No text or background color, this is just a container.
    Ghost,
    Surface,
    Outlined,

    /// Used for info tooltips.
    Info,
    /// Used for error tooltips.
    Error,
}

impl Catalog for Theme {
    type Class<'a> = ContainerClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerClass::Background
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        if *class == ContainerClass::Ghost {
            return Style {
                text_color: None,
                background: None,
                border: NO_BORDER,
                shadow: NO_SHADOW,
                snap: true,
            };
        }

        if *class == ContainerClass::Info {
            return Style {
                text_color: Some(self.info),
                background: Some(Background::Color(self.surface)),
                border: Border {
                    color: self.info,
                    ..DEFAULT_BORDER
                },
                shadow: NO_SHADOW,
                snap: true,
            };
        }

        if *class == ContainerClass::Error {
            return Style {
                text_color: Some(self.danger),
                background: Some(Background::Color(self.surface)),
                border: Border {
                    color: self.danger,
                    ..DEFAULT_BORDER
                },
                shadow: NO_SHADOW,
                snap: true,
            };
        }

        let (background, border_color) = match class {
            ContainerClass::Background => (Some(self.background), None),
            ContainerClass::Surface => (Some(self.surface), None),
            ContainerClass::Outlined => (None, Some(self.primary)),
            ContainerClass::Ghost | ContainerClass::Info | ContainerClass::Error => unreachable!(
                "the normal, info, and error case are handled in guard clauses before the match"
            ),
        };

        Style {
            text_color: None,
            background: background.map(Background::Color),
            border: Border {
                color: border_color.unwrap_or(Color::TRANSPARENT),
                ..NO_BORDER_ROUNDED
            },
            shadow: NO_SHADOW,
            snap: true,
        }
    }
}
