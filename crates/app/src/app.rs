mod message;
mod navigation;
mod tab;
mod view;

use iced::{
    Element,
    Length::Fill,
    Subscription, Task, Theme,
    alignment::Horizontal,
    widget::{button, column, container, row, rule, space, stack},
};
use style::ThemeVariant;
use tracing::info;
use widgets::{Icon, IconName, IconSize};

use crate::{
    message::Message,
    navigation::{Navigation, NavigationMessage},
    tab::TabManager,
};

pub struct Initiative {
    theme: Theme,
    navigation: Navigation,
    tabs: TabManager,
}

impl Default for Initiative {
    fn default() -> Self {
        Self {
            theme: ThemeVariant::default().into(),
            navigation: Navigation::default(),
            tabs: TabManager::default(),
        }
    }
}

impl Initiative {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigation(navigation_message) => self.navigation.update(navigation_message),
            Message::TabRequest(request) => {
                info!("requesting opening of tab: {:?}", request);

                self.tabs.handle_request(request)
            }
            Message::TabMessage(tab_message) => self.tabs.update(tab_message),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let palette = self.theme.palette();

        let icon = if self.navigation.collapsed() {
            Icon::new(IconName::NavigationOpen).size(IconSize::Large)
        } else {
            Icon::new(IconName::NavigationClose).size(IconSize::Large)
        }
        .color(palette.text);

        let topbar = row![
            button(icon)
                .style(button::text)
                .on_press(Message::Navigation(NavigationMessage::ToggleCollapse)),
            space::horizontal().width(Fill)
        ]
        .padding(5);

        let navigation = self.navigation.view().map(Message::Navigation);

        let current_view = container(self.tabs.view().map(Message::TabMessage))
            .align_x(Horizontal::Center)
            .width(Fill)
            .height(Fill);

        let topbar = column![topbar, rule::horizontal(1)];

        let main = stack![current_view, navigation];

        column![topbar, main].into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.navigation.subscription().map(Message::Navigation)
    }

    pub fn theme(&self) -> Option<Theme> {
        Some(self.theme.clone())
    }
}
