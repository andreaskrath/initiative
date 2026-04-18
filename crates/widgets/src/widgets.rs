pub mod animation;
// pub mod combo_box;
pub mod markdown;
pub mod multi_text_input;

use style::theme::Theme;



/// Type alias for [`iced::Element`] to use custom Theme from the `style` crate.
pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;
