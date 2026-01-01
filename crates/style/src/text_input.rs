use iced::{
    Background, Border, Color, Theme,
    widget::text_input::{Status, Style},
};

use crate::DEFAULT_BORDER;

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let background = Background::Color(palette.background);

    let border_color = match status {
        Status::Active => extended.background.strong.color,
        Status::Hovered => palette.text.scale_alpha(0.4),
        Status::Focused { is_hovered: _ } => palette.text,
        Status::Disabled => Color::TRANSPARENT,
    };

    let border = Border {
        color: border_color,
        ..DEFAULT_BORDER
    };

    Style {
        background,
        border,
        icon: palette.text,
        placeholder: palette.text.scale_alpha(0.6),
        value: palette.text,
        selection: extended.background.strong.color,
    }
}

pub fn error(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let background = Background::Color(extended.danger.weak.color);

    let border_color = match status {
        Status::Active => palette.text,
        Status::Hovered => extended.danger.base.color,
        Status::Focused { is_hovered: _ } => extended.danger.strong.color,
        Status::Disabled => Color::TRANSPARENT,
    };

    let border = Border {
        color: border_color,
        ..DEFAULT_BORDER
    };

    Style {
        background,
        border,
        ..default(theme, status)
    }
}
