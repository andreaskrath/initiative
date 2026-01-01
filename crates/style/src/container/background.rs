use iced::{Background, Theme, widget::container::Style};

use crate::{NO_BORDER, NO_SHADOW};

pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();

    let background = Background::Color(palette.background);

    Style {
        text_color: None,
        background: Some(background),
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}

pub fn weak(theme: &Theme) -> Style {
    let extended = theme.extended_palette();

    let background = Background::Color(extended.background.weak.color);

    Style {
        background: Some(background),
        ..default(theme)
    }
}

pub fn strong(theme: &Theme) -> Style {
    let extended = theme.extended_palette();

    let background = Background::Color(extended.background.strong.color);

    Style {
        background: Some(background),
        ..default(theme)
    }
}
