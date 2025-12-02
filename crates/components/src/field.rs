use std::marker::PhantomData;

use iced::{
    Color, Element,
    Length::Fill,
    widget::{column, row, space, text, tooltip, tooltip::Position},
};

use crate::{Icon, IconName};

pub struct Field<'a, T, M>
where
    T: Into<Element<'a, M>>,
{
    label: &'a str,
    input: T,
    required: bool,
    error: Option<&'a str>,
    _marker: PhantomData<&'a M>,
}

impl<'a, T, M> Field<'a, T, M>
where
    T: Into<Element<'a, M>>,
{
    pub fn new(label: &'a str, input: T) -> Self {
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

    pub fn error(mut self, error: &'a str) -> Self {
        self.error = Some(error);
        self
    }
}

impl<'a, T, M> From<Field<'a, T, M>> for Element<'a, M>
where
    T: Into<Element<'a, M>>,
{
    fn from(field: Field<'a, T, M>) -> Self {
        let icon = Icon::new(IconName::Error).color(Color::from_rgb(1.0, 0.0, 0.0));

        let potential_tooltip = field
            .error
            .map(|err| tooltip(icon, text(err), Position::Bottom));

        let required: Element<'a, M> = if field.required {
            text("*").color(Color::from_rgb(1.0, 0.0, 0.0)).into()
        } else {
            space::horizontal().into()
        };

        let mut label = row![text(field.label), required, space::horizontal().width(Fill)];
        if let Some(tooltip) = potential_tooltip {
            label = label.push(tooltip);
        }

        let input = field.input.into();

        column![label, input].spacing(5).into()
    }
}
