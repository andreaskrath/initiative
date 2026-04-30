mod state;

pub use state::*;

use crate::label::Label;
use style::layout::INPUT_PADDING;
use widgets::Element;

use iced::Length;
use iced::widget;
use iced::widget::text::Shaping;
use style::pick_list::PickListClass;

pub fn select_field<'a, Value, Message>(
    label: &'a str,
    state: &'a SelectFieldState<Value>,
    on_select: impl Fn(Value) -> Message + 'a,
) -> SelectField<'a, Value, Message> {
    SelectField::new(label, state, on_select)
}

pub struct SelectField<'a, Value, Message> {
    state: &'a SelectFieldState<Value>,
    label: &'a str,
    on_select: Box<dyn Fn(Value) -> Message + 'a>,
    placeholder: Option<&'a str>,
    width: Length,
}

impl<'a, Value, Message> SelectField<'a, Value, Message> {
    pub fn new(
        label: &'a str,
        state: &'a SelectFieldState<Value>,
        on_select: impl Fn(Value) -> Message + 'a,
    ) -> Self {
        Self {
            label,
            state,
            on_select: Box::new(on_select),
            placeholder: None,
            width: Length::Fill,
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

impl<'a, Value, Message> From<SelectField<'a, Value, Message>> for Element<'a, Message>
where
    Value: PartialEq + Clone + ToString,
    Message: Clone + 'a,
{
    fn from(widget: SelectField<'a, Value, Message>) -> Self {
        let label = Label::new(widget.label)
            .required(widget.state.is_required())
            .error(widget.state.error());

        let placeholder = widget.placeholder.unwrap_or("");

        let mut select = widget::pick_list(
            widget.state.options(),
            widget.state.selected(),
            widget.on_select,
        )
        .font(fonts::display::regular())
        .text_size(fonts::display::DEFAULT_DISPLAY_TEXT_SIZE)
        .text_shaping(Shaping::Advanced)
        .placeholder(placeholder)
        .padding(INPUT_PADDING)
        .width(widget.width);

        if widget.state.error().is_some() {
            select = select.class(PickListClass::Error);
        }

        widget::column![label, select].spacing(5).into()
    }
}
