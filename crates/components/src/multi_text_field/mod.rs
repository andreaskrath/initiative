mod rules;
mod state;

pub use rules::*;
pub use state::*;

use crate::label::Label;
use style::layout::LABEL_SPACING;
use style::text_input::TextInputClass;
use widgets::Element;

use iced::widget::Column;

pub fn multi_text_field<'a, Message>(
    label: Option<&'a str>,
    state: &'a MultiTextFieldState,
) -> MultiTextField<'a, Message> {
    MultiTextField::new(label, state)
}

pub struct MultiTextField<'a, Message> {
    state: &'a MultiTextFieldState,
    label: Option<&'a str>,
    placeholder: Option<&'a str>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    on_remove: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> MultiTextField<'a, Message> {
    pub fn new(label: Option<&'a str>, state: &'a MultiTextFieldState) -> Self {
        Self {
            state,
            label,
            placeholder: None,
            on_input: None,
            on_submit: None,
            on_remove: None,
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

    pub fn on_submit(mut self, on_submit: Message) -> Self {
        self.on_submit = Some(on_submit);
        self
    }

    pub fn on_remove(mut self, on_remove: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_remove = Some(Box::new(on_remove));
        self
    }
}

impl<'a, Message> From<MultiTextField<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(widget: MultiTextField<'a, Message>) -> Self {
        let label = widget.label.map(|label_text| {
            Label::new(label_text)
                .required(widget.state.is_required())
                .error(widget.state.error())
        });

        let placeholder = widget.placeholder.unwrap_or("");

        let mut input =
            widgets::multi_text_input(placeholder, widget.state.value(), widget.state.selections());

        if let Some(on_input) = widget.on_input {
            if widget.state.normalize {
                input = input.on_input(move |s| on_input(s.to_lowercase()));
            } else {
                input = input.on_input(on_input);
            }
        }

        if let Some(on_submit) = widget.on_submit {
            input = input.on_submit(on_submit);
        }

        if let Some(on_remove) = widget.on_remove {
            input = input.on_remove(on_remove);
        }

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
