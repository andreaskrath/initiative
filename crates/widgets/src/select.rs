use iced::{
    Color, Element,
    Length::{self, Fill},
    widget::{self, tooltip::Position},
};

use crate::{Icon, IconName};

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
}

impl<'a, Value, Message> Select<Value, Message>
where
    Value: PartialEq + Copy + Clone + ToString,
    Message: Clone + 'a,
{
    pub fn view(&'a self) -> Element<'a, Message> {
        let required: Element<'a, Message> = if self.required {
            widget::text("*")
                .color(Color::from_rgb(1.0, 0.0, 0.0))
                .into()
        } else {
            widget::space::horizontal().into()
        };

        let mut label = iced::widget::row![
            widget::text(&self.label),
            required,
            widget::space::horizontal().width(Fill)
        ];

        if let Some(err) = self.error() {
            let icon = Icon::new(IconName::Error).color(Color::from_rgb(1.0, 0.0, 0.0));

            let tooltip = widget::tooltip(icon, widget::text(err), Position::Bottom);

            label = label.push(tooltip);
        }

        let options = self.options.as_ref();
        let selected = self.selected;
        let on_select = self.on_select.as_ref();
        let placeholder = self.placeholder.as_ref().map(|p| p.as_ref()).unwrap_or("");

        let select = widget::pick_list(options, selected, on_select)
            .placeholder(placeholder)
            .width(self.width);

        widget::column![label, select].spacing(5).into()
    }
}
