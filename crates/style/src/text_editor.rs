use crate::{DEFAULT_BORDER, NO_BORDER_ROUNDED, disabled, hovered, pressed, theme::Theme};

use iced::{
    Background, Border,
    widget::text_editor::{Catalog, Status, Style},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TextEditorClass {
    Normal,
    Error,
}

impl Catalog for Theme {
    type Class<'a> = TextEditorClass;

    fn default<'a>() -> Self::Class<'a> {
        TextEditorClass::Normal
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        let mut background = self.interaction;

        match status {
            Status::Active => {}
            Status::Hovered => {
                background = hovered(background);
            }
            Status::Focused { is_hovered: _ } => {
                background = pressed(background);
            }
            Status::Disabled => {
                background = disabled(background);
            }
        }

        let border = match class {
            TextEditorClass::Normal => NO_BORDER_ROUNDED,
            TextEditorClass::Error => Border {
                color: self.danger,
                ..DEFAULT_BORDER
            },
        };

        Style {
            background: Background::Color(background),
            border,
            placeholder: self.text_dimmed,
            value: self.text,
            selection: self.primary,
        }
    }
}
