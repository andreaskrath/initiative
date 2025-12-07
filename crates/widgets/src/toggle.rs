use iced::{Element, widget};

pub struct Toggle<Message> {
    label: String,
    value: bool,
    on_toggle: Option<Message>,
}

impl<Message> Toggle<Message> {
    pub fn new(label: impl Into<String>, value: bool) -> Self {
        Self {
            label: label.into(),
            value,
            on_toggle: None,
        }
    }

    pub fn on_toggle(mut self, on_toggle: Option<Message>) -> Self {
        self.on_toggle = on_toggle;
        self
    }

    pub fn value(&self) -> bool {
        self.value
    }

    pub fn toggle(&mut self) {
        self.value = !self.value;
    }
}

impl<'a, Message> Toggle<Message>
where
    Message: Clone + 'a,
{
    pub fn view(&'a self) -> Element<'a, Message> {
        let label = widget::text(&self.label);

        let style = if self.value {
            widget::button::primary
        } else {
            widget::button::subtle
            // widget::button::secondary
        };

        widget::button(label)
            .style(style)
            .on_press_maybe(self.on_toggle.clone())
            .into()
    }
}
