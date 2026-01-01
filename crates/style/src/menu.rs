use iced::{Background, Theme, overlay::menu::Style};

use crate::{NO_BORDER, NO_SHADOW};

pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let background = Background::Color(extended.background.weak.color);

    let border = NO_BORDER;
    let selected_background = Background::Color(extended.background.strong.color);

    Style {
        background,
        border,
        text_color: palette.text,
        selected_text_color: palette.text,
        selected_background,
        shadow: NO_SHADOW,
    }
}
