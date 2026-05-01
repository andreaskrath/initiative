mod context;
mod message;
mod session;
mod status;
mod view;

use crate::message::LoadMessage;
use crate::message::Message;
use crate::session::Session;
use crate::status::Status;
use storage::clients::local::Local;
use style::container::ContainerClass;
use style::theme::Theme;
use style::theme::variant::ThemeVariant;
use widgets::Element;

use iced::Length::Fill;
use iced::Task;
use iced::alignment::Horizontal;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::rule;
use iced::widget::space;
use iced::widget::stack;
use std::path::PathBuf;

pub struct Application {
    status: Status<Loader, Session>,
}

struct Loader {}

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
            status: Status::Loading(Box::new(Loader {})),
        };

        (app, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match (&mut self.status, message) {
            (Status::Loading(loader), Message::Load(load_message)) => match load_message {
                LoadMessage::DatabaseConnected(Ok(pool)) => {
                    let repository = Local::new(pool);
                    let session = Session::new(repository);
                    self.status = Status::Ready(Box::new(session));

                    Task::none()
                }
                LoadMessage::DatabaseConnected(Err(err)) => {
                    panic!("{err:?}");
                }
            },
            (Status::Ready(session), Message::OpenView(request)) => session.open_view(request),
            (Status::Ready(session), Message::CloseView(view_id)) => session.close_view(view_id),
            (Status::Ready(session), Message::FocusView(view_id)) => session.focus_view(view_id),
            (Status::Ready(session), Message::ViewUpdated(view_id, view_message)) => {
                session.update_view(view_id, view_message)
            }
            (_invalid_state, invalid_message) => {
                tracing::error!("invalid message {invalid_message:?} for state");

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.status {
            Status::Loading(loader) => components::text::heading("Loading").into(),
            Status::Ready(session) => {
                let topbar = row![space::horizontal().width(Fill)].padding(5);

                let view = session.active_view();
                let containered_view = container(view)
                    .class(ContainerClass::Background)
                    .align_x(Horizontal::Center)
                    .width(Fill)
                    .height(Fill);

                let topbar = column![topbar, rule::horizontal(1)];

                let main = stack![containered_view];

                column![topbar, main].into()
            }
        }
    }

    pub fn theme(&self) -> Option<Theme> {
        if let Status::Ready(session) = &self.status {
            Some(session.context().theme())
        } else {
            Some(ThemeVariant::default().into())
        }
    }
}
