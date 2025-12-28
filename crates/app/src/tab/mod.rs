mod bar;
mod content;
mod message;
mod request;

use std::sync::atomic::{AtomicU64, Ordering};

use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{self, Space, column},
};
use tracing::{debug, error};

use crate::{
    message::Message,
    tab::bar::tab_bar,
    view::{Dashboard, SpellForm, SpellList},
};

pub use content::*;
pub use message::*;
pub use request::*;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TabId(u64);

impl TabId {
    fn raw(&self) -> u64 {
        self.0
    }

    /// Calculate the numeric difference between two `TabId`s.
    ///
    /// Given that the raw value of a `TabId` is `u64`, the difference is absolute to avoid underflow issues.
    fn difference(&self, other: TabId) -> u64 {
        self.raw().abs_diff(other.raw())
    }
}

/// This enum defines the actual elements that can be shown as tabs.
pub enum Tab {
    Dashboard(Dashboard),
    SpellForm(Box<SpellForm>),
    SpellList(Box<SpellList>),
}

impl Tab {
    fn id(&self) -> TabId {
        match self {
            Tab::Dashboard(dashboard) => dashboard.id(),
            Tab::SpellForm(spell_form) => spell_form.id(),
            Tab::SpellList(spell_list) => spell_list.id(),
        }
    }

    fn title(&self) -> &str {
        match self {
            Tab::Dashboard(dashboard) => dashboard.title(),
            Tab::SpellForm(spell_form) => spell_form.title(),
            Tab::SpellList(spell_list) => spell_list.title(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TabAction {
    Open(TabRequest),
    Close(TabId),
    Focus(TabId),
}

pub struct TabManager {
    tabs: Vec<Tab>,
    active: TabId,
}

impl Default for TabManager {
    fn default() -> Self {
        let mut tabs = Vec::with_capacity(1);
        let id = Self::unique();
        let initial_tab = Tab::Dashboard(Dashboard::new(id));
        tabs.push(initial_tab);

        Self { tabs, active: id }
    }
}

impl TabManager {
    fn unique() -> TabId {
        TabId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    fn get(&self, id: TabId) -> Option<&Tab> {
        self.tabs.iter().find(|tab| id == tab.id())
    }

    fn get_mut(&mut self, id: TabId) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|tab| id == tab.id())
    }

    pub fn perform(&mut self, action: TabAction) -> Task<Message> {
        debug!("performing tab action '{action:?}'");

        match action {
            TabAction::Open(request) => return self.handle_request(request),
            TabAction::Close(tab_id) => {
                self.tabs.retain(|tab| tab.id() != tab_id);

                // Only update active if it is the tab being closed
                if self.active == tab_id {
                    debug!("closing active tab");

                    let closest_tab = self
                        .tabs
                        .iter()
                        .min_by_key(|tab| tab.id().difference(tab_id))
                        .expect("the dashboard cannot be closed, and always exists");

                    self.active = closest_tab.id();
                }
            }
            TabAction::Focus(tab_id) => self.active = tab_id,
        }

        Task::none()
    }

    pub fn update(&mut self, tab_id: TabId, message: TabMessage) -> Task<Message> {
        let Some(tab) = self.get_mut(tab_id) else {
            error!("could not find tab with id '{tab_id:?}'");

            return Task::none();
        };

        match message {
            TabMessage::Dashboard(dashboard_message) => {
                let Tab::Dashboard(dashboard) = tab else {
                    error!(
                        "tab with id '{tab_id:?}' does not match message of type '{dashboard_message:?}'"
                    );

                    return Task::none();
                };

                dashboard.update(dashboard_message)
            }
            TabMessage::SpellForm(spell_form_message) => {
                let Tab::SpellForm(spell_form) = tab else {
                    error!(
                        "tab with id '{tab_id:?}' does not match message of type '{spell_form_message:?}'"
                    );

                    return Task::none();
                };

                spell_form.update(spell_form_message)
            }
            TabMessage::SpellList(spell_list_message) => {
                let Tab::SpellList(spell_list) = tab else {
                    error!(
                        "tab with id '{tab_id:?}' does not match message of type '{spell_list_message:?}'"
                    );

                    return Task::none();
                };

                spell_list.update(spell_list_message)
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let tab_bar = tab_bar(&self.tabs, self.active);

        let divider = widget::rule::horizontal(1).style(style::rule::default);

        let Some(tab) = self.get(self.active) else {
            error!("could not find tab '{:?}'", self.active);

            return Space::new().into();
        };

        let view = match tab {
            Tab::Dashboard(dashboard) => dashboard.view().map(TabMessage::Dashboard),
            Tab::SpellForm(spell_form) => spell_form.view().map(TabMessage::SpellForm),
            Tab::SpellList(spell_list) => spell_list.view().map(TabMessage::SpellList),
        }
        .map(|tm| Message::Tab(tab.id(), tm));

        let constrained_view = widget::container(view)
            .max_width(1200)
            .width(Fill)
            .center_x(Fill);

        column![tab_bar, divider, constrained_view].into()
    }

    pub fn handle_request(&mut self, request: TabRequest) -> Task<Message> {
        debug!("handling tab request: {request:?}");

        match request {
            TabRequest::Dashboard => {
                // Check if dashboard already exists
                let Some(tab) = self.tab_exists(|tab| matches!(tab, Tab::Dashboard(_))) else {
                    let id = Self::unique();
                    let new_tab = Tab::Dashboard(Dashboard::new(id));
                    self.tabs.push(new_tab);
                    self.active = id;

                    return Task::none();
                };

                self.active = tab.id();
            }
            TabRequest::SpellForm { mode } => {
                let id = Self::unique();
                let form = SpellForm::new(id, mode);
                self.tabs.push(Tab::SpellForm(Box::new(form)));
                self.active = id;
            }
            TabRequest::SpellList => {
                // Check if index already exists
                let Some(tab) = self.tab_exists(|tab| matches!(tab, Tab::SpellList(_))) else {
                    let id = Self::unique();
                    let new_tab = Tab::SpellList(Box::new(SpellList::new(id)));
                    self.tabs.push(new_tab);
                    self.active = id;

                    return Task::none();
                };

                self.active = tab.id();
            }
        }

        Task::none()
    }

    /// Returns a reference to a `Tab` if it exists.
    fn tab_exists(&self, predicate: impl Fn(&Tab) -> bool) -> Option<&Tab> {
        self.tabs.iter().find(|tab| predicate(tab))
    }
}
