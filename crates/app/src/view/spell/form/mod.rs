mod message;

use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{container, text},
};
use types::{FormMode, Spell};

use crate::{
    message::Message,
    tab::{TabContent, TabId},
};

pub use message::SpellFormMessage;

pub struct SpellForm {
    id: TabId,
    mode: FormMode,
    spell: Spell,
}

impl SpellForm {
    pub fn new(id: TabId, mode: FormMode) -> Self {
        Self {
            id,
            mode,
            spell: Spell::default(),
        }
    }
}

impl TabContent for SpellForm {
    type ContentMessage = SpellFormMessage;

    fn id(&self) -> crate::tab::TabId {
        self.id
    }

    fn title(&self) -> &str {
        match self.mode {
            FormMode::Create => "Create spell",
            FormMode::Edit(_) => "Edit spell",
        }
    }

    fn update(&mut self, message: Self::ContentMessage) -> Task<Message> {
        match message {
            SpellFormMessage::NameChanged(name) => self.spell.name = name,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::ContentMessage> {
        container(text("Spell form"))
            .width(Fill)
            .center_x(Fill)
            .into()
    }
}
