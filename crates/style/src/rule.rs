use crate::NO_BORDER;
use crate::theme::Theme;

use iced::widget::rule::Catalog;
use iced::widget::rule::FillMode;
use iced::widget::rule::Style;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RuleClass {
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
    type Class<'a> = RuleClass;

    fn default<'a>() -> Self::Class<'a> {
        RuleClass::Normal
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        let color = match class {
            RuleClass::Normal => self.text,
            RuleClass::Dimmed => self.text_dimmed,
            RuleClass::Primary => self.primary,
            RuleClass::Secondary => self.secondary,
            RuleClass::Tertiary => self.tertiary,
            RuleClass::Info => self.info,
            RuleClass::Success => self.success,
            RuleClass::Warning => self.warning,
            RuleClass::Danger => self.danger,
        };

        Style {
            color,
            radius: NO_BORDER.radius,
            fill_mode: FillMode::Full,
            snap: true,
        }
    }
}
