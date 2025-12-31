use iced::{
    Background, Border, Theme,
    border::Radius,
    widget::button::{Status, Style},
};

use super::base;

pub mod ghost {
    use super::*;

    pub fn default(theme: &Theme, status: Status) -> Style {
        let extended = theme.extended_palette();

        let background = match status {
            Status::Active | Status::Disabled => {
                Some(Background::Color(extended.background.base.color))
            }
            Status::Hovered => Some(Background::Color(extended.background.strong.color)),
            Status::Pressed => Some(Background::Color(extended.background.strongest.color)),
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
            width: 1.0,
            radius: Radius::new(0.0),
        };

        Style {
            border,
            ..default(theme, status)
        }
    }
}
