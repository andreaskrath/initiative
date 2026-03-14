use iced::{
    Background, Border, Theme,
    widget::pick_list::{Status, Style},
};
use style::{DEFAULT_BORDER, MUTED_SCALE, NO_BORDER, NO_SHADOW, color};

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let background = match status {
        Status::Active => Background::Color(color::background::default(extended)),
        Status::Hovered => Background::Color(color::background::hover(extended)),
        Status::Opened { is_hovered: _ } => Background::Color(color::background::active(extended)),
    };

    let border_color = match status {
        Status::Active => color::border::default(extended),
        Status::Hovered => color::border::hover(extended),
        Status::Opened { is_hovered: _ } => color::border::focus(extended),
    };

    let border = Border {
        color: border_color,
        ..DEFAULT_BORDER
    };

    Style {
        text_color: palette.text,
        placeholder_color: palette.text.scale_alpha(MUTED_SCALE),
        handle_color: extended.background.strong.color,
        background,
        border,
    }
}

pub fn error(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let border_color = match status {
        Status::Active => palette.text,
        Status::Hovered => extended.danger.base.color,
        Status::Opened { is_hovered: _ } => extended.danger.strong.color,
    };

    let border = Border {
        color: border_color,
        ..DEFAULT_BORDER
    };
    Style {
        border,
        ..default(theme, status)
    }
}

pub mod menu {
    use super::*;
    use iced::overlay::menu::Style;

    pub fn default(theme: &Theme) -> Style {
        let palette = theme.palette();
        let extended = theme.extended_palette();

        let background = Background::Color(color::background::default(extended));

        let border = NO_BORDER;
        let selected_background = Background::Color(color::background::hover(extended));

        Style {
            background,
            border,
            text_color: palette.text,
            selected_text_color: palette.text,
            selected_background,
            shadow: NO_SHADOW,
        }
    }
}
