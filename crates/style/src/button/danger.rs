use iced::{
    Background, Theme,
    widget::button::{Status, Style},
};

use super::{NO_BORDER, NO_SHADOW};

pub mod ghost {
    use super::*;

    pub fn default(theme: &Theme, status: Status) -> Style {
        let palette = theme.palette();
        let extended = theme.extended_palette();

        let background = match status {
            Status::Active | Status::Disabled => None,
            Status::Hovered => Some(Background::Color(extended.danger.base.color)),
            Status::Pressed => Some(Background::Color(extended.danger.strong.color)),
        };

        let text_color = palette.text;

        Style {
            background,
            text_color,
            border: NO_BORDER,
            shadow: NO_SHADOW,
            snap: true,
        }
    }
}
