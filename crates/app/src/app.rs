mod navigation;

use components::{Icon, IconSize, icon};
use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{button, column, horizontal_rule, horizontal_space, row, svg, text},
};
use types::{Message, NavigationMessage};

use crate::navigation::Navigation;

pub struct Initiative {
    navigation: Navigation,
}

impl Default for Initiative {
    fn default() -> Self {
        Self {
            navigation: Navigation::default(),
        }
    }
}

impl Initiative {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigation(navigation_message) => self.navigation.update(navigation_message),
            Message::OpenTab(view) => {
                println!("{:?}", view);

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let icon = if self.navigation.collapsed() {
            icon(Icon::NavigationOpen, IconSize::Large)
        } else {
            icon(Icon::NavigationClose, IconSize::Large)
        };
        let topbar = row![
            button(icon)
                .style(button::text)
                .on_press(Message::Navigation(NavigationMessage::ToggleCollapse)),
            horizontal_space().width(Fill)
        ]
        .padding(5);

        let navigation = self.navigation.view().map(Message::Navigation);
        let main = text("main_view").width(Fill);
        let container = row![navigation, main];

        column![topbar, horizontal_rule(0), container].into()
    }
}
