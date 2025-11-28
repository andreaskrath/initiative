mod message;
mod navigation;
mod tab;
mod view;

use components::{Icon, IconSize, icon};
use iced::{
    Element,
    Length::Fill,
    Subscription,
    Task,
    widget::{button, column, horizontal_rule, horizontal_space, row},
};
use tracing::info;

use crate::{
    message::Message,
    navigation::{Navigation, NavigationMessage},
    tab::TabManager,
};

pub struct Initiative {
    navigation: Navigation,
    tabs: TabManager,
}

impl Default for Initiative {
    fn default() -> Self {
        Self {
            navigation: Navigation::default(),
            tabs: TabManager::default(),
        }
    }
}

impl Initiative {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigation(navigation_message) => self.navigation.update(navigation_message),
            Message::TabRequest(tab_request) => {
                info!("opening tab for {:?}", tab_request);

                Task::none()
            }
            Message::TabMessage(tab_message) => self.tabs.update(tab_message),
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
        let main = self.tabs.view().map(Message::TabMessage);
        let container = row![navigation, main];

        column![topbar, horizontal_rule(0), container].into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.navigation
            .subscription()
            .map(Message::Navigation)
    }
}
