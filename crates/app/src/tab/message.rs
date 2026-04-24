use crate::tab::TabId;
use crate::view::dashboard::message::DashboardMessage;
use crate::view::request::ViewRequest;
use crate::view::spell::form::message::SpellFormMessage;
use crate::view::spell::list::message::SpellListMessage;

#[derive(Debug, Clone)]
pub enum TabManagerEffect {}

#[derive(Debug, Clone)]
pub enum TabManagerMessage {
    /// For routing messages to Tab instances in the TabManager.
    TabUpdated(TabId, TabMessage),

    OpenTab(ViewRequest),
    CloseTab(TabId),
    FocusTab(TabId),
}

#[derive(Debug, Clone)]
pub enum TabMessage {
    Dashboard(DashboardMessage),
    SpellForm(SpellFormMessage),
    SpellList(SpellListMessage),
}
