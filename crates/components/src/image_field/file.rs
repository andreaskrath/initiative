use crate::image_field::error::ImageError;

use rfd::AsyncFileDialog;
use std::path::PathBuf;

/// Open a native image picker, that may resolve to a `PathBuf`.
pub async fn open_image_picker() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .add_filter("Images", &["png", "jpg", "jpeg", "webp"])
        .pick_file()
        .await
        .map(|handle| handle.path().to_owned())
}

pub async fn load_image(path: PathBuf) -> Result<Box<[u8]>, ImageError> {
    let Ok(bytes) = tokio::fs::read(&path).await else {
        return Err(ImageError::LoadImageFailed(
            path.to_string_lossy().to_string(),
        ));
    };

    if image::load_from_memory(&bytes).is_err() {
        return Err(ImageError::FileIsNotImage(
            path.to_string_lossy().to_string(),
        ));
    }

    Ok(bytes.into_boxed_slice())
}
