use create_new_enemy::CreateNewEnemy;
use entity::{Entity, EntityTable};
use iced::{
    widget::{button, text},
    Element, Task,
};
use message::Message;
use screen::Screen;

mod condition;
mod create_new_enemy;
mod damage;
mod enemy;
mod entity;
mod message;
mod player;
mod screen;

fn main() {
    iced::application(
        "Dungeons and Dragons - Combat Tracker",
        CombatTracker::update,
        CombatTracker::view,
    )
    .run()
    .expect("could not start combat tracker");
}

struct CombatTracker {
    screen: Screen,
    entities: EntityTable,
}

impl Default for CombatTracker {
    fn default() -> Self {
        Self {
            screen: Default::default(),
            entities: Vec::new(),
        }
    }
}

impl CombatTracker {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::NewEncounter => {
                self.screen = Screen::Encounter;

                Task::none()
            }
            Message::CreateNewEnemy => {
                self.screen = Screen::CreateNewEnemy(CreateNewEnemy::new());

                Task::none()
            }
            Message::UpdateNewEnemy(message) => {
                if let Screen::CreateNewEnemy(screen) = &mut self.screen {
                    screen.update(message)
                } else {
                    Task::none()
                }
            }
            Message::SubmitNewEnemy(enemy) => {
                self.entities.push(Entity::Enemy(enemy));
                self.screen = Screen::NoEncounter;

                Task::none()
            }
            Message::CreateNewPlayer => {
                self.screen = Screen::CreateNewPlayer;

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::Encounter => text("no encounter data").into(),
            Screen::NoEncounter => button("New Enemy").on_press(Message::CreateNewEnemy).into(),
            Screen::CreateNewEnemy(create_new_enemy) => {
                create_new_enemy.view().map(Message::UpdateNewEnemy)
            }
            Screen::CreateNewPlayer => todo!(),
        }
    }
}
