use super::base;
use crate::{MUTED_SCALE, NO_BORDER, color};
use iced::{
    Background, Theme,
    widget::button::{Status, Style},
};

pub fn default(theme: &Theme, status: Status) -> Style {
    let extended = theme.extended_palette();

    let background = match status {
        Status::Active => Some(Background::Color(color::background::danger(extended))),
        Status::Disabled => Some(Background::Color(color::background::disabled(extended))),
        Status::Hovered | Status::Pressed => Some(Background::Color(
            color::background::danger(extended).scale_alpha(MUTED_SCALE),
        )),
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
            Status::Active | Status::Disabled => None,
            Status::Hovered => Some(Background::Color(extended.danger.base.color)),
            Status::Pressed => Some(Background::Color(extended.danger.strong.color)),
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
