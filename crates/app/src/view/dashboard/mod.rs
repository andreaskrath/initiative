mod message;

use crate::message::Message;
use crate::view::ViewContent;
pub use message::DashboardMessage;
use widgets::Element;

use iced::Task;

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
        widgets::text::view_title("Dashboard").into()
    }
}
