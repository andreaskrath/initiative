use iced::{Element, Length, widget};

pub fn toggle<'a, Message>(label: &'a str, value: bool) -> Toggle<'a, Message> {
    Toggle::new(label, value)
}

pub struct Toggle<'a, Message> {
    label: &'a str,
    value: bool,
    width: Length,
    on_toggle: Option<Message>,
}

impl<'a, Message> Toggle<'a, Message> {
    pub fn new(label: &'a str, value: bool) -> Self {
        Self {
            label,
            value,
            width: Length::Shrink,
            on_toggle: None,
        }
    }

    pub fn on_toggle(mut self, on_toggle: Message) -> Self {
        self.on_toggle = Some(on_toggle);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message> From<Toggle<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(widget: Toggle<'a, Message>) -> Self {
        let label = crate::text::display(widget.label);

        let style = if widget.value {
            style::button::background::default
        } else {
            style::button::background::ghost::default
        };

        widget::button(label)
            .width(widget.width)
            .style(style)
            .on_press_maybe(widget.on_toggle)
            .into()
    }
}
