pub mod dashboard;
pub mod request;
pub mod spell;

use widgets::Element;

use iced::Task;
use std::fmt::Debug;

/// A common interface for all views of the application.
pub trait View {
    /// The message type that is native to `Self`.
    type Message: Debug + Clone;

    /// A separate message type that allows upward communication.
    type Effect: Debug + Clone;

    /// Defines the title of `Self`, this will be utilized for the tab title, recent items
    /// name, and potentially the name of the window itself.
    fn title(&self) -> &str;

    /// Updates the internal state of `Self` via its native message type.
    ///
    /// Can launch tasks for further internal communication, or optionally emit an `Effect` for the
    /// caller to manage and either handle, or further emit up the chain until a caller can.
    fn update(&mut self, message: Self::Message) -> (Task<Self::Message>, Option<Self::Effect>);

    /// Generate the view of `Self`.
    fn view(&self) -> Element<'_, Self::Message>;
}

enum State<Loader, Data> {
    Loading(Box<Loader>),
    Active(Box<Data>),
}
