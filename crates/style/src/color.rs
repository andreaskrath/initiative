use crate::MUTED_SCALE;
use iced::{
    Color,
    theme::{Palette, palette::Extended},
};

pub mod background {
    use super::*;

    pub fn default(extended: &Extended) -> Color {
        extended.background.base.color
    }

    pub fn hover(extended: &Extended) -> Color {
        extended.background.weak.color
    }

    pub fn active(extended: &Extended) -> Color {
        extended.background.strong.color
    }

    pub fn danger(extended: &Extended) -> Color {
        extended.danger.base.color
    }

    pub fn disabled(extended: &Extended) -> Color {
        extended.background.weakest.color
    }
}

pub mod border {
    use super::*;

    pub fn default(extended: &Extended) -> Color {
        extended.primary.base.color
    }

    pub fn hover(extended: &Extended) -> Color {
        extended.primary.weak.color
    }

    pub fn focus(extended: &Extended) -> Color {
        extended.primary.strong.color
    }

    pub fn error(extended: &Extended) -> Color {
        extended.danger.base.color
    }

    pub fn disabled(extended: &Extended) -> Color {
        extended.primary.weak.color.scale_alpha(MUTED_SCALE)
    }
}

pub mod text {
    use super::*;

    pub fn default(palette: &Palette) -> Color {
        palette.text
    }

    pub fn muted(palette: &Palette) -> Color {
        palette.text.scale_alpha(MUTED_SCALE)
    }

    pub fn error(palette: &Palette) -> Color {
        palette.danger
    }

    pub fn disabled(palette: &Palette) -> Color {
        palette.text.scale_alpha(MUTED_SCALE)
    }
}
