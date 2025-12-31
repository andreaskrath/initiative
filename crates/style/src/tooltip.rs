use iced::{Background, Border, Theme, border::Radius, widget::container::Style};

use crate::NO_SHADOW;

pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let text_color = Some(palette.text);

    let background = Some(Background::Color(extended.background.base.color));

    let border = Border {
        color: palette.text,
        width: 1.0,
        radius: Radius::new(0.0),
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
        width: 1.0,
        radius: Radius::new(0.0),
    };

    Style {
        text_color,
        border,
        ..default(theme)
    }
}
