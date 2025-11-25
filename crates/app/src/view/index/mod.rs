use iced::{Task, widget::text};

use crate::{
    message::Message,
    tab::{TabContent, TabId},
};

mod message;

pub use message::IndexMessage;

pub struct Index {
    id: TabId,
}

impl Index {
    pub fn new(id: TabId) -> Self {
        Self { id }
    }
}

impl TabContent for Index {
    type ContentMessage = IndexMessage;

    fn id(&self) -> TabId {
        self.id
    }

    fn title(&self) -> &str {
        "Index"
    }

    fn update(&mut self, message: Self::ContentMessage) -> Task<Message> {
        match message {}

        Task::none()
    }

    fn view(&self) -> iced::Element<'_, Self::ContentMessage> {
        text("Index").into()
    }
}
