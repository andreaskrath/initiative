use rust_embed::RustEmbed;
use thiserror::Error;
use tracing::{debug, error, warn};

#[derive(Error, Debug)]
pub enum AssetsError {
    #[error("failed to load asset '{0}'")]
    Load(String),
}

#[derive(RustEmbed, Default)]
#[folder = "../../assets"]
#[include = "fonts/**/*.otf"]
#[include = "icons/**/*.svg"]
struct Assets;

pub mod icons {
    use super::*;
    use iced::widget::svg::Handle;

    pub fn get(path: &str) -> Result<Handle, AssetsError> {
        let Some(bytes) = Assets::get(path) else {
            return Err(AssetsError::Load(path.to_string()));
        };

        Ok(Handle::from_memory(bytes.data))
    }
}

pub mod fonts {
    use super::*;
    use std::borrow::Cow;

    /// Get all embedded fonts.
    pub fn get() -> Vec<Cow<'static, [u8]>> {
        Assets::iter()
            .filter_map(|path| {
                let path_str = path.as_ref();

                if path_str.starts_with("fonts/") {
                    if let Some(bytes) = Assets::get(path_str) {
                        debug!("loading font '{path_str}'");
                        Some(Cow::Owned(bytes.data.to_vec()))
                    } else {
                        warn!("failed to load font '{path_str}'");
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}
