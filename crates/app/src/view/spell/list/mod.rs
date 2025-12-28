use iced::{Element, Task, widget};
use tracing::debug;
use types::FormMode;

use crate::{
    message::Message,
    tab::{TabAction, TabContent, TabId, TabRequest},
};

mod message;

pub use message::SpellListMessage;

pub struct SpellList {
    id: TabId,
}

impl SpellList {
    pub fn new(id: TabId) -> Self {
        Self { id }
    }
}

impl TabContent for SpellList {
    type ContentMessage = SpellListMessage;

    fn id(&self) -> crate::tab::TabId {
        self.id
    }

    fn title(&self) -> &str {
        "Spell List"
    }

    fn update(&mut self, message: Self::ContentMessage) -> Task<Message> {
        debug!("updating spell list with: {:?}", message);

        match message {
            SpellListMessage::CreateNewSpell => {
                let request = TabRequest::SpellForm {
                    mode: FormMode::Create,
                };

                Task::done(Message::TabAction(TabAction::Open(request)))
            }
        }
    }

    fn view(&self) -> Element<'_, Self::ContentMessage> {
        let create_spell_button = widget::button("Create New Spell")
            .on_press(SpellListMessage::CreateNewSpell)
            .style(style::button::ghost_danger_no_edges);

        widget::container(create_spell_button).center_x(1200).into()
    }
}
