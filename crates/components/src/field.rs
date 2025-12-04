use std::marker::PhantomData;

use iced::{
    Color, Element,
    Length::Fill,
    widget::{column, row, space, text, tooltip, tooltip::Position},
};

use crate::{Icon, IconName};

pub struct Field<'a, Input, Message, Label> {
    label: Label,
    input: Input,
    required: bool,
    error: Option<&'a str>,
    _marker: PhantomData<&'a Message>,
}

impl<'a, Input, Message, Label> Field<'a, Input, Message, Label> {
    pub fn new(label: Label, input: Input) -> Self {
        Self {
            label,
            input,
            required: false,
            error: None,
            _marker: PhantomData,
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

impl<'a, Input, Message, Label> From<Field<'a, Input, Message, Label>> for Element<'a, Message>
where
    Input: Into<Element<'a, Message>>,
    Label: Into<&'a str>,
{
    fn from(field: Field<'a, Input, Message, Label>) -> Self {
        let required: Element<'a, Message> = if field.required {
            text("*").color(Color::from_rgb(1.0, 0.0, 0.0)).into()
        } else {
            space::horizontal().into()
        };

        let mut label = row![
            text(field.label.into()),
            required,
            space::horizontal().width(Fill)
        ];

        if let Some(err) = field.error {
            let icon = Icon::new(IconName::Error).color(Color::from_rgb(1.0, 0.0, 0.0));

            let tooltip = tooltip(icon, text(err), Position::Bottom);

            label = label.push(tooltip);
        }

        let input = field.input.into();

        column![label, input].spacing(5).into()
    }
}
