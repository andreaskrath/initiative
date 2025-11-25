use std::fmt::Debug;

use iced::{Element, Task};

use crate::{Message, tab::TabId};

/// This trait defines a common interface for all `TabContent` elements of the application.
///
/// A `TabContent` should be understood as a view, that can be displayed as part of a tab.
pub trait TabContent {
    /// The message type that is native to `Self`.
    type ContentMessage: Debug + Clone;

    /// Returns the [`TabId`] of `Self`.
    fn id(&self) -> TabId;

    /// Defines the title of `Self`, this will be utilized for the tab title, recent items
    /// name, and potentially the name of the window itself.
    fn title(&self) -> &str;

    /// Updates the internal state of `Self` via its native message type.
    ///
    /// Might optionally spawn a task with a Message that is emitted to the root application.
    fn update(&mut self, message: Self::ContentMessage) -> Task<Message>;

    /// Generate the view of `Self`.
    fn view(&self) -> Element<'_, Self::ContentMessage>;
}
