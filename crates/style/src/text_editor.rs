use crate::DEFAULT_BORDER;
use crate::NO_BORDER_ROUNDED;
use crate::disabled;
use crate::hovered;
use crate::pressed;
use crate::theme::Theme;

use iced::Background;
use iced::Border;
use iced::widget::text_editor::Catalog;
use iced::widget::text_editor::Status;
use iced::widget::text_editor::Style;

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
