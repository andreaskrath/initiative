use crate::{NO_BORDER_ROUNDED, disabled, hovered, theme::Theme};

use iced::{
    Background, Border,
    widget::checkbox::{Catalog, Status, Style},
};

impl Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>, status: Status) -> Style {
        let (background, text) = match status {
            Status::Active { is_checked } => {
                if is_checked {
                    (self.primary, self.text)
                } else {
                    (self.interaction, self.text)
                }
            }
            Status::Hovered { is_checked } => {
                if is_checked {
                    (hovered(self.primary), hovered(self.text))
                } else {
                    (hovered(self.interaction), hovered(self.text))
                }
            }
            Status::Disabled { is_checked } => {
                if is_checked {
                    (disabled(self.primary), disabled(self.text))
                } else {
                    (disabled(self.interaction), disabled(self.text))
                }
            }
        };

        Style {
            background: Background::Color(background),
            icon_color: self.secondary,
            border: Border {
                color: self.primary,
                ..NO_BORDER_ROUNDED
            },
            text_color: Some(text),
        }
    }
}
