use crate::{
    NO_BORDER, NO_SHADOW,
    container::{fade_edges, left_to_right},
};
use iced::{Background, Theme, widget::container::Style};

pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();

    let background = Background::Color(palette.primary);

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

    let background = Background::Color(extended.primary.weak.color);

    Style {
        background: Some(background),
        ..default(theme)
    }
}

pub fn strong(theme: &Theme) -> Style {
    let extended = theme.extended_palette();

    let background = Background::Color(extended.primary.strong.color);

    Style {
        background: Some(background),
        ..default(theme)
    }
}

pub fn gradient_left_to_right(theme: &Theme) -> Style {
    let background = left_to_right(theme.palette().primary);

    Style {
        background: Some(background),
        ..Default::default()
    }
}

pub fn gradient_faded_edges(theme: &Theme) -> Style {
    let background = fade_edges(theme.palette().primary);

    Style {
        background: Some(background),
        ..Default::default()
    }
}
