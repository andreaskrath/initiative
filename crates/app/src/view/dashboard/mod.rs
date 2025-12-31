use iced::{Element, Task};
use widgets::heading;

use crate::{message::Message, view::ViewContent};

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
        heading("Dashboard").into()
    }
}
