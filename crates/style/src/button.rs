use iced::{
    Background, Border, Color, Shadow, Theme, Vector,
    border::Radius,
    widget::button::{Status, Style},
};

pub fn ghost(theme: &Theme, status: Status) -> Style {
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
                extended.background.weak.color.scale_alpha(0.2),
            )),
            Border {
                color: extended.background.strong.color,
                width: 1.0,
                radius: Radius::new(4.0),
            },
        ),
        Status::Pressed => (
            Some(Background::Color(
                extended.background.strong.color.scale_alpha(0.5),
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
        snap: false,
    }
}
