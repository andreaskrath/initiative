mod message;

pub use message::SpellListMessage;

use crate::message::Message;
use crate::view::ViewContent;
use crate::view::ViewRequest;
use types::FormMode;
use widgets::Element;

use iced::Task;
use iced::widget;
use iced::widget::column;
use tracing::debug;

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
        let title = components::text::view_title("Spell List");

        let create_spell_button =
            widget::button("Create New Spell").on_press(SpellListMessage::CreateNewSpell);

        column![title, create_spell_button].into()
    }
}
