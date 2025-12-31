use iced::{
    Border, Color, Shadow, Theme, Vector,
    border::Radius,
    widget::button::{Status, Style},
};

pub mod background;
pub mod danger;
pub mod primary;

const NO_SHADOW: Shadow = Shadow {
    color: Color::TRANSPARENT,
    offset: Vector::ZERO,
    blur_radius: 0.0,
};

const NO_BORDER: Border = Border {
    color: Color::TRANSPARENT,
    width: 0.0,
    radius: Radius {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    },
};

fn base(theme: &Theme, _status: Status) -> Style {
    let palette = theme.palette();

    let text_color = palette.text;

    Style {
        background: None,
        text_color,
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}
