use crate::Label;
use iced::{Element, widget};

mod rules;
mod state;
mod style;

pub use rules::*;
pub use state::*;

pub fn text_input<'a, Message>(
    label: &'a str,
    state: &'a TextInputState,
) -> TextInput<'a, Message> {
    TextInput::new(label, state)
}

pub struct TextInput<'a, Message> {
    state: &'a TextInputState,
    label: &'a str,
    placeholder: Option<&'a str>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

impl<'a, Message> TextInput<'a, Message> {
    pub fn new(label: &'a str, state: &'a TextInputState) -> Self {
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

impl<'a, Message> From<TextInput<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(widget: TextInput<'a, Message>) -> Self {
        let label = Label::new(widget.label)
            .required(widget.state.is_required())
            .error(widget.state.error());

        let placeholder = widget.placeholder.unwrap_or("");

        let style = if widget.state.error().is_some() {
            style::error
        } else {
            style::default
        };

        let input = widget::text_input(placeholder, widget.state.value())
            .font(fonts::display::regular())
            .size(fonts::display::DEFAULT_DISPLAY_TEXT_SIZE)
            .style(style)
            .on_input_maybe(widget.on_input);

        widget::column![label, input].spacing(5).into()
    }
}
