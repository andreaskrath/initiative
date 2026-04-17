mod rules;
mod state;

use crate::{
    Element,
    form::{INPUT_PADDING, LABEL_SPACING},
    label::Label,
};
pub use rules::*;
pub use state::*;

use iced::{
    Alignment, Length,
    widget::{self, Column},
};
use style::text_input::TextInputClass;

pub fn number_input<'a, Message>(
    label: Option<&'a str>,
    state: &'a NumberInputState,
) -> NumberInput<'a, Message> {
    NumberInput::new(label, state)
}

pub struct NumberInput<'a, Message> {
    state: &'a NumberInputState,
    label: Option<&'a str>,
    placeholder: Option<&'a str>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    width: Length,
}

impl<'a, Message> NumberInput<'a, Message> {
    pub fn new(label: Option<&'a str>, state: &'a NumberInputState) -> Self {
        Self {
            state,
            label,
            placeholder: None,
            on_input: None,
            width: Length::Fill,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message> From<NumberInput<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(widget: NumberInput<'a, Message>) -> Self {
        let label = widget.label.map(|label_text| {
            Label::new(label_text)
                .required(widget.state.is_required())
                .error(widget.state.error())
        });

        let placeholder = widget.placeholder.unwrap_or("");

        let mut input = widget::text_input(placeholder, widget.state.raw_value())
            .font(fonts::display::regular())
            .size(fonts::display::DEFAULT_DISPLAY_TEXT_SIZE)
            .align_x(Alignment::Center)
            .padding(INPUT_PADDING)
            .on_input_maybe(widget.on_input);

        if widget.state.error().is_some() {
            input = input.class(TextInputClass::Error);
        }

        let mut column = Column::with_capacity(2)
            .spacing(LABEL_SPACING)
            .width(widget.width);

        if let Some(label_element) = label {
            column = column.push(label_element);
        }

        column.push(input).into()
    }
}
