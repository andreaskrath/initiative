pub mod dashboard;
pub mod request;
pub mod spell;

use crate::view::dashboard::Dashboard;
use crate::view::spell::form::SpellForm;
use crate::view::spell::list::SpellList;
use dashboard::message::Message as DashboardMessage;
use spell::form::message::Message as SpellFormMessage;
use spell::list::message::Message as SpellListMessage;
use widgets::Element;

use iced::Task;
use std::fmt::Debug;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

#[derive(Debug, Clone)]
pub enum ViewMessage {
    Dashboard(DashboardMessage),
    SpellForm(SpellFormMessage),
    SpellList(SpellListMessage),
}

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ViewId(u64);

impl ViewId {
    /// Generate and return a new unique `ViewId`.
    pub fn unique() -> ViewId {
        ViewId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

/// A common interface for all views of the application.
pub trait Viewable {
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

/// The views of the application.
pub enum View {
    Dashboard(Dashboard),
    SpellForm(Box<SpellForm>),
    SpellList(Box<SpellList>),
}

impl View {
    pub fn title(&self) -> &str {
        match self {
            View::Dashboard(dashboard) => dashboard.title(),
            View::SpellForm(spell_form) => spell_form.title(),
            View::SpellList(spell_list) => spell_list.title(),
        }
    }
}
