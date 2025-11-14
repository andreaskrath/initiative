use std::borrow::Cow;

use gpui::{App, AssetSource, Result, SharedString};
use rust_embed::RustEmbed;
use thiserror::Error;
use tracing::debug;

#[derive(Error, Debug)]
pub enum AssetsError {
    #[error("failed to load asset '{0}'")]
    Load(String),
}

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or(AssetsError::Load(path.to_string()).into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|a| {
                if a.starts_with(path) {
                    Some(a.into())
                } else {
                    None
                }
            })
            .collect())
    }
}

impl Assets {
    /// Add all `.ttf` fonts in the `assets/fonts` directory to the provided `App`.
    pub fn load_fonts(&self, cx: &App) -> Result<()> {
        debug!("loading fonts");
        let font_paths = self.list("fonts")?;
        let mut embedded_fonts = Vec::new();

        for font_path in font_paths {
            if font_path.ends_with(".ttf") {
                let font_bytes = cx
                    .asset_source()
                    .load(&font_path)?
                    .expect("asset should never return None");
                embedded_fonts.push(font_bytes);
            }
        }

        cx.text_system().add_fonts(embedded_fonts)
    }
}
