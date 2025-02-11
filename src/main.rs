use entity::EntityTable;
use iced::{
    widget::{button, text},
    Element,
};
use message::Message;
use screen::Screen;

mod condition;
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

struct CombatTracker<'a> {
    screen: Screen,
    entities: EntityTable<'a>,
}

impl<'a> Default for CombatTracker<'a> {
    fn default() -> Self {
        Self {
            screen: Default::default(),
            entities: Vec::new(),
        }
    }
}

impl<'a> CombatTracker<'a> {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::NewEncounter => self.screen = Screen::Encounter,
        }
    }

    fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::Encounter => text("no encounter data").into(),
            Screen::NoEncounter => button("New Encounter")
                .on_press(Message::NewEncounter)
                .into(),
        }
    }
}
