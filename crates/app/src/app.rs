mod message;
mod navigation;
mod tab;
mod view;

use crate::{
    message::Message,
    navigation::{Navigation, NavigationMessage},
    tab::{TabAction, TabManager},
};
use style::{
    button::ButtonClass,
    container::ContainerClass,
    svg::SvgClass,
    theme::{Theme, variant::ThemeVariant},
};
use widgets::{Element, icon::IconName, icon::IconSize};

use iced::{
    Length::Fill,
    Subscription, Task,
    alignment::Horizontal,
    widget::{button, column, container, row, rule, space, stack},
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
            Message::Tab(tab_id, tab_message) => self.tabs.update(tab_id, tab_message),
            Message::TabAction(tab_action) => self.tabs.perform(tab_action),
            Message::Navigate(view_request) => self.tabs.perform(TabAction::Open(view_request)),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let icon = if self.navigation.collapsed() {
            widgets::icon(IconName::NavigationOpen).size(IconSize::Large)
        } else {
            widgets::icon(IconName::NavigationClose).size(IconSize::Large)
        }
        .class(SvgClass::Text);

        let topbar = row![
            button(icon)
                .class(ButtonClass::Ghost)
                .on_press(Message::Navigation(NavigationMessage::ToggleCollapse)),
            space::horizontal().width(Fill),
        ]
        .padding(5);

        let navigation = self.navigation.view().map(Message::Navigation);

        let current_view = container(self.tabs.view())
            .class(ContainerClass::Background)
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
