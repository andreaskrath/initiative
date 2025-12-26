use iced::{Theme, widget::text::Style};

pub fn muted(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        color: Some(palette.text.scale_alpha(0.6)),
    }
}
