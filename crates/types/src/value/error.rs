use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum ValueError {
    #[error("Must be specified.")]
    Required,
    #[error("Must be longer than {0} characters.")]
    Short(usize),
    #[error("Must be shorter than {0} characters.")]
    Long(usize),
    #[error("Must be between {0} and {1} characters.")]
    Between(usize, usize),
}
