mod message;

use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{container, text},
};
use types::{FormMode, Spell};

use crate::{message::Message, tab::TabContent};

pub use message::SpellFormMessage;

pub struct SpellForm {
    mode: FormMode,
    spell: Spell,
}

impl SpellForm {
    pub fn new(mode: FormMode) -> Self {
        Self {
            mode,
            spell: Spell::default(),
        }
    }
}

impl TabContent for SpellForm {
    type ContentMessage = SpellFormMessage;

    fn title(&self) -> &str {
        match self.mode {
            FormMode::Create => "Create spell",
            FormMode::Edit(_) => "Edit spell",
        }
    }

    fn tab_icon(&self) -> &components::Icon {
        todo!()
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
