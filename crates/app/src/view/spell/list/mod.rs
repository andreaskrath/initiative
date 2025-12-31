use iced::{Element, Task, widget, widget::column};
use tracing::debug;
use types::FormMode;

use crate::{
    message::Message,
    view::{ViewContent, ViewRequest},
};

mod message;

pub use message::SpellListMessage;

pub struct SpellList {}

impl SpellList {
    pub fn new() -> Self {
        Self {}
    }
}

impl ViewContent for SpellList {
    type ContentMessage = SpellListMessage;

    fn title(&self) -> &str {
        "Spell List"
    }

    fn update(&mut self, message: Self::ContentMessage) -> Task<Message> {
        debug!("updating spell list with: {:?}", message);

        match message {
            SpellListMessage::CreateNewSpell => {
                let request = ViewRequest::SpellForm {
                    mode: FormMode::Create,
                };

                Task::done(Message::Navigate(request))
            }
        }
    }

    fn view(&self) -> Element<'_, Self::ContentMessage> {
        let title = widgets::view_title("Spell List");

        let create_spell_button = widget::button("Create New Spell")
            .on_press(SpellListMessage::CreateNewSpell)
            .style(style::button::danger::ghost::default);

        column![title, create_spell_button].into()
    }
}
