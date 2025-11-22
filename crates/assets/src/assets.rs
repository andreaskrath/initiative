use iced::widget::svg::Handle;
use rust_embed::RustEmbed;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AssetsError {
    #[error("failed to load asset '{0}'")]
    Load(String),
}

#[derive(RustEmbed, Default)]
#[folder = "../../assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl Assets {
    pub fn icon(&self, path: &str) -> Result<Handle, AssetsError> {
        let path = format!("icons/{path}");
        let Some(bytes) = Self::get(&path) else {
            error!("failed to find icon with path '{}'", path);
            return Err(AssetsError::Load(path));
        };

        Ok(Handle::from_memory(bytes.data))
    }
}
