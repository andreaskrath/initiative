use iced::{Element, Task};

use crate::Message;

/// This trait defines a common interface for all `Content` elements of the application.
///
/// A `Content` should be understood as a view, that can be displayed as part of a tab.
pub trait Content {
    /// The message type that is native to this `Content`.
    type ContentMessage;

    /// Defines the title of the `Content`, this will be utilized for the tab title, recent items
    /// name, and potentially the name of the window itself.
    fn title(&self) -> &str;

    /// Updates the internal state of `Content` via its native message type.
    ///
    /// Might optionally spawn a task with a Message that is emitted to the root application.
    fn update(&mut self, message: Self::ContentMessage) -> Task<Message>;

    /// Generate the view of `Content`.
    fn view(&self) -> Element<'_, Self::ContentMessage>;
}
