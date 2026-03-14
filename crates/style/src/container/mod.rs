use iced::{Background, Color, Gradient, gradient::Linear};
use std::f32::consts::PI;

pub mod background;
pub mod primary;

fn fade_edges(color: Color) -> Background {
    Background::Gradient(Gradient::Linear(
        Linear::new(PI / 2.0)
            .add_stop(0.0, Color::TRANSPARENT)
            .add_stop(0.5, color)
            .add_stop(1.0, Color::TRANSPARENT),
    ))
}

fn left_to_right(color: Color) -> Background {
    Background::Gradient(Gradient::Linear(
        Linear::new(PI / 2.0)
            .add_stop(0.0, color)
            .add_stop(1.0, Color::TRANSPARENT),
    ))
}

fn right_to_left(color: Color) -> Background {
    Background::Gradient(Gradient::Linear(
        Linear::new(3.0 * PI / 2.0)
            .add_stop(0.0, color)
            .add_stop(1.0, Color::TRANSPARENT),
    ))
}
