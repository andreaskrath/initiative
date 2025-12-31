use iced::{
    Theme,
    widget::button::{Status, Style},
};

use crate::{NO_BORDER, NO_SHADOW};

pub mod background;
pub mod danger;
pub mod primary;

fn base(theme: &Theme, _status: Status) -> Style {
    let palette = theme.palette();

    let text_color = palette.text;

    Style {
        background: None,
        text_color,
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}
