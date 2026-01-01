use iced::{
    Background, Border, Color, Shadow, Theme, Vector, border::Radius, widget::container::Style,
};

pub mod background;
pub mod primary;

use crate::MUTED_SCALE;

pub fn primary_muted(theme: &Theme) -> Style {
    let palette = theme.palette();

    let background = Background::Color(palette.primary.scale_alpha(MUTED_SCALE));

    let border = Border {
        color: Color::TRANSPARENT,
        width: 0.0,
        radius: Radius::new(0.0),
    };

    let shadow = Shadow {
        color: Color::TRANSPARENT,
        offset: Vector::ZERO,
        blur_radius: 0.0,
    };

    Style {
        text_color: None,
        background: Some(background),
        border,
        shadow,
        snap: true,
    }
}
