pub mod message;

use crate::view::content::ViewContent;
use message::DashboardEffect;
use message::DashboardMessage;
use widgets::Element;

use iced::Task;

pub struct Dashboard;

impl Dashboard {
    pub fn new() -> Self {
        Self
    }
}

impl ViewContent for Dashboard {
    type Message = DashboardMessage;

    type Effect = DashboardEffect;

    fn title(&self) -> &str {
        "Dashboard"
    }

    fn update(&mut self, message: Self::Message) -> (Task<Self::Message>, Option<Self::Effect>) {
        (Task::none(), None)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        components::text::heading("Dashboard").into()
    }
}
