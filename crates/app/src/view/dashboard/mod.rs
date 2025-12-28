use iced::{Task, widget};
use style::Typography;

use crate::{
    message::Message,
    tab::{TabContent, TabId},
};

mod message;

pub use message::DashboardMessage;

pub struct Dashboard {
    id: TabId,
}

impl Dashboard {
    pub fn new(id: TabId) -> Self {
        Self { id }
    }
}

impl TabContent for Dashboard {
    type ContentMessage = DashboardMessage;

    fn id(&self) -> TabId {
        self.id
    }

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
