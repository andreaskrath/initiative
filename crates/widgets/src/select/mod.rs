use crate::Label;
use iced::{
    Element,
    Length::{self},
    widget::{self},
};

mod state;
mod style;

pub use state::*;

pub fn select<'a, Value, Message>(
    label: &'a str,
    state: &'a SelectState<Value>,
    on_select: impl Fn(Value) -> Message + 'a,
) -> Select<'a, Value, Message> {
    Select::new(label, state, on_select)
}

pub struct Select<'a, Value, Message> {
    state: &'a SelectState<Value>,
    label: &'a str,
    on_select: Box<dyn Fn(Value) -> Message + 'a>,
    placeholder: Option<&'a str>,
    width: Length,
}

impl<'a, Value, Message> Select<'a, Value, Message> {
    pub fn new(
        label: &'a str,
        state: &'a SelectState<Value>,
        on_select: impl Fn(Value) -> Message + 'a,
    ) -> Self {
        Self {
            label,
            state,
            on_select: Box::new(on_select),
            placeholder: None,
            width: Length::Shrink,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Value, Message> From<Select<'a, Value, Message>> for Element<'a, Message>
where
    Value: PartialEq + Copy + Clone + ToString,
    Message: Clone + 'a,
{
    fn from(widget: Select<'a, Value, Message>) -> Self {
        let label = Label::new(widget.label)
            .required(widget.state.is_required())
            .error(widget.state.error());

        let placeholder = widget.placeholder.unwrap_or("");
        let style = if widget.state.error().is_some() {
            style::error
        } else {
            style::default
        };

        let select = widget::pick_list(
            widget.state.options(),
            widget.state.selected(),
            widget.on_select,
        )
        .font(fonts::display::regular())
        .text_size(fonts::display::DEFAULT_DISPLAY_TEXT_SIZE)
        .placeholder(placeholder)
        .style(style)
        .menu_style(style::menu::default)
        .width(widget.width);

        widget::column![label, select].spacing(5).into()
    }
}
