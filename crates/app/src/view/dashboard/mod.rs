use crate::{message::Message, view::ViewContent};
use iced::{Element, Task};

mod message;

pub use message::DashboardMessage;

pub struct Dashboard {}

impl Dashboard {
    pub fn new() -> Self {
        Self {}
    }
}

impl ViewContent for Dashboard {
    type ContentMessage = DashboardMessage;

    fn title(&self) -> &str {
        "Dashboard"
    }

    fn update(&mut self, _message: Self::ContentMessage) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::ContentMessage> {
        widgets::view_title("Dashboard").into()
    }
}
