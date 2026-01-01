use iced::{Background, Border, Theme, widget::container::Style};

use crate::{DEFAULT_BORDER, NO_SHADOW};

pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let text_color = Some(palette.text);

    let background = Some(Background::Color(extended.background.base.color));

    let border = Border {
        color: palette.text,
        ..DEFAULT_BORDER
    };

    Style {
        text_color,
        background,
        border,
        shadow: NO_SHADOW,
        snap: true,
    }
}

pub fn danger(theme: &Theme) -> Style {
    let extended = theme.extended_palette();

    let text_color = Some(extended.danger.base.color);

    let border = Border {
        color: extended.danger.base.color,
        ..DEFAULT_BORDER
    };

    Style {
        text_color,
        border,
        ..default(theme)
    }
}
