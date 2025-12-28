use iced::{Task, widget};
use style::Typography;

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

    fn view(&self) -> iced::Element<'_, Self::ContentMessage> {
        widget::text("Dashboard")
            .font(Typography::heading_bold())
            .size(24)
            .into()
    }
}
