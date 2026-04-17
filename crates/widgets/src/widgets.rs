pub mod animation;
pub mod form;
pub mod icon;
pub mod label;
pub mod number_input;
pub mod select;
pub mod text;
pub mod text_area;
pub mod text_input;
pub mod toggle;

pub use icon::icon;
pub use number_input::number_input;
pub use select::select;
pub use text_area::text_area;
pub use text_input::text_input;
pub use toggle::toggle;

use style::theme::Theme;

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ValidationError {
    #[error("Must be specified.")]
    Required,
    #[error("Must be longer than {0} characters.")]
    Short(usize),
    #[error("Must be shorter than {0} characters.")]
    Long(usize),
    #[error("Must be between {0} and {1} characters.")]
    Between(usize, usize),
    #[error("Must be less than {0}.")]
    LessThan(i32),
    #[error("Must be greater than {0}.")]
    GreaterThan(i32),
    #[error("Must be between {0} and {1}.")]
    BetweenValue(i32, i32),
}

/// Type alias for [`iced::Element`] to use custom Theme from the `style` crate.
pub type Element<'a, M> = iced::Element<'a, M, Theme>;
