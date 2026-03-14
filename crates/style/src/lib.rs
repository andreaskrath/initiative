use iced::{Border, Color, Shadow, Vector, border::Radius};

pub mod button;
pub mod color;
pub mod container;
pub mod icon;
pub mod menu;
pub mod rule;
pub mod text;
pub mod theme;
pub mod tooltip;

pub use theme::ThemeVariant;

/// A scale used to adjust alpha for muted styles.
pub const MUTED_SCALE: f32 = 0.6;

/// A `Shadow` instance that is transparent, with no offset and no radius.
pub const NO_SHADOW: Shadow = Shadow {
    color: Color::TRANSPARENT,
    offset: Vector::ZERO,
    blur_radius: 0.0,
};

/// A `Border` instance that is transparent, with zero width and no radius.
pub const NO_BORDER: Border = Border {
    color: Color::TRANSPARENT,
    width: 0.0,
    radius: Radius {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    },
};

/// The default border utilized by the application.
///
/// This should mostly be utilized while providing a `color`.
pub const DEFAULT_BORDER: Border = Border {
    color: Color::TRANSPARENT,
    width: 1.0,
    radius: Radius {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    },
};
