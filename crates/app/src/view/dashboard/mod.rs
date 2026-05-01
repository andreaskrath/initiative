pub mod message;

use crate::view::Viewable;
use crate::view::request::Request;
use message::Effect;
use message::Message;
use types::FormMode;
use widgets::Element;

use iced::Task;
use iced::widget;
use iced::widget::row;

pub struct Dashboard;

impl Dashboard {
    pub fn new() -> Self {
        Self
    }
}

impl Viewable for Dashboard {
    type Message = Message;

    type Effect = Effect;

    fn title(&self) -> &str {
        "Dashboard"
    }

    fn update(&mut self, message: Self::Message) -> (Task<Self::Message>, Option<Self::Effect>) {
        match message {
            Message::OpenSpells => {
                let request = Request::SpellList;
                let effect = Effect::OpenView(request);

                (Task::none(), Some(effect))
            }
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
        let open_new_spell = widget::button("new spell").on_press(Message::OpenNewSpell);

        let open_spells = widget::button("spells").on_press(Message::OpenSpells);

        row![open_new_spell, open_spells].into()
    }
}
