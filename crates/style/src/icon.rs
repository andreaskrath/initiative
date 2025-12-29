use iced::{Theme, widget::svg::Style};

use crate::MUTED_SCALE;

pub fn default(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().text),
    }
}

pub fn muted(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().text.scale_alpha(MUTED_SCALE)),
    }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().primary),
    }
}

pub fn danger(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().danger),
    }
}
