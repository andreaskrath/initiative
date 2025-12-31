use iced::{Border, Color, Shadow, Vector, border::Radius};

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
