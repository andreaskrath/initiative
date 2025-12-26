use iced::{Theme, widget::text::Style};

use crate::MUTED_SCALE;

pub fn muted(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        color: Some(palette.text.scale_alpha(MUTED_SCALE)),
    }
}
