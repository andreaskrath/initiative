use crate::NO_BORDER_ROUNDED;
use crate::hovered;
use crate::theme::Theme;

use iced::Background;
use iced::widget::radio::Catalog;
use iced::widget::radio::Status;
use iced::widget::radio::Style;

impl Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>, status: Status) -> Style {
        let (background, text) = match status {
            Status::Active { is_selected } => {
                if is_selected {
                    (self.primary, self.text)
                } else {
                    (self.interaction, self.text)
                }
            }
            Status::Hovered { is_selected } => {
                if is_selected {
                    (hovered(self.primary), hovered(self.text))
                } else {
                    (hovered(self.interaction), hovered(self.text))
                }
            }
        };

        Style {
            background: Background::Color(background),
            dot_color: background,
            border_width: NO_BORDER_ROUNDED.width,
            border_color: background,
            text_color: Some(text),
        }
    }
}
