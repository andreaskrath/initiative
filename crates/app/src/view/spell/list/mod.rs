pub mod message;

use crate::view::Viewable;
use crate::view::request::Request;
use crate::view::spell::list::message::Effect;
use crate::view::spell::list::message::Message;
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

impl Viewable for SpellList {
    type Message = Message;

    type Effect = Effect;

    fn title(&self) -> &str {
        "Spell List"
    }

    fn update(&mut self, message: Self::Message) -> (Task<Self::Message>, Option<Self::Effect>) {
        match message {
            Message::OpenNewSpell => {
                let request = Request::SpellForm {
                    mode: FormMode::Create,
                };
                let effect = Effect::OpenView(request);

                (Task::none(), Some(effect))
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = components::text::view_title("Spell List");

        let create_spell_button =
            widget::button("Create New Spell").on_press(Message::OpenNewSpell);

        column![title, create_spell_button].into()
    }
}
