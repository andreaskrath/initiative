use super::base;
use crate::color;
use iced::{
    Background, Theme,
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
    use crate::NO_BORDER;

    use super::*;

    pub fn default(theme: &Theme, status: Status) -> Style {
        let extended = theme.extended_palette();

        let background = match status {
            Status::Active => Some(Background::Color(color::background::default(extended))),
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

    pub fn no_border(theme: &Theme, status: Status) -> Style {
        Style {
            border: NO_BORDER,
            ..default(theme, status)
        }
    }
}
