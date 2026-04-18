mod rules;
mod state;

pub use rules::*;
pub use state::*;

use crate::label::Label;
use style::layout::INPUT_PADDING;
use style::layout::LABEL_SPACING;
use style::text_input::TextInputClass;
use widgets::Element;

use iced::widget;
use iced::widget::Column;

pub fn text_field<'a, Message>(
    label: Option<&'a str>,
    state: &'a TextFieldState,
) -> TextField<'a, Message> {
    TextField::new(label, state)
}

pub struct TextField<'a, Message> {
    state: &'a TextFieldState,
    label: Option<&'a str>,
    placeholder: Option<&'a str>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

impl<'a, Message> TextField<'a, Message> {
    pub fn new(label: Option<&'a str>, state: &'a TextFieldState) -> Self {
        Self {
            state,
            label,
            placeholder: None,
            on_input: None,
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
}

impl<'a, Message> From<TextField<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(widget: TextField<'a, Message>) -> Self {
        let label = widget.label.map(|label_text| {
            Label::new(label_text)
                .required(widget.state.is_required())
                .error(widget.state.error())
        });

        let placeholder = widget.placeholder.unwrap_or("");

        let mut input = widget::text_input(placeholder, widget.state.value())
            .font(fonts::display::regular())
            .size(fonts::display::DEFAULT_DISPLAY_TEXT_SIZE)
            .padding(INPUT_PADDING)
            .on_input_maybe(widget.on_input);

        if widget.state.error().is_some() {
            input = input.class(TextInputClass::Error);
        }

        let mut column = Column::with_capacity(2).spacing(LABEL_SPACING);

        if let Some(label_element) = label {
            column = column.push(label_element);
        }

        column.push(input).into()
    }
}
