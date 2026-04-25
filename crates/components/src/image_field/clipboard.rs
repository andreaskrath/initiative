use crate::image_field::error::ImageError;

use arboard::Clipboard;
use image::ImageBuffer;
use image::ImageFormat;
use image::Rgba;
use std::io::Cursor;

pub async fn get_image() -> Result<Box<[u8]>, ImageError> {
    // Have to spawn a blocking task, since clipboard stuff on Linux/X11 can be weird and cause blocking.
    tokio::task::spawn_blocking(|| {
        let Ok(mut clipboard) = Clipboard::new() else {
            return Err(ImageError::Clipboard);
        };

        let Ok(clipboard_data) = clipboard.get_image() else {
            return Err(ImageError::NoImageContent);
        };

        let Some(image_data) = ImageBuffer::<Rgba<u8>, _>::from_raw(
            clipboard_data.width as u32,
            clipboard_data.height as u32,
            clipboard_data.bytes,
        ) else {
            return Err(ImageError::MalformedImageData);
        };

        // Capacity is set to assume no compression from converting to PNG.
        //
        // This is unrealistic, but ensures no re-allocations during write.
        //
        // This is the dimensions of the image times four, because it is RGBA.
        let mut bytes = Vec::with_capacity((clipboard_data.width * clipboard_data.height) * 4);
        if image_data
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .is_err()
        {
            return Err(ImageError::EncodingFailed);
        }

        Ok(bytes.into_boxed_slice())
    })
    .await
    .unwrap_or(Err(ImageError::Unknown))
}
