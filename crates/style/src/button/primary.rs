use super::base;
use crate::{DEFAULT_BORDER, color};
use iced::{
    Background, Border, Theme,
    widget::button::{Status, Style},
};

pub fn default(theme: &Theme, status: Status) -> Style {
    let extended = theme.extended_palette();

    let background = match status {
        Status::Active => Some(Background::Color(color::background::active(extended))),
        Status::Disabled => Some(Background::Color(color::background::disabled(extended))),
        Status::Hovered | Status::Pressed => {
            Some(Background::Color(color::background::hover(extended)))
        }
    };

    Style {
        background,
        ..base(theme, status)
    }
}

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

    pub fn background_outline_strong(theme: &Theme, status: Status) -> Style {
        let border = Border {
            color: theme.extended_palette().background.strong.color,
            ..DEFAULT_BORDER
        };

        Style {
            border,
            ..default(theme, status)
        }
    }
}
