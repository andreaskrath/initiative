use iced::{
    widget::{button, column, row, text, text_input},
    Element, Task,
};

use crate::{enemy::Enemy, message::Message};

pub struct CreateNewEnemy {
    name: String,
    initiative_mod: String,
    hp: String,
}

#[derive(Debug, Clone)]
pub enum Action {
    Name(String),
    InitiativeMod(String),
    HitPoints(String),
    Submit,
}

impl CreateNewEnemy {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            initiative_mod: String::new(),
            hp: String::new(),
        }
    }

    pub fn update(&mut self, msg: Action) -> Task<Message> {
        match msg {
            Action::Name(s) => {
                self.name = s;

                Task::none()
            }
            Action::InitiativeMod(s) => {
                self.initiative_mod = s;

                Task::none()
            }
            Action::HitPoints(s) => {
                self.hp = s;

                Task::none()
            }
            Action::Submit => Task::perform(submit_new_enemy(), Message::SubmitNewEnemy),
        }
    }

    pub fn view(&self) -> Element<Action> {
        let name_label = text("Name:");
        let name_input = text_input("", &self.name).on_input(Action::Name);

        let initiative_label = text("Initiative Modifier:");
        let initiative_input = text_input("", &self.initiative_mod).on_input(Action::InitiativeMod);

        let hp_label = text("Hit Points:");
        let hp_input = text_input("", &self.hp).on_input(Action::HitPoints);

        let submit = button("Submit").on_press(Action::Submit);

        column![
            row![name_label, name_input],
            row![initiative_label, initiative_input],
            row![hp_label, hp_input],
            row![submit],
        ]
        .into()
    }
}

async fn submit_new_enemy() -> Enemy {
    let name = String::new();
    let initiative = 0;
    let max_hp = 0;
    Enemy::new(name, initiative, max_hp)
}
