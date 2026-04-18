pub mod button;
pub mod checkbox;
pub mod combobox;
pub mod container;
pub mod float;
pub mod layout;
pub mod menu;
pub mod pick_list;
pub mod progress_bar;
pub mod radio;
pub mod rule;
pub mod scrollable;
pub mod slider;
pub mod svg;
pub mod table;
pub mod text;
pub mod text_editor;
pub mod text_input;
pub mod theme;
pub mod toggler;

use iced::Border;
use iced::Color;
use iced::Shadow;
use iced::Vector;
use iced::border::Radius;

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

/// No border but rounded edges.
pub const NO_BORDER_ROUNDED: Border = Border {
    color: Color::TRANSPARENT,
    width: 0.0,
    radius: Radius {
        top_left: 2.0,
        top_right: 2.0,
        bottom_right: 2.0,
        bottom_left: 2.0,
    },
};

/// This should mostly be used when overriding the color.
pub const DEFAULT_BORDER: Border = Border {
    color: Color::TRANSPARENT,
    width: 1.0,
    radius: Radius {
        top_left: 2.0,
        top_right: 2.0,
        bottom_right: 2.0,
        bottom_left: 2.0,
    },
};

/// Adjust the `color` to the hovered variant.
pub(crate) fn hovered(color: Color) -> Color {
    /// This value is used to lighten the color.
    ///
    /// The value 0.1 means 10% more towards white.
    const HOVER_SCALE: f32 = 0.05;

    Color {
        r: color.r + (1.0 - color.r) * HOVER_SCALE,
        g: color.g + (1.0 - color.g) * HOVER_SCALE,
        b: color.b + (1.0 - color.b) * HOVER_SCALE,
        a: color.a,
    }
}

/// Adjust the `color` to the pressed variant.
pub(crate) fn pressed(color: Color) -> Color {
    /// This value is used to lighten the color.
    ///
    /// The value 0.1 means 10% more towards white.
    const HOVER_SCALE: f32 = 0.1;

    Color {
        r: color.r + (1.0 - color.r) * HOVER_SCALE,
        g: color.g + (1.0 - color.g) * HOVER_SCALE,
        b: color.b + (1.0 - color.b) * HOVER_SCALE,
        a: color.a,
    }
}

// /// Adjust the `color` to the pressed variant.
// pub(crate) fn pressed(color: Color) -> Color {
//     /// This value is used to darken the color.
//     ///
//     /// The value 0.1 means 10% more towards black.
//     const PRESSED_SCALE: f32 = 0.1;
//
//     Color {
//         r: color.r * (1.0 - PRESSED_SCALE),
//         g: color.g * (1.0 - PRESSED_SCALE),
//         b: color.b * (1.0 - PRESSED_SCALE),
//         a: color.a,
//     }
// }

/// Adjust the `color` to the disabled variant.
pub(crate) fn disabled(color: Color) -> Color {
    /// This value sets the opacity of the input color.
    const DISABLED_SCALE: f32 = 0.5;

    color.scale_alpha(DISABLED_SCALE)
}
