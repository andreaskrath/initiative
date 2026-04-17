pub mod form;
pub mod icon;
pub mod label;
pub mod multi_text_field;
pub mod number_field;
pub mod select_field;
pub mod text;
pub mod text_area_field;
pub mod text_field;
pub mod toggle;

pub use icon::icon;
pub use number_field::number_field;
pub use select_field::select_field;
pub use text_area_field::text_area_field;
pub use text_field::text_field;
pub use toggle::toggle;

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
