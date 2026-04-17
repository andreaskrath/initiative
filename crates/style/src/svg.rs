use crate::hovered;
use crate::theme::Theme;

use iced::widget::svg::Catalog;
use iced::widget::svg::Status;
use iced::widget::svg::Style;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SvgClass {
    /// Applies no color styling, meaning it inherits from outer elements.
    Normal,

    /// Applies normal text color styling, wtih hover indication.
    Text,

    /// Applies primary color styling, with hover indication.
    Primary,

    /// Applies danger color styling, with hover indication.
    Danger,
}

impl Catalog for Theme {
    type Class<'a> = SvgClass;

    fn default<'a>() -> Self::Class<'a> {
        SvgClass::Normal
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        let color = match (class, status) {
            (SvgClass::Normal, _) => None,
            (SvgClass::Text, Status::Idle) => Some(self.text),
            (SvgClass::Text, Status::Hovered) => Some(hovered(self.text)),
            (SvgClass::Primary, Status::Idle) => Some(self.primary),
            (SvgClass::Primary, Status::Hovered) => Some(hovered(self.primary)),
            (SvgClass::Danger, Status::Idle) => Some(self.danger),
            (SvgClass::Danger, Status::Hovered) => Some(hovered(self.danger)),
        };

        Style { color }
    }
}
