use gpui::SharedString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("{0} must be specified")]
    Required(SharedString),
    #[error("{0} must be at least {1} characters")]
    TooShort(SharedString, usize),
    #[error("{0} must be at most {1} characters")]
    TooLong(SharedString, usize),
    #[error("{0} must be between {1} and {2} characters")]
    Between(SharedString, usize, usize),
}
