//! Defines low-level primitives and custom widgets.
//!
//! Rarely if ever, should the items defined in this crate be used directly in UI, they are meant
//! to be wrapped into more high-level and usable components.
//!
//! Additionally, this crate also defines default styled variants of some iced widgets.

pub mod animation;
// pub mod combo_box;
pub mod markdown;
pub mod multi_text_input;

pub use multi_text_input::multi_text_input;

use style::layout::INPUT_PADDING;
use style::theme::Theme;

use iced::widget;
use iced::widget::TextInput;

/// Type alias for [`iced::Element`] to use custom Theme from the `style` crate.
pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

/// A wrapper around [`iced::widget::text_input::text_input`] that applies default styling.
pub fn text_input<'a, Message, Renderer>(
    placeholder: &'a str,
    value: &'a str,
) -> TextInput<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
{
    widget::text_input(placeholder, value)
        .font(fonts::display::regular())
        .size(fonts::display::DEFAULT_DISPLAY_TEXT_SIZE)
        .padding(INPUT_PADDING)
}
