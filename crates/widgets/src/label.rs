use crate::Element;
use crate::icon::Icon;
use crate::icon::IconName;
use crate::text;
use style::container::ContainerClass;
use style::svg::SvgClass;
use style::text::TextClass;

use iced::Alignment;
use iced::Length::Fill;
use iced::widget;
use iced::widget::row;
use iced::widget::tooltip::Position;
use std::time::Duration;

pub struct Label<'a> {
    label: &'a str,
    required: bool,
    error: Option<&'a str>,
}

impl<'a> Label<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            required: false,
            error: None,
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn error(mut self, error: Option<&'a str>) -> Self {
        self.error = error;
        self
    }
}

impl<'a, Message> From<Label<'a>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(label: Label<'a>) -> Element<'a, Message> {
        let text = text::label(label.label);
        let mut row = row![text].align_y(Alignment::Center);

        if label.required {
            let asterisk = widget::text("*")
                .size(18)
                .font(fonts::display::regular())
                .class(TextClass::Danger);

            row = row.push(asterisk);
        }

        if let Some(error) = label.error {
            let space = widget::space::horizontal().width(Fill);
            let icon = Icon::new(IconName::Error).class(SvgClass::Danger);
            let tooltip_text = widget::text(error).size(18).font(fonts::display::regular());
            let tooltip = widget::tooltip(icon, tooltip_text, Position::Top)
                .gap(5)
                .delay(Duration::from_millis(500))
                .class(ContainerClass::Error);

            row = row.push(space);
            row = row.push(tooltip);
        }

        row.into()
    }
}
