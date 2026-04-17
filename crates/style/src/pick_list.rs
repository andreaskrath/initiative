use crate::NO_BORDER_ROUNDED;
use crate::hovered;
use crate::theme::Theme;

use iced::Background;
use iced::Border;
use iced::Color;
use iced::widget::pick_list::Catalog;
use iced::widget::pick_list::Status;
use iced::widget::pick_list::Style;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PickListClass {
    Normal,
    Error,
}

impl Catalog for Theme {
    type Class<'a> = PickListClass;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        PickListClass::Normal
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        let mut background = self.interaction;
        let mut border_color = match class {
            PickListClass::Normal => None,
            PickListClass::Error => Some(self.danger),
        };

        match status {
            Status::Active => {}
            Status::Hovered => {
                background = hovered(background);
                border_color = border_color.map(hovered);
            }
            Status::Opened { is_hovered } => {
                if is_hovered {
                    background = hovered(background);
                }
            }
        }

        Style {
            text_color: self.text,
            placeholder_color: self.text_dimmed,
            handle_color: self.primary,
            background: Background::Color(background),
            border: Border {
                color: border_color.unwrap_or(Color::TRANSPARENT),
                ..NO_BORDER_ROUNDED
            },
        }
    }
}
