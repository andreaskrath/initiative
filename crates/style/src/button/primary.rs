use super::base;
use crate::DEFAULT_BORDER;
use iced::{
    Background, Border, Theme,
    widget::button::{Status, Style},
};

pub mod ghost {
    use super::*;

    pub fn default(theme: &Theme, status: Status) -> Style {
        let extended = theme.extended_palette();

        let background = match status {
            Status::Active | Status::Disabled | Status::Hovered => {
                Some(Background::Color(extended.primary.strong.color))
            }
            Status::Pressed => Some(Background::Color(extended.primary.base.color)),
        };

        Style {
            background,
            ..base(theme, status)
        }
    }

    pub fn outline(theme: &Theme, status: Status) -> Style {
        let palette = theme.palette();
        let border = Border {
            color: palette.text,
            ..DEFAULT_BORDER
        };

        Style {
            border,
            ..default(theme, status)
        }
    }
}
