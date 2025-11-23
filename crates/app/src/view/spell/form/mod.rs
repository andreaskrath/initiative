mod message;

use iced::{Element, Task, widget::text};
use types::{FormMode, Spell};

pub use message::SpellFormMessage;

use crate::message::Message;

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

    pub fn update(&mut self) -> Task<Message> {
        Task::none()
    }

    pub fn view(&mut self) -> Element<'_, SpellFormMessage> {
        text("Spell form").into()
    }
}
