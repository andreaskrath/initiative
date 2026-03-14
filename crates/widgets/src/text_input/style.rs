use iced::{
    Background, Border, Color, Theme,
    widget::text_input::{Status, Style},
};
use style::{DEFAULT_BORDER, MUTED_SCALE, color};

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let background = match status {
        Status::Active => Background::Color(color::background::default(extended)),
        Status::Disabled => Background::Color(color::background::disabled(extended)),
        Status::Hovered => Background::Color(color::background::hover(extended)),
        Status::Focused { is_hovered: _ } => Background::Color(color::background::active(extended)),
    };

    let border_color = match status {
        Status::Disabled => color::border::disabled(extended),
        Status::Active => color::border::default(extended),
        Status::Hovered => color::border::hover(extended),
        Status::Focused { is_hovered: _ } => color::border::focus(extended),
    };

    let border = Border {
        color: border_color,
        ..DEFAULT_BORDER
    };

    Style {
        background,
        border,
        icon: palette.text,
        placeholder: palette.text.scale_alpha(MUTED_SCALE),
        value: palette.text,
        selection: color::background::default(extended),
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
