use iced::{
    Background, Border, Color, Shadow, Theme, Vector,
    border::Radius,
    widget::button::{Status, Style},
};

use crate::MUTED_SCALE;

pub fn ghost_background(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let (background, border) = match status {
        Status::Active | Status::Disabled => (
            None,
            Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::new(4.0),
            },
        ),
        Status::Hovered => (
            Some(Background::Color(
                extended
                    .background
                    .weak
                    .color
                    .scale_alpha(MUTED_SCALE / 2.0),
            )),
            Border {
                color: extended.background.strong.color,
                width: 1.0,
                radius: Radius::new(4.0),
            },
        ),
        Status::Pressed => (
            Some(Background::Color(
                extended.background.strong.color.scale_alpha(MUTED_SCALE),
            )),
            Border {
                color: extended.primary.base.color,
                width: 2.0,
                radius: Radius::new(4.0),
            },
        ),
    };

    let text_color = palette.text;

    let shadow = Shadow {
        color: Color::TRANSPARENT,
        offset: Vector::ZERO,
        blur_radius: 0.0,
    };

    Style {
        background,
        text_color,
        border,
        shadow,
        snap: true,
    }
}

pub fn ghost_danger_no_edges(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let border = Border {
        color: Color::TRANSPARENT,
        width: 0.0,
        radius: Radius::new(0),
    };

    let (background, border) = match status {
        Status::Active | Status::Disabled => (None, border),
        Status::Hovered => (Some(Background::Color(palette.danger)), border),
        Status::Pressed => (
            Some(Background::Color(extended.danger.strong.color)),
            border,
        ),
    };

    let text_color = palette.text;

    let shadow = Shadow {
        color: Color::TRANSPARENT,
        offset: Vector::ZERO,
        blur_radius: 0.0,
    };

    Style {
        background,
        text_color,
        border,
        shadow,
        snap: true,
    }
}

pub fn ghost_background_outline(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let border = Border {
        color: palette.text,
        width: 1.0,
        radius: Radius::new(0.0),
    };

    let background = match status {
        Status::Active | Status::Disabled | Status::Hovered => None,
        // Status::Hovered => Some(Background::Color(extended.background.base.color)),
        Status::Pressed => Some(Background::Color(extended.background.strong.color)),
    };

    let text_color = palette.text;

    let shadow = Shadow {
        color: Color::TRANSPARENT,
        offset: Vector::ZERO,
        blur_radius: 0.0,
    };

    Style {
        background,
        text_color,
        border,
        shadow,
        snap: true,
    }
}

pub fn ghost_primary_outline(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let border = Border {
        color: palette.text,
        width: 1.0,
        radius: Radius::new(0.0),
    };

    let background = match status {
        Status::Active | Status::Disabled | Status::Hovered => {
            Some(Background::Color(extended.primary.strong.color))
        }
        Status::Pressed => Some(Background::Color(extended.primary.base.color)),
    };

    let text_color = palette.text;

    let shadow = Shadow {
        color: Color::TRANSPARENT,
        offset: Vector::ZERO,
        blur_radius: 0.0,
    };

    Style {
        background,
        text_color,
        border,
        shadow,
        snap: true,
    }
}
