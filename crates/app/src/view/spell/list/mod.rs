pub mod message;

use crate::view::content::ViewContent;
use crate::view::spell::list::message::SpellListEffect;
use crate::view::spell::list::message::SpellListMessage;
use types::FormMode;
use widgets::Element;

use iced::Task;
use iced::widget;
use iced::widget::column;

pub struct SpellList {}

impl SpellList {
    pub fn new() -> Self {
        Self {}
    }
}

impl ViewContent for SpellList {
    type Message = SpellListMessage;

    type Effect = SpellListEffect;

    fn title(&self) -> &str {
        "Spell List"
    }

    fn update(&mut self, message: Self::Message) -> (Task<Self::Message>, Option<Self::Effect>) {
        match message {
            SpellListMessage::CreateNewSpell => {
                let effect = Some(SpellListEffect::OpenSpellForm {
                    mode: FormMode::Create,
                });

                (Task::none(), effect)
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = components::text::view_title("Spell List");

        let create_spell_button =
            widget::button("Create New Spell").on_press(SpellListMessage::CreateNewSpell);

        column![title, create_spell_button].into()
    }
}
