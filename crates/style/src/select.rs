use iced::{
    Background, Border, Theme,
    widget::pick_list::{Status, Style},
};

use crate::{DEFAULT_BORDER, MUTED_SCALE};

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let background = match status {
        Status::Active | Status::Hovered => Background::Color(palette.background),
        Status::Opened { is_hovered: _ } => Background::Color(extended.background.weakest.color),
    };

    let border_color = match status {
        Status::Active => extended.background.strong.color,
        Status::Hovered => palette.text.scale_alpha(0.4),
        Status::Opened { is_hovered: _ } => palette.text,
    };

    let border = Border {
        color: border_color,
        ..DEFAULT_BORDER
    };

    Style {
        text_color: palette.text,
        placeholder_color: palette.text.scale_alpha(MUTED_SCALE),
        handle_color: extended.background.strong.color,
        background,
        border,
    }
}

pub fn error(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let border_color = match status {
        Status::Active => palette.text,
        Status::Hovered => extended.danger.base.color,
        Status::Opened { is_hovered: _ } => extended.danger.strong.color,
    };

    let border = Border {
        color: border_color,
        ..DEFAULT_BORDER
    };
    Style {
        border,
        ..default(theme, status)
    }
}
