mod animation;
mod icon;
mod select;
mod text;
mod text_area;
mod text_input;
mod toggle;

pub use animation::*;
pub use icon::*;
pub use select::*;
pub use text::*;
pub use text_area::*;
pub use text_input::*;
pub use toggle::*;

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
}
