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

/// Unwraps a [`Status`] as [`Status::Ready`], returning a mutable reference to the inner state.
///
/// If the status is not [`Status::Ready`], logs an error and early-returns
/// `Task::none()` from the enclosing function.
macro_rules! ready {
    ($self:expr) => {
        match $self.as_ready_mut() {
            Some(state) => state,
            None => {
                tracing::error!("received in non-ready state");

                return Task::none();
            }
        }
    };
}

/// Unwraps a [`Status`] as [`Status::Loading`], returning a mutable reference to the inner loader.
///
/// If the status is not [`Status::Loading`], logs an error and early-returns
/// `Task::none()` from the enclosing function.
macro_rules! loading {
    ($self:expr) => {
        match $self.as_loading_mut() {
            Some(loader) => loader,
            None => {
                tracing::error!("received in non-loader state");

                return Task::none();
            }
        }
    };
}

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
        let tasks = vec![Task::perform(
            Local::new(db_path),
            LoadMessage::LocalConnected,
        )];

        let task = Task::batch(tasks).map(Message::Load);

        let app = Self {
            status: Status::Loading(Box::new(Loader {})),
        };

        (app, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Load(load_message) => {
                let _ = loading!(self.status);

                match load_message {
                    LoadMessage::LocalConnected(Ok(local)) => {
                        let session = Session::new(local);
                        self.status = Status::Ready(Box::new(session));

                        Task::none()
                    }
                    LoadMessage::LocalConnected(Err(err)) => {
                        panic!("{err:?}");
                    }
                }
            }
            Message::OpenView(request) => {
                let session = ready!(self.status);

                session.open_view(request)
            }
            Message::CloseView(view_id) => {
                let session = ready!(self.status);

                session.close_view(view_id)
            }
            Message::FocusView(view_id) => {
                let session = ready!(self.status);

                session.focus_view(view_id)
            }
            Message::ViewUpdated(view_id, view_message) => {
                let session = ready!(self.status);

                session.update_view(view_id, view_message)
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
