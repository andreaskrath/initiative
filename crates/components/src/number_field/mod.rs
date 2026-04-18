mod rules;
mod state;

pub use rules::*;
pub use state::*;

use crate::label::Label;
use style::layout::INPUT_PADDING;
use style::layout::LABEL_SPACING;
use style::text_input::TextInputClass;
use widgets::Element;

use iced::Alignment;
use iced::Length;
use iced::widget;
use iced::widget::Column;

pub fn number_field<'a, Message>(
    label: Option<&'a str>,
    state: &'a NumberFieldState,
) -> NumberField<'a, Message> {
    NumberField::new(label, state)
}

pub struct NumberField<'a, Message> {
    state: &'a NumberFieldState,
    label: Option<&'a str>,
    placeholder: Option<&'a str>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    width: Length,
}

impl<'a, Message> NumberField<'a, Message> {
    pub fn new(label: Option<&'a str>, state: &'a NumberFieldState) -> Self {
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

impl<'a, Message> From<NumberField<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(widget: NumberField<'a, Message>) -> Self {
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
