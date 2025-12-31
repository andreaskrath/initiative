use iced::{
    Background, Border, Theme,
    border::Radius,
    widget::button::{Status, Style},
};

use super::{NO_BORDER, NO_SHADOW};

pub mod ghost {
    use super::*;

    pub fn default(theme: &Theme, status: Status) -> Style {
        let palette = theme.palette();
        let extended = theme.extended_palette();

        let background = match status {
            Status::Active | Status::Disabled | Status::Hovered => {
                Some(Background::Color(extended.primary.strong.color))
            }
            Status::Pressed => Some(Background::Color(extended.primary.base.color)),
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
