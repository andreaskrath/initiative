use iced::{Theme, widget::text::Style};

use crate::MUTED_SCALE;

pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        color: Some(palette.text),
    }
}

pub fn muted(theme: &Theme) -> Style {
    let mut style = default(theme);
    style.color = style.color.map(|color| color.scale_alpha(MUTED_SCALE));

    style
}

pub mod primary {
    use super::*;

    pub fn default(theme: &Theme) -> Style {
        let extended = theme.extended_palette();

        Style {
            color: Some(extended.primary.base.color),
        }
    }

    pub fn weak(theme: &Theme) -> Style {
        let extended = theme.extended_palette();

        Style {
            color: Some(extended.primary.weak.color),
        }
    }

    pub fn strong(theme: &Theme) -> Style {
        let extended = theme.extended_palette();

        Style {
            color: Some(extended.primary.strong.color),
        }
    }

    pub fn muted(theme: &Theme) -> Style {
        let mut style = default(theme);
        style.color = style.color.map(|color| color.scale_alpha(MUTED_SCALE));

        style
    }
}

pub mod danger {
    use super::*;

    pub fn default(theme: &Theme) -> Style {
        let extended = theme.extended_palette();

        Style {
            color: Some(extended.danger.base.color),
        }
    }

    pub fn weak(theme: &Theme) -> Style {
        let extended = theme.extended_palette();

        Style {
            color: Some(extended.danger.weak.color),
        }
    }

    pub fn strong(theme: &Theme) -> Style {
        let extended = theme.extended_palette();

        Style {
            color: Some(extended.danger.strong.color),
        }
    }

    pub fn muted(theme: &Theme) -> Style {
        let mut style = default(theme);
        style.color = style.color.map(|color| color.scale_alpha(MUTED_SCALE));

        style
    }
}
