mod message;

use crate::message::Message;
use crate::view::ViewContent;
pub use message::DashboardMessage;
use style::container::ContainerClass;
use widgets::Element;

use iced::Task;

pub struct Dashboard {
    value: String,
    values: Vec<String>,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            values: vec![String::from("Value 1"), String::from("Value 2")],
        }
    }
}

impl ViewContent for Dashboard {
    type ContentMessage = DashboardMessage;

    fn title(&self) -> &str {
        "Dashboard"
    }

    fn update(&mut self, message: Self::ContentMessage) -> Task<Message> {
        match message {
            DashboardMessage::ValueChanged(value) => self.value = value,
            DashboardMessage::ValueRemoved(index) => {
                self.values.remove(index);
            }
            DashboardMessage::ValueAdded => {
                // Add if not empty, and not a duplicate
                if !self.value.is_empty() && !self.values.contains(&self.value) {
                    let value = std::mem::take(&mut self.value);
                    self.values.push(value);
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::ContentMessage> {
        let example = widgets::multi_text_input::MultiTextInput::new(
            "Write a tag",
            &self.value,
            &self.values,
        )
        .on_input(DashboardMessage::ValueChanged)
        .on_submit(DashboardMessage::ValueAdded)
        .on_remove(DashboardMessage::ValueRemoved);

        iced::widget::container(example)
            .class(ContainerClass::Surface)
            .into()
    }
}
