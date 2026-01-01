use crate::{Label, ValidationError};
use iced::{
    Element,
    Length::{self},
    widget::{self},
};

pub struct Select<Value, Message> {
    label: String,
    selected: Option<Value>,
    options: Box<[Value]>,
    on_select: Box<dyn Fn(Value) -> Message>,
    required: bool,
    error: Option<String>,
    placeholder: Option<String>,
    width: Length,
}

impl<Value, Message> Select<Value, Message> {
    pub fn new(
        label: impl Into<String>,
        value: Option<Value>,
        options: impl IntoIterator<Item = Value>,
        on_select: impl Fn(Value) -> Message + 'static,
    ) -> Self {
        let collected_options = options.into_iter().collect();

        Self {
            label: label.into(),
            selected: value,
            options: collected_options,
            on_select: Box::new(on_select),
            required: false,
            error: None,
            placeholder: None,
            width: Length::Shrink,
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn select(&mut self, selected: Option<Value>) {
        self.selected = selected;
    }

    /// Get the error of `Self`.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn validate(&mut self) -> bool {
        if !self.required || self.selected.is_some() {
            true
        } else {
            self.error = Some(ValidationError::Required.to_string());
            false
        }
    }
}

impl<'a, Value, Message> Select<Value, Message>
where
    Value: PartialEq + Copy + Clone + ToString,
    Message: Clone + 'a,
{
    pub fn view(&'a self) -> Element<'a, Message> {
        let label = Label::new(&self.label)
            .required(self.required)
            .error(self.error());

        let options = self.options.as_ref();
        let selected = self.selected;
        let on_select = self.on_select.as_ref();
        let placeholder = self.placeholder.as_ref().map(|p| p.as_ref()).unwrap_or("");

        let mut select = widget::pick_list(options, selected, on_select)
            .placeholder(placeholder)
            .font(fonts::display::regular())
            .menu_style(style::menu::default)
            .width(self.width);

        if self.error().is_some() {
            select = select.style(style::select::error);
        } else {
            select = select.style(style::select::default);
        }

        widget::column![label, select].spacing(5).into()
    }
}
