mod message;
mod navigation;
mod tab;
mod view;

use crate::message::Message;
use crate::navigation::Navigation;
use crate::navigation::message::NavigationEffect;
use crate::navigation::message::NavigationMessage;
use crate::tab::TabManager;
use crate::tab::TabManagerMessage;
use components::icon::IconName;
use components::icon::IconSize;
use style::button::ButtonClass;
use style::container::ContainerClass;
use style::svg::SvgClass;
use style::theme::Theme;
use style::theme::variant::ThemeVariant;
use widgets::Element;

use iced::Length::Fill;
use iced::Subscription;
use iced::Task;
use iced::alignment::Horizontal;
use iced::widget::button;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::rule;
use iced::widget::space;
use iced::widget::stack;

pub struct Initiative {
    theme: Theme,
    navigation: Navigation,
    tab_manager: TabManager,
}

impl Default for Initiative {
    fn default() -> Self {
        Self {
            theme: ThemeVariant::default().into(),
            navigation: Navigation::default(),
            tab_manager: TabManager::default(),
        }
    }
}

impl Initiative {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigation(navigation_message) => {
                let (navigation_task, maybe_effect) = self.navigation.update(navigation_message);

                let mut tasks = Vec::with_capacity(2);
                tasks.push(navigation_task.map(|message| Message::Navigation(message)));

                if let Some(navigation_effect) = maybe_effect {
                    match navigation_effect {
                        NavigationEffect::Navigate(view_request) => {
                            let effect =
                                Message::TabManager(TabManagerMessage::OpenTab(view_request));
                            tasks.push(Task::done(effect));
                        }
                    }
                }

                Task::batch(tasks)
            }
            Message::TabManager(tab_manager_message) => {
                let (tab_manager_task, maybe_effect) = self.tab_manager.update(tab_manager_message);

                let mut tasks = Vec::with_capacity(2);
                tasks.push(tab_manager_task.map(|message| Message::TabManager(message)));

                if let Some(tab_manager_effect) = maybe_effect {
                    match tab_manager_effect {}
                }

                Task::batch(tasks)
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let icon = if self.navigation.collapsed() {
            components::icon(IconName::NavigationOpen).size(IconSize::Large)
        } else {
            components::icon(IconName::NavigationClose).size(IconSize::Large)
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

        let tab_manager_view = self.tab_manager.view().map(Message::TabManager);
        let current_view = container(tab_manager_view)
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
