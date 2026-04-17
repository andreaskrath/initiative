use crate::{DEFAULT_BORDER, NO_BORDER_ROUNDED, disabled, hovered, pressed, theme::Theme};

use iced::{
    Background, Border,
    widget::text_input::{Catalog, Status, Style},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TextInputClass {
    Normal,
    Error,
}

impl Catalog for Theme {
    type Class<'a> = TextInputClass;

    fn default<'a>() -> Self::Class<'a> {
        TextInputClass::Normal
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        let mut background = self.interaction;
        let mut border_color = match class {
            TextInputClass::Normal => None,
            TextInputClass::Error => Some(self.danger),
        };

        match status {
            Status::Active => {}
            Status::Hovered => {
                background = hovered(background);
                border_color = border_color.map(hovered);
            }
            Status::Focused { is_hovered: _ } => {
                background = pressed(background);
            }
            Status::Disabled => {
                background = disabled(background);
                border_color = border_color.map(disabled);
            }
        }

        let border = match border_color {
            Some(color) => Border {
                color,
                ..DEFAULT_BORDER
            },
            None => NO_BORDER_ROUNDED,
        };

        Style {
            background: Background::Color(background),
            border,
            icon: self.primary,
            placeholder: self.text_dimmed,
            value: self.text,
            selection: self.primary,
        }
    }
}
