use crate::theme::Theme;

use iced::widget::text::Catalog;
use iced::widget::text::Style;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TextClass {
    Normal,
    Dimmed,
    Primary,
    Secondary,
    Tertiary,
    Info,
    Success,
    Warning,
    Danger,
}

impl Catalog for Theme {
    type Class<'a> = TextClass;

    fn default<'a>() -> Self::Class<'a> {
        TextClass::Normal
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        let color = match class {
            TextClass::Normal => self.text,
            TextClass::Dimmed => self.text_dimmed,
            TextClass::Primary => self.primary,
            TextClass::Secondary => self.secondary,
            TextClass::Tertiary => self.tertiary,
            TextClass::Info => self.info,
            TextClass::Success => self.success,
            TextClass::Warning => self.warning,
            TextClass::Danger => self.danger,
        };

        Style { color: Some(color) }
    }
}
