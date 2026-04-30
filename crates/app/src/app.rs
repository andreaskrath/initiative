mod message;
mod navigation;
mod state;
mod tab;
mod view;

use crate::message::LoadMessage;
use crate::message::Message;
use crate::navigation::Navigation;
use crate::navigation::message::NavigationEffect;
use crate::navigation::message::NavigationMessage;
use crate::state::State;
use crate::tab::TabManager;
use crate::tab::TabManagerEffect;
use crate::tab::TabManagerMessage;
use components::icon::IconName;
use components::icon::IconSize;
use storage::clients::local::Local;
use storage::repositories::Repository;
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
use std::path::PathBuf;
use std::sync::Arc;

pub struct Application {
    theme: Theme,
    state: State<Loader, Initiative>,
}

struct Loader {}

struct Initiative {
    navigation: Navigation,
    tab_manager: TabManager,
    repository: Arc<dyn Repository>,
}

impl Initiative {
    fn new(repository: Arc<dyn Repository>) -> Self {
        Self {
            navigation: Navigation::default(),
            tab_manager: TabManager::default(),
            repository,
        }
    }
}

fn db_path() -> Option<PathBuf> {
    let dir = dirs::data_dir()?.join("initiative");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join("initiative.db"))
}

impl Application {
    pub fn new() -> (Self, Task<Message>) {
        let db_path = db_path().expect("failed");
        let messages = vec![Task::perform(
            storage::clients::local::connect(db_path),
            LoadMessage::DatabaseConnected,
        )];

        let task = Task::batch(messages).map(Message::Load);

        let app = Self {
            theme: ThemeVariant::default().into(),
            state: State::Loading(Box::new(Loader {})),
        };

        (app, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match (&mut self.state, message) {
            (State::Loading(loader), Message::Load(load_message)) => match load_message {
                LoadMessage::DatabaseConnected(Ok(pool)) => {
                    let repository = Arc::new(Local::new(pool));
                    let initiative = Initiative::new(repository);
                    self.state = State::Active(Box::new(initiative));

                    Task::none()
                }
                LoadMessage::DatabaseConnected(Err(err)) => {
                    panic!("{err:?}");
                }
            },
            (State::Active(initiative), Message::Navigation(navigation_message)) => {
                let (navigation_task, maybe_effect) =
                    initiative.navigation.update(navigation_message);

                let mut tasks = Vec::with_capacity(2);
                tasks.push(navigation_task.map(Message::Navigation));

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
            (State::Active(initiative), Message::TabManager(tab_manager_message)) => {
                let (tab_manager_task, maybe_effect) = initiative
                    .tab_manager
                    .update(tab_manager_message, &initiative.repository);

                let mut tasks = Vec::with_capacity(2);
                tasks.push(tab_manager_task.map(Message::TabManager));

                if let Some(tab_manager_effect) = maybe_effect {
                    match tab_manager_effect {
                        TabManagerEffect::LoadFailed(error) => {
                            panic!("{error:?}");
                        }
                    }
                }

                Task::batch(tasks)
            }
            (_invalid_state, invalid_message) => {
                tracing::error!("invalid message {invalid_message:?} for state");

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.state {
            State::Loading(loader) => components::text::heading("Loading").into(),
            State::Active(initiative) => {
                let icon = if initiative.navigation.collapsed() {
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

                let navigation = initiative.navigation.view().map(Message::Navigation);

                let tab_manager_view = initiative.tab_manager.view().map(Message::TabManager);
                let current_view = container(tab_manager_view)
                    .class(ContainerClass::Background)
                    .align_x(Horizontal::Center)
                    .width(Fill)
                    .height(Fill);

                let topbar = column![topbar, rule::horizontal(1)];

                let main = stack![current_view, navigation];

                column![topbar, main].into()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if let State::Active(state) = &self.state {
            state.navigation.subscription().map(Message::Navigation)
        } else {
            Subscription::none()
        }
    }

    pub fn theme(&self) -> Option<Theme> {
        Some(self.theme.clone())
    }
}
