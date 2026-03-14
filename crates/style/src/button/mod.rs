use crate::{DEFAULT_BORDER, NO_SHADOW, color};
use iced::{
    Border, Theme,
    widget::button::{Status, Style},
};

pub mod background;
pub mod danger;
pub mod primary;

fn base(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let text_color = palette.text;

    let border_color = match status {
        Status::Disabled => color::border::disabled(extended),
        Status::Active => color::border::default(extended),
        Status::Hovered => color::border::hover(extended),
        Status::Pressed => color::border::focus(extended),
    };

    let border = Border {
        color: border_color,
        ..DEFAULT_BORDER
    };

    Style {
        background: None,
        text_color,
        border,
        shadow: NO_SHADOW,
        snap: true,
    }
}
