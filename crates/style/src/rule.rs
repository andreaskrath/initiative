use iced::{
    Theme,
    widget::rule::{FillMode, Style},
};

pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();

    Style {
        color: palette.text,
        radius: 0.0.into(),
        snap: true,
        fill_mode: FillMode::Full,
    }
}
