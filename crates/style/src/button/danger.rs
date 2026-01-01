use super::base;
use iced::{
    Background, Theme,
    widget::button::{Status, Style},
};

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
}
