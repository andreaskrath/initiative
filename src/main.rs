use iced::{
    widget::{button, text},
    Element,
};
use message::Message;
use screen::Screen;

mod condition;
mod message;
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
}

impl Default for CombatTracker {
    fn default() -> Self {
        Self {
            screen: Default::default(),
        }
    }
}

impl CombatTracker {
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
