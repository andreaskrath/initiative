use crate::{DEFAULT_BORDER, NO_SHADOW, color};
use iced::{Background, Border, Theme, widget::container::Style};

pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let text_color = Some(palette.text);

    let background = Some(Background::Color(color::background::default(extended)));

    let border = Border {
        color: color::border::default(extended),
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
    let palette = &theme.palette();
    let extended = theme.extended_palette();

    let text_color = Some(color::text::error(palette));

    let border = Border {
        color: color::border::error(extended),
        ..DEFAULT_BORDER
    };

    Style {
        text_color,
        border,
        ..default(theme)
    }
}
