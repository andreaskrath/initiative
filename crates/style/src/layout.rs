//! Defines constants related to layout of widgets and components.

use iced::Padding;

/// Spacing between sections.
pub const SECTION_SPACING: u32 = 50;

/// Spacing internal to section bodies.
pub const BODY_SPACING: u32 = 10;

/// Spacing in between the label and the item it is labeling.
pub const LABEL_SPACING: u32 = 5;

/// Padding for all "input"-like elements; essentially anything that is part of a form.
pub const INPUT_PADDING: Padding = Padding {
    top: 8.0,
    right: 12.0,
    bottom: 8.0,
    left: 12.0,
};
