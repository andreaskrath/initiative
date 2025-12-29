use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{self, Space, column},
};
use tracing::{debug, error};

use crate::{
    message::Message,
    tab::{Tab, TabAction, TabId, TabMessage, bar::tab_bar},
    view::{Dashboard, SpellForm, SpellList, ViewContent, ViewRequest},
};

pub struct TabManager {
    tabs: Vec<(TabId, Tab)>,
    active: TabId,
}

impl Default for TabManager {
    fn default() -> Self {
        let mut tabs = Vec::with_capacity(1);
        let id = TabId::unique();
        let initial_tab = Tab::Dashboard(Dashboard::new());
        tabs.push((id, initial_tab));

        Self { tabs, active: id }
    }
}

impl TabManager {
    pub fn perform(&mut self, action: TabAction) -> Task<Message> {
        debug!("performing tab action '{action:?}'");

        match action {
            TabAction::Open(request) => return self.handle_request(request),
            TabAction::Close(close_id) => {
                self.tabs.retain(|(tab_id, _)| *tab_id != close_id);

                // TODO: Never redirect to Dashboard (TabId(0)), unless it is the last tab open
                // Only update active if it is the tab being closed
                if self.active == close_id {
                    debug!("closing active tab");

                    let (closest_id, _) = self
                        .tabs
                        .iter()
                        .min_by_key(|(tab_id, _)| tab_id.difference(close_id))
                        .expect("the dashboard cannot be closed, and always exists");

                    self.active = *closest_id;
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
        .map(|tm| Message::Tab(self.active, tm));

        let constrained_view = widget::container(view)
            .max_width(1200)
            .width(Fill)
            .center_x(Fill);

        column![tab_bar, divider, constrained_view].into()
    }

    fn get(&self, id: TabId) -> Option<&Tab> {
        self.tabs
            .iter()
            .find(|(tab_id, _)| *tab_id == id)
            .map(|(_, tab)| tab)
    }

    fn get_mut(&mut self, id: TabId) -> Option<&mut Tab> {
        self.tabs
            .iter_mut()
            .find(|(tab_id, _)| *tab_id == id)
            .map(|(_, tab)| tab)
    }

    fn handle_request(&mut self, request: ViewRequest) -> Task<Message> {
        debug!("handling tab request: {request:?}");

        match request {
            ViewRequest::SpellForm { mode } => {
                let id = TabId::unique();
                let tab = Tab::SpellForm(Box::new(SpellForm::new(mode)));
                self.tabs.push((id, tab));
                self.active = id;
            }
            ViewRequest::SpellList => {
                // Check if index already exists
                let Some(tab_id) = self.tab_exists(|tab| matches!(tab, Tab::SpellList(_))) else {
                    let id = TabId::unique();
                    let new_tab = Tab::SpellList(Box::new(SpellList::new()));
                    self.tabs.push((id, new_tab));
                    self.active = id;

                    return Task::none();
                };

                self.active = tab_id;
            }
        }

        Task::none()
    }

    /// Returns a reference to a `Tab` if it exists.
    fn tab_exists(&self, predicate: impl Fn(&Tab) -> bool) -> Option<TabId> {
        self.tabs
            .iter()
            .find(|(_, tab)| predicate(tab))
            .map(|(tab_id, _)| *tab_id)
    }
}
