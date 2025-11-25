use std::fmt::Debug;

use components::Icon;
use iced::{Element, Task};

use crate::Message;

/// This trait defines a common interface for all `TabContent` elements of the application.
///
/// A `TabContent` should be understood as a view, that can be displayed as part of a tab.
pub trait TabContent {
    /// The message type that is native to this `TabContent`.
    type ContentMessage: Debug + Clone;

    /// Defines the title of the `TabContent`, this will be utilized for the tab title, recent items
    /// name, and potentially the name of the window itself.
    fn title(&self) -> &str;

    fn tab_icon(&self) -> &Icon;

    /// Updates the internal state of `TabContent` via its native message type.
    ///
    /// Might optionally spawn a task with a Message that is emitted to the root application.
    fn update(&mut self, message: Self::ContentMessage) -> Task<Message>;

    /// Generate the view of `TabContent`.
    fn view(&self) -> Element<'_, Self::ContentMessage>;
}
