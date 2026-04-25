use thiserror::Error;

#[derive(Debug, Error, Clone)]
/// A user facing error providing context on what prevented an image from being added.
pub enum ImageError {
    #[error("An unknown error occured.")]
    Unknown,

    #[error(
        "Could not communicate with your clipboard. Make sure you have a clipboard provider installed and exposed."
    )]
    Clipboard,

    #[error("The clipboard does not contain image data.")]
    NoImageContent,

    #[error("The clipboard image data is malformed.")]
    MalformedImageData,

    #[error("Failed to encode clipboard image as PNG.")]
    EncodingFailed,

    #[error("Failed to load {0}.")]
    LoadImageFailed(String),

    #[error("{0} is not a valid image file.")]
    FileIsNotImage(String),
}
