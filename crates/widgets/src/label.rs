use crate::{Icon, IconName};
use iced::{
    Element,
    Length::Fill,
    widget::{self, row, tooltip::Position},
};
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
        let text = widget::text(label.label)
            .size(18)
            .font(fonts::display::regular())
            .style(style::text::default);

        let mut row = row![text];

        if label.required {
            let asterisk = widget::text("*")
                .size(18)
                .font(fonts::display::regular())
                .style(style::text::danger::default);

            row = row.push(asterisk);
        }

        if let Some(error) = label.error {
            let space = widget::space::horizontal().width(Fill);
            let icon = Icon::new(IconName::Error).style(style::icon::danger);
            let tooltip_text = widget::text(error).size(18).font(fonts::display::regular());
            let tooltip = widget::tooltip(icon, tooltip_text, Position::Top)
                .gap(5)
                .delay(Duration::from_millis(500))
                .style(style::tooltip::danger);

            row = row.push(space);
            row = row.push(tooltip);
        }

        row.into()
    }
}
