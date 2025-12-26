use iced::{Theme, widget::svg::Style};

pub fn default(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().text),
    }
}

pub fn muted(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().text.scale_alpha(0.6)),
    }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().primary),
    }
}
